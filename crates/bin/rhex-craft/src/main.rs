use clap::Parser;

use cli::Commands;

mod build;
mod cli;
mod genesis;
mod view;

fn main() {
    println!("Lattice R⬢ Crafting Tool {}", env!("CARGO_PKG_VERSION"));
    let cli = cli::Cli::parse();
    let enclave_path = match cli.enclave_path {
        Some(ep) => ep,
        None => "./keys".to_string(),
    };
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
        Commands::Genesis { key, output } => {
            let _ = genesis::genesis(key, enclave_path, output);
        }
        Commands::View { input } => {
            let _ = view::view(input);
        }
    }
}
