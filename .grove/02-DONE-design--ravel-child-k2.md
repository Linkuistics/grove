# ravel-child-k2

## Goal

Design how Grove starts `ravel` as an ordinary child process after the
`keyed-launch` crate is gone. Settle the five points that the issue gives to the
design, within the *Requirements* of the root brief. Put the design into the
records where it belongs, and cut the leaf that follows this one.

## Context

- The root brief: the owner settled its *Requirements* in `plan-k1`. Do not ask
  those questions again.
- `plan-k1`: its running log gives the reason for each decision.
- Linkuistics/grove#22, section *Known work*: the five points.
- The two call sites: the lifecycle start of `ravel` in the loop driver of
  `grove-loop`, and the standalone start in `grove run`.

## Done when

- Each of the five points has a decision:
  - The handling of SIGINT, SIGTERM and SIGHUP while `ravel` runs, and the
    report of an interrupted session.
  - The start and reap notices for the driver lease.
  - The replacement for `Group::Present`. The owner settled it: a successful
    Ravel status with an exact result, and no check by Grove.
  - The scrub list and the `GROVE_LAUNCH_DIR` grant.
  - The removal of the crate, with the list of its tasks, tests and documents,
    and the list of records to rework.
- Each production use of the crate has a new home or a reason to go. These uses
  are `Argv`, `LaunchEvent`, `Channel::discard_abandoned`, `regular_file_at`,
  `take_interrupt` and `reraise`.
- The process arrangement of `grove run` has a decision, within the behaviour
  that the root brief keeps.
- The design is in the records where it belongs. These are the caller contract
  spec and the ADRs that the design reworks. Where no agreement point needs a
  spec, the root brief holds the design.
- The next leaf exists. It is a `planning` leaf, or a `review-design` leaf first
  when the design needs an adversarial read.

## Notes

- The driver ignores SIGINT. Ravel gives its harness the dispositions that Ravel
  inherited, so the shared function must reset the job-control signals before
  `exec`. A plain spawn would make Ctrl-C stop working in the harness.
- Ravel lends the terminal only when its own process group holds the
  foreground. A Ravel in a background group takes no foreground for itself.
- Ravel does not publish its runner as a library, so Grove takes no dependency
  on it.
- The job-control checks need a pseudo-terminal and the real Ravel. The tests in
  `crates/grove/tests/loop_driver.rs` already drive the real Grove process with
  the installed Ravel and fake agents, and some open a pseudo-terminal. Prefer
  that harness. The installed fixtures of `grove-loop` open no pseudo-terminal.
- The `signal-` channel files that `Channel::discard_abandoned` removes came
  from drivers older than Ravel. Current Grove creates none. Grove performs no
  migration, so decide whether the cleanup stays.

## Decisions (running log)

- **One shared function with two job shapes (design, 2026-10-22).** A new
  module of `grove-loop` starts `ravel` for both callers. The **foreground
  shape** serves the lifecycle and the **detached shape** serves `grove run`.
  The shape is an argument of the one function, so the two callers share every
  other step.
- **The foreground shape is the current interactive job (design,
  2026-10-22).** `ravel` leads a process group in the session of Grove. It
  takes the foreground when the group of Grove holds it. Stdin and stderr are
  inherited, and stdout goes to the held result file.
- **The detached shape is the current noninteractive job (design,
  2026-10-22).** `ravel` leads a new POSIX session with stdin at end of file.
  Stdout goes to the held result file and stderr to the transcript. Inherited
  descriptors above stderr close at exec. `grove run` keeps the terminal, so its
  relay writes from the foreground group.
- **One latch catches INT, TERM and HUP, except an ignored signal (design,
  2026-10-22).** The rule is the same for both shapes. The driver ignores
  SIGINT, so Ctrl-C never ends the driver. `grove run` leaves SIGINT at its
  default, so Ctrl-C cancels it. The handlers stay after the call.
- **Grove forwards a signal to the `ravel` process only (design,
  2026-10-22).** The cancellation contract of Ravel names its CLI process.
  Ravel stops its own policy worker and its managed job. Grove signals no
  group.
- **Grove kills nothing and waits without a bound (design, 2026-10-22).** The
  owner removed the KILL bounds. Grove forwards each latched signal at each
  poll, reports the first, and waits until `ravel` exits.
- **The spawn never clears the latch (design, 2026-10-22).** The runner of
  today clears it before the spawn. A TERM after the last check of the loop is
  then lost, and the session runs. Now the next poll forwards it to `ravel`.
- **The reap freezes the interruption (design, 2026-10-22).** At the reap the
  function drains the latch into its report. A later signal belongs to the
  caller: the post-reap check of the loop, or the publication checks of
  `grove run`.
- **The child gets the default for the seven job-control signals (design,
  2026-10-22).** These are INT, QUIT, TERM, HUP, TSTP, TTIN and TTOU. SIGCHLD
  gets its entry state. Each other disposition and the mask pass through, as
  the current harness probe requires.
- **The terminal rules do not change (design, 2026-10-22).** The gate, the
  handover in both processes, the re-lend at each poll, the saved modes and the
  three reclaim rules move unchanged. Ravel describes the matching rules for its
  own side.
- **The notices keep their contract (design, 2026-10-22).** The function takes
  an observer. Started comes once after a successful spawn. Reaped comes once
  after a successful wait, before terminal recovery. A failed wait sends no
  Reaped and no KILL.
- **The end of the job is status, interruption and elapsed time (design,
  2026-10-22).** It has no group field. Rule 4 of the lifecycle interpretation
  and the survivor guard of `grove run` go, as the owner settled.
<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- **One scrub list, and the shape carries the grant (design, 2026-10-22).** The
  ten names live once, in the new module, which scrubs them from every job. A
  helper scrubs them from the version probe and from `stty`. Only the
  foreground shape holds a launch directory, so only it grants
  `GROVE_LAUNCH_DIR`.
- **Each production use of the crate has a new home or a reason to go (design,
  2026-10-22).** `Argv` becomes a value type in `session_command`.
  `LaunchEvent`, `take_interrupt` and `reraise` move into the new module.
  `regular_file_at` moves into the standalone module of `grove`, its only
  caller.
- **The cleanup of `signal-` files goes (design, 2026-10-22).** No current
  Grove writes them, nothing reads them, and the epoch names only launch
  directories. Grove performs no migration. The cleanup of abandoned launch
  directories stays.
- **The design needs an adversarial read before planning (design,
  2026-10-22).** It changes signal and terminal contracts in two ADRs and the
  caller spec. A `review-design` leaf comes first, and a `planning` leaf
  follows it.
- **The ADR on the driver lease waits for the implementation (design,
  2026-10-22).** Its decision does not change. Only a citation and the name of
  the notices change, and they follow the shipped code. The root brief lists it
  with the other records to rework.
