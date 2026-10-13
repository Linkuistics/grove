//! Explicit-source installed Ravel pairs for Grove's consumer fixtures.
//!
//! This is test-only support. It copies the installed public front/worker
//! layout; it neither builds Ravel nor changes the source installation.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct InstalledRavel {
    executable: PathBuf,
    bin: PathBuf,
}

impl InstalledRavel {
    /// Copy both files from the canonical installed prefix to a fresh fixture.
    /// The caller owns the destination's temporary-directory lifetime.
    pub fn copy_from(source: &Path, prefix: &Path) -> Self {
        let source = fs::canonicalize(source).expect("explicit installed Ravel front");
        let source_prefix = source
            .parent()
            .and_then(Path::parent)
            .expect("installed Ravel front sits in prefix/bin");
        let bin = prefix.join("bin");
        let worker_dir = prefix.join("libexec/ravel");
        fs::create_dir_all(&bin).expect("fixture Ravel bin directory");
        fs::create_dir_all(&worker_dir).expect("fixture Ravel worker directory");
        let executable = bin.join("ravel");
        // Refuse an occupied destination; no fixture may overwrite another's pair.
        for (from, to) in [
            (source.clone(), executable.clone()),
            (
                source_prefix.join("libexec/ravel/ravel-policy"),
                worker_dir.join("ravel-policy"),
            ),
        ] {
            let mut input = fs::File::open(&from).unwrap_or_else(|error| {
                panic!("opening Ravel fixture source {}: {error}", from.display())
            });
            let mut output = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&to)
                .unwrap_or_else(|error| {
                    panic!("creating Ravel fixture file {}: {error}", to.display())
                });
            std::io::copy(&mut input, &mut output).expect("copying installed Ravel fixture file");
            output
                .set_permissions(
                    input
                        .metadata()
                        .expect("Ravel source metadata")
                        .permissions(),
                )
                .expect("preserving Ravel executable permissions");
        }
        Self {
            executable: fs::canonicalize(executable).expect("copied Ravel executable"),
            bin: fs::canonicalize(bin).expect("copied Ravel bin directory"),
        }
    }

    pub fn executable(&self) -> &Path {
        &self.executable
    }

    pub fn bin(&self) -> &Path {
        &self.bin
    }

    /// A process isolated from inherited session authority and host tool lookup.
    /// Tests may explicitly add their own HOME, policy, arguments and cwd.
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.executable);
        self.isolate(&mut command);
        command
    }

    /// Apply the same isolation to a caller binary or an entry-observation probe.
    pub fn isolate(&self, command: &mut Command) {
        command.env("PATH", &self.bin);
        for name in super::grove_env_names() {
            command.env_remove(name);
        }
    }
}
