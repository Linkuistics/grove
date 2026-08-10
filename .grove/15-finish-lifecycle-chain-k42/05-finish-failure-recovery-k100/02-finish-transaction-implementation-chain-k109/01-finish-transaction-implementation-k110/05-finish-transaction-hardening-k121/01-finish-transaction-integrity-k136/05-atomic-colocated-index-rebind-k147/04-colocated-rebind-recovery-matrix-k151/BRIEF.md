# colocated-rebind-recovery-matrix-k151 — brief

**Kind:** impl

## Goal

Integrate the bound replacement protocol into colocated-Jujutsu finish and prove
the complete crash, retry, and substitution matrix at process level.

## Context

- The preceding slices provide child quiescence, recoverable marker replacement,
  and exact replacement ownership.
- Repository preparation and finish-witness recovery must now drive those
  primitives without losing the exact Git index backup.
- `bound-success-index-replacement-integrate-k154` added
  `reclaim_unbound_replacement`, which **unlinks** a regular file at the
  deterministic replacement name when no state document owns it. Its ownership
  argument is the collision gate plus the attempt-scoped name, not proof that
  Grove created that inode, so it is the one place in this subtree that can
  delete bytes it did not prove it wrote. The substitution matrix below owns
  disproving that: the reachable counterexample is a substitution landing inside
  `replace_artifact_from`'s post-copy identity window, which leaves a foreign
  regular file at that name for a later recovery to reclaim.
- That counterexample was confirmed reachable while planning this node, and the
  cheap disposition — refuse instead of unlink — is **not** available: the
  finish attempt identity is the per-launch signal nonce, so two `finish-commit`
  runs inside one driver launch share an attempt and a refusal would wedge the
  same launch. The disposition therefore has to remove the unowned state rather
  than react to it.

## Done when

- Every marker-rebind process checkpoint recovers without same-attempt marker
  collision and restores or disposes the correct index.
- Synchronous errors roll back exact index bytes and permit a clean retry.
- Process tests cover artifact and marker substitution without deleting external
  bytes, including a substituted replacement reaching the reclamation seam.
  Either that reclamation is shown never to delete a foreign entry, or its
  disposition changes to one that cannot.

## Decomposition

- `bound-replacement-staging-order-chain-k155`: change the disposition first,
  because it changes the state machine the two matrices below enumerate — no
  unowned entry is ever published at the deterministic replacement name, so the
  reclamation that could delete foreign bytes stops existing. Reviewed as a
  chain because the preceding protocol slice of the same shape drew a
  high-severity finding.
- `colocated-rebind-checkpoint-matrix-k159`: process-level interruption seam for
  the marker-rebind steps, then the colocated-Jujutsu crash and
  synchronous-failure matrix, including clean same-attempt retry.
- `colocated-rebind-substitution-matrix-k160`: process-level artifact and marker
  substitution, proving no external bytes are moved or deleted.

## Notes

This node closes `atomic-colocated-index-rebind-k147` when the parent Done-when
remains true end to end.
