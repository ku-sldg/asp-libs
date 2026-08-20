// run_command_cargo_verus: cargo-verus verify in a crate dir, evidence =
// the NORMALIZED verification results.
//
// cargo-verus stdout varies run to run (cold builds prepend cargo
// progress lines; --output-json carries timings and toolchain info), so
// raw stdout can only be appraised semantically. This ASP extracts the
// verification-results object from the output and emits canonical JSON
//   {"verification-results": {...}}
// (serde_json maps are BTree-ordered): byte-identical across warm/cold
// runs of an unchanged crate. That makes the evidence goldenbytes-
// appraisable — pinning the verified COUNT, error count, and the
// is-verifying-entire-crate flag — while run_command_verus_appr's
// errors==0 check keeps working on the same shape.
//
// If no verification-results can be extracted (build failure, aborted
// run), the evidence is {"verification-results": null} plus the raw
// output on stderr for diagnostics: never matching any golden, so
// appraisal fails closed.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::process::Command;

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandCargoVerus {
    exe_args: Vec<String>,
    #[serde(default)]
    cwd: Option<String>,
}

// COLD cargo-verus builds pollute stdout with build progress lines ahead
// of the JSON (and a trailing "Finished ..." line can follow it). Retry
// a parse from each line that opens a JSON object; serde stops at the
// end of the first complete value, so trailing text is tolerated.
// Return the LAST verification-results object in the output, not the first.
// `cargo-verus verify` builds the whole dependency graph and every crate
// cargo recompiles prints its own verification-results JSON, in dependency
// order (deps before dependents). The crate being measured (the one in
// `cwd`) is the root, so it is always LAST — a dependency edit inserts the
// dep's results (e.g. verified:0) ahead of it, and picking the first would
// mismeasure the target. The count of recompiled crates depends on cargo's
// build cache, so first-match is nondeterministic; last-match is the root.
fn extract_json(bytes: &[u8]) -> Value {
    let text = String::from_utf8_lossy(bytes);
    let mut offset = 0usize;
    let mut last = Value::Null;
    for line in text.split_inclusive('\n') {
        if line.trim_start().starts_with('{') {
            let mut de = serde_json::Deserializer::from_str(&text[offset..]);
            if let Ok(v) = Value::deserialize(&mut de) {
                if v.get("verification-results").is_some() {
                    last = v;
                }
            }
        }
        offset += line.len();
    }
    last
}

fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let myaspargs: ASP_ARGS_RunCommandCargoVerus = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP run_command_cargo_verus")?;

    let command_string = "cargo-verus".to_string();
    let my_exe_args = myaspargs.exe_args;

    let error_string = format! {"Error executing {command_string} command on PATH"};

    let mut command = Command::new(command_string);
    command.env("RUSTC_BOOTSTRAP", "1").args(my_exe_args);
    if let Some(dir) = myaspargs.cwd {
        command.current_dir(dir);
    }
    let output = command.output().expect(error_string.as_str());

    let err_res = output.stderr;
    let out_res: Vec<u8> = output.stdout;

    if !err_res.is_empty() {
        eprint!(
            "FYI:  stderr output after invoking run_command_cargo_verus ASP via the CVM: {:?}",
            String::from_utf8(err_res)
        )
    }

    let results = extract_json(&out_res)
        .get("verification-results")
        .cloned()
        .unwrap_or(Value::Null);
    if results.is_null() {
        eprint!(
            "FYI:  no verification-results JSON in cargo-verus output: {:?}",
            String::from_utf8_lossy(&out_res)
        )
    }
    let normalized = json!({ "verification-results": results });
    let res = serde_json::to_vec(&normalized)?;

    Ok(vec![res])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
