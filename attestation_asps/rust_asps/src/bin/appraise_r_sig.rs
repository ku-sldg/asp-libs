// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

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
    let key = include_bytes!("../../../../common_files/unsecure_pub_key_dont_use.pem");
    let pkey = openssl::pkey::PKey::public_key_from_pem(key)?;
    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    verifier.update(&message_sig_input)?;
    let signature = hex::decode(message_signature)?;
    let res = verifier.verify(&signature)?;

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
    Ok(vec![res_string.as_bytes().to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
