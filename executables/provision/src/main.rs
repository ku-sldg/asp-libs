

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Provision {
    env_var: String,
    filepath: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {

    let myaspargs : ASP_ARGS_Provision = serde_json::from_value(args)
    .context("Could not parse ASP_ARGS for ASP r_provision_id")?;

    // Code for specific for this ASP.
    let env_var: String = myaspargs.env_var;
    let filename: String = myaspargs.filepath;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let filename_full = format! {"{env_var_string}{filename}"};

    let evidence_in = ev.first().context("No input evidence provided to ASP: provision")?;

    eprintln!("Attempting to write to filename: {filename_full}");
    std::fs::write(filename_full, evidence_in)?;
    //eprintln!("WROTE TO filename!!!!: {filename_full}");
    Ok(vec![])

}

fn main() {
    handle_body(body);
}
