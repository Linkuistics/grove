# ravel-extraction-k10

**Integrates:** ravel-extraction-k9

## Goal

Triage the findings of `ravel-extraction-k9` against the committed tree, the
source and the recorded evidence, and reshape the ordered Ravel extraction
leaves so implementation consumes a corrected plan.

## Context

- Read the findings from `ravel-extraction-k9`'s own commit, not from this body.
  Reproduce or derive each from the source before acting; classify it as a
  contract stated unclearly, a real issue, a visible trade-off or noise.
- The artifact is the ordered `ravel-cli-k3` through `grove-session-control-k8`
  leaves and the root brief. Edit leaf bodies and the root brief; do not
  implement any of them here.
- `plan-k1`'s running log and the root brief's human requirements stand. A
  finding that would reopen a settled requirement, or that needs a human
  choice (for example whether the work stays one grove or splits), is handed
  back with a recommendation and its evidence rather than decided here.
- Recheck cited source lines at integration time; the tree may have moved.

## Done when

- Each finding of the review carries a recorded classification and, where real,
  is resolved in the leaf or brief that owns it, or is handed to the human with
  a recommendation.
- Every leaf still lands a working, independently demonstrable handoff and fits
  one focused session after the changes; any split or reorder keeps the root
  brief's table, each leaf's `Requires`, and the cross-project handoff text
  consistent.
- Substantial redesign, if a finding demands it, becomes a new producer review
  chain beside the artifact rather than an unreviewed rewrite here.

## Notes

This leaf sits before `ravel-cli-k3` so that no implementation starts from the
uncorrected tree. It changes planning artifacts only: no source, installation
or release step, and nothing that touches the live session's installed
binaries, configuration or ending channel.
