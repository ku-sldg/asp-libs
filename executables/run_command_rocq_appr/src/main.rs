#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use std::fs;
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body, RawEv};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandRocq_Appr {
    env_var_golden: String,
    filepath_golden: String,
    asp_id_appr: String, 
    targ_id_appr: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    // Suppose the file contents are to be extracted from evidence...

    let myaspargs: ASP_ARGS_RunCommandRocq_Appr = serde_json::from_value(args)
    .context("Could not parse ASP_ARGS for ASP run_command_rocq_appr")?;

    let env_var: String = myaspargs.env_var_golden;
    let filename: String = myaspargs.filepath_golden;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let filename_full = format! {"{env_var_string}{filename}"};

    let contents = fs::read_to_string(filename_full).expect("Couldn't read (Evidence, GlobalContext) JSON file in run_command_rocq_appr");
    eprintln!{"\n\nAttempting to decode (Evidence, GlobalContext)...\n\n"};
    let my_contents: (copland::Evidence, copland::GlobalContext) = serde_json::from_str(&contents)?;
    eprintln!("\nDecoded (Evidence, GlobalContext) as:");
    eprintln!("{:?}", my_contents);

    let my_evidence: copland::Evidence = my_contents.0;
    let my_glob_ctxt: copland::GlobalContext = my_contents.1;

    let my_asp_params: copland::ASP_PARAMS = copland::ASP_PARAMS{ ASP_ID: myaspargs.asp_id_appr, ASP_ARGS: serde_json::Value::Null, ASP_PLC: "".to_string(), ASP_TARG_ID: myaspargs.targ_id_appr};

    let my_et = copland::get_et(my_evidence.clone());
    let my_rawev= copland::get_rawev(my_evidence);

    let evidence_slice = copland::do_EvidenceSlice(my_et, my_rawev, my_glob_ctxt, my_asp_params)?;

    let evidence_slice_rawev = RawEv::RawEv(evidence_slice);
    let golden_bytes = copland::rawev_to_vec(evidence_slice_rawev);

    let evidence_in = ev;

    let bytes_equal: bool = golden_bytes.eq(&evidence_in);

    match bytes_equal {
        true => Ok(Ok(())),
        false => Ok(Err(anyhow::anyhow!("Evidence bytes contents do not match"))),
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
