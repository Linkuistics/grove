# colocated-rebind-recovery-matrix-k151

**Kind:** impl

## Goal

Integrate the bound replacement protocol into colocated-Jujutsu finish and prove
the complete crash, retry, and substitution matrix at process level.

## Context

- The preceding slices provide child quiescence, recoverable marker replacement,
  and exact replacement ownership.
- Repository preparation and finish-witness recovery must now drive those
  primitives without losing the exact Git index backup.

## Done when

- Every marker-rebind process checkpoint recovers without same-attempt marker
  collision and restores or disposes the correct index.
- Synchronous errors roll back exact index bytes and permit a clean retry.
- Process tests cover artifact and marker substitution without deleting external
  bytes.

## Notes

This is the integration slice that closes `atomic-colocated-index-rebind-k147`
when the parent Done-when remains true end to end.
