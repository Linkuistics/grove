# finish-failure-recovery-k100

**Kind:** design

## Goal

Design recoverable finish semantics when repository validation, index
preparation, staging, or commit fails after `finish-commit` has removed
`.grove/` from the working tree.

## Context

- Surfaced while integrating `finish-lifecycle-review-k44` F5 and the narrow
  completion review of `finish-lifecycle-integrate-k45`: index state can be
  preserved, but the current caller deletes the task tree before entering the
  repository commit seam.
- Preserve path/fileset-scoped commits, unrelated Git/jj work, the universal
  tree lock, explicit finish confirmation, and the rule that a successful
  finish leaves no `.grove/` in the integrated history.
- Coordinate with `post-teardown-restart-k99`, which owns the distinct crash
  window after a successful deletion commit.

## Done when

- The design states the transaction boundary and recovery behavior for every
  pre-commit and commit failure in plain Git, native jj, and colocated jj.
- A reported failure either restores a live, selectable finish tree or leaves a
  fail-closed recoverable witness with an actionable diagnostic; it never makes
  a failed finish look like a fresh rootless grove.
- The minimum coherent spec/ADR/glossary set records the settled contract, and
  implementation is cut as separate reviewed work inside
  `finish-lifecycle-chain-k42`.

## Notes

Do not fold this lifecycle transaction redesign into the index-preservation
fixes in `finish-lifecycle-integrate-k45`.
