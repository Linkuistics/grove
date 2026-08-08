use super::{vcs_of, Vcs};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Refuse a plain-Git teardown that cannot produce a path-scoped deletion
/// commit. A hand-built unborn finish tree has no tracked `.grove/` to delete.
pub fn validate_finish_commit(worktree: &Path) -> Result<()> {
    match vcs_of(worktree) {
        Some(Vcs::Git) => {
            let output = vcs_command(worktree, "git")
                .args(["ls-tree", "-r", "--name-only", "HEAD", "--", ".grove"])
                .output()
                .context("checking whether Git tracks the grove being finished")?;
            if !output.status.success() || output.stdout.is_empty() {
                bail!(
                    "cannot finish this plain-Git grove: `.grove/` has no tracked state in HEAD, so no focused deletion commit can be recorded"
                );
            }
            Ok(())
        }
        Some(Vcs::Jj { .. }) => Ok(()),
        None => bail!(
            "cannot commit the finished grove: {} is not a git or jj working tree",
            worktree.display()
        ),
    }
}

/// Commit only the already-removed `.grove/`, preserving unrelated staged or
/// working-copy changes. The caller owns the tree lock and all finish facts.
pub fn commit_finish(worktree: &Path, finish_handle: &str) -> Result<()> {
    let message = format!("{finish_handle}: remove completed grove task tree");
    match vcs_of(worktree) {
        Some(Vcs::Git) => commit_git_finish(worktree, &message),
        Some(Vcs::Jj { workspace_root }) => commit_jj_finish(&workspace_root, &message),
        None => bail!(
            "cannot commit the finished grove: {} is not a git or jj working tree",
            worktree.display()
        ),
    }
}

fn commit_git_finish(worktree: &Path, message: &str) -> Result<()> {
    run_vcs_command(worktree, "git", &["add", "-A", "--", ".grove"])?;
    run_vcs_command(
        worktree,
        "git",
        &["commit", "--only", "-m", message, "--", ".grove"],
    )
}

fn commit_jj_finish(worktree: &Path, message: &str) -> Result<()> {
    if !worktree.join(".git").exists() {
        return run_vcs_command(worktree, "jj", &["commit", "-m", message, "root:.grove"]);
    }

    let git_index = git_path(worktree, "index")?;
    let saved_index = git_index.with_file_name("grove-finish-index");
    let had_git_index = git_index.exists();
    if had_git_index {
        fs::copy(&git_index, &saved_index).with_context(|| {
            format!(
                "preserving colocated Git index {} before the finish commit",
                git_index.display()
            )
        })?;
    }

    let result = run_vcs_command(worktree, "jj", &["commit", "-m", message, "root:.grove"]);
    let restore = if had_git_index {
        fs::rename(&saved_index, &git_index)
    } else {
        fs::remove_file(&git_index)
    };
    match (result, restore) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        (Ok(()), Err(error)) => {
            eprintln!(
                "warning: restore the colocated Git index {} after the finish commit: {error}",
                git_index.display()
            );
            Ok(())
        }
        (Err(command_error), Ok(())) => Err(command_error),
        (Err(command_error), Err(restore_error)) => Err(command_error.context(format!(
            "also failed to restore the colocated Git index {}: {restore_error}",
            git_index.display()
        ))),
    }
}

fn git_path(worktree: &Path, name: &str) -> Result<PathBuf> {
    let output = vcs_command(worktree, "git")
        .args(["rev-parse", "--git-path", name])
        .output()
        .with_context(|| format!("resolving Git {name} path in {}", worktree.display()))?;
    if !output.status.success() {
        bail!(
            "git rev-parse --git-path {name} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let path = PathBuf::from(String::from_utf8(output.stdout)?.trim());
    Ok(if path.is_absolute() {
        path
    } else {
        worktree.join(path)
    })
}

fn run_vcs_command(worktree: &Path, binary: &str, arguments: &[&str]) -> Result<()> {
    let output = vcs_command(worktree, binary)
        .args(arguments)
        .output()
        .with_context(|| {
            format!(
                "running {binary} {} in {}",
                arguments.join(" "),
                worktree.display()
            )
        })?;
    command_result(binary, arguments, output)
}

fn command_result(binary: &str, arguments: &[&str], output: Output) -> Result<()> {
    if !output.status.success() {
        bail!(
            "{binary} {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn vcs_command(worktree: &Path, binary: &str) -> Command {
    let mut command = Command::new(binary);
    command.current_dir(worktree);
    crate::launch::scrub_internal_child_env(&mut command);
    super::anchor_git_worktree_environment(&mut command, worktree);
    command
}
