// Common Packages
use anyhow::{Context, Result};
use lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    let evidence_in = ev.first().context("No file evidence found")?;

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let filename: &String = args
        .get("filepath")
        .context("filepath argument not provided to ASP, r_readfile_id")?;

    std::fs::write(filename, evidence_in)?;
    Ok(vec![])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
