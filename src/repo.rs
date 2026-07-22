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

/// Resolve the working-tree top directory of `cwd` via `git rev-parse
/// --show-toplevel`. This *is* the grove worktree (user-owned-worktrees) —
/// grove runs from inside it and never creates or relocates it.
pub fn git_toplevel(cwd: &Path) -> Result<PathBuf> {
    let out = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(cwd)
        .output()
        .context("running git rev-parse --show-toplevel")?;
    if !out.status.success() {
        anyhow::bail!("not in a git repo (cwd: {})", cwd.display());
    }
    let s = String::from_utf8(out.stdout).context("git output not utf-8")?;
    Ok(PathBuf::from(s.trim()))
}

/// Absolutized `git rev-parse --git-common-dir` of `cwd`: the checkout's own
/// `.git` in a plain repo, the main repo's `.git` from a linked worktree
/// (whose own gitdir is a subpath of it). Git may print the path *relative*
/// (`.git`, from a plain checkout's toplevel), so it is absolutized against
/// `cwd` before use. Also the dir a codex launch grants back via `--add-dir`
/// (codex-gitdir-grant).
pub fn git_common_dir(cwd: &Path) -> Result<PathBuf> {
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
