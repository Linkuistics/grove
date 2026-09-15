# typed-observation-k18


## Goal
Ship the parent's typed try_observe operation with idle and legacy-active
compatibility results, consumed by the viewer's existing tree browsing path.



## Context
Consume discover-namespace-k17 and shared-selection-k16. Read the parent's
full Done when and docs/specs/item-status.md. Current capture/capture_once and
Root are in grove-tui/src/observation.rs; runtime parsing and lock barriers are
in grove-loop/src/driver_lease.rs. Reuse those mechanisms, not admission.

## Done when
- Move production tree/file capture and the retained opaque TreeLifetime behind
  try_observe; migrate existing viewer browsing to consume it. Keep two-capture
  acceptance and UI state in the viewer. No duplicate production observer.
- Return independent typed tree/activity results. Implement every idle,
  legacy-active Unavailable, malformed/mismatched record and Busy path required
  by the parent, using exact discovery and mandatory parser/identity logic.
- Prove read-only, nonblocking, close-on-exec, regular-file/directory checks,
  64 KiB bounds and eight identity retries. Never probe the lease lock, acquire
  ownership/admission, resolve launch configuration or create controls.
- Land the parent's after-capture and in-epoch-guard pause controls, bounded
  handoff/recovery assertions, multi-observer cases and stale-admission
  regressions. All advisory guards are released before return.
- Update landed seam ownership and waiting/handoff/restart guidance in USAGE,
  ARCHITECTURE, module-decomposition and CONTEXT-MAP. Repair affected grove-loop
  and jj-workspace walkthroughs in the same artifact; include new corpus files.
- Focused loop/tui/workspace tests and bash scripts/check.sh pass. Commission
  review-impl with bare stem idle-next once this load-bearing artifact exists,
  naming this producer and the parent contract. The review owns any integration.

## Notes
The next child owns visible activity chrome, idle NEXT attachments, independent
tree/activity consistency acceptance and end-to-end Viewer tests. Preserve
working browsing now; do not publish unused witness records or invent RUNNING.
