#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// run_command_hamr_appr: appraiser for run_command_hamr measurements.
//
// Appraisal logic: parse the JSON evidence produced by run_command_hamr and
// check that exit_code == 0.  On failure, surface the last non-empty line of
// stderr as the diagnostic message so the dashboard can show a meaningful error.
//
// This appraiser does NOT compare against a provisioned golden — the tool's
// own exit code is the ground truth.  A non-zero exit means the HAMR tool
// (tipe, logika, or test) found a problem with the target files.

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};

fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    let evidence = ev.first().context("run_command_hamr_appr: no evidence received")?;

    let result: serde_json::Value = serde_json::from_slice(evidence)
        .context("run_command_hamr_appr: evidence is not valid JSON")?;

    let exit_code = result["exit_code"]
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("run_command_hamr_appr: missing or non-integer exit_code"))?;

    if exit_code == 0 {
        return Ok(Ok(()));
    }

    // Build a diagnostic from the last non-empty line of stderr (or stdout if
    // stderr is empty), so the dashboard shows something actionable.
    let stderr = result["stderr"].as_str().unwrap_or("").trim();
    let stdout = result["stdout"].as_str().unwrap_or("").trim();

    let diag_source = if !stderr.is_empty() { stderr } else { stdout };
    // Trim each candidate line: Java stack traces begin with \t which is an
    // invalid unescaped control character in JSON and breaks downstream parsing.
    let last_line = diag_source
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim())
        .unwrap_or("(no output)");

    Ok(Err(anyhow::anyhow!(
        "Sireum exited {} — {}",
        exit_code,
        last_line
    )))
}

fn main() {
    handle_appraisal_body(body);
}
