use anyhow::Result;
use scope::rhex;

pub fn view(input: String) -> Result<()> {
    // FIXME: This needs to work with the chains as well.
    let rhex = rhex::Rhex::single_disk_get(&input);
    println!("{}", rhex.pretty_print());
    let valid = rhex.validate();
    match valid {
        true => println!("✅ Valid"),
        false => println!("❌ Invalid"),
    }
    Ok(())
}
