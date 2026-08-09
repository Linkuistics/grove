# driver-witness-recovery-k123

**Kind:** impl

## Goal

Recover every in-tree finish witness in the bare driver without exposing an
uncommitted finish as fresh rootless lifecycle state.

## Context

- Run only after complete configuration validation and lease acquisition,
  under the universal tree lock and before normal selection/migration.
- Reconstruct exact plain-Git, native-jj, and colocated-jj repository/index
  disposition from the witness manifest; ambiguity remains visibly blocked.

## Done when

- Restart rolls an exactly uncommitted witness back to a selectable finish leaf
  or completes an exactly committed witness forward into ordinary fresh-root
  initialization before selection.
- Repeated crashes during repository/index restoration retain enough
  attempt-owned evidence to retry; a legitimately absent Git index round-trips.
- Divergent topology blocks with diagnostics naming the recorded start and
  exact result recovery paths.
- Driver/process tests cover pre-commit rollback, committed forward recovery,
  divergent blocking, configuration-before-recovery, and fresh-root launch.

## Notes

Do not implement orphan cleanup reaping, adversarial filesystem hardening, or
broaden the driver lease/session-epoch interfaces in this child.
