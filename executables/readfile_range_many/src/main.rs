#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};

use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;

use flate2::write::GzEncoder;
//use flate2::read::GzDecoder;
use flate2::Compression;
//use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
//use serde::de::DeserializeOwned;
use serde_stacker::Deserializer;

use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
//use serde::{Deserialize, Serialize};

// This ASP ("readfile_range_many") is a measurement ASP that reads the contents of the specified lines of text from a collection of files.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing the following arguments:
// - "filepath": A string path to the file to be read.
// - "start_index": A number for the starting line index (starting at 1).
// - "end_index":   A number for the ending line index.
//
// OUTPUT:
// The ASP returns a raw evidence package (`RawEv`) containing a vector of length 1 with the only member being a byte array (Vec<u8>), 
//     containing the encoded contents of the Slices_Map structure defined below.  The keys in that HashMap structure are of the form:  `<filepath>::<start_index>-<end_index>`, and   
//     the values are byte arrays (encoded Vec<u8>s) of the file contents at those line ranges.  For simplicity, we chose not to preserve line boundaries
//     of the contents because that would make the output evidence structure depend on the input file range.   

#[derive(Serialize, Deserialize, Debug, Clone)]
struct File_Slice {
    filepath: String,
    start_index: usize,
    end_index: usize
}

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRangeMany {
    slices_file: String
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

type Slices_Map = HashMap<String, Vec<u8>>;

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

fn deserialize_deep_json_string(json_data: &str) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_str(json_data);
    de.disable_recursion_limit(); // This method is only available with the feature
    
    // Wrap with serde_stacker's Deserializer to use a dynamically growing stack
    let stacker_de = Deserializer::new(&mut de);
    
    // Deserialize the data
    let value = Value::deserialize(stacker_de)?;
    
    Ok(value)
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting readfile_range_many ASP execution\n");

    let myaspargs: ASP_ARGS_ReadfileRangeMany =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP readfile_range_many")?;

    let slices_file = myaspargs.slices_file;

    let contents = fs::read_to_string(slices_file).expect("Couldn't read slices_file JSON file in readfile_range_many");
    debug_print!{"\n\nAttempting to decode Vec<File_Slice>...\n\n"};
    let my_contents_val = deserialize_deep_json_string(&contents)?;
    let slices: Vec<File_Slice> = from_value(my_contents_val)?;



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
    

    //let res= serde_json::to_vec(&m)?;

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
