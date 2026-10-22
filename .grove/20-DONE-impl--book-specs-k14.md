# book-specs-k14

## Goal

Bring the book structure specs and the coverage spec into agreement with the
books as the earlier leaves left them. This is the last leaf of the grove.

## Context

- The root brief: *Records to rework*, items for the book structure specs and
  `user-guide-coverage.md`.
- The books after `keyed-launch-removal-k11`: `grove-loop`, `overview` and
  `jj-workspace`. The chapters and slices that the earlier leaves added or
  changed are the facts that the specs must state.
- `docs/specs/grove-loop-book-structure.md`,
  `docs/specs/overview-book-structure.md`,
  `docs/specs/grove-llm-book-structure.md` and
  `docs/specs/user-guide-coverage.md`.

## Done when

- Each structure spec states the chapters, slices and indexes that its book now
  has. No spec cites the removed crate, its book or its types.
- `user-guide-coverage.md` no longer cites `keyed_launch::reraise` (line 83). It
  names the new home of `reraise`. Row L12 (line 104) no longer says "no
  surviving group". It states the post-reap order of the caller contract.
  `crates/grove/tests/user_guide_coverage.rs` passes with the edited spec and
  with the guide that `user-records-k13` edited.
- Every edited spec passes the technical-communication checker. Convert each
  spec, or only the sections that the edit touches, as its own change before
  the edit.
- The final sweep of current records is the instrument of the grove. It searches
  the tree, excluding `.git/`, `.jj/`, `target/` and `.grove/`, for
  `keyed-launch`, `keyed_launch`, `survivor`, `surviving` and `group`. The
  `.grove/` tree is excluded because its task files and notes record the
  removed crate and are not current records. The finish step tears it down.
  Only dated evidence, released changelog entries and unrelated uses remain.
  Enumerate the matches, and classify each one. The launch prompt text
  "only after the group is gone" in `crates/grove-loop/src/prompt.rs` is one
  such classification: Ravel reaps that group.
- The sweep has its controls. A seeded match is found. A deliberately broken
  pattern fails to find a known match, so the instrument rejects broken input.
- `bash scripts/check.sh` passes. The grove is ready to finish.

## Notes

- A structure spec can go stale at the leaf that changes its chapter. The
  earlier leaves leave these specs alone on purpose, so that each spec
  converts once. If an earlier leaf could not wait, it records the reason in
  its running log.

## Decisions (running log)

- Each spec landed as two changes: a conversion of the touched blocks to controlled
  English, then the edit. Untouched text sits in `pending` regions. The coverage
  spec converted its `L` table, because a marker cannot sit inside a table.
- The four specs record the cut as decided, and the corpus moved after it. So each
  gained or kept a statement that the manifest rules where a count differs. The
  `grove-loop` and `overview` specs gained a section, *The book now*, with the
  pages, slices, roots and blocks from the manifest. The overview book has six
  chapters and eight roots, and the spec still described five and three. The
  `grove-loop` book holds eighteen roots, and the spec described twelve.
- The slice ID `four-things-a-runner-cannot-choose` stays. A rename would migrate the
  manifest, the ledgers and every anchor that cites it. The spec says so.
- The history of the removed runner crate in the `grove-loop` spec now says "the local
  runner crate" without its name. The spec keeps the history, as the decision records
  keep theirs.
- The quote of the `lib.rs` header in the `grove-loop` spine was out of date before
  this grove (it quoted "the one library crate … domain-bound"). It now quotes the
  current header.
- `user-guide-coverage.md` names `grove_loop::reraise`, defined in `ravel_job`. Row L12
  states the post-reap order of the caller contract and says Grove probes no process
  group.
- Fallout that the five patterns do not match, found by reading counts. The fixes:
  `release.toml` said "seven crates" (now six). The `grove-loop` manifest said "three
  reusable library crates" (now two, with the identical bytes in chapter 1 of the
  `grove-loop` book). The `grove-llm` book said "five dev-dependencies" (now four, with
  its concept index entry). A loop-driver test comment now says "Ravel's escalation".
- Final sweep, tree minus `.git/`, `.jj/`, `target/` and `.grove/`, for
  `keyed-launch`, `keyed_launch`, `survivor`, `surviving` and `group`. Positive
  control: a seeded file with one line per token returned 5 of 5. Negative control: a
  typo pattern returned 0 of 5 on the same file. The seed was removed.
  - `keyed-launch` and `keyed_launch`: only `CHANGELOG.md` (released entries) and four
    dated evidence files. `docs/grove-session-control-evidence.md` has no match.
  - `survivor` and `surviving`: plain English, test names about viewer and lock-holder
    survivors, and the records that state the removed check as history or as rejected
    (`dispatch-supervises-the-harness.md`, `harness-selection-and-execution.md`).
  - `group` has three classes. The first is Ravel's managed group in the current
    records. The second is the foreground group in job-control code and tests. The
    third is unrelated uses: manifest `[[block]]` groups, regex groups, diagram
    groups, the Lifecycle group of the formal model, the `shared-version` group and
    lint groups. The launch prompt text "only after the group is gone" in
    `prompt.rs` is Ravel's group. `docs/legacy-driver-transition.md` describes the legacy
    supervisor.
- `bash scripts/check.sh` passed with all 20 principal checks.

## Findings for later work

- `crates/grove-llm/tests/removed_surface.rs`: the finding of `keyed-launch-removal-k11`
  stands. The filter in `no_removed_launch_policy_is_named_outside_the_scrub_module`
  tests `starts_with("src")` against repository-relative paths, so it checks nothing.
  `SCRUB_MODULE` also names a file that no longer holds the list. No leaf owns the
  correction.
- The word "runner" remains in prose that describes the removed crate (chapters 16 and
  18 of `grove-loop`, comments in `driver_lease.rs`, Part V of the `grove-loop` spec).
