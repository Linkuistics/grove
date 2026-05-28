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
/// --show-toplevel`. For a grove session this is the grove worktree at
/// `<repo>/.grove-worktrees/<name>/`.
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

/// Create a worktree at `<repo>/.grove-worktrees/<name>/` on a new branch
/// `<name>`, branching from `start_point` (or origin's HEAD).
pub fn create_grove_worktree(
    repo: &Path,
    name: &str,
    start_point: Option<&str>,
) -> Result<PathBuf> {
    let worktree = repo.join(".grove-worktrees").join(name);
    if worktree.exists() {
        anyhow::bail!("worktree already exists: {}", worktree.display());
    }
    let branch = name.to_string();
    let start = match start_point {
        Some(s) => s.to_string(),
        None => default_start_point(repo)?,
    };
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("worktree")
        .arg("add")
        .arg(&worktree)
        .arg("-b")
        .arg(&branch)
        .arg(&start)
        .status()
        .context("running git worktree add")?;
    if !status.success() {
        anyhow::bail!("git worktree add failed");
    }
    Ok(worktree)
}

fn default_start_point(repo: &Path) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("symbolic-ref")
        .arg("--short")
        .arg("refs/remotes/origin/HEAD")
        .output()
        .context("running git symbolic-ref")?;
    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if let Some(short) = s.strip_prefix("origin/") {
            return Ok(short.to_string());
        }
        return Ok(s);
    }
    Ok("main".to_string())
}

/// Path of an existing grove worktree.
pub fn grove_worktree(repo: &Path, name: &str) -> PathBuf {
    repo.join(".grove-worktrees").join(name)
}

/// Directory holding all per-grove worktrees.
pub fn grove_worktrees_dir(repo: &Path) -> PathBuf {
    repo.join(".grove-worktrees")
}

/// Whether a local branch `<branch>` exists in `repo`.
pub fn branch_exists(repo: &Path, branch: &str) -> Result<bool> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(format!("refs/heads/{}", branch))
        .status()
        .context("running git show-ref")?;
    Ok(status.success())
}

/// Re-attach a worktree at `<repo>/.grove-worktrees/<name>/` to the
/// existing branch `<name>`. Used by `grove do` to recover an orphaned
/// grove (branch present, worktree deleted).
pub fn attach_grove_worktree(repo: &Path, name: &str) -> Result<PathBuf> {
    let worktree = repo.join(".grove-worktrees").join(name);
    if worktree.exists() {
        anyhow::bail!("worktree already exists: {}", worktree.display());
    }
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("worktree")
        .arg("add")
        .arg(&worktree)
        .arg(name)
        .status()
        .context("running git worktree add")?;
    if !status.success() {
        anyhow::bail!("git worktree add failed");
    }
    Ok(worktree)
}
