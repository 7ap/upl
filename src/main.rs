use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    file: PathBuf,

    /// Upload destination
    destination: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = args.file;
    let destination = args.destination;

    if !file.exists() {
        panic!("File does not exist.")
    }

    if !destination.exists() {
        panic!("Destination does not exist.")
    }

    Ok(())
}
