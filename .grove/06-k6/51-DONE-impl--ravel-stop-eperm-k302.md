# ravel-stop-eperm-k302

## Goal

Find out why the accepted Ravel 2.0 build ended one staging session with
`supervision_failed`. Repair Ravel before its release cut if the cause is in
Ravel. A real Grove session that ends this way stops the Grove loop.

## Context

- core-candidate-restaging-k300 ran the staging case `stale-copy` on Codex
  0.167.0 with `gpt-6.1-sol` at low effort. The record is
  `~/.local/state/owner-staging/trials/stale-copy-codex-2`. The agent ran
  `ravel end-session staging-ok` as its last event.
- Ravel then exited 5. `ravel.stderr` in that record holds this message:

  ```text
  required job stop failed; stop attempt 1 target -32130 signal 15:
  Operation not permitted (os error 1); stop attempt 3 target -32130
  signal 9: Operation not permitted (os error 1); and reaped
  ```
- The build is the accepted 2.0 build at
  `~/.local/state/owner-staging/ravel-2.0`. Its build ID is
  `58d479ed058f…`, from `Ravel.skill-directory-delivery` at `ff14cba826b0`.
- No other staging record holds `supervision_failed`. The same case passed
  with Ravel exit 0 in `stale-copy-codex-1`. The session started Codex's MCP
  servers, which include `codebase-memory-mcp` and `npm exec
  @upstash/context7-mcp`.
- One hypothesis is that macOS `killpg` returns `EPERM` for a group whose
  members have all exited but are not yet reaped. Ravel would then treat a
  stopped group as a failed stop. No measurement supports this hypothesis yet.

## Done when

- A measurement or a reproduction names the cause.
- A Ravel cause has a repair and a test on the 2.0 line before the release
  cut. Otherwise, the record states why Ravel's report is accurate and what
  the owner must do.
- The evidence of the 2.0 line records the result.

## Decisions (running log)

- Cause, measured 2026-11-11 on macOS 27.0.1 (26A434). A C probe made a
  group leader exit after it touched 64 MB or 2 GB. During the exit
  teardown, `kill(-pgid, SIGTERM)` failed with `EPERM` while
  `waitid(WNOWAIT|WNOHANG)` reported no exit. `kill(pid, SIGTERM)`
  succeeded. The window opened within 0.1 ms of the exit and lasted 1 to
  26 ms in six runs. A zombie-only group also gave `EPERM` to signal 0,
  TERM and KILL, and `ESRCH` after the reap.
- The failure signature matches. Attempts 1 and 3 were group TERM and
  KILL with `EPERM`. Attempts 2 and 4, against PID 32130 (Codex, which
  `trial-launch.sh` execs), succeeded, and the reap followed.
- The cause is in Ravel. `watch()` in `crates/keyed-launch/src/run.rs`
  ends supervision at the first stop report. The approved exception for
  a zombie-only Darwin group (`supervise_reported`, book chapter 9) runs
  only on the path after the observed exit. The escalation TERM after
  the 2-second grace met Codex's own exit teardown.
- Repair, on the Ravel 2.0 line: `watch()` keeps a group-target `EPERM`
  stop report and keeps watching. The report goes with whatever ends the
  watch. After the observed exit, the approved rule retains it only
  beside the reap, terminal recovery and `ESRCH` absence. A PID-target
  refusal or another errno still ends supervision at once. The fake
  `Process` seam in `wait_events.rs` gained stop reports. Two new tests
  failed before the repair with the staging trace, and two negative
  controls pass on both sides.
- Reproduction through the Ravel CLI, 2026-11-11: a managed program sent
  its result, touched 2 GB and exited 2100 to 3500 ms after start. The
  accepted build (`3a9a9129…`) failed in each of its 5 trials whose group
  TERM met the exit teardown, with the staging message. The repaired
  build (`503759e5…`) passed each of its 7 such trials and kept the TERM
  report as a `ravel:` line. The 2100 and 3500 ms controls passed on
  both builds. Digests of both builds, the program, the script and the
  sweep did not change across the runs.
- Review: this leaf spent its one in-session reviewer on the watch change.
  It found no path to success without the three confirmations. Findings 1
  and 2 were valid test gaps, and two new tests close them. Five mutations
  of `run.rs` each fail at least one test. Finding 3 was an unclear
  contract, and the specification now says "before the observed exit".
  Finding 4 is an accepted limit that Ravel's evidence records. Finding 5
  is covered by the deadline case of the new test.
- Ravel 1.1.0 has the same check in `watch`, so the current driver keeps
  the defect until promotion. The root brief keeps the 1.1.0 keg, so 1.1
  gets no patch. Grove's evidence states the owner's remedy: rerun Grove.
- Result: Ravel commit `576811bcb4b0` on the unpushed
  `skill-directory-delivery`. Ravel's `task check` passed 713 of 713 run
  tests. The section "Stop record of an exiting group" of Ravel's design
  evidence and the section "The stop of Ravel 2.0" of Grove's owner
  evidence record the result.
