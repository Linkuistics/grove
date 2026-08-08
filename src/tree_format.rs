use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) const CURRENT_FILE_CONTENTS: &str = "session-kinds-v1\n";

pub fn require_current(grove_root: &Path) -> Result<()> {
    let path = grove_root.join("FORMAT");
    let contents = match fs::read(&path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "Grove tree format witness is missing at {}; this is a legacy tree \
                 and must be migrated before current tree operations can run",
                path.display()
            )
        }
        Err(error) => return Err(error).with_context(|| format!("reading {}", path.display())),
    };
    if contents != CURRENT_FILE_CONTENTS.as_bytes() {
        let found = String::from_utf8_lossy(&contents);
        bail!(
            "unsupported Grove tree format: found {:?} in {}; this binary requires {:?}",
            found,
            path.display(),
            CURRENT_FILE_CONTENTS
        );
    }
    Ok(())
}

/// Write the positive format witness after the rest of a freshly-created tree
/// is complete. Same-directory rename prevents readers from observing a torn
/// marker; recovery of a partial root scaffold belongs to the driver lifecycle.
pub fn write_current_last(grove_root: &Path) -> Result<PathBuf> {
    let path = grove_root.join("FORMAT");
    let temporary_path = grove_root.join(".FORMAT.tmp");
    match fs::symlink_metadata(&temporary_path) {
        Ok(metadata) => {
            if !metadata.file_type().is_file() {
                bail!(
                    "format witness temporary path is not a regular file: {}",
                    temporary_path.display()
                );
            }
            let found = fs::read(&temporary_path)
                .with_context(|| format!("reading {}", temporary_path.display()))?;
            if found != CURRENT_FILE_CONTENTS.as_bytes() {
                bail!(
                    "format witness temporary file has unexpected contents: {}",
                    temporary_path.display()
                );
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut options = fs::OpenOptions::new();
            options.write(true).create_new(true);
            let mut temporary = options
                .open(&temporary_path)
                .with_context(|| format!("creating {}", temporary_path.display()))?;
            std::io::Write::write_all(&mut temporary, CURRENT_FILE_CONTENTS.as_bytes())
                .with_context(|| format!("writing {}", temporary_path.display()))?;
        }
        Err(error) => {
            return Err(error).with_context(|| format!("checking {}", temporary_path.display()))
        }
    }
    fs::rename(&temporary_path, &path).with_context(|| format!("installing {}", path.display()))?;
    Ok(path)
}
