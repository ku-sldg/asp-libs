
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

    let myaspargs : ASP_ARGS_Hashfile_Appr = serde_json::from_value(args)
    .context("Could not decode ASP_ARGS for ASP hashfile_appr")?;

        let filename: String = myaspargs.filepath_golden;
    
        // TODO:  consider making this an ASP_ARGS field
        let env_var_key = "AM_ROOT";
        let env_var_string = match std::env::var(env_var_key) {
            Ok(val) => val,
            Err(_e) => {
                panic!("Did not set environment variable AM_ROOT")
            }
        };
    
        //let filename_string = (filename).clone();
        let filename_full = format! {"{env_var_string}{filename}"};

        eprint!("Attempting to read from file: {}\n", filename_full);
        let golden_bytes = std::fs::read(filename_full)?;

        // Suppose the file contents are to be extracted from evidence...

        let evidence_in = ev.first().context("No file evidence found")?;

        let bytes_equal: bool = golden_bytes.eq(evidence_in);

        match bytes_equal {
            true => Ok(Ok(())),
            false => Ok(Err(anyhow::anyhow!("File contents do not match"))),
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
