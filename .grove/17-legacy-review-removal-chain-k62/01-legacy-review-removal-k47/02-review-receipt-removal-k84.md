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
- `review-routing-removal-k78` already took the receipt **writer** with the
  ambient session target it read its harness and model from — the two could not
  be separated, since removing the variable left the writer no way to build a
  target. Retirement and pruning therefore already write nothing, and
  `producer_generation_unlocked` is gone with the evidence path that called it.

## Done when

- The receipt **format** is gone: `ProducerLaunchReceipt`, `LaunchTarget`, the
  `**Producer launch:**` marker and its parser, and
  `TaskRelationships::producer_launch` no longer exist, and receipt-era fixtures
  in `tests/producer_receipt.rs` and `tests/kind.rs` go with them.
- Retirement still creates or updates no producer launch receipt, producer
  generation, or factual source session — now because there is nothing left to
  create rather than because no caller reaches it.
- Promotion after a launch-window insert and terminal/pruned composition cases
  remain covered through public seams.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Receipt removal is independently useful once `review-routing-removal-k78` has
made the evidence unreachable at launch. What that slice left standing is the
*parse* side: Grove still understands a legacy receipt line well enough to
preserve it byte-for-byte, which is the behaviour this slice retires along with
the type.
