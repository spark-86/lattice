use std::fs;

use anyhow::{Ok, Result};
use base64::Engine as _;
use scope::rhex::{Rhex, intent::RhexIntent};

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
        Some(d) => fs::read(d)?,
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
    rhex.intent = RhexIntent::build(
        prev,
        scope,
        author.try_into().unwrap(),
        usher.try_into().unwrap(),
        schema,
        rt,
        data_hash,
    );
    rhex.data = data_slice;
    println!("R⬢ crafted.");
    rhex.disk_put(&output);
    println!("Wrote to {}.", output);
    Ok(())
}
