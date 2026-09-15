# item-status-k15

**Integrates:** item-status-k14

## Goal

Triage the `item-status-k14` planning review's findings against the
decomposition `item-status-k4` produced — the root brief's working-increment
sections, `full-width-view-k9`, `lifecycle-rows-k10`, `idle-next-k11`, and the
`witnessed-activity-k12` node with its planning child `witnessed-activity-k13`
— and apply the real ones before the first implementation leaf consumes them.

## Context

Read the findings from the review's own commit, not from this body: they are
in the `## Findings` section of `item-status-k14`'s leaf, each anchored to
`path:line` in the producer's commit `d2202fe693bb`, with the evidence and the
classification the reviewer proposed. Grade each one yourself as a contract
stated unclearly, a real issue, a visible trade-off, or noise; the review's
"What held" list says which attack axes were checked and passed, so they need
not be re-derived. The root brief carries the human-agreed requirements and
test seams; `docs/specs/item-status.md` as integrated through `item-status-k8`
remains the contract and is not this leaf's to change.

The artifacts are task bodies and briefs under `.grove/`; nothing in this leaf
runs the viewer, the driver or the principal checks. Resizing or reordering a
slice is a tree edit — `leaf-insert`, `leaf-add`, or editing a body in place —
and the spine's rules for a good child leaf apply to any leaf this integration
cuts. A finding that asks for the decomposition to be rethought rather than
repaired becomes a new `planning` producer beside this leaf, not a fix here.

## Done when

Every finding in the review has a recorded disposition in this leaf's running
log, the accepted ones are applied to the leaf bodies, node brief and root
brief with the tree still well-formed, and the waiting `full-width-view-k9`
implementation can start from a decomposition that needs no further correction
before consumption.

## Notes

The integration may spend one narrow in-session reviewer if a change is
substantive and non-mechanical; it need not. Positions after this leaf shifted
by one when the review inserted it; the review's citations name the current
entries.
