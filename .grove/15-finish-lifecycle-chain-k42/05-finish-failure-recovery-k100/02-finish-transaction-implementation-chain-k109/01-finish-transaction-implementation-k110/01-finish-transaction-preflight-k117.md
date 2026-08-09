# finish-transaction-preflight-k117

**Kind:** impl

## Goal

Introduce the deep finish-transaction module and integrate a mutation-free
preflight that rejects unsafe Git, native-jj, and colocated-jj teardown inputs
before the existing finish path deletes or stages anything.

## Context

- Preserve the current successful `tree_lifecycle::finish_commit` behavior in
  this slice; later children replace its mutation/commit path repository by
  repository.
- The external interface should be one orchestration entry point. Repository
  inspection and filesystem traversal are internal seams with production and
  test adapters, not new caller-visible policy.
- Preflight owns no-follow `.grove/` identity validation, canonical recursive
  root-entry digests, reserved-witness collision checks, special-entry refusal,
  and same-device cleanup-quarantine validation. Witness preparation,
  finish-attempt identity, and repository outcome anchors remain with the Git
  and jj transaction children that first consume them.
- Reuse the exact-result and workspace-control primitives already established
  by `post-teardown-restart-contract-integrate-k104`; do not duplicate their
  proof rules.

## Done when

- A focused transaction-interface test is observed failing before production
  code, then passes for valid plain Git, native jj, and colocated jj fixtures.
- A symlinked/replaced task root, reserved collision, unsupported entry type,
  undefined digest input, or cross-device cleanup target is rejected before any
  source move, repository staging/snapshot, or Git-index change. Existing plain
  Git empty-tracked-deletion refusal remains intact.
- Canonical no-follow digests cover length-delimited path/type/mode records,
  raw-name-byte-ordered directory children, file bytes, and symlink-target
  bytes without traversing the target.
- Existing successful finish tests remain green, proving this slice strengthens
  preflight without prematurely absorbing evacuation or recovery work.
- `cargo fmt --check` and the focused transaction/finish tests pass.

## Notes

Use TDD and injected local filesystem/repository inspection adapters. Do not
implement evacuation, rollback, commit classification, quarantine handoff, or
driver recovery in this leaf; those are `k118` through `k121`.
