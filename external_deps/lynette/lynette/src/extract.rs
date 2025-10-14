// MODIFICATION: This entire file is new.
// It contains functions for extracting specific parts of a Verus file.

use std::path::PathBuf;
use crate::utils::{fextract_verus_macro, fprint_file, Formatter};
use crate::deghost::remove_ghost_from_file;
use crate::DeghostMode;

pub fn extract_spec_functions(filepath: &PathBuf) -> Result<String, crate::utils::Error> {
    let spec_mode = DeghostMode {
        requires: true,
        ensures: true,
        invariants: true,
        spec: true,
        asserts: true,
        asserts_anno: true,
        decreases: true,
        assumes: true,
        sig_output: true,
    };

    let (_, file) = fextract_verus_macro(filepath)?;
    let spec_file = remove_ghost_from_file(&file, &spec_mode);
    Ok(fprint_file(&spec_file, Formatter::VerusFmt))
}

pub fn extract_implementation(filepath: &PathBuf) -> Result<String, crate::utils::Error> {
    let impl_mode = DeghostMode {
        requires: false,
        ensures: false,
        invariants: false,
        spec: false,
        asserts: false,
        asserts_anno: false,
        decreases: false,
        assumes: false,
        sig_output: false,
    };

    let (_, file) = fextract_verus_macro(filepath)?;
    let impl_file = remove_ghost_from_file(&file, &impl_mode);
    Ok(fprint_file(&impl_file, Formatter::VerusFmt))
}
