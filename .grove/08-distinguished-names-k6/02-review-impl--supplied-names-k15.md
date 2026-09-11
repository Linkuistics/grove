# supplied-names-k15

**Reviews:** supplied-names-k14

## Goal

Adversarially review the supplied distinguished-name API boundary before the
following enforcement child builds on it.

## Context

The producer commit is identified by the `supplied-names` child handle above.
Read its diff, the parent brief, `docs/ordinal-fs-tree/ARCHITECTURE.md`, the
entry-name seam ADR and formalism finding 049. This is a scoped API boundary,
not completion of the parent contract: reader/planner level enforcement and
expected-level conformance samples are explicitly assigned to the next child.

## Done when

- Check that caller-supplied initialization/promotion names determine the actual
  destination, preserve bytes, stay confined to one component, and reject
  positioned destinations before effects without weakening guard or rollback
  behavior. Inspect every adapted caller, including Grove and coarse-parts
  equality fixtures, rather than trusting successful compilation alone.
- Check canonical-rendered distinguished identity and retained positioned
  view/species identity, with meaningful regressions and explicit conformance
  samples. Distinct distinguished values are lawful; at-most-one at a level is
  the remaining enforcement child's contract, not a singleton name law.
- Check that source and books describe this boundary consistently. In
  particular, reject a claim of already-enforced level validation: the trait
  method exists, but its reader/planner invocation is the next increment.
- Classify actionable findings with precise source evidence and expected
  behavior. If findings require integration, insert that leaf before the
  enforcement sibling so it consumes a reviewed API.

## Notes

No in-session reviewer was used by the producer. Review only; do not implement
fixes or treat the explicitly queued enforcement work as an unplanned omission.
The installed driver, plugin and live-tree grammar remain untouched.
