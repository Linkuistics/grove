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
    marker: ControlMarker,
}

impl WorkspaceControl {
    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }

    pub fn control_dir(&self) -> &Path {
        &self.control_dir
    }

    pub fn marker(&self) -> &ControlMarker {
        &self.marker
    }
}

/// The on-disk VCS marker that produced a [`WorkspaceControl`], carried
/// alongside the paths it resolved to.
///
/// The workspace layout preflight reports this, and re-deriving it at diagnostic
/// time would walk the ancestors a second time and could name a different marker
/// than the one that actually resolved. The gitfile variant carries its target
/// because that indirection *is* the step which leaves the working tree — it is
/// what the operator has to look at, not a detail of the marker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControlMarker {
    /// A `.jj/` directory: native, secondary, or colocated.
    JjDirectory { path: PathBuf },
    /// A `.git/` directory: a plain checkout.
    GitDirectory { path: PathBuf },
    /// A `.git` **file**: a linked worktree or a submodule, plus the canonical
    /// gitdir it named.
    GitFile { path: PathBuf, gitdir: PathBuf },
}

impl std::fmt::Display for ControlMarker {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JjDirectory { path } => {
                write!(formatter, "the `.jj` directory {}", path.display())
            }
            Self::GitDirectory { path } => {
                write!(formatter, "the `.git` directory {}", path.display())
            }
            Self::GitFile { path, gitdir } => write!(
                formatter,
                "the `.git` file {}, naming gitdir {}",
                path.display(),
                gitdir.display()
            ),
        }
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
                marker: ControlMarker::JjDirectory {
                    path: worktree_root.join(".jj"),
                },
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
                control_dir: git_dir.join("grove"),
                marker: ControlMarker::GitDirectory {
                    path: worktree_root.join(".git"),
                },
                worktree_root,
            });
        }
        if git_marker.is_file() {
            let worktree_root = candidate.canonicalize().with_context(|| {
                format!("canonicalizing Git working tree {}", candidate.display())
            })?;
            let git_dir = gitfile_target(&git_marker)?;
            return Ok(WorkspaceControl {
                control_dir: git_dir.join("grove"),
                marker: ControlMarker::GitFile {
                    path: worktree_root.join(".git"),
                    gitdir: git_dir,
                },
                worktree_root,
            });
        }
    }

    bail!("not in a git or jj working tree (path: {})", path.display())
}

/// The filesystem Grove acts on for `path`, and the single point where a test may
/// substitute a second one.
///
/// Grove makes two device comparisons — the workspace layout preflight at
/// driver-lease acquisition, and the finish transaction's own quarantine
/// preflight — and each reads a real `st_dev` and passes it through here. A second
/// filesystem is the one operand this suite cannot stage portably: mounting one
/// needs privileges on Linux and a disk image on macOS. `GROVE_TEST_FOREIGN_FILESYSTEM`
/// names a directory, and every measurement at or under it reports a distinct
/// filesystem, so the acceptance matrix drives both real refusals — resolution,
/// ordering, diagnostic, and each one's no-mutation guarantee — through the real
/// processes instead of a unit call.
///
/// The seam is a **path** rather than a device number on purpose: a test has to
/// name the exact directory resolution landed on, so a run cannot pass while the
/// resolver walked somewhere else entirely. It is an internal test control, not
/// launch configuration, so [`crate::launch`] scrubs it from every spawn, and
/// with the variable unset — every production invocation — this is the identity,
/// down to performing no extra syscall.
///
/// Deliberately one helper rather than a substitution per call site. The two
/// preflights must stay independent — neither may consult the other's verdict —
/// and the only thing they may share is *how a filesystem is measured*. Sharing
/// that is also what lets one seam express both halves of the independence: a
/// prefix naming the control directory makes the layout cross-device, while one
/// naming `.grove/` leaves acquisition passing and refuses only at finish.
pub(crate) fn measured_device(path: &Path, device: u64) -> u64 {
    let Some(prefix) = foreign_filesystem_root() else {
        return device;
    };
    let resolved = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if resolved.starts_with(&prefix) {
        // Any value distinct from the real one; `^ 1` cannot collide with it.
        device ^ 1
    } else {
        device
    }
}

fn foreign_filesystem_root() -> Option<PathBuf> {
    let value = std::env::var_os("GROVE_TEST_FOREIGN_FILESYSTEM")?;
    if value.is_empty() {
        return None;
    }
    let path = PathBuf::from(value);
    Some(path.canonicalize().unwrap_or(path))
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

/// Is `path` **tracked** by the VCS that owns the tree `path` itself sits in?
///
/// The one read-only question [`crate::session_config`] asks a VCS, and the
/// enforcement behind [the untracked configuration
/// delta](../docs/adr/untracked-configuration-delta.md): a delta names a program
/// to execute, so a repository that could ship one would choose what Grove
/// spawns in any checkout of it. Documentation cannot establish that boundary
/// and neither can an ignore rule — a file already committed stays tracked when
/// a `.gitignore` line is added.
///
/// Anchored to the candidate's **own** directory rather than to the leased
/// worktree, because the two searched roots may live in different trees (a
/// linked worktree, a secondary jj workspace) and the tree that owns the file is
/// the one whose index or working-copy commit can hold it. Lane chosen jj-first
/// by [`vcs_of`], Git anchored by [`anchor_git_worktree_environment`], jj kept
/// read-only by `--ignore-working-copy` — the same three idioms every other
/// probe here follows.
///
/// No VCS marker at all answers `false` rather than failing: nothing owns the
/// file, so nothing tracks it, and the hostile repository this guards against
/// has a marker by definition. A probe that cannot be *completed* — the binary
/// missing, the command failing — is an error, and its caller fails closed.
pub(crate) fn path_is_tracked(path: &Path) -> Result<bool> {
    let directory = path
        .parent()
        .with_context(|| format!("candidate path has no parent directory: {}", path.display()))?;
    let name = path
        .file_name()
        .with_context(|| format!("candidate path has no file name: {}", path.display()))?;
    match vcs_of(directory) {
        Some(Vcs::Jj { .. }) => jj_path_is_tracked(directory, Path::new(name)),
        Some(Vcs::Git) => git_path_is_tracked(directory, Path::new(name)),
        None => Ok(false),
    }
}

/// `jj file list --ignore-working-copy <name>` in the candidate's own directory:
/// non-empty stdout means the working-copy commit holds it.
///
/// `--ignore-working-copy` is what keeps the probe read-only, and it has a
/// consequence worth knowing rather than smoothing over: jj snapshots
/// automatically, so an *unignored* delta reads untracked until the next
/// snapshot and refused after it. That is the design forcing the ignore line.
fn jj_path_is_tracked(directory: &Path, name: &Path) -> Result<bool> {
    let out = Command::new("jj")
        .arg("file")
        .arg("list")
        .arg("--ignore-working-copy")
        .arg(name)
        .current_dir(directory)
        .output()
        .context("running jj file list --ignore-working-copy")?;
    if !out.status.success() {
        bail!(
            "jj file list --ignore-working-copy failed in {}: {}",
            directory.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(!out.stdout.is_empty())
}

/// `git ls-files -- <name>` in the candidate's own directory: it prints the path
/// when the index holds it and nothing when it does not, so trackedness is the
/// emptiness of stdout rather than an exit status (`--error-unmatch` would make
/// "untracked" indistinguishable from "the probe broke").
fn git_path_is_tracked(directory: &Path, name: &Path) -> Result<bool> {
    let worktree = workspace_control(directory)?.worktree_root().to_path_buf();
    let mut command = Command::new("git");
    command
        .arg("ls-files")
        .arg("--")
        .arg(name)
        .current_dir(directory);
    anchor_git_worktree_environment(&mut command, &worktree);
    let out = command.output().context("running git ls-files")?;
    if !out.status.success() {
        bail!(
            "git ls-files failed in {}: {}",
            directory.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(!out.stdout.is_empty())
}
