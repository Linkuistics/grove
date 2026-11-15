# incremental-delivery-k4

**Integrates:** incremental-delivery-k3

## Goal

Triage the findings of review `incremental-delivery-k3` and apply the real ones
to the guidance that producer `incremental-delivery-k2` changed.

## Context

Read the findings from the review's own commit and from its body in
`03-DONE-review-impl--incremental-delivery-k3.md`. Verify each against the
source. Do not take the review's conclusions as given.

## Done when

- The integrating session classifies every finding as an unclear contract, a
  real issue, a visible trade-off or noise. It uses
  `references/integrate-review.md`.
- The session repairs each real finding in the guidance and keeps the repair
  small.
- `bash scripts/check.sh` or the narrower `task check:skills` passes after the
  repair. The session records the result here.
- No further review round is cut unless `references/decompose.md` earns it.

## Notes
