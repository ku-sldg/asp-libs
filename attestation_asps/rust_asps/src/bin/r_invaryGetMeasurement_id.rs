
// Common Packages
use rust_am_lib::copland::*;
use anyhow::{Result};
use std::env;
// use base64::{Engine as _, engine::{general_purpose}};
use base64::{engine::general_purpose, Engine as _};

use std::str;
use std::io::Read;
use std::fs::{self, DirEntry};
use std::path::{PathBuf};
use curl::easy::Easy;
use curl::easy::List;
use std::io::{Error, ErrorKind};
use std::thread;
use std::time::{Duration, UNIX_EPOCH};
use serde::{Deserialize, Serialize};


const APPRAISAL_DIR : &'static str = "/var/opt/invary-appraiser/appraisals";
//    handle.url("https://127.0.0.1:8443/api/measurements/jobs").unwrap();
const DEMAND_MEASURE_URL : &'static str = "https://localhost:8443/api/measurements/jobs";

#[derive(Serialize, Deserialize, Debug)]
pub struct InvaryMeasureCheck {
        pub id        : String,
        pub created   : String,
        pub expires   : String,
        pub endpoints : Vec<String>,
        pub hostnames : Vec<String>,
        pub tags      : Vec<String>,
        pub measured  : i64
}

fn body() -> Result<String> {
    // For every ASP, an ASPRunRequest appears as the single command-line argument
    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow::anyhow!("ASPRunRequest not supplied as command line argument"));
    }

    let json_request = &args[1];
    // May fail.  If so, return an Err Result
    // for this ASP, no information in the request is required.
    let _req: ASPRunRequest = serde_json::from_str(json_request)?;

    let measure_job_id = demand_measure("veritas")?;
    println!("ID = {:?}", measure_job_id);
    thread::sleep(Duration::new(4, 0));
    let done = check_job_complete(&measure_job_id)?;
    println!("Done = {:?}", done);

    if done {
        let path = newest_file_in_dir(APPRAISAL_DIR)?;
        println!("Appraisal file: {:?}", path);
        let bytes = std::fs::read(path)?; // Vec<u8>
        let bytes_encoded: String = general_purpose::STANDARD.encode(bytes);
        let evidence = RawEv::RawEv(vec![bytes_encoded]);
        let  response = successfulASPRunResponse (evidence);
        let response_json = serde_json::to_string(&response)?;
        Ok (response_json)
    } else {
        Err(anyhow::anyhow!("Measurement did not complete."))
    }
}

fn main() {
    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let  response = failureASPRunResponse (_error.to_string());
            // If an error occurs converting the failure response to JSON
            // there is nothing else to do but panic.
            // This should never happen.
            serde_json::to_string(&response).unwrap_or_else(|error| {panic!("Failed to json.encode failure response: {error:?}");})
        }
    };
    // The ASP output (ASPRunRequest) is written to stdout.
    // The caller will capture stdout to receive the response from this ASP.
    println!("{response_json}");
}

fn check_job_complete (job_id : &str) -> std::io::Result <bool > {
    let mut received_data = Vec::new();
    let mut url = String::from(DEMAND_MEASURE_URL);
    url.push_str("/");
    url.push_str(job_id);

    let mut handle = Easy::new();
    handle.url(url.as_str()).unwrap();
    // --insecure
    handle.ssl_verify_peer(false).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
           received_data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let response = str::from_utf8(&received_data).unwrap();
    let check_response: InvaryMeasureCheck  = serde_json::from_str(response).unwrap();
    Ok(check_response.measured > 0)
}

fn demand_measure (hostname : &str) -> std::io::Result <String > {
    let mut received_data = Vec::new();
    let mut list = List::new();
    let mut handle = Easy::new();

    handle.url(DEMAND_MEASURE_URL).unwrap();

//    for debugging connection
//    handle.verbose(true).unwrap();

    // --insecure
    handle.ssl_verify_peer(false).unwrap();

    list.append("Content-Type: application/json").unwrap();
    handle.http_headers(list).unwrap();

    let mut data = String::new();
    data.push_str("{\"hostnames\": [\"");
    data.push_str(hostname);
    data.push_str( "\"]}");
    let mut data_to_send = data.as_bytes();

    handle.post(true).unwrap();
    handle.post_field_size(data_to_send.len() as u64).unwrap();

    {
        let mut transfer = handle.transfer();
        transfer.read_function(|buf| {
            Ok(data_to_send.read(buf).unwrap_or(0))
        }).unwrap();

        transfer.write_function(|new_data| {
           received_data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let response = str::from_utf8(&received_data).unwrap();
    let check_response: InvaryMeasureCheck  = serde_json::from_str(response).unwrap();
    Ok(check_response.id)
}

fn newest_file_in_dir (dir : &str) -> std::io::Result<PathBuf> {

    let mut latest_time = UNIX_EPOCH;
    let mut latest_entry : Option<DirEntry> = None;

    let entries = fs::read_dir(dir).unwrap();

    for e in entries {
        let entry = e.unwrap();
        let meta = entry.metadata()?;
        if meta.is_file() {
            let mod_time = meta.modified().unwrap();
            if mod_time > latest_time {
                latest_time = mod_time;
                latest_entry = Some(entry);
            }
        }
    }
    match latest_entry {
        Some(entry) => Ok(entry.path().to_owned()),
        None => Err(Error::new(ErrorKind::NotFound, "No files in directory.")),
    }
}

// for debugging
// fn print_type_of<T>(_: &T) {println!("{}", std::any::type_name::<T>());}
