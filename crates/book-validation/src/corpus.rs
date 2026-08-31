//! The corpus rule, evaluated against the real repository.
//!
//! [`derive`] enumerates every file each `[corpus] include` pattern reaches,
//! removes every `[[corpus.exclude]]` path and adds every `[[corpus.add]]`
//! path. The resulting **derived set** travels to the core in
//! [`crate::BookSnapshot::derived_corpus`], where it is required to equal the
//! set of declared `[[root]]` paths.
//!
//! **What this control is, and what it is not.** It replaces the pair of
//! hand-written lists the compiled ledger carried, and in one direction it is
//! stronger than they were: a list checked against the tree it describes cannot
//! be wrong in the same direction as the tree, so a production file added to the
//! crate and forgotten by the book is a failure rather than a silence. In
//! another direction it is weaker, and saying otherwise would misdescribe it.
//! Derivation proves the declared patterns matched. It cannot prove the author
//! declared the right patterns, and it cannot judge an exception. Two rules
//! elsewhere close that gap and neither lives here: the base patterns are fixed
//! by `[book].subject` ([`crate::manifest`]), and every exception is declared a
//! second time in `docs/specs/walkthrough-books.md`'s corpus exception
//! inventory, which a repository test compares against every manifest.
//!
//! The walk lives in the library rather than in the CLI because the test fixture
//! derives its own corpus through it. A fixture that restated the answer would
//! make the core tests agree with themselves by construction, which is the
//! self-witnessing this module exists to remove.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::manifest::{Manifest, Pattern};

/// A corpus rule that could not be evaluated. Reported by the CLI as `U002`
/// with this reason, like a schema-invalid manifest: both are the book's own
/// declaration failing before any validation runs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorpusError {
    reason: String,
}

impl CorpusError {
    fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// The set of repository-relative paths the book's corpus rule reaches.
pub fn derive(manifest: &Manifest, repository: &Path) -> Result<BTreeSet<String>, CorpusError> {
    let corpus = manifest.corpus();
    let mut derived = BTreeSet::new();
    for pattern in corpus.include() {
        match pattern {
            Pattern::Exact(path) => {
                if is_regular_file(&repository.join(path)) {
                    derived.insert(path.clone());
                }
            }
            Pattern::Recursive {
                directory,
                extension,
            } => {
                let suffix = format!(".{extension}");
                walk(repository, directory, &suffix, &mut derived)?;
            }
        }
    }
    for exception in corpus.exclude() {
        let path = exception.path();
        if !is_regular_file(&repository.join(path)) {
            return Err(CorpusError::new(format!(
                "`[[corpus.exclude]]` path `{path}` is not a file in this repository; a stale exception is a failure rather than a silent no-op"
            )));
        }
        // An exclusion that excludes nothing is either a typo or a rule that
        // has moved, and both are worth a refusal — a silent no-op here would
        // let a corpus boundary rot in place while the book still reads green.
        if !derived.remove(path) {
            return Err(CorpusError::new(format!(
                "`[[corpus.exclude]]` path `{path}` is matched by no `[corpus] include` pattern, so it excludes nothing"
            )));
        }
    }
    for exception in corpus.add() {
        let path = exception.path();
        if !is_regular_file(&repository.join(path)) {
            return Err(CorpusError::new(format!(
                "`[[corpus.add]]` path `{path}` is not a file in this repository; a stale exception is a failure rather than a silent no-op"
            )));
        }
        derived.insert(path.to_owned());
    }
    Ok(derived)
}

fn walk(
    repository: &Path,
    directory: &str,
    suffix: &str,
    derived: &mut BTreeSet<String>,
) -> Result<(), CorpusError> {
    let absolute = repository.join(directory);
    let Ok(entries) = fs::read_dir(&absolute) else {
        // A pattern whose directory does not exist reaches nothing. It is not
        // an error here: every declared root it would have matched is already
        // reported as a root the rule does not reach, which names the book's
        // own row rather than a path the reader has to map back to one.
        return Ok(());
    };
    let mut children = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            CorpusError::new(format!(
                "cannot enumerate `{directory}` for `[corpus] include`: {error}"
            ))
        })?;
        children.push(entry.file_name());
    }
    children.sort();
    for name in children {
        let Some(name) = name.to_str() else {
            continue;
        };
        let relative = format!("{directory}/{name}");
        let path = absolute.join(name);
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        // Symlinks are followed by neither branch: a link could reach outside
        // the repository, and a corpus whose membership depends on link targets
        // is not one a reader can evaluate by inspection.
        if metadata.file_type().is_dir() {
            walk(repository, &relative, suffix, derived)?;
        } else if metadata.file_type().is_file() && name.ends_with(suffix) {
            derived.insert(relative);
        }
    }
    Ok(())
}

fn is_regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
}
