# sought-k24

## Goal

Give the store a word for **a search that matched nothing**, and use it across
the whole search surface so there is one word for one concept.

## Context

`docs/specs/module-decomposition.md`, decision 2, the fourth operation:

```rust
pub enum Sought<T> { Match(T), Nothing }

impl<N: EntryName> Snapshot<N> {
    pub fn seek(&self, predicate: impl FnMut(&Entry<'_, N>) -> bool) -> Sought<Entry<'_, N>>;
    pub fn by_key(&self, key: Key) -> Sought<Entry<'_, N>>;
}
```

`decomposition-k2`'s running log, **"The fourth new operation is `Sought<T>`"**,
carries the warrant: `Refusal`'s twelve variants are all refusals to *mutate*; a
search that completed and matched nothing is neither a refusal nor an error, and
a store whose only word for it is `None` forces every consumer to invent the
answer in its own vocabulary — which is precisely what grove's
`Option<SelectedLeaf>` (`src/task_tree.rs:584`) is.

## Done when

- `Sought<T>` is public, documented as *not a refusal — nothing was asked to
  change, and nothing is wrong with the tree*, and carries whatever ergonomics
  the crate's own style justifies (a `match`-free accessor, `Option` interop) —
  but **`Option` does not remain in the public search surface**.
- `find` becomes `seek`; `by_key` returns `Sought`. Every public search on
  `Snapshot` (and `Walk`, if it has one) answers `Sought`.
- Consumers inside the crate — including the `syllabus` binary and the
  `conformance` kit — are reconciled.
- The crate's glossary (`docs/ordinal-fs-tree/CONTEXT.md`) gains the term and
  says what distinguishes it from `Refusal`; `ARCHITECTURE.md` and `CLI.md` are
  reconciled.
- The crate's tests cover both variants through the public interface.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is the smallest of this node's three. It is also the one
whose value is easiest to under-deliver: renaming `find` to `seek` while leaving
an `Option` beside it moves the problem rather than solving it. The spec is
explicit — *it replaces the whole optional search surface*.

**Grove is not changed here.** `collapse-tree-access-k13` and
`loop-crate-verbs-k21` are where grove's `Option<SelectedLeaf>` dies; this leaf
supplies the word they will use.

**The formal models may have nothing to say about this one.** A search is a pure
read and adds no state transition; if `operations.qnt` and `structure.als` are
genuinely untouched, say so explicitly rather than leaving it unmentioned — a
silent omission reads the same as an oversight.
