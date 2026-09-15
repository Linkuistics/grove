# live-reading-k13 — brief

## Goal

Complete live-viewer-k5: reconcile reading anchors across content edits and
establish the parent's complete real-clock recovery and delivery evidence.

## Context

live-observation-k12 supplies a runnable poll-driven monitor with keyed item
state and root lifetimes. Markdown Anchor currently describes a source byte
and offset within a transformed event, with no source edit mapping.

## Done when

- Implement source edit mapping for current and saved per-item positions,
  retaining unchanged source lines/blocks, deterministic duplicate matching,
  nearest surviving fallback and clamping. Preserve positions through selected
  file errors and content inserted above the visible marker.
- Complete every parent Done when, including its separate production-clock
  filesystem recovery matrix, no-write checks, actual Grove mutation, real
  binary PTY live/resize/navigation/restoration smoke and install/package path.
- Run all parent-required workspace, Rust 1.85, build and book checks; record
  commands/results. Reconcile README, usage/key help, architecture, command
  coverage and delivery docs; remove the temporary edit-anchor limitation.
- Check the parent and root contracts against delivered evidence and close the
  node only when satisfied; add precise follow-up work for actual gaps.

## Notes

Keep the parent's full acceptance list authoritative. No injected notification
or direct Refresh action substitutes for the production-clock integration tests.

## Decomposition

- edit-anchors-k14 delivers source-edit mapping through the application seam,
  including focused regression tests and accurate reading-position documentation.
- live-delivery-k15 completes the parent's production-clock recovery matrix,
  workspace/Rust-floor/book checks, real binary terminal smoke and package/install
  evidence. It checks every parent/root acceptance clause before closure.
