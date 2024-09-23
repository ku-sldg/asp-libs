// TEMPLATE.txt
// General structure for ASP's written in rust

// Common Packages
use rust_am_lib::copland::*;
use anyhow::{Context, Result};
use std::env;

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Sha256, Digest};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body() -> Result<String> {

    // For every ASP, an ASPRunRequest appears as the single command-line argument
    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow::anyhow!("ASPRunRequest not supplied as command line argument"));
    }

    let json_request = &args[1];
    // May fail.  If so, return an Err Result
    let req: ASPRunRequest = serde_json::from_str(json_request)?;

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let args_map = req.ASP_ARGS;
    let filename = &args_map.get("filepath").context("filename argument not provided to ASP, hashfile_id")?;

    let bytes = std::fs::read(filename)?; // Vec<u8>

    let hash = Sha256::digest(&bytes);

    // End of code specific for this ASP.

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission
    let hash_b64: String = base64::encode(hash);

    // Step 2:
    // wrap the value as Evidence
    let evidence = RawEv::RawEv(vec![hash_b64]);

    // Step 3:
    // Construct the ASPRunResponse with this evidence.
    let  response = successfulASPRunResponse (evidence);
    let response_json = serde_json::to_string(&response)?;
    Ok (response_json)
}

// Main simply invokes the body() funciton above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {

    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let  response = failureASPRunResponse (_error.to_string());
            // If an error occurs converting the failure response to JSON
            // there is nothing else to do but panic.
            // This should never happen.
            serde_json::to_string(&response).unwrap_or_else(|error| {panic!("Failed to json.encode failure response: {error:?}");})
        }
    };
    // The ASP output (ASPRunRequest) is written to stdout.
    // The caller will capture stdout to receive the response from this ASP.
    println!("{response_json}");
}
