#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result, anyhow};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

// This ASP ("readfile_marker_range") is a measurement ASP that extracts the content
// between two named marker strings within a file.
//
// Designed for measuring HAMR-generated BEGIN/END contract blocks in Slang component
// files, where static line numbers are fragile (they shift as implementation code grows
// outside the markers), but the marker strings themselves are stable anchors.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing:
// - "filepath":     Path to the file to be read.
// - "begin_marker": The exact string that marks the start of the range (inclusive).
// - "end_marker":   The exact string that marks the end of the range (inclusive).
//
// Both markers are matched by exact comment-line comparison: the marker must be the sole
// content of the line (after stripping whitespace), optionally wrapped in a // comment.
// That is, a line matches if strip(strip_prefix("//", strip(line))) == marker.
// If begin_marker appears multiple times, the first occurrence is used.
// If end_marker appears multiple times after begin_marker, the first is used.
//
// OUTPUT:
// A raw evidence package containing a single byte array: the flattened contents of all
// lines from begin_marker to end_marker (inclusive), with newlines stripped (same
// output contract as readfile_range).
//
// Returns an error if either marker is not found, or if end_marker precedes begin_marker.

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileMarkerRange {
    filepath: String,
    begin_marker: String,
    end_marker: String,
}

/// Returns true when `line` is a comment whose sole content equals `marker`.
/// Strips leading/trailing whitespace, then strips an optional "//" prefix and
/// its following whitespace before comparing.  This prevents developer comments
/// that merely *contain* a marker string from triggering a false boundary match.
fn marker_matches(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    let content = if let Some(rest) = trimmed.strip_prefix("//") {
        rest.trim()
    } else {
        trimmed
    };
    content == marker
}

fn extract_marker_range(
    path: &PathBuf,
    begin_marker: &str,
    end_marker: &str,
) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut in_range = false;
    let mut result: Vec<u8> = Vec::new();
    let mut found_begin = false;
    let mut found_end = false;

    for line_result in reader.lines() {
        let line = line_result?;

        if !in_range && marker_matches(&line, begin_marker) {
            in_range = true;
            found_begin = true;
            result.extend_from_slice(line.as_bytes());
            continue;
        }

        if in_range {
            result.extend_from_slice(line.as_bytes());
            if marker_matches(&line, end_marker) {
                found_end = true;
                break;
            }
        }
    }

    if !found_begin {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("begin_marker not found: {:?}", begin_marker),
        ));
    }
    if !found_end {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("end_marker not found after begin_marker: {:?}", end_marker),
        ));
    }

    Ok(result)
}

fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting readfile_marker_range ASP execution\n");

    let myaspargs: ASP_ARGS_ReadfileMarkerRange =
        serde_json::from_value(args)
            .context("Could not decode ASP_ARGS for ASP readfile_marker_range")?;

    let file_path = PathBuf::from(&myaspargs.filepath);
    let begin_marker = &myaspargs.begin_marker;
    let end_marker = &myaspargs.end_marker;

    debug_print!(
        "Extracting marker range from {:?}: {:?} .. {:?}\n",
        file_path, begin_marker, end_marker
    );

    let bytes = extract_marker_range(&file_path, begin_marker, end_marker)
        .map_err(|e| anyhow!("readfile_marker_range: {}", e))?;

    Ok(vec![bytes])
}

fn main() {
    if let Ok(_cwd) = std::env::current_dir() {
        debug_print!("Current working directory: {}\n", _cwd.display());
    }
    handle_body(body);
}
