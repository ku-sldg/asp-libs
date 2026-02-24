#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod extract;
pub use extract::{get_attestation_report_json, HAMR_AttestationReport, 
    HAMR_ComponentReport, HAMR_ComponentContractReport, 
    HAMR_attestation_report_to_File_Slices, File_Slice,
    HAMR_Slice, HAMR_Pos, relpath_to_abspath, Slices_Map
};

