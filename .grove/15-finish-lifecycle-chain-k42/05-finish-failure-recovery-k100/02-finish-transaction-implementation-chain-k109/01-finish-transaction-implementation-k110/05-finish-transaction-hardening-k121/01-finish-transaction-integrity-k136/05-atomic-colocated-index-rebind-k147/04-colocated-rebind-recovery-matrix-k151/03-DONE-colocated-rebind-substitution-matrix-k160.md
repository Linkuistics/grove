# colocated-rebind-substitution-matrix-k160

**Kind:** impl

## Goal

Prove at process level that substituting a colocated-Jujutsu finish auxiliary's
artifact or marker fails closed without moving or deleting external bytes.

## Context

- The unit suite already covers substitution at the transaction interface. This
  leaf proves the same properties through the real `grove-llm finish-commit` and
  bare `grove` recovery path in a colocated tree, where the auxiliary directory
  is the user's `.git/` and the neighbouring names are `index`, `HEAD` and
  `config`.
- `bound-replacement-staging-order-k156` removed the reclamation that could
  delete an unproven entry; this leaf is where that removal is demonstrated
  end to end, including a foreign entry left at the deterministic replacement
  name and at the reserved staging namespace.
- Crash and synchronous-failure axes belong to
  `colocated-rebind-checkpoint-matrix-k159`.

## Done when

- Process tests substitute the artifact, the canonical marker, the staged
  marker, the replacement state document, and a foreign entry at each Grove-owned
  replacement name, and each run leaves every external inode and its bytes
  intact.
- Each refusal names the witness or auxiliary and gives an actionable diagnostic
  that does not claim anything was left untouched on a path where a mutation
  landed.
- Any contract gap the matrix demonstrates is fixed at the transaction
  interface.

## Notes

Fix only gaps first demonstrated by a failing test.
