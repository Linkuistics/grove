# corpus-subject-anchor-k50

## Goal

Pin `[book].subject` to something outside the book, so the corpus rule's floor
is genuinely not the author's to choose.

## Why this exists

`validator-fragments-k22` implemented the specification's *Groups* rule that
`[corpus] include` must contain both base patterns derived from
`[book].subject`. That rule is what makes filesystem derivation an external
witness rather than a self-declaration: derivation compares the declared roots
against the tree the *patterns* reach, so an author free to narrow the rule
narrows the witness with it.

**`subject` itself is unanchored, which reopens the hole one level up.** Nothing
in the schema, the validator or the repository suite ties `[book].subject` to
anything a second party wrote. `[book].id` must equal the book directory name;
`subject` must only be non-empty. So a book can set

```toml
subject = "crates/ordinal-fs-tree/src/fs"
```

and its base patterns become `crates/ordinal-fs-tree/src/fs/Cargo.toml` and
`crates/ordinal-fs-tree/src/fs/src/**/*.rs`. Every root outside `src/fs/` is then
reached by no pattern, so the author drops those `[[root]]` rows too, drops the
pages that reconstructed them — and the book validates green while covering a
fifth of the crate it claims to be about. `book-check --final` reports nothing,
because every check it runs is now internally consistent.

Found while implementing k22, by asking what a book could still do to claim
complete reconstruction while omitting production source. The exception
inventory closes that for exclusions and the base patterns close it for the
include list; this is the remaining move.

## Context

- The mechanism to reuse is already decided and already built: **declare it
  twice**. `docs/specs/walkthrough-books.md`'s *The corpus rule and its witness*
  carries a normative corpus exception inventory, and
  `crates/grove/tests/corpus_exception_inventory.rs` compares it against every
  manifest under `docs/walkthroughs/`, per book. A per-book `subject` is the
  same shape of fact and wants the same treatment — a column or a companion
  table, and the same test extended.
- This is why the leaf is `impl` rather than `design`: the agreement mechanism
  was settled by `walkthrough-books-spec-k20` and `-k47`; applying it to one
  more field is implementation. If working it reveals that `subject` wants a
  *different* mechanism — say, deriving it from the ownership table rather than
  restating it — that is a design question and this leaf decomposes.
- The root brief already fixes the answer for all six deliverables: its **The
  corpus, exactly** table gives the source directory per deliverable. That table
  lives in `.grove/` and is not a durable artifact, so the spec is where the
  rows have to land.
- `book-assurance-surface-k39` (the next leaf) makes every book's row in
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table discoverable. If that
  row is going to name a book's subject anyway, the two leaves should agree on
  one home for the fact rather than record it twice more.

## Done when

- `[book].subject` is declared in a second artifact the book does not own, and a
  repository test compares the two per book, seen to fail against a manifest
  whose subject the artifact does not carry.
- The narrowing attack above is a test: a manifest whose `subject` names a
  subdirectory of its real subject fails, and is seen to fail.
- `bash scripts/check.sh` passes.

## Notes

**Sequenced before the five remaining books, deliberately.** Each of them
declares a `subject`, and a rule that arrives after they are written has to be
retrofitted against five manifests instead of being the thing they are authored
against.
