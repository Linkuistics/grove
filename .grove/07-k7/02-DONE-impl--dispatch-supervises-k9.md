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

## Decisions (running log)

**S1 — the runner's three options.** `Launch` gains `grant` (set after the
scrub and before the channel, so no grant replaces the channel's variable) and
`transparent: Option<&EntrySignals>`. `Ended` gains `signalled`, the channel's
appearance after the reap whatever it holds (`Channel::appeared`, an
`fstatat` beside the held directory that follows nothing); the watch loop uses
the same test, so an empty file or a link now starts the escalation too. The
token stays beside it until `launch-cutover`. `End::Signalled` is renamed
`End::Escalated`, as decision 7 names it, so that it no longer reads as the
new field.

**S2 — what "transparent" carries.** `EntrySignals` is a caller-recorded
ignored set and mask (bits for 1–64); the runner only hands it on, since only
the caller can record it before the Rust runtime changes SIGPIPE. In the
child every changeable signal goes to its entry ignore or default, then the
mask, allocation-free. A transparent launcher is also cancelled by SIGINT
when it has a terminal: a typed Ctrl-C goes to the child's group, so an INT
that reaches dispatch was sent to dispatch, and the spec's *handled signals*
are INT, TERM and HUP. Folding it into the same option keeps it one option.

**S3 — two small additions the spec's surface implies.** `Argv::with_arg0`,
because dispatch spawns the resolved path and the harness's `argv[0]` is the
program as `select` returned it. `LaunchError::raw_os_error`, set only for a
failed spawn, because a launch failure exits 127 for `ENOENT` and 126
otherwise.

**S4 — where the exit channel is made.** `--exit-dir` is checked (canonical,
an existing directory) before selection and refuses as `exit_dir_unusable`,
exit 2. The channel itself, and the private directory, are allocated after
selection and before the commit: the worker has been reaped by then, so it
can never have been told the path, and a failure records nothing. Both are
removed before dispatch ends, which matters because a death by signal runs
no destructor. The private directory's mode is set to 0700 explicitly:
`tempfile` was measured creating it 0755 on macOS, which the new seam case
caught.

**S5 — the linearization point, now with a spawn.** The selection's handlers
stay installed until the runner's own replace them inside `run_observed`;
dispatch restores the caller's mask in the `Started` callback, so a handled
signal that arrived after the final check is delivered to the runner's
handler and cancels the run. The selection's handlers are dropped after the
run, which reinstates the entry dispositions a death by signal reproduces
under. No `Started` and an error is a launch failure (126/127 from the spawn's
errno, appended as before); an error after `Started` is a supervision
failure, exit 5.

**S6 — the end notice is provisional on its fields.** One stderr line,
`{"schemaVersion":1,"end":{runId, ending, exitCode, signal, durationMs,
group}}` under `--json`. `run-ending` owns the end observation and the ending
file and may align this with the observation's shape; nothing parses it yet
except the seam suite.

**S7 — the seam's terminal cases use a `sh` session leader on a PTY.** A
plain leader keeps the front in its own group, so what it measures afterwards
(`stty -g`, `ps -o tpgid=`) is what the front restored; `set -m` makes the
front a job of its own for the Ctrl-C-during-selection case, so the leader
survives to report. Modes are compared less PENDIN, which BSD sets after a
canonical restore with input queued.
