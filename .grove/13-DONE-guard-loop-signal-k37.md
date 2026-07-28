# guard-loop-signal-k37

**Kind:** impl

## Goal

`cargo test` in this repo kills the very `grove do` session it was typed into.
Make the authority to end a session a **capability the driver hands to exactly
one process**, not an ambient path any descendant inherits — so only a
deliberate `grove-llm complete` from the session itself can stop the loop.

## The report

Observed by the human 2026-07-28, in this grove, repeatedly: running `cargo
test` ends the live session. The cost is not the test run — it is that a killed
session loses its in-flight context, and a relaunch is "expensive and lossy".

The human's own narrowing, and it is load-bearing for scope: **this is a
meta-grove problem** — a grove whose subject *is* the grove machinery. An
ordinary grove's test suite has no reason to touch the loop's control channel.

## Context

The kill channel is `GROVE_SIGNAL_FILE`. The driver derives it per worktree
(`loop_driver::signal_file_path`, a `$TMPDIR` path keyed on a worktree-identity
hash), exports it into the session's environment, and watches it for the whole
life of the child; the instant the file appears it applies grace → SIGTERM →
kill-grace → SIGKILL (*self-driving-loop*, `src/loop_driver.rs`).

Two facts make that a live hazard under `cargo test`, both confirmed by reading:

1. **The variable is inherited by every descendant.** It is set in this
   session's env; `cargo test`, the test binaries, and every subprocess they
   spawn all carry it.
2. **The file's mere existence is sufficient.** `complete::read_signal` reads
   content only to tell `Relaunch` from `Done` — nothing establishes *who* wrote
   it, or that it belongs to the session currently running. `resolve_opts(None,
   …)` falls back to `$GROVE_SIGNAL_FILE`, so any code path under test that
   reaches `signal_complete` with no explicit path writes the **real** session's
   signal.

So the authority to end a session is ambient. That is the defect, independent of
which test fires it.

There is **precedent for exactly this hazard class, already defended against one
notch down**: `tests/support/mod.rs:77-84` scrubs `HERDR_ENV`,
`HERDR_SOCKET_PATH` and `HERDR_PANE_ID` for a blunt reason stated in its own
comment — these tests usually run *inside* a herdr pane, so without the scrub
"`cargo test` would visibly hijack the sidebar row of the terminal it was typed
into". `GROVE_SIGNAL_FILE` is the same shape of leak with a worse outcome: not a
mislabelled pane, a dead one. It is **absent** from `grove_env_names()`.

Note what does *not* leak, so the diagnosis is not over-drawn: a loop test
driving a `TempDir` worktree derives a *different* signal path (different
identity hash), so the driver-under-test and its fake harness scripts are
already isolated. The leak is specifically via the **env fallback** — a
subprocess spawned without `env_remove`, or a lib call passing `None`.

## Done when

- The **specific** `cargo test` path that fires the live signal is identified by
  measurement, not inference — the reasoning above establishes the *capability*
  is ambient, not which test spends it. Run the suite from a shell with
  `GROVE_SIGNAL_FILE` pointed at a scratch path and see which test writes it;
  never run it with the live value inherited.
- Ending a session requires something a descendant does not automatically hold.
  The shape to cost first: the driver mints a per-session **nonce**, exports it
  beside the path, and `complete` writes it into the signal; the driver ignores
  a signal whose nonce is not the running session's. That makes the file's
  existence insufficient, and it buys two things beyond this bug — a **stale**
  signal from a previous session and a signal from a *different* grove both stop
  being lethal. Cost the cheaper alternatives honestly against it (scrub-only in
  the test harness; a test-runner sniff in `complete`), and say why the loser
  lost.
- Whatever the mechanism, the **test harness is fixed too** — belt and braces,
  because the two failures are independent: `grove_env_names()` grows the loop
  control vars, and subprocess spawns `env_remove` them. A test that wants the
  signal live sets it back, pointing at a path it owns (the pattern
  `tests/report_turn.rs:35-44` already uses).
- A test that would have killed the session **fails loudly instead of silently
  succeeding**. The current failure mode is invisible from inside the suite: the
  suite passes, the terminal dies.
- ADR *self-driving-loop* is reworked **in place** where it describes the signal
  as a bare file, and the guarantee is stated as the contract it now is. Never
  append a superseding record (`linkuistics:decision-records`).

## Notes

**Do not run the full `cargo test` from a live session to reproduce.** That is
the bug. Reproduce with the variable redirected, or from a shell outside the
loop.

The scope is this repo's own dogfooding, per the human's narrowing — but the
nonce fix is product-side and holds for any grove, which is an argument for it
over a tests-only patch, not against.

`tests/support/mod.rs` `grove_env_names()` is the single list both `EnvGuard`
(this process's env) and subprocess `env_remove` call sites read, so growing it
fixes both at once — that sharing is deliberate and documented at its
definition.
