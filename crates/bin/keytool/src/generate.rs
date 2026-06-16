use base64::{Engine as _, engine};
use key;

pub fn generate(name: Option<String>, output: String) {
    println!("Generating key...");
    let key: [u8; 32] = rand::random();
    let key = key::Key::new(key, name);
    let base64_pk = engine::general_purpose::URL_SAFE_NO_PAD.encode(key.pk.unwrap());
    println!("Public Key: {}", base64_pk);
    println!("SigilID: {}", key.sigid_id());
    println!("Writing key to {}", output);
    key.disk_put(&output);
}
