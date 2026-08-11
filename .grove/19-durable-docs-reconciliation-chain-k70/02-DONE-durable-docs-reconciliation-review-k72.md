# durable-docs-reconciliation-review-k72

**Kind:** review-impl
**Reviews:** durable-docs-reconciliation-k49
**Producer launch:** {"producer":"durable-docs-reconciliation-k49","session":"glossary-definition-compression-k167","generation":"k167","harness":"claude","model":"opus"}

## Goal

Adversarially review `durable-docs-reconciliation-k49` and record concrete findings for its integration step.

## Context

- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `durable-docs-reconciliation-integrate-k73` owns every
  fix and all post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The review relies on the producer's recorded verification evidence; no test,
  build, lint, or format command is run.
- No production or test code is changed.

## Notes

## Findings

1. **Medium — the legacy-claim sweep suppresses every later occurrence once a
   file/token pair is classified.** `tests/legacy_claim_sweep.rs:269-276`
   returns early for a token whenever `REFUTATIONS` contains `(path, token)`;
   the table already exempts, for example, `("docs/USAGE.md", "diversity
   warnings")` at line 154. Adding an affirmative current-state sentence such
   as “Grove emits diversity warnings” anywhere else in that same file therefore
   produces no finding, and the stale-entry check still passes because the
   legitimate denial remains. This contradicts the module claim at lines 18-20
   that a reintroduced affirmative claim is caught, and the positive control at
   lines 407-440 does not exercise the blind spot because it uses `grove do`, a
   token not classified for `docs/USAGE.md`. Classify individual occurrences or
   add a same-file/same-token control that distinguishes the permitted denial
   from a newly affirmative claim.

2. **Medium — the compressed glossary gives an incorrect interrupted-finish
   recovery invariant.** `CONTEXT.md:34-35` says that a declined *or
   interrupted* finish leaves the same finish leaf live for later resume. That
   is true only before teardown, or after a transaction that proves rollback.
   `docs/USAGE.md:201-208` records the other mid-transaction result: recovery
   may remain blocked; `docs/adr/one-live-driver-per-working-tree.md:92-101`
   records the post-commit result: if deletion commits and the driver dies
   before observing `done`, `.grove/` is absent and a later bare invocation
   initializes a new grove. Restore those distinctions in the `_Avoid_` line so
   the load-bearing glossary does not promise a selectable leaf in states where
   the transaction or exact committed deletion says otherwise.

## Verification basis

- Inspected the producer commits `59605d19`, `3cdceaaa`, `d3b8a3ec`, and
  `7d315b72`, their diffs, current documentation/specifications, relevant source,
  and the evidence recorded in each commit message.
- Relied on the producer's recorded `cargo fmt --check` and
  `cargo test --locked` results. Per this review's mandate, ran no test, build,
  lint, or format command and changed no production or test code.
