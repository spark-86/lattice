use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long, default_value = "./output.rdata")]
    pub output: String,
}
