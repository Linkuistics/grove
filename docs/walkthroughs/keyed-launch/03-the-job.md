# The child is a job
<!-- book-page id="the-job" slice="nothing-else-added" order="3" -->
[Previous: Appearance is the event](02-the-channel.md) | [Contents](README.md) | [Next: The watch and the escalation](04-the-escalation.md)

<a id="nothing-else-added"></a>
## The interactive launch contract

Chapter 1 read an `Argv` whose words nothing changes once it is built, and chapter 2
produced a path nothing has written to. This chapter is where the two meet. `run`
takes both, spawns the program the argv names, and hands the child one
environment variable holding one path — and that is the whole of what it adds.

What this stage must not add and must not interpret is **the launch itself**: no
argument, no flag, no variable the operator did not write. The claim is easy to
state and easy to lose, because a launcher is exactly the layer where additions
look like helpfulness. A `--yes` because the child is non-interactive. A `HOME`
because the child seemed to want one. A working directory because none was given.
Each is a value the caller cannot see in the argv it built, and each is the
launcher deciding what the child is for.
`a_scrubbed_variable_is_removed_from_an_inherited_environment` and
`a_caller_built_argv_is_spawned_whole_and_directly` are where the crate is held
to the claim from the two directions it could fail in.

The file is `src/run.rs`, 1,171 lines, and it splits between
this chapter and the next **by whose signal it is**. Everything done *to* the
child is here: the shape of a launch, the dispositions it is handed, the terminal
it is lent and how that terminal comes back, and the spawn that puts it in a
process group of its own. Everything about *endings* is chapter 4's: the
supervisor's state machine, the launcher's own signals, the escalation, and the
end of the child's group. This chapter owns lines 1–146 and 333–837; chapter 4
owns the 147–332 between them and the 838–1171 after. The 651 lines here include
detached-mode setup and descriptor isolation, because those establish the
process the watch owns.

Read the source closely on this page. The source comments explain the order
and purpose of the process operations: the escalation, the child's
dispositions and the terminal each carry a full case in situ, stating the
alternative and what it would have cost. The fragments below reproduce them
exactly, and the prose between the fragments does two things and no third —
it connects an argument in one item to an argument in another, and it names the
test that holds each claim.

<a id="the-spawn"></a>
## The spawn

This section takes step 3 of the four-step trace chapter 1 wrote and runs it to
the point where a child is running. The half after that — the poll, the
grace, the escalation and the value that comes back — is chapter 4's; this
one ends with a process, not with an `Ended`.

Everything the call needs already exists. The `Argv` is chapter 1's, built by
its caller with `Argv::new`; the `Channel` is chapter 2's, drawn in grove's
control directory and written to by nobody. grove supplies the four remaining
fields, and every one of them is grove's rather than this crate's.

```text
run(Launch {
    argv:        ["claude", "--model", "opus", "<the mandate>"],
    channel:     Channel { path: "/work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281" },
    channel_var: "GROVE_SIGNAL_FILE",
    scrub:       ["GROVE_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID"],
    cwd:         Some("/work/atlas"),
    escalation:  Escalation { grace: 2s, kill_grace: 5s },
})
```

The observable end of this chapter's half is a process, and it is worth writing
down as a picture rather than as a return value, because most of what is true of
it is true of no ordinary `Command::spawn`.

```text
launcher  pid 4100  pgid 4100
  child   pid 4137  pgid 4137   ← its own group, and now the terminal's foreground

  argv     claude --model opus <the mandate>
  cwd      /work/atlas
  env      the launcher's own, minus GROVE_HARNESS_PID and GROVE_CLAUDE_PID,
           plus GROVE_SIGNAL_FILE = /work/atlas/.jj/grove/signal-3f9c…e281
  signals  SIGINT SIGQUIT SIGTERM SIGHUP SIGTSTP SIGTTIN SIGTTOU
           at their default dispositions, whatever the launcher's were
```

Four things in that picture are this chapter's, and each is one of the sections
below. The child is **in a process group of its own**, and holds the terminal
whenever the launcher had one to give, so a Ctrl-C the human types reaches the
child rather than the launcher. Its environment is the launcher's **minus a list
and plus one entry**. Its signal dispositions are the **defaults**, whatever the
launcher's happened to be. And its `argv` is the four words the caller built,
with nothing appended.

Two things about that environment are worth reading before the source, because
both look wrong at first glance. `GROVE_SIGNAL_FILE` is in the scrub list and is
nonetheless set: the list names the launch-control variables a nested launcher
must not inherit, the channel variable is the first of them, and the grant is
exactly the exception that this ordering preserves.
`granting_the_channel_survives_a_scrub_list_that_names_it` is what holds the two
in that order. And nothing in the picture names the variables the child *does*
receive — its `PATH`, its `TERM`, its locale — because they are not
granted at all; they arrive by inheritance, and *minus a list* is the only
operation performed on them.
`a_scrubbed_variable_is_removed_from_an_inherited_environment` is what makes that
checkable, and the field's own section below reads how.

**Five names appear in this chapter's source and are explained in chapter 4.**
The spawn's first acts are `install_termination_handler(…)` and
`restore_child_watching()`, it stores a zero into `INTERRUPTED_BY` just before
the spawn, and it ends by calling `supervise(…)` with a `Mode`. The minimum
needed here is that `INTERRUPTED_BY` is a process-global latch holding the
number of a cancelling signal the *launcher* received;
`install_termination_handler` is what puts a handler behind it, for every
cancelling signal the launcher does not ignore; `restore_child_watching` undoes
an inherited ignored SIGCHLD for the launcher alone and says whether it did;
`Mode` names which of the three launch functions is running; and `supervise` is
the poll loop that watches the child, ends its group and gives the terminal
back. Why the latch is cleared at exactly that point, why it carries a number
rather than a flag, which signals cancel and why an ignored SIGCHLD matters are
chapter 4's, where the handler and the loop are reproduced.

<a id="what-the-blocks-answer"></a>
## What the two blocks answer

This chapter's 651 lines are two blocks with chapter 4's first block between
them: one constant, five types and a module thesis at the top of the file; then a
constant, the terminal wrapper, the launch's lease on it, the public runner
entry points and launch events. The table collects what each answers and what
pins it. Tests named without a path are in `crates/keyed-launch/tests/launch.rs`;
the exceptions name their own files.

| Item | Answers | Pinned by |
|---|---|---|
| `POLL_INTERVAL` | how late an escalation may start | — |
| `Escalation` | how long after the token, and how long after SIGTERM | `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace`, `a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` |
| `Launch::argv` | which program, and which arguments | `a_caller_built_argv_is_spawned_whole_and_directly`, `a_program_that_does_not_exist_names_itself_and_says_what_to_check` |
| `Launch::channel`, `Launch::channel_var` | which path this child signals on, under which name | `the_channel_path_is_published_under_the_callers_chosen_variable_name` |
| `Launch::scrub` | which inherited variables the child must not receive | `a_scrubbed_variable_is_removed_from_an_inherited_environment`, `granting_the_channel_survives_a_scrub_list_that_names_it` |
| `Launch::cwd` | where the child starts | `the_child_starts_in_the_given_directory` |
| `Ended` | which ending, with what status, after how long, what the child said, and what remained of its group | `a_child_that_never_signals_ends_with_no_token` |
| `Group` | whether the child's group was confirmed gone, or is reported beside the status | `a_term_ignoring_descendant_is_gone_before_the_launch_returns` |
| `End::Exited` | the child ended itself, token or no token | `an_unsignalled_child_runs_to_its_own_exit_untouched`, `a_child_that_signals_and_exits_inside_the_grace_is_never_touched` |
| `End::Signalled` | the escalation ended the child | `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` |
| `End::Interrupted` | the launcher was signalled during this launch | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` (`tests/interrupt.rs`) |
| `DEFAULT_DISPOSITION_IN_CHILD` | which dispositions the child gets back at their defaults | `an_ignored_sigint_in_the_launcher_does_not_reach_the_child` |
| `Terminal::open` | is there a controlling terminal to hand over | — |
| `Terminal::foreground`, `Terminal::hand_to` | who owns the terminal, and how ownership moves | `a_launcher_started_in_the_background_takes_no_foreground` (`tests/job.rs`) |
| `Terminal::attributes`, `Terminal::restore` | what the terminal looked like when it was lent, and putting that back | `a_raw_mode_child_killed_by_the_escalation_leaves_the_terminal_restored` (`tests/job.rs`) |
| `own_group` | which group the launcher is in | — |
| `Lease::lend`, `Lease::reclaim` | whether this launch held the foreground, and from which group it takes the terminal back | `a_child_that_exits_normally_leaves_the_foreground_with_another_group`, `after_a_death_by_signal_the_launcher_takes_the_terminal_from_another_group` (`tests/job.rs`) |
| `run` | one child, spawned whole, in a job of its own | every test above |

Two of those rows have no test of their own. `POLL_INTERVAL` bounds a latency
nothing observes, which is the same argument the constant's own comment makes
for it not being a knob. `Terminal::open` and `own_group` are queries that every
terminal test runs through and none isolates. **The terminal rows are pinned on a
terminal the test made.** A launcher under `cargo test` may or may not have a
controlling terminal, depending on how the runner was started, so a test that
relied on the runner's terminal would exercise the handover in whichever form the
runner happened to produce and could assert on neither. `tests/job.rs` instead
re-runs its own binary as the leader of a new session on a fresh
pseudo-terminal, which becomes that launcher's controlling terminal and nobody
else's. There it reads the foreground group and the terminal's local modes before
and after a launch. Chapter 5 reads how that suite is built.

The first composite is the file's opening 146 lines — the thesis, the poll
interval, and the five public types this file defines.

<!-- fragment «launch-shape» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="1-146" parent="source-run" -->
<!-- insert «run-thesis» -->
<!-- insert «run-poll-interval» -->
<!-- insert «run-escalation» -->
<!-- insert «run-launch» -->
<!-- insert «run-ended» -->
<!-- insert «run-group» -->
<!-- insert «run-end» -->
<!-- /fragment -->

The second is the block after chapter 4's, and it is the machinery of a spawn:
the dispositions, the terminal, and `run` itself.

<!-- fragment «terminal-and-spawn» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="333-837" parent="source-run" -->
<!-- insert «run-default-dispositions» -->
<!-- insert «run-terminal-type» -->
<!-- insert «run-terminal-open» -->
<!-- insert «run-terminal-accessors» -->
<!-- insert «run-terminal-hand-to» -->
<!-- insert «run-terminal-attributes» -->
<!-- insert «run-own-group» -->
<!-- insert «run-lease-type» -->
<!-- insert «run-lease-hold» -->
<!-- insert «run-lease-lend» -->
<!-- insert «run-lease-reclaim» -->
<!-- insert «run-the-child-is-a-job» -->
<!-- insert «run-command-and-environment» -->
<!-- insert «run-terminal-handover» -->
<!-- insert «run-process-group» -->
<!-- insert «run-pre-exec» -->
<!-- insert «run-clear-and-spawn» -->
<!-- insert «run-parent-group-and-supervise» -->
<!-- /fragment -->

<a id="a-file-in-two-halves"></a>
## Two verbs, and an interval that is not a knob

The module's own first line names both halves of the file and, with them, both
chapters: *spawning one child directly* is this one, *supervising it until it
ends* is the next.

<!-- fragment «run-thesis» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="1-14" parent="launch-shape" -->
````rust
//! Spawning one child directly and supervising it until it ends.

use std::ffi::OsStr;
use std::fs::File;
use std::io::Write as _;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicI32, AtomicU8, Ordering};
use std::time::{Duration, Instant};

use crate::channel::{Channel, Token};
use crate::error::LaunchError;
````
<!-- /fragment -->

Eight `std` imports and two of the crate's own. Two omissions show the module
boundary. There is no `use libc`: every call into it below
is written out as `libc::…`, so a reader scanning the file can see each place the
crate leaves `std` without following an import to find out. And `Argv` is absent
too, although this file names it — it appears exactly once, fully qualified
as `crate::Argv`, in the field the next-but-one fragment declares. The command
enters this file at that one field and nowhere else, and `Channel`, `Token` and
`LaunchError` are the whole of what it imports from its own crate.

<!-- fragment «run-poll-interval» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="15-22" parent="launch-shape" -->
````rust

/// How long the supervisor waits between checks of the child's liveness and the
/// completion channel.
///
/// Not a knob. The interval only bounds how late an escalation starts, and the
/// escalation's own graces are measured in seconds — a caller tuning this would
/// be tuning latency it cannot observe.
const POLL_INTERVAL: Duration = Duration::from_millis(500);
````
<!-- /fragment -->

`POLL_INTERVAL` is read in one place, chapter 4's `watch`, and is reproduced here
only because it stands at the top of the file. Its argument is the argument for
*not* exposing something, and it is the mirror image of every field of `Launch`
below: those are the caller's because the caller is the only one who can know
them, and this is not the caller's because knowing it would not help. Half a
second against graces measured in whole seconds is the whole of the relation, and
the next fragment is where those seconds are named.

<a id="the-two-waits"></a>
## The two waits

`Escalation` is two `Duration`s, and the fifteen lines above them are where the
crate says why a launcher ends a child at all.

<!-- fragment «run-escalation» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="23-43" parent="launch-shape" -->
````rust

/// The two waits of the kill escalation.
///
/// **The escalation exists because an interactive child is never reaped on its
/// own.** A child that returns to a prompt after finishing its work has not
/// exited and will not: it sits waiting for input that is not coming. The token
/// is the only evidence it is done, and ending it is therefore the launcher's
/// job — which is a job only the launcher can do, since it is the child's own
/// parent process, outside whatever sandbox the child runs under. A child asked
/// to end itself may simply be denied (macOS Seatbelt refuses a same-sandbox
/// process signalling its own session), and denied silently.
///
/// `grace` runs from the token's appearance to SIGTERM, so a child that
/// signalled mid-operation gets to finish that operation and let its own call
/// return. `kill_grace` runs from SIGTERM to SIGKILL, for a child that installs
/// a handler and declines to die.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Escalation {
    pub grace: Duration,
    pub kill_grace: Duration,
}
````
<!-- /fragment -->

This is the consequence of chapter 2's opening fact, arriving one chapter later
as an obligation. Chapter 2 said that an interactive child does not end by
exiting: it finishes its turn and returns to its prompt, so its exit is not an
observable a launcher can wait for. The comment above takes the next step —
if the child will not end itself, somebody has to end it, and the only process
that can is its own parent. Everything else on this page follows from that
sentence, including the process group two sections down, because *ending it*
turns out to mean ending a job rather than a process.

The two `Duration`s are separately named because they answer to different
failures, and each has its own test. `grace` is the wait a child that did its job
is owed: it signalled mid-operation, and the grace lets that operation's own call
return before anything is sent.
`a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` is the case
it exists for — a child that has signalled and then goes on waiting forever
— and it ends `End::Signalled`. `kill_grace` answers a different child
entirely, one that caught the SIGTERM and declined to act on it, and
`a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` is its case.

The type is `Copy` and `Eq` and has no `Default`, which is the same restraint the
whole of `Launch` shows below: a launcher that has not thought about how long to
wait has not been given an answer to copy. grove's own is two seconds and five,
and nothing in this crate knows or could check that.

<a id="everything-one-launch-is"></a>
## Everything one launch is

The next struct combines the values introduced by the last two sections, and its
comment says in one sentence why none of its fields has a default.

<!-- fragment «run-launch» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="44-77" parent="launch-shape" -->
````rust

/// Everything one launch is.
///
/// Every field is the caller's: this crate supplies no default program, no
/// default environment, and no default variable name. What it supplies is that
/// the `argv` is spawned **whole and directly** — no shell, no appended
/// argument, no reordering — and that the child's environment is the caller's
/// own minus `scrub` plus the one channel path.
pub struct Launch<'a> {
    /// The program and arguments, built by the caller with
    /// [`Argv::new`](crate::Argv::new).
    pub argv: &'a crate::Argv,
    /// This launch's completion channel. Its path is published to the child;
    /// its appearance ends the launch.
    pub channel: &'a Channel,
    /// The environment variable the channel path is published under. The name
    /// is the caller's because the child is the caller's: only the two of them
    /// have agreed on it.
    pub channel_var: &'a str,
    /// Variable names removed from the child's inherited environment.
    ///
    /// **Scrubbing is the caller's obligation and this is where it is
    /// discharged.** An environment is inherited, not addressed: a launcher
    /// that merely declines to *set* its own control variables still hands the
    /// child whatever its own environment carried — including, for a nested
    /// launcher, a live channel path belonging to somebody else's launch, which
    /// is authority to end a session nobody meant to grant.
    pub scrub: &'a [&'a OsStr],
    /// The child's working directory. `None` inherits the launcher's, which is
    /// rarely what a launcher wants: it is wherever a human happened to be
    /// standing.
    pub cwd: Option<&'a Path>,
    pub escalation: Escalation,
}
````
<!-- /fragment -->

Six fields, every one borrowed, and the lifetime is what says this is a call's
arguments given a name rather than a value anyone keeps. There is no builder and
no `Default`; a caller that has not decided what to scrub has to write `&[]` and
see itself do it.

The three fields whose comments carry an argument are worth reading against each
other, because each argues about a *different* boundary and together they are the
whole of the chapter's claim.

`argv` argues about **what happens to the words**. It borrows a `crate::Argv`,
which chapter 1 showed has private fields and no method that changes one, so what
reaches this field is exactly what a caller built with `Argv::new`. `Launch` has no other field that could name a program or add an
argument, which is why the promise *no appended argument* is checkable at all
rather than merely intended. `a_caller_built_argv_is_spawned_whole_and_directly`
walks the route, from five words a shell would re-read to the same five words in
the child.

`scrub` argues about **inheritance**, and it is where chapter 2's central
property turns into a hazard. Chapter 2 established that a channel path is
authority: allocation writes nothing, so the file's appearance is the event that
ends a launch, and whoever holds the path can produce that appearance. An
environment is inherited rather than addressed, so a launcher that merely
declines to *set* its own control variable still hands a nested child whatever
its own environment carried — and if the launcher is itself running as a
session, what it carried is a live path belonging to somebody else's launch.
Removing is therefore a different operation from not-setting, and
`a_scrubbed_variable_is_removed_from_an_inherited_environment` is written to tell
them apart: it scrubs one variable the test process really has and checks a
second one arrived untouched, because a fixture that invented both could not
distinguish a removal from an omission.

`cwd` argues about **defaults that look like no decision**. `None` is not a
neutral choice; it is a choice of the launcher's own directory, which is wherever
a human happened to be standing when they typed the command. The field is an
`Option` rather than a required path because the crate will not invent a
directory either, and `the_child_starts_in_the_given_directory` pins the arm a
caller should almost always take.

`channel_var` is the smallest of the six and states the same rule as the rest:
the name is the caller's because the agreement is between the caller and its
child, and this crate is neither. `escalation` is the only field with no comment
at all, because the type two fragments above carries its argument and repeating it
here would put the same case in two places.

<a id="which-of-three-happened"></a>
## Which of three things happened

What comes back from a launch is one struct of five fields, and its comment is
doing the same work `Escalation`'s did — naming, in advance, the distinction a
caller would otherwise collapse.

<!-- fragment «run-ended» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="78-101" parent="launch-shape" -->
````rust

/// How a launch ended.
///
/// `Signalled` and `Exited` both describe a child that is gone; they differ in
/// *who ended it*, which is what a caller needs to distinguish a launch that
/// completed its work from one that fell over. `token` is orthogonal to all
/// three: a child that signals and then exits before the grace elapses ends
/// `Exited` with a token, and is a perfectly ordinary completion.
#[derive(Debug)]
pub struct Ended {
    pub end: End,
    pub status: ExitStatus,
    pub elapsed: Duration,
    pub token: Option<Token>,
    /// Whether the child's process group was confirmed gone after the reap.
    ///
    /// **A caller acts on the ending only beside [`Group::Gone`].** The child
    /// is reaped and `status` is its own whatever this says. But a member of its
    /// group that survived two SIGKILLs is still running: it can hold a lock,
    /// still write an output, or still hold shared epoch admission. A caller
    /// that relaunched, published or reported success beside it would be
    /// acting beside it.
    pub group: Group,
}
````
<!-- /fragment -->

Five fields, and the one to read first is the relationship between `end` and
`token`. `token` is `Option<Token>` — chapter 2's value, read back off
the path — and it is *orthogonal* to `end` rather than a refinement of it.
The reason is that they answer different questions: `token` says whether the
child spoke, and `end` says who ended the child. Both are needed because either
one alone is ambiguous. `a_child_that_never_signals_ends_with_no_token` is the
case where the child exits of its own accord having said nothing, and it also
checks that the channel file does not exist — which is chapter 2's
writes-nothing property observed from the far end of a real launch.

The fifth field, `group`, answers a question the other four cannot: whether
anything of the child's job is still running. The child is reaped by the time
`Ended` exists, so `status` is final. But the child led a process group, and a
member of that group is not reaped with it. Chapter 4 reads the end of a launch
that kills what remains of the group and then asks the system whether the group
is gone. This type is how the answer comes back.

<!-- fragment «run-group» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="102-114" parent="launch-shape" -->
````rust

/// What remained of the child's process group once the launch ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// The system answered that no such process group exists — the only answer
    /// that confirms it.
    Gone,
    /// The group still answered a second after it was killed, or the query
    /// failed in some other way. `pgid` names it for a diagnostic. It was the
    /// child's pid, so once the child is reaped it may be reused: name it, but
    /// never signal it.
    Present { pgid: i32 },
}
````
<!-- /fragment -->

`Gone` and `Present` are the only two answers, and the asymmetry between them is
deliberate. `Gone` is claimed only on the system's own word that no such
process group exists. `Present` covers both a group that still answered after a
second and a query that failed some other way, because the safe reading of
anything short of *no such group* is that something survived. `Present` carries
the group's ID so that a caller's diagnostic can name it. The ID was the child's
pid, so once the child is reaped it can be reused; the comment says to name it
and never to signal it. **The field sits beside `status` rather than replacing
it**, which is the design's reason for a field instead of an error: a caller such
as `harness-dispatch` records what the child did and refuses to act on the
ending, and it needs both halves to do that. Grove's two callers act on
`Present` today, the loop stopping with its leaf live and `grove run`
publishing nothing. `a_term_ignoring_descendant_is_gone_before_the_launch_returns`
pins `Gone` from a real launch. `a_surviving_group_is_reported_beside_the_childs_status`,
a private test of the supervisor, pins that `Present` leaves the child's status
and ending intact.

<!-- fragment «run-end» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="115-146" parent="launch-shape" -->
````rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    /// The child exited of its own accord — whether or not it left a token on
    /// the way out.
    Exited,
    /// The escalation ended the child: its token appeared, the grace elapsed
    /// with the child still running, and it was signalled.
    ///
    /// This is deliberately *narrower* than "a token appeared", which `token`
    /// already reports. A child that signals and then exits inside its own
    /// grace was never touched, and comes back `Exited` with a token.
    Signalled,
    /// The *launcher's* process was sent SIGTERM or SIGHUP — or SIGINT, for a
    /// launch with no terminal — **during this launch**. An interactive or
    /// noninteractive child's group was sent the same signal and, after the
    /// kill-grace, SIGKILL; a confined child's group was killed at once. Either
    /// way it was reaped, never left orphaned onto the terminal. The channel
    /// cannot express this case — an interrupt normally leaves no token at all.
    ///
    /// **The signal is carried rather than merely noted** so a launcher can
    /// report it onward. A process that catches a termination signal, tidies up
    /// and then exits 0 tells its own parent it finished its work; the only way
    /// to say what actually happened is to die of the same signal, and that
    /// needs its number. [`reraise`] is that ending, and this field is its
    /// argument.
    ///
    /// A signal arriving *between* launches is not this: `run` discards it, and
    /// [`take_interrupt`] is where a looping launcher collects it.
    Interrupted { signal: i32 },
}

````
<!-- /fragment -->

`Signalled` is the case this chapter has to state most carefully, and the source
states it in the comment above: it is **narrower than *a token appeared***. The
two are easy to conflate because the escalation begins when the token appears, so
one might expect every launch with a token to end `Signalled`. It does not, and
what separates the two is only whether the child's own exit lands inside the
grace.
`a_child_that_signals_and_exits_inside_the_grace_is_never_touched` builds
precisely that child — it writes its token, sleeps, and exits — and
runs it under a thirty-second grace so that its own exit lands well inside;
`End::Exited` is what comes back, with a token, and the test also asserts that
`elapsed` is less than the grace, so a passing run cannot be one that waited the
grace out and got lucky. The complementary case,
`an_unsignalled_child_runs_to_its_own_exit_untouched`, is the same ending reached
with no token at all.

The two tests establish the rule: `End` reports **who acted**, and
nothing else. A caller that wants *did the child say it was done* reads `token`;
a caller that wants *did this launch complete or fall over* reads `end`; and a
caller that conflated them would be inferring one from the other, which is
exactly the failure this crate is designed to prevent.

`Interrupted` is **defined here and produced only in chapter 4**. Its argument
— that the signal is carried rather than merely noted, so a launcher can
die of the same signal instead of exiting zero — is `reraise`'s, and
`reraise` is chapter 4's; so is the distinction the last paragraph draws between
a signal arriving *during* a launch and one arriving between two, which is
`take_interrupt`'s. `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other`,
in `crates/keyed-launch/tests/interrupt.rs`, is the test that holds both halves,
and chapter 4 reads it. What belongs here is only that the third case exists and
that `run` can return it, because a reader meeting `Ended` needs all three arms
to know what they are matching on.

The derives split the types along the same line. `Ended` is `Debug` and
nothing else — it holds an `ExitStatus` and a `Token`, and it is a report
rather than a value to compare. `End` and `Group` add `Clone`, `Copy`,
`PartialEq` and `Eq`, because comparing them is the whole of what a caller does
with them, and every test above is an `assert_eq!` against one of their arms.

<a id="what-survives-an-exec"></a>
## The one disposition that survives an exec

The block resumes after chapter 4's supervisor, handler and latch, and it opens
on a list of seven signals.

<!-- fragment «run-default-dispositions» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="333-359" parent="terminal-and-spawn" -->
````rust

/// The signals the child is handed back at their **default** disposition.
///
/// **Only an *ignored* disposition survives `execve`.** POSIX resets a caught
/// handler to the default across an exec and leaves an ignore in place, and
/// `std::process::Command` restores exactly one thing on top of that — SIGPIPE,
/// which Rust ignores process-wide at start-up. So this list is not about
/// handlers, which take care of themselves. It is about a launcher that
/// *ignores* one of these for its own reasons and would otherwise hand the
/// ignore to its child, to that child's children, and to every wrapper in
/// between: a login shell or an `ssh` hop that inherits an ignored SIGINT keeps
/// ignoring it *and propagates it onward*, so an interactive session under one
/// cannot be interrupted at all and nothing in it can tell why.
///
/// A launcher that wanted a child to inherit an ignore has to say so some other
/// way. That is the right default for this crate, whose child owns a terminal:
/// a terminal-generated signal the human types must reach the process the
/// human is looking at.
const DEFAULT_DISPOSITION_IN_CHILD: [libc::c_int; 7] = [
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGTERM,
    libc::SIGHUP,
    libc::SIGTSTP,
    libc::SIGTTIN,
    libc::SIGTTOU,
];
````
<!-- /fragment -->

The argument above turns on one asymmetry: across `execve`, a *caught* handler is
reset to the default and an *ignore* is kept. So for six of the seven entries the
loop is not defending against a launcher's handlers, which the exec resets on its
own. It is defending against a launcher's **ignores**, which the exec preserves
and which propagate onward through every wrapper the command happens to name.
The seventh entry is SIGTTOU, which is doing a second job the other six are not,
and the handover section below is where that job appears.

`an_ignored_sigint_in_the_launcher_does_not_reach_the_child` is the test, and it
is worth naming what makes it a test rather than a demonstration. Its child
*reports what it inherited* — `kill -INT $$` against whatever disposition
arrived — instead of installing a handler of its own, because a child that
installed one would overwrite the inherited disposition and behave identically
whether or not the launcher had leaked anything. And it runs a positive control
first: the same script under a plain `Command::status()`, which does inherit the
ignore and does write the marker file. Without that control a passing assertion
would be equally consistent with a fixture that could never see the fault at all.

Two of the seven connect forward on this page, and both connections are the same
job-control fact seen from a different side. **SIGTTOU** is in the list, and it is
also ignored across the `tcsetpgrp` in `Terminal::hand_to` below and again inside
the `pre_exec` closure — three appearances, one reason, which the terminal
sections read. **SIGINT** is in the list, and chapter 4's
`install_termination_handler` catches it only for a launch with no terminal, and
says why in so many words. Those are the two halves of a single decision: a
Ctrl-C the human types is delivered to the terminal's foreground group, which
after the handover below is the child's, so the child must have SIGINT at its
default to receive it and a launcher with a terminal has no business
intercepting it.

<a id="the-gate-that-needs-no-flag"></a>
## The gate that needs no flag

The controlling terminal is wrapped in a private newtype rather than carried
around as a descriptor, and the wrapper itself is four lines.

<!-- fragment «run-terminal-type» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="360-363" parent="terminal-and-spawn" -->
````rust

/// The launcher's controlling terminal, open for as long as a launch needs to
/// hand it back and forth.
struct Terminal(OwnedFd);
````
<!-- /fragment -->

One owned descriptor, private, and the type exists so that the close is the
compiler's problem rather than a launch's. Every method below is on it.

<!-- fragment «run-terminal-open» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="364-388" parent="terminal-and-spawn" -->
````rust

impl Terminal {
    /// `/dev/tty` rather than stdin, and the difference is the gate.
    ///
    /// `/dev/tty` *is* the controlling terminal by definition, so a launcher
    /// whose stdin was redirected still hands over the right device — and a
    /// launcher that has no controlling terminal at all (a test runner, a CI
    /// job, a daemon) simply fails to open it and gets no job control. There is
    /// no flag to set and nothing for a caller to configure wrongly.
    fn open() -> Option<Self> {
        // SAFETY: `open(2)` against a constant NUL-terminated path. The
        // returned descriptor is owned from here on and closed by `OwnedFd`.
        let fd = unsafe {
            libc::open(
                c"/dev/tty".as_ptr(),
                libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return None;
        }
        // SAFETY: a fresh descriptor this process just opened and has not
        // handed to anything else.
        Some(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }
````
<!-- /fragment -->

The return type is the design. `Option<Self>` and not `Result<Self, LaunchError>`
means *having no controlling terminal is not a failure*, and that single choice is
what removes a configuration flag from the crate's surface. A launcher running
under a CI job or a systemd unit opens nothing, gets `None`, and proceeds with no
job control at all; a launcher running under a human's terminal
opens it and gets the handover. Nothing had to be told which of the two it was,
and there is no flag for an operator to set to the wrong value — which is
the same shape as `Escalation` having no `Default`, arrived at from the opposite
direction: the crate refuses to guess what only the caller knows, and refuses to
ask about what the operating system already answers.

`/dev/tty` rather than stdin is what makes that work, because `/dev/tty` *is* the
controlling terminal by definition and stdin is merely whatever the launcher was
handed. A launcher whose stdin is a pipe still gets the right device; one with no
controlling terminal at all cannot open the path, which is the `None` above.

`O_CLOEXEC` is the flag worth reading against the spawn at the end of this
chapter, and worth reading for what it does *not* buy. The descriptor is used
inside the `pre_exec` closure, which runs *after* `fork` and *before* `execve`,
so the handover happens while it is still open and the exec then closes it. What
that prevents is a fourth, unnamed descriptor onto the terminal riding into the
child and every one of its descendants. It is **not** isolation: `run` configures
no stdio at all, so the child inherits the launcher's own standard input, output
and error, which in the case this code exists for *are* that terminal — and
a child that holds the terminal can open `/dev/tty` for itself whenever it likes.
The flag is descriptor hygiene. The child's ownership of the terminal comes from
the process group and the `tcsetpgrp` below, not from which descriptors survive
the exec.

<!-- fragment «run-terminal-accessors» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="389-398" parent="terminal-and-spawn" -->
````rust

    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// Which process group currently owns the terminal, or `-1`.
    fn foreground(&self) -> libc::pid_t {
        // SAFETY: `tcgetpgrp(3)` on a descriptor this struct owns.
        unsafe { libc::tcgetpgrp(self.fd()) }
    }
````
<!-- /fragment -->

`fd` exists so that the raw descriptor can be captured into the spawn closure
below, which is the one place a `&Terminal` cannot go. `foreground` is the
question every handover in this file asks before it acts: *who owns the terminal
right now*. Every call site uses it as a guard rather than as information. The
lease below asks it before the spawn and on every poll tick, to decide whether
this launcher has a terminal it is entitled to lend, and again once the child is
reaped, to decide whether it has one it is entitled to take back and restore.

<!-- fragment «run-terminal-hand-to» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="399-416" parent="terminal-and-spawn" -->
````rust

    /// Make `pgid` the terminal's foreground process group.
    ///
    /// **`tcsetpgrp` from a group that is not already the foreground one raises
    /// SIGTTOU at the caller**, whose default action stops it — so a launcher
    /// reclaiming the terminal from its child would stop itself in the act of
    /// taking it back. Ignoring SIGTTOU across the call and restoring the
    /// previous disposition afterwards is the standard job-control dance, and
    /// it is why this is a method rather than a bare call at three sites.
    fn hand_to(&self, pgid: libc::pid_t) {
        // SAFETY: `signal(2)` and `tcsetpgrp(3)` on a descriptor this struct
        // owns; the previous disposition is restored before returning.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetpgrp(self.fd(), pgid);
            libc::signal(libc::SIGTTOU, previous);
        }
    }
````
<!-- /fragment -->

Three sites in this file hand the terminal to a process group, and this method
exists so that two of them do not have to write the dance out. Those two are the
lease's `lend` and `reclaim` below: the first gives the terminal to the child on
every poll tick of chapter 4's `watch`, and the second takes it back once the
child is reaped. The third site cannot use it: it is the
`tcsetpgrp` inside `pre_exec`, in a forked child where the only async-signal-safe
work permitted is the bare calls themselves, and where the disposition is put back
not by a saved handler but by the `DEFAULT_DISPOSITION_IN_CHILD` loop that runs
immediately after.

A terminal handed back in the wrong mode is half a handover. A child that sets
the terminal raw and is then killed by the escalation cannot undo that, and the
human is left with a shell that neither echoes nor waits for a line. So the same
impl carries the two halves of a save and a restore.

<!-- fragment «run-terminal-attributes» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="417-443" parent="terminal-and-spawn" -->
````rust

    /// The terminal's current attributes, or `None` if they cannot be read.
    fn attributes(&self) -> Option<libc::termios> {
        // SAFETY: `tcgetattr(3)` filling an initialised struct from a
        // descriptor this struct owns.
        unsafe {
            let mut attributes: libc::termios = std::mem::zeroed();
            (libc::tcgetattr(self.fd(), &mut attributes) == 0).then_some(attributes)
        }
    }

    /// Put back attributes saved earlier: cooked mode and echo, for a child
    /// that set the terminal raw and was killed before it could undo that.
    ///
    /// Made with SIGTTOU ignored for the same reason as [`Terminal::hand_to`].
    /// It is called only from the foreground, but the guard costs nothing.
    fn restore(&self, attributes: &libc::termios) {
        // SAFETY: `signal(2)` and `tcsetattr(3)` on a descriptor this struct
        // owns, with attributes `tcgetattr` produced; the previous disposition
        // is restored before returning.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetattr(self.fd(), libc::TCSANOW, attributes);
            libc::signal(libc::SIGTTOU, previous);
        }
    }
}
````
<!-- /fragment -->

`attributes` answers `None` rather than failing, for the same reason `open` does:
a terminal whose attributes cannot be read is no reason to refuse a launch, and
the lease below then simply has nothing to restore. `restore` is `TCSANOW`
because the child is already gone and there is no output of its to drain. The
SIGTTOU guard is belt and braces: the lease calls `restore` only once the
launcher is back in the foreground, where `tcsetattr` raises nothing, but a
background `tcsetattr` would stop the launcher. The comment says the guard costs
nothing, which is the whole case for it.
`a_raw_mode_child_killed_by_the_escalation_leaves_the_terminal_restored` pins the
pair. Its child runs `stty raw -echo` and records `stty -a`, which is the positive
control showing the terminal really was raw, and the launcher's local modes after
the launch must equal the ones before it.

<!-- fragment «run-own-group» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="444-449" parent="terminal-and-spawn" -->
````rust

/// This process's own process group.
fn own_group() -> libc::pid_t {
    // SAFETY: `getpgrp(2)` takes no argument and cannot fail.
    unsafe { libc::getpgrp() }
}
````
<!-- /fragment -->

Six lines, and the reason it is a function is that `own_group()` reads as a
question in the guards that ask it — *is the terminal mine to lend*, *is it
mine again* — where `libc::getpgrp()` would read as a system call whose result
the reader has to interpret.

<a id="the-lease"></a>
## The lease on the terminal

A launch borrows the terminal and owes it back, and what it owes depends on what
happened during the launch. A launcher that never held the foreground lent
nothing and owes nothing. One that did owes the terminal back, in the modes it
was lent in, and from the right group. That is three facts carried across the
whole launch, so they live in one value rather than in locals the spawn and the
supervisor would each have to thread through.

<!-- fragment «run-lease-type» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="450-467" parent="terminal-and-spawn" -->
````rust

/// One launch's share of the terminal: when it was held, what it looked like
/// then, and how it is given back.
///
/// **"Held the foreground" is the gate for everything on the way back.** A
/// launcher in the foreground lends the terminal to its child and owes the
/// human a terminal back. One that never held it during the launch lent
/// nothing, so it takes nothing and restores nothing: anything else would
/// steal from whichever job does hold it.
struct Lease {
    terminal: Option<Terminal>,
    child: libc::pid_t,
    /// Whether the launch has held the foreground at any moment so far.
    held: bool,
    /// The attributes at the moment the launch first held it, when they
    /// could be read.
    saved: Option<libc::termios>,
}
````
<!-- /fragment -->

The comment's bold sentence is the rule everything below obeys. **Held the
foreground** is not "had a terminal": a launcher in the background of its own
terminal has one and holds nothing. `held` and `saved` are separate fields
because they can disagree. A launcher that held the foreground but could not
read the attributes still owes the terminal back, and owes no modes. `child` is
the child's process group, which is also its pid: it is zero until the spawn
returns and the launch fills it in.

<!-- fragment «run-lease-hold» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="468-480" parent="terminal-and-spawn" -->
````rust

impl Lease {
    /// Before the spawn: whether this launcher holds the foreground now, and
    /// if so the descriptor the child hands itself the terminal through.
    fn hold_before_spawn(&mut self) -> Option<RawFd> {
        let terminal = self.terminal.as_ref()?;
        if terminal.foreground() != own_group() {
            return None;
        }
        self.held = true;
        self.saved = terminal.attributes();
        Some(terminal.fd())
    }
````
<!-- /fragment -->

`hold_before_spawn` runs once, before the child exists, and it answers the
question the spawn needs answered: may the child hand itself the terminal from
inside `pre_exec`? Only when this launcher is the terminal's foreground group,
because handing over a terminal somebody else's job owns is theft. **The
attributes are saved here, before the spawn**, and the ordering is the point: a
child that sets raw mode on its first instruction would otherwise have its raw
modes saved as the ones to restore.

<!-- fragment «run-lease-lend» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="481-498" parent="terminal-and-spawn" -->
````rust

    /// Called on every poll tick, and once before the spawn. Re-checked each
    /// time, so a launcher started in the background and later brought
    /// forward (`grove &`, then `fg`) hands the terminal on to the job that is
    /// actually running under it, and only then saves what it will restore.
    fn lend(&mut self) {
        let Some(terminal) = &self.terminal else {
            return;
        };
        if terminal.foreground() != own_group() {
            return;
        }
        if !self.held {
            self.held = true;
            self.saved = terminal.attributes();
        }
        terminal.hand_to(self.child);
    }
````
<!-- /fragment -->

`lend` is the per-tick half, and its comment carries the case for re-checking
every tick: `grove &`, then `fg`. A launcher started in the background holds
nothing at the spawn; brought forward later, it finds itself the foreground
group on the next tick and passes the terminal on to the child that is actually
running under it. That later moment is also when it first holds the foreground,
so that is when it saves the attributes. `a_launcher_started_in_the_background_takes_no_foreground`
pins the other side: a launcher the session leader keeps in the background never
lends, its child never holds the terminal, and the foreground is still the
session leader's afterwards.

<!-- fragment «run-lease-reclaim» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="499-537" parent="terminal-and-spawn" -->
````rust

    /// Take the terminal back once the child is reaped, and restore it.
    ///
    /// From the child's group after an ordinary exit. That group is gone by
    /// now, but the terminal still names it until it is given to another.
    /// **After a death by signal, from whichever group then holds it**,
    /// unless that is the launcher's own or the session leader's. A child
    /// that is itself a supervisor and was killed can leave its own child's
    /// group holding the terminal, an orphan in the foreground reading the
    /// human's input, and only this rule takes it back. After an ordinary
    /// exit the rule stays narrow, because a group other than the child's that
    /// holds the terminal then took it on purpose. A job-control shell nested
    /// in this session is one example.
    ///
    /// The attributes go back whenever the launcher ends in the foreground,
    /// so a raw-mode child killed by the escalation still leaves cooked mode
    /// behind.
    fn reclaim(&self, status: Option<ExitStatus>) {
        use std::os::unix::process::ExitStatusExt as _;
        let Some(terminal) = self.terminal.as_ref().filter(|_| self.held) else {
            return;
        };
        let holder = terminal.foreground();
        let own = own_group();
        let take = if status.and_then(|status| status.signal()).is_some() {
            // SAFETY: `getsid(2)` on this process, which cannot fail.
            let leader = unsafe { libc::getsid(0) };
            holder > 0 && holder != own && holder != leader
        } else {
            holder == self.child
        };
        if take {
            terminal.hand_to(own);
        }
        if let Some(saved) = self.saved.as_ref().filter(|_| terminal.foreground() == own) {
            terminal.restore(saved);
        }
    }
}
````
<!-- /fragment -->

`reclaim` is called once, by chapter 4's `supervise`, after the child is reaped,
and with its status when it was reaped. Its two branches are the job
contract's two rules, and the comment says why they differ.

After an **ordinary exit** the terminal is taken back only from the child's own
group. That group is gone by now, but the terminal still names it: measured on
macOS, `tcgetpgrp` keeps answering the vanished group's ID until the foreground is
given to another, and `kill(-id, 0)` answers that no such group exists. So the
comparison with `child` still finds it. Any other group that holds the terminal
at that moment took it on purpose, and the rule leaves it there.
`a_child_that_exits_normally_leaves_the_foreground_with_another_group` builds
that case. Its child starts a process that leads a group of its own and takes the
terminal, then exits 0, and the foreground afterwards is still that group's.

After a **death by signal** the rule widens to any group except the launcher's
own and the session leader's. The case it exists for is a child that is itself a
supervisor and was killed. Its own child's group, now an orphan, is left in the
foreground reading the human's input, and nothing else would ever take the
terminal back from it. The two exclusions keep the wide rule from stealing.
Once the launcher has lent the terminal, only its descendants can hold it, or the
shell that stopped the launcher's job, and that shell leads its session in an
ordinary terminal, a tmux pane and an ssh login alike.
`after_a_death_by_signal_the_launcher_takes_the_terminal_from_another_group` is
the same fixture as the previous test with `kill -KILL $$` in place of `exit 0`,
and the two are each other's control: only how the child ends differs, and the
foreground afterwards differs with it.

Either way, the saved attributes go back whenever the launcher ends up in the
foreground, whether it took the terminal just now or never lost it. A `status` of
`None` means the child could not be reaped, and is read as no death by signal:
the narrow rule, which takes nothing it has no claim to.

<a id="the-child-is-a-job"></a>
## The child is a job

`run`'s own documentation is the chapter's title and its argument, and it is
thirty-nine lines because five separate cases live in it.

<!-- fragment «run-the-child-is-a-job» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="538-577" parent="terminal-and-spawn" -->
````rust

/// Spawn `launch`'s argv directly and supervise the child until it ends.
///
/// The child's environment is the launcher's, minus [`Launch::scrub`], plus the
/// channel path under [`Launch::channel_var`]. Nothing else is added: no
/// argument, no flag, no variable. A child that needs one is given it by the
/// caller, in the argv the caller built.
///
/// **The child is a job, not just a process.** It is put in a process group of
/// its own and — when this launcher owns a controlling terminal and is the
/// foreground group of it — handed that terminal, exactly as a shell does for a
/// foreground job. Two things follow, and both are the point. A terminal signal
/// the human types reaches *the child's* group rather than the launcher's, so
/// the launcher survives a Ctrl-C it never has to catch; and the escalation can
/// signal the whole group, so a grandchild the child spawned is reaped with it
/// rather than left running and attached to the terminal. The child's group is
/// *not* a new session: `setsid` leaves it with no controlling terminal, so
/// the handover fails silently at both ends — the return is ignored — and an
/// interactive child reads on unstopped while the launcher keeps the Ctrl-C.
///
/// The child's signal dispositions are the defaults, whatever the launcher's
/// are — see [`DEFAULT_DISPOSITION_IN_CHILD`]. The launcher's own are respected:
/// no handler goes over a signal it ignores, and an ignored SIGCHLD is repaired
/// for the launcher alone, so that its child is not reaped unwatched.
///
/// **The group ends with the launch, whatever ended the child.** Once the child
/// has exited (a stop is not an exit), its group is killed while the unreaped
/// child still reserves the group's ID. Then the child is reaped, the terminal
/// is taken back with the attributes saved at the handover, and the group is
/// confirmed gone. A group that is still present is [`Ended::group`].
///
/// Supervision polls three things, and they are the only three ways a launch
/// ends: the child exits, the token appears, or the launcher itself is
/// signalled. **A child that finishes its work and never signals reaches none
/// of them** — an interactive one returns to its prompt instead of exiting, so
/// the launch *stalls* rather than ending. That is a real failure mode with no
/// cheap fix here: nothing this crate can observe distinguishes a child that
/// forgot to signal from one still working, so a second completion observable
/// would only trade a stall for a wrong kill. It is the caller's to close, at
/// the layer that instructs the child.
````
<!-- /fragment -->

The first paragraph is the chapter's claim stated as a contract, and the sentence
that makes it operable is the last one: *a child that needs one is given it by the
caller, in the argv the caller built.* That is where the
promise stops being restraint and becomes a property of the system — the
refusal to add is what keeps the caller's argv a complete account of what
will run, and a complete account is the only thing anyone can audit.

The second paragraph's rejected alternative is the one to read against the
disposition list above. The **decision** is not in doubt and its shape is this
page's to connect: the child gets a process *group* and not a new *session*, and
SIGTTIN — the sixth entry of `DEFAULT_DISPOSITION_IN_CHILD` — is handed back at
its default rather than the crate relying on the child never being in a position
to receive it. Those are two halves of one job-control decision, and neither
comment mentions the other.

The **mechanism** for that rejection is the one thing on this page the book once
declined to adopt, and it has since been run. The comment used to say that a
child in a fresh session would be stopped by SIGTTIN on its first read. Measured
on a pseudo-terminal — allocated by a test program and made a controlling
terminal by a leader that calls `setsid` and opens the slave — it is not. SIGTTIN
is raised only for a background group *of a controlling terminal*, and `setsid`
is exactly what leaves the child without one, so its reads succeed. The control
arm is what makes that a reading rather than a blind instrument: the same
reader, with the same line already queued and its own process group, but *in*
the launcher's session, was stopped by SIGTTIN every run.

What fails instead is the handover, and it fails everywhere it could be tried.
`tcsetpgrp` returns ENOTTY from inside the child and EPERM from the launcher,
which cannot name a group in another session; `TIOCSCTTY` — the idiom a serious
implementation of the rejected option would reach for — is EPERM in both its
plain and its stealing form, because the terminal already belongs to the
launcher's session; and reopening the device by name yields a descriptor but
still no controlling terminal. Both of this crate's handover sites discard
`tcsetpgrp`'s return value, and `watch` retries the launcher's every poll tick,
so none of that would ever be reported — it would fail silently, in a loop.

The consequence is what the comment now carries, and the rejection is stronger
for it. The same run showed the child reading a line off the terminal while the
launcher's group still held the foreground, and showed a typed Ctrl-C reaching
the launcher and not the child. A session does not fail safe; it fails quietly,
with the child taking input nobody handed it and the interrupt going to the
process that was supposed to be shielded from it. All of this is one platform —
macOS 26.6 on arm64 — and the errnos are POSIX's, not Darwin's, but the reading
is a reading and not a portability proof.

The last three paragraphs belong to chapter 4 and are reproduced here only
because they are `run`'s contract. The third adds to the disposition list that
the launcher's own dispositions are respected too: no handler over an ignore,
and an ignored SIGCHLD repaired for the launcher alone. The fourth is the end of
the launch: the group killed while the zombie child still reserves its ID, the
reap, the terminal returned through the lease above, and the group confirmed
gone or reported as `Group::Present`. The fifth is the three observables, and
the honest statement that a child which finishes its interactive turn and never
signals reaches none of them. Chapter 4 owns `watch`, where those three are
polled and the end of the group is carried out, and owns the stall as a named
failure mode.

<a id="nothing-added"></a>
## Nothing added

`run` delegates to `run_observed` with a no-op callback, preserving existing
`Launch` literals and results. The observed entry point installs chapter 4's
handler and builds the same `Command`. `LaunchEvent` reports parent-side
boundaries independently of `End`: a successfully spawned child always emits
Started, even if it exits immediately, while only confirmed reap emits Reaped.
The callback returns unit and must handle its own failures without blocking or
panicking. It supplies neither launch authority nor a child acknowledgement.
Noninteractive entry points also select captured streams and a separate
POSIX session. The confined entry point adds a native filesystem wrapper.
Those choices are explicit APIs; the ordinary interactive path retains the
command, environment and terminal contract explained here.

<!-- fragment «run-command-and-environment» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="578-692" parent="terminal-and-spawn" -->
<!-- insert «run-output-modes» -->
<!-- insert «run-output-setup» -->
<!-- /fragment -->

<a id="run-output-modes"></a>
### Choose the process mode

`run` and `run_observed` preserve interactive job control. The observed entry
point reports Started after successful spawn and Reaped after confirmed reap.
`run_noninteractive` instead requires a regular log file; `run_confined` also
constructs a mandatory native filesystem policy before spawning. All four return
the same `Ended` shape. A no-op observer keeps the simpler APIs free of callback
requirements, while the caller still interprets the returned token.

The two detached entry points differ in how a cancellation reaches the child,
and their comments say why. A noninteractive child is forwarded the cancelling
signal and given the kill-grace, because it may itself be a supervisor that
needs that long to end its own child. A confined child is killed at once,
because its launcher may be a nested supervisor already inside its own caller's
grace. Chapter 4's `Mode` is how supervision tells the three apart, and chapter 7
reads the confined case.

<!-- fragment «run-output-modes» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="578-648" parent="run-command-and-environment" -->
````rust
pub fn run(launch: Launch<'_>) -> Result<Ended, LaunchError> {
    run_observed(launch, &mut |_| {})
}

/// Parent-side evidence about one launched child, independent of its token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchEvent {
    /// Spawn succeeded. The child may already have exited.
    Started,
    /// A wait confirmed reap, even if supervision returns an error afterwards.
    Reaped,
}

/// Run a launch with synchronous, infallible parent-side notifications.
///
/// Started occurs exactly once after successful spawn; failed spawn emits no
/// events. Reaped occurs exactly once on confirmed reap, before token reading
/// and terminal recovery. A token alone or an unsuccessful wait is not reap.
/// The callback must return promptly and must not panic; observation failures
/// must be handled within it. No child acknowledgement or outcome override is
/// involved. All other behavior is the same as [`run`].
pub fn run_observed(
    launch: Launch<'_>,
    observer: &mut dyn FnMut(LaunchEvent),
) -> Result<Ended, LaunchError> {
    run_with_output(launch, observer, None, None)
}

/// Run a noninteractive child in a new POSIX session, without a controlling
/// terminal or inherited input. Both output streams go to a caller-owned regular
/// file. Cancellation forwards the launcher's signal to the job and, after the
/// kill-grace, kills it: the child may itself be a supervisor, and needs that
/// long to end its own child. Remaining members of the child's process group
/// are killed when it exits, as in every launch. This isolates process control, not filesystem access or processes that
/// deliberately leave the child's process group.
pub fn run_noninteractive(launch: Launch<'_>, output: File) -> Result<Ended, LaunchError> {
    let regular = output
        .metadata()
        .map_err(|error| LaunchError::new(format!("cannot inspect the launch log: {error}")))?;
    if !regular.is_file() {
        return Err(LaunchError::new(
            "the noninteractive launch log must be a regular file",
        ));
    }
    run_with_output(launch, &mut |_| {}, Some(output), None)
}

/// As [`run_noninteractive`], under mandatory filesystem confinement, with no
/// unconfined fallback. The program is an absolute path: no name is looked up.
///
/// **Cancellation kills a confined job at once**, with no kill-grace. Its
/// launcher may itself be a nested supervisor in its own caller's grace, and
/// must finish its cancellation inside that grace.
pub fn run_confined(
    launch: Launch<'_>,
    output: File,
    policy: &crate::Confinement<'_>,
) -> Result<Ended, LaunchError> {
    if !output
        .metadata()
        .map_err(|error| LaunchError::new(error.to_string()))?
        .is_file()
    {
        return Err(LaunchError::new(
            "the confined launch log must be a regular file",
        ));
    }
    let command = crate::confinement::command(launch.argv, policy)?;
    run_with_output(launch, &mut |_| {}, Some(output), Some(command))
}

````
<!-- /fragment -->

<a id="run-output-setup"></a>
### Set streams and scrub authority

`run_with_output` derives the launch's `Mode` from its two optional arguments:
a wrapper command means confined, an output file alone means noninteractive, and
neither means interactive. Only an interactive launch opens the terminal,
because a detached child is in a session of its own and could never be handed
it. Whether a terminal opened is what `install_termination_handler` needs, to
decide whether SIGINT cancels. `restore_child_watching` runs next, and its answer
travels into `pre_exec`. Then the descriptor bound is checked, stdin is directed
to EOF and both output streams to the log for a detached mode. A confined call
supplies the wrapper command; otherwise the original executable and arguments
are used directly. The scrub list is applied before the fresh
channel grant, preserving the current invocation's authority while removing
any inherited channel named by the caller.

<!-- fragment «run-output-setup» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="649-692" parent="run-command-and-environment" -->
````rust
fn run_with_output(
    launch: Launch<'_>,
    observer: &mut dyn FnMut(LaunchEvent),
    output: Option<File>,
    confined_command: Option<Command>,
) -> Result<Ended, LaunchError> {
    let mode = match (&output, &confined_command) {
        (_, Some(_)) => Mode::Confined,
        (Some(_), None) => Mode::Noninteractive,
        (None, None) => Mode::Interactive,
    };
    let detached = mode != Mode::Interactive;
    // A detached child is in a session of its own and could never be handed
    // the terminal, so only an interactive launch opens it.
    let terminal = if detached { None } else { Terminal::open() };
    install_termination_handler(terminal.is_some());
    let sigchld_ignored = restore_child_watching();
    let descriptor_limit = if detached { descriptor_limit()? } else { 3 };

    let mut command = confined_command.unwrap_or_else(|| {
        let mut command = Command::new(launch.argv.program());
        command.args(launch.argv.args());
        command
    });
    if let Some(output) = output {
        let stderr = output.try_clone().map_err(|error| {
            LaunchError::new(format!("cannot duplicate the launch log: {error}"))
        })?;
        command.stdin(Stdio::null()).stdout(output).stderr(stderr);
    }
    if let Some(cwd) = launch.cwd {
        command.current_dir(cwd);
    }
    // Scrub first, grant second, and the order is load-bearing rather than
    // stylistic: a caller whose scrub list *contains* its own `channel_var` is
    // the expected shape, not a mistake — the list names the launch-control
    // variables a nested launcher must not inherit, and the channel variable is
    // the first of them. Granting before scrubbing would remove the path this
    // launch just published and leave the child unable to signal, which reads
    // as a session that hung. `tests/launch.rs` pins it.
    for name in launch.scrub {
        command.env_remove(name);
    }
    command.env(launch.channel_var, launch.channel.path());
````
<!-- /fragment -->

The final scrub-and-grant block applies to every mode, and the
comment argues the one thing about them that is not obvious: **scrub first, grant
second**. The order is load-bearing because the expected caller's scrub list
*contains* its own `channel_var` — grove's does, and the variable is the
first of the three — so a grant applied before the scrub would remove the
path this launch had just published. What that failure looks like is the reason it
is worth a comment: not an error, but a child that runs to completion, cannot
signal, and a launcher that waits out its grace and escalates. An operator reads
that as a session that hung.

`granting_the_channel_survives_a_scrub_list_that_names_it` is the test, and it
writes through `${TEST_CHANNEL?unset}` so that a scrubbed-away grant fails
loudly in the child rather than silently producing no token.

For the unconfined interactive path, command construction is the claim’s other
half. `Command::new(program)` with `args(…)` and no `sh -c` is what *spawned whole
and directly* means concretely; `current_dir` is set only when the caller gave a
path, so `None` really is inheritance rather than a default this crate chose. And
`launch.argv.program()` and `launch.argv.args()` are chapter 1's two accessors,
which exist split precisely because this is what a spawn wants.

<a id="both-sides-of-the-handover"></a>
## Both sides of the handover

With the environment settled, the interactive path turns to the terminal and asks two
separate questions of it before handing anything over.

<!-- fragment «run-terminal-handover» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="693-707" parent="terminal-and-spawn" -->
````rust

    // Hand the terminal over from *inside* the child as well as from the parent
    // below, which cannot be early enough on its own: the parent's handover
    // waits on `spawn` returning, nothing orders the child's first read after
    // that, and a read from a background group is what SIGTTIN stops. Only when
    // this launcher is the terminal's current owner — handing over a terminal
    // owned by somebody else's job is theft, not job control. Holding it now is
    // also when its attributes are saved, before a child can set it raw.
    let mut lease = Lease {
        terminal,
        child: 0,
        held: false,
        saved: None,
    };
    let handover_fd = lease.hold_before_spawn();
````
<!-- /fragment -->

One guard and one duplication, and they answer different questions. The guard,
inside `hold_before_spawn`, is a question of **entitlement**: this launcher may
hand over the terminal only if it currently owns it, because otherwise it would
transfer control away from a different job. That is the guard
`Terminal::foreground` exists for, and the lease's `reclaim` asks its mirror
image before taking the terminal back. Answering yes is also what marks the
launch as having held the foreground, and saves the attributes to restore.

The comment's own argument is about **timing**, and it is why a `RawFd` is
captured into a closure at all. The parent's own handover cannot run before
`spawn` has returned and `watch` has reached its first tick, and nothing orders
the child's first read after that point: a child that got there first would be
reading from a background process group, which is what SIGTTIN stops. The closure
is the answer because of *where* it runs — between the fork and the exec, ordered
before the first instruction of the program being launched. The cost is that the
same operation is performed at two sites: here, as a descriptor captured for the
closure that will perform it inside the child, and again from the parent, which
is not in this function at all but in the lease's `lend`, which chapter 4's
`watch` calls every tick. That second site is not the other half of a race. It
earns its place for a reason of its own, which the lease's section gave: it is
re-asked every tick.

<!-- fragment «run-process-group» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="708-715" parent="terminal-and-spawn" -->
````rust

    // The group, through `std`'s own checked path rather than a `setpgid` of our
    // own: it runs it before the `pre_exec` callbacks below and reports a
    // failure as a failed spawn, which a raw call in the closure could only do
    // by hand.
    if !detached {
        command.process_group(0);
    }
````
<!-- /fragment -->

`process_group(0)` is `std`'s own spelling of *put the child in a group of its
own*, and the comment's reason for preferring it to a `setpgid` in the closure is
about **where a failure lands**. Inside `pre_exec` a failed call could only be
reported by hand, from a context that must stay async-signal-safe; through `std`
it runs before the closures and a failure comes back as a failed spawn, which is
an ordinary `Err` the caller already handles. This is the same reasoning chapter 2
gave for checking the channel directory before the launch rather than letting the
child's write fail: move the failure to where somebody is reading.

<!-- fragment «run-pre-exec» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="716-760" parent="terminal-and-spawn" -->
````rust

    // SAFETY: the closure runs between `fork` and `exec`, so it may call only
    // async-signal-safe functions. `signal`, `getpid` and `tcsetpgrp` (an
    // `ioctl`) are all on POSIX's list; nothing here allocates, locks, or
    // touches Rust runtime state. `std` itself resets only SIGPIPE across a
    // spawn and inherits the signal mask, so everything below is work nothing
    // else is doing.
    unsafe {
        command.pre_exec(move || {
            if detached {
                // Close on exec rather than close here: std's error-reporting
                // pipe must survive until exec succeeds (it is CLOEXEC too).
                // fcntl is async-signal-safe; all allocation happened in parent.
                for fd in 3..descriptor_limit {
                    if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1 {
                        let error = std::io::Error::last_os_error();
                        if error.raw_os_error() != Some(libc::EBADF) {
                            return Err(error);
                        }
                    }
                }
            }
            // setsid must precede becoming a process-group leader. It creates
            // both the session and the group, so setpgid is omitted in this mode.
            if detached && libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            // Until the `tcsetpgrp` below returns, this process is a background
            // group touching the terminal, which is precisely what SIGTTOU is
            // raised for. The loop that follows puts the disposition back.
            libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            if let Some(fd) = handover_fd {
                libc::tcsetpgrp(fd, libc::getpid());
            }
            for signal in DEFAULT_DISPOSITION_IN_CHILD {
                libc::signal(signal, libc::SIG_DFL);
            }
            // The launcher repaired an inherited ignored SIGCHLD for itself
            // alone; the child gets the disposition it would have inherited.
            if sigchld_ignored {
                libc::signal(libc::SIGCHLD, libc::SIG_IGN);
            }
            Ok(())
        });
    }
````
<!-- /fragment -->

The pre-exec closure must use only async-signal-safe operations. Detached mode
first marks descriptors above stderr close-on-exec, preserving Rust’s spawn-error
pipe until exec, then calls `setsid` without an earlier `setpgid`. Interactive
mode instead keeps the existing session for terminal handover. What is worth connecting is the *order* of the two blocks inside the
closure, because it is the third appearance of SIGTTOU on this page and the one
that explains the other two. The child ignores SIGTTOU, performs the handover
— which is precisely the operation a background group is stopped for —
and then walks `DEFAULT_DISPOSITION_IN_CHILD`, whose SIGTTOU entry restores the
default. `Terminal::hand_to` saves and restores the previous disposition because
its caller is a long-lived launcher whose own policy must survive; here nothing
needs saving, because the loop that follows is already going to set every one of
the seven to its default. Two different restorations, one rule.

The closure's last act is the child's half of a repair chapter 4 makes for the
launcher. A launcher that inherited SIGCHLD ignored has had the default restored
for itself by `restore_child_watching`, so that its child is not reaped
unwatched. The child would otherwise inherit that repair, which was never its
own. So when the launcher's entry disposition was the ignore, the child is put
back to it here, and receives the disposition it would have inherited from its
launcher. `a_launcher_with_sigchld_ignored_still_supervises_its_child_to_an_end`
asserts both halves: the launcher reads SIGCHLD at its default after the launch,
and the child reads it ignored.

<a id="the-latch-and-the-child-away"></a>
## The latch cleared, and the child away

Everything is now built and nothing has been started. Two acts stand between the
assembled `Command` and a running child, and the first of them is a single
store.

<!-- fragment «run-clear-and-spawn» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="761-772" parent="terminal-and-spawn" -->
````rust

    // Clear the latch *before* the spawn, never after: see `INTERRUPTED_BY`. A
    // signal that arrived while no child existed is the launcher's to handle
    // through `take_interrupt`, and is not evidence about the child below.
    INTERRUPTED_BY.store(0, Ordering::Relaxed);

    let child = command.spawn().map_err(|error| {
        LaunchError::new(format!(
            "cannot spawn {:?}: {error}; check that the program exists and is executable",
            launch.argv.program()
        ))
    })?;
````
<!-- /fragment -->

The store is chapter 4's to argue and this chapter's only to place: it happens
**immediately before the spawn**, with the `Command` fully built and nothing left
between the two but the call. Everything above it — opening the terminal,
building the environment, installing the closure — is work during which a
signal to the launcher belongs to no child, and the clear is what makes the latch
mean *this launch* rather than *some time recently*.

The spawn's refusal is the only `LaunchError` this function constructs. It names
the program in `{:?}`, so a path with spaces or a name that is empty comes back
legible, carries the operating system's own message, and ends by naming what to
check — the same *say what is wrong, say where, say what fixes it* shape
chapter 1 read off the error type. `a_program_that_does_not_exist_names_itself_and_says_what_to_check`
asserts on both halves: the program's name, and the word *executable*.

The callback receives Started immediately after successful spawn, before any
wait can report the child gone. The handoff below fills in the lease's child and
passes supervision two closures over it, one that lends the terminal each tick
and one that reclaims it once.

<!-- fragment «run-parent-group-and-supervise» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="773-837" parent="terminal-and-spawn" -->
<!-- insert «run-spawn-handoff» -->
<!-- insert «run-descriptor-bound» -->
<!-- /fragment -->

<a id="run-spawn-handoff"></a>
### Hand the child to supervision

After a successful spawn, the observer receives Started and the parent records
the child ID as its process-group ID. The parent-side `setpgid` is used only for
interactive launches; detached launches already created a session and group.
The lease learns the child's group, and is put in a `RefCell` because the two
closures `supervise` receives both reach it: `lend` mutably on every tick, and
`reclaim` once at the end. `supervise` receives the launch's `Mode` instead of a
group ID, because the child it supervises already knows its own.

<!-- fragment «run-spawn-handoff» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="773-799" parent="run-parent-group-and-supervise" -->
````rust

    observer(LaunchEvent::Started);

    // The parent's half of the same `setpgid` — insurance, not a race. The
    // `pre_exec` above takes `std` off `posix_spawn` onto fork-and-exec, and
    // `spawn` then returns only after the child has exec'd, so this call is
    // measured to fail EACCES. Kept: that ordering is undocumented, not a rule.
    let pgid = child.id() as libc::pid_t;
    // SAFETY: `setpgid(2)` naming this process's own child.
    if !detached {
        unsafe { libc::setpgid(pgid, pgid) };
    }

    lease.child = pgid;
    let lease = std::cell::RefCell::new(lease);

    supervise(
        child,
        launch.channel,
        launch.escalation,
        mode,
        observer,
        || lease.borrow_mut().lend(),
        |status| lease.borrow().reclaim(status),
    )
}

````
<!-- /fragment -->

<a id="run-descriptor-bound"></a>
### Close inherited descriptor capabilities

`descriptor_limit` combines the current soft descriptor limit with every open
descriptor visible through the platform's descriptor directory. An already-open
high descriptor remains covered even after a caller lowers its limit. The
pre-exec loop uses the resulting bound to mark inherited descriptors
close-on-exec. Inspection failure refuses launch; callers must not concurrently
change descriptor limits or signal dispositions during a noninteractive launch.

<!-- fragment «run-descriptor-bound» owner="nothing-else-added" source="crates/keyed-launch/src/run.rs" lines="800-837" parent="run-parent-group-and-supervise" -->
````rust
/// Include existing descriptors even if the caller lowered its soft limit
/// after opening them. The remaining range covers descriptors std opens while
/// preparing the child. Noninteractive launches must not run concurrently with
/// a thread changing the process descriptor limit or signal dispositions.
fn descriptor_limit() -> Result<RawFd, LaunchError> {
    // SAFETY: getrlimit writes the initialized rlimit structure only.
    let mut limit: libc::rlimit = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        return Err(LaunchError::new(format!(
            "cannot inspect descriptor limit: {}",
            std::io::Error::last_os_error()
        )));
    }
    let mut maximum = i32::try_from(limit.rlim_cur).map_err(|_| {
        LaunchError::new(
            "cannot isolate an unbounded descriptor table; set a finite open-file limit",
        )
    })?;
    let directory = if cfg!(target_os = "linux") {
        "/proc/self/fd"
    } else {
        "/dev/fd"
    };
    for entry in std::fs::read_dir(directory).map_err(|error| {
        LaunchError::new(format!("cannot inspect inherited descriptors: {error}"))
    })? {
        let entry = entry.map_err(|error| LaunchError::new(error.to_string()))?;
        if let Some(fd) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<i32>().ok())
        {
            maximum = maximum.max(fd.saturating_add(1));
        }
    }
    Ok(maximum)
}

````
<!-- /fragment -->

For the interactive path, the parent’s `setpgid` is the same call `process_group(0)` already made inside
the child, and the comment above it now says what that makes it: insurance, not a
race. **There is no race left to lose, and it was measured.** Installing a
`pre_exec` closure takes `std` off its `posix_spawn` fast path and onto
fork-and-exec, and that path's own synchronisation makes `Command::spawn` return
only once the child has `execve`d — by which point the child is no longer a
candidate for its parent's `setpgid`, and the call fails with `EACCES` every
time. Thirty spawns of this exact shape returned `EACCES` thirty times, with the
child already its own group leader on each; the same measurement's controls show
the call can return success and can return `ESRCH`, so that is a reading rather
than a stuck instrument.

**Nothing in this repository pins that ordering**, and `std` does not document
it, which is exactly why the second call stays. It is insurance against an
implementation detail changing, and the comment says so. Either way the parent
ends up holding a `pgid` it knows exists before it can possibly need to signal
it, which is what chapter 4's escalation addresses when it sends to `-pgid`
rather than to the child alone. The ignored return value is right on either
reading: every failure the call can produce here means the group already
exists.

`supervise` takes the child, channel, escalation, mode, observer and the two
lease closures. The child moves into supervision; `run_with_output` retains the
lease, which the reclaim closure returns after the watch, whether supervision
succeeded or failed. Chapter 4 owns the wait, the end of the group and the
notification boundaries.

That is the interactive spawn. A program the caller named, an environment the launcher's
own minus a list and plus one path, a working directory the caller named, seven
dispositions handed back at their defaults, a process group of the child's own,
and a terminal — if there was one to give, lent on terms that say how it comes
back. Nothing was added, and the one
thing that was granted was granted because the caller asked for it by name. What
happens next is not this crate deciding the child is finished; it is this crate
waiting for the child to say so, and for the grace to run out when it says so and
keeps sitting there. Chapter 4 is the watch, the escalation, and the launcher's
own signals.

[Previous: Appearance is the event](02-the-channel.md) | [Contents](README.md) | [Next: The watch and the escalation](04-the-escalation.md)
