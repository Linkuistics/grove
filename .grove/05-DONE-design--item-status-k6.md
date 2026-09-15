# item-status-k6

## Goal

Repair the process-death lifetime gap in the item-status runtime design before
`item-status-k4` plans implementation. Preserve the root brief's truthful
RUNNING and no-rebinding requirements without durable task-tree runtime state.

## Context

`item-status-k5` integrated the eight findings of `item-status-k3`. Its one
narrow reviewer identified an additional boundary: explicit witness-before-pin
release handles ordinary return and unwind, but does not establish kernel
cleanup order on process death. A possible interleaving is root pin release,
root replacement with reused inode/key, observer pins the replacement, then
observer probes the old witness before its lock is released. Pin-before-probe
alone does not exclude this trace. The earlier review's “What held” claim on
this axis is insufficient evidence for this death path.

The current spec marks this unresolved boundary. Establish whether the trace
is possible on supported platforms with primary-source evidence, and either
justify a death-safe relationship or redesign the identity/liveness mechanism.
Do not assume descriptor allocation order, Rust field order or orderly Drop
controls kernel process teardown. Do not weaken the agreed guarantee silently.

The spec owns protocol and result semantics; the driver-lease ADR owns rationale
and trade-offs. Keep the short runtime-only epoch guard and explicit residual
suspension trade-off unless evidence demands changing them. Preserve the other
integrated findings: shared duplicate-key refusal, identity-based workspace
matching, nonblocking witness allocation, help and concrete rendering styles.

## Done when

The spec and ADR establish the death-path guarantee with a testable mechanism
and precise process/lock/filesystem controls, or identify a trade-off that
requires the human. Reconcile their citations and remove the unresolved note
only when resolved. Cut a `review-design` leaf with the same bare stem ahead of
`item-status-k4`; this producer is the start of a new review chain, and planning
must read the reviewed correction before slicing implementation.

## Decisions (running log)

The separate pin and witness do not establish a death-order guarantee. Linux
v6.12 `close_files` walks descriptors upwards and can reschedule between closes;
XNU xnu-11215.1.10 `fdfree` walks them downwards. Their final-close paths protect
the object being closed, not another descriptor's object. The durable ADR will
cite those primary sources; descriptor numbers and Rust drop order are not the
repair.

Add an exclusive, nonblocking observation lock to the existing task-root pin.
Keep the private per-launch file witness for start/reap evidence and summaries
when the old root is inaccessible. Attaching activity to a numerically matching
tree requires probing that captured directory's lock before the final private
witness probe, under the short shared epoch guard. Successful shared probes are
unlocked immediately. A replacement may acquire a directory witness only after
exclusive predecessor-epoch invalidation. The same-object final-close ordering
in Linux `__fput` and XNU `vn_closefile` makes the root lock disappear before its
own pin can disappear; no ordering between separate descriptors is required.

The review is a new tree step as this leaf requires, so no in-session reviewer
is used. Validation will distinguish kernel-source evidence and deterministic
process/lock controls from the future implementation's acceptance tests. No
formal model is needed to assume the very kernel lifetime fact at issue.

The directory lock reserves an additional observation-lock location, under the
existing cooperating-process contract. The ADR explicitly states the foreign
exclusive-holder limitation and the conservative launch-setup failure caused
by a shared probe. A typed tree relation carries verification to the viewer;
numeric equality alone cannot drive either RUNNING attachment or NEXT exclusion.

A temporary Python process probe passed on Darwin 25.6.0 in three cases:
directory closed first, private witness closed first, and SIGKILL with both
held. Each checked positive live contention, independent containing-directory
locking, removal/recreation while the old root was open, release observations,
actual SIGKILL/reap, unchanged started bytes, and compatible independent shared
probes. This tests OS primitives, not the unimplemented observation API. Linux
evidence here is the cited kernel source; forced inode-number reuse and epoch
handoff mutation controls remain explicit implementation acceptance scenarios.

The spec/ADR link and anchor check passed, including positive references and
deliberately invalid anchor/path controls. The checked spec, ADR and glossary
bytes were unchanged across that run. `item-status-k7` now reviews this producer
ahead of planning; the planning body and root brief name that dependency. The
handoff remains prose only, with production behavior and its implementation
tests left to the reviewed planning chain.
