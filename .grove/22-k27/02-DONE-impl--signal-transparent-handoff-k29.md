# signal-transparent-handoff-k29

## Goal

Make the handoff a linearization point and make it transparent to signal state.
Cancellation observed before the final check launches nothing and marks the
committed attempt not executed. The harness receives the entry signal mask and
every disposition that survives exec, exactly as the front process inherited
them, including SIGPIPE.

## Context

The spec's `#execution-contract` spells out the order: commit, block the handled
signals, check pending cancellation, then append-and-re-raise or restore-and-exec.
It also states the Rust runtime facts. The standard runtime ignores SIGPIPE
before `main`. Rust's exec path resets SIGPIPE to default before pre-exec
hooks, while keeping the thread's mask. So record the entry SIGPIPE
disposition before runtime initialization changes it, and reinstate it after
that reset. The leaf chooses the mechanism, verifies it against the Rust
source for the workspace's minimum toolchain, and records it in its running log.

## Done when

- After the commit, the handled signals are blocked and pending cancellation is
  checked once more. On cancellation nothing launches, a not-executed detail is
  appended where possible, and the process re-raises. A failed append leaves
  the attempt unknown, never a success.
- Otherwise the entry mask and dispositions are restored and the process execs.
  A caller that ignored SIGPIPE or HUP, or blocked a signal, has that state
  observed unchanged by the fake harness. A caller with default SIGPIPE has
  default observed. Positive control: a fixture that deliberately alters the
  state is seen to change what the harness observes.
- Command-seam tests cover cancellation between commit and exec. They show
  `record show` reporting the attempt not executed, with no harness marker.
- The usage documentation states the one window that remains: a signal between
  restoring the mask and exec can leave the attempt recorded with execution
  unknown.

## Decisions (running log)

**The entry signal state is recorded by a pre-`main` initializer, and
reinstated by a pre-exec hook.** Verified against the Rust source of both
toolchains in play: 1.85.0, the workspace's `rust-version`
(`library/std/src/sys/pal/unix/mod.rs`, `init` → `reset_sigpipe`;
`library/std/src/sys/pal/unix/process/process_unix.rs`, `do_exec`), and
1.98.1, the Homebrew build toolchain (`sys/pal/unix/mod.rs`;
`sys/process/unix/unix.rs`). In both, `lang_start` → `sys::init` sets SIGPIPE
to `SIG_IGN` before `main` unless the unstable `-Zon-broken-pipe` is used, and
`Command::exec` runs `do_exec` in this process: it keeps the calling thread's
mask ("do not call pthread_sigmask"), sets SIGPIPE to `SIG_DFL`, and only then
runs the `pre_exec` closures before `execvp`. So nothing inside `main` can see
the caller's SIGPIPE, and nothing before the closures can restore it. The
front places one function in the executable's initializer section
(`.init_array` on ELF, `__DATA,__mod_init_func,mod_init_funcs` on Mach-O, the
spelling std itself uses for `ARGV_INIT_ARRAY` and the `ctor` crate uses for
Apple). The loader runs it before the C `main` that calls `lang_start`. It
records the thread's mask and, for every signal, whether it was ignored. The
pre-exec hook sets every recorded signal to its entry disposition, then the
entry mask, as the last thing before `execvp`. Only ignored and default
dispositions survive exec, and a caught one is reset by exec anyway, so
reinstating every signal rather than SIGPIPE and the handled three makes the
property true by construction rather than by an audit of what today's
dependencies change. `#![no_main]` was rejected: it would also skip std's
standard-descriptor sanitizing and stack-overflow handler.

**Order at the handoff: check, commit, announce, block, final check.** The
evaluation handlers now stay installed across the commit. After the program is
resolved, a received signal still refuses with `selection_cancelled` and
nothing recorded. Then the commit, then the one-line announcement on stderr,
then the handled signals are blocked in the main thread (the drain threads
already have them blocked), then the final check: the handler's note, or a
handled signal now pending that the caller did not block at entry. The
announcement goes before the linearization point so that a slow stderr cannot
widen the window after it.

**Cancellation after the commit is `handoff_cancelled`, stage `exec`, with a
launch-failure detail of cause `cancelled`.** A distinct code, because unlike
`selection_cancelled` a run now exists; the refusal carries the run note, and
the detail names the signal. It reuses the `launch_failures` table that an
exec error already writes, so `record show` reports `launch_failure` /
`not_executed` and `record observe` refuses a confirmation, with no schema
change. A failed append leaves the attempt `handoff_attempt` / `unknown`, as a
failed exec-error append does. The process then prints its refusal, restores
the entry dispositions and mask, and re-raises.

**A commit failure while a signal is noted is reported as the selection's
cancellation.** Nothing was recorded, so `selection_cancelled` is accurate,
and k28's rule that a received signal decides the outcome carries through to
the linearization point.

**Between commit and exec is tested by stalling the announcement.** The
command seam hands the front a stderr pipe the test has already filled, waits
until the store holds the run, signals, and only then drains. The front
cannot reach its final check before the drain, so the signal lands between
the commit and the check on every run, with no timing assumption. The same
stall without a signal is the positive control.

**The fake harness that observes signal state is a C program compiled by the
test.** A shell rewrites its own dispositions at startup (bash ignores QUIT
for itself), macOS `ps` reports neither ignored nor blocked sets, and a Rust
probe would need the very initializer under test. The C runtime changes
neither, so the probe's `main` reads exactly what exec delivered. The crate
already needs a C compiler for bundled SQLite, so `cc` is present wherever
these tests can build.

**The final check reads `sigpending` as well as the handler's note, and a unit
test covers that half.** A signal arriving between the block and the look
never reaches the handler, so only `sigpending` sees it. No seam can place a
signal in that window, so
`cancellation::tests::a_handled_signal_pending_behind_the_block_cancels` raises
one in the looking thread behind the block.

**Every new test was seen to fail against a mutated front**, and the sources
were restored byte for byte after each (digest checked). No pre-exec hook
fails the mask, ignored-HUP and alter cases: the harness got INT, TERM and HUP
blocked. Capture moved into `run` fails the default case with SIGPIPE ignored,
which is the runtime's change. The hook restoring only the mask fails the
SIGPIPE-ignored case with SIGPIPE default, which is `do_exec`'s reset, seen
rather than read. No mask restore fails the four mask-bearing cases. No final
check launches the harness with exit 0 after a signal the handler caught and
swallowed. No append leaves the run `handoff_attempt`. Handlers dropped before
the commit, as k28 left them, lets the stalled signal kill the front with no
refusal and no detail. No `sigpending` fails the unit test.

**The probe also shows a caller-blocked TERM reaching the harness pending.**
Sent while the policy holds, it neither cancels nor is lost: the harness starts
with TERM blocked and pending. The control, TERM not blocked, cancels.

**The spec's acceptance rows this leaf owns map to named tests.** The
command-seam row's "a caller-ignored HUP or SIGPIPE and the entry signal mask
reach the fake harness unchanged" is
`handoff::the_harness_inherits_the_callers_mask_and_ignored_signals`, with
`a_state_altered_between_the_front_and_the_harness_is_seen` as its positive
control, and `a_signal_the_caller_blocked_reaches_the_harness_pending_and_cancels_nothing`
and `a_signal_the_caller_ignored_does_not_cancel_the_handoff` beside it. The
records row's "cancellation after the commit launches nothing and marks the
attempt not executed" is
`handoff::a_signal_between_the_commit_and_exec_launches_nothing_and_marks_the_attempt_not_executed`,
with `the_same_stall_unsignalled_reaches_the_harness` as control,
`a_cancelled_handoff_whose_detail_cannot_be_appended_stays_unknown` for the
failed append, and `a_cancelled_handoff_in_text_mode_names_its_run_and_the_signal`.
The controlling-PTY row is `grove-dispatch-k31`'s.

**The installed smoke test gains `signal_state`, so the Linux initializer path
is exercised too.** Every command-seam test runs on macOS, which takes the
Mach-O section; `.init_array` would otherwise ship unexecuted. The existing
cases already fail if the initializer never runs, since `run` then refuses
`signal_state_unavailable`. The new case shows the order as well: a harness
that signals itself survives a caller-ignored SIGPIPE or HUP and dies of a
default one, with only `sh`, which the glibc-2.17 userland has. It was seen to
fail against a front that reinstated no disposition (the ignore lost) and one
that recorded its state inside `main` (the default lost).

**A handoff refusal does not repeat the policy's output.** The handoff notice
has already printed it, so, as after an exec error, `handoff_cancelled`
carries no `diagnostics`.

**Measured on the final source.** `task check` passes all 12 principal checks,
with the sources it reads digested unchanged across the run, and
`rustup run 1.85 cargo check --locked --all-targets -p harness-dispatch`
passes. `task release:smoke` passed `signal_state` through both fronts in
three glibc-2.17 Linux userlands: x86_64 under the pinned QEMU at the Nehalem
floor, and aarch64 both in a native container and under QEMU at the
Cortex-A53 floor. The native macOS arm64 run used a
`dispatch.sh install` of this checkout. The Linux archives were built before
the diagnostics change above, which only alters that refusal's JSON and which
no smoke case reaches.
