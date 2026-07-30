#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use base64::Engine;
use rust_am_lib::copland::{self, handle_appraisal_body};
use serde_json::Value;
use sha2::{Digest, Sha256};

// Appraises FULL-FILE evidence produced by the readfile ASP against golden
// values that were derived from (or must be derivable from) that content.
// The intended use is the signed golden-spec baseline: an administrator
// signs the WHOLE model file at provisioning (the file is blessed by
// policy as a unit), and this appraiser checks that the measurement
// protocol's per-slice goldens are extractable from the blessed content —
// slice definitions are appraiser INPUTS checked against signed bytes,
// never independently trusted labels.
//
// ASP_ARGS (all checks optional, at least one required):
//
//   "golden_b64"       — base64 of the expected full file bytes
//   "hash_golden_b64"  — base64 of the expected SHA-256 digest of the
//                        file bytes (the hashfile ASP's golden format)
//   "slices"           — {targ_id: {"start_index": n, "end_index": n,
//                        "golden_b64": s}}: each slice is extracted from
//                        the evidence with readfile_range's exact
//                        semantics (1-based inclusive lines, line
//                        terminators stripped, concatenated with no
//                        separator) and compared to its golden bytes
//
// Failure reasons name the failing check/targets, sanitized single-line
// (the CVM splices reasons into response JSON unescaped).

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

// BufReader::lines semantics: split on '\n', strip a trailing '\r'.
fn file_lines(bytes: &[u8]) -> Vec<&[u8]> {
    let mut lines: Vec<&[u8]> = bytes.split(|b| *b == b'\n').collect();
    if let Some(last) = lines.last() {
        if last.is_empty() {
            lines.pop(); // no line after a trailing newline
        }
    }
    lines.into_iter()
        .map(|l| if l.ends_with(b"\r") { &l[..l.len() - 1] } else { l })
        .collect()
}

fn extract_slice(lines: &[&[u8]], start_index: usize, end_index: usize) -> Vec<u8> {
    let start = start_index.saturating_sub(1);
    lines.iter()
        .skip(start)
        .take(end_index.saturating_sub(start))
        .flat_map(|l| l.iter().copied())
        .collect()
}

fn b64(s: &str) -> Result<Vec<u8>> {
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .context("invalid base64 golden value")
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    let evidence = ev.first().context("No file evidence found")?;
    let lines = file_lines(evidence);

    let mut checks = 0;
    let mut problems: Vec<String> = Vec::new();

    if let Some(golden) = args.get("golden_b64").and_then(|v| v.as_str()) {
        checks += 1;
        if b64(golden)? != *evidence {
            problems.push("blessed file content mismatch".to_string());
        }
    }

    if let Some(golden) = args.get("hash_golden_b64").and_then(|v| v.as_str()) {
        checks += 1;
        if b64(golden)? != Sha256::digest(evidence).to_vec() {
            problems.push("file hash golden not derivable from blessed content".to_string());
        }
    }

    if let Some(slices) = args.get("slices").and_then(|v| v.as_object()) {
        for (targ_id, spec) in slices {
            checks += 1;
            let start = spec.get("start_index").and_then(Value::as_u64)
                .context("slice missing start_index")? as usize;
            let end = spec.get("end_index").and_then(Value::as_u64)
                .context("slice missing end_index")? as usize;
            let golden = spec.get("golden_b64").and_then(Value::as_str)
                .context("slice missing golden_b64")?;
            if b64(golden)? != extract_slice(&lines, start, end) {
                problems.push(format!(
                    "{targ_id}: slice golden not extractable from blessed content"));
            }
        }
    }

    if checks == 0 {
        return Ok(Err(anyhow::anyhow!(
            "Nothing to appraise: no golden_b64, hash_golden_b64, or slices in args")));
    }
    if problems.is_empty() {
        Ok(Ok(()))
    } else {
        Ok(Err(anyhow::anyhow!("Blessed-content appraisal failed: {}",
                               sanitize(&problems.join("; ")))))
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
