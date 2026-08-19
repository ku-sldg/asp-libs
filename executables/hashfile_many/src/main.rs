// hashfile_many: batch whole-file hashing as ONE evidence blob.
//
// Hashes every file in `files` (relative to `root`) plus every file with
// suffix `walk_suffix` found by a sorted recursive walk of each dir in
// `walk_dirs`, and emits a single canonical JSON map
// {relpath: sha256-b64}. One term node instead of one per file: a
// 124-file linear bseq chain costs the CVM ~30s per pass (superlinear in
// chain length), the batch costs one ASP spawn. Because the walk
// self-enumerates, ADDED files change the evidence too — per-file target
// lists can't see additions.
//
// Determinism: BTreeMap ordering + canonical serde_json, so an unchanged
// tree yields byte-identical evidence; appraisal is an exact golden
// comparison via hashfile_many_appr (which names the drifted paths).

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn default_suffix() -> String {
    ".rs".to_string()
}

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_HashfileMany {
    // Named root (not filepath) so provisioning leaves it untouched: the
    // batch measures the LIVE tree at provision and attest time alike,
    // the same measure-in-place semantics as cheat_scan_verus.
    root: String,
    #[serde(default)]
    files: Vec<String>,
    #[serde(default)]
    walk_dirs: Vec<String>,
    #[serde(default = "default_suffix")]
    walk_suffix: String,
}

fn walk(dir: &Path, suffix: &str, out: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("Could not read directory {}", dir.display()))?
        .collect::<std::io::Result<_>>()?;
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, suffix, out)?;
        } else if path.to_string_lossy().ends_with(suffix) {
            out.push(path);
        }
    }
    Ok(())
}

fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let myaspargs: ASP_ARGS_HashfileMany = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP hashfile_many")?;

    let root = PathBuf::from(&myaspargs.root);
    let mut paths: Vec<PathBuf> = Vec::new();
    for rel in &myaspargs.files {
        paths.push(root.join(rel));
    }
    for dir in &myaspargs.walk_dirs {
        walk(&root.join(dir), &myaspargs.walk_suffix, &mut paths)?;
    }
    if paths.is_empty() {
        bail!("hashfile_many: no files to hash (empty files + walk_dirs)");
    }

    let mut hashes: BTreeMap<String, String> = BTreeMap::new();
    for path in &paths {
        let bytes = fs::read(path)
            .with_context(|| format!("Could not read {}", path.display()))?;
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        hashes.insert(rel, B64.encode(Sha256::digest(&bytes)));
    }

    let out = serde_json::to_vec(&hashes)?;
    Ok(vec![out])
}

fn main() {
    handle_body(body);
}
