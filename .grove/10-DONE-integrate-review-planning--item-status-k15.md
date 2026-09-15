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

## Decisions (running log)

- F1 — real issue in sizing and explicit ownership, with a qualification: the
  root brief already binds every producer to walkthrough repair, so this was
  not an absent inherited requirement. The book manifests and walkthrough
  spec confirm same-commit source reconstruction. Extract shared validated
  selection ahead of idle-next, including its driver/viewer consumer, tests,
  duplicate-key guidance and grove-loop book. Name the remaining grove-loop
  and jj-workspace book work in idle-next itself. This is a bounded split of
  existing work, not a redesigned plan.
- F2 — contract stated unclearly: the node requires both platform results but
  does not assign responsibility for obtaining Linux execution. No claim about
  all available hosts follows from a repository search. Require k13 to resolve
  an available authorized Linux execution route before cutting implementation;
  if none is available, explicitly hand off the platform run to the human.
  The evidence owner must prepare the exact command, revision and macOS result
  before that handoff, remain live while evidence is missing, and preserve the
  node's Linux acceptance condition. This chooses no unverified host or new
  infrastructure on the human's behalf.
- F3 — real issue. The spec and ADR put the runtime epoch guard and its accepted
  suspension cost in the observer introduced by idle-next. Assign its outside/
  inside pause control and waiting/30-second-bound/restart guidance to that
  leaf; witnessed-activity retains the control as a regression obligation.
- F4 — real ownership gap. G6 explicitly defers these changes to implementation.
  Every shipping slice, including lifecycle rows and the extracted selector,
  must reconcile its portion of docs/specs/user-guide-coverage.md, retaining
  only obligations still deferred. The witnessed node owns the final activity
  reconciliation; unrelated terminal-restoration evidence stays separate.
- F5 — contract stated unclearly. Clarify in both node and planner that leaves
  may deliver independently tested protocol steps, with end-to-end product
  behavior required at node closure. Partial witness metadata remains
  Unavailable and each step carries its tests and source-derived documentation.
  The protocol and node acceptance conditions are unchanged; no new design or
  in-session reviewer is needed for these ownership and sequencing repairs.
- Applied the split as shared-selection-k16, immediately before idle-next-k11.
  Reviewed the complete diff: every removed selector acceptance item moved to
  k16; observation and witnessed controls remain owned. The insert's reported
  position references all belong to k14's commit-anchored historical findings,
  so those citations remain unchanged. Live handoffs use permanent handles.
- Verification: brief-chain with explicit file paths accepts the new selector
  and witnessed planner and returns their expected ancestors; enumeration shows
  contiguous root positions 01–15 and unique keys 1–16. The first brief-chain
  attempt used a handle where that verb requires a file path; the corrected
  calls succeeded. The final diff is confined to planning artifacts under
  .grove/. No viewer, driver, build or principal test gate was run, as this
  leaf's charter directs; no runtime result is claimed.
- Evidence was read from k14's commit ae27cbf1 and the current prose contracts.
  Tier 2 project confirmation reports ready at generation 2026-09-15T11:44:22Z.
  Coverage flags docs/scripts as excluded and task/glossary metadata as changed
  or untracked; direct reads of the relevant prose and manifest/check-script
  ranges supply the evidence. No graph completeness or new code claim is made.
  Root requirements, spec and ADR remain unchanged; the root retains live
  implementation work, so retirement closes no ancestor node.
