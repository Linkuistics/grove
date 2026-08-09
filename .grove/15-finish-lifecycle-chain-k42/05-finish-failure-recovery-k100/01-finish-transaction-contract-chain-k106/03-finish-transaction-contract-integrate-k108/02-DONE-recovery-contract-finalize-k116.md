# recovery-contract-finalize-k116

**Kind:** integrate-review-design

## Goal

Finalize the decomposed design integration after recovery-proof hardening by
reconciling every finding and the complete durable contract.

## Context

- Consume `recovery-proof-hardening-k115`; do not reopen its decisions without
  concrete contradictory evidence.
- Re-read both `finish-transaction-contract-review-k107` and the narrow-review
  summary in the parent brief against the binding spec, ADR, glossary, context
  map, and `finish-transaction-implementation-k110`.
- No in-session reviewer remains for this integration node.

## Done when

- Every original and narrow-review finding has a final, evidence-backed
  disposition in the parent brief, with no stale "fixed" claim contradicted by
  the durable artifacts.
- The minimum coherent spec/ADR/glossary set has no duplicate, dangling, or
  backend-ambiguous contract, and `CONTEXT-MAP.md` remains accurate.
- `finish-transaction-implementation-k110` is an executable handoff for the
  final reviewed design and the parent brief's Done-when holds.

## Notes

This is reconciliation only; newly discovered substantive redesign must be
externalized rather than absorbed.
