# interrupted-errors-k10

## Goal

Make every error that stops the loop keep a caught signal. Grove reports the
error, and then `grove` dies of the signal. This closes the loss that the
review `ravel-child-k3` found as F1.

## Context

- The root brief: *Requirements* ("Behaviour that stays", the TERM and HUP
  items) and *The shared function* (the error that carries the interruption).
  Section *Lifecycle interpretation* of the caller contract states the rule.
- The lifecycle in `crates/grove-loop/src/loop_driver.rs` propagates a launch
  error before it reads the latch. The CLI in `crates/grove/src/cli.rs`
  propagates it, so `reraise` never runs.
- The log of `ravel-child-k5` records the reason and the scope of the rule.

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
