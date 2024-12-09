// Common Packages
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvaryAppraisal {
    pub status: String, /*
                        pub created   : String,
                        pub expires   : String,
                        pub endpoints : Vec<String>,
                        pub hostnames : Vec<String>,
                        pub tags      : Vec<String>,
                        pub measured  : i64
                        */
}

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    let appraisal_response: Value = serde_json::from_slice(&evidence_in)?;

    let appraisal_response_string = appraisal_response["status"].as_str();

    let app_resp_bool = appraisal_response_string == Some("SUCCESSFUL");

    let out_contents: String = match app_resp_bool {
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
