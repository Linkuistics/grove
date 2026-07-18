# grove-harness-switch — brief

## Goal

Trial hardening: keep the codex+gpt-5.6-sol vs pi+K3 harness trial (running to
~2026-08-18) working smoothly. Friction discovered while driving groves on
either harness lands here as new leaves rather than new groves.

## Done when

The trial runs its course without loop-mechanics friction: sessions end and
relaunch hands-free on every harness, and no known harness-specific defect in
grove's runtime is left unaddressed.

## Decomposition

First concern (grilled 2026-07-18): `grove-llm complete` cannot stop a codex
session — codex's Seatbelt sandbox denies the in-agent delayed killer's
signals (`(allow signal (target same-sandbox))`; EPERM, silenced by
`2>/dev/null`). Agreed fix: **driver-side watcher** — the loop driver polls
for the signal file while the harness runs and applies grace → SIGTERM →
kill-grace → SIGKILL to its own child; `complete` reduces to writing the
signal file; the `sh -c 'export GROVE_HARNESS_PID=$$ …'` wrapper and both PID
exports are removed; the `self-driving-loop` ADR is reworked in place.

## Pointers

- Trial design + dates: `docs/superpowers/specs/2026-07-18-codex-pi-harness-switch-design.md`
- Current mechanism ADR (to rework): `docs/adr/self-driving-loop.md`
- Kill path today: `src/complete.rs` (`spawn_delayed_killer`), `src/loop_driver.rs` (`launch_session`)
- Codex-side evidence: `~/.codex/sessions/2026/07/18/rollout-2026-07-18T20-00-07-*.jsonl`

## Notes

Test seam (agreed): the existing fake-harness-script seam via
`GROVE_HARNESS_BIN` driving the real `run_loop`, with
`GROVE_KILL_GRACE`/`GROVE_KILL_GRACE_KILL` (now read by the driver) keeping
tests fast. No new seams.
