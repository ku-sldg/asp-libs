// Common Packages
use anyhow::Result;
use rust_am_lib::copland;
use std::fs;
/*
use base64::prelude::*;
*/

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(raw_ev: copland::RawEv, _args: copland::ASP_ARGS) -> Result<copland::RawEv> {
    // Code for specific for this ASP.
    let copland::RawEv::RawEv(message_in) = raw_ev;
    let message_signature = message_in[0].clone();
    let message_sig_input = message_in[1..].join("");
    // Use openssl to verify the signature
    // TODO: fix reading key.pem with actual public key from args
    let pkey = openssl::pkey::PKey::public_key_from_pem(&fs::read("key.pem")?)?;
    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    verifier.set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)?;
    let res = verifier.verify_oneshot(
        &hex::decode(message_signature)?,
        message_sig_input.as_bytes(),
    )?;

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission

    /*
     let hash_b64: String = BASE64_STANDARD.encode(bytes);
    */

    let res_string: String = if res {
        "PASSED".to_string()
    } else {
        "FAILED".to_string()
    };
    // Using HEX encoding for now...will switch to b64
    let hash_b64: String = hex::encode(res_string);

    // Step 2:
    // wrap the value as Evidence
    let evidence = copland::RawEv::RawEv(vec![hash_b64]);
    Ok(evidence)
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    copland::handle_body(body);
}
