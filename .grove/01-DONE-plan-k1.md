# plan-k1

**Kind:** planning

## Goal

Analyse the significant upstream update to `mattpocock/skills` and report what
is worth incorporating into (a) the **grove** repo (this repo — bundles
`grilling.md`, `CONTEXT-FORMAT.md`, and an `ADR-FORMAT.md` placement note from
mattpocock) and (b) the **`~/Development/skills`** repo (`Linkuistics/skills` —
whose `decision-records`, `authoring-conventions`, and research docs derive from
mattpocock material). Deliver a research report; then grill on incorporation
decisions and grow the tree.

## Context

- grove's bundled provenance pin: `mattpocock/skills@b8be62f` (headers in
  `content/grilling.md`, `content/CONTEXT-FORMAT.md`). **189 commits** upstream
  since the pin; repo restructured.
- Upstream HEAD: `d574778` (v1.1). Clone for analysis in scratchpad:
  `.../scratchpad/mattpocock-skills`.
- Structural change: `grill-with-docs/SKILL.md` gutted (−87 lines); its
  `ADR-FORMAT.md` and `CONTEXT-FORMAT.md` **moved** into a new
  `skills/engineering/domain-modeling/` skill. New planning family:
  `wayfinder`, `to-spec`, `to-tickets`, `research`, `implement`, `triage`,
  `prototype`, `domain-modeling`, `codebase-design`. New productivity
  `grilling` skill. `writing-great-skills` gained Negation/Negative-Space
  failure modes + a GLOSSARY.

## Done when

- A research report exists (`docs/research/…`) mapping the upstream changes to
  grove and Linkuistics, cluster by cluster, with primary citations
  (commit/file/quote) and a per-item incorporate/skip recommendation.
- The tree has grown the leaves the report implies (grove-side vs skills-side
  incorporation work), or a decision is recorded to defer.

## Decisions (running log)

**Prior art exists — build on it, don't repeat it.** `Linkuistics/skills` ran a
full prior-art survey (`docs/research/skill-repo-prior-art.md`,
`grove-recommendations.md`, dated 2026-06-25) that already deep-dived
mattpocock/skills. It led Linkuistics to author `codebase-design` +
`decision-records`, and flagged grove's grilling.md bundle drift
(grove-recommendations §8). This report's job is the **delta since that survey /
since grove's pin b8be62f**, not a from-scratch analysis.

**Confirmed structural findings (own git analysis of the clone):**
- grove's pin `b8be62f` predates the whole `decision-mapping → wayfinding →
  wayfinder` lineage. `decision-mapping` (the prior survey's *headline grove
  analog*) was renamed to `wayfinding`, then `/wayfinder`, then **graduated to
  engineering in v1.1** (`639df6e`), reframed around destination/frontier + deep
  issue-tracker integration (native blocking, claim-by-assignment). The prior
  survey's key grove finding is now stale.
- The three format files grove/Linkuistics derive from — `grill-with-docs`
  (grove's grilling.md source, −87 lines, gutted to a pointer),
  `CONTEXT-FORMAT.md`, `ADR-FORMAT.md` — all **moved into a new
  `domain-modeling` skill**. Need: did content change or was it a pure move?
  (agent B).

**Research complete** — report at
`docs/research/mattpocock-skills-v1.1-incorporation.md` (grove-side G1–G5,
Linkuistics-side L1–L5, plus explicit non-actions).

**One system, both repos (user, 2026-07-09).** grove and `Linkuistics/skills`
are tightly related — the user treats them as one system and is happy to do
work on both repos from either. The prior survey's "recommendation-only, never
implemented from this worktree" boundary is **rescinded**: this grove
implements ALL recommendations, grove-side and Linkuistics-side.

**Do all recommendations** — G1–G5 and L1–L5, including the MAYBEs.

**Three new planning concerns raised by the user (each gets its own leaf):**
1. **PRD → spec.** Adopt upstream's rationale: what grove produces is *not*
   really a PRD — reframe/rename PRD as spec. (Collision to grill: grove
   already has `docs/specs/*-design.md` for design specs.)
2. **Brainstorm: `.grove/` structure into GitHub issues.** Wayfinder proves the
   fog-of-war shape works on a tracker substrate. Genuine open brainstorm —
   engage the tension with constraints 1 & 6 (artifacts-not-state,
   walk-away-able) on merit, no sunk-cost defence of the current substrate.
3. **Task kinds beyond planning/work, wired to model selection.** Upstream's
   four-way taxonomy (research / prototype / grilling / task) suggests
   enriching grove's binary kind — useful for per-kind model selection
   (`GROVE_PLANNING_MODEL`/`GROVE_WORK_MODEL`, ADR model-per-task-kind).

**Tree confirmed and grown (user, 2026-07-09):** six leaves k2–k7 as listed in
the root brief's Decomposition — sure fixes first (k2, k3), then the three
planning leaves with the substrate brainstorm (k5) ahead of the two leaves its
outcome reshapes (k6, k7). Seam-sketching folded into k4; k3 is a single
cross-repo leaf.


## Notes

Research dispatched as 4 parallel cluster deep-dives (grilling; CONTEXT+ADR
format / domain-modeling; new planning family; writing-great-skills +
skill-authoring). Synthesis + grove/Linkuistics incorporation judgment done in
this session.
