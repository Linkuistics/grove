# skill-evaluation-k22

## Goal

Run the predeclared behavioral scenarios with the new skill enabled, compare
them to baseline, and refine only demonstrated gaps until the deployment claim
is supported.

## Context

- Inputs: `skill-baseline-k20`, `writing-code-walkthroughs-k21`, and this
  subtree's brief.
- Use the same prompts, rubric, sample sizes, and contamination controls as the
  baseline; any necessary rubric change invalidates the comparison and requires
  rerunning both sides.

## Done when

- Every scenario runs in fresh skill-enabled contexts and its raw outputs and
  scores are preserved beside the baseline.
- The comparison reports per-behavior and aggregate changes, variance, remaining
  failures, and whether each claimed improvement is material.
- Wording is tightened only in response to observed enabled failures, and every
  refinement is rerun against the affected scenario without regressing others.
- The final skill meets the predeclared acceptance rubric or the unresolved gap
  is surfaced rather than hidden by changing the scoring rule.
- Frontmatter/structure checks, plugin installation tests, local links, and any
  reusable validator/template tests pass after the final edit.
- The evaluation artifact states exactly what behavior is proven, what remains
  judgment-dependent, and which mechanical checks provide stronger guarantees.

## Notes

Keep the skill generic throughout refinement. An `ordinal-fs-tree`-specific fix
is evidence that the scenario or instruction is shaped at the wrong level.
