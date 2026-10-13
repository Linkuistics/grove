//! Grove's launch-scoped control area: identity, interpretation and removal.

use std::ffi::OsStr;
use std::fs::{self, DirBuilder, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub(crate) struct LaunchDirectory(PathBuf);

pub(crate) struct LaunchReading {
    pub(crate) teardown: bool,
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

    /// Teardown is interpreted only after Ravel's reap and epoch invalidation.
    /// The public result is held separately, never reopened through this path.
    pub(crate) fn read(&self) -> LaunchReading {
        LaunchReading {
            teardown: fs::symlink_metadata(self.0.join("teardown"))
                .is_ok_and(|metadata| metadata.file_type().is_file()),
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
    fn teardown_requires_a_regular_marker_and_ignores_legacy_reports() {
        let control = tempfile::tempdir().unwrap();
        let launch = LaunchDirectory::allocate(control.path()).unwrap();
        assert!(!launch.read().teardown);
        fs::write(launch.path().join("ending.json"), "grove-continue").unwrap();
        assert!(!launch.read().teardown);
        let other = control.path().join("other-teardown");
        fs::write(&other, "").unwrap();
        let marker = launch.path().join("teardown");
        std::os::unix::fs::symlink(other, &marker).unwrap();
        assert!(!launch.read().teardown);
        fs::remove_file(&marker).unwrap();
        fs::create_dir(&marker).unwrap();
        assert!(!launch.read().teardown);
        fs::remove_dir(&marker).unwrap();
        fs::write(marker, "").unwrap();
        assert!(launch.read().teardown);
    }
}
