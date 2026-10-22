# keyed-launch-removal-k11

## Goal

Remove the `keyed-launch` crate with its tasks, tests, book and structure spec.
This is the contract step. It runs only after no caller uses the crate.

## Context

- The root brief: *Design* sections *Removal* and *Tests*, and the first
  items of *Records to rework* that the gate checks.
- `docs/specs/walkthrough-books.md` holds the row of the book and the subject
  inventory. `crates/grove/tests/corpus_exception_inventory.rs` checks them.
- Comments that cite the crate are in `crates/grove-llm/tests/removed_surface.rs`,
  `crates/jj-workspace/tests/environment.rs`,
  `crates/grove/tests/reference_navigation.rs` and
  `crates/grove/tests/loop_driver.rs`.

## Done when

- `crates/keyed-launch` does not exist. `Cargo.lock` and the root `Cargo.toml`,
  with its decision comment, carry no trace. The earlier leaves removed the
  dependency lines of `grove` and `grove-loop`.
- `Taskfile.yml` has no `test:keyed-launch` and no call to it in
  `ravel:activation`. The description of `ravel:fixtures` no longer says "both
  stdout runners". `testing/support.rs` has no `keyed-launch` member.
- `docs/walkthroughs/keyed-launch/`, `docs/specs/keyed-launch-book-structure.md`
  and the row in `docs/specs/walkthrough-books.md` are gone. Chapters 01 and 20
  of `grove-loop`, chapters 01 and 05 of `overview`, and chapter 07 of
  `jj-workspace` describe the crate graph and the checks without the crate.
- The four test comments no longer cite the crate.
- Every Markdown link into the removed book or structure spec is gone, so
  `every_repository_markdown_reference_resolves`
  (`crates/grove/tests/reference_navigation.rs`) passes at this boundary. The
  two inbound links are the row at `docs/ARCHITECTURE.md:45` (remove the row)
  and the sentence at `docs/specs/grove-loop-book-structure.md:19` (rewrite it
  without the link). Convert only the touched sections first, as their own
  changes. The rest of each document stays with `design-records-k12` and
  `book-specs-k14`. This leaf also edits the passage in
  `docs/specs/walkthrough-books.md` (around line 978) that cites the removed
  book's chapter. The leaf already edits that document for the row.
- A search of the tree for `keyed-launch` and `keyed_launch`, excluding
  `.grove/`, finds matches only in these classes. Enumerate the matches, and
  classify each one. Show that the search finds a seeded match.
  - Code, manifests, tasks, tests, scripts and books: none.
  - Dated evidence and released changelog entries: kept.
  - Current records that a later leaf owns: kept, and listed by file with the
    owning leaf in the running log. The owners are `design-records-k12`
    (`CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`, `module-decomposition.md`,
    `one-live-driver-per-working-tree.md`, `grove-delivery-contraction.md`,
    `diagrams.json`) and `book-specs-k14` (the structure specs and
    `user-guide-coverage.md`).
- `bash scripts/check.sh` passes. `ravel:activation` passes with `RAVEL` set.

## Notes

- Dated evidence stays, as `docs/adr/evidence-outlives-the-instrument.md`
  requires. The files are `docs/ravel-caller-surface-audit.md`,
  `docs/ravel-caller-surface-inventory.tsv`,
  `docs/grove-ravel-docs-evidence.md`, `docs/preservation-baseline.md` and
  `docs/grove-session-control-evidence.md`.
- Rework of the remaining records is in the next three leaves. Prose in
  `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and the other records still names
  the crate when this leaf ends. Only links are not allowed to dangle.
- Structure specs are not gated by a content test, but the Markdown reference
  test scans them for links. This is why the removed spec's inbound link is
  fixed here.
- Convert each edited record as its own change before the edit.
