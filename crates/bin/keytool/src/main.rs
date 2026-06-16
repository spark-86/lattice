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
    /// Keyfile to work with. On signing it's used to pull a key,
    /// on generation it serves as an output
    #[arg(short, long, global = true)]
    keyfile: Option<String>,

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
    if cli.keyfile.is_none() {
        println!("Must specify keyfile (-k)");
        return;
    }

    match cli.command {
        Commands::Sign {
            sig_type,
            input,
            output,
        } => {
            sign::sign(&cli.keyfile.unwrap(), &sig_type, &input, &output);
        }
        Commands::Generate { name } => {
            generate::generate(name, cli.keyfile.unwrap());
        }
        Commands::View { secret, rust } => {
            view::view(&cli.keyfile.unwrap(), secret, rust);
        }
        Commands::Vanity { sigil_prefix, name } => {
            vanity::vanity(sigil_prefix, name, cli.keyfile.unwrap());
        }
    }
}
