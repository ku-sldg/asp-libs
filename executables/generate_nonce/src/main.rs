#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! generate_nonce: a measurement ASP whose "measurement" is fresh randomness.
//!
//! Placed first in a Copland protocol (`lseq (asp generate_nonce) ...`) so
//! that every later measurement, and the closing `sig`, is bound to a value
//! that did not exist before this run. The nonce is drawn from the OS
//! CSPRNG and emitted as the ASP's single output slot; nothing is written
//! to disk, the environment, or the network. The companion appraiser is
//! `nonce_appr`. Design: pybb docs/ledger_plan.md, section B.

use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

const DEFAULT_BYTES: usize = 32;
const MAX_BYTES: usize = 1024;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_GenerateNonce {
    /// Nonce length in bytes (default 32). Shared with nonce_appr via the
    /// evidence type, which carries these args to the companion.
    #[serde(default)]
    bytes: Option<usize>,
    /// pybb's self-identification key; unused here, carried for attribution.
    #[serde(default)]
    asp_targid: Option<String>,
}

fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting generate_nonce ASP execution\n");
    let myaspargs: ASP_ARGS_GenerateNonce = if args.is_null() {
        ASP_ARGS_GenerateNonce { bytes: None, asp_targid: None }
    } else {
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP generate_nonce")?
    };
    let n = myaspargs.bytes.unwrap_or(DEFAULT_BYTES);
    if n == 0 || n > MAX_BYTES {
        anyhow::bail!("generate_nonce: bytes must be in 1..={MAX_BYTES}, got {n}");
    }
    let mut nonce = vec![0u8; n];
    openssl::rand::rand_bytes(&mut nonce).context("CSPRNG failure in generate_nonce")?;
    debug_print!("Generated {} nonce bytes\n", nonce.len());
    Ok(vec![nonce])
}

fn main() {
    handle_body(body);
}
