# session-kind-migration-integrate-k29

**Kind:** integrate-review-impl
**Integrates:** session-kind-migration-review-k28

## Goal

Apply the verified findings from `session-kind-migration-review-k28` while preserving the reviewed artifact's contract.

## Context

- Verify every `session-kind-migration-review-k28` finding against the spec and
  the existing promotion transaction's fail-closed discipline.
- Preserve process-interruption consistency without expanding the claim to
  power-loss durability.

## Done when

- Every finding has a recorded disposition; verified issues are fixed with
  deterministic recovery or VCS regression tests.
- The migration/fresh-tree interface remains suitable for one lifecycle caller
  and current trees remain a no-op.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Driver ordering belongs to `lifecycle-cutover-k39`; keep this integration at
the tree/VCS seam.
