# coordinated-ravel-release-k40

**Integrates:** coordinated-ravel-release-k39

## Goal

Triage the review's findings on the coordinated release contract and apply the
real ones in their owning repositories. Do this before the Rhai contract repair
and the rollout planner consume the design.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifacts are Grove's `docs/specs/coordinated-release.md`,
`docs/coordinated-release-evidence.md` and the two release views, and Ravel's
`docs/specs/release-channels.md` at commit
`80e382223129cc659783b78e6e93bd6d3e391739` on bookmark
`coordinated-release-design` in `Ravel.coordinated-release-design`. Work
there under that repository's instructions and VCS discipline. The producer
task `coordinated-ravel-release-k36` records the artifact set and the owner's
acknowledgement. No release, tap change, owner activation or paused-tree edit
belongs to this step.

## Done when

- The session classifies each finding as a contract stated unclearly, a real
  issue, a visible trade-off, or noise, and records the reason.
- The session applies each accepted finding in its owning document and reworks
  each ADR set as `ADR-FORMAT.md` directs, with every citation reconciled.
- The two specifications, the evidence document, the release views and the root
  and node handoffs agree with each other after the edits.
- Grove's document checks pass, Ravel's isolated workspace is sealed on an empty
  successor, and the task records the exact Ravel commit for k34 and rollout.
- A finding that demands a redesign rather than a repair becomes a new producer
  review chain beside this leaf, not an edit here.

## Notes

This step may spend one narrow in-session reviewer. Preserve every running
session's original ending command. ravel-rhai-skill-delivery-k34 and the
rollout planner consume this step's result.
