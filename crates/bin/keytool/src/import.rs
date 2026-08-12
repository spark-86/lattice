use base64::engine::{Engine as _, general_purpose::URL_SAFE_NO_PAD};

pub fn import(enclave_path: String, name: Option<String>, input: String) {
    let mut enclave = key::enclave::Enclave::new(Some(enclave_path.clone()));
    let _ = enclave.populate();
    let secret_key = URL_SAFE_NO_PAD.decode(input).unwrap();
    let key = enclave
        .import(secret_key.try_into().unwrap(), name)
        .unwrap();
    println!("{}", key.pretty_format(true));
}
