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
//! # use ordinal_fs_tree::fs::Reading;
//! # use ordinal_fs_tree::reference::SyllabusName;
//! let opened = ordinal_fs_tree::fs::read::<SyllabusName>(Path::new("syllabus"))?;
//! let Reading::Tree(tree) = opened else { return Ok(()) };
//! for entry in tree.walk() {
//!     println!("{:indent$}{}", "", entry.name(), indent = (entry.depth() - 1) * 2);
//! }
//! # Ok::<(), ordinal_fs_tree::Error<SyllabusName>>(())
//! ```
//!
//! # *Is there a tree here* is a shape, not a predicate
//!
//! [`read`] and [`write`] do not answer *yes*; they answer with the tree, with a
//! vacancy, or with an error saying what is at the root instead. There is no
//! `exists`, and that absence is the design: a predicate beside an opening is a
//! check-then-act split, and the act it splits from is creating a tree — so
//! between the check and the create another writer fits. One lock acquisition
//! answers the question and hands back the only operation valid for the answer.
//!
//! What that buys is a whole class of call the type system refuses to spell.
//! [`Vacancy::initialize`] exists on a vacancy and nowhere else, so initializing
//! over a live tree does not typecheck; and the [`WriteGuard`] mutations exist on
//! a guard and nowhere else, so mutating a tree that is not there does not
//! either. Neither needs a run-time refusal, and neither has one.
//!
//! ```no_run
//! # use std::path::Path;
//! # use ordinal_fs_tree::fs::Writing;
//! # use ordinal_fs_tree::reference::SyllabusName;
//! match ordinal_fs_tree::fs::write::<SyllabusName>(Path::new("syllabus"))? {
//!     Writing::Tree(tree) => println!("{} entries", tree.walk().count()),
//!     // Still under the exclusive lock: nothing can create the tree between
//!     // learning it is absent and creating it here.
//!     Writing::Vacancy(vacancy) => {
//!         vacancy.initialize(Some(b"the course".to_vec()), Vec::new())?;
//!     }
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
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use crate::ops::{self, NewEntry, Target};
use crate::plan::Decision;
use crate::report::{Removed, Report};
use crate::snapshot::Snapshot;
use crate::{EntryName, Error, Key, Ordinal};

mod apply;
mod lock;
mod read;
mod remove;

/// Read a tree under a **shared** lock: other readers may hold it at the same
/// time, no writer may.
///
/// Blocks until the tree is free. Halts — rather than skipping anything — on a
/// name the consumer recognises and cannot parse, wherever in the tree it sits.
///
/// Answers with a shape and not a predicate; see this module's header. A
/// [`Reading::Vacant`] carries no guard, because a reader of a tree that is not
/// there has nothing to hold a lock against — the lock is released before this
/// returns.
///
/// # Errors
///
/// [`Error::RootIsNotATree`] when something that is not a directory is at the
/// root; [`Error::Malformed`] or [`Error::Reserved`] for a name the consumer
/// owns and refuses, carrying the consumer's own recovery advice;
/// [`Error::NonUtf8Name`] for a filename that cannot be classified at all;
/// [`Error::Io`] for a filesystem refusal; [`Error::NoContainingDirectory`] for
/// a root with nothing to lock.
pub fn read<N: EntryName>(root: &Path) -> Result<Reading<N>, Error<N>> {
    match acquire(root, lock::Mode::Shared)? {
        Opened::Tree(guard, snapshot) => Ok(Reading::Tree(ReadGuard {
            _guard: guard,
            root: root.to_path_buf(),
            snapshot,
        })),
        Opened::Vacant(_) => Ok(Reading::Vacant),
    }
}

/// Read a tree under an **exclusive** lock: nothing else holds it while this
/// guard lives.
///
/// This is the lock every mutation runs under, and the lock a
/// [`Vacancy`] holds while it is deciding whether to create the tree. It reads
/// the tree exactly as [`read`] does, because a mutation is a snapshot, a
/// decision and a plan before it is an effect.
///
/// # Errors
///
/// The same as [`read`].
pub fn write<N: EntryName>(root: &Path) -> Result<Writing<N>, Error<N>> {
    match acquire(root, lock::Mode::Exclusive)? {
        Opened::Tree(guard, snapshot) => Ok(Writing::Tree(WriteGuard {
            _guard: guard,
            root: root.to_path_buf(),
            snapshot,
        })),
        Opened::Vacant(guard) => Ok(Writing::Vacancy(Vacancy {
            _guard: guard,
            root: root.to_path_buf(),
            name: PhantomData,
        })),
    }
}

/// What an opening found, before it is dressed as a [`Reading`] or a
/// [`Writing`].
///
/// One function answers both modes because the sequence is the same either way
/// and it is the sequence that matters: find the directory to lock, lock it,
/// *then* look at the root. Only the third step can see a vacancy, and it sees
/// it under the lock.
enum Opened<N> {
    Tree(File, Snapshot<N>),
    Vacant(File),
}

fn acquire<N: EntryName>(root: &Path, mode: lock::Mode) -> Result<Opened<N>, Error<N>> {
    let directory = read::containing_directory::<N>(root)?;
    let guard = lock::take(&directory, mode).map_err(|source| Error::Io {
        path: directory.clone(),
        doing: "locking the directory containing the tree",
        source,
    })?;
    // Under the lock, and only under it. For a tree that is a snapshot which
    // could otherwise be stale before the caller saw it; for a vacancy it is the
    // absence itself, which could otherwise be false before the caller acted on
    // it.
    match read::presence::<N>(root)? {
        read::Presence::Vacant => Ok(Opened::Vacant(guard)),
        read::Presence::NotATree(found) => Err(Error::RootIsNotATree {
            root: root.to_path_buf(),
            found,
        }),
        read::Presence::Tree => Ok(Opened::Tree(guard, read::snapshot(root)?)),
    }
}

/// What [`read`] found: the tree, or no tree.
///
/// Two variants and no third — something at the root that is not a tree is an
/// [`Error::RootIsNotATree`] rather than a variant here, because it is not an
/// answer a reader can do anything with and the library will not clear it away.
#[must_use]
pub enum Reading<N> {
    /// A tree, read under a shared lock this guard holds.
    Tree(ReadGuard<N>),
    /// No tree. Nothing is held: a reader of an absent tree has nothing to
    /// exclude.
    Vacant,
}

/// What [`write`] found: the tree, or a vacancy that can become one.
///
/// The write-side twin of [`Reading`], and the difference between them is the
/// whole point of this shape. [`Writing::Vacancy`] **holds the exclusive lock**,
/// so there is no window between learning that a tree is absent and creating it.
#[must_use]
pub enum Writing<N> {
    /// A tree, read under an exclusive lock this guard holds.
    Tree(WriteGuard<N>),
    /// No tree, and the exclusive lock under which one may be created.
    Vacancy(Vacancy<N>),
}

impl<N> Reading<N> {
    /// Whether a tree was there.
    #[must_use]
    pub const fn is_tree(&self) -> bool {
        matches!(self, Self::Tree(_))
    }

    /// Whether nothing was there.
    #[must_use]
    pub const fn is_vacant(&self) -> bool {
        matches!(self, Self::Vacant)
    }

    /// The guard, panicking with `message` when the tree was not there.
    ///
    /// For a caller that has already established the tree exists — a test over
    /// a tree it built itself, most of all. [`Sought::expect`] is the same
    /// affordance for the same reason, and there is no `unwrap` beside either:
    /// an unwrap has no room to say what it was relying on.
    ///
    /// # Panics
    ///
    /// When there was no tree.
    ///
    /// [`Sought::expect`]: crate::Sought::expect
    #[must_use]
    pub fn expect_tree(self, message: &str) -> ReadGuard<N> {
        match self {
            Self::Tree(guard) => guard,
            Self::Vacant => panic!("{message}"),
        }
    }
}

impl<N> Writing<N> {
    /// Whether a tree was there.
    #[must_use]
    pub const fn is_tree(&self) -> bool {
        matches!(self, Self::Tree(_))
    }

    /// Whether nothing was there.
    #[must_use]
    pub const fn is_vacant(&self) -> bool {
        matches!(self, Self::Vacancy(_))
    }

    /// The guard, panicking with `message` when the tree was not there.
    ///
    /// See [`Reading::expect_tree`].
    ///
    /// # Panics
    ///
    /// When there was no tree.
    #[must_use]
    pub fn expect_tree(self, message: &str) -> WriteGuard<N> {
        match self {
            Self::Tree(guard) => guard,
            Self::Vacancy(_) => panic!("{message}"),
        }
    }

    /// The vacancy, panicking with `message` when a tree was there.
    ///
    /// # Panics
    ///
    /// When a tree was there.
    #[must_use]
    pub fn expect_vacancy(self, message: &str) -> Vacancy<N> {
        match self {
            Self::Vacancy(vacancy) => vacancy,
            Self::Tree(_) => panic!("{message}"),
        }
    }
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

/// The exclusive lock on a tree root that holds no tree, and the one operation
/// valid for it.
///
/// A guard, exactly as [`WriteGuard`] is one, and for the same reason: it holds
/// the lock for as long as it lives, so nothing can create the tree between
/// [`write`] answering *there is none* and [`initialize`](Vacancy::initialize)
/// making one. There is no snapshot, because there are no names to read.
///
/// # The ill-formed calls are the ones that do not exist
///
/// This type has `initialize` and no mutations; [`WriteGuard`] has the mutations
/// and no `initialize`. So *initialize over a live tree* and *append to a tree
/// that is not there* are not refusals this library states — they are programs
/// that do not compile:
///
/// ```compile_fail
/// # use std::path::Path;
/// # use ordinal_fs_tree::fs::Writing;
/// # use ordinal_fs_tree::reference::SyllabusName;
/// let Writing::Tree(tree) = ordinal_fs_tree::fs::write::<SyllabusName>(Path::new("s"))?
/// else { unreachable!() };
/// // `initialize` is on `Vacancy`, and a live tree is not one.
/// tree.initialize(None, Vec::new())?;
/// # Ok::<(), ordinal_fs_tree::Error<SyllabusName>>(())
/// ```
///
/// ```compile_fail
/// # use std::path::Path;
/// # use ordinal_fs_tree::fs::Writing;
/// # use ordinal_fs_tree::reference::SyllabusName;
/// # use ordinal_fs_tree::{NewEntry, Target};
/// let Writing::Vacancy(vacancy) = ordinal_fs_tree::fs::write::<SyllabusName>(Path::new("s"))?
/// else { unreachable!() };
/// // `append` is on `WriteGuard`, and a vacancy is not one.
/// vacancy.append(Target::Root, NewEntry::empty(todo!()))?;
/// # Ok::<(), ordinal_fs_tree::Error<SyllabusName>>(())
/// ```
///
/// ```compile_fail
/// # use std::path::Path;
/// # use ordinal_fs_tree::fs::Writing;
/// # use ordinal_fs_tree::reference::SyllabusName;
/// let Writing::Vacancy(vacancy) = ordinal_fs_tree::fs::write::<SyllabusName>(Path::new("s"))?
/// else { unreachable!() };
/// // `delete` is on `WriteGuard` too. Deleting a tree that is not there is not
/// // a refusal this library states — there is nothing to delete and nothing to
/// // report having deleted, so the call does not exist.
/// vacancy.delete()?;
/// # Ok::<(), ordinal_fs_tree::Error<SyllabusName>>(())
/// ```
pub struct Vacancy<N> {
    _guard: File,
    root: PathBuf,
    /// The domain this vacancy will be initialized in, which nothing on disk
    /// carries yet: a vacancy holds no names, so `N` appears in no field.
    ///
    /// `fn() -> N` rather than `N`, so that the marker imposes none of `N`'s
    /// variance, auto-traits or drop behaviour on a type that only ever
    /// *produces* names.
    name: PhantomData<fn() -> N>,
}

impl<N: EntryName> Vacancy<N> {
    /// The tree root that is not there, in the caller's own spelling.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// **`initialize`**: create the tree — the root directory, its distinguished
    /// child, and a first run of entries — under the lock this vacancy already
    /// holds.
    ///
    /// `distinguished` is the bytes of the root's own distinguished child, or
    /// `None` for a root without one; the two are different trees, so the
    /// choice is the caller's. `entries` are placed at the root level by exactly
    /// the rule [`append_many`](WriteGuard::append_many) uses, which over an
    /// empty tree means [`Ordinal::FIRST`] onward with keys from 1.
    ///
    /// # Why the bytes and not a [`NewEntry`]
    ///
    /// [`NewEntry`] describes a *positioned* entry — parts, from which an
    /// ordinal and a key are composed — and the distinguished child is the one
    /// entry that cannot be described that way: it carries no parts, and its
    /// name is [`EntryName::distinguished`]. The library already writes one like
    /// this when a promotion moves a leaf's bytes into a new node, so the seam
    /// stays exactly one trait and gains no method
    /// (`docs/adr/entry-name-is-the-only-seam.md`).
    ///
    /// Without it, the consumer would write the root's own content itself —
    /// outside the lock and outside the store — at the first operation of every
    /// fresh tree.
    ///
    /// # The root itself is not in the report
    ///
    /// It has no name: the root is a level, not an entry, so there is no
    /// [`Created`](crate::Created) row that could describe it.
    /// [`Report::created`](crate::Report::created) holds every *named* thing this
    /// call placed, distinguished child first.
    ///
    /// # Errors
    ///
    /// [`Error::Refused`] when bytes were supplied for a distinguished child in
    /// a domain that has none — [`Refusal::NoDistinguishedChild`], the same
    /// refusal a promotion gives for the same reason — or when bytes were
    /// supplied for an entry whose parts make a node; [`Error::Failed`] when the
    /// filesystem refused, in which case **this call left nothing behind** —
    /// not even the root, which it removes again;
    /// [`Error::FailedPartiallyRolledBack`] when undoing that failed too.
    ///
    /// *Left nothing behind* is a claim about this call and not about the state
    /// of the root, and the difference is observable: a writer that ignores the
    /// advisory lock can create the root between the vacancy being handed out
    /// and `create_dir`, which fails as [`Error::Failed`] with an
    /// [`std::io::ErrorKind::AlreadyExists`] source. Nothing this call did
    /// survives, and there is now a tree — which is the same distinction
    /// `claim_vacant` draws for every other operation, and the same neighbour it
    /// draws it against.
    ///
    /// [`Refusal::NoDistinguishedChild`]: crate::Refusal::NoDistinguishedChild
    pub fn initialize(
        self,
        distinguished: Option<Vec<u8>>,
        entries: Vec<NewEntry<N::Parts>>,
    ) -> Result<Report<N>, Error<N>> {
        // A vacancy holds no names, so the snapshot the plan is checked against
        // is the empty one — and the arithmetic over it is the ordinary
        // arithmetic rather than a first-entry special case.
        let snapshot = Snapshot::empty();
        let plan = match ops::initialize(&snapshot, distinguished, entries) {
            Decision::Refuse(refusal) => return Err(Error::Refused(refusal)),
            Decision::Proceed(plan) => plan,
        };
        // Both checks that can refuse a plan before it runs, run before the root
        // is created: the algebra's, above, and the seventh obligation's, here.
        // Otherwise a domain that renders a name badly would leave behind an
        // empty root directory while reporting an error whose whole promise is
        // that nothing changed.
        apply::names_are_one_component(&self.root, &plan)?;
        // The root is not an effect — it has no name for one to place — so this
        // is the one create the interpreter does not do. It is still under the
        // lock: the lock is on the directory *containing* the root, which is
        // what makes a tree's creation coverable at all.
        std::fs::create_dir(&self.root).map_err(|source| Error::Failed {
            path: self.root.clone(),
            doing: "creating the tree root",
            source,
        })?;
        match apply::apply(&self.root, &snapshot, &plan, apply::Faults::none()) {
            Ok(report) => Ok(report),
            // Plan atomicity says the tree is as the *plan* found it, which here
            // means a root directory holding nothing — so removing it is the
            // last step of the same unwind, and `remove_dir` refusing a
            // non-empty one is a check rather than an obstacle.
            Err(Error::Failed {
                path,
                doing,
                source,
            }) => Err(match std::fs::remove_dir(&self.root) {
                Ok(()) => Error::Failed {
                    path,
                    doing,
                    source,
                },
                // The unwind's goal was that no root be left, and there is none.
                // Reporting `FailedPartiallyRolledBack` — *the tree is in
                // neither state* — for a removal that found its work already
                // done would send a consumer looking for damage that is not
                // there, and that variant's whole value is that it is rare and
                // means what it says.
                Err(unwind_source) if unwind_source.kind() == io::ErrorKind::NotFound => {
                    Error::Failed {
                        path,
                        doing,
                        source,
                    }
                }
                Err(unwind_source) => Error::FailedPartiallyRolledBack {
                    path,
                    doing,
                    source,
                    unwinding: self.root.clone(),
                    undoing: "removing the tree root this operation had created at",
                    unwind_source,
                },
            }),
            // The interpreter's own rollback already failed, so the root is not
            // known to be empty and removing it is not this call's to attempt.
            Err(error) => Err(error),
        }
    }
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

    /// **`rewrite`**: replace the parts of the entry with this key, keeping its
    /// ordinal, its key and its species.
    ///
    /// This is how an attribute changes: the entry keeps its identity and its
    /// place, and only the opaque remainder of its name moves. One rename, and
    /// on a node it is the directory that is renamed, so its whole subtree comes
    /// with it untouched.
    ///
    /// It is the general form of every *mark this entry* operation a consumer
    /// might want, and with no operation that removes an *entry*
    /// (`docs/adr/entries-are-never-removed.md`) it is also how a domain retires
    /// one: rewrite an attribute.
    ///
    /// Named by [`Key`] and not by [`Target`], like
    /// [`promote`](WriteGuard::promote): a rewrite's target is an entry, and the
    /// tree root is not one — it has no name to rewrite.
    ///
    /// # The species cannot change
    ///
    /// A leaf is a regular file and a node is a directory, so parts implying a
    /// different species would ask for a rename that is not a rename. Changing
    /// shape is [`promote`](WriteGuard::promote)'s job, and it goes one way
    /// only: a leaf's content has somewhere to land and a node's children have
    /// nowhere.
    ///
    /// # Rewriting to the parts an entry already carries succeeds
    ///
    /// It is a rename onto the entry's own path, and it changes nothing rather
    /// than being refused as a collision with itself — occupancy excludes the
    /// object being moved. The report still names it, because a caller reading
    /// [`Report::renamed`](crate::Report::renamed) to learn where an entry lives
    /// wants the answer whether or not the filesystem was touched.
    ///
    /// # Errors
    ///
    /// [`Error::Refused`] when the key names no entry, or when `parts` imply a
    /// different species from the one the entry has. [`Error::Failed`] when the
    /// filesystem refused and the tree was left as it was found;
    /// [`Error::FailedPartiallyRolledBack`] when undoing that failed too — which
    /// a single-effect plan reaches only through a failing unwind of its one
    /// rename.
    pub fn rewrite(self, key: Key, parts: N::Parts) -> Result<Report<N>, Error<N>> {
        let decision = ops::rewrite(&self.snapshot, key, parts);
        self.run(decision, apply::Faults::none())
    }

    /// **`delete`**: remove the tree root and everything beneath it, following
    /// no symbolic link, and report the paths that went.
    ///
    /// The one mutation that is not planned from the snapshot. A plan is a list
    /// of effects over *names*, and a deletion acts on the root — so it acts on
    /// everything that is there, including the entries the domain declined to
    /// parse as its own, which are in no snapshot and have no name for a plan to
    /// carry. [`Removed`] therefore reports **paths**, and reports the foreign
    /// ones too: a report that left them out would undercount what was
    /// destroyed.
    ///
    /// The root goes last and everything beneath it goes children-first, in each
    /// level's sorted listing order. That order is for reproducibility and buys
    /// no property — unlike the highest-first shift, whose order decides what an
    /// interruption leaves. There is no order in which an interrupted deletion
    /// leaves a shape this design admits.
    ///
    /// # Following no link is a security property
    ///
    /// Descent is decided by the same unfollowed look a snapshot is read
    /// through, so a symbolic link — even one naming a directory, even one
    /// naming a directory **outside** the root — is unlinked as a link and its
    /// target is untouched. The bound on that is stated in this module's
    /// `remove` submodule: the look and the descent are two syscalls, and the
    /// writer who could exploit the gap between them is the writer who ignores
    /// the advisory lock, which is already outside what this library defends
    /// against.
    ///
    /// # The root's own spelling must name the root
    ///
    /// This is the one operation with a precondition on how the root was
    /// *spelled*, and it is the only one that acts on the root as an **object**
    /// rather than as the directory things are in. So a spelling whose last
    /// component is a symbolic link is refused rather than followed — a link and
    /// what it names are two things, and this library will not choose between
    /// them — and so is one that descends into the tree and comes back out
    /// through `..`, whose own components the removal would take away.
    /// [`Error::RootIsNotSpelledDirectly`] carries which, and nothing is removed.
    /// Every other operation accepts both spellings, deliberately.
    ///
    /// # This is not an escape from a tree the domain cannot read
    ///
    /// Deletion begins where every operation begins — at an opening — so a
    /// [`Malformed`](Error::Malformed) or [`Reserved`](Error::Reserved) name
    /// halts it before this method is reachable at all. That is deliberate: the
    /// library will not destroy a tree it was refused permission to understand.
    /// Whoever meets that halt has the domain's own recovery advice, and the
    /// shell.
    ///
    /// # There is no rollback, and no recovery path
    ///
    /// An unlinked file is gone. A removal that could be undone would be one
    /// that copied the tree aside first, and staging a rollback for a
    /// destruction is exactly the machinery a version control system already
    /// provides — the library's part is to be honest about what it did.
    ///
    /// # Errors
    ///
    /// [`Error::RootIsNotSpelledDirectly`] before anything is removed, for the
    /// two spellings above. [`Error::RemovalStopped`] once the removal has
    /// begun: it carries the step that failed and the paths that had already
    /// gone, and its message distinguishes a removal that got nowhere from one
    /// that got partway. No refusal is reachable — the algebra is not consulted,
    /// because there is nothing for it to decide.
    pub fn delete(self) -> Result<Removed, Error<N>> {
        remove::tree(&self.root)
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
