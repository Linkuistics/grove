# review-diversity-warning-k16

**Kind:** impl

## Goal

Warn, without gating launch, when a `review-*` session cannot be verified to
differ from its producer on both effective harness and exact model selector.

## Context

Consume producer receipts from `producer-target-receipt-k15` and the existing
kind/family routing lattice. Keep relationship identity stable and never infer a
producer from position or filename convention.

## Done when

- A pure comparison covers same harness, same model, both, neither, null model
  defaults, explicit selectors, and every uncheckable relationship/receipt case.
- Each real review spawn renders one compact diagnostic to stderr and prepends
  the same notice to the session prompt while leaving its resolved command and
  guide-not-gate behavior unchanged.
- Warnings name the review always, name a producer only from a valid `Reviews`
  relation, render default targets clearly, and survive configuration changes
  and driver restarts.
- Focused fake-harness and routing tests are green.

## Notes

Do not reconcile the full methodology/doubt documentation matrix here; the last
child owns the canonical-surface pass after executable behavior is settled.
