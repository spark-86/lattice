use std::fs;

use anyhow::{Ok, Result};
use base64::Engine as _;
use scope::rhex::{Rhex, data::RhexData, intent::RhexIntent};

pub fn build(
    prev: Option<String>,
    scope: String,
    author: String,
    usher: String,
    schema: Option<String>,
    rt: String,
    data: Option<String>,
    output: String,
) -> Result<()> {
    // First, make sure we can get the data payload from the string
    let data_slice = match data {
        Some(d) => {
            let dat = fs::read(d);

            if dat.is_err() {
                anyhow::bail!("Can't read data file");
            }
            dat.unwrap()
        }
        None => vec![],
    };
    let data_hash = if data_slice.len() > 0 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&data_slice);
        Some(hasher.finalize().as_bytes().clone())
    } else {
        None
    };
    if data_slice.len() > 0 {
        let mut data_hash = blake3::Hasher::new();
        data_hash.update(&data_slice);
    }
    let mut rhex = Rhex::new();
    let prev = match prev {
        // Take the base64 of the previous hash and decode it
        Some(p) => Some(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(p)
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        None => None,
    };
    // Same for author's and usher's public keys
    let author = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(author)
        .unwrap();
    let usher = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(usher)
        .unwrap();
    let schema = match &schema {
        Some(s) => {
            let it = s.as_str();
            Some(it)
        }
        None => None,
    };
    let data: Option<RhexData> = match &data_hash {
        Some(_) => Some(minicbor::decode(&data_slice).unwrap()),
        None => None,
    };

    rhex.intent = RhexIntent::build(
        prev,
        scope.as_str(),
        author.try_into().unwrap(),
        usher.try_into().unwrap(),
        schema,
        rt.as_str(),
        data_hash,
    );
    rhex.data = match data {
        Some(d) => d,
        None => RhexData::None,
    };
    println!("R⬢ crafted.");
    rhex.disk_put(&output);
    println!("Wrote to {}.", output);
    Ok(())
}
