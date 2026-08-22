//! Turning a directory tree into a [`Snapshot`].
//!
//! This is the whole of the parse trichotomy in practice: every name in every
//! directory a walk reaches is handed to the consumer's
//! [`parse`](EntryName::parse) together with **what the listing found under it,
//! unfollowed**, and the three outcomes are the three things that can happen to
//! a name. `Entry` joins the tree. `Foreign` is skipped — and skipped
//! recursively when it is a directory, which is sound precisely because the
//! consumer said the name was not its own. `Malformed` and `Reserved` halt.
//!
//! Snapshot scope is the **whole tree**, so a halt anywhere halts everything.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, Error, Found, Verdict};

/// Read a whole tree, or halt.
pub(super) fn snapshot<N: EntryName>(root: &Path) -> Result<Snapshot<N>, Error<N>> {
    let mut builder = Builder::new();
    // An explicit worklist rather than recursion: the depth of a tree on disk
    // is the user's to choose, and a stack overflow is not a refusal any
    // consumer can handle.
    let mut pending = vec![(root.to_path_buf(), builder.root())];
    while let Some((directory, place)) = pending.pop() {
        let mut descend = Vec::new();
        for (name, found) in listing(&directory)? {
            let path = directory.join(&name);
            let Some(name) = name.to_str() else {
                return Err(Error::NonUtf8Name { path });
            };
            match N::parse(name, found) {
                // Not this consumer's name, so it is not this consumer's
                // problem — and not this library's either.
                Verdict::Foreign => {}
                Verdict::Malformed(source) => return Err(Error::Malformed { path, source }),
                Verdict::Reserved(source) => return Err(Error::Reserved { path, source }),
                // A walk descends into recognised nodes and nothing else, and
                // `add` answers with a place exactly for those. A distinguished
                // child is a node's own content rather than a level of the
                // tree, and it is a regular file — a domain holding the
                // obligations cannot produce one that is not.
                Verdict::Entry(parsed) => {
                    if let Some(below) = builder.add(place, parsed) {
                        descend.push((path, below));
                    }
                }
            }
        }
        // Sorted order is the order the *listing* was read in; pushing the
        // subdirectories in reverse makes the stack pop them in that order, so
        // which of two broken names halts the tree does not depend on where the
        // filesystem happened to put them.
        while let Some(child) = descend.pop() {
            pending.push(child);
        }
    }
    Ok(builder.finish())
}

/// One directory's names and what is under each, sorted.
///
/// Sorted because the halt has to be deterministic: a tree carrying two names
/// the consumer cannot parse would otherwise report whichever one `read_dir`
/// reached first, so the recovery advice a consumer sees would depend on the
/// filesystem rather than on the tree.
fn listing<N: EntryName>(directory: &Path) -> Result<Vec<(OsString, Found)>, Error<N>> {
    let reading = fs::read_dir(directory).map_err(|source| Error::<N>::Io {
        path: directory.to_path_buf(),
        doing: "reading the directory",
        source,
    });
    let mut found = Vec::new();
    for entry in reading? {
        let entry = entry.map_err(|source| Error::<N>::Io {
            path: directory.to_path_buf(),
            doing: "reading the directory",
            source,
        })?;
        // `DirEntry::file_type` does not traverse a symbolic link — it is
        // `symlink_metadata`, not `metadata`. That is what makes a symbolic
        // link wearing an entry's name `Found::Other`, and therefore
        // `Malformed`, rather than whatever it points at.
        // <https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type>
        let kind = entry.file_type().map_err(|source| Error::<N>::Io {
            path: entry.path(),
            doing: "inspecting",
            source,
        })?;
        let kind = if kind.is_file() {
            Found::File
        } else if kind.is_dir() {
            Found::Dir
        } else {
            Found::Other
        };
        found.push((entry.file_name(), kind));
    }
    // By name: two entries of one directory cannot share one, so this is total.
    found.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

/// The directory whose lock covers this tree: the one **containing** the root.
///
/// Lexical, and deliberately so — see `lock.rs`. An empty parent is the current
/// directory, which is what `Path::parent` reports for a bare relative name.
pub(super) fn containing_directory<N: EntryName>(root: &Path) -> Result<PathBuf, Error<N>> {
    match root.parent() {
        None => Err(Error::NoContainingDirectory {
            root: root.to_path_buf(),
        }),
        Some(parent) if parent.as_os_str().is_empty() => Ok(PathBuf::from(".")),
        Some(parent) => Ok(parent.to_path_buf()),
    }
}
