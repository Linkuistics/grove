# noninteractive-stdin-flake-k55

## Goal

Make `standalone_child_has_a_new_session_and_cannot_consume_callers_stdin` in
`crates/keyed-launch/tests/noninteractive.rs` deterministic, so that a green
`task check` does not depend on scheduling.

## Context

`dispatched-terminal-k33` met it once, in a full `task check` on a machine
also running other groves' sessions. It passed 15 times in a row on rerun,
alone and with its binary. It failed at line 327, the parent's write of
`parent input` to the supervisor's piped stdin:

    called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }

The write happens only after `spawn` returns. The supervisor runs its child
with null stdin and never reads its own, so if the parent is descheduled long
enough, the supervisor, and every holder of the pipe's read end, can exit
first. That is a hypothesis from reading the test, not yet a reproduction.

The flakes this repository has fixed before each became their own leaf, and
their fixes conditioned on process liveness rather than time:
`driver-lease-readiness-flake-k145` and `cleanup-barrier-readiness-flake-k165`.

## Done when

- The race is reproduced deliberately, for example by delaying the parent's
  write, and the hypothesis above is confirmed or replaced.
- The test no longer depends on the write winning. Whatever it proves about
  the caller's stdin, it still proves: the child consumes none of it.
- A control shows the stdin assertion still fails when the child can read the
  caller's input.
- `task check` passes.

## Decisions (running log)

**The hypothesis is confirmed, and widened.** A sleep between `spawn` and the
write, swept on an idle machine: the write succeeded at every delay up to
600 ms and failed with the reported `BrokenPipe` at 610 ms and beyond. The
supervisor lived 606 to 637 ms in every passing run. That is one
`POLL_INTERVAL` (500 ms, `src/run.rs`) and the start-up of two test binaries,
since `watch` sleeps one interval before it first sees the child gone. So the
mechanism is as the leaf supposed: the supervisor holds the pipe's only read
end, and a write after its exit gets `EPIPE`.

The same error has a second cause, which the leaf did not name. With the
supervisor's `TMPDIR` pointed at a missing directory it panicked in 5 ms. At
no delay the test reported that panic, through its status assertion. From a
5 ms delay on it reported `BrokenPipe` at the write and nothing else. The
write's `unwrap` runs before the supervisor's status and output are read, so
any early supervisor death is reported as a broken pipe.

k33's failure cannot be attributed to one cause or the other, because the
record that would tell them apart is what the write discarded. A healthy
supervisor needs the parent held for 600 ms; a failed one needs 5 ms. The
repair has to remove both, and it does not need to know which it was.

**The caller's input is in place before the supervisor exists.** It is a file
the test writes and opens, and the supervisor's stdin is a duplicate of that
descriptor. Nothing is written after the spawn, so nothing races the
supervisor's exit, and a supervisor that fails is reported by its own status
and output. This orders the two by construction, which is stronger than the
liveness waits of k145 and k165: there is no wait.

Tolerating `EPIPE` on the old write was set aside. In a run where the write
lost, the caller's stdin would have held nothing, and "the child read none of
it" would pass without having been tested. A pipe filled before the spawn was
set aside too: `std::io::pipe` is stable from Rust 1.87 and the workspace's
`rust-version` is 1.85, so it would need `libc::pipe` and hand-set
close-on-exec flags to say what a file says in two lines.

**The test observes the caller's side as well as the child's report.** A
duplicated descriptor shares its offset, so the test's own handle reads how
far anything downstream got through the input. The child's report says what
it read; the offset says nothing consumed the caller's input by any route
through that descriptor.

**The control is a test in the suite, and each assertion was also seen to
fail against a broken launcher.**
`a_plainly_launched_child_consumes_callers_stdin_and_shares_its_session_and_descriptors`
runs the same `child` fixture under a new `plain_supervisor` fixture, which
launches it with nothing of `run_noninteractive` between them. All four
observations come back the other way there: the child reads the input, the
offset reaches its end, the child leads no session and it holds descriptor
197. So the control covers the session and descriptor assertions too, which
had none.

Then the launcher itself was mutated, by removing `stdin(Stdio::null())` from
`run_with_output`. The test failed on the child's report, which held
`parent input`. With the two stdin assertions swapped it failed on the offset,
12 against 0. `src/run.rs` was restored and its digest checked against the one
taken before.

With the supervisor's `TMPDIR` missing, the repaired test failed at its status
assertion with the supervisor's own output, in 5 runs of 5, and never as a
broken pipe. Under 48 busy loops on 16 cores the test and its control passed
40 times of 40. An earlier run of that loop read 0 of 40 and was void: `ls -t`
is aliased on this machine, so the binary's path was empty and nothing ran.

`crates/keyed-launch/tests/` is evidence and not corpus for the keyed-launch
walkthrough book (`walkthrough.toml`), so no book page changes with it.

**No review leaf is cut, and the in-session reviewer was not spent.** The
change is one test file and no shipped source. Every claim it makes is held
by an executable control that was seen to fail, which is the case
`references/execute.md` excuses, and a test's repair is not the load-bearing
artifact a review chain is earned by.

`task check` passed all twelve checks on the committed bytes: the test file
and `src/run.rs` had the same digests after the run as before it.
