
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};


use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Hashfile_Appr {
    filepath_golden: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    
    /*
    
    let golden_filename_value = args
        .get("filepath_golden")
        .context("'filepath_golden' argument not provided to ASP, hashfile_appr")?;

    if golden_filename_value.is_string()
    {

    */

    let myaspargs : ASP_ARGS_Hashfile_Appr = serde_json::from_value(args)
    .context("Could not parse ASP_ARGS for ASP hashfile_appr")?;
        //let golden_filename: String = golden_filename_value.to_string();

        let filename: String = myaspargs.filepath_golden;
    
        /*
        args
            .get("filepath-golden")
            .context("filepath-golden argument not provided to ASP, r_readfile_id")?;
        */
    
        let env_var_key = "AM_ROOT";
        let env_var_string = match std::env::var(env_var_key) {
            Ok(val) => val,
            Err(_e) => {
                panic!("Did not set environment variable AM_ROOT")
            }
        };
    
        let filename_string = (filename).clone();
        let filename_full = format! {"{env_var_string}{filename_string}"};




        eprint!("Attempting to read from file: {}\n", filename_full.clone());
        let golden_bytes = std::fs::read(filename_full)?;

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

        match bytes_equal {
            true => Ok(Ok(())),
            false => Ok(Err(anyhow::anyhow!("File contents do not match"))),
        }
    }
    /*
    else {
        Err(anyhow::anyhow!("Failed to decode 'filepath-golden' ASP arg as JSON String in hashfile_appr ASP"))
    }
}
    */

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
