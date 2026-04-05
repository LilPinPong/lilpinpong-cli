use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};

use crate::stacks::{ ProjectSpec, Stack};

pub fn generate_project(spec: &ProjectSpec) -> Result<PathBuf> {

    let root = Path::new(&spec.name);
    if root.exists() {
        bail!("❌ Folder '{}' already exists", root.display());
    }

    fs::create_dir_all(root.join("app"))?;
    fs::create_dir_all(root.join("server"))?;

    let stack = spec.stack.unwrap_or(Stack::Mean);
    let source = stack.source().context("Failed to get stack source")?;
    let tmp_dir = self::make_tmp_dir()?;
    let repo_root = Stack::download_repo(&source.repo_url, &source.git_ref, "main", &tmp_dir)?;
    Stack::copy_stack_files(&repo_root, root, &source.app_dir, &source.server_dir)?;

    
    let _ = TmpDirGuard(tmp_dir.clone());
    println!(" ✅ Generated project directory '{}'", &spec.name);
    Ok(root.to_path_buf())
}


fn make_tmp_dir() -> Result<PathBuf> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let dir = std::env::temp_dir().join(format!("lpp-tmp-{}-{}", std::process::id(), ts));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

struct TmpDirGuard(PathBuf);

impl Drop for TmpDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}