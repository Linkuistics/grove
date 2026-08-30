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

## Decisions (running log)

- Only the completed Case C sample supplies refinement evidence. Its scored
  `C23` failures identify a generic wrong-shaped-output loophole: actor naming
  was scoped to “important” sentences rather than every effect. Incomplete
  Case A and B samples and the transfer-probe shortfall cause no wording change.
- The minimal positive contract is “Name the actor for every effect.” It
  replaces the weaker phrase “explicit actors” without changing frontmatter,
  the skill's routing, or any domain-specific rule.
- The unchanged five-control/five-enabled Case C schedule is both the wording
  micro-test and the affected-scenario rerun. Its other atomic rows cover the
  previously successful regression behaviors without adding a new prompt or
  scoring rule.
- The refined arm moves `C23` from `2/5` to `4/5` while the contemporaneous
  control is `5/5`. Preserve the remaining “important sentence” miss and the
  indeterminate skill-body-use limitation; do not strengthen unrelated
  frontmatter or claim comparative material improvement.
