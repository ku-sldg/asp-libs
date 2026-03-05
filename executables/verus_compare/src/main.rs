#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

// This ASP ("verus_compare") is a measurement ASP that extracts specification and implementation code
// from two Verus files.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing the following arguments:
// - "original": A string path to the original Verus file.
// - "modified": A string path to the modified Verus file.
//
// OUTPUT:
// The ASP returns a raw evidence package (`RawEv`) containing a vector of four byte arrays (Vec<Vec<u8>>),
// structured as follows:
// 1. Original Spec: The extracted specification code from the "original" file.
// 2. Modified Spec: The extracted specification code from the "modified" file.
// 3. Original Impl: The extracted implementation code from the "original" file.
// 4. Modified Impl: The extracted implementation code from the "modified" file.

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_VerusCompare {
    original: String,
    modified: String,
}

use lynette::{extract_implementation, extract_spec_signatures, 
    strip_proof_functions, strip_verifier_attributes};
use std::path::PathBuf;



// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting verus_compare ASP execution\n");

    let myaspargs: ASP_ARGS_VerusCompare =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP verus_compare")?;

    let original_path = PathBuf::from(myaspargs.original);
    let modified_path = PathBuf::from(myaspargs.modified);
    // check that the files exist
    if !original_path.exists() {
        return Err(anyhow::anyhow!(
            "Original file does not exist: {}",
            original_path.display()
        ));
    }
    if !modified_path.exists() {
        return Err(anyhow::anyhow!(
            "Modified file does not exist: {}",
            modified_path.display()
        ));
    }

    debug_print!(
        "Original file: {}\\nModified file: {}\\n",
        original_path.display(),
        modified_path.display()
    );

    let original_spec = extract_spec_signatures(&original_path)?;
    let modified_spec = extract_spec_signatures(&modified_path)?;
    let original_impl = extract_implementation(&original_path)?;
    let modified_impl = extract_implementation(&modified_path)?;

    let original_spec_minus_proof = strip_proof_functions(&strip_verifier_attributes(&original_spec));
    let modified_spec_minus_proof = strip_proof_functions(&strip_verifier_attributes(&modified_spec));

    let original_impl_minus_proof = strip_proof_functions(&strip_verifier_attributes(&original_impl));
    let modified_impl_minus_proof = strip_proof_functions(&strip_verifier_attributes(&modified_impl));


    debug_print!("Extraction complete\n");
    debug_print!("Original Spec:\\n{}\\n", original_spec);
    debug_print!("Modified Spec:\\n{}\\n", modified_spec);
    debug_print!("Original Impl:\\n{}\\n", original_impl);
    debug_print!("Modified Impl:\\n{}\\n", modified_impl);

    Ok(vec![
        original_spec_minus_proof.into_bytes(),
        modified_spec_minus_proof.into_bytes(),
        original_impl_minus_proof.into_bytes(),
        modified_impl_minus_proof.into_bytes(),
    ])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    // debug print the current working directory
    if let Ok(_cwd) = std::env::current_dir() {
        debug_print!("Current working directory: {}\n", _cwd.display());
    } else {
        debug_print!("Could not get current working directory\n");
    }
    // debug print the program arguments on newlines
    for _arg in std::env::args() {
        debug_print!("arg: {}\n", _arg);
    }
    handle_body(body);
}