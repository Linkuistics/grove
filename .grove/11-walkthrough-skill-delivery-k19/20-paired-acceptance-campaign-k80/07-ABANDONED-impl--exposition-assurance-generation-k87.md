# exposition-assurance-generation-k87

## Goal

Produce the complete frozen paired execution record for exposition and
assurance without adjudicating any behavioral row.

## Context

- Sole execution authority: the committed manifest from `campaign-freeze-k84`
  and its settled review chain.
- Preceding raw records: `intake-generation-k85` and
  `source-fragment-generation-k86`.

## Done when

- The manifest, treatment, arm request bodies, prompt, fixtures, runtime identity, and
  preceding raw-record status are verified before launch.
- Every planned control/enabled pair runs back-to-back in frozen order with
  byte-identical user messages, no declared tools, and the verified treatment
  only in the enabled request's authoritative field.
- Complete attempt histories preserve assignment, pair id, timestamps, all
  relevant digests, delivery-receipt evidence, emitted-assistant-token state,
  raw events, final output, manifests, access evidence, invalidity reason, and
  operator declaration where applicable.
- Carrier-only replacement and automatic resource resumption use only frozen deterministic state and
  cannot depend on outcomes from either preceding surface. Post-delivery
  instruction failures remain valid behavioral outcomes.
- The leaf publishes a terminal complete, protocol-failed, or unavailable raw
  surface record without scores, row citations, arm guesses, or a verdict.
- A generation-barrier record proves all three raw surface records and complete
  attempt histories are sealed before `intake-adjudication-k88` may begin.

## Notes

Sealing the barrier is an evidence-ordering control, not an aggregate behavioral
result.

Generation runs after any earlier surface protocol failure or unavailability,
unless the shared apparatus is invalid, so the campaign has no outcome-aware
stop branch.
