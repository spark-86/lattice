use clap::Parser;

pub mod cli;
pub mod convert;

fn main() {
    let cli = cli::Cli::parse();
    let _ = convert::run(cli.input, cli.output);
}
