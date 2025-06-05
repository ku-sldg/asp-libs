// TEMPLATE.txt
// General structure for ASP's written in rust

use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Digest, Sha256};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting hashfile ASP execution\n");
    let filename_value = args
        .get("filepath")
        .context("'filepath' argument not provided to ASP, hashfile")?;

    if filename_value.is_string() {
        let filename: String = filename_value.to_string();
        debug_print!("Attempting to read from file: {}\n", filename);
        let bytes = std::fs::read(filename)?; // Vec<u8>
        debug_print!("Read {} bytes from file\n", bytes.len());
        let hash = Sha256::digest(&bytes);
        debug_print!("Generated hash of {} bytes\n", hash.len());
        Ok(vec![hash.to_vec()])
    } else {
        Err(anyhow::anyhow!(
            "Failed to decode 'filepath' ASP arg as JSON String in hashfile ASP"
        ))
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
