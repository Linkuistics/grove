# session-kind-tree-review-k24

**Kind:** review-impl
**Reviews:** session-kind-tree-k23
**Producer launch:** {"producer":"session-kind-tree-k23","session":"session-kind-tree-k23","generation":"k23","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `session-kind-tree-k23` and record concrete findings for its integration step.

## Context

- Review `session-kind-tree-k23` against the filename grammar, task-tree scheme,
  pick semantics, promotion contract, and viewer compatibility in
  `docs/specs/config-driven-sessions.md`.
- Attack parser-prefix ambiguity, terminal handling, key monotonicity, foreign
  file lenience, finish starvation, accidental body-field fallback, and any
  current/legacy dual-reader leakage.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-kind-tree-integrate-k25` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, exact reproducer or code evidence,
  and the contract each finding threatens, or an explicit no-finding result.
- The review cites the inspected source, specifications, diff, and producer's
  recorded verification evidence for every conclusion.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-kind-tree-integrate-k25` owns
fixes.
