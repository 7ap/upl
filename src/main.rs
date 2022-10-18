use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    file: PathBuf,

    /// Upload destination
    destination: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = args.file;
    let destination = args.destination;

    if !file.exists() {
        panic!("File does not exist.")
    }

    if destination.is_some() {
        let destination = destination.as_ref().unwrap();

        if destination.exists() {
            panic!("Destination does not exist.")
        }

        unimplemented!("Custom destinations are WIP.")
    }

    Ok(())
}
