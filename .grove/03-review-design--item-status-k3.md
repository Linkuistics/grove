# item-status-k3

## Goal

Adversarially review the item-status design produced by `item-status-k2` against
the human-agreed requirements in the root brief. The artifacts are
`docs/specs/item-status.md` and the observation extension in
`docs/adr/one-live-driver-per-working-tree.md`, with their glossary and module
ownership citations. Produce findings; implementation is not this leaf's work.

## Context

Read the producer's committed artifact and the root requirements as the contract.
The human's final refinements require ✓ for DONE and ✗ for ABANDONED at the
start of each label and status colors for terminal items, while explicit words
remain. The last refinement reserves the active highlight for RUNNING; ordinary
LIVE text stays neutral and cursor selection remains independent.

Aim the read at concrete ways the design could fail:

- Can a dead predecessor or another viewer manufacture a current RUNNING
  observation during replacement's old-epoch handoff? Check witness probe mode,
  independent descriptors, lease-owned lifetime and release order.
- Does the generic runner's Started/Reaped contract cover failed spawn,
  immediate exit, supervision errors, escalation and publication failure
  without changing admission or completion authority?
- Can tree replacement, including a first viewer opened after replacement,
  attach an old mandate to a reused key? Check when selection pins the root and
  when that pin can be dropped, including decomposition and disappearance, and
  the observer's final probe after pinning its own tree identity.
- Does `try_observe` actually admit a read-only, nonblocking implementation
  with separate tree/runtime errors, safe guard order and bounded consistency
  checks? Attack stale current pairs and multi-viewer contention.
- Does exclusion happen before finish eligibility without hiding malformed
  input, excluding descendants of a running branch or inventing a sentinel?
- Can the 60 × 10 layout, colors, cursor and glyphs satisfy the requirements
  while preserving both viewports, file source anchors and activity summaries?

Source anchors for feasibility: `DriverLease`, epoch record parsing and
`probe_live_lease_with_post_unlock_hook` in
`crates/grove-loop/src/driver_lease.rs`; `launch_configured_session` and `drive`
in `crates/grove-loop/src/loop_driver.rs`; `selected` in
`crates/grove-loop/src/task_tree.rs`; `run`, `supervise` and `watch` in
`crates/keyed-launch/src/run.rs`; `Workspace` in
`crates/jj-workspace/src/lib.rs`; the Viewer application and observation adapter
in `crates/grove-tui/src/lib.rs` and `observation.rs`.

The producer used graph Tier 2, project
`Users-antony-Development-grove.make-item-status-obvious-in-tui`, generation
`2026-09-15T11:44:22Z`. Those code paths and the browser/driver-lease tests had
matching metadata and no recorded coverage gaps; docs are excluded and were
read directly. Search/trace pages used for the bounded source anchors completed.
Some inferred call edges pointed to unrelated helpers, so exact source governed
material claims. Reconfirm freshness; this is not repository-completeness evidence.

## Done when

The spec, runtime decision and acceptance seams have been checked against the
agreed contract and source feasibility, with actionable findings recorded in
the review's own artifact. If findings need integration, place its leaf ahead
of the waiting `item-status-k4` planning step. A clean review needs no integration
leaf. Keep the spec and ADR set coherent; do not turn preferences into findings.
