// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    let filename = args
        .get("filepath")
        .context("filepath argument not provided to ASP, r_readfile_id")?;

    let env_var_key = "DEMO_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable DEMO_ROOT")
        }
    };

    let filename_string = (*filename).clone();
    let filename_full = format! {"{env_var_string}{filename_string}"};

    print!("Attempting to read from file: {}\n", filename_full); 

    let bytes = std::fs::read(&filename_full).context(
        "could not read file contents in ASP, r_readfile_id.  Perhaps the file doesn't exits?",
    )?; // Vec<u8>
    Ok(vec![bytes])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
