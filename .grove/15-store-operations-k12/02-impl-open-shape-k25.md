# open-shape-k25

## Goal

Answer *is there a tree here* **as a shape rather than as a predicate**, and give
the resulting vacancy the ability to create the tree under the lock it already
holds.

## Context

`docs/specs/module-decomposition.md`, decision 2, stated verbatim:

```rust
pub fn read<N: EntryName>(root: &Path)  -> Result<Reading<N>, Error<N>>;
pub fn write<N: EntryName>(root: &Path) -> Result<Writing<N>, Error<N>>;
pub enum Reading<N> { Tree(ReadGuard<N>), Vacant }
pub enum Writing<N> { Tree(WriteGuard<N>), Vacancy(Vacancy<N>) }

impl<N: EntryName> Vacancy<N> {
    pub fn initialize(
        self,
        distinguished: Option<Vec<u8>>,
        entries: Vec<NewEntry<N::Parts>>,
    ) -> Result<Report<N>, Error<N>>;
}
```

Two warrants, both settled:

- **Why a shape and not `exists()`.** `decomposition-k2`'s running log: a
  separate predicate is a check-then-act split, and check-then-act over a tree
  under a lock is exactly the disease grove's two-phase
  `Classification`/`settle` dance exists to paper over. One lock acquisition, and
  the answer hands back the only operation valid for it — initializing over a
  live tree and deleting a vacancy are **not expressible**.
- **Why `initialize` takes bytes.** Design-review finding 2. `NewEntry` can
  create only a positioned entry, and the distinguished child is the one entry it
  cannot express: it carries no parts and its name is `N::distinguished()`. The
  library **already writes one this way** when a promotion moves a leaf's bytes
  into a new node, so `entry-name-is-the-only-seam` holds with **no new trait
  method**. Without it the consumer would write the charter itself, outside the
  lock and outside the store, and *the store is the only thing that touches the
  task tree* would fail at the first operation of every fresh tree.

## Done when

- `read` and `write` return the two enums; the guards keep their names and
  behaviour. `Vacancy` is a guard: it holds the exclusive lock, so there is **no
  window** between deciding a tree is absent and creating it.
- `initialize` creates the root, writes the distinguished child, and places the
  first entries, under the lock already held. `None` creates a root without a
  distinguished child; `Some` in a domain whose `distinguished()` is `None` is
  the **same refusal a promotion gives for the same reason** — reuse it rather
  than inventing a second.
- Something at the root that is neither a tree nor nothing — a regular file, a
  symlink — is an `Error` **carrying what was found**, not a third variant.
- **Every consumer that must still compile is migrated here, mechanically.**
  Outside the crate that is exactly three call sites, all inside one module:
  `src/task_tree.rs:77`, `:117` and `:144` (`Tree` and `TreeWrite` alias the two
  guards at `:60` and `:94`). Each learns to match the new enum and to raise
  Grove's existing *absent tree* diagnostic on the vacant arm — the same message
  Grove's own lock layer produces today, moved, not redesigned. Nothing else in
  Grove changes shape.
- The crate's tests cover: vacancy → initialize → read back; initialize refused
  over a live tree by construction (a compile-fail or doc test is the honest form
  of *not expressible*); the non-tree root error.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, `CONTEXT.md` and `CLI.md` reconciled;
  `models/operations.qnt` gains root creation as a transition and
  `models/structure.als` gains the vacancy/tree/neither trichotomy, or each
  states why it does not.
- `docs/adr/grove-does-not-stage-its-own-renames.md` and
  `docs/adr/bulk-marks-are-not-atomic.md` **re-checked** against a store that now
  owns root creation — re-checked, not assumed. Record the outcome either way.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green.** This is the widest of the three inside the crate — every
existing caller of `fs::read` / `fs::write` sees a new return shape — but the
crate is small and its consumers are in-tree, so expand → migrate → contract fits
one session.

**Grove is *adapted* here; it is not *migrated* here.** The distinction is the
whole of what keeps this leaf green, and `module-split-k4` collapsed the two.
Adapting means the three call sites above learn the new return shape so the
workspace compiles — the mechanical half, which cannot be deferred to a later
sibling without leaving `cargo test` red for the sessions in between. Migrating
means Grove's own `read`/`write` mirror of this shape (decision 9,
`loop-crate-verbs-k21`) and the deletion of Grove's second lock layer
(`collapse-tree-access-k13`): both stay outside this node, and both are cheaper
*because* the adaptation already happened.

**Lands green as a whole workspace, not as a crate.** `cargo test` at the
workspace root is the check, not `cargo test -p ordinal-fs-tree` — a leaf that
leaves the root package uncompilable has not landed, whatever the crate's own
suite says.

**The type is the guarantee.** If a reviewer can write a program that calls
`initialize` on a live tree, the leaf is not done — the point of the shape is
that the ill-formed call does not typecheck.
