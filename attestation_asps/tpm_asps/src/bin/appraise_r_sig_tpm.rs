// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland;
use std::fs;

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
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
    let pkey = openssl::pkey::PKey::public_key_from_pem(&fs::read("key.pem")?)?;
    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    verifier.set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)?;
    let res = verifier.verify_oneshot(&hex::decode(message_signature)?, &message_sig_input)?;

    let res_string: String = if res {
        "PASSED".to_string()
    } else {
        "FAILED".to_string()
    };
    Ok(vec![res_string.as_bytes().to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    copland::handle_body(body);
}
