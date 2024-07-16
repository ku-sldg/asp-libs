# asp-libs
Repository for implementations of attestation service provider (asp) libraries (libs). ASP implementations serve as primitive units of work in attestation protocols and collections of ASPs build attestation manager configurations.

This repository will hold example ASP implementations, interface descriptions, and other documentation/tutorials for writing new ASPs and integrating them into larger attestation workflows.

## Testing

Use the `test_req.json` file and execute `./bin/<your_asp> "$(cat test_req.json)"`
to test out how your asp will respond to input.

NOTE: You will need to make sure you are doing the strings surrounding the input json (otherwise it won't be escaped properly).

## common_files

This has some stub/example public/private keys.
**NEVER** use these keys for anything importance. They are in the clear, unsecured, and available online!