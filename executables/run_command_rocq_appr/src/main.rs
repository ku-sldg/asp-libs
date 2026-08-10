#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body};
use serde_json::Value;

// Appraises evidence produced by run_command_rocq / run_command_dune —
// a single JSON object {status, stdout, stderr} — in one of two modes
// selected by ASP_ARGS:
//
//   default          — build mode, for `dune build` (or a plain
//                      `rocq compile`): PASS iff exit status is 0. Both
//                      tools are silent on success and report failures
//                      on stderr; the reason carries the Error lines.
//                      NOTE an exit status of 0 proves elaboration only:
//                      `Admitted.` and `Axiom` declarations compile
//                      cleanly — provability is judged in assumptions
//                      mode, not here.
//
//   {"goals": [...]}  — assumptions mode, for a Print Assumptions audit
//                      file (`rocq compile Assumptions.v` with one
//                      `Print Assumptions <goal>.` per listed goal, in
//                      order): PASS iff status is 0 AND stdout parses
//                      into exactly one section per goal AND every
//                      section is "Closed under the global context".
//                      An "Axioms:" section means the goal depends on
//                      an Admitted proof or a smuggled Axiom — the
//                      reason names the goal and its axioms. A section
//                      count mismatch FAILS (fail closed on output the
//                      audit cannot account for).

// Failure reasons travel inside the CVM's response JSON, which does not
// escape them — a reason containing double quotes, backslashes, or control
// characters (axiom types can be multi-line) breaks the response. Keep
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

const CLOSED: &str = "Closed under the global context";
const AXIOMS: &str = "Axioms:";

// stdout of a Print Assumptions audit -> one entry per section, either
// None (closed) or Some(axiom names). Sections arrive in the order the
// Print Assumptions commands appear in the audit file; axiom entries
// are "name : type" lines (types may wrap onto indented continuations,
// which are skipped — the name line is what identifies the dependency).
fn parse_sections(stdout: &str) -> Vec<Option<Vec<String>>> {
    let mut sections: Vec<Option<Vec<String>>> = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim_end();
        if trimmed == CLOSED {
            sections.push(None);
        } else if trimmed == AXIOMS {
            sections.push(Some(Vec::new()));
        } else if let Some(Some(axioms)) = sections.last_mut() {
            if !line.starts_with(char::is_whitespace) {
                if let Some((name, _)) = trimmed.split_once(':') {
                    let name = name.trim();
                    if !name.is_empty() {
                        axioms.push(name.to_string());
                    }
                }
            }
        }
    }
    sections
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    let evidence_in = ev.first().context("No command-output evidence found")?;
    let evidence: Value = serde_json::from_slice(evidence_in)
        .context("run_command_rocq_appr expects JSON {status, stdout, stderr} evidence")?;
    let status = evidence["status"].as_i64();
    let stdout = evidence["stdout"].as_str().unwrap_or("");
    let stderr = evidence["stderr"].as_str().unwrap_or("");

    if status != Some(0) {
        let detail: Vec<&str> = stderr
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();
        return Ok(Err(anyhow::anyhow!(
            "Rocq command failed (status {}): {}",
            status.map_or("none".to_string(), |c| c.to_string()),
            sanitize(&detail.join(" | ")))));
    }

    let goals: Option<Vec<String>> = args.get("goals").and_then(|v| {
        v.as_array().map(|a| {
            a.iter()
                .filter_map(|g| g.as_str().map(str::to_string))
                .collect()
        })
    });
    let Some(goals) = goals else {
        return Ok(Ok(())); // build mode: a clean exit is the judgment
    };

    let sections = parse_sections(stdout);
    if sections.len() != goals.len() {
        return Ok(Err(anyhow::anyhow!(
            "Assumptions audit mismatch: {} goals but {} Print Assumptions sections in output",
            goals.len(), sections.len())));
    }

    let mut problems: Vec<String> = Vec::new();
    for (goal, section) in goals.iter().zip(sections) {
        if let Some(axioms) = section {
            let deps = if axioms.is_empty() {
                "unparsed axioms".to_string()
            } else {
                axioms.join(", ")
            };
            problems.push(format!("{goal} depends on axioms: {deps}"));
        }
    }

    if problems.is_empty() {
        Ok(Ok(()))
    } else {
        Ok(Err(anyhow::anyhow!(
            "Rocq assumptions audit failed: {}",
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
