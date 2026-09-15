# launch-events-k24


## Goal

Expose optional synchronous parent-side Started/Reaped notifications from the
generic runner, at the actual spawn/reap boundaries. Existing callers keep
their current behavior without an observer.



## Context

`keyed_launch::run`, `supervise` and `watch` own spawn, all wait paths, token
reading and terminal recovery. The current wait-error branch attempts kill and
wait; it knows whether reap succeeded even when it returns LaunchError.
`launch_configured_session` is the next leaf's production consumer.

Starting source: `crates/keyed-launch/src/run.rs`, its public exports, and
`crates/keyed-launch/tests/launch.rs`. Follow the graph's actual callers before
choosing an API; preserve domain-free vocabulary and ordinary Launch callers.
The keyed-launch walkthrough reproduces this source and must move with it.

## Done when

- A caller can observe Started exactly once after successful spawn, including
  immediate child exit; failed spawn emits neither Started nor Reaped.
- Every confirmed reap emits Reaped exactly once, including try_wait success,
  SIGTERM/SIGKILL escalation, interrupted launch and recovery from wait errors.
  A failed recovery wait emits no Reaped. Delivery precedes terminal recovery
  and post-reap token reading; a signal's appearance alone emits no Reaped.
- Notifications are infallible, synchronous and parent-side, introduce no
  child acknowledgement, and do not alter End, token, status or LaunchError.
  No-observer callers retain their supported entry point and semantics.
- Actual child fixtures cover normal/immediate exit, nonexistent program,
  signal-before-reap and escalation. A narrow internal wait/event seam covers
  confirmed versus unconfirmed reap on error paths and event ordering before
  terminal recovery/token read. Readiness and actual reap establish transitions;
  a failure timeout bounds each wait. Reuse existing process-group/PTY controls.
- Update the runner's API/module documentation, affected architecture/module
  summaries and source-derived keyed-launch book fragments/indexes/manifest.
  G6 still records witnessed viewing as pending; do not claim it ships here.
  Focused keyed-launch tests and `bash scripts/check.sh` pass after all edits.

## Notes

This is the generic event seam only. Grove witness ownership and publication
belong to launch-witnesses-k25. Do not put Grove types or filesystem locks in
the runner. The callbacks' nonblocking discipline is that consumer's obligation.
The final whole-protocol review is commissioned by witnessed-view-k27; if an
unexpected event-order doubt needs review earlier, name that doubt explicitly.

## Decisions (running log)

- Keep `run(Launch)` source-compatible and add `run_observed(Launch, callback)`
  with `LaunchEvent::{Started, Reaped}`. A borrowed synchronous `FnMut` returns
  unit; consumers own nonblocking, non-panicking notification handling.
- Use a private process/wait seam for deterministic confirmed/unconfirmed error
  paths and a recovery closure to check event ordering against terminal recovery.
  Production still uses the real Child and existing process-group signaling.
- Implementation sequence: public real-child event tests; runner seam and error
  ordering tests; shipped API/architecture and exact-source book repair; focused
  tests and the principal gate; retire and describe the leaf.
- The private wait trace rejects a deliberate suppression of Reaped (expected
  assertion failure), then passes with production restored. Real-child tests
  cover immediate/ordinary exits, missing program, token-before-exit, SIGTERM,
  SIGKILL and interruption. The existing no-observer tests remain controls.
- The keyed-launch book now reconstructs the changed API and private process
  seam; its final validation accepts 9 roots and 2,145 lines with no deferrals.
  No framework-version decision or new dependency was needed: this is event
  plumbing around the existing Child wait and process-group operations.
- Final host verification: `cargo test --locked -p keyed-launch` passed;
  `bash scripts/check.sh` passed all eight principal checks, including workspace
  process/PTY regressions and all six books. SHA-256 comparison of 1,749 files
  under crates/docs/scripts/plugins/.cargo and root check inputs found no changes
  during the gate. These are native macOS results; Linux witness evidence remains
  owned by witnessed-observation-k26. G6 still records witnessed viewing pending.
