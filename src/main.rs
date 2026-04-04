use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lpp", version, about = "Lilpinpong CLI", long_about = "lpp is a CLI tool that generates projects from templates. It supports multiple languages and frameworks, making it easy to kickstart your development process.")]
struct Cli {
    path: Option<PathBuf>
}


fn main() {
    let cli: Cli = Cli::parse();
}
