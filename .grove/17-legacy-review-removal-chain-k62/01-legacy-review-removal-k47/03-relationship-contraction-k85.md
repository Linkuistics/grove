# relationship-contraction-k85

**Kind:** impl

## Goal

Contract the relationship module and its fixtures around only the stable
composition and promotion behavior that survives receipt deletion.

## Context

- Depends on `review-receipt-removal-k84`.
- Split surviving composition and promotion helpers by responsibility if the
  receipt-shaped module shell no longer earns its interface.
- Reconcile all tests and guidance-facing fixtures against the smaller seam.

## Done when

- `Reviews` / `Integrates`, chain construction, reviewed-entity resolution,
  promotion transaction recovery, pruning scope, and one-review ownership have
  focused homes and public-seam coverage.
- Removed-surface checks enumerate and classify candidates with positive and
  cross-tree controls.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

This final cleanup follows two behavior-complete increments, so module
restructuring never masks whether the removals themselves were correct.
