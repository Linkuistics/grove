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
The resulting dependency order is below. Keep the independently testable
protocol steps in this one product increment: each has a production consumer,
its own tests and current documentation. Publishing metadata alone is not a
completed product increment; active evidence the current consumer cannot verify
continues to yield Unavailable. The node retains its end-to-end exit condition.

| Leaf | Independently verifiable handoff |
|---|---|
| launch-events-k24 | Generic callers observe real spawn and confirmed reap, with no-observer compatibility. |
| launch-witnesses-k25 | Real launches hold both witnesses through the correct lifetime and publish bound metadata; admission survives observation failure. |
| witnessed-observation-k26 | Production observation verifies Running and tree relation; native macOS/Linux and forced-reuse controls challenge the inference. Viewer activity stays conservative until binding lands. |
| witnessed-view-k27 | Public viewing attaches RUNNING and forecasts NEXT from accepted evidence, completing presentation and G6. |

### Acceptance ownership

These rows assign the remaining contract, not a substitute for the spec's
scenario table. Each owner keeps its behavior green and documents what ships.

| Contract/scenarios | Owner |
|---|---|
| Started/Reaped; immediate/failed spawn; grace/escalation; confirmed/unconfirmed wait errors; notification ordering; no-observer API | launch-events-k24; lease consequences in launch-witnesses-k25 |
| Selected-root capture/recheck; finish preparation; lease ownership; epoch-before-either-witness; no guard across spawn; orderly/unwind/helper lifetime | launch-witnesses-k25 |
| OS-random exclusive allocation; extension/admission separation; marker failure; private-before-directory release; cleanup after invalidation | launch-witnesses-k25 |
| Foreign shared holders/lock errors; containing-directory mutation remains usable; stale admission and handoff controls | launch-witnesses-k25; independent shared-holder and repeated-viewer controls in witnessed-observation-k26 |
| Bounded read-only runtime; directory-before-private precedence; typed relation; old/malformed/mismatched metadata; FIFO/open/lock/path races | witnessed-observation-k26 |
| macOS and Linux positive/kill/reap/leftover/exec-survivor controls; real replacement and first arriving observer | witnessed-observation-k26 |
| Both teardown orders/both released/post-directory release; forced numeric/key reuse; directory-check and epoch-preparation mutations with positive controls | witnessed-observation-k26 |
| Multiple shared viewers, old-epoch-delayed replacement, repeated launches; capture/runtime pauses, handoff bound/restart; no escaped guards | witnessed-observation-k26 |
| Row binding/current species, moves/renames/renumbering, lifecycle plus RUNNING, branch children, item/tree absence, replacement/reused keys | witnessed-view-k27; typed relation in witnessed-observation-k26 |
| Exclusion before finish eligibility and after validation; no viewing allocation; independent two-capture acceptance and stale-row clearing | witnessed-view-k27 |
| Highlight/lifecycle/cursor separation; 60 × 10 summaries/qualifiers; folded/offscreen/File; Unicode, resize, saved state and color-disabled regressions | witnessed-view-k27 |
| Read-only absent/non-jj/aliases/missing namespace/multiple observers | witnessed-observation-k26 and public Viewer controls in witnessed-view-k27 |
| Usage/architecture/module/context-map and source-derived books; G6 current-state reconciliation | Each producer for its shipped seam; witnessed-view-k27 removes witnessed-view deferral |
| Adversarial review of complete implementation and platform/mutation evidence, with actionable findings resolved before close | witnessed-view-k27 commissions it after the artifact exists |

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

### Resolved Linux execution route

Planning verified the running local Docker Desktop `desktop-linux` context.
A disposable container executed natively on Linux 6.10.14-linuxkit aarch64
with overlayfs and Rust/Cargo 1.85.1. The cached image is
`rust@sha256:e51d0265072d2d9d5d320f6a44dde6b9ef13653b035098febd68cce8fa7c0bc4`.
This uses the existing authorized local Linux VM and requires no remote host,
new CI service or human-run dependency. Kernel-backed fixture paths must live
inside its Linux filesystem, with source copied in; a macOS bind mount is not
the locking subject. Native macOS remains a separate run on the host.

witnessed-observation-k26 owns the complete platform result. Put the native
process controls in the grove-loop library's private test seam, using real
self-spawned holders and configured child launches with temporary `.jj` markers;
these controls need neither a real jj command nor personal configuration.
Name the required cases with `witness_` in their test names so the same bounded
suite can be listed and executed on both hosts. Other process integration
regressions still run through the repository's ordinary principal gate.

Prerequisites: the running local Docker engine and cached digest above, a source
archive containing exact tracked files (including `.cargo/config.toml` and
Cargo.lock), locked dependencies downloadable or cached, `/bin/sh` and the
image's native C compiler. Record the source/image fingerprint and explicit
required test-name list before execution; zero selected tests cannot pass.
Planning also verified Rust/Cargo and writable overlayfs as uid/gid 1000:1000.
Run under that identity so access-failure controls do not become root-only
skips. Use writable `/tmp` for source, build output, Cargo cache and lock
fixtures; mount no host administration directory and pass no loop-control
environment.

For a prepared `source.tar`, the Linux execution shape is:

```sh
docker --context desktop-linux run --rm -i --user 1000:1000 --env CARGO_HOME=/tmp/cargo \
  --env TMPDIR=/tmp --workdir /tmp \
  rust@sha256:e51d0265072d2d9d5d320f6a44dde6b9ef13653b035098febd68cce8fa7c0bc4 \
  sh -ec 'mkdir work; tar -xf - -C work; cd work;
    uname -srm; rustc --version; cargo --version; stat -f -c %T .;
    cargo test --locked -p grove-loop --lib witness_ -- --list;
    cargo test --locked -p grove-loop --lib witness_ -- --nocapture --test-threads=1' \
  < source.tar
```

This is the route, not a claim that future tests exist or pass. k26 must freeze
the final source, identify its jj revision plus any tested diff and archive
digest, enumerate required cases, compare each platform's actual results, and
preserve the macOS result beside the Linux one. If toolchain/dependencies change,
verify the replacement image rather than using a moving tag. If this route
becomes unavailable, use the explicit human-run fallback above; that does not
relax this node or its owner's exit.

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
