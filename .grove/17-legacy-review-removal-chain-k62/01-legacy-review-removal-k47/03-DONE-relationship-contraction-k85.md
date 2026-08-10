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
- `review-receipt-removal-k84` established that `task_relationship` is not
  merely shallow, it is **entirely unreferenced**: `src/lib.rs`'s `pub mod` line
  is the only mention of it anywhere in `src/`, `tests/`, `build.rs` or
  `scripts/`. Flipping that line to `pub(crate)` makes the compiler report every
  item in the file — both markers, `read`, `parse`, `parse_handle_marker` and
  `validate_handle` — as never used. `dead_code` is silent today only because a
  `pub` item in a `pub` module is reachable by definition, the same blind spot
  `legacy-launch-cleanup-k83` worked around and `dead-non-launch-exports-k166`
  records the technique for.
- So the question here is not how to split the module but whether it should
  exist. The behaviour it parses is live and *separately implemented*:
  `tree_promotion::declarations` scans `**Reviews:**` / `**Integrates:**` itself
  over a directory, resolving to the node path when a decomposed task carries
  the relationship in its `BRIEF.md` — which is why it never called this. Decide
  between deleting the module and adopting it as the shared parser; do not leave
  two readers of one format.
- `dead-non-launch-exports-k166` explicitly defers `task_relationship` to this
  node ("leave them alone"), so this leaf is where that answer gets recorded.

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
