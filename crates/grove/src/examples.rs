//! Install the authoritative example bytes without reading active launch policy.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

/// Fixed, inactive destinations; include the repository fixtures at compile time.
const EXAMPLES: &[(&str, &[u8])] = &[
    (
        "config.modular.example.kdl",
        include_bytes!("../../../docs/examples/modular-configuration/config.modular.example.kdl"),
    ),
    (
        "grove.codex-led.example.kdl",
        include_bytes!("../../../docs/examples/modular-configuration/grove.codex-led.example.kdl"),
    ),
    (
        "grove.claude-led.example.kdl",
        include_bytes!("../../../docs/examples/modular-configuration/grove.claude-led.example.kdl"),
    ),
    (
        "grove.high-effort.example.kdl",
        include_bytes!(
            "../../../docs/examples/modular-configuration/grove.high-effort.example.kdl"
        ),
    ),
    (
        "grove.local-override.example.kdl",
        include_bytes!(
            "../../../docs/examples/modular-configuration/grove.local-override.example.kdl"
        ),
    ),
    (
        "grove.legacy-override.example.kdl",
        include_bytes!(
            "../../../docs/examples/modular-configuration/grove.legacy-override.example.kdl"
        ),
    ),
    (
        "CONFIGURATION.examples.md",
        include_bytes!("../../../docs/examples/modular-configuration/README.md"),
    ),
];

/// Filesystem effects are separate so acceptance can interleave a competing
/// creation or fail a write after bytes have reached a real temporary file.
trait Storage {
    type Writer: Write;
    fn existing(&self, path: &Path) -> io::Result<Option<Vec<u8>>>;
    fn prepare(&self, directory: &Path) -> io::Result<()>;
    fn create(&self, path: &Path) -> io::Result<Self::Writer>;
}

struct Disk;

impl Storage for Disk {
    type Writer = File;

    fn existing(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_file() => fs::read(path).map(Some),
            Ok(_) => Err(io::Error::other(
                "not a regular file (symlinks are refused)",
            )),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn prepare(&self, directory: &Path) -> io::Result<()> {
        fs::create_dir_all(directory)
    }

    fn create(&self, path: &Path) -> io::Result<File> {
        // Exclusive creation also refuses dangling symlinks and occupants that
        // arrived after preflight. https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
        OpenOptions::new().write(true).create_new(true).open(path)
    }
}

#[derive(Default)]
struct Report {
    created: Vec<PathBuf>,
    unchanged: Vec<PathBuf>,
    failures: Vec<String>,
}

impl Report {
    fn render(&self) -> String {
        let mut lines = Vec::new();
        for path in &self.created {
            lines.push(format!("Created: {}", path.display()));
        }
        for path in &self.unchanged {
            lines.push(format!("Unchanged: {}", path.display()));
        }
        lines.extend(self.failures.iter().cloned());
        lines.join("\n")
    }
}

/// Preflight the whole set before creating even the destination directory.
/// Successful opens are recorded before writes: a short write leaves a new,
/// possibly partial file, which must be reported rather than silently removed.
fn install(directory: &Path, storage: &impl Storage) -> Report {
    let mut report = Report::default();
    let mut missing = Vec::new();
    for &(name, bytes) in EXAMPLES {
        let path = directory.join(name);
        match storage.existing(&path) {
            Ok(Some(existing)) if existing == bytes => report.unchanged.push(path),
            Ok(Some(_)) => report
                .failures
                .push(format!("Conflict: {}: different contents", path.display())),
            Ok(None) => missing.push((path, bytes)),
            Err(error) => report
                .failures
                .push(format!("Conflict: {}: {error}", path.display())),
        }
    }
    if !report.failures.is_empty() {
        report.failures.push("No files created. Move conflicting entries aside or choose to keep them; no overwrite option is provided.".into());
        return report;
    }
    if missing.is_empty() {
        return report;
    }
    if let Err(error) = storage.prepare(directory) {
        report.failures.push(format!(
            "Cannot create directory {}: {error}. No files created.",
            directory.display()
        ));
        return report;
    }
    for (path, bytes) in missing {
        let result = match storage.create(&path) {
            Ok(mut file) => {
                report.created.push(path.clone());
                file.write_all(bytes).and_then(|()| file.flush())
            }
            Err(error) => {
                report.failures.push(format!("Cannot exclusively create {}: {error}. Existing entries were not replaced; inspect this path before retrying.", path.display()));
                break;
            }
        };
        if let Err(error) = result {
            report.failures.push(format!("Cannot finish writing {}: {error}. This newly created file may be partial; inspect it before retrying.", path.display()));
            break;
        }
    }
    if !report.failures.is_empty() {
        report.failures.push(
            "Installation incomplete. Created files listed above remain; no files were removed."
                .into(),
        );
    }
    report
}

/// Resolve the fixed personal directory independently of workspace/epoch state.
pub fn run() -> anyhow::Result<()> {
    let home = std::env::var_os("HOME").filter(|value| !value.is_empty())
        .context("HOME is missing or empty; set it to your home directory before running grove config examples")?;
    let report = install(&PathBuf::from(home).join(".config/grove"), &Disk);
    let output = report.render();
    if !report.failures.is_empty() {
        bail!("{output}");
    }
    println!("{output}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    enum Fault {
        Race,
        Write,
        Prepare,
        Unreadable,
    }

    struct FaultyDisk {
        fault: Fault,
        creates: Cell<usize>,
    }

    struct ShortWriter {
        file: File,
        fail: bool,
        wrote: bool,
    }

    impl Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail && self.wrote {
                return Err(io::Error::other("injected disk full"));
            }
            self.wrote = true;
            self.file.write(if self.fail { &bytes[..3] } else { bytes })
        }

        fn flush(&mut self) -> io::Result<()> {
            self.file.flush()
        }
    }

    impl Storage for FaultyDisk {
        type Writer = ShortWriter;

        fn existing(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
            if matches!(self.fault, Fault::Unreadable) && path.ends_with(EXAMPLES[2].0) {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected unreadable entry",
                ));
            }
            Disk.existing(path)
        }

        fn prepare(&self, directory: &Path) -> io::Result<()> {
            if matches!(self.fault, Fault::Prepare) {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected directory refusal",
                ));
            }
            Disk.prepare(directory)
        }

        fn create(&self, path: &Path) -> io::Result<ShortWriter> {
            let index = self.creates.get();
            self.creates.set(index + 1);
            if matches!(self.fault, Fault::Race) && index == 1 {
                fs::write(path, "competing writer")?;
            }
            Ok(ShortWriter {
                file: Disk.create(path)?,
                fail: matches!(self.fault, Fault::Write) && index == 1,
                wrote: false,
            })
        }
    }

    #[test]
    fn racing_occupant_survives_and_prior_creation_is_reported() {
        let directory = tempfile::tempdir().unwrap();
        let storage = FaultyDisk {
            fault: Fault::Race,
            creates: Cell::new(0),
        };
        let report = install(directory.path(), &storage);
        assert_eq!(report.created, [directory.path().join(EXAMPLES[0].0)]);
        assert!(!report.failures.is_empty());
        assert!(report.render().contains("Cannot exclusively create"));
        assert!(report.render().contains(EXAMPLES[1].0));
        assert_eq!(
            fs::read(directory.path().join(EXAMPLES[0].0)).unwrap(),
            EXAMPLES[0].1
        );
        assert_eq!(
            fs::read_to_string(directory.path().join(EXAMPLES[1].0)).unwrap(),
            "competing writer"
        );
        assert!(!directory.path().join(EXAMPLES[2].0).exists());
    }

    #[test]
    fn late_write_failure_reports_partial_file_and_preserves_matching_file() {
        let directory = tempfile::tempdir().unwrap();
        let matching = directory.path().join(EXAMPLES[6].0);
        fs::write(&matching, EXAMPLES[6].1).unwrap();
        let modified = fs::metadata(&matching).unwrap().modified().unwrap();
        let storage = FaultyDisk {
            fault: Fault::Write,
            creates: Cell::new(0),
        };
        let report = install(directory.path(), &storage);
        assert_eq!(report.created.len(), 2);
        assert_eq!(report.unchanged, std::slice::from_ref(&matching));
        assert!(report.render().contains("may be partial"));
        assert_eq!(
            fs::read(directory.path().join(EXAMPLES[0].0)).unwrap(),
            EXAMPLES[0].1
        );
        assert_eq!(
            fs::read(directory.path().join(EXAMPLES[1].0)).unwrap(),
            &EXAMPLES[1].1[..3]
        );
        assert!(!directory.path().join(EXAMPLES[2].0).exists());
        assert_eq!(
            fs::metadata(matching).unwrap().modified().unwrap(),
            modified
        );
        let retry = install(directory.path(), &Disk);
        assert!(retry.created.is_empty());
        assert!(retry.render().contains("different contents"));
    }

    #[test]
    fn unreadable_preflight_and_directory_failure_create_nothing() {
        for fault in [Fault::Unreadable, Fault::Prepare] {
            let directory = tempfile::tempdir().unwrap();
            let destination = directory.path().join("new");
            let storage = FaultyDisk {
                fault,
                creates: Cell::new(0),
            };
            let report = install(&destination, &storage);
            assert!(report.created.is_empty());
            assert!(!report.failures.is_empty());
            assert_eq!(storage.creates.get(), 0);
            assert!(!destination.exists());
        }
    }
}
