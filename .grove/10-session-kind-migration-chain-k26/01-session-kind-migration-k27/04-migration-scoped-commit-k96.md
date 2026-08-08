# migration-scoped-commit-k96

**Kind:** impl

## Goal

Record migration as one focused plain-Git or Jujutsu commit while preserving
unrelated user work and excluding the live transaction witness.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` section "Scoped Git
  and Jujutsu commits".
- Reuse the jj-first repository seam; colocated repositories are Jujutsu.

## Done when

- Plain Git stages and commits only `.grove/` with the transaction witness
  excluded, including tracked deletions and an unborn repository.
- Pre-existing staged and working-tree changes outside `.grove/` remain outside
  the migration commit and retain their prior state.
- Native and colocated Jujutsu commit only the corresponding `.grove/` fileset,
  leave unrelated working-copy changes in the successor, and do not mutate the
  colocated Git index.
- Commit diagnostics are actionable and failure leaves transaction recovery
  possible.
- Integration tests prove the exact pathspec/fileset behavior.

## Notes

The migration commit is driver-owned and therefore carries no leaf handle.
