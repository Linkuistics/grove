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

## Decisions (running log)

The human added ticks for DONE, crosses for ABANDONED, and lifecycle text color
in Tree view. Use ✓ and ✗ alongside the agreed explicit lifecycle words;
selection styling must preserve both the symbol and readable status text. The
human clarified that the markers belong at the start of the label, before the
lifecycle word, indentation and item name (after the separate cursor gutter).

The existing spec set places module ownership in `module-decomposition` and
guide coverage in `user-guide-coverage`; neither owns viewer behavior. Put the
behavioral contract in `docs/specs/item-status.md` and reconcile those pointers.
The runtime extension belongs in the existing driver-lease ADR, which already
owns the distinction between live locks and leftover records.

Use one typed Grove observation operation to pair a guarded tree reading with
runtime activity. A separate runtime getter would leave every consumer to join
two changing lifetimes correctly; a viewer-owned lock/record implementation would
duplicate admission-adjacent protocol knowledge. The combined operation keeps
that knowledge in grove-loop while allowing tree browsing when activity fails.

An active epoch and matching contended lease are insufficient evidence for
RUNNING: replacement deliberately keeps the predecessor's lease bytes while
waiting for epoch handoff. Bind observational mandate metadata to a fresh
per-launch session witness, locked only by the driver and marked started only
after successful spawn. Releasing that witness at reap or driver death defeats
the stale-predecessor case without changing session admission. The driver also
pins the selected task-tree directory through the witness lifetime so reused
keys in a replacement root cannot acquire the old activity.

Factor selection before display: validate the whole snapshot, exclude only the
running permanent key in the same tree lifetime, then apply ordinary depth-first
order and finish eligibility to the remaining candidates. Do not exclude a
decomposed item's descendants or filter the result of an already-finished pick.

Use a fixed 22-cell cursor/status area at the supported 60-column minimum.
Lifecycle markers begin the label; words and semantic text colors survive
selection. Tree and File retain independent viewport state; activity has two
reserved summary lines in either full-width view.

The runtime claim spans process death, descriptor ownership and epoch handoff.
The model-led check routes its evidence to controlled process/lock fixtures and
an adversarial design read; this session does not claim a formal proof. A
tree-level `review-design` step will contest the protocol before planning.
No in-session reviewer is spent beside that scheduled read.

Self-review found a multi-viewer probe hazard: an exclusive successful observer
probe could make another observer see contention on a dead witness. Observers
therefore use only shared probes of the per-launch witness and never lock the
driver lease. Shared observer probes are mutually compatible; the witness's
exclusive lock belongs only to the driver. The owning lease releases the
witness before driver ownership. This also avoids adding observer probes to
the existing agent-admission lease protocol.

The human then corrected the emphasis: highlight the currently active item
instead of highlighting all LIVE items. Interpret active as the witnessed
RUNNING mandate, consistently with the agreed cursor/activity distinction.
Ordinary LIVE text is neutral; DONE/ABANDONED retain status colors and their
leading glyphs. RUNNING highlights the item text and activity word while keeping
its lifecycle cue readable, including DONE plus RUNNING.

Finalize the witness probe after pinning the observed tree and its snapshot.
The driver's root pin can disappear on reap/death while an observer still holds
the epoch shared guard; probing first and matching a later tree could therefore
misidentify a reused inode. The typed observation keeps the accepted tree pin
and performs its final runtime check after that identity is established.

## Verification

`bash scripts/check.sh` passed all eight principal checks, including the full
workspace tests and final validation of all six walkthrough books. Hashes of
all 1,750 tracked subjects stayed unchanged during that run. The subsequent
capture/probe-order clarification and prose cleanup changed Markdown only;
local-link/anchor validation (167 links) and all six final book validations
passed again afterwards. These checks preserve the repository baseline and
document integrity; the proposed runtime and display behavior awaits the
design review and implementation tests specified in the handoff.
