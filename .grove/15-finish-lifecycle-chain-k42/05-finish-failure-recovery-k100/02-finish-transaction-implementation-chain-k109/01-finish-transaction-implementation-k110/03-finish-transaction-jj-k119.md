# finish-transaction-jj-k119

**Kind:** impl

## Goal

Extend the fail-closed finish transaction to native and colocated Jujutsu
without weakening plain-Git behavior.

## Context

- Model jj partial commits by exact working-copy commit ID, change identity,
  and parents; change-id presence elsewhere is not proof.
- In colocated jj, back up the user's Git index before any preflight snapshot,
  prepare the success image before commit, and activate or restore the correct
  image only after the corresponding repository outcome is proven.

## Done when

- Native-jj and colocated-jj exact committed and exact uncommitted outcomes are
  disjoint and preserve unrelated working-copy/index bytes.
- Rollback reproduces the manifest's exact preflight commit ID; an unexpected
  rewrite, `jj edit`, workspace topology, or auxiliary restoration failure
  remains `Recovery pending`.
- Lost-result and quarantine recovery always move forward after exact proof and
  never restore the old tree.
- Focused native/colocated process tests and transaction transition tests pass.

## Notes

Reuse the transaction interface and manifest written by `k117`/`k118`.
