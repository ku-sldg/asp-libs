#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use std::fs;
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_appraisal_body, RawEv},
    debug_print
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
use serde_stacker::Deserializer;
use std::collections::HashMap;



pub const DEFAULT_TEMP_COMP_MAP_FILENAME: &'static str = "comp_map_temp.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRangeMany_Appr {
    env_var_golden: String,
    filepath_golden: String,
    outdir: String,
    asp_id_appr: String, 
    targ_id_appr: String
}

type Slices_Map = HashMap<String, Vec<u8>>;

type Slices_Comp_Map = HashMap<String, bool>;

fn deserialize_deep_json(json_data: &Vec<u8>) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_slice(json_data);
    de.disable_recursion_limit(); // This method is only available with the feature
    
    // Wrap with serde_stacker's Deserializer to use a dynamically growing stack
    let stacker_de = Deserializer::new(&mut de);
    
    // Deserialize the data
    let value = Value::deserialize(stacker_de)?;
    
    Ok(value)
}

fn deserialize_deep_json_string(json_data: &str) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_str(json_data);
    de.disable_recursion_limit(); // This method is only available with the feature
    
    // Wrap with serde_stacker's Deserializer to use a dynamically growing stack
    let stacker_de = Deserializer::new(&mut de);
    
    // Deserialize the data
    let value = Value::deserialize(stacker_de)?;
    
    Ok(value)
}

fn get_slices_comp_map (golden_map:Slices_Map, candidate_map:Slices_Map) -> Slices_Comp_Map {

    let mut res : Slices_Comp_Map = HashMap::new();

    for(k,golden_bytes) in golden_map.into_iter()
    {
        if candidate_map.contains_key(&k)
        {
            let candidate_bytes = candidate_map.get(&k).unwrap();
            let bytes_equal: bool = golden_bytes.eq(candidate_bytes);
            res.insert(k, bytes_equal);

        }
        else {
            res.insert(k, false);
        }

    };

    res

}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {
    //panic!("--------BEGIN body() in goldenevidence_appr ASP");

    let myaspargs: ASP_ARGS_ReadfileRangeMany_Appr = serde_json::from_value(args)
        .context("Could not parse ASP_ARGS for ASP readfile_range_many_appr")?;

    // Code for specific for this ASP.
    let env_var: String = myaspargs.env_var_golden;
    let filename: String = myaspargs.filepath_golden;

    let env_var_string = rust_am_lib::copland::get_env_var_val(env_var)?;

    let filename_full = format! {"{env_var_string}{filename}"};

    let contents = fs::read_to_string(filename_full).expect("Couldn't read (Evidence, GlobalContext) JSON file in goldenevidence_appr");
    debug_print!{"\n\nAttempting to decode (Evidence, GlobalContext)...\n\n"};
    let my_contents_val = deserialize_deep_json_string(&contents)?;
    let my_contents: (copland::Evidence, copland::GlobalContext) = from_value(my_contents_val)?;//serde_json::from_str(&contents)?;
    eprintln!("\nDecoded (Evidence, GlobalContext) as:");
    eprintln!("{:?}", my_contents);

    let my_evidence: copland::Evidence = my_contents.0;
    let my_glob_ctxt: copland::GlobalContext = my_contents.1;

    let my_asp_params: copland::ASP_PARAMS = copland::ASP_PARAMS{ ASP_ID: myaspargs.asp_id_appr, ASP_ARGS: serde_json::Value::Null, ASP_PLC: "".to_string(), ASP_TARG_ID: myaspargs.targ_id_appr};

    let my_et = copland::get_et(my_evidence.clone());
    let my_rawev= copland::get_rawev(my_evidence);

    //panic!("--------BEFORE do_EvidenceSlice() in goldenevidence_appr ASP");

    let evidence_slice = copland::do_EvidenceSlice(my_et, my_rawev, my_glob_ctxt, my_asp_params)?;

    let evidence_slice_rawev = RawEv::RawEv(evidence_slice);
    let golden_bytes = copland::rawev_to_vec(evidence_slice_rawev);

    let evidence_in = ev;

    if (golden_bytes.len() != 1) || evidence_in.len() != 1 {
        panic!("Evidence vectors have unexpected length in readfile_range_many_appr ASP");
        //return Ok(Err(anyhow::anyhow!("Evidence vectors have unexpected length in readfile_range_many_appr ASP")))
    }

    let golden_map_encoded: &Vec<u8> = golden_bytes.first().unwrap();
    let candidate_map_encoded: &Vec<u8> = evidence_in.first().unwrap();

    let golden_map_json: Value = deserialize_deep_json(golden_map_encoded)?;
    let golden_map : Slices_Map = serde_json::from_value(golden_map_json)?;

    let candidate_map_json: Value = deserialize_deep_json(candidate_map_encoded)?;
    let candidate_map : Slices_Map = serde_json::from_value(candidate_map_json)?;

    let res_map: Slices_Comp_Map = get_slices_comp_map(golden_map, candidate_map);

    let out_string = serde_json::to_string(&res_map)?;
    let dir = myaspargs.outdir;
    let full_comp_map_fp: String = format!("{dir}/{DEFAULT_TEMP_COMP_MAP_FILENAME}");
    fs::write(&full_comp_map_fp, out_string)?;

    let mut res_bool = true;

    for (_k,v) in res_map.into_iter() {
        if ! v {res_bool = false; break}
    }

    match res_bool {
        true => {
            //panic!("--------END match bytes_equal in goldenevidence_appr ASP");
            Ok(Ok(()))
        },
        false => Ok(Err(anyhow::anyhow!("At least one file slice bytes contents do not match in readfile_range_many_appr ASP"))),
    }


}

fn main() {
    //panic!("--------BEFORE handle_appraisal_body() in goldenevidence_appr ASP");
    handle_appraisal_body(body);

}
