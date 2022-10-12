use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
}
