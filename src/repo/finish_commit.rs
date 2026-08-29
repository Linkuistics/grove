// The finish teardown commit, jj only (`docs/adr/jj-is-the-only-lane.md`).
//
// **Grove takes a commit; it does not implement a transaction.** There is no
// witness, no manifest, no rollback proof, no index image, no quarantine and no
// recovery path, because the version control system already owns all of them:
// jj snapshots the working copy before every command, and its operation log is
// the transaction record. So the whole of this module is one path-scoped
// `jj commit`, and the one thing it adds is the remedy it prints when that
// command does not complete — the operation-log command that puts the working
// copy back (principle 2: an error that only reports detection is unfinished).
//
// Measured rather than assumed (jj 0.44.0, colocated): deleting `.grove/` with
// no jj command run and then `jj restore .grove` returns every file; a partial
// deletion followed by `jj undo` reports *"Added 2 files"* — exactly the missing
// ones.

use super::require_jj_workspace;
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// The one precondition deletion has: jj can only put back what it tracks.
///
/// **This is not a surviving piece of the transaction.** A transaction promises
/// to undo its own work; this promises nothing and repairs nothing. It is the
/// gate that makes the version control system's guarantee *applicable* — an
/// untracked `.grove/` is outside the operation log, so no `jj undo` would
/// return it and the deletion below would be the unrecoverable kind. Principle
/// 2's answer to that is a message naming what is wrong and how to fix it, which
/// is what this is. One read-only probe, and nothing is written to record it.
pub(crate) fn require_recoverable_grove(worktree: &Path) -> Result<()> {
    let workspace_root =
        require_jj_workspace(worktree).context("cannot commit the finished grove")?;
    let output = jj_command(&workspace_root)
        // Deliberately *without* `--ignore-working-copy`, unlike
        // [`super::path_is_tracked`]: jj snapshots first, so the answer is about
        // the tree as it is on disk right now rather than as it was at the last
        // snapshot. The caller is one line away from deleting it, so a probe
        // that reads a stale working-copy commit would refuse a grove that is
        // tracked, or admit one that is not.
        .args(["file", "list", "-r", "@", "root:.grove"])
        .output()
        .with_context(|| {
            format!(
                "asking Jujutsu whether `.grove/` is tracked in {}",
                workspace_root.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "jj file list root:.grove failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    if output.stdout.is_empty() {
        bail!(
            "Jujutsu tracks nothing under {}, so deleting the task tree could not be undone\n\n\
             Grove takes a commit; it does not implement a transaction, and the operation log \
             can only restore what it tracks. Commit or track the task tree and rerun:\n      \
             jj commit -m \"track the grove task tree\" root:.grove\n\n\
             Nothing was deleted or changed.",
            workspace_root.join(".grove").display()
        );
    }
    Ok(())
}

/// Commit the `.grove/` teardown the caller has already performed.
///
/// Path-scoped, so unrelated working-copy changes stay in the working copy: jj
/// snapshots everything and then commits only the fileset named here.
pub(crate) fn commit_finished_grove(worktree: &Path, finish_handle: &str) -> Result<()> {
    let workspace_root =
        require_jj_workspace(worktree).context("cannot commit the finished grove")?;
    let message = finish_commit_message(finish_handle);
    let output = jj_command(&workspace_root)
        .args(["commit", "-m", &message, "root:.grove"])
        .output()
        .with_context(|| {
            format!(
                "running jj commit in {}, to record the {finish_handle} teardown",
                workspace_root.display()
            )
        })
        .map_err(|error| uncommitted_teardown(&workspace_root, finish_handle, error))?;
    if !output.status.success() {
        return Err(uncommitted_teardown(
            &workspace_root,
            finish_handle,
            anyhow::anyhow!(
                "jj commit failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    Ok(())
}

/// The one refusal this module has, and the reason it exists: the task tree is
/// deleted in the working copy and the commit recording that did not land.
/// Nothing here repairs it — the operation log does, and this names the command.
fn uncommitted_teardown(
    workspace_root: &Path,
    finish_handle: &str,
    error: anyhow::Error,
) -> anyhow::Error {
    error.context(format!(
        "the {finish_handle} teardown was not committed, and `.grove/` is deleted in the \
         working copy at {}\n\n\
         Jujutsu snapshots the working copy before every command and its operation log is the \
         transaction record, so restore the task tree with:\n      \
         jj undo                    # reverse the snapshot that recorded the deletion\n      \
         jj op log                  # inspect the operations first, if `jj undo` is not the one\n\n\
         Grove runs no recovery of its own. Once the tree is back, fix what made the commit \
         fail and rerun `grove-llm finish-commit {finish_handle}`.",
        workspace_root.display(),
    ))
}

fn finish_commit_message(finish_handle: &str) -> String {
    format!("{finish_handle}: remove completed grove task tree")
}

fn jj_command(worktree: &Path) -> Command {
    let mut command = Command::new("jj");
    command.current_dir(worktree);
    crate::launch::scrub_internal_child_env(&mut command);
    command
}
