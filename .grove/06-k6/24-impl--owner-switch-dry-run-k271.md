# owner-switch-dry-run-k271

## Goal

Build the dry-run instrument and its read-only comparison command. The dry
run executes a plan once in a disposable copy of HOME, including full
restore, and identifies each input by content.

## Context

- The section "The rehearsed plan" of `docs/specs/owner-skill-delivery.md`
  owns the contract. "Test seams" lists its controls. D1 requires the
  content identity of each file that a plan runs.
- owner-switch-contraction-k270 leaves the program without rehearsal
  admission. The switch consumes prepared values, not a passing record.
- `scripts/owner_switch_rehearsal.py` builds disposable layouts for the
  fixture suite. Reuse its mechanics where they fit.
- owner-switch-rehearsal-k255 executes this instrument on the real plan.

## Done when

- The dry run executes the ordinary writes, hook commands, probe cleanup,
  final comparison and full restore. It also tests `native-off` without a
  switch backup. It changes no real configuration, launchd job or login
  item.
- Its result holds a machine-readable input table. Each row names a logical
  input, its executed path, its content digest and its role. The rows cover
  the plan, program, invoked scripts, imported local helpers, command inputs,
  interpreters and external tool versions.
- The dry run digests each subject before and after execution. A changed
  subject invalidates the run.
- The comparison command reads the real prepared inputs through the same
  instrument and reports differences row by row. It permits only the
  declared prefix substitutions, with equal content and versions.
- Fixture controls cover each case of "Test seams". A changed helper at the
  same path fails. Equal content at a declared prefix passes. An unknown
  substitution fails.
- The result also holds per-probe outcomes, the restore comparison and its
  limits. A fixture result claims no real-harness acceptance.

## Notes

The comparison command and its result stay readable outside `.grove/`. The
owner runs it before each real attempt, including after finish.
