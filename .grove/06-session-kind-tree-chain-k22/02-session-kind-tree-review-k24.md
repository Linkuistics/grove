# session-kind-tree-review-k24

**Kind:** review-impl
**Reviews:** session-kind-tree-k23

## Goal

Adversarially review `session-kind-tree-k23` and record concrete findings for its integration step.

## Context

- Review `session-kind-tree-k23` against the filename grammar, task-tree scheme,
  pick semantics, promotion contract, and viewer compatibility in
  `docs/specs/config-driven-sessions.md`.
- Attack parser-prefix ambiguity, terminal handling, key monotonicity, foreign
  file lenience, finish starvation, accidental body-field fallback, and any
  current/legacy dual-reader leakage.

## Done when

- Findings are recorded here with severity, exact reproducer or code evidence,
  and the contract each finding threatens, or an explicit no-finding result.
- Representative tree operations are exercised through the public
  `grove-llm` seam, including malformed and terminal trees.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-kind-tree-integrate-k25` owns
fixes.
