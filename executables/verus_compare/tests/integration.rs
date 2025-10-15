fn test_exec_and_output(test_name: &str, exec: &str, args: &str, expected_output: &str) {
    // Capture the output of running [exec] with [args] as a single argument
    let output = std::process::Command::new(exec)
        .arg(args)
        .output()
        .expect("Failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // If we fail this, print the full output for debugging
    assert!(
        stdout.contains(expected_output),
        "Test {}: Output did not contain expected string\nOutput: ```\n{}\n```\nExpected to contain: \n```{}\n```\nFull output for debugging: \n{:?}",
        test_name,
        stdout,
        expected_output,
        output
    );
}

const SAME_REQ: &str = include_str!("../test_files/test_req.json");
const SAME_IMPL_REQ: &str = include_str!("../test_files/test_req_same_impl.json");
const SAME_SPEC_REQ: &str = include_str!("../test_files/test_req_same_spec.json");
const DIFF_REQ: &str = include_str!("../test_files/test_req_diff_both.json");

// Invoke the executable. Cargo sets CARGO_BIN_EXE_<name> for integration tests;
// use the compile-time env! macro to get the built binary path for `verus_compare`.
const OUR_EXEC: &str = env!("CARGO_BIN_EXE_verus_compare");

#[test]
fn same_req() {
    // For each of the test cases, run the executable with the JSON string as an argument
    // Schema: <our_exec> "<json_string>"
    // Capture the output
    test_exec_and_output(
        "Same Request",
        OUR_EXEC,
        SAME_REQ,
        r#"{"TYPE":"RESPONSE","ACTION":"ASP_RUN","SUCCESS":true,"PAYLOAD":{"RawEv":["dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMjAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMjAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDIKfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDIKfQoKfSAvLyB2ZXJ1cyEK"]}}"#,
    );
}

#[test]
fn same_impl_req() {
    test_exec_and_output(
        "Same Implementation Request",
        OUR_EXEC,
        SAME_IMPL_REQ,
        r#"{"TYPE":"RESPONSE","ACTION":"ASP_RUN","SUCCESS":true,"PAYLOAD":{"RawEv":["dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMjAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMzAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDIKfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDIKfQoKfSAvLyB2ZXJ1cyEK"]}}"#,
    );
}

#[test]
fn same_spec_req() {
    test_exec_and_output(
        "Same Specification Request",
        OUR_EXEC,
        SAME_SPEC_REQ,
        r#"{"TYPE":"RESPONSE","ACTION":"ASP_RUN","SUCCESS":true,"PAYLOAD":{"RawEv":["dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMjAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMjAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDIKfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDEKfQoKfSAvLyB2ZXJ1cyEK"]}}"#,
    );
}

#[test]
fn different_req() {
    test_exec_and_output(
        "Different Request",
        OUR_EXEC,
        DIFF_REQ,
        r#"{"TYPE":"RESPONSE","ACTION":"ASP_RUN","SUCCESS":true,"PAYLOAD":{"RawEv":["dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMjAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiAocmVzOiB1MzIpCiAgICByZXF1aXJlcwogICAgICAgIHggPCAxMDAsCiAgICBlbnN1cmVzCiAgICAgICAgcmVzIDwgMTAwLAp7Cn0KCn0gLy8gdmVydXMhCg==","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDIKfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCmZuIHRlc3RfZnVuY3Rpb24oeDogdTMyKSAtPiB1MzIgewogICAgeCAqIDEKfQoKfSAvLyB2ZXJ1cyEK"]}}"#,
    );
}
