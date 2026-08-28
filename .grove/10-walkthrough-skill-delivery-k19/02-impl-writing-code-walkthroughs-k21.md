# writing-code-walkthroughs-k21

## Goal

Author the generic Linkuistics `writing-code-walkthroughs` skill from the
validated book method and observed baseline failures.

## Context

- Inputs: `skill-baseline-k20`, `walkthrough-method-k5`, the completed book and
  review findings, and this subtree's brief.
- Before editing a skill, load and follow `writing-skills` and the user-invoked
  Linkuistics `authoring-conventions` skill.

## Done when

- `plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md` has a valid
  capability-plus-`Use when` description, `harnesses` declaration, and generic
  trigger vocabulary without workflow leakage in frontmatter.
- The workflow elicits target, source scope, audience, depth, output form,
  style, and verification requirements one question at a time before authoring.
- Instructions cover evidence gathering, concept and dependency ordering,
  exact fragment ownership/tangling, scoped progress, final coverage,
  self-contained prose, local repetition versus cross-references, deterministic
  checks, and independent technical/editorial review.
- Only baseline-demonstrated behavior gaps become behavior-shaping guidance;
  mechanical constraints remain scripts/tests where that is the deeper seam.
- Supporting references, scripts, or templates exist only when progressive
  disclosure or reuse earns them, stay one level deep, and carry required source
  citations.
- Skill structure, plugin installation, and all deterministic tests pass.

## Notes

This leaf does not claim behavioral success from prose inspection. That claim
belongs to `skill-evaluation-k22` using fresh enabled contexts.
