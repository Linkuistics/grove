# skill-refinement-regression-k48


## Goal

Refine generic skill wording only where the complete enabled campaign
demonstrates a gap, then prove the affected behavior and prior successes still
hold.

## Context

- Frozen rubric and all baseline and enabled run directories under
  `docs/evaluations/writing-code-walkthroughs/`.
- Skill under test: `plugins/linkuistics/skills/writing-code-walkthroughs/`.

## Done when

- Every wording change maps to a scored enabled failure rather than a
  hypothetical or infrastructure-only gap.
- Each change is micro-tested and rerun against its affected frozen scenario;
  regression cases cover previously successful behaviors.
- The rubric, prompts, scoring rule, and contamination controls remain unchanged.
- Any unresolved acceptance gap is preserved explicitly rather than hidden by
  revised scoring or codebase-specific wording.

## Notes

Apply the house authoring conventions and test-driven skill-editing discipline.
