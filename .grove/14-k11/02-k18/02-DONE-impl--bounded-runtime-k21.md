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

## Implementation plan
1. Extend the consumed capture with independent tree/activity values.
2. Share mandatory record parsers; add bounded read-only runtime acquisition.
3. Exercise compatibility, invalid controls, identity races, shared observers,
   capture/runtime pauses, driver handoff and stale admission.
4. Reconcile seam/usage documentation and exact-source book fragments; run
   focused suites and the principal gate, commission idle-next review, retire.

## Decisions (running log)
- Runtime parsing lives in a private driver_lease child module so it can reuse
  mandatory grammar and FileIdentity without exposing admission internals.
  Admission retains its existing read and ownership checks.
- ObservationGuard carries independent tree and activity results. Current
  browsing consumes only tree; k19 owns runtime consistency and visible chrome.
- Runtime opens use O_NONBLOCK/O_CLOEXEC and type checks; one eight-attempt
  acquisition loop validates root, namespace, lease and epoch identities.
  No lease-lock probe is performed. Legacy active epochs remain Unavailable.
- Guard controls use the real typed capture and driver acquisition loop with
  deterministic clock advancement for the existing 30-second handoff bound.
  The scheduled idle-next review owns the adversarial pass.
- Pin the workspace before namespace discovery; validate it again afterward so
  a retargeted alias cannot mix a new root with an old namespace's absent lease.
- Verification: focused grove-loop/grove-tui/jj-workspace suites pass; the final
  bash scripts/check.sh run passes all eight checks, including all six books.
  Runtime controls include nine unit tests plus independent-result integration
  coverage. The loop book reconstructs 15 roots and 11,218 source lines.
- Commissioned idle-next-k22 under idle-next-k11. typed-observation-k18's
  conditions are delivered; promoted its consumed API and guard-test obligations
  into idle-next-k11. Visible activity remains owned by k19.
