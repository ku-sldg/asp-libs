#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::Result;
use rust_am_lib::{
    copland::{self, handle_appraisal_body},
    debug_print,
};

// This ASP ("verus_compare_appr") is an appraisal ASP that compares the evidence produced by
// the "verus_compare" measurement ASP.
//
// INPUT:
// The ASP expects a raw evidence package (`RawEv`) containing a vector of four byte arrays (Vec<Vec<u8>>),
// structured as follows:
// 1. Original Spec: The extracted specification code from the "original" file.
// 2. Modified Spec: The extracted specification code from the "modified" file.
// 3. Original Impl: The extracted implementation code from the "original" file.
// 4. Modified Impl: The extracted implementation code from the "modified" file.

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    debug_print!("Starting verus_compare_appr ASP execution\n");

    if ev.len() != 4 {
        anyhow::bail!("Expected 4 evidence items, but got {}", ev.len());
    }

    let original_spec = &ev[0];
    let modified_spec = &ev[1];
    let original_impl = &ev[2];
    let modified_impl = &ev[3];

    if original_spec == modified_spec {
        debug_print!("Original spec matches modified spec.\n");
        if original_impl == modified_impl {
            debug_print!("Original impl matches modified impl.\n");
            Ok(Ok(()))
        } else {
            debug_print!("Original impl does NOT match modified impl.\n");
            Ok(Err(anyhow::anyhow!("Impl's mismatch")))
        }
    } else {
        debug_print!("Original spec does NOT match modified spec.\n");
        Ok(Err(anyhow::anyhow!("Spec's mismatch")))
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
