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
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

const AUXILIARY_PREFIX: &[u8] = b"GROVE-FINISH-AUXILIARY-";
const MARKER_SUFFIX: &[u8] = b".json";

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

#[derive(Debug)]
pub(crate) struct AuxiliaryCleanup {
    artifact_path: PathBuf,
    marker_path: PathBuf,
    marker_identity: FileIdentity,
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
    let parent = open_directory(parent_path).with_context(|| {
        format!(
            "opening finish auxiliary directory {} without following symlinks",
            parent_path.display()
        )
    })?;
    validate_file_name(&parent, &artifact_name)?;
    validate_file_name(&parent, &marker_name)?;
    if entry_exists(&parent, &artifact_name)? || entry_exists(&parent, &marker_name)? {
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
    validate_file_name(&parent, &artifact_name)?;
    validate_file_name(&parent, &marker_name)?;
    if entry_exists(&parent, &artifact_name)? || entry_exists(&parent, &marker_name)? {
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
    let target_name = target_path
        .file_name()
        .context("finish auxiliary target has no file name")?;
    let artifact_exists = entry_exists(&parent, &artifact_name)?;
    let marker_exists = entry_exists(&parent, &marker_name)?;
    match (artifact_exists, marker_exists) {
        (false, false) => Ok(None),
        (true, false) => bail!(
            "Recovery pending: unmarked finish auxiliary artifact {:?} is present in {}",
            artifact_name,
            parent_path.display()
        ),
        (_, true) => AuxiliaryCleanup::from_marker_at(
            &parent,
            &parent_path.join(marker_name),
            target_name,
            role,
            finish_handle,
            attempt_identity,
        )
        .map(Some),
    }
}

#[allow(dead_code)] // Consumed by the following finish-driver reaping leaf.
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

#[allow(dead_code)] // Consumed by the following finish-driver reaping leaf.
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
    let (_, marker) = read_marker(marker_path, marker_file)?;
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

    pub(crate) fn rebind_artifact_identity(&mut self) -> Result<()> {
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
        let artifact_name = OsString::from_vec(self.marker.artifact_name.clone());
        let artifact = open_file_at(&parent, &artifact_name).with_context(|| {
            format!(
                "opening changed finish auxiliary artifact {} without following symlinks",
                self.artifact_path.display()
            )
        })?;
        ensure_regular_file(&artifact, &self.artifact_path)?;
        let artifact_identity = FileIdentity::from_file(&artifact)?;
        validate_entry_identity(&parent, &artifact_name, artifact_identity)
            .context("finish auxiliary artifact changed while rebinding its marker")?;
        if artifact_identity.device == self.marker.device
            && artifact_identity.inode == self.marker.inode
        {
            return Ok(());
        }

        let mut marker = self.marker.clone();
        marker.device = artifact_identity.device;
        marker.inode = artifact_identity.inode;
        self.remove_marker(&parent)?;
        publish_marker(&parent, &self.marker_path, &marker)?;
        *self = Self::from_marker_at(
            &parent,
            &self.marker_path,
            OsString::from_vec(marker.target_name.clone()).as_os_str(),
            marker.role,
            &marker.finish_handle,
            &marker.attempt_identity,
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
        let (identity, marker) = read_marker(&self.marker_path, marker_file)?;
        if identity != self.marker_identity || marker != self.marker {
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
        let marker_name = marker_path
            .file_name()
            .context("finish auxiliary marker has no file name")?;
        let marker_file = open_file_at(parent, marker_name).with_context(|| {
            format!(
                "opening finish auxiliary marker {} without following symlinks",
                marker_path.display()
            )
        })?;
        let (marker_identity, marker) = read_marker(marker_path, marker_file)?;
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

fn read_marker(
    marker_path: &Path,
    mut marker_file: File,
) -> Result<(FileIdentity, AuxiliaryMarker)> {
    ensure_regular_file(&marker_file, marker_path)?;
    let marker_identity = FileIdentity::from_file(&marker_file)?;
    let mut body = Vec::new();
    marker_file
        .read_to_end(&mut body)
        .with_context(|| format!("reading finish auxiliary marker {}", marker_path.display()))?;
    let marker = serde_json::from_slice(&body)
        .with_context(|| format!("parsing finish auxiliary marker {}", marker_path.display()))?;
    Ok((marker_identity, marker))
}

fn validate_marker(
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
    if marker_path.file_name() != Some(marker_file_name(role, attempt_identity).as_os_str())
        || marker.artifact_name != artifact_file_name(role, attempt_identity).as_bytes()
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
