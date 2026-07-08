use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "usherd")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "config.json")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Listen {
        #[arg(short, long)]
        port: Option<u16>,

        #[arg(short, long)]
        rebuild: bool,

        #[arg(long)]
        root: Option<String>,

        #[arg(long)]
        enclave: Option<String>,

        #[arg(long)]
        scopes: Option<String>,

        #[arg(long)]
        i_am: Option<String>,

        #[arg(long)]
        transform_registry: Option<String>,

        #[arg(long)]
        usher_map: Option<String>,

        #[arg(long)]
        bootstrap: Option<String>,
    },
    Send {
        #[arg(short, long)]
        usher: String,

        #[arg(short, long)]
        rhex_file: PathBuf,

        #[arg(long)]
        usher_map: Option<PathBuf>,
    },
}
