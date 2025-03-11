// Common Packages
use anyhow::{Context, Result};
use lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    let golden_filename = args
        .get("filepath-golden")
        .context("filepath-golden argument not provided to ASP, appraise_r_readfile_id")?;

    let env_var_key = "AM_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable AM_ROOT")
        }
    };

    let filename_string = (*golden_filename).clone();
    let filename_full = format! {"{env_var_string}{filename_string}"};

    let golden_bytes: Vec<u8> = std::fs::read(&filename_full)?; // Vec<u8>

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission
    //let golden_bytes_b64: String = base64::encode(bytes);

    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    // Evidence is always base64 encoded, so decode this
    // Using HEX decoding for now...will switch to b64
    //let file_bytes = hex::decode(latest_evidence)?; //base64::decode(latest_evidence)?;
    let bytes_equal: bool = golden_bytes.eq(evidence_in);
    /*file_bytes*/

    // End of code specific for this ASP.

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission

    let out_contents: String = match bytes_equal {
        true => "PASSED".to_string(),
        false => "FAILED".to_string(),
    };
    Ok(vec![out_contents.as_bytes().to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
