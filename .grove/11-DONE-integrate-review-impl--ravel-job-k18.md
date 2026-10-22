# ravel-job-k18

**Integrates:** ravel-job-k17

## Goal

Triage the findings of `ravel-job-k17` and apply the valid corrections to the
implementation produced by `ravel-job-k7`.

## Context

- Read the findings from the committed review `ravel-job-k17`.
- The producer is `ravel-job-k7`. The root brief supplies the caller contract
  and the agreed test seams.

## Done when

- Each finding has a recorded classification and its supporting evidence.
- Valid corrections include the corresponding source-book changes.
- Required checks pass for the corrected implementation.

## Notes

This step precedes both caller migrations. The findings remain in the review,
so this task can reject a finding when the evidence warrants rejection.

## Decisions (running log)

- **F1 — real issue; fixed.** The producer wrote `interrupted.or_else(take_interrupt)`
  at the reap, and `failure.sent.or_else(take_interrupt)` in the error
  constructor. `or_else` is lazy, so both skipped the drain once a signal had
  been sent. The old runner drained unconditionally at `run.rs:1344`. This
  leaf restores that. One private function, `freeze`, now drains the latch and
  keeps the earlier signal. Both boundaries call it. A unit test in the module
  stores HUP in the latch and checks both orders. It was seen to fail with the
  lazy form. The race has no black-box test, because no caller can time the
  window between the last latch check and the reap.
- **F2 — real issue; fixed.** The two installed cases have no terminal, so a
  disabled handover passed them. `tests/ravel_job_terminal.rs` runs the
  foreground shape on a pseudo-terminal of its own against the installed
  Ravel. The harness runs `ps` on itself and reports its group and the group
  that holds the terminal. The case checks that the harness leads a group of
  its own, that the group holds the terminal, and that the terminal is back
  with Grove after the reap. It was seen to fail with both `Lease` handover
  paths disabled. `task ravel:job` runs it, and `ravel:activation` includes
  that task. No stand-in child takes part.
- **Claims that held.** The reviewer reported six claims as held. This leaf did
  not re-derive them. Nothing that it read contradicts them, so none changes.
- **Book.** Chapter 19 of the `grove-loop` book quotes the new `freeze`
  function and the unit test, in a new fragment `ravel-job-tests`. The ranges,
  the source index, the manifest and the totals follow (757 lines, 15,273 in
  all). The prose of the latch section and of the evidence section now says
  what is checked. `book-specs-k14` records the new block in the structure
  spec, as the log of `ravel-job-k7` says.
- **For the next leaves.** `ravel_job::launch` is unchanged in signature. The
  migration leaves keep `tests/ravel_job_terminal.rs`, since it checks the
  function and not a caller. The contract leaf removes `test:keyed-launch` from
  `ravel:activation`, beside the new `ravel:job` line.
