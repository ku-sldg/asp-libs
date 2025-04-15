

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Provision {
    filepath: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let evidence_in = ev.first().context("No file evidence found")?;


    let myaspargs : ASP_ARGS_Provision = serde_json::from_value(args)
    .context("Could not parse ASP_ARGS for ASP r_provision_id")?;

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let filename: String = myaspargs.filepath;
    
    /*
    args
        .get("filepath-golden")
        .context("filepath-golden argument not provided to ASP, r_readfile_id")?;
    */

    let env_var_key = "AM_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable AM_ROOT")
        }
    };

    let filename_string = (filename).clone();
    let filename_full = format! {"{env_var_string}{filename_string}"};

    //let bytes = std::fs::read(filename).context("could not read file contents in ASP, r_readfile_id.  Perhaps the file doesn't exits?")?; // Vec<u8>

    eprintln!("Attempting to write to filename: {filename_full}");
    std::fs::write(filename_full, evidence_in)?;
    //eprintln!("WROTE TO filename!!!!: {filename_full}");
    Ok(vec![])

}




/*



// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let evidence_in = ev.first().context("No file evidence found")?;

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let filename_value = args
        .get("filepath")
        .context("'filepath' argument not provided to ASP, provision")?;

    if filename_value.is_string()
    {
        let filename: String = filename_value.to_string();
        let filename_full = filename.clone();
        let filename2: String = filename.clone();
        //let errstr = format!("Attempting to write to filename: {}", filename);
        eprintln!("Attempting to write to filename: {filename}");
        std::fs::write(filename_full, evidence_in)?;
        eprintln!("WROTE TO filename!!!!: {filename2}");
        Ok(vec![])
    }
    else {
        Err(anyhow::anyhow!("Failed to decode 'filepath' ASP arg as JSON String in provision ASP"))
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

*/

fn main() {
    handle_body(body);
}
