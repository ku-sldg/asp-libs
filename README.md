# asp-libs
Repository for implementations of attestation service provider (asp) libraries (libs). ASP implementations serve as primitive units of work in attestation protocols and collections of ASPs build attestation manager configurations.

This repository will hold example ASP implementations, interface descriptions, and other documentation/tutorials for writing new ASPs and integrating them into larger attestation workflows.

## Building

Simply type `make` to build all ASP executables in this repo.  You can also descend into each sub-directory and type `make` to build subsets of the ASPs.  Each sub-directory should have its own Makefile (which produces executables in that sub-directory's `/bin`).

To avoid building executables for a certain sub-directory, use the `OMIT_DIRS` make variable to provide a (comma-separated) list of directories to omit building.  For example, to omit all openssl sub-directories type: `make OMIT_DIRS=openssl`.  For an example with multiple sub-directories:  `make OMIT_DIRS="openssl, appraisal_asps"` will omit building all asps in `/openssl` and `/appraisal_asps` sub-directories (DON'T FORGET to put multiple entries in quotes).

## Testing

Use the `test_req.json` file and execute `./bin/<your_asp> "$(cat test_req.json)"`
to test out how your asp will respond to input.

NOTE: You will need to make sure you are doing the strings surrounding the input json (otherwise it won't be escaped properly).

## common_files

This has some stub/example public/private keys.
**NEVER** use these keys for anything importance. They are in the clear, unsecured, and available online!