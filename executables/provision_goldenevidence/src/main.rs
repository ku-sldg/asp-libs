#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
//use std::fs;
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, GlobalContext, EvidenceT, handle_body, vec_to_rawev},
    debug_print,
};
use serde::{Deserialize, Serialize};
//use serde_json::{Value, from_value};
//use serde_stacker::Deserializer;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Provision_GoldenEvidence {
    env_var_golden: String,
    filepath_golden: String,
    et_context: GlobalContext,
    et_golden: EvidenceT
}

/*
fn deserialize_deep_json(json_data: &str) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_str(json_data);
    de.disable_recursion_limit(); // This method is only available with the feature
    
    // Wrap with serde_stacker's Deserializer to use a dynamically growing stack
    let stacker_de = Deserializer::new(&mut de);
    
    // Deserialize the data
    let value = Value::deserialize(stacker_de)?;
    
    Ok(value)
}
*/

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let myaspargs: ASP_ARGS_Provision_GoldenEvidence = serde_json::from_value(args)
        .context("Could not parse ASP_ARGS for ASP provision_goldenevidence")?;

    // Code for specific for this ASP.
    let env_var: String = myaspargs.env_var_golden;
    let filename: String = myaspargs.filepath_golden;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let filename_full = format! {"{env_var_string}{filename}"};

    let vecvec = ev.clone();

    let my_rawev = vec_to_rawev(vecvec);

    /*

    let fp = myaspargs.et_golden;
    debug_print!{"\n\nAttempting to read (EvidenceT, GlobalContext) JSON structure from file: {}\n\n", fp};

    let contents = fs::read_to_string(fp).expect("Couldn't read (EvidenceT, GlobalContext) JSON file in provision_goldenevidence ASP at fp: {}");
    debug_print!{"\n\nAttempting to decode (EvidenceT, GlobalContext)...\n\n"};
    let my_contents_val = deserialize_deep_json(&contents)?;
    let my_contents: (copland::EvidenceT, copland::GlobalContext) = from_value(my_contents_val)?;
    debug_print!("\nDecoded (EvidenceT, GlobalContext) as:");
    debug_print!("{:?}", my_contents);

    */

    let my_evidenceT: copland::EvidenceT = myaspargs.et_golden; //my_contents.0;
    let my_glob_ctxt: copland::GlobalContext = myaspargs.et_context; //my_contents.1;


    let my_evidence: copland::Evidence = (my_rawev,my_evidenceT);

    let my_evidence_w_context: (copland::Evidence, copland::GlobalContext) = (my_evidence, my_glob_ctxt);

    let my_json_string = serde_json::to_string(&my_evidence_w_context)?;

    debug_print!("Attempting to write golden evidence to filename: {filename_full}");
    std::fs::write(filename_full, my_json_string)?;
    Ok(vec![])
}

fn main() {
    handle_body(body);
}
