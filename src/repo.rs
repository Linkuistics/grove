// Working-tree and workspace resolution. Grove drives **jj and nothing else**:
// [`require_jj_workspace`] is the precondition gate every path passes through,
// and a working tree with no `.jj/` at or above it is refused with the command
// that fixes it, before anything is created or changed
// (`docs/adr/jj-is-the-only-lane.md`). A `.git` beside a `.jj` is a colocated
// repo and is jj's business, not Grove's — nothing here reads it, spawns `git`,
// or branches on its presence.
//
// The probe is a thin filesystem walk, not an abstraction layer. Renaming never
// went through it: since the flip every entry moves inside an `ordinal-fs-tree`
// operation, which is a plain `rename(2)`
// (`docs/adr/grove-does-not-stage-its-own-renames.md`).

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

mod finish_commit;

pub(crate) use finish_commit::{
    abort_preparing_finish, prepare_finish, recover_finish, verify_lost_finish_result,
    FinishCommitOutcome, FinishProof, FinishRecoveryOutcome, FinishStartAnchor, FinishStartProof,
    PreparedFinish,
};

/// Canonical paths that scope Grove's untracked process coordination to one
/// exact jj workspace.
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

    /// The on-disk marker this control resolved from, for a diagnostic to name.
    ///
    /// Derived rather than carried: with one lane the marker is always the
    /// resolved root's own `.jj`, so there is nothing a second walk could
    /// disagree about.
    pub fn marker(&self) -> PathBuf {
        self.worktree_root.join(".jj")
    }
}

/// The jj workspace root at or above `path` — the closest ancestor holding a
/// `.jj/` directory, whether native, secondary, or colocated. `None` when there
/// is none, which is the only answer *absence* has: it is a refusal everywhere
/// except where nothing-owns-it is itself the answer ([`path_is_tracked`]).
pub fn jj_workspace_root(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .find(|dir| dir.join(".jj").is_dir())
        .map(Path::to_path_buf)
}

/// The precondition gate. Every Grove path that is about to read or mutate a
/// task tree passes through this or through [`workspace_control`], and both
/// refuse the same way, before any mutation.
///
/// The refusal is the product: it names what was looked for, where, and the one
/// command that fixes it (principle 2 — an error that only reports detection is
/// unfinished). Both remedies are stated unconditionally rather than chosen by
/// probing for a `.git`, because a message that guesses which one applies can
/// guess wrong, and the pair is two lines.
pub fn require_jj_workspace(path: &Path) -> Result<PathBuf> {
    jj_workspace_root(path).ok_or_else(|| not_a_jj_workspace(path))
}

/// The one refusal, so every gate says the same thing. Returned as a value
/// rather than bailed so the callers that report an outcome instead of a
/// `Result` — finish recovery — can carry it too.
pub fn not_a_jj_workspace(path: &Path) -> anyhow::Error {
    anyhow::anyhow!(
        "Grove drives jj, and this is not a jj working tree\n  \
         looked for a `.jj` directory at and above: {}\n\n\
         Make the tree jj-enabled and rerun:\n      \
         jj git init --colocate     # an existing Git repository, history kept\n      \
         jj git init                # no repository here yet\n\n\
         Nothing was created or changed.",
        path.display()
    )
}

/// Resolve the workspace-scoped directory for untracked Grove process
/// coordination from the closest `.jj/` marker. This deliberately does not
/// invoke `jj`, so repository-selection environment variables and a shared
/// repository store cannot redirect the result.
pub fn workspace_control(path: &Path) -> Result<WorkspaceControl> {
    let candidate = require_jj_workspace(path)?;
    let worktree_root = candidate
        .canonicalize()
        .with_context(|| format!("canonicalizing jj workspace root {}", candidate.display()))?;
    Ok(WorkspaceControl {
        control_dir: worktree_root.join(".jj/grove"),
        worktree_root,
    })
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

/// Resolve the working-tree top directory of `cwd`. This *is* the grove
/// worktree (user-owned-worktrees) — grove runs from inside it and never
/// creates or relocates it. It is the directory holding `.jj/`, already found
/// by the probe: no jj binary is spawned to answer it.
pub fn toplevel(cwd: &Path) -> Result<PathBuf> {
    require_jj_workspace(cwd)
}

/// The **main repo** behind the working tree holding `cwd` — the checkout the
/// worktree belongs to, whose basename names the repo in session names and
/// whose path anchors the harness stamp. From a secondary jj workspace that is
/// the `default` workspace's root (`jj workspace root --name default`); a
/// native or colocated checkout is its own main repo.
pub fn main_repo_of(cwd: &Path) -> Result<PathBuf> {
    require_jj_workspace(cwd)?;
    jj_default_workspace_root(cwd)
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

/// Is `path` **tracked** by the jj workspace `path` itself sits in?
///
/// The one read-only question [`crate::session_config`] asks the VCS, and the
/// enforcement behind [the untracked configuration
/// delta](../docs/adr/untracked-configuration-delta.md): a delta names a program
/// to execute, so a repository that could ship one would choose what Grove
/// spawns in any checkout of it. Documentation cannot establish that boundary
/// and neither can an ignore rule — a file already committed stays tracked when
/// an ignore line is added.
///
/// Anchored to the candidate's **own** directory rather than to the leased
/// worktree, because the two searched roots may live in different workspaces (a
/// secondary jj workspace) and the one that owns the file is the one whose
/// working-copy commit can hold it. The probe is spawned as an internal child by
/// [`vcs_probe`] and kept read-only by `--ignore-working-copy`.
///
/// No `.jj` marker at all answers `false` rather than refusing: this is the one
/// place absence is an answer rather than a precondition failure — nothing owns
/// the file, so nothing tracks it, and the hostile repository this guards
/// against has a marker by definition. A probe that cannot be *completed* — the
/// binary missing, the command failing — is an error, and its caller fails
/// closed.
pub(crate) fn path_is_tracked(path: &Path) -> Result<bool> {
    let directory = path
        .parent()
        .with_context(|| format!("candidate path has no parent directory: {}", path.display()))?;
    let name = path
        .file_name()
        .with_context(|| format!("candidate path has no file name: {}", path.display()))?;
    if jj_workspace_root(directory).is_none() {
        return Ok(false);
    }
    jj_path_is_tracked(directory, Path::new(name))
}

/// One read-only jj probe, spawned as an **internal child**: it must answer
/// about the workspace Grove selected, not about whatever repository the
/// process that launched Grove had selected for itself.
///
/// [`crate::launch::scrub_internal_child_env`] removes the selectors Grove never
/// wants, `JJ_*` and the inherited `GIT_*` a colocated repo would still honour
/// among them. Routing the probe through one constructor is what stops the next
/// one being written without it.
fn vcs_probe(program: &str, directory: &Path) -> Command {
    let mut command = Command::new(program);
    command.current_dir(directory);
    crate::launch::scrub_internal_child_env(&mut command);
    command
}

/// `jj file list --ignore-working-copy <name>` in the candidate's own directory:
/// non-empty stdout means the working-copy commit holds it.
///
/// `--ignore-working-copy` is what keeps the probe read-only, and it has a
/// consequence worth knowing rather than smoothing over: jj snapshots
/// automatically, so an *unignored* delta reads untracked until the next
/// snapshot and refused after it. That is the design forcing the ignore line.
fn jj_path_is_tracked(directory: &Path, name: &Path) -> Result<bool> {
    let out = vcs_probe("jj", directory)
        .arg("file")
        .arg("list")
        .arg("--ignore-working-copy")
        .arg(name)
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
