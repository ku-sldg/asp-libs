#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use std::fs;
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_appraisal_body, RawEv},
    debug_print
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
use serde_stacker::Deserializer;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_GoldenEvidence_Appr {
    env_var_golden: String,
    filepath_golden: String,
    asp_id_appr: String, 
    targ_id_appr: String
}

fn deserialize_deep_json(json_data: &str) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_str(json_data);
    de.disable_recursion_limit(); // This method is only available with the feature
    
    // Wrap with serde_stacker's Deserializer to use a dynamically growing stack
    let stacker_de = Deserializer::new(&mut de);
    
    // Deserialize the data
    let value = Value::deserialize(stacker_de)?;
    
    Ok(value)
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {

    let myaspargs: ASP_ARGS_GoldenEvidence_Appr = serde_json::from_value(args)
        .context("Could not parse ASP_ARGS for ASP goldenevidence_appr")?;

    // Code for specific for this ASP.
    let env_var: String = myaspargs.env_var_golden;
    let filename: String = myaspargs.filepath_golden;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let filename_full = format! {"{env_var_string}{filename}"};

    let contents = fs::read_to_string(filename_full).expect("Couldn't read (Evidence, GlobalContext) JSON file in goldenevidence_appr");
    debug_print!{"\n\nAttempting to decode (Evidence, GlobalContext)...\n\n"};
    let my_contents_val = deserialize_deep_json(&contents)?;
    let my_contents: (copland::Evidence, copland::GlobalContext) = from_value(my_contents_val)?;
    debug_print!("\nDecoded (Evidence, GlobalContext) as:");
    debug_print!("{:?}", my_contents);

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
        true => {
            Ok(Ok(()))
        },
        false => Ok(Err(anyhow::anyhow!("Evidence bytes contents do not match"))),
    }

}

fn main() {
    handle_appraisal_body(body);
}
