// TEMPLATE.txt
// General structure for ASP's written in rust

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//use std::io::Read;

use anyhow::{Result, Context};
use rust_am_lib::copland::{self, handle_body};
use serde::{Deserialize, Serialize};

use std::process::{Command};
//use std::process::Output;

//use subprocess::{Popen, PopenConfig};
//use subprocess::communicate_bytes;

// Packages required to perform specific ASP action.
// e.g.
//use sha2::{Digest, Sha256};


// ASP Arguments (JSON-decoded)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_RunCommandVerus {
    exe_args: Vec<String>
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.

    let myaspargs : ASP_ARGS_RunCommandVerus = serde_json::from_value(args)
    .context("Could not decode JSON ASP_ARGS for ASP run_command_verus")?;


    //let my_exe_path: String = myaspargs.exe_path;
    let verus_command_string = "verus".to_string();
    let my_exe_args= myaspargs.exe_args;


    let output = Command::new(verus_command_string)
                                .args(my_exe_args).output().expect("Error executing 'verus' command on PATH");
                            

    /*
    let output = Command::new("ls")
    .args(["-l", "-a"]).output().expect("hi");
*/

    let err_res = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    if ! err_res.is_empty() {eprint!("FYI:  stderr output after invoking run_command_verus ASP via the CVM: {:?}", String::from_utf8(err_res))}

    let res = out_res; 
    //if err_res.is_empty() {out_res} 
    //else {err_res};

    Ok (vec![res])

    /*
    let myaspargs : ASP_ARGS_Hashfile = serde_json::from_value(args)
        .context("Could not decode JSON ASP_ARGS for ASP hashfile")?;
    
    let env_var: String = myaspargs.env_var;
    let filename : String = myaspargs.filepath;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let filename_full = format! {"{env_var_string}{filename}"};

    eprint!("Attempting to read from file: {}\n", filename_full);
    let bytes = std::fs::read(filename_full)?; // Vec<u8>

    let hash = Sha256::digest(&bytes);
    */


    /*
    let proc = Popen::create(&["ls", "-la"], PopenConfig::default())?;

    let hhh = proc.communicate(input_data)

    let mut buff: Vec<u8> = Vec::new();
    let file = proc.stdout.ok_or(anyhow!("hi"))?;

    std::io::read_to_end(&file, &mut buff);  //read_to_end(buff);


    Ok (vec![buff])
 
    //proc.communicate_bytes(&mut self, input_data)

    */


    //Ok(vec![hash.to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
