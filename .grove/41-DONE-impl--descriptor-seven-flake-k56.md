# descriptor-seven-flake-k56

## Goal

Make `the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors` in
`crates/harness-dispatch/tests/run.rs` deterministic, so that a green
`task check` does not depend on scheduling.

## Context

`review-selector-k36` met it once, in a full `cargo test --locked -p
harness-dispatch` on a machine also running other groves' sessions. It failed
at `tests/run.rs:181`, the assertion that the file behind descriptor 7 holds
`written through descriptor 7\n`. The run's other assertions before it, exit 0
and the stdin echo on stdout, had passed. The panic's full message was not
captured. On rerun it passed 5 times alone and 6 times with its binary, and
once more in a full package run. k36 changed nothing on that path: it added an
example module to the worker and moved the handoff tests' stall helper into
`tests/support/stall.rs`.

The test hands the front a descriptor 7 through `dup2` in `pre_exec`, and the
fake harness (`FAKE_HARNESS` in `tests/support/mod.rs`) writes to it with
`echo … >&7` after `cat` has copied its stdin. Where the bytes can go missing
is the question: how `through` is opened and read, whether another test's
child can hold or replace descriptor 7, and whether the read races the write.
That is not yet known; this leaf starts by reproducing it.

The flakes this repository has fixed before each became their own leaf, and
their fixes conditioned on process state rather than time:
`driver-lease-readiness-flake-k145`, `cleanup-barrier-readiness-flake-k165`
and `noninteractive-stdin-flake-k55`.

k55 needed no wait: it put the caller's input in place before the process
that could race it existed. Its running log has the method, a swept delay
that found the window, and one finding that may bear here. An `unwrap` that
ran before the child's status was read reported any early death of that child
as a broken pipe, so the first failure seen was not the cause.

## Done when

- The failure is reproduced deliberately, its message captured, and the cause
  named.
- The test no longer depends on the losing interleaving, and still proves
  what it proves: the harness keeps the caller's stdin, stdout and descriptor
  7, and inherits none of the front's own descriptors.
- A control shows the descriptor assertion still fails when the front closes
  or replaces the caller's descriptor 7.
- `task check` passes.

## Decisions (running log)

**The cause is the number the caller's file was opened on, and no race with
the read.** The test opens `through` and hands it down with `dup2(fd, 7)` in
`pre_exec`. Rust opens every file close-on-exec. When `File::create` itself
returns 7, `dup2(7, 7)` does nothing and leaves that flag set, so the
descriptor closes when the front is executed. The fake harness's `echo … >&7`
then fails on its redirection alone, and `sh` goes on to `exit 0`. The file is
left empty, which is the assertion k36 saw fail.

Which number `File::create` returns is the lowest one free in the test
process at that instant, and the other tests' threads hold pipes. Alone it is
always 3, which is why every solo rerun passed.

Seen, with the test instrumented to log that number and nothing else changed:

- Forced to 7 by holding 3 to 6 open, alone: failed at the same assertion,
  `left: ""`, `right: "written through descriptor 7\n"`. The same binary
  unforced logged 3 and passed.
- The whole binary 100 times on an idle machine: 96 passed and 4 failed. The
  4 failures are the 4 runs that logged 7, each with that message. The passing
  runs logged 3, 4, 5, 6, 8, 9, 11 and 22.
- The unmodified binary had passed 40 of 40 just before, which a rate of 4 in
  100 allows about one time in five.

So the brief's three questions have these answers. The read does not race the
write: `wait_with_output` returns after the harness has exited. No other
test's child takes part. `through` is opened in a process whose other threads
decide its number.

**A second window exists, and is far rarer.** macOS has no `pipe2`, so Rust's
`std` makes a pipe with `pipe()` and marks it close-on-exec afterwards
(`library/std/src/sys/pipe/unix.rs` in the installed 1.98.1). A child started
by another thread between the two inherits that pipe. Every test here starts
children with piped output, and two tests assert on exactly which of
descriptors 3 to 9 the harness holds: this one, and
`records::a_run_commits_its_handoff_record_before_the_harness_starts`.

A scratch program measured it. With 15 threads each starting `/usr/bin/true`
with piped output as fast as they could, 7 of 7,326 children started by a
sixteenth thread held a descriptor from 3 to 9. With no other threads, 0 of
946 did. That is about 8,600 other spawns a second, against about 50 in a
test binary, so the rate in the suite is of the order of one run in 100,000.
It was not seen in the 100 runs above, and it would fail the last assertion
with an extra number, where k36's failure was the empty file.

**k55's finding bears here, on what a failing front reports.** The test wrote
the caller's input to the front's piped stdin after `spawn`. A healthy front
is not raced by that write: the harness's `cat` cannot end before the test
closes its end, and the test passed with the write held back 200 ms and 1 s.
A front that refuses at once is another matter. With its policy removed
(exit 3), the test reported `left: Some(3)` at the status assertion with no
delay, and `BrokenPipe` at the write from 5 ms on. So a front that failed for
any reason on a busy machine was reported as a broken pipe.

**The caller's descriptors are decided in the forked child, by one helper.**
`support::caller_leaves_open` takes the file and its number, or nothing. It
duplicates the file above the probed numbers before the fork, so its `dup2`
in `pre_exec` never names one descriptor twice. It then marks every other
probed descriptor close-on-exec. The forked child has no other thread, so
both hold whatever this process's threads were doing. There is one path and
no losing case, as in k55, and no wait.

The guard the shipped front uses, `fcntl` when the two numbers are equal
(`src/worker.rs`, `tests/support/direct.rs`), was set aside for this helper.
It leaves a branch that runs in about one run in 25, which only a forced
number could test, and forcing a number races the other threads again.

The second half does not hide a descriptor of the front's own. The front
opens its descriptors after it starts, so they take the numbers the sweep
freed, inside the probe's range.

**Three tests use it.** This one, with the file on 7. The records test above,
with nothing, since its `harness_fds` assertion had the second window too.
And `authority::the_worker_evaluates_policy_in_the_root_directory_with_null_stdin_and_a_fresh_environment`,
which had the same `dup2(inherited_fd, 7)`. That one could not fail from it.
It asserts that the worker does not hold the caller's descriptor 7, so a
descriptor that never reached the front passed it untested.
`a_caller_descriptor_above_the_soft_descriptor_limit_never_reaches_the_worker`
keeps its own `dup2` onto 100: a file would have to open on 100 to lose
there, and the helper takes only probed numbers.

**The caller's input is in place before the front exists**, as a file that is
the front's stdin, which is k55's repair. The status assertion now carries the
front's stderr.

**The control is a test in the suite.**
`a_stand_in_front_that_closes_replaces_or_adds_a_descriptor_or_reads_stdin_is_told_apart`
runs the same caller and the same fake harness through shell stand-ins for
the front. A faithful one reads exactly as the front does. One that closes 7
leaves the file empty and the harness with no descriptor. One that replaces 7
leaves the file empty and the harness still holding 7, so only the file tells
it apart. One that opens 5 gives `[5, 7]`. One that reads a line leaves
stdout empty.

**Each claim was seen both ways.** The test build carried three switches for
this: one put the old `dup2` back, one forced the file onto 7, and one
planted an inheritable descriptor in the test process, as a pipe half made
looks to a fork. They are removed, and the four test files' digests match
the ones taken before they went in.

- The file forced onto 7: the old code failed with the empty file, and the
  helper passed.
- A descriptor planted on 5: the old code failed with `[5, 7]`, and the
  helper passed. In the records test the old code failed with `[3]` against
  `[]`, and the helper passed.
- The whole binary 100 times: all 22 tests passed in every run. This test's
  file was on 7 in 5 of them.
- A front with its policy removed: the status assertion reported `Some(3)`
  with the front's own refusal text, 5 times of 5.

The shipped front was then mutated in `src/run.rs`, just before its `exec`.
Closing 7 failed this test with the empty file, and so did replacing 7 with
`/dev/null`. Opening a descriptor of its own failed it with `[3, 7]`, so the
helper's sweep hides nothing of the front's. With the front's sweep of
inherited descriptors removed in `src/worker.rs`, the old authority test
failed, except with its file forced onto 7, where it passed. With the helper
it failed both ways. Both sources are restored, and their digests match the
ones taken before.

**No review leaf is cut, and the in-session reviewer was not spent.** The
change is four test files and no shipped source. Each claim is held by an
executable control that was seen to fail, which `references/execute.md`
excuses.

`task check` passed all twelve checks on the committed bytes: the four test
files and the two shipped sources had the same digests after the run as
before it.
