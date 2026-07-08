use anyhow::Result;
use key::enclave::Enclave;

pub fn generate(name: Option<String>, output: String) -> Result<()> {
    println!("Generating key...");
    let mut enclave = Enclave::new(Some(output.clone()));
    let key = enclave.generate(name)?;
    println!("{}", key.pretty_format(true));
    Ok(())
}
