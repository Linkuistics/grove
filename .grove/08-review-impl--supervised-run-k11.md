# supervised-run-k11

**Reviews:** supervised-run-k7
**Creator:** run f4dc461c-04d8-4df3-ac80-eff2d4291f0d

## Goal

Adversarially read the implementation that `supervised-run-k7`'s three leaves
produced against the spec and the root brief's guarantees, and report findings.
Fix nothing.

## Context

- The artifact is the commits of `runner-job-k8`, `dispatch-supervises-k9` and
  `run-ending-k10`. Each commit message names its handle. Read their diffs
  against the current source.
- The contract: the spec's *Execution and authority*, *Supervision* and
  *Records and later observations*, and decision 7 of
  `docs/specs/module-decomposition.md`.
- Where the producer was least certain, and where the compiler proves nothing:
  - signal state across the spawn: SIGPIPE's reinstatement after std's reset,
    a caller-ignored HUP, an inherited ignored SIGCHLD, and pending signals
    staying with dispatch;
  - the linearization point, now that the handlers stay across the spawn;
  - kill-before-reap and the 1 s confirmation, on macOS and on Linux;
  - the two rules for taking the terminal back: from the child's group alone
    after a normal exit, and from any group but the launcher's own and the
    session leader's after a death by signal;
  - reproducing a death by signal without a core dump;
  - the ending's precedence, with the channel looked for after the reap;
  - whether Grove's 10 s bound still covers dispatch's kill-grace, the group
    confirmation and the lock wait.

## Done when

- Every finding is recorded with its location and why it matters, or the
  review records that it found none.
- If any finding warrants action, an `integrate-review-impl` leaf with this
  stem is `leaf-insert`ed before the next root-level sibling with live work,
  so `confined-run` builds on an agreed supervisor.
