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

**Grove is not migrated here.** Grove's own `read`/`write` mirror of this shape
is decision 9's and belongs to `loop-crate-verbs-k21`; the deletion of grove's
second lock layer is `collapse-tree-access-k13`. Both are outside this node.

**The type is the guarantee.** If a reviewer can write a program that calls
`initialize` on a live tree, the leaf is not done — the point of the shape is
that the ill-formed call does not typecheck.
