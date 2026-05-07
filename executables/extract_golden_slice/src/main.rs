#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// extract_golden_slice: extract per-target golden bytes from a
// provision_goldenevidence bundle using do_EvidenceSlice.
//
// Usage:
//   extract_golden_slice <protocol_dir> <asp_id> <targ_id>
//
// Reads:
//   <protocol_dir>/provision_bundle.json  — (Evidence, GlobalContext) written
//                                           by provision_goldenevidence
//   <protocol_dir>/asp_args.json          — measurement args for (asp_id, targ_id)
//
// Writes base64-encoded raw golden bytes to stdout.
//
// The ASP_ARGS used for matching are the measurement args from asp_args.json
// plus the asp_id_appr field injected by rust-am-lib::add_provisioning_args
// at provision time.  This must exactly match the ASP_PARAMS stored in the
// EvidenceT inside the bundle.

use std::path::Path;
use std::fs;
use anyhow::{Context, Result, bail};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rust_am_lib::copland::{
    self, do_EvidenceSlice, GlobalContext, Evidence, RawEv,
};
// RawEvT is private in rust_am_lib — use RawEv wrapper + rawev_to_vec instead.
use serde::Deserialize;
use serde_json::{Value, from_value};
use serde_stacker::Deserializer as StackerDeserializer;

fn deserialize_deep_json(json_data: &str) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_str(json_data);
    de.disable_recursion_limit();
    let stacker_de = StackerDeserializer::new(&mut de);
    Value::deserialize(stacker_de)
}

fn load_bundle(bundle_path: &Path) -> Result<(Evidence, GlobalContext)> {
    let contents = fs::read_to_string(bundle_path)
        .with_context(|| format!("Could not read bundle file: {}", bundle_path.display()))?;
    let val = deserialize_deep_json(&contents)
        .context("Could not parse bundle JSON")?;
    let bundle: (Evidence, GlobalContext) = from_value(val)
        .context("Could not deserialize (Evidence, GlobalContext) from bundle")?;
    Ok(bundle)
}

fn load_asp_args(asp_args_path: &Path, asp_id: &str, targ_id: &str) -> Result<Value> {
    let contents = fs::read_to_string(asp_args_path)
        .with_context(|| format!("Could not read asp_args.json: {}", asp_args_path.display()))?;
    let root: Value = serde_json::from_str(&contents)
        .context("Could not parse asp_args.json")?;

    let targ_args = root
        .get(asp_id)
        .with_context(|| format!("asp_id '{}' not found in asp_args.json", asp_id))?
        .get(targ_id)
        .with_context(|| format!("targ_id '{}' not found under asp_id '{}' in asp_args.json", targ_id, asp_id))?
        .clone();

    Ok(targ_args)
}

fn add_asp_id_appr(mut args: Value, asp_id: &str) -> Value {
    // Replicate rust-am-lib::add_provisioning_args_asp: inject asp_id_appr
    // into the ASP_ARGS so the match key equals what's in the EvidenceT.
    if let Value::Object(ref mut map) = args {
        map.insert(
            "asp_id_appr".to_string(),
            Value::String(asp_id.to_string()),
        );
    }
    args
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        bail!(
            "Usage: extract_golden_slice <protocol_dir> <asp_id> <targ_id>\n\
             Reads provision_bundle.json and asp_args.json from <protocol_dir>."
        );
    }
    let protocol_dir = Path::new(&args[1]);
    let asp_id = &args[2];
    let targ_id = &args[3];

    let bundle_path = protocol_dir.join("provision_bundle.json");
    let asp_args_path = protocol_dir.join("asp_args.json");

    // Load the (Evidence, GlobalContext) bundle
    let (evidence, glob_ctxt) = load_bundle(&bundle_path)?;

    // Load the measurement args for this (asp_id, targ_id) and add asp_id_appr
    let raw_args = load_asp_args(&asp_args_path, asp_id, targ_id)?;
    let match_args = add_asp_id_appr(raw_args, asp_id);

    // Build the ASP_PARAMS to search for in the EvidenceT
    let asp_params = copland::ASP_PARAMS {
        ASP_ID: asp_id.clone(),
        ASP_ARGS: match_args,
    };

    // Extract the evidence type and raw evidence from the bundle
    let et = copland::get_et(evidence.clone());
    let rawev = copland::get_rawev(evidence);

    // Slice out the bytes for this target.
    // do_EvidenceSlice returns the private RawEvT (Vec<String> of base64 chunks).
    // Wrap in RawEv and use rawev_to_vec to decode to Vec<Vec<u8>>.
    let sliced = do_EvidenceSlice(et, rawev, glob_ctxt, asp_params)
        .with_context(|| format!(
            "do_EvidenceSlice failed for asp_id='{}' targ_id='{}'",
            asp_id, targ_id
        ))?;

    let bytes_vec: Vec<Vec<u8>> = copland::rawev_to_vec(RawEv::RawEv(sliced));
    // Concatenate all chunks into a single flat byte sequence.
    let flat: Vec<u8> = bytes_vec.into_iter().flatten().collect();

    // Output base64 to stdout (no trailing newline — callers trim).
    print!("{}", B64.encode(&flat));

    Ok(())
}
