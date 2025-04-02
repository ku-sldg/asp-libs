// Very simple use of the sysinfo crate.
// Returns seconds since most recent book.

// The sysinfo crate provides access to a wide range of system information,
// including a variety of dynamic characteristics.

use anyhow::Result;
use rust_am_lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, _args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // returns seconds since last boot.
    let up = sysinfo::System::uptime();
    Ok(vec![up.to_be_bytes().to_vec()])
}

fn main() {
    handle_body(body);
}
