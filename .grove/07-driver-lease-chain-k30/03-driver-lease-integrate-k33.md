# driver-lease-integrate-k33

**Kind:** integrate-review-impl
**Integrates:** driver-lease-review-k32

## Goal

Apply the verified findings from `driver-lease-review-k32` while preserving the reviewed artifact's contract.

## Context

- Verify every `driver-lease-review-k32` finding against the design's explicit
  workflow-consistency and repository-corruption limits.
- Do not solve epoch admission early by widening the lease interface.

## Done when

- Every finding has a recorded disposition; verified issues are fixed with
  deterministic race or multi-workspace regression coverage.
- The driver lease remains a small process-owned interface and leaves tree
  operation serialization to the existing tree seam.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Substantial epoch work is externalized under `session-epoch-chain-k34`.
