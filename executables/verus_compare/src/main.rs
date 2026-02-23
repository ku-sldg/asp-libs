#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

// This ASP ("verus_compare") is a measurement ASP that extracts specification and implementation code
// from two Verus files.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing the following arguments:
// - "original": A string path to the original Verus file.
// - "modified": A string path to the modified Verus file.
//
// OUTPUT:
// The ASP returns a raw evidence package (`RawEv`) containing a vector of four byte arrays (Vec<Vec<u8>>),
// structured as follows:
// 1. Original Spec: The extracted specification code from the "original" file.
// 2. Modified Spec: The extracted specification code from the "modified" file.
// 3. Original Impl: The extracted implementation code from the "original" file.
// 4. Modified Impl: The extracted implementation code from the "modified" file.

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_VerusCompare {
    original: String,
    modified: String,
}

use lynette::{extract_implementation, extract_spec_signatures};
use std::path::PathBuf;



/// Strips standalone `#[verifier::...]` and `#[verifier(...)]` attribute lines
/// from a Verus source string.
///
/// These attributes (e.g. `#[verifier::loop_isolation(false)]`) are
/// file-level or item-level annotations that do not affect the logical
/// specification or implementation being compared.  Leaving them in causes
/// false mismatches when one file has an attribute the other does not.
///
/// Only removes attributes that appear on their own line (possibly with
/// surrounding whitespace).  Inline attributes attached to an item on the
/// same line are left untouched.  Consecutive blank lines left behind by a
/// removal are collapsed to a single blank line so that whitespace differences
/// don't produce false mismatches.
fn strip_verifier_attributes(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut blank_run = 0usize;

    for line in source.lines() {
        let trimmed = line.trim();

        // Match #[verifier::...] and #[verifier(...)] attribute-only lines.
        // `extract_spec_signatures` (via lynette) may insert spaces around
        // punctuation when reconstructing the token stream, producing forms
        // like `# [verifier :: loop_isolation (false)]`.  We therefore
        // compare against the whitespace-collapsed version of the line.
        let compact: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
        let is_verifier_attr = (compact.starts_with("#[verifier::") || compact.starts_with("#[verifier("))
            && compact.ends_with(']');
        if is_verifier_attr {
            continue;
        }

        if trimmed.is_empty() {
            blank_run += 1;
            // Allow at most one consecutive blank line.
            if blank_run <= 1 {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            blank_run = 0;
            result.push_str(line);
            result.push('\n');
        }
    }

    // Preserve original trailing-newline behaviour.
    if !source.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }
    result
}

/// Strips proof functions from a Verus source string and returns the result.
///
/// Removes any function introduced by `proof fn` (optionally preceded by
/// visibility qualifiers such as `pub`, `pub(crate)`, `pub(super)`) or by the
/// `#[verifier::proof]` / `#[verifier(proof)]` attribute.  The entire
/// function — signature and body — is removed regardless of where on a line
/// the keyword appears, so constructs like:
///
/// ```text
///     }       pub proof fn lemma_min(...) { }  }
/// ```
///
/// are handled correctly: only the proof function is excised; surrounding
/// tokens (`}` from a prior block, `}` closing an enclosing module, etc.) are
/// preserved.
///
/// The scanner is character-level and skips over string literals, char
/// literals, and `//` / `/* */` comments so that braces inside those contexts
/// do not affect brace-depth tracking.
///
/// # Arguments
/// * `source` - A string slice containing the raw Verus / Rust source code.
///
/// # Returns
/// A new `String` with all proof functions removed.
fn strip_proof_functions(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(source.len());
    let mut i = 0;

    while i < len {
        // --- Skip comments and string/char literals, copying them verbatim ---

        // Line comment
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // Block comment
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            result.push(chars[i]);
            result.push(chars[i + 1]);
            i += 2;
            while i < len {
                if i + 1 < len && chars[i] == '*' && chars[i + 1] == '/' {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                    i += 2;
                    break;
                }
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // String literal
        if chars[i] == '"' {
            result.push(chars[i]);
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                    i += 2;
                } else if chars[i] == '"' {
                    result.push(chars[i]);
                    i += 1;
                    break;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            }
            continue;
        }

        // Char literal
        if chars[i] == '\'' {
            result.push(chars[i]);
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                    i += 2;
                } else if chars[i] == '\'' {
                    result.push(chars[i]);
                    i += 1;
                    break;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            }
            continue;
        }

        // --- Detect `#[verifier::proof]` / `#[verifier(proof)]` attribute ---
        if chars[i] == '#' {
            let attr_start = i;
            // Peek ahead for `[verifier::proof]` or `[verifier(proof)]`
            let rest: String = chars[i..].iter().collect();
            if rest.starts_with("#[verifier::proof]") || rest.starts_with("#[verifier(proof)]") {
                // Consume the attribute, then whitespace/newlines, then the fn.
                i += if rest.starts_with("#[verifier::proof]") { 18 } else { 17 };
                // Skip whitespace and any further attributes/doc-comments until `fn`.
                i = skip_to_proof_fn_keyword(&chars, i);
                if i < len {
                    i = consume_proof_fn(&chars, i);
                }
                continue;
            }
            // Not a proof attribute — emit the `#` and continue.
            result.push(chars[attr_start]);
            i = attr_start + 1;
            continue;
        }

        // --- Detect `proof fn` keyword (possibly preceded by `pub`, etc.) ---
        // We only attempt this at a word boundary (start of source, or after
        // whitespace / `{` / `}` / `;`).
        if is_word_boundary(&chars, i) {
            if let Some(fn_start) = match_proof_fn_keyword(&chars, i) {
                // fn_start points to the `f` of `fn` — back up to include any
                // visibility qualifier that was already emitted to `result`.
                // Strategy: trim trailing "pub", "pub(crate)", "pub(super)" from
                // `result`, then consume from the visibility qualifier onward.
                trim_trailing_visibility(&mut result);
                i = consume_proof_fn(&chars, fn_start);
                continue;
            }
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// After copying a `pub`/`pub(crate)`/`pub(super)` qualifier to `result` we
/// discover it belongs to a proof fn.  Remove it from the output.
fn trim_trailing_visibility(result: &mut String) {
    let trimmed = result.trim_end();
    // Check for visibility suffixes.
    for suffix in &["pub(crate)", "pub(super)", "pub"] {
        if trimmed.ends_with(suffix) {
            let new_len = trimmed.len() - suffix.len();
            // Also strip any whitespace before the qualifier.
            let new_len = trimmed[..new_len].trim_end().len();
            // Keep at least one newline so surrounding code isn't concatenated.
            result.truncate(new_len);
            result.push('\n');
            return;
        }
    }
    // No visibility qualifier found — just ensure a newline separator.
    if !result.ends_with('\n') {
        result.push('\n');
    }
}

/// Returns `true` if position `i` in `chars` is a word boundary, i.e. the
/// character before it (if any) is whitespace, `{`, `}`, or `;`.
fn is_word_boundary(chars: &[char], i: usize) -> bool {
    if i == 0 {
        return true;
    }
    matches!(chars[i - 1], ' ' | '\t' | '\n' | '\r' | '{' | '}' | ';')
}

/// Starting at `i`, attempt to match an optional visibility qualifier followed
/// by `proof fn `.  Returns `Some(j)` where `j` is the position of the `f` in
/// `fn` if matched, or `None` otherwise.
fn match_proof_fn_keyword(chars: &[char], i: usize) -> Option<usize> {
    let mut j = i;

    // Optional visibility qualifier.
    j = skip_visibility(chars, j);

    // Must see `proof` followed by whitespace then `fn`.
    let rest: String = chars[j..].iter().collect();
    if !rest.starts_with("proof") {
        return None;
    }
    j += 5; // len("proof")
    if j >= chars.len() || !chars[j].is_whitespace() {
        return None;
    }
    while j < chars.len() && chars[j].is_whitespace() {
        j += 1;
    }
    let rest2: String = chars[j..].iter().collect();
    if rest2.starts_with("fn") && (j + 2 >= chars.len() || !chars[j + 2].is_alphanumeric() && chars[j + 2] != '_') {
        Some(j)
    } else {
        None
    }
}

/// Skip an optional `pub`, `pub(crate)`, or `pub(super)` qualifier plus
/// following whitespace.  Returns the updated position.
fn skip_visibility(chars: &[char], i: usize) -> usize {
    let rest: String = chars[i..].iter().collect();
    let mut j = i;
    for prefix in &["pub(crate)", "pub(super)", "pub"] {
        if rest.starts_with(prefix) {
            j += prefix.len();
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            return j;
        }
    }
    j
}

/// Advance `i` past whitespace and any intervening attributes/doc-comments
/// until we're positioned at a `proof fn` or plain `fn` keyword.
/// Used after consuming a `#[verifier::proof]` attribute.
fn skip_to_proof_fn_keyword(chars: &[char], mut i: usize) -> usize {
    let len = chars.len();
    while i < len {
        // Skip whitespace.
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        // Skip further attributes.
        if chars[i] == '#' {
            while i < len && chars[i] != ']' {
                i += 1;
            }
            i += 1; // consume ']'
            continue;
        }
        // Skip doc comments.
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        // We've reached non-whitespace, non-attribute content — stop here so
        // consume_proof_fn can take over.
        break;
    }
    i
}

/// Consume a proof function starting at `i` (which points at `fn` or at a
/// visibility qualifier / `proof` keyword).  Advances past the entire
/// signature and body (balanced braces).  Returns the position after the
/// closing `}`.
fn consume_proof_fn(chars: &[char], mut i: usize) -> usize {
    let len = chars.len();
    let mut brace_depth: i32 = 0;
    let mut found_open = false;

    while i < len {
        // Skip line comments.
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        // Skip block comments.
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            i += 2;
            while i < len {
                if i + 1 < len && chars[i] == '*' && chars[i + 1] == '/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }
        // Skip string literals.
        if chars[i] == '"' {
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len { i += 2; }
                else if chars[i] == '"' { i += 1; break; }
                else { i += 1; }
            }
            continue;
        }
        // Skip char literals.
        if chars[i] == '\'' {
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len { i += 2; }
                else if chars[i] == '\'' { i += 1; break; }
                else { i += 1; }
            }
            continue;
        }

        match chars[i] {
            '{' => { brace_depth += 1; found_open = true; i += 1; }
            '}' => {
                brace_depth -= 1;
                i += 1;
                if found_open && brace_depth <= 0 {
                    break;
                }
            }
            _ => { i += 1; }
        }
    }
    i
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting verus_compare ASP execution\n");

    let myaspargs: ASP_ARGS_VerusCompare =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP verus_compare")?;

    let original_path = PathBuf::from(myaspargs.original);
    let modified_path = PathBuf::from(myaspargs.modified);
    // check that the files exist
    if !original_path.exists() {
        return Err(anyhow::anyhow!(
            "Original file does not exist: {}",
            original_path.display()
        ));
    }
    if !modified_path.exists() {
        return Err(anyhow::anyhow!(
            "Modified file does not exist: {}",
            modified_path.display()
        ));
    }

    debug_print!(
        "Original file: {}\\nModified file: {}\\n",
        original_path.display(),
        modified_path.display()
    );

    let original_spec = extract_spec_signatures(&original_path)?;
    let modified_spec = extract_spec_signatures(&modified_path)?;
    let original_impl = extract_implementation(&original_path)?;
    let modified_impl = extract_implementation(&modified_path)?;

    let original_spec_minus_proof = strip_proof_functions(&strip_verifier_attributes(&original_spec));
    let modified_spec_minus_proof = strip_proof_functions(&strip_verifier_attributes(&modified_spec));

    let original_impl_minus_proof = strip_proof_functions(&strip_verifier_attributes(&original_impl));
    let modified_impl_minus_proof = strip_proof_functions(&strip_verifier_attributes(&modified_impl));


    debug_print!("Extraction complete\n");
    debug_print!("Original Spec:\\n{}\\n", original_spec);
    debug_print!("Modified Spec:\\n{}\\n", modified_spec);
    debug_print!("Original Impl:\\n{}\\n", original_impl);
    debug_print!("Modified Impl:\\n{}\\n", modified_impl);

    Ok(vec![
        original_spec_minus_proof.into_bytes(),
        modified_spec_minus_proof.into_bytes(),
        original_impl_minus_proof.into_bytes(),
        modified_impl_minus_proof.into_bytes(),
    ])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    // debug print the current working directory
    if let Ok(_cwd) = std::env::current_dir() {
        debug_print!("Current working directory: {}\n", _cwd.display());
    } else {
        debug_print!("Could not get current working directory\n");
    }
    // debug print the program arguments on newlines
    for _arg in std::env::args() {
        debug_print!("arg: {}\n", _arg);
    }
    handle_body(body);
}