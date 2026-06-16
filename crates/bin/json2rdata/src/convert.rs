use std::{fs, path::PathBuf};

use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use lattice::rhex;

pub fn run(input: String, output: String) {
    println!("Converting {} to {}", input, output);
    let json_str = fs::read_to_string(input).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    println!("data_type: {}", json["data_type"]);
    let data_type = json["data_type"].as_str().unwrap();
    let rhex_data = match data_type {
        "none" => rhex::data::RhexData::None,
        "json" => rhex::data::RhexData::Json(json["data"].clone()),
        "binary" => {
            let data = resolve_bytes(json["data"].as_str().unwrap()).unwrap();
            rhex::data::RhexData::Binary { data }
        }
        "mixed" => {
            let meta = json["meta"].clone();
            let binary = json["binary"]
                .as_array()
                .unwrap()
                .iter()
                .map(|b| resolve_bytes(b.as_str().unwrap()).unwrap())
                .collect();
            rhex::data::RhexData::Mixed { meta, binary }
        }
        _ => panic!("Unknown data type: {}", data_type),
    };

    println!("rhex_data: {:?}", rhex_data);
    fs::write(output, serde_cbor::to_vec(&rhex_data).unwrap()).unwrap();
    println!("Done!");
}

fn resolve_bytes(input: &str) -> Result<Vec<u8>> {
    let path = PathBuf::from(input);
    if path.exists() && path.is_file() {
        let bytes = std::fs::read(path)?;
        return Ok(bytes);
    };

    let clean_input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    Ok(URL_SAFE_NO_PAD.decode(clean_input.as_bytes())?)
}
