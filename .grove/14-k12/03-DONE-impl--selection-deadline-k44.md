# selection-deadline-k44

## Goal

Bound policy evaluation from its first import. When the whole-selection
deadline expires, `harness-dispatch` stops the worker, reaps it, launches
nothing and exits 124. Trusted TypeScript that hangs can no longer hold an
unattended caller forever.

## Context

The contract is the whole-selection row of the spec's `#bounded-context`
resource table and the deadline paragraph of `#execution-contract`. Static
`routes` policy already executes arbitrary TypeScript at import. So the bound
belongs to the first increment that evaluates policy, not to the evaluation
boundary (review `harness-selection-and-execution-k42`, finding F2). A
worker-side timer cannot end a synchronous spin. Nor can the worker, on its
own, see that a promise kept pending by live asynchronous work is stuck. Only
the front process's wall-clock deadline ends both.

Handled INT, TERM and HUP, and the post-result cancellation checks, stay with
`selection-cancellation-k28`. `computed-selection-k21` and `bounded-context-k22`
add the same hang shapes for `select` and `loadContext` once those callbacks
exist. The record-store lock wait arrives in the next leaf. It neither extends
nor consumes this bound.

## Done when

- `--timeout-ms` accepts 1 to 120 seconds, and the default is 30 seconds. The
  bound counts from worker start to its result and covers module import. An
  out-of-range value is malformed CLI input and exits 2.
- At expiry the front stops the worker with a cleanup grace of at most one
  second, then KILL, and reaps it before returning. It exits 124 with a
  structured diagnostic naming the bound, in text and in `--json`, and launches
  nothing. The deadline does not rely on the worker's cooperation.
- Command-seam tests run `inspect` and `run` with a short explicit bound. The
  policy spins synchronously at import in one case. In another, it awaits a
  promise that a live timer keeps pending. Each asserts exit 124, the
  diagnostic, no surviving worker process and no fake-harness marker.
- Positive control: variants of both fixtures whose spin or pending promise
  ends within the bound do reach the fake harness. No timeout case can then
  pass by never evaluating.
- Inspection reports the effective selection bound. The usage documentation
  states the bound, its ceiling and exit 124. The spec's notice states what is
  delivered.

## Decisions (running log)

**The front measures the deadline on its own channel reads; the worker is not
changed.** Every read and write of the protocol channel carries a socket
timeout recomputed from the time left before `started + bound`, where
`started` is taken just before spawning the worker. The hello therefore counts
too: a worker that never identifies itself is stopped at the same deadline as
a policy that never finishes loading. Verified in the installed std source
(Rust 1.98.1, `library/std/src/sys/net/connection/socket/unix.rs:387`,
`set_timeout`): a zero duration is refused and a sub-microsecond one rounds up
to 1 µs, so the remaining time is checked for zero before it is set.

**At expiry the worker gets TERM, at most one second, then KILL, and is reaped
before the front returns.** Only the worker's own PID is signalled: it shares
the caller's process group, so a group signal would reach the front and its
caller. Waiting polls `try_wait`, which never reaps behind the front's back, so
the PID cannot be reused before KILL is sent.

**The reap after a result is bounded too.** A probe on the current build had a
policy install `process.on("exit", () => { while (true) {} })`. The result
arrived, and the front then blocked in `child.wait()` until an outer
`timeout 8` killed its group. The leaf's goal is that hanging TypeScript cannot
hold a caller forever, so after its result the worker has the same one-second
grace to exit on its own before KILL. The selection itself stands, since its
result arrived within the bound.

**A timeout is `selection_timeout` at stage `evaluation`, exit 124.** The
existing stages name a part of the evaluation (`load`, `validation`,
`selection`), but the front cannot see which part was running when the bound
ran out, and once `select` and `loadContext` exist a timeout could fall in any
of them. So the new `evaluation` stage names the whole of it. The message says
whether the worker had been handed the entry: after the hand-over the policy
held evaluation and `source` is the entry; before it, the worker never became
ready and `source` is the worker.

**A refusal can name the bound it exhausted.** The error object gains
`"bound": {"name": "selection", "ms": 1000, "from": "--timeout-ms"}`, and text
mode a `bound:` line. `from` is the flag or `default`, and `input` names the
flag only when the caller set it. Inspection reports the same value under
`"bounds": {"selection": {"ms": …, "from": …}}` and a `bounds` row. The shape
is keyed by unit so `bounded-context-k22` can add `context` in bytes beside it.

**`--timeout-ms` is plain digits, 1000 to 120000, or malformed.** It is parsed
in `inputs.rs`, like `--task-id`, so its refusal carries its own remedy rather
than clap's generic one. A sign, an exponent, a space or an out-of-range value
is not read generously or clamped. It is a visible input now, and help lists
the default and the range.

**The two diagnostic drains share one grace.** Each drain used to wait up to a
second in turn after the reap. They now wait for one common instant, so a
descendant holding both streams costs one second, not two.

**Controls seen to fire.** Each was a mutation run, then reverted (`cmp`
against the saved source):

- No deadline at all: the five timeout tests failed on the watchdog, and the
  never-identifies worker test got exit 5 when its sleep ended.
- No KILL after the grace: the TERM-ignoring and post-result tests failed.
- An unbounded reap after the result: only the post-result test failed.
- KILL at once instead of TERM: only the cleanup-grace test failed.
- Accepting a sign in `--timeout-ms`: the range test failed on `+1000`.

No worker process survived any of these runs.

**If `try_wait` fails, the kill decides.** An error from `try_wait` breaks the
grace loop and falls through to KILL and a blocking wait, rather than returning
early and possibly leaving a worker running.

**No in-session reviewer.** The node's scheduled `review-impl` names the
deadline's hard kill and reaping among its doubts, so this producer's review is
already scheduled.

**The timeout tests use a three-second bound, because the bound counts from
spawn.** The first full `task check` failed one test. A fake worker that never
says hello was stopped at its one-second bound, and it exited 124 as expected.
But under the full workspace suite, on a machine running another project's
check, it had not yet run its first line, so its PID file did not exist. The
deadline was right, since it counts from spawn. The test's evidence was racy.
Every test whose evidence depends on the worker reaching the policy within the
bound now uses 3000 ms (`BOUND_MS` in `tests/deadline.rs`), and the positive
control holds for 3.5 s under a 15-second bound. Tests that must *not* time out
use 15 or 20 seconds. The boundary value 1000 is checked as the effective bound
through either the report or the refusal, whichever a loaded machine produces.
Mutations 1 to 5 were rerun against the revised tests, with the same detections.
The pre-hello test also lost its elapsed-time cap. It runs a fresh copy of the
front, whose first exec once took 5.7 s under parallel load. Exit 124 is its
evidence, because without the deadline it exits 5 when its 30-second sleep
ends.

**Done-when instruments.** The command-seam tests are in
`crates/harness-dispatch/tests/deadline.rs` unless another file is named.

- The flag, its range, its default and exit 2:
  `the_bound_is_one_to_one_hundred_and_twenty_seconds_in_milliseconds` (ten
  malformed spellings, each refused before a sentinel policy runs, beside the
  accepted boundary values) and `inspection_reports_the_effective_selection_bound`.
  That the bound counts from the worker's start and covers import:
  `a_worker_that_never_identifies_itself_is_stopped_at_the_deadline`
  (`worker.rs`) and the spin-at-import case.
- Expiry, the grace, KILL and reaping:
  `a_policy_that_spins_at_import_is_stopped_at_the_deadline` and
  `a_policy_awaiting_a_promise_that_live_work_keeps_pending_is_stopped_at_the_deadline`,
  each through `inspect` and `run`. Both assert exit 124, the structured
  diagnostic with its `bound`, a recorded worker PID that no longer exists when
  the front exits, and no fake-harness marker.
  `a_worker_that_ignores_term_is_killed_after_at_most_a_second_of_grace` shows
  KILL, and `the_worker_is_offered_its_cleanup_grace_before_kill` shows TERM
  comes first. Text form: `a_timeout_in_text_mode_names_the_bound_and_keeps_the_policy_output`.
  The bounded reap after a result: `a_worker_that_will_not_exit_after_its_result_is_killed_after_the_grace`.
- Positive control: `a_hold_that_ends_within_the_bound_reaches_the_harness`,
  both holds ended after longer than the timeout cases' bound.
- Documentation: the README's *The selection bound* section, its inputs and
  refusal tables and its inspection sample; the spec's notice and its reap
  clause under *Execution and authority*.
- Checks: `task check` passed all 12 principal checks, with 1516 tests and none
  failing. Every measured source predates the run's start. The run's own digest
  wrapper was broken: zsh did not split its file list, so both digests were
  empty. File modification times were used instead, checked against a file
  touched after the run.
