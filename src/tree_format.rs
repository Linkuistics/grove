use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

const CURRENT_FILE_CONTENTS: &str = "session-kinds-v1\n";

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
    fs::write(&temporary_path, CURRENT_FILE_CONTENTS)
        .with_context(|| format!("writing {}", temporary_path.display()))?;
    fs::rename(&temporary_path, &path).with_context(|| format!("installing {}", path.display()))?;
    Ok(path)
}
