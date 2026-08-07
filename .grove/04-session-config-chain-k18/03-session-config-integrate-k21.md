# session-config-integrate-k21

**Kind:** integrate-review-impl
**Integrates:** session-config-review-k20

## Goal

Apply the verified findings from `session-config-review-k20` while preserving the reviewed artifact's contract.

## Context

- Read `session-config-review-k20` and verify every finding against the binding
  spec before changing the reviewed module.
- Preserve the expand-only boundary: do not wire configuration into the driver
  or delete legacy routing in this integration step.

## Done when

- Every finding is classified as unclear contract, real issue, visible
  trade-off, or noise, with the disposition recorded in this task file.
- Verified issues are fixed at the configuration seam with regression tests;
  rejected findings have concrete evidence.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Substantial driver or tree redesign is new work inside `session-config-chain-k18`,
not scope to absorb here.
