# supplied-names-k17

**Integrates:** supplied-names-k15

## Goal

Triage the findings of the `review-impl` leaf `supplied-names-k15` against the
supplied-name API boundary delivered at `supplied-names-k14`, apply the ones
that are real, and leave the boundary and its books current for
`valid-levels-k16` to consume.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `kyymvozo` (`9d09fb67`) and to `path:line` coordinates in that tree. The
  review's own insert of this leaf shifted `valid-levels-k16` from position 03
  to 04; handles are unchanged.
- The producer's contract is `01-DONE-impl--supplied-names-k14.md` and the
  parent brief in this directory. The reader and projected-level enforcement
  and the expected-level conformance samples belong to `valid-levels-k16`, and
  no finding asks for them here.
- The artifact is code, tests and the two books whose roots this boundary
  changed: `ordinal-fs-tree` and `grove-loop` under `docs/walkthroughs/`, plus
  `CHANGELOG.md`. Each touched source root lands with its fragments, ledgers,
  indexes and prose in this commit; `bash scripts/check.sh` passes on
  unchanged tracked inputs.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- Where a finding names an API decision the enforcement child inherits, the
  decision is settled and recorded here so `valid-levels-k16` builds on it
  rather than reopening it.
- `valid-levels-k16` still follows this leaf; positions and keys are otherwise
  untouched. The installed driver, plugin and live-tree grammar remain
  untouched.

## Notes

- Substantial redesign is not this leaf's: externalise it as a new producer
  review chain beside the leaf being integrated.
- One narrow in-session reviewer is available for a non-mechanical fix; a
  second need is the signal to cut a `review-impl` leaf instead.
