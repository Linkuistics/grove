# harness-selection-and-execution-k7

**Integrates:** harness-selection-and-execution-k5

## Goal

Triage the design review's findings against the accepted requirements and
current Grove contracts, and apply the real ones to the harness-dispatch design
before planning cuts implementation increments.

## Context

Read the findings from the commit of `harness-selection-and-execution-k5`, not
from this body. The reviewed artifact is the committed work of
`harness-selection-and-execution-k3`: `docs/specs/harness-selection-and-execution.md`,
`docs/adr/harness-selection-is-owned-by-policy.md`,
`docs/adr/policy-evaluation-precedes-process-replacement.md`, the glossary entries
it added to `CONTEXT.md`, and `docs/design/harness-selection-and-execution/`.
The root brief and the k1/k4 running logs remain the requirements authority.

## Done when

- Each finding is classified — contract stated unclearly, real issue, visible
  trade-off, or noise — with the reason recorded in this leaf's running log.
- Accepted findings are applied to the spec, the ADR set (reworked in place, with
  citations reconciled) and the design evidence; choices only the human can make
  are put to the human with a recommendation and evidence.
- Settled human requirements stay settled; design choices the review questions
  are re-decided on evidence, not reopened as requirements.
- `harness-selection-and-execution-k6` can plan against a design that needs no
  further review, or a substantial redesign is externalised as its own producer
  and review chain ahead of planning.

## Notes

Several findings turn on trade-offs the human has not accepted rather than on
edits alone; stopping to ask is expected for this kind. New runtime claims made
while integrating need their own probes and positive controls rather than
restated documentation.
