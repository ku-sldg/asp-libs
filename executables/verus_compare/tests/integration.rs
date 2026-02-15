use std::io::Write;
use std::process::{Command, Stdio};

fn test_exec_and_output(test_name: &str, exec: &str, input_data: &str, expected_output: &str) {
    let mut child = Command::new(exec)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin
            .write_all(input_data.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains(expected_output),
        "Test {}: Output did not contain expected string\n\
         --- Captured STDOUT ---\n{}\n\
         --- Expected to contain ---\n{}\n\
         --- Full Metadata ---\n{:?}",
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
const LIN_SEARCH_REQ: &str = include_str!("../test_files/test_req_lin_search.json");

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

#[test]
fn lin_search_req() {
    test_exec_and_output(
        "Linear Search Request",
        OUR_EXEC,
        LIN_SEARCH_REQ,
        r#"{"TYPE":"RESPONSE","ACTION":"ASP_RUN","SUCCESS":true,"PAYLOAD":{"RawEv":["dmVydXMhIHsKCnB1YiBmbiBsaW5lYXJfc2VhcmNoKGE6ICZWZWM8aTMyPiwgZTogaTMyKSAtPiAobjogdXNpemUpCiAgICByZXF1aXJlcwogICAgICAgIGV4aXN0c3xpOiBpbnR8ICgwIDw9IGkgPCBhLmxlbigpIGFzIGludCkgJiYgYVtpXSA9PSBlLAogICAgZW5zdXJlcwogICAgICAgIDAgPD0gbiA8IGEubGVuKCksCiAgICAgICAgYVtuIGFzIGludF0gPT0gZSwKICAgICAgICBmb3JhbGx8azogaW50fCAoMCA8PSBrIDwgbiBhcyBpbnQpID09PiBhW2tdICE9IGUsCnsKfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCnB1YiBmbiBsaW5lYXJfc2VhcmNoKGE6ICZWZWM8aTMyPiwgZTogaTMyKSAtPiAobjogdXNpemUpCiAgICByZXF1aXJlcwogICAgICAgIGV4aXN0c3xpOiBpbnR8ICgwIDw9IGkgPCBhLmxlbigpIGFzIGludCkgJiYgYVtpXSA9PSBlLAogICAgZW5zdXJlcwogICAgICAgIDAgPD0gbiA8IGEubGVuKCksCiAgICAgICAgYVtuIGFzIGludF0gPT0gZSwKICAgICAgICBmb3JhbGx8azogaW50fCAoMCA8PSBrIDwgbiBhcyBpbnQpID09PiBhW2tdICE9IGUsCnsKfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCnB1YiBmbiBsaW5lYXJfc2VhcmNoKGE6ICZWZWM8aTMyPiwgZTogaTMyKSAtPiB1c2l6ZSB7CiAgICBsZXQgbXV0IG46IHVzaXplID0gMDsKICAgIHdoaWxlIG4gIT0gYS5sZW4oKSB7CiAgICAgICAgaWYgYVtuXSA9PSBlIHsKICAgICAgICAgICAgcmV0dXJuIG47CiAgICAgICAgfQogICAgICAgIG4gPSBuICsgMTsKICAgIH0KICAgIG4KfQoKfSAvLyB2ZXJ1cyEK","dmVydXMhIHsKCnB1YiBmbiBsaW5lYXJfc2VhcmNoKGE6ICZWZWM8aTMyPiwgZTogaTMyKSAtPiB1c2l6ZSB7CiAgICBsZXQgbXV0IG46IHVzaXplID0gMDsKICAgIHdoaWxlIG4gIT0gYS5sZW4oKSB7CiAgICAgICAgaWYgYVtuXSA9PSBlIHsKICAgICAgICAgICAgcmV0dXJuIG47CiAgICAgICAgfQogICAgICAgIG4gPSBuICsgMTsKICAgIH0KICAgIG4KfQoKfSAvLyB2ZXJ1cyEK"]}}"#,
    );
}
