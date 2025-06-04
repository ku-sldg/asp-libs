// Common Packages
use anyhow::{Context, Result};
use curl::easy::Easy;
use curl::easy::List;
use serde::{Deserialize, Serialize};
use std::fs::{self, DirEntry};
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str;
use std::thread;
use std::time::{Duration, UNIX_EPOCH};

//const APPRAISAL_DIR: &'static str = "/var/opt/invary-appraiser/appraisals";
//    handle.url("https://127.0.0.1:8443/api/measurements/jobs")?;
const DEMAND_MEASURE_URL: &'static str = "https://localhost:8443/api/measurements/jobs";

#[derive(Serialize, Deserialize, Debug)]
pub struct InvaryMeasureCheck {
    pub id: String,
    pub created: String,
    pub expires: String,
    pub endpoints: Vec<String>,
    pub hostnames: Vec<String>,
    pub tags: Vec<String>,
    pub measured: i64,
}

use rust_am_lib::copland::{self, handle_body};

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let dynamic_arg_val = args
        .get("dynamic")
        .context("'dynamic' argument not provided to ASP, invary_get_measurement")?;

    let appraisaldir_arg_val = args
        .get("appraisal-dir")
        .context("'appraisal-dir' argument not provided to ASP, invary_get_measurement")?;

    if dynamic_arg_val.is_string() && appraisaldir_arg_val.is_string()
    {
        let dynamic_arg_val_string: String = dynamic_arg_val.to_string();
        let appraisaldir_arg_val_string: String = appraisaldir_arg_val.to_string();
        let true_val_string: String = "true".to_string();
        let dynamic_arg_bool: bool = dynamic_arg_val_string.eq(&true_val_string);

        if dynamic_arg_bool {
            eprint!("\nRequesting dynamic KIM measurement...\n\n");

            let measure_job_id = demand_measure("veritas")?;
            thread::sleep(Duration::new(10, 0));
            let done = check_job_complete(&measure_job_id)?;

            if done {
                eprint!(
                    "Reading latest KIM appraisal from directory: {}\n",
                    appraisaldir_arg_val
                );
                let path = newest_file_in_dir(appraisaldir_arg_val_string.as_str())?;
                let bytes = std::fs::read(path)?; // Vec<u8>
                Ok(vec![bytes])
            } else {
                Err(anyhow::anyhow!("Measurement did not complete."))
            }
        } else {
            eprint!("\nSkipping Request for dynamic KIM measurement...\n\n");
            eprint!(
                "\nReading latest KIM appraisal from directory: {}\n\n",
                appraisaldir_arg_val
            );
            let path = newest_file_in_dir(appraisaldir_arg_val_string.as_str())?;
            let bytes = std::fs::read(path)?; // Vec<u8>
            Ok(vec![bytes])
        }
    }
    else {
        Err(anyhow::anyhow!("Failed to decode both 'dynamic' and 'appraisal-dir' ASP args as JSON Strings in invary_get_measurement ASP"))
    }
}

fn main() {
    handle_body(body);
}

fn check_job_complete(job_id: &str) -> std::io::Result<bool> {
    let mut received_data = Vec::new();
    let mut url = String::from(DEMAND_MEASURE_URL);
    url.push_str("/");
    url.push_str(job_id);

    let mut handle = Easy::new();
    handle.url(url.as_str())?;
    // --insecure
    handle.ssl_verify_peer(false)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            received_data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let response = str::from_utf8(&received_data).unwrap();
    let check_response: InvaryMeasureCheck = serde_json::from_str(response)?;
    Ok(check_response.measured > 0)
}

fn demand_measure(hostname: &str) -> std::io::Result<String> {
    let mut received_data = Vec::new();
    let mut list = List::new();
    let mut handle = Easy::new();

    handle.url(DEMAND_MEASURE_URL)?;
    // --insecure
    handle.ssl_verify_peer(false)?;

    list.append("Content-Type: application/json")?;
    handle.http_headers(list)?;

    let mut data = String::new();
    data.push_str("{\"hostnames\": [\"");
    data.push_str(hostname);
    data.push_str("\"]}");
    let mut data_to_send = data.as_bytes();

    handle.post(true)?;
    handle.post_field_size(data_to_send.len() as u64)?;
    {
        let mut transfer = handle.transfer();
        transfer.read_function(|buf| Ok(data_to_send.read(buf).unwrap_or(0)))?;

        transfer.write_function(|new_data| {
            received_data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let response = str::from_utf8(&received_data).unwrap();
    let check_response: InvaryMeasureCheck = serde_json::from_str(response)?;
    Ok(check_response.id)
}

fn newest_file_in_dir(dir: &str) -> std::io::Result<PathBuf> {
    let mut latest_time = UNIX_EPOCH;
    let mut latest_entry: Option<DirEntry> = None;

    let entries = fs::read_dir(dir)?;

    for e in entries {
        let entry = e?;
        let meta = entry.metadata()?;
        if meta.is_file() {
            let mod_time = meta.modified()?;
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
