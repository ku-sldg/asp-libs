// Common Packages
use anyhow::Result;
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};
use sha2::{Digest, Sha256};
use std::process::Stdio;

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting selinux_pol_dump ASP execution\n");
    // let policy_name = _args.get("policy_name").unwrap();
    let policy_name = "demo_pipeline";
    debug_print!("Using policy_name: {}\n", policy_name);
    let command = "semodule";
    let args = ["--cil", "--extract", policy_name];
    debug_print!("Executing command: {} {:?}\n", command, args);
    let mut binding = std::process::Command::new(command); //.args(&args).stdout(Stdio::null());
    let output = binding.args(&args).stdout(Stdio::null());
    if output.status().is_err() {
        debug_print!(
            "Failed to execute the command to dump the policy: Error Code: {:?}\n",
            output.status()
        );
        return Err(anyhow::anyhow!(
            "Failed to execute the command to dump the policy"
        ));
    }
    // This will place the output in a file named after the policy in the current directory
    let filename = format!("{policy_name}.cil");
    debug_print!("Attempting to read from file: {}\n", filename);
    let bytes = std::fs::read(filename.clone())?; // Vec<u8>
    debug_print!("Read {} bytes from file\n", bytes.len());
    std::fs::remove_file(filename)?;
    let hash = Sha256::digest(&bytes);
    debug_print!("Generated hash of {} bytes\n", hash.len());
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
