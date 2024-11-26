// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::*;
use std::env;
/*
use base64::prelude::*;
*/
use hex;

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body() -> Result<String> {
    // For every ASP, an ASPRunRequest appears as the single command-line argument
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow::anyhow!(
            "ASPRunRequest not supplied as command line argument"
        ));
    }

    let json_request = &args[1];
    // May fail.  If so, return an Err Result
    let req: ASPRunRequest = serde_json::from_str(json_request)?;

    // Code for specific for this ASP.

    let message_in = match req.RAWEV {
        RawEv::RawEv(x) => x,
    };
    let message_signature = message_in[0].clone();
    let message_sig_input = message_in[1..].join("");
    // Use openssl to verify the signature
    let key = include_bytes!("../../../../common_files/unsecure_pub_key_dont_use.pem");
    let pkey = openssl::pkey::PKey::public_key_from_pem(key)?;
    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    verifier.update(message_sig_input.as_bytes())?;
    let signature = hex::decode(message_signature)?;
    let res = verifier.verify(&signature)?;

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission

    /*
     let hash_b64: String = BASE64_STANDARD.encode(bytes);
    */

    let res_string : String = if res {"PASSED".to_string()} else {"FAILED".to_string()};
    // Using HEX encoding for now...will switch to b64
    let hash_b64: String = hex::encode(res_string);

    // Step 2:
    // wrap the value as Evidence
    let evidence = RawEv::RawEv(vec![hash_b64]);

    // Step 3:
    // Construct the ASPRunResponse with this evidence.
    let response = successfulASPRunResponse(evidence);
    let response_json = serde_json::to_string(&response)?;
    Ok(response_json)
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let response = failureASPRunResponse(_error.to_string());
            // If an error occurs converting the failure response to JSON
            // there is nothing else to do but panic.
            // This should never happen.
            serde_json::to_string(&response).unwrap_or_else(|error| {
                panic!("Failed to json.encode failure response: {error:?}");
            })
        }
    };
    // The ASP output (ASPRunRequest) is written to stdout.
    // The caller will capture stdout to receive the response from this ASP.
    println!("{response_json}");
}
