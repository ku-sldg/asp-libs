// Common Packages
use anyhow::{Context, Result};
use lib::copland::{self, handle_appraisal_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<Result<()>> {
    let golden_filename = args
        .get("filepath-golden")
        .context("filepath-golden argument not provided to ASP, appraise_r_hashfile_id")?;

    eprint!("Attempting to read from file: {}\n", golden_filename);

    let golden_bytes = std::fs::read(golden_filename)?; // Vec<u8>

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission
    //let golden_bytes_b64: String = base64::encode(bytes);

    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    // Evidence is always base64 encoded, so decode this
    // Using HEX decoding for now...will switch to b64
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
