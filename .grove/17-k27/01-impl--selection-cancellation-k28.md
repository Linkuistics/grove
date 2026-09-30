# selection-cancellation-k28

## Goal

Bound and cancel selection. A worker that hangs, spins or is interrupted is
stopped and reaped, nothing launches, and the caller sees the documented exit
or the re-raised signal.

## Context

The spec's `#execution-contract` and its resource table own the contract. The
worker joins the caller's existing job, and neither process creates a session
or detaches. Signal cancellation reaches ordinary descendants through the
enclosing process group.

## Done when

- `--timeout-ms` accepts 1 to 120 seconds, and the default is 30. The bound runs
  from worker start to result. A policy stuck in import, in `loadContext`, in a
  synchronous loop or in a never-settling `select` is killed. That means a
  grace of at most one second, then KILL. It exits 124 with a structured
  diagnostic, and the fake harness never starts.
- INT, TERM and HUP received while evaluating stop and reap the worker, launch
  nothing, and end by restoring the entry disposition and re-raising. A signal
  ignored at entry gets no handler and cannot cancel selection.
- Cancellation is checked after the result arrives, after descriptor close and
  reap, after choice validation and after executable resolution.
- Command-seam tests interrupt import, the loader and the callback. They cover a
  timeout in each, and assert that no fake-harness marker appears, no worker
  process survives, and the exit is correct. Positive control: the same
  fixtures without interruption do reach the harness.
- Structured `--json` output stays a single clean error on interruption.
