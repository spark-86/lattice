use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "usherd")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Listen {
        #[arg(short, long, default_value_t = 1984)]
        port: u16,

        #[arg(long)]
        rebuild: bool,
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
