#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland;
use rust_am_lib::debug_print;

use serde::{Deserialize, Serialize};
use std::fs;


// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Sig_Tpm_Appr {
    tpm_folder: String
}

fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    debug_print!("Starting sig_tpm_appr ASP execution\n");
  
  /*
    let tpm_folder_value = args
        .get("tpm_folder")
        .context("'tpm_folder' argument not provided to ASP, sig_tpm_appr")?;
     */
  
    let myaspargs : ASP_ARGS_Sig_Tpm_Appr = serde_json::from_value(args)
    .context("Could not decode ASP_ARGS for ASP sig_tpm_appr")?;

    if true {
        //let tpm_folder: String = tpm_folder_value.to_string();
        let tpm_folder : String = myaspargs.tpm_folder;
        debug_print!("Using tpm_folder: {}\n", tpm_folder);
        let message_signature = ev.first().context("No message signature found")?;
        debug_print!(
            "Got message signature of {} bytes\n",
            message_signature.len()
        );
        let message_sig_input = ev
            .get(1..)
            .context("No message found")?
            .to_vec()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();
        debug_print!("Got message input of {} bytes\n", message_sig_input.len());
        // Use openssl to verify the signature
        // TODO: fix reading key.pem with actual public key from args
        let pkey =
            openssl::pkey::PKey::public_key_from_pem(&fs::read(format!("{tpm_folder}/key.pem"))?)?;
        let mut verifier =
            openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
        verifier.set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)?;
        let res = verifier.verify_oneshot(&message_signature, &message_sig_input)?;

        if res {
            Ok(Ok(()))
        } else {
            Ok(Err(anyhow::anyhow!("TPM Signature verification failed")))
        }
    } else {
        Err(anyhow::anyhow!(
            "Failed to decode 'tpm_folder' ASP arg as JSON String in sig_tpm_appr ASP"
        ))
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
