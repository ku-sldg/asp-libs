// Common Packages
use anyhow::{Context, Result};
use lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    let filename = args
        .get("filepath")
        .context("filepath argument not provided to ASP, r_readfile_id")?;

    eprint!("Attempting to read from file: {}\n", filename);

    let bytes = std::fs::read(&filename).context(
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
