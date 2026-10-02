//! Which policy entry may run, and on whose authority
//! (`docs/specs/harness-selection-and-execution.md`, *Policy authority and
//! runtime discovery*).
//!
//! There are exactly two authorities. The owner's personal default,
//! `~/.config/harness-dispatch/policy.ts`, and an explicit `--config`, resolved
//! against the caller's original cwd. Nothing else selects an entry: no search
//! of the cwd or its parents, no environment variable, no repository override.
//! Cloning or entering a repository therefore runs none of its code; naming a
//! file there with `--config` is the caller's explicit opt-in, and inspection
//! reports it as such.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

use crate::refusal::{Refusal, Stage, EXIT_REFUSED};

/// The personal default, relative to HOME. `XDG_CONFIG_HOME` is deliberately
/// not consulted: it would be an environment-selected entry.
pub const PERSONAL_DEFAULT: &str = ".config/harness-dispatch/policy.ts";

#[derive(Debug)]
pub enum Authority {
    Personal,
    /// The `--config` argument exactly as the caller gave it.
    Explicit {
        argument: OsString,
    },
}

#[derive(Debug)]
pub struct PolicyEntry {
    /// The canonical path, exactly the string the worker imports; symlinks are
    /// resolved, so this is the file whose directory relative imports start
    /// from. [`import_specifier`] has proven that it names this file.
    pub path: String,
    pub authority: Authority,
    /// SHA-256 of the entry's bytes, read just before the worker starts, in
    /// lowercase hex. It describes this file only, not what it imports.
    pub sha256: String,
}

impl PolicyEntry {
    pub fn display(&self) -> String {
        self.path.clone()
    }
}

/// Resolve the selected entry and prove it is a readable regular file.
pub fn resolve(
    config: Option<&Path>,
    cwd: &Path,
    home: Option<&OsStr>,
) -> Result<PolicyEntry, Refusal> {
    let (candidate, authority) = match config {
        Some(config) => (
            cwd.join(config),
            Authority::Explicit {
                argument: config.as_os_str().to_owned(),
            },
        ),
        None => (personal_default(home)?, Authority::Personal),
    };
    let shown = candidate.to_string_lossy().into_owned();
    let remedy = match &authority {
        Authority::Personal => format!(
            "run `harness-dispatch init` to install the sample policy there, write {shown} \
             yourself exporting a `policy`, or name another entry with --config PATH"
        ),
        Authority::Explicit { .. } => format!(
            "check the --config path; a relative path resolves against the current directory {}",
            cwd.display()
        ),
    };

    let metadata = fs::metadata(&candidate).map_err(|error| {
        let (code, what) = if error.kind() == ErrorKind::NotFound {
            ("policy_missing", "does not exist".to_owned())
        } else {
            ("policy_unreadable", format!("cannot be read: {error}"))
        };
        Refusal::new(
            code,
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} {what}"),
            remedy.clone(),
        )
        .source(shown.clone())
    })?;
    if !metadata.is_file() {
        return Err(Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} is not a regular file"),
            remedy,
        )
        .source(shown));
    }
    let path = fs::canonicalize(&candidate).map_err(|error| {
        Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} cannot be resolved: {error}"),
            remedy.clone(),
        )
        .source(shown.clone())
    })?;
    let path = import_specifier(&path)
        .map_err(|why| {
            Refusal::new(
                "policy_unreadable",
                Stage::Authority,
                EXIT_REFUSED,
                format!(
                    "the selected policy entry {shown} resolves to {}, which {why}, so the policy \
                     worker cannot import that exact file",
                    path.display()
                ),
                "rename the entry, or the directories above it, so that its resolved path is \
                 UTF-8 with no `?`, or name another entry with --config PATH",
            )
            .source(shown.clone())
        })?
        .to_owned();
    // Reading proves read permission and gives the digest the run records.
    // The worker imports the file itself.
    let bytes = fs::read(&path).map_err(|error| {
        Refusal::new(
            "policy_unreadable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the selected policy entry {shown} cannot be read: {error}"),
            remedy.clone(),
        )
        .source(shown.clone())
    })?;
    let sha256 = Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Ok(PolicyEntry {
        path,
        authority,
        sha256,
    })
}

/// The canonical `path` as the exact string the worker imports, or why no
/// string names it. The worker's runtime takes a string, and reads a `?` in it
/// as the start of a query even through a `file:` URL: under Bun 1.4.2,
/// `policy.ts?x` loads `policy.ts`, and `d?q/policy.ts` loads `d.ts`. A path
/// that is not UTF-8 has no exact string, only a lossy one naming another
/// file. Either would evaluate code other than the file admitted and hashed
/// here, so both refuse before a worker starts. (`#`, `%`, spaces and newlines
/// import exactly; a `\` fails to load rather than loading another file.)
fn import_specifier(path: &Path) -> Result<&str, &'static str> {
    let path = path.to_str().ok_or("is not valid UTF-8")?;
    if path.contains('?') {
        return Err("contains `?`, read by the worker's runtime as a query");
    }
    Ok(path)
}

/// The personal default's path under `home`, which `init` writes and
/// [`resolve`] reads.
pub fn personal_default(home: Option<&OsStr>) -> Result<PathBuf, Refusal> {
    let remedy = "set HOME to your absolute home directory, or name an entry with --config PATH";
    let home = home.filter(|home| !home.is_empty()).ok_or_else(|| {
        Refusal::new(
            "home_unset",
            Stage::Authority,
            EXIT_REFUSED,
            format!("HOME is not set, so the personal policy ~/{PERSONAL_DEFAULT} has no location"),
            remedy,
        )
        .input("HOME")
    })?;
    let home = Path::new(home);
    // A relative HOME would resolve against the cwd, which is exactly the
    // repository-selected entry this module exists to rule out.
    if !home.is_absolute() {
        return Err(Refusal::new(
            "home_unset",
            Stage::Authority,
            EXIT_REFUSED,
            format!(
                "HOME is the relative path {}, so the personal policy would depend on the current directory",
                home.display()
            ),
            remedy,
        )
        .input("HOME"));
    }
    Ok(home.join(PERSONAL_DEFAULT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::ffi::OsStrExt as _;

    #[test]
    fn only_a_path_the_worker_imports_exactly_is_a_specifier() {
        for exact in ["/p/policy.ts", "/p/a b#c%41\n.ts", "/p/é/policy.ts"] {
            assert_eq!(import_specifier(Path::new(exact)), Ok(exact));
        }
        for queried in ["/p/policy.ts?x", "/p/d?q/policy.ts"] {
            let why = import_specifier(Path::new(queried)).unwrap_err();
            assert!(why.contains('?'), "{queried}: {why}");
        }
        // The lossy form of this path is a different, UTF-8 file name.
        let native = Path::new(OsStr::from_bytes(b"/p/policy-\xff.ts"));
        assert_eq!(import_specifier(native), Err("is not valid UTF-8"));
    }
}
