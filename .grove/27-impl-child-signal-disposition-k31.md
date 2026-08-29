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
