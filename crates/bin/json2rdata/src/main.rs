use clap::Parser;

pub mod cli;
pub mod convert;

fn main() {
    let cli = cli::Cli::parse();
    convert::run(cli.input, cli.output);
}
