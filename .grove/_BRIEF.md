# grove.make-item-status-obvious-in-tui — brief

## Goal

Make item lifecycle and running/next activity obvious at a glance in `grove view`.
Use switchable full-width Tree and File views so deep trees and long handles
have the terminal's available width.

## Requirements

The human explicitly requested both lifecycle state and running/next activity,
then added pane switching because the side-by-side layout restricts tree depth
and display width. NEXT means the next eligible task excluding the running one.

The human agreed the following requirements and test seams in `plan-k1`.

### Full-width views

- Start in Tree view. Show one full-width content view at a time. Tab switches
  between Tree and the selected item's File view; make the active view and the
  switch key discoverable in the screen chrome and help.
- Keep tree selection, expansion and viewport when returning from File view.
  Keep each file's reading position and horizontal offset when switching or
  revisiting. The selected root/branch opens its brief; a leaf opens its task.
- Preserve the existing tree navigation and fold/expand keys. Tab opens files
  for any selected item, including branches; there is no need to repurpose Enter
  or Space. File navigation, Markdown rendering, help, refresh and quit retain
  their existing behavior within the active view.
- Automatic observation continues in either view. Switching and resizing
  preserve reading position; tree changes preserve selection by permanent key
  within the same tree lifetime.

### Status visible before the name

- Give each visible row a stable left-hand status area before depth indentation,
  the handle and the kind. Long names and deep indentation must not clip the
  lifecycle or activity cue. Bound the remaining indentation as needed to leave
  recognizable item text at the existing minimum supported terminal size.
- Keep explicit lifecycle words: LIVE, DONE and ABANDONED for leaves, and the
  existing aggregate LIVE/DONE/ABANDONED/EMPTY meaning for root and branches.
  Branch counts still include all descendants, including folded ones.
- Use color as a secondary cue; text remains sufficient without color. Cursor
  selection, expansion and activity must be visually distinguishable. The
  selected row must retain a readable lifecycle and activity indication.
- Use a tick (✓) for DONE and a cross (✗) for ABANDONED, alongside distinct
  text colors for DONE and ABANDONED. Ordinary LIVE items use normal text;
  highlight the currently RUNNING item. Put the tick/cross at the start of the
  label, before the lifecycle word, indentation and item name. Keep the explicit
  words and the separate cursor-selection cue.
- RUNNING and NEXT are separate from lifecycle. A task may be DONE while its
  launched session is still finishing; retain both facts until that session ends.

### Running and next

- RUNNING identifies the item named by the live driver's current session mandate.
  Do not infer it from cursor selection, the first live leaf, a PID alone, or
  leftover control-file bytes. The driver's existing lease/epoch ownership
  contract remains authoritative.
- At most one item is directly RUNNING. Follow its permanent key through
  renumbering and renaming; if it becomes a branch through decomposition, the
  mandate still belongs to that item. Never transfer the indication to a reused
  key in a replacement tree. If the item is absent, report that fact in the
  activity summary instead of attaching its activity to another row.
- At most one leaf is NEXT: compute Grove's selection over the current eligible
  live leaves with the running item excluded. If no session is running, compute
  the ordinary next selection. Apply depth-first position order and the finish
  exception to the remaining candidates, including the case where only finish
  remains after excluding the last running ordinary task. Recompute on tree
  changes; this is a forecast from the current tree, not a promised future launch.
- Keep a concise RUNNING/NEXT summary visible in both views, so folding a branch,
  scrolling an activity row out of view, or reading a file does not hide activity.
- Observe start, end, handoff and driver death on the existing live-refresh
  cadence. When runtime observation is contended or unverifiable, indicate that
  activity is unavailable or stale and avoid presenting an old RUNNING/NEXT pair
  as current. Missing runtime metadata must not prevent ordinary tree browsing.

### Observation constraints

The viewer remains read-only and responsive. It must not create a workspace,
driver lease, control directory, tree file, or persisted viewer state; require
launch configuration; acquire driver ownership; or wait behind a long driver
handoff. Continue supporting temporary non-jj trees and multiple viewers.
Runtime activity is ephemeral observation, with no new outcome infix or durable
workflow state under `.grove/`.

## Test seams (agreed)

- Reuse the public `Viewer::new`, `act`, `tick` and `render` application seam
  with temporary trees and Ratatui TestBackend. Check rendered text and styles,
  wide/deep/long-name trees at the supported minimum width, branch aggregates,
  full-width view switching, saved selection/reading positions, resize, hidden
  activity, lifecycle transitions and the read-only filesystem contract.
- Exercise actual driver/lease behavior through existing controlled-launch and
  process fixtures. Cover session start/end, failed launch, handoff, killed
  driver, leftover or malformed records, contention and tree replacement. Assert
  that observation identifies the launched item and cannot manufacture a current
  RUNNING indication from stale records or block navigation/quit.
- Add activity observation behind the highest useful typed Grove seam, shared by
  production observation and application tests. Prefer adapting existing seams
  to a second status implementation inside the viewer. Use existing terminal
  fixtures for the switching key mapping and cleanup where those change.

## Done when

The agreed full-width interaction, lifecycle and RUNNING/NEXT behavior work in
the shipped viewer, are covered through the agreed seams, and the viewer usage
and architecture documentation describe the resulting behavior. The runtime
protocol preserves the existing driver ownership and session-admission guarantees.

## Decomposition

`plan-k1` established the requirements with the human. `item-status-k2` records
the design in `docs/specs/item-status.md` and the driver-lease ADR.
`item-status-k3` reviews that design, followed by any integration it needs;
`item-status-k4` then plans implementation slices. Reliable activity crosses
driver ownership and read-only viewing, so the design is reviewed before those
slices are cut. Integration `item-status-k5` applied the original review and
exposed a process-death release-order gap. `item-status-k6` specifies a directory
witness on the task-root pin and a verified tree relation, reviewed by
`item-status-k7` ahead of `item-status-k4`. That review found the protocol
sound and cut `item-status-k8` to integrate its findings on evidence anchors and
boundary wording. Implementation planning consumes the integrated correction.

### Working increments

These independent handoffs keep the viewer useful after each increment and
limit how many interfaces a session invents at once. A one-session increment
stays a leaf; the larger witnessed increment has its own node and grows its
children when its predecessor lands.

| Increment | Working behavior at handoff |
|---|---|
| full-width-view-k9 | Full-width Tree/File switching preserves both viewports. |
| lifecycle-rows-k10 | Typed leading lifecycle cues and bounded indentation stay readable at 60 × 10. |
| shared-selection-k16 | Driver, pick and viewer share malformed-tree refusal and a validated selector. |
| idle-next-k11 | Shared validated selection and read-only observation show honest NEXT while idle; legacy active epochs show activity unavailable. |
| witnessed-activity-k12 | Real driver witnesses deliver RUNNING and exclusion-aware NEXT through the same observer and viewer. |

witnessed-activity-k13 plans the last increment's session-sized implementation
against the landed idle observer. Its node brief already owns all process,
mutation, application and documentation obligations. There is no standalone
schema publication milestone or separate docs/test catch-up stage. Each producer
documents the behavior it ships; the design remains the final contract.

Integration item-status-k15 split shared-selection-k16 out before idle-next-k11
to bound selection and book repair separately from observation. Each increment
reconciles its shipped portion of G6 in docs/specs/user-guide-coverage.md,
including lifecycle rows; only future behavior remains deferred. idle-next owns
the epoch-guard pause controls and waiting/handoff-bound/restart guidance when
it introduces that behavior; witnessed activity retains them as regressions.
The witnessed node's leaves may be independently verified protocol steps; the
node closes only on complete product behavior. witnessed-activity-k13 resolves
the Linux execution route before implementation, with an explicit human run
handoff if no authorized route is available. Missing platform evidence keeps
its owning leaf live and cannot satisfy node closure.

item-status-k14 reviews the decomposition before consumption. Later implementation reviews
are lazy: a producer cuts one after its artifact exists; a reviewer with findings
places integration where the walk reaches it next. The witnessed protocol must
receive adversarial implementation review before its node closes. Do not add an
in-session reviewer alongside scheduled review.

### Verification and source-derived documentation

Use the public Viewer seam for presentation and observation; the generic runner,
driver/lease and internal lock/filesystem barriers for lifetime and concurrency.
Tests and production share one typed loop observer. Assertions use readiness,
actual reap and lock transitions with bounded failure timeouts. The witnessed
increment requires live positive and forced-reuse mutation controls.

Run focused crate tests while implementing, then `bash scripts/check.sh` after
all edits and before describing each change. This repository principal gate
includes source-derived walkthrough validation. Changes to grove-loop,
jj-workspace or keyed-launch can affect books under `docs/walkthroughs/`; update
affected explained fragments, source/concept indexes and manifests in the same
behavior leaf, using their authoring and validation rules. Do not strand book
repair in later cleanup. No runtime-test claim follows from prose validation
in the planning session.

## Pointers

- `docs/adr/one-live-driver-per-working-tree.md` — driver lease and session epoch.
- `docs/specs/item-status.md` — viewer behavior, typed observation and acceptance
  scenarios, including the human's leading tick/cross and text-color refinement.
- `docs/ARCHITECTURE.md`, Read-only viewer — current application/observation seam.
- `docs/USAGE.md`, Viewing a tree — current interaction and recovery behavior.
- Glossary: Leaf, DONE infix, Pruning, Pick, Permanent key, Driver lease and
  Session epoch in `CONTEXT.md`.

## Notes

The source at the design handoff builds one plain row label with lifecycle after
handle and kind; the renderer allocates 55% of the body to the tree and 45% to the
file. The current epoch record carries a process identity and signal path, but no selected
item identity. A truthful RUNNING indication therefore requires driver support.

### Landed idle observation

The idle-next-k11 subtree delivers production `try_observe` and visible idle
NEXT through the shared selector. The viewer compares runtime activity across
two captures even when tree reading or root opening fails; NEXT and row activity
require an accepted tree. Active legacy epoch records remain Unavailable and do
not prove a live session. The guard-lifetime and handoff controls remain in the
loop tests for witnessed-activity-k12 to preserve.

idle-next-k22 was integrated by idle-next-k23. Its F3 limitation is accepted:
alias retargeting during discovery and replacement immediately before locking
or after copying have source-order checks but no deterministic injection tests.
The guarded replacement window and eight-attempt bound are tested. F4's duplicate
selector call is retained; both calls use the same snapshot and shared rule.
Neither finding leaves a repair obligation for witnessed-activity-k13.
