# witnessed-activity-k12 — brief


## Goal

Deliver witnessed RUNNING and exclusion-aware NEXT through the real driver and
viewer. This working increment follows idle-next-k11, which supplies the
shared typed observation, selector and activity chrome. Publication and its
consumer belong to this one increment, which is larger than one session.



## Context

The inherited requirements and `docs/specs/item-status.md` are the contract.
The driver-lease ADR owns the native filesystem assumption, process-death
argument and accepted suspension trade-off. The integrated correction through
item-status-k8 requires the typed verified tree relation, not a viewer numeric
identity comparison. Preserve that design while finding session-sized leaves.

## Done when

- A real successful launch reaches the production typed observer and public
  Viewer seam as RUNNING. The common selector excludes its key only with a
  verified same-tree relation, before finish eligibility. Visible and summary
  activity describe the accepted current observation.
- Generic parent-side Started/Reaped events cover successful and immediate
  exits, failed spawn, grace/escalation and confirmed reap on error paths.
  Existing callers can run without an observer. The lease owns both witnesses
  through unconfirmed-reap helper errors; callbacks wait on no epoch/tree lock
  and observation failures change neither launch authority nor outcome.
- Preparation pins/checks the selected task root under its tree guard,
  transfers the pin and private witness to one lease-owned value, and rechecks
  before publication. No tree/epoch guard spans spawn. Each exclusive witness
  is acquired nonblocking only after exclusive predecessor-epoch invalidation.
  Mandatory admission remains valid when observation-only setup fails.
- Versioned observational fields bind key/handle/kind, launch tree identity,
  witness basename/identity, nonce and signal path. Fresh witness names use
  independent OS-random 128-bit suffixes and exclusive creation with bounded
  retries. Started publishes only the exact eight-byte marker; Reaped releases
  before terminal recovery, epoch handoff or signal interpretation. Completion
  signal and retirement alone do not clear RUNNING. Orderly release closes the
  private witness before the directory pin; descriptors are close-on-exec.
  Cleanup follows epoch invalidation and is never a viewer operation.
- `try_observe` implements the spec's bounded directory-before-private probe
  and exact result precedence, returning captures with no advisory guards. The
  accepted tree pin is the descriptor probed. Multiple viewers use compatible
  shared probes and never probe the driver's lease lock. Runtime inconsistency
  clears current attachments independently of tree capture.
- Real launch/application controls cover LIVE, DONE and ABANDONED plus RUNNING;
  renumbering, renaming, moving, decomposition to a branch, item absence, root
  removal, root replacement with reused keys, and a first viewer arriving after
  replacement. Previous-tree/absent-tree/item-absent summaries preserve identity;
  only same-tree evidence licenses exclusion. Children of a running branch can
  be NEXT, and finish is NEXT when exclusion leaves only finish.
- RUNNING is bold yellow on item text and its activity word while lifecycle
  marker/word retain their colors. NEXT is bold normal activity text only;
  cursor adds its gutter alone. Summary labels and absence/freshness qualifiers
  survive long handles, folded/offscreen rows and File view at 60 × 10. Prior
  saved-view, Unicode, resize, aggregate and color-disabled cases still pass.
- Real macOS and Linux subprocess controls acknowledge readiness, prove both
  exclusive witnesses with independent contended shared probes, kill/reap the
  holder, and prove both shared probes succeed despite leftover started bytes.
  An exec'd surviving child cannot retain the driver's witnesses. Positive
  running controls prevent blanket unavailability from passing stale rejection.
  Record platform results precisely; a macOS run is not Linux evidence.
  The planner resolves the execution route before implementation as specified
  below; missing Linux results keep the evidence-owning leaf live.
- Barrier controls independently force directory-first release with reused
  numeric identity/key, private-first release, both released, and release after
  directory verification but before the final private probe. A replacement
  awaiting epoch invalidation cannot acquire either witness. Disabling the
  directory check and disabling epoch-before-preparation must each expose the
  corresponding false attachment while the valid running control still passes.
  These model kernel interleavings; they do not claim to pause kernel teardown.
- Multi-viewer/replacement controls cover unlocked started bytes while old
  lease bytes remain, an old epoch reader delaying handoff, and continuous
  viewers across repeated launches without viewer-caused preparation failure.
  Foreign shared holders of either witness and reported locking errors cause
  prompt observation-only failure. The containing-directory mutation lock stays
  usable while the directory witness is held. Native locking is a precondition;
  do not claim detection of silently ineffective backend locks.
- Fault and liveness controls cover allocation/publication failure,
  malformed/truncated/oversized/mismatched records, old observation versions,
  FIFO/directory replacement and open/lock races. Pause separately outside and
  inside the runtime guard to show only the latter can delay epoch handoff,
  retaining idle-next-k11's control and user-facing liveness guidance.
  Confirm supervision error, unwind, normal drop and failed spawn release rules.
  Use readiness/reap/lock events and failure timeouts, never elapsed sleeps as
  proof. The real replacement test proves binding, not inode non-reuse; retain
  the ADR's open-object evidence and the forced-reuse controls.
- Read-only/no-configuration filesystem snapshots include absent tree,
  non-jj locations, missing namespace, aliases and multiple viewers. Existing
  admission/process controls still reject ambient stale sessions, including
  rotation, root replacement and previously admitted operations during handoff.
- Usage, architecture, module and context-map summaries describe shipped
  behavior and ownership. Reconcile spec/glossary/ADR citations and affected
  source-derived walkthroughs using their existing validation gates. Reconcile
  G6 in docs/specs/user-guide-coverage.md for the shipped witnessed activity,
  removing its implementation deferral
  while preserving unrelated signal/panic-restoration evidence obligations.
  Principal checks pass, and adversarial review of the implemented protocol
  has been resolved before closing this node.

## Decomposition

witnessed-activity-k13 plans this increment against the landed predecessor.
It cuts concrete implementation leaves at independently verifiable seams and
keeps sessions bounded. Publishing metadata with no useful reader is not a
completed product increment. Individual leaves may be independently testable
protocol steps whose product behavior arrives with the last step; each must
pass its own tests and documentation gates, and partial metadata must continue
to yield Unavailable. The node retains the end-to-end product exit condition.
Review leaves are commissioned after artifacts exist; integrations are created
only for actionable findings, where the
directory-local walk reaches them next. The final implementation producer must
commission review-impl for the complete protocol and its controls before this
node can close.

Before cutting implementation, k13 checks for an available authorized Linux
host or VM and records the execution pointer in this brief (personal environment
details belong here, not in product documentation). If no route is available,
explicitly arrange a human Linux run and name that dependency in the evidence
owner's task. That owner prepares a runnable command against an exact revision,
the fixture prerequisites and recorded macOS result before requesting the run;
ask for the Linux command's result and platform details, not a generic approval.
Keep the owner live until both platform results are obtained. No cross-build,
macOS result or prose validation substitutes for native Linux execution, and
the node must not reach closure with this obligation silently deferred.

## Pointers

- Spec: `docs/specs/item-status.md`, especially One bounded observation,
  Process death and tree identity, and Acceptance scenarios and test seams.
- ADR: `docs/adr/one-live-driver-per-working-tree.md`.
- Shared seams: Viewer application; typed loop observation and snapshot
  selection; DriverLease; Workspace namespace discovery; generic launch events.
- Existing process/lock barriers in grove-loop; controlled launch and terminal
  fixtures in keyed-launch; TestBackend application fixtures.

## Notes

The protocol crosses production crates but has one observation implementation.
Tests may replace internal lock/filesystem operations and observe runner events
through the real seam; they may not create a viewer-only status provider. The
root requirements remain the close condition after this node; name and cut any
missing work before finishing.
