use super::{vcs_of, Vcs};
use anyhow::{bail, Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const INDEX_BACKUP_NAME: &str = "grove-finish-index";
const INDEX_SUCCESS_NAME: &str = "grove-finish-success-index";

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
    let backup = preserve_git_index(worktree)?;
    let result = (|| {
        run_vcs_command(worktree, "git", &["add", "-A", "--", ".grove"])?;
        run_vcs_command(
            worktree,
            "git",
            &["commit", "--only", "-m", message, "--", ".grove"],
        )
    })();

    match result {
        Ok(()) => {
            backup.discard();
            Ok(())
        }
        Err(command_error) => match backup.restore() {
            Ok(()) => Err(command_error),
            Err(restore_error) => Err(command_error.context(format!(
                "also failed to restore the Git index after the finish commit: {restore_error}"
            ))),
        },
    }
}

fn commit_jj_finish(worktree: &Path, message: &str) -> Result<()> {
    if !worktree.join(".git").exists() {
        return run_vcs_command(worktree, "jj", &["commit", "-m", message, "root:.grove"]);
    }

    let backup = preserve_git_index(worktree)?;
    let success_index = match backup.prepare_without_grove(worktree) {
        Ok(success_index) => success_index,
        Err(error) => {
            backup.discard();
            return Err(error);
        }
    };

    let result = run_vcs_command(worktree, "jj", &["commit", "-m", message, "root:.grove"]);
    match result {
        Ok(()) => backup.activate(success_index),
        Err(command_error) => {
            discard_temporary_index(success_index.as_deref());
            match backup.restore() {
                Ok(()) => Err(command_error),
                Err(restore_error) => Err(command_error.context(format!(
                    "also failed to restore the colocated Git index after the finish commit: {restore_error}"
                ))),
            }
        }
    }
}

struct GitIndexBackup {
    git_index: PathBuf,
    backup_index: PathBuf,
    had_git_index: bool,
}

impl GitIndexBackup {
    fn prepare_without_grove(&self, worktree: &Path) -> Result<Option<PathBuf>> {
        if !self.had_git_index {
            return Ok(None);
        }

        let success_index = self.git_index.with_file_name(INDEX_SUCCESS_NAME);
        fs::copy(&self.backup_index, &success_index).with_context(|| {
            format!(
                "preparing the successful colocated Git index {}",
                success_index.display()
            )
        })?;
        if let Err(error) = remove_grove_entries(worktree, &success_index) {
            discard_temporary_index(Some(&success_index));
            return Err(
                error.context("preparing the colocated Git index before the Jujutsu finish commit")
            );
        }
        Ok(Some(success_index))
    }

    fn restore(self) -> Result<()> {
        restore_git_index(&self.git_index, &self.backup_index, self.had_git_index)
    }

    fn activate(self, success_index: Option<PathBuf>) -> Result<()> {
        if let Some(success_index) = success_index {
            fs::rename(&success_index, &self.git_index).with_context(|| {
                format!(
                    "activating the prepared colocated Git index {}",
                    success_index.display()
                )
            })?;
        } else {
            restore_git_index(&self.git_index, &self.backup_index, false)?;
        }
        self.discard();
        Ok(())
    }

    fn discard(self) {
        if let Err(error) = fs::remove_file(&self.backup_index) {
            if error.kind() != std::io::ErrorKind::NotFound {
                eprintln!(
                    "warning: remove the temporary Git index {} after the finish commit: {error}",
                    self.backup_index.display()
                );
            }
        }
    }
}

fn discard_temporary_index(path: Option<&Path>) {
    let Some(path) = path else {
        return;
    };
    if let Err(error) = fs::remove_file(path) {
        if error.kind() != std::io::ErrorKind::NotFound {
            eprintln!(
                "warning: remove the temporary Git index {} after the finish commit: {error}",
                path.display()
            );
        }
    }
}

fn preserve_git_index(worktree: &Path) -> Result<GitIndexBackup> {
    let git_index = git_path(worktree, "index")?;
    let backup_index = git_index.with_file_name(INDEX_BACKUP_NAME);
    let had_git_index = git_index.exists();
    if had_git_index {
        fs::copy(&git_index, &backup_index).with_context(|| {
            format!(
                "preserving Git index {} before the finish commit",
                git_index.display()
            )
        })?;
    }
    Ok(GitIndexBackup {
        git_index,
        backup_index,
        had_git_index,
    })
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

fn remove_grove_entries(worktree: &Path, git_index: &Path) -> Result<()> {
    let grove_paths = vcs_command(worktree, "git")
        .env("GIT_INDEX_FILE", git_index)
        .args(["ls-files", "-z", "--", ".grove"])
        .output()
        .context("listing grove entries in the preserved colocated Git index")?;
    if !grove_paths.status.success() {
        bail!(
            "git ls-files -z -- .grove failed: {}",
            String::from_utf8_lossy(&grove_paths.stderr).trim()
        );
    }
    if grove_paths.stdout.is_empty() {
        return Ok(());
    }

    let mut child = vcs_command(worktree, "git")
        .env("GIT_INDEX_FILE", git_index)
        .args(["update-index", "--force-remove", "-z", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("starting Git index cleanup for the completed grove")?;
    child
        .stdin
        .take()
        .context("opening stdin for Git index cleanup")?
        .write_all(&grove_paths.stdout)
        .context("writing completed-grove paths to Git index cleanup")?;
    let output = child
        .wait_with_output()
        .context("waiting for Git index cleanup for the completed grove")?;
    command_result(
        "git",
        &["update-index", "--force-remove", "-z", "--stdin"],
        output,
    )
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
