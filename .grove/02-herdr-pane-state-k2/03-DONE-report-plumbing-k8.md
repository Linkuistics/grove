# report-plumbing-k8

**Kind:** work

## Goal

The driver-side reporter itself: transport, report sites, state mapping,
release-on-exit, tests. Everything `herdr-authority-route-k7` decided but did
not build.

**Depends on `herdr-authority-patch-k9`** having shipped a patched herdr. Until
it has, there is no configuration in which these reports are accepted, so
nothing here can be verified end-to-end.

## The route, settled

Full reasoning is in `01-DONE-herdr-authority-route-k7.md`'s `## Decisions
(running log)`. What binds this leaf:

- grove reports as **`grove`** with a source of ours, and carries **no
  `session_ref`** — ever. That is not incidental: the accepted patch keys
  entirely off `session_ref.is_some()`, so a report that grows a session_ref
  would start conflicting with the harness's session owner again and would be
  dropped.
- **Precedence, not full lifecycle authority.** grove is not in herdr's
  authority allowlist and must not be added to it. herdr keeps running screen
  detection underneath and keeps `fallback_state` current; grove's report simply
  wins while grove holds authority.
- **Release on every catchable exit.** grove's authority never expires — there
  is no TTL in herdr, and the clear-on-process-exit path does not fire for a
  label that doesn't parse to a known agent. So whatever grove reported last is
  what the pane shows, indefinitely, until something releases it.

## Context

- `src/loop_driver.rs` — `run_loop` already distinguishes the three terminal
  cases: `Disposition::Relaunch`, `Disposition::Done`, and no-signal
  (crash / Ctrl-C / `/exit`) → `LoopOutcome::Stopped`. Those branches are three
  report sites; `launch_session` is the fourth.
- **The driver installs no signal handler today.** Ctrl-C reaches it as SIGINT
  to the foreground process group and kills it outright, so release-on-exit is
  new work, not a call added to an existing path. Whatever handler this leaf
  adds must not disturb the existing grace → SIGTERM → kill-grace → SIGKILL
  sequence the driver applies to *its child*.
- `herdr pane report-agent <PANE_ID> --source … --agent … --state …` — the
  positional pane id comes **first**; flags before it fail with a bare
  `unknown option: <value>`.
- The node `BRIEF.md` carries the measured herdr behaviour. Its `state.rs` line
  references are stale (upstream took +1281/-812 on that file); the behaviour
  descriptions survived re-verification, the line numbers did not.

## Done when

- Session launch reports `working`; `complete --done` reports `idle`.
- **A HITL stall reads as `blocked`** — the headline complaint this whole grove
  exists to fix. Note that the driver cannot see intra-session turn boundaries;
  what it *can* see is that a session ended without a signal. Whether that is
  sufficient for the "stalled overnight on a question" case, or whether it
  genuinely needs `herdr-turn-hooks-k4`, is worth stating plainly in this
  leaf rather than papering over.
- A no-signal exit lumps together a crash, a deliberate Ctrl-C, and `/exit`. If
  the driver can separate them from the child's exit status or terminating
  signal, do; if it cannot, **say so plainly and pick the safer default** rather
  than inventing a distinction it cannot observe.
- Authority is **released** on clean relaunch-stop, on `complete --done`, and on
  SIGINT/SIGTERM. Uncovered by design: SIGKILL, panic, OOM, power loss — the
  pane pins at grove's last state and the user recovers with `herdr pane
  release-agent`. Document that; do not silently pretend it is covered.
- Reporting is a **no-op** when `HERDR_ENV` / `HERDR_SOCKET_PATH` /
  `HERDR_PANE_ID` are absent, or the socket refuses, or herdr is unpatched and
  drops the report — never a failed launch, never a stalled loop
  (*herdr-optional-ui*). A slow or wedged socket must not block the driver;
  bound the attempt.
- Tests cover the `LoopOutcome` / `Disposition` → reported-state mapping without
  needing a live herdr.

## Notes

- Send **no** `seq`, or always send one — never mix. `accept_hook_report`
  accepts a `seq: None` report only while no seq was ever recorded for that
  source, and a per-process counter would restart at 1 after a Ctrl-C +
  `grove do` resume and be rejected as stale. No-seq is the robust choice for a
  restartable driver.
- The latching hazard the node brief measured is **dissolved by the patch**, not
  by anything this leaf does: with the owner gate no longer vetoing, grove's
  later reports land, so grove can always correct itself. Do not build a latch
  guard; verify the absence instead.
- The `source` string is ours to pick — herdr accepts any (its own tests use
  `custom:hermes`). Pick one and keep it stable: release matches on
  `(source, agent)`, so a source that varies between report and release would
  leave authority behind.

**Scope guard**: intra-session turn boundaries are `04`; the plugin is `05`;
patching herdr is `herdr-authority-patch-k9`.
