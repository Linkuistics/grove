# harness-selection-and-execution-k4

**Integrates:** harness-selection-and-execution-k2

## Goal

Triage the requirements review's findings against the agreed first-release
requirements, and apply the real ones to the requirements artifact before design
begins.

## Context

Read the findings from the commit of `harness-selection-and-execution-k2`, not
from this body. The reviewed artifact is the committed work of
`harness-selection-and-execution-k1`: the root brief, its running decisions and
`docs/adr/harness-selection-is-owned-by-policy.md`, with
`docs/research/grove-model-effort-routing.md` as starting evidence.

## Done when

- Each finding is classified — contract stated unclearly, real issue, visible
  trade-off, or noise — with the reason recorded in this leaf's running log.
- Accepted findings are applied to the root brief, the ADR or the research record,
  and choices only the human can make are put to the human with a recommendation.
- Settled human choices stay settled; the superseded stronger review-identity
  rules are not restored.
- `harness-selection-and-execution-k3` can start from requirements that need no
  further interview.

## Notes

Several findings ask for a human decision rather than an edit; stopping to ask is
expected for this kind.
