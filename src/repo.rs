// Working-tree and repo resolution, jj-aware. Grove drives whichever VCS owns
// the working tree, decided **jj-first** by [`vcs_of`]: a `.jj/` directory
// makes the tree jj-enabled and picks jj plumbing even when a `.git` sits
// beside it (colocated) — the symmetric VCS rule the using-jujutsu skill also
// follows. Git remains the interface only in not-jj-enabled trees. The probe
// is a thin filesystem walk, not an abstraction layer: the handful of call
// sites (launch, llm_cli, tree_rename, tree_migrate) each branch on it where
// the two VCSes genuinely differ.

use anyhow::{anyhow, bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The VCS that owns a working tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Vcs {
    /// jj-enabled: a `.jj/` directory heads the tree — a native repo, a
    /// secondary `jj workspace`, or a colocated repo (jj-first). The carried
    /// root is the workspace root (the directory holding `.jj/`).
    Jj { workspace_root: PathBuf },
    /// A plain git working tree (checkout or linked worktree), not jj-enabled.
    Git,
}

/// The VCS owning `path`, walking up from it: at each ancestor a `.jj/`
/// directory wins (jj-first, even when a `.git` sits beside it), then a
/// `.git` (directory, or a linked worktree's gitfile). The *closest* marker
/// decides, so a plain-git checkout nested under a jj-enabled tree stays git.
/// `None`: no VCS marker all the way up.
pub fn vcs_of(path: &Path) -> Option<Vcs> {
    for dir in path.ancestors() {
        if dir.join(".jj").is_dir() {
            return Some(Vcs::Jj {
                workspace_root: dir.to_path_buf(),
            });
        }
        if dir.join(".git").exists() {
            return Some(Vcs::Git);
        }
    }
    None
}

/// Resolve the repo path: if `arg` is Some, use it; otherwise use cwd's main
/// repo (see [`main_repo_of`]).
pub fn resolve(arg: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = arg {
        if !p.join(".jj").is_dir() && !p.join(".git").exists() && git_common_dir(p).is_err() {
            bail!("not a git or jj repo: {}", p.display());
        }
        return Ok(p.to_path_buf());
    }
    let cwd = std::env::current_dir().context("getting cwd")?;
    main_repo_of(&cwd)
}

/// Resolve the working-tree top directory of `cwd`. This *is* the grove
/// worktree (user-owned-worktrees) — grove runs from inside it and never
/// creates or relocates it. In a jj-enabled tree the workspace root is the
/// directory holding `.jj/` (already found by the probe — no jj binary
/// needed); in a git tree it is `git rev-parse --show-toplevel`.
pub fn toplevel(cwd: &Path) -> Result<PathBuf> {
    match vcs_of(cwd) {
        Some(Vcs::Jj { workspace_root }) => Ok(workspace_root),
        Some(Vcs::Git) => git_show_toplevel(cwd),
        None => bail!("not in a git or jj repo (cwd: {})", cwd.display()),
    }
}

/// The **main repo** behind the working tree holding `cwd` — the checkout the
/// worktree belongs to, whose basename names the repo in session names and
/// whose path anchors the harness stamp. From a git linked worktree that is
/// the parent of the git common dir; from a secondary jj workspace it is the
/// `default` workspace's root (`jj workspace root --name default`). A plain
/// checkout of either VCS is its own main repo.
pub fn main_repo_of(cwd: &Path) -> Result<PathBuf> {
    match vcs_of(cwd) {
        Some(Vcs::Jj { .. }) => jj_default_workspace_root(cwd),
        Some(Vcs::Git) => {
            let common_dir = git_common_dir(cwd)?;
            // common_dir points to .git or worktrees/<name>/.git; the main
            // repo is its parent.
            Ok(common_dir
                .parent()
                .ok_or_else(|| anyhow!("git common-dir has no parent"))?
                .to_path_buf())
        }
        None => bail!("not in a git or jj repo (cwd: {})", cwd.display()),
    }
}

/// `jj workspace root --name default` — the main repo of a jj-enabled tree.
/// `--ignore-working-copy` keeps the probe read-only: without it every jj
/// command snapshots the working copy, a mutation no resolution step should
/// perform (and one that would fail outright in a stale workspace).
fn jj_default_workspace_root(cwd: &Path) -> Result<PathBuf> {
    let out = Command::new("jj")
        .args([
            "workspace",
            "root",
            "--name",
            "default",
            "--ignore-working-copy",
        ])
        .current_dir(cwd)
        .output()
        .context("running jj workspace root --name default")?;
    if !out.status.success() {
        bail!(
            "jj workspace root --name default failed in {}: {}",
            cwd.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let s = String::from_utf8(out.stdout).context("jj output not utf-8")?;
    Ok(PathBuf::from(s.trim()))
}

/// `git rev-parse --show-toplevel` of `cwd` — the git branch of [`toplevel`].
fn git_show_toplevel(cwd: &Path) -> Result<PathBuf> {
    let out = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(cwd)
        .output()
        .context("running git rev-parse --show-toplevel")?;
    if !out.status.success() {
        bail!("not in a git repo (cwd: {})", cwd.display());
    }
    let s = String::from_utf8(out.stdout).context("git output not utf-8")?;
    Ok(PathBuf::from(s.trim()))
}

/// Absolutized `git rev-parse --git-common-dir` of `cwd`: the checkout's own
/// `.git` in a plain repo, the main repo's `.git` from a linked worktree
/// (whose own gitdir is a subpath of it). Git may print the path *relative*
/// (`.git`, from a plain checkout's toplevel), so it is absolutized against
/// `cwd` before use. Also the dir a codex launch in a git tree grants back
/// via `--add-dir` (codex-gitdir-grant; jj trees derive their grants from the
/// main workspace root instead).
pub fn git_common_dir(cwd: &Path) -> Result<PathBuf> {
    let out = Command::new("git")
        .arg("rev-parse")
        .arg("--git-common-dir")
        .current_dir(cwd)
        .output()
        .context("running git rev-parse --git-common-dir")?;
    if !out.status.success() {
        bail!("not in a git repo (cwd: {})", cwd.display());
    }
    let s = String::from_utf8(out.stdout).context("git output not utf-8")?;
    let common = PathBuf::from(s.trim());
    if common.is_absolute() {
        Ok(common)
    } else {
        Ok(cwd.join(common))
    }
}
