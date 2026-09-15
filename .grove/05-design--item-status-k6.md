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
