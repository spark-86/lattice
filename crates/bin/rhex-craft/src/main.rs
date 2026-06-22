use clap::Parser;

use cli::Commands;

mod build;
mod cli;
mod genesis;
mod view;

fn main() {
    println!("Lattice R⬢ Crafting Tool {}", env!("CARGO_PKG_VERSION"));
    let cli = cli::Cli::parse();
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
            let _ = genesis::genesis(key, output);
        }
        Commands::View { input } => {
            let _ = view::view(input);
        }
    }
}
