# dispatch-supervises-k9

## Goal

`harness-dispatch run` no longer replaces itself with the harness. It spawns the
selected command as a job of its own through the runner and stays its parent
until it has reaped it:

- **Signal state.** The harness receives the entry signal mask and every
  disposition that survives exec, as dispatch inherited them, SIGPIPE's
  included, and none of dispatch's own handlers. A signal the caller blocked
  and sent to dispatch stays pending in dispatch (I5).
- **The exit channel.** Every run gets a fresh one, allocated in `--exit-dir`
  when given (which must exist; dispatch neither creates nor removes it), and
  otherwise in a private owner-only per-run directory under TMPDIR, which
  dispatch removes. It is published to the harness as
  `HARNESS_DISPATCH_EXIT_FILE`, beside `HARNESS_DISPATCH_RUN_ID` and
  `HARNESS_DISPATCH_STATE_DIR`, each replacing an inherited value, and never to
  the worker.
- **`harness-dispatch exit`** sends the exit signal: it creates the file and
  carries nothing. It reads no owner setting, policy or record. Outside a
  supervised run it signals nothing, says so and exits 0.
- **The escalation's constants**: grace 2 s, kill-grace 5 s.
- **Cancellation past the linearization point.** The handlers stay installed
  across the spawn, so a signal after the linearization point cancels the run:
  an interactive harness's group is sent that signal and then SIGKILL after the
  kill-grace.
- **The run ending and exit status.** The ending is chosen once the harness is
  reaped: `cancelled`, `exit_signal` or `harness_exit`. Dispatch's exit status
  follows W6. A death by signal is reproduced without a core dump of dispatch's
  own. A one-line end notice goes to stderr, as one JSON line under `--json`.
  A group still present after the run is a supervision failure, which exits 5.

The runner gains the three options this caller is the first to need: the
transparent caller's entry signal state, a channel whose appearance is the whole
signal, and granted variables. Grove's driver raises its kill-grace to 10 s in
this leaf, so its own escalation of the now-supervising dispatch cannot orphan a
harness.

## Context

- The spec's *Execution and authority* (the linearization point, the
  transparent start), *Supervision* (the exit signal, the escalation,
  cancellation, the run ending), *Diagnostics and exits*, and `exit` under
  *Command interface*. Decision 7 (a transparent caller passes its own entry
  state). `harness-wrapper-k2`'s W3, W4, W6, W7 and W9.
- Today's handoff is `crates/harness-dispatch/src/run.rs`, with
  `cancellation.rs` holding the linearization point and `signal_state.rs`
  recording the entry state in the initializer section. The `exec` path and its
  caveat about the window "no userspace exec closes" both go. W9 settled that
  no `exec` verb is kept.
- Seam 1's PTY suite is `crates/harness-dispatch/tests/run.rs`. The driver's
  graces are in `crates/grove-loop/src/loop_driver.rs`.

## Done when

- Seam 1's supervision cases pass through dispatch's command under a PTY. The
  first row of the spec's seams table lists them:
  - The harness runs in its own group holding the terminal, and the terminal
    comes back to dispatch with its modes restored.
  - The entry mask and dispositions reach the harness, and dispatch's handlers
    do not. That covers SIGPIPE's disposition and a caller-ignored HUP.
  - A blocked signal sent before the spawn stays pending in dispatch.
  - A typed Ctrl-C reaches the harness alone, and during selection dispatch and
    its worker alone.
  - The exit signal drives grace → TERM → kill-grace → KILL on the group, and
    a fake that exits within the grace is not signalled.
  - The fake's own exit code or signal is dispatch's when it exits on its own.
  - Every run gets a fresh exit channel, so a channel left by an earlier run
    ends nothing. The channel is allocated in `--exit-dir` when given.
  - `exit` with no channel is a no-op that exits 0, and a nested run's harness
    ends only the nested run.
  - A signal after the linearization point ends the run as `cancelled`, with
    the group reaped.
  - A missing exit directory refuses before selection.
- Grove's kill-grace is 10 s, and its launch-boundary suite passes with
  dispatch supervising beneath Grove's channel.
- The `keyed-launch` and `grove-loop` books follow (P2), and
  `## Unreleased` records that `run` supervises and that `exit` exists.
- `bash scripts/check.sh` passes.

## Notes

- The ending file and the end observation are `run-ending`'s. Here the ending
  is chosen and reported on stderr and in the exit status only.
- Seam 1 absorbs the runner's interface cases. `keyed-launch`'s suite supports
  the seam and does not replace it.
- `HARNESS_DISPATCH_*` names are already excluded from worker grants. Check
  that the new variable is excluded too, and that the worker never sees the
  channel. The channel carries the authority to end the run.
