#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body, vec_to_rawev, EvidenceT, GlobalContext},
    debug_print,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Provision_GoldenEvidence {
    env_var_golden: String,
    filepath_golden: String,
    et_context: GlobalContext,
    et_golden: EvidenceT
}

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

    let vecvec = ev.clone(); //vec!((*evidence_in).clone());

    let my_rawev = vec_to_rawev(vecvec);

    let my_evidence: copland::Evidence = (my_rawev, myaspargs.et_golden);

    let my_ctxt: copland::GlobalContext = myaspargs.et_context;

    let my_evidence_w_context: (copland::Evidence, copland::GlobalContext) = (my_evidence, my_ctxt);

    let my_json_string = serde_json::to_string(&my_evidence_w_context)?;

    debug_print!("Attempting to write golden evidence to filename: {filename_full}");
    std::fs::write(filename_full, my_json_string)?;
    Ok(vec![])
}

fn main() {
    handle_body(body);
}
