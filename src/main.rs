mod commands;
mod engines;
mod stacks;

use clap::{Parser, Subcommand};
use commands::init::{InitArgs};

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

fn main() {
    let cli = Cli::parse();
}
