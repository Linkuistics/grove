# architecture-records-reconciliation-k88

**Kind:** impl

## Goal

Reconcile architecture, ADRs, specs, and citations into the minimum coherent
durable description of the implemented system.

## Context

- Depends on `user-docs-reconciliation-k80`.
- Primary artifacts: `docs/ARCHITECTURE.md`, the ADR set named by the root
  brief, `docs/specs/config-driven-sessions.md`, and
  `docs/specs/doubt-grove-review-mechanics.md`.
- Edit, merge, split, or delete records in place; never append a superseding
  record, and reconcile every citation changed by the rework.

## Done when

- Architecture describes configuration, process ownership, tree/migration,
  bare driver, finish, provisioning, Git/jj symmetry, and lock ordering at their
  implemented seams.
- The ADR set is minimal and coherent; the review-mechanics spec retains only
  promotion, locking, and ownership material that still binds after receipt and
  target comparison deletion.
- Every changed citation resolves, and focused docs/navigation checks plus
  `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Architecture, ADRs, and specs stay together because they are one binding set;
landing only one would leave contradictory design authority.
