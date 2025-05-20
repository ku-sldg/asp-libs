
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, handle_appraisal_body, GlobalContext, Evidence, EvidenceT, ASP_PARAMS, EvidenceSliceRequest, EvidenceSliceResponse, rawev_to_vec, RawEv};
//use rust_am_lib::tcp::{am_sendRec_string_all};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_GoldenEvidence_Appr {
    env_var_golden: String,
    filepath_rawev_golden: String, 
    filepath_et_golden: String,
    filepath_glob_golden: String,
    attestation_aspid: String,
    attestation_targid: String
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {

    let myaspargs : ASP_ARGS_GoldenEvidence_Appr = serde_json::from_value(args)
    .context("Could not decode ASP_ARGS for ASP goldenevidence_appr")?;

    let env_var: String = myaspargs.env_var_golden;
    let rawev_filename: String = myaspargs.filepath_rawev_golden;
    let et_filename: String = myaspargs.filepath_et_golden;
    let glob_filename: String = myaspargs.filepath_glob_golden;

    let attest_id: String = myaspargs.attestation_aspid;
    let targ_id: String = myaspargs.attestation_targid;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let rawev_filename_full = format! {"{env_var_string}{rawev_filename}"};
    let et_filename_full = format! {"{env_var_string}{et_filename}"};
    let glob_filename_full = format! {"{env_var_string}{glob_filename}"};

    let glob_ctxt_filepath = glob_filename_full;
    let glob_ctxt_contents = fs::read_to_string(glob_ctxt_filepath).expect("Couldn't read glob_ctxt JSON file");
    
    let my_glob_ctxt: GlobalContext = serde_json::from_str(&glob_ctxt_contents)?;
    eprintln!("\nDecoded glob_ctxt as:");
    eprintln!("{:?}", my_glob_ctxt);


    eprintln!("Trying to read from file: {}", rawev_filename_full);
    let rawev_contents = fs::read_to_string(rawev_filename_full).expect("Couldn't read RawEv JSON file");
    let my_rawev: RawEv = serde_json::from_str(&rawev_contents)?;
        eprintln!("\nDecoded RawEv as:");
        eprintln!("{:?}", my_rawev);

    let evidencet_contents = fs::read_to_string(et_filename_full).expect("Couldn't read EvidenceT JSON file");
    let my_evidencet: EvidenceT = serde_json::from_str(&evidencet_contents)?;
        eprintln!("\nDecoded EvidenceT as:");
        eprintln!("{:?}", my_evidencet);

    let my_asp_params : ASP_PARAMS = 
    ASP_PARAMS {
        ASP_ID: attest_id,
        ASP_PLC: "".to_string(),
        ASP_TARG_ID: targ_id,
        ASP_ARGS: json!({})
    };

    let my_evidence = 
        Evidence {
            EVIDENCET: my_evidencet, 
            RAWEV: my_rawev
        };

    let vreq : EvidenceSliceRequest = 
        EvidenceSliceRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "EVSLICE".to_string(),
            GLOBAL_CONTEXT: my_glob_ctxt, 
            EVIDENCE: my_evidence,
            ASP_PARAMS: my_asp_params};

    let req_str = serde_json::to_string(&vreq)?;


    /*
    // TODO: remove these hardcodings
    let att_server_uuid_string = "127.0.0.1:5004".to_string();
    let client_uuid_string = "".to_string();

    //let stream = connect_tcp_stream(att_server_uuid_string, client_uuid_string).await?;
    */
    eprintln!("\nTrying to send EvidenceSliceRequest via FS: \n");
    eprintln!("{req_str}\n");

    let fs_path = "/Users/adampetz/Documents/Spring_2023/am-cakeml/build/bin/json_handler";


    let mut child = Command::new(fs_path)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect(format!("Failed to spawn child proces: {}", fs_path).as_str());

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    std::thread::spawn(move || {
        stdin.write_all(req_str.as_bytes()).expect("Failed to write to stdin"); 
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    let err_res = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    let res = if err_res.is_empty() {out_res} 
                       else {err_res};


    let resp_str = String::from_utf8_lossy(&res);

    //let resp_str = am_sendRec_string_all(att_server_uuid_string, client_uuid_string, req_str)?;
    eprintln!("Got a Response String from FS: \n");
    eprintln!("{resp_str}\n");

    let resp : EvidenceSliceResponse = serde_json::from_str(&resp_str)?;
    eprintln!("Decoded EvidenceSliceResponse: \n");
    eprintln!("{:?}\n", resp);

    let golden_bytes = &rawev_to_vec(resp.PAYLOAD);

    /* TODO:  handle non-singleton RawEv results here...? */
    let evidence_in = ev.first().context("No file evidence found")?;

    let candidate_ev = vec!((evidence_in.clone()));

    eprintln!("golden_bytes{:?}\n", golden_bytes);
    eprintln!("evidence_in{:?}\n", candidate_ev);

    let bytes_equal: bool = golden_bytes.eq(&candidate_ev);

    match bytes_equal {
        true => Ok(Ok(())),
        false => Ok(Err(anyhow::anyhow!("File contents do not match"))),
    }
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {
    handle_appraisal_body(body);
}
