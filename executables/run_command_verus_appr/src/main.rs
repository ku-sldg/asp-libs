#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
use serde_json::Value;


// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
// COLD cargo-verus builds pollute stdout with build progress lines ahead
// of the verification-results JSON (warm builds emit pure JSON). The
// appraisal must not depend on build temperature: parse the whole
// evidence first, and on failure retry from each line that opens a JSON
// object until one parses.
fn extract_json(bytes: &[u8]) -> Value {
    if let Ok(v) = serde_json::from_slice::<Value>(bytes) {
        return v;
    }
    let text = String::from_utf8_lossy(bytes);
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        if line.trim_start().starts_with('{') {
            if let Ok(v) = serde_json::from_str::<Value>(&text[offset..]) {
                return v;
            }
        }
        offset += line.len();
    }
    Value::Null
}

fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = ev.first().context("No file evidence found")?;

    let appraisal_response_decoded: serde_json::Value = extract_json(&evidence_in);

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
