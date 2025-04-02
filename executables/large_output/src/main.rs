// Common Packages
use anyhow::Result;
use rust_am_lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // construct a 4096 byte string to return
    let mut large_output = String::new();
    for i in 0..4096 / 4 {
        // make the string 4096 bytes long and uniquely identifiable
        // 0000 0001 0002 0003 ... 4096/4
        large_output.push_str(&format!("{:04}", i));
    }
    Ok(vec![large_output.as_bytes().to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
