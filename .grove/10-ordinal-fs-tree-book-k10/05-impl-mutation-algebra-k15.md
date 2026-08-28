# mutation-algebra-k15

## Goal

Explain the pure mutation algebra as total decisions over one snapshot, including
operation-specific refusals, ordered plans, primitive effects, and reports.

## Context

- Inputs: `read-path-k14`, `book-system-k6`, and this subtree's brief.
- Primary source emphasis: `src/ops.rs`, `src/plan.rs`, and `src/report.rs` as
  assigned by the design ledger.

## Done when

- Append, append-many, insert, promote, and rewrite are derived from their input
  snapshot and shown to return either a guarded plan or a refusal with no third
  outcome.
- One insert is followed completely through target resolution, highest-first
  shifting, key allocation, effect construction, plan guarding, and report
  meaning.
- Promotion's intermediate-state requirements, rewrite idempotency, stable keys,
  and derived ordinal shifts are explained explicitly.
- Every refusal category is located at the decision that creates it and states
  why refusal changes nothing.
- Assigned fragments tangle exactly and scoped source, Markdown/link, model-
  supported claims, and crate verification pass.

## Notes

The algebra cannot reach the filesystem. Preserve that seam in the explanation
as carefully as the code and its guard test preserve it structurally.
