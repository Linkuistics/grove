# selection-cancellation-k28

## Goal

Cancel selection on a signal. A worker interrupted by INT, TERM or HUP is
stopped and reaped, nothing launches, and the caller sees the re-raised
signal. The whole-selection deadline keeps working beside the handlers.

## Context

The spec's `#execution-contract` and its resource table own the contract. The
worker joins the caller's existing job, and neither process creates a session
or detaches. Signal cancellation reaches ordinary descendants through the
enclosing process group. The deadline, its hard kill and exit 124 arrived with
`selection-deadline-k44` in static dispatch. `computed-selection-k21` and
`bounded-context-k22` extended its timeout cases to their callbacks. This leaf
adds the handled signals and must not weaken the deadline.

## Done when

- INT, TERM and HUP received while evaluating stop and reap the worker, launch
  nothing, and end by restoring the entry disposition and re-raising. A signal
  ignored at entry gets no handler and cannot cancel selection.
- The existing timeout cases still exit 124 with the handlers installed.
- Cancellation is checked after the result arrives, after descriptor close and
  reap, after choice validation and after executable resolution.
- Command-seam tests interrupt import, the loader and the callback. They
  assert that no fake-harness marker appears, no worker process survives, and
  the signal is re-raised. Positive control: the same fixtures without
  interruption do reach the harness.
- Structured `--json` output stays a single clean error on interruption.

## Decisions (running log)

**The handled window is evaluation, from just before the worker starts to
the end of selection.** `choose` installs the handlers after the inputs, the
policy entry, the state directory and the worker's location are settled, so
a signal while reading a prompt file still takes its entry course, and no
worker exists then. At the end of selection, after executable resolution, it
restores the entry dispositions *first* and only then checks for a received
signal. A signal before the restore is caught by that check; one after it
takes its entry course, and before `run`'s commit that launches nothing. So
k28 opens no window in which a signal is swallowed and a harness still runs.
`signal-transparent-handoff-k29` moves the restore past the commit, behind
its block-and-check linearization point and not-executed append.

**A received handled signal decides the outcome, whatever else the selection
came to.** A refusal, a result, a lookup's exit-4 refusal or an output
overflow reached while a signal was pending is reported as the cancellation,
keeping the policy's diagnostics. A timeout without a signal still exits 124.

**The capture threads never take a handled signal.** They are spawned with
INT, TERM and HUP blocked, so a process-directed signal can be taken only by
the main thread. A terminal Ctrl-C reaches the front and the worker together;
the main thread runs the handler before it can return from the read that
would see the worker's death, so a group interrupt is never misreported as a
worker failure. The worker itself is spawned with the entry mask: std's
`Command` inherits the calling thread's mask
(`library/std/src/sys/process/unix/unix.rs`, `do_exec`, Rust 1.98.1).

**Handlers use `SA_RESTART`, and the channel's 50 ms poll sees the flag.**
Restarted syscalls keep every other wait in the front (the store, the drains)
free of `EINTR`. The longest a cancellation waits is a run lookup's lock
wait, which is at most 2 seconds and never past the deadline.

**The worker is stopped as at the deadline: TERM, the one-second grace, then
KILL.** So `host.signal` aborts on cancellation too, and a policy's own TERM
listener runs.

**The cancellation is a refusal like the others, then the re-raise.** Code
`selection_cancelled`, stage `evaluation`, source the policy entry, a
`signal` field naming it, and `exit` the conventional `128 + N` a shell
reports for the death that follows. `--json` prints it as the one JSON error
on stderr, nothing on stdout; then the process re-raises the signal under its
restored entry disposition. A refused `run` still names its `inspect`
invocation.

**The shielding is also k29's precondition.** Today the looks after the reap
and at the end of selection would catch a signal whichever thread took it, so
no test can show the shielding necessary. It becomes necessary when
`signal-transparent-handoff-k29` blocks the handled signals in the main thread
and checks once more: a drain thread still alive past its grace, holding a
pipe a policy descendant kept open, would otherwise take a signal after that
check and swallow it before exec. With the drains shielded, blocking in the
main thread blocks the signals in the process.

**Every cancellation test was seen to fail against a mutated front**, and the
sources were restored byte for byte after each: a handler installed for a
signal ignored at entry fails the ignored-at-entry case; `raise` removed fails
the re-raise; no handlers fails all seven cancellation cases; no look after
the result (post-reap, after expansion, end of selection) records and launches
the harness in the post-result case; no precedence over a refusal reports
`record_store_locked`, exit 4, in the lock-wait case. The post-reap look alone
keeps the post-result case green, as designed: the looks are layered.

**The README's intro sentence about a refused input named no input.** It now
says what is refused, `--policy-env`, beside the signal handling still to
come.

**A signal blocked at entry is left blocked.** Evaluation never changes the
mask, so a handled signal the caller blocked stays pending and cannot cancel
selection, as an ignored one cannot. Unblocking it would deliver early what
the caller deferred; `signal-transparent-handoff-k29` hands the harness the
entry mask, and the pending signal with it. The worker is spawned with the
same entry mask.

**Installing the handlers can refuse, as `cancellation_unavailable`, exit 5,**
rather than panic past the one-JSON-error contract. `sigaction` fails only
for an invalid or uncatchable signal, so no test reaches it.
