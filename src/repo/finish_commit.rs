use super::{vcs_of, Vcs};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
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

pub(crate) struct PreparedJjFinish {
    worktree: PathBuf,
    start: JjStartAnchor,
    deletion_fingerprint: [u8; 32],
    index_backup: Option<GitIndexBackup>,
    success_index: Option<PathBuf>,
}

pub(crate) enum PreparedFinish {
    Git(PreparedGitFinish),
    Jj(PreparedJjFinish),
}

pub(crate) enum FinishCommitOutcome {
    Committed(FinishProof),
    NotCommitted {
        proof: FinishStartProof,
        error: anyhow::Error,
    },
    RecoveryPending(anyhow::Error),
}

pub(crate) enum FinishRecoveryOutcome {
    Committed(FinishProof),
    NotCommitted(FinishStartProof),
    RecoveryPending(anyhow::Error),
}

pub(crate) enum FinishProof {
    Git(GitFinishProof),
    Jj(JjFinishProof),
}

pub(crate) enum FinishStartProof {
    Git(GitStartProof),
    Jj(JjStartProof),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "vcs", rename_all = "kebab-case")]
pub(crate) enum FinishStartAnchor {
    PlainGit {
        head: String,
        #[serde(default = "default_had_git_index")]
        had_git_index: bool,
    },
    Jujutsu {
        commit_id: String,
        change_id: String,
        parent_commit_ids: Vec<String>,
        git_index_existed: Option<bool>,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct JjStartAnchor {
    commit_id: String,
    change_id: String,
    parent_commit_ids: Vec<String>,
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
    index_backup: Option<GitIndexBackup>,
}

fn default_had_git_index() -> bool {
    true
}

pub(crate) struct JjFinishProof {
    worktree: PathBuf,
    start: JjStartAnchor,
    message: String,
    deletion_fingerprint: [u8; 32],
}

pub(crate) struct JjStartProof {
    worktree: PathBuf,
    start: JjStartAnchor,
    message: String,
    deletion_fingerprint: [u8; 32],
    index_backup: Option<GitIndexBackup>,
    auto_track_configuration: String,
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

    fn restore_index_preserving_backup(&self) -> Result<()> {
        match &self.index_backup {
            Some(backup) => backup.restore_preserving_backup(),
            None => Ok(()),
        }
    }

    fn revalidate_after_rollback(mut self) -> Result<()> {
        self.revalidate()?;
        if let Some(backup) = self.index_backup.take() {
            backup.discard();
        }
        Ok(())
    }
}

impl JjFinishProof {
    fn revalidate(&self) -> Result<()> {
        validate_exact_jj_finish(
            &self.worktree,
            &self.start,
            &self.message,
            self.deletion_fingerprint,
        )
    }
}

impl JjStartProof {
    fn revalidate_before_rollback(&self) -> Result<()> {
        validate_jj_uncommitted(
            &self.worktree,
            &self.start,
            &self.message,
            self.deletion_fingerprint,
        )
    }

    fn restore_index_preserving_backup(&mut self) -> Result<()> {
        match &self.index_backup {
            Some(backup) => backup.restore_preserving_backup(),
            None => Ok(()),
        }
    }

    fn revalidate_after_rollback(mut self) -> Result<()> {
        let observed = jj_topology(
            &self.worktree,
            "@",
            false,
            Some(&self.auto_track_configuration),
        )
        .context("snapshotting the restored Jujutsu finish tree")?;
        if observed != self.start {
            bail!(
                "Recovery pending: restored Jujutsu finish expected preflight commit {}, change {} at parents {:?}, but observed commit {}, change {} at parents {:?}",
                self.start.commit_id,
                self.start.change_id,
                self.start.parent_commit_ids,
                observed.commit_id,
                observed.change_id,
                observed.parent_commit_ids
            );
        }
        if jj_deletion_fingerprint(&self.worktree, &self.start)? != self.deletion_fingerprint {
            bail!(
                "Recovery pending: restored Jujutsu grove does not match its preflight fingerprint"
            );
        }
        self.restore_index_preserving_backup()?;
        if let Some(backup) = self.index_backup.take() {
            backup.discard();
        }
        Ok(())
    }
}

impl FinishProof {
    pub(crate) fn revalidate(&self) -> Result<()> {
        match self {
            Self::Git(proof) => proof.revalidate(),
            Self::Jj(proof) => proof.revalidate(),
        }
    }
}

impl FinishStartProof {
    pub(crate) fn revalidate_before_rollback(&self) -> Result<()> {
        match self {
            Self::Git(proof) => proof.revalidate(),
            Self::Jj(proof) => proof.revalidate_before_rollback(),
        }
    }

    pub(crate) fn revalidate_after_rollback(self) -> Result<()> {
        match self {
            Self::Git(proof) => proof.revalidate_after_rollback(),
            Self::Jj(proof) => proof.revalidate_after_rollback(),
        }
    }
}

impl PreparedFinish {
    pub(crate) fn start_anchor(&self) -> FinishStartAnchor {
        match self {
            Self::Git(prepared) => FinishStartAnchor::PlainGit {
                head: prepared.start_head.clone(),
                had_git_index: prepared.index_backup.had_git_index,
            },
            Self::Jj(prepared) => FinishStartAnchor::Jujutsu {
                commit_id: prepared.start.commit_id.clone(),
                change_id: prepared.start.change_id.clone(),
                parent_commit_ids: prepared.start.parent_commit_ids.clone(),
                git_index_existed: prepared
                    .index_backup
                    .as_ref()
                    .map(|backup| backup.had_git_index),
            },
        }
    }

    pub(crate) fn deletion_fingerprint(&self) -> [u8; 32] {
        match self {
            Self::Git(prepared) => prepared.deletion_fingerprint(),
            Self::Jj(prepared) => prepared.deletion_fingerprint,
        }
    }

    pub(crate) fn commit(self, finish_handle: &str, attempt_identity: &str) -> FinishCommitOutcome {
        match self {
            Self::Git(prepared) => prepared.commit(finish_handle, attempt_identity),
            Self::Jj(prepared) => prepared.commit(finish_handle, attempt_identity),
        }
    }
}

impl PreparedGitFinish {
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
            return FinishCommitOutcome::Committed(FinishProof::Git(committed));
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
            index_backup: Some(self.index_backup),
        };
        if let Err(topology_error) = start.revalidate() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "{topology_error:#}; preserve divergent work, restore the recorded start or the exact teardown result, then retry"
            )));
        }
        if let Err(index_error) = start.restore_index_preserving_backup() {
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
            proof: FinishStartProof::Git(start),
            error: command_error,
        }
    }
}

pub(crate) fn prepare_finish(worktree: &Path) -> Result<PreparedFinish> {
    match vcs_of(worktree) {
        Some(Vcs::Git) => Ok(PreparedFinish::Git(prepare_plain_git_finish(worktree)?)),
        Some(Vcs::Jj { workspace_root }) => {
            Ok(PreparedFinish::Jj(prepare_jj_finish(&workspace_root)?))
        }
        None => bail!(
            "cannot commit the finished grove: {} is not a git or jj working tree",
            worktree.display()
        ),
    }
}

pub(crate) fn recover_finish(
    worktree: &Path,
    start: &FinishStartAnchor,
    finish_handle: &str,
    attempt_identity: &str,
    deletion_fingerprint: [u8; 32],
) -> FinishRecoveryOutcome {
    let message = finish_commit_message(finish_handle, attempt_identity);
    match (vcs_of(worktree), start) {
        (
            Some(Vcs::Git),
            FinishStartAnchor::PlainGit {
                head,
                had_git_index,
            },
        ) => recover_plain_git_finish(
            worktree,
            head,
            *had_git_index,
            &message,
            deletion_fingerprint,
        ),
        (
            Some(Vcs::Jj { workspace_root }),
            FinishStartAnchor::Jujutsu {
                commit_id,
                change_id,
                parent_commit_ids,
                git_index_existed,
            },
        ) => recover_jj_finish(
            &workspace_root,
            JjStartAnchor {
                commit_id: commit_id.clone(),
                change_id: change_id.clone(),
                parent_commit_ids: parent_commit_ids.clone(),
            },
            finish_handle,
            *git_index_existed,
            &message,
            deletion_fingerprint,
        ),
        (Some(Vcs::Git), FinishStartAnchor::Jujutsu { .. }) => {
            FinishRecoveryOutcome::RecoveryPending(anyhow!(
                "Recovery pending: finish manifest records Jujutsu, but the working tree is plain Git"
            ))
        }
        (Some(Vcs::Jj { .. }), FinishStartAnchor::PlainGit { .. }) => {
            FinishRecoveryOutcome::RecoveryPending(anyhow!(
                "Recovery pending: finish manifest records plain Git, but the working tree is Jujutsu"
            ))
        }
        (None, _) => FinishRecoveryOutcome::RecoveryPending(anyhow!(
            "Recovery pending: {} is no longer a git or jj working tree",
            worktree.display()
        )),
    }
}

fn recover_plain_git_finish(
    worktree: &Path,
    start_head: &str,
    had_git_index: bool,
    message: &str,
    deletion_fingerprint: [u8; 32],
) -> FinishRecoveryOutcome {
    let committed = GitFinishProof {
        worktree: worktree.to_path_buf(),
        start_head: start_head.to_owned(),
        message: message.to_owned(),
        deletion_fingerprint,
    };
    if committed.revalidate().is_ok() {
        if let Ok(backup) = existing_git_index_backup(worktree, had_git_index) {
            backup.discard();
        }
        return FinishRecoveryOutcome::Committed(FinishProof::Git(committed));
    }

    let start = GitStartProof {
        worktree: worktree.to_path_buf(),
        start_head: start_head.to_owned(),
        index_backup: None,
    };
    if let Err(error) = start.revalidate() {
        return FinishRecoveryOutcome::RecoveryPending(error.context(
            "preserve divergent work, restore the recorded start or the exact teardown result, then retry",
        ));
    }
    let backup = match existing_git_index_backup(worktree, had_git_index) {
        Ok(backup) => backup,
        Err(error) => return FinishRecoveryOutcome::RecoveryPending(error),
    };
    let start = GitStartProof {
        index_backup: Some(backup),
        ..start
    };
    if let Err(error) = start.restore_index_preserving_backup() {
        return FinishRecoveryOutcome::RecoveryPending(
            error.context("restoring the original Git index during finish recovery"),
        );
    }
    if let Err(error) = start.revalidate() {
        return FinishRecoveryOutcome::RecoveryPending(
            error.context("Git topology changed after index restoration during finish recovery"),
        );
    }
    FinishRecoveryOutcome::NotCommitted(FinishStartProof::Git(start))
}

fn recover_jj_finish(
    worktree: &Path,
    start: JjStartAnchor,
    finish_handle: &str,
    git_index_existed: Option<bool>,
    message: &str,
    deletion_fingerprint: [u8; 32],
) -> FinishRecoveryOutcome {
    if git_index_existed.is_some() != worktree.join(".git").exists() {
        return FinishRecoveryOutcome::RecoveryPending(anyhow!(
            "Recovery pending: the recorded Jujutsu repository shape no longer matches the presence of its colocated Git repository"
        ));
    }
    let committed = JjFinishProof {
        worktree: worktree.to_path_buf(),
        start: start.clone(),
        message: message.to_owned(),
        deletion_fingerprint,
    };
    if committed.revalidate().is_ok() {
        if let Err(error) = activate_recovered_jj_index(worktree, git_index_existed) {
            return FinishRecoveryOutcome::RecoveryPending(error);
        }
        return FinishRecoveryOutcome::Committed(FinishProof::Jj(committed));
    }

    let auto_track_configuration = format!(
        "snapshot.auto-track={:?}",
        format!("all() ~ root:.grove/FINISHING-{finish_handle}")
    );
    let mut start = JjStartProof {
        worktree: worktree.to_path_buf(),
        start,
        message: message.to_owned(),
        deletion_fingerprint,
        index_backup: match recover_jj_index_backup(worktree, git_index_existed) {
            Ok(backup) => backup,
            Err(error) => return FinishRecoveryOutcome::RecoveryPending(error),
        },
        auto_track_configuration,
    };
    if let Err(error) = start.revalidate_before_rollback() {
        return FinishRecoveryOutcome::RecoveryPending(error.context(
            "preserve divergent work, restore the recorded start or the exact teardown result, then retry",
        ));
    }
    discard_temporary_index(recovered_success_index(worktree).ok().flatten().as_deref());
    if let Err(error) = start.restore_index_preserving_backup() {
        return FinishRecoveryOutcome::RecoveryPending(
            error.context("restoring the original colocated Git index during finish recovery"),
        );
    }
    if let Err(error) = start.revalidate_before_rollback() {
        return FinishRecoveryOutcome::RecoveryPending(
            error
                .context("Jujutsu topology changed after index restoration during finish recovery"),
        );
    }
    FinishRecoveryOutcome::NotCommitted(FinishStartProof::Jj(start))
}

fn recover_jj_index_backup(
    worktree: &Path,
    git_index_existed: Option<bool>,
) -> Result<Option<GitIndexBackup>> {
    git_index_existed
        .map(|had_git_index| existing_git_index_backup(worktree, had_git_index))
        .transpose()
}

fn recovered_success_index(worktree: &Path) -> Result<Option<PathBuf>> {
    if !worktree.join(".git").exists() {
        return Ok(None);
    }
    let git_index = git_path(worktree, "index")?;
    Ok(Some(git_index.with_file_name(INDEX_SUCCESS_NAME)))
}

fn activate_recovered_jj_index(worktree: &Path, git_index_existed: Option<bool>) -> Result<()> {
    let Some(had_git_index) = git_index_existed else {
        return Ok(());
    };

    let git_index = git_path(worktree, "index")?;
    let backup_index = git_index.with_file_name(INDEX_BACKUP_NAME);
    if !had_git_index {
        return GitIndexBackup {
            git_index,
            backup_index,
            had_git_index: false,
        }
        .activate(None)
        .context("restoring the absent preflight Git index after the Jujutsu finish commit");
    }
    let success_index = git_index.with_file_name(INDEX_SUCCESS_NAME);
    match (backup_index.is_file(), success_index.is_file()) {
        (true, true) => GitIndexBackup {
            git_index,
            backup_index,
            had_git_index: true,
        }
        .activate(Some(success_index))
        .context("activating the recovered colocated Git success index"),
        (true, false) => {
            validate_active_jj_finish_index(worktree)?;
            GitIndexBackup {
                git_index,
                backup_index,
                had_git_index: true,
            }
            .discard();
            Ok(())
        }
        (false, true) => Err(anyhow!(
            "Recovery pending: the committed colocated-Jujutsu finish has a prepared success index but no matching backup"
        )),
        (false, false) => validate_active_jj_finish_index(worktree),
    }
}

fn validate_active_jj_finish_index(worktree: &Path) -> Result<()> {
    if !git_bytes(worktree, &["ls-files", "-z", "--", ".grove"])?.is_empty() {
        bail!(
            "Recovery pending: the committed colocated-Jujutsu finish still has `.grove/` entries in the active Git index"
        );
    }
    Ok(())
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

pub(crate) fn prepare_jj_finish(worktree: &Path) -> Result<PreparedJjFinish> {
    let index_backup = worktree
        .join(".git")
        .exists()
        .then(|| preserve_git_index(worktree))
        .transpose()?;
    let prepared = (|| -> Result<(JjStartAnchor, [u8; 32], Option<PathBuf>)> {
        let start = jj_topology(worktree, "@", false, None)
            .context("recording Jujutsu finish start topology")?;
        let deletion_fingerprint = jj_deletion_fingerprint(worktree, &start)?;
        let success_index = match &index_backup {
            Some(backup) => backup.prepare_without_grove(worktree)?,
            None => None,
        };
        Ok((start, deletion_fingerprint, success_index))
    })();
    match prepared {
        Ok((start, deletion_fingerprint, success_index)) => Ok(PreparedJjFinish {
            worktree: worktree.to_path_buf(),
            start,
            deletion_fingerprint,
            index_backup,
            success_index,
        }),
        Err(error) => {
            if let Some(backup) = index_backup {
                backup.discard();
            }
            Err(error)
        }
    }
}

impl PreparedJjFinish {
    fn commit(self, finish_handle: &str, attempt_identity: &str) -> FinishCommitOutcome {
        let message = finish_commit_message(finish_handle, attempt_identity);
        let fileset = format!("root:.grove ~ root:.grove/FINISHING-{finish_handle}");
        let auto_track_configuration = format!(
            "snapshot.auto-track={:?}",
            format!("all() ~ root:.grove/FINISHING-{finish_handle}")
        );
        let command_result = run_vcs_command(
            &self.worktree,
            "jj",
            &[
                "--config",
                &auto_track_configuration,
                "commit",
                "-m",
                &message,
                &fileset,
            ],
        );
        self.classify_command_result(&message, &auto_track_configuration, command_result)
    }

    fn classify_command_result(
        mut self,
        message: &str,
        auto_track_configuration: &str,
        command_result: Result<()>,
    ) -> FinishCommitOutcome {
        let committed = JjFinishProof {
            worktree: self.worktree.clone(),
            start: self.start.clone(),
            message: message.to_owned(),
            deletion_fingerprint: self.deletion_fingerprint,
        };
        let committed_error = match committed.revalidate() {
            Ok(()) => {
                if let Some(backup) = self.index_backup.take() {
                    if let Err(index_error) = backup.activate(self.success_index.take()) {
                        return FinishCommitOutcome::RecoveryPending(
                            index_error.context(
                                "activating the successful colocated Git index after the Jujutsu finish commit",
                            ),
                        );
                    }
                }
                discard_temporary_index(self.success_index.as_deref());
                return FinishCommitOutcome::Committed(FinishProof::Jj(committed));
            }
            Err(error) => error,
        };

        let command_error = match command_result {
            Err(error) => error,
            Ok(()) => anyhow::anyhow!(
                "Jujutsu reported a successful finish commit, but the exact scoped result could not be proven: {committed_error:#}"
            ),
        };
        discard_temporary_index(self.success_index.take().as_deref());
        let mut start = JjStartProof {
            worktree: self.worktree,
            start: self.start,
            message: message.to_owned(),
            deletion_fingerprint: self.deletion_fingerprint,
            index_backup: self.index_backup,
            auto_track_configuration: auto_track_configuration.to_owned(),
        };
        if let Err(topology_error) = start.revalidate_before_rollback() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "{topology_error:#}; preserve divergent work, restore the recorded start or the exact teardown result, then retry"
            )));
        }
        if let Err(index_error) = start.restore_index_preserving_backup() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "restoring the original colocated Git index failed: {index_error:#}"
            )));
        }
        if let Err(topology_error) = start.revalidate_before_rollback() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "Jujutsu topology changed after index restoration: {topology_error:#}"
            )));
        }
        FinishCommitOutcome::NotCommitted {
            proof: FinishStartProof::Jj(start),
            error: command_error,
        }
    }
}

/// Commit the prepared `.grove/` deletion, preserving unrelated staged or
/// working-copy changes. The caller owns the tree lock and all finish facts.
pub fn commit_finish(worktree: &Path, finish_handle: &str, attempt_identity: &str) -> Result<()> {
    match prepare_finish(worktree)?.commit(finish_handle, attempt_identity) {
        FinishCommitOutcome::Committed(_) => Ok(()),
        FinishCommitOutcome::NotCommitted { error, .. }
        | FinishCommitOutcome::RecoveryPending(error) => Err(error),
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

    fn restore_preserving_backup(&self) -> Result<()> {
        if self.had_git_index {
            fs::copy(&self.backup_index, &self.git_index).with_context(|| {
                format!(
                    "restoring Git index {} from {}",
                    self.git_index.display(),
                    self.backup_index.display()
                )
            })?;
            Ok(())
        } else {
            match fs::remove_file(&self.git_index) {
                Ok(()) => Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(error) => Err(error).with_context(|| {
                    format!(
                        "removing newly created Git index {}",
                        self.git_index.display()
                    )
                }),
            }
        }
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

fn existing_git_index_backup(worktree: &Path, had_git_index: bool) -> Result<GitIndexBackup> {
    let git_index = git_path(worktree, "index")?;
    let backup_index = git_index.with_file_name(INDEX_BACKUP_NAME);
    if had_git_index && !backup_index.is_file() {
        bail!(
            "Recovery pending: the recorded finish has no recoverable Git index backup at {}",
            backup_index.display()
        );
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

fn validate_exact_jj_finish(
    worktree: &Path,
    start: &JjStartAnchor,
    message: &str,
    deletion_fingerprint: [u8; 32],
) -> Result<()> {
    let successor = jj_topology(worktree, "@", true, None)?;
    let [candidate] = successor.parent_commit_ids.as_slice() else {
        bail!(
            "the Jujutsu finish result is not the exact parent of the current working-copy successor"
        );
    };
    validate_jj_finish_candidate(worktree, start, candidate, message, deletion_fingerprint)
}

fn validate_jj_finish_candidate(
    worktree: &Path,
    start: &JjStartAnchor,
    candidate_commit_id: &str,
    message: &str,
    deletion_fingerprint: [u8; 32],
) -> Result<()> {
    let candidate = jj_topology(worktree, candidate_commit_id, true, None)?;
    if candidate.change_id != start.change_id
        || candidate.parent_commit_ids != start.parent_commit_ids
    {
        bail!(
            "the Jujutsu finish result has unexpected topology: recorded change {} at parents {:?}, observed change {} at parents {:?}",
            start.change_id,
            start.parent_commit_ids,
            candidate.change_id,
            candidate.parent_commit_ids
        );
    }
    let observed_message = jj_stdout(
        worktree,
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            candidate_commit_id,
            "--no-graph",
            "-T",
            "description",
        ],
    )?;
    if observed_message != message {
        bail!(
            "the Jujutsu finish result has the wrong message: expected {message:?}, observed {observed_message:?}"
        );
    }
    if !jj_bytes(
        worktree,
        &[
            "--ignore-working-copy",
            "file",
            "list",
            "-r",
            candidate_commit_id,
            "root:.grove",
        ],
    )?
    .is_empty()
    {
        bail!("the Jujutsu finish result still tracks `.grove/`");
    }
    if jj_deletion_fingerprint(worktree, start)? != deletion_fingerprint {
        bail!("the recorded Jujutsu deletion fingerprint no longer matches its start anchor");
    }

    let changed = jj_bytes(
        worktree,
        &[
            "--ignore-working-copy",
            "diff",
            "-r",
            candidate_commit_id,
            "-T",
            "status ++ \"\\0\" ++ path ++ \"\\0\"",
        ],
    )?;
    let fields = changed
        .split(|byte| *byte == 0)
        .filter(|field| !field.is_empty())
        .collect::<Vec<_>>();
    if fields.is_empty()
        || fields.len() % 2 != 0
        || fields
            .chunks_exact(2)
            .any(|entry| entry[0] != b"removed" || !entry[1].starts_with(b".grove/"))
    {
        bail!("the Jujutsu finish result changes paths outside the exact `.grove/` deletion");
    }
    Ok(())
}

fn validate_jj_uncommitted(
    worktree: &Path,
    start: &JjStartAnchor,
    message: &str,
    deletion_fingerprint: [u8; 32],
) -> Result<()> {
    let revset = format!("change_id({})", start.change_id);
    let versions = jj_stdout(
        worktree,
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            &revset,
            "--no-graph",
            "-T",
            "commit_id ++ \"\\n\"",
        ],
    )?;
    for candidate in versions.lines() {
        if validate_jj_finish_candidate(worktree, start, candidate, message, deletion_fingerprint)
            .is_ok()
        {
            bail!(
                "Recovery pending: the exact Jujutsu finish result {} exists but is not the current successor's immediate parent",
                candidate
            );
        }
    }

    let observed = jj_topology(worktree, "@", true, None)?;
    if observed.change_id != start.change_id
        || observed.parent_commit_ids != start.parent_commit_ids
    {
        bail!(
            "Recovery pending: Jujutsu finish recorded change {} at parents {:?}, but observed change {} at parents {:?}",
            start.change_id,
            start.parent_commit_ids,
            observed.change_id,
            observed.parent_commit_ids
        );
    }
    Ok(())
}

fn jj_topology(
    worktree: &Path,
    revision: &str,
    ignore_working_copy: bool,
    auto_track_configuration: Option<&str>,
) -> Result<JjStartAnchor> {
    let mut command = vcs_command(worktree, "jj");
    if let Some(configuration) = auto_track_configuration {
        command.args(["--config", configuration]);
    }
    if ignore_working_copy {
        command.arg("--ignore-working-copy");
    }
    let output = command
        .args([
            "log",
            "-r",
            revision,
            "--no-graph",
            "-T",
            "commit_id ++ \"\\n\" ++ change_id ++ \"\\n\" ++ parents.map(|parent| parent.commit_id()).join(\" \") ++ \"\\n\"",
        ])
        .output()
        .with_context(|| format!("reading Jujutsu topology for {revision}"))?;
    if !output.status.success() {
        bail!(
            "jj log -r {revision} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let body = String::from_utf8(output.stdout)?;
    let mut lines = body.lines();
    let commit_id = lines.next().context("Jujutsu topology omitted commit ID")?;
    let change_id = lines.next().context("Jujutsu topology omitted change ID")?;
    let parents = lines
        .next()
        .context("Jujutsu topology omitted parent commit IDs")?;
    if commit_id.is_empty() || change_id.is_empty() || lines.next().is_some() {
        bail!("Jujutsu topology output was malformed for {revision}");
    }
    Ok(JjStartAnchor {
        commit_id: commit_id.to_owned(),
        change_id: change_id.to_owned(),
        parent_commit_ids: parents.split_whitespace().map(str::to_owned).collect(),
    })
}

fn jj_deletion_fingerprint(worktree: &Path, start: &JjStartAnchor) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut has_tracked_grove = false;
    for parent in &start.parent_commit_ids {
        let tree = jj_bytes(
            worktree,
            &[
                "--ignore-working-copy",
                "diff",
                "--from",
                "root()",
                "--to",
                parent,
                "--git",
                "root:.grove",
            ],
        )?;
        has_tracked_grove |= !tree.is_empty();
        hasher.update((parent.len() as u64).to_le_bytes());
        hasher.update(parent.as_bytes());
        hasher.update((tree.len() as u64).to_le_bytes());
        hasher.update(tree);
    }
    if !has_tracked_grove {
        bail!(
            "cannot finish this Jujutsu grove: `.grove/` has no tracked state at the recorded parents of {}, so no focused deletion commit can be recorded",
            start.commit_id
        );
    }
    Ok(hasher.finalize().into())
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

fn jj_stdout(worktree: &Path, arguments: &[&str]) -> Result<String> {
    Ok(String::from_utf8(jj_bytes(worktree, arguments)?)?
        .trim()
        .to_owned())
}

fn jj_bytes(worktree: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = vcs_command(worktree, "jj")
        .args(arguments)
        .output()
        .with_context(|| {
            format!(
                "running jj {} in {}",
                arguments.join(" "),
                worktree.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "jj {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
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
        finish_commit_message, git_bytes, prepare_jj_finish, prepare_plain_git_finish,
        recover_finish, run_vcs_command, FinishCommitOutcome, FinishRecoveryOutcome,
        FinishStartAnchor,
    };
    use anyhow::anyhow;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    fn native_jj_finish_fixture() -> TempDir {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path();
        run_vcs_command(
            repository,
            "jj",
            &[
                "--config",
                "git.colocate=false",
                "git",
                "init",
                "--quiet",
                ".",
            ],
        )
        .unwrap();
        run_vcs_command(
            repository,
            "jj",
            &[
                "config",
                "set",
                "--workspace",
                "user.name",
                "\"Grove Test\"",
            ],
        )
        .unwrap();
        run_vcs_command(
            repository,
            "jj",
            &[
                "config",
                "set",
                "--workspace",
                "user.email",
                "\"grove-test@example.com\"",
            ],
        )
        .unwrap();
        fs::create_dir(repository.join(".grove")).unwrap();
        fs::write(repository.join(".grove/FORMAT"), "session-kinds-v1\n").unwrap();
        fs::write(repository.join("outside"), "before\n").unwrap();
        run_vcs_command(repository, "jj", &["commit", "-m", "fixture"]).unwrap();
        fs::write(repository.join(".grove/finish"), "finish\n").unwrap();
        fs::write(repository.join("outside"), "after\n").unwrap();
        fixture
    }

    fn colocated_jj_finish_fixture() -> TempDir {
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
        fs::write(repository.join("outside"), "before\n").unwrap();
        run_vcs_command(repository, "git", &["add", "-A"]).unwrap();
        run_vcs_command(repository, "git", &["commit", "-q", "-m", "fixture"]).unwrap();
        run_vcs_command(
            repository,
            "jj",
            &["git", "init", "--colocate", "--quiet", "."],
        )
        .unwrap();
        run_vcs_command(
            repository,
            "jj",
            &[
                "config",
                "set",
                "--workspace",
                "user.name",
                "\"Grove Test\"",
            ],
        )
        .unwrap();
        run_vcs_command(
            repository,
            "jj",
            &[
                "config",
                "set",
                "--workspace",
                "user.email",
                "\"grove-test@example.com\"",
            ],
        )
        .unwrap();
        fs::write(repository.join(".grove/finish"), "finish\n").unwrap();
        fs::write(repository.join("outside"), "after\n").unwrap();
        fixture
    }

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

    #[test]
    fn recorded_plain_git_start_restores_the_preflight_index_before_rollback() {
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
        fs::write(repository.join("outside"), "before\n").unwrap();
        run_vcs_command(repository, "git", &["add", "-A"]).unwrap();
        run_vcs_command(repository, "git", &["commit", "-q", "-m", "fixture"]).unwrap();
        fs::write(repository.join("staged"), "preserve\n").unwrap();
        run_vcs_command(repository, "git", &["add", "staged"]).unwrap();

        let prepared = prepare_plain_git_finish(repository).unwrap();
        let start = FinishStartAnchor::PlainGit {
            head: prepared.start_head.clone(),
            had_git_index: prepared.index_backup.had_git_index,
        };
        let deletion_fingerprint = prepared.deletion_fingerprint;
        fs::remove_dir_all(repository.join(".grove")).unwrap();
        run_vcs_command(repository, "git", &["add", "-A", "--", ".grove"]).unwrap();

        let outcome = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );

        let FinishRecoveryOutcome::NotCommitted(proof) = outcome else {
            panic!("the recorded start was not recovered as an uncommitted finish");
        };
        proof.revalidate_before_rollback().unwrap();
        drop(proof);

        let retried = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );
        let FinishRecoveryOutcome::NotCommitted(proof) = retried else {
            panic!("index restoration consumed the only restart evidence");
        };
        proof.revalidate_before_rollback().unwrap();
        let staged = String::from_utf8(
            Command::new("git")
                .current_dir(repository)
                .args(["diff", "--cached", "--name-only"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        assert_eq!(staged, "staged\n");
    }

    #[test]
    fn recorded_plain_git_start_recovers_a_legitimately_absent_index() {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path();
        let init = Command::new("git")
            .current_dir(repository)
            .args(["init", "-q", "."])
            .output()
            .unwrap();
        assert!(init.status.success());
        run_vcs_command(repository, "git", &["config", "user.name", "Grove Test"]).unwrap();
        run_vcs_command(
            repository,
            "git",
            &["config", "user.email", "grove-test@example.com"],
        )
        .unwrap();
        fs::create_dir(repository.join(".grove")).unwrap();
        fs::write(repository.join(".grove/FORMAT"), "session-kinds-v1\n").unwrap();
        run_vcs_command(repository, "git", &["add", "-A"]).unwrap();
        run_vcs_command(repository, "git", &["commit", "-q", "-m", "fixture"]).unwrap();
        fs::remove_file(repository.join(".git/index")).unwrap();

        let prepared = prepare_plain_git_finish(repository).unwrap();
        let start = FinishStartAnchor::PlainGit {
            head: prepared.start_head.clone(),
            had_git_index: prepared.index_backup.had_git_index,
        };
        let deletion_fingerprint = prepared.deletion_fingerprint;
        fs::remove_dir_all(repository.join(".grove")).unwrap();

        let outcome = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );

        let FinishRecoveryOutcome::NotCommitted(proof) = outcome else {
            panic!("an absent preflight index was not recovered");
        };
        proof.revalidate_before_rollback().unwrap();
        assert!(!repository.join(".git/index").exists());
    }

    #[test]
    fn exact_jj_finish_result_reclassifies_a_lost_command_as_committed() {
        let fixture = native_jj_finish_fixture();
        let repository = fixture.path();
        let prepared = prepare_jj_finish(repository).unwrap();
        let message = finish_commit_message("finish-k2", "11111111111111111111111111111111");
        let auto_track_configuration =
            "snapshot.auto-track=\"all() ~ root:.grove/FINISHING-finish-k2\"";
        fs::remove_dir_all(repository.join(".grove")).unwrap();
        run_vcs_command(
            repository,
            "jj",
            &[
                "--config",
                auto_track_configuration,
                "commit",
                "-m",
                &message,
                "root:.grove ~ root:.grove/FINISHING-finish-k2",
            ],
        )
        .unwrap();

        let outcome = prepared.classify_command_result(
            &message,
            auto_track_configuration,
            Err(anyhow!("lost result")),
        );

        let FinishCommitOutcome::Committed(proof) = outcome else {
            panic!("the exact Jujutsu result was not classified as committed");
        };
        proof.revalidate().unwrap();

        run_vcs_command(repository, "jj", &["new"]).unwrap();
        let error = proof.revalidate().unwrap_err();
        assert!(
            error.to_string().contains("unexpected topology"),
            "{error:#}"
        );
    }

    #[test]
    fn recorded_native_jj_start_is_recoverable_when_the_exact_result_is_absent() {
        let fixture = native_jj_finish_fixture();
        let repository = fixture.path();
        let prepared = prepare_jj_finish(repository).unwrap();
        let start = FinishStartAnchor::Jujutsu {
            commit_id: prepared.start.commit_id.clone(),
            change_id: prepared.start.change_id.clone(),
            parent_commit_ids: prepared.start.parent_commit_ids.clone(),
            git_index_existed: prepared
                .index_backup
                .as_ref()
                .map(|backup| backup.had_git_index),
        };
        let deletion_fingerprint = prepared.deletion_fingerprint;
        fs::remove_dir_all(repository.join(".grove")).unwrap();

        let outcome = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );

        let FinishRecoveryOutcome::NotCommitted(proof) = outcome else {
            panic!("the recorded Jujutsu start was not recovered as uncommitted");
        };
        proof.revalidate_before_rollback().unwrap();
    }

    #[test]
    fn recorded_colocated_jj_start_recovers_a_legitimately_absent_index() {
        let fixture = colocated_jj_finish_fixture();
        let repository = fixture.path();
        fs::remove_file(repository.join(".git/index")).unwrap();
        let prepared = prepare_jj_finish(repository).unwrap();
        let start = FinishStartAnchor::Jujutsu {
            commit_id: prepared.start.commit_id.clone(),
            change_id: prepared.start.change_id.clone(),
            parent_commit_ids: prepared.start.parent_commit_ids.clone(),
            git_index_existed: prepared
                .index_backup
                .as_ref()
                .map(|backup| backup.had_git_index),
        };
        let deletion_fingerprint = prepared.deletion_fingerprint;
        fs::remove_dir_all(repository.join(".grove")).unwrap();

        let outcome = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );

        let FinishRecoveryOutcome::NotCommitted(proof) = outcome else {
            panic!("an absent colocated preflight index was not recovered");
        };
        proof.revalidate_before_rollback().unwrap();
        assert!(!repository.join(".git/index").exists());
    }

    #[test]
    fn committed_colocated_jj_recovery_requires_a_grove_free_active_index() {
        let fixture = colocated_jj_finish_fixture();
        let repository = fixture.path();
        let prepared = prepare_jj_finish(repository).unwrap();
        let start = FinishStartAnchor::Jujutsu {
            commit_id: prepared.start.commit_id.clone(),
            change_id: prepared.start.change_id.clone(),
            parent_commit_ids: prepared.start.parent_commit_ids.clone(),
            git_index_existed: prepared
                .index_backup
                .as_ref()
                .map(|backup| backup.had_git_index),
        };
        let deletion_fingerprint = prepared.deletion_fingerprint;
        let message = finish_commit_message("finish-k2", "11111111111111111111111111111111");
        let auto_track_configuration =
            "snapshot.auto-track=\"all() ~ root:.grove/FINISHING-finish-k2\"";
        fs::remove_dir_all(repository.join(".grove")).unwrap();
        run_vcs_command(
            repository,
            "jj",
            &[
                "--config",
                auto_track_configuration,
                "commit",
                "-m",
                &message,
                "root:.grove ~ root:.grove/FINISHING-finish-k2",
            ],
        )
        .unwrap();

        let first = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );
        let FinishRecoveryOutcome::Committed(proof) = first else {
            panic!("the exact committed result was not recovered")
        };
        proof.revalidate().unwrap();
        let tracked = git_bytes(repository, &["ls-files", "--", ".grove"]).unwrap();
        assert!(tracked.is_empty());

        let repeated = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );
        match repeated {
            FinishRecoveryOutcome::Committed(_) => {}
            FinishRecoveryOutcome::NotCommitted(_) => {
                panic!("the exact committed result was reclassified as uncommitted")
            }
            FinishRecoveryOutcome::RecoveryPending(error) => panic!("{error:#}"),
        }

        fs::create_dir(repository.join(".grove")).unwrap();
        fs::write(repository.join(".grove/foreign"), "foreign\n").unwrap();
        run_vcs_command(repository, "git", &["add", ".grove/foreign"]).unwrap();
        let corrupted = recover_finish(
            repository,
            &start,
            "finish-k2",
            "11111111111111111111111111111111",
            deletion_fingerprint,
        );
        let FinishRecoveryOutcome::RecoveryPending(error) = corrupted else {
            panic!("a grove-tracking active index was accepted as completed");
        };
        assert!(format!("{error:#}").contains("active Git index"));
    }

    #[test]
    fn unexpected_jj_topology_stays_recovery_pending() {
        let fixture = native_jj_finish_fixture();
        let repository = fixture.path();
        let prepared = prepare_jj_finish(repository).unwrap();
        let message = finish_commit_message("finish-k2", "11111111111111111111111111111111");
        let auto_track_configuration =
            "snapshot.auto-track=\"all() ~ root:.grove/FINISHING-finish-k2\"";

        fs::remove_dir_all(repository.join(".grove")).unwrap();
        run_vcs_command(repository, "jj", &["new"]).unwrap();
        let outcome = prepared.classify_command_result(
            &message,
            auto_track_configuration,
            Err(anyhow!("command failed after changing topology")),
        );

        let FinishCommitOutcome::RecoveryPending(error) = outcome else {
            panic!("unexpected Jujutsu topology was not retained for recovery");
        };
        assert!(format!("{error:#}").contains("command failed"), "{error:#}");
    }
}
