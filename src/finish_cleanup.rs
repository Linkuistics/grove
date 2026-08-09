use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

mod unix;
use unix::{
    create_new_file_at, entry_exists, open_directory, open_directory_at, open_file_at,
    remove_directory_contents, rename_at_noreplace, unlink_at, validate_entry_identity,
    validate_file_name,
};

const CLEANUP_MARKER_PREFIX: &[u8] = b"GROVE-FINISH-CLEANUP-";
const CLEANUP_MARKER_SUFFIX: &[u8] = b".json";
const REAPING_PREFIX: &[u8] = b"REAPING-";
const CLEANUP_TEST_PAUSE_TIMEOUT: Duration = Duration::from_secs(30);

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
    BeforeMarkerPublication,
    BeforeTemporaryMarkerPublication(OsString),
    AfterMarkerPublication,
    BeforeClaim,
    AfterClaim,
    BetweenOwnerProbes,
    BeforeEmptyOwnerMarkerRemoval,
    BeforeEntry(OsString),
    BeforeNonDirectoryUnlink(OsString),
    BeforeRootRemoval,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CleanupOutcome {
    Disposed,
    NothingToDispose,
}

pub(crate) fn prepare_quarantine(
    grove_root: &Path,
    quarantine_path: &Path,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<QuarantineCleanup> {
    prepare_quarantine_with(
        grove_root,
        quarantine_path,
        finish_handle,
        attempt_identity,
        cleanup_test_checkpoint,
    )
}

fn prepare_quarantine_with(
    grove_root: &Path,
    quarantine_path: &Path,
    finish_handle: &str,
    attempt_identity: &str,
    mut checkpoint: impl FnMut(CleanupStep) -> io::Result<()>,
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
    let quarantine_parent_directory = open_directory(quarantine_parent).with_context(|| {
        format!(
            "opening finish cleanup marker directory {}",
            quarantine_parent.display()
        )
    })?;
    validate_file_name(&quarantine_parent_directory, quarantine_name).with_context(|| {
        format!(
            "finish quarantine name {:?} cannot be created in {}",
            quarantine_name,
            quarantine_parent.display()
        )
    })?;
    let claimed_name = [REAPING_PREFIX, quarantine_name.as_bytes()].concat();
    validate_file_name(
        &quarantine_parent_directory,
        OsStr::from_bytes(&claimed_name),
    )
    .with_context(|| {
        format!(
            "claimed finish quarantine name {:?} cannot be created in {}",
            OsStr::from_bytes(&claimed_name),
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
        claimed_name,
        device: grove_identity.device,
        inode: grove_identity.inode,
    };
    let marker_path = quarantine_parent.join(marker_file_name(attempt_identity));
    let marker_name = marker_path
        .file_name()
        .context("Grove cleanup marker has no file name")?;
    checkpoint(CleanupStep::BeforeMarkerPublication)
        .context("cleanup checkpoint before marker publication")?;

    if entry_exists(&quarantine_parent_directory, marker_name)? {
        let existing =
            QuarantineCleanup::from_marker_at(&quarantine_parent_directory, &marker_path)?;
        if existing.marker != marker {
            bail!(
                "finish cleanup marker collision at {}",
                marker_path.display()
            );
        }
        return Ok(existing);
    }

    let mut marker_body =
        serde_json::to_vec_pretty(&marker).context("serializing Grove cleanup marker")?;
    marker_body.push(b'\n');
    let (mut temporary, temporary_name, temporary_identity) =
        create_temporary_marker(&quarantine_parent_directory).with_context(|| {
            format!(
                "creating temporary Grove cleanup marker in {}",
                quarantine_parent.display()
            )
        })?;
    if let Err(error) = temporary.write_all(&marker_body) {
        let cleanup = remove_temporary_marker(
            &quarantine_parent_directory,
            &temporary_name,
            temporary_identity,
        );
        let primary = anyhow::Error::new(error).context("writing temporary Grove cleanup marker");
        return Err(attach_cleanup_failure(
            primary,
            cleanup,
            "removing failed temporary Grove cleanup marker",
        ));
    }
    checkpoint(CleanupStep::BeforeTemporaryMarkerPublication(
        temporary_name.clone(),
    ))
    .context("cleanup checkpoint before temporary marker publication")?;
    match rename_at_noreplace(
        &quarantine_parent_directory,
        &temporary_name,
        &quarantine_parent_directory,
        marker_name,
    ) {
        Ok(()) => {
            checkpoint(CleanupStep::AfterMarkerPublication)
                .context("cleanup checkpoint after marker publication")?;
            let existing =
                QuarantineCleanup::from_marker_at(&quarantine_parent_directory, &marker_path)?;
            if existing.marker == marker {
                Ok(existing)
            } else {
                bail!(
                    "finish cleanup marker collision at {}",
                    marker_path.display()
                )
            }
        }
        Err(error) => {
            let error_kind = error.kind();
            let cleanup = remove_temporary_marker(
                &quarantine_parent_directory,
                &temporary_name,
                temporary_identity,
            );
            if let Err(cleanup_error) = cleanup {
                let primary = anyhow::Error::new(error).context(format!(
                    "publishing Grove cleanup marker {}",
                    marker_path.display()
                ));
                return Err(attach_cleanup_failure(
                    primary,
                    Err(cleanup_error),
                    "removing unpublished temporary Grove cleanup marker",
                ));
            }
            if error_kind != io::ErrorKind::AlreadyExists {
                return Err(error).with_context(|| {
                    format!("publishing Grove cleanup marker {}", marker_path.display())
                });
            }
            let existing =
                QuarantineCleanup::from_marker_at(&quarantine_parent_directory, &marker_path)?;
            if existing.marker == marker {
                Ok(existing)
            } else {
                bail!(
                    "finish cleanup marker collision at {}",
                    marker_path.display()
                )
            }
        }
    }
}

fn create_temporary_marker(parent: &File) -> Result<(File, OsString, FileIdentity)> {
    for _ in 0..crate::driver_lease::RANDOM_DRAW_RETRY_LIMIT {
        let suffix =
            crate::driver_lease::fresh_nonce().context("drawing temporary cleanup-marker nonce")?;
        let name = OsString::from(format!(".grove-cleanup-{suffix}.tmp"));
        validate_file_name(parent, &name)
            .context("temporary Grove cleanup marker name is not valid for its directory")?;
        match create_new_file_at(parent, &name) {
            Ok(file) => {
                let identity = FileIdentity::from_file(&file)?;
                return Ok((file, name, identity));
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error).context("creating temporary Grove cleanup marker"),
        }
    }
    bail!(
        "could not allocate a unique temporary Grove cleanup marker after {} attempts",
        crate::driver_lease::RANDOM_DRAW_RETRY_LIMIT
    )
}

fn remove_temporary_marker(parent: &File, name: &OsStr, identity: FileIdentity) -> Result<()> {
    validate_entry_identity(parent, name, identity)
        .context("temporary Grove cleanup marker identity changed; replacement left untouched")?;
    unlink_at(parent, name, 0).context("removing temporary Grove cleanup marker")
}

fn attach_cleanup_failure(
    primary: anyhow::Error,
    cleanup: Result<()>,
    cleanup_context: &str,
) -> anyhow::Error {
    match cleanup {
        Ok(()) => primary,
        Err(cleanup_error) => primary.context(format!("{cleanup_context}: {cleanup_error:#}")),
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
    #[cfg(test)]
    pub(crate) fn from_marker(marker_path: &Path) -> Result<Self> {
        let parent_path = marker_path
            .parent()
            .context("Grove cleanup marker has no parent")?;
        let parent = open_directory(parent_path).with_context(|| {
            format!(
                "opening Grove cleanup marker directory {}",
                parent_path.display()
            )
        })?;
        Self::from_marker_at(&parent, marker_path)
    }

    fn from_marker_at(parent: &File, marker_path: &Path) -> Result<Self> {
        let marker_name = marker_path
            .file_name()
            .context("Grove cleanup marker has no file name")?;
        let marker_file = open_file_at(parent, marker_name).with_context(|| {
            format!(
                "opening Grove cleanup marker {} without following symlinks",
                marker_path.display()
            )
        })?;
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

    pub(crate) fn dispose(&self) -> Result<CleanupOutcome> {
        self.dispose_with(cleanup_test_checkpoint)
    }

    pub(crate) fn dispose_with(
        &self,
        mut checkpoint: impl FnMut(CleanupStep) -> io::Result<()>,
    ) -> Result<CleanupOutcome> {
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
        checkpoint(CleanupStep::BetweenOwnerProbes)
            .context("cleanup checkpoint between owner probes")?;
        let claimed_exists = entry_exists(&parent, &claimed_name)?;
        let first_observation = (quarantine_exists, claimed_exists);
        let stable_observation = (
            entry_exists(&parent, &quarantine_name)?,
            entry_exists(&parent, &claimed_name)?,
        );
        if first_observation != stable_observation {
            bail!(
                "Grove cleanup owner changed during observation beside {}: observed owners {:?}, then {:?} for {:?} and {:?}; neither candidate nor marker was removed",
                self.marker_path.display(),
                first_observation,
                stable_observation,
                quarantine_name,
                claimed_name
            );
        }
        let (directory, active_name) = match stable_observation {
            (true, true) => bail!(
                "ambiguous Grove cleanup owner: both {:?} and {:?} exist beside {}",
                quarantine_name,
                claimed_name,
                self.marker_path.display()
            ),
            (false, false) => {
                self.remove_marker_with(&parent, || {
                    checkpoint(CleanupStep::BeforeEmptyOwnerMarkerRemoval)
                        .context("cleanup checkpoint before empty-owner marker removal")?;
                    let final_observation = (
                        entry_exists(&parent, &quarantine_name)?,
                        entry_exists(&parent, &claimed_name)?,
                    );
                    if final_observation != (false, false) {
                        bail!(
                            "Grove cleanup owner appeared before marker removal beside {}: observed {:?}={}, {:?}={}; neither candidate nor marker was removed",
                            self.marker_path.display(),
                            quarantine_name,
                            final_observation.0,
                            claimed_name,
                            final_observation.1
                        );
                    }
                    Ok(())
                })?;
                return Ok(CleanupOutcome::NothingToDispose);
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
                let claimed_path = parent_path.join(&claimed_name);
                let claimed = (|| -> Result<File> {
                    checkpoint(CleanupStep::AfterClaim)
                        .context("cleanup checkpoint after quarantine claim")?;
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
                    Ok(claimed)
                })()
                .with_context(|| self.pending_cleanup_context(&claimed_path))?;
                (claimed, claimed_name)
            }
            (false, true) => {
                let claimed_path = parent_path.join(&claimed_name);
                let claimed = (|| -> Result<File> {
                    let claimed = open_directory_at(&parent, &claimed_name).with_context(|| {
                        format!("opening claimed finish quarantine {:?}", claimed_name)
                    })?;
                    self.validate_quarantine_identity(&claimed)?;
                    Ok(claimed)
                })()
                .with_context(|| self.pending_cleanup_context(&claimed_path))?;
                (claimed, claimed_name)
            }
        };

        let active_path = parent_path.join(&active_name);
        let disposal = (|| -> Result<()> {
            remove_directory_contents(&directory, &mut checkpoint)?;
            checkpoint(CleanupStep::BeforeRootRemoval)
                .context("cleanup checkpoint before quarantine removal")?;
            validate_entry_identity(&parent, &active_name, FileIdentity::from_file(&directory)?)?;
            unlink_at(&parent, &active_name, libc::AT_REMOVEDIR)
                .with_context(|| format!("removing claimed finish quarantine {:?}", active_name))?;
            Ok(())
        })();
        if let Err(error) = disposal {
            return Err(error).with_context(|| self.pending_cleanup_context(&active_path));
        }
        self.remove_marker(&parent)?;
        Ok(CleanupOutcome::Disposed)
    }

    fn pending_cleanup_context(&self, active_path: &Path) -> String {
        format!(
            "completed Grove cleanup remains at {}; retry with marker {}",
            active_path.display(),
            self.marker_path.display()
        )
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
        self.remove_marker_with(parent, || Ok(()))
    }

    fn remove_marker_with(
        &self,
        parent: &File,
        before_unlink: impl FnOnce() -> Result<()>,
    ) -> Result<()> {
        self.revalidate_marker(parent)?;
        before_unlink()?;
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
    cleanup_test_checkpoint_with_timeout(step, CLEANUP_TEST_PAUSE_TIMEOUT)
}

fn cleanup_test_checkpoint_with_timeout(
    step: CleanupStep,
    pause_timeout: Duration,
) -> io::Result<()> {
    let name = match &step {
        CleanupStep::BeforeMarkerPublication => "before-marker-publication",
        CleanupStep::BeforeTemporaryMarkerPublication(_) => "before-temporary-marker-publication",
        CleanupStep::AfterMarkerPublication => "after-marker-publication",
        CleanupStep::BeforeClaim => "before-claim",
        CleanupStep::AfterClaim => "after-claim",
        CleanupStep::BetweenOwnerProbes => "between-owner-probes",
        CleanupStep::BeforeEmptyOwnerMarkerRemoval => "before-empty-owner-marker-removal",
        CleanupStep::BeforeEntry(_) => "before-entry",
        CleanupStep::BeforeNonDirectoryUnlink(_) => "before-non-directory-unlink",
        CleanupStep::BeforeRootRemoval => "before-root-removal",
    };
    if std::env::var("GROVE_TEST_FINISH_CLEANUP_FAIL_AT").as_deref() == Ok(name) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("injected finish cleanup interruption at {name}"),
        ));
    }
    if std::env::var("GROVE_TEST_FINISH_CLEANUP_PAUSE_AT").as_deref() != Ok(name) {
        return Ok(());
    }
    let barrier = std::env::var_os("GROVE_TEST_FINISH_CLEANUP_BARRIER").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "GROVE_TEST_FINISH_CLEANUP_PAUSE_AT needs GROVE_TEST_FINISH_CLEANUP_BARRIER",
        )
    })?;
    let barrier = PathBuf::from(barrier);
    let detail = match &step {
        CleanupStep::BeforeTemporaryMarkerPublication(entry)
        | CleanupStep::BeforeEntry(entry)
        | CleanupStep::BeforeNonDirectoryUnlink(entry) => entry.as_bytes(),
        _ => name.as_bytes(),
    };
    fs::write(&barrier, detail)?;
    wait_for_cleanup_barrier_release(&barrier, pause_timeout)
}

fn wait_for_cleanup_barrier_release(barrier: &Path, timeout: Duration) -> io::Result<()> {
    let deadline = Instant::now() + timeout;
    while barrier.try_exists()? {
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!(
                    "timed out waiting for finish cleanup barrier release at {}",
                    barrier.display()
                ),
            ));
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
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
    validate_marker_component(marker_path, "quarantine", &marker.quarantine_name)?;
    validate_marker_component(marker_path, "claimed", &marker.claimed_name)?;
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

fn validate_marker_component(marker_path: &Path, field: &str, name: &[u8]) -> Result<()> {
    let path = Path::new(OsStr::from_bytes(name));
    let mut components = path.components();
    let is_single_normal_component = matches!(
        (components.next(), components.next()),
        (Some(std::path::Component::Normal(component)), None)
            if component.as_bytes() == name && !name.contains(&0)
    );
    if !is_single_normal_component {
        bail!(
            "Grove cleanup marker {field} name is not a single non-empty path component at {}",
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
