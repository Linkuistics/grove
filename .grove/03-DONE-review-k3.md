# review-k3

**Kind:** review

## Goal

Fresh-context adversarial review of driver-side-kill-k2 before it is
released: try to disprove that the watcher is correct and that the killer
removal left nothing dangling.

## Context

The change touches the loop's liveness core: a bug here either fails to kill
(the codex symptom again) or kills a session that didn't signal. Review runs
per the trial convention (K3 reviews everywhere when loop-driven).

## Done when

Findings triaged: real defects fixed in this leaf or externalized as new
leaves; the reviewer's pass leaves no CONFIRMED correctness finding open.
Specific angles: signal-file races (stale file at iteration start, file
appearing during teardown), zombie/reap correctness after SIGKILL, terminal
state after a killed TUI, docs/tests still referencing removed env handles.

## Notes

**Outcome.** Two independent fresh-context adversarial reviewers completed;
findings below are all CONFIRMED (traced, and in two cases empirically
reproduced). A third, cross-family reviewer (codex/gpt-5.6-sol via the codex
MCP) **timed out after 30 minutes and returned nothing** — the cross-family
angle went uncovered this pass. Not chased further: this session ran on Claude
Code, where the trial's documented reviewer recipes (codex → headless pi, pi →
`codex exec`) do not apply anyway.

Fixed in this leaf (local, cheap, and about code this change introduced):

- `kill_graces` passed non-finite / unrepresentable operator values straight to
  `Duration::from_secs_f64`, which panics: `GROVE_KILL_GRACE=inf` or `1e300`
  took the driver down. Now sanitised via a pure `sanitise_grace` seam, with
  unit tests.
- `tests/loop_driver.rs` header claimed the suite proves "the PID handle reaches
  the child"; it now asserts the opposite.
- `Cargo.toml` justified the `libc` dep by the deleted self-spawned killer.
- `docs/adr/self-driving-loop.md` claimed `grove-llm pick` is the loop
  condition; `run_loop` never invokes `pick` (the *agent* does, in-session).
  Pre-existing, but the ADR set is current-state-only.

Externalized, all sequenced ahead of release-k4 — none should ship:

- signal-file-identity-k6 — a foreign grove can kill and infinitely re-loop an
  innocent session; escalated from benign to session-killing by this change.
- watcher-test-hardening-k7 — three mutants survive all 20 tests.
- harness-spawn-preflight-k8 — missing rerouted harness now aborts mid-loop.

Angles that came up clean, traced rather than assumed: zombie/reap correctness
and PID reuse (every return path reaps; `kill` only runs on an iteration where
`try_wait` returned `None`, so the pid cannot be recycled); process-group / TTY
/ Ctrl-C semantics (unchanged — the old `sh` used `exec`, so same pid, pgid,
session, and controlling terminal, and `SIG_IGN` is inherited across both fork
and exec either way); SIGTERM reaching the harness's own children (a
*pre-existing* structural limit, not a regression — the old killer targeted the
identical single pid); and timing drift (the grace is now strictly ≥ the old
one, and no message, doc, or test asserts the old guarantee).
