# session-kind-tree-integrate-k25

**Kind:** integrate-review-impl
**Integrates:** session-kind-tree-review-k24

## Goal

Apply the verified findings from `session-kind-tree-review-k24` while preserving the reviewed artifact's contract.

## Context

- Verify every `session-kind-tree-review-k24` finding against the binding spec.
- Keep legacy interpretation confined to migration inputs; do not restore
  current body metadata as a compatibility shortcut.

## Done when

- Every finding has a recorded disposition and every verified issue is fixed
  with a public-seam regression test.
- The format witness, filename grammar, finish reservation, and composition
  relationships remain one coherent contract.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

If a fix requires migration transaction design, externalize it under
`session-kind-tree-chain-k22` rather than absorbing `session-kind-migration-k27`.
