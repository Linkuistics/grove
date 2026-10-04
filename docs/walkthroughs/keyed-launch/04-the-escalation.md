# The watch and the escalation
<!-- book-page id="the-escalation" slice="the-launchers-job" order="4" -->
[Previous: The child is a job](03-the-job.md) | [Contents](README.md) | [Next: How this is checked](05-how-checked.md)

<a id="the-launchers-job"></a>
## The launcher's job

Chapter 3 ended with a process: a program its caller named, running in a
process group of its own, holding the terminal, with one path in its
environment. Nothing had been decided about it. This chapter is where the
launcher waits, and then acts.

What this stage must not add and must not interpret is **the ending**. The crate
cannot know that the child is done; it can know only that the child *said* so.
Everything on this page follows from that one restriction, including a
limitation. Supervision polls three things, and they are the only
three ways a launch ends: the child exits, the channel appears, or the
launcher itself is signalled. An interactive child that finishes its turn and
never signals reaches none of them: it returns to its prompt rather than exiting,
so the launch does not end — it **stalls**. That is a real failure mode
with no cheap fix here, because nothing this crate can observe distinguishes a
child that forgot to signal from one still working, and a second completion
observable would only trade a stall for a wrong kill. It is the caller's to
close, at the layer that instructs the child, and chapter 6 closes the book on
it.

Chapter 3 read the reason an escalation exists at all: an interactive child is
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

The same reason reaches past the escalation, to every ending. A child that exits
on the TERM, or within the grace, or on its own with no escalation at all, can
leave members of its group running: a tool that ignored the TERM, or one still at
work. A caller that relaunched or published on that ending would act beside
them. So whatever ended the child, the launcher kills what remains of its group
before it reaps the child, and then confirms the group is gone. That end is the
last thing this page reads.

`src/run.rs` splits between chapter 3 and this one by whose signal it is, and
this chapter owns the two blocks chapter 3 left: lines 169–357, which sit between
that chapter’s two, and lines 958–1300, which close the file — 532 lines. The
first block is the supervisor's state type and the launcher's own signal
machinery; the second is the supervisor itself, with the end of the group. After this page every byte of
`src/run.rs` is accounted for.

<a id="the-two-graces"></a>
## The two graces

This section takes the second half of step 3 of the four-step trace chapter 1
wrote — the half chapter 3 stopped at — and runs it to a value. It starts where
chapter 3 finished, with the child running and its channel not yet created, and it
ends with an `Ended`. It runs twice, because two different things can end the
same launch, and the second is the one the completion channel cannot express.

The launch is the one chapter 3 spawned: `claude --model opus <the mandate>`,
pid 4137 and pgid 4137, with `HARNESS_DISPATCH_EXIT_FILE` set to
`/work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281`, under grove's
own escalation of two seconds and five. The supervisor polls every 500ms.

The first run is the ordinary one for an interactive child. It does its work,
writes `relaunch\n` to the path, and returns to its prompt, where it sits waiting
for input that is not coming. The trace below is the whole escalation, and the
column on the right is the value of the private `Watch` the supervisor carries.

```text
 t=0.00   spawn returns, the latch having been cleared just before it
          poll: exited → no; signal file absent                     Running
 t=0.50   poll: exited → no; signal file absent                     Running
 t=1.00   poll: exited → no; signal file absent                     Running
 t=1.30   the child writes "relaunch\n" and returns to its prompt
          — nothing observes this yet —
 t=1.50   poll: exited → no; signal file EXISTS                     Signalled(1.50)
 t=2.00   poll: 0.50s of the 2s grace elapsed                       Signalled(1.50)
 t=2.50   poll: 1.00s elapsed                                       Signalled(1.50)
 t=3.00   poll: 1.50s elapsed                                       Signalled(1.50)
 t=3.50   poll: 2.00s elapsed ≥ grace
          kill(-4137, SIGTERM) then kill(4137, SIGTERM)             Terminated(3.50)
 t=4.00   poll: exited → yes; the child is a zombie, still unreaped
          kill(-4137, SIGKILL), 20ms, kill(-4137, SIGKILL)
          reap → signal 15; the terminal back, its modes restored
          kill(-4137, 0) → ESRCH: the group is gone
          → Ended { end: Escalated, status: signal 15, elapsed: 4.02s,
                    signalled: true, group: Gone }
```

Two things in that column are worth reading before the source explains them. The
signal was created at `t=1.30` and observed at `t=1.50`: the grace runs from the
**observation**, not from the appearance, so the child's real reprieve is the
grace plus up to one poll interval. That is exactly what chapter 3's
`POLL_INTERVAL` meant by *bounding how late an escalation starts*, and it is why
the crate's own tests, which do assert that a grace elapsed, use the *signal* to
say which step of the escalation actually ran. `SIGTERM` at `t=3.50` did not end
the launch: the next tick observed the child's exit, on the ordinary path,
because a child that has been asked to die may still decline. And the exit did
not end it either. Between `exited → yes` and the reap, the launcher kills what
remains of the child's group, so a tool the child left behind dies while the
zombie child still reserves the group's ID.

A child that does decline takes the same trace to its second step. With
`trap '' TERM` installed it survives `t=3.50` untouched, the five-second
`kill_grace` runs from there, and at `t=8.50` the supervisor sends
`kill(-4137, SIGKILL)`, observes the exit ten milliseconds later, ends the group
and reaps the child as above, and returns
`Ended { end: Escalated, status: signal 9, elapsed: 8.53s, signalled: true, group: Gone }`.
That is the whole escalation: grace, SIGTERM, kill grace, SIGKILL.

The second run is the ending the channel has no way to express. Nothing about the
child changes; the *launcher* is sent SIGTERM at `t=1.10`, before the child has
written anything.

```text
 t=1.10   the launcher (pid 4100) is sent SIGTERM
          on_terminate stores 15 into INTERRUPTED_BY and returns
 t=1.50   take_interrupt() → Some(15); interrupted = Some(15)
          kill(-4137, 15) then kill(4137, 15)                       Terminated(1.50)
          poll: exited → no
 t=2.00   poll: exited → yes
          the group killed twice, the child reaped, the terminal back,
          the group confirmed gone
          → Ended { end: Interrupted { signal: 15 }, status: signal 15,
                    elapsed: 2.02s, signalled: false, group: Gone }
```

The child was signalled and reaped rather than orphaned onto the terminal, and
the value that comes back names the number the *launcher* was sent. `signalled`
is false because nothing wrote the file, which is the
ordinary shape of an interrupt. Four fields differ between the two runs, and
only one of them is the distinction: `end` says who acted — `Escalated` is the
escalation, `Interrupted` is the launcher's own death arriving mid-launch —
while `elapsed` and `signalled` differ only because the second launch
was cut short before its child spoke.
`a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` and
`a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` in
`crates/keyed-launch/tests/launch.rs` hold the first run's two steps;
`an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` in
`crates/keyed-launch/tests/interrupt.rs` holds the second.

<a id="what-the-blocks-answer"></a>
## What the two blocks answer

This chapter's 532 lines are two blocks with chapter 3's spawn between them: one
private enum, two statics and six functions at the top of the file; then the
end's three constants, the launch mode, the private process seam, the
supervisor, the poll loop and the signalling helper that close it. The table
collects what each answers and what pins it. Tests named without a path are in
`crates/keyed-launch/tests/launch.rs`; the others name their own files.

| Item | Answers | Pinned by |
|---|---|---|
| `Watch` | where this launch is in its ending | — |
| `INTERRUPTED_BY` | which signal the launcher was sent, during which launch | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| `take_interrupt` | which signal arrived when no launch was running | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| `reraise` | how a launcher dies of the signal it was sent | `a_reraised_signal_reaches_the_parent_as_a_wait_status` (`tests/reraise.rs`) |
| `on_terminate` | what a signal handler may safely do | — |
| `install_termination_handler`, `disposition` | which signals cancel a launch, and that an ignored one stays ignored | `a_launcher_with_hup_ignored_at_entry_installs_no_hup_handler`, `int_cancels_a_launch_with_no_terminal` (`tests/job.rs`) |
| `SIGCHLD_IGNORED_AT_ENTRY`, `restore_child_watching` | whether the launcher can still see its child exit | `a_launcher_with_sigchld_ignored_still_supervises_its_child_to_an_end` (`tests/job.rs`) |
| `SECOND_KILL_PAUSE`, `GROUP_CONFIRMATION`, `KILLED_POLL_INTERVAL` | how the end of the group is timed | — |
| `Mode` | how a cancellation reaches the child | `cancellation_forwards_or_kills_by_mode` (private), the cancellation cases of `tests/noninteractive.rs` |
| `Process`, `confirm_gone` | what ends the group, and what confirms it gone | `a_term_ignoring_descendant_is_gone_before_the_launch_returns`, `a_stopped_child_is_neither_reaped_nor_killed` |
| `supervise` | the order of the end: group, reap, terminal, confirmation | `confirmed_reap_precedes_recovery_on_every_wait_path` (private) |
| `watch` | which of the three observables happened first | `a_child_that_never_signals_ends_unsignalled`, `an_unsignalled_child_runs_to_its_own_exit_untouched`, `a_child_that_signals_and_exits_inside_the_grace_is_never_touched`, `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace`, `a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` |
| `kill` | what the escalation reaches | `the_escalation_reaps_the_childs_descendants` |

Three rows have no test of their own, and the reasons differ. `Watch` is private
and is exercised by every test in the table's `watch` row. `on_terminate` is
private too, but none of those five signals the *launcher*, so the only thing
that runs it is `tests/interrupt.rs`, which the two rows above it already name.
Asserting on either directly would need an interface the crate does not have and
does not want. The three constants time the end, and their comments carry the
measured reasons for the values. A test could only assert the values back.

Two rows name private tests, which live in `tests/internal/wait_events.rs` and
are compiled into the crate itself so they can reach `supervise` through the
`Process` seam. They trace the order of the end with a fake child, which no real
process can be made to fail on demand. Chapter 5 reads them.

The first composite is the block between chapter 3's two: the supervisor's state
type, the interrupt latch, the functions that stand behind it, and the repair for
an inherited ignored SIGCHLD.

<!-- fragment «watch-and-launcher-signals» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="197-385" parent="source-run" -->
<!-- insert «run-watch-states» -->
<!-- insert «run-interrupted-by» -->
<!-- insert «run-take-interrupt» -->
<!-- insert «run-reraise» -->
<!-- insert «run-on-terminate» -->
<!-- insert «run-install-termination-handler» -->
<!-- insert «run-disposition» -->
<!-- insert «run-sigchld-ignored-at-entry» -->
<!-- insert «run-restore-child-watching» -->
<!-- /fragment -->

The second is the end of the file, and it is the supervisor: the constants and
the seam the end of the group is made of, the supervisor that orders that end,
the poll loop that decides which of the three observables happened, and the
two-call helper that signals a job.

<!-- fragment «supervise-and-escalate» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="999-1340" parent="source-run" -->
<!-- insert «run-second-kill-pause» -->
<!-- insert «run-group-confirmation» -->
<!-- insert «run-killed-poll-interval» -->
<!-- insert «run-mode» -->
<!-- insert «run-process-seam» -->
<!-- insert «run-process-for-child» -->
<!-- insert «run-confirm-gone» -->
<!-- insert «run-watched-and-failed» -->
<!-- insert «run-supervise» -->
<!-- insert «run-watch-signature» -->
<!-- insert «run-watch-lend» -->
<!-- insert «run-watch-forward-interrupt» -->
<!-- insert «run-watch-exited» -->
<!-- insert «run-watch-escalation» -->
<!-- insert «run-watch-end-the-group» -->
<!-- insert «run-kill» -->
<!-- /fragment -->

<a id="three-states"></a>
## Three states, two of which are clocks

The type the supervisor carries is private, has no derives, and never leaves the
file. Its two lines of comment say what it tracks.

<!-- fragment «run-watch-states» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="197-205" parent="watch-and-launcher-signals" -->
````rust
/// The supervisor's state machine: idle until the channel appears, then timed
/// toward SIGTERM and finally SIGKILL, after which only the exit is awaited.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
    /// SIGKILL has been sent; only the exit is awaited.
    Killed,
}
````
<!-- /fragment -->

`Watch` is easy to mistake for a description of the child, and it is not one: it
is a description of *the ending*. `Running` is the state in which no ending has
begun, whatever the child is doing; `Signalled` means the channel has been seen
and a clock is now running toward SIGTERM; `Terminated` means a termination signal
has gone and a second clock is running toward SIGKILL; `Killed` means SIGKILL has
gone and only the exit is awaited. The two clocks are why the middle variants
carry an `Instant` and the outer two carry nothing — there is no deadline until
something has started one, and none left once the last signal is sent. `Killed`
is new with the end of the group: the supervisor used to reap a killed child
with a blocking wait, and now goes on polling, briefly, so that every exit is
observed the same way, unreaped.

Read it against `End`, which chapter 3 defined, and the pair is the crate's whole
answer to *what is this launch doing*. The table sets the two side by side; the
row to read is the last, which is where one launch's escalation moves from the
first type to the second.

| | `Watch` | `End` |
|---|---|---|
| Where it is visible | private, no derives, never leaves `src/run.rs` | public, and what a caller matches on |
| What it describes | where the escalation has got to | who acted |
| Its cases | `Running`, `Signalled(Instant)`, `Terminated(Instant)`, `Killed` | `Exited`, `Escalated`, `Interrupted { signal }` |
| How often it is written | several times per launch | once, at the end |
| The escalation, in it | `Signalled`: the channel has been seen | `Escalated`: the escalation ran |

The types share nothing, which is deliberate. One line in the state machine below
turns the first of those into the second, and its *placement* — not its content
— is the whole of the distinction. `End`'s variant was named `Signalled` too
until `Ended` gained a field of that name; `Watch`'s private state kept it,
because inside the supervisor it means exactly what the field means: the channel
has appeared.

<a id="the-latch"></a>
## A latch that outlives its launch

The launcher's own signals need somewhere to be written down, and there is
exactly one place. Its comment carries four arguments, and every one of them is
a decision this file makes about scope.

<!-- fragment «run-interrupted-by» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="206-224" parent="watch-and-launcher-signals" -->
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

Chapter 3 placed the clear and deferred the argument for it: `run` stores a zero
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

<!-- fragment «run-take-interrupt» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="225-243" parent="watch-and-launcher-signals" -->
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

Fourteen functions in the crate carry `#[must_use]`, and thirteen of them return
a value that is the only reason to call them. This is the fourteenth, and the
one that breaks the shape: the body is a `swap`, so asking the
question consumes the answer. That is the design rather than a shortcut. A signal
is an event and not a condition, and an event that could be collected twice would
stop a looping launcher twice — which the test asserts in the same phase that
first collects it, by calling `take_interrupt` a second time and requiring
`None`. The attribute is doing correspondingly harder work here than on the
other eight: a launcher that called this and dropped the result would have consumed
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
once it has it, and this is the crate's answer. It diverges, and its comment
gives the argument for doing so.

<!-- fragment «run-reraise» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="244-286" parent="watch-and-launcher-signals" -->
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

This is where `End::Interrupted`'s payload earns its existence. Chapter 3 read
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
chapter. The crate owns the *call* because the
crate installed the handler, and a consumer undoing a disposition it did not set
would get it wrong for a signal `run` starts catching later. What stays the
consumer's is *whether* to re-raise at all, because that is a statement about the
consumer's own exit status and not about this launch. grove takes exactly that
half: `crates/grove/src/cli.rs` matches its loop's interrupted outcome and calls
`reraise` with the number, and nothing in this crate knows or could check that it
did.

<a id="the-handler"></a>
## One store, because one store is all that is safe

The handler behind the latch is a single statement, and its comment explains the
size rather than apologising for it.

<!-- fragment «run-on-terminate» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="287-293" parent="watch-and-launcher-signals" -->
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

<!-- fragment «run-install-termination-handler» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="294-335" parent="watch-and-launcher-signals" -->
````rust

/// Catch the signals that cancel this launch, so a launcher can forward
/// termination to its child and reap it rather than orphan it.
///
/// SIGTERM and SIGHUP always; SIGINT for a launch with **no terminal** or a
/// [transparent](Launch::transparent) launcher. A launch with a terminal has
/// handed it to the child, so a typed Ctrl-C reaches the child's group and not
/// the launcher's. What any other launcher does about a SIGINT it receives
/// anyway is its own policy, not this crate's. A launch with no terminal has no
/// other route by which an interrupt could reach its child, and a transparent
/// launcher stands in for its child, so an interrupt sent to it is one sent to
/// the child.
///
/// **A disposition the launcher ignores is left ignored.** Ignoring a signal is
/// a statement the launcher made, the Grove driver's ignored SIGINT for one,
/// and a handler installed over it would turn a signal the launcher chose to
/// survive into a cancellation. Checked on every launch: once this crate
/// installs its handler the disposition is no longer an ignore, so a repeat
/// call finds its own handler and re-installs it, which is idempotent.
///
/// Installed by [`run`] rather than exported, because [`End::Interrupted`] is a
/// promise this crate makes and a caller cannot be relied on to have enabled
/// it.
fn install_termination_handler(catch_interrupt: bool) {
    // Through the function *pointer* rather than casting the function item
    // straight to an integer, which rustc warns about: a function item is
    // zero-sized and the cast reads as a value conversion rather than the
    // address-taking it is.
    let handler = on_terminate as extern "C" fn(libc::c_int) as usize;
    let cancelling: &[libc::c_int] = if catch_interrupt {
        &[libc::SIGTERM, libc::SIGHUP, libc::SIGINT]
    } else {
        &[libc::SIGTERM, libc::SIGHUP]
    };
    for &signal in cancelling {
        if disposition(signal) != libc::SIG_IGN {
            // SAFETY: `signal(2)` with a handler that performs one relaxed
            // atomic store.
            unsafe { libc::signal(signal, handler as libc::sighandler_t) };
        }
    }
}
````
<!-- /fragment -->

Up to three signals are caught, and `run` calls this once per launch: the
choice depends on the launch, and re-installing the same handler is idempotent
and costs nothing. The reason it is called by `run` rather than exposed for a
caller to call is a promise: chapter 3 read `End::Interrupted` as one of three
cases `run` can return, and a case that only materialises when the caller
remembered to enable it is not a case the return type can honestly declare.

SIGINT's place in the list is the argument, and chapter 3 read its other half.
The disposition list `DEFAULT_DISPOSITION_IN_CHILD` hands SIGINT back to the
child at its default; this handler catches it in the launcher only for a launch
with **no terminal**, or for a transparent launcher. Both follow from one fact
about job control: Ctrl-C is
delivered to the terminal's *foreground* process group, which after chapter 3's
handover is the child's group and not the launcher's. So the child must have
SIGINT at its default in order to receive it, and a launcher with a terminal has
no business intercepting a signal that is not addressed to it. A launch with no
terminal is the opposite case. A detached child is in a session of its own, and
an interactive launcher with no controlling terminal has no foreground group for
a Ctrl-C to reach, so a SIGINT sent to the launcher is the only interrupt there
is, and it cancels like TERM. `int_cancels_a_launch_with_no_terminal`, in
`tests/job.rs`, sends SIGINT to a launcher with no controlling terminal and
reads back `interrupted:2`. Its control runs the same launcher on a terminal,
where the launcher holds no handler for SIGINT and dies of it.

A transparent launcher is the exception the same fact makes. It stands in for
its child to its own caller, and a typed Ctrl-C still goes to the child's group,
never to it, so an interrupt that does reach it was *sent* to it — by a caller
that sees it as the child. That means what TERM or HUP would, and it cancels the
same way. `a_transparent_launcher_with_a_terminal_is_cancelled_by_int`, in
`tests/job.rs`, is the terminal launcher of the control above with only
`transparent` set, and it reads back `interrupted:2` with the terminal back in
the launcher's group.

**An ignored signal stays ignored.** The comment's second argument is the
check in the loop: a handler goes in only over a disposition that is not
`SIG_IGN`. Ignoring a signal is something the launcher said, as the Grove driver
says it of SIGINT so that a typed Ctrl-C cannot kill the loop. A handler
installed over it would turn a signal the launcher chose to survive into a
cancellation of its child. The check runs on every launch and needs no memory:
once this crate has installed its handler, the disposition is the handler and
not an ignore, so a later launch re-installs it.
`a_launcher_with_hup_ignored_at_entry_installs_no_hup_handler` starts a launcher
with SIGHUP ignored and reads its dispositions after a launch. SIGHUP is still
ignored, and SIGTERM, which it did not ignore, is handled: that second reading is
the positive control showing the launch did install its handlers.

The function-pointer cast in the body is the one line here that is about Rust
rather than about signals. A function item is zero-sized, so casting it straight
to an integer reads as a value conversion rather than the address-taking it
actually is, and rustc warns about it; going through the function *pointer* type
first says what is meant. The `SAFETY` comment justifies the `unsafe` by naming
what the handler does — one relaxed atomic store — which is the same sentence
`on_terminate`'s own comment makes from the other side.

The check itself is a three-line query, separated out because the SIGCHLD repair
below asks it too.

<!-- fragment «run-disposition» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="336-346" parent="watch-and-launcher-signals" -->
````rust

/// The current disposition of `signal`, read without changing it.
fn disposition(signal: libc::c_int) -> libc::sighandler_t {
    // SAFETY: `sigaction(2)` with a null new action only reads the current one
    // into an initialised struct.
    unsafe {
        let mut current: libc::sigaction = std::mem::zeroed();
        libc::sigaction(signal, std::ptr::null(), &mut current);
        current.sa_sigaction
    }
}
````
<!-- /fragment -->

`sigaction` with a null new action is the only way to *read* a disposition
without changing it. The older `signal(2)` can answer only by installing
something in its place.

<a id="entry-sigchld"></a>
## The one ignore the launcher cannot keep

One inherited ignore cannot be respected, because it would blind the supervisor.
**An ignored SIGCHLD tells the kernel to reap children unwatched**: the child's
exit leaves no zombie, so there is nothing for the supervisor's wait to observe,
and nothing reserving the child's group ID while the rest of the group is killed.
So the launcher repairs it, and has to remember that it did.

<!-- fragment «run-sigchld-ignored-at-entry» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="347-353" parent="watch-and-launcher-signals" -->
````rust

/// Whether this process inherited SIGCHLD ignored: `0` not yet read, `1` yes,
/// `2` no.
///
/// Latched on the first launch, because that launch changes the disposition
/// and every later launch would otherwise read back its own repair.
static SIGCHLD_IGNORED_AT_ENTRY: AtomicU8 = AtomicU8::new(0);
````
<!-- /fragment -->

The latch exists because the repair erases the evidence. The first launch reads
an ignore and replaces it with the default; a second launch reading the
disposition afresh would find that default and conclude the launcher never
ignored SIGCHLD at all. So the entry state is read once per process and kept.
Three values rather than a boolean, because *not yet read* is a third state that
a boolean would have to fold into one of the other two.

<!-- fragment «run-restore-child-watching» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="354-385" parent="watch-and-launcher-signals" -->
````rust

/// **An ignored SIGCHLD makes the kernel reap children unwatched.** The child's
/// exit then leaves no zombie. So there is nothing for `waitid` to observe, and
/// nothing reserves the group's ID while the rest of the group is killed. A
/// launcher that inherited the ignore therefore gets the default back for
/// itself, with no flags, and **never a handler**: a handler would turn every
/// child's exit into EINTR on whatever the launcher is doing.
///
/// Answers whether the entry disposition was the ignore, so the spawn can hand
/// the child the disposition it would have inherited. The repair is the
/// launcher's, not the child's.
pub(crate) fn restore_child_watching() -> bool {
    let ignored = match SIGCHLD_IGNORED_AT_ENTRY.load(Ordering::Relaxed) {
        0 => {
            let ignored = disposition(libc::SIGCHLD) == libc::SIG_IGN;
            SIGCHLD_IGNORED_AT_ENTRY.store(if ignored { 1 } else { 2 }, Ordering::Relaxed);
            ignored
        }
        latched => latched == 1,
    };
    if ignored {
        // SAFETY: `sigaction(2)` installing the default disposition with an
        // empty mask and no flags, which clears SA_NOCLDWAIT with it.
        unsafe {
            let mut default: libc::sigaction = std::mem::zeroed();
            default.sa_sigaction = libc::SIG_DFL;
            libc::sigemptyset(&mut default.sa_mask);
            libc::sigaction(libc::SIGCHLD, &default, std::ptr::null_mut());
        }
    }
    ignored
}
````
<!-- /fragment -->

Two decisions in the body are worth reading. The default goes back with an empty
mask and no flags, which also clears `SA_NOCLDWAIT`, the flag that asks for the
same unwatched reaping without an ignore. And it is **never a handler**: a
handler for SIGCHLD would interrupt whatever the launcher was doing with EINTR
every time any child exited, which is a cost the launcher did not ask for. The
answer travels into chapter 3's `pre_exec`, which puts the child back to the
ignore it would have inherited, so that the repair stays the launcher's alone.
`a_launcher_with_sigchld_ignored_still_supervises_its_child_to_an_end` runs a
launcher with SIGCHLD ignored and a child that leaves a TERM-ignoring descendant
and exits 3. The launch still ends `exited` with code 3 and the group gone. The
launcher reads SIGCHLD at its default afterwards, and the child read it ignored.

<a id="taking-the-terminal-back"></a>
## The order of the end

The rest of the file is the supervisor. It is easiest to read from its result
inward: two private types say what watching can produce, and `supervise` turns
either into what `run` returns.

<!-- fragment «run-watched-and-failed» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1132-1145" parent="supervise-and-escalate" -->
````rust

/// What watching produced: a reaped child, and why it ended.
struct Watched {
    status: ExitStatus,
    interrupted: Option<i32>,
    escalated: bool,
    elapsed: Duration,
}

/// A supervision that failed, with the child's status if it was reaped anyway.
struct Failed {
    error: LaunchError,
    status: Option<ExitStatus>,
}
````
<!-- /fragment -->

`Watched` is a child that was reaped, with the two latches that say why it
ended and its duration frozen at the reap. `Failed` is a supervision that went wrong, and it carries the child's
status whenever the child was reaped anyway, because the terminal's return
depends on how the child ended even when the launch reports an error.

<!-- fragment «run-supervise» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1146-1195" parent="supervise-and-escalate" -->
````rust

/// Watch the child to its end, then end its group, take back the terminal,
/// and confirm the group gone. The order is the contract:
///
/// 1. observe the exit, never a stop, without reaping;
/// 2. kill what remains of the group, twice, while the zombie child still
///    reserves its ID;
/// 3. reap, which is when [`LaunchEvent::Reaped`] is emitted;
/// 4. take the terminal back and restore it;
/// 5. confirm, querying only, that the group is gone.
///
/// The child's status is fixed at step 1, so nothing after it can change what
/// the launch reports about the child. Step 5 can only add whether anything
/// survived it.
fn supervise(
    mut child: impl Process,
    channel: Option<&Channel>,
    escalation: Escalation,
    mode: Mode,
    observer: &mut dyn FnMut(LaunchEvent),
    mut lend: impl FnMut(),
    reclaim: impl FnOnce(Option<ExitStatus>),
) -> Result<Ended, LaunchError> {
    match watch(&mut child, channel, escalation, mode, observer, &mut lend) {
        Ok(Watched {
            status,
            interrupted,
            escalated,
            elapsed,
        }) => {
            reclaim(Some(status));
            let group = child.confirm_gone();
            Ok(Ended {
                end: match (interrupted, escalated) {
                    (Some(signal), _) => End::Interrupted { signal },
                    (None, true) => End::Escalated,
                    (None, false) => End::Exited,
                },
                status,
                elapsed,
                signalled: channel.is_some_and(Channel::appeared),
                group,
            })
        }
        Err(Failed { error, status }) => {
            reclaim(status);
            Err(error)
        }
    }
}
````
<!-- /fragment -->

The comment's numbered list is the job contract's end, and **the order is the
contract**. Step 1 observes the exit without reaping, so the child is a zombie
that still holds its pid. Step 2 kills what remains of its group while that pid
still reserves the group's ID, so the signal cannot reach a stranger that
inherited the number. Step 3 reaps, and is the moment `LaunchEvent::Reaped` is
emitted. Step 4 gives the terminal back through chapter 3's lease. Step 5 asks,
without signalling, whether the group is gone. The child's status was fixed at
step 1, so nothing after it can change what the launch reports about the child;
step 5 can only add whether anything survived. The private test
`confirmed_reap_precedes_recovery_on_every_wait_path` traces that
order with a fake child on every wait path, and
`a_surviving_group_is_reported_beside_the_childs_status` pins step 5's other
answer: a group still present comes back as `Group::Present`, with the child's
own status and ending intact.

The `match` on `(interrupted, escalated)` is a precedence rule written as a
pattern, and the order is the claim. An interrupt outranks an escalation, so a
launch in which the channel appeared, the grace ran out, SIGTERM was sent **and**
the launcher was then signalled comes back `Interrupted` rather than `Escalated`.
That is the right way round because `End` reports who acted, and the launcher's
own death is the outermost thing that acted; a caller that saw `Escalated` there
would conclude the launch completed its work and fell to an ordinary escalation.
Watching takes its final signal sample immediately after the wait, before the
reap observer runs. Terminal recovery and group confirmation cannot relabel a
reaped launch: a signal they receive stays in the latch for the caller's next
`take_interrupt`. The third arm is where an
ordinary completion lands, and three tests reach it:
`a_child_that_never_signals_ends_unsignalled` and
`an_unsignalled_child_runs_to_its_own_exit_untouched` without signalling, and
`a_child_that_signals_and_exits_inside_the_grace_is_never_touched` signalled.

`signalled` is asked after the child is gone. It uses chapter 2's `appeared`,
so a child that creates its channel and exits immediately has still signalled,
whatever ended it. The runner never opens the entry; channel content contributes
nothing to the result. This observation is separate from confirmed reap and
from whether the process group was confirmed gone.

<a id="three-observables"></a>
## The three ways a launch ends

The poll loop is one function, and it opens by declaring everything the loop
will decide with: the state machine and two latches.

<!-- fragment «run-watch-signature» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1196-1208" parent="supervise-and-escalate" -->
````rust

fn watch(
    child: &mut impl Process,
    channel: Option<&Channel>,
    escalation: Escalation,
    mode: Mode,
    observer: &mut dyn FnMut(LaunchEvent),
    lend: &mut dyn FnMut(),
) -> Result<Watched, Failed> {
    let started = Instant::now();
    let mut watch = Watch::Running;
    let mut interrupted: Option<i32> = None;
    let mut escalated = false;
````
<!-- /fragment -->

Two of the three locals are latches rather than state, and the difference matters
for what the loop can report. `watch` moves between four values many times.
`interrupted` and `escalated` are each written at most once and never cleared,
because each records *that something happened*, not that it is still happening —
the child ended by an escalation is gone by the time the value is read, and so is
the launcher's chance to un-receive a signal. The start time is `watch`'s,
and its duration stops at the reap, including the group kills before it but
excluding the observer, terminal recovery and group confirmation afterwards.

What the loop actually observes is three things — the child, the channel and the
latch — and those three locals are where each is recorded or timed. The table
below is the crate's complete account of how a launch can end, and its fourth row
is the one with no mechanism behind it.

| What the loop observes | Where it is polled | The ending it produces | Held by |
|---|---|---|---|
| the child has exited | `child.exited()` | `End::Exited`, signalled or not | `a_child_that_never_signals_ends_unsignalled`, `an_unsignalled_child_runs_to_its_own_exit_untouched`, `a_child_that_signals_and_exits_inside_the_grace_is_never_touched` |
| the channel has appeared | `channel.appeared()` | `End::Escalated`, once the grace has run out | `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` |
| the launcher was signalled | `take_interrupt()` | `End::Interrupted { signal }` | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| **the interactive child finished its turn and never said so** | nowhere | **none — the launch stalls** | nothing, because there is nothing to hold |

<a id="the-poll"></a>
## Two questions asked every tick

The loop's first act is not about the child at all. It lends the terminal,
through the closure chapter 3's lease supplied.

<!-- fragment «run-watch-lend» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1209-1211" parent="supervise-and-escalate" -->
````rust

    loop {
        lend();
````
<!-- /fragment -->

The spawn's handover ran once, at a moment when the launcher may have had no
terminal to give — a launcher started in the background has none, because it is
not the foreground group of one. Asking again every tick is what lets `grove &`
followed by `fg` hand the terminal on to the job that is actually running under
it, minutes after the spawn decided there was nothing to hand over. The guard is
the spawn's own — *is this launcher the terminal's current owner* — so the loop
can ask an unconditional question every 500ms without ever handing over a
terminal it does not own. The lease's section in chapter 3 reads the guard and
the attributes it saves.

<a id="forwarding"></a>
## Forwarding the signal that was actually sent

Cancellation is checked next, before the wait, so an already-exited child cannot
hide a pending interrupt. How a cancellation reaches the child depends on which
launch function is running, so supervision is told.

<!-- fragment «run-mode» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1023-1031" parent="supervise-and-escalate" -->
````rust

/// Which of the three launch functions this is, as far as supervision cares:
/// how a cancellation reaches the child.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Interactive,
    Noninteractive,
    Confined,
}
````
<!-- /fragment -->

Three values, one per launch function, and nothing else reads them: the block
that checks the latch is their only consumer.

<!-- fragment «run-watch-forward-interrupt» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1212-1230" parent="supervise-and-escalate" -->
````rust

        // Check cancellation before accepting even an already-exited child.
        // In nested launches the outer supervisor may have only a short grace
        // left, which is why a confined child is killed at once.
        if interrupted.is_none() {
            if let Some(signal) = take_interrupt() {
                interrupted = Some(signal);
                if mode == Mode::Confined {
                    child.signal(libc::SIGKILL);
                    watch = Watch::Killed;
                } else {
                    child.signal(signal);
                    if matches!(watch, Watch::Running | Watch::Signalled(_)) {
                        watch = Watch::Terminated(Instant::now());
                    }
                }
            }
        }

````
<!-- /fragment -->

The two branches are the cancellation modes. An interactive or noninteractive
child is forwarded the signal the launcher was sent, and its group then gets the
kill-grace, because a noninteractive child may be a supervisor of its own that
needs the signal and the time to end its child. A confined child is sent SIGKILL
at once and the watch goes straight to `Killed`, because its launcher may be a
nested supervisor already inside its own caller's grace. The private test
`cancellation_forwards_or_kills_by_mode` traces all three modes, and
`tests/noninteractive.rs` drives the two detached ones end to end: a cooperative
noninteractive child dies of the forwarded signal well inside a ten-second
kill-grace, a stubborn one is killed only after its kill-grace, and a confined
one is killed at once.

Forwarding a fixed SIGTERM would lose information: forwarding SIGTERM for a
SIGHUP would tell a child that its terminal had not gone away when it had. That
is the fourth place the number rather than the fact is load-bearing, after the
latch, `End::Interrupted` and `reraise`, and it is the one where the wrong choice
is silently wrong — the child still dies, and it dies believing something false
about the world it dies in.

Two guards bound the block, and they answer different questions. The outer
`interrupted.is_none()` makes collection happen at most once per launch, so a
second signal to the launcher is not a second forwarding. The inner
`matches!(watch, Watch::Running | Watch::Signalled(_))` protects the deadline:
writing a fresh `Terminated(Instant::now())` over one already counting down would
**extend** the child's life by a full `kill_grace`, so a supervisor trying to
hurry a stuck teardown along would be told to wait longer. Writing it over
`Killed` would be worse, re-arming a clock for a SIGKILL already sent. These are
the only places `watch` is assigned outside the `match` below, and the guard is
what keeps the writers from disagreeing about which clock is running.

Note what is *not* set here: `escalated` stays false. The launch is being ended
by the launcher's own death, not by the escalation, and `supervise` turns that
pair into `End::Interrupted`. Phase 4 of
`an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` is the
interactive form of this block under test — it raises SIGTERM as soon as the
launch reports `Started`, with a child that loops forever, and asserts that the
ending names the signal, that the child's own status carries the same number, and
that the latch is empty afterwards so the next launch is not stopped by an
interrupt already reported.

The wait comes next, and it is a question, not a reap.

<!-- fragment «run-watch-exited» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1231-1256" parent="supervise-and-escalate" -->
````rust
        match child.exited() {
            Ok(true) => break,
            Ok(false) => {}
            Err(error) => {
                // The child's state is now unknown, and returning here would
                // leave an interactive one holding the terminal with nothing
                // left to reap it. Ending it is the last thing this launch can
                // still do correctly, so it does that before reporting.
                child.signal(libc::SIGKILL);
                let status = child.reap().ok();
                if status.is_some() {
                    observer(LaunchEvent::Reaped);
                }
                return Err(Failed {
                    error: LaunchError::new(format!(
                        "cannot wait on the launched child: {error}; it has been sent SIGKILL and {}",
                        if status.is_some() {
                            "reaped"
                        } else {
                            "could not be reaped — check for an orphaned process"
                        }
                    )),
                    status,
                });
            }
        }
````
<!-- /fragment -->

An exit breaks out of the loop to the end of the group below. A failed query
leaves the child's state unknown, and returning the error directly could leave
an interactive one holding the terminal with nothing left in the process tree to
reap it. So the launch does the last thing it can still do correctly — SIGKILL
the group, try to reap — and only then reports, handing back the status if the
reap succeeded so the terminal can still be returned by the right rule. The
message is built to the same shape chapter 1 read off the error type and chapter
3 applied to the failed spawn: it names what went wrong, carries the operating
system's own words, and ends by naming what the reader must do, which here is
either nothing or *check for an orphaned process*, depending on which of the two
the launcher managed. It is the one branch in this crate that reports a partial
failure rather than a clean one, and the wording is what makes the difference
legible.

<a id="the-escalation-runs"></a>
## Where the escalation actually runs

The state machine is the last thing each tick does, and it is where the
channel's appearance becomes a deadline and the deadline becomes a signal.

<!-- fragment «run-watch-escalation» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1257-1285" parent="supervise-and-escalate" -->
````rust

        watch = match watch {
            Watch::Running if channel.is_some_and(Channel::appeared) => {
                Watch::Signalled(Instant::now())
            }
            Watch::Signalled(at) if at.elapsed() >= escalation.grace => {
                // `escalated` is latched *here*, where the escalation actually
                // runs, and not where the channel appeared. `End` would
                // otherwise be telling the caller only what `signalled` already
                // tells it, while claiming something stronger: that this launch
                // had to be ended. A child that signals and then exits inside
                // its own grace was never touched, and says so.
                escalated = true;
                child.signal(libc::SIGTERM);
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= escalation.kill_grace => {
                child.signal(libc::SIGKILL);
                Watch::Killed
            }
            other => other,
        };

        std::thread::sleep(if matches!(watch, Watch::Killed) {
            KILLED_POLL_INTERVAL
        } else {
            POLL_INTERVAL
        });
    }
````
<!-- /fragment -->

The comment inside the second arm explains a placement decision. `escalated` is
latched where the escalation runs, not where the channel appeared. Chapter 3
stated the consequence as a property of the type — `End::Escalated` is narrower
than *the channel appeared* — and this is the line that keeps it. Had the latch
been set in the first arm, `end` would have told the caller only what
`signalled` already told it, while claiming something stronger: that this launch
had to be ended. `a_child_that_signals_and_exits_inside_the_grace_is_never_touched`
is the case that separates them, running under a thirty-second grace so the
child's own exit lands well inside it; it comes back `End::Exited` *with* a
signal, and asserts
`elapsed < grace` so that a passing run cannot be one that waited the grace out.

The three arms are guarded by conditions rather than by state alone, which is why
`other => other` is needed and is not a default: `Running` with no file, either
timed state before its deadline, and `Killed` all mean *nothing to do this tick*.
Read downward, the arms are the escalation in order — appearance starts a clock,
the grace expires into SIGTERM and a second clock, the kill grace expires into
SIGKILL — and each transition records the `Instant` the next one is measured
from.

Both kill arms now end the same way, and that is new. Each sends its signal and
the loop **continues**, so the exit is observed on a later tick, unreaped, like
every other exit. SIGTERM is a request and a child may decline it, which is
precisely the case the third arm exists for. SIGKILL cannot be declined, and the
supervisor used to reap straight after it with a blocking wait. That wait would
have reaped before the group was killed, so it went. In its place the watch
enters `Killed` and polls on a much shorter interval, because the child is dying,
not deciding.

<!-- fragment «run-killed-poll-interval» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1018-1022" parent="supervise-and-escalate" -->
````rust

/// How often the supervisor checks for the child's exit after the
/// escalation's SIGKILL. The child is dying, not deciding, so this is much
/// shorter than [`POLL_INTERVAL`].
const KILLED_POLL_INTERVAL: Duration = Duration::from_millis(10);
````
<!-- /fragment -->

`a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` is the test that
walks both arms, with `trap '' TERM` in the child, and it asserts on
`status.signal() == SIGKILL` and on `elapsed >= grace + kill_grace`.

The sleep closes the tick, and `POLL_INTERVAL` is the constant chapter 3 read as
*not a knob*. Its consequence is visible in the tests rather than in the code,
and it is arithmetic over the constant. Every observation the loop makes is
quantised to a tick: the appearance is seen up to half a second after it appears, each
deadline is noticed up to half a second after it passes, and the child's death is
noticed up to half a second after it happens. So `elapsed` carries slack the two
steps of the escalation do not separate.
`a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` therefore
uses `elapsed >= grace` as a lower bound only, and asserts
`status.signal() == SIGTERM` to say *which* step actually ran. A test that tried
to tell the two apart by timing would be measuring the poll interval.

<a id="the-group-ends"></a>
## The group ends with the launch

The loop has broken out on an exit, and the child is a zombie. What remains is
steps 2 and 3 of the end, and they are the reason the wait was a question.

<!-- fragment «run-watch-end-the-group» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1286-1307" parent="supervise-and-escalate" -->
````rust

    // A child ended by the escalation exits non-zero, or by signal. That is
    // the normal completion path, not a failure: the caller judges the work;
    // the runner reports channel appearance and process status separately.
    child.kill_group();
    let status = child.reap().map_err(|error| Failed {
        error: LaunchError::new(format!("cannot reap the exited child: {error}")),
        status: None,
    })?;
    // Freeze the launch at the reap, before an observer, terminal recovery or
    // group confirmation can delay it. Later signals belong to the caller.
    let at_reap = take_interrupt();
    let elapsed = started.elapsed();
    let interrupted = interrupted.or(at_reap);
    observer(LaunchEvent::Reaped);
    Ok(Watched {
        status,
        interrupted,
        escalated,
        elapsed,
    })
}
````
<!-- /fragment -->

The comment above the calls separates the caller's outcome policy from the
runner's process report. A child ended by escalation exits non-zero or by signal;
that alone is no failure of supervision. `run` returns `Ok` with separate
`end`, `status` and `signalled` fields. A child that exits 3 without signalling
returns `End::Exited`, `status.code() == Some(3)` and `signalled: false`.

**Why the group is killed at every ending, not only after an escalation.** A
member that ignored the TERM outlives a child that exits on it. A member still
working outlives a child that exits within the grace, or on its own with no
escalation at all. Any caller that relaunched or published on that ending would
act beside it, and in Grove's case a surviving `grove-llm` would go on holding
shared epoch admission. Two alternatives were weighed and declined. A second
grace for the survivors would lengthen every ending, and a child that wants a
descendant to finish already has the place to wait for it: its own shutdown.
Reaping first and signalling afterwards would signal a group ID that the reap had
just released, which the system may already have handed to somebody else.
`a_term_ignoring_descendant_is_gone_before_the_launch_returns` runs all three
endings. Each child starts a TERM-ignoring descendant, then exits on the
escalation's TERM, within the grace, or on its own. The test checks the
descendant at the return boundary with no grace at all, and finds it gone every
time, with the child's own exit code still the launch's. Disabling the group kill
turns that test red.

The reap sample is the adjacent latch read after `reap` returns. It consumes
any newly noted signal even when an earlier cancellation already decided the
ending; `interrupted.or(at_reap)` retains that earlier signal. A signal delivered
between the kernel's reap and this read counts in the sample, while one delivered
after it belongs to the caller. `elapsed` is captured next, before the observer
can delay it. The private tests `the_reap_sample_drains_a_second_cancellation`,
`a_signal_after_the_reap_does_not_cancel_the_reaped_launch` and
`duration_stops_at_the_reap_before_observation_and_recovery` distinguish those
boundaries by injecting signals or delays at the process/observer/reclaim seams.

The calls go through the private seam that lets tests stand in for a real child.

<!-- fragment «run-process-seam» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1032-1051" parent="supervise-and-escalate" -->
````rust

/// The launched child as supervision sees it: a process that can be watched,
/// signalled and reaped, and the group it leads.
///
/// A private seam, so tests can force wait errors and trace the order of the
/// end without faking launch events.
trait Process {
    /// Whether the child has exited, **leaving it unreaped**. A stop is not an
    /// exit.
    fn exited(&mut self) -> std::io::Result<bool>;
    /// Reap the child, blocking, and answer its status.
    fn reap(&mut self) -> std::io::Result<ExitStatus>;
    /// Signal the whole group, then the child itself.
    fn signal(&mut self, signal: i32);
    /// SIGKILL what remains of the group, twice with a pause, while the
    /// unreaped child still reserves its ID.
    fn kill_group(&mut self);
    /// After the reap, and only querying: whether the group is gone.
    fn confirm_gone(&mut self) -> Group;
}
````
<!-- /fragment -->

Five methods, and the two that matter are the split of what a plain wait does
in one call: `exited` observes the exit, and `reap` collects it. The real child
implements them with the system calls below.

<!-- fragment «run-process-for-child» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1052-1107" parent="supervise-and-escalate" -->
````rust

impl Process for Child {
    fn exited(&mut self) -> std::io::Result<bool> {
        // WNOWAIT observes the exit without releasing the child's PID, which
        // is also its group's ID. The rest of the group is killed before the
        // reap, so the ID cannot have been reused by the time the signal lands.
        // SAFETY: initialized siginfo, this process's own child, no reap.
        let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
        let result = unsafe {
            libc::waitid(
                libc::P_PID,
                self.id(),
                &mut info,
                libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
            )
        };
        if result != 0 {
            return Err(std::io::Error::last_os_error());
        }
        // SAFETY: `waitid` filled `info`; `si_pid` is zero when nothing was
        // reported.
        if unsafe { info.si_pid() } == 0 {
            return Ok(false);
        }
        // macOS reports a stopped child here despite WEXITED alone, and goes on
        // reporting it on every poll until it is continued. A stopped child is
        // not an exit: it must be neither reaped nor have its group killed.
        Ok(matches!(
            info.si_code,
            libc::CLD_EXITED | libc::CLD_KILLED | libc::CLD_DUMPED
        ))
    }

    fn reap(&mut self) -> std::io::Result<ExitStatus> {
        Child::wait(self)
    }

    fn signal(&mut self, signal: i32) {
        kill(self.id() as libc::pid_t, signal);
    }

    fn kill_group(&mut self) {
        let pgid = self.id() as libc::pid_t;
        // SAFETY: `kill(2)` on the group the unreaped child still leads. A
        // failure is ignored: ESRCH means nothing is left, and EPERM a group
        // holding only zombies.
        unsafe { libc::kill(-pgid, libc::SIGKILL) };
        std::thread::sleep(SECOND_KILL_PAUSE);
        // SAFETY: as above; the child is still unreaped.
        unsafe { libc::kill(-pgid, libc::SIGKILL) };
    }

    fn confirm_gone(&mut self) -> Group {
        confirm_gone(self.id() as libc::pid_t)
    }
}
````
<!-- /fragment -->

`exited` is the wait that does not reap: `waitid` with `WNOWAIT` reports the
exit and leaves the child a zombie, and the zombie's pid is what keeps the group's
ID from being reused. **The `si_code` check is a measured platform fact.** macOS
reports a *stopped* child here despite `WEXITED` alone, and keeps reporting it on
every poll until the child is continued, after which the real exit is reported.
A supervisor that believed it would kill the group of, and reap, a child that was
only paused. `a_stopped_child_is_neither_reaped_nor_killed` stops its child with
SIGSTOP, holds it stopped across several poll ticks, checks that the launch has
not returned, and then continues it; the child must finish its own script and
exit 4. Treating every report as an exit turns that test red.

`kill_group` sends SIGKILL to the group twice, and the pause between the two is
the second measured fact.

<!-- fragment «run-second-kill-pause» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="999-1008" parent="supervise-and-escalate" -->
````rust
/// How long the runner waits between the two SIGKILLs it sends what remains of
/// the child's group.
///
/// **Two kills, because one is measured to miss.** On macOS a member that is
/// forking while the group's SIGKILL lands can leave a new process that the
/// kill never reached. In a tight fork loop that happened in about two runs in
/// three; Linux restarts the fork instead. The second kill, sent while the
/// unreaped child still reserves the group's ID, reaches that process. The
/// pause only has to outlast one fork.
const SECOND_KILL_PAUSE: Duration = Duration::from_millis(20);
````
<!-- /fragment -->

The race is in the kernel's fork: a member forking as the group's SIGKILL lands
can leave a new process the kill never reached. Linux restarts the fork instead,
so one kill would do there, and the second costs twenty milliseconds. Both calls
go to `-pgid` alone and ignore their result: there is no single process left to
address, ESRCH means nothing is left, and EPERM means a group of zombies.

The fifth step is a query with a bound.

<!-- fragment «run-group-confirmation» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1009-1017" parent="supervise-and-escalate" -->
````rust

/// How long, after the reap, the runner waits for the system to answer that
/// the child's group is gone.
///
/// Every member has been sent SIGKILL twice by then, so the wait only covers
/// what follows a kill: members dying, and their parent or `init` reaping
/// them. A group of zombies still answers. A member still present after this
/// bound is reported, not waited for.
const GROUP_CONFIRMATION: Duration = Duration::from_secs(1);
````
<!-- /fragment -->

The query itself is a free function, so a test of the seam can stand in for it
and the real child's method is one line.

<!-- fragment «run-confirm-gone» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1108-1131" parent="supervise-and-escalate" -->
````rust

/// Query, without signalling, until the system answers that `pgid` names no
/// process group, for at most [`GROUP_CONFIRMATION`].
///
/// **Only ESRCH confirms.** EPERM answers for a member the launcher cannot
/// signal, and macOS also gives it for a group of zombies, so it is "present"
/// like a success is. A query that hits a reused ID after the reap reads as
/// present too, which fails safe.
fn confirm_gone(pgid: libc::pid_t) -> Group {
    let deadline = Instant::now() + GROUP_CONFIRMATION;
    loop {
        // SAFETY: `kill(2)` with signal 0, the existence probe, which sends
        // nothing.
        if unsafe { libc::kill(-pgid, 0) } == -1
            && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
        {
            return Group::Gone;
        }
        if Instant::now() >= deadline {
            return Group::Present { pgid };
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
````
<!-- /fragment -->

**Only ESRCH confirms**, and that is the third measured fact: `kill(-pgid, 0)`
answers EPERM for a member the launcher cannot signal, and macOS gives EPERM for
a group of zombies too. So EPERM counts as present, exactly as a success does. A
killed member is a zombie until its parent, or `init` once its parent is gone,
reaps it, which is why the confirmation needs a bound rather than a single query.
A second is ample for that, and a member still present after it is reported, not
waited for: `Group::Present` goes back beside the child's status. A query that hits
a reused ID after the reap reads as present too, which fails safe.

<a id="the-whole-group"></a>
## The whole group, and then the child

Every escalating `kill` on this page has gone through one function whose body is
two calls, and its twenty lines of comment are the record this chapter keeps.

The final test-module declaration loads the private supervision tests from
`tests/internal/wait_events.rs`, outside the production-source corpus. They drive
`supervise` through the `Process` seam with a fake child and trace the order of
the end on every wait path, the survivor report, and the three cancellation
modes. Real launch tests cover immediate exit, failed spawn, appearance-before-exit,
both escalation stages, the end of the group, and interrupts through the same
observed entry point.

<!-- fragment «run-kill» owner="the-launchers-job" source="crates/keyed-launch/src/run.rs" lines="1308-1340" parent="supervise-and-escalate" -->
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
/// `pgid` is the child's pid, made a group leader by the `process_group(0)`
/// [`run`] sets before the spawn, or by the `setsid` a detached launch makes —
/// a failure there is a failed spawn, and no child. A group with that id can
/// only have been created by that process, so `-pgid` cannot name an unrelated
/// job even in the impossible case where the group was never created; the
/// direct `kill` behind it covers that case. Every call is made before the
/// child is reaped, while its pid still reserves the id.
///
/// A failure is ignored on purpose — ESRCH means the process exited between the
/// poll and the signal, which the next poll reports anyway. This is the shell's
/// `kill … 2>/dev/null`, written down.
fn kill(pgid: libc::pid_t, signal: libc::c_int) {
    // SAFETY: `kill(2)` on the process group of, and then the pid of, a child
    // of this process.
    unsafe {
        libc::kill(-pgid, signal);
        libc::kill(pgid, signal);
    }
}

#[cfg(test)]
#[path = "../tests/internal/wait_events.rs"]
mod wait_events;
````
<!-- /fragment -->

Two calls, in that order, and the first is the one that makes the escalation
worth having. A grandchild the child spawned — a tool subprocess, a language
server, an agent's own in-flight command — is a member of the child's process
group and is reaped with its parent rather than surviving it. The comment names
the cost of the alternative in one clause: such a grandchild can hold a lock its
launcher's caller is about to wait on, and then the escalation's SIGKILL causes
a stall rather than a teardown. That is the book's question seen from the far
end — a launcher that ended the process it could see and inferred that the job
was over. The end of the group above is the same answer applied after every
exit, not only after an escalation.

This function is where *the launched child is a job* is kept, and the record
settles which of the child's identities changes. The child gets a process group
of its own so that the group can be signalled. In interactive mode it remains
in the existing session,
because a session leader has no controlling terminal — which makes the handover
fail outright and leaves an interactive child reading the terminal in competition
with the launcher rather than stopped by it. Chapter 3 owns the
spawn side of that record — the group, the terminal, the dispositions — and this
function is its other half: signalling `-pgid` is the reason the group exists at
all. The record weighs four alternatives and rejects every one; interactive handover rules out a new session, while group cleanup rules out
an escalation addressed to the pid alone. Noninteractive mode intentionally
uses a new session because it must receive no controlling terminal.

`the_escalation_reaps_the_childs_descendants` is the test that observes the
escalation's reach. Its child starts a background `sh` loop, writes that
grandchild's pid to a file, signals, and then goes on waiting, so the grace runs
out and the SIGTERM step fires — the child has no `trap`, so it dies there and
SIGKILL is never reached. The assertion that the grandchild is gone is only half
of it. The test also spawns a **bystander** — the same shape of process, started
at the same moment, in the *test process's* group rather than the child's — and
asserts it is untouched. Without that control a fixture that reported "gone" for
any pid, a `kill(2)` probe misreading its errno, would pass identically. The pair
is what makes the claim *the group and only the group* checkable rather than
merely observed.

The third paragraph names where the group leadership comes from, and it names
`process_group(0)` rather than the parent's `setpgid` because chapter 3 measured
that spawn: installing a `pre_exec` closure takes `std` off its `posix_spawn`
fast path, `Command::spawn` returns only once the child has already `execve`d,
and the parent's `setpgid` fails `EACCES` every time — thirty of thirty, with
controls showing that the same call can return success and can return `ESRCH`.
[The measurement and its controls](03-the-job.md#the-latch-and-the-child-away)
are chapter 3's; what matters here is that the comment's **conclusion** never
depended on which call created the group. A process group with that id can only
have been created by that process, so `-pgid` cannot name an unrelated job, and
the direct `kill` behind it covers the case the comment calls impossible — a
case that is now unreachable twice over, since a `process_group(0)` that failed
would have failed the spawn and left no child to signal. The paragraph's last
sentence is the end of the group's rule stated for the escalation: every call is
made while the child is unreaped.

The last paragraph is the one that turns a discarded return value into a
decision. `ESRCH` means the process exited between the poll and the signal, which
the next poll reports anyway, so there is nothing for a caller to do with the
failure and nothing for the loop to change. Naming it as *the shell's
`kill … 2>/dev/null`, written down* is what separates it from a suppressed
warning: the shell discards the same failure for the same reason, and a launcher
that propagated it would be reporting the child's normal exit as an error.

That is `src/run.rs`. A path was drawn and written by nobody, an argv arrived
exactly as its caller built it, a
child was spawned into a job with nothing added, and a launcher waited for one of
three things and acted on whichever came first. Then it ended the child's whole
group, took the terminal back as it was lent, and asked whether anything
survived. At no point did the crate decide that the child was finished. The
result is an `Ended` naming who acted, what remained of the group and, whether the
child created its completion channel.

Chapter 5 reads how this is checked: the ten tests inside `src/channel.rs`
that reach a function no integration test can, and the suites under `tests/`
that the book cites and does not reproduce.

[Previous: The child is a job](03-the-job.md) | [Contents](README.md) | [Next: How this is checked](05-how-checked.md)
