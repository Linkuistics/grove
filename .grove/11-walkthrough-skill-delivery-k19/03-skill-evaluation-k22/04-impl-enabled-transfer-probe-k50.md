# enabled-transfer-probe-k50


## Goal

Select, freeze, and run the rubric's out-of-sample transfer probe with paired
contemporaneous no-skill and skill-enabled contexts.

## Context

- Transfer contract in the freeze-boundary section of
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Exclude the skill bytes, completed book, baseline outcomes, selected codebase,
  and campaign artifacts from each selection stage exactly as the rubric states.
- Preserve outcomes under `docs/evaluations/writing-code-walkthroughs/enabled/transfer-probe/`.

## Done when

- A fresh selector freezes a bounded codebase and prompt from only the allowed
  subject and transfer constraints.
- A separate criterion author freezes case-specific criteria before receiving
  the selected source.
- Five contemporaneous no-skill and five enabled repetitions run under the
  common controls and preserve complete raw evidence and scores.
- The rubric's transfer threshold is applied and every count and the verdict are
  reported regardless of outcome.

## Notes

The probe is reported separately and cannot cause skill wording changes without
a new evaluation cycle.
