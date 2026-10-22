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
