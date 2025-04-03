// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let filename_value = args
        .get("filepath")
        .context("'filepath' argument not provided to ASP, readfile")?;

    if filename_value.is_string()
    {
        let filename: String = filename_value.to_string();

        eprint!("Attempting to read from file: {}\n", filename);

        let bytes = std::fs::read(&filename).context(
            "could not read file contents in ASP, readfile.  Perhaps the file doesn't exits?",
        )?;
        Ok(vec![bytes])
    }
    else {
        Err(anyhow::anyhow!("Failed to decode 'filepath' ASP arg as JSON String in readfile ASP"))
    }

}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_body(body);
}
