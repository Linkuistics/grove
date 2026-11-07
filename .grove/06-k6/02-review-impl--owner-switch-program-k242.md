# owner-switch-program-k242

**Reviews:** owner-switch-program-k91

## Goal

Read the whole owner switch program and its rehearsal adversarially. Find
each way that the program can change the real HOME against
`docs/specs/owner-skill-delivery.md`. Find each success that it reports
without evidence. Produce findings, not fixes.

## Context

- The node brief `owner-switch-program-k91` and its three retired slices
  hold the contract, the decisions and the verification of each slice:
  owner-switch-core-k239, owner-switch-preludes-k240 and
  owner-switch-gates-k241.
- The subjects are `scripts/owner_switch.py`,
  `scripts/owner_switch_rehearsal.py`, the task `owner:switch:rehearse` and
  `docs/models/owner-switch.qnt`.
- The spec sections are "The switch inside finish", "Concurrent writers",
  "Resuming the switch", "The rehearsed plan", "Rollback", "The recovery
  model", the two prelude sections and "Test seams".

## Done when

- Each finding names the defect, a concrete input or state that shows it,
  and the spec rule or control it breaks.
- The findings cover these doubts. The recovery mechanism has no executable
  test outside its rehearsal, so each doubt needs a reading of the code, not
  only of the controls.

## Notes

The doubts to test:

- **Fidelity of the fixtures.** The stubs of `claude`, `brew`, `mise` and
  `ravel` reproduce shapes measured once, on 2026-11-07. Find each place
  where a real tool can answer in a shape that the program reads as a pass.
  Examples are a `brew info` revision, a `mise` state other than
  `installed`, or a `ravel` failure document.
- **The recovery paths.** Check each state of the table against
  `find_state` and `run_switch`, including the continuation that runs the
  program copy of a backup. Check that the active continuation reads step
  4's admitted digests from its own backup.
- **The write intervals.** Check that each write runs inside a closed
  process gate and leaves a kept copy that the flag rule reads.
- **Step 7.** The program cannot see which directories finish loaded.
  Judge whether the registrations of enabled plugins and the paths in
  `OWNER_SWITCH_READERS` suffice, and whether an update during finish
  escapes them.
- **The plan's authority.** The accepted values, the commands and the
  baseline come from the plan, and a passing rehearsal record admits it.
  Find a command or field that a plan can use to act outside the rehearsed
  behaviour.
- **The environment.** Each command now runs with HOME set to the switch's
  HOME, and a resolution drops every `RAVEL_*` name. Find a variable that
  still reaches a probe, a trial or a resolution and changes its subject.
