// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
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

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    let appraisal_response: Value = serde_json::from_slice(&evidence_in)?;

    let appraisal_response_string = appraisal_response["status"].as_str();

    let app_resp_bool = appraisal_response_string == Some("SUCCESSFUL");

    match app_resp_bool {
        true => Ok(Ok(())),
        false => Ok(Err(anyhow::anyhow!("Appraisal was not successful"))),
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
