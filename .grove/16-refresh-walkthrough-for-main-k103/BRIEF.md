# refresh-walkthrough-for-main-k103 — brief

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

Each implementation child owns a source-coherent book slice. The source slices
run before the whole-book exposition reconciliation so later prose and
navigation checks inspect exact, current fragments. The final child commissions
proportionate independent review after the refreshed artifact exists.

## Decomposition

- `public-seam-k104` refreshes the manifest and library façade in the
  orientation chapter.
- `name-and-reference-k105` refreshes names, sought-object resolution, and the
  reference-domain chapters.
- `read-model-k106` refreshes snapshots and filesystem reading.
- `mutation-algebra-k107` refreshes operations, plans, and reports.
- `filesystem-lifecycle-k108` refreshes filesystem guards, application,
  removal, rollback, and errors.
- `syllabus-cli-k109` refreshes the demonstration CLI, including deletion and
  stream boundaries.
- `exposition-reconciliation-k110` reconciles whole-book prose and navigation,
  runs final checks, and commissions independent assurance.

## Decisions (running log)

The rebase delta spans thirteen source roots and several thousand current source
lines, so the refresh is decomposed at source-owning book slices. Each child
must restore exact tangling for its roots rather than merely update ledger line
counts; the final child owns cross-book claims and review commissioning.

Direct production-source enumeration found two new roots not represented in the
old fifteen-root fixed inventory: `src/sought.rs` and `src/fs/remove.rs`. The
name/reference and filesystem-lifecycle children respectively own adding and
explaining them, so completion requires seventeen current production roots
rather than preserving the obsolete inventory cardinality.
