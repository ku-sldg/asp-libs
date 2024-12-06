// Common Packages
use rust_am_lib::copland::*;
use anyhow::{Context, Result};
use std::env;
use hex;

use serde::{Deserialize, Serialize};
use serde_json::{Value};
use base64::prelude::*;



#[derive(Serialize, Deserialize, Debug)]
pub struct InvaryAppraisal {
        pub status        : String
        /*
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
fn body() -> Result<String> {

    // For every ASP, an ASPRunRequest appears as the single command-line argument
    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow::anyhow!("ASPRunRequest not supplied as command line argument"));
    }

    let json_request = &args[1];
    // May fail.  If so, return an Err Result
    let req: ASPRunRequest = serde_json::from_str(json_request)?;

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let args_map = req.ASP_ARGS;


    /*
    let golden_filename = args_map.get("filepath-golden").context("filepath-golden argument not provided to ASP, appraise_r_readfile_id")?;


    let env_var_key = "DEMO_ROOT";
    let env_var_string = 
        match env::var(env_var_key) {
            Ok(val) => {val}
            Err(_e) => {panic!("Did not set environment variable DEMO_ROOT")}

        };

    let filename_string = (*golden_filename).clone();
    let filename_full = format!{"{env_var_string}{filename_string}"};


    let golden_bytes : Vec<u8> = std::fs::read(&filename_full)?; // Vec<u8>

    let golden_bytes_string : &String = &String::from_utf8(golden_bytes)?;

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission
    //let golden_bytes_b64: String = base64::encode(bytes);

    */

    // Suppose the file contents are to be extracted from evidence...


    let evidence_in = match req.RAWEV {RawEv::RawEv(x) => x,};

    let latest_evidence : &String = &evidence_in[0];


    /*

    let bytes_decoded  = BASE64_STANDARD.decode(latest_evidence)?;

    let bytes_decoded_string = String::from_utf8(bytes_decoded)?;

    let appraisal_response: Value  = serde_json::from_str(&bytes_decoded_string)?;

    let appraisal_response_string = appraisal_response["status"].as_str();

    let good_result =  Some ("SUCCESSFUL");

    */

    let app_resp_bool = true;  // appraisal_response_string == good_result;



    // Evidence is always base64 encoded, so decode this
    // Using HEX decoding for now...will switch to b64
    //let file_bytes = hex::decode(latest_evidence)?; //base64::decode(latest_evidence)?;
    //let bytes_equal : bool = golden_bytes_string.eq(latest_evidence);
    /*file_bytes*/


    // End of code specific for this ASP.

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission

    let out_contents: String =
        match app_resp_bool {
            true => {"PASSED".to_string()}
            false => {"FAILED".to_string()}
        };


    // Using HEX encoding for now...will switch to b64
    let out_contents_b64 = hex::encode(out_contents); //base64::encode(out_contents);




    // Step 2:
    // wrap the value as Evidence
    let evidence = RawEv::RawEv(vec![out_contents_b64]);

    // Step 3:
    // Construct the ASPRunResponse with this evidence.
    let response = successfulASPRunResponse (evidence);
    let response_json = serde_json::to_string(&response)?;
    Ok (response_json)
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {

    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let  response = failureASPRunResponse (_error.to_string());
            // If an error occurs converting the failure response to JSON
            // there is nothing else to do but panic.
            // This should never happen.
            serde_json::to_string(&response).unwrap_or_else(|error| {panic!("Failed to json.encode failure response: {error:?}");})
        }
    };
    // The ASP output (ASPRunRequest) is written to stdout.
    // The caller will capture stdout to receive the response from this ASP.
    println!("{response_json}");
}