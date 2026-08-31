# Filesystem interpreter
<!-- book-page id="filesystem-interpreter" slice="filesystem-interpreter-k16" order="6" -->
[Previous: Mutation algebra](05-mutation-algebra.md) | [Contents](README.md) | [Next: Syllabus CLI](07-syllabus-cli.md)

The filesystem layer owns both halves of a tree's lifetime. A `Vacancy`
initializes an absent root while retaining the exclusive lock that established
its absence. A live `WriteGuard` either sends an algebraic `Plan` through the
shared interpreter or deletes the whole root through the removal walk. Planned
mutations validate rendered destinations, apply effects in order, record a
`Report`, and unwind landed effects in reverse after a reported forward
failure. Deletion reports removed paths and has no rollback.

The guarantee is bounded. A completed unwind undoes every effect this run
landed; when no process writes outside the locking protocol, that restores the
tree the snapshot described. A failed unwind reports a partial rollback and
requires repair. Process termination can expose an intermediate state because
there is no journal or restart recovery. The advisory lock hides those
intermediate states only from processes that use this library's locking
protocol.

<!-- fragment «filesystem-error-source» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="1-510" parent="source-error" -->
<!-- insert «error-boundary» -->
<!-- insert «error-taxonomy» -->
<!-- insert «error-debug» -->
<!-- insert «error-display» -->
<!-- insert «error-sources» -->
<!-- /fragment -->

<!-- fragment «filesystem-write-guard-api» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="535-812" parent="source-filesystem-module" -->
<!-- insert «write-guard-accessors» -->
<!-- insert «write-guard-append» -->
<!-- insert «write-guard-insert» -->
<!-- insert «write-guard-promote» -->
<!-- insert «write-guard-rewrite» -->
<!-- insert «write-guard-delete» -->
<!-- insert «write-guard-dispatch» -->
<!-- /fragment -->

<!-- fragment «filesystem-interpreter-source» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="1-488" parent="source-filesystem-apply" -->
<!-- insert «apply-contract» -->
<!-- insert «apply-plan» -->
<!-- insert «apply-run-state» -->
<!-- insert «apply-effect-step» -->
<!-- insert «apply-unwind-and-paths» -->
<!-- insert «apply-undo» -->
<!-- insert «apply-destination-claim» -->
<!-- insert «apply-fault-seam» -->
<!-- /fragment -->

<!-- fragment «filesystem-removal-source» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/remove.rs" lines="1-275" parent="source-filesystem-remove" -->
<!-- insert «remove-contract» -->
<!-- insert «remove-tree» -->
<!-- insert «remove-spelling-guard» -->
<!-- insert «remove-worklist-and-failure» -->
<!-- /fragment -->

<!-- fragment «filesystem-lock-source» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/lock.rs" lines="1-91" parent="source-filesystem-lock" -->
<!-- insert «lock-contract» -->
<!-- insert «lock-modes» -->
<!-- insert «lock-take» -->
<!-- /fragment -->

<a id="write-lock"></a>
## Exclusive acquisition and snapshot timing

Both public guard constructors use `acquire`. A read takes `Mode::Shared`; a
write takes `Mode::Exclusive`. Acquisition resolves the directory containing
the root, opens that directory, blocks in `flock`, and reads the complete
snapshot only after the lock succeeds. The returned `File` descriptor is the
lock. Keeping it in the guard keeps the lock held, and dropping the guard
releases it.

The containing directory is the lock object because it exists before the tree
root is created and after that root is removed. The kernel resolves the
caller-spelled `<root>/..`, so distinct path spellings that reach the same
directory reach the same inode lock without canonicalising the paths returned
to the consumer. `flock` is advisory: library readers and writers coordinate,
but a process that edits the directory without taking this lock remains able to
race the operation.

The write constructor and shared acquisition function establish the timing.
For a live tree, the snapshot and lock descriptor enter `WriteGuard` together;
no consumer can receive a snapshot taken before the exclusive lock. For an
absent root, the descriptor enters `Vacancy` without a snapshot, retaining the
same lock across the decision to initialize.

<!-- fragment «filesystem-write-acquire» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="129-155" parent="source-filesystem-module" -->
````rust

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

````
<!-- /fragment -->

The lock module owns the common contract used by shared reads and this page's
exclusive write. This fragment turns a resolved containing-directory identity
into a descriptor-held advisory lock, establishes one lock object across root
spellings and root creation or removal, and supplies the lifetime boundary that
keeps the worked insert locked from snapshot through application.

<!-- fragment «lock-contract» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/lock.rs" lines="1-40" parent="filesystem-lock-source" -->
````rust
//! The advisory lock, on the directory *containing* the tree root.
//!
//! Not on the root itself. The containing directory exists before the root is
//! created and persists after it is deleted, so the tree's creation and
//! destruction fall under the same lock as every ordinary operation. That
//! reasoning is general rather than domain-specific, which is why locking is the
//! library's own rule and not a parameter: consumers never mention it.
//!
//! # Nothing here canonicalises a path
//!
//! The lock follows **inode identity through the descriptor**: `flock` attaches
//! to the open file description, so two processes that opened the same directory
//! by different spellings — `/var/x` and `/private/var/x` on macOS, a relative
//! path and an absolute one, a route through a symbolic link — hold the *same*
//! lock without anything comparing their paths. Canonicalising here would buy
//! nothing and cost something visible: every path a read verb returns would come
//! back in a spelling the caller never used, so merely *adding* locking would
//! rewrite the library's output.
//!
//! # The directory is named `<root>/..`, and the kernel resolves it
//!
//! Not `Path::parent`. That is a lexical operation on a string, and `reading-k19`
//! disproved the claim that it converges: the accepted spelling `x/y/..` reads
//! the tree `x/y/..` — that is, `x` — while its lexical parent is `x/y`, which is
//! not the directory `x`'s own spelling locks. A final-component symbolic link
//! has the same shape. Two spellings of one tree took two locks, so a writer
//! through one did not exclude a reader through the other, and the intermediate
//! states a mutation is entitled to leave were observable.
//!
//! `read::containing_directory` therefore hands this module `<root>/..` and lets
//! the kernel resolve it: the root's own components first, symbolic links
//! followed, then one step to the directory that really contains it. That keeps
//! the no-canonicalisation rule intact — the path is still built from the
//! caller's spelling, character for character — while making the *lock* follow
//! the tree rather than the spelling.

use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;
use std::path::Path;
````
<!-- /fragment -->

`Mode` is present here because lock acquisition must translate the reader or
writer selected by `fs::read` and `fs::write` into the kernel's shared or
exclusive `flock` operation. The lock layer owns that mapping: a `Mode` becomes
one libc flag without changing the caller-spelled path, preserving the page's
invariant that cooperating readers may overlap while a writer excludes every
cooperating access. The worked write uses `Exclusive` before its snapshot is
read.

<!-- fragment «lock-modes» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/lock.rs" lines="41-58" parent="filesystem-lock-source" -->
````rust

/// Which lock to take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Shared: other readers may hold it at the same time.
    Shared,
    /// Exclusive: nothing else holds it while this does.
    Exclusive,
}

impl Mode {
    const fn operation(self) -> libc::c_int {
        match self {
            Self::Shared => libc::LOCK_SH,
            Self::Exclusive => libc::LOCK_EX,
        }
    }
}
````
<!-- /fragment -->

`take` performs the lock transition used by the worked write. The lock layer
opens the containing directory and combines that handle with the selected mode
to produce a `File` whose lifetime is the lock; interrupted waits retry, while
other operating-system failures return unchanged. This establishes that the
exclusive lock remains held across snapshot, decision, and application and is
released by dropping the guard rather than by a separate unlock path.

<!-- fragment «lock-take» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/lock.rs" lines="59-91" parent="filesystem-lock-source" -->
````rust

/// Take the lock on `directory`, blocking until it is available.
///
/// The returned [`File`] *is* the lock: `flock` is released when the last
/// descriptor of the open file description is closed, so holding the file is
/// holding the lock and dropping it releases it. There is no unlock call and no
/// unlock path to get wrong.
///
/// Blocking, with no way to ask for a refusal instead, because the architecture
/// document says consumers never mention locking — an API that offered
/// *try-lock* would be an API that mentioned it.
pub(crate) fn take(directory: &Path, mode: Mode) -> io::Result<File> {
    // A read-only open is enough: `flock` is advisory and attaches to the
    // descriptor, not to the file's contents, and a directory cannot be opened
    // for writing anyway.
    let handle = File::open(directory)?;
    let descriptor = handle.as_raw_fd();
    loop {
        // SAFETY: `descriptor` is open for the whole call — `handle` owns it and
        // is still alive — and `flock` touches nothing else.
        let result = unsafe { libc::flock(descriptor, mode.operation()) };
        if result == 0 {
            return Ok(handle);
        }
        let error = io::Error::last_os_error();
        // A signal delivered while waiting is not a failure to lock. Without
        // this the library would report a spurious error to a consumer whose
        // process merely received, say, a window-resize signal.
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}
````
<!-- /fragment -->

`take` retries `EINTR` because interruption while waiting does not mean that
the lock was acquired or refused. Other I/O failures become `Error::Io` in
`acquire`. There is no try-lock mode: locking is an internal concurrency rule,
not a consumer-selectable operation outcome.

<a id="write-guard"></a>
## One guard owns one mutation

`WriteGuard` stores the exclusive-lock descriptor, the caller-spelled root, and
the snapshot read under that lock. The descriptor field is never read; its
lifetime is the guard's ownership of the lock. The guard dereferences to the
snapshot, so a consumer may inspect the locked view before selecting a
mutation.

Every mutation method takes `self`, not `&mut self`. A successful operation
changes the filesystem described by the stored snapshot, so allowing another
operation through the same guard would plan from stale names and stale maximum
keys. Consuming the guard makes one acquisition correspond to one decision and
one interpreter run. `append_many` is the supported way to place several
entries under one snapshot and one rollback boundary.

<!-- fragment «filesystem-write-guard» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="305-388" parent="source-filesystem-module" -->
````rust
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
````
<!-- /fragment -->

`WriteGuard` owns access to the caller-spelled root and captured snapshot. This
fragment turns shared borrows of the guard into references to those unchanged
inputs, preserving the invariant that the worked insert plans from the snapshot
taken after its exclusive lock was acquired.

<!-- fragment «write-guard-accessors» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="535-547" parent="filesystem-write-guard-api" -->
````rust
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
````
<!-- /fragment -->

<a id="opening-lifecycle"></a>
## Opening and initialization lifecycle

`Reading` and `Writing` make presence part of the opening result. A reader gets
either a shared `ReadGuard` or an unguarded `Reading::Vacant`. A writer gets
either an exclusive `WriteGuard` or a `Vacancy` that still owns the exclusive
descriptor. `expect_tree` and `expect_vacancy` are explicit assertion helpers;
the ordinary control flow is exhaustive matching on these shapes.

<!-- fragment «filesystem-writing-shape» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="203-215" parent="source-filesystem-module" -->
````rust
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

````
<!-- /fragment -->

`Writing` retains whichever exclusive capability the opening established: a
live-tree guard or the vacancy that may create the root.

<!-- fragment «filesystem-writing-api» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="249-290" parent="source-filesystem-module" -->
````rust

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
````
<!-- /fragment -->

The vacancy API consumes that exclusive capability exactly once. Its plan uses
the ordinary initialization algebra, while its filesystem wrapper owns the
extra root create and root unwind that no named effect can represent.

<!-- fragment «filesystem-vacancy-api» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="389-520" parent="source-filesystem-module" -->
````rust

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

````
<!-- /fragment -->

`Vacancy::initialize` constructs its algebraic plan against `Snapshot::empty`
and validates every rendered name before creating the root. The root itself is
not a plan effect because it is a level rather than a named entry. If applying
the initial contents fails and unwinds cleanly, initialization removes the
empty root too. A failed root removal becomes
`FailedPartiallyRolledBack`; a concurrent outsider that already removed it is
treated as a completed unwind. The successful `Report` contains the optional
distinguished child and positioned entries, but no row for the unnamed root.

The write guard also dereferences to the snapshot. This is read-only access;
the methods that alter the tree consume the guard.

<!-- fragment «filesystem-write-deref» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="821-827" parent="source-filesystem-module" -->
````rust
impl<N: EntryName> core::ops::Deref for WriteGuard<N> {
    type Target = Snapshot<N>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}
````
<!-- /fragment -->

<a id="mutation-dispatch"></a>
## Public mutation to interpreter

The five planned public operations differ only in the algebra function they
call and its inputs. Each computes a `Decision` from `self.snapshot`, then calls
the private `run`. Before returning `Decision::Proceed`, the algebra passes its
constructed plan through `Plan::guarded`, which folds the effects through a
simulated state in order and refuses any destination occupied by the snapshot
or an earlier effect. A refusal crosses the filesystem boundary as
`Error::Refused` without invoking the interpreter. A proceeding, guarded plan
reaches `apply` while `self` still owns the exclusive lock. Dropping the
consumed guard releases the lock after `run` returns. This sequential plan
guard is distinct from `apply`'s later pre-effect check that every rendered name
is one path component. `delete` is the sixth live-tree mutation, but it does not
enter this dispatch because it has no algebraic plan.

`WriteGuard` owns the public append dispatch. This fragment turns a consumed
guard, target, and one or many new entries into one algebraic `Decision` and
then one `Report` or `Error`; guard consumption keeps one captured snapshot
behind one decision, while `append_many` supplies the page's multi-entry form
under a single rollback boundary alongside the worked insert.

<!-- fragment «write-guard-append» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="548-586" parent="filesystem-write-guard-api" -->
````rust

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

````
<!-- /fragment -->

The worked insert from the previous page plans its two highest-first moves and
one create from the captured snapshot, then applies that plan with production
fault injection disabled.

<!-- fragment «write-guard-insert» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="587-621" parent="filesystem-write-guard-api" -->
````rust
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

````
<!-- /fragment -->

Promotion documents the exceptional intermediate state at the public seam: the
new node and old leaf coexist between its create and move effects.

<!-- fragment «write-guard-promote» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="622-684" parent="filesystem-write-guard-api" -->
````rust
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

````
<!-- /fragment -->

Rewrite uses the same interpreter even when its source and destination path are
equal. The interpreter treats that move as a successful no-op and still records
the report entry.

<!-- fragment «write-guard-rewrite» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="685-730" parent="filesystem-write-guard-api" -->
````rust
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

````
<!-- /fragment -->

Deletion is the lifecycle operation on a live guard. It bypasses the algebraic
plan because it removes foreign entries as well as parsed names, but it remains
under the guard's exclusive lock.

<!-- fragment «write-guard-delete» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="731-798" parent="filesystem-write-guard-api" -->
````rust
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

````
<!-- /fragment -->

The private `WriteGuard::run` dispatch owns the boundary between pure algebra
and filesystem interpretation. This fragment turns `Decision::Refuse` into
`Error::Refused` or applies a guarded plan to produce `Report` or an application
`Error`, preserving total algebra without exposing a plan and carrying the
worked insert into its ordered effect trace.

<!-- fragment «write-guard-dispatch» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="799-812" parent="filesystem-write-guard-api" -->
````rust
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

````
<!-- /fragment -->

<a id="worked-apply-and-unwind"></a>
## Successful application and reverse unwind

The guarded three-effect insert plan from the previous page starts over the
orientation module level. The first trace applies its two moves and create to
produce a `Report`; the second fails before the create, reverses both landed
moves, and produces `Error::Failed` over the restored starting tree.

<a id="ordered-application"></a>
### Ordered application

`apply` first renders every effect name and checks that each is exactly one
path component. Snapshot names passed the equivalent check during reading; this
second pass covers names newly composed by the algebra. It completes before
any effect runs, so a bad plan name returns `NameIsNotOneComponent` without
requiring rollback.

The interpreter then creates one `Run`. Its `landed` vector maps
`Level::Created(effect_index)` to the path produced by that earlier effect.
`moved` overrides snapshot paths for an entry moved more than once in one plan.
`undo` records inverse actions in landing order, and `report` records public
outcomes in the same forward sequence.

The interpreter module owns the common application and rollback contract. This
fragment turns a root, captured snapshot, and guarded plan into either an
ordered `Report` or an `Error` after reverse unwind, establishing bounded
plan-level recovery and exclusive destination claims for both the successful
insert and its failed-forward trace.

<!-- fragment «apply-contract» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="1-49" parent="filesystem-interpreter-source" -->
````rust
//! The interpreter: the one component that applies a plan and unwinds what it
//! applied.
//!
//! One interpreter and one rollback, shared by every operation. That is the
//! whole reason a plan is a *value* rather than five hand-written procedures:
//! five unwinds are five things that drift apart, and atomicity becomes five
//! properties instead of one. If an operation ever needs a different rollback,
//! `ARCHITECTURE.md` is explicit that this is a finding about the plan shape and
//! not a licence to add a second interpreter.
//!
//! The specification is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections *How an
//! operation runs*, *When rollback fails* and the invariant *Plan atomicity*;
//! the model is `operations.qnt`'s `applyStep…`/`unwindStep…` actions and its
//! `failures` and `rollback_fails` instances.
//!
//! # The promise is bounded, and the bound is in the type
//!
//! Rollback covers **reported errors**. A process killed mid-apply is not
//! recoverable and the library says so rather than implying otherwise, and a
//! rollback that *itself* fails leaves the tree in neither the state it was
//! found in nor the one intended — which is [`Error::FailedPartiallyRolledBack`]
//! and not [`Error::Failed`]. Two variants, because a consumer that cannot tell
//! them apart has been promised something the library does not do: on the
//! promotion path the second one leaves a leaf and a node sharing an ordinal and
//! a key, and that is the single path by which this library damages a tree it
//! was handed.
//!
//! # Every destination is claimed, not assumed
//!
//! A `create_new` for a leaf, a `create_dir` for a node, and — for a rename,
//! which `rename(2)` would otherwise perform *over* whatever is there — an
//! explicit unfollowed look before the call. The algebra already folded the plan
//! through the snapshot, so under the lock this can only fire on a plan that
//! collides with itself, and `operations.qnt`'s
//! `inv_interpreterNeverFindsADestinationTaken` says it never does. It stays
//! because **the lock is advisory**: a writer that does not take it can occupy a
//! destination between the snapshot and the apply, and that neighbour is the
//! only thing left that this check catches. Without this paragraph the check
//! reads as dead code to whoever next tidies up.

use std::fs;
use std::io;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::plan::{Effect, Level, Plan};
use crate::report::Report;
use crate::{EntryName, EntryNameExt, Error, Snapshot, Species};

````
<!-- /fragment -->

`apply` is the interpreter's entry point for the worked insert. It receives the
caller-spelled root, the locked snapshot, and the algebra's guarded plan; it
first refuses any rendering that is not one path component, then turns the
ordered effects into either the accumulated `Report` or an `Error` produced by
unwind. The preflight loop preserves the no-partial-application invariant for a
bad rendered name, while the effect loop preserves plan order for the two moves
and final create.

<!-- fragment «apply-plan» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="50-105" parent="filesystem-interpreter-source" -->
````rust
/// Apply a plan under the exclusive lock, or leave the tree as it was found.
pub(super) fn apply<N: EntryName>(
    root: &Path,
    snapshot: &Snapshot<N>,
    plan: &Plan<N>,
    faults: Faults,
) -> Result<Report<N>, Error<N>> {
    names_are_one_component(root, plan)?;
    let mut run = Run {
        root,
        snapshot,
        faults,
        landed: Vec::new(),
        moved: Vec::new(),
        undo: Vec::new(),
        report: Report::empty(),
    };
    for (index, effect) in plan.effects().iter().enumerate() {
        if let Err(failure) = run.step(index, effect) {
            return Err(run.unwind(failure));
        }
    }
    Ok(run.report)
}

/// The seventh obligation, at the second of the two boundaries where a name
/// becomes a path.
///
/// Run **before any effect does**, so a plan carrying one bad name changes
/// nothing rather than landing what it can and unwinding. The snapshot's own
/// names were checked when it was read, so between the two checks every
/// rendering [`apply`] will join is one path component.
///
/// Separate from [`apply`] — which still calls it, and is the only path most
/// operations take — because [`Vacancy::initialize`] creates the tree root
/// before applying anything, and a plan refused after that would leave an empty
/// root behind while reporting an error that promises nothing changed.
///
/// [`Vacancy::initialize`]: super::Vacancy::initialize
pub(super) fn names_are_one_component<N: EntryName>(
    root: &Path,
    plan: &Plan<N>,
) -> Result<(), Error<N>> {
    for effect in plan.effects() {
        let rendered = effect.name().to_string();
        if let Some(reason) = crate::name::not_one_component(&rendered) {
            return Err(Error::NameIsNotOneComponent {
                root: root.to_path_buf(),
                rendered,
                reason,
            });
        }
    }
    Ok(())
}

````
<!-- /fragment -->

`Run` holds the mutable state for that single application. The interpreter owns
the current destinations of landed and repeatedly moved entries, the inverse
actions captured before each effect, and the report built from successful
steps. Those collections turn the plan and snapshot into forward progress that
can be unwound in reverse without consulting a changed directory, which is the
state needed for both the page's successful insert and its failure trace.

<!-- fragment «apply-run-state» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="106-123" parent="filesystem-interpreter-source" -->
````rust
/// One application of one plan.
struct Run<'a, N> {
    root: &'a Path,
    snapshot: &'a Snapshot<N>,
    faults: Faults,
    /// The destination of each effect that has landed, by its position in the
    /// plan — which is what [`Level::Created`] names.
    landed: Vec<PathBuf>,
    /// Where each entry this run has moved now lives, so that a plan moving one
    /// entry twice reads its *current* path rather than the snapshot's stale
    /// one.
    moved: Vec<(usize, PathBuf)>,
    /// How to undo what has landed, in the order it landed — captured against
    /// the state before each effect, and walked backwards.
    undo: Vec<Undo>,
    report: Report<N>,
}

````
<!-- /fragment -->

For the successful worked insert, `Run::step` receives these effects in written
order:

```text
0. Move key 6: 03-draft-matrices-i6.md -> 04-draft-matrices-i6.md
1. Move key 5: 02-published-vectors-i5.md -> 03-published-vectors-i5.md
2. Create key 7: 02-draft-limits-i7.md
```

Each move resolves its current source path, checks the destination without
following symbolic links, renames it, registers `Undo::Restore`, and records a
rename. The create uses `create_new` for the lesson file, registers
`Undo::Remove` immediately after claiming the path, writes the bytes, and
records a creation. `apply` returns the accumulated report only after all three
steps succeed.

The resulting level is:

```text
02-linear-algebra-i2/
├── OVERVIEW.md
├── 01-published-foundations-i3.md
├── 02-draft-limits-i7.md
├── 03-published-vectors-i5.md
└── 04-draft-matrices-i6.md
```

The report preserves both per-effect-species order and the complete landing
order:

```text
renamed():
  key 6: 03-draft-matrices-i6.md -> 04-draft-matrices-i6.md
  key 5: 02-published-vectors-i5.md -> 03-published-vectors-i5.md
created():
  key 7: 02-draft-limits-i7.md
paths():
  02-linear-algebra-i2/04-draft-matrices-i6.md
  02-linear-algebra-i2/03-published-vectors-i5.md
  02-linear-algebra-i2/02-draft-limits-i7.md
```

<a id="effect-steps"></a>
### Effect-specific steps and paths

`Create` obtains exclusive ownership of its destination through the creation
syscall itself. `create_dir` refuses an occupied node path. `OpenOptions` with
`create_new(true)` provides the corresponding exclusive file claim. The file
undo is registered before `write_all`; a short or failed write therefore removes
the partial file during unwind.

`MoveTo` requires an explicit vacancy check because `rename` would otherwise
replace an occupied destination. The check uses unfollowed metadata, so a
symbolic link, including a dangling link, occupies the name. The look and rename
are not one atomic syscall on the supported platforms. A writer that ignores
the advisory lock may still win that interval, but a destination already taken
when the look runs is never overwritten.

A move whose current path equals its destination is the rewrite no-op. It
claims nothing and registers no undo because no filesystem change occurred,
but it updates the run's current-path table and the public report.

<!-- fragment «apply-effect-step» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="124-228" parent="filesystem-interpreter-source" -->
````rust
impl<N: EntryName> Run<'_, N> {
    /// Apply one effect, recording how to undo it the moment its destination is
    /// claimed.
    fn step(&mut self, index: usize, effect: &Effect<N>) -> Result<(), Failure> {
        let landed = match effect {
            Effect::Create { at, name, content } => {
                let path = self.level_path(*at).join(name.to_string());
                self.faults.strike_effect(index, &path)?;
                match name.species() {
                    // `create_dir` fails rather than succeeding when something
                    // is already there, which is the exclusive claim.
                    Species::Node => {
                        fs::create_dir(&path).map_err(|source| Failure {
                            path: path.clone(),
                            doing: "creating the node",
                            source,
                        })?;
                        self.undo.push(Undo::Remove {
                            path: path.clone(),
                            species: Species::Node,
                        });
                    }
                    // `create_new` is the same claim for a regular file: it is
                    // one syscall with `O_EXCL`, so nothing can slip between the
                    // question and the answer.
                    Species::Leaf | Species::Distinguished => {
                        let mut file = fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&path)
                            .map_err(|source| Failure {
                                path: path.clone(),
                                doing: "creating the leaf",
                                source,
                            })?;
                        // Registered before the bytes are written, deliberately:
                        // from here the path is this run's to remove, and a
                        // write that fails half way must still be unwound.
                        self.undo.push(Undo::Remove {
                            path: path.clone(),
                            species: Species::Leaf,
                        });
                        // And this is the control on that *deliberately*. The
                        // ordering above is a second mechanism behind atomicity,
                        // and `strike_effect` above cannot reach it: it fires
                        // before the create, so moving the registration below the
                        // write leaves every whole-effect test green while a real
                        // short write returns `Error::Failed` over a partial file
                        // the variant promises was removed. This seam stands in
                        // for that write, in the one interval where the file
                        // exists and its bytes do not.
                        self.faults.strike_content(index, &path)?;
                        file.write_all(content).map_err(|source| Failure {
                            path: path.clone(),
                            doing: "writing the leaf's content",
                            source,
                        })?;
                    }
                }
                self.report.record_created(name.clone(), path.clone());
                path
            }
            Effect::MoveTo { entry, to, name } => {
                let from = self.entry_path(*entry);
                let path = self.level_path(*to).join(name.to_string());
                self.faults.strike_effect(index, &path)?;
                // A move onto the entry's own path, which is what a `rewrite` to
                // the parts an entry already carries plans. The algebra excludes
                // the mover from occupancy for exactly this — `operations.qnt`'s
                // `wit_rewriteToSameParts` says the no-op must **succeed** — and
                // that exclusion has to be carried across the boundary or the
                // interpreter refuses the plan the algebra just proved
                // applicable. One property, two mechanisms; this is the second.
                //
                // Nothing is claimed and nothing is undone, and both follow from
                // the same fact: the destination is the source, so it is occupied
                // by the very entry being moved, and `rename(2)` on one path is
                // defined to change nothing. An `Undo::Restore` here would be a
                // rename onto an occupied path — its own — which `claim_vacant`
                // would refuse, turning a clean unwind into
                // `FailedPartiallyRolledBack`.
                let noop = from == path;
                if !noop {
                    claim_vacant(&path, "renaming onto")?;
                    fs::rename(&from, &path).map_err(|source| Failure {
                        path: path.clone(),
                        doing: "renaming the entry to",
                        source,
                    })?;
                    self.undo.push(Undo::Restore {
                        from: path.clone(),
                        to: from.clone(),
                    });
                }
                self.moved.push((*entry, path.clone()));
                // Reported either way. The operation did place this name, and a
                // consumer reading `renamed()` to learn where an entry now lives
                // needs the answer whether or not the filesystem was touched.
                self.report.record_renamed(name.clone(), from, path.clone());
                path
            }
        };
        self.landed.push(landed);
        Ok(())
    }
````
<!-- /fragment -->

Path resolution is stateful within a run. Snapshot entries supply initial
paths, `moved` supplies the most recent path of a moved entry, and `landed`
supplies paths for levels created earlier in the plan. This is how promotion's
second effect can address the directory created by its first.

<!-- fragment «apply-unwind-and-paths» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="229-288" parent="filesystem-interpreter-source" -->
````rust

    /// Undo what landed, in reverse, and say which of the two failures this was.
    fn unwind(self, failure: Failure) -> Error<N> {
        for (index, step) in self.undo.into_iter().rev().enumerate() {
            if let Err(unwound) = step.perform(index, self.faults) {
                return Error::FailedPartiallyRolledBack {
                    path: failure.path,
                    doing: failure.doing,
                    source: failure.source,
                    unwinding: unwound.path,
                    undoing: unwound.doing,
                    unwind_source: unwound.source,
                };
            }
        }
        Error::Failed {
            path: failure.path,
            doing: failure.doing,
            source: failure.source,
        }
    }

    /// The directory a plan's [`Level`] names.
    ///
    /// # Panics
    ///
    /// If the level names an effect that has not landed, or one that created no
    /// directory. Neither is reachable through this crate's operations: a plan
    /// is built in order, so a `Created` level always names an earlier effect.
    fn level_path(&self, level: Level) -> PathBuf {
        match level {
            Level::Root => self.root.to_path_buf(),
            Level::Entry(index) => self.entry_path(index),
            Level::Created(effect) => self
                .landed
                .get(effect)
                .expect("a plan names only levels its earlier effects created")
                .clone(),
        }
    }

    /// Where an entry of the snapshot is *now*: the caller's spelling of the
    /// root, then every containing node's name, then its own — unless this run
    /// has already moved it, in which case where it moved it to.
    fn entry_path(&self, index: usize) -> PathBuf {
        if let Some((_, path)) = self.moved.iter().rev().find(|(moved, _)| *moved == index) {
            return path.clone();
        }
        let entry = self.snapshot.at(index);
        let mut path = self.root.to_path_buf();
        for container in entry.ancestors() {
            if let Some(node) = container.entry() {
                path.push(node.name().to_string());
            }
        }
        path.push(entry.name().to_string());
        path
    }
}

````
<!-- /fragment -->

<a id="rollback"></a>
### Forward failure and reverse unwind

Every landed create records `Undo::Remove`; every landed non-no-op move records
`Undo::Restore`. `Run::unwind` consumes that list in reverse. Reverse order is
required because later effects may depend on earlier ones. Promotion, for
example, restores the moved leaf out of the new node before removing that node.
Interpreter rollback can remove only a path created by this run. There is no
public mutation operation for removing one entry from an existing tree; the
separate `delete` boundary removes the whole tree without entering the
plan/effect algebra.

The insert trace can be failed at effect index 2, immediately before the final
create. At that point both highest-first moves have landed:

```text
02-linear-algebra-i2/
├── OVERVIEW.md
├── 01-published-foundations-i3.md
├── 03-published-vectors-i5.md
└── 04-draft-matrices-i6.md
```

The undo list contains the restoration of key 6 followed by the restoration of
key 5. Reverse traversal first moves key 5 from ordinal 3 back to ordinal 2,
then moves key 6 from ordinal 4 back to ordinal 3. If both restores succeed,
every filesystem change this run made has been reversed when `Error::Failed`
returns. In the absence of a writer outside the locking protocol, the original
tree is therefore present. The error identifies the create destination and
forward action that failed, and its message states that every landed effect was
undone.

<!-- fragment «apply-undo» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="289-348" parent="filesystem-interpreter-source" -->
````rust
/// How to undo one effect that landed.
///
/// Two variants, and the first is the only removal this library ever performs.
/// `operations.qnt` gives `Effect` a `Remove` variant and then records in a
/// comment that it *never appears in a forward plan*; here that comment is the
/// type system's, because a `Remove` can only be built from a `Create` this run
/// just performed. The model's `inv_rollbackRemovesOnlyItsOwn` — *rollback
/// removes only entries the run itself created, so it cannot destroy something
/// that was already there* — is therefore structural rather than checked at
/// run time.
enum Undo {
    /// Remove something this run created, one moment ago.
    Remove {
        /// Exactly the path the create claimed.
        path: PathBuf,
        /// Which of the two removals it takes.
        species: Species,
    },
    /// Put an entry this run moved back where it was.
    Restore {
        /// Where the run left it.
        from: PathBuf,
        /// Where it was found.
        to: PathBuf,
    },
}

impl Undo {
    fn perform(self, index: usize, faults: Faults) -> Result<(), Failure> {
        match self {
            Self::Remove { path, species } => {
                faults.strike_unwind(index, &path)?;
                // A directory this run created can only hold things this run
                // put in it, and those were unwound first — so it is empty, and
                // `remove_dir` refusing a non-empty one is a check rather than
                // an obstacle.
                let removal = if species == Species::Node {
                    fs::remove_dir(&path)
                } else {
                    fs::remove_file(&path)
                };
                removal.map_err(|source| Failure {
                    path,
                    doing: "removing what this operation had created at",
                    source,
                })
            }
            Self::Restore { from, to } => {
                faults.strike_unwind(index, &to)?;
                claim_vacant(&to, "restoring")?;
                fs::rename(&from, &to).map_err(|source| Failure {
                    path: to,
                    doing: "putting back the entry this operation had moved from",
                    source,
                })
            }
        }
    }
}

````
<!-- /fragment -->

The vacancy check is used both before a forward rename and before a restoring
rename. A failed restore therefore becomes a distinct partial-rollback outcome
instead of overwriting a path that appeared during the run.

<!-- fragment «apply-destination-claim» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="349-382" parent="filesystem-interpreter-source" -->
````rust
/// Refuse a destination that is occupied by anything at all, deciding it
/// **without following links**.
///
/// `rename(2)` replaces its destination silently, so a rename is the one effect
/// whose claim cannot be the syscall itself. macOS has no portable no-replace
/// rename, so the claim is a look followed by the call — which is not atomic
/// against a racing writer, and does not need to be: the writer it would have to
/// beat is one that ignores the advisory lock, which is already outside what the
/// library defends against. What this does defend is the common case that
/// matters, a destination that was occupied *before* the rename started.
///
/// An occupancy that cannot be determined is a refusal rather than an
/// assumption, which is why anything but `NotFound` is a failure.
fn claim_vacant(path: &Path, doing: &'static str) -> Result<(), Failure> {
    // `symlink_metadata` and not `metadata`: a symbolic link occupies a name
    // whatever it points at, and a dangling one occupies it too.
    match fs::symlink_metadata(path) {
        Ok(_) => Err(Failure {
            path: path.to_path_buf(),
            doing,
            source: io::Error::new(
                io::ErrorKind::AlreadyExists,
                "something is already there, and this operation will not replace it",
            ),
        }),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(Failure {
            path: path.to_path_buf(),
            doing,
            source,
        }),
    }
}

````
<!-- /fragment -->

<a id="partial-rollback"></a>
## Failed unwind and repair

`Faults` is an internal test seam, not consumer configuration. Production calls
use three `None` positions. Tests can fail one forward effect, the content write
inside a create, or one forward effect followed by one unwind step. The content
position verifies the narrow interval after a file is exclusively created and
before its bytes are complete.

The promotion failure trace uses two effects: create the node with the leaf's
ordinal and key, then move the leaf into it as the distinguished child. A
forward failure at effect 1 leaves the empty node created by effect 0. Normally
unwind step 0 removes that node and returns `Error::Failed` over the original
tree. If that removal also fails, `Error::FailedPartiallyRolledBack` returns and
both entries remain:

```text
01-draft-first-i1.md   # original leaf, ordinal 1, key 1
01-topology-i1/        # empty new node, ordinal 1, key 1
```

This state is neither the input nor the intended promoted tree. The error
message gives the mechanical repair for this exact promotion state: when the
node has no distinguished child, remove either the leaf or the node. Removing
the node restores the input; removing the leaf keeps the new empty node. A
consumer must not retry blindly because key lookup on the damaged tree is
ambiguous and a later plan can be refused by a destination the wreckage already
occupies. One half must be removed before another mutation is attempted.

<!-- fragment «apply-fault-seam» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="383-488" parent="filesystem-interpreter-source" -->
````rust
/// One effect, or one undo, that did not happen.
struct Failure {
    path: PathBuf,
    doing: &'static str,
    source: io::Error,
}

/// The seam that makes an effect fail on demand.
///
/// **Internal, and it must stay internal.** A second *public* seam would
/// contradict `docs/adr/entry-name-is-the-only-seam.md`, which is the record
/// saying the entry name is the only genericity this library has. It is here
/// because atomicity is otherwise untestable: the property is *after a mutation
/// returns an error, either every effect landed or none did*, and without a way
/// to make effect two of three fail there is no error to observe.
///
/// A production build carries the three `None`s and the branches that read
/// them, and nothing else — the constructors below are compiled only under
/// `cfg(test)`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Faults {
    effect: Option<usize>,
    content: Option<usize>,
    unwind: Option<usize>,
}

impl Faults {
    /// What every operation runs with.
    pub(crate) const fn none() -> Self {
        Self {
            effect: None,
            content: None,
            unwind: None,
        }
    }

    /// Fail the effect at this position in the plan, before it has touched
    /// anything.
    #[cfg(test)]
    pub(crate) const fn at_effect(index: usize) -> Self {
        Self {
            effect: Some(index),
            content: None,
            unwind: None,
        }
    }

    /// Fail the create at this position in the plan **after** its destination
    /// has been claimed exclusively and before its bytes are written — the one
    /// interval in which a leaf exists on disk and its content does not.
    ///
    /// A whole-effect failure cannot reach it, which is the point: the
    /// registration of the undo sits inside that interval, so it is a mechanism
    /// with no control in front of it otherwise.
    #[cfg(test)]
    pub(crate) const fn at_content(index: usize) -> Self {
        Self {
            effect: None,
            content: Some(index),
            unwind: None,
        }
    }

    /// Fail the effect at this position in the plan, and then the unwind step at
    /// this position in the rollback — the rollback runs backwards, so step 0 is
    /// the undo of the *last* effect that landed.
    #[cfg(test)]
    pub(crate) const fn at_effect_and_unwind(effect: usize, unwind: usize) -> Self {
        Self {
            effect: Some(effect),
            content: None,
            unwind: Some(unwind),
        }
    }

    fn strike_effect(self, index: usize, path: &Path) -> Result<(), Failure> {
        Self::strike(self.effect, index, path, "applying an effect to")
    }

    fn strike_content(self, index: usize, path: &Path) -> Result<(), Failure> {
        Self::strike(self.content, index, path, "writing the leaf's content to")
    }

    fn strike_unwind(self, index: usize, path: &Path) -> Result<(), Failure> {
        Self::strike(self.unwind, index, path, "unwinding an effect at")
    }

    fn strike(
        at: Option<usize>,
        index: usize,
        path: &Path,
        doing: &'static str,
    ) -> Result<(), Failure> {
        if at != Some(index) {
            return Ok(());
        }
        Err(Failure {
            path: path.to_path_buf(),
            doing,
            source: io::Error::other("a failure injected by this crate's own tests"),
        })
    }
}

#[cfg(test)]
mod tests;
````
<!-- /fragment -->

<a id="whole-tree-deletion"></a>
## Whole-tree deletion

Deletion traverses the filesystem rather than the snapshot because it removes
everything under the root, including foreign entries that have no `EntryName`.
It first refuses an indirect root spelling: the final component must be a real
name rather than a symbolic link, `.` or a separator, and no `..` may cancel a
preceding name. These checks run before the first unlink, so
`RootIsNotSpelledDirectly` means nothing was removed.

The removal uses an explicit worklist. It schedules children in reverse sorted
order so popping restores deterministic listing order, removes each level only
after its children, and removes the root last. Symbolic links encountered
inside the tree are unlinked and never descended into. The look and descent are
separate syscalls, so a writer that ignores the advisory lock can still replace
a directory in that interval; this is the same external-writer boundary as the
interpreter's pre-rename vacancy check.

<!-- fragment «remove-contract» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/remove.rs" lines="1-64" parent="filesystem-removal-source" -->
````rust
//! Removing a tree root and everything beneath it.
//!
//! The one mutation that is not a [`Plan`](crate::plan::Plan), and the reason is
//! that a plan is a list of effects over *names* — values the domain produced
//! and the algebra can reason about. A deletion acts on the root, so it acts on
//! everything that is there, including the entries the domain declined to parse
//! as its own. Those have no name in the algebra's sense and never appear in a
//! [`Snapshot`](crate::Snapshot), so the removal reads the directories itself.
//!
//! # There is nothing to unwind
//!
//! Every other operation's failure path is *put back what this run did*. A
//! removal has nothing to put back: an unlinked file is gone, and a library that
//! wanted one back would have had to copy it aside first — which is the staging
//! machinery this design does not have and does not want. So the failure is
//! reported as [`Error::RemovalStopped`], which claims neither of the two things
//! [`Error::Failed`] and [`Error::FailedPartiallyRolledBack`] claim: it says how
//! far the removal got, and stops.
//!
//! # Following no link is the security property
//!
//! Descent is decided by [`read::listing`](super::read::listing), whose
//! `Found` comes from `DirEntry::file_type` — `symlink_metadata`, not
//! `metadata`. A symbolic link naming a directory is therefore [`Found::Other`]
//! and is *unlinked*, never descended into, so a link pointing outside the tree
//! costs its target nothing. Nothing here follows a link **inside the tree** —
//! and the qualifier is load-bearing, not hedging: the root's own last component
//! is named by the caller rather than found by the walk, and it is the next
//! paragraph's subject.
//!
//! **The root's own last component is the exception, and it is refused rather
//! than followed.** Everything above is about the entries *inside* the tree; the
//! root itself is named by the caller, and every other operation lets the kernel
//! resolve that name — a link naming a directory is an accepted spelling of the
//! tree. A deletion cannot accept it, because it acts on the root as an object
//! rather than as a container. See [`spelled_directly`].
//!
//! **The bound on the no-link claim, stated rather than hidden.** The look and
//! the descent are two syscalls, so a hand that replaces a directory with a link
//! *between* them is not defeated by this — the same window
//! [`claim_vacant`](super::apply) already names, and the same neighbour: a
//! writer that ignores the advisory lock. `std::fs::remove_dir_all` closes it
//! with `openat`-based descent and reports nothing about what it removed, which
//! is the whole of what this operation is for; a consumer that wants the race
//! closed and no report has the standard library's own call.

use std::fs;
use std::path::{Component, Path, PathBuf};

use super::read;
use crate::{EntryName, Error, Found, Removed};

/// Remove everything beneath the root, then the root.
///
/// Post-order: a level goes only after everything it holds, because a directory
/// cannot be removed while anything is in it. Within a level the order is the
/// listing's own — sorted — for determinism and for nothing else.
///
/// **No order here buys a property, and that is worth saying beside the shift
/// rule, which does.** A sibling shift runs highest-first so that an interrupted
/// run leaves a merely *gapped* level, a shape this design admits everywhere. An
/// interrupted removal leaves a tree with entries missing and its key maximum
/// lowered, which is not a shape this design admits in any order — so the order
/// is chosen to be reproducible rather than to be safe.
````
<!-- /fragment -->

The removal entry point validates the caller's root spelling, executes the
post-order worklist, and accumulates each removed child path before deleting
the root itself.

<!-- fragment «remove-tree» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/remove.rs" lines="65-106" parent="filesystem-removal-source" -->
````rust
pub(super) fn tree<N: EntryName>(root: &Path) -> Result<Removed, Error<N>> {
    // Before anything goes, so a refused spelling removes nothing at all.
    spelled_directly(root)?;
    let mut entries: Vec<PathBuf> = Vec::new();
    // An explicit worklist rather than recursion, for the reason
    // `read::snapshot` gives: the depth of a tree on disk is the operator's to
    // choose, and a stack overflow is not a refusal any consumer can handle.
    let mut pending = Vec::new();
    // The root's own children, and not `descend` — the root's removal is the
    // last line of this function rather than a step on the worklist, because it
    // is reported on its own field.
    if let Err(stopped) = children(root, &mut pending) {
        return Err(stopped.error(root, entries));
    }
    while let Some(step) = pending.pop() {
        let outcome = match &step {
            Step::Descend(directory) => descend(directory, &mut pending),
            Step::Unlink(path) => unlink(path),
            Step::RemoveLevel(path) => remove_level(path),
        };
        if let Err(stopped) = outcome {
            return Err(stopped.error(root, entries));
        }
        match step {
            // Descending removes nothing; what it found is on the worklist.
            Step::Descend(_) => {}
            Step::Unlink(path) | Step::RemoveLevel(path) => entries.push(path),
        }
    }
    // The root is not one of the entries — it is the level they were in, and it
    // has no name the domain ever parsed. It goes last because nothing else
    // could go after it, and it is reported on its own field for the same
    // reason `initialize` puts no row in the report for creating it.
    if let Err(stopped) = remove_level(root) {
        return Err(stopped.error(root, entries));
    }
    Ok(Removed {
        root: root.to_path_buf(),
        entries,
    })
}

````
<!-- /fragment -->

`spelled_directly` bounds deletion to a stable, directly named root object. It
rejects ambiguous or self-invalidating spellings before the worklist starts.

<!-- fragment «remove-spelling-guard» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/remove.rs" lines="107-188" parent="filesystem-removal-source" -->
````rust
/// The one precondition a deletion has that no other operation does: the root's
/// spelling must **name the root**, and must stay resolvable while the root is
/// being taken apart.
///
/// # Why this operation and no other
///
/// Everywhere else the root is a **container**, and the kernel resolving its
/// last component is exactly right — `read::containing_directory` goes to some
/// trouble to make a symbolic link naming a directory, and a spelling ending in
/// `..`, reach the same tree and take the same lock as the direct spelling.
/// This is the one operation that acts on the root as an **object**, and there
/// the last component decides *which* object. A link and what it names are two
/// things, and destroying the second while leaving the first is not what either
/// answer would have been.
///
/// It is also the one operation that removes the components a path is built
/// from. `syllabus/topic/..` names the tree perfectly well until the walk
/// removes `topic`, after which every path built on that spelling stops
/// resolving — with the tree half destroyed and nothing left that can finish
/// the job.
///
/// # The two conditions, and why they are between them complete
///
/// **No `..` may cancel a name.** With that rule, every component before the
/// last resolves to a strict *ancestor* of the root, and the walk removes only
/// what is at or below the root — so no component of the spelling is one the
/// removal can take away. A **leading** `..` cancels nothing and is fine:
/// `../course` is an ordinary spelling and is accepted.
///
/// The rule is **coarser than the danger, deliberately**. A `..` cancelling a
/// component *above* the tree — `/a/../b/course` — is harmless, and is refused
/// with the rest of the class. Telling the two apart means resolving the path to
/// learn which components lie inside the tree, and this module resolves nothing;
/// the cost of the coarse rule is one message asking for a direct spelling, and
/// the cost of the precise one is a resolution step on the destructive path.
///
/// **The last component must be a name, and must not be a symbolic link.** That
/// leaves exactly one object it can mean. `.` and a bare separator name no
/// object to remove at all — `rmdir(".")` is `EINVAL`, and refusing it with a
/// sentence beats passing that through.
///
/// Both are decided on a path rebuilt from its own components, because a
/// trailing separator makes `symlink_metadata` resolve the final link — which
/// is how `link/` would otherwise slip past the very check that exists to catch
/// `link`.
fn spelled_directly<N: EntryName>(root: &Path) -> Result<(), Error<N>> {
    let mut previous_was_a_name = false;
    for component in root.components() {
        if previous_was_a_name && component == Component::ParentDir {
            return Err(Error::RootIsNotSpelledDirectly {
                root: root.to_path_buf(),
                reason: "descends into a name and comes back out through `..`, so one of its \
                         own components is something this removal would take \
                         away — after which nothing else under the root \
                         resolves, and the tree is left half gone with no \
                         spelling that can finish it",
            });
        }
        previous_was_a_name = matches!(component, Component::Normal(_));
    }
    let direct: PathBuf = root.components().collect();
    if !matches!(direct.components().next_back(), Some(Component::Normal(_))) {
        return Err(Error::RootIsNotSpelledDirectly {
            root: root.to_path_buf(),
            reason: "does not end in a name, so its last component names no object \
                     this operation could remove",
        });
    }
    let here = fs::symlink_metadata(&direct).map_err(|source| Error::Io {
        path: root.to_path_buf(),
        doing: "looking at the tree root",
        source,
    })?;
    if here.file_type().is_symlink() {
        return Err(Error::RootIsNotSpelledDirectly {
            root: root.to_path_buf(),
            reason: "is a symbolic link, and a link is not the tree it names",
        });
    }
    Ok(())
}

````
<!-- /fragment -->

The worklist distinguishes descent, unlinking, and empty-directory removal.
Each failure becomes a `Stopped` value that is reframed with the root and the
already removed paths as `Error::RemovalStopped`.

<!-- fragment «remove-worklist-and-failure» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/remove.rs" lines="189-275" parent="filesystem-removal-source" -->
````rust
/// One thing left to do.
enum Step {
    /// List this directory and schedule what is in it, then remove it.
    Descend(PathBuf),
    /// Remove something that is not a directory — a file, a socket, or a
    /// symbolic link **as a link**.
    Unlink(PathBuf),
    /// Remove a directory this walk has already emptied.
    RemoveLevel(PathBuf),
}

/// Schedule a directory's own removal, and then its contents.
///
/// Pushing the level's removal **first** is what makes the worklist post-order:
/// everything pushed after it pops before it.
fn descend(directory: &Path, pending: &mut Vec<Step>) -> Result<(), Stopped> {
    pending.push(Step::RemoveLevel(directory.to_path_buf()));
    children(directory, pending)
}

/// Schedule what is in a directory, in the listing's own sorted order.
///
/// The children go on in *reverse* of that order so that popping restores it,
/// which is the same trick — and the same reason — as `read::snapshot`'s.
fn children(directory: &Path, pending: &mut Vec<Step>) -> Result<(), Stopped> {
    let found = read::listing(directory).map_err(Stopped::from_unlistable)?;
    for (name, found) in found.into_iter().rev() {
        let path = directory.join(name);
        pending.push(match found {
            Found::Dir => Step::Descend(path),
            // A symbolic link — even one naming a directory — is `Found::Other`,
            // because the listing did not follow it. Unlinking is what removes
            // the link and leaves whatever it named alone.
            Found::File | Found::Other => Step::Unlink(path),
        });
    }
    Ok(())
}

fn unlink(path: &Path) -> Result<(), Stopped> {
    fs::remove_file(path).map_err(|source| Stopped {
        path: path.to_path_buf(),
        doing: "removing",
        source,
    })
}

fn remove_level(path: &Path) -> Result<(), Stopped> {
    // `remove_dir` and never `remove_dir_all`: this walk has already emptied the
    // directory, so a refusal here is a check — something arrived under it while
    // the removal was running — rather than an obstacle to route around.
    fs::remove_dir(path).map_err(|source| Stopped {
        path: path.to_path_buf(),
        doing: "removing the directory",
        source,
    })
}

/// One removal, or one listing, that did not happen.
struct Stopped {
    path: PathBuf,
    doing: &'static str,
    source: std::io::Error,
}

impl Stopped {
    fn from_unlistable(unlistable: read::Unlistable) -> Self {
        Self {
            path: unlistable.path,
            doing: unlistable.doing,
            source: unlistable.source,
        }
    }

    /// The removal's framing of a failure: what stopped it, and how far it had
    /// got. `removed` is moved in rather than counted, because a caller that has
    /// to say what it destroyed needs the paths and not the number.
    fn error<N: EntryName>(self, root: &Path, removed: Vec<PathBuf>) -> Error<N> {
        Error::RemovalStopped {
            root: root.to_path_buf(),
            path: self.path,
            doing: self.doing,
            source: self.source,
            removed,
        }
    }
}
````
<!-- /fragment -->

`Removed` separates the caller-spelled root from the child paths and records
children before their containing levels. The order is reproducible, not a
safety guarantee. If a listing, unlink, or directory removal fails,
`RemovalStopped` carries the failing path and operation plus every path already
removed. An empty list proves the tree is unchanged; a non-empty list means the
tree is in neither its initial nor fully deleted state. No undo is attempted,
because unlinked bytes were not staged anywhere from which they could be
restored.

<a id="intermediate-states"></a>
## Intermediate states and concurrency limits

Multi-effect operations are not filesystem transactions. While the exclusive
lock is held, cooperating library calls cannot observe these states:

- an insert after some highest-first shifts has a gap and distinct ordinals;
- append-many may contain a proper prefix of its requested entries;
- a newly created leaf may exist before all of its bytes are written;
- promotion between create and move contains a leaf and node with the same
  ordinal and key;
- unwind may have restored only a suffix of the landed effects.

Deletion has a different intermediate-state contract: every successful unlink
is permanent, and `RemovalStopped` reports the completed prefix instead of
claiming rollback. Cooperating calls cannot observe the prefix while the guard
lives, but a process failure or ignored lock can expose it.

The report-error guarantee covers the last observable state after a function
returns. `Error::Failed` means unwind completed and every effect this run landed
was undone; if no process wrote outside the locking protocol, the tree matches
the snapshot. `Error::FailedPartiallyRolledBack` means unwind stopped and the
error names both failures. No guarantee restores a process killed during apply
or unwind. The next process observes whatever filesystem state the completed
syscalls left.

The lock also does not serialize a writer that does not take it. The interpreter
therefore claims every destination even though the guarded plan was valid
against the locked snapshot. Exclusive create syscalls prevent replacement for
new entries. The explicit pre-rename check prevents replacement when it sees an
occupied path, but a non-cooperating writer can race between that check and
`rename`. The library provides coordination for cooperating processes, bounded
collision defense for other writers, and no isolation from arbitrary external
filesystem mutation.

<a id="errors"></a>
## Refusals, boundary errors, and recovery errors

`Error` separates outcomes by where they arise and by what the consumer may
infer about tree state:

- `Refused` is a total algebraic decision. No effect ran.
- `Malformed`, `Reserved`, `NonUtf8Name`, and
  `NameIsNotOneComponent` reject names at the read or render boundary.
- `NoContainingDirectory` rejects a root for which the lock object cannot be
  defined.
- `RootIsNotATree` reports a non-directory occupying the root and leaves it in
  place.
- `RootIsNotSpelledDirectly` refuses an unsafe deletion spelling before any
  removal.
- `Io` reports a filesystem failure outside an interpreter run, such as locking
  or snapshot reading.
- `Failed` reports a forward interpreter failure followed by a complete unwind.
  Every effect this run landed was undone; without external mutation, the tree
  is as the snapshot found it.
- `FailedPartiallyRolledBack` reports both the forward failure and the failed
  undo. The tree requires inspection and repair before retry.
- `RemovalStopped` reports a deletion failure and the paths already removed. It
  promises no rollback; an empty path list is the only unchanged case.

Every public mutation consumes its `WriteGuard`, whatever outcome it returns.
A retry therefore starts by acquiring a new guard and reading a fresh snapshot.
After `Failed`, the consumer first addresses the reported forward cause; after
`FailedPartiallyRolledBack`, it inspects and repairs the partial state before
acquiring that guard. A refusal also requires a new guard, but no filesystem
effect from the refused operation needs recovery.

The error module owns the boundary for operation failures. This fragment turns
domain refusals, malformed or reserved names, filesystem causes, and interpreter
failures into typed `Error` outcomes, preserving the distinction between no
effect, complete unwind, and partial unwind that the worked failure traces
require.

<!-- fragment «error-boundary» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="1-23" parent="filesystem-error-source" -->
````rust
//! What the library says when it cannot proceed.
//!
//! Two of these carry the **consumer's own** error value, because the
//! architecture document requires a refusal to say what to *do* about the
//! problem and only the domain knows that: the library halts the whole tree on a
//! `Malformed` or a `Reserved` name wherever it sits, and an error saying only
//! *something is wrong* leaves whoever hit it with a frozen tree and no next
//! step.
//!
//! The rest are the library's own, and each is a case where it can see a problem
//! and has no domain error value with which to report it.

use core::fmt;
use std::path::PathBuf;

use crate::{EntryName, Refusal};

/// Why an operation could not proceed.
///
/// Generic over the name type so that a consumer can match on its **own** error
/// rather than on a string: `Malformed` and `Reserved` carry
/// [`EntryName::Err`] verbatim, and it is reachable through
/// [`std::error::Error::source`] as well.
````
<!-- /fragment -->

`Error<N>` is the public taxonomy for operation failures in this page. The
library maps an algebraic `Refusal` to `Error::Refused`, and maps consumer
parsing failures, filesystem operations, invalid path components,
forward-application failures, and unwind failures into distinct variants whose
fields retain the path, action, and original cause needed for recovery. This
separation preserves the invariant that a cleanly rolled-back worked failure is
distinguishable from a partially rolled-back tree, while keeping domain-owned
advice in `N::Err`.

<!-- fragment «error-taxonomy» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="24-259" parent="filesystem-error-source" -->
````rust
pub enum Error<N: EntryName> {
    /// The filesystem refused. `doing` is the step, in the imperative, for a
    /// message that reads `reading the tree root /x/y: permission denied`.
    Io {
        /// What was being acted on, in the caller's own spelling. Paths are
        /// never canonicalised, so what comes back is what went in.
        path: PathBuf,
        /// The step that failed.
        doing: &'static str,
        /// What the filesystem said.
        source: std::io::Error,
    },
    /// A name the consumer recognises as its own, and cannot parse. Halts every
    /// operation, wherever in the tree it sits — snapshot scope is the whole
    /// tree, and that blast radius is the point.
    Malformed {
        /// The offending name, in the caller's spelling of its path.
        path: PathBuf,
        /// The consumer's own error, carrying the recovery advice.
        source: N::Err,
    },
    /// A name the consumer owns that is deliberately not an entry — a
    /// transaction witness, a lock marker, a sentinel left by an interrupted
    /// operation. Halts the same way, and for the same reason: the library
    /// cannot know what it means, so proceeding past it is a guess.
    Reserved {
        /// The offending name, in the caller's spelling of its path.
        path: PathBuf,
        /// The consumer's own error, carrying the recovery advice.
        source: N::Err,
    },
    /// The algebra refused: a stated outcome in which the operation changed
    /// nothing.
    ///
    /// A refusal is not an error thrown from inside the algebra — the algebra
    /// *returns* it, as one of the two halves of a decision, and this variant is
    /// where it becomes an `Err` at the filesystem boundary. Every one of them
    /// is total and stated: `ARCHITECTURE.md`'s *Refusals* section is the list,
    /// and [`Refusal`] carries the recovery advice.
    Refused(Refusal),
    /// An effect failed, and the run unwound everything it had applied.
    ///
    /// **The tree is as it was found**: this is *plan atomicity*, which
    /// `operations.qnt` checks as `inv_atomicity` — after a mutation returns an
    /// error, either every effect landed or none did. The promise covers
    /// reported errors and nothing else; a process killed mid-apply is not
    /// recoverable, and the library says so rather than implying otherwise.
    Failed {
        /// What the failing effect was acting on, in the caller's spelling.
        path: PathBuf,
        /// The step that failed, in the imperative.
        doing: &'static str,
        /// What the filesystem said.
        source: std::io::Error,
    },
    /// An effect failed, and unwinding it failed too.
    ///
    /// **The tree is in neither the state it was found in nor the one
    /// intended.** This is the exception *When rollback fails* states, and the
    /// one path by which this library damages a tree it was handed — which is
    /// why it is a variant of its own rather than an `Io` with a longer
    /// message. On the promotion path it leaves a leaf and a node sharing an
    /// ordinal and a key, and the `Display` below says how to resolve that,
    /// because a consumer meeting this needs a next step and not a diagnosis.
    ///
    /// `operations.qnt`'s `rollback_fails` instance exists to exhibit it:
    /// `wit_partialRollbackLeavesADuplicateKey` is reached, and that instance is
    /// the only one that does not claim key uniqueness at rest.
    FailedPartiallyRolledBack {
        /// What the failing effect was acting on.
        path: PathBuf,
        /// The step that failed.
        doing: &'static str,
        /// What the filesystem said about it.
        source: std::io::Error,
        /// What the failing *unwind* step was acting on.
        unwinding: PathBuf,
        /// The unwind step that failed.
        undoing: &'static str,
        /// What the filesystem said about that.
        unwind_source: std::io::Error,
    },
    /// A removal stopped partway, and **nothing was put back**.
    ///
    /// The third of the three failure shapes, and the one a deletion needs
    /// because neither of the other two is honest about it. [`Error::Failed`]
    /// promises *the tree is as it was found — every effect this operation had
    /// applied was undone*, and a removal has nothing to undo: an unlinked file
    /// is gone, and putting it back would mean having copied it aside first,
    /// which is staging machinery this library does not have.
    /// [`Error::FailedPartiallyRolledBack`] is wrong in the other direction —
    /// it reports an unwind that failed, and here no unwind was ever attempted.
    ///
    /// So this variant claims nothing and reports: what stopped it, and the
    /// paths that had already gone. `removed` is **empty** exactly when the
    /// failure came before anything was removed, which is the difference
    /// between a tree that is as it was found and one that is in neither state;
    /// the `Display` below says which.
    ///
    /// *Neither model can pose this*: both hold trees, and neither models a
    /// root that ceases to exist.
    RemovalStopped {
        /// The tree root the removal was asked for, in the caller's spelling.
        root: PathBuf,
        /// What the failing step was acting on.
        path: PathBuf,
        /// The step that failed, in the imperative.
        doing: &'static str,
        /// What the filesystem said.
        source: std::io::Error,
        /// What had already been removed, in the order it went. Paths and not
        /// names, for the reason [`Removed`](crate::Removed) gives.
        removed: Vec<PathBuf>,
    },
    /// A root spelled a way a deletion cannot act on.
    ///
    /// Every other operation uses the root as a **container**, so the kernel
    /// resolving its last component is exactly right: a symbolic link naming a
    /// directory is an accepted spelling of the tree it names, and
    /// `read::containing_directory` goes out of its way to make every spelling
    /// of one tree take one lock.
    ///
    /// A deletion is the one operation that acts on the root as an **object**,
    /// and there the last component decides *which* object — a link and what it
    /// names are two, and only one of them is the tree. It is also the one
    /// operation that removes the very components a path is built from, so a
    /// spelling that descends into the tree and comes back out through `..`
    /// stops resolving partway through its own removal.
    ///
    /// Both are refused before anything is removed, and refused rather than
    /// guessed at: this library will not decide on a caller's behalf whether a
    /// link or its target was meant. `reason` says which spelling it was, so
    /// the message can name the fix.
    RootIsNotSpelledDirectly {
        /// The root as the caller spelled it.
        root: PathBuf,
        /// What is wrong with that spelling, as a clause.
        reason: &'static str,
    },
    /// A filename that is not UTF-8, which halts.
    ///
    /// [`EntryName::parse`] takes a `&str`, so the library cannot ask the
    /// domain about such a name at all — there is no verdict to be had and
    /// therefore no domain error to carry, which is why the advice here is the
    /// library's own. It halts rather than being skipped because skipping is
    /// the failure the parse trichotomy exists to prevent: a hand-edit that
    /// mangles one byte of a real name produces exactly this, and a skipped
    /// *directory* takes its whole subtree out of every traversal while the
    /// tree reports itself healthy. The cost is that a genuinely foreign file
    /// with a non-UTF-8 name freezes the tree too, and that is the same blast
    /// radius `Malformed` already carries.
    NonUtf8Name {
        /// The offending name, lossily rendered, joined to its directory.
        path: PathBuf,
    },
    /// A name rendered as something that is not one filename.
    ///
    /// The seventh obligation — *a name renders as one path component* — and the
    /// only one the library enforces rather than assumes. Everywhere else a
    /// broken obligation is a tree the library quietly corrupts; this one would
    /// take it **outside the tree**, because the rendering is what gets joined to
    /// a level's directory. Occupancy compares views, so the offending name looks
    /// perfectly canonical to the algebra and only the path betrays it.
    ///
    /// Checked at both boundaries where a name becomes a path: every name a
    /// snapshot admits, and every name a plan will place — the latter before any
    /// effect runs, so a plan carrying one changes nothing at all.
    ///
    /// *Neither model can pose this*, in the same position as
    /// [`Refusal::ContentForANode`]: both models hold no strings by design, so
    /// there is no witness behind this variant and none to look for.
    ///
    /// [`Refusal::ContentForANode`]: crate::Refusal::ContentForANode
    NameIsNotOneComponent {
        /// The tree root, in the caller's own spelling.
        root: PathBuf,
        /// What the domain rendered.
        rendered: String,
        /// Why that is not a filename.
        reason: &'static str,
    },
    /// Something is at the tree root that a tree cannot be: a regular file, a
    /// socket, a symbolic link naming one of those, or a dangling one.
    ///
    /// The **third** answer to *is there a tree here*, and the reason that
    /// question is a trichotomy rather than a pair. [`fs::read`] and
    /// [`fs::write`] answer the other two as shapes — a tree, or a vacancy the
    /// caller may initialize — and neither of those is honest about a root
    /// occupied by something else: reporting it as a vacancy would send
    /// `initialize` at a name that is already taken, and reporting it as a tree
    /// would ask the listing for a directory that is not one.
    ///
    /// It is an error and not a variant of either shape for the reason every
    /// halt in this module is one: nothing here can know whether the thing
    /// sitting there is precious, so removing it is not the library's to
    /// choose. `found` is what it turned out to be, so the message can say.
    ///
    /// Decided by **following** symbolic links, because a link naming a
    /// directory is an accepted spelling of a root — see
    /// `fs::read::containing_directory`. A link naming nothing is therefore
    /// [`Found::Other`]: it is not a directory, and it is not nothing either,
    /// since it occupies the name.
    ///
    /// *Neither model can pose this*: both hold trees, and a root that is not a
    /// directory is not a tree for them to hold.
    ///
    /// [`fs::read`]: crate::fs::read
    /// [`fs::write`]: crate::fs::write
    /// [`Found::Other`]: crate::Found::Other
    RootIsNotATree {
        /// The root as the caller spelled it.
        root: PathBuf,
        /// What is there instead of a tree.
        found: crate::Found,
    },
    /// The tree root has no containing directory, so there is nothing to lock.
    ///
    /// The advisory lock is taken on the directory *containing* the root — it
    /// exists before the root is created and persists after it is deleted, so
    /// the tree's creation and destruction fall under the same lock as every
    /// ordinary operation. A filesystem root has no such directory.
    ///
    /// Decided on the filesystem's own identity rather than on the shape of the
    /// path, so `/`, `/..` and a symbolic link naming `/` are refused alike: a
    /// root that *is* its own containing directory has nothing above it to
    /// lock.
    NoContainingDirectory {
        /// The root as the caller spelled it.
        root: PathBuf,
    },
}

// Debug by hand rather than by derive: a derive would bound `N: Debug`, and this
// type mentions `N` only through `N::Err` — which is already `Debug`, since
// `std::error::Error` requires it. The same spurious-bound avoidance `Triple`
// and `Entry` make.
````
<!-- /fragment -->

The error module also owns structural debug rendering. This fragment turns a
borrowed `Error<N>` into variant-specific debug fields without imposing
`N: Debug`, preserving the generic consumer seam while making the worked
failure's path, action, and causes inspectable and completing that representation
for the taxonomy.

<!-- fragment «error-debug» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="260-354" parent="filesystem-error-source" -->
````rust
impl<N: EntryName> fmt::Debug for Error<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                path,
                doing,
                source,
            } => f
                .debug_struct("Io")
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .finish(),
            Self::Malformed { path, source } => f
                .debug_struct("Malformed")
                .field("path", path)
                .field("source", source)
                .finish(),
            Self::Reserved { path, source } => f
                .debug_struct("Reserved")
                .field("path", path)
                .field("source", source)
                .finish(),
            Self::Refused(refusal) => f.debug_tuple("Refused").field(refusal).finish(),
            Self::Failed {
                path,
                doing,
                source,
            } => f
                .debug_struct("Failed")
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .finish(),
            Self::FailedPartiallyRolledBack {
                path,
                doing,
                source,
                unwinding,
                undoing,
                unwind_source,
            } => f
                .debug_struct("FailedPartiallyRolledBack")
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .field("unwinding", unwinding)
                .field("undoing", undoing)
                .field("unwind_source", unwind_source)
                .finish(),
            Self::RemovalStopped {
                root,
                path,
                doing,
                source,
                removed,
            } => f
                .debug_struct("RemovalStopped")
                .field("root", root)
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .field("removed", removed)
                .finish(),
            Self::RootIsNotSpelledDirectly { root, reason } => f
                .debug_struct("RootIsNotSpelledDirectly")
                .field("root", root)
                .field("reason", reason)
                .finish(),
            Self::NonUtf8Name { path } => {
                f.debug_struct("NonUtf8Name").field("path", path).finish()
            }
            Self::NameIsNotOneComponent {
                root,
                rendered,
                reason,
            } => f
                .debug_struct("NameIsNotOneComponent")
                .field("root", root)
                .field("rendered", rendered)
                .field("reason", reason)
                .finish(),
            Self::RootIsNotATree { root, found } => f
                .debug_struct("RootIsNotATree")
                .field("root", root)
                .field("found", found)
                .finish(),
            Self::NoContainingDirectory { root } => f
                .debug_struct("NoContainingDirectory")
                .field("root", root)
                .finish(),
        }
    }
}

````
<!-- /fragment -->

`Display` is recovery-oriented. Consumer-owned errors and algebraic refusals
render their own advice. `Failed` states that nothing changed;
`FailedPartiallyRolledBack` states that neither target state was reached and
describes interrupted-promotion repair. Name and root errors state the action
needed before another attempt.

<!-- fragment «error-display» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="355-489" parent="filesystem-error-source" -->
````rust
impl<N: EntryName> fmt::Display for Error<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                path,
                doing,
                source,
            } => {
                write!(f, "{doing} {}: {source}", path.display())
            }
            // The consumer's advice *is* the message. Prefixing it with a
            // second sentence of the library's own would push the actionable
            // half off the end of a terminal line.
            Self::Malformed { source, .. } | Self::Reserved { source, .. } => {
                fmt::Display::fmt(source, f)
            }
            // The refusal's own advice *is* the message, for the reason the
            // domain's is: a second sentence in front of it pushes the
            // actionable half off the end of a terminal line.
            Self::Refused(refusal) => fmt::Display::fmt(refusal, f),
            Self::Failed {
                path,
                doing,
                source,
            } => write!(
                f,
                "{doing} {}: {source}. Nothing was changed \u{2014} every effect this \
                 operation had applied was undone.",
                path.display()
            ),
            Self::FailedPartiallyRolledBack {
                path,
                doing,
                source,
                unwinding,
                undoing,
                unwind_source,
            } => write!(
                f,
                "{doing} {}: {source}. Undoing what had already been applied then \
                 failed as well \u{2014} {undoing} {}: {unwind_source} \u{2014} so this tree is \
                 now in neither the state the operation found nor the one it \
                 intended, and it needs a human. If a node and a leaf share an \
                 ordinal and a key and the node holds no distinguished child, \
                 that is an interrupted promotion: removing either half resolves \
                 it.",
                path.display(),
                unwinding.display()
            ),
            // Two sentences and not one, because the two cases want different
            // next steps: a removal that got nowhere left a tree a consumer can
            // still use, and one that got partway left a tree it cannot.
            Self::RemovalStopped {
                root,
                path,
                doing,
                source,
                removed,
            } if removed.is_empty() => write!(
                f,
                "{doing} {}: {source}. Nothing was removed, so the tree {} is as it \
                 was found.",
                path.display(),
                root.display()
            ),
            Self::RemovalStopped {
                path,
                doing,
                source,
                removed,
                root,
            } => write!(
                f,
                "{doing} {}: {source}. {} path{} beneath the tree root {} had already \
                 been removed, and a removal has nothing to put back \u{2014} so this \
                 tree is now in neither the state the operation found nor the one it \
                 intended, and it needs a human. Deleting again removes what is left; \
                 nothing here restores what has gone.",
                path.display(),
                removed.len(),
                if removed.len() == 1 { "" } else { "s" },
                root.display()
            ),
            Self::RootIsNotSpelledDirectly { root, reason } => write!(
                f,
                "the tree root {} {reason}. Nothing was removed. Every other operation \
                 accepts this spelling, because it uses the root as the directory \
                 things are in; a deletion acts on the root itself, so its last \
                 component decides what gets destroyed and this library will not guess \
                 which you meant. Name the directory itself and delete that.",
                root.display()
            ),
            Self::NonUtf8Name { path } => write!(
                f,
                "the filename {} is not valid UTF-8, so it cannot be classified: \
                 a name that cannot be read cannot be disclaimed either, and skipping it \
                 would lose it — and everything beneath it if it is a directory. \
                 Rename it to valid UTF-8, or move it out of the tree.",
                path.display()
            ),
            Self::NameIsNotOneComponent {
                root,
                rendered,
                reason,
            } => write!(
                f,
                "this domain rendered a name as `{rendered}`, which {reason}, so it is \
                 not one filename. A name is exactly one path component: the library \
                 joins it to a level's directory to reach the entry, so this one would \
                 address outside the tree {} \u{2014} whose containing directory is the \
                 only thing the lock covers. Nothing was changed. Fix the domain's \
                 `Display`, `compose` or `parse` so that every name it renders is one \
                 filename, and check it with `conformance::check`, which reports this \
                 before there is a tree.",
                root.display()
            ),
            Self::RootIsNotATree { root, found } => write!(
                f,
                "the tree root {} is {found}, and a tree is a directory. Nothing was \
                 changed: this library will not move aside or replace something it \
                 did not put there, because it cannot know what it is. Move it out \
                 of the way yourself, or name a different root.",
                root.display()
            ),
            Self::NoContainingDirectory { root } => write!(
                f,
                "the tree root {} has no containing directory to lock. \
                 The advisory lock is taken on the directory holding the root, because \
                 that directory outlives the root itself; a filesystem root has none.",
                root.display()
            ),
        }
    }
}

````
<!-- /fragment -->

The standard error source is the forward cause. A partial rollback retains the
unwind cause in its fields and display text, while `source()` returns the
forward failure that caused unwind to begin.

<!-- fragment «error-sources» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="490-510" parent="filesystem-error-source" -->
````rust
impl<N: EntryName> std::error::Error for Error<N> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Malformed { source, .. } | Self::Reserved { source, .. } => Some(source),
            // The failing *effect*, not the failing unwind: `source` is a chain
            // of causes and the effect is what caused the unwind to be needed.
            // The unwind's own error is in the `Display`, where a consumer
            // reading the message meets both.
            Self::Failed { source, .. }
            | Self::FailedPartiallyRolledBack { source, .. }
            | Self::RemovalStopped { source, .. } => Some(source),
            Self::Refused(_)
            | Self::RootIsNotSpelledDirectly { .. }
            | Self::NonUtf8Name { .. }
            | Self::NameIsNotOneComponent { .. }
            | Self::RootIsNotATree { .. }
            | Self::NoContainingDirectory { .. } => None,
        }
    }
}
````
<!-- /fragment -->

The filesystem boundary is now complete. A read or write guard owns one locked
snapshot; a vacancy retains the exclusive lock without inventing a snapshot.
Accepted mutations of an existing tree reach the ordered interpreter.
Initialization first creates the root and can still fail before application;
once application begins, its forward failures use the same reverse unwind as
other mutations, followed by root removal when restoration succeeds. The result
type distinguishes no-effects refusal, clean restoration, partial restoration,
and pre-apply boundary failures without describing a multi-effect operation as
atomic against process termination.

[Previous: Mutation algebra](05-mutation-algebra.md) | [Contents](README.md) | [Next: Syllabus CLI](07-syllabus-cli.md)
