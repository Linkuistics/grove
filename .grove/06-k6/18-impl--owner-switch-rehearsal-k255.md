# owner-switch-rehearsal-k255

## Goal

Rehearse the program on the prepared real plan in disposable layouts, and
write the passing rehearsal record that step 8 and the preludes need.

## Context

- The section "The rehearsed plan" of `docs/specs/owner-skill-delivery.md`
  states the rule. The real HOME receives only a rehearsed switch. A
  passing record names the digests of the plan and the program, and the
  result of each control of "Test seams". It also names the Claude Code
  version of a passing rerun of `settings-writes.sh`.
- Order 6 of the cutover contract states the controls. They cover the
  native settings, the hooks installation, the switch, the probes, the
  inverse and each control of "Test seams".
- `task owner:switch:rehearse` rehearses the fixture plan only. The real
  plan needs a disposable layout built from that plan, with its real watch
  patterns.
- The plan's `accepted` holds real digests. The rehearsal therefore
  resolves with the accepted 2.0 build against a copy of the real kegs, or
  with a stub that reports those digests. The promoted `ravel` exists only
  inside finish.
- The rehearsal runs the real `mac-check` with HOME set to each disposable
  HOME. It reads `~/.local/bin/mac-check` and its folder, and
  `--mac-check` names another command.
- owner-marketplace-freeze-k253 holds Claude Code's version.

## Done when

- A rehearsal task runs the real plan in disposable layouts, and a committed
  record names the plan, the program and each control's result.
- The rerun of `settings-writes.sh` passed, and the record names the Claude
  Code version. That version equals the held version.
- The rehearsal record is in `<state>/rehearsals` of the real HOME, so step
  8 and the preludes admit the prepared plan and program.
- The evidence document states the conditions that keep the record valid.
  An unchanged plan, an unchanged program and the same Claude Code version
  must hold until the switch.

## Notes

discovery-rollout-k12 records each accepted digest again with the published
Ravel archives. If those digests differ, the plan changes, and this
rehearsal runs again.

The rehearsal is reproducible, so a new run after a change costs little.
Never edit a record by hand.
