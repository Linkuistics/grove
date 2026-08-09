use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

mod unix;
use unix::{
    entry_exists, open_directory, open_directory_at, open_file_at, remove_directory_contents,
    rename_at_noreplace, unlink_at, validate_entry_identity,
};

const CLEANUP_MARKER_PREFIX: &[u8] = b"GROVE-FINISH-CLEANUP-";
const CLEANUP_MARKER_SUFFIX: &[u8] = b".json";
const REAPING_PREFIX: &[u8] = b"REAPING-";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct QuarantineMarker {
    version: u32,
    kind: String,
    finish_handle: String,
    attempt_identity: String,
    quarantine_name: Vec<u8>,
    claimed_name: Vec<u8>,
    device: u64,
    inode: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

impl FileIdentity {
    fn from_file(file: &File) -> Result<Self> {
        let metadata = file.metadata().context("reading cleanup object identity")?;
        Ok(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }

    fn from_stat(metadata: &libc::stat) -> Self {
        Self {
            device: metadata.st_dev as u64,
            inode: metadata.st_ino as u64,
        }
    }
}

#[derive(Debug)]
pub(crate) struct QuarantineCleanup {
    marker_path: PathBuf,
    marker_identity: FileIdentity,
    marker: QuarantineMarker,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CleanupStep {
    BeforeClaim,
    BeforeEntry(OsString),
    BeforeRootRemoval,
}

pub(crate) fn prepare_quarantine(
    grove_root: &Path,
    quarantine_path: &Path,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<QuarantineCleanup> {
    validate_attempt_identity(attempt_identity)?;
    let quarantine_name = quarantine_path
        .file_name()
        .context("finish cleanup quarantine has no file name")?;
    let expected_name = OsString::from(format!("FINISHED-{finish_handle}-{attempt_identity}"));
    if quarantine_name != expected_name {
        bail!(
            "finish cleanup quarantine name {:?} does not match handle {finish_handle} and attempt {attempt_identity}",
            quarantine_name
        );
    }
    let quarantine_parent = quarantine_path
        .parent()
        .context("finish cleanup quarantine has no parent")?;
    fs::create_dir_all(quarantine_parent).with_context(|| {
        format!(
            "creating finish cleanup marker directory {}",
            quarantine_parent.display()
        )
    })?;

    let grove_directory = open_directory(grove_root).with_context(|| {
        format!(
            "opening task root for cleanup marking: {}",
            grove_root.display()
        )
    })?;
    let grove_identity = FileIdentity::from_file(&grove_directory)?;
    let marker = QuarantineMarker {
        version: 1,
        kind: "finish-quarantine".to_owned(),
        finish_handle: finish_handle.to_owned(),
        attempt_identity: attempt_identity.to_owned(),
        quarantine_name: quarantine_name.as_bytes().to_vec(),
        claimed_name: [REAPING_PREFIX, quarantine_name.as_bytes()].concat(),
        device: grove_identity.device,
        inode: grove_identity.inode,
    };
    let marker_path = quarantine_parent.join(marker_file_name(attempt_identity));

    if marker_path.exists() {
        let existing = QuarantineCleanup::from_marker(&marker_path)?;
        if existing.marker != marker {
            bail!(
                "finish cleanup marker collision at {}",
                marker_path.display()
            );
        }
        return Ok(existing);
    }

    let marker_body =
        serde_json::to_vec_pretty(&marker).context("serializing Grove cleanup marker")?;
    let mut temporary = NamedTempFile::new_in(quarantine_parent).with_context(|| {
        format!(
            "creating temporary Grove cleanup marker in {}",
            quarantine_parent.display()
        )
    })?;
    temporary
        .write_all(&marker_body)
        .context("writing temporary Grove cleanup marker")?;
    temporary
        .write_all(b"\n")
        .context("terminating temporary Grove cleanup marker")?;
    match temporary.persist_noclobber(&marker_path) {
        Ok(_) => QuarantineCleanup::from_marker(&marker_path),
        Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists => {
            let existing = QuarantineCleanup::from_marker(&marker_path)?;
            if existing.marker == marker {
                Ok(existing)
            } else {
                bail!(
                    "finish cleanup marker collision at {}",
                    marker_path.display()
                )
            }
        }
        Err(error) => Err(error.error)
            .with_context(|| format!("publishing Grove cleanup marker {}", marker_path.display())),
    }
}

#[cfg(test)]
pub(crate) fn marker_paths(control_directory: &Path) -> Result<Vec<PathBuf>> {
    let mut markers = fs::read_dir(control_directory)
        .with_context(|| {
            format!(
                "reading Grove cleanup marker directory {}",
                control_directory.display()
            )
        })?
        .filter_map(|entry| match entry {
            Ok(entry) if is_marker_name(&entry.file_name()) => Some(Ok(entry.path())),
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<io::Result<Vec<_>>>()?;
    markers.sort_by(|left, right| {
        left.as_os_str()
            .as_bytes()
            .cmp(right.as_os_str().as_bytes())
    });
    Ok(markers)
}

impl QuarantineCleanup {
    pub(crate) fn from_marker(marker_path: &Path) -> Result<Self> {
        let marker_file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(marker_path)
            .with_context(|| format!("opening Grove cleanup marker {}", marker_path.display()))?;
        let (marker_identity, marker) = read_marker(marker_path, marker_file)?;
        Ok(Self {
            marker_path: marker_path.to_path_buf(),
            marker_identity,
            marker,
        })
    }

    #[cfg(test)]
    pub(crate) fn marker_path(&self) -> &Path {
        &self.marker_path
    }

    pub(crate) fn handoff(&self, grove_root: &Path) -> Result<()> {
        let source_parent_path = grove_root
            .parent()
            .context("task root has no parent for cleanup handoff")?;
        let source_name = grove_root
            .file_name()
            .context("task root has no file name for cleanup handoff")?;
        let destination_parent_path = self
            .marker_path
            .parent()
            .context("Grove cleanup marker has no parent")?;
        let destination_name = OsString::from_vec(self.marker.quarantine_name.clone());
        let source_parent = open_directory(source_parent_path).with_context(|| {
            format!(
                "opening task-root parent for cleanup handoff {}",
                source_parent_path.display()
            )
        })?;
        let destination_parent = open_directory(destination_parent_path).with_context(|| {
            format!(
                "opening cleanup handoff destination {}",
                destination_parent_path.display()
            )
        })?;
        self.revalidate_marker(&destination_parent)?;
        let source = open_directory_at(&source_parent, source_name).with_context(|| {
            format!(
                "opening marked task root without following symlinks: {}",
                grove_root.display()
            )
        })?;
        self.validate_quarantine_identity(&source)?;
        let expected_identity = FileIdentity::from_file(&source)?;
        validate_entry_identity(&source_parent, source_name, expected_identity)
            .context("marked task-root identity changed before cleanup handoff")?;
        rename_at_noreplace(
            &source_parent,
            source_name,
            &destination_parent,
            &destination_name,
        )
        .with_context(|| {
            format!(
                "handing off marked task root {} as {:?}",
                grove_root.display(),
                destination_name
            )
        })?;
        let destination = open_directory_at(&destination_parent, &destination_name)
            .with_context(|| format!("opening handed-off task root {:?}", destination_name))?;
        if FileIdentity::from_file(&destination)? != expected_identity {
            bail!(
                "marked task-root identity changed during cleanup handoff; replacement left untouched"
            );
        }
        Ok(())
    }

    pub(crate) fn restore(&self, grove_root: &Path) -> Result<()> {
        let source_parent_path = self
            .marker_path
            .parent()
            .context("Grove cleanup marker has no parent")?;
        let source_name = OsString::from_vec(self.marker.quarantine_name.clone());
        let destination_parent_path = grove_root
            .parent()
            .context("task root has no parent for cleanup restore")?;
        let destination_name = grove_root
            .file_name()
            .context("task root has no file name for cleanup restore")?;
        let source_parent = open_directory(source_parent_path).with_context(|| {
            format!(
                "opening cleanup quarantine parent {}",
                source_parent_path.display()
            )
        })?;
        let destination_parent = open_directory(destination_parent_path).with_context(|| {
            format!(
                "opening task-root parent for cleanup restore {}",
                destination_parent_path.display()
            )
        })?;
        self.revalidate_marker(&source_parent)?;
        let source = open_directory_at(&source_parent, &source_name)
            .with_context(|| format!("opening marked cleanup quarantine {:?}", source_name))?;
        self.validate_quarantine_identity(&source)?;
        let expected_identity = FileIdentity::from_file(&source)?;
        validate_entry_identity(&source_parent, &source_name, expected_identity)
            .context("marked cleanup quarantine identity changed before restore")?;
        rename_at_noreplace(
            &source_parent,
            &source_name,
            &destination_parent,
            destination_name,
        )
        .with_context(|| {
            format!(
                "restoring marked task root {:?} to {}",
                source_name,
                grove_root.display()
            )
        })?;
        let destination = open_directory_at(&destination_parent, destination_name)
            .with_context(|| format!("opening restored task root {}", grove_root.display()))?;
        if FileIdentity::from_file(&destination)? != expected_identity {
            bail!(
                "marked task-root identity changed during cleanup restore; replacement left untouched"
            );
        }
        Ok(())
    }

    pub(crate) fn dispose(&self) -> Result<()> {
        self.dispose_with(cleanup_test_checkpoint)
    }

    pub(crate) fn dispose_with(
        &self,
        mut checkpoint: impl FnMut(CleanupStep) -> io::Result<()>,
    ) -> Result<()> {
        let parent_path = self
            .marker_path
            .parent()
            .context("Grove cleanup marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening Grove cleanup marker directory {}",
                parent_path.display()
            )
        })?;
        self.revalidate_marker(&parent)?;

        let quarantine_name = OsString::from_vec(self.marker.quarantine_name.clone());
        let claimed_name = OsString::from_vec(self.marker.claimed_name.clone());
        let quarantine_exists = entry_exists(&parent, &quarantine_name)?;
        let claimed_exists = entry_exists(&parent, &claimed_name)?;
        let (directory, active_name) = match (quarantine_exists, claimed_exists) {
            (true, true) => bail!(
                "ambiguous Grove cleanup owner: both {:?} and {:?} exist beside {}",
                quarantine_name,
                claimed_name,
                self.marker_path.display()
            ),
            (false, false) => {
                self.remove_marker(&parent)?;
                return Ok(());
            }
            (true, false) => {
                let original = open_directory_at(&parent, &quarantine_name).with_context(|| {
                    format!(
                        "opening marked finish quarantine {:?} without following symlinks",
                        quarantine_name
                    )
                })?;
                self.validate_quarantine_identity(&original)?;
                checkpoint(CleanupStep::BeforeClaim)
                    .context("cleanup checkpoint before quarantine claim")?;
                validate_entry_identity(
                    &parent,
                    &quarantine_name,
                    FileIdentity::from_file(&original)?,
                )
                .context(
                    "finish quarantine identity changed before claim; replacement left untouched",
                )?;
                rename_at_noreplace(&parent, &quarantine_name, &parent, &claimed_name)
                    .with_context(|| {
                        format!(
                            "claiming finish quarantine {:?} as {:?}",
                            quarantine_name, claimed_name
                        )
                    })?;
                let claimed = open_directory_at(&parent, &claimed_name).with_context(|| {
                    format!("opening claimed finish quarantine {:?}", claimed_name)
                })?;
                let original_identity = FileIdentity::from_file(&original)?;
                let claimed_identity = FileIdentity::from_file(&claimed)?;
                if original_identity != claimed_identity {
                    bail!(
                        "finish quarantine identity changed while claiming {:?}; replacement left untouched",
                        claimed_name
                    );
                }
                (claimed, claimed_name)
            }
            (false, true) => {
                let claimed = open_directory_at(&parent, &claimed_name).with_context(|| {
                    format!("opening claimed finish quarantine {:?}", claimed_name)
                })?;
                self.validate_quarantine_identity(&claimed)?;
                (claimed, claimed_name)
            }
        };

        remove_directory_contents(&directory, &mut checkpoint)?;
        checkpoint(CleanupStep::BeforeRootRemoval)
            .context("cleanup checkpoint before quarantine removal")?;
        validate_entry_identity(&parent, &active_name, FileIdentity::from_file(&directory)?)?;
        unlink_at(&parent, &active_name, libc::AT_REMOVEDIR)
            .with_context(|| format!("removing claimed finish quarantine {:?}", active_name))?;
        self.remove_marker(&parent)
    }

    fn revalidate_marker(&self, parent: &File) -> Result<()> {
        let marker_name = self
            .marker_path
            .file_name()
            .context("Grove cleanup marker has no file name")?;
        let marker_file = open_file_at(parent, marker_name).with_context(|| {
            format!(
                "opening Grove cleanup marker {} without following symlinks",
                self.marker_path.display()
            )
        })?;
        let (identity, marker) = read_marker(&self.marker_path, marker_file)?;
        if identity != self.marker_identity || marker != self.marker {
            bail!(
                "Grove cleanup marker changed at {}; replacement left untouched",
                self.marker_path.display()
            );
        }
        Ok(())
    }

    fn validate_quarantine_identity(&self, directory: &File) -> Result<()> {
        let observed = FileIdentity::from_file(directory)?;
        let expected = FileIdentity {
            device: self.marker.device,
            inode: self.marker.inode,
        };
        if observed != expected {
            bail!(
                "finish quarantine identity does not match marker {}: expected device/inode {}/{}, observed {}/{}; replacement left untouched",
                self.marker_path.display(),
                expected.device,
                expected.inode,
                observed.device,
                observed.inode
            );
        }
        Ok(())
    }

    fn remove_marker(&self, parent: &File) -> Result<()> {
        self.revalidate_marker(parent)?;
        let marker_name = self
            .marker_path
            .file_name()
            .context("Grove cleanup marker has no file name")?;
        unlink_at(parent, marker_name, 0).with_context(|| {
            format!(
                "removing completed Grove cleanup marker {}",
                self.marker_path.display()
            )
        })
    }
}

fn read_marker(
    marker_path: &Path,
    mut marker_file: File,
) -> Result<(FileIdentity, QuarantineMarker)> {
    let metadata = marker_file
        .metadata()
        .with_context(|| format!("reading Grove cleanup marker {}", marker_path.display()))?;
    if !metadata.file_type().is_file() {
        bail!(
            "Grove cleanup marker is not a regular file: {}",
            marker_path.display()
        );
    }
    let marker_identity = FileIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    };
    let mut body = Vec::new();
    marker_file
        .read_to_end(&mut body)
        .with_context(|| format!("reading Grove cleanup marker {}", marker_path.display()))?;
    let marker: QuarantineMarker = serde_json::from_slice(&body)
        .with_context(|| format!("parsing Grove cleanup marker {}", marker_path.display()))?;
    validate_marker(marker_path, &marker)?;
    Ok((marker_identity, marker))
}

/// Deterministic process-interruption seam for black-box cleanup tests. The
/// variable is deliberately test-prefixed and is not user configuration.
fn cleanup_test_checkpoint(step: CleanupStep) -> io::Result<()> {
    let name = match &step {
        CleanupStep::BeforeClaim => "before-claim",
        CleanupStep::BeforeEntry(_) => "before-entry",
        CleanupStep::BeforeRootRemoval => "before-root-removal",
    };
    if std::env::var("GROVE_TEST_FINISH_CLEANUP_FAIL_AT").as_deref() == Ok(name) {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("injected finish cleanup interruption at {name}"),
        ))
    } else {
        Ok(())
    }
}

fn validate_attempt_identity(attempt_identity: &str) -> Result<()> {
    if attempt_identity.len() != 32
        || !attempt_identity
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        bail!("finish cleanup attempt identity is not a 128-bit hexadecimal value")
    }
    Ok(())
}

fn validate_marker(marker_path: &Path, marker: &QuarantineMarker) -> Result<()> {
    if marker.version != 1 {
        bail!(
            "unsupported Grove cleanup marker version {} at {}",
            marker.version,
            marker_path.display()
        );
    }
    if marker.kind != "finish-quarantine" {
        bail!(
            "unsupported Grove cleanup marker kind {:?} at {}",
            marker.kind,
            marker_path.display()
        );
    }
    validate_attempt_identity(&marker.attempt_identity)?;
    let expected_marker_name = marker_file_name(&marker.attempt_identity);
    if marker_path.file_name() != Some(expected_marker_name.as_os_str()) {
        bail!(
            "Grove cleanup marker attempt does not match its path: {}",
            marker_path.display()
        );
    }
    let expected_quarantine = format!(
        "FINISHED-{}-{}",
        marker.finish_handle, marker.attempt_identity
    );
    if marker.quarantine_name != expected_quarantine.as_bytes() {
        bail!(
            "Grove cleanup marker quarantine name does not match its handle and attempt at {}",
            marker_path.display()
        );
    }
    if marker.claimed_name != [REAPING_PREFIX, marker.quarantine_name.as_slice()].concat() {
        bail!(
            "Grove cleanup marker claimed name is invalid at {}",
            marker_path.display()
        );
    }
    Ok(())
}

fn marker_file_name(attempt_identity: &str) -> OsString {
    OsString::from_vec(
        [
            CLEANUP_MARKER_PREFIX,
            attempt_identity.as_bytes(),
            CLEANUP_MARKER_SUFFIX,
        ]
        .concat(),
    )
}

#[cfg(test)]
fn is_marker_name(name: &OsStr) -> bool {
    let bytes = name.as_bytes();
    let Some(identity) = bytes
        .strip_prefix(CLEANUP_MARKER_PREFIX)
        .and_then(|value| value.strip_suffix(CLEANUP_MARKER_SUFFIX))
    else {
        return false;
    };
    identity.len() == 32 && identity.iter().all(u8::is_ascii_hexdigit)
}

#[cfg(test)]
mod tests;
