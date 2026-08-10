mod marker_replacement;

use self::marker_replacement::{
    publish_marker_replacement, recover_marker_replacement, replacement_state_file_name,
    replacement_state_path, settle_marker_replacement, MarkerReplacementStep,
};
use super::unix::{
    create_new_file_at, directory_names, entry_exists, open_directory, open_file_at,
    rename_at_noreplace, rename_at_replace, unlink_at, validate_entry_identity, validate_file_name,
};
use super::{
    attach_cleanup_failure, create_temporary_marker, remove_temporary_marker,
    validate_attempt_identity, FileIdentity,
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

const AUXILIARY_PREFIX: &[u8] = b"GROVE-FINISH-AUXILIARY-";
const MARKER_SUFFIX: &[u8] = b".json";
const STAGING_INFIX: &[u8] = b".staging-";
const STAGING_NONCE_LENGTH: usize = 32;

/// Where `replace_artifact_from` is in publishing a new artifact identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReplacementPublicationStep {
    BeforeStatePublication,
    AfterStatePublication,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AuxiliaryRole {
    GitIndexBackup,
    GitIndexSuccess,
}

impl AuxiliaryRole {
    fn name(self) -> &'static str {
        match self {
            Self::GitIndexBackup => "git-index-backup",
            Self::GitIndexSuccess => "git-index-success",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct AuxiliaryMarker {
    version: u32,
    kind: String,
    role: AuxiliaryRole,
    finish_handle: String,
    attempt_identity: String,
    artifact_name: Vec<u8>,
    target_name: Vec<u8>,
    device: u64,
    inode: u64,
}

struct MarkerDocument {
    identity: FileIdentity,
    sha256: String,
    marker: AuxiliaryMarker,
}

#[derive(Debug)]
pub(crate) struct AuxiliaryCleanup {
    artifact_path: PathBuf,
    marker_path: PathBuf,
    marker_identity: FileIdentity,
    marker_sha256: String,
    marker: AuxiliaryMarker,
}

pub(crate) fn prepare_auxiliary(
    source_path: &Path,
    target_path: &Path,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<AuxiliaryCleanup> {
    validate_attempt_identity(attempt_identity)?;
    if finish_handle.is_empty() {
        bail!("finish auxiliary cleanup handle is empty");
    }
    let parent_path = target_path
        .parent()
        .context("finish auxiliary target has no parent")?;
    if source_path.parent() != Some(parent_path) {
        bail!("finish auxiliary source and target must share one directory");
    }
    let target_name = target_path
        .file_name()
        .context("finish auxiliary target has no file name")?;
    let artifact_name = artifact_file_name(role, attempt_identity);
    let marker_name = marker_file_name(role, attempt_identity);
    let replacement_state_name = replacement_state_file_name(role, attempt_identity);
    let parent = open_directory(parent_path).with_context(|| {
        format!(
            "opening finish auxiliary directory {} without following symlinks",
            parent_path.display()
        )
    })?;
    validate_file_name(&parent, &artifact_name)?;
    validate_file_name(&parent, &marker_name)?;
    validate_file_name(&parent, &replacement_state_name)?;
    if entry_exists(&parent, &artifact_name)?
        || entry_exists(&parent, &marker_name)?
        || entry_exists(&parent, &replacement_state_name)?
    {
        bail!(
            "finish auxiliary cleanup collision for role {} and attempt {} in {}",
            role.name(),
            attempt_identity,
            parent_path.display()
        );
    }

    let source_name = source_path
        .file_name()
        .context("finish auxiliary source has no file name")?;
    let mut source = open_file_at(&parent, source_name).with_context(|| {
        format!(
            "opening finish auxiliary source {} without following symlinks",
            source_path.display()
        )
    })?;
    ensure_regular_file(&source, source_path)?;
    let mut artifact = create_new_file_at(&parent, &artifact_name).with_context(|| {
        format!(
            "creating finish auxiliary artifact {:?} in {}",
            artifact_name,
            parent_path.display()
        )
    })?;
    if let Err(error) = io::copy(&mut source, &mut artifact) {
        let identity = FileIdentity::from_file(&artifact)?;
        let cleanup = remove_artifact(&parent, &artifact_name, identity);
        return Err(attach_cleanup_failure(
            anyhow::Error::new(error).context("copying finish auxiliary artifact"),
            cleanup,
            "removing incomplete finish auxiliary artifact",
        ));
    }
    let artifact_identity = FileIdentity::from_file(&artifact)?;
    let marker = AuxiliaryMarker {
        version: 1,
        kind: "finish-index-auxiliary".to_owned(),
        role,
        finish_handle: finish_handle.to_owned(),
        attempt_identity: attempt_identity.to_owned(),
        artifact_name: artifact_name.as_bytes().to_vec(),
        target_name: target_name.as_bytes().to_vec(),
        device: artifact_identity.device,
        inode: artifact_identity.inode,
    };
    let marker_path = parent_path.join(&marker_name);
    publish_marker(&parent, &marker_path, &marker).or_else(|error| {
        let cleanup = remove_artifact(&parent, &artifact_name, artifact_identity);
        Err(attach_cleanup_failure(
            error,
            cleanup,
            "removing unmarked finish auxiliary artifact",
        ))
    })?;
    AuxiliaryCleanup::from_marker_at(
        &parent,
        &marker_path,
        target_name,
        role,
        finish_handle,
        attempt_identity,
    )
}

pub(crate) fn auxiliary_artifact_path(
    target_path: &Path,
    role: AuxiliaryRole,
    attempt_identity: &str,
) -> Result<PathBuf> {
    validate_attempt_identity(attempt_identity)?;
    Ok(target_path.with_file_name(artifact_file_name(role, attempt_identity)))
}

pub(crate) fn ensure_auxiliary_available(
    target_path: &Path,
    role: AuxiliaryRole,
    attempt_identity: &str,
) -> Result<()> {
    validate_attempt_identity(attempt_identity)?;
    let parent_path = target_path
        .parent()
        .context("finish auxiliary target has no parent")?;
    let parent = open_directory(parent_path).with_context(|| {
        format!(
            "opening finish auxiliary preflight directory {} without following symlinks",
            parent_path.display()
        )
    })?;
    let artifact_name = artifact_file_name(role, attempt_identity);
    let marker_name = marker_file_name(role, attempt_identity);
    let replacement_state_name = replacement_state_file_name(role, attempt_identity);
    validate_file_name(&parent, &artifact_name)?;
    validate_file_name(&parent, &marker_name)?;
    validate_file_name(&parent, &replacement_state_name)?;
    if entry_exists(&parent, &artifact_name)?
        || entry_exists(&parent, &marker_name)?
        || entry_exists(&parent, &replacement_state_name)?
    {
        bail!(
            "finish auxiliary cleanup collision for role {} and attempt {} in {}",
            role.name(),
            attempt_identity,
            parent_path.display()
        );
    }
    Ok(())
}

pub(crate) fn recover_auxiliary(
    target_path: &Path,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<Option<AuxiliaryCleanup>> {
    validate_attempt_identity(attempt_identity)?;
    let parent_path = target_path
        .parent()
        .context("finish auxiliary target has no parent")?;
    let parent = open_directory(parent_path).with_context(|| {
        format!(
            "opening finish auxiliary recovery directory {} without following symlinks",
            parent_path.display()
        )
    })?;
    let artifact_name = artifact_file_name(role, attempt_identity);
    let marker_name = marker_file_name(role, attempt_identity);
    let replacement_state_name = replacement_state_file_name(role, attempt_identity);
    let target_name = target_path
        .file_name()
        .context("finish auxiliary target has no file name")?;
    if entry_exists(&parent, &marker_name)? {
        return AuxiliaryCleanup::from_marker_at(
            &parent,
            &parent_path.join(marker_name),
            target_name,
            role,
            finish_handle,
            attempt_identity,
        )
        .map(Some);
    }
    if entry_exists(&parent, &replacement_state_name)? {
        bail!(
            "Recovery pending: finish auxiliary replacement state {:?} has no canonical marker in {}",
            replacement_state_name,
            parent_path.display()
        );
    }
    if entry_exists(&parent, &artifact_name)? {
        bail!(
            "Recovery pending: unmarked finish auxiliary artifact {:?} is present in {}",
            artifact_name,
            parent_path.display()
        );
    }
    Ok(None)
}

pub(crate) fn auxiliary_marker_paths(directory_path: &Path) -> Result<Vec<PathBuf>> {
    let directory = open_directory(directory_path).with_context(|| {
        format!(
            "opening finish auxiliary discovery directory {} without following symlinks",
            directory_path.display()
        )
    })?;
    let mut markers = Vec::new();
    for name in directory_names(&directory)? {
        let bytes = name.as_bytes();
        if bytes.starts_with(AUXILIARY_PREFIX) && bytes.ends_with(MARKER_SUFFIX) {
            markers.push(directory_path.join(name));
        }
    }
    Ok(markers)
}

pub(crate) fn recover_auxiliary_marker(marker_path: &Path) -> Result<AuxiliaryCleanup> {
    let parent_path = marker_path
        .parent()
        .context("finish auxiliary marker has no parent")?;
    let parent = open_directory(parent_path).with_context(|| {
        format!(
            "opening discovered finish auxiliary directory {} without following symlinks",
            parent_path.display()
        )
    })?;
    let marker_name = marker_path
        .file_name()
        .context("finish auxiliary marker has no file name")?;
    let marker_file = open_file_at(&parent, marker_name).with_context(|| {
        format!(
            "opening discovered finish auxiliary marker {} without following symlinks",
            marker_path.display()
        )
    })?;
    let marker = read_marker(marker_path, marker_file)?.marker;
    let target_name = OsString::from_vec(marker.target_name.clone());
    AuxiliaryCleanup::from_marker_at(
        &parent,
        marker_path,
        &target_name,
        marker.role,
        &marker.finish_handle,
        &marker.attempt_identity,
    )
}

impl AuxiliaryCleanup {
    pub(crate) fn artifact_path(&self) -> &Path {
        &self.artifact_path
    }

    pub(super) fn is_owned_by(&self, owner: &super::CleanupOwner) -> bool {
        self.marker.finish_handle == owner.finish_handle
            && self.marker.attempt_identity == owner.attempt_identity
    }

    /// Record which inode the artifact name is about to adopt.
    ///
    /// The replacement must sit at a freshly drawn staging name inside this
    /// auxiliary's own role-and-attempt namespace. The state document this
    /// publishes is the only authority a later recovery has for the two names it
    /// exchanges, and a writer able to rewrite that document in place can always
    /// make it agree with itself. The namespace is therefore a bound rather than
    /// a proof: it keeps every entry such a document can name inside Grove's own
    /// reserved staging names in the VCS administration directory, which that
    /// writer already owns. Accepting an arbitrary regular file here would break
    /// the bound and let the document redirect the exchange at an unrelated
    /// entry in the same directory.
    fn bind_artifact_replacement(
        &mut self,
        staging_name: &OsStr,
        staging_identity: FileIdentity,
    ) -> Result<()> {
        let parent_path = self
            .marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?;
        if !is_staging_replacement_name(
            self.marker.role,
            &self.marker.attempt_identity,
            staging_name,
        ) {
            bail!(
                "finish auxiliary replacement must be published from this attempt's staging namespace, not {:?}",
                staging_name
            );
        }
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening finish auxiliary replacement-binding directory {}",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;
        let artifact_name = OsString::from_vec(self.marker.artifact_name.clone());
        self.open_validated_artifact(&parent, &artifact_name)?
            .context("marked finish auxiliary artifact is absent while binding its replacement")?;

        let staging_path = parent_path.join(staging_name);
        let staging = open_file_at(&parent, staging_name).with_context(|| {
            format!(
                "opening staged finish auxiliary replacement {} without following symlinks",
                staging_path.display()
            )
        })?;
        ensure_regular_file(&staging, &staging_path)?;
        let observed = FileIdentity::from_file(&staging)?;
        if observed != staging_identity {
            bail!(
                "staged finish auxiliary replacement identity changed at {}: expected device/inode {}/{}, observed {}/{}; no entry was moved",
                staging_path.display(),
                staging_identity.device,
                staging_identity.inode,
                observed.device,
                observed.inode
            );
        }
        validate_entry_identity(&parent, staging_name, staging_identity)
            .context("finish auxiliary replacement changed while binding its identity")?;

        let mut replacement_marker = self.marker.clone();
        replacement_marker.device = staging_identity.device;
        replacement_marker.inode = staging_identity.inode;
        publish_marker_replacement(
            &parent,
            &self.marker_path,
            &replacement_marker,
            self.marker_identity,
            &self.marker_sha256,
            self.artifact_identity(),
            staging_name,
        )
    }

    pub(crate) fn replace_artifact_from(&mut self, source_path: &Path) -> Result<()> {
        self.replace_artifact_from_with(source_path, |_| Ok(()))
    }

    /// Copy `source_path` in as this auxiliary's new artifact identity.
    ///
    /// The staging entry is created under a freshly drawn name and its identity
    /// is recorded in the published state document before anything else can
    /// name it. Nothing derivable from the role and attempt is ever claimed, so
    /// an interruption leaves no entry that a later recovery would have to
    /// adopt or remove without proof of what wrote it.
    fn replace_artifact_from_with(
        &mut self,
        source_path: &Path,
        mut checkpoint: impl FnMut(ReplacementPublicationStep) -> io::Result<()>,
    ) -> Result<()> {
        let source_parent_path = source_path
            .parent()
            .context("filtered finish auxiliary source has no parent")?;
        let source_parent = open_directory(source_parent_path).with_context(|| {
            format!(
                "opening filtered finish auxiliary source directory {} without following symlinks",
                source_parent_path.display()
            )
        })?;
        let source_name = source_path
            .file_name()
            .context("filtered finish auxiliary source has no file name")?;
        let mut source = open_file_at(&source_parent, source_name).with_context(|| {
            format!(
                "opening filtered finish auxiliary source {} without following symlinks",
                source_path.display()
            )
        })?;
        ensure_regular_file(&source, source_path)?;

        let parent_path = self
            .marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening finish auxiliary publication directory {} without following symlinks",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;
        let artifact_name = OsString::from_vec(self.marker.artifact_name.clone());
        self.open_validated_artifact(&parent, &artifact_name)?
            .context("marked finish auxiliary artifact is absent before publication")?;

        let (mut staging, staging_name, staging_identity) = create_staging_entry(
            &parent,
            parent_path,
            artifact_file_name(self.marker.role, &self.marker.attempt_identity).as_os_str(),
        )?;
        if let Err(error) = io::copy(&mut source, &mut staging) {
            let cleanup = remove_artifact(&parent, &staging_name, staging_identity);
            return Err(attach_cleanup_failure(
                anyhow::Error::new(error).context("copying staged finish auxiliary replacement"),
                cleanup,
                "removing incomplete staged finish auxiliary replacement",
            ));
        }
        if let Err(error) = validate_entry_identity(&parent, &staging_name, staging_identity) {
            let cleanup = remove_artifact(&parent, &staging_name, staging_identity);
            return Err(attach_cleanup_failure(
                error.context("staged finish auxiliary replacement changed after copying"),
                cleanup,
                "removing substituted staged finish auxiliary replacement",
            ));
        }
        drop(staging);

        let published = checkpoint(ReplacementPublicationStep::BeforeStatePublication)
            .context("replacement-publication checkpoint before the state document")
            .and_then(|()| self.bind_artifact_replacement(&staging_name, staging_identity));
        if let Err(error) = published {
            let cleanup = remove_artifact(&parent, &staging_name, staging_identity);
            return Err(attach_cleanup_failure(
                error,
                cleanup,
                "removing unpublished staged finish auxiliary replacement",
            ));
        }
        checkpoint(ReplacementPublicationStep::AfterStatePublication)
            .context("replacement-publication checkpoint after the state document")?;
        self.rebind_artifact_identity()
    }

    pub(crate) fn rebind_artifact_identity(&mut self) -> Result<()> {
        self.rebind_artifact_identity_with(|_| Ok(()))
    }

    fn rebind_artifact_identity_with(
        &mut self,
        mut checkpoint: impl FnMut(MarkerReplacementStep) -> io::Result<()>,
    ) -> Result<()> {
        let parent_path = self
            .marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening finish auxiliary rebind directory {}",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;
        settle_marker_replacement(
            &parent,
            &self.marker_path,
            OsString::from_vec(self.marker.target_name.clone()).as_os_str(),
            self.marker.role,
            &self.marker.finish_handle,
            &self.marker.attempt_identity,
            &mut checkpoint,
        )?;
        *self = Self::from_marker_at(
            &parent,
            &self.marker_path,
            OsString::from_vec(self.marker.target_name.clone()).as_os_str(),
            self.marker.role,
            &self.marker.finish_handle,
            &self.marker.attempt_identity,
        )?;
        Ok(())
    }

    pub(crate) fn restore_target(&self) -> Result<()> {
        let parent_path = self
            .marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening finish auxiliary restore directory {}",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;
        let artifact_name = OsString::from_vec(self.marker.artifact_name.clone());
        let mut artifact = self
            .open_validated_artifact(&parent, &artifact_name)?
            .context("marked finish auxiliary artifact is absent during restoration")?;
        let target_name = OsString::from_vec(self.marker.target_name.clone());
        let (mut temporary, temporary_name, temporary_identity) = create_temporary_marker(&parent)?;
        if let Err(error) = io::copy(&mut artifact, &mut temporary) {
            let cleanup = remove_temporary_marker(&parent, &temporary_name, temporary_identity);
            return Err(attach_cleanup_failure(
                anyhow::Error::new(error).context("copying restored finish auxiliary target"),
                cleanup,
                "removing incomplete finish auxiliary restore file",
            ));
        }
        if let Err(error) = validate_entry_identity(&parent, &temporary_name, temporary_identity) {
            let cleanup = remove_temporary_marker(&parent, &temporary_name, temporary_identity);
            return Err(attach_cleanup_failure(
                error.context("finish auxiliary restore file changed before activation"),
                cleanup,
                "removing invalid finish auxiliary restore file",
            ));
        }
        match rename_at_replace(&parent, &temporary_name, &parent, &target_name) {
            Ok(()) => Ok(()),
            Err(error) => {
                let cleanup = remove_temporary_marker(&parent, &temporary_name, temporary_identity);
                Err(attach_cleanup_failure(
                    anyhow::Error::new(error).context(format!(
                        "restoring finish auxiliary target {:?} in {}",
                        target_name,
                        parent_path.display()
                    )),
                    cleanup,
                    "removing failed finish auxiliary restore file",
                ))
            }
        }
    }

    pub(crate) fn activate(&self) -> Result<()> {
        let parent_path = self
            .marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening finish auxiliary activation directory {}",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;
        self.ensure_no_pending_replacement(&parent)?;
        let artifact_name = OsString::from_vec(self.marker.artifact_name.clone());
        let target_name = OsString::from_vec(self.marker.target_name.clone());
        if self
            .open_validated_artifact(&parent, &artifact_name)?
            .is_some()
        {
            rename_at_replace(&parent, &artifact_name, &parent, &target_name).with_context(
                || {
                    format!(
                        "activating marked finish auxiliary artifact {}",
                        self.artifact_path.display()
                    )
                },
            )?;
        }

        let target_path = parent_path.join(&target_name);
        let target = open_file_at(&parent, &target_name).with_context(|| {
            format!(
                "opening activated finish auxiliary target {} without following symlinks",
                target_path.display()
            )
        })?;
        ensure_regular_file(&target, &target_path)?;
        let observed = FileIdentity::from_file(&target)?;
        let expected = self.artifact_identity();
        if observed != expected {
            bail!(
                "activated finish auxiliary target identity does not match marker {}: expected device/inode {}/{}, observed {}/{}; replacement left untouched",
                self.marker_path.display(),
                expected.device,
                expected.inode,
                observed.device,
                observed.inode
            );
        }
        validate_entry_identity(&parent, &target_name, expected)
            .context("activated finish auxiliary target changed; marker left untouched")?;
        self.remove_marker(&parent)
    }

    pub(crate) fn dispose(&self) -> Result<()> {
        let parent_path = self
            .marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening finish auxiliary cleanup directory {}",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;
        self.ensure_no_pending_replacement(&parent)?;
        let artifact_name = OsString::from_vec(self.marker.artifact_name.clone());
        let artifact = match open_file_at(&parent, &artifact_name) {
            Ok(artifact) => artifact,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                if entry_exists(&parent, &artifact_name)? {
                    bail!(
                        "finish auxiliary artifact appeared during cleanup at {}; marker left untouched",
                        self.artifact_path.display()
                    );
                }
                self.remove_marker(&parent)?;
                return Ok(());
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "opening marked finish auxiliary artifact {} without following symlinks",
                        self.artifact_path.display()
                    )
                })
            }
        };
        ensure_regular_file(&artifact, &self.artifact_path)?;
        let observed = FileIdentity::from_file(&artifact)?;
        let expected = FileIdentity {
            device: self.marker.device,
            inode: self.marker.inode,
        };
        if observed != expected {
            bail!(
                "finish auxiliary artifact identity does not match marker {}: expected device/inode {}/{}, observed {}/{}; replacement left untouched",
                self.marker_path.display(),
                expected.device,
                expected.inode,
                observed.device,
                observed.inode
            );
        }
        validate_entry_identity(&parent, &artifact_name, expected).context(
            "finish auxiliary artifact changed before disposal; replacement left untouched",
        )?;
        unlink_at(&parent, &artifact_name, 0).with_context(|| {
            format!(
                "removing marked finish auxiliary artifact {}",
                self.artifact_path.display()
            )
        })?;
        self.remove_marker(&parent)
    }

    /// Refuse to retire a marker whose replacement is still on disk.
    ///
    /// A published replacement state means this value's snapshot describes a
    /// superseded phase: the canonical marker and artifact are mid-transition,
    /// and only settlement decides which identities survive. Both still validate
    /// against the pre-replacement snapshot until the artifact exchange runs, so
    /// a caller holding that snapshot — the production path discards the
    /// temporary index through exactly this value — would remove the pair
    /// settlement and recovery need, turning a synchronous failure into an
    /// unrecoverable shape. Recovery re-reads this auxiliary from disk and
    /// settles the replacement first, so it never reaches this guard.
    fn ensure_no_pending_replacement(&self, parent: &File) -> Result<()> {
        let state_path = replacement_state_path(&self.marker_path)?;
        let state_name = state_path
            .file_name()
            .context("finish auxiliary replacement state has no file name")?;
        if entry_exists(parent, state_name)? {
            bail!(
                "finish auxiliary replacement is still in progress at {}; recovery must settle it before this marker can be retired",
                state_path.display()
            );
        }
        Ok(())
    }

    fn revalidate_marker(&self, parent: &File) -> Result<()> {
        let marker_name = self
            .marker_path
            .file_name()
            .context("finish auxiliary marker has no file name")?;
        let marker_file = open_file_at(parent, marker_name).with_context(|| {
            format!(
                "opening finish auxiliary marker {} without following symlinks",
                self.marker_path.display()
            )
        })?;
        let observed = read_marker(&self.marker_path, marker_file)?;
        if observed.identity != self.marker_identity
            || observed.sha256 != self.marker_sha256
            || observed.marker != self.marker
        {
            bail!(
                "finish auxiliary cleanup marker changed at {}; replacement left untouched",
                self.marker_path.display()
            );
        }
        Ok(())
    }

    fn remove_marker(&self, parent: &File) -> Result<()> {
        self.revalidate_marker(parent)?;
        let marker_name = self
            .marker_path
            .file_name()
            .context("finish auxiliary marker has no file name")?;
        unlink_at(parent, marker_name, 0).with_context(|| {
            format!(
                "removing completed finish auxiliary marker {}",
                self.marker_path.display()
            )
        })
    }

    fn from_marker_at(
        parent: &File,
        marker_path: &Path,
        target_name: &OsStr,
        role: AuxiliaryRole,
        finish_handle: &str,
        attempt_identity: &str,
    ) -> Result<Self> {
        recover_marker_replacement(
            parent,
            marker_path,
            target_name,
            role,
            finish_handle,
            attempt_identity,
        )?;
        let marker_name = marker_path
            .file_name()
            .context("finish auxiliary marker has no file name")?;
        let marker_file = open_file_at(parent, marker_name).with_context(|| {
            format!(
                "opening finish auxiliary marker {} without following symlinks",
                marker_path.display()
            )
        })?;
        let document = read_marker(marker_path, marker_file)?;
        let marker_identity = document.identity;
        let marker_sha256 = document.sha256;
        let marker = document.marker;
        validate_marker(
            marker_path,
            &marker,
            target_name,
            role,
            finish_handle,
            attempt_identity,
        )?;
        let artifact_path = marker_path
            .parent()
            .context("finish auxiliary marker has no parent")?
            .join(OsString::from_vec(marker.artifact_name.clone()));
        let artifact_name = artifact_path
            .file_name()
            .context("finish auxiliary artifact has no file name")?
            .to_owned();
        let cleanup = Self {
            artifact_path,
            marker_path: marker_path.to_path_buf(),
            marker_identity,
            marker_sha256,
            marker,
        };
        cleanup.open_validated_artifact(parent, &artifact_name)?;
        Ok(cleanup)
    }

    fn open_validated_artifact(
        &self,
        parent: &File,
        artifact_name: &OsStr,
    ) -> Result<Option<File>> {
        let artifact = match open_file_at(parent, artifact_name) {
            Ok(artifact) => artifact,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                if entry_exists(parent, artifact_name)? {
                    bail!(
                        "finish auxiliary artifact appeared during validation at {}; marker left untouched",
                        self.artifact_path.display()
                    );
                }
                return Ok(None);
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "opening marked finish auxiliary artifact {} without following symlinks",
                        self.artifact_path.display()
                    )
                })
            }
        };
        ensure_regular_file(&artifact, &self.artifact_path)?;
        let observed = FileIdentity::from_file(&artifact)?;
        let expected = self.artifact_identity();
        if observed != expected {
            bail!(
                "finish auxiliary artifact identity does not match marker {}: expected device/inode {}/{}, observed {}/{}; replacement left untouched",
                self.marker_path.display(),
                expected.device,
                expected.inode,
                observed.device,
                observed.inode
            );
        }
        validate_entry_identity(parent, artifact_name, expected)
            .context("finish auxiliary artifact changed; replacement left untouched")?;
        Ok(Some(artifact))
    }

    fn artifact_identity(&self) -> FileIdentity {
        FileIdentity {
            device: self.marker.device,
            inode: self.marker.inode,
        }
    }
}

fn publish_marker(parent: &File, marker_path: &Path, marker: &AuxiliaryMarker) -> Result<()> {
    let marker_name = marker_path
        .file_name()
        .context("finish auxiliary marker has no file name")?;
    let mut body =
        serde_json::to_vec_pretty(marker).context("serializing finish auxiliary cleanup marker")?;
    body.push(b'\n');
    let (mut temporary, temporary_name, temporary_identity) = create_temporary_marker(parent)?;
    if let Err(error) = temporary.write_all(&body) {
        let cleanup = remove_temporary_marker(parent, &temporary_name, temporary_identity);
        return Err(attach_cleanup_failure(
            anyhow::Error::new(error).context("writing finish auxiliary cleanup marker"),
            cleanup,
            "removing failed temporary finish auxiliary marker",
        ));
    }
    match rename_at_noreplace(parent, &temporary_name, parent, marker_name) {
        Ok(()) => Ok(()),
        Err(error) => {
            let cleanup = remove_temporary_marker(parent, &temporary_name, temporary_identity);
            Err(attach_cleanup_failure(
                anyhow::Error::new(error).context(format!(
                    "publishing finish auxiliary cleanup marker {}",
                    marker_path.display()
                )),
                cleanup,
                "removing unpublished finish auxiliary marker",
            ))
        }
    }
}

fn read_marker(marker_path: &Path, mut marker_file: File) -> Result<MarkerDocument> {
    ensure_regular_file(&marker_file, marker_path)?;
    let identity = FileIdentity::from_file(&marker_file)?;
    let mut body = Vec::new();
    marker_file
        .read_to_end(&mut body)
        .with_context(|| format!("reading finish auxiliary marker {}", marker_path.display()))?;
    let marker = serde_json::from_slice(&body)
        .with_context(|| format!("parsing finish auxiliary marker {}", marker_path.display()))?;
    Ok(MarkerDocument {
        identity,
        sha256: sha256(&body),
        marker,
    })
}

fn sha256(body: &[u8]) -> String {
    format!("{:x}", Sha256::digest(body))
}

fn validate_marker(
    marker_path: &Path,
    marker: &AuxiliaryMarker,
    target_name: &OsStr,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<()> {
    validate_marker_binding(
        marker_path,
        marker,
        target_name,
        role,
        finish_handle,
        attempt_identity,
    )?;
    if marker_path.file_name() != Some(marker_file_name(role, attempt_identity).as_os_str()) {
        bail!(
            "finish auxiliary marker path does not match its role and attempt at {}",
            marker_path.display()
        );
    }
    Ok(())
}

fn validate_marker_binding(
    marker_path: &Path,
    marker: &AuxiliaryMarker,
    target_name: &OsStr,
    role: AuxiliaryRole,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<()> {
    if marker.version != 1 || marker.kind != "finish-index-auxiliary" {
        bail!(
            "unsupported finish auxiliary cleanup marker at {}",
            marker_path.display()
        );
    }
    if marker.role != role
        || marker.finish_handle != finish_handle
        || marker.attempt_identity != attempt_identity
    {
        bail!(
            "finish auxiliary marker role, handle, or attempt does not match recovery at {}",
            marker_path.display()
        );
    }
    validate_attempt_identity(&marker.attempt_identity)?;
    validate_component(marker_path, "artifact", &marker.artifact_name)?;
    validate_component(marker_path, "target", &marker.target_name)?;
    if marker.artifact_name != artifact_file_name(role, attempt_identity).as_bytes()
        || marker.target_name != target_name.as_bytes()
    {
        bail!(
            "finish auxiliary marker path does not match its role and attempt at {}",
            marker_path.display()
        );
    }
    Ok(())
}

fn validate_component(marker_path: &Path, role: &str, name: &[u8]) -> Result<()> {
    if name.is_empty()
        || name == b"."
        || name == b".."
        || name.contains(&b'/')
        || name.contains(&b'\0')
    {
        bail!(
            "finish auxiliary marker has an invalid {role} file name at {}",
            marker_path.display()
        );
    }
    Ok(())
}

fn ensure_regular_file(file: &File, path: &Path) -> Result<()> {
    if !file.metadata()?.file_type().is_file() {
        bail!(
            "finish auxiliary cleanup object is not a regular file: {}",
            path.display()
        );
    }
    Ok(())
}

fn remove_artifact(parent: &File, name: &OsStr, identity: FileIdentity) -> Result<()> {
    validate_entry_identity(parent, name, identity)
        .context("finish auxiliary artifact identity changed; replacement left untouched")?;
    unlink_at(parent, name, 0).context("removing finish auxiliary artifact")
}

fn artifact_file_name(role: AuxiliaryRole, attempt_identity: &str) -> OsString {
    OsString::from_vec(
        [
            AUXILIARY_PREFIX,
            role.name().as_bytes(),
            b"-",
            attempt_identity.as_bytes(),
        ]
        .concat(),
    )
}

fn staging_file_name(base_name: &OsStr, nonce: &str) -> OsString {
    OsString::from_vec([base_name.as_bytes(), STAGING_INFIX, nonce.as_bytes()].concat())
}

/// Whether `name` is a staging entry belonging to `base_name`.
///
/// This is what bounds a forged state document, not what proves ownership. The
/// nonce makes the name undrawable in advance, so nothing else can *anticipate*
/// the entry a live publication is about to claim; but a document rewritten in
/// place can always describe an entry its author put at some other name of this
/// shape. What the prefix guarantees is that every such entry lies inside this
/// role and attempt's own reserved namespace in the VCS administration
/// directory — a writer who can forge the document already owns that directory
/// outright, so the redirection confers nothing. No name outside the namespace
/// is reachable, and no ordinary path ever removes an entry Grove did not
/// create.
fn is_staging_file_name(base_name: &OsStr, name: &OsStr) -> bool {
    let prefix = [base_name.as_bytes(), STAGING_INFIX].concat();
    name.as_bytes()
        .strip_prefix(prefix.as_slice())
        .is_some_and(|nonce| {
            nonce.len() == STAGING_NONCE_LENGTH && nonce.iter().all(u8::is_ascii_hexdigit)
        })
}

pub(super) fn is_staging_replacement_name(
    role: AuxiliaryRole,
    attempt_identity: &str,
    name: &OsStr,
) -> bool {
    is_staging_file_name(artifact_file_name(role, attempt_identity).as_os_str(), name)
}

pub(super) fn is_staging_marker_name(
    role: AuxiliaryRole,
    attempt_identity: &str,
    name: &OsStr,
) -> bool {
    is_staging_file_name(marker_file_name(role, attempt_identity).as_os_str(), name)
}

/// Create an entry under a freshly drawn name belonging to `base_name`.
///
/// A staging entry outlives the publication that creates it, so its name — not
/// a document that may not have been written yet — is what says which role and
/// attempt owns it after a process death.
fn create_staging_entry(
    parent: &File,
    parent_path: &Path,
    base_name: &OsStr,
) -> Result<(File, OsString, FileIdentity)> {
    for _ in 0..crate::driver_lease::RANDOM_DRAW_RETRY_LIMIT {
        let nonce =
            crate::driver_lease::fresh_nonce().context("drawing finish auxiliary staging nonce")?;
        let name = staging_file_name(base_name, &nonce);
        validate_file_name(parent, &name)
            .context("staged finish auxiliary name is not valid for its directory")?;
        match create_new_file_at(parent, &name) {
            Ok(file) => {
                let identity = FileIdentity::from_file(&file)?;
                return Ok((file, name, identity));
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "creating staged finish auxiliary entry in {}",
                        parent_path.display()
                    )
                })
            }
        }
    }
    bail!(
        "could not allocate a staged finish auxiliary entry in {} after {} attempts",
        parent_path.display(),
        crate::driver_lease::RANDOM_DRAW_RETRY_LIMIT
    )
}

fn marker_file_name(role: AuxiliaryRole, attempt_identity: &str) -> OsString {
    OsString::from_vec(
        [
            artifact_file_name(role, attempt_identity).as_bytes(),
            MARKER_SUFFIX,
        ]
        .concat(),
    )
}

#[cfg(test)]
mod tests;
