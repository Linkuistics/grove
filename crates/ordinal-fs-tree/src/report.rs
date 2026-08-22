//! What a mutating operation tells the consumer it did.
//!
//! A plan is internal — `ARCHITECTURE.md` says so in as many words: *a consumer
//! calls `tree.insert(...)` and receives a report of what happened, never a plan
//! to apply*. This is that report, and it is written as the interpreter goes, so
//! it describes what the filesystem did rather than what the algebra intended.
//!
//! Paths are here and names are here, and both are needed. A name is the
//! library's own currency — a consumer reads the fresh key off it, which is the
//! one thing an `append` produces that the caller could not have known — while a
//! path is what the consumer opens, and it is built from the caller's own
//! spelling of the root, because nothing in this crate canonicalises anything.

use core::fmt;
use std::path::{Path, PathBuf};

use crate::EntryName;

/// An entry this operation brought into being.
pub struct Created<N> {
    /// Its name, carrying the ordinal and the key the library allocated.
    pub name: N,
    /// Where it now is, in the caller's own spelling of the root.
    pub path: PathBuf,
}

/// An entry this operation renamed — a sibling shift, or a promoted leaf moving
/// into its new node.
pub struct Renamed<N> {
    /// The name it now carries.
    pub name: N,
    /// Where it was.
    pub from: PathBuf,
    /// Where it is now.
    pub to: PathBuf,
}

/// What a mutating operation did.
///
/// Empty when the operation had nothing to do — an `append_many` of no entries
/// is a plan of no effects, which succeeds and changes nothing.
pub struct Report<N> {
    created: Vec<Created<N>>,
    renamed: Vec<Renamed<N>>,
}

impl<N: EntryName> Report<N> {
    /// A report of nothing yet.
    pub(crate) fn empty() -> Self {
        Self {
            created: Vec::new(),
            renamed: Vec::new(),
        }
    }

    pub(crate) fn record_created(&mut self, name: N, path: PathBuf) {
        self.created.push(Created { name, path });
    }

    pub(crate) fn record_renamed(&mut self, name: N, from: PathBuf, to: PathBuf) {
        self.renamed.push(Renamed { name, from, to });
    }

    /// The entries this operation created, in the order it created them.
    ///
    /// For an `append_many` that is the order the run was asked for, at
    /// consecutive ordinals and consecutive keys.
    #[must_use]
    pub fn created(&self) -> &[Created<N>] {
        &self.created
    }

    /// The entries this operation renamed, in the order it renamed them.
    ///
    /// A sibling shift renames highest-ordinal-first, so this is that order and
    /// not the level's: reading it is how a caller sees the property the
    /// architecture document argues for, which is about the *intermediate*
    /// states an interrupted operation leaves.
    #[must_use]
    pub fn renamed(&self) -> &[Renamed<N>] {
        &self.renamed
    }

    /// Every path this operation left behind, created and renamed alike, in the
    /// order the effects landed.
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.created
            .iter()
            .map(|created| created.path.as_path())
            .chain(self.renamed.iter().map(|renamed| renamed.to.as_path()))
    }
}

// `Debug` by hand rather than by derive, for the reason `Triple` and `Entry`
// give: a derive would bound `N: Debug`, and a spurious bound on a public type
// propagates into consumers' signatures. A name is `Display`, which is the one
// rendering the library knows about.
impl<N: EntryName> fmt::Debug for Created<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Created")
            .field("name", &self.name.to_string())
            .field("path", &self.path)
            .finish()
    }
}

impl<N: EntryName> fmt::Debug for Renamed<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renamed")
            .field("name", &self.name.to_string())
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

impl<N: EntryName> fmt::Debug for Report<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Report")
            .field("created", &self.created)
            .field("renamed", &self.renamed)
            .finish()
    }
}
