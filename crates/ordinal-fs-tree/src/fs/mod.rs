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
use crate::{EntryName, Error, Key, Ordinal};

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

    /// **`insert`**: add a child at an occupied ordinal, shifting the occupant
    /// and every later sibling up by one.
    ///
    /// The target is the **node** the child goes into, named by key or by
    /// [`Target::Root`]; `at` is the ordinal within it. One rename per shifted
    /// sibling and one create — and each shift is a single rename, so a shifted
    /// node carries its whole subtree with it and nothing inside it is touched.
    ///
    /// The renames run highest-ordinal-first, which
    /// [`Report::renamed`](crate::Report::renamed) shows in that order.
    /// [`Report::paths`](crate::Report::paths) shows the plan's own landing
    /// order instead — shifts, then the create — which for this operation is
    /// neither species' order and is what a caller reading *what happened*
    /// wants.
    ///
    /// # Errors
    ///
    /// [`Error::Refused`] when the target names no entry or names something
    /// that is not a node; when **nothing occupies `at`**, which covers both
    /// inserting past the last sibling — `append`'s job, refused rather than
    /// quietly redirected — and inserting into a gap in a hand-edited level,
    /// which no operation fills; or when bytes were supplied for parts that
    /// make a node. [`Error::Failed`] when the filesystem refused and the tree
    /// was left as it was found; [`Error::FailedPartiallyRolledBack`] when
    /// undoing that failed too.
    pub fn insert(
        self,
        target: Target,
        at: Ordinal,
        entry: NewEntry<N::Parts>,
    ) -> Result<Report<N>, Error<N>> {
        let decision = ops::insert(&self.snapshot, target, at, entry);
        self.run(decision, apply::Faults::none())
    }

    /// **`promote`**: turn the leaf with this key into a node, moving its bytes
    /// verbatim into the new node's distinguished child.
    ///
    /// The node keeps the leaf's **own** ordinal and its **own** key: the entry
    /// that was a leaf *is* the node, so every reference to it by key still
    /// resolves. That is about the entity and not about the file — the node is a
    /// new directory, and the leaf's own file survives inside it as the
    /// distinguished child holding its content. A consumer holding a path is
    /// stale either way; one holding a key is not, which is the whole reason the
    /// key exists.
    ///
    /// `parts` are the **node's**, and they come from the caller because the
    /// library cannot make them: `Parts` is opaque, so every value the library
    /// can reach belongs to a name already in the tree and none of those
    /// describes this entry as a node. `first_child` optionally creates one
    /// child inside the new node in the same unit, at [`Ordinal::FIRST`] with
    /// the tree's next key — for consumers that want both or neither.
    ///
    /// Named by [`Key`] and not by [`Target`], because a promotion's target is
    /// an entry that has to be a leaf and the tree root is neither.
    ///
    /// # This is the one operation that breaks an invariant on the way through
    ///
    /// The node has to exist before the leaf's content can move into it, and it
    /// carries the leaf's own ordinal and key — so **between the two effects
    /// both are on disk, sharing an ordinal and a key**. There is no ordering
    /// that avoids it: the library has no name for a temporary, and a node with
    /// any other ordinal or key would not be the same entry. The library's
    /// invariants therefore hold of **quiescent** trees — trees between
    /// operations — and not of every state the filesystem passes through, and
    /// the exclusive lock is what makes that safe, since no cooperating reader
    /// observes an intermediate state. What a *crash* exposes is exactly this
    /// state, and [`Error::FailedPartiallyRolledBack`] says how to resolve it.
    ///
    /// # And it is the one path by which this library damages a tree
    ///
    /// Rollback covers reported errors. If the unwind of the created node
    /// *itself* fails, the leaf and the node are both left in place sharing an
    /// ordinal and a key — a duplicate key in a tree the library built, which is
    /// otherwise a defect it only ever inherits. Recovery is mechanical, and the
    /// error says it: **a node and a leaf sharing an ordinal and a key, with the
    /// node holding no distinguished child, is an interrupted promotion, and
    /// removing either half resolves it.**
    ///
    /// # Errors
    ///
    /// [`Error::Refused`] when the key names no entry; when it names something
    /// that is not a leaf; when this domain has no distinguished child, so the
    /// leaf's content would have nowhere to go; when `parts` do not imply a
    /// node; or when bytes were supplied for a first child whose parts make a
    /// node. [`Error::Failed`] when the filesystem refused and the tree was left
    /// as it was found; [`Error::FailedPartiallyRolledBack`] when undoing that
    /// failed too, which on this path is the case above.
    pub fn promote(
        self,
        key: Key,
        parts: N::Parts,
        first_child: Option<NewEntry<N::Parts>>,
    ) -> Result<Report<N>, Error<N>> {
        let decision = ops::promote(&self.snapshot, key, parts, first_child);
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
