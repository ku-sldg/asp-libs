#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Common Packages
use std::fs;
use std::env;
use std::path::{self, Path};
use anyhow::{Context, Result};
use rust_am_lib::{
    copland::{self, handle_appraisal_body, RawEv},
    debug_print
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
use serde::de::DeserializeOwned;
use serde_stacker::Deserializer;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRangeMany_Appr {
    env_var_golden: String,
    filepath_golden: String,
    outdir: String,
    report_filepath: String,
    asp_id_appr: String, 
    targ_id_appr: String
}

type Slices_Map = HashMap<String, Vec<u8>>;

type Slices_Comp_Map = HashMap<String, bool>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HAMR_AttestationReport {
    r#type: String,
    reports: Vec<HAMR_ComponentReport>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_ComponentReport {
    r#type: String,
    idPath: Vec<String>,
    classifier: Vec<String>,
    reports: Vec<HAMR_ComponentContractReport>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_ComponentContractReport {
    r#type: String,
    id: String,
    kind: String,
    meta: String,
    slices: Vec<HAMR_Slice>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_Slice {
    r#type: String,
    kind: String,
    meta: String,
    pos: HAMR_Pos
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_Pos {
    r#type: String,
    uri: String,
    beginLine: usize,
    beginCol: usize,
    endLine: usize,
    endCol: usize,
    offset: usize,
    length: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolute_Appsumm_Member {
    component:String,
    contract_id:String,
    location:String,
    meta:String,
    result:bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResoluteAppraisalSummaryResponse {
    pub TYPE: String,
    pub ACTION: String,
    pub SUCCESS: bool,
    pub APPRAISAL_RESULT: bool,
    pub PAYLOAD: Vec<Resolute_Appsumm_Member>
}

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

fn decode_from_file_and_print<T: DeserializeOwned + std::fmt::Debug + Clone>(term_fp:String, type_string:String) -> Result<T, serde_json::Error> {

     let err_string = format!("Couldn't read {type_string} JSON file");
     let term_contents = fs::read_to_string(term_fp).expect(err_string.as_str());
                                eprintln!("\n{type_string} contents:\n{term_contents}");
                                let term : T = serde_json::from_str(&term_contents)?;
                                eprintln!("\nDecoded Term as:");
                                eprintln!("{:?}", term);
                                Ok(term)
}

pub fn get_attestation_report_json (hamr_report_fp:String) -> std::io::Result<HAMR_AttestationReport>  {

    let res: HAMR_AttestationReport = decode_from_file_and_print(hamr_report_fp, "HAMR_AttestationReport".to_string())?;

    Ok (res)
}

fn relpath_to_abspath (project_root_fp:String, relpath:String) -> String {

    let root = Path::new(&project_root_fp);
    let relative = Path::new(&relpath);

    let combined_path = root.join(relative);
    
    // Normalize the path using std::path::absolute
    let normalized_absolute_path = path::absolute(&combined_path).unwrap();

    let canonnicalized_path = fs::canonicalize(normalized_absolute_path).unwrap();

    let res = canonnicalized_path.to_str().unwrap().to_string();
    res

}

fn merge_resolute_slice (root_fp:String, c:HAMR_Slice, m:Slices_Comp_Map, component_id:String, contract_id:String, hm: &mut std::collections::HashMap<String, Resolute_Appsumm_Member>) -> () {

    let pos: HAMR_Pos = c.pos.clone();
    let relative_uri = pos.uri;

    let uri_absolute = relpath_to_abspath(root_fp, relative_uri);
    let bline = c.pos.beginLine;
    let eline = c.pos.endLine;

    let bline_string= bline.to_string();
    let eline_string = eline.to_string();
    let uri_slice_string = format!("{uri_absolute}::{bline_string}-{eline_string}");

    let r = m.get(&uri_slice_string).unwrap();

    let v: Resolute_Appsumm_Member = 
        Resolute_Appsumm_Member 
            { component: component_id, contract_id: contract_id, location: uri_slice_string.clone(), meta: c.meta, result: *r };
    
    hm.entry(uri_slice_string).or_insert(v);

}

fn merge_resolute_contract (root_fp:String, c:HAMR_ComponentContractReport, m:Slices_Comp_Map, component_id:String, hm: &mut std::collections::HashMap<String, Resolute_Appsumm_Member>) -> () {


    let slices = c.slices;

    let _ : Vec<()> = slices.iter().map(|x| merge_resolute_slice (root_fp.clone(), x.clone(), m.clone(), component_id.clone(), c.id.clone(), &mut (*hm))).collect();
}

fn merge_resolute_component (root_fp:String, c:HAMR_ComponentReport, m: &Slices_Comp_Map, hm: &mut std::collections::HashMap<String, Resolute_Appsumm_Member>) -> () {

    let reports = c.reports;

    let idpath_string = c.idPath.join("::");

    let _ : Vec<()> = reports.iter().map(|x| merge_resolute_contract (root_fp.clone(), x.clone(), m.clone(), idpath_string.clone(), &mut (*hm))).collect();
}

fn merge_resolute_appsumm (root_fp:String, r:HAMR_AttestationReport, m:&Slices_Comp_Map) -> ResoluteAppraisalSummaryResponse {

    let mut hm : std::collections::HashMap<String,Resolute_Appsumm_Member> = std::collections::HashMap::new();

    let reports = r.reports;

    let _ : Vec<()> = reports.iter().map(|x| merge_resolute_component(root_fp.clone(), x.clone(), m, &mut hm)).collect();

    let targvec:  Vec<Resolute_Appsumm_Member>   = hm.into_values().collect();

     let mut res_bool = true;

    for v in targvec.clone().into_iter() {
        if ! v.result {res_bool = false; break}
    }

    let res : ResoluteAppraisalSummaryResponse = 
        ResoluteAppraisalSummaryResponse 
            { TYPE: "".to_string(), ACTION: "".to_string(), SUCCESS: true, APPRAISAL_RESULT: res_bool, PAYLOAD: targvec };
    res
}

pub fn generate_resolute_appsumm(hamr_root_dir: String, report_filename: String, comp_map:Slices_Comp_Map) -> std::io::Result<ResoluteAppraisalSummaryResponse>  {

    let attestation_report_fp = format!("{hamr_root_dir}/{report_filename}");

    let att_report: HAMR_AttestationReport = get_attestation_report_json(attestation_report_fp.clone())?;

    let resolute_appsumm = merge_resolute_appsumm(hamr_root_dir, att_report, &comp_map);

    Ok(resolute_appsumm)

}

pub fn write_string_to_output_dir (maybe_out_dir:Option<String>, fp_suffix: String, default_mid_path:String, outstring:String) -> std::io::Result<String> {

    let fp_prefix : String = match &maybe_out_dir {
        Some(fp) => {
            fp.to_string()
        }
        None => {

            let cur_dir = env::current_dir()?;
            let cur_dir_string = cur_dir.to_str().unwrap();
            let default_path = default_mid_path;
            let default_prefix: String = format!("{cur_dir_string}/{default_path}");
            default_prefix
        }
    };

    let full_req_fp = format!("{fp_prefix}/{fp_suffix}");

    fs::create_dir_all(fp_prefix)?;
    fs::write(&full_req_fp, outstring)?;
    Ok(full_req_fp)
}

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<Result<()>> {

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

    let evidence_slice = copland::do_EvidenceSlice(my_et, my_rawev, my_glob_ctxt, my_asp_params)?;

    let evidence_slice_rawev = RawEv::RawEv(evidence_slice);
    let golden_bytes = copland::rawev_to_vec(evidence_slice_rawev);

    let evidence_in = ev;

    if (golden_bytes.len() != 1) || evidence_in.len() != 1 {
        panic!("Evidence vectors have unexpected length in readfile_range_many_appr ASP");
    }

    let golden_map_encoded: &Vec<u8> = golden_bytes.first().unwrap();
    let candidate_map_encoded: &Vec<u8> = evidence_in.first().unwrap();

    let golden_map_json: Value = deserialize_deep_json(golden_map_encoded)?;
    let golden_map : Slices_Map = serde_json::from_value(golden_map_json)?;

    let candidate_map_json: Value = deserialize_deep_json(candidate_map_encoded)?;
    let candidate_map : Slices_Map = serde_json::from_value(candidate_map_json)?;

    let res_map: Slices_Comp_Map = get_slices_comp_map(golden_map, candidate_map);


    let resolute_appsumm_response: ResoluteAppraisalSummaryResponse = generate_resolute_appsumm(myaspargs.outdir, myaspargs.report_filepath, res_map.clone())?;

    let resolute_appsumm_resp_string = serde_json::to_string(&resolute_appsumm_response)?;
    let appsumm_resp_suffix = "appsumm_response.json".to_string();
    let appsumm_resp_mid_path = "testing/outputs/".to_string();
    let _ = write_string_to_output_dir(None, appsumm_resp_suffix, appsumm_resp_mid_path.clone(), resolute_appsumm_resp_string.clone())?;

    let mut res_bool = true;

    for (_k,v) in res_map.into_iter() {
        if ! v {res_bool = false; break}
    }

    match res_bool {
        true => {
            Ok(Ok(()))
        },
        false => Ok(Err(anyhow::anyhow!("At least one file slice bytes contents do not match in readfile_range_many_appr ASP"))),
    }

}

fn main() {
    handle_appraisal_body(body);

}
