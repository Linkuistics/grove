use super::super::unix::{
    entry_exists, open_file_at, rename_at_exchange, rename_at_noreplace, unlink_at,
    validate_entry_identity,
};
use super::super::{
    attach_cleanup_failure, create_temporary_marker, remove_temporary_marker, FileIdentity,
};
use super::{
    artifact_file_name, create_staging_entry, ensure_regular_file, is_staging_marker_name,
    is_staging_replacement_name, marker_file_name, read_marker, validate_component,
    validate_marker, validate_marker_binding, AuxiliaryMarker, AuxiliaryRole, MarkerDocument,
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

const REPLACEMENT_SUFFIX: &[u8] = b".replacing";

#[derive(Debug, Eq, PartialEq)]
pub(super) enum MarkerReplacementStep {
    BeforeArtifactExchange,
    AfterArtifactExchange,
    BeforeExchange,
    AfterExchange,
    BeforeRetiredMarkerRemoval,
    AfterRetiredMarkerRemoval,
    BeforeRetiredArtifactRemoval,
    AfterRetiredArtifactRemoval,
}

impl MarkerReplacementStep {
    pub(super) fn name(&self) -> &'static str {
        match self {
            Self::BeforeArtifactExchange => "before-artifact-exchange",
            Self::AfterArtifactExchange => "after-artifact-exchange",
            Self::BeforeExchange => "before-marker-exchange",
            Self::AfterExchange => "after-marker-exchange",
            Self::BeforeRetiredMarkerRemoval => "before-retired-marker-removal",
            Self::AfterRetiredMarkerRemoval => "after-retired-marker-removal",
            Self::BeforeRetiredArtifactRemoval => "before-retired-artifact-removal",
            Self::AfterRetiredArtifactRemoval => "after-retired-artifact-removal",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct MarkerReplacementState {
    version: u32,
    kind: String,
    role: AuxiliaryRole,
    finish_handle: String,
    attempt_identity: String,
    state_identity: FileIdentity,
    canonical_name: Vec<u8>,
    staged_name: Vec<u8>,
    artifact_name: Vec<u8>,
    staged_artifact_name: Vec<u8>,
    previous_artifact: FileIdentity,
    replacement_artifact: FileIdentity,
    previous: MarkerSnapshot,
    replacement: MarkerSnapshot,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct MarkerSnapshot {
    identity: FileIdentity,
    sha256: String,
}

struct StateDocument {
    identity: FileIdentity,
    sha256: String,
    state: MarkerReplacementState,
}

enum ReplacementPhase {
    BeforeExchange,
    AfterExchange { retired: MarkerDocument },
    RetiredMarkerRemoved,
}

pub(super) fn publish_marker_replacement(
    parent: &File,
    marker_path: &Path,
    marker: &AuxiliaryMarker,
    previous_identity: FileIdentity,
    previous_sha256: &str,
    previous_artifact: FileIdentity,
    staged_artifact_name: &OsStr,
) -> Result<()> {
    let marker_body = serialize_json(marker, "finish auxiliary replacement marker")?;
    let canonical_name = marker_path
        .file_name()
        .context("finish auxiliary marker has no file name")?;
    let parent_path = marker_path
        .parent()
        .context("finish auxiliary marker has no parent")?;
    let (mut staged_file, staged_name, staged_identity) =
        create_staging_entry(parent, parent_path, canonical_name)?;
    if let Err(error) = staged_file.write_all(&marker_body) {
        let cleanup = remove_temporary_marker(parent, &staged_name, staged_identity);
        return Err(attach_cleanup_failure(
            anyhow::Error::new(error).context("writing staged finish auxiliary marker"),
            cleanup,
            "removing failed staged finish auxiliary marker",
        ));
    }

    let state_path = replacement_state_path(marker_path)?;
    if let Err(error) = publish_state(parent, &state_path, |state_identity| {
        MarkerReplacementState {
            version: 1,
            kind: "finish-auxiliary-marker-replacement".to_owned(),
            role: marker.role,
            finish_handle: marker.finish_handle.clone(),
            attempt_identity: marker.attempt_identity.clone(),
            state_identity,
            canonical_name: canonical_name.as_bytes().to_vec(),
            staged_name: staged_name.as_bytes().to_vec(),
            artifact_name: marker.artifact_name.clone(),
            staged_artifact_name: staged_artifact_name.as_bytes().to_vec(),
            previous_artifact,
            replacement_artifact: FileIdentity {
                device: marker.device,
                inode: marker.inode,
            },
            previous: MarkerSnapshot {
                identity: previous_identity,
                sha256: previous_sha256.to_owned(),
            },
            replacement: MarkerSnapshot {
                identity: staged_identity,
                sha256: sha256(&marker_body),
            },
        }
    }) {
        let cleanup = remove_temporary_marker(parent, &staged_name, staged_identity);
        return Err(attach_cleanup_failure(
            error,
            cleanup,
            "removing unpublished staged finish auxiliary marker",
        ));
    }
    Ok(())
}

pub(super) fn recover_marker_replacement(
    parent: &File,
    marker_path: &Path,
    target_name: &OsStr,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<()> {
    let state_path = replacement_state_path(marker_path)?;
    let state_name = state_path
        .file_name()
        .context("finish auxiliary replacement state has no file name")?;
    if !entry_exists(parent, state_name)? {
        return Ok(());
    }
    settle_marker_replacement(
        parent,
        marker_path,
        target_name,
        role,
        finish_handle,
        attempt_identity,
        &mut |_| Ok(()),
    )
    .with_context(|| {
        format!(
            "Recovery pending: resolving finish auxiliary marker replacement {}",
            state_path.display()
        )
    })
}

pub(super) fn settle_marker_replacement(
    parent: &File,
    marker_path: &Path,
    target_name: &OsStr,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
    checkpoint: &mut impl FnMut(MarkerReplacementStep) -> io::Result<()>,
) -> Result<()> {
    let state_path = replacement_state_path(marker_path)?;
    let state = read_validated_state(
        parent,
        &state_path,
        marker_path,
        role,
        finish_handle,
        attempt_identity,
    )?;
    settle_artifact_exchange(parent, marker_path, target_name, &state, checkpoint)?;
    match classify_phase(parent, marker_path, target_name, &state)? {
        ReplacementPhase::BeforeExchange => {
            checkpoint(MarkerReplacementStep::BeforeExchange)
                .context("marker-replacement checkpoint before atomic exchange")?;
            revalidate_phase(
                parent,
                marker_path,
                target_name,
                &state_path,
                &state,
                PhaseExpectation::BeforeExchange,
            )?;
            let marker_name = OsStr::from_bytes(&state.state.canonical_name);
            let staged_name = OsStr::from_bytes(&state.state.staged_name);
            rename_at_exchange(parent, marker_name, parent, staged_name).with_context(|| {
                format!(
                    "atomically exchanging finish auxiliary marker {:?} with staged marker {:?}",
                    marker_name, staged_name
                )
            })?;
            checkpoint(MarkerReplacementStep::AfterExchange)
                .context("marker-replacement checkpoint after atomic exchange")?;
        }
        ReplacementPhase::AfterExchange { .. } => {}
        ReplacementPhase::RetiredMarkerRemoved => {
            remove_retired_artifact(parent, marker_path, &state, checkpoint)?;
            remove_state_document(parent, &state_path, &state)?;
            return Ok(());
        }
    }

    let retired = match revalidate_phase(
        parent,
        marker_path,
        target_name,
        &state_path,
        &state,
        PhaseExpectation::AfterExchange,
    )? {
        ReplacementPhase::AfterExchange { retired } => retired,
        _ => {
            bail!(
                "finish auxiliary marker replacement did not remain exchanged at {}; no entry was removed",
                state_path.display()
            )
        }
    };
    checkpoint(MarkerReplacementStep::BeforeRetiredMarkerRemoval)
        .context("marker-replacement checkpoint before retired marker removal")?;
    revalidate_phase(
        parent,
        marker_path,
        target_name,
        &state_path,
        &state,
        PhaseExpectation::AfterExchange,
    )?;
    remove_marker_document(
        parent,
        OsStr::from_bytes(&state.state.staged_name),
        &retired,
    )?;
    checkpoint(MarkerReplacementStep::AfterRetiredMarkerRemoval)
        .context("marker-replacement checkpoint after retired marker removal")?;
    revalidate_phase(
        parent,
        marker_path,
        target_name,
        &state_path,
        &state,
        PhaseExpectation::RetiredMarkerRemoved,
    )?;
    remove_retired_artifact(parent, marker_path, &state, checkpoint)?;
    remove_state_document(parent, &state_path, &state)
}

#[derive(Clone, Copy)]
enum ArtifactReplacementPhase {
    BeforeExchange,
    AfterExchange,
    RetiredArtifactRemoved,
}

fn settle_artifact_exchange(
    parent: &File,
    marker_path: &Path,
    target_name: &OsStr,
    state: &StateDocument,
    checkpoint: &mut impl FnMut(MarkerReplacementStep) -> io::Result<()>,
) -> Result<()> {
    match classify_artifact_phase(parent, marker_path, state)? {
        ArtifactReplacementPhase::BeforeExchange => {
            validate_artifact_exchange_authority(parent, marker_path, target_name, state)?;
            checkpoint(MarkerReplacementStep::BeforeArtifactExchange)
                .context("marker-replacement checkpoint before artifact exchange")?;
            revalidate_artifact_phase(
                parent,
                marker_path,
                state,
                ArtifactReplacementPhase::BeforeExchange,
            )?;
            let artifact_name = OsStr::from_bytes(&state.state.artifact_name);
            let staged_name = OsStr::from_bytes(&state.state.staged_artifact_name);
            rename_at_exchange(parent, artifact_name, parent, staged_name).with_context(|| {
                format!(
                    "atomically exchanging finish auxiliary artifact {:?} with bound replacement {:?}",
                    artifact_name, staged_name
                )
            })?;
            checkpoint(MarkerReplacementStep::AfterArtifactExchange)
                .context("marker-replacement checkpoint after artifact exchange")?;
            revalidate_artifact_phase(
                parent,
                marker_path,
                state,
                ArtifactReplacementPhase::AfterExchange,
            )?;
            Ok(())
        }
        ArtifactReplacementPhase::AfterExchange
        | ArtifactReplacementPhase::RetiredArtifactRemoved => Ok(()),
    }
}

/// Prove the recorded marker pair is Grove's own before exchanging any bytes.
///
/// The artifact exchange is the transaction's first mutation, and the marker
/// side cannot be classified until it has happened — `validate_bound_artifact`
/// expects the replacement inode to be at the canonical artifact name already.
/// Without this gate the exchange would run on the authority of the state
/// document alone, so a substituted document or marker would be caught only
/// after two entries had already been swapped.
fn validate_artifact_exchange_authority(
    parent: &File,
    marker_path: &Path,
    target_name: &OsStr,
    state: &StateDocument,
) -> Result<()> {
    let canonical = read_marker_at(parent, marker_path)?;
    validate_marker(
        marker_path,
        &canonical.marker,
        target_name,
        state.state.role,
        &state.state.finish_handle,
        &state.state.attempt_identity,
    )?;
    let staged_name = OsStr::from_bytes(&state.state.staged_name);
    let staged_path = marker_path.with_file_name(staged_name);
    let staged_file = open_file_at(parent, staged_name).with_context(|| {
        format!(
            "opening staged finish auxiliary marker {} without following symlinks",
            staged_path.display()
        )
    })?;
    let staged = read_marker(&staged_path, staged_file)?;
    validate_marker_binding(
        &staged_path,
        &staged.marker,
        target_name,
        state.state.role,
        &state.state.finish_handle,
        &state.state.attempt_identity,
    )?;
    if !matches_snapshot(&canonical, &state.state.previous)
        || !matches_snapshot(&staged, &state.state.replacement)
        || marker_artifact_identity(&canonical.marker) != state.state.previous_artifact
        || marker_artifact_identity(&staged.marker) != state.state.replacement_artifact
    {
        bail!(
            "finish auxiliary marker replacement does not describe the recorded artifact exchange around {}; both artifacts were left untouched",
            marker_path.display()
        );
    }
    Ok(())
}

fn remove_retired_artifact(
    parent: &File,
    marker_path: &Path,
    state: &StateDocument,
    checkpoint: &mut impl FnMut(MarkerReplacementStep) -> io::Result<()>,
) -> Result<()> {
    match classify_artifact_phase(parent, marker_path, state)? {
        ArtifactReplacementPhase::AfterExchange => {
            checkpoint(MarkerReplacementStep::BeforeRetiredArtifactRemoval)
                .context("marker-replacement checkpoint before retired artifact removal")?;
            revalidate_artifact_phase(
                parent,
                marker_path,
                state,
                ArtifactReplacementPhase::AfterExchange,
            )?;
            let staged_name = OsStr::from_bytes(&state.state.staged_artifact_name);
            unlink_at(parent, staged_name, 0).with_context(|| {
                format!(
                    "removing retired finish auxiliary artifact {:?}",
                    staged_name
                )
            })?;
            checkpoint(MarkerReplacementStep::AfterRetiredArtifactRemoval)
                .context("marker-replacement checkpoint after retired artifact removal")?;
            revalidate_artifact_phase(
                parent,
                marker_path,
                state,
                ArtifactReplacementPhase::RetiredArtifactRemoved,
            )?;
            Ok(())
        }
        ArtifactReplacementPhase::RetiredArtifactRemoved => Ok(()),
        ArtifactReplacementPhase::BeforeExchange => bail!(
            "finish auxiliary retired artifact cannot be removed before its replacement is published at {}",
            marker_path.display()
        ),
    }
}

fn revalidate_artifact_phase(
    parent: &File,
    marker_path: &Path,
    expected_state: &StateDocument,
    expectation: ArtifactReplacementPhase,
) -> Result<()> {
    let state_path = replacement_state_path(marker_path)?;
    let observed = read_validated_state(
        parent,
        &state_path,
        marker_path,
        expected_state.state.role,
        &expected_state.state.finish_handle,
        &expected_state.state.attempt_identity,
    )?;
    if observed.identity != expected_state.identity
        || observed.sha256 != expected_state.sha256
        || observed.state != expected_state.state
    {
        bail!(
            "finish auxiliary marker replacement state identity changed at {}; no entry was removed",
            state_path.display()
        );
    }
    let observed_phase = classify_artifact_phase(parent, marker_path, expected_state)?;
    if !matches!(
        (observed_phase, expectation),
        (
            ArtifactReplacementPhase::BeforeExchange,
            ArtifactReplacementPhase::BeforeExchange
        ) | (
            ArtifactReplacementPhase::AfterExchange,
            ArtifactReplacementPhase::AfterExchange
        ) | (
            ArtifactReplacementPhase::RetiredArtifactRemoved,
            ArtifactReplacementPhase::RetiredArtifactRemoved
        )
    ) {
        bail!(
            "finish auxiliary artifact replacement phase changed at {}; no entry was removed",
            state_path.display()
        );
    }
    Ok(())
}

fn classify_artifact_phase(
    parent: &File,
    marker_path: &Path,
    state: &StateDocument,
) -> Result<ArtifactReplacementPhase> {
    let artifact_name = OsStr::from_bytes(&state.state.artifact_name);
    let artifact_path = marker_path.with_file_name(artifact_name);
    let canonical = read_artifact_identity(parent, &artifact_path, artifact_name)?
        .context("canonical finish auxiliary artifact is absent during replacement")?;
    let staged_name = OsStr::from_bytes(&state.state.staged_artifact_name);
    let staged_path = marker_path.with_file_name(staged_name);
    let staged = read_artifact_identity(parent, &staged_path, staged_name)?;
    match staged {
        Some(staged)
            if canonical == state.state.previous_artifact
                && staged == state.state.replacement_artifact =>
        {
            Ok(ArtifactReplacementPhase::BeforeExchange)
        }
        Some(staged)
            if canonical == state.state.replacement_artifact
                && staged == state.state.previous_artifact =>
        {
            Ok(ArtifactReplacementPhase::AfterExchange)
        }
        None if canonical == state.state.replacement_artifact => {
            Ok(ArtifactReplacementPhase::RetiredArtifactRemoved)
        }
        _ => bail!(
            "bound finish auxiliary artifact replacement identity changed around {}; no entry was removed",
            marker_path.display()
        ),
    }
}

fn read_artifact_identity(
    parent: &File,
    path: &Path,
    name: &OsStr,
) -> Result<Option<FileIdentity>> {
    let artifact = match open_file_at(parent, name) {
        Ok(artifact) => artifact,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if entry_exists(parent, name)? {
                bail!(
                    "finish auxiliary artifact appeared during replacement at {}; no entry was removed",
                    path.display()
                );
            }
            return Ok(None);
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "opening finish auxiliary artifact replacement {} without following symlinks",
                    path.display()
                )
            })
        }
    };
    ensure_regular_file(&artifact, path)?;
    let identity = FileIdentity::from_file(&artifact)?;
    validate_entry_identity(parent, name, identity)
        .context("finish auxiliary artifact replacement changed during validation")?;
    Ok(Some(identity))
}

#[derive(Clone, Copy)]
enum PhaseExpectation {
    BeforeExchange,
    AfterExchange,
    RetiredMarkerRemoved,
}

fn revalidate_phase(
    parent: &File,
    marker_path: &Path,
    target_name: &OsStr,
    state_path: &Path,
    expected_state: &StateDocument,
    expectation: PhaseExpectation,
) -> Result<ReplacementPhase> {
    let observed_state = read_validated_state(
        parent,
        state_path,
        marker_path,
        expected_state.state.role,
        &expected_state.state.finish_handle,
        &expected_state.state.attempt_identity,
    )?;
    if observed_state.identity != expected_state.identity
        || observed_state.sha256 != expected_state.sha256
        || observed_state.state != expected_state.state
    {
        bail!(
            "finish auxiliary marker replacement state identity changed at {}; no entry was removed",
            state_path.display()
        );
    }
    let phase = classify_phase(parent, marker_path, target_name, expected_state)?;
    let matches = matches!(
        (&phase, expectation),
        (
            ReplacementPhase::BeforeExchange,
            PhaseExpectation::BeforeExchange
        ) | (
            ReplacementPhase::AfterExchange { .. },
            PhaseExpectation::AfterExchange
        ) | (
            ReplacementPhase::RetiredMarkerRemoved,
            PhaseExpectation::RetiredMarkerRemoved
        )
    );
    if !matches {
        bail!(
            "finish auxiliary marker replacement phase changed at {}; no entry was removed",
            state_path.display()
        );
    }
    Ok(phase)
}

fn classify_phase(
    parent: &File,
    marker_path: &Path,
    target_name: &OsStr,
    state: &StateDocument,
) -> Result<ReplacementPhase> {
    let canonical = read_marker_at(parent, marker_path)?;
    validate_marker(
        marker_path,
        &canonical.marker,
        target_name,
        state.state.role,
        &state.state.finish_handle,
        &state.state.attempt_identity,
    )?;
    let staged_name = OsStr::from_bytes(&state.state.staged_name);
    let staged_path = marker_path.with_file_name(staged_name);
    let staged = match open_file_at(parent, staged_name) {
        Ok(file) => {
            let document = read_marker(&staged_path, file)?;
            validate_marker_binding(
                &staged_path,
                &document.marker,
                target_name,
                state.state.role,
                &state.state.finish_handle,
                &state.state.attempt_identity,
            )?;
            Some(document)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if entry_exists(parent, staged_name)? {
                bail!(
                    "finish auxiliary staged marker appeared during recovery at {}; no entry was removed",
                    staged_path.display()
                );
            }
            None
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "opening staged finish auxiliary marker {} without following symlinks",
                    staged_path.display()
                )
            })
        }
    };

    let canonical_is_previous = matches_snapshot(&canonical, &state.state.previous);
    let canonical_is_replacement = matches_snapshot(&canonical, &state.state.replacement);
    match staged {
        Some(staged)
            if canonical_is_previous && matches_snapshot(&staged, &state.state.replacement) =>
        {
            validate_bound_artifact(parent, marker_path, &staged.marker)?;
            Ok(ReplacementPhase::BeforeExchange)
        }
        Some(staged)
            if canonical_is_replacement && matches_snapshot(&staged, &state.state.previous) =>
        {
            validate_bound_artifact(parent, marker_path, &canonical.marker)?;
            Ok(ReplacementPhase::AfterExchange { retired: staged })
        }
        None if canonical_is_replacement => {
            validate_bound_artifact(parent, marker_path, &canonical.marker)?;
            Ok(ReplacementPhase::RetiredMarkerRemoved)
        }
        _ => bail!(
            "finish auxiliary marker replacement identity changed around {}; no entry was removed",
            marker_path.display()
        ),
    }
}

fn validate_bound_artifact(
    parent: &File,
    marker_path: &Path,
    marker: &AuxiliaryMarker,
) -> Result<()> {
    let artifact_name = OsStr::from_bytes(&marker.artifact_name);
    let artifact_path = marker_path.with_file_name(artifact_name);
    let artifact = open_file_at(parent, artifact_name).with_context(|| {
        format!(
            "opening bound finish auxiliary replacement {} without following symlinks",
            artifact_path.display()
        )
    })?;
    ensure_regular_file(&artifact, &artifact_path)?;
    let observed = FileIdentity::from_file(&artifact)?;
    let expected = marker_artifact_identity(marker);
    if observed != expected {
        bail!(
            "bound finish auxiliary artifact identity does not match replacement marker {}: expected device/inode {}/{}, observed {}/{}; no entry was removed",
            marker_path.display(),
            expected.device,
            expected.inode,
            observed.device,
            observed.inode
        );
    }
    validate_entry_identity(parent, artifact_name, expected)
        .context("bound finish auxiliary artifact changed before marker adoption")
}

fn marker_artifact_identity(marker: &AuxiliaryMarker) -> FileIdentity {
    FileIdentity {
        device: marker.device,
        inode: marker.inode,
    }
}

fn matches_snapshot(document: &MarkerDocument, snapshot: &MarkerSnapshot) -> bool {
    document.identity == snapshot.identity && document.sha256 == snapshot.sha256
}

fn publish_state(
    parent: &File,
    state_path: &Path,
    make_state: impl FnOnce(FileIdentity) -> MarkerReplacementState,
) -> Result<()> {
    let state_name = state_path
        .file_name()
        .context("finish auxiliary replacement state has no file name")?;
    let (mut temporary, temporary_name, temporary_identity) = create_temporary_marker(parent)?;
    let state = make_state(temporary_identity);
    let body = match serialize_json(&state, "finish auxiliary marker replacement state") {
        Ok(body) => body,
        Err(error) => {
            let cleanup = remove_temporary_marker(parent, &temporary_name, temporary_identity);
            return Err(attach_cleanup_failure(
                error,
                cleanup,
                "removing unserialized finish auxiliary replacement state",
            ));
        }
    };
    if let Err(error) = temporary.write_all(&body) {
        let cleanup = remove_temporary_marker(parent, &temporary_name, temporary_identity);
        return Err(attach_cleanup_failure(
            anyhow::Error::new(error).context("writing finish auxiliary replacement state"),
            cleanup,
            "removing failed temporary finish auxiliary replacement state",
        ));
    }
    match rename_at_noreplace(parent, &temporary_name, parent, state_name) {
        Ok(()) => Ok(()),
        Err(error) => {
            let cleanup = remove_temporary_marker(parent, &temporary_name, temporary_identity);
            Err(attach_cleanup_failure(
                anyhow::Error::new(error).context(format!(
                    "publishing finish auxiliary replacement state {}",
                    state_path.display()
                )),
                cleanup,
                "removing unpublished finish auxiliary replacement state",
            ))
        }
    }
}

fn read_validated_state(
    parent: &File,
    state_path: &Path,
    marker_path: &Path,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<StateDocument> {
    let state_name = state_path
        .file_name()
        .context("finish auxiliary replacement state has no file name")?;
    let mut state_file = open_file_at(parent, state_name).with_context(|| {
        format!(
            "opening finish auxiliary replacement state {} without following symlinks",
            state_path.display()
        )
    })?;
    ensure_regular_file(&state_file, state_path)?;
    let identity = FileIdentity::from_file(&state_file)?;
    let mut body = Vec::new();
    state_file.read_to_end(&mut body).with_context(|| {
        format!(
            "reading finish auxiliary marker replacement state {}",
            state_path.display()
        )
    })?;
    let state: MarkerReplacementState = serde_json::from_slice(&body).with_context(|| {
        format!(
            "parsing finish auxiliary marker replacement state {}",
            state_path.display()
        )
    })?;
    if state.state_identity != identity {
        bail!(
            "finish auxiliary marker replacement state identity changed at {}; no entry was removed",
            state_path.display()
        );
    }
    validate_state(
        state_path,
        marker_path,
        &state,
        role,
        finish_handle,
        attempt_identity,
    )?;
    Ok(StateDocument {
        identity,
        sha256: sha256(&body),
        state,
    })
}

fn validate_state(
    state_path: &Path,
    marker_path: &Path,
    state: &MarkerReplacementState,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<()> {
    if state.version != 1 || state.kind != "finish-auxiliary-marker-replacement" {
        bail!(
            "unsupported finish auxiliary marker replacement state at {}",
            state_path.display()
        );
    }
    if state.role != role
        || state.finish_handle != finish_handle
        || state.attempt_identity != attempt_identity
    {
        bail!(
            "finish auxiliary marker replacement role, handle, or attempt does not match recovery at {}",
            state_path.display()
        );
    }
    validate_component(state_path, "canonical marker", &state.canonical_name)?;
    validate_component(state_path, "staged marker", &state.staged_name)?;
    validate_component(state_path, "canonical artifact", &state.artifact_name)?;
    validate_component(state_path, "staged artifact", &state.staged_artifact_name)?;
    let canonical_name = marker_path
        .file_name()
        .context("finish auxiliary marker has no file name")?;
    if state_path.file_name()
        != Some(replacement_state_file_name(role, attempt_identity).as_os_str())
        || state.canonical_name != canonical_name.as_bytes()
        || state.artifact_name != artifact_file_name(role, attempt_identity).as_bytes()
        || !is_staging_marker_name(
            role,
            attempt_identity,
            OsStr::from_bytes(&state.staged_name),
        )
        || !is_staging_replacement_name(
            role,
            attempt_identity,
            OsStr::from_bytes(&state.staged_artifact_name),
        )
    {
        bail!(
            "finish auxiliary marker replacement path does not match its role and attempt at {}",
            state_path.display()
        );
    }
    for digest in [&state.previous.sha256, &state.replacement.sha256] {
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            bail!(
                "finish auxiliary marker replacement has an invalid SHA-256 digest at {}",
                state_path.display()
            );
        }
    }
    Ok(())
}

fn read_marker_at(parent: &File, marker_path: &Path) -> Result<MarkerDocument> {
    let marker_name = marker_path
        .file_name()
        .context("finish auxiliary marker has no file name")?;
    let marker_file = open_file_at(parent, marker_name).with_context(|| {
        format!(
            "opening finish auxiliary marker {} without following symlinks",
            marker_path.display()
        )
    })?;
    read_marker(marker_path, marker_file)
}

fn remove_marker_document(
    parent: &File,
    marker_name: &OsStr,
    expected: &MarkerDocument,
) -> Result<()> {
    let marker_path = Path::new(marker_name);
    let marker_file = open_file_at(parent, marker_name).with_context(|| {
        format!(
            "opening retired finish auxiliary marker {:?} without following symlinks",
            marker_name
        )
    })?;
    let observed = read_marker(marker_path, marker_file)?;
    if observed.identity != expected.identity
        || observed.sha256 != expected.sha256
        || observed.marker != expected.marker
    {
        bail!(
            "retired finish auxiliary marker identity changed at {:?}; no entry was removed",
            marker_name
        );
    }
    validate_entry_identity(parent, marker_name, expected.identity)
        .context("retired finish auxiliary marker identity changed before removal")?;
    unlink_at(parent, marker_name, 0)
        .with_context(|| format!("removing retired finish auxiliary marker {:?}", marker_name))
}

fn remove_state_document(parent: &File, state_path: &Path, expected: &StateDocument) -> Result<()> {
    let state_name = state_path
        .file_name()
        .context("finish auxiliary replacement state has no file name")?;
    let observed = read_validated_state(
        parent,
        state_path,
        &state_path.with_file_name(OsString::from_vec(expected.state.canonical_name.clone())),
        expected.state.role,
        &expected.state.finish_handle,
        &expected.state.attempt_identity,
    )?;
    if observed.identity != expected.identity
        || observed.sha256 != expected.sha256
        || observed.state != expected.state
    {
        bail!(
            "finish auxiliary marker replacement state identity changed at {}; no entry was removed",
            state_path.display()
        );
    }
    validate_entry_identity(parent, state_name, expected.identity)
        .context("finish auxiliary marker replacement state changed before removal")?;
    unlink_at(parent, state_name, 0).with_context(|| {
        format!(
            "removing completed finish auxiliary marker replacement state {}",
            state_path.display()
        )
    })
}

fn serialize_json(value: &impl Serialize, description: &str) -> Result<Vec<u8>> {
    let mut body =
        serde_json::to_vec_pretty(value).with_context(|| format!("serializing {description}"))?;
    body.push(b'\n');
    Ok(body)
}

fn sha256(body: &[u8]) -> String {
    format!("{:x}", Sha256::digest(body))
}

pub(super) fn replacement_state_file_name(role: AuxiliaryRole, attempt_identity: &str) -> OsString {
    let mut name = marker_file_name(role, attempt_identity).as_bytes().to_vec();
    name.extend_from_slice(REPLACEMENT_SUFFIX);
    OsString::from_vec(name)
}

pub(super) fn replacement_state_path(marker_path: &Path) -> Result<PathBuf> {
    let marker_name = marker_path
        .file_name()
        .context("finish auxiliary marker has no file name")?;
    let mut replacement_name = marker_name.as_bytes().to_vec();
    replacement_name.extend_from_slice(REPLACEMENT_SUFFIX);
    Ok(marker_path.with_file_name(OsString::from_vec(replacement_name)))
}
