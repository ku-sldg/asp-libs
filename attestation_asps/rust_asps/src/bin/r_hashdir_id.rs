// TEMPLATE.txt
// General structure for ASP's written in rust

use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Digest, Sha256};
use std::{fs, io};
use std::path::PathBuf;
//use lexical_sort::{StringSort, natural_lexical_cmp};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::EvidenceT, args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let dirname = args
        .get("dirpath")
        .context("dirpath argument not provided to ASP, hashdir_id")?;

    let suffix = args
        .get("suffix")
        .context("suffix argument not provided to ASP, hashdir_id")?;


    let env_var_key = "DEMO_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable DEMO_ROOT")
        }
    };

    let dirname_string = (*dirname).clone();
    let dirname_full = format! {"{env_var_string}{dirname_string}"};

    eprint!("Attempting to read from direcory: {}\n", dirname_full);

    let mut entries = fs::read_dir(&dirname_full)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;


    

    entries.sort();  /* string_sort_unstable(natural_lexical_cmp); */

    let filtered_entries: Vec<PathBuf> = entries.into_iter()
        .filter(|x| x.to_string_lossy().ends_with(&*suffix) /* x.to_owned().ends_with(".json") */ )
        .collect();
    
    let mut comp_hash: Vec<u8> = Vec::new();

    for entry in filtered_entries {
        // let dir = entry?;

        let bytes = std::fs::read(&entry)?;
        comp_hash.extend_from_slice(&bytes);
        //let v = entry.to_owned();
        eprintln!("{:?}", entry);
    }


    /*
        let ev_flattened: Vec<u8> = ev.into_iter().flatten().collect();

    let hash = Sha256::digest(&ev_flattened);
    */


    
    //let bytes = std::fs::read(dirname_full)?; // Vec<u8>

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
