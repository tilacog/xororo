use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use clap::{Parser, Subcommand};
use std::io::{self, Read};
use xplit::{recover_secret, split_secret};

#[derive(Parser)]
#[command(name = "xplit")]
#[command(about = "Split and recover secrets using 2-of-2 secret sharing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Split a secret into two shares
    Split {
        /// Secret to split (if not provided, reads from stdin)
        secret: Option<String>,
    },
    /// Recover a secret from two shares
    Recover {
        /// First share (base64 encoded)
        share1: String,
        /// Second share (base64 encoded)
        share2: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Split { secret } => {
            let secret_bytes = if let Some(s) = secret {
                s.into_bytes()
            } else {
                let mut buffer = Vec::new();
                io::stdin().read_to_end(&mut buffer)?;
                buffer
            };

            let shares = split_secret(&secret_bytes).expect("Failed to split secret");

            println!("Share 1: {}", BASE64.encode(&shares.share1));
            println!("Share 2: {}", BASE64.encode(&shares.share2));
        }
        Commands::Recover { share1, share2 } => {
            let share1_bytes = BASE64
                .decode(share1)
                .expect("Failed to decode share1 from base64");
            let share2_bytes = BASE64
                .decode(share2)
                .expect("Failed to decode share2 from base64");

            let recovered =
                recover_secret(&share1_bytes, &share2_bytes).expect("Failed to recover secret");

            match String::from_utf8(recovered.clone()) {
                Ok(s) => println!("{s}"),
                Err(_) => {
                    // If not valid UTF-8, output as hex
                    println!("Binary data (hex): {}", hex::encode(recovered));
                }
            }
        }
    }

    Ok(())
}
