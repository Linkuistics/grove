# author-doubt-driven-development-k16

**Kind:** work

## Goal

Author a new **`doubt-driven-development`** skill: an *in-flight, per-decision adversarial
verify* discipline. (Synthesis skills disposition **AUTHOR #2**, source addyosmani-S1.)

## Context

- The gap: neither our marketplace nor superpowers has an *in-flight* doubt skill —
  superpowers ships only *post-hoc* `requesting`/`receiving-code-review`. This cross-
  examines non-trivial decisions *while course-correction is still cheap*.
- The protocol (model to adapt, NOT fork): `addyosmani/agent-skills` →
  `doubt-driven-development/SKILL.md` (quoted in survey §addyosmani S1/G1).
  CLAIM→EXTRACT→DOUBT→RECONCILE→STOP, with the load-bearing rules: **bias control** (pass
  ARTIFACT + CONTRACT only, never the CLAIM — handing the reviewer your conclusion biases
  it toward agreement); reviewer prompt **must be adversarial** ("find what is wrong");
  reviewer-output-is-data with a precedence classifier; a **bounded loop** (stop at trivial
  findings / 3 cycles / user override; if 3 is "obviously insufficient", *decompose the
  artifact*, don't lift the bound); and the **doubt-theater** self-check.
- It is a **main-session orchestrator** skill (it spawns a subagent) — note that
  dependency in the skill body. Model-invoked.

## Done when

- `plugins/linkuistics/skills/doubt-driven-development/SKILL.md` exists, spec-conformant,
  following k14's house conventions, registered in `marketplace.json` + README.

## Notes

- **Dual-target with k18:** this *same source* (addyosmani) is grove's headline doubt-pass
  recommendation (G-1). Here it is a marketplace SKILL.md; in k18 it is written up as a
  grove-repo recommendation. Keep them consistent but they are separate artifacts for
  separate targets — do not conflate.
- Invoke `brainstorming` + `writing-skills` when authoring. Optionally note wshobson-G2's
  diverse-lens composition (N reviewers each on a named failure axis) as an advanced mode.
