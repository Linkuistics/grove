# session-epoch-integrate-k37

**Kind:** integrate-review-impl
**Integrates:** session-epoch-review-k36

## Goal

Apply the verified findings from `session-epoch-review-k36` while preserving the reviewed artifact's contract.

## Context

- Verify every `session-epoch-review-k36` finding against the protocol's
  cooperative-workflow and process-interruption scope.
- Keep the lease, epoch, and Tree access lock as three distinct interfaces with
  the specified acquisition order.

## Done when

- Every finding has a recorded disposition; verified races are fixed with a
  deterministic barrier/event-trace or black-box regression.
- No test-only clock, randomness, lock, or grace control becomes user-visible
  environment configuration.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Any lifecycle policy change belongs to `lifecycle-cutover-k39`.
