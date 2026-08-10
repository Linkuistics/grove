# finish-transaction-docs-acceptance-k122 — brief

**Kind:** impl

## Goal

Reconcile Grove's shipped methodology and durable documentation with the
implemented finish transaction, then prove the complete acceptance contract.

## Context

- Update the minimum coherent existing docs in place; do not append superseding
  ADRs/specs or create Grove-specific durable process artifacts.
- The binding design and glossary already state the contract; documentation
  work should describe implemented names and diagnostics, not redesign it.
- **This assumption did not hold for the lost-result contract.** The spec's
  "Crash and retry semantics" and the glossary's Complete finish cycle require a
  rootless retry to prove the exact handle-and-attempt-named `.grove/`-scoped
  commit, but `tree_lifecycle::finish_commit` refuses on task-root absence
  before reaching any repository seam. Two of the named acceptance scenarios —
  lost result and reused handle — therefore test unimplemented behavior. That
  gap is this node's first child; documentation stays a reconciliation task
  behind it.
- The existing `repo::recover_finish` proof is manifest-anchored and cannot
  serve the rootless case, whose witness is already disposed. Do not weaken it
  or grow a competing witness-based rule.

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

Non-finish documentation debt is out of scope here. `grove do`, `--harness`
flags, harness/model policy, and producer launch receipts still appear across
`docs/USAGE.md`, `docs/CONFIGURATION.md`, and `content/SKILL.md`; those belong
to the root-level legacy-removal and durable-docs leaves. Reconcile only what
the finish transaction actually changed.

## Decomposition

- `finish-lost-result-retry-k163`: implement the self-contained, attempt-bound
  rootless retry proof and cover lost result and reused handle across plain Git,
  native jj, and colocated jj.
- `finish-teardown-docs-acceptance-k164`: reconcile the methodology, CLI
  help/diagnostics, and durable docs with the implemented transaction, then run
  the complete acceptance verification.
