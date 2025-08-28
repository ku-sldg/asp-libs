// r_hashdir_ids.rs
// Follows general structure for ASP's written in rust

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use serde::{Deserialize, Serialize};

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::{fs, io};
//use lexical_sort::{StringSort, natural_lexical_cmp};

// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_Hashdir {
    env_var: String,
    paths: Vec<String>,
    omit_file_suffixes: Vec<String>,
    recursive: bool,
}

fn has_suffix(in_path: &PathBuf, suffixes: &Vec<String>) -> bool {
    let in_path_string: String = in_path.to_string_lossy().to_string();

    for suffix in suffixes {
        debug_print!(
            "Checking if path:  {:?} ends in suffix: {}",
            in_path_string,
            suffix
        );
        if
        /* (*in_path).ends_with(suffix.as_str()) */
        in_path_string.ends_with(suffix) {
            debug_print!("\n\n\n\n\n Found file with suffix:  {}\n\n\n\n", suffix);
            return true;
        }
    }
    return false;
}

fn get_all_file_entries(in_path: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>> {
    debug_print!("get_all_file_entries ({}, _)", in_path.display());

    let mut outvec: Vec<PathBuf> = Vec::new();

    if in_path.is_file() {
        outvec.push(in_path.into());
        Ok(outvec)
    } else if in_path.is_dir() {
        let entries = fs::read_dir(in_path.clone())?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        if recursive {
            for entry in entries {
                if entry.is_file() {
                    outvec.push(entry);
                } else if entry.is_dir() {
                    let inner_entries = fs::read_dir(&entry)?
                        .map(|res| res.map(|e| e.path()))
                        .collect::<Result<Vec<PathBuf>, io::Error>>()?;

                    for inner_entry in inner_entries {
                        let mut v = get_all_file_entries(&inner_entry, recursive)?;
                        outvec.append(&mut v);
                    }
                } else {
                    debug_print!("\n\nWARNING:  encountered file such that file.is_file() and file.is_dir() are both false\n");
                }
            }

            Ok(outvec)
        }
        // end if recursive
        else {
            let file_entries_only: Vec<PathBuf> =
                entries.into_iter().filter(|x| x.is_file()).collect();

            Ok(file_entries_only)
        }
    }
    // end else if in_path.is_dir()
    else {
        debug_print!("\n\nWARNING:  encountered path such that path.is_file() and path.is_dir() are both false\n");
        Ok(outvec)
    }
}

fn string_path_into_full_pathbuf(
    env_var_string_prefix: &String,
    in_string_path: String,
) -> PathBuf {
    let dirname_string = in_string_path;
    let dirname_full = format! {"{env_var_string_prefix}{dirname_string}"};
    dirname_full.into()
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Code for specific for this ASP.
    // This example computes the HASH Composite of file paths specified in the "paths" argument to the ASP.
    // May return an Err Result, which will be captured in main.

    let myaspargs: ASP_ARGS_Hashdir =
        serde_json::from_value(args).context("Could not decode ASP_ARGS for ASP hashdir")?;

    let env_var: String = myaspargs.env_var;

    let paths: Vec<String> = myaspargs.paths;

    let suffixes: Vec<String> = myaspargs.omit_file_suffixes;

    let is_recursive: bool = myaspargs.recursive;

    debug_print!("Will omit suffixes:  {:?}", suffixes);

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let dir_entries: Vec<PathBuf> = paths
        .into_iter()
        .map(|x| string_path_into_full_pathbuf(&env_var_string, x))
        .collect(); //Vec::new();

    let mut file_entries: Vec<PathBuf> = Vec::new();

    for dir_entry in dir_entries {
        debug_print!(
            "Attempting to read from direcory: {}\n",
            dir_entry.display()
        );

        let mut v = get_all_file_entries(&dir_entry, is_recursive)?;

        file_entries.append(&mut v);
    }

    let mut filtered_entries_no_suffixes: Vec<PathBuf> = file_entries
        .into_iter()
        .filter(|x| !(has_suffix(x, &suffixes)))
        .collect();

    filtered_entries_no_suffixes.sort();

    let mut comp_hash: Vec<u8> = Vec::new();

    for entry in filtered_entries_no_suffixes {
        let bytes = std::fs::read(&entry)?;
        comp_hash.extend_from_slice(&bytes);
        debug_print!("Entry:  {:?}", entry);
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
