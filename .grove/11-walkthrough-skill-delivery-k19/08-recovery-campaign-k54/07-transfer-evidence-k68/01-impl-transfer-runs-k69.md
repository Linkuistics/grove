# transfer-runs-k69

## Goal

Execute the five frozen external control/enabled pairs without selecting on
discovery, access behavior, or answer quality.

## Context

- Transfer artifacts: `transfer-freeze-k58`.
- Joint runner, auditor contract, schedule, treatment manifest, and outcome
  rules: `campaign-freeze-k59`.

## Done when

- Each assigned pair reaches a complete terminal record with replacements only
  for failures the deterministic exposure-phase gate classifies as
  pre-exposure. Such failures consume the frozen resource budget rather than an
  attempt ceiling.
- Post-exposure violations, refusals, timeouts, truncations, missing finals, and
  non-discovery remain assigned outcomes.
- Fixture byte equality, raw streams, manifests, prompt bytes, runtime identity,
  pair/order, back-to-back per-arm start/end timestamps, complete attempt
  history, and delivery/read/announcement observations are preserved under the
  recovery transfer namespace.
- No score, same-case comparison, target change, criterion change, or treatment
  edit occurs.

## Notes

Between pair-atomic execution windows, automatic resumption continues at the
frozen schedule's earliest incomplete pair without inspecting arm or outcome.
Mid-pair reserve exhaustion makes the pair terminally unavailable rather than
resuming its second arm later. An unavailable record never contributes to
attainment and does not prevent this leaf from retiring coherently.
