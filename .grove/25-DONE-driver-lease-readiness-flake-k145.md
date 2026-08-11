# driver-lease-readiness-flake-k145

**Kind:** impl

## Goal

Make the driver process tests wait on observable child state without intermittent
readiness timeouts under the full parallel test suite.

## Context

- Surfaced while verifying `finish-task-root-identity-integrate-k144`; that diff
  does not touch driver lease, launch, provisioning, or the process-test support.
- `cargo test --locked` timed out both
  `a_second_driver_reprovisions_then_refuses_before_tree_access_or_launch` and
  `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` waiting
  for their first fake harness readiness file.
- The first test then timed out once in isolation, passed when one second of
  child-status instrumentation was added, and the unchanged 19-test
  `driver_lease` binary passed in full after that instrumentation was removed.
- The earlier `driver-lease-parallel-test-isolation-k135` change is relevant
  history; verify whether these two remaining process tests share the same
  fork/readiness seam before choosing a fix.

## Done when

- A failing stress or deterministic regression reproduces the readiness race
  without changing production timing.
- The two affected tests distinguish an exited first driver from a live but
  not-yet-ready child and do not rely on an arbitrary readiness deadline where
  a condition-based seam is available.
- The full `driver_lease` binary and `cargo test --locked` pass repeatedly.

## Notes

Keep finish-transaction code out of this leaf; the failure was non-causal and
belongs to process-test isolation.
