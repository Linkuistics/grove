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

## Finding dispositions

- **F1 — fixed.** Recovery now treats the durable destination (`moved`) as the
  authoritative landing witness before consulting `source`. This makes a
  same-path legacy-v2 node brief idempotent after commit-before-`COMMITTED`.
  Forward and rollback transition-boundary matrices now exercise both old-NNN
  and legacy-v2 node trees.
- **F2 — fixed.** Partial root-init recovery now requires a routed leaf or
  `.FORMAT.tmp` as strong scaffold evidence; the root brief is format-neutral.
  It preserves custom first-leaf slugs, routes custom brief-only trees through
  legacy classification, and distinguishes accepted legacy-v2 slugs beginning
  `requirements-` from current root-init leaves by their explicit legacy kind
  metadata. Exact scaffold collisions still fail closed.
- **F3 — fixed.** Both compatibility migration entry points now refuse the
  canonical pending session-kind transaction witness before planning or
  adoption commit work.
- **F4 — fixed.** Git pathspecs and jj filesets derive the excluded witness from
  `tree_access::MIGRATION_TRANSACTION`; the duplicate literal was removed.
- **F5 — fixed.** `write_current_last` owns the safety check: an existing
  `.FORMAT.tmp` must be a regular file with exact expected bytes, and a new
  temporary is created exclusively. Recovery cannot follow or clobber a
  symlink.
- **F6 — fixed.** Top-level legacy routing markers are removed, one-to-three
  space indented markers outside code fences are refused, and valid fenced
  examples are preserved. A backtick fence candidate whose info string
  contains a backtick is not treated as a fence, so it cannot hide metadata.
- **F7 — fixed.** Fresh verification evidence is recorded below.
- **F8 — accepted informational tradeoff; no code or spec change.** In a
  colocated jj workspace, jj remains authoritative while the original Git
  index is restored byte-for-byte. A transient staged-in-reverse Git view is
  therefore expected and does not lose or absorb user work; the existing
  colocated-index regression tests cover the promised behavior.

The fresh-context integration review found two residual cases within F2 and
F6 (the `requirements-` legacy-slug overlap and invalid backtick opener); both
were reproduced with failing tests and fixed as recorded above.

## Verification evidence

- `cargo fmt --check` — exit 0.
- `cargo test --locked --lib tree_migration_transaction::tests` — 25 passed,
  0 failed.
- `cargo test --locked --lib tree_migrate::tests::current_plan_` — 14 passed,
  0 failed.
- `cargo test --locked --test migration_commit` — 10 passed, 0 failed.
- `cargo test --locked` — exit 0; 403 library tests and every integration and
  doc-test binary passed with no failures.
