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
- `user-guide-coverage.md` no longer cites `keyed_launch::reraise`. It names the
  new home of `reraise`.
- Every edited spec passes the technical-communication checker. Convert each
  spec, or only the sections that the edit touches, as its own change before
  the edit.
- A search of the whole tree for `keyed-launch` and `keyed_launch` finds only
  dated evidence and released changelog entries. Enumerate the matches, and
  classify each one.
- `bash scripts/check.sh` passes. The grove is ready to finish.

## Notes

- A structure spec can go stale at the leaf that changes its chapter. The
  earlier leaves leave these specs alone on purpose, so that each spec
  converts once. If an earlier leaf could not wait, it records the reason in
  its running log.
