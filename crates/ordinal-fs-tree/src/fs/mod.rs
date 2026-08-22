//! The filesystem, and the only module in this crate that may name it.
//!
//! Everything else here is the algebra: pure, testable without a directory, and
//! modellable without an abstraction of one. That boundary is what makes a later
//! split of this crate into separately-modellable units mechanical rather than
//! archaeological, and `tests/algebra_has_no_filesystem.rs` is what holds it —
//! inside one crate, a seam the compiler does not enforce is a seam nothing
//! measures.
//!
//! # What a guard is
//!
//! One advisory lock and one [`Snapshot`], taken together. Every operation the
//! library performs begins by reading the tree's names under a lock, so a guard
//! is that pair and holding one is holding both: the lock is released when the
//! guard is dropped, and there is no unlock call to forget.
//!
//! ```no_run
//! # use std::path::Path;
//! # use ordinal_fs_tree::reference::SyllabusName;
//! let tree = ordinal_fs_tree::fs::read::<SyllabusName>(Path::new("syllabus"))?;
//! for entry in tree.walk() {
//!     println!("{:indent$}{}", "", entry.name(), indent = (entry.depth() - 1) * 2);
//! }
//! # Ok::<(), ordinal_fs_tree::Error<SyllabusName>>(())
//! ```
//!
//! # Locking is invisible in the interface
//!
//! There is no lock type in this module's public surface, no *try* variant and
//! no timeout — an API offering any of those would be an API that mentions
//! locking, which the architecture document says consumers never do. What that
//! costs is stated rather than hidden: [`read`] and [`write`] block until the
//! tree is free.
//!
//! # Paths come back the way they went in
//!
//! Nothing here canonicalises. The lock follows inode identity through the
//! descriptor, so it does not need a canonical path to be correct, and every
//! path a guard reports is built from the caller's own spelling of the root. On
//! macOS `/var` and `/private/var` name the same inode, so canonicalising would
//! make the mere presence of a lock observably rewrite every path the reading
//! operations return.
//!
//! The lock still has to name the **tree** rather than a spelling of it, or two
//! accepted spellings of one root would not exclude each other. That is bought
//! by asking the kernel instead of the string: the directory to lock is
//! `<root>/..`, which resolves through `..` and through a symbolic link naming
//! the root and lands on one inode however the caller wrote the path. See
//! `read::containing_directory`, which carries the counterexample that made a
//! lexical parent insufficient.

use std::fs::File;
use std::path::{Path, PathBuf};

use crate::ops::{self, NewEntry, Target};
use crate::plan::Decision;
use crate::report::Report;
use crate::snapshot::Snapshot;
use crate::{EntryName, Error};

mod apply;
mod lock;
mod read;

/// Read a tree under a **shared** lock: other readers may hold it at the same
/// time, no writer may.
///
/// Blocks until the tree is free. Halts — rather than skipping anything — on a
/// name the consumer recognises and cannot parse, wherever in the tree it sits.
///
/// # Errors
///
/// [`Error::Malformed`] or [`Error::Reserved`] for a name the consumer owns and
/// refuses, carrying the consumer's own recovery advice; [`Error::NonUtf8Name`]
/// for a filename that cannot be classified at all; [`Error::Io`] for a
/// filesystem refusal; [`Error::NoContainingDirectory`] for a root with nothing
/// to lock.
pub fn read<N: EntryName>(root: &Path) -> Result<ReadGuard<N>, Error<N>> {
    let (guard, snapshot) = acquire(root, lock::Mode::Shared)?;
    Ok(ReadGuard {
        _guard: guard,
        root: root.to_path_buf(),
        snapshot,
    })
}

/// Read a tree under an **exclusive** lock: nothing else holds it while this
/// guard lives.
///
/// This is the lock every mutation runs under. It reads the tree exactly as
/// [`read`] does, because a mutation is a snapshot, a decision and a plan before
/// it is an effect.
///
/// # Errors
///
/// The same as [`read`].
pub fn write<N: EntryName>(root: &Path) -> Result<WriteGuard<N>, Error<N>> {
    let (guard, snapshot) = acquire(root, lock::Mode::Exclusive)?;
    Ok(WriteGuard {
        _guard: guard,
        root: root.to_path_buf(),
        snapshot,
    })
}

fn acquire<N: EntryName>(root: &Path, mode: lock::Mode) -> Result<(File, Snapshot<N>), Error<N>> {
    let directory = read::containing_directory::<N>(root)?;
    let guard = lock::take(&directory, mode).map_err(|source| Error::Io {
        path: directory.clone(),
        doing: "locking the directory containing the tree",
        source,
    })?;
    // Under the lock, and only under it: a snapshot read outside one could be
    // stale before the caller saw it.
    let snapshot = read::snapshot(root)?;
    Ok((guard, snapshot))
}

/// A tree read under a shared lock.
///
/// Derefs to its [`Snapshot`], so the reading operations are called on the guard
/// itself: `tree.walk()`, `tree.by_key(k)`.
pub struct ReadGuard<N> {
    // The lock *is* this descriptor: `flock` is released when the last handle to
    // the open file description closes, so the field never has to be read and
    // dropping the guard is the whole of releasing it.
    _guard: File,
    root: PathBuf,
    snapshot: Snapshot<N>,
}

/// A tree read under an exclusive lock, and the surface every mutation is on.
///
/// # A mutation consumes its guard
///
/// One guard, one mutation. The alternative — a `&mut self` that leaves the
/// guard alive — would leave the snapshot describing a tree that no longer
/// exists, and every operation is planned *from the snapshot*: a second
/// `append` would compute the same fresh key as the first and collide with what
/// the first had just written. Refreshing the snapshot instead would mean a
/// mutation that succeeded returning the error of the re-read that followed it,
/// which is exactly the shape *plan atomicity* promises not to have.
///
/// So the guard is consumed and the lock is released with it. Reading first is
/// unaffected — the reading operations are on the guard, so
/// `guard.by_key(k)` then `guard.append(…)` is one lock and one snapshot — and a
/// caller wanting several entries at once has [`WriteGuard::append_many`], which
/// is a run planned from one snapshot rather than a loop that takes several.
pub struct WriteGuard<N> {
    _guard: File,
    root: PathBuf,
    snapshot: Snapshot<N>,
}

impl<N: EntryName> ReadGuard<N> {
    /// The tree root, in the caller's own spelling.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The tree's names, as they were when the lock was taken.
    #[must_use]
    pub fn snapshot(&self) -> &Snapshot<N> {
        &self.snapshot
    }
}

impl<N: EntryName> WriteGuard<N> {
    /// The tree root, in the caller's own spelling.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The tree's names, as they were when the lock was taken.
    #[must_use]
    pub fn snapshot(&self) -> &Snapshot<N> {
        &self.snapshot
    }
}

impl<N: EntryName> WriteGuard<N> {
    /// **`append`**: add a child at the end of a level — the next free ordinal,
    /// and a key that is `max + 1` over every name in the tree.
    ///
    /// The target is named by key, or by [`Target::Root`], because an ordinal is
    /// stale as soon as anything is inserted before it and a path is stale as
    /// soon as anything is renamed.
    ///
    /// # Errors
    ///
    /// [`Error::Refused`] when the target names no entry, or names something
    /// that is not a node, or when bytes were supplied for parts that make a
    /// node; [`Error::Failed`] when the filesystem refused and the tree was left
    /// as it was found; [`Error::FailedPartiallyRolledBack`] when undoing that
    /// failed too, which is the one case the tree needs a human.
    pub fn append(self, target: Target, entry: NewEntry<N::Parts>) -> Result<Report<N>, Error<N>> {
        let decision = ops::append(&self.snapshot, target, entry);
        self.run(decision, apply::Faults::none())
    }

    /// **`append_many`**: add several children at consecutive ordinals with
    /// consecutive keys, planned from one snapshot and applied as a unit.
    ///
    /// Either the whole run lands or none of it does. An empty run succeeds and
    /// changes nothing.
    ///
    /// # Errors
    ///
    /// The same as [`WriteGuard::append`].
    pub fn append_many(
        self,
        target: Target,
        entries: Vec<NewEntry<N::Parts>>,
    ) -> Result<Report<N>, Error<N>> {
        let decision = ops::append_many(&self.snapshot, target, entries);
        self.run(decision, apply::Faults::none())
    }

    /// Turn a decision into an outcome: refuse, or apply under the lock this
    /// guard holds.
    ///
    /// The one place a [`Decision`] becomes a `Result`, which is what keeps
    /// *every operation is total* true of the algebra while the surface stays
    /// ordinary Rust.
    fn run(self, decision: Decision<N>, faults: apply::Faults) -> Result<Report<N>, Error<N>> {
        match decision {
            Decision::Refuse(refusal) => Err(Error::Refused(refusal)),
            Decision::Proceed(plan) => apply::apply(&self.root, &self.snapshot, &plan, faults),
        }
    }
}

impl<N: EntryName> core::ops::Deref for ReadGuard<N> {
    type Target = Snapshot<N>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl<N: EntryName> core::ops::Deref for WriteGuard<N> {
    type Target = Snapshot<N>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}
