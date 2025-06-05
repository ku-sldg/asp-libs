// Common Packages
use anyhow::Result;
use rust_am_lib::copland::{self, handle_body};
use rust_am_lib::debug_print;

use sha2::{Digest, Sha256};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting hashevidence ASP execution\n");
    let ev_flattened: Vec<u8> = ev.into_iter().flatten().collect();
    debug_print!("Flattened evidence to {} bytes\n", ev_flattened.len());
    let hash = Sha256::digest(&ev_flattened);
    debug_print!("Generated hash of {} bytes\n", hash.len());
    Ok(vec![hash.to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
