use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[arg(long)]
        prev: Option<String>,
        #[arg(long)]
        scope: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        usher: String,
        #[arg(long)]
        schema: Option<String>,
        #[arg(long)]
        rt: String,
        #[arg(long)]
        data: String,
        output: String,
    },
    Genesis {
        key: String,
        output: String,
    },
    View {
        input: String,
    },
}
