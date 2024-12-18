// Very simple use of the sysinfo crate.
// Returns seconds since most recent book.

// The sysinfo crate provides access to a wide range of system information,
// including a variety of dynamic characteristics.


use rust_am_lib::copland::*;
use anyhow::{Result};
use std::env;
use base64::prelude::*;

use sysinfo::System;

fn body() -> Result<String> {

    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow::anyhow!("ASPRunRequest not supplied as command line argument"));
    }
    let json_request = &args[1];

    // decoding the ASPRunRequest just to confirm
    // that we are being called as expected, even though
    // no information in the request is required for execution.
    let _req: ASPRunRequest = serde_json::from_str(json_request)?;

    // This ASP has no arguments.
    //    let args_map = req.ASP_ARGS;

    // returns seconds since last boot.
    let up = System::uptime();
    // Base64 encode the evidence
    let up_b64: String = BASE64_STANDARD.encode(up.to_be_bytes());

    let evidence = RawEv::RawEv(vec![up_b64]);

    let  response = successfulASPRunResponse (evidence);

    let response_json = serde_json::to_string(&response)?;
    Ok (response_json)
}

fn main() {

    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let  response = failureASPRunResponse (_error.to_string());
            serde_json::to_string(&response).unwrap_or_else(|error| {panic!("Failed to json.encode failure response: {error:?}");})
        }
    };

    // The ASP output (ASPRunRequest) is written to stdout.
    // The caller will capture stdout to receive the response from this ASP.
    println!("{response_json}");
}
