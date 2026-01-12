#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

// This ASP ("readfile_range") is a measurement ASP that reads the contents of the specified lines of text from a file.
//
// INPUT:
// The ASP expects a JSON object with an "ASP_ARGS" field containing the following arguments:
// - "filepath": A string path to the file to be read.
// - "start_index": A number for the starting line index (starting at 1).
// - "end_index":   A number for the ending line index.
//
// OUTPUT:
// The ASP returns a raw evidence package (`RawEv`) containing a vector of length 1 with the only member being a byte array (Vec<u8>), 
//     containing the "flattened" contents of the file in the specified range.  For simplicity, we chose not to preserve line boundaries
//     of the contents because that would make the output evidence structure depend on the input file range.   


// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRange {
    filepath: String,
    start_index: usize,
    end_index: usize
}

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

//use lynette::{extract_implementation, extract_spec_signatures};
use std::path::PathBuf;

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting verus_compare ASP execution\n");

    let myaspargs: ASP_ARGS_ReadfileRange =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP readfile_range")?;

    let file_path = PathBuf::from(myaspargs.filepath);
    //let modified_path = PathBuf::from(myaspargs.modified);
    let start_index = myaspargs.start_index;
    let end_index = myaspargs.end_index;

    let lines = read_line_range(file_path, start_index, end_index)?;

    eprintln!("{:?}", lines);

    let res: Vec<u8> = lines.into_iter()
                                .flat_map(|s| s.into_bytes())
                                .collect();



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
