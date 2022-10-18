use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    file: PathBuf,

    /// Configuration file (.toml or .sxcu)
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = args.file;
    let config = args.config;

    Ok(())
}
