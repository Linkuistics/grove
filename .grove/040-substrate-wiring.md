# 040-substrate-wiring

**Kind:** work

## Goal

Build and prove the **self-driving loop** that is grove's new runtime (ADR-0032):
a stateless loop driver that runs one grove task per fresh foreground `claude`,
ended by an out-of-band completion signal, until `grove-llm pick` is empty. This
is the **critical-path leaf** — the whole substrate decision rides on it working.

## Context

Read **ADR-0032** (the substrate decision) and the **030 running log** (D1, D3,
D4, D6, and the loop-interrupt OPEN ISSUE) first. The mechanism is settled at the
decision altitude; this leaf realises it and resolves the wiring details a PoC
must decide. No PTY wrapper, no `portable-pty`, no DB.

## Done when

- A **loop driver** exists (`grove do <name>` drives the *whole loop*, not one
  task): `while grove-llm pick has work → launch foreground claude → on exit,
  relaunch-or-stop`. Grilling/resize/Ctrl-C work natively (claude owns the TTY).
- A `grove-llm` **completion-signal verb** exists, run by the agent as its last
  step (after commit + retire); it triggers the **out-of-band kill** of the claude
  session.
- The **kill realisation** is chosen and built — lean **(b) self-spawned delayed
  killer** (verb forks a detached `sleep <grace>; kill -TERM $GROVE_CLAUDE_PID`,
  returns immediately; SIGKILL fallback), with **(a) file-watch daemon** as the
  fallback if the PoC finds (b) unreliable.
- **Interrupt/stop semantics** (the OPEN ISSUE) are implemented: **relaunch is
  opt-in** — the loop relaunches *only* when the completion signal fired; any
  other exit (human `/exit`/Ctrl-C, crash) **stops** the loop, resumable by
  re-running `grove do <name>`. The driver itself survives the human's interrupt
  (trap/ignore SIGINT) so it reaches the relaunch-vs-stop decision.
- **The PoC passes:** a foreground `claude` in the loop survives a real
  interactive session (multi-turn grilling, terminal resize, Ctrl-C within the
  task), the signal verb's env-handle (`GROVE_CLAUDE_PID`) reaches the agent's
  Bash tool, the kill ends the session cleanly, the terminal is reset
  (`stty sane`/`tput rmcup`) and a fresh task relaunches. If the PoC surfaces a
  blocker, escalate to ADR-0032.
- The loop body is **stateless/self-locating** (re-derives from `pick`), so
  restart ≡ continuation; commit-before-retire keeps a crashed mid-task leaf
  redoable.

## Notes

- This leaf may itself decompose (driver / signal verb / interrupt semantics / PoC)
  if it proves too big for one session — decide lazily when picked.
- The signal verb is the in-loop hook the skill's loop instructions call; design
  it so the skill can name it as "the last thing you do when a task is complete."
- Keep it walk-away-able: the driver is a small shell/Rust surface, no hidden
  state; a plain `while` loop must be able to stand in for it.
