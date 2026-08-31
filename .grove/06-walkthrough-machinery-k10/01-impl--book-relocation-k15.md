# book-relocation-k15

## Goal

Move `docs/ordinal-fs-tree/book/` to `docs/walkthroughs/ordinal-fs-tree/` and
re-point every reference to it, leaving the repository green. Nothing else
changes: no validator semantics, no book prose, no source.

## Context

- Decision 3 of `plan-k1`: one root, `docs/walkthroughs/`, and the existing book
  moves into it — chosen over two roots for uniformity, with the wide mechanical
  rename accepted as its price.
- `CONTEXT-MAP.md` argues that `docs/walkthroughs/` asserts no bounded-context
  boundary, which a per-crate directory under `docs/` would. Keep the book's
  *other* artifacts — `ARCHITECTURE.md`, `CONTEXT.md`, `CLI.md`, `models/` —
  exactly where they are under `docs/ordinal-fs-tree/`. Only `book/` moves, and
  `CONTEXT-MAP.md`'s citation of the walkthrough is what needs re-pointing.
- The old path appears 78 times across 16 files outside `.grove/`: the
  validator's `src/cli.rs`, `src/ledger.rs`, `src/markdown.rs` and
  `src/validator.rs`; seven test files plus `tests/support/mod.rs`;
  `crates/book-validation/README.md`; `CONTEXT-MAP.md`;
  `docs/specs/ordinal-fs-tree-book.md`; and
  `docs/ordinal-fs-tree/book/08-invariants-and-trade-offs.md` inside the book
  itself. Re-derive that set rather than trusting this list — it was measured
  before any of it moved.
- The book's pages link outward with relative paths, and the directory gains a
  level of nesting change. Every `../` in the moved pages has to be recomputed,
  and `every_repository_markdown_reference_resolves` is what proves it.

## Done when

- `docs/walkthroughs/ordinal-fs-tree/` holds the eleven book files and
  `docs/ordinal-fs-tree/book/` no longer exists.
- A repository-wide sweep for the old path finds nothing outside `.grove/` and
  the version-control history, run with a positive control that is seen to fail
  before the clean read is credited.
- `bash scripts/check.sh` passes in full.

## Notes

**This is one leaf and not three, and the reasoning is on the record.** The
`walkthroughs-k3` task file prescribed `expand → migrate → contract`; decision 2
of that leaf's log records why the sequence was collapsed — the blast radius is
entirely in-repo, nothing external consumes the path, and no intermediate state
needs both forms alive. If you find a call site that *does* need the old path to
survive a commit, that decision is wrong and the leaf should decompose into the
three stages rather than land a red tree.

**Do not take the opportunity to tidy.** Constants this move rewrites are deleted
two leaves later by `validator-fragments-k22`; rewriting them now is the cheap
half of a change whose expensive half is already scheduled. A rename that also
refactors cannot be reviewed as a rename.
