// TEMPLATE.txt
// General structure for ASP's written in rust

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Result, Context};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};

use std::process::{Command};

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandVerus {
    exe_args: Vec<String>
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.

    let myaspargs : ASP_ARGS_RunCommandVerus = serde_json::from_value(args)
    .context("Could not decode JSON ASP_ARGS for ASP run_command_verus")?;

    let command_string = "verus".to_string();
    let my_exe_args= myaspargs.exe_args;

    let error_string = format! {"Error executing {command_string} command on PATH"};

    let output = Command::new(command_string)
                                .args(my_exe_args).output().expect(error_string.as_str());

    let err_res = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    if ! err_res.is_empty() {eprint!("FYI:  stderr output after invoking run_command_verus ASP via the CVM: {:?}", String::from_utf8(err_res))}

    let res = out_res; 

    Ok (vec![res])

}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
