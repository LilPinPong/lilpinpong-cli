use crate::stacks::EXCLUDED_NAMES;
use crate::stacks::{Stack, StackSource, mean};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
            "dotnet-angular" => Some(Stack::DotNetAPIWithAngular),
            "dotnet-react" => Some(Stack::DotNetAPIWithReact),
            "laravel-vue" => Some(Stack::LaravelWithVue),
            _ => None,
        }
    }

    pub fn source(&self) -> Option<StackSource> {
        match self {
            Stack::Mean => Some(mean::source()),
            _ => None, // Other stacks not implemented yet
        }
    }

    fn try_clone(repo_url: &str, git_ref: &str, target: &Path) -> Result<bool> {
        let status = Command::new("git")
            .args(["clone", "--depth", "1", "--branch", git_ref, repo_url])
            .arg(target)
            .status()
            .context("failed to run git clone")?;

        Ok(status.success())
    }

    pub fn download_repo(
        repo_url: &str,
        git_ref: &str,
        fallback_ref: &str,
        tmp_dir: &Path,
    ) -> Result<PathBuf> {
        let target = tmp_dir.join("repo");

        if Self::try_clone(repo_url, git_ref, &target)? {
            return Ok(target);
        }

        let _ = fs::remove_dir_all(&target);

        if git_ref != fallback_ref && Self::try_clone(repo_url, fallback_ref, &target)? {
            eprintln!(
                "Ref '{}' introuvable, fallback sur '{}'",
                git_ref, fallback_ref
            );
            return Ok(target);
        }

        bail!(
            "Impossible de cloner '{}': refs testées '{}' puis '{}'",
            repo_url,
            git_ref,
            fallback_ref
        );
    }

    pub fn copy_stack_files(
        repo_root: &Path,
        project_root: &Path,
        app_dir: &str,
        server_dir: &str,
    ) -> Result<()> {
        let src_app = repo_root.join(app_dir);
        let src_server = repo_root.join(server_dir);

        let dest_app = project_root.join("app");
        let dest_server = project_root.join("server");

        Self::copy_dir(&src_app, &dest_app)?;
        Self::copy_dir(&src_server, &dest_server)?;
        Ok(())
    }

    fn copy_dir(from: &Path, to: &Path) -> Result<()> {
        if !from.exists() {
            bail!("source path does not exist: {}", from.display());
        }

        fs::create_dir_all(to)?;

        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let entry_name = entry.file_name().to_string_lossy().to_string();

            if EXCLUDED_NAMES.contains(&entry_name.as_str()) || entry_name.ends_with(".log") {
                continue;
            }

            let src = entry.path();
            let dst = to.join(&entry_name);

            if src.is_dir() {
                Self::copy_dir(&src, &dst)?;
            } else {
                fs::copy(&src, &dst).with_context(|| {
                    format!("failed to copy '{}' -> '{}'", src.display(), dst.display())
                })?;
            }
        }

        Ok(())
    }
}
