# decomposed-producer-receipt-implementation-integrate-k27

**Kind:** integrate-review-impl

**Integrates:** decomposed-producer-receipt-implementation-review-k26

## Goal

Verify and apply the implementation review's findings, leaving decomposed
producer handoff complete and internally consistent.

## Context

Read the design artifacts, the implementation diff, and findings in
`decomposed-producer-receipt-implementation-review-k26`. Reproduce each claim
before changing code or documentation.

## Done when

- Every finding is classified and each verified issue is fixed or its trade-off
  made visible.
- Focused and full relevant tests, formatting, and lints pass.
- Receipt parsing, lifecycle semantics, routing warnings, glossary,
  methodology, architecture, and user docs agree.
- No work required by the reviewed decomposed-producer contract remains unnamed
  in the tree.

## Notes

Externalise substantial redesign as a new producer review chain inside this
chain node rather than expanding this integration leaf.

## Review reconciliation

- **I1 — valid/actionable.** Candidate discovery is now infallible: the direct
  leaf is always retained, ancestor walk/name/generation failures become
  `receipt-preparation-failed` plans, and diagnostics materialise only after
  `DONE` lands.
- **I2 — valid/actionable.** Lifecycle closure and guarded receipt validation
  now share `tree_read::live_leaf_paths_unlocked`, which hides the same
  filesystem-kind reconciliation and recursive walk used by `pick`, `resolve`,
  and generation reads. Both foreign kind-mismatch directions are exercised at
  the CLI seam.
- **I3 — valid/actionable.** Relationship and preparation failures now construct
  typed `ReceiptDiagnostic` values at their source; no reason code is recovered
  from rendered prose. The required-nullable deserializer was renamed for its
  field-presence contract.
- **I4 — valid/actionable.** Checkable evidence requires a `DONE` direct producer
  and a `DONE` source session inside a decomposed producer. Forged receipts for
  `ABANDONED` work are uncheckable, and the current-state ADR now says `DONE`
  rather than the broader terminal state.
- **I5 — valid/actionable.** Negative coverage now proves candidate failures
  cannot block `DONE`, kind-mismatched foreign entries cannot silently suppress
  a checkable node handoff, and a terminal linked review remains byte-identical
  and terminal across supported producer reopen/reclose.

## In-session doubt reconciliation

One leaf-wide fresh-context reviewer examined the bounded integration diff
against the advisory lifecycle, shared-classification, typed-diagnostic,
`DONE`-only evidence, and terminal-review contracts. It found no substantive
issues, satisfying the bounded stop condition without another review cycle.
