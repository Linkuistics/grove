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

## Decisions (running log)

**The delay is the first exec of a freshly written executable, and it queues.**
Measured with a scratch probe that reproduces the test's shape outside cargo: a
fresh copy of the front, a fresh fake worker script, `inspect --timeout-ms`.
The worker's spawn is taken as the front's exit less the bound, since TERM
ends `sleep` at once.

- Idle, one trial at a time: the fake's first line ran about 0.36 s after
  spawn, on every trial. Executing the same script once beforehand cut that to
  0.01–0.02 s. Lowering `ulimit -n` from 1048576 to 256 changed nothing, so the
  front's pre-exec descriptor loop is not the cost.
- 48 trials at once: at 3000 ms, 48 of 48 fake workers never ran their first
  line, and the front's wall time reached 11.3 s. At 1000 ms, also 48 of 48.
- The unchanged test binary run 32 times at once: 0 of 32 passed, each
  panicking at `worker.rs:158` reading the PID file (`NotFound`), which is the
  failure k24 saw.

The first-exec cost of a fresh file grows with how many are being started at
once, so no bound comfortably exceeds it. The mechanism behind it was not
identified; only its effect was measured.

**Survival is judged by the front's process group, not by a PID the worker
records.** The test starts the front as the leader of a new process group. The
front forks its worker into its own group and never moves it (it signals only
the worker's PID for that reason). So the worker is a member from the fork,
before any line of its own runs. After the front is reaped, `kill(-group, 0)`
failing with `ESRCH` shows nothing the front started outlived it. The fake
worker no longer writes a PID file.

**The bound returns to 1000 ms.** No part of the evidence now depends on the
worker running within the bound, so the three-second margin only made the test
slower.

**Controls seen to fire.** Each was a mutation of `src/worker.rs`, then
restored and compared with the saved source. The digests of `src/worker.rs`,
`tests/worker.rs` and `tests/deadline.rs` matched those taken before the first
mutation.

- No deadline (the deadline an hour away): this test failed with exit 5,
  `worker_failed`, once the fake's 30-second sleep ended. The five timeout
  tests in `deadline.rs` failed on their watchdog, as in k44.
- No KILL after the grace (k44's): the TERM-ignoring and post-result tests
  failed, as in k44. This test passes under it, because its fake dies on TERM.
  It did not detect this mutation in k44 either.
- No stop at all on expiry (neither signal nor reap): this test failed with
  "the worker survived the front". Under 32 concurrent runs it failed 32 of 32,
  so the group check sees a survivor whether or not the fake ran a line. The
  old PID check could not see that case. The mutation was run on `tests/worker.rs`
  alone, since `deadline.rs`'s spinning workers would never exit by themselves.

The restore copied the saved file with its old modification time, so cargo
kept the last mutated front. The final restore was followed by a `touch` and a
rebuild, which compiled, and the test then passed, which the no-stop front
cannot do. Each mutation's own run was a fresh build, because applying it wrote
a new time.

**The changed test under load: 32 of 32 and then 64 of 64 concurrent runs
passed**, where the unchanged test passed 0 of 32. Alone it takes about 1.4 s
instead of about 3.4 s.

**`deadline.rs` is not exposed the same way, and is unchanged.** Its timeout
tests run the built front and the compiled worker, which earlier test binaries
have already executed, so neither pays a first exec there. Their PID evidence
depends on the policy's first line by design: it is how they show evaluation
began.

**Checks.** `task check` passed all 12 principal checks, with 1536 tests and
none failing, in 6 min 13 s. The digests of `src/worker.rs`, `tests/worker.rs`,
`tests/support/mod.rs` and `tests/deadline.rs` were the same before and after
the run. No in-session reviewer: the node's scheduled `review-impl` covers this
producer.
