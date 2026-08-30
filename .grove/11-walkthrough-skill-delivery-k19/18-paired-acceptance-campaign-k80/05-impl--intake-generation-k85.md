# intake-generation-k85

## Goal

Produce the complete frozen paired execution record for walkthrough intake
without adjudicating any behavioral row.

## Context

- Sole execution authority: the committed manifest from `campaign-freeze-k84`
  and its settled review chain.

## Done when

- The manifest, verified treatment digest, arm request bodies, runtime identity, and
  absence of any pre-freeze evaluated output are revalidated before launch.
- Every planned control/enabled intake pair runs back-to-back in its frozen
  order with byte-identical user messages and no declared tools. Controls omit
  treatment; enabled requests carry the verified treatment through the reviewed
  authoritative field before the user message.
- Complete attempt histories preserve assignment, pair id, timestamps, prompt
  and request digests, authoritative delivery-receipt evidence, emitted-assistant-token
  state, raw events, final output, manifests, access evidence, invalidity reason, and operator
  declaration where a human performs the bounded action.
- Replacement and resumption use only the deterministic frozen taxonomy and
  resource windows. A delivery mismatch invalidates the apparatus; carrier
  failures alone are replaceable, and any model output is retained as behavior.
- The leaf publishes a terminal complete, protocol-failed, or unavailable raw
  surface record without scores, arm guesses, row citations, or a verdict.

## Notes

All three generation leaves must retire before any adjudication leaf runs; this
leaf must not inspect or anticipate later scores.

A terminal intake protocol failure or unavailable record does not authorize an
operator to stop the later surfaces; generation continues unless the shared
apparatus itself is invalid.
