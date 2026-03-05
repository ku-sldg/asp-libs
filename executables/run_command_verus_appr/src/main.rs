#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
use serde_json::Value;


// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    let appraisal_response: std::result::Result<Value, serde_json::Error>= serde_json::from_slice(&evidence_in);

    let appraisal_response_decoded: serde_json::Value = {
        match appraisal_response {
            Ok(v) => {v}
            Err(_) => {Value::Null}

        }
    };

    let appraisal_response_string = appraisal_response_decoded["verification-results"]["errors"].as_number();

    match appraisal_response_string {
        None => {
            Ok(Err(anyhow::anyhow!("Could not parse JSON response from running verus executable")))
        }
        Some(num_errors) => {

            let num_errors_int = num_errors.as_i64().unwrap();
            let app_resp_bool = num_errors_int == 0;

            match app_resp_bool {
                true => Ok(Ok(())),
                false => Ok(Err(anyhow::anyhow!("Appraisal was not successful"))),
            }

        }

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
