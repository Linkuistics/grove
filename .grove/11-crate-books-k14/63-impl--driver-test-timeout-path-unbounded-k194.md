# driver-test-timeout-path-unbounded-k194

## Goal

Make the driver tests' own timeout paths terminate. A fixture that misses its
deadline must fail with its diagnostic, not hang forever collecting it.

## Context

- **The observed hang, diagnosed at `canonicalisation-sites-k149`.**
  `bash scripts/check.sh` stopped for thirteen minutes inside
  `cargo test --locked --workspace` with no output and no CPU. `sample` on the
  test binary named the thread:
  `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`, in
  `crates/grove/tests/loop_driver.rs`, blocked at line 1009 in `read()` under
  `std::process::Child::wait_with_output`.
- **The mechanism, and it is not timing.** The fixture waits up to 25 s for
  `epoch_held` to appear (`let setup_deadline = Instant::now() +
  Duration::from_secs(25)`). On a missed deadline it takes the failure path:
  `child.kill().unwrap()` and then `child.wait_with_output().unwrap()`, to put
  the driver's stderr into the panic message. `wait_with_output` reads the pipe
  to **EOF**, and EOF requires every writer to close. The driver had already
  launched a session — `/bin/sh <tmpdir>/configured-command.sh …` — which
  inherited the piped stderr, was reparented to launchd when its parent died,
  and holds the write end open indefinitely. `lsof` confirmed it: pid 17690,
  PPID 1, holding the same pipe the test was reading, while both of the test's
  direct children were already zombies. **Killing the child is not enough when a
  grandchild holds the pipe.**
- **Load is the trigger, not the fault.** The same suite had passed in full
  minutes earlier, on the same bytes. The re-run coincided with three other
  grove sessions on the machine and a load average of 7.79, which is what pushed
  the fixture past 25 s. A timeout that becomes an infinite hang under load is a
  worse instrument than no timeout, because it cannot be told from slowness.
- **Scope is unknown and must be enumerated, not assumed.** The one hang seen was
  in `crates/grove/tests/loop_driver.rs`, but `wait_with_output` after a `kill`
  is a pattern, and the driver tests spawn sessions in several places.
  `crates/grove-loop/tests/driver_lease.rs` is the other obvious candidate.
- **The machine is carrying dozens of orphans of the same shape, and that is the
  more alarming half.** `ps -eo pid,ppid,args | awk '$2==1'` at the time of the
  diagnosis found **24** `/bin/sh <tmpdir>/configured-command.sh` processes
  reparented to launchd, with elapsed times from 18 hours to **8 days**, plus two
  `sh -c while : ; do sleep 0.05 ; done` fixtures. Count them again at the time of
  the fix rather than trusting this number. Two cautions before treating all of
  them as debris: some may belong to grove sessions that are genuinely still
  running, and telling those apart from leaked ones is part of this leaf's work;
  and the accumulation is self-reinforcing, because idle orphans are load, and it
  was load that pushed the fixture past its 25 s deadline in the first place.
  Whatever fix lands should stop leaking them, and should say how an operator
  clears the ones already there.
- **Prior art in this tree.** `driver-lease-fixture-timing-k85` recorded this
  family as *timing flakes*. That framing is what let it survive: this instance
  is not a flaky assertion but an unbounded wait in the reporting path, and
  naming it that way is part of the fix.

## Done when

- Every driver-test failure path that collects a killed child's output is bounded
  — by dropping the pipe handles before waiting, by reading with a deadline, by
  killing the whole process group rather than the direct child, or by not
  reading the pipe at all on the timeout branch. Which of those is this leaf's to
  choose and to record.
- The call sites are **enumerated at the time of the fix** rather than taken from
  this file: every `wait_with_output`, `wait()` and pipe read in
  `crates/grove/tests/` and `crates/grove-loop/tests/` that can run after a
  `kill`, classified by whether a surviving grandchild could hold the pipe.
- The fixture is shown to fail — with its diagnostic, promptly — when its
  deadline is missed. A timeout path that has never been seen to complete is not
  evidence it terminates; force the miss (shorten the deadline, or hold the
  fixture's precondition) and watch it report.
- No `configured-command.sh` or fake-session process survives a completed run.
- `bash scripts/check.sh` passes.

## Notes

**This leaf changes tests, not the frozen corpus.** `crates/grove/tests/` and
`crates/grove-loop/tests/` are evidence rather than book roots (root brief,
*The corpus, exactly*), so no fragment, ledger row or line count moves and the
one-commit rule costs nothing here. Check that still holds at the time of the fix
rather than assuming it.

**Do not weaken the assertion to make the hang go away.** The test is pinning
that an orphaned epoch guard stops before consuming the relaunch signal, which is
a real property of the loop driver. Raising the 25 s deadline, marking the test
`#[ignore]`, or deleting it are all ways to make a green run that proves less
than the red one did.

## Decisions (running log)
