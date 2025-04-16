
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};


// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Readfile {
    filepath: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {

    let myaspargs : ASP_ARGS_Readfile = serde_json::from_value(args)
    .context("Could not decode ASP_ARGS for ASP readfile")?;
    
    let filename : String = myaspargs.filepath;

    eprint!("Attempting to read from file: {}\n", filename);

    let bytes = std::fs::read(&filename).context(
        "could not read file contents in ASP, readfile.  Perhaps the file doesn't exist?",
    )?;
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
