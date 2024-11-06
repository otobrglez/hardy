use clap::{Error, Parser};
use hardy::size::Size;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArguments {
    #[arg(short, long, default_value_t=Size::Size3)]
    size: Size,
}

async fn arguments() -> Result<CliArguments, Error> {
    CliArguments::try_parse()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = arguments().await;
    if arguments.is_err() {
        eprintln!("Error parsing arguments: {}", arguments.unwrap_err());
        std::process::exit(1);
    }

    let arguments = arguments.unwrap();
    println!("Generating w/ size {}", arguments.size);
    Ok(())
}
