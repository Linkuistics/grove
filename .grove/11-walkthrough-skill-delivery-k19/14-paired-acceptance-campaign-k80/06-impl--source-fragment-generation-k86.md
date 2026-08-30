# source-fragment-generation-k86

## Goal

Produce the complete frozen paired execution record for external source
inventory and fragment planning without adjudicating any behavioral row.

## Context

- Sole execution authority: the committed manifest from `campaign-freeze-k84`
  and its settled review chain.
- Preceding raw record: `intake-generation-k85`.

## Done when

- The exact external non-Rust fixture and every prompt, manifest, treatment,
  template, and runtime digest match the frozen campaign manifest before launch.
- Every planned control/enabled pair runs back-to-back in frozen order with the
  same user-prompt bytes and declared fixture. Access remains within the frozen
  read-only boundary.
- Complete attempt histories preserve assignment, pair id, timestamps, prompt,
  fixture and template digests, preload evidence, exposure phase, raw events,
  final output, manifests, access evidence, invalidity reason, and operator
  declaration where applicable.
- Deterministic replacement and pair-atomic non-selective resumption are applied
  without consulting intake outcomes or any behavioral score. Delivery failure
  and delivered non-adherence retain their distinct frozen meanings.
- The leaf publishes a terminal complete, protocol-failed, or unavailable raw
  surface record without scores, row citations, arm guesses, or a verdict.

## Notes

This surface is bounded cross-codebase and cross-language evidence inside the
accepted instrument. It is not a separate transfer endpoint.
