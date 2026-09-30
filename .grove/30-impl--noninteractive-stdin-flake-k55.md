# noninteractive-stdin-flake-k55

## Goal

Make `standalone_child_has_a_new_session_and_cannot_consume_callers_stdin` in
`crates/keyed-launch/tests/noninteractive.rs` deterministic, so that a green
`task check` does not depend on scheduling.

## Context

`dispatched-terminal-k33` met it once, in a full `task check` on a machine
also running other groves' sessions. It passed 15 times in a row on rerun,
alone and with its binary. It failed at line 327, the parent's write of
`parent input` to the supervisor's piped stdin:

    called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }

The write happens only after `spawn` returns. The supervisor runs its child
with null stdin and never reads its own, so if the parent is descheduled long
enough, the supervisor, and every holder of the pipe's read end, can exit
first. That is a hypothesis from reading the test, not yet a reproduction.

The flakes this repository has fixed before each became their own leaf, and
their fixes conditioned on process liveness rather than time:
`driver-lease-readiness-flake-k145` and `cleanup-barrier-readiness-flake-k165`.

## Done when

- The race is reproduced deliberately, for example by delaying the parent's
  write, and the hypothesis above is confirmed or replaced.
- The test no longer depends on the write winning. Whatever it proves about
  the caller's stdin, it still proves: the child consumes none of it.
- A control shows the stdin assertion still fails when the child can read the
  caller's input.
- `task check` passes.
