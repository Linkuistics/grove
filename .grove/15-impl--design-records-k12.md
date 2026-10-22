# design-records-k12

## Goal

Rework the design records that state the reversed decision, so that they
describe the shipped arrangement. The code and the books are final by now.

## Context

- The root brief: *Records to rework*.
- The shipped code: the new module of `grove-loop` and its interface, from the
  running logs of `ravel-job-k7`, `grove-run-job-k8` and `lifecycle-job-k9`.
- `docs/adr/the-launched-child-is-a-job.md` and
  `docs/adr/dispatch-supervises-the-harness.md` are already reworked. Read them
  so that no record contradicts them.

## Done when

- `docs/specs/module-decomposition.md`: decision 1, with its table and prose,
  describes the crates that remain. Decision 7 records the shipped interface.
- `docs/adr/one-live-driver-per-working-tree.md`: the passage on orphaned
  processes credits the guarantee of Ravel with the reap of the group. The
  "two generic runner events" are the two notices of the new module.
- The entry *Run ending* in `CONTEXT.md`, and the rows and paragraphs of
  `CONTEXT-MAP.md` on the crate, including "no surviving group".
- `docs/ARCHITECTURE.md` and `docs/grove-delivery-contraction.md` describe the
  arrangement without the crate.
- The current-discussion panel is removed from
  `docs/design/harness-selection-and-execution/diagrams.json`. The file parses.
- A search of the records that this leaf owns finds no statement of the
  reversed decision: `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`,
  `docs/grove-delivery-contraction.md`, `module-decomposition.md`,
  `one-live-driver-per-working-tree.md` and `diagrams.json`. The pattern covers
  `keyed-launch`, `keyed_launch`, `survivor`, `surviving`, "group is gone" and
  "runner". Enumerate the matches, and classify each one. Show that the
  search finds a seeded match.
- Records that `user-records-k13` and `book-specs-k14` still own may keep their
  matches. List them by file in the running log.
- Every edited record passes the technical-communication checker. Convert each
  record as its own change before the edit. A large document converts only the
  sections that the edit touches.
- `bash scripts/check.sh` passes.

## Notes

- Describe the current arrangement on its own terms. Do not carry the old
  scheme as contrast. The ADRs and the changelog hold the history.
- Follow `docs/adr` rules for a changed decision: edit in place, and chase the
  citations. `ADR-FORMAT.md` of the `grove` skill states them.
