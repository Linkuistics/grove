# refresh-walkthrough-for-main-k103

## Goal

Refresh the completed `ordinal-fs-tree` walkthrough against the production
source now present after `rebase-onto-main-k102`, preserving the book's exact
source-reconstruction contract while explaining the current library and
demonstration CLI rather than the pre-rebase implementation.

## Context

- The rebase moved the grove onto `main` at `d3c28ad9` and resolved CLI stream
  conflicts without losing whole-tree deletion or percent-encoded terminal
  records.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/ordinal-fs-tree/book --final --check all` now reports F006 source-length
  mismatches for 13 of the 15 fixed source roots.
- The mismatches cover the manifest, `bin/syllabus.rs`, and eleven library
  modules. This is a source/book synchronization increment, not a reason to
  restore the older implementation.
- The existing book system, fragment IDs, source ledger, tangle checks, style
  contract, and technical/editorial review history remain the governing
  constraints. The doubt pass in `rebase-onto-main-k102` specifically found
  stale claims that the CLI has no removal command or destructive verb.

## Done when

- The fixed source inventory matches every current production source root and
  the crate manifest exactly.
- Recursively expanding each book root fragment reproduces its authoritative
  source file byte-for-byte, with every production line covered once.
- Exposition and navigation describe the current APIs, filesystem lifecycle,
  errors, reports, CLI deletion behavior, stream boundaries, and trade-offs;
  superseded pre-rebase claims are removed or rewritten.
- `book-check --final --check all`, the crate's tests, and relevant repository
  checks pass.
- The refreshed book receives proportionate independent technical/editorial
  assurance before the finish sentinel is allowed to tear down the grove.

## Notes

This work may prove too large for one focused session. If so, decompose this
leaf at the natural source-owning book slices rather than weakening exact-source
coverage or treating updated line counts as sufficient.
