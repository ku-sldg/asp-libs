#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
//use std::fs;
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
//use serde::{Deserialize, Serialize};
use serde_json::Value;
//use serde::{Deserialize, Serialize};

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Run_Command_Verus_Appr {
    env_var_golden: String,
    filepath_golden: String,
    asp_id_appr: String, 
    targ_id_appr: String
}
*/

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    // Suppose the file contents are to be extracted from evidence...

    /*
    let myaspargs: ASP_ARGS_Run_Command_Verus_Appr = serde_json::from_value(args)
    .context("Could not parse ASP_ARGS for ASP run_command_verus_appr")?;
    */

    //let targid = myaspargs.targ_id_appr.clone();


    let evidence_in = ev.first().context("No file evidence found")?;

    let appraisal_response: Value = serde_json::from_slice(&evidence_in)?;

    let appraisal_response_string = appraisal_response["verification-results"]["errors"].as_number();

    let num_errors = appraisal_response_string.unwrap();

    let num_errors_int = num_errors.as_i64().unwrap();

    let app_resp_bool = num_errors_int == 0; //appraisal_response_string == Some("SUCCESSFUL");

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
