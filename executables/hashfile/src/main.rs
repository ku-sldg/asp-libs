// TEMPLATE.txt
// General structure for ASP's written in rust

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Digest, Sha256};

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Hashfile {
    env_var: String,
    filepath: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.

    let myaspargs : ASP_ARGS_Hashfile = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP hashfile")?;
    
    let env_var: String = myaspargs.env_var;
    let filename : String = myaspargs.filepath;

    let env_var_string = match std::env::var(&env_var) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable {}\n", env_var)
        }
    };

    //let filename_string = (filename).clone();
    let filename_full = format! {"{env_var_string}{filename}"};


    eprint!("Attempting to read from file: {}\n", filename_full);
    let bytes = std::fs::read(filename_full)?; // Vec<u8>

    let hash = Sha256::digest(&bytes);
    Ok(vec![hash.to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
