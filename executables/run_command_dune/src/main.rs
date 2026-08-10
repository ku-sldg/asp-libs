#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};

use std::process::Command;

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandDune {
    exe_args: Vec<String>,
    #[serde(default)]
    cwd: Option<String>,
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Invokes dune (e.g. `dune build` to elaborate a Rocq theory — the
    // lake analogue for the Rocq examples). `cwd` selects the dune
    // project directory. dune is SILENT on success (both streams, cold
    // and warm) and reports failures on stderr with a nonzero exit.
    //
    // Evidence is a single JSON object {status, stdout, stderr},
    // appraised by run_command_rocq_appr (build mode).

    let myaspargs: ASP_ARGS_RunCommandDune = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP run_command_dune")?;

    let mut command = Command::new("dune");
    command.args(myaspargs.exe_args);
    if let Some(dir) = myaspargs.cwd {
        command.current_dir(dir);
    }
    let output = command
        .output()
        .context("Error executing dune command on PATH")?;

    let evidence = serde_json::json!({
        "status": output.status.code(),
        "stdout": String::from_utf8_lossy(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr),
    });

    Ok(vec![evidence.to_string().into_bytes()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
