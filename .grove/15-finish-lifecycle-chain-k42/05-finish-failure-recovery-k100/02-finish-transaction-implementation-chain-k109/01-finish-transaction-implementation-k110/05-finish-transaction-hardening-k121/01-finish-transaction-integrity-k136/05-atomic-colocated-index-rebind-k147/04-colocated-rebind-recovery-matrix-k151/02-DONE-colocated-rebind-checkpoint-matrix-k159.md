# colocated-rebind-checkpoint-matrix-k159

**Kind:** impl

## Goal

Make every marker-rebind interruption reachable from a real process and prove
each one recovers in a colocated-Jujutsu tree with the correct index.

## Context

- `rebind_artifact_identity_with` already takes a step checkpoint, but the
  production entry point passes a no-op, so every rebind boundary is currently
  reachable only from unit tests. `finish_cleanup` and `finish_transaction`
  already carry deterministic test-only interruption seams to copy.
- The rebind happens during repository preparation, so an interruption there is
  recovered by the preparing-witness path, not by the committed-finish path.
- This leaf owns the crash and synchronous-failure axes. Substitution is
  `colocated-rebind-substitution-matrix-k160`.

## Done when

- Every marker-replacement step is reachable as a deterministic process
  interruption through a test-prefixed seam that is not user configuration and
  is scrubbed from launched sessions like its siblings.
- A colocated-Jujutsu process test drives each step, restarts, and shows the
  original Git index restored byte-identically, no auxiliary evidence left, the
  finish leaf still live, and the working-copy commit unchanged.
- A synchronous failure at each step leaves the exact index bytes and permits a
  clean same-attempt retry rather than a collision.
- Any contract gap the matrix demonstrates is fixed here, in the transaction
  interface rather than in its callers.

## Notes

Fix only gaps first demonstrated by a failing test.
