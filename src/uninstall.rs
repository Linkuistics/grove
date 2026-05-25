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
            "live groves exist in {}/groves/ — pass --force to uninstall anyway",
            repo_path.display()
        );
    }

    fs::remove_dir_all(&dest)
        .with_context(|| format!("removing {}", dest.display()))?;
    eprintln!("grove: removed {}", dest.display());
    Ok(())
}

/// A grove is "live" if `groves/<name>/` contains at least one `.md` leaf
/// outside any `done/` directory.
fn has_live_groves(repo_path: &Path) -> Result<bool> {
    let groves = repo_path.join("groves");
    if !groves.is_dir() {
        return Ok(false);
    }
    for entry in fs::read_dir(&groves).with_context(|| format!("reading {}", groves.display()))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        if entry.file_name() != "done" {
            if has_live_leaf(&entry.path())? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn has_live_leaf(grove_dir: &Path) -> Result<bool> {
    for entry in fs::read_dir(grove_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        if file_name == "done" {
            continue;
        }
        if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
            return Ok(true);
        }
        if path.is_dir() {
            if has_live_leaf(&path)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
