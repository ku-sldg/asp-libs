// Common Packages
use anyhow::Result;
use rust_am_lib::copland::{self, handle_appraisal_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    Ok(Ok(()))
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
