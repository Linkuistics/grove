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
- A search of the whole tree for `keyed-launch` and `keyed_launch` finds only
  dated evidence and released changelog entries. Enumerate the matches, and
  classify each one. Show that the search finds a seeded match.
- `bash scripts/check.sh` passes. `ravel:activation` passes with `RAVEL` set.

## Notes

- Dated evidence stays, as `docs/adr/evidence-outlives-the-instrument.md`
  requires. The files are `docs/ravel-caller-surface-audit.md`,
  `docs/ravel-caller-surface-inventory.tsv`,
  `docs/grove-ravel-docs-evidence.md`, `docs/preservation-baseline.md` and
  `docs/grove-session-control-evidence.md`.
- Rework of the remaining records is in the next three leaves. Prose in
  `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and the other records still names
  the crate when this leaf ends.
- Convert each edited record as its own change before the edit.
