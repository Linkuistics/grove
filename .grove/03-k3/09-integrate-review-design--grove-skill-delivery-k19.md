# grove-skill-delivery-k19

**Integrates:** grove-skill-delivery-k16

## Goal

Triage the adoption review's findings on Grove's skill-delivery design and
apply the real ones before implementation planning consumes the design.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifacts are Grove's delivery and caller specifications,
the four records they cite, the design evidence and the caller views, at Grove
commit `d0e0a2ee`. The producer task grove-skill-delivery-k8 records the
artifact set. The upstream Ravel contract is commit `b93bb927` in
`/Users/antony/Development/AgenticOrchestration/Ravel`. Product code, owner
configuration and installed skills stay unchanged by this step.

## Done when

- The session classifies each finding as a contract stated unclearly, a real
  issue, a visible trade-off, or noise, and records the reason.
- The session applies each accepted finding in its owning document and reworks
  the record set as `ADR-FORMAT.md` directs, with every citation reconciled.
- The delivery spec, caller spec, records, evidence and views agree with each
  other after the edits. Where a finding needs a Ravel or tap change, the
  session records that handoff rather than editing those repositories here.
- Grove's design validation for links, diagram sources and served resources
  passes, and the task records the commit.
- A finding that demands a redesign rather than a repair becomes a new producer
  review chain beside this leaf, not an edit here.

## Notes

This step may spend one narrow in-session reviewer. Preserve the running
sessions' original ending commands. common-skill-loading-k9 follows this step
and consumes its result. Implementation and harness trials remain later work.
