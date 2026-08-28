# skill-baseline-k20

## Goal

Design and run the no-skill behavioral baseline for generic code-walkthrough
authoring before `writing-code-walkthroughs` exists.

## Context

- Inputs: the completed and reviewed book, `walkthrough-method-k5`, and this
  subtree's brief.
- The baseline must not load draft skill instructions or book-specific task
  files into the evaluated context.

## Done when

- Scenario prompts and a scoring rubric are committed before any run, covering
  one-question-at-a-time scope elicitation, exact source inventory, conceptual
  ordering, fragment completeness, self-contained prose, repetition versus
  links, validation, and independent review planning.
- Each behavior-shaping case is run at least five times in fresh no-skill
  contexts with contamination controls recorded.
- Raw outcomes, scores, recurring omissions, and rationalizations are preserved
  in a compact durable artifact.
- The report distinguishes failures the new skill should address from behavior
  already reliable without guidance and from constraints better enforced
  mechanically.
- Every proposed skill rule is justified by an observed baseline gap; cases
  whose controls do not fail are explicitly excluded from the authoring brief.

## Notes

Do not write or scaffold the new skill in this leaf. The temporal separation is
what makes the baseline credible.
