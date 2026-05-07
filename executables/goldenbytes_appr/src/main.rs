#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// goldenbytes_appr: appraises a measurement by comparing incoming evidence
// bytes against a pre-provisioned golden value.
//
// The golden value (raw bytes, base64-encoded) must have been extracted via
// do_EvidenceSlice from a provision_goldenevidence bundle.  Two storage modes:
//
//   golden_b64       — inline base64 bytes in ASP_ARGS (default)
//   golden_file_b64  — path to a file containing a base64 string
//
// Exactly one must be provided; golden_b64 takes priority if both present.

use anyhow::{Context, Result, bail};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rust_am_lib::{
    copland::{self, handle_appraisal_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_GoldenBytes_Appr {
    /// Inline base64-encoded raw golden bytes (takes priority).
    golden_b64: Option<String>,
    /// Path to a file containing a base64-encoded golden bytes string.
    golden_file_b64: Option<String>,
}

fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    debug_print!("Starting goldenbytes_appr ASP execution\n");

    let myaspargs: ASP_ARGS_GoldenBytes_Appr = serde_json::from_value(args)
        .context("Could not decode ASP_ARGS for goldenbytes_appr")?;

    let golden_bytes: Vec<u8> = match &myaspargs.golden_b64 {
        Some(b64) if !b64.is_empty() => {
            debug_print!("Using inline golden_b64 from ASP_ARGS\n");
            B64.decode(b64).context("Could not base64-decode golden_b64")?
        }
        _ => match &myaspargs.golden_file_b64 {
            Some(path) if !path.is_empty() => {
                debug_print!("Reading golden from file: {}\n", path);
                let b64 = std::fs::read_to_string(path)
                    .with_context(|| format!("Could not read golden_file_b64 at {}", path))?;
                B64.decode(b64.trim())
                    .context("Could not base64-decode contents of golden_file_b64")?
            }
            _ => bail!(
                "goldenbytes_appr: neither golden_b64 nor golden_file_b64 provided (or both empty)"
            ),
        },
    };

    let evidence_in = ev.first().context("No evidence bytes found in incoming RawEv")?;

    debug_print!(
        "Comparing {} golden bytes against {} evidence bytes\n",
        golden_bytes.len(),
        evidence_in.len()
    );

    match golden_bytes.eq(evidence_in) {
        true => Ok(Ok(())),
        false => Ok(Err(anyhow::anyhow!(
            "Evidence bytes do not match golden"
        ))),
    }
}

fn main() {
    handle_appraisal_body(body);
}
