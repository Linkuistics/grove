use super::{
    auxiliary_marker_is_owned_by, auxiliary_marker_paths, marker_paths, recover_auxiliary_marker,
    QuarantineCleanup,
};
use anyhow::{bail, Context, Result};
use std::path::Path;

pub(crate) fn reap_orphaned(worktree: &Path) -> Result<()> {
    let owner = crate::finish_transaction::cleanup_owner(&worktree.join(".grove"))?;
    let control_directory = crate::repo::workspace_control(worktree)?
        .control_dir()
        .to_path_buf();
    let mut failures = Vec::new();

    for marker_path in marker_paths(&control_directory)? {
        match QuarantineCleanup::from_marker(&marker_path).and_then(|cleanup| match &owner {
            Some(owner) if cleanup.is_owned_by(owner) => Ok(()),
            _ => cleanup.dispose().map(|_| ()),
        }) {
            Ok(()) => {}
            Err(error) => failures.push(format!("{}: {error:#}", marker_path.display())),
        }
    }

    if let Some(git_index) = crate::repo::git_index_path(worktree)? {
        let auxiliary_directory = git_index
            .parent()
            .context("resolved Git index has no parent for finish auxiliary cleanup")?;
        for marker_path in auxiliary_marker_paths(auxiliary_directory)? {
            if !is_orphaned_auxiliary(&marker_path, owner.as_ref()) {
                continue;
            }
            if let Err(error) = reap(&marker_path) {
                failures.push(format!("{}: {error:#}", marker_path.display()));
            }
        }
    }

    finish(failures)
}

/// Whether this reaper may act on a discovered auxiliary marker at all.
///
/// Attribution comes from the marker document alone, and it has to: recovering
/// an auxiliary settles whatever artifact-identity replacement is still in
/// flight, which exchanges and unlinks entries. An auxiliary a live in-tree
/// witness owns is settled by that witness's own lifecycle recovery, which runs
/// immediately after this sweep and refuses hard on anything wrong with it.
///
/// A marker that will not parse carries no valid cleanup manifest, so while a
/// witness is live it is not an orphan either — nothing here can attribute it,
/// and calling it one would report a *warning* about the very path the lifecycle
/// refusal is about to name with the same cause. With no witness live, an
/// unattributable marker is reaped as before, so its failure is still reported;
/// a witness only defers that to the invocation after the witness clears.
fn is_orphaned_auxiliary(
    marker_path: &Path,
    owner: Option<&crate::finish_cleanup::CleanupOwner>,
) -> bool {
    match owner {
        Some(owner) => matches!(auxiliary_marker_is_owned_by(marker_path, owner), Ok(false)),
        None => true,
    }
}

fn reap(marker_path: &Path) -> Result<()> {
    recover_auxiliary_marker(marker_path).and_then(|cleanup| cleanup.dispose())
}

fn finish(failures: Vec<String>) -> Result<()> {
    if failures.is_empty() {
        Ok(())
    } else {
        bail!(
            "one or more orphaned finish cleanup artifacts remain for a later driver:\n{}",
            failures.join("\n")
        )
    }
}
