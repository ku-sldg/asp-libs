#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! nonce_appr: companion appraiser of `generate_nonce`.
//!
//! Receives the nonce slot as evidence and the generate_nonce event's args
//! (the CVM hands a companion the appraised event's ASP_ARGS). Judges:
//!   - well-formedness: the slot is exactly `bytes` long (default 32) and
//!     not all zero;
//!   - when `expected_b64` is present in the args (pre-registered
//!     challenge, pybb docs/ledger_plan.md section G): byte equality.
//! Without `expected_b64` there is no expectation to compare against and
//! the verdict says only that a plausible nonce is present. No file,
//! environment, or network access. Verdict convention: "" = pass, else the
//! reason.

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rust_am_lib::{
    copland::{self, handle_appraisal_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

const DEFAULT_BYTES: usize = 32;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_NonceAppr {
    #[serde(default)]
    bytes: Option<usize>,
    #[serde(default)]
    expected_b64: Option<String>,
    #[serde(default)]
    asp_targid: Option<String>,
}

fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    debug_print!("Starting nonce_appr ASP execution\n");
    let myaspargs: ASP_ARGS_NonceAppr = if args.is_null() {
        ASP_ARGS_NonceAppr { bytes: None, expected_b64: None, asp_targid: None }
    } else {
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP nonce_appr")?
    };
    let want = myaspargs.bytes.unwrap_or(DEFAULT_BYTES);
    let nonce = ev.first().context("No nonce evidence found")?;
    debug_print!("Appraising nonce of {} bytes (expected {})\n", nonce.len(), want);

    if nonce.len() != want {
        return Ok(Err(anyhow::anyhow!(
            "nonce length {} does not match declared bytes {}", nonce.len(), want)));
    }
    if nonce.iter().all(|b| *b == 0) {
        return Ok(Err(anyhow::anyhow!("nonce is all zero")));
    }
    if let Some(expected_b64) = myaspargs.expected_b64 {
        let expected = B64
            .decode(expected_b64.as_bytes())
            .context("Could not base64-decode expected_b64")?;
        if expected.as_slice() != nonce.as_slice() {
            return Ok(Err(anyhow::anyhow!("nonce does not match the pre-registered challenge")));
        }
    }
    Ok(Ok(()))
}

fn main() {
    handle_appraisal_body(body);
}
