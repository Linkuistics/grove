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

## Decisions (running log)

- Landed as two changes. The first converted the touched passages of the books
  and of `docs/specs/walkthrough-books.md` to controlled English. The second
  removed the crate and edited the converted text. Chapter 20 of `grove-loop`
  has one very long paragraph that lists corrected claims. That edit touched one
  sentence, so the conversion covered only that sentence.
- `crates/grove-loop/src/lib.rs` and `Cargo.toml` keep their line counts, so no
  fragment range in the `grove-loop` book moved. The header of `lib.rs` now
  names `ravel_job` as the module that starts `ravel`.
- The gate transcripts in chapter 20 of `grove-loop` and chapter 07 of
  `jj-workspace` lost the two lines of the removed book. They now say `5 book(s)`
  and carry one sentence that names the omission. A kept line would have been a
  `keyed-launch` match in a book.
- `ravel:fixtures` now says "the resolver and Grove's shared result reader".
  `ravel:result` now says "both shapes of the shared launch function". The test
  `both_stdout_runner_apis_read_only_the_installed_session_result` is now
  `both_shapes_read_only_the_installed_session_result`.
- The comment in `removed_surface.rs` lost its history paragraph, because the
  scrub list now lives in `ravel_job`. The `loop_driver.rs` comments say "driver"
  and "the job's terminal recovery" in place of "runner".
- Search for `keyed-launch` and `keyed_launch` outside `.grove/`. Positive
  control: a seeded `keyed_launch` in `crates/grove-loop/seed-control.rs` was
  listed, and the removed file left the list. Code, manifests, tasks, tests,
  scripts and books: no match. The kept matches follow.
  - Released changelog entries: `CHANGELOG.md`. The `Unreleased` section holds none.
  - Dated evidence: `docs/ravel-caller-surface-audit.md`,
    `docs/ravel-caller-surface-inventory.tsv`, `docs/grove-ravel-docs-evidence.md`
    and `docs/preservation-baseline.md`.
  - `design-records-k12`: `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`,
    `docs/specs/module-decomposition.md`, `docs/grove-delivery-contraction.md`
    and `docs/design/harness-selection-and-execution/diagrams.json`.
  - `book-specs-k14`: `docs/specs/grove-loop-book-structure.md`,
    `docs/specs/overview-book-structure.md`,
    `docs/specs/grove-llm-book-structure.md` and `docs/specs/user-guide-coverage.md`.
- `docs/adr/one-live-driver-per-working-tree.md` no longer matches, so
  `design-records-k12` finds that passage already clean of the crate name.

## Findings for later leaves

- `crates/grove-llm/tests/removed_surface.rs`: the filter in
  `no_removed_launch_policy_is_named_outside_the_scrub_module` tests
  `occurrence.file.starts_with("src")`. The sweep reports paths relative to the
  repository root, such as `crates/…`. So the filter never matches and the test
  checks nothing. `SCRUB_MODULE` also names a file that no longer holds the list.
  No leaf of this grove owns the repair.
- The word "runner" remains in prose that describes the removed crate. Examples
  are chapters 16 and 19 of `grove-loop`, `driver_lease.rs` comments and the
  slice name `four-things-a-runner-cannot-choose`. The removal leaf did not
  change them. `book-specs-k14` and `design-records-k12` can sweep them.
