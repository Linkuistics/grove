# finish-transaction-docs-acceptance-k122

**Kind:** impl

## Goal

Reconcile Grove's shipped methodology and durable documentation with the
implemented finish transaction, then prove the complete acceptance contract.

## Context

- Update the minimum coherent existing docs in place; do not append superseding
  ADRs/specs or create Grove-specific durable process artifacts.
- The binding design and glossary already state the contract; documentation
  work should describe implemented names and diagnostics, not redesign it.

## Done when

- `content/SKILL.md`, help/diagnostics, architecture, usage/configuration docs,
  spec, ADR set, glossary, and test-seam descriptions agree with the code.
- Plain Git, native jj, colocated jj, driver restart, lost result, reused handle,
  and cleanup/recovery acceptance tests pass.
- `cargo fmt --check` and `cargo test --locked` pass from a clean verification
  run, and no stale unsafe teardown description remains.

## Notes

This is the final child before the scheduled review of
`finish-transaction-implementation-k110`.
