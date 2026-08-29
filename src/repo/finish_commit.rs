// The finish teardown commit, jj only (`docs/adr/jj-is-the-only-lane.md`).
//
// Preparation observes — the working-copy topology and a fingerprint of the
// tracked `.grove/` at its parents — and allocates nothing. The commit is one
// fileset-scoped `jj commit`; whatever it does to a colocated Git index is jj's
// own business, and Grove neither backs that index up nor restores it. What
// makes the outcome provable is the same in every arm: the exact result is
// re-derived from the repository and compared against the recorded start, so a
// lost command is classified rather than assumed.

use super::require_jj_workspace;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub(crate) struct PreparedFinish {
    worktree: PathBuf,
    start: FinishStartAnchor,
    deletion_fingerprint: [u8; 32],
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

/// The working-copy revision the finish attempt started from, as the manifest
/// records it. One shape rather than a tagged union: with one lane there is no
/// second anchor a recovery could be handed by mistake.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct FinishStartAnchor {
    commit_id: String,
    change_id: String,
    parent_commit_ids: Vec<String>,
}

pub(crate) struct FinishProof {
    worktree: PathBuf,
    start: FinishStartAnchor,
    message: String,
    deletion_fingerprint: [u8; 32],
}

pub(crate) struct FinishStartProof {
    worktree: PathBuf,
    start: FinishStartAnchor,
    message: String,
    deletion_fingerprint: [u8; 32],
    auto_track_configuration: String,
}

impl FinishProof {
    pub(crate) fn revalidate(&self) -> Result<()> {
        validate_exact_finish(
            &self.worktree,
            &self.start,
            &self.message,
            self.deletion_fingerprint,
        )
    }
}

impl FinishStartProof {
    pub(crate) fn revalidate_before_rollback(&self) -> Result<()> {
        validate_uncommitted(
            &self.worktree,
            &self.start,
            &self.message,
            self.deletion_fingerprint,
        )
    }

    pub(crate) fn revalidate_after_rollback(self) -> Result<()> {
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
        if deletion_fingerprint(&self.worktree, &self.start)? != self.deletion_fingerprint {
            bail!(
                "Recovery pending: restored Jujutsu grove does not match its preflight fingerprint"
            );
        }
        Ok(())
    }
}

impl PreparedFinish {
    pub(crate) fn start_anchor(&self) -> FinishStartAnchor {
        self.start.clone()
    }

    pub(crate) fn deletion_fingerprint(&self) -> [u8; 32] {
        self.deletion_fingerprint
    }

    pub(crate) fn commit(self, finish_handle: &str, attempt_identity: &str) -> FinishCommitOutcome {
        let message = finish_commit_message(finish_handle, attempt_identity);
        let fileset = format!("root:.grove ~ root:.grove/FINISHING-{finish_handle}");
        let auto_track_configuration = auto_track_configuration(finish_handle);
        let command_result = run_jj(
            &self.worktree,
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

    /// Nothing to undo. Preparation reads the repository and writes nothing to
    /// it, so abandoning it leaves no artifact and no half-applied state — the
    /// method survives as the place a caller says *this attempt is over*, not
    /// as a rollback.
    pub(crate) fn abort(self) -> Result<()> {
        Ok(())
    }

    fn classify_command_result(
        self,
        message: &str,
        auto_track_configuration: &str,
        command_result: Result<()>,
    ) -> FinishCommitOutcome {
        let committed = FinishProof {
            worktree: self.worktree.clone(),
            start: self.start.clone(),
            message: message.to_owned(),
            deletion_fingerprint: self.deletion_fingerprint,
        };
        let committed_error = match committed.revalidate() {
            Ok(()) => return FinishCommitOutcome::Committed(committed),
            Err(error) => error,
        };

        let command_error = match command_result {
            Err(error) => error,
            Ok(()) => anyhow!(
                "Jujutsu reported a successful finish commit, but the exact scoped result could not be proven: {committed_error:#}"
            ),
        };
        let start = FinishStartProof {
            worktree: self.worktree,
            start: self.start,
            message: message.to_owned(),
            deletion_fingerprint: self.deletion_fingerprint,
            auto_track_configuration: auto_track_configuration.to_owned(),
        };
        if let Err(topology_error) = start.revalidate_before_rollback() {
            return FinishCommitOutcome::RecoveryPending(command_error.context(format!(
                "{topology_error:#}; preserve divergent work, restore the recorded start or the exact teardown result, then retry"
            )));
        }
        FinishCommitOutcome::NotCommitted {
            proof: start,
            error: command_error,
        }
    }
}

pub(crate) fn prepare_finish(
    worktree: &Path,
    _finish_handle: &str,
    _attempt_identity: &str,
) -> Result<PreparedFinish> {
    let workspace_root = require_jj_workspace(worktree)
        .context("cannot commit the finished grove")?;
    let start = jj_topology(&workspace_root, "@", false, None)
        .context("recording Jujutsu finish start topology")?;
    let deletion_fingerprint = deletion_fingerprint(&workspace_root, &start)?;
    Ok(PreparedFinish {
        worktree: workspace_root,
        start,
        deletion_fingerprint,
    })
}

/// Abandon a preparation whose own state is already gone — a restart between
/// witness and commit. Preparation allocates nothing, so there is nothing to
/// dispose of; what remains worth doing is proving the workspace is still the
/// one the witness was written for, because a tree that stopped being jj-enabled
/// under a pending finish is a recovery-pending condition rather than a clean
/// abort.
pub(crate) fn abort_preparing_finish(
    worktree: &Path,
    _finish_handle: &str,
    _attempt_identity: &str,
) -> Result<()> {
    require_jj_workspace(worktree)
        .context("Recovery pending: cannot abort finish preparation")?;
    Ok(())
}

pub(crate) fn recover_finish(
    worktree: &Path,
    start: &FinishStartAnchor,
    finish_handle: &str,
    attempt_identity: &str,
    fingerprint: [u8; 32],
) -> FinishRecoveryOutcome {
    let message = finish_commit_message(finish_handle, attempt_identity);
    let workspace_root = match require_jj_workspace(worktree) {
        Ok(root) => root,
        Err(error) => {
            return FinishRecoveryOutcome::RecoveryPending(
                error.context("Recovery pending: the recorded finish cannot be resolved"),
            )
        }
    };

    let committed = FinishProof {
        worktree: workspace_root.clone(),
        start: start.clone(),
        message: message.clone(),
        deletion_fingerprint: fingerprint,
    };
    if committed.revalidate().is_ok() {
        return FinishRecoveryOutcome::Committed(committed);
    }

    let start = FinishStartProof {
        worktree: workspace_root,
        start: start.clone(),
        message,
        deletion_fingerprint: fingerprint,
        auto_track_configuration: auto_track_configuration(finish_handle),
    };
    if let Err(error) = start.revalidate_before_rollback() {
        return FinishRecoveryOutcome::RecoveryPending(error.context(
            "preserve divergent work, restore the recorded start or the exact teardown result, then retry",
        ));
    }
    FinishRecoveryOutcome::NotCommitted(start)
}

fn finish_commit_message(finish_handle: &str, attempt_identity: &str) -> String {
    format!("{finish_handle} (finish attempt {attempt_identity}): remove completed grove task tree")
}

/// The snapshot rule every finish command and every post-rollback observation
/// runs under: track everything *except* the working-tree-only finish witness,
/// which must stay out of the commit that deletes the tree it witnesses.
fn auto_track_configuration(finish_handle: &str) -> String {
    format!(
        "snapshot.auto-track={:?}",
        format!("all() ~ root:.grove/FINISHING-{finish_handle}")
    )
}

/// Prove, without a manifest, that the repository's *immediate* result is the
/// exact `.grove/`-scoped teardown commit this launch's finish attempt would
/// have made.
///
/// Reached only when the task root is already absent, where successful cleanup
/// may have disposed the witness that anchors [`recover_finish`]. The proof is
/// therefore self-contained: it derives the parent from the result itself
/// rather than from a recorded start anchor, and never requires the
/// working-tree-only finish leaf to have existed in that parent. The attempt
/// identity in the message is what keeps it narrow — a reused handle from an
/// older grove was committed under a different launch nonce and can never
/// satisfy the current one.
pub(crate) fn verify_lost_finish_result(
    worktree: &Path,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<()> {
    let message = finish_commit_message(finish_handle, attempt_identity);
    let workspace_root = require_jj_workspace(worktree)?;
    let successor = jj_topology(&workspace_root, "@", true, None)?;
    let [candidate] = successor.parent_commit_ids.as_slice() else {
        bail!("the immediate Jujutsu result is not the exact parent of the current working-copy successor");
    };
    let observed_message = jj_description(&workspace_root, candidate)?;
    if observed_message != message {
        bail!(
            "the immediate Jujutsu result is not this finish attempt's teardown commit: expected message {message:?}, observed {observed_message:?}"
        );
    }
    if tracks_grove(&workspace_root, candidate)? {
        bail!("the immediate Jujutsu result still tracks `.grove/`");
    }
    if !only_grove_deletions(&jj_scoped_diff(&workspace_root, candidate)?) {
        bail!("the immediate Jujutsu result changes paths outside the exact `.grove/` deletion");
    }
    Ok(())
}

/// A teardown delta is a non-empty set of `.grove/` deletions and nothing else,
/// read from jj's NUL-delimited `<status>\0<path>\0` template.
fn only_grove_deletions(changed: &[u8]) -> bool {
    let fields = changed
        .split(|byte| *byte == 0)
        .filter(|field| !field.is_empty())
        .collect::<Vec<_>>();
    !fields.is_empty()
        && fields.len() % 2 == 0
        && fields
            .chunks_exact(2)
            .all(|entry| entry[0] == b"removed" && entry[1].starts_with(b".grove/"))
}

fn validate_exact_finish(
    worktree: &Path,
    start: &FinishStartAnchor,
    message: &str,
    fingerprint: [u8; 32],
) -> Result<()> {
    let successor = jj_topology(worktree, "@", true, None)?;
    let [candidate] = successor.parent_commit_ids.as_slice() else {
        bail!(
            "the Jujutsu finish result is not the exact parent of the current working-copy successor"
        );
    };
    validate_finish_candidate(worktree, start, candidate, message, fingerprint)
}

fn validate_finish_candidate(
    worktree: &Path,
    start: &FinishStartAnchor,
    candidate_commit_id: &str,
    message: &str,
    fingerprint: [u8; 32],
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
    let observed_message = jj_description(worktree, candidate_commit_id)?;
    if observed_message != message {
        bail!(
            "the Jujutsu finish result has the wrong message: expected {message:?}, observed {observed_message:?}"
        );
    }
    if tracks_grove(worktree, candidate_commit_id)? {
        bail!("the Jujutsu finish result still tracks `.grove/`");
    }
    if deletion_fingerprint(worktree, start)? != fingerprint {
        bail!("the recorded Jujutsu deletion fingerprint no longer matches its start anchor");
    }
    if !only_grove_deletions(&jj_scoped_diff(worktree, candidate_commit_id)?) {
        bail!("the Jujutsu finish result changes paths outside the exact `.grove/` deletion");
    }
    Ok(())
}

fn validate_uncommitted(
    worktree: &Path,
    start: &FinishStartAnchor,
    message: &str,
    fingerprint: [u8; 32],
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
        if validate_finish_candidate(worktree, start, candidate, message, fingerprint).is_ok() {
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
) -> Result<FinishStartAnchor> {
    let mut command = jj_command(worktree);
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
    Ok(FinishStartAnchor {
        commit_id: commit_id.to_owned(),
        change_id: change_id.to_owned(),
        parent_commit_ids: parents.split_whitespace().map(str::to_owned).collect(),
    })
}

/// The tracked `.grove/` at the anchor's parents, hashed. Refuses a grove with
/// no tracked state at all: there is then no focused deletion commit to record.
fn deletion_fingerprint(worktree: &Path, start: &FinishStartAnchor) -> Result<[u8; 32]> {
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
            "cannot finish this grove: `.grove/` has no tracked state at the recorded parents of {}, so no focused deletion commit can be recorded",
            start.commit_id
        );
    }
    Ok(hasher.finalize().into())
}

fn jj_description(worktree: &Path, revision: &str) -> Result<String> {
    jj_stdout(
        worktree,
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            revision,
            "--no-graph",
            "-T",
            "description",
        ],
    )
}

fn tracks_grove(worktree: &Path, revision: &str) -> Result<bool> {
    Ok(!jj_bytes(
        worktree,
        &[
            "--ignore-working-copy",
            "file",
            "list",
            "-r",
            revision,
            "root:.grove",
        ],
    )?
    .is_empty())
}

fn jj_scoped_diff(worktree: &Path, revision: &str) -> Result<Vec<u8>> {
    jj_bytes(
        worktree,
        &[
            "--ignore-working-copy",
            "diff",
            "-r",
            revision,
            "-T",
            "status ++ \"\\0\" ++ path ++ \"\\0\"",
        ],
    )
}

fn jj_stdout(worktree: &Path, arguments: &[&str]) -> Result<String> {
    Ok(String::from_utf8(jj_bytes(worktree, arguments)?)?
        .trim()
        .to_owned())
}

fn jj_bytes(worktree: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = jj_command(worktree)
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

fn run_jj(worktree: &Path, arguments: &[&str]) -> Result<()> {
    let output = jj_command(worktree)
        .args(arguments)
        .output()
        .with_context(|| {
            format!(
                "running jj {} in {}",
                arguments.join(" "),
                worktree.display()
            )
        })?;
    command_result(arguments, output)
}

fn command_result(arguments: &[&str], output: Output) -> Result<()> {
    if !output.status.success() {
        bail!(
            "jj {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn jj_command(worktree: &Path) -> Command {
    let mut command = Command::new("jj");
    command.current_dir(worktree);
    crate::launch::scrub_internal_child_env(&mut command);
    command
}
