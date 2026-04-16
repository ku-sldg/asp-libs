#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};

use rust_am_lib::{
    copland::{self, handle_appraisal_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Hashfile_Appr {
    env_var_golden: String,
    filepath_golden: String,
    /// Base64-encoded SHA-256 golden hash injected from the Copland evidence
    /// bundle at run time.  When present this takes priority over filepath_golden,
    /// eliminating the need for a binary .bin golden file on disk.
    golden_b64: Option<String>,
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    debug_print!("Starting hashfile_appr ASP execution\n");
    let myaspargs: ASP_ARGS_Hashfile_Appr =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP hashfile_appr")?;

    let golden_bytes: Vec<u8> = match &myaspargs.golden_b64 {
        Some(b64) => {
            debug_print!("Using base64-encoded golden from ASP_ARGS\n");
            B64.decode(b64).context("Could not base64-decode golden_b64")?
        }
        None => {
            let env_var_string =
                rust_am_lib::copland::get_env_var_val(myaspargs.env_var_golden)?;
            let filename_full = format!("{}{}", env_var_string, myaspargs.filepath_golden);
            debug_print!("Attempting to read from file: {}\n", filename_full);
            let bytes = std::fs::read(&filename_full)?;
            debug_print!("Read {} bytes from golden file\n", bytes.len());
            bytes
        }
    };

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
