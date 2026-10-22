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
