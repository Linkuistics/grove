// Working-tree and repo resolution, jj-aware. Grove drives whichever VCS owns
// the working tree, decided **jj-first** by [`vcs_of`]: a `.jj/` directory
// makes the tree jj-enabled and picks jj plumbing even when a `.git` sits
// beside it (colocated) — the symmetric VCS rule the using-jujutsu skill also
// follows. Git remains the interface only in not-jj-enabled trees. The probe
// is a thin filesystem walk, not an abstraction layer: the handful of call
// sites (launch, llm_cli, tree_rename, tree_migrate) each branch on it where
// the two VCSes genuinely differ.

use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod finish_commit;
mod migration_commit;

pub(crate) use finish_commit::{
    abort_preparing_finish, git_index_path, prepare_finish, recover_finish,
    verify_lost_finish_result, FinishCommitOutcome, FinishProof, FinishRecoveryOutcome,
    FinishStartAnchor, FinishStartProof, PreparedFinish,
};
pub use migration_commit::commit_session_kind_migration;

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

/// Canonical paths that scope Grove's untracked process coordination to one
/// exact working tree or workspace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceControl {
    worktree_root: PathBuf,
    control_dir: PathBuf,
}

impl WorkspaceControl {
    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }

    pub fn control_dir(&self) -> &Path {
        &self.control_dir
    }
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

/// Resolve the workspace-scoped directory for untracked Grove process
/// coordination from the closest on-disk VCS marker. This deliberately does
/// not invoke `git` or `jj`, so repository-selection environment variables and
/// a shared repository store cannot redirect the result.
pub fn workspace_control(path: &Path) -> Result<WorkspaceControl> {
    for candidate in path.ancestors() {
        let jj_dir = candidate.join(".jj");
        if jj_dir.is_dir() {
            let worktree_root = candidate.canonicalize().with_context(|| {
                format!("canonicalizing jj workspace root {}", candidate.display())
            })?;
            return Ok(WorkspaceControl {
                control_dir: worktree_root.join(".jj/grove"),
                worktree_root,
            });
        }

        let git_marker = candidate.join(".git");
        if git_marker.is_dir() {
            let worktree_root = candidate.canonicalize().with_context(|| {
                format!("canonicalizing Git working tree {}", candidate.display())
            })?;
            let git_dir = git_marker
                .canonicalize()
                .with_context(|| format!("canonicalizing {}", git_marker.display()))?;
            return Ok(WorkspaceControl {
                worktree_root,
                control_dir: git_dir.join("grove"),
            });
        }
        if git_marker.is_file() {
            let worktree_root = candidate.canonicalize().with_context(|| {
                format!("canonicalizing Git working tree {}", candidate.display())
            })?;
            let git_dir = gitfile_target(&git_marker)?;
            return Ok(WorkspaceControl {
                worktree_root,
                control_dir: git_dir.join("grove"),
            });
        }
    }

    bail!("not in a git or jj working tree (path: {})", path.display())
}

/// The empty hooks directory every internal Grove Git commit runs with.
///
/// Migration and finish both promise that their path-scoped commit preserves
/// unrelated staged and working-tree bytes, and both roll back from an index
/// image. A user hook is an arbitrary program that can mutate those unrelated
/// bytes even while rejecting the commit, and no index image restores them — so
/// the guard belongs to the seam both commits share rather than to either
/// transaction. The directory is untracked workspace-control scratch, created on
/// demand and never swept: it carries no cleanup manifest, so the finish reaper
/// leaves it alone.
///
/// Being empty is the whole contract, so a non-directory or non-empty path is
/// refused rather than emptied — Grove did not put anything there.
pub(crate) fn empty_hooks_path(worktree: &Path) -> Result<PathBuf> {
    let control = workspace_control(worktree)?;
    fs::create_dir_all(control.control_dir()).with_context(|| {
        format!(
            "creating workspace-control directory for internal commit hooks {}",
            control.control_dir().display()
        )
    })?;
    let hooks_path = control.control_dir().join("internal-hooks-empty");
    match fs::create_dir(&hooks_path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(&hooks_path).with_context(|| {
                format!(
                    "checking internal commit hooks path {}",
                    hooks_path.display()
                )
            })?;
            if !metadata.file_type().is_dir() {
                bail!(
                    "internal commit hooks path is not a directory: {}",
                    hooks_path.display()
                );
            }
            if fs::read_dir(&hooks_path)
                .with_context(|| {
                    format!(
                        "reading internal commit hooks path {}",
                        hooks_path.display()
                    )
                })?
                .next()
                .is_some()
            {
                bail!(
                    "internal commit hooks path is not empty: {}",
                    hooks_path.display()
                );
            }
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "creating internal commit hooks path {}",
                    hooks_path.display()
                )
            })
        }
    }
    Ok(hooks_path)
}

fn gitfile_target(gitfile: &Path) -> Result<PathBuf> {
    let contents = fs::read_to_string(gitfile)
        .with_context(|| format!("reading Git worktree marker {}", gitfile.display()))?;
    let target = contents
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("gitdir:"))
        .map(str::trim)
        .filter(|target| !target.is_empty())
        .with_context(|| format!("malformed Git worktree marker {}", gitfile.display()))?;
    let target = PathBuf::from(target);
    let target = if target.is_absolute() {
        target
    } else {
        gitfile
            .parent()
            .context("Git worktree marker has no parent")?
            .join(target)
    };
    target
        .canonicalize()
        .with_context(|| format!("canonicalizing Git worktree gitdir {}", target.display()))
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

/// Make one subprocess resolve Git operations against the exact on-disk
/// worktree Grove already selected.
///
/// Removing repository selectors prevents an inherited Git context from
/// choosing a foreign repository. Setting `GIT_WORK_TREE` also overrides a
/// hostile `core.worktree` while leaving Git to discover the selected root's
/// own `.git` marker.
pub(crate) fn anchor_git_worktree_environment(command: &mut Command, worktree: &Path) {
    command
        .env_remove("GIT_DIR")
        .env("GIT_WORK_TREE", worktree)
        .env_remove("GIT_COMMON_DIR");
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
/// `cwd` before use.
pub fn git_common_dir(cwd: &Path) -> Result<PathBuf> {
    let worktree = workspace_control(cwd)?.worktree_root().to_path_buf();
    let mut command = Command::new("git");
    command
        .arg("rev-parse")
        .arg("--git-common-dir")
        .current_dir(&worktree);
    anchor_git_worktree_environment(&mut command, &worktree);
    let out = command
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
        Ok(worktree.join(common))
    }
}
