use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::stacks::{ ProjectSpec, Stack,EXCLUDED_NAMES};

pub fn generate_project(spec: &ProjectSpec) -> Result<PathBuf> {

    let root = Path::new(&spec.name);
    if root.exists() {
        bail!("❌ Folder '{}' already exists", root.display());
    }

    fs::create_dir_all(root.join("app"))?;
    fs::create_dir_all(root.join("server"))?;

    let stack_name = spec.stack.as_ref().and_then(|s: &Stack| Stack::from_cli(s)).unwrap(Stack::Mean);
    let source = stack_name.source().context("Failed to get stack source")?;
    let tmp_dir = makeTmpDir(root)?;
    Stack::download_and_extract(&source.repo_url, &source.git_ref, &tmp_dir)?;
    Stack::copy_stack_files(&tmp_dir, root, &source.app_dir, &source.server_dir, EXCLUDED_NAMES)?;
    Stack::copy_stack_files(&tmp_dir, root, &source.server_dir, &source.server_dir, EXCLUDED_NAMES)?;
    fs::remove_dir_all(&tmp_dir)?;

    println!(" ✅ Generated project directory '{}'", &spec.name);

    Ok(root.to_path_buf())
}


pub fn makeTmpDir(path: &Path) -> Result<PathBuf> {
    fs::create_dir_all(path.join("tmp"))?;
    Ok(path.join("tmp"))
}