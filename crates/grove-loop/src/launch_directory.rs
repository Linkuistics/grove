//! Grove's launch-scoped control area: identity, interpretation and removal.

use std::ffi::OsStr;
use std::fs::{self, DirBuilder, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use std::io::Read;

pub(crate) struct LaunchDirectory(PathBuf);

pub(crate) struct LaunchReading {
    pub(crate) teardown: bool,
    pub(crate) exit_signal: bool,
}

pub(crate) fn is_launch_name(name: Option<&OsStr>) -> bool {
    name.and_then(OsStr::to_str)
        .and_then(|name| name.strip_prefix("launch-"))
        .is_some_and(|suffix| {
            suffix.len() == 32
                && suffix
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
}

impl LaunchDirectory {
    pub(crate) fn allocate(control: &Path) -> Result<Self> {
        use std::io::Read;
        for _ in 0..8 {
            let mut bytes = [0u8; 16];
            fs::File::open("/dev/urandom")?.read_exact(&mut bytes)?;
            let suffix: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            let path = control.join(format!("launch-{suffix}"));
            match DirBuilder::new().mode(0o700).create(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error).context("allocating launch directory"),
            }
        }
        bail!("could not allocate launch directory after 8 occupied draws")
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    /// Interpretation happens only after dispatch's reap and epoch invalidation.
    /// Missing, unreadable, malformed or unsupported reports never relaunch.
    pub(crate) fn read(&self) -> LaunchReading {
        let exit_signal = (|| {
            let mut bytes = Vec::new();
            let file = OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
                .open(self.0.join("ending.json"))
                .ok()?;
            if !file.metadata().ok()?.is_file() {
                return None;
            }
            file.take(1024 * 1024 + 1).read_to_end(&mut bytes).ok()?;
            if bytes.len() > 1024 * 1024 {
                return None;
            }
            let report: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
            Some(
                report["schemaVersion"] == 1
                    && report["source"] == "harness-dispatch"
                    && report["measurements"]["ending"]["state"] == "observed"
                    && report["measurements"]["ending"]["value"] == "exit_signal",
            )
        })()
        .unwrap_or(false);
        LaunchReading {
            teardown: fs::symlink_metadata(self.0.join("teardown"))
                .is_ok_and(|metadata| metadata.file_type().is_file()),
            exit_signal,
        }
    }

    pub(crate) fn discard(&self) -> Result<()> {
        fs::remove_dir_all(&self.0).context("removing interpreted launch directory")
    }

    /// The replacement owns the lease and has invalidated the predecessor's
    /// epoch. Contents carry no meaning across drivers and are never read.
    pub(crate) fn discard_abandoned(control: &Path) -> Result<()> {
        for entry in fs::read_dir(control)? {
            let entry = entry?;
            if is_launch_name(Some(&entry.file_name())) {
                let kind = entry.file_type()?;
                if kind.is_dir() {
                    fs::remove_dir_all(entry.path())?;
                } else if kind.is_symlink() {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        Ok(())
    }
}

pub(crate) fn record_teardown(
    worktree: &Path,
    launch_dir: Option<&Path>,
) -> Result<crate::verbs::Recorded> {
    let Some(launch_dir) = launch_dir else {
        return Ok(crate::verbs::Recorded::NoLoop);
    };
    // A dangling symlink still occupies .grove. Never turn a metadata failure
    // into proof that the finish-commit precondition has been met.
    match fs::symlink_metadata(worktree.join(".grove")) {
        Ok(_) => bail!(".grove/ still exists; run `grove-llm finish-commit <finish-handle>` before `record-teardown`"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
        Err(error) => return Err(error).context("checking grove teardown"),
    }
    let path = launch_dir.join("teardown");
    // Exclusive creation is atomic and never truncates an existing record.
    // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&path)
    {
        Ok(_) => {}
        Err(error)
            if error.kind() == std::io::ErrorKind::AlreadyExists
                && fs::symlink_metadata(&path)?.file_type().is_file() => {}
        Err(error) => return Err(error).context("recording launch teardown"),
    }
    Ok(crate::verbs::Recorded::Wrote(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_supported_observed_exit_signal_relaunches() {
        let control = tempfile::tempdir().unwrap();
        let launch = LaunchDirectory::allocate(control.path()).unwrap();
        let report = launch.path().join("ending.json");
        assert!(!launch.read().exit_signal);
        for (document, want) in [
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
                true,
            ),
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"harness_exit"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"cancelled"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":2,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":1,"source":"other","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"unknown","value":"exit_signal"}}}"#,
                false,
            ),
            ("{}", false),
            ("broken", false),
        ] {
            fs::write(&report, document).unwrap();
            assert_eq!(launch.read().exit_signal, want, "{document}");
        }
        fs::remove_file(&report).unwrap();
        fs::create_dir(&report).unwrap();
        assert!(!launch.read().exit_signal);
    }

    #[test]
    fn an_ending_symlink_does_not_relaunch() {
        let control = tempfile::tempdir().unwrap();
        let launch = LaunchDirectory::allocate(control.path()).unwrap();
        let other = control.path().join("other-ending");
        fs::write(&other, r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#).unwrap();
        std::os::unix::fs::symlink(other, launch.path().join("ending.json")).unwrap();
        assert!(!launch.read().exit_signal);
    }
}
