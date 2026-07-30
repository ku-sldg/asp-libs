# Changelog

All notable changes to asp-libs are documented here. Each entry covers one
push and is written as a narrative summary of what changed and why.

---

## [hamr-asps] — 2026-07-30 (2)

### New ASP: `model_slices_appr`

Appraises whole-file evidence (from the `readfile` ASP) against golden
values that must be derivable from that content, serving pybb's signed
golden-spec baselines: an administrator blesses a WHOLE model file
(readfile + SIG at provisioning), and this appraiser checks that the
measurement protocols' goldens are extractable from the blessed bytes —
slice definitions are appraiser inputs checked against signed content,
never independently trusted labels.

Three optional checks via ASP_ARGS (fails closed if none present — no
vacuous blessings): `golden_b64` (full-content equality),
`hash_golden_b64` (SHA-256 recomputation, byte-compatible with the
hashfile ASP's goldens), and `slices` ({targ: {start_index, end_index,
golden_b64}} — each range re-extracted with readfile_range's exact
semantics: 1-based inclusive lines, BufReader::lines terminator handling,
concatenation with no separator). Failure reasons name the failing
checks/targets and are sanitized single-line (the CVM splices reasons
into response JSON unescaped; fix tracked separately).

---

## [hamr-asps] — 2026-07-30

### New ASPs: `run_command_lean`, `run_command_lean_appr`

Added a measurement/appraisal pair for the Lean 4 toolchain, serving the
pybb temp-control-lean example (a Lean specification and executable as the
third attested verification ecosystem, after Logika/JVM and Verus/Microkit).

`run_command_lean` invokes `lake` with caller-supplied `exe_args` and an
optional `cwd` (lake must run inside the Lake package; elan resolves the
pinned toolchain from the package's `lean-toolchain` file). One ASP serves
both semantic tiers: `lake lean <file> -- --json` re-elaborates a
specification, and `lake exe <target> <args>` runs the built executable.
As with `run_command_hamr`, the binary is resolved by name from PATH only.

`run_command_lean_appr` appraises in two modes selected by ASP_ARGS:

- **diagnostics mode** (default): parses the JSON-lines diagnostics that
  `lean --json` emits on stdout; PASS iff no diagnostic has severity
  `error` and none has kind `hasSorry`. A `sorry` exits 0 and only WARNS,
  so exit-code appraisal would bless a vacuous proof. Non-JSON stdout
  lines are skipped — build chatter is not evidence of failure (the
  cargo-verus cold-build lesson).
- **expected mode** (`{"expected": s}`): trimmed stdout comparison, for
  appraising the behavior of an executed binary against a known vector.

Failure reasons are sanitized to single-line, quote-free, bounded strings:
the CVM currently splices appraisal reasons into its response JSON without
escaping them, and Lean goal states are multi-line. (CVM-side escaping fix
tracked separately.)

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
