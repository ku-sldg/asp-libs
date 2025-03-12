// Common Packages
use anyhow::{Context, Result};
use lib::copland;
use std::fs;

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    let env_var_key = "AM_TPM_DIR";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable AM_TPM_DIR")
        }
    };

    let message_signature = ev.first().context("No message signature found")?;
    let message_sig_input = ev
        .get(1..)
        .context("No message found")?
        .to_vec()
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();
    // Use openssl to verify the signature
    // TODO: fix reading key.pem with actual public key from args
    let pkey =
        openssl::pkey::PKey::public_key_from_pem(&fs::read(format!("{env_var_string}/key.pem"))?)?;
    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    verifier.set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)?;
    let res = verifier.verify_oneshot(&message_signature, &message_sig_input)?;

    if res {
        Ok(Ok(()))
    } else {
        Ok(Err(anyhow::anyhow!("TPM Signature verification failed")))
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    copland::handle_appraisal_body(body);
}
