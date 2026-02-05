// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
use rust_am_lib::debug_print;

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    debug_print!("Starting sig_appr ASP execution\n");
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
    //let good_pubkey_path = "../../../common_files/unsecure_pub_key_dont_use.pem";
    //let bad_pubkey_path = "../../../common_files/unsecure_pub_key_dont_use_bad.pem";
    let key = include_bytes!("../../../common_files/unsecure_pub_key_dont_use.pem");
    let pkey = openssl::pkey::PKey::public_key_from_pem(key)?;
    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    verifier.update(&message_sig_input)?;
    let res = verifier.verify(&message_signature)?;

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission

    /*
     let hash_b64: String = BASE64_STANDARD.encode(bytes);
    */

    if res {
        Ok(Ok(()))
    } else {
        Ok(Err(anyhow::anyhow!("Signature verification failed")))
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
