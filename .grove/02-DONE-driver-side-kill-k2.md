# driver-side-kill-k2

**Kind:** work

## Goal

Move the session-end kill from the sandboxed in-agent killer to the loop
driver: the driver watches for the signal file while the harness child runs
and applies grace → SIGTERM → kill-grace → SIGKILL itself.

## Context

Root cause and design are in the root `BRIEF.md` (grilled 2026-07-18). The
in-agent killer is dead under codex (Seatbelt: `(allow signal (target
same-sandbox))` denies signalling the TUI process; `2>/dev/null` hides it).
The driver is the harness's parent, outside any sandbox — it can always
signal its own child.

## Done when

- `run_loop`/`launch_session` spawn the harness (no `sh -c export…exec`
  wrapper), poll for `GROVE_SIGNAL_FILE` (~500ms) alongside `try_wait`, and
  on signal: sleep `GROVE_KILL_GRACE` (default 2s) → SIGTERM → sleep
  `GROVE_KILL_GRACE_KILL` (default 5s) → SIGKILL → reap.
- `complete.rs` only writes the signal file: `spawn_delayed_killer`, the PID
  resolution (`GROVE_HARNESS_PID`/`GROVE_CLAUDE_PID`), and the
  `--pid`/`--grace`/`--kill-grace` flags are gone; "under the loop?"
  messaging keys off `GROVE_SIGNAL_FILE` alone.
- Tests through the agreed seam (fake harness via `GROVE_HARNESS_BIN`): a
  fake that writes the signal file then sleeps long is killed promptly, with
  the right loop outcome for both relaunch and `--done`; existing
  PID-asserting tests updated.
- `docs/adr/self-driving-loop.md` reworked in place (killer → driver-side
  watcher); `content/SKILL.md` and `docs/workflows/finish.md` no longer
  mention `GROVE_HARNESS_PID`; CHANGELOG carries a breaking-change note for
  the removed env handles.

## Notes

Keep TERM-before-KILL (lets the harness flush session logs). The 2s grace
now covers "the agent's `complete` call returned and the turn is wrapping
up" — same semantics as today, tunable via env.
