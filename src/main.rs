use std::path::PathBuf;

use clap::Parser;
use reqwest::blocking::multipart::Form;
use reqwest::blocking::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = Client::new();

    let form = Form::new()
        .text("reqtype", "fileupload")
        .file("fileToUpload", args.file.as_os_str())?;

    let response = client
        .post("https://catbox.moe/user/api.php")
        .multipart(form)
        .send()?;

    println!("URL: {:#?}", response.text().unwrap());

    Ok(())
}
