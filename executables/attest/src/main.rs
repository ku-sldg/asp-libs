// Common Packages
use anyhow::Result;
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting attest ASP execution\n");
    let result = "attest".as_bytes().to_vec();
    debug_print!("Attest ASP completed, returning {} bytes\n", result.len());
    Ok(vec![result])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
