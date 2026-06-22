# 10-[22]-complete-terminate-signal

**Kind:** work

## Goal

Give the self-driving loop a **positive** end-of-grove signal: `grove-llm complete`
gains a parameter so the agent can either **relaunch** for the next task (today's
behaviour, the default) or **terminate** the loop cleanly when the whole grove is
finished — instead of the loop only ever stopping by the *absence* of a signal.

## Context

Read **ADR-0032** (the self-driving loop) first. Today (`src/complete.rs`,
`src/loop_driver.rs`): `complete` writes a relaunch flag + kills the session, so
the loop relaunches; the finish cycle deliberately does **not** call `complete`
("Signal/Finish" in `content/SKILL.md`), so the loop stops by seeing no flag. That
conflates three different stops — clean whole-grove finish, human Ctrl-C, and a
crash all look identical to the loop. Making termination an explicit signal lets
the loop tell a *clean finish* from an *abnormal exit*.

This is loop-runtime work (touches `complete` + the driver), independent of the
080/090 sheds, so its trailing position doesn't matter — but it **must land before
the major release** (root BRIEF rollout note).

## Done when

- `grove-llm complete` takes a flag (e.g. `--done` / `--terminate`, vs the default
  relaunch). Default behaviour is unchanged (relaunch the next task).
- The loop driver, on a `--done` signal, **stops without relaunching** and reports
  a clean finish — distinct from the no-signal stop (which stays "stopped, resume
  with `grove do`").
- `relaunch is opt-in` (ADR-0032) still holds: any non-signalled exit still stops.
- `content/SKILL.md` (and its skill mirror) updated: the **Finish** step changes
  from "do NOT run `complete`" to "run `complete --done` as the last teardown
  action"; the per-task **Signal** step stays plain `complete`. Edit both copies
  in one commit (the `content/` canonical + `.claude/skills/grove/` mirror).
- Tests: `tests/complete.rs` / `tests/loop_driver.rs` cover the terminate path
  (no relaunch flag written; driver exits clean) alongside the existing relaunch
  path.

## Notes

- Captured 2026-06-21 from the 050 session: the param lets the LLM both restart
  *and* terminate-when-fully-done, rather than relying on "just don't signal".
- Small, well-scoped; the design above is essentially settled — no grilling needed,
  hence **work**, not planning.
- Decide the exact flag spelling at implementation (e.g. `--done` reads best
  against the Finish-cycle call site `grove-llm complete --done`).
