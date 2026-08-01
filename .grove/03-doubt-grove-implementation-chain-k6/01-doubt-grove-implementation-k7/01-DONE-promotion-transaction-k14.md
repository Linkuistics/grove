# promotion-transaction-k14

**Kind:** impl

## Goal

Implement `grove-llm leaf-promote-chain` as the atomic, fail-closed task-tree
operation specified by `docs/specs/doubt-grove-review-mechanics.md`.

## Context

This is the first child of `doubt-grove-implementation-k7`. Implement only the
tree-access lock, pending-promotion guard and recovery, stable relationship
metadata, CLI surface, and VCS-symmetric promotion transaction. Producer launch
receipts and review-target comparison belong to later siblings.

## Done when

- Every steady-state reader and mutator serialises through the shared/exclusive
  root-directory guard and refuses a recursively visible `PROMOTING-*` witness.
- Promotion strictly accepts only the currently picked plain producer, preserves
  its bytes and stable handle, derives a brief-less review chain with stable
  relationships, and is idempotent by stale path, handle, or recovery path.
- Process interruption and reported failures leave either the original producer,
  a blocking recoverable transaction, or the complete chain; tracked Git and
  native/colocated Jujutsu retain their documented rename/index behavior.
- Plain and JSON CLI output, help, error codes, allocation, ordering, and focused
  concurrency/recovery/failure tests match the reviewed design.

## Notes

Use TDD at the CLI boundary. Keep transaction internals behind one deep module
interface; do not begin the receipt or launch-warning slices here.
