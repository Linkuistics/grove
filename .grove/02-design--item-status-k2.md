# item-status-k2

## Goal

Design the agreed full-width viewer and explicit lifecycle/RUNNING/NEXT display,
including the typed runtime-observation seam needed to identify the launched
item truthfully. Record the durable behavioral design and any qualifying change
to the existing driver-ownership decision.

## Context

The root brief holds the human-agreed requirements and test seams. `plan-k1`
records the choices and source anchors; synthesise them without re-interviewing
the human. Read the driver-lease ADR cited by the brief before changing the
runtime protocol.

The current `EpochRecord` carries `ProcessRecord` and an optional signal path.
`launch_configured_session` receives a `Selection` but passes only the signal
path into epoch activation. The viewer's `Row` stores a preformatted label, and
`Viewer::render` divides the frame into tree/file columns. These facts locate the
design seams; they do not prescribe the implementation.

Resolve these concrete design questions:

- How does the driver's current mandate expose item identity and tree lifetime,
  with accurate start/end/handoff observation and no change to session-admission
  authority? Define what the observer can establish during launch failure,
  retirement, decomposition, disappearance, driver death and root replacement.
- What read-only, nonblocking typed operation lets a viewer observe activity
  without creating control files, resolving launch configuration or acquiring
  driver ownership? Distinguish idle from busy, unavailable or stale observations;
  handle older/missing metadata and non-jj trees without blocking browsing.
- Where is the shared selection rule expressed so NEXT applies normal order and
  finish eligibility to candidates after excluding the running item? Keep that
  rule consistent with the driver rather than recreating it from display labels.
- How do row data, the fixed status area, the persistent activity summary and
  full-width view switching fit the existing application seam? Account for deep
  indentation, long handles, selected-row styling and saved reading positions.

## Done when

- A coherent, current-state spec describes the agreed viewer behavior and the
  observation/selection interfaces. It cites the applicable ADR rather than
  duplicating it. Use `docs/specs/item-status.md` if no existing spec owns this
  area; reconcile the existing spec set before choosing the location.
- Any load-bearing runtime decision that meets the ADR test is recorded in the
  minimum coherent ADR set. The observation protocol defines freshness,
  ownership, guard lifetime and failure behavior sufficiently for implementation.
- The human-agreed test seams are mapped to acceptance scenarios, including
  view switching and real process-lifetime failures. Prefer existing application,
  driver and terminal fixtures; do not introduce a test-only status implementation.
- Remaining work has a concrete Grove handoff. Follow the design skill's boundary
  for review and planning; implementation leaves are planning's responsibility.

## Notes

The current source was explored at codebase-memory Tier 2 in project
`Users-antony-Development-grove.make-item-status-obvious-in-tui`, generation
`2026-09-15T11:44:22Z`. Relevant code paths had no recorded coverage gaps and
matched metadata; docs were excluded from the fast index and were read directly.
Reconfirm freshness for this session. The initial broad searches were narrowed
to the exact source anchors in `plan-k1`; they were not used to claim repository
completeness.

This design is needed because true running-item identity crosses the driver and
viewer boundary. The requirements session changed only the task tree and did
not implement or test application code.
