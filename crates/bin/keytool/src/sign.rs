use rhex::signature::RhexSignature;

pub fn sign(keyfile: &String, sig_type: &String, rhexfile: &String, output: &Option<String>) {
    let mut rhex = rhex::Rhex::disk_get(rhexfile);
    let sig_type = match sig_type.as_str() {
        "author" => rhex::signature::RhexSignatureType::Author,
        "usher" => rhex::signature::RhexSignatureType::Usher,
        "quorum" => rhex::signature::RhexSignatureType::Quorum,
        "observer" => rhex::signature::RhexSignatureType::Observer,
        "other" => rhex::signature::RhexSignatureType::Other,
        _ => {
            println!("Invalid signature type");
            return;
        }
    };
    let hash = rhex.get_hash(sig_type.clone());
    let key = key::Key::disk_get(keyfile);
    let sig = key.sign(&hash);
    let output = match output {
        Some(o) => o,
        None => rhexfile,
    };
    rhex.sigs.push(RhexSignature {
        pk: key.pk.unwrap(),
        sig,
        t: sig_type.clone(),
    });
    rhex.disk_put(output);
}
