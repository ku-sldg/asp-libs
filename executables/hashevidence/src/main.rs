// Common Packages
use anyhow::Result;
use lib::copland::{self, handle_body};

use sha2::{Digest, Sha256};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    let ev_flattened: Vec<u8> = ev.into_iter().flatten().collect();

    let hash = Sha256::digest(&ev_flattened);

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
