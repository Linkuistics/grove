# root-delete-k26

## Goal

Let the store remove the tree root and everything beneath it, and report what it
removed honestly.

## Context

`docs/specs/module-decomposition.md`, decision 2, stated verbatim:

```rust
impl<N: EntryName> WriteGuard<N> {
    /// Remove the tree root and everything beneath it, following no symlink.
    pub fn delete(self) -> Result<Removed, Error<N>>;
}

pub struct Removed { pub root: PathBuf, pub entries: Vec<PathBuf> }
```

**Why paths, where every other mutation reports names.** Design-review finding 3,
and the spec states it as the operation's own asymmetry rather than an oversight:
the existing `Report` has a created bucket and a renamed bucket, both keyed by
`N`, because every other mutation acts on entries the domain named. Deletion acts
on the *root* and therefore on everything beneath it — **including the entries
the domain deliberately declines to parse as `N`**, which the walk already skips
and which the report has no `N` to name. A third bucket of `N` would still be
unable to say what it removed. `Removed` is the honest postcondition: the paths
that are gone, which is exactly what a caller needs to say what it destroyed and
is the whole of what the operation knows.

## Done when

- `delete` consumes the write guard, removes the root and everything beneath it,
  and **follows no symlink**. The no-symlink rule is a security property, not a
  detail: assert it in a test with a symlink pointing outside the root.
- `Removed { root, entries }` reports paths **in the order they went**, foreign
  entries included.
- `docs/adr/entries-are-never-removed.md` is **amended with one clause**
  distinguishing removing an *entry* from deleting the *root*. Its argument is
  untouched — removing an entry lowers the visible key maximum and the next
  allocation re-issues a live key — but its **opening sentence** says the library
  offers no removal operation, and that is what stops being true. Brief principle
  5 already states the distinction; the record does not.
- Deleting a vacancy is not expressible (it is a method on the tree guard, which
  a vacancy is not) and a test or doc test says so.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, `CONTEXT.md` and `CLI.md` reconciled;
  `models/operations.qnt` and `models/structure.als` gain root removal or state
  why they do not.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it depends on `open-shape-k25` only for `WriteGuard` to be
reachable through the new `Writing` shape.

**This is the operation that makes the finish teardown the store's.** Grove's
`finish-commit` currently removes `.grove/` itself; after this it can ask the
store, under the lock, and get back the paths it needs to name in a commit
message. Wiring that up is `loop-crate-verbs-k21`'s, not this leaf's.

**Do not add a recovery path.** Principle 1: the version control system owns
safety and history — `jj undo` restores a deleted root, as `minimalism-k1`
measured directly. A `delete` that quarantines, backs up, or stages a rollback is
the machinery this whole grove exists to remove.
