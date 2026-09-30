# deadline-test-load-race-k45

## Goal

Make `a_worker_that_never_identifies_itself_is_stopped_at_the_deadline`
(`crates/harness-dispatch/tests/worker.rs`) reliable under a loaded machine. It
must keep proving that a worker which never says hello is stopped at the
deadline, and does not survive the front.

## Context

`selection-deadline-k44` found this test's evidence racy. The deadline counts
from spawn. A fake worker that has not yet run its first line when the bound
expires has written no PID file, so the test cannot check survival, and it
panics reading the file. k44 raised the bound to 3000 ms and recorded why.

The race recurred. During `handoff-records-k24`'s `task check`, with a
virtual machine running on the host, the test panicked at `worker.rs:158`
reading the PID file (`NotFound`). The binary took 5.4 s, against about 3.4 s
when run alone. The front's exit 124 and its diagnostic had already passed, and
the next line is the PID read. Three isolated reruns of `--test worker` passed,
and so did a full `cargo test --workspace` rerun (1535 tests). Nothing in k24
changes the worker's spawn: the store is opened only after the worker is
reaped. The failure is load-dependent evidence, not a deadline defect.

## Done when

- The test's survival evidence does not depend on the fake worker reaching its
  first line within the bound. Two ways to get there are a bound that
  comfortably exceeds a loaded first exec, or evidence the front's reap
  guarantees whether or not the worker ran. Either way the test still fails
  when the deadline is removed. Rerun that mutation, and the no-KILL one from
  k44's running log, and see each fail.
- The change is recorded in this leaf's running log with the timing evidence.
  `task check` passes.

## Notes

This is a test-evidence repair inside `static-dispatch-k12`. It sits ahead of
`choice-and-refusals-k15`, so that k15 stays the node's last leaf and cuts the
node's review.
