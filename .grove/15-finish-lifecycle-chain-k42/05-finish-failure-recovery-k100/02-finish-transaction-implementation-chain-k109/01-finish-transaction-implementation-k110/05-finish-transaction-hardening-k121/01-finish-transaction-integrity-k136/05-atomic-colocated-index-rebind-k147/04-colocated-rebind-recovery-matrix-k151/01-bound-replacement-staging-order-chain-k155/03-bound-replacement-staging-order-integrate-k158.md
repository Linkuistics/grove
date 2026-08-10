# bound-replacement-staging-order-integrate-k158

**Kind:** integrate-review-impl
**Integrates:** bound-replacement-staging-order-review-k157

## Goal

Apply the verified findings from `bound-replacement-staging-order-review-k157`
while preserving the reviewed artifact's contract.

## Done when

- Every accepted finding is fixed, each first demonstrated by a test that fails
  against the reviewed producer commit; every rejected finding is answered here
  with its reason.
- The publication order still leaves no entry that no live document describes,
  and nothing deletes or moves an entry whose identity it has not proven.
- `cargo fmt --check` and `cargo test --locked` pass from the final tree.
