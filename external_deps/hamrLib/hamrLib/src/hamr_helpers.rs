#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages

use std::fs;
use std::path::{self, Path};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File_Slice {
    pub filepath: String,   // - "filepath": A string path to the file to be read.
    pub start_index: usize, // - "start_index": A number for the starting line index (starting at 1).
    pub end_index: usize    // - "end_index":   A number for the ending line index.
}

pub type Slices_Map = HashMap<String, Vec<u8>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_Pos {
    pub r#type: String,
    pub uri: String,
    pub beginLine: usize,
    pub beginCol: usize,
    pub endLine: usize,
    pub endCol: usize,
    pub offset: usize,
    pub length: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_Slice {
    pub r#type: String,
    pub kind: String,
    pub meta: String,
    pub pos: HAMR_Pos
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_ComponentContractReport {
    pub r#type: String,
    pub id: String,
    pub kind: String,
    pub meta: String,
    pub slices: Vec<HAMR_Slice>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_ComponentReport {
    pub r#type: String,
    pub idPath: Vec<String>,
    pub classifier: Vec<String>,
    pub reports: Vec<HAMR_ComponentContractReport>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_AttestationReport {
    r#type: String,
    pub reports: Vec<HAMR_ComponentReport>
}

pub fn get_attestation_report_json (hamr_report_fp:&Path) -> std::io::Result<HAMR_AttestationReport>  {

    let type_string = "HAMR_AttestationReport".to_string();
    let err_string = format!("Couldn't read {type_string} JSON file");
    let term_contents = fs::read_to_string(hamr_report_fp).expect(err_string.as_str());
    let term : HAMR_AttestationReport = serde_json::from_str(&term_contents)?;
    Ok(term)
}

pub fn HAMR_attestation_report_to_File_Slices (hamr_report:HAMR_AttestationReport, project_root_fp:&Path) -> Vec<File_Slice> {

    let reports = hamr_report.reports;

    let res1 : Vec<Vec<File_Slice>> = reports.iter().map(|x| HAMR_component_report_to_File_Slices(x.clone(), project_root_fp)).collect();

    let res = res1.into_iter().flatten().collect();
    res
}

fn HAMR_component_report_to_File_Slices (hamr_component_report:HAMR_ComponentReport, project_root_fp:&Path) -> Vec<File_Slice> {

    let reports = hamr_component_report.reports;

    let res1 : Vec<Vec<File_Slice>> = reports.iter().map(|x| HAMR_component_contract_report_to_File_Slice(x.clone(), project_root_fp)).collect();

    let res = res1.into_iter().flatten().collect();

    res
}

pub fn relpath_to_abspath (project_root_fp:&Path, relpath:&Path) -> String {

    let root = Path::new(project_root_fp);
    let relative = Path::new(relpath);

    let combined_path = root.join(relative);
    
    // Normalize the path using std::path::absolute
    let normalized_absolute_path = path::absolute(&combined_path).unwrap();

    let canonnicalized_path = fs::canonicalize(normalized_absolute_path).unwrap();

    let res = canonnicalized_path.to_str().unwrap().to_string();
    res

}

fn HAMR_Slice_to_File_Slice (hamr_slice:&HAMR_Slice, project_root_fp:&Path) -> File_Slice {

    let uri_relative = hamr_slice.pos.uri.clone();
    let uri_relative_path = Path::new(&uri_relative);

    let uri_absolute = relpath_to_abspath(project_root_fp, uri_relative_path);
    let bline = hamr_slice.pos.beginLine;
    let eline = hamr_slice.pos.endLine;

    let res : File_Slice = 
        File_Slice { filepath: uri_absolute, 
                        start_index: bline, 
                        end_index: eline };
    res
}

fn HAMR_component_contract_report_to_File_Slice (hamr_component_contract_report:HAMR_ComponentContractReport, project_root_fp:&Path) -> Vec<File_Slice> {

    let slices = hamr_component_contract_report.slices;
    let file_slices : Vec<File_Slice> = slices.iter().map(|x| HAMR_Slice_to_File_Slice(x, project_root_fp)).collect();
    file_slices

}