// cheat_scan_verus: measure the Verus proof-escape surface of one crate.
//
// Walks <crate_dir>/src/**/*.rs and counts the constructs that make
// verification succeed without proof ("cheats" in the sense of verus
// --no-cheating, plus the axiom/broadcast and bare-external families).
// Evidence is canonical JSON (serde_json maps are BTree-ordered), so an
// unchanged tree always yields byte-identical evidence; appraisal is an
// exact golden comparison via goldenbytes_appr.
//
// Counting is textual (comments and strings included): the appraisal
// semantics is drift-from-baseline, not standalone judgment, so a false
// positive in the golden run cancels against the same false positive at
// attestation time.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_CheatScanVerus {
    // Named crate_dir (not filepath) so provisioning leaves it untouched:
    // the scan measures the LIVE tree at provision and attest time alike,
    // the same measure-in-place semantics as the cargo-verus run itself.
    crate_dir: String,
}

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Count occurrences of `word` in `text` with no word-character on either
/// side. With `call_only`, require the next non-whitespace char after the
/// word to be '(' — this keeps GUMBO's `*_assume(` helper names and
/// `assume_specification` out of the `assume` count.
fn count_word(text: &str, word: &str, call_only: bool) -> u64 {
    let bytes = text.as_bytes();
    let mut count = 0u64;
    let mut start = 0usize;
    while let Some(pos) = text[start..].find(word) {
        let at = start + pos;
        let end = at + word.len();
        let left_ok = at == 0 || !is_word(bytes[at - 1] as char);
        let right_ok = end == bytes.len() || !is_word(bytes[end] as char);
        let call_ok = !call_only
            || text[end..].trim_start().starts_with('(');
        if left_ok && right_ok && call_ok {
            count += 1;
        }
        start = at + word.len().max(1);
    }
    count
}

/// Bare `verifier::external` — the family members (external_body,
/// external_fn_specification, ...) are word-suffixed, so the plain
/// word-boundary count excludes them.
fn count_bare_external(text: &str) -> u64 {
    let mut count = 0u64;
    let mut start = 0usize;
    while let Some(pos) = text[start..].find("verifier::external") {
        let at = start + pos;
        let end = at + "verifier::external".len();
        if end == text.len() || !is_word(text.as_bytes()[end] as char) {
            count += 1;
        }
        start = end;
    }
    count
}

fn count_substring(text: &str, needle: &str) -> u64 {
    let mut count = 0u64;
    let mut start = 0usize;
    while let Some(pos) = text[start..].find(needle) {
        count += 1;
        start += pos + needle.len();
    }
    count
}

fn path_class(rel: &str) -> &'static str {
    if rel.contains("/bridge/") {
        "bridge"
    } else if rel.contains("/component/") {
        "component"
    } else {
        "other"
    }
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("Could not read directory {}", dir.display()))?
        .collect::<std::io::Result<_>>()?;
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            rust_sources(&path, out)?;
        } else if path.extension().map_or(false, |e| e == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

fn scan_text(text: &str) -> BTreeMap<&'static str, u64> {
    let mut counts = BTreeMap::new();
    counts.insert("assume", count_word(text, "assume", true));
    counts.insert("admit", count_word(text, "admit", true));
    counts.insert("external_body", count_substring(text, "external_body"));
    counts.insert("external", count_bare_external(text));
    counts.insert(
        "assume_specification",
        count_substring(text, "assume_specification"),
    );
    counts.insert(
        "external_specification",
        count_substring(text, "external_fn_specification")
            + count_substring(text, "external_type_specification")
            + count_substring(text, "external_trait_specification"),
    );
    counts.insert("axiom", count_word(text, "axiom", false));
    counts.insert("broadcast", count_word(text, "broadcast", false));
    counts
}

fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let myaspargs: ASP_ARGS_CheatScanVerus = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP cheat_scan_verus")?;

    let crate_dir = PathBuf::from(&myaspargs.crate_dir);
    let src = crate_dir.join("src");
    let mut sources = Vec::new();
    rust_sources(&src, &mut sources)?;

    let mut totals: BTreeMap<&'static str, u64> = BTreeMap::new();
    let mut external_body_by_class: BTreeMap<&'static str, u64> = BTreeMap::new();
    for class in ["bridge", "component", "other"] {
        external_body_by_class.insert(class, 0);
    }
    let mut files = Map::new();

    for path in &sources {
        let text = fs::read_to_string(path)
            .with_context(|| format!("Could not read {}", path.display()))?;
        let rel = path
            .strip_prefix(&crate_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let counts = scan_text(&text);
        for (k, v) in &counts {
            *totals.entry(k).or_insert(0) += v;
        }
        *external_body_by_class
            .entry(path_class(&rel))
            .or_insert(0) += counts["external_body"];
        let nonzero: Map<String, Value> = counts
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| (k.to_string(), json!(v)))
            .collect();
        if !nonzero.is_empty() {
            files.insert(rel, Value::Object(nonzero));
        }
    }

    let mut report = Map::new();
    for (k, v) in &totals {
        if *k == "external_body" {
            let by_class: Map<String, Value> = external_body_by_class
                .iter()
                .map(|(c, n)| (c.to_string(), json!(n)))
                .collect();
            report.insert("external_body".to_string(), Value::Object(by_class));
        } else {
            report.insert(k.to_string(), json!(v));
        }
    }
    report.insert("files".to_string(), Value::Object(files));
    report.insert("files_scanned".to_string(), json!(sources.len()));

    let out = serde_json::to_vec(&Value::Object(report))?;
    Ok(vec![out])
}

fn main() {
    handle_body(body);
}
