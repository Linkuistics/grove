use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolve the repo path: if `arg` is Some, use it; otherwise use cwd's git root.
pub fn resolve(arg: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = arg {
        if !p.join(".git").exists() && git_common_dir(p).is_err() {
            anyhow::bail!("not a git repo: {}", p.display());
        }
        return Ok(p.to_path_buf());
    }
    // Use cwd.
    let cwd = std::env::current_dir().context("getting cwd")?;
    let common_dir = git_common_dir(&cwd)?;
    // common_dir points to .git or worktrees/<name>/.git; the main repo
    // is its parent.
    let main_repo = common_dir
        .parent()
        .ok_or_else(|| anyhow!("git common-dir has no parent"))?
        .to_path_buf();
    Ok(main_repo)
}

fn git_common_dir(cwd: &Path) -> Result<PathBuf> {
    let out = Command::new("git")
        .arg("rev-parse")
        .arg("--git-common-dir")
        .current_dir(cwd)
        .output()
        .context("running git rev-parse --git-common-dir")?;
    if !out.status.success() {
        anyhow::bail!("not in a git repo (cwd: {})", cwd.display());
    }
    let s = String::from_utf8(out.stdout).context("git output not utf-8")?;
    let common = PathBuf::from(s.trim());
    if common.is_absolute() {
        Ok(common)
    } else {
        Ok(cwd.join(common))
    }
}
