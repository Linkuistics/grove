# review-routing-removal-k78

**Kind:** impl

## Goal

Remove review routing peeks, ambient target evidence, comparisons, and
diversity notices without disturbing review-chain composition.

## Context

- Depends on `legacy-launch-removal-k46` and
  `receipt-guidance-test-cleanup-k17`.
- Primary surfaces: `src/tree_read.rs`, `src/llm_cli.rs`,
  `src/loop_driver.rs`, `src/launch.rs`, config/env guards, and routing tests.
- Preserve producer receipts temporarily; `review-receipt-removal-k84` removes
  them after no launch path consumes target comparison.

## Done when

- Structured `kind --with-harness --json` evidence, routing peeks,
  `GROVE_SESSION_TARGET`, target comparison, and diversity warnings/notices are
  removed from production and tests.
- Mandated promotion still trusts its named live producer after structural and
  epoch gates and never recomputes pick.
- Focused routing/promotion tests, `cargo fmt --check`, and `cargo test --locked`
  pass.

## Notes

This increment removes runtime policy while leaving receipt cleanup as a
separate, green contraction.
