use crate::stacks::{ Stack, StackSource, mean };
use std::process::Command;
use std::path::Path;
use anyhow::Result;

impl Stack {
    pub fn from_cli(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "mean" => Some(Stack::Mean),
            "mern" => Some(Stack::Mern),
            "angular" => Some(Stack::Angular),
            "react" => Some(Stack::React),
            "vue" => Some(Stack::Vue),
            "express" => Some(Stack::Express),
            "mongodb" => Some(Stack::MongoDB),
            "blazor" => Some(Stack::Blazor),
            "dotnet-api-with-angular" => Some(Stack::DotNetAPIWithAngular),
            "dotnet-api-with-react" => Some(Stack::DotNetAPIWithReact),
            "laravel-with-vue" => Some(Stack::LaravelWithVue),
            _ => None,
        }
    }

    pub fn source(&self) -> Option<StackSource> {
        match self {
            Stack::Mean => Some(mean::source()),
            _ => None, // Other stacks not implemented yet
        }
    }

    pub fn download_and_extract(repo_url: &str, git_ref: &str, tmp_dir: &Path) -> Result<()> {
        let status = Command::new("git")
            .args(["clone", "--depth", "1", "--branch", git_ref, repo_url, tmp_dir.to_str().unwrap()])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to clone repository: {}", repo_url);
        }
        
        Ok(())

    }

    pub fn copy_stack_files(tmp_dir: &Path, dest_dir: &Path, app_dir: &str, server_dir: &str, excluded_names: &[&str]) -> Result<()> {
        // Implementation to copy files from the extracted template to the project directory
        Ok(())
    }
}