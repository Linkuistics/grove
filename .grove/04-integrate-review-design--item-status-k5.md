# item-status-k5

**Integrates:** item-status-k3

## Goal

Triage the `item-status-k3` design review's findings against
`docs/specs/item-status.md` and the observation extension in
`docs/adr/one-live-driver-per-working-tree.md`, and apply the real ones so the
design the `item-status-k4` planning step decomposes is the corrected one.

## Context

Read the findings from the review's own commit, not from this body: they are
in the `## Findings` section of `item-status-k3`'s leaf, each anchored to
`path:line` in the producer's commit `d13209e1d39a`, with the source evidence
and the classification the reviewer proposed. Grade each one yourself as a
contract stated unclearly, a real issue, a visible trade-off, or noise; the
review's "What held" list says which attack axes were checked and passed, so
they need not be re-derived. The root brief carries the human-agreed
requirements and test seams, and the review cited them by line.

The spec and ADR are prose; nothing in this leaf runs the viewer or driver. The
spine's `ADR-FORMAT.md` governs any rework of the ADR set (edit in place, keep
the set minimal, reconcile every citation, including `CONTEXT.md`,
`CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and the two specs that point at the
design). A finding that demands the observation model be rethought rather than
repaired becomes a new producer review chain beside this leaf, not a fix here.

## Done when

Every finding in the review has a recorded disposition in this leaf's running
log, the accepted ones are applied to the spec and ADR set with citations
reconciled, the spec and ADR still agree with each other and with the root
brief, and the waiting `item-status-k4` planning step can read a design that
needs no further correction before slicing.

## Notes

The integration may spend one narrow in-session reviewer if a fix is
substantive and non-mechanical; it need not.
