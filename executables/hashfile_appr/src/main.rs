// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_appraisal_body},
    debug_print,
};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    let golden_filename_value = args
        .get("filepath-golden")
        .context("'filepath-golden' argument not provided to ASP, hashfile_appr")?;

    if golden_filename_value.is_string() {
        let golden_filename: String = golden_filename_value.to_string();

        debug_print!("Attempting to read from file: {}\n", golden_filename);
        let golden_bytes = std::fs::read(golden_filename)?;

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
    } else {
        Err(anyhow::anyhow!(
            "Failed to decode 'filepath-golden' ASP arg as JSON String in hashfile_appr ASP"
        ))
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
