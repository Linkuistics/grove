# grove-handoff-k18

**Kind:** work

## Goal

Extract the survey's grove-project findings into a **standalone, self-contained,
grove-repo-ready** recommendation doc — droppable straight into `Linkuistics/grove`.
(Closes the root brief's third done-when: "Grove-project findings are written up as
recommendations to carry to the grove repo.")

## Context

- Source material: the `## Synthesis` → "Grove project — ranked recommendations" section of
  `docs/research/skill-repo-prior-art.md` (items **G-1…G-9**) plus the §1b grove notes
  (moai-adk, plannotator, aider, pchalasani). The per-dive "Findings — grove project"
  sections hold the primary-source quotes if more detail is needed.
- Make the doc **self-contained**: a grove maintainer reading it in the grove repo should
  not need this survey. Each recommendation: source citation(s) + walk-away + a concrete
  "what to change in grove" line. Rank actionable-first (doubt-pass spec, unattended mode,
  confabulation guard, wire-to-verification-before-completion, model-by-leaf-kind), then
  declined-with-cost (dependency edges), then the small `driving.md` notes, then the
  `grilling.md` drift annotation, then the spine-validation citation bench.
- Suggested output path: `docs/research/grove-recommendations.md` (decide the name).

## Done when

- A standalone grove-recommendations doc exists, ranked, each item cited + walk-away +
  concrete change.

## Notes

- ⚠ **Never edit the grove repo or the grove skill from this worktree** (memory:
  *grove lives in its own repo*; CONTEXT.md: grove findings are *recommendations only*).
  This leaf produces a **handoff artifact here**; implementation happens later in
  `~/Development/grove`.
- The `grilling.md` drift (G-8) is a concrete grove-the-skill edit to *describe* in the
  doc, not perform here.
