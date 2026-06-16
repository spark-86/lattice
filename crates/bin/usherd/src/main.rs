use crate::cli::{Cli, Commands};
use base64::{Engine as _, engine};
use clap::Parser;
use std::{error::Error, path::PathBuf, str::FromStr};

pub mod cli;
pub mod client;
pub mod rebuild;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("*****************************************************");
    println!("* LATTICE USHER DAEMON                              *");
    println!("*****************************************************");
    println!("Starting what is probably a terrible excuse for a server...");
    println!("Bro, welcome to the dungeon lol... ⚔️🐉");
    match cli.command {
        Commands::Listen { port, rebuild } => {
            server::run(port, rebuild).await?;
        }
        Commands::Send {
            usher,
            rhex_file,
            usher_map,
        } => {
            let usher_key = engine::general_purpose::URL_SAFE_NO_PAD.decode(usher)?;
            let usher_map = match usher_map {
                Some(usher_map) => usher_map,
                None => PathBuf::from_str("./ushers.cbor").unwrap(),
            };
            client::run(usher_key.try_into().unwrap(), rhex_file, usher_map).await?;
        }
    }
    Ok(())
}
