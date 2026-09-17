# modular-fixtures-k2

## Goal

Move fixtures for retained behavior to modular configuration so removing the
flat reader does not invalidate unrelated tests. Keep production behavior and
each existing assertion's purpose intact during this increment.

## Context

- Requirements decisions: `plan-k1` and the root brief.
- Primary acceptance: `crates/keyed-launch/tests/` and
  `crates/grove-loop/tests/session_config.rs` load/expansion tests.
- Other consumers include `grove` configuration/loop tests, `grove-llm`
  kind-admission tests, `grove-loop` driver-lease tests and `grove-tui` witnessed
  tests. These are starting points, not an exhaustive file list; inspect helper-
  generated configuration too.

## Done when

- Fixtures testing retained success and failure behavior use modular
  command/bind/route declarations. Assertions still exercise the intended
  routing, argv, admission and semantic errors, with accurate source locations.
- Load/expansion tests and existing downstream consumer tests pass. Do not
  weaken assertions or turn a semantic-error test into old-syntax rejection.
- Dedicated flat/mixed compatibility assertions remaining for `modular-only-k3`
  are identified; they may stay while this increment still supports that format.
- `bash scripts/check.sh` passes. If changing an inline fixture helper covered
  by a walkthrough, reconcile that book in the same increment.

## Notes

Production removal belongs to the next leaf. Use existing test seams; no new
test framework or CLI acceptance suite is requested.

Local documents cannot define commands. Define alternate commands in personal
fixtures and reach them through local bindings/routes while preserving personal
kind admission. Translate literal-dollar cases according to the named-template
scanner so their intended argv remains tested. Preserve the meta-grove signal
guard and existing fake executables; never launch a real agent from tests.
