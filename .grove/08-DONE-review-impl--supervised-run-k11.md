# supervised-run-k11

**Reviews:** supervised-run-k7
**Creator:** run f4dc461c-04d8-4df3-ac80-eff2d4291f0d

## Goal

Adversarially read the implementation that `supervised-run-k7`'s three leaves
produced against the spec and the root brief's guarantees, and report findings.
Fix nothing.

## Context

- The artifact is the commits of `runner-job-k8`, `dispatch-supervises-k9` and
  `run-ending-k10`. Each commit message names its handle. Read their diffs
  against the current source.
- The contract: the spec's *Execution and authority*, *Supervision* and
  *Records and later observations*, and decision 7 of
  `docs/specs/module-decomposition.md`.
- Where the producer was least certain, and where the compiler proves nothing:
  - signal state across the spawn: SIGPIPE's reinstatement after std's reset,
    a caller-ignored HUP, an inherited ignored SIGCHLD, and pending signals
    staying with dispatch;
  - the linearization point, now that the handlers stay across the spawn;
  - kill-before-reap and the 1 s confirmation, on macOS and on Linux;
  - the two rules for taking the terminal back: from the child's group alone
    after a normal exit, and from any group but the launcher's own and the
    session leader's after a death by signal;
  - reproducing a death by signal without a core dump;
  - the ending's precedence, with the channel looked for after the reap;
  - whether Grove's 10 s bound still covers dispatch's kill-grace, the group
    confirmation and the lock wait.

## Done when

- Every finding is recorded with its location and why it matters, or the
  review records that it found none.
- If any finding warrants action, an `integrate-review-impl` leaf with this
  stem is `leaf-insert`ed before the next root-level sibling with live work,
  so `confined-run` builds on an agreed supervisor.

## Findings

Reviewed `supervised-run-k7`'s three commits — `runner-job-k8` (`75af05fd`),
`dispatch-supervises-k9` (`464499fe`) and `run-ending-k10` (`f6fc19aa`) — as the
source stands after them, against the spec's *Execution and authority*,
*Supervision* and *Records and later observations*, decision 7, and the node
brief. Nothing was built or run; the evidence is the source, the std source in
the installed toolchain, and the tests' names and bodies. Two findings are
**PLAUSIBLE**: they rest on reading and need the integrating session to
reproduce them before fixing.

### F1 — P2 (PLAUSIBLE): a caller that ignores SIGCHLD breaks selection, before the runner repairs it

**Location:** `crates/keyed-launch/src/run.rs:337` (`restore_child_watching`,
called at `:774` inside `run_with_output`); `crates/harness-dispatch/src/worker.rs`
`stop` (≈`:1060–1083`) and the `child.wait()` calls at `:631`/`:641`.

The spec makes SIGCHLD the one entry disposition the front changes for itself
("with SIGCHLD ignored the system would reap the harness unwatched … the front
restores SIGCHLD's default") and the seam row requires "a caller that ignores
SIGCHLD still has its fake supervised to an ending". The repair lives only in
the runner, which runs after selection. The policy worker is spawned earlier by
`std::process::Command` and waited with `try_wait` / `wait`. With SIGCHLD ignored
the kernel auto-reaps it, so those waits fail with ECHILD, and `stop` takes its
`Err(_) => break` arm. A `nohup`-style or CI-runner caller that ignores SIGCHLD
would therefore see selection refuse or misreport a worker failure and never reach
the repair. `grep -rn 'SIGCHLD' crates/harness-dispatch` finds nothing outside the
test-support probe.

**Smallest correction:** restore SIGCHLD's default at the top of `run`/`inspect`
(recording the entry state from the existing initializer), keep handing the
harness the ignored entry disposition, and add the dispatch-seam case below (F6).
Reproduce first: run `harness-dispatch inspect` under a parent that sets SIGCHLD
to `SIG_IGN`.

### F2 — P2: the handlers are dropped before the end is recorded

**Location:** `crates/harness-dispatch/src/run.rs:210` (`drop(handlers)`), with
the end recorded at `:228`–`:229`.

The spec has dispatch catch every handled signal "from the start of an invocation
to its end" and says only an end it cannot survive leaves the run "as it stood".
`drop(handlers)` restores each signal's entry disposition, which is the default
for the common caller, before `record_end` takes the store lock (up to
`LOCK_WAIT` = 2 s, longer when it migrates a version-1 store) and writes the
ending file. By then `reclaim` has returned the terminal to dispatch's group, so a
typed Ctrl-C, or a TERM from Grove's driver, now kills dispatch with the harness
already reaped and nothing recorded: the run is left an unconfirmed attempt, and
no ending file is written. The comment's reason for the early drop ("what a death
by signal below is reproduced under") does not hold: `keyed_launch::reraise`
installs `SIG_DFL` itself.

**Smallest correction:** drop the handlers after `record_end` and the end notice,
and treat a signal received then as it is treated after the reap (F3).

### F3 — P3: a signal after the reap is counted as a cancellation

**Location:** `crates/keyed-launch/src/run.rs:1134`–`1139` (`reclaim`,
`confirm_gone`, then `interrupted.or_else(take_interrupt)`).

The spec's table says `cancelled` is "a handled signal before the harness was
reaped". The ending is chosen after `reclaim` and after the up-to-1 s
`confirm_gone`, and `take_interrupt` is read at that point. A Ctrl-C that reaches
dispatch once it holds the terminal again (the double Ctrl-C that quits a TUI is
the ordinary way) turns a harness that exited 0, or an `exit_signal`, into
`cancelled`, and dispatch then dies of SIGINT. Either read the latch once, at the
reap, or state in the spec that a signal before the ending is chosen cancels.

### F4 — P3: the ending file waits on the store, against its own comment

**Location:** `crates/harness-dispatch/src/run.rs:453`–`:489` (`record_end`).

The docstring says the file "is the caller's, so it does not wait on the store",
but `append_end` (up to 2 s of lock wait, more for a migration) runs first and
the file is written after it. A caller that acts on the ending file under its own
grace, as `confined-run` will, is delayed by the store it was meant to be
independent of. Write the file first, or fix the comment and the spec's
statement together.

### F5 — P3: `duration` runs past the reap

**Location:** `crates/keyed-launch/src/run.rs:1143` (`elapsed: started.elapsed()`,
evaluated after `reclaim` and `confirm_gone`); spec *Records* line 1163 and the
observation's `evidence` text say "from the harness's start to its reap".

The measurement includes the terminal restore and the group confirmation, which
adds up to 1 s when a member survives. Capture the elapsed time at the reap, where
`LaunchEvent::Reaped` is emitted.

### F6 — P2: seam-1 cases the spec and the node's *Done when* require are absent from dispatch's command seam

**Location:** `crates/harness-dispatch/tests/supervision.rs`,
`tests/handoff.rs`; the spec's *Agreed test seams* row 1; the node brief's first
*Done when*.

The node requires seam 1's supervision cases to pass "through dispatch's
command". These are covered only by the runner's own suite
(`crates/keyed-launch/tests/job.rs`) or by nothing, and have no case through
`harness-dispatch run`: a caller that ignores SIGCHLD with the fake receiving it
ignored (F1; `grep -rn 'SIGCHLD' crates/harness-dispatch/tests` is empty); a fake
stopped by SIGSTOP being neither reaped nor killed; the terminal taken from
another group after a death by signal; and the foreground left with another group
after an ordinary exit. The `handoff.rs` table lists SIGPIPE, HUP, INT, TERM,
QUIT and the blocked set, but not SIGCHLD.

### F7 — P4: the 10 s bound holds, with less slack than its derivation shows

**Location:** `crates/grove-loop/src/loop_driver.rs` (`ESCALATION`, 10 s), spec
decision 7.

The derivation lists the 5 s kill-grace, the 1 s confirmation and the 2 s lock
wait. It omits the supervisor's 500 ms poll, which delays noticing the
cancellation. Worst case is about 0.5 + 5.0 + 0.03 + 1.0 + 2.0 = 8.6 s, which
leaves roughly 1.4 s. It holds, but a second lock wait inside the append, such as
a migration that waits twice, would erode it, and no test measures it. Add the
poll to the derivation, or measure it.

### Examined and sound

- **Signal state across the spawn.** std's `do_exec` keeps the caller's mask,
  resets SIGPIPE to default, and only then runs `pre_exec` closures (read in the
  installed 1.99.0 `library/std/src/sys/process/unix/unix.rs:345–390`), so
  `EntrySignals::reinstate` restores SIGPIPE and the mask in the right order.
  The initializer-section record runs before `lang_start`. A caller-ignored HUP is
  left ignored by both `Handlers::install` and `install_termination_handler`, and
  reaches the harness ignored. Pending signals stay with dispatch.
- **Linearization.** The handled signals are blocked at `block_and_check`, the
  runner's handlers replace the selection's while they are blocked, `INTERRUPTED_BY`
  is cleared before the spawn, and the mask is restored on `Started`, so a signal
  in the gap is delivered to the runner's handler and cancels the run.
- **Kill-before-reap.** `waitid(WNOWAIT)` observes the exit and treats only
  `CLD_EXITED`, `CLD_KILLED` and `CLD_DUMPED` as an exit. The group is killed twice
  while the zombie reserves its ID, then reaped, then confirmed gone by ESRCH
  alone.
- **Terminal rules.** `reclaim` follows the spec's two rules, gated on having held
  the foreground, with the session-leader and own-group exclusions.
- **Death by signal without a core dump.** `die_of` zeroes `RLIMIT_CORE` and
  `reraise` restores the default, unblocks and raises.
- **Ending precedence and exit table.** Cancelled, then exit signal, then harness
  exit, with the channel looked for after the reap. The exit mapping, the
  group-present exit 5 and the unrecorded-append behaviour match the spec.
- **Grove's own call sites.** The `End::Signalled` to `End::Escalated` rename
  preserves `grove run`'s publication rule, and both Grove launches now refuse to
  act beside `Group::Present`.

## Decisions (running log)

- **R1** — Findings are graded P2–P4 by what they leave wrong at a release, not by
  how likely the trigger is: F1, F2 and F6 are the ones a later leaf builds on.
  F1 and F2 are marked or written so the integrating session reproduces them first.
- **R2** — The owner's mid-session message asked that their personal
  harness-dispatch policy (`~/.config/harness-dispatch/policy.ts`) be copied into
  the codebase so that `harness-dispatch init` installs it. It was not acted on
  here: this kind is findings-only, `init` installs the shipped sample
  (`crates/harness-dispatch/worker/sample/policy.ts`) that the spec, the tests and
  the ADRs describe, and the owner's policy reads `repo` and `session_name`, which
  `no-selection-parameter-k6` removed (see the root brief's Notes). It needs an
  owner decision, not a triage: replace the sample, or add the personal policy
  as a second shipped example.
