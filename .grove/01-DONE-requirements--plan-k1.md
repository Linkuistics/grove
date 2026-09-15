# plan-k1

## Goal

Make each item's lifecycle state and running/next activity obvious at a glance
in the TUI, and replace side-by-side panes with a pane-switching interface that
uses the available terminal width.

## Context

The initial task and root brief were empty. The workstream name supplied the
starting topic; the requirements were established with the human here.

Source anchors for the next session:

- `Viewer::render` and `Viewer::act` in `crates/grove-tui/src/lib.rs`.
- `Row` and `capture_once` in `crates/grove-tui/src/observation.rs`.
- `selected` in `crates/grove-loop/src/task_tree.rs`.
- `EpochRecord`, `ProcessRecord` and `probe_live_lease_with_post_unlock_hook`
  in `crates/grove-loop/src/driver_lease.rs`.
- `launch_configured_session` in `crates/grove-loop/src/loop_driver.rs`.
- Existing application tests in `crates/grove-tui/tests/browser.rs`, and driver
  process fixtures in `crates/grove-loop/tests/driver_lease.rs`.

## Done when

The human's lifecycle/activity and pane-switching requirements are recorded,
NEXT semantics and test seams are agreed, and the root brief carries the
acceptance criteria into a concrete follow-on design leaf. This leaf establishes
requirements; completion of the root workstream requires the shipped changes.

## Notes

The current viewer appends lifecycle text after each handle and kind. Long rows
can hide that text beyond the tree pane. Existing application tests exercise
`Viewer::new`, `act`, `tick` and `render` using Ratatui's TestBackend.

## Decisions (running log)

The human chose **both lifecycle state and running/next** when asked which
information should be visible at a glance. The change must expose both kinds of
information; showing lifecycle outcomes alone would not meet the request.

The human added: **"We should also have a pane switch UX rather than
side-by-side, because of the tree depth and display width issue."** Full-width
switchable Tree and File views are in scope. Preserve the user's place while
switching views; using the existing Tab action is the proposed interaction.

The human chose **"Next eligible task excluding the running task"** for NEXT.
Recompute it from the current tree as that tree changes. When idle, use the first
eligible live leaf; during a session, exclude the launched item even while its
leaf remains live. Grove's finish eligibility still applies.

The human confirmed **"Yes, requirements and test seams agreed"** after
reviewing the proposed root brief. This agrees the full-width Tab interaction,
left-hand text/color status cues, persistent activity summary, detailed running
and NEXT semantics, and the viewer/driver test seams recorded there. Hand the
runtime-observation design to the next Grove session; no implementation is part
of this requirements leaf.
