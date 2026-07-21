use clap::{Parser, Subcommand};

pub use key::Key;
pub use rhex::Rhex;

mod generate;
mod sign;
mod vanity;
mod view;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// This is the key in base64 format that we will be using
    /// to pull from the enclave
    #[arg(short, long, global = true)]
    key: Option<String>,

    #[arg(short, long, global = true)]
    enclave_path: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sign {
        /// Signature Type
        #[arg(short, long)]
        sig_type: String,

        /// Path to Rhex for signing/verifying
        #[arg(short, long, global = true)]
        input: String,

        /// Path to Rhex for signing
        #[arg(short, long, global = true)]
        output: Option<String>,

        /// Delta of how long since usher's added `at`
        #[arg(short, long)]
        delta: Option<u64>,
    },
    Generate {
        #[arg(short, long)]
        name: Option<String>,
    },
    View {
        #[arg(long)]
        secret: bool,

        #[arg(long)]
        rust: bool,
    },
    Vanity {
        #[arg(short, long)]
        sigil_prefix: String,
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() {
    println!("Lattice Key Tool v{}", env!("CARGO_PKG_VERSION"));
    let cli = Cli::parse();
    if cli.key.is_none() {
        println!("Must specify key in base64 format (-k)");
        return;
    }
    let enclave_path = match cli.enclave_path {
        Some(ep) => ep,
        None => "./keys".to_string(),
    };
    match cli.command {
        Commands::Sign {
            sig_type,
            input,
            output,
            delta,
        } => {
            let _ = sign::sign(
                &cli.key.unwrap(),
                &enclave_path,
                &sig_type,
                &input,
                &output,
                &delta,
            );
        }
        Commands::Generate { name } => {
            let _ = generate::generate(name, cli.key.unwrap());
        }
        Commands::View { secret, rust } => {
            view::view(&cli.key.unwrap(), secret, rust);
        }
        Commands::Vanity { sigil_prefix, name } => {
            vanity::vanity(sigil_prefix, name, cli.key.unwrap());
        }
    }
}
