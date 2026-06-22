use anyhow::Result;
use scope::rhex;

pub fn view(input: String) -> Result<()> {
    let rhex = rhex::Rhex::disk_get(&input);
    println!("{}", rhex.pretty_print());
    let valid = rhex.validate();
    match valid {
        true => println!("✅ Valid"),
        false => println!("❌ Invalid"),
    }
    Ok(())
}
