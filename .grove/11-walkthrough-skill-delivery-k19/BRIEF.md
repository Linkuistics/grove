# walkthrough-skill-delivery-k19 — brief

## Goal

Turn the validated walkthrough method into a generic deployable Linkuistics
skill whose behavior is demonstrated against a no-skill baseline.

## Context

The skill belongs at
`plugins/linkuistics/skills/writing-code-walkthroughs`. It must apply across
codebases and languages; `ordinal-fs-tree` supplies evidence and a worked
development history, not skill-specific vocabulary.

## Done when

- Before the skill exists, committed behavioral scenarios and a predeclared
  rubric expose the relevant no-skill failures.
- The skill elicits target, included source, audience, depth, output form, style,
  and verification requirements one question at a time before authoring.
- The body captures the proven method for concept ordering, source inventory,
  fragment ownership/tangling, self-contained prose, scoped and final checks,
  and independent technical/editorial review without copying this book's domain.
- Skill frontmatter, progressive disclosure, citations, harness declaration,
  plugin layout, and description follow the Linkuistics authoring conventions.
- The same scenarios run in fresh contexts with the skill enabled, meet the
  rubric, and show a material improvement over baseline.
- Plugin installation/structure checks and any skill-specific deterministic
  tests pass.

## Decomposition

- `skill-baseline-k20` defines and runs the no-skill control before skill bytes
  can influence the result.
- `writing-code-walkthroughs-k21` authors the smallest generic skill that
  addresses observed failures and preserves successful default behavior.
- `skill-evaluation-k22` reruns the same rubric with the skill enabled, closes
  demonstrated wording gaps, and proves deployment.

## Pointers

- Method evidence: `walkthrough-method-k5`.
- Proven artifact and workflow: `ordinal-fs-tree-book-k10` and its completed
  review chain.
- House rules: `plugins/linkuistics/skills/authoring-conventions/SKILL.md` and
  the `writing-skills` skill, including its referenced best-practices file.
- Plugin provenance: `plugins/linkuistics/PROVENANCE.md`.

## Notes

Apply test-driven skill authoring. Predeclare each scenario's prompt, observable
behaviors, scoring rule, sample size, and contamination controls. For wording
intended to shape behavior, run at least five no-skill repetitions; if the
control does not exhibit the target failure, do not add guidance for it. Run the
enabled evaluation against the same cases and distinguish deterministic checks
from judgment scored over agent outputs.

`skill-baseline-k20` and `skill-evaluation-k22` are expected
`leaf-decompose` candidates split by scenario group when the campaign exceeds a
focused session. The first baseline child commits the rubric once; all remaining
baseline and enabled-evaluation children share it unchanged.
