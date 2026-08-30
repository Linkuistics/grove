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

## Decisions (running log)

Partition the three owned source files into twelve intent-named literal
fragments: operation inputs, append, insert, promote, rewrite, shared helpers,
effect vocabulary, ordered guarding, decisions/refusals, refusal messages,
report ordering, and report diagnostics. Keep the three ledger-owned top-level
blocks as composites so their stable IDs and exact ranges remain unchanged.

Reuse the opening syllabus insert as the complete decision example. Present its
target resolution, highest-first shift, key allocation, sequential plan guard,
and report ordering before cataloguing the remaining operations and refusal
variants.

Explain promotion's duplicate ordinal/key state as a forced consequence of
identity preservation and destination creation, and leave locking, interruption,
and rollback mechanics to `filesystem-interpreter-k16` while stating the local
contract required to understand the plan.

Keep `guarded-plans` and `reports` as subsections of the anchored worked insert
so its required example boundary includes decision guarding and concrete report
meaning. State that sequential folding is not decisive for the pristine example
because complete names differ; its live insert case is a hand-edited duplicated
key-and-parts tree. Qualify append-many atomicity locally by excluding process
death and failed rollback.
