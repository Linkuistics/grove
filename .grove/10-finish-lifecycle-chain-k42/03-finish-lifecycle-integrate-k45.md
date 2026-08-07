# finish-lifecycle-integrate-k45

**Kind:** integrate-review-impl
**Integrates:** finish-lifecycle-review-k44

## Goal

Apply the verified findings from `finish-lifecycle-review-k44` while preserving the reviewed artifact's contract.

## Context

- Verify every `finish-lifecycle-review-k44` finding against the spec and the
  human-confirmation boundary.
- Keep branch/bookmark integration and working-tree removal out of Grove.

## Done when

- Every finding has a recorded disposition; verified issues are fixed with
  isolated Git/jj finish regressions.
- Finish remains a normal configured session plus one deterministic guarded
  teardown helper, with `complete --done` last.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Substantial lifecycle redesign is new work inside
`finish-lifecycle-chain-k42`, not cleanup to absorb here.
