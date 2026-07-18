# plan-k1

**Kind:** planning

## Goal

Charter this grove and grow the tree. Driving concern (reported 2026-07-18,
live in the harness-switch trial): `grove-llm complete` does not stop a codex
session — every codex task needs a manual `/exit` before the loop relaunches.

## Context

Root cause, established by investigation (reproduced empirically):

- `grove-llm complete` runs inside codex's exec tool, which sandboxes commands
  with macOS Seatbelt. The detached delayed killer it forks inherits that
  sandbox.
- The codex binary's embedded policy contains `(allow signal (target
  same-sandbox))`; the codex TUI process is outside the sandbox, so the
  killer's SIGTERM and SIGKILL both fail with EPERM — silenced by the
  killer script's `2>/dev/null`.
- The signal-file write to `$TMPDIR` is allowed, so after a manual `/exit`
  the loop still relaunches — the loop limps rather than breaks.
- The in-agent killer (self-driving-loop ADR, "self-spawned delayed killer")
  was never harness-neutral: it silently depends on the harness's shell-tool
  sandbox policy. Codex session log evidence:
  `~/.codex/sessions/2026/07/18/rollout-2026-07-18T20-00-07-*.jsonl`
  (complete printed "this session ends in ~2s" — env fine, kill ineffective).

## Done when

The fix's design is agreed (grilled), the tree carries the work as leaves, and
the self-driving-loop ADR rework is chartered.

## Notes

Candidate direction (not yet agreed): move the kill to the loop driver — it is
the harness's parent, outside any sandbox; it can watch for the signal file
mid-session and apply grace → SIGTERM → SIGKILL itself.
