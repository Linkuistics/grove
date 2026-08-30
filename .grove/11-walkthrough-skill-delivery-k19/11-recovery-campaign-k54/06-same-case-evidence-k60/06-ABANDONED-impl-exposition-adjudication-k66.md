# exposition-adjudication-k66

## Goal

Audit and publish the exact historical-rubric exposition/assurance result.

## Context

- Assignment records: `exposition-runs-k65`.
- Frozen criteria, auditor, scorer instruments, and outcome rules:
  `campaign-freeze-k59`.

## Done when

- Complete attempt history is checked against the historical Case C no-tool,
  replacement, sampling, and scoring rules. Only the historical C01–C24 rows
  determine the acceptance result.
- New exposure classifications, dual scoring, forced arm guesses, and added
  assurance rows are excluded and cannot alter a historical row.
- The case record reports the historical C01–C24 descriptive counts,
  invalid-attempt counts, truncations, discovery count, and required
  adjudication disagreements. Because Case C has no valid historical baseline
  operand, materiality and regression classifications are published as
  `undefined`, never zero, miss, or omission.
- Historical valid, invalid, truncated, and shortfall outcomes retain the exact
  rubric meaning. New fail-closed semantics are out of scope.

## Notes

Judged plans to run deterministic checks remain answer behavior. They are not
reported as proof that a future walkthrough would pass those checks.
