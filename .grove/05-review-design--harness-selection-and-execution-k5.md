# harness-selection-and-execution-k5

**Reviews:** harness-selection-and-execution-k3

## Goal

Independently assess the harness-dispatch design against the accepted requirements
and current Grove contracts before planning turns it into implementation work.

## Context

Read the producer's committed artifact via its stable handle. The enduring area
spec is `docs/specs/harness-selection-and-execution.md`; the boundary and runtime
decisions are `docs/adr/harness-selection-is-owned-by-policy.md` and
`docs/adr/policy-evaluation-precedes-process-replacement.md`. Visual sources and
bounded runtime/source evidence are under
`docs/design/harness-selection-and-execution/`. The root brief and k1/k4 decisions
remain the requirements authority; the design is not evidence of new human
approval or of implemented runtime guarantees.

## Done when

- Findings distinguish contract gaps, unnecessary obligations and accepted
  trade-offs, using source/runtime evidence where it bears on the conclusion.
- The whole acceptance surface is assessed, including standalone use, runtime
  delivery, authority, context limits, original creator, records and both process
  seams. The spec/ADR set is coherent and does not claim implementation results.
- If findings warrant integration, create the appropriate integration leaf
  immediately before the later planning sibling; its charter points to this
  review, rather than copying the findings as obligations.

## Notes

Specific producer doubts worth adversarial examination: whether the compiled
worker can exclude ambient config/module injection across supported targets;
whether the scope binding survives live-tree restarts without confusing inode
identity with durable evidence; whether creator registration remains simple and
actually satisfies the provider rule across retries; and whether cancellation,
same-job helpers and exec timing have an implementable contract. Test the design
against these doubts rather than accepting its stated remedies. Linux execution
and final process integration are explicitly unmeasured in this producer.

No extra in-session reviewer competed with this scheduled review. The configured
review-design route was inspected, not launched or modified.
