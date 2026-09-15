# idle-next-k11 — brief


## Goal

Ship an honest NEXT forecast when no session is running, with typed and
read-only activity observation using shared validated selection. This working
increment consumes full-width-view-k9, lifecycle-rows-k10 and shared-selection-k16.



## Context

Implement the idle and compatibility paths of One typed observation operation,
One bounded observation and One selection rule in `docs/specs/item-status.md`.
Older active epochs with no supported observation metadata must be Unavailable;
do not infer running or idle from PID, cursor, first live leaf or lease bytes.
Witnessed RUNNING is the next increment, witnessed-activity-k12.

Consume shared-selection-k16's landed typed selector for the viewer's forecast.
Read that predecessor's implementation and any review/integration; do not
recreate validation or selection. The renderer must not own a pick algorithm.

`Workspace::control_dir` in `crates/jj-workspace/src/lib.rs` creates a namespace;
add exact-workspace read-only discovery sharing its derivation/validation.
Reuse mandatory record parsing and identity logic from
`crates/grove-loop/src/driver_lease.rs` without calling admission or acquiring
driver ownership. The existing observer's capture/capture_once and root-pin
behavior move behind the typed loop operation as required, while two-capture
acceptance and in-memory UI state remain viewer responsibilities.

## Done when

- Idle NEXT uses the landed common selector without exclusion, preserves its
  validation and finish ordering, and never allocates a finish sentinel.
  Application tests demonstrate its forecast and malformed-tree refusal;
  shared-selection-k16 owns the full selector test matrix and driver refusal.
- The production `try_observe` seam returns independent typed tree/activity
  results and captured selected-file bytes with a retained opaque TreeLifetime;
  all advisory guards are released before return. Tree/file capture finishes
  before the short runtime-only epoch guard. Activity failure preserves a
  readable tree; a tree error prevents row attachments and NEXT.
- Exact non-jj locations, missing namespaces/leases and matching inactive
  epochs yield Idle. Existing lease with absent, malformed, mismatched or
  unreadable epoch yields Unavailable; epoch contention yields Busy. Active
  epochs without supported witness evidence yield Unavailable. All opens and
  probes are read-only, nonblocking and close-on-exec, validate descriptor/path
  identity, bound record reads to 64 KiB and identity-race retries to eight,
  and handle FIFO/directory substitution promptly. Do not probe the lease lock.
- Alias paths to the same exact workspace work by device/inode; a subdirectory
  never borrows ancestor runtime. No jj subprocess, launch configuration,
  ambient session admission, file creation, control cleanup or persisted viewer
  state is introduced. Existing mandatory session admission remains unchanged.
- In either view, chrome reserves one location/view line, one tree-observation
  line, one RUNNING line, one NEXT line and the footer. At 60 × 10 the bordered
  body still has three content rows. Idle shows no running item and the common
  selector's NEXT row plus summary; the NEXT word alone is bold normal text.
  Hidden/folded/offscreen NEXT stays in the summary. Labels and freshness
  qualifiers survive elision before handle slugs, preserving keys where possible.
- Busy/Unavailable and failed tree capture withhold the current pair and clear
  old row activity. Tree consistency and activity consistency are compared
  separately across the existing two captures. Unstable activity cannot discard
  a consistent tree; any retained description is explicitly stale. Both views,
  help and undersized frames retry on the existing deadline, with at most one
  retry and responsive navigation/quit.
- Tests use the same typed operation as production, temporary trees and the
  public Viewer seam, existing lease/process/lock barriers, and filesystem
  snapshots including the administration area. Cover idle-to-active legacy
  epoch transitions, contention/recovery, multi-viewer shared reads, aliases,
  no-configuration launch bypass and all read-only cases above. Current driver
  fixtures still reject stale admission after rotation.
- Pause the observer after tree/file capture and separately inside the runtime
  epoch guard, using readiness/lock barriers and bounded failure timeouts.
  Establish that no epoch guard spans capture or caller work; only suspension
  inside runtime read can reach the documented driver handoff bound, and release
  permits recovery. This control lands here and k12 retains it as a regression.
- Update viewing guidance in `docs/USAGE.md`, preserving the predecessor's
  duplicate-key repair guidance. Explain ordinary overlap's waiting diagnostic,
  a suspended runtime reader's 30-second handoff failure bound, and restarting
  the driver after that holder releases its guard. Update
  `docs/ARCHITECTURE.md`, `docs/specs/module-decomposition.md` and
  `CONTEXT-MAP.md` for the landed seam and its legacy-active Unavailable behavior.
  Reconcile G6 in `docs/specs/user-guide-coverage.md` for shipped idle activity,
  retaining the remaining witnessed-activity obligation.
  Keep the full witnessed protocol documented as the next increment.
- Update affected explained fragments, source/concept indexes and manifests in
  `docs/walkthroughs/grove-loop/` and `docs/walkthroughs/jj-workspace/` in the
  same change, following their authoring rules. Include every newly added
  grove-loop source file matched by the corpus; budget this work with namespace
  discovery and observation rather than leaving it to a later cleanup.
- Run focused tests for grove-tui, grove-loop and jj-workspace, then the root
  brief's principal checks. This increment must demonstrate visible idle NEXT
  and preserve working browsing while a current older driver is active.

## Notes

This is a complete compatibility behavior, not a test-only activity provider.
Do not add unconsumed witness records, fake RUNNING rows or a second production
observation implementation. If the work exceeds one session, decompose at an
independently verifiable seam while retaining this increment's end-to-end exit
condition. The typed observation change is load-bearing: when its
artifact exists, commission review-impl with the bare stem idle-next. Keep
integration adjacent to that review under Grove's normal rule.

## Decomposition

The landed selector is small, but this increment still crosses three distinct
seams with separate verification: namespace discovery, bounded typed capture,
and application acceptance/rendering. Complete only the first child in the
decomposing session. This node keeps every end-to-end condition above.

- discover-namespace-k17 delivers exact read-only namespace discovery through
  the public workspace seam, with independent filesystem tests and book repair.
- typed-observation-k18 consumes that discovery and moves capture behind the
  typed loop operation, including legacy epochs, guard lifetime controls,
  admission regressions and its source-derived documentation.
- idle-activity-view-k19 consumes the operation in the viewer, demonstrating
  idle NEXT, independent consistency, responsive retries and the final chrome.

The typed operation's producer commissions the required `idle-next` review
after its artifact exists. Node closure requires that review and any integration
as well as the shipped viewer behavior; discovery alone completes neither.

## Landed typed observation

captured-tree-k20 and bounded-runtime-k21 close typed-observation-k18.
`grove_loop::try_observe` now returns `ObservationGuard { tree, activity }`;
`tree` retains the previous Ready/Vacant/Busy/error capture and opaque lifetime.
`activity` independently returns Idle, Busy(reason) or Unavailable(reason).
Tree capture releases its guard before the runtime-only shared epoch read,
and no advisory guard escapes. idle-activity-view-k19 consumes both results for visible idle NEXT, and
idle-next-k23 integrates the required idle-next-k22 review. Tree errors and
absence preserve independently compared activity; NEXT requires an accepted
tree. The active-record diagnostic makes no claim that a session is alive.
F3's uninjected race windows are an accepted test limitation documented in the
book, and F4's repeated shared selection is retained. Neither is deferred work
for witnessed-activity-k13.

Runtime compatibility, descriptor/type/size/race bounds, concurrent observers,
after-capture and in-epoch barriers, handoff timeout/recovery, and stale admission
are exercised through the typed operation and existing driver seams. The timeout
control advances the driver's clock seam to 30 seconds rather than waiting in
wall time. Preserve these controls when adding witnessed activity.
