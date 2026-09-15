# idle-next-k11


## Goal

Ship an honest NEXT forecast when no session is running, with typed and
read-only activity observation and shared validated selection. This third
working increment consumes full-width-view-k9 and lifecycle-rows-k10. It also
makes the driver refuse ambiguous duplicate keys consistently with the viewer.



## Context

Implement the idle and compatibility paths of One typed observation operation,
One bounded observation and One selection rule in `docs/specs/item-status.md`.
Older active epochs with no supported observation metadata must be Unavailable;
do not infer running or idle from PID, cursor, first live leaf or lease bytes.
Witnessed RUNNING is the next increment, witnessed-activity-k12.

The existing `selected` helper in `crates/grove-loop/src/task_tree.rs` serves
`select_in` and `select_in_write` but only checks multiple live finishes. The
viewer has its own duplicate-key check in `crates/grove-tui/src/observation.rs`.
Move whole-tree validation and selection to the highest useful typed loop seam,
with an optional excluded permanent key, and use it for ordinary driver selection
and the viewer's forecast. The renderer must not own another pick algorithm.

`Workspace::control_dir` in `crates/jj-workspace/src/lib.rs` creates a namespace;
add exact-workspace read-only discovery sharing its derivation/validation.
Reuse mandatory record parsing and identity logic from
`crates/grove-loop/src/driver_lease.rs` without calling admission or acquiring
driver ownership. The existing observer's capture/capture_once and root-pin
behavior move behind the typed loop operation as required, while two-capture
acceptance and in-memory UI state remain viewer responsibilities.

## Done when

- The common selector validates every key, including terminal and branch items,
  and multiple live finishes before exclusion. It then removes only the named
  candidate key, takes the first remaining ordinary leaf in depth-first position
  order, or the sole remaining finish. A running branch's children remain
  eligible. No ordinary driver ordering changes on valid trees, and no viewing
  operation allocates a finish sentinel.
- Selection tests cover no exclusion, excluded ordinary/finish/terminal/branch
  keys, finish-only remainder, early finish with later ordinary work, no
  candidates, duplicate finishes and duplicate keys that exclusion might hide.
  Driver and application tests show the same duplicate-key refusal.
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
- Update selection and viewing guidance in `docs/USAGE.md`, including how to
  repair accidental duplicate keys without reusing permanent identity. Update
  `docs/ARCHITECTURE.md`, `docs/specs/module-decomposition.md` and
  `CONTEXT-MAP.md` for the landed seam and its legacy-active Unavailable behavior.
  Keep the full witnessed protocol documented as the next increment.
- Run focused tests for grove-tui, grove-loop and jj-workspace, then the root
  brief's principal checks. This increment must demonstrate visible idle NEXT
  and preserve working browsing while a current older driver is active.

## Notes

This is a complete compatibility behavior, not a test-only activity provider.
Do not add unconsumed witness records, fake RUNNING rows or a second production
observation implementation. If the work exceeds one session, decompose at an
independently verifiable seam while retaining this increment's end-to-end exit
condition. The typed observation and selector change is load-bearing: when its
artifact exists, commission review-impl with the bare stem idle-next. Keep
integration adjacent to that review under Grove's normal rule.
