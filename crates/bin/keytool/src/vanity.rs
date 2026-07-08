use std::fs;

pub fn vanity(sigil_prefix: String, name: Option<String>, output: String) {
    let mut checkpoint: u64 = 0;
    let mut done = false;
    let sigil_prefix = sigil_prefix.to_uppercase();
    while !done {
        let key = key::Key::new(rand::random(), name.clone());
        let sigil_id = key.sigid_id();
        if sigil_id.starts_with(&sigil_prefix) {
            println!("Sigil ID: {}", sigil_id);
            println!("Writing key to {}", output);
            let _ = fs::write(output.clone(), &key.to_vec());
            done = true;
        } else {
            checkpoint += 1;
            if checkpoint % 10_000 == 0 {
                println!("Checkpoint {}: {}", checkpoint, sigil_id);
            }
        }
    }
}
