// MODIFICATION: This entire file is new.
// It contains functions for extracting specific parts of a Verus file that we care about most (the specification parts vs. the implementations part).

use crate::deghost::remove_ghost_from_file;
use crate::utils::{fextract_verus_macro, fprint_file, Formatter};
use crate::DeghostMode;
use std::path::PathBuf;

pub fn extract_spec_signatures(filepath: &PathBuf) -> Result<String, crate::utils::Error> {
    let spec_mode = DeghostMode {
        requires: true,
        ensures: true,
        invariants: false,
        spec: false,
        asserts: false,
        asserts_anno: false,
        decreases: false,
        assumes: true,
        sig_output: false,
        strip_bodies: true,
    };

    let (mut files, _) = fextract_verus_macro(filepath)?;
    let file = files.pop().unwrap();
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
        strip_bodies: false,
    };

    let (mut files, _) = fextract_verus_macro(filepath)?;
    let file = files.pop().unwrap();
    let impl_file = remove_ghost_from_file(&file, &impl_mode);
    Ok(fprint_file(&impl_file, Formatter::VerusFmt))
}
