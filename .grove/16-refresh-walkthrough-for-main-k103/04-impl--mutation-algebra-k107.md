# mutation-algebra-k107


## Goal

Refresh the pure mutation algebra against the current operations, planning, and
reporting sources.


## Context

- Source roots: `crates/ordinal-fs-tree/src/ops.rs`, `src/plan.rs`, and
  `src/report.rs`.
- Book surfaces: `05-mutation-algebra.md` and `source-index.md`.

## Done when

- All three owned roots tangle byte-for-byte and their inventory entries are
  current.
- Exposition accurately covers the current operation set, decisions,
  refusals, effects, reports, and whole-tree deletion semantics.
- Full validation has no mismatch for an owned root.

## Notes

Keep filesystem interpretation out of this pure-algebra slice.
