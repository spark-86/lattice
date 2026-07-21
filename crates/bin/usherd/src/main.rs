use crate::{
    cli::{Cli, Commands},
    config::UsherdConfig,
};
use base64::{Engine as _, engine};
use clap::Parser;
use std::{error::Error, path::PathBuf, str::FromStr};

pub mod check;
pub mod cli;
pub mod client;
pub mod config;
pub mod firing;
pub mod keys;
pub mod process;
pub mod rebuild;
pub mod receive;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = UsherdConfig::from_file(&cli.config)?;
    println!("*****************************************************");
    println!("* LATTICE USHER DAEMON                              *");
    println!("*****************************************************");
    println!("Starting what is probably a terrible excuse for a server...");
    println!("Bro, welcome to the dungeon lol... ⚔️🐉");
    match cli.command {
        Commands::Listen {
            port,
            rebuild,
            root,
            enclave,
            scopes,
            i_am,
            transform_registry,
            usher_map,
            bootstrap,
        } => {
            let config = UsherdConfig {
                root_path: root.or(Some(config.root_path)).unwrap(),
                enclave: enclave.or(Some(config.enclave)).unwrap(),
                scopes: scopes.or(Some(config.scopes)).unwrap(),
                i_am: i_am.or(Some(config.i_am)).unwrap(),
                transform_registry: transform_registry
                    .or(Some(config.transform_registry))
                    .unwrap(),
                usher_map: usher_map.or(Some(config.usher_map)).unwrap(),
                port: port.or(Some(config.port)).unwrap(),
                rebuild,
                bootstrap: bootstrap.or(Some(config.bootstrap)).unwrap(),
            };
            server::run(config).await?;
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
