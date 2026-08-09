use super::{auxiliary_marker_paths, marker_paths, recover_auxiliary_marker, QuarantineCleanup};
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
            match recover_auxiliary_marker(&marker_path).and_then(|cleanup| match &owner {
                Some(owner) if cleanup.is_owned_by(owner) => Ok(()),
                _ => cleanup.dispose(),
            }) {
                Ok(()) => {}
                Err(error) => failures.push(format!("{}: {error:#}", marker_path.display())),
            }
        }
    }

    finish(failures)
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
