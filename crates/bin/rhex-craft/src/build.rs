use std::fs;

use base64::Engine as _;
use scope::rhex::{Rhex, data::RhexData, intent::RhexIntent};

pub fn build(
    prev: Option<String>,
    scope: String,
    author: String,
    usher: String,
    schema: Option<String>,
    rt: String,
    data: String,
    output: String,
) {
    // First, make sure we can get the data payload from the string
    let data: RhexData = serde_cbor::from_slice(&fs::read(data).unwrap()).unwrap();
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
        data,
    );
    println!("R⬢ crafted.");
    rhex.disk_put(&output);
    println!("Wrote to {}.", output);
}
