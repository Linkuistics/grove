# harness-selection-and-execution-k10

**Integrates:** harness-selection-and-execution-k9

## Goal

Triage this design review and apply the real findings before planning turns the
current harness-dispatch design into implementation work.

## Context

Read the findings from the committed review `harness-selection-and-execution-k9`,
not from this body. Its reviewed artifact is the whole current design at
`4158f6cb3e8c`, including k7's repairs, against the root brief and the k1/k4/k7/k8
running decisions. The area spec, its three design ADRs, glossary and visual
design remain the artifacts to reconcile. The review distinguishes design
contract gaps from accepted trade-offs and makes no implementation claims.

## Done when

- Every finding has a reasoned disposition in this leaf's running log.
- Real issues are repaired in the existing spec and ADR set, with related
  glossary, views, acceptance cases and planned methodology amendments kept
  coherent. This leaf repairs the design; production fixes and their acceptance
  checks remain implementation work.
- Settled human choices remain settled, and any new human trade-off is presented
  with evidence and a recommendation.
- Planning `harness-selection-and-execution-k6` can consume a coherent current
  design, or any substantial redesign and required review is explicitly placed
  ahead of it.

## Notes

This integration was inserted as the next live sibling after k9 so its citations
are consumed before planning changes the workstream. The inspection-only review
did not run repository checks; this integration owns verification appropriate
to its repairs, using the project's Taskfile workflow.
