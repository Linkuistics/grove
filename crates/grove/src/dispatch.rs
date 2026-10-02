//! Where Grove finds `harness-dispatch`: beside its own executable.
//!
//! The two ship in one `bin/` directory, so the sibling of Grove's real path is
//! the matching release. PATH and the cwd take no part, and a symlink to Grove
//! does not move the lookup (`docs/specs/harness-selection-and-execution.md`,
//! *Grove integration*).
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use anyhow::{ensure, Context, Result};

pub(crate) fn locate() -> Result<PathBuf> {
    let grove = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .context("resolving grove's own executable path")?;
    let dispatch = grove.with_file_name("harness-dispatch");
    let executable = std::fs::metadata(&dispatch)
        .is_ok_and(|file| file.is_file() && file.permissions().mode() & 0o111 != 0);
    ensure!(
        executable,
        "grove launches through {}, which is missing or not executable. Install grove and \
         harness-dispatch together, from one release: they ship in the same bin directory.",
        dispatch.display()
    );
    Ok(dispatch)
}
