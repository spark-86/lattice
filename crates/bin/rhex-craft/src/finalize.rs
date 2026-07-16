use std::fs;

use anyhow::Result;
use scope::rhex::Rhex;

pub fn finalize(input: String, output: String, use_curr: bool) -> Result<()> {
    let file = fs::read(input)?;
    let mut rhex: Rhex = minicbor::decode(&file)?;
    rhex.curr = Some(rhex.calc_curr());
    println!("R⬢ Finalized!");
    println!("{}", rhex.pretty_print());
    let mut buf = Vec::new();
    let filename = if use_curr {
        format!("{}/{}.rhex", output, hex::encode(rhex.curr.unwrap()))
    } else {
        output.clone()
    };
    minicbor::encode(rhex, &mut buf)?;
    fs::write(filename, buf)?;
    Ok(())
}
