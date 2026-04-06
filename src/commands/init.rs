use clap::{Args};
use dialoguer::Input;
use anyhow::{Result, bail};

use crate::stacks::{ ProjectSpec, Stack, ServerSpec};
use crate::engines::generator::generate_project;

#[derive(Args, Debug, Clone, Default)]
pub struct InitArgs {
    #[arg(short, long, help = "Project name or path to create")]
    pub name: Option<String>,
    #[arg(short, long, help = "Project stack to generate")]
    pub stack: Option<String>,
    #[arg(long, help = "Server framework to use")]
    pub server: Option<String>,
    #[arg(long, help = "Skip prompts and use defaults when possible")]
    pub yes: bool,
}

pub fn run_init(args: &InitArgs) -> Result<()> {
    let yes = args.yes;
    let name = resolve_name(&args, yes)?;
    let stack = resolve_stack(&args, yes)?;

    let spec = ProjectSpec {
        name,
        stack: Some(stack),
        server: args.server.clone().map(|s| ServerSpec { framework: s }),
        yes,
    };

    let project_root = generate_project(&spec)?;

    println!("Project created at '{}'", project_root.display());
    println!("Frontend path: '{}/app'", spec.name);
    println!("Backend path: '{}/server'", spec.name);

    Ok(())
}


fn resolve_name(args: &InitArgs, yes: bool) -> Result<String> {
    match (&args.name, yes) {
        (Some(name), _) => normalize_name(name),
        (None, true) => Ok(String::from("my-mean-app")),
        (None, false) => {
            let input: String = Input::new()
                .with_prompt("Project name")
                .default(String::from("my-mean-app"))
                .interact_text()?;
            normalize_name(&input)
        }
    }
}


fn resolve_stack(args: &InitArgs, yes: bool) -> Result<Stack> {
    let raw_value = match (&args.stack, yes) {
        (Some(stack), _) => stack.trim().to_string(),
        (None, true) => String::from("mean"),
        (None, false) => Input::new()
            .with_prompt("Stack")
            .default(String::from("mean"))
            .interact_text()?,
    };

    match Stack::from_cli(raw_value.trim()) {
        Some(stack) => Ok(stack),
        None => bail!(
            "Invalid stack '{}'. Supported now: mean",
            raw_value.trim()
        ),
    }
}

fn normalize_name(name: &str) -> Result<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        bail!("Project name cannot be empty");
    }
    Ok(trimmed.to_string())
}