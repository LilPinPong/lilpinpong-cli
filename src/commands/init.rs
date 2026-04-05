use clap::{Args};

#[derive(Args, Debug, Clone, Default)]
pub struct InitArgs {
    #[arg(short, long, help = "Project name or path to create")]
    pub name: String,
    #[arg(short, long, value_enum, help = "Project stack to generate")]
    pub stack: String,
    #[arg(long, help = "Skip prompts and use defaults when possible")]
    pub yes: bool,
    #[arg(short, long, help = "Server framework to use")]
    pub server: String,
}

