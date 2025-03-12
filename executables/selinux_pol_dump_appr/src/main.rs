// Common Packages
use anyhow::Result;
use lib::copland::{self, handle_appraisal_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::EvidenceT, _args: copland::ASP_ARGS) -> Result<Result<()>> {
    let env_var_key = "AM_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable AM_ROOT")
        }
    };

    let pol_hash = ev.first().unwrap();

    // let policy_name = _args.get("policy_name").unwrap();
    let _policy_name = "demo_pipeline";

    let golden_policy_path =
        format!("{env_var_string}/tests/DemoFiles/goldenFiles/demo_pipeline_golden.cil");

    eprint!("Attempting to read from file: {}\n", golden_policy_path);

    let golden_bytes = std::fs::read(golden_policy_path)?; // Vec<u8>
    eprintln!("Policy Bytes: {:?}", pol_hash);
    eprintln!("Golden Bytes: {:?}", golden_bytes);

    match golden_bytes.eq(pol_hash) {
        true => Ok(Ok(())),
        false => Ok(Err(anyhow::anyhow!("Policy contents do not match"))),
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
