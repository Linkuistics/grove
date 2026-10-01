# descriptor-seven-flake-k56

## Goal

Make `the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors` in
`crates/harness-dispatch/tests/run.rs` deterministic, so that a green
`task check` does not depend on scheduling.

## Context

`review-selector-k36` met it once, in a full `cargo test --locked -p
harness-dispatch` on a machine also running other groves' sessions. It failed
at `tests/run.rs:181`, the assertion that the file behind descriptor 7 holds
`written through descriptor 7\n`. The run's other assertions before it, exit 0
and the stdin echo on stdout, had passed. The panic's full message was not
captured. On rerun it passed 5 times alone and 6 times with its binary, and
once more in a full package run. k36 changed nothing on that path: it added an
example module to the worker and moved the handoff tests' stall helper into
`tests/support/stall.rs`.

The test hands the front a descriptor 7 through `dup2` in `pre_exec`, and the
fake harness (`FAKE_HARNESS` in `tests/support/mod.rs`) writes to it with
`echo … >&7` after `cat` has copied its stdin. Where the bytes can go missing
is the question: how `through` is opened and read, whether another test's
child can hold or replace descriptor 7, and whether the read races the write.
That is not yet known; this leaf starts by reproducing it.

The flakes this repository has fixed before each became their own leaf, and
their fixes conditioned on process state rather than time:
`driver-lease-readiness-flake-k145`, `cleanup-barrier-readiness-flake-k165`
and `noninteractive-stdin-flake-k55`.

k55 needed no wait: it put the caller's input in place before the process
that could race it existed. Its running log has the method, a swept delay
that found the window, and one finding that may bear here. An `unwrap` that
ran before the child's status was read reported any early death of that child
as a broken pipe, so the first failure seen was not the cause.

## Done when

- The failure is reproduced deliberately, its message captured, and the cause
  named.
- The test no longer depends on the losing interleaving, and still proves
  what it proves: the harness keeps the caller's stdin, stdout and descriptor
  7, and inherits none of the front's own descriptors.
- A control shows the descriptor assertion still fails when the front closes
  or replaces the caller's descriptor 7.
- `task check` passes.
