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

use std::fs::File;
use std::path::{Path, PathBuf};

use crate::snapshot::Snapshot;
use crate::{EntryName, Error};

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

fn acquire<N: EntryName>(
    root: &Path,
    mode: lock::Mode,
) -> Result<(File, Snapshot<N>), Error<N>> {
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

/// A tree read under an exclusive lock. The mutations are added to this type by
/// the leaves that implement them; today it reads exactly as [`ReadGuard`] does.
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
