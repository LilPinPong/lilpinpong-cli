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
            cli::commands::init::run(name);
        }
    }
}
