// TEMPLATE.txt
// General structure for ASP's written in rust

use anyhow::{Context, Result};
use lib::copland::{self, handle_body};

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Digest, Sha256};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let filename = args
        .get("filepath")
        .context("filename argument not provided to ASP, hashfile_id")?;

    eprint!("Attempting to read from file: {}\n", filename);

    let bytes = std::fs::read(filename)?; // Vec<u8>

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
