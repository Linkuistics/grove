# bounded-runtime-k21

## Goal
Extend captured-tree-k20's consumed try_observe operation with independent typed
runtime evidence satisfying every remaining typed-observation-k18 condition.

## Context
Read the full ancestor contracts, docs/specs/item-status.md, discover-namespace-k17
and captured-tree-k20. Use exact read-only namespace discovery and reuse mandatory
driver_lease parser/identity logic without admission or lease-lock probes.

## Done when
- Independent tree/activity results implement absent exact workspace/namespace/
  lease Idle, matching inactive Idle, legacy active Unavailable, malformed or
  mismatched/unreadable Unavailable and contended epoch Busy.
- All control opens validate type and descriptor/path identity, are read-only,
  nonblocking and close-on-exec, with 64 KiB record bounds and eight identity
  attempts. Preserve aliases by device/inode and exact-workspace isolation.
- Tree/file capture precedes the runtime-only epoch guard; all advisory guards
  release before return. Land after-capture and in-epoch pause controls proving
  multi-observer behavior, bounded handoff failure/recovery and stale-admission
  regressions. Retain the parent's no-creation and no-configuration constraints.
- Migrate existing browsing through the extended result without requiring runtime
  success. idle-activity-view-k19 still owns visible activity and separate
  two-capture acceptance of runtime versus tree.
- Update waiting/handoff/restart USAGE guidance, ARCHITECTURE, module-decomposition,
  CONTEXT-MAP and all affected source-derived walkthrough fragments/indexes.
- Focused loop/tui/workspace tests and bash scripts/check.sh pass. Commission
  review-impl with bare stem idle-next under idle-next-k11, naming this producer,
  captured-tree-k20 and typed-observation-k18's full contract. Review owns integration.
