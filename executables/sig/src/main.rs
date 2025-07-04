// Common Packages
use anyhow::Result;
use rust_am_lib::{
    copland::{self, handle_body},
    debug_print,
};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    debug_print!("Starting sig ASP execution\n");
    let ev_flattened: Vec<u8> = ev.into_iter().flatten().collect();
    debug_print!("Flattened evidence to {} bytes\n", ev_flattened.len());

    // Use openssl to sign the input message
    let key = include_bytes!("../../../common_files/unsecure_priv_key_dont_use.pem");
    let pkey = openssl::pkey::PKey::private_key_from_pem(key)?;
    let mut signer = openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &pkey)?;
    signer.update(&ev_flattened)?;
    let signature = signer.sign_to_vec()?;
    debug_print!("Generated signature of {} bytes\n", signature.len());

    Ok(vec![signature])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
