use clap::{Parser, Subcommand};

mod cli;

#[derive(Parser)]
#[command(name = "lpp")]
#[command(about = "LilPinPong CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { name : String },
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            if let Err(e) = cli::commands::init::run(name) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
