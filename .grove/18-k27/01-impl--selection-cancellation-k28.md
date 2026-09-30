# selection-cancellation-k28

## Goal

Cancel selection on a signal. A worker interrupted by INT, TERM or HUP is
stopped and reaped, nothing launches, and the caller sees the re-raised
signal. The whole-selection deadline keeps working beside the handlers.

## Context

The spec's `#execution-contract` and its resource table own the contract. The
worker joins the caller's existing job, and neither process creates a session
or detaches. Signal cancellation reaches ordinary descendants through the
enclosing process group. The deadline, its hard kill and exit 124 arrived with
`selection-deadline-k44` in static dispatch. `computed-selection-k21` and
`bounded-context-k22` extended its timeout cases to their callbacks. This leaf
adds the handled signals and must not weaken the deadline.

## Done when

- INT, TERM and HUP received while evaluating stop and reap the worker, launch
  nothing, and end by restoring the entry disposition and re-raising. A signal
  ignored at entry gets no handler and cannot cancel selection.
- The existing timeout cases still exit 124 with the handlers installed.
- Cancellation is checked after the result arrives, after descriptor close and
  reap, after choice validation and after executable resolution.
- Command-seam tests interrupt import, the loader and the callback. They
  assert that no fake-harness marker appears, no worker process survives, and
  the signal is re-raised. Positive control: the same fixtures without
  interruption do reach the harness.
- Structured `--json` output stays a single clean error on interruption.
