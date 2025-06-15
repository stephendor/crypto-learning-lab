use classical_crypto::Caesar;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "caesar")]
#[command(about = "Caesar cipher encryption/decryption tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long)]
        shift: u8,
        #[arg(short, long)]
        text: String,
    },
    Decrypt {
        #[arg(short, long)]
        shift: u8,
        #[arg(short, long)]
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { shift, text } => {
            let cipher = Caesar::new(shift);
            println!("Encrypted: {}", cipher.encrypt(&text));
        }
        Commands::Decrypt { shift, text } => {
            let cipher = Caesar::new(shift);
            println!("Decrypted: {}", cipher.decrypt(&text));
        }
    }
}
