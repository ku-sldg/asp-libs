#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
use serde_json::Value;

// Appraises evidence produced by run_command_lean, in one of two modes
// selected by ASP_ARGS:
//
//   default          — diagnostics mode, for `lake env lean --json <file>`:
//                      each stdout line is a JSON diagnostic object. PASS iff
//                      no diagnostic has severity "error" and none has kind
//                      "hasSorry" (a `sorry` only WARNS and exits 0, so exit
//                      codes and error severities alone would bless it).
//                      Non-JSON lines are skipped (build chatter is not
//                      evidence of failure; lean puts diagnostics on stdout).
//
//   {"expected": s}  — executable mode, for `lake exe <target> -- <args>`:
//                      PASS iff trimmed stdout equals the trimmed expected
//                      string. The measured behavior of the built binary.

// Failure reasons travel inside the CVM's response JSON, which does not
// escape them — a reason containing double quotes, backslashes, or control
// characters (Lean goal states are multi-line) breaks the response. Keep
// reasons single-line, quote-free, and bounded.
fn sanitize(s: &str) -> String {
    let mut out: String = s.chars().take(400).map(|c| match c {
        '"' => '\'',
        '\\' => '/',
        c if c.is_control() => ' ',
        c => c,
    }).collect();
    if s.chars().count() > 400 {
        out.push_str(" ...");
    }
    out
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {

    let evidence_in = ev.first().context("No command-output evidence found")?;
    let text = String::from_utf8_lossy(evidence_in);

    if let Some(expected) = args.get("expected").and_then(|v| v.as_str()) {
        return if text.trim() == expected.trim() {
            Ok(Ok(()))
        } else {
            Ok(Err(anyhow::anyhow!(
                "Executable output mismatch: expected '{}', got '{}'",
                sanitize(expected.trim()), sanitize(text.trim()))))
        };
    }

    let mut problems: Vec<String> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let diag: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let severity = diag["severity"].as_str().unwrap_or("");
        let kind = diag["kind"].as_str().unwrap_or("");
        if severity == "error" || kind == "hasSorry" {
            problems.push(sanitize(&format!(
                "{}:{} {}{}: {}",
                diag["fileName"].as_str().unwrap_or("?"),
                diag["pos"]["line"],
                severity,
                if kind.is_empty() { String::new() } else { format!(" [{kind}]") },
                diag["data"].as_str().unwrap_or(""))));
        }
    }

    if problems.is_empty() {
        Ok(Ok(()))
    } else {
        Ok(Err(anyhow::anyhow!("Lean check failed: {}", problems.join("; "))))
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
