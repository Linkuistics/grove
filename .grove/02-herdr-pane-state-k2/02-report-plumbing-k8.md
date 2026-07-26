# report-plumbing-k8

**Kind:** work

## Goal

The driver-side reporter itself: transport, report sites, state mapping, tests.
Everything `01-herdr-authority-route-k7` does *not* decide.

**Do not start this leaf before `01` has settled the route** — the routes
produce materially different reporters (full lifecycle authority implies
release-on-exit and screen-detection suppression; an unforked route implies a
disabled-by-default flag and a latch guard). Rewriting this task file to match
the chosen route is `01`'s last act.

## Context

- `src/loop_driver.rs` — `run_loop` already distinguishes the three terminal
  cases: `Disposition::Relaunch`, `Disposition::Done`, and no-signal
  (crash / Ctrl-C / `/exit`) → `LoopOutcome::Stopped`. Those branches are three
  report sites; `launch_session` is the fourth.
- The node `BRIEF.md` carries the measured herdr behaviour and `state.rs` line
  references. Re-verify before building on any of it — herdr is a repo we do
  not control, and the measurements are against 0.7.5.
- `herdr pane report-agent <PANE_ID> --source … --agent … --state …` — the
  positional pane id comes **first**.

## Done when

- Session launch reports `working`; a `complete --done` finish reports `idle`;
  a no-signal stop reports the state `01` settled for it.
- Reporting is a **no-op** when `HERDR_ENV`/`HERDR_SOCKET_PATH`/`HERDR_PANE_ID`
  are absent or the socket refuses — never a failed launch, never a stalled
  loop (*herdr-optional-ui*). A slow or wedged socket must not block the
  driver; bound the attempt.
- The loop does not leave a **latched** pane behind on exit (node brief, "A
  landed report latches").
- Tests cover the `LoopOutcome`/`Disposition` → reported-state mapping without
  needing a live herdr.

## Notes

- Send **no** `seq`, or always send one — never mix. `accept_hook_report`
  (`state.rs:1159`) accepts a `seq: None` report only while no seq was ever
  recorded for that source, and a per-process counter would restart at 1 after
  a Ctrl-C + `grove do` resume and be rejected as stale. No-seq is the robust
  choice for a restartable driver.
- The no-signal exit still lumps together a crash, a deliberate Ctrl-C, and
  `/exit`. If the driver cannot tell them apart from the child's exit status or
  terminating signal, say so plainly and pick the safer default rather than
  inventing a distinction it cannot observe.

**Scope guard**: intra-session turn boundaries are `04`; the plugin is `05`.
