use rust_am_lib::copland;

use tss_esapi::{
    attributes::{ObjectAttributesBuilder, SessionAttributesBuilder},
    constants::{
        tss::{TPM2_RH_NULL, TPM2_ST_HASHCHECK},
        SessionType,
    },
    handles::{KeyHandle, ObjectHandle},
    interface_types::{
        algorithm::{HashingAlgorithm, PublicAlgorithm},
        key_bits::RsaKeyBits,
        resource_handles::Hierarchy,
        session_handles::PolicySession,
    },
    structures::{
        Digest, HashScheme, HashcheckTicket, Nonce, PcrSelectionListBuilder, PcrSlot, Private,
        Public, PublicBuilder, PublicKeyRsa, PublicRsaParametersBuilder, RsaExponent, RsaScheme,
        RsaSignature, Signature, SignatureScheme, SymmetricCipherParameters, SymmetricDefinition,
        SymmetricDefinitionObject,
    },
    traits::UnMarshall,
    tss2_esys::TPMT_TK_HASHCHECK,
    Context, TctiNameConf,
};

use anyhow::Context as _;
use std::{env, fs, path::Path};

fn body(ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> anyhow::Result<copland::EvidenceT> {
    let env_var_key = "AM_TPM_DIR";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable AM_TPM_DIR")
        }
    };

    // Code adapted from tpm_sign
    let use_key_context: bool = true; // true = try to load keys from context
                                      // false = reload keys manually every time
    let mut context = Context::new(TctiNameConf::from_environment_variable()?)?;

    let approved_policy = Digest::try_from(fs::read(format!(
        "{env_var_string}/policy/pcr.policy_desired"
    ))?)?;
    let policy_digest = Digest::try_from(&openssl::sha::sha256(&approved_policy)[..])?;
    let session = context
        .start_auth_session(
            None,
            None,
            None,
            SessionType::Policy,
            SymmetricDefinition::AES_128_CFB,
            HashingAlgorithm::Sha256,
        )?
        .ok_or(tss_esapi::Error::WrapperError(
            tss_esapi::WrapperErrorKind::WrongValueFromTpm,
        ))?;
    let (session_attributes, session_attributes_mask) = SessionAttributesBuilder::new()
        .with_decrypt(true)
        .with_encrypt(true)
        .build();
    context.tr_sess_set_attributes(session, session_attributes, session_attributes_mask)?;
    let policy_session: PolicySession = session.try_into()?;
    set_policy(&env_var_string, &mut context, policy_session)?;

    let policy_key_handle = if use_key_context {
        if let Ok(key_handle) = reload_key_context(&mut context, env::temp_dir().join("policy.ctx"))
        {
            key_handle
        } else {
            let policy_key_handle = load_external_signing_key(&env_var_string, &mut context)?;
            let _ = save_key_context(
                &mut context,
                policy_key_handle.into(),
                env::temp_dir().join("policy.ctx"),
            );
            policy_key_handle
        }
    } else {
        load_external_signing_key(&env_var_string, &mut context)?
    };
    let key_sign = context.tr_get_name(policy_key_handle.into())?;

    let policy_signature = Signature::RsaSsa(RsaSignature::create(
        HashingAlgorithm::Sha256,
        PublicKeyRsa::try_from(fs::read(format!("{env_var_string}/policy/pcr.signature"))?)?,
    )?);
    let check_ticket =
        context.verify_signature(policy_key_handle, policy_digest, policy_signature)?;
    // policy_key_handle is no longer necessary and keeping it loaded slows things down
    context.flush_context(policy_key_handle.into())?;

    context.policy_authorize(
        policy_session,
        approved_policy,
        Nonce::default(),
        &key_sign,
        check_ticket,
    )?;

    let ev_flattened: Vec<u8> = ev.into_iter().flatten().collect();
    let digest = Digest::try_from(&openssl::sha::sha256(&ev_flattened)[..])?;

    let key_handle = load_signing_key(&env_var_string, &mut context, use_key_context)?;

    let signature = context.execute_with_session(Some(session), |context| {
        context.sign(
            key_handle,
            digest.clone(),
            SignatureScheme::RsaPss {
                //SignatureScheme::EcDsa {
                hash_scheme: HashScheme::new(HashingAlgorithm::Sha256),
            },
            // temporary workaround because validation is erroneously non-optional in tss_esapi v7.5.1
            HashcheckTicket::try_from(TPMT_TK_HASHCHECK {
                tag: TPM2_ST_HASHCHECK,
                hierarchy: TPM2_RH_NULL,
                digest: Default::default(),
            })?,
        )
    })?;
    let signature = match signature {
        Signature::RsaSsa(sig) | Signature::RsaPss(sig) => sig.signature().value().to_vec(),
        _ => return Err(anyhow::anyhow!("really bad")),
    };
    Ok(vec![signature])
}

fn load_external_signing_key(
    env_var_string: &String,
    context: &mut Context,
) -> anyhow::Result<KeyHandle> {
    let der = fs::read(format!("{env_var_string}/policy/policy_key.pem"))?;
    let key = openssl::rsa::Rsa::public_key_from_pem(&der)?;
    let modulus = key.n().to_vec();
    let exponent = key
        .e()
        .to_vec()
        .iter()
        .enumerate()
        .fold(0u32, |v, (i, &x)| v + (u32::from(x) << (8 * i as u32)));

    let public_policy_key = PublicBuilder::new()
        .with_public_algorithm(PublicAlgorithm::Rsa)
        .with_name_hashing_algorithm(HashingAlgorithm::Sha256)
        .with_rsa_parameters(
            PublicRsaParametersBuilder::new()
                .with_symmetric(SymmetricDefinitionObject::Null)
                .with_scheme(RsaScheme::Null)
                .with_key_bits(RsaKeyBits::try_from(modulus.len() as u16 * 8)?)
                .with_exponent(RsaExponent::create(exponent)?)
                .build()?,
        )
        .with_object_attributes(
            ObjectAttributesBuilder::new()
                .with_sign_encrypt(true)
                .with_decrypt(true)
                .with_user_with_auth(true)
                .build()?,
        )
        .with_rsa_unique_identifier(PublicKeyRsa::try_from(modulus)?)
        .build()?;

    let policy_key_handle = context.load_external_public(public_policy_key, Hierarchy::Owner)?;

    Ok(policy_key_handle)
}

fn set_policy(
    env_var_string: &String,
    context: &mut Context,
    session: PolicySession,
) -> anyhow::Result<()> {
    let pcr_selection_list = PcrSelectionListBuilder::new()
        .with_selection(HashingAlgorithm::Sha256, &[PcrSlot::Slot0])
        .build()?;

    let concatenated_pcr_values = fs::read(format!("{env_var_string}/policy/pcr0.sha256"))?;
    let hashed_pcrs = Digest::try_from(&openssl::sha::sha256(&concatenated_pcr_values)[..])?;

    context.policy_pcr(session, hashed_pcrs, pcr_selection_list)?;

    Ok(())
}

fn reload_key_context<P: AsRef<Path>>(
    context: &mut Context,
    context_path: P,
) -> anyhow::Result<KeyHandle> {
    let buf = fs::read(context_path)?;
    let ctx = serde_json::from_slice(&buf)?;
    Ok(context.context_load(ctx)?.into())
}

fn save_key_context<P: AsRef<Path>>(
    context: &mut Context,
    handle: ObjectHandle,
    path: P,
) -> anyhow::Result<()> {
    let policy_context = context.context_save(handle)?;
    fs::write(path, serde_json::to_vec(&policy_context)?)?;
    Ok(())
}

fn load_signing_key(
    env_var_string: &String,
    context: &mut Context,
    use_key_context: bool,
) -> anyhow::Result<KeyHandle> {
    if use_key_context {
        if let Ok(key_handle) = reload_key_context(context, env::temp_dir().join("signing.ctx")) {
            return Ok(key_handle);
        }
    }

    let old_session_handles = context.sessions();

    let auth_session = context
        .start_auth_session(
            None,
            None,
            None,
            SessionType::Hmac,
            SymmetricDefinition::AES_128_CFB,
            HashingAlgorithm::Sha256,
        )?
        .ok_or(tss_esapi::Error::WrapperError(
            tss_esapi::WrapperErrorKind::WrongValueFromTpm,
        ))?;
    let (session_attributes, session_attributes_mask) = SessionAttributesBuilder::new()
        .with_decrypt(true)
        .with_encrypt(true)
        .build();
    context.tr_sess_set_attributes(auth_session, session_attributes, session_attributes_mask)?;

    context.set_sessions((Some(auth_session), None, None));
    let primary_key_handle = create_primary_handle(context)?;
    let public = Public::unmarshall(
        fs::read(format!("{env_var_string}/key.pub"))?
            .get(2..)
            .context("Slicing out of bounds")?,
    )?;
    let private = Private::try_from(
        fs::read(format!("{env_var_string}/key.priv"))?
            .get(2..)
            .context("Slicing out of bounds")?,
    )?;
    let key_handle = context.load(primary_key_handle, private, public)?;
    // primary_key_handle is no longer necessary and keeping it loaded slows things down
    context.flush_context(primary_key_handle.into())?;
    context.set_sessions(old_session_handles);

    if use_key_context {
        let _ = save_key_context(
            context,
            key_handle.into(),
            env::temp_dir().join("signing.ctx"),
        );
    }

    Ok(key_handle)
}

fn create_primary_handle(context: &mut Context) -> anyhow::Result<KeyHandle> {
    let object_attributes = ObjectAttributesBuilder::new()
        .with_fixed_tpm(true)
        .with_fixed_parent(true)
        .with_st_clear(false)
        .with_sensitive_data_origin(true)
        .with_user_with_auth(true)
        .with_decrypt(true)
        .with_restricted(true)
        .build()?;

    let primary_pub = PublicBuilder::new()
        .with_public_algorithm(PublicAlgorithm::SymCipher)
        .with_name_hashing_algorithm(HashingAlgorithm::Sha256)
        .with_object_attributes(object_attributes)
        .with_symmetric_cipher_parameters(SymmetricCipherParameters::new(
            SymmetricDefinitionObject::AES_128_CFB,
        ))
        .with_symmetric_cipher_unique_identifier(Digest::default())
        .build()?;

    let result = context.create_primary(Hierarchy::Owner, primary_pub, None, None, None, None)?;

    Ok(result.key_handle)
}

fn main() {
    copland::handle_body(body)
}
