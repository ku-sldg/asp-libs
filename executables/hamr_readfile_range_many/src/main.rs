#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};

use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{self, Path};
use std::collections::HashMap;

use flate2::write::GzEncoder;
//use flate2::read::GzDecoder;
use flate2::Compression;

use serde::{Deserialize, Serialize};

use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};

// This ASP ("hamr_readfile_range_many") is a measurement ASP that parses a HAMR Attestation Report and reads the contents of the specified lines of text from a collection of files.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing the following arguments:
// - "attestation_report_filepath":  A filepath(String) pointing to a HAMR Attestation Report JSON object

//
// OUTPUT:
// The ASP returns a raw evidence package (`RawEv`) containing a vector of length 1 with the only member being a byte array (Vec<u8>), 
//     containing the encoded contents of the Slices_Map structure defined below.  The keys in that HashMap structure are of the form:  `<filepath>::<start_index>-<end_index>`, and   
//     the values are byte arrays (encoded Vec<u8>s) of the file contents at those line ranges.  For simplicity, we chose not to preserve line boundaries
//     of the contents because that would make the output evidence structure depend on the input file range.

//     NOTE:  Additionally, we choose to gzip compress the Slices_Map structure to trim down the output evidence size.  
//            Any dual appraisal ASP will first need to decompress the raw data before decoding and proceeding with appraisal.

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_HAMR_ReadfileRangeMany {
    attestation_report_filepath: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct File_Slice {
    filepath: String,   // - "filepath": A string path to the file to be read.
    start_index: usize, // - "start_index": A number for the starting line index (starting at 1).
    end_index: usize    // - "end_index":   A number for the ending line index.
}

type Slices_Map = HashMap<String, Vec<u8>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_Pos {
    r#type: String,
    uri: String,
    beginLine: usize,
    beginCol: usize,
    endLine: usize,
    endCol: usize,
    offset: usize,
    length: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_Slice {
    r#type: String,
    kind: String,
    meta: String,
    pos: HAMR_Pos
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_ComponentContractReport {
    r#type: String,
    id: String,
    kind: String,
    meta: String,
    slices: Vec<HAMR_Slice>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_ComponentReport {
    r#type: String,
    idPath: Vec<String>,
    classifier: Vec<String>,
    reports: Vec<HAMR_ComponentContractReport>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_AttestationReport {
    r#type: String,
    reports: Vec<HAMR_ComponentReport>
}

fn compress_string(s: &str) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes())?;
    encoder.finish()
}

/*
fn decompress_string(compressed_data: &[u8]) -> io::Result<String> {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut s = String::new();
    decoder.read_to_string(&mut s)?;
    Ok(s)
}
*/

fn read_line_range<P: AsRef<Path>>(
    path: P,
    start_line: usize,
    end_line: usize
) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut lines_in_range = Vec::new();
  
    // Line numbers are typically 1-based, so we adjust for 0-based indexing
    let start_index = start_line.saturating_sub(1);
    // end_line is inclusive in this implementation

    for (index, line_result) in reader.lines().enumerate() {
        if index >= start_index && index < end_line {
            lines_in_range.push(line_result?);
        } else if index >= end_line {
            // Stop reading once the end of the range is passed
            break;
        }
    }
    
    Ok(lines_in_range)
}

fn get_bytevec_fileslice (
    s: File_Slice ) -> io::Result<Vec<u8>> {

        let lines  = read_line_range(s.filepath, s.start_index, s.end_index)?;
        let res: Vec<u8> = lines.into_iter()
                            .flat_map(|s| s.into_bytes())
                            .collect();
        Ok(res)
}

fn get_attestation_report_json (hamr_report_fp:&Path) -> std::io::Result<HAMR_AttestationReport>  {

    let type_string = "HAMR_AttestationReport".to_string();
    let err_string = format!("Couldn't read {type_string} JSON file");
    let term_contents = fs::read_to_string(hamr_report_fp).expect(err_string.as_str());
    let term : HAMR_AttestationReport = serde_json::from_str(&term_contents)?;
    Ok(term)
}

fn HAMR_attestation_report_to_File_Slices (hamr_report:HAMR_AttestationReport, project_root_fp:&Path) -> Vec<File_Slice> {

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

fn relpath_to_abspath (project_root_fp:&Path, relpath:&Path) -> String {

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

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting readfile_range_many ASP execution\n");

    let myaspargs: ASP_ARGS_HAMR_ReadfileRangeMany =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP hamr_readfile_range_many")?;

    let report_filepath_string = myaspargs.attestation_report_filepath;
    let report_filepath = Path::new(&report_filepath_string);

    let att_report: HAMR_AttestationReport = get_attestation_report_json(report_filepath)?;

    let attestation_report_root = report_filepath.parent().unwrap();

    let slices = HAMR_attestation_report_to_File_Slices(att_report, attestation_report_root);

    let mut m : Slices_Map = HashMap::new();

    for s in slices.into_iter() {

        let bline = s.start_index.clone();
        let eline = s.end_index.clone();
        let uri = s.filepath.clone();

        let bline_string= bline.to_string();
        let eline_string = eline.to_string();
        let uri_slice_string = format!("{uri}::{bline_string}-{eline_string}");

        if ! m.contains_key(&uri_slice_string) {
            let v = get_bytevec_fileslice(s)?;
            m.insert(uri_slice_string, v);
        }
    };

    let res_str = serde_json::to_string(&m)?;

    let compressed_str = compress_string(&res_str)?;

    let res = compressed_str;

     Ok(vec![res])
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
