# launch-cutover-k19

**Reviews:** launch-cutover-k15
**Creator:** run e81ed5ab-1c47-42e6-9fe5-226f17dc4cc2

## Goal

Adversarially read the implementation that `launch-cutover-k15`'s three leaves
produced against the spec, the root brief's guarantees and the methodology it
now states, and report findings. Fix nothing.

## Context

- The artifact is the commits of `launch-directory-k16`, `dispatch-ending-k17`
  and `signal-contract-k18`. Each commit message names its handle.
- The contract: the spec's *Grove integration*, decisions 7 and 9 of
  `docs/specs/module-decomposition.md`, and
  `docs/adr/one-live-driver-per-working-tree.md`.
- Where the producer was least certain:
  - the stale-session guarantee now that admission keys on the launch
    directory, including a stale session's `record-teardown` and exit signal
    against a later launch;
  - the reading table's precedence: a teardown record finishes whatever the
    ending, and only `exit_signal` relaunches;
  - a replacement driver removing abandoned launch directories while reading
    nothing in them;
  - the terminal after dispatch's death, and a driver in the background;
  - the 10 s bound against dispatch's end;
  - whether every ending the prompt and the shipped skills now describe is one
    the driver actually reads that way, the finish's two commands above all.

## Done when

- Every finding is recorded with its location and why it matters, or the
  review records that it found none.
- If any finding warrants action, an `integrate-review-impl` leaf with this
  stem is `leaf-insert`ed before the next root-level sibling with live work,
  so the documentation and the release describe an agreed cutover.
