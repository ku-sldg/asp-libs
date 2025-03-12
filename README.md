# asp-libs

Repository for implementations of attestation service provider (asp) libraries (libs). ASP implementations serve as primitive units of work in attestation protocols and collections of ASPs build attestation manager configurations.

This repository will hold example ASP implementations, interface descriptions, and other documentation/tutorials for writing new ASPs and integrating them into larger attestation workflows.

## Building

Simply type `make` will build the `default` target. Which will build all targets except those specified with `DEFAULT_EXCLUDES` variable. To build all ASP executables in this repo, run `make all`.
Underneath, this makefile utilizes just basic `cargo` commands to build, thus you can also build specific executables with `cargo build -p <exec_name>`

## Testing

Check out the `testing` folder, specifically the `test_req.json` file and execute `./target/debug/<your_asp> "$(cat test_req.json)"`
to test out how your asp will respond to input.

NOTE: You will need to make sure you are doing the strings surrounding the input json (otherwise it won't be escaped properly).

## Conventions

ASPs should go into the `executables/<asp_name>` folder, with a `Cargo.toml` for their dependencies, and `src/main.rs` for their relevant code.
Namewise, this should have well-defined names that indicate their meaning. Do not add `_id` or other unnecessary identification marks, but rather let the name speak for itself. Names should be **snake_case**, and if an ASP is meant for appraisal of another ASP named `asp_x` should be named `asp_x_appr` (NOTE the `_appr` added)

## common_files

This has some stub/example public/private keys.
**NEVER** use these keys for anything importance. They are in the clear, unsecured, and available online!
