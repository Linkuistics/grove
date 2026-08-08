# review-receipt-removal-k84

**Kind:** impl

## Goal

Remove producer launch receipts, generations, source-session evidence, and
retirement side effects after review target comparison no longer exists.

## Context

- Depends on `review-routing-removal-k78`.
- Primary surfaces: `src/task_relationship.rs`, `src/tree_lifecycle.rs`,
  `src/tree_promotion.rs`, and `tests/producer_receipt.rs` plus terminal/pruning
  fixtures.
- Preserve `Reviews` / `Integrates`, reviewed-entity resolution, promotion
  recovery, pruning scope, and the one-review ownership rule.

## Done when

- Retirement no longer creates or updates producer launch receipts, producer
  generations, or factual source sessions, and receipt-era fixtures are gone.
- Promotion after a launch-window insert and terminal/pruned composition cases
  remain covered through public seams.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Receipt removal is independently useful once `review-routing-removal-k78` has
made the evidence unreachable at launch.
