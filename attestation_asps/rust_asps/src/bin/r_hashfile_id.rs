use rust_am_lib::copland::*;
use data_encoding::BASE64;
use anyhow::{Context, Result};
use std::env;

use sha2::{Sha256, Digest};

fn body() -> Result<String> {

    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        Err(anyhow::anyhow!("ASPRunRequest not supplied as command line argument"))
    } else {

        let json_request = &args[1];

        let req: ASPRunRequest = serde_json::from_str(json_request)?;

        let args_map = req.ASP_ARGS;
        let filename = &args_map.get("filepath").context("filename argument not provided to ASP, hashfile_id")?;

        let bytes = std::fs::read(filename)?; // Vec<u8>

        let hash = Sha256::digest(&bytes);

        let hash_string = format!("{:?}", hash);
        let hash_b64: String = BASE64.encode(&hash_string.into_bytes());

        let evidence = RawEv::RawEv(vec![hash_b64]);

        let  response = ASPRunResponse { TYPE: "RESPONSE".to_string(),
                                         ACTION: "ASP_RUN".to_string(),
                                         SUCCESS: true,
                                         PAYLOAD:  evidence};
        let response_json = serde_json::to_string(&response)?;
        Ok (response_json)
    }
}

fn main() -> () {

    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let  response = ASPRunResponse { TYPE: "RESPONSE".to_string(),
                                             ACTION: "ASP_RUN".to_string(),
                                             SUCCESS: false,
                                             PAYLOAD:  RawEv::RawEv(Vec::new())};
            serde_json::to_string(&response).unwrap_or_else(|error| {panic!("Failed to json.encode failure response: {error:?}");})
        }
    };

    println!("{response_json}");
    ()
}
