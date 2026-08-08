# partial-root-recovery-k95

**Kind:** impl

## Goal

Recover an interrupted current-format fresh-tree scaffold without confusing it
with a legacy tree or overwriting foreign partial contents.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` section "Fresh tree".
- Scaffold bytes remain owned by the existing root/leaf writers established by
  `session-kind-tree-k23`.

## Done when

- Every exact subset of `BRIEF.md`, `01-requirements-plan-k1.md`, and absent
  `FORMAT` is recognized and completed under the universal exclusive guard.
- Completion reuses the ordinary scaffold writers and atomically writes
  `FORMAT` last.
- Any differing body, extra task-shaped entry, collision, or foreign partial
  scaffold is refused without overwrite and routes clearly to migration or an
  ambiguity diagnostic.
- Tests cover every interruption point plus near-match refusals.

## Notes

Do not broaden `root_init` into a second migration implementation.
