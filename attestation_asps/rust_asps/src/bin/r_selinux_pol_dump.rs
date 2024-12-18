// Common Packages
use anyhow::Result;
use rust_am_lib::copland::{self, handle_body};
use sha2::{Digest, Sha256};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> Result<copland::EvidenceT> {
    // let policy_name = _args.get("policy_name").unwrap();
    let policy_name = "demo_pipeline";

    // Execute the shell command to dump the selinux policy
    let mut output = std::process::Command::new(format!("semodule --cil --extract {policy_name}"));

    if output.status().is_err() {
        eprint!("Failed to execute the command to dump the policy\n");
        return Err(anyhow::anyhow!(
            "Failed to execute the command to dump the policy"
        ));
    }

    // This will place the output in a file named after the policy in the current directory
    let filename = format!("{policy_name}.cil");

    eprint!("Attempting to read from file: {}\n", filename);

    let bytes = std::fs::read(filename.clone())?; // Vec<u8>
    std::fs::remove_file(filename)?;

    let hash = Sha256::digest(&bytes);
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