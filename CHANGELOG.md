# Changelog

All notable changes to asp-libs are documented here. Each entry covers one
push and is written as a narrative summary of what changed and why.

---

## [dev] — 2026-04-16

### New ASPs: `run_command_hamr`, `run_command_hamr_appr`

Added a measurement/appraisal pair for invoking Sireum HAMR tools (tipe,
logika, test) as part of a Copland attestation protocol.

`run_command_hamr` runs a sireum command and returns a JSON evidence package
containing `exit_code`, `stdout`, and `stderr`. The appraiser
`run_command_hamr_appr` checks that `exit_code == 0`, surfacing the last
non-empty line of stderr as a diagnostic when the check fails. No provisioned
golden value is needed — the sireum tools are decision procedures (type
checker, formal verifier, test runner) that encode their own correctness
criterion; exit 0 means the property was proved or the tests passed.

**Security hardening:** The ASP accepts only `exe_args` — the arguments
forwarded to sireum. The sireum executable is always resolved by name from the
system PATH, and `SIREUM_HOME` is inherited from the CVM process environment.
Accepting a caller-supplied binary path (`sireum_bin`) or runtime home
directory (`sireum_home`) would allow any Copland protocol to execute
arbitrary code on the host; both were excluded for this reason.

### New ASP: `readfile_marker_range`

Added a new measurement ASP for reading a bounded range of bytes from a file,
identified by marker strings rather than fixed byte offsets.

### `hashfile_appr` — `golden_b64` support

Added an optional `golden_b64: Option<String>` field to `ASP_ARGS_Hashfile_Appr`.
When present, the golden SHA-256 hash is decoded from a base64 string injected
into ASP_ARGS at run time (from the dashboard's evidence bundle) rather than
read from a `.bin` file on disk. Falls back to the original
`env_var_golden` + `filepath_golden` file-based path when `golden_b64` is
absent. Added `base64 = "0.22"` as a workspace dependency.

### `readfile_appr` — `golden_b64` support

Same `golden_b64: Option<String>` pattern added to `readfile_appr`, with
identical priority and fallback logic.

---
