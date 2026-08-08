# legacy-review-removal-k47 — brief

## Goal

Remove obsolete routing peeks, session-target receipts, review target
comparison, and diversity notices while preserving stable review/integration
relationships and promotion.

## Context

- Depends on `legacy-launch-removal-k46` and the earlier
  `receipt-guidance-test-cleanup-k17` compatibility correction.
- Binding design: `docs/adr/grove-owns-escalated-review.md`, the root brief, and
  `docs/specs/config-driven-sessions.md` sections "Authoritative selection and
  mandate" and "Removed surfaces and compatibility".
- Primary code surfaces: `src/task_relationship.rs`, `src/tree_read.rs`,
  `src/tree_lifecycle.rs`, `src/tree_promotion.rs`, `src/llm_cli.rs`,
  `src/loop_driver.rs`, `src/launch.rs`, env scrub support, and
  `tests/producer_receipt.rs` / `tests/kind.rs` / promotion/composition fixtures.

## Done when

- Structured `kind --with-harness --json` routing evidence, launch peeks used
  for routing, `GROVE_SESSION_TARGET`, its scrub/config guards, producer launch
  receipts/generations/source sessions, target comparisons, warnings/notices,
  and receipt-era fixtures are deleted.
- Retirement no longer writes or updates review launch evidence. Mandated
  promotion trusts the named live producer after structural and epoch gates and
  never recomputes pick.
- `Reviews` / `Integrates`, reviewed-entity resolution, chain construction,
  promotion transaction recovery, pruning scope, and the one-review ownership
  rule remain intact behind a smaller relationship module.
- Tests distinguish surviving composition behavior from removed launch policy,
  including promotion after a launch-window insert and terminal/pruned cases.
- Removed-surface sweeps enumerate then classify candidates with positive and
  cross-tree controls; `cargo fmt --check` and `cargo test --locked` pass.

## Decomposition

- `review-routing-removal-k78` removes routing peeks, ambient session targets,
  comparisons, and diversity notices.
- `review-receipt-removal-k84` removes producer launch receipts and retirement
  side effects while preserving promotion and stable relationships.
- `relationship-contraction-k85` contracts the surviving relationship module
  and reconciles fixtures after the obsolete behavior is gone.

## Notes

If `task_relationship.rs` remains shallow after deletion, split surviving
composition and promotion helpers by responsibility rather than keeping a
receipt-shaped shell.

Each child leaves review-chain composition working. The enclosing review chain
stays intact because its reviewer and integrator assess the completed node as
one artifact; neither is a useful product increment without that producer.
