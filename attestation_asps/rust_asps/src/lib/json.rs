
use serde::*;

pub fn encode_gen<T>(v: &T) -> std::io::Result<String>
    where T: ?Sized + Serialize
    {
        let maybe_string = serde_json::to_string(v);
        match maybe_string {
            Err (e) => {panic!("Error Encoding val: {e:?}")}
            Ok (s) => Ok (s)
        }
    }

pub fn decode_gen<'a, T>(s: &'a str) -> std::io::Result<T>
    where T: de::Deserialize<'a>
    {
        let maybe_val = serde_json::from_str(s);
        match maybe_val {
            Err (e) => {panic!("Error Decoding val: {e:?}")}
            Ok (v) => Ok (v)
        }
    }
