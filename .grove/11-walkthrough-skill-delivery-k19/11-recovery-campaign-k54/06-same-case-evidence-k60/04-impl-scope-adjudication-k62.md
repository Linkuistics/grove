# scope-adjudication-k62

## Goal

Audit and publish the exact historical-rubric scope-elicitation result.

## Context

- Assignment records: `scope-runs-k61`.
- Auditor, scorer instruments, criteria, and outcome rules: joint campaign
  freeze from `campaign-freeze-k59`.

## Done when

- Before acceptance scoring, the auditor proves that every attempt and
  replacement follows the historical Case A no-tool and sampling rules. The
  historical atomic rows and adjudication contract alone determine acceptance
  scores.
- Dual scoring, forced arm guesses, exposure classifications, and fail-closed
  protocol outcomes absent from the historical rubric are excluded.
- The case record reports the historical rubric's per-row counts, material
  deltas, `R`/`G` classifications, invalid-attempt counts, truncations,
  discovery count, and required adjudication disagreements only.
- Historical complete, invalid, truncated, and shortfall endings retain their
  exact rubric meaning. No cross-case acceptance claim is made here.

## Notes

Full raw events remain in the audit record and outside the behavioral scoring
bundle except for normalized case evidence the frozen criteria require.
