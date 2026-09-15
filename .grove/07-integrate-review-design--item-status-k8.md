# item-status-k8

**Integrates:** item-status-k7

## Goal

Triage the `item-status-k7` design review's findings against
`docs/specs/item-status.md` and the process-death sections of
`docs/adr/one-live-driver-per-working-tree.md`, and apply the real ones so the
design the `item-status-k4` planning step decomposes is the corrected one.

## Context

Read the findings from the review's own commit, not from this body: they are
in the `## Findings` section of `item-status-k7`'s leaf, each anchored to
`path:line` in the producer's commit `ab1b760685337392feb30aef4e5b3cd29171f9bc`,
with the evidence and the classification the reviewer proposed. Grade each one
yourself as a contract stated unclearly, a real issue, a visible trade-off, or
noise; the review's "What held" list says which attack axes were checked and
passed, and records the real kernel line numbers it read, so they need not be
re-derived. The root brief carries the human-agreed requirements and test
seams.

The spec and ADR are prose; nothing in this leaf runs the viewer or driver.
The spine's `ADR-FORMAT.md` governs any rework of the ADR (edit in place, keep
the set minimal, reconcile every citation). A finding that demands the
observation model be rethought rather than repaired becomes a new producer
review chain beside this leaf, not a fix here; the review proposed none such.

## Done when

Every finding in the review has a recorded disposition in this leaf's running
log, the accepted ones are applied to the spec and ADR with citations
reconciled, the spec and ADR still agree with each other and with the root
brief, and the waiting `item-status-k4` planning step can read a design that
needs no further correction before slicing.

## Notes

The integration may spend one narrow in-session reviewer if a fix is
substantive and non-mechanical; it need not. If it opens the external kernel
links, record that the ranges were read rather than only the local anchors.
