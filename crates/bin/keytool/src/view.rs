use std::fs;

use key::Key;

pub fn view(path: &String, show_secret: bool, show_rust: bool) {
    let key = fs::read(path).unwrap();
    let key: Key = serde_cbor::from_slice(&key).unwrap();
    println!("{}", key.pretty_format(show_secret));
    if show_rust {
        println!("Rust Array: {:?}", key.pk.unwrap());
    }
}
