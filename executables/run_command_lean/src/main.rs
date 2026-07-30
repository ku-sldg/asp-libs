// TEMPLATE.txt
// General structure for ASP's written in rust

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Result, Context};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};

use std::{process::Command};

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandLean {
    exe_args: Vec<String>,
    #[serde(default)]
    cwd: Option<String>
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Invokes the Lean 4 build tool `lake` (e.g. `lake env lean --json <file>`
    // for proof checking, or `lake exe <target> -- <args>` to run a built
    // executable). `cwd` selects the Lake package directory — lake must run
    // inside the package, and elan resolves the toolchain from its
    // lean-toolchain file there.

    let myaspargs : ASP_ARGS_RunCommandLean = serde_json::from_value(args)
    .context("Could not decode JSON ASP_ARGS for ASP run_command_lean")?;

    let command_string = "lake".to_string();
    let my_exe_args= myaspargs.exe_args;

    let error_string = format! {"Error executing {command_string} command on PATH"};

    let mut command = Command::new(command_string);
    command.args(my_exe_args);
    if let Some(dir) = myaspargs.cwd {
        command.current_dir(dir);
    }
    let output = command.output().expect(error_string.as_str());

    let err_res = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    if ! err_res.is_empty() {eprint!("FYI:  stderr output after invoking run_command_lean ASP via the CVM: {:?}", String::from_utf8(err_res))}

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
