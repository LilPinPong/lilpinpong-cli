mod commands;
mod engines;
mod stacks;

use anyhow::Result;
use clap::{Parser, Subcommand};

use commands::init::{run_init, InitArgs};

#[derive(Parser)]
#[command(
    name = "lpp",
    version,
    about = "Lilpinpong CLI",
    long_about = "lpp is a CLI tool that generates projects from templates. It supports multiple languages and frameworks, making it easy to kickstart your development process."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init(args)) => run_init(args)?,
        None => {
            println!("No command provided. Use 'lpp init' to create a project.");
        }
    }

    Ok(())
}
