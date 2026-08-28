# Filesystem interpreter
<!-- book-page id="filesystem-interpreter" slice="filesystem-interpreter-k16" order="6" -->
[Previous: Mutation algebra](05-mutation-algebra.md) | [Contents](README.md)

The pure algebra produces an ordered `Plan`; the filesystem layer interprets
that value while holding the exclusive lock captured with its source
`Snapshot`. Every public mutation follows the same path: a consuming
`WriteGuard` asks the algebra for a `Decision`, converts a refusal into
`Error::Refused`, or sends the plan through one interpreter. The interpreter
validates every rendered destination, applies effects in order, records a
`Report`, and unwinds landed effects in reverse after a reported forward
failure.

The guarantee is bounded. A completed unwind undoes every effect this run
landed; when no process writes outside the locking protocol, that restores the
tree the snapshot described. A failed unwind reports a partial rollback and
requires repair. Process termination can expose an intermediate state because
there is no journal or restart recovery. The advisory lock hides those
intermediate states only from processes that use this library's locking
protocol.

<!-- fragment «filesystem-error-source» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="1-342" parent="source-error" -->
<!-- insert «error-boundary» -->
<!-- insert «error-taxonomy» -->
<!-- insert «error-debug» -->
<!-- insert «error-display» -->
<!-- insert «error-sources» -->
<!-- /fragment -->

<!-- fragment «filesystem-write-guard-api» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="169-378" parent="source-filesystem-module" -->
<!-- insert «write-guard-accessors» -->
<!-- insert «write-guard-append» -->
<!-- insert «write-guard-insert» -->
<!-- insert «write-guard-promote» -->
<!-- insert «write-guard-rewrite» -->
<!-- insert «write-guard-dispatch» -->
<!-- /fragment -->

<!-- fragment «filesystem-interpreter-source» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="1-471" parent="source-filesystem-apply" -->
<!-- insert «apply-contract» -->
<!-- insert «apply-plan» -->
<!-- insert «apply-run-state» -->
<!-- insert «apply-effect-step» -->
<!-- insert «apply-unwind-and-paths» -->
<!-- insert «apply-undo» -->
<!-- insert «apply-destination-claim» -->
<!-- insert «apply-fault-seam» -->
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
The snapshot and lock descriptor enter `WriteGuard` together; no consumer can
receive a snapshot taken before the exclusive lock.

<!-- fragment «filesystem-write-acquire» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="87-105" parent="source-filesystem-module" -->
````rust
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

````
<!-- /fragment -->

The lock module fixes the lock location, inode-identity behavior, shared and
exclusive modes, blocking semantics, and descriptor lifetime.

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

<!-- fragment «filesystem-write-guard» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="132-154" parent="source-filesystem-module" -->
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

````
<!-- /fragment -->

The accessors preserve the caller's spelling of the root and expose the exact
snapshot used by the later decision.

<!-- fragment «write-guard-accessors» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="169-181" parent="filesystem-write-guard-api" -->
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

The write guard also dereferences to the snapshot. This is read-only access;
the only methods that alter the tree consume the guard.

<!-- fragment «filesystem-write-deref» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="387-393" parent="source-filesystem-module" -->
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

The five public operations differ only in the algebra function they call and
its inputs. Each computes a `Decision` from `self.snapshot`, then calls the
private `run`. Before returning `Decision::Proceed`, the algebra passes its
constructed plan through `Plan::guarded`, which folds the effects through a
simulated state in order and refuses any destination occupied by the snapshot
or an earlier effect. A refusal crosses the filesystem boundary as
`Error::Refused` without invoking the interpreter. A proceeding, guarded plan
reaches `apply` while `self` still owns the exclusive lock. Dropping the
consumed guard releases the lock after `run` returns. This sequential plan
guard is distinct from `apply`'s later pre-effect check that every rendered name
is one path component.

Append and append-many share this dispatch shape.

<!-- fragment «write-guard-append» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="182-220" parent="filesystem-write-guard-api" -->
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

<!-- fragment «write-guard-insert» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="221-255" parent="filesystem-write-guard-api" -->
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

<!-- fragment «write-guard-promote» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="256-318" parent="filesystem-write-guard-api" -->
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

<!-- fragment «write-guard-rewrite» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="319-364" parent="filesystem-write-guard-api" -->
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
    /// might want, and with no removal operation
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

The private dispatch is the only place a pure `Decision` becomes an ordinary
Rust `Result`.

<!-- fragment «write-guard-dispatch» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="365-378" parent="filesystem-write-guard-api" -->
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
## Successful application and failed unwind

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

The module contract states why every operation shares this interpreter and
states the exact limits of rollback and destination claiming.

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

<!-- fragment «apply-plan» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="50-87" parent="filesystem-interpreter-source" -->
````rust
/// Apply a plan under the exclusive lock, or leave the tree as it was found.
pub(super) fn apply<N: EntryName>(
    root: &Path,
    snapshot: &Snapshot<N>,
    plan: &Plan<N>,
    faults: Faults,
) -> Result<Report<N>, Error<N>> {
    // The seventh obligation, at the second of the two boundaries where a name
    // becomes a path — and **before any effect runs**, so a plan carrying one
    // bad name changes nothing rather than landing what it can and unwinding.
    // The snapshot's own names were checked when it was read, so between the two
    // checks every rendering this function will join is one path component.
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
````
<!-- /fragment -->

<!-- fragment «apply-run-state» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="88-105" parent="filesystem-interpreter-source" -->
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

<!-- fragment «apply-effect-step» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="106-211" parent="filesystem-interpreter-source" -->
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

<!-- fragment «apply-unwind-and-paths» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="212-270" parent="filesystem-interpreter-source" -->
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
Removal can target only a path created by this run, which makes the absence of a
public remove operation structural in the interpreter as well as the algebra.

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

<!-- fragment «apply-undo» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="271-330" parent="filesystem-interpreter-source" -->
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

<!-- fragment «apply-destination-claim» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="331-371" parent="filesystem-interpreter-source" -->
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

/// One effect, or one undo, that did not happen.
struct Failure {
    path: PathBuf,
    doing: &'static str,
    source: io::Error,
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

<!-- fragment «apply-fault-seam» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="372-471" parent="filesystem-interpreter-source" -->
````rust

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
- `Io` reports a filesystem failure outside an interpreter run, such as locking
  or snapshot reading.
- `Failed` reports a forward interpreter failure followed by a complete unwind.
  Every effect this run landed was undone; without external mutation, the tree
  is as the snapshot found it.
- `FailedPartiallyRolledBack` reports both the forward failure and the failed
  undo. The tree requires inspection and repair before retry.

Every public mutation consumes its `WriteGuard`, whatever outcome it returns.
A retry therefore starts by acquiring a new guard and reading a fresh snapshot.
After `Failed`, the consumer first addresses the reported forward cause; after
`FailedPartiallyRolledBack`, it inspects and repairs the partial state before
acquiring that guard. A refusal also requires a new guard, but no filesystem
effect from the refused operation needs recovery.

The enum carries consumer errors for domain-owned malformed and reserved names,
filesystem sources for boundary and interpreter failures, and separate fields
for the failed forward and unwind actions.

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

<!-- fragment «error-taxonomy» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="24-163" parent="filesystem-error-source" -->
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
````
<!-- /fragment -->

The manual `Debug` implementation avoids imposing `N: Debug`; the enum stores
the name type only through `N::Err`, which is already an error and therefore
debuggable.

<!-- fragment «error-debug» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="164-238" parent="filesystem-error-source" -->
````rust

// Debug by hand rather than by derive: a derive would bound `N: Debug`, and this
// type mentions `N` only through `N::Err` — which is already `Debug`, since
// `std::error::Error` requires it. The same spurious-bound avoidance `Triple`
// and `Entry` make.
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

<!-- fragment «error-display» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="239-322" parent="filesystem-error-source" -->
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

<!-- fragment «error-sources» owner="filesystem-interpreter-k16" source="crates/ordinal-fs-tree/src/error.rs" lines="323-342" parent="filesystem-error-source" -->
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
            Self::Failed { source, .. } | Self::FailedPartiallyRolledBack { source, .. } => {
                Some(source)
            }
            Self::Refused(_)
            | Self::NonUtf8Name { .. }
            | Self::NameIsNotOneComponent { .. }
            | Self::NoContainingDirectory { .. } => None,
        }
    }
}
````
<!-- /fragment -->

The filesystem boundary is now complete. One guard owns one locked snapshot and
one mutation. Every proceeding decision reaches the same ordered interpreter;
every reported forward failure reaches the same reverse unwind. The result type
distinguishes no-effects refusal, clean restoration, partial restoration, and
pre-apply boundary failures without describing a multi-effect operation as
atomic against process termination.

[Previous: Mutation algebra](05-mutation-algebra.md) | [Contents](README.md)
