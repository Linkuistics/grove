# post-teardown-restart-k99

**Kind:** design

## Goal

Resolve restart semantics when `finish-commit` has successfully committed
`.grove/` deletion but the driver dies before observing `complete --done`.

## Context

- Surfaced while integrating `finish-lifecycle-review-k44` F3; the local retry
  can report "already finished", but a later bare driver sees the same rootless
  shape as a never-started grove and currently initializes new requirements.
- Preserve the artifact-only task tree, fresh-root bootstrap, driver lease and
  session-epoch contracts. Branch/bookmark integration and working-tree removal
  remain outside Grove.
- Binding inputs: `docs/specs/config-driven-sessions.md` sections "Fresh tree"
  and "Finish leaf", plus the Complete finish cycle glossary contract.

## Done when

- The design states which crash windows are recoverable and how a restarted
  bare driver distinguishes a new grove from a completed one, or explicitly
  records why that distinction cannot be made under the existing constraints.
- The minimum coherent spec/ADR/glossary set describes the settled behavior,
  including the no-signal path and handle reuse after legitimate reinitialization.
- Any implementation is cut as separate reviewed work inside
  `finish-lifecycle-chain-k42`; this design leaf does not absorb it.

## Notes

This is lifecycle redesign, not part of `finish-lifecycle-integrate-k45`'s
review cleanup.
