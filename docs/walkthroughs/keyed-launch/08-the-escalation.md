# The watch and the escalation
<!-- book-page id="the-escalation" slice="the-launchers-job" order="8" -->
[Previous: The child is a job](07-the-job.md) | [Contents](README.md) | [Next: How this is checked](09-how-checked.md)

<a id="the-launchers-job"></a>
## The launcher's job

Chapter 7 ended with a process: a program a template authored, running in a
process group of its own, holding the terminal, with one path in its
environment. Nothing had been decided about it. This chapter is where the
launcher waits, and then acts.

What this stage must not add and must not interpret is **the ending**. The crate
cannot know that the child is done; it can know only that the child *said* so.
Everything on this page follows from that one restriction, and so does the
crate's largest admission. Supervision polls three things, and they are the only
three ways a launch ends: the child exits, the token's file appears, or the
launcher itself is signalled. A child that finishes its work and never signals
reaches none of them. An interactive one returns to its prompt rather than
exiting, so the launch does not end — it **stalls**. That is a real failure mode
with no cheap fix here, because nothing this crate can observe distinguishes a
child that forgot to signal from one still working, and a second completion
observable would only trade a stall for a wrong kill. It is the caller's to
close, at the layer that instructs the child, and chapter 10 closes the book on
it.

Chapter 7 read the reason an escalation exists at all: an interactive child is
never reaped on its own, so ending it is somebody's job, and the only process
that can do it is the child's own parent — outside whatever sandbox the child
runs under, where a child asked to end itself may simply be denied, and denied
silently. That argument settled *why*, and it also fixes two things on this page
that look like details and are not. Ending an interactive child means ending the
whole **job** it started rather than the one process the launcher can see, which
is why the escalation is addressed to `-pgid` as well as to the pid. And a signal
sent from outside to a process that may already be gone can fail without that
failure meaning anything, which is why `kill`'s return value is discarded on
purpose.

`src/run.rs` splits between chapter 7 and this one by whose signal it is, and
this chapter owns the two blocks chapter 7 left: lines 124–243, which sit between
that chapter's two, and lines 449–607, which close the file — 279 lines. The
first block is the supervisor's state type and the launcher's own signal
machinery; the second is the supervisor itself. After this page every byte of
`src/run.rs` is accounted for.

<a id="the-two-graces"></a>
## The two graces

This section takes the second half of step 4 of the five-call trace chapter 1
wrote — the half chapter 7 stopped at — and runs it to a value. It starts where
chapter 7 finished, with the child running and its token not yet written, and it
ends with an `Ended`. It runs twice, because two different things can end the
same launch, and the second is the one the completion channel cannot express.

The launch is the one chapter 7 spawned: `claude --model opus <the mandate>`,
pid 4137 and pgid 4137, with `GROVE_SIGNAL_FILE` set to
`/work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281`, under grove's
own escalation of two seconds and five. The supervisor polls every 500ms.

The first run is the ordinary one for an interactive child. It does its work,
writes `relaunch\n` to the path, and returns to its prompt, where it sits waiting
for input that is not coming. The trace below is the whole escalation, and the
column on the right is the value of the private `Watch` the supervisor carries.

```text
 t=0.00   spawn returns, the latch having been cleared just before it
          poll: try_wait → None; signal file absent                 Running
 t=0.50   poll: try_wait → None; signal file absent                 Running
 t=1.00   poll: try_wait → None; signal file absent                 Running
 t=1.30   the child writes "relaunch\n" and returns to its prompt
          — nothing observes this yet —
 t=1.50   poll: try_wait → None; signal file EXISTS                 Signalled(1.50)
 t=2.00   poll: 0.50s of the 2s grace elapsed                       Signalled(1.50)
 t=2.50   poll: 1.00s elapsed                                       Signalled(1.50)
 t=3.00   poll: 1.50s elapsed                                       Signalled(1.50)
 t=3.50   poll: 2.00s elapsed ≥ grace
          kill(-4137, SIGTERM) then kill(4137, SIGTERM)             Terminated(3.50)
 t=4.00   poll: try_wait → Some(status)
          → Ended { end: Signalled, status: signal 15,
                    elapsed: 4.00s, token: Some("relaunch") }
```

Two things in that column are worth reading before the source explains them. The
token was written at `t=1.30` and observed at `t=1.50`: the grace runs from the
**observation**, not from the appearance, so the child's real reprieve is the
grace plus up to one poll interval. That is exactly what chapter 7's
`POLL_INTERVAL` meant by *bounding how late an escalation starts*, and it is why
the crate's own tests, which do assert that a grace elapsed, use the *signal* to
say which step of the escalation actually ran. And `SIGTERM` at `t=3.50` did not end the launch: the child was
reaped by the next tick's `try_wait`, on the ordinary path, because a child that
has been asked to die may still decline.

A child that does decline takes the same trace to its second step. With
`trap '' TERM` installed it survives `t=3.50` untouched, the five-second
`kill_grace` runs from there, and at `t=8.50` the supervisor sends
`kill(-4137, SIGKILL)`, reaps the child itself, and returns
`Ended { end: Signalled, status: signal 9, elapsed: 8.50s, token: Some("relaunch") }`.
That is the whole escalation: grace, SIGTERM, kill grace, SIGKILL.

The second run is the ending the channel has no way to express. Nothing about the
child changes; the *launcher* is sent SIGTERM at `t=1.10`, before the child has
written anything.

```text
 t=1.10   the launcher (pid 4100) is sent SIGTERM
          on_terminate stores 15 into INTERRUPTED_BY and returns
 t=1.50   poll: try_wait → None
          take_interrupt() → Some(15); interrupted = Some(15)
          kill(-4137, 15) then kill(4137, 15)                       Terminated(1.50)
 t=2.00   poll: try_wait → Some(status)
          → Ended { end: Interrupted { signal: 15 },
                    status: signal 15, elapsed: 2.00s, token: None }
```

The child was signalled and reaped rather than orphaned onto the terminal, and
the value that comes back names the number the *launcher* was sent. `token` is
`None` because nothing wrote the file, which is the ordinary shape of an
interrupt. Three fields differ between the two runs, and only one of them is the
distinction: `end` says who acted — `Signalled` is the escalation, `Interrupted`
is the launcher's own death arriving mid-launch — while `elapsed` and `token`
differ only because the second launch was cut short before its child spoke. `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace`
and `a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` in
`crates/keyed-launch/tests/launch.rs` hold the first run's two steps;
`an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` in
`crates/keyed-launch/tests/interrupt.rs` holds the second.

<a id="what-the-blocks-answer"></a>
## What the two blocks answer

This chapter's 279 lines are two blocks with chapter 7's spawn between them: one
private enum, one static and four functions at the top of the file; then the
supervisor, the poll loop and the signalling helper that close it. The table
collects what each answers and what pins it. Tests named without a path are in
`crates/keyed-launch/tests/launch.rs`; the other two name their own files.

| Item | Answers | Pinned by |
|---|---|---|
| `Watch` | where this launch is in its ending | — |
| `INTERRUPTED_BY` | which signal the launcher was sent, during which launch | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| `take_interrupt` | which signal arrived when no launch was running | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| `reraise` | how a launcher dies of the signal it was sent | `a_reraised_signal_reaches_the_parent_as_a_wait_status` (`tests/reraise.rs`) |
| `on_terminate` | what a signal handler may safely do | — |
| `install_termination_handler` | which signals the launcher catches, and which it deliberately does not | — |
| `supervise` | who owns the terminal when the launch returns | — |
| `watch` | which of the three observables happened first | `a_child_that_never_signals_ends_with_no_token`, `an_unsignalled_child_runs_to_its_own_exit_untouched`, `a_child_that_signals_and_exits_inside_the_grace_is_never_touched`, `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace`, `a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` |
| `kill` | what the escalation reaches | `the_escalation_reaps_the_childs_descendants` |

Four rows have no test of their own, and the reasons differ. `Watch` is private
and is exercised by every test in the table's `watch` row. `on_terminate` is
private too, but none of those five signals the *launcher*, so the only thing
that runs it is `tests/interrupt.rs`, which the two rows above it already name.
Asserting on either directly would need an interface the crate does not have and
does not want.

The other two rows are the real gaps. `supervise`'s is the terminal gap chapter 7
stated: nothing under `crates/keyed-launch/tests/` names a controlling terminal
or a foreground process group, so the reclaim runs in whichever of its two forms
the test runner happens to produce and nothing asserts on either.
`install_termination_handler`'s is narrower than it looks. That the handler is
installed is not asserted, but it is *relied on*: `tests/interrupt.rs` opens by
saying that `raise(SIGTERM)` is safe only after a first `run`, and its phase 1 is
there to install the handler rather than for the one assertion it also makes. A
run in which `run` did not install it would kill the test binary outright. What
no test reaches is the argument for SIGINT's absence, which is a claim about a
signal the handler never sees.

The first composite is the block between chapter 7's two: the supervisor's state
type, the interrupt latch, and the four functions that stand behind it.

<!-- fragment «watch-and-launcher-signals» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="124-243" parent="source-run" -->
<!-- insert «run-watch-states» -->
<!-- insert «run-interrupted-by» -->
<!-- insert «run-take-interrupt» -->
<!-- insert «run-reraise» -->
<!-- insert «run-on-terminate» -->
<!-- insert «run-install-termination-handler» -->
<!-- /fragment -->

The second is the end of the file, and it is the supervisor: the terminal
reclaim, the poll loop that decides which of the three observables happened, and
the two-call helper that signals a job.

<!-- fragment «supervise-and-escalate» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="449-607" parent="source-run" -->
<!-- insert «run-supervise» -->
<!-- insert «run-watch-signature» -->
<!-- insert «run-watch-ended» -->
<!-- insert «run-watch-terminal-recheck» -->
<!-- insert «run-watch-try-wait» -->
<!-- insert «run-watch-forward-interrupt» -->
<!-- insert «run-watch-escalation» -->
<!-- insert «run-kill» -->
<!-- /fragment -->

<a id="three-states"></a>
## Three states, two of which are clocks

The type the supervisor carries is private, has no derives, and never leaves the
file. Its two lines of comment say what it tracks.

<!-- fragment «run-watch-states» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="124-130" parent="watch-and-launcher-signals" -->
````rust
/// The supervisor's state machine: idle until the token appears, then timed
/// toward SIGTERM and finally SIGKILL.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
}
````
<!-- /fragment -->

`Watch` is easy to mistake for a description of the child, and it is not one: it
is a description of *the ending*. `Running` is the state in which no ending has
begun, whatever the child is doing; `Signalled` means the token has been seen and
a clock is now running toward SIGTERM; `Terminated` means a termination signal
has gone and a second clock is running toward SIGKILL. The two clocks are why those variants
carry an `Instant` and `Running` carries nothing — there is no deadline until
something has started one.

Read it against `End`, which chapter 7 defined, and the pair is the crate's whole
answer to *what is this launch doing*. `End` is public, has three cases, reports
**who acted** and is produced once, at the end. `Watch` is private, has three
cases, records **where the escalation is** and changes several times per launch.
The names overlap by one word and the types share nothing, which is deliberate:
`Watch::Signalled` says the token was seen, and `End::Signalled` says the
escalation ran. One line in the state machine below turns the first into the
second, and its *placement* — not its content — is the whole of the distinction.

<a id="the-latch"></a>
## A latch that outlives its launch

The launcher's own signals need somewhere to be written down, and there is
exactly one place. Its comment carries four arguments, and every one of them is
a decision this file makes about scope.

<!-- fragment «run-interrupted-by» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="131-149" parent="watch-and-launcher-signals" -->
````rust

/// The signal [`on_terminate`] last received, or `0`, read by [`run`]'s poll
/// loop.
///
/// Process-global because a signal disposition is process-global, and latched
/// because the launch on which the child finally exits still has to report it.
/// The *number* rather than a flag, because the ending is reported onward and a
/// launcher that re-raises SIGTERM for a SIGHUP has told its parent the wrong
/// thing.
///
/// **A latch that outlives its launch is a loaded gun**, and this one is scoped
/// to exactly one launch at both ends. [`run`] clears it immediately before
/// spawning, so a signal that arrived while no child existed can never be
/// spent on a fresh child that has not signalled and has done nothing wrong;
/// [`take_interrupt`] lets a launcher consume it between launches, which is
/// where such a signal actually belongs. Without the clear, a driver signalled
/// in the gap between two iterations starts the next session and SIGTERMs it on
/// its first poll.
static INTERRUPTED_BY: AtomicI32 = AtomicI32::new(0);
````
<!-- /fragment -->

Chapter 7 placed the clear and deferred the argument for it: `run` stores a zero
here immediately before `command.spawn()`, with nothing left between the two but
the call. This is the argument's other end. The latch is scoped to one launch at
both ends — cleared at the spawn, and consumed either by the poll loop that
reports it as `End::Interrupted` or by `take_interrupt` below — and both ends
exist for the same failure. Without the clear, a driver signalled in the gap
between two sessions starts the next one and SIGTERMs it on its first poll. That
case is phase 3 of
`an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other`, which
raises SIGTERM while nothing is running, deliberately does *not* collect it, and
then asserts that the next launch comes back `End::Exited` with a successful
status — a child killed for a signal that predated it would fail both.

Three properties of the declaration are worth naming because none of them is
argued in the comment. It is a `static` rather than a thread-local, which is the
storage that matches the scope the comment claims for it: a disposition belongs
to the process, so the record of one firing has to as well. It is an `AtomicI32`
because `i32` is what `libc`'s handler signature delivers and what `reraise`
below has to take back. And every one of the three accesses — the store in the
handler, the swap in `take_interrupt`, the store in `run` — uses
`Ordering::Relaxed`, which is sufficient here because the value publishes nothing
else: there is no second write for a stronger ordering to order against it.

<a id="between-launches"></a>
## The signal that arrives between launches

A launcher that runs one launch and exits has no use for the next function. A
launcher that runs launches in a loop cannot do without it, and the comment says
why in its second paragraph.

<!-- fragment «run-take-interrupt» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="150-168" parent="watch-and-launcher-signals" -->
````rust

/// Which signal, if any, was sent to this process outside a launch — clearing
/// the latch.
///
/// For a launcher that runs launches in a loop: [`run`] reports a signal that
/// arrives *during* a launch as [`End::Interrupted`], but one arriving between
/// two launches has no launch to be reported against, and `run` deliberately
/// discards it rather than spending it on the next child. Call this at the top
/// of the loop to honour it instead.
///
/// It answers `None` before this process's first [`run`], because nothing has
/// installed a handler yet and the signal took its default disposition.
#[must_use]
pub fn take_interrupt() -> Option<i32> {
    match INTERRUPTED_BY.swap(0, Ordering::Relaxed) {
        0 => None,
        signal => Some(signal),
    }
}
````
<!-- /fragment -->

Chapter 5 counted the crate's twelve `#[must_use]` functions and found eleven of
them queries whose returned value is the only reason to call them. This is the
twelfth, and the one that breaks the shape: the body is a `swap`, so asking the
question consumes the answer. That is the design rather than a shortcut. A signal
is an event and not a condition, and an event that could be collected twice would
stop a looping launcher twice — which the test asserts in the same phase that
first collects it, by calling `take_interrupt` a second time and requiring
`None`. The attribute is doing correspondingly harder work here than on the
eleven: a launcher that called this and dropped the result would have consumed
the signal and acted on nothing.

The division of labour with `run` is exact and is stated nowhere else. A signal
arriving **during** a launch has a launch to be reported against, and `run`
reports it as `End::Interrupted`. A signal arriving **between** two launches has
none, and `run` discards it — not by ignoring it, but by clearing the latch at
the next spawn, which is the same act. `take_interrupt` is the only way to
observe it before that clear happens, and calling it at the top of a loop is what
turns *discarded* into *honoured*.

The last paragraph's `None` before the first `run` is not a special case in the
code — there is no such branch — but a consequence of the latch's initial zero
and of the handler being installed by `run` rather than at start-up. Before the
first launch the process has no handler, so a termination signal takes its
default disposition and there is no process left to ask.

<a id="dying-of-it"></a>
## An exit code cannot say *was signalled*

The other half of a looping launcher's obligation is what it does with the number
once it has it, and this is the crate's answer. It diverges, and its comment is
the sharpest argument in the file.

<!-- fragment «run-reraise» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="169-211" parent="watch-and-launcher-signals" -->
````rust

/// Die of the signal that ended this launcher, so **its** parent sees the
/// conventional `128 + N` in the wait status.
///
/// A process that catches SIGTERM, cleans up and exits 0 has told a systemd
/// unit, a `timeout(1)` or a shell `wait` that it finished its work. An exit
/// *code* cannot express "was signalled" at all — only a wait status can, and
/// the only way to produce one is to actually die of the signal. So the
/// disposition this crate installed is put back to the default, the signal is
/// unblocked in case it is still masked from the handler that ran, and it is
/// raised.
///
/// **This crate owns the call because this crate installed the handler.** A
/// consumer undoing it would be reaching for a disposition it did not set and
/// cannot see, and would get it wrong for a signal `run` starts catching later.
/// What stays the consumer's is *whether* to re-raise, which is a statement
/// about that process's own exit status.
pub fn reraise(signal: i32) -> ! {
    // Buffered output is lost by a signal death the way it is lost by
    // `process::exit`, and the last diagnostic before a termination is the one
    // a reader most wants.
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();

    // SAFETY: restoring the default disposition of a signal this crate set a
    // handler for, unblocking that one signal, and raising it against this
    // process. `sigset_t` is initialised by `sigemptyset` before use.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        let mut unblock: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut unblock);
        libc::sigaddset(&mut unblock, signal);
        libc::sigprocmask(libc::SIG_UNBLOCK, &unblock, std::ptr::null_mut());
        libc::raise(signal);
    }

    // Unreachable for any signal whose default action terminates, which is
    // every signal a launcher is interrupted by. If a caller passes one that
    // does not — SIGCHLD, SIGURG — saying so in the exit code is still better
    // than falling through to whatever the caller does after an infallible
    // call it believed diverged.
    std::process::exit(128 + signal)
}
````
<!-- /fragment -->

This is where `End::Interrupted`'s payload earns its existence. Chapter 7 read
the field and deferred the reason: the signal is carried rather than merely
noted, so that a launcher can report it onward. *Onward* turns out to mean
something narrower than reporting — it means dying of the same signal, because a
wait status is the only thing that can say *was signalled* and the only way to
produce one is to actually die of it. The three sites are one fact seen three
times: `on_terminate` records the number, `End::Interrupted` carries it out of
the launch, and `reraise` takes it back in. A crate that latched a boolean would
have made the third impossible.

`a_reraised_signal_reaches_the_parent_as_a_wait_status` in
`crates/keyed-launch/tests/reraise.rs` is the test, and its shape follows from
the claim: since the function ends the process it is called in, the test re-runs
*itself* as a child and reads the wait status the child leaves behind. Its second
assertion is the one that matters. `status.signal()` being `Some(15)` shows the
signal death happened; `status.code()` being `None` is the half a launcher that
catches SIGTERM and exits zero gets wrong, because there is no exit code — zero
included — that means *killed*. The file is its own integration-test binary for
the same reason, since a `reraise` in a shared binary would take every other test
in it along.

What happens before the `unsafe` block and what happens inside it answer
different questions. The two flushes are there because a signal death loses
buffered output exactly as `process::exit` does, and the diagnostic a reader most
wants is the last one before a termination. Inside, the disposition is put back to the
default, the signal is unblocked in case the handler that ran left it masked, and
only then is it raised — three steps because skipping any one of them leaves the
raise either caught, or blocked, or both. The `std::process::exit(128 + signal)`
after an infallible call is not dead code but the crate declining to lie about
its own signature: `-> !` is a promise, and a caller passing a signal whose
default action does not terminate would otherwise fall through it.

The comment's last paragraph draws a boundary this book has drawn in every
chapter, and this is its sharpest form. The crate owns the *call* because the
crate installed the handler, and a consumer undoing a disposition it did not set
would get it wrong for a signal `run` starts catching later. What stays the
consumer's is *whether* to re-raise at all, because that is a statement about the
consumer's own exit status and not about this launch. Grove takes exactly that
half: `crates/grove/src/cli.rs` matches its loop's interrupted outcome and calls
`reraise` with the number, and nothing in this crate knows or could check that it
did.

<a id="the-handler"></a>
## One store, because one store is all that is safe

The handler behind the latch is a single statement, and its comment explains the
size rather than apologising for it.

<!-- fragment «run-on-terminate» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="212-218" parent="watch-and-launcher-signals" -->
````rust

/// A single store is the *only* work done here, because it is the only work
/// that is async-signal-safe. Signalling and reaping the child happen one poll
/// tick later, on [`run`]'s ordinary stack.
extern "C" fn on_terminate(signal: libc::c_int) {
    INTERRUPTED_BY.store(signal, Ordering::Relaxed);
}
````
<!-- /fragment -->

A signal handler may call only async-signal-safe functions, so the set of things
this function *could* do is very small, and the crate takes the smallest member
of it. That is why the latch is an atomic integer rather than a channel, a queue
or a flag with a payload: the storage was chosen to fit what a handler is allowed
to write to it, not the other way round. Everything a real termination needs —
signalling the child's group, waiting for it, reaping it, building an `Ended` —
happens up to one poll tick later on the supervisor's ordinary stack, where none
of those restrictions apply. The cost is that latency, and it is the same 500ms
bound `POLL_INTERVAL` already sets on the escalation.

The handler is installed rather than exported, and the function that installs it
is the last of this block. Its comment carries an argument about a signal that is
*not* in it.

<!-- fragment «run-install-termination-handler» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="219-243" parent="watch-and-launcher-signals" -->
````rust

/// Catch SIGTERM and SIGHUP so a launcher can forward termination to its child
/// and reap it rather than orphan it.
///
/// Installed by [`run`] rather than exported, because [`End::Interrupted`] is a
/// promise this crate makes and a caller cannot be relied on to have enabled
/// it. Re-installing the same handler is idempotent, so calling it once per
/// launch costs nothing.
///
/// SIGINT is deliberately absent: Ctrl-C is delivered to the terminal's
/// foreground process group, which — once [`run`] has handed the terminal over
/// — is the child's and not the launcher's. What a launcher does about a SIGINT
/// it does receive is its policy, not this crate's.
fn install_termination_handler() {
    // Through the function *pointer* rather than casting the function item
    // straight to an integer, which rustc warns about: a function item is
    // zero-sized and the cast reads as a value conversion rather than the
    // address-taking it is.
    let handler = on_terminate as extern "C" fn(libc::c_int) as usize;
    // SAFETY: `signal(2)` with a handler that performs one relaxed atomic store.
    unsafe {
        libc::signal(libc::SIGTERM, handler as libc::sighandler_t);
        libc::signal(libc::SIGHUP, handler as libc::sighandler_t);
    }
}
````
<!-- /fragment -->

Two signals are caught, and `run` calls this once per launch because
re-installing the same handler is idempotent and costs nothing. The reason it is
called by `run` rather than exposed for a caller to call is a promise: chapter 7
read `End::Interrupted` as one of three cases `run` can return, and a case that
only materialises when the caller remembered to enable it is not a case the
return type can honestly declare.

SIGINT's absence is the argument, and chapter 7 read its other half. The
disposition list `DEFAULT_DISPOSITION_IN_CHILD` hands SIGINT back to the child at
its default; this handler declines to catch it in the launcher. Both follow from
one fact about job control: Ctrl-C is delivered to the terminal's *foreground*
process group, which after chapter 7's handover is the child's group and not the
launcher's. So the child must have SIGINT at its default in order to receive it,
and the launcher has no business intercepting a signal that is not addressed to
it. What a launcher does about a SIGINT it does receive — one typed before the
handover, or sent directly — is left as that launcher's policy.

The function-pointer cast in the body is the one line here that is about Rust
rather than about signals. A function item is zero-sized, so casting it straight
to an integer reads as a value conversion rather than the address-taking it
actually is, and rustc warns about it; going through the function *pointer* type
first says what is meant. The `SAFETY` comment justifies the `unsafe` by naming
what the handler does — one relaxed atomic store — which is the same sentence
`on_terminate`'s own comment makes from the other side.

<a id="taking-the-terminal-back"></a>
## Taking the terminal back, and only from this job

The rest of the file is the supervisor. `supervise` does two things, and five of
its twelve body lines are the comment on the second: it runs the watch, and then
it puts the terminal back.

<!-- fragment «run-supervise» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="449-469" parent="supervise-and-escalate" -->
````rust

fn supervise(
    child: Child,
    channel: &Channel,
    escalation: Escalation,
    terminal: Option<Terminal>,
    pgid: libc::pid_t,
) -> Result<Ended, LaunchError> {
    let outcome = watch(child, channel, escalation, terminal.as_ref(), pgid);
    // Take the terminal back, and only from the job this launch owned. A
    // launcher that returned while the terminal still belonged to a dead group
    // would leave its own next write — `stty`, a diagnostic — to a terminal it
    // is a background job on, which is a SIGTTOU stop rather than an error
    // anybody could read.
    if let Some(terminal) = &terminal {
        if terminal.foreground() == pgid {
            terminal.hand_to(own_group());
        }
    }
    outcome
}
````
<!-- /fragment -->

The shape of the function is the argument. `outcome` is bound before the reclaim
and returned after it, so the terminal is taken back on **every** exit from
`watch` — including the two `Err` returns the poll loop can produce, where the
launch has failed and a launcher that skipped the reclaim would fail while
holding somebody else's terminal. Nothing in the comment says this; it is what
`let outcome = …; …; outcome` is for, against the alternative of returning the
call directly.

The guard is chapter 7's handover guard run in reverse. That one asked *is this
launcher the terminal's current owner*, because handing over a terminal owned by
somebody else's job is theft; this one asks *is the child's group still the
owner*, because taking the terminal back from a job this launch did not own is
the same theft in the other direction. The consequence of getting it wrong is
named in the comment and is worse than an error: a launcher that returned while
the terminal belonged to a dead group would be a background job on that terminal,
and its own next write — an `stty`, a diagnostic — would raise SIGTTOU and stop
it, which no reader could diagnose from the outside.

The two owned parameters go in and neither comes back. `terminal` is passed to
`watch` as a borrow and kept here so that it is still alive for the reclaim, then
dropped at the end of this function, which closes the descriptor `Terminal::open`
took. The `Child` was moved into `watch`, which is where it is reaped. This is
the last function that holds either, and after it a launch owns nothing.

<a id="three-observables"></a>
## The three ways a launch ends

The poll loop is one function, and it opens by declaring everything the loop
will decide with: a start time, the state machine, and two latches.

<!-- fragment «run-watch-signature» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="470-481" parent="supervise-and-escalate" -->
````rust

fn watch(
    mut child: Child,
    channel: &Channel,
    escalation: Escalation,
    terminal: Option<&Terminal>,
    pgid: libc::pid_t,
) -> Result<Ended, LaunchError> {
    let started = Instant::now();
    let mut watch = Watch::Running;
    let mut interrupted: Option<i32> = None;
    let mut signalled = false;
````
<!-- /fragment -->

Two of the four locals are latches rather than state, and the difference matters
for what the loop can report. `watch` moves between three values many times;
`started` never changes. `interrupted` and `signalled` are each written at most
once and never cleared, because each records *that something happened*, not that
it is still happening — the child ended by an escalation is gone by the time the
value is read, and so is the launcher's chance to un-receive a signal.

What the loop actually observes is three things — the child, the channel and the
latch — and those four locals are where each is recorded or timed. The table
below is the crate's complete account of how a launch can end, and its fourth row
is the one with no mechanism behind it.

| What the loop observes | Where it is polled | The ending it produces | Held by |
|---|---|---|---|
| the child is gone | `child.try_wait()` | `End::Exited`, with or without a token | `a_child_that_never_signals_ends_with_no_token`, `an_unsignalled_child_runs_to_its_own_exit_untouched`, `a_child_that_signals_and_exits_inside_the_grace_is_never_touched` |
| the token's file exists | `channel.path().exists()` | `End::Signalled`, once the grace has run out | `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` |
| the launcher was signalled | `take_interrupt()` | `End::Interrupted { signal }` | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| **the child finished and never said so** | nowhere | **none — the launch stalls** | nothing, because there is nothing to hold |

The closure that builds the return value is next, and it is where the first three
rows are turned into one of them.

<!-- fragment «run-watch-ended» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="482-494" parent="supervise-and-escalate" -->
````rust

    let ended = |status: ExitStatus, interrupted: Option<i32>, signalled: bool| Ended {
        end: match (interrupted, signalled) {
            (Some(signal), _) => End::Interrupted { signal },
            (None, true) => End::Signalled,
            (None, false) => End::Exited,
        },
        status,
        elapsed: started.elapsed(),
        // Read after the child is gone, so a child still mid-write cannot be
        // observed half-signalled.
        token: channel.read(),
    };
````
<!-- /fragment -->

The match on `(interrupted, signalled)` is a precedence rule written as a
pattern, and the order is the claim. An interrupt outranks an escalation, so a
launch in which the token appeared, the grace ran out, SIGTERM was sent **and**
the launcher was then signalled comes back `Interrupted` rather than `Signalled`.
That is the right way round because `End` reports who acted, and the launcher's
own death is the outermost thing that acted; a caller that saw `Signalled` there
would conclude the launch completed its work and fell to an ordinary escalation.
The third arm is where an ordinary completion lands, and three tests reach it:
`a_child_that_never_signals_ends_with_no_token` and
`an_unsignalled_child_runs_to_its_own_exit_untouched` with no token, and
`a_child_that_signals_and_exits_inside_the_grace_is_never_touched` with one.

`token` is read once, inside the closure, and the comment says the important part
of when: after the child is gone. That is the second half of chapter 6's argument
about the empty file. Chapter 6 established that an empty channel file reads back
as `None` rather than as an empty token, because a child killed between creating
the file and writing to it leaves a real, empty file behind. The loop's other use
of the channel — `path().exists()` — can see a file mid-write, because appearance
is the whole of what that test asks. Reading the *content* is different: nowhere
in this file is `read` called but here, and this closure runs only once the child
is gone, so there is no tick on which the loop reads a file the child is still in
the middle of writing. The two together are why *appearance
is the event* survives contact with a child being killed for taking too long: the
appearance may be early, and the content is never partial.

<a id="the-poll"></a>
## Two questions asked every tick

The loop's first act is not about the child at all. It is the second of the two
`hand_to` calls chapter 7 counted as this chapter's, and it is the one that hands
the terminal *forward*, to the child's group, on every tick rather than once. Its
comment names the case that makes the repetition necessary.

<!-- fragment «run-watch-terminal-recheck» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="495-506" parent="supervise-and-escalate" -->
````rust

    loop {
        // Re-checked every tick rather than only at the spawn, so a launcher
        // started in the background and later brought forward (`grove &`, then
        // `fg`) hands the terminal on to the job that is actually running under
        // it. The guard is the same one the spawn used: hand over only what
        // this launcher currently owns.
        if let Some(terminal) = terminal {
            if terminal.foreground() == own_group() {
                terminal.hand_to(pgid);
            }
        }
````
<!-- /fragment -->

The spawn's handover ran once, at a moment when the launcher may have had no
terminal to give — a launcher started in the background has none, because it is
not the foreground group of one. Re-asking every tick is what lets `grove &`
followed by `fg` hand the terminal on to the job that is actually running under
it, minutes after the spawn decided there was nothing to hand over. The guard is
the spawn's own — *is this launcher the terminal's current owner* — so the loop
can ask an unconditional question every 500ms without ever handing over a
terminal that is not its to give. The reclaim in the last section asks the
mirrored question against a different group, which is why the three sites are two
conditions rather than one.

The second question is about the child, and it is the one that can end the loop
two different ways.

<!-- fragment «run-watch-try-wait» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="507-532" parent="supervise-and-escalate" -->
````rust

        let waited = match child.try_wait() {
            Ok(waited) => waited,
            Err(error) => {
                // The child's state is now unknown, and returning here would
                // leave an interactive one holding the terminal with nothing
                // left to reap it. Ending it is the last thing this launch can
                // still do correctly, so it does that before reporting.
                kill(pgid, libc::SIGKILL);
                let reaped = child.wait().is_ok();
                return Err(LaunchError::new(format!(
                    "cannot wait on the launched child: {error}; it has been sent SIGKILL and {}",
                    if reaped {
                        "reaped"
                    } else {
                        "could not be reaped — check for an orphaned process"
                    }
                )));
            }
        };
        if let Some(status) = waited {
            // A child ended by the escalation exits non-zero, or by signal.
            // That is the normal completion path, not a failure: the token,
            // never the exit status, says what the launch meant.
            return Ok(ended(status, interrupted, signalled));
        }
````
<!-- /fragment -->

`try_wait` failing is the crate's worst case, and the comment states why it
cannot simply be returned. The child's state is unknown; returning would leave an
interactive one holding the terminal with nothing left in the process tree to
reap it. So the launch does the last thing it can still do correctly — SIGKILL
the group, try to reap — and only then reports. The message is built to the same
shape chapter 1 read off the two error types and chapter 7 applied to the failed
spawn: it names what went wrong, carries the operating system's own words, and
ends by naming what the reader must do, which here is either nothing or *check
for an orphaned process*, depending on which of the two the launcher managed. It
is the one branch in this crate that reports a partial failure rather than a
clean one, and the wording is what makes the difference legible.

The ordinary return sits just below it, and its comment closes the distinction
chapter 7 opened between `end` and `token`. A child that the
escalation ended exits non-zero or by signal, and none of that is a failure of
the launch: `run` returns `Ok`, and the *token* — never the exit status — is what
says whether the child finished its work. `a_child_that_never_signals_ends_with_no_token`
is the pure form of it, running a child that exits 3 and asserting all three of
`token: None`, `End::Exited` and `status.code() == Some(3)`, so a caller reading
the exit code learns what the child did and nothing about what it meant.

<a id="forwarding"></a>
## Forwarding the signal that was actually sent

The third observable is checked next, and unlike the other two it also *acts*.
Its comment argues for the one decision in the block that could plausibly have
gone the other way.

<!-- fragment «run-watch-forward-interrupt» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="533-553" parent="supervise-and-escalate" -->
````rust

        // A signalled launcher forwards **the signal it was sent** and hands
        // over to the same escalation the token path uses, so a child that
        // ignores it is still SIGKILL'd rather than left on the terminal.
        // Forwarding a fixed SIGTERM instead would tell a child that its
        // terminal had not gone away when it had.
        if interrupted.is_none() {
            if let Some(signal) = take_interrupt() {
                interrupted = Some(signal);
                kill(pgid, signal);
                // Start the kill grace only if nothing is already counting one
                // down. Overwriting a running `Terminated` deadline would
                // *extend* the child's life by a full `kill_grace` — so a
                // supervisor trying to hurry a stuck teardown along would be
                // told to wait longer, and each further signal would re-arm it
                // again.
                if !matches!(watch, Watch::Terminated(_)) {
                    watch = Watch::Terminated(Instant::now());
                }
            }
        }
````
<!-- /fragment -->

A launcher told to terminate could forward a fixed SIGTERM to its child, and the
comment rejects it in one sentence: forwarding SIGTERM for a SIGHUP would tell a
child that its terminal had not gone away when it had. That is the fourth place
the number rather than the fact is load-bearing, after the latch, `End::Interrupted`
and `reraise`, and it is the one where the wrong choice is silently wrong — the
child still dies, and it dies believing something false about the world it dies
in.

Two guards bound the block, and they answer different questions. The outer
`interrupted.is_none()` makes collection happen at most once per launch, so a
second signal to the launcher is not a second forwarding. The inner
`!matches!(watch, Watch::Terminated(_))` protects the deadline: writing a fresh
`Terminated(Instant::now())` over one already counting down would **extend** the
child's life by a full `kill_grace`, so a supervisor trying to hurry a stuck
teardown along would be told to wait longer, and each further signal would re-arm
it again. This is the only place `watch` is assigned outside the `match` below,
and the guard is what keeps the two writers from disagreeing about which clock is
running.

Note what is *not* set here: `signalled` stays false. The launch is being ended
by the launcher's own death, not by the escalation, and the closure above turns
that pair into `End::Interrupted`. Phase 4 of
`an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` is the
whole of this block under test — it raises SIGTERM from a thread 300ms into a
launch whose child loops forever, and asserts that the ending names the signal,
that the child's own status carries the same number, and that the latch is empty
afterwards so the next launch is not stopped by an interrupt already reported.

<a id="the-escalation-runs"></a>
## Where the escalation actually runs

The state machine is the last thing each tick does, and it is where the token
becomes a deadline and the deadline becomes a signal.

<!-- fragment «run-watch-escalation» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="554-580" parent="supervise-and-escalate" -->
````rust

        watch = match watch {
            Watch::Running if channel.path().exists() => Watch::Signalled(Instant::now()),
            Watch::Signalled(at) if at.elapsed() >= escalation.grace => {
                // `signalled` is latched *here*, where the escalation actually
                // runs, and not where the token appeared. `End` would otherwise
                // be telling the caller only what `token` already tells it,
                // while claiming something stronger: that this launch had to be
                // ended. A child that signals and then exits inside its own
                // grace was never touched, and says so.
                signalled = true;
                kill(pgid, libc::SIGTERM);
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= escalation.kill_grace => {
                kill(pgid, libc::SIGKILL);
                let status = child.wait().map_err(|error| {
                    LaunchError::new(format!("cannot reap the killed child: {error}"))
                })?;
                return Ok(ended(status, interrupted, signalled));
            }
            other => other,
        };

        std::thread::sleep(POLL_INTERVAL);
    }
}
````
<!-- /fragment -->

The comment inside the second arm is this chapter's most consequential line, and
it is about *placement* rather than about behaviour. `signalled` is latched
where the escalation runs, not where the token appeared. Chapter 7 stated the
consequence as a property of the type — `End::Signalled` is narrower than *a
token appeared* — and this is the line that keeps it. Had the latch been set in
the first arm, `end` would have told the caller only what `token` already told
it, while claiming something stronger: that this launch had to be ended.
`a_child_that_signals_and_exits_inside_the_grace_is_never_touched` is the case
that separates them, running under a thirty-second grace so the child's own exit
lands well inside it; it comes back `End::Exited` *with* a token, and asserts
`elapsed < grace` so that a passing run cannot be one that waited the grace out.

The three arms are guarded by conditions rather than by state alone, which is why
`other => other` is needed and is not a default: `Running` with no file, and
either timed state before its deadline, all mean *nothing to do this tick*. Read
downward, the arms are the escalation in order — appearance starts a clock, the
grace expires into SIGTERM and a second clock, the kill grace expires into
SIGKILL — and each transition records the `Instant` the next one is measured
from.

The two kill arms end differently, and the asymmetry is the point. SIGTERM is
sent and the loop **continues**, so the child is reaped by the next tick's
`try_wait` on the ordinary path — because SIGTERM is a request and a child may
decline it, which is precisely the case the third arm exists for. SIGKILL is sent
and the function **blocks** on `child.wait()`, because a child cannot decline
SIGKILL and there is nothing left to poll for. `a_child_that_ignores_sigterm_is_killed_after_the_kill_grace`
is the test that walks both, with `trap '' TERM` in the child, and it asserts on
`status.signal() == SIGKILL` and on `elapsed >= grace + kill_grace`.

`std::thread::sleep(POLL_INTERVAL)` closes the tick, and it is the constant
chapter 7 read as *not a knob*. Its consequence is visible in the tests rather
than in the code, and it is arithmetic over the constant. Every observation the
loop makes is quantised to a tick: the token is seen up to half a second after it
appears, each deadline is noticed up to half a second after it passes, and the
child's death is noticed up to half a second after it happens. So `elapsed`
carries slack the two steps of the escalation do not separate.
`a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` therefore
uses `elapsed >= grace` as a lower bound only, and asserts
`status.signal() == SIGTERM` to say *which* step actually ran. A test that tried
to tell the two apart by timing would be measuring the poll interval.

<a id="the-whole-group"></a>
## The whole group, and then the child

Every `kill` on this page has gone through one function whose body is two calls,
and its eighteen lines of comment are the record this chapter keeps.

<!-- fragment «run-kill» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="581-607" parent="supervise-and-escalate" -->
````rust

/// Signal the job this process launched — **the whole process group, then the
/// child itself**.
///
/// A grandchild the child spawned — a tool subprocess, a language server, an
/// agent's own in-flight command — is a member of that group and is reaped with
/// its parent rather than surviving it. That matters beyond tidiness: such a
/// grandchild can hold a lock its launcher's caller is about to wait on, and
/// then the escalation's SIGKILL buys a stall rather than a teardown.
///
/// `pgid` is the child's pid, made a group leader by the `setpgid` on both
/// sides of the fork in [`run`]. A group with that id can only have been
/// created by that process, so `-pgid` cannot name an unrelated job even in the
/// impossible case where both `setpgid` calls failed; the direct `kill` behind
/// it covers that case.
///
/// A failure is ignored on purpose — ESRCH means the process exited between the
/// poll and the signal, which the next `try_wait` reports anyway. This is the
/// shell's `kill … 2>/dev/null`, written down.
fn kill(pgid: libc::pid_t, signal: libc::c_int) {
    // SAFETY: `kill(2)` on the process group of, and then the pid of, a child
    // of this process.
    unsafe {
        libc::kill(-pgid, signal);
        libc::kill(pgid, signal);
    }
}
````
<!-- /fragment -->

Two calls, in that order, and the first is the one that makes the escalation
worth having. A grandchild the child spawned — a tool subprocess, a language
server, an agent's own in-flight command — is a member of the child's process
group and is reaped with its parent rather than surviving it. The comment names
the cost of the alternative in one clause: such a grandchild can hold a lock its
launcher's caller is about to wait on, and then the escalation's SIGKILL buys a
stall rather than a teardown. That is the third arm of the book's outcome seen
from the far end — a launcher that ended the process it could see and inferred
that the job was over.

This function is where *the launched child is a job* is kept, and the record
settles which of the child's identities changes. The child gets a process group
of its own so that the group can be signalled, and explicitly **not** a session,
because a session leader has no controlling terminal and an interactive child
would then be a background job stopped on its first read. Chapter 7 owns the
spawn side of that record — the group, the terminal, the dispositions — and this
function is its other half: signalling `-pgid` is the reason the group exists at
all. The record weighs four alternatives and rejects every one; the two these two
chapters are in a position to rule out between them are a new session and an
escalation addressed to the pid alone.

`the_escalation_reaps_the_childs_descendants` is the test, and chapter 7 named it
as this chapter's to read because it is the only place the child's process group
is observed at all — indirectly, through what the escalation reaches. Its child
starts a background `sh` loop, writes that grandchild's pid to a file, signals,
and then goes on waiting, so the grace runs out and the SIGTERM step fires — the
child has no `trap`, so it dies there and SIGKILL is never reached. The assertion
that the grandchild is gone is only half of it. The test also spawns a
**bystander** — the same shape of process, started at the same moment, in the
*test process's* group rather than the child's — and asserts it is untouched.
Without that control a fixture that reported "gone" for any pid, a `kill(2)`
probe misreading its errno, would pass identically. The pair is what makes the
claim *the group and only the group* checkable rather than merely observed.

One sentence of the comment is not this book's to repeat. It attributes the
child's group leadership to *the `setpgid` on both sides of the fork*, and
chapter 7 measured that spawn: installing a `pre_exec` closure takes `std` off
its `posix_spawn` fast path, `Command::spawn` returns only once the child has
already `execve`d, and the parent's `setpgid` fails `EACCES` every time — thirty
of thirty, with controls showing that the same call can return success and can
return `ESRCH`. The leadership comes from `command.process_group(0)` alone.
[The measurement and its controls](07-the-job.md#the-latch-and-the-child-away)
are chapter 7's; what matters here is that the comment's **conclusion** is
untouched by it. A process group with that id can only have been created by that
process, so `-pgid` cannot name an unrelated job, and the direct `kill` behind it
covers the case the comment calls impossible.

The last paragraph is the one that turns a discarded return value into a
decision. `ESRCH` means the process exited between the poll and the signal, which
the next `try_wait` reports anyway, so there is nothing for a caller to do with
the failure and nothing for the loop to change. Naming it as *the shell's
`kill … 2>/dev/null`, written down* is what separates it from a suppressed
warning: the shell discards the same failure for the same reason, and a launcher
that propagated it would be reporting the child's normal exit as an error.

That is `src/run.rs`, and with it the launch half of the crate. A path was drawn
and written by nobody, an argv was authored by a template and by nothing else, a
child was spawned into a job with nothing added, and a launcher waited for one of
three things and acted on whichever came first. At no point did the crate decide
that the child was finished. What it has to show for the launch is an `Ended`
naming who acted and, if the child spoke, the string it wrote — which this crate
carried across two processes and never read.

Both halves are now complete, and neither has been held to a contract from
outside. Chapter 9 is where they are: the conformance kit that checks a
consumer's configuration without knowing what a key is for, and the nine tests
inside `src/channel.rs` that reach a function no integration test can.

[Previous: The child is a job](07-the-job.md) | [Contents](README.md) | [Next: How this is checked](09-how-checked.md)
