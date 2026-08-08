use super::{vcs_of, Vcs};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const GIT_PATHS: [&str; 2] = [".grove", ":(exclude).grove/MIGRATING-session-kinds"];
const JJ_FILESET: &str = "root:.grove ~ root:.grove/MIGRATING-session-kinds";
const INDEX_BACKUP_NAME: &str = "grove-session-kind-migration-index";

/// Record the automatic session-kind migration without absorbing changes
/// outside `.grove/` or the live fail-closed migration witness.
pub fn commit_session_kind_migration(worktree: &Path, grove_name: &str) -> Result<()> {
    let message = format!("grove({grove_name}): migrate task tree to session-kind filenames");
    match vcs_of(worktree) {
        Some(Vcs::Jj { workspace_root }) => commit_jj_migration(&workspace_root, &message),
        Some(Vcs::Git) => commit_git_migration(worktree, &message),
        None => bail!(
            "cannot commit the session-kind migration: {} is not a git or jj working tree",
            worktree.display()
        ),
    }
}

fn commit_git_migration(worktree: &Path, message: &str) -> Result<()> {
    let git_index = git_path(worktree, "index")?;
    let backup_index = git_index.with_file_name(INDEX_BACKUP_NAME);
    if backup_index.exists() {
        if git_migration_already_committed(worktree, message)? {
            if let Err(error) = fs::remove_file(&backup_index) {
                eprintln!(
                    "warning: remove recovered Git index backup {}: {error}",
                    backup_index.display()
                );
            }
            return Ok(());
        }
        restore_git_index(&git_index, &backup_index, true).with_context(|| {
            format!(
                "recovering interrupted migration commit from {}",
                backup_index.display()
            )
        })?;
    }
    if git_migration_already_committed(worktree, message)? {
        return Ok(());
    }
    let had_git_index = git_index.exists();
    if had_git_index {
        fs::copy(&git_index, &backup_index).with_context(|| {
            format!(
                "copying Git index {} to {}",
                git_index.display(),
                backup_index.display()
            )
        })?;
    }

    let commit_result = (|| {
        run_vcs_command(
            worktree,
            "git",
            &["add", "-A", "--", GIT_PATHS[0], GIT_PATHS[1]],
        )?;
        run_vcs_command(
            worktree,
            "git",
            &[
                "commit",
                "--only",
                "-m",
                message,
                "--",
                GIT_PATHS[0],
                GIT_PATHS[1],
            ],
        )
    })();

    match commit_result {
        Ok(()) => {
            if let Err(error) = fs::remove_file(&backup_index) {
                if error.kind() != std::io::ErrorKind::NotFound {
                    eprintln!(
                        "warning: remove the temporary Git index {} after the migration commit: {error}",
                        backup_index.display()
                    );
                }
            }
            Ok(())
        }
        Err(commit_error) => match restore_git_index(&git_index, &backup_index, had_git_index) {
            Ok(()) => Err(commit_error),
            Err(restore_error) => Err(commit_error.context(format!(
                "also failed to restore the Git index {}: {restore_error}",
                git_index.display()
            ))),
        },
    }
}

fn git_migration_already_committed(worktree: &Path, message: &str) -> Result<bool> {
    let subject = Command::new("git")
        .args(["log", "-1", "--format=%s"])
        .current_dir(worktree)
        .output()
        .with_context(|| format!("reading Git HEAD subject in {}", worktree.display()))?;
    if !subject.status.success()
        || String::from_utf8(subject.stdout)
            .context("git log output is not UTF-8")?
            .trim()
            != message
    {
        return Ok(false);
    }

    let diff = Command::new("git")
        .args(["diff", "--quiet", "HEAD", "--", GIT_PATHS[0], GIT_PATHS[1]])
        .current_dir(worktree)
        .output()
        .with_context(|| format!("checking recovered Git migration in {}", worktree.display()))?;
    match diff.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => bail!(
            "git diff --quiet HEAD failed while checking recovered migration in {}: {}",
            worktree.display(),
            String::from_utf8_lossy(&diff.stderr).trim()
        ),
    }
}

fn restore_git_index(git_index: &Path, backup_index: &Path, had_git_index: bool) -> Result<()> {
    if had_git_index {
        fs::rename(backup_index, git_index).with_context(|| {
            format!(
                "restoring Git index {} from {}",
                git_index.display(),
                backup_index.display()
            )
        })
    } else {
        match fs::remove_file(git_index) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error).with_context(|| {
                format!("removing newly created Git index {}", git_index.display())
            }),
        }
    }
}

fn commit_jj_migration(worktree: &Path, message: &str) -> Result<()> {
    let arguments = ["commit", "-m", message, JJ_FILESET];
    if !worktree.join(".git").exists() {
        if jj_migration_already_committed(worktree, message)? {
            return Ok(());
        }
        return run_vcs_command(worktree, "jj", &arguments);
    }

    let git_index = git_path(worktree, "index")?;
    let alternate_index = git_index.with_file_name(INDEX_BACKUP_NAME);
    if alternate_index.exists() {
        let already_committed = jj_migration_already_committed(worktree, message)?;
        restore_git_index(&git_index, &alternate_index, true).with_context(|| {
            format!(
                "recovering interrupted Jujutsu migration commit from {}",
                alternate_index.display()
            )
        })?;
        if already_committed {
            return Ok(());
        }
    }
    let had_git_index = git_index.exists();
    if had_git_index {
        fs::copy(&git_index, &alternate_index).with_context(|| {
            format!(
                "copying colocated Git index {} to {}",
                git_index.display(),
                alternate_index.display()
            )
        })?;
    }

    let result = (|| {
        if jj_migration_already_committed(worktree, message)? {
            return Ok(());
        }
        run_vcs_command(worktree, "jj", &arguments)
    })();
    let restore = if had_git_index {
        fs::rename(&alternate_index, &git_index)
    } else {
        fs::remove_file(&git_index)
    };
    match (result, restore) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        (Ok(()), Err(error)) => {
            eprintln!(
                "warning: restore the colocated Git index {} after the migration commit: {error}",
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

fn jj_migration_already_committed(worktree: &Path, message: &str) -> Result<bool> {
    let description = Command::new("jj")
        .args(["log", "-r", "@-", "--no-graph", "-T", "description"])
        .current_dir(worktree)
        .output()
        .with_context(|| {
            format!(
                "reading Jujutsu parent description in {}",
                worktree.display()
            )
        })?;
    if !description.status.success() {
        bail!(
            "jj log failed while checking recovered migration in {}: {}",
            worktree.display(),
            String::from_utf8_lossy(&description.stderr).trim()
        );
    }
    if String::from_utf8(description.stdout)
        .context("jj log output is not UTF-8")?
        .trim()
        != message
    {
        return Ok(false);
    }

    let scoped_diff = Command::new("jj")
        .args(["diff", "-r", "@", "--summary", JJ_FILESET])
        .current_dir(worktree)
        .output()
        .with_context(|| {
            format!(
                "checking recovered Jujutsu migration in {}",
                worktree.display()
            )
        })?;
    if !scoped_diff.status.success() {
        bail!(
            "jj diff failed while checking recovered migration in {}: {}",
            worktree.display(),
            String::from_utf8_lossy(&scoped_diff.stderr).trim()
        );
    }
    Ok(scoped_diff.stdout.is_empty())
}

fn git_path(worktree: &Path, name: &str) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", name])
        .current_dir(worktree)
        .output()
        .with_context(|| format!("resolving Git {name} path in {}", worktree.display()))?;
    if !output.status.success() {
        bail!(
            "git rev-parse --git-path {name} failed in {}: {}",
            worktree.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let path = PathBuf::from(
        String::from_utf8(output.stdout)
            .context("git --git-path output is not UTF-8")?
            .trim(),
    );
    Ok(if path.is_absolute() {
        path
    } else {
        worktree.join(path)
    })
}

fn run_vcs_command(worktree: &Path, binary: &str, arguments: &[&str]) -> Result<()> {
    let output = Command::new(binary)
        .current_dir(worktree)
        .args(arguments)
        .output()
        .with_context(|| {
            format!(
                "running {binary} {} in {}",
                arguments.join(" "),
                worktree.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "{binary} {} failed in {}: {}",
            arguments.join(" "),
            worktree.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}
