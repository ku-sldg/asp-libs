// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting provision ASP execution\n");
    let evidence_in = ev.first().context("No file evidence found")?;
    debug_print!("Retrieved evidence of {} bytes\n", evidence_in.len());

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let filename_value = args
        .get("filepath")
        .context("'filepath' argument not provided to ASP, provision")?;

    if filename_value.is_string() {
        let filename: String = filename_value.to_string();
        debug_print!("Writing evidence to file: {}\n", filename);
        std::fs::write(&filename, evidence_in)?;
        debug_print!("Successfully wrote evidence to file: {}\n", filename);
        Ok(vec![])
    } else {
        Err(anyhow::anyhow!(
            "Failed to decode 'filepath' ASP arg as JSON String in provision ASP"
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
