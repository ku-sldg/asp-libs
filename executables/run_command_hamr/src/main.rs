#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// run_command_hamr: measurement ASP that invokes a Sireum HAMR tool (proyek tipe,
// proyek logika, proyek test, etc.) and returns a structured JSON result.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing:
// - "exe_args":    Arguments passed to sireum (e.g. ["proyek", "tipe", "/path"]).
//
// NOTE: The sireum executable is always resolved by name ("sireum") from the
// system PATH, and SIREUM_HOME is inherited from the CVM process environment.
// Callers may not supply binary paths or environment overrides — doing so
// would allow a Copland protocol to execute arbitrary code on the host.
//
// OUTPUT:
// A raw evidence package containing a single byte array: a JSON object with
// fields "exit_code" (integer), "stdout" (string), and "stderr" (string).
// The appraiser (run_command_hamr_appr) checks exit_code == 0.

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandHamr {
    /// Arguments forwarded to sireum (e.g. ["proyek", "tipe", "/path/to/project"]).
    exe_args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HamrResult {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let myaspargs: ASP_ARGS_RunCommandHamr = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP run_command_hamr")?;

    let output = Command::new("sireum")
        .args(&myaspargs.exe_args)
        .output()
        .context("Failed to execute sireum — ensure it is on PATH and SIREUM_HOME is set")?;

    let result = HamrResult {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    };

    let json_bytes = serde_json::to_vec(&result)
        .context("Failed to serialize run_command_hamr result to JSON")?;

    Ok(vec![json_bytes])
}

fn main() {
    handle_body(body);
}
