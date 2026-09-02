# orientation-k77

## Goal

Create the overview book and prove its first slice: `compiler-held`,
`01-orientation.md`, owning the whole of `crates/grove/Cargo.toml`.

## Context

- Draft stage, child 1 of 5 of `overview-k76`. The structure brief is
  `docs/specs/overview-book-structure.md`, and chapter 1's responsibilities are
  its *1 · Orientation* section: what `grove` is and is for; the two products
  and that neither installs the other; the crate-not-a-`[[bin]]` argument, the
  absence of `[lib]`, and the repository-surface tests' residence here.
- The first slice carries the book's scaffolding under
  `docs/specs/walkthrough-books.md`, *Authoring workflow and scoped proof*: the
  complete manifest, `README.md`, both lookup indexes, every source-root
  directive, the full ownership ledger with a defer for every later-owned
  block, and all five early-use rows `pending`.
- The two obligations outside the book are this child's: the `CONTEXT.md`
  anchors `guaranteed-core` and `task-tree-scheme`, and the overview's row in
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table.

## Done when

- `book-check --book docs/walkthroughs/overview --through compiler-held --check
  all` is valid: 3 files, 54 resolved lines, 150 deferred, `final=false`.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row` and the corpus-inventory
  tests pass with the new book root present.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

The worked example fixes the values every later chapter reuses; they are
recorded in `overview-book-k30`'s brief under *Pointers*.
