# item-status-k4

## Goal

Turn the reviewed item-status design into small, independently verifiable
implementation slices that deliver the root brief's full-width viewer,
lifecycle cues and truthful RUNNING/NEXT behavior.

## Context

The behavioral and runtime protocol contract is `docs/specs/item-status.md`;
its cited driver-lease ADR owns the observation decision and trade-offs.
The producer is `item-status-k2`, and the design
review is `item-status-k3`. Read that review and any integration it creates
before cutting implementation work; findings are evidence to triage, not this
leaf's charter.

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
