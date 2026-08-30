# intake-generation-k85

## Goal

Produce the complete frozen paired execution record for walkthrough intake
without adjudicating any behavioral row.

## Context

- Sole execution authority: the committed manifest from `campaign-freeze-k84`
  and its settled review chain.

## Done when

- The manifest, verified treatment digest, templates, runtime identity, and
  absence of any pre-freeze evaluated output are revalidated before launch.
- Every planned control/enabled intake pair runs back-to-back in its frozen
  order with byte-identical user prompts. Controls receive no skill; enabled
  contexts receive the verified preload before the prompt.
- Complete attempt histories preserve assignment, pair id, timestamps, prompt
  and template digests, preload evidence, exposure phase, raw events, final
  output, manifests, access evidence, invalidity reason, and operator
  declaration where a human performs the bounded action.
- Replacement and resumption use only the deterministic frozen functions. A
  preload failure invalidates the apparatus; post-delivery non-adherence and
  other post-exposure outcomes are retained exactly as frozen.
- The leaf publishes a terminal complete, protocol-failed, or unavailable raw
  surface record without scores, arm guesses, row citations, or a verdict.

## Notes

All three generation leaves must retire before any adjudication leaf runs; this
leaf must not inspect or anticipate later scores.
