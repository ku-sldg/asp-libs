// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

use serde::{Deserialize, Serialize};
use serde_json::{Value};


/*
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Appraise_Hashfile {
    filepath_golden: String
}
    */

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Hashdir {
    env_var: String,
    paths: Vec<String>,
    filepath_golden: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {

    let myaspargs : ASP_ARGS_Hashdir = serde_json::from_value(args)
    .context("Could not parse ASP_ARGS for ASP appraise_r_hashfile_id")?;


    /*
    let golden_filename = args
        .get("filepath-golden")
        .context("filepath-golden argument not provided to ASP, appraise_r_hashfile_id")?;
    */
    let golden_filename = myaspargs.filepath_golden;

    let env_var_key = "DEMO_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable DEMO_ROOT")
        }
    };

    let filename_string = (golden_filename).clone();
    let filename_full = format! {"{env_var_string}{filename_string}"};

    eprint!("Attempting to read from file: {}\n", filename_full);

    let golden_bytes = std::fs::read(filename_full)?; // Vec<u8>

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission
    //let golden_bytes_b64: String = base64::encode(bytes);

    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    // Evidence is always base64 encoded, so decode this
    // Using HEX decoding for now...will switch to b64
    let bytes_equal: bool = golden_bytes.eq(evidence_in);

    let out_contents: String = match bytes_equal {
        true => "PASSED".to_string(),
        false => "FAILED".to_string(),
    };

    Ok(vec![out_contents.as_bytes().to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
