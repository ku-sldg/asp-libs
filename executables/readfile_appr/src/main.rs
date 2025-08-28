#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
use rust_am_lib::debug_print;
use serde::{Deserialize, Serialize};

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Readfile_Appr {
    env_var_golden: String,
    filepath_golden: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
  
    debug_print!("Starting readfile_appr ASP execution\n");

    let myaspargs : ASP_ARGS_Readfile_Appr = serde_json::from_value(args)
    .context("Could not decode ASP_ARGS for ASP readfile_appr")?;

    let env_var: String = myaspargs.env_var_golden;
    let filename: String = myaspargs.filepath_golden;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let golden_filename = format! {"{env_var_string}{filename}"};
  
    debug_print!("Attempting to read from file: {}\n", golden_filename);

    let golden_bytes: Vec<u8> = std::fs::read(&golden_filename.to_string())?; // Vec<u8>
  
    debug_print!("Read {} bytes from golden file\n", golden_bytes.len());

    // Suppose the file contents are to be extracted from evidence...
    let evidence_in = ev.first().context("No file evidence found")?;

    let bytes_equal: bool = golden_bytes.eq(evidence_in);

    match bytes_equal {
        true => Ok(Ok(())),
        false => Ok(Err(anyhow::anyhow!("readfile: File contents do not match"))),
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
