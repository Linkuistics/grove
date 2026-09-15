# item-status-k4

## Goal

Turn the reviewed item-status design into small, independently verifiable
implementation slices that deliver the root brief's full-width viewer,
lifecycle cues and truthful RUNNING/NEXT behavior.

## Context

The behavioral and runtime protocol contract is `docs/specs/item-status.md`;
its cited driver-lease ADR owns the observation decision and trade-offs.
The initial producer is `item-status-k2`, reviewed by `item-status-k3` and
integrated by `item-status-k5`. The process-death correction is `item-status-k6`,
reviewed by `item-status-k7`. Read both review chains and any integration of the
correction before cutting implementation work; findings are evidence to triage,
not this leaf's charter. Plan against the resulting current spec, including its
verified tree relation and witness lifetime contract.

The typed observation seam crosses the existing loop, VCS namespace and generic
runner interfaces. Viewport switching and lifecycle rendering use the public
Viewer application seam. Selection must share the driver's rule and exclude
the running item before applying finish eligibility. The root brief and spec
carry the acceptance scenarios and the human's leading ✓/✗ refinement. The last
color refinement highlights RUNNING, with ordinary LIVE text neutral and
DONE/ABANDONED keeping their status colors.

Source/test starting points: `crates/grove-tui/src/lib.rs` and
`crates/grove-tui/src/observation.rs`; `crates/grove-tui/tests/browser.rs`;
`crates/grove-loop/src/driver_lease.rs`, `loop_driver.rs`, `task_tree.rs` and
`crates/grove-loop/tests/driver_lease.rs`; `crates/jj-workspace/src/lib.rs`;
`crates/keyed-launch/src/run.rs` and its launch fixtures. Reconfirm graph
freshness and coverage before structural claims.

The design session changes prose only. Runtime events, session witnesses,
`try_observe`, common exclusion-aware selection, row types and view switching
are design contracts, not implemented interfaces. Current usage and the viewer
architecture still describe the shipped starting behavior. Plan their updates
with the implementation; also reconcile module and context-map summaries then.

## Done when

The tree contains concrete vertical implementation slices with their acceptance
scenarios, existing test seams and appropriate review placement. Every slice can
be verified on its own; no test-only status implementation duplicates runtime
observation. Include real process-lifetime and multi-viewer controls, the
read-only/no-configuration contract, 60 × 10 presentation and saved viewports,
and the user/architecture documentation needed to finish the root workstream.

## Decisions (running log)

The working increments are full-width navigation, typed lifecycle rows, an
honest idle NEXT forecast, then witnessed RUNNING/NEXT. Each leaves a usable
viewer and includes its own tests and current-behavior documentation. The first
two fit individual implementation leaves. Idle NEXT is also a complete vertical
slice: the shared selector, read-only typed observation and visible forecast
land together. An existing active epoch without witness metadata yields
Unavailable, so this intermediate release never guesses whether a task runs.

Witnessed activity is a separate, larger increment. Its namespace, runner
events, lease-owned witnesses, verified tree relation and viewer consumer must
deliver behavior together; publishing a schema with no reader is not a working
increment. Give that increment its own node and a planning child, which cuts
the implementation leaves against the landed idle-observation seam. Record its
complete acceptance obligations now rather than speculatively slicing a future
interface. No standalone documentation or test-only catch-up stage is needed.

The integrated spec, including both review chains through item-status-k8,
remains the contract. Preserve the directory-before-private probe and
epoch-before-preparation rules; real process controls do not replace forced
identity-reuse barriers. A planning review is warranted before implementation
consumes these boundaries. Implementation producers commission their own review
only after an artifact exists; witnessed activity requires adversarial protocol
review and real Linux and macOS process evidence before its node can close.

Created full-width-view-k9, lifecycle-rows-k10 and idle-next-k11 as concrete
implementation leaves, plus witnessed-activity-k12 with planning child
witnessed-activity-k13. Each leaf includes its own tests and documentation;
the node carries the final protocol controls in full. item-status-k14 is
inserted before the first implementation so its review can still change this
decomposition before consumption. No integration or in-session reviewer is
pre-created. The root remains live, so retiring this planning leaf closes no
ancestor.

Source verification used Tier 2, graph generation 2026-09-15T11:44:22Z. All
relied-on Rust and test paths had matching metadata with no recorded gaps.
Docs/scripts were excluded and prose metadata changed; those sources were read
directly. Exact source confirmed the renderer, row formatting, selection,
namespace creation and launch/reap boundaries; heuristic cross-crate call edges
were not treated as proof. Query pagination used for these anchors completed.
