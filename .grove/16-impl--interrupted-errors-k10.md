# interrupted-errors-k10

## Goal

Make every error that stops the loop keep a caught signal. Grove reports the
error, and then `grove` dies of the signal. This closes the loss that the
review `lifecycle-job-k22` found as F1. `lifecycle-job-k23` triaged that review
and left this work here.

## Context

- The root brief: *Requirements* ("Behaviour that stays", the TERM and HUP
  items) and *The shared function* (the error that carries the interruption).
  Section *Lifecycle interpretation* of the caller contract states the rule.
- The lifecycle in `crates/grove-loop/src/loop_driver.rs` propagates a launch
  error before it reads the latch. The CLI in `crates/grove/src/cli.rs`
  propagates it, so `reraise` never runs.
- The log of `ravel-child-k5` records the reason and the scope of the rule.
- The finding F1 of `lifecycle-job-k22` lists three loss paths. A failed spawn
  or wait carries the signal in `JobError::interrupted()`, and `drive` and the
  CLI never use it. A successful reap can carry `JobEnd::interrupted`, and a
  failed invalidation drops the whole end in the `(Ok(_), Err(_))` arm of
  `complete_post_reap_epoch_handoff`. The latch is already empty by then, so a
  final latch check cannot recover it. When launch and invalidation both fail,
  the launch error survives only as text, so its typed signal is lost.
- A signal can also stay in the latch. A `?` can return before the checks at the
  top of the loop and after the reap consume it.
- The places to change are `loop_driver::run`, `complete_post_reap_epoch_handoff`,
  the error boundary in `lib.rs`, and the reporting and outcome handling in
  `cli.rs`. The audit of the fallible paths covers these steps:
  - lease revalidation, before the transition and before the launch
  - the transition and the selection
  - the construction of the ending commands
  - the allocation of the launch directory and the capture
  - the duplication of stdout
  - the activation of the epoch, and the launch
  - the return of the handoff
- The unit cases of the handoff check diagnostic text and the suppression of
  interpretation today. They do not check that the interruption survives, so
  they do not cover F1.

## Done when

- A launch error that carries a signal keeps the error for the report and ends
  with the signal.
- A failed invalidation after an interrupted job does the same.
- A TERM or HUP between sessions, or after the reap, stops the loop. `grove`
  dies of that signal, even when a later step returns an error.
- `grove run` keeps its exit 1 for each failure.
- Unit cases at the post-reap seam cover the launch error and the failed
  invalidation. They drive `interpret_launch` with a result value that Grove
  owns and need no OS process.
- The book of `grove-loop` and the book `overview` match every changed line.
  The conversions of their chapters land first as their own changes.
- `bash scripts/check.sh` passes. `ravel:activation` passes with `RAVEL` set.

## Notes

- This leaf follows `lifecycle-job-k9`, because that leaf makes the loop
  use the shared function and its error type.
- Look at each `?` of the loop after the handlers exist. A failed pick after a
  TERM between sessions is one example.
