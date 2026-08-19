// hashfile_many_appr: appraise a hashfile_many evidence blob against the
// provisioned golden. Exact comparison of the two {relpath: sha256-b64}
// maps, with ATTRIBUTION: the refusal message names each drifted,
// missing, and added path, so a single batched target keeps the per-file
// diagnosis the old one-target-per-file shape gave in the verdict.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rust_am_lib::copland::{self, handle_appraisal_body};
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_HashfileMany_Appr {
    golden_b64: Option<String>,
}

const MAX_NAMED: usize = 8; // cap the paths named per category

fn named(mut paths: Vec<&String>) -> String {
    paths.sort();
    let extra = paths.len().saturating_sub(MAX_NAMED);
    let mut s = paths
        .into_iter()
        .take(MAX_NAMED)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    if extra > 0 {
        s.push_str(&format!(" (+{extra} more)"));
    }
    s
}

fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    let myaspargs: ASP_ARGS_HashfileMany_Appr = serde_json::from_value(args)
        .context("Could not decode ASP_ARGS for hashfile_many_appr")?;

    let golden_b64 = myaspargs
        .golden_b64
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow!("hashfile_many_appr: no golden_b64 provided"))?;
    let golden_bytes = B64
        .decode(&golden_b64)
        .context("Could not base64-decode golden_b64")?;
    let golden: BTreeMap<String, String> = serde_json::from_slice(&golden_bytes)
        .context("Could not decode golden as a {relpath: hash} map")?;

    let evidence_in = ev.first().context("No evidence bytes found in incoming RawEv")?;
    let measured: BTreeMap<String, String> = serde_json::from_slice(evidence_in)
        .context("Could not decode evidence as a {relpath: hash} map")?;

    if measured == golden {
        return Ok(Ok(()));
    }

    let drifted: Vec<&String> = golden
        .iter()
        .filter(|(p, h)| measured.get(*p).is_some_and(|m| m != *h))
        .map(|(p, _)| p)
        .collect();
    let missing: Vec<&String> =
        golden.keys().filter(|p| !measured.contains_key(*p)).collect();
    let added: Vec<&String> =
        measured.keys().filter(|p| !golden.contains_key(*p)).collect();

    let mut parts = Vec::new();
    if !drifted.is_empty() {
        parts.push(format!("hash drift: {}", named(drifted)));
    }
    if !missing.is_empty() {
        parts.push(format!("missing: {}", named(missing)));
    }
    if !added.is_empty() {
        parts.push(format!("added: {}", named(added)));
    }
    Ok(Err(anyhow!(
        "Evidence does not match golden — {}",
        parts.join("; ")
    )))
}

fn main() {
    handle_appraisal_body(body);
}
