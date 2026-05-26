use crate::cli::UninstallArgs;
use crate::harness::{self, Harness, SelectMode};
use crate::repo;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn run(args: &UninstallArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let harnesses = harness::select(&repo_path, &args.harnesses, SelectMode::Multi)?;

    for h in &harnesses {
        uninstall_one(&repo_path, h, args.force)?;
    }
    Ok(())
}

fn uninstall_one(repo_path: &Path, harness: &Harness, force: bool) -> Result<()> {
    let dest = harness.install_path(repo_path);
    if !dest.exists() {
        eprintln!("grove: nothing at {}", dest.display());
        return Ok(());
    }

    if !force && has_live_groves(repo_path)? {
        anyhow::bail!(
            "live groves exist in {} — pass --force to uninstall anyway",
            repo::grove_worktrees_dir(repo_path).display()
        );
    }

    fs::remove_dir_all(&dest)
        .with_context(|| format!("removing {}", dest.display()))?;
    eprintln!("grove: removed {}", dest.display());
    Ok(())
}

/// A grove is "live" iff its worktree exists in `.grove-worktrees/`.
fn has_live_groves(repo_path: &Path) -> Result<bool> {
    let dir = repo::grove_worktrees_dir(repo_path);
    if !dir.is_dir() {
        return Ok(false);
    }
    for entry in fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            return Ok(true);
        }
    }
    Ok(false)
}
