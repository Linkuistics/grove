# child-signal-disposition-k31

## Goal

Fix the three signal-boundary defects an adversarial read of `keyed-launch-run-k11`
found and that leaf deliberately did not absorb: what the launched child
**inherits** across `exec`, what the escalation **reaps**, and what a signalled
driver **reports** to whatever started it.

## Context

`keyed-launch-run-k11` moved the spawn, the supervision and the kill escalation
into `crates/keyed-launch` and spent its one in-session reviewer on them. Eight
findings were fixed there. These three were not, because each is a behaviour
change to something the meta-grove itself runs interactively, and none of them
was a regression that leaf introduced — all three predate it.

**1 — `SIG_IGN` for SIGINT is inherited by the whole session subtree.**
`loop_driver::ignore_interrupts` sets `libc::signal(SIGINT, SIG_IGN)` so a
terminal Ctrl-C does not kill the loop. An *ignored* disposition survives
`execve` — `std::process::Command` resets only SIGPIPE — so the configured
session starts with SIGINT ignored, and so does everything it spawns. Its own doc
comment claims "the child session installs its own handler and still responds",
which holds only for a child that installs one unconditionally. The shapes
`report_build_pairing`'s doc explicitly declares supported — a wrapper, a login
shell, an `ssh` hop, a container — do the opposite: a non-interactive shell that
inherits the ignore keeps ignoring it and propagates it onward, so with
`bash -lc 'claude …'` configured, Ctrl-C does nothing at all and the only way out
is `/exit`. The same ignore reaches the `grove-llm --content-hash` probe and
`stty`.

**2 — the escalation signals one pid, not the process group.** `run`'s `kill`
targets the direct child, so every grandchild the harness spawned — tool
subprocesses, language servers, an in-flight `grove-llm` — survives SIGKILL of
its parent and stays attached to the terminal. `keyed_launch::kill`'s comment
already says so and says why it was not simply fixed: signalling the group needs
the spawn to lead its own session, and a child leading its own session is no
longer the terminal's foreground process group, so an interactive one takes
SIGTTIN on its first read and stops. That is the trade-off this leaf has to
resolve rather than restate.

The compound failure worth naming, because it is the expensive one: a session's
`grove-llm complete` holds the **shared** epoch lock through its whole operation
(`driver_lease::admit_ambient_session`). If the harness is SIGKILLed while that
grandchild is still alive and blocked, the driver's `invalidate_session_epoch`
waits the full `EPOCH_HANDOFF_TIMEOUT` (30s) for the exclusive lock, times out,
and `complete_post_reap_epoch_handoff` turns a successful session into a fatal
error with its token discarded uninterpreted — a 30-second stall followed by a
spurious abort.

**3 — a SIGTERM'd driver exits 0.** `End::Interrupted` prints a line and returns
`Ok(LoopOutcome::Stopped)`, and `run_configured` maps every `LoopOutcome` to
`Ok(())`. A systemd unit, a `timeout(1)` or a shell `wait` therefore cannot tell
a driver that was killed mid-grove from one that finished cleanly. The
conventional answer is to re-raise the signal with the default disposition after
cleanup, so the parent sees `128+N`.

## Done when

- SIGINT reaches the configured session with its **default** disposition, whatever
  the template wraps it in, and Ctrl-C still does not kill the driver. Note the
  two candidate mechanisms differ: resetting SIGINT to `SIG_DFL` in the child via
  `Command::pre_exec` puts the fix in the runner, where the spawn is; having the
  driver install an empty handler instead of `SIG_IGN` puts it in the loop and
  relies on `exec` resetting handlers but not ignores. Pick one and say why.
- The escalation reaps the child's descendants, **or** it is decided on the
  record that it does not — with the SIGTTIN/foreground-process-group cost
  measured against an actual interactive session rather than reasoned about. If
  the group is signalled, `tcsetpgrp` and the terminal handoff are part of the
  work, not a follow-up.
- A driver ended by SIGTERM or SIGHUP reports it to its own parent.
- `crates/keyed-launch`'s suite covers each of the three against a fake child;
  the SIGINT one needs a child that reports its inherited disposition rather than
  one that installs a handler.
- `cargo test --workspace` and `cargo clippy --workspace --all-targets` clean;
  `CHANGELOG.md` updated.

## Notes

**Positioned before `spec-to-current-state-k23` and after the loop crate**, so it
lands on the final module boundaries and the spec rewrite describes the finished
behaviour. Nothing forces it later than `loop-crate-driver-k22`; if it is picked
earlier, `ignore_interrupts` is still in `src/loop_driver.rs`.

**This is a behaviour leaf in a meta-grove, so verify it by running it.** All
three defects are invisible to a test that does not own a terminal: the SIGINT
one only bites through a wrapper, the process-group one only shows with a child
that spawns children, and the exit-status one is only visible to a parent
process. Drive a real session under a real terminal before claiming any of them.

## Decisions (running log)

**Defect 1 — the reset goes in the runner's `pre_exec`, not in the driver's
handler.** The two candidates both work: `execve` resets *caught* handlers to
default and preserves *ignores*, so either an empty driver-side handler or a
child-side `SIG_DFL` reset makes SIGINT default in the session and everything it
wraps. The runner side wins on two counts. First, the empty handler buys the
driver EINTR it does not have today — under `SIG_IGN` a syscall never sees the
signal at all, and a driver whose whole job is to survive Ctrl-C should keep
that. Second, the guarantee belongs to the thing that spawns: a runner that
hands an interactive child the terminal cannot know what its caller did to its
own dispositions, and *the child starts with default dispositions* is a contract
`keyed-launch` can state and test at its own seam, where the done-when asks for
the coverage. The driver keeps `SIG_IGN`, and its doc comment — which claimed
the child "installs its own handler and still responds" — is corrected.

**Defect 2 — the group is signalled, and the trade-off `keyed_launch::kill`
recorded was false.** That comment says signalling the group "needs the spawn to
lead its own session", and a session leader loses the controlling terminal, so
an interactive child would take SIGTTIN on its first read. It needs its own
process *group*, not its own session: `setpgid(0, 0)` keeps the controlling
terminal, and the launcher then hands the terminal to that group with
`tcsetpgrp` exactly as a shell does for a foreground job. There is no SIGTTIN
cost to measure because the arrangement never puts the child in a background
group while it owns the terminal. The escalation therefore signals `-pgid` and
the pid, so a grandchild — a tool subprocess, a language server, an in-flight
`grove-llm` holding the shared epoch lock — is reaped with its parent.

**Defect 3 — the binary re-raises; the runner owns undoing its own handler.**
`keyed_launch::reraise` restores the default disposition, unblocks and re-raises,
because the crate that installed the handler is the one that can take it off.
`LoopOutcome` gains `Interrupted(signal)` so the loop reports *which* signal, and
`crates/grove` — which owns the process's exit status — calls `reraise`. A
systemd unit, `timeout(1)` or a shell `wait` then sees `128 + N` instead of 0.

## Why this leaf does not install anything

Re-derived rather than inherited, by the matrix `delete-migration-k6` ran: is
there a cell where the **installed** build meets the tree this leaf leaves and
fails?

There is not, and the reason is short. This leaf leaves the tree
byte-identical in shape — no file added, removed or renamed beyond the ordinary
retire, no grammar moved, no witness touched — so the installed 19.6.0 build
meets exactly the tree it met before. What changed is what the driver *does* to
a process it launches, and nothing about that is written into `.grove/` or read
out of it.

The consequence to state, because it is the one thing a reader could mistake for
a fault: the driver running this loop is the old build in memory, so the
remaining sessions of *this* loop still launch under the old signal boundary —
Ctrl-C still swallowed, descendants still surviving, a kill still exiting 0. That
is a delay in acquiring the fix, not a tree the old build cannot read, and the
human acquires it by restarting `grove` on a build that carries this change.
Nothing here forces that restart, so this is not a cutover leaf and it publishes
no release.

**The reviewer allowance was not spent, and the reason is an instruction rather
than a judgement.** This session was launched with an explicit standing
instruction not to materialise subagents unless asked, and user instructions
outrank a skill's. The allowance is permissive rather than obliged, so it lapses
rather than being violated. What stood in for it: the three claims were each
driven against a real pty rather than reasoned about (below), each new test was
watched to **fail** under a deliberate mutation of the fix before its pass was
credited, and the `std` claim the child-side reset rests on — that a spawn resets
SIGPIPE and nothing else — was read out of the installed toolchain's own source
rather than recalled.

**One adjacent hazard found by that pass and closed here.**
`loop_driver::reset_terminal` spawns `stty sane`, and `tcsetattr` from a
*background* process group raises SIGTTOU whatever `TOSTOP` says — so a driver
that is not the terminal's owner spawns an `stty` that stops on its first act and
then waits on a stopped child forever. It predates this leaf (`grove &` reaches
it), but the terminal handoff is this leaf's territory and the guard is three
lines, so it is closed here rather than left for a reader to rediscover next to
code this leaf just wrote.

## Verification under a real terminal

The leaf's own note is that all three defects are invisible to a test that does
not own a terminal, so each was driven under a pty (`script -q /dev/null`) with a
real `grove`, a real jj workspace, and a configured session that is a **wrapper**
— `sh -lc '…'` — which is the shape the report names.

- **Inherited disposition.** The session records `trap` with no arguments before
  installing any trap of its own, which prints an entry for a signal ignored on
  entry and nothing for one at its default. It printed `inherited-traps<>` —
  empty. A typed Ctrl-C (`\003` written to the pty) then fired the session's own
  `INT` trap, and the driver survived it and finished the loop; the pre-fix build
  swallowed the interrupt entirely.
- **Descendants.** The session's background grandchild was gone (`kill -0` →
  ESRCH) after the escalation. In the suite, the same claim failed under a
  mutation that signalled the pid alone — and failed *loudly*: the surviving
  grandchild held cargo's captured output pipe open and hung the run, which is
  the failure mode itself.
- **The SIGTTIN cost, measured rather than argued.** A second pty run made the
  session genuinely interactive — it blocked on `read` from the terminal — and it
  read the line typed into the pty (`read-from-terminal<typed-into-the-tty>`).
  The cost the old comment predicted is zero because the arrangement is not the
  one it predicted for: the child leads a process *group*, not a session, so it
  keeps the controlling terminal and is handed the foreground.
- **Exit status.** In that same run an outside `kill -TERM` on the driver mid-
  session gave the wrapping shell `DRIVER-STATUS=143`.
