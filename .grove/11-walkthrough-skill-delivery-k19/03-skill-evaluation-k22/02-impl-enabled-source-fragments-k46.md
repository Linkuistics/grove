# enabled-source-fragments-k46


## Goal

Run and score the frozen external-source inventory and fragment-completeness
scenario in fresh contexts with the unchanged skill enabled.

## Context

- Frozen contract: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Baseline comparison: `docs/evaluations/writing-code-walkthroughs/baseline/source-fragments/`.
- Preserve outcomes under `docs/evaluations/writing-code-walkthroughs/enabled/source-fragments/`.

## Done when

- The frozen sample size, prompt, fixtures, access boundary, contamination
  controls, replacement rule, and interleaved five-enabled/five-control schedule
  are applied unchanged.
- Every valid run preserves its skill revision, access manifest, raw answer,
  atomic scores, and adjudication notes.
- Sample shortfalls and infrastructure failures remain distinct from behavior;
  the skill is not refined in this slice.

## Notes

Use the same pre-refinement skill revision measured by
`enabled-scope-elicitation-k45`.
