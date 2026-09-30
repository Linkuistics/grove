# dispatched-terminal-k33

## Goal

Prove under a controlling PTY that a dispatched session behaves as Grove's
foreground job. Identity, terminal, cwd, native exits and signal state reach
the harness intact. Cancellation during selection or execution follows Grove's
job contract.

## Context

The spec's controlling-PTY row lists the cases. Grove's existing PTY-driven loop
tests show how the foreground job, grace, TERM and KILL escalation are
observed. The signal semantics under test were built in
`evaluation-boundary-k27`. This leaf observes them from Grove's side.

## Done when

- The dispatched fake harness reports the PID and process group Grove launched,
  the intended cwd, and the controlling terminal as its foreground group. Its
  native exit code and a native signal death reach Grove unmodified.
- The fake harness observes Grove's entry signal mask and dispositions,
  including SIGPIPE, unchanged.
- The policy worker observes null stdin and a scrubbed control environment.
  The final harness observes the fresh completion channel.
- Interrupting through the PTY during a deliberately slow selection launches
  nothing and leaves no worker. Grove's response to the ended child is the
  existing one. Interrupting during execution, and Grove's descendant
  escalation, behave as they do for a direct harness.
- Positive controls: each observation is seen to change when the fixture
  deliberately alters it, so that no case can pass without being exercised.

## Decisions (running log)

**The cases join `crates/grove/tests/loop_driver.rs`, beside k32's.** They
extend its `Dispatch` fixture with a driver that takes a pseudo-terminal as
its controlling terminal (`setsid` then `TIOCSCTTY`), so keyed-launch's
foreground handover really runs. The existing suite detaches its driver
(`grove_driver`, null stdin), which is right for every other case and is kept.
The master side stands for the human: writing `^C` to it is the interrupt.

**The harness is a C probe of Grove's own, not dispatch's `signal-probe.c`.**
It must be C for the reason dispatch's is: a shell rewrites its dispositions
as it starts, and a Rust runtime ignores SIGPIPE before `main`. It lives at
`crates/grove/tests/support/session-probe.c` rather than reaching into
`crates/harness-dispatch/tests/`, so Grove's suite does not depend on a test
file across the package boundary an extraction would cut. It reports its
PID, group, the terminal's foreground group, its stdin's terminal, its cwd and
its signal state, then execs a shell step that records the channel, run
identity and epoch and does the case's action. Exec keeps everything the probe
observed, so that step's exit or signal death is the harness's own.

**"As for a direct harness" is measured against a direct launch of the same
probe under the same Grove**, as k32 measured the prompt. Grove's entry state
is what Grove's launch hands its configured command. Read from the pinned
toolchain's source (rustc 1.98.1,
`library/std/src/sys/process/unix/unix.rs`, `do_exec`): std keeps the calling
thread's mask and resets only SIGPIPE to default, before `pre_exec` hooks.
keyed-launch then resets its seven terminal signals. So the reference is not a
constant, the driver starts with SIGUSR1 ignored and SIGUSR2 blocked, and the
direct reference must show both.

**The PID Grove launched comes from the process table**, as the driver's only
child while the session is held on a marker. Grove records no PID, and the
probe's own claim about itself is what is under test.

**One altered dispatched run is every identity, cwd, terminal and signal
control.** The probe's `--alter` forks, and the child leaves the group,
swaps its stdin for `/dev/null`, changes directory, flips SIGPIPE and blocks
SIGALRM before it reports. The same run starts the driver with neither
SIGUSR1 ignored nor SIGUSR2 blocked, and grants the worker `GROVE_SIGNAL_FILE`.
The entry and the alteration touch disjoint signals, so each change is
attributable. The worker's null stdin cannot be altered from outside the
front, so its control is the same measurement applied to `/dev/tty`, which it
reports as a terminal.

**Each driver run gets a fresh terminal, and the master is always read.**
Both are macOS kernel behaviour met in the first run. A session leader's exit
revokes its controlling terminal, so a terminal cannot serve a second driver.
The same exit first drains the terminal's output, and the echo of a typed
Ctrl-C is output, so an unread master left the driver in state `E` forever,
and `finish_within`'s own kill could not reap it. `Pty::open` starts a reader
thread, as a terminal emulator has one.

**Controls seen to fire, by hand**, beside the permanent ones in the tests.
A descendant that ignores SIGTERM survived the escalation, and the escalation
case failed on it. With `TIOCSCTTY` removed, the driver owned no terminal, and
the identity case failed on the direct reference's foreground group (`-1`).
That run's report also showed the reference is not a constant: SIGUSR1 (30)
ignored and SIGUSR2 (31) blocked, as the driver started, and SIGPIPE default.

**Done-when instruments**, all in `crates/grove/tests/loop_driver.rs`, which
together are the spec's controlling-PTY row:

- The PID and group Grove launched, the cwd, the terminal with the harness's
  group in the foreground; Grove's entry mask and dispositions, SIGPIPE's
  included, equal to a direct harness's; the worker's null stdin and scrubbed
  environment; the fresh channel; and every control:
  `a_dispatched_harness_is_the_foreground_job_grove_launched`.
- The native exit code, one dispatch itself uses for a refusal, and a native
  signal death:
  `a_dispatched_harness_s_exit_and_signal_death_reach_grove_as_a_direct_one_s`.
- An interrupt during a slow selection launches nothing, records nothing and
  leaves no worker, and Grove reports the same status as for a direct harness
  interrupted as it runs; the same hold, ended, launches and is interrupted as
  it runs:
  `an_interrupt_typed_at_the_terminal_ends_a_dispatched_job_as_it_ends_a_direct_one`.
- The escalation reaps a dispatched session's descendant as a direct one's,
  beside a bystander:
  `the_escalation_reaps_a_dispatched_session_s_descendants_as_a_direct_one_s`.

**A keyed-launch flake is externalised, not fixed here.** The first full
`task check` failed once in
`standalone_child_has_a_new_session_and_cannot_consume_callers_stdin`, with
EPIPE on the parent's write to its supervisor's stdin. It touches nothing this
leaf changed, and passed 15 times on rerun. It is
`noninteractive-stdin-flake-k55`, at the grove root, ahead of the finish
sequence's release.

**No in-session reviewer**, as in k32. These are tests of behaviour
`evaluation-boundary-k27` built and its node review covered, and each claim
has an assertion whose control was seen to fire.
