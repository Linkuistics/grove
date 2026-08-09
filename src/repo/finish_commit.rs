use super::{vcs_of, Vcs};
use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const INDEX_BACKUP_NAME: &str = "grove-finish-index";
const INDEX_SUCCESS_NAME: &str = "grove-finish-success-index";

pub(crate) struct PreparedGitFinish {
    worktree: PathBuf,
    start_head: String,
    deletion_fingerprint: [u8; 32],
    index_backup: GitIndexBackup,
    hooks_path: PathBuf,
}

pub(crate) enum FinishCommitOutcome {
    Committed(GitFinishProof),
    NotCommitted {
        proof: GitStartProof,
        error: anyhow::Error,
    },
    RecoveryPending(anyhow::Error),
}

pub(crate) struct GitFinishProof {
    worktree: PathBuf,
    start_head: String,
    message: String,
    deletion_fingerprint: [u8; 32],
}

pub(crate) struct GitStartProof {
    worktree: PathBuf,
    start_head: String,
}

impl GitFinishProof {
    pub(crate) fn revalidate(&self) -> Result<()> {
        validate_exact_git_finish(
            &self.worktree,
            &self.start_head,
            &self.message,
            self.deletion_fingerprint,
        )
    }
}

impl GitStartProof {
    pub(crate) fn revalidate(&self) -> Result<()> {
        let observed = git_stdout(&self.worktree, &["rev-parse", "HEAD"])?;
        if observed != self.start_head {
            bail!(
                "Recovery pending: plain-Git finish recorded start {}, but observed HEAD {}",
                self.start_head,
                observed
            );
        }
        Ok(())
    }
}

impl PreparedGitFinish {
    pub(crate) fn start_head(&self) -> &str {
        &self.start_head
    }

    pub(crate) fn deletion_fingerprint(&self) -> [u8; 32] {
        self.deletion_fingerprint
    }

    pub(crate) fn commit(self, finish_handle: &str, attempt_identity: &str) -> FinishCommitOutcome {
        let message = finish_commit_message(finish_handle, attempt_identity);
        let witness_exclusion = format!(":(exclude).grove/FINISHING-{finish_handle}");
        let hooks_configuration = format!("core.hooksPath={}", self.hooks_path.display());
        let result = (|| {
            run_vcs_command(
                &self.worktree,
                "git",
                &["add", "-A", "--", ".grove", &witness_exclusion],
            )?;
            run_vcs_command(
                &self.worktree,
                "git",
                &[
                    "-c",
                    &hooks_configuration,
                    "commit",
                    "--only",
                    "-m",
                    &message,
                    "--",
                    ".grove",
                    &witness_exclusion,
                ],
            )
        })();
        self.classify_command_result(&message, result)
    }

    fn classify_command_result(
        self,
        message: &str,
        command_result: Result<()>,
    ) -> FinishCommitOutcome {
        let committed = GitFinishProof {
            worktree: self.worktree.clone(),
            start_head: self.start_head.clone(),
            message: message.to_owned(),
            deletion_fingerprint: self.deletion_fingerprint,
        };
        if committed.revalidate().is_ok() {
            self.index_backup.discard();
            return FinishCommitOutcome::Committed(committed);
        }

        let command_error = match command_result {
            Err(error) => error,
            Ok(()) => anyhow::anyhow!(
                "Git reported a successful finish commit, but the exact scoped result could not be proven"
            ),
        };
        let start = GitStartProof {
            worktree: self.worktree,
            start_head: self.start_head,
        };
        if let Err(topology_error) = start.revalidate() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "{topology_error:#}; preserve divergent work, restore the recorded start or the exact teardown result, then retry"
            )));
        }
        if let Err(index_error) = self.index_backup.restore() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "restoring the original Git index failed: {index_error:#}"
            )));
        }
        if let Err(topology_error) = start.revalidate() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "Git topology changed after index restoration: {topology_error:#}"
            )));
        }
        FinishCommitOutcome::NotCommitted {
            proof: start,
            error: command_error,
        }
    }
}

pub(crate) fn prepare_plain_git_finish(worktree: &Path) -> Result<PreparedGitFinish> {
    validate_finish_commit(worktree)?;
    let start_head = git_stdout(worktree, &["rev-parse", "HEAD"])
        .context("recording plain-Git finish start HEAD")?;
    let deletion_fingerprint = tracked_grove_fingerprint(worktree, &start_head)?;
    let hooks_path = prepare_empty_hooks_directory(worktree)?;
    let index_backup = preserve_git_index(worktree)?;
    Ok(PreparedGitFinish {
        worktree: worktree.to_path_buf(),
        start_head,
        deletion_fingerprint,
        index_backup,
        hooks_path,
    })
}

fn prepare_empty_hooks_directory(worktree: &Path) -> Result<PathBuf> {
    let control = super::workspace_control(worktree)?;
    fs::create_dir_all(control.control_dir()).with_context(|| {
        format!(
            "creating workspace-control directory for finish hooks {}",
            control.control_dir().display()
        )
    })?;
    let hooks_path = control.control_dir().join("finish-hooks-empty");
    match fs::create_dir(&hooks_path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(&hooks_path)
                .with_context(|| format!("checking finish hooks path {}", hooks_path.display()))?;
            if !metadata.file_type().is_dir() {
                bail!(
                    "finish hooks path is not a directory: {}",
                    hooks_path.display()
                );
            }
            if fs::read_dir(&hooks_path)
                .with_context(|| format!("reading finish hooks path {}", hooks_path.display()))?
                .next()
                .is_some()
            {
                bail!("finish hooks path is not empty: {}", hooks_path.display());
            }
        }
        Err(error) => {
            return Err(error)
                .with_context(|| format!("creating finish hooks path {}", hooks_path.display()))
        }
    }
    Ok(hooks_path)
}

fn finish_commit_message(finish_handle: &str, attempt_identity: &str) -> String {
    format!("{finish_handle} (finish attempt {attempt_identity}): remove completed grove task tree")
}

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
pub fn commit_finish(worktree: &Path, finish_handle: &str, attempt_identity: &str) -> Result<()> {
    let message = finish_commit_message(finish_handle, attempt_identity);
    match vcs_of(worktree) {
        Some(Vcs::Git) => {
            match prepare_plain_git_finish(worktree)?.commit(finish_handle, attempt_identity) {
                FinishCommitOutcome::Committed(_) => Ok(()),
                FinishCommitOutcome::NotCommitted { error, .. }
                | FinishCommitOutcome::RecoveryPending(error) => Err(error),
            }
        }
        Some(Vcs::Jj { workspace_root }) => commit_jj_finish(&workspace_root, &message),
        None => bail!(
            "cannot commit the finished grove: {} is not a git or jj working tree",
            worktree.display()
        ),
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

fn validate_exact_git_finish(
    worktree: &Path,
    start_head: &str,
    message: &str,
    deletion_fingerprint: [u8; 32],
) -> Result<()> {
    let observed_head = git_stdout(worktree, &["rev-parse", "HEAD"])?;
    if observed_head == start_head {
        bail!("the plain-Git finish result is absent: HEAD is still {start_head}");
    }
    let observed_parent = git_stdout(worktree, &["rev-parse", "HEAD^"])?;
    if observed_parent != start_head {
        bail!(
            "the plain-Git finish result is not immediate: recorded start {start_head}, observed parent {observed_parent}"
        );
    }
    let observed_message = git_stdout(worktree, &["log", "-1", "--format=%s", "HEAD"])?;
    if observed_message != message {
        bail!(
            "the plain-Git finish result has the wrong message: expected {message:?}, observed {observed_message:?}"
        );
    }
    if !tracked_grove_bytes(worktree, "HEAD")?.is_empty() {
        bail!("the plain-Git finish result still tracks `.grove/`");
    }
    if tracked_grove_fingerprint(worktree, start_head)? != deletion_fingerprint {
        bail!("the recorded plain-Git deletion fingerprint no longer matches its start anchor");
    }

    let changed = git_bytes(
        worktree,
        &["diff", "--name-status", "-z", start_head, "HEAD", "--", "."],
    )?;
    let fields = changed
        .split(|byte| *byte == 0)
        .filter(|field| !field.is_empty())
        .collect::<Vec<_>>();
    if fields.is_empty()
        || fields.len() % 2 != 0
        || fields
            .chunks_exact(2)
            .any(|entry| entry[0] != b"D" || !entry[1].starts_with(b".grove/"))
    {
        bail!("the plain-Git finish result changes paths outside the exact `.grove/` deletion");
    }
    Ok(())
}

fn tracked_grove_fingerprint(worktree: &Path, revision: &str) -> Result<[u8; 32]> {
    let entries = tracked_grove_bytes(worktree, revision)?;
    if entries.is_empty() {
        bail!(
            "cannot finish this plain-Git grove: `.grove/` has no tracked state in {revision}, so no focused deletion commit can be recorded"
        );
    }
    Ok(Sha256::digest(entries).into())
}

fn tracked_grove_bytes(worktree: &Path, revision: &str) -> Result<Vec<u8>> {
    git_bytes(
        worktree,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--full-tree",
            revision,
            "--",
            ".grove",
        ],
    )
}

fn git_stdout(worktree: &Path, arguments: &[&str]) -> Result<String> {
    Ok(String::from_utf8(git_bytes(worktree, arguments)?)?
        .trim()
        .to_owned())
}

fn git_bytes(worktree: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = vcs_command(worktree, "git")
        .args(arguments)
        .output()
        .with_context(|| {
            format!(
                "running git {} in {}",
                arguments.join(" "),
                worktree.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
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

#[cfg(test)]
mod tests {
    use super::{
        finish_commit_message, prepare_plain_git_finish, run_vcs_command, FinishCommitOutcome,
    };
    use anyhow::anyhow;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn exact_git_finish_result_reclassifies_a_lost_command_as_committed() {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path();
        let init = Command::new("git")
            .current_dir(repository)
            .args(["init", "-q", "."])
            .output()
            .unwrap();
        assert!(
            init.status.success(),
            "{}",
            String::from_utf8_lossy(&init.stderr)
        );
        run_vcs_command(repository, "git", &["config", "user.name", "Grove Test"]).unwrap();
        run_vcs_command(
            repository,
            "git",
            &["config", "user.email", "grove-test@example.com"],
        )
        .unwrap();
        fs::create_dir(repository.join(".grove")).unwrap();
        fs::write(repository.join(".grove/FORMAT"), "session-kinds-v1\n").unwrap();
        fs::write(repository.join("outside"), "keep\n").unwrap();
        run_vcs_command(repository, "git", &["add", "-A"]).unwrap();
        run_vcs_command(repository, "git", &["commit", "-q", "-m", "fixture"]).unwrap();

        let prepared = prepare_plain_git_finish(repository).unwrap();
        let message = finish_commit_message("finish-k2", "11111111111111111111111111111111");
        fs::remove_dir_all(repository.join(".grove")).unwrap();
        run_vcs_command(repository, "git", &["add", "-A", "--", ".grove"]).unwrap();
        run_vcs_command(
            repository,
            "git",
            &["commit", "--only", "-m", &message, "--", ".grove"],
        )
        .unwrap();

        let outcome = prepared.classify_command_result(&message, Err(anyhow!("lost result")));

        let FinishCommitOutcome::Committed(proof) = outcome else {
            panic!("the exact committed result was not classified as committed");
        };
        proof.revalidate().unwrap();

        fs::write(repository.join("outside"), "changed\n").unwrap();
        run_vcs_command(repository, "git", &["add", "outside"]).unwrap();
        run_vcs_command(repository, "git", &["commit", "-q", "-m", "divergent"]).unwrap();
        let error = proof.revalidate().unwrap_err();
        assert!(error.to_string().contains("not immediate"), "{error:#}");
    }
}
