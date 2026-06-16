use clap::{Parser, Subcommand};

mod build;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        prev: Option<String>,
        scope: String,
        author: String,
        usher: String,
        schema: Option<String>,
        rt: String,
        data: String,
        output: String,
    },
}

fn main() {
    println!("Lattice R⬢ Crafting Tool {}", env!("CARGO_PKG_VERSION"));
    let cli = Cli::parse();
    match cli.command {
        Commands::Build {
            prev,
            scope,
            author,
            usher,
            schema,
            rt,
            data,
            output,
        } => {
            build::build(prev, scope, author, usher, schema, rt, data, output);
        }
    }
}
