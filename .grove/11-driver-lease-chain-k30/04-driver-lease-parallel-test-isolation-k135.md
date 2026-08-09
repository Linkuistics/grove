# driver-lease-parallel-test-isolation-k135

**Kind:** impl

## Goal

Make the driver-lease unit suite deterministic under Cargo's default parallel
test runner without weakening the lease/epoch handoff assertions.

## Context

- A full `cargo test --locked` run while completing
  `cleanup-driver-acceptance-k127` intermittently failed
  `a_successful_liveness_probe_releases_the_lease_before_validation` and
  `replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff`.
- Both tests passed immediately when rerun individually with `--exact`, which
  points to process-global test-hook or synchronization interference rather
  than a deterministic production failure.
- `driver-lease-review-k32` previously established that this subsystem must
  remain safe under Cargo's parallel runner; preserve that standard.

## Done when

- The shared state or timing dependency that lets the two named tests interfere
  with parallel siblings is identified with a repeatable stress reproducer.
- Tests isolate the affected process-global hooks or coordinate them through one
  explicit test seam while retaining their original event-order assertions.
- Repeated default-parallel driver-lease runs and a full default-parallel
  `cargo test --locked` pass without relying on global `--test-threads=1`.

## Notes

Do not change production lease semantics merely to silence a test race. If the
reproducer instead proves a production race, externalize that implementation
work at the genuine seam before changing behavior.
