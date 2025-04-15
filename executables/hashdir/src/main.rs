// r_hashdir_ids.rs
// Follows general structure for ASP's written in rust

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Digest, Sha256};
use std::{fs, io};
use std::path::PathBuf;
//use lexical_sort::{StringSort, natural_lexical_cmp};

use serde::{Deserialize, Serialize};
//use serde_json::Value;


// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Hashdir {
    env_var: String,
    paths: Vec<String>,
    filepath_golden: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Code for specific for this ASP.
    // This example computes the HASH Composite of file paths specified in the "paths" argument to the ASP.
    // May return an Err Result, which will be captured in main.

    let myaspargs : ASP_ARGS_Hashdir = serde_json::from_value(args)
        .context("Could not parse ASP_ARGS for ASP hashdir")?;
    
    let env_var : String = myaspargs.env_var;

    let paths : Vec<String> = myaspargs.paths;

    /* TODO: check for empty string, if so set env_var_string to "" */
    let env_var_string = match std::env::var(&env_var) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable {}\n", env_var)
        }
    };

    let mut dir_entries : Vec<PathBuf> = Vec::new();

    for path in paths {

        let dirname_string = (path).clone();
        let dirname_full = format! {"{env_var_string}{dirname_string}"};
        dir_entries.push(dirname_full.into());

    }

    let mut file_entries : Vec<PathBuf> = Vec::new();

    for dir_entry in dir_entries {

        eprint!("Attempting to read from direcory: {}\n", dir_entry.display());

        let mut entries = fs::read_dir(&dir_entry)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        file_entries.append(&mut entries);

    }

    let mut filtered_entries: Vec<PathBuf> = file_entries.into_iter()
        .filter(|x| x.is_file() )
        .collect();

    filtered_entries.sort();

    
    let mut comp_hash: Vec<u8> = Vec::new();

    for entry in filtered_entries {
        // let dir = entry?;

        let bytes = std::fs::read(&entry)?;
        comp_hash.extend_from_slice(&bytes);
        //let v = entry.to_owned();
        eprintln!("{:?}", entry);
    }

    let hash = Sha256::digest(&comp_hash);
    

    Ok(vec![hash.to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}