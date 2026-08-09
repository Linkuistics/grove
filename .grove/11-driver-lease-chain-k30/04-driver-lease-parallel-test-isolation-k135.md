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

## Outcome

The affected unit tests dropped a driver lease and immediately asserted that a
probe or replacement could acquire it. In the full library test process, an
unrelated parallel test could fork a Git or jj subprocess while that lease was
live. BSD `flock` survives fork until exec, so the sibling child briefly retained
the close-on-exec descriptor after the owning test dropped it. The driver-lease
module alone did not spawn those siblings and remained green.

One test-only subprocess seam now runs all four immediate-unlock assertions with
no parallel siblings. Their production lock, probe, and epoch-handoff event-order
assertions are unchanged inside the isolated process; production lease code is
unchanged.

## Verification

- RED: repeated unrestricted `cargo test --locked --lib --quiet` reproduced
  `a_successful_liveness_probe_releases_the_lease_before_validation` returning
  `Ok(())` on run 4; twenty driver-lease-only runs passed before the fix.
- GREEN: twenty repeated unrestricted default-parallel library runs passed.
- GREEN: fifty repeated default-parallel driver-lease runs passed.
- FINAL: ten repeated unrestricted default-parallel library runs passed after
  simplifying the isolation seam to its final additive form.
- `cargo test --locked` passed, including 490 library tests and every integration
  test binary.
- `cargo fmt --all --check` passed.
- `cargo clippy --all-targets --locked` exited 0 with the pre-existing warning
  set unchanged by this test-only edit.
