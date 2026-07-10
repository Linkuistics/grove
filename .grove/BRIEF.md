# update-with-new-pocock-skills — brief

## Goal

Incorporate the `mattpocock/skills` v1.1 update (189 commits past grove's
provenance pin `b8be62f`, upstream HEAD `d574778`) across **both** repos of the
one system: **grove** (this repo) and **`Linkuistics/skills`**
(`~/Development/skills`). Analysis and per-item verdicts live in
`docs/research/mattpocock-skills-v1.1-incorporation.md` (G1–G5 grove-side,
L1–L5 Linkuistics-side); the user chose to do **all** recommendations,
including the MAYBEs.

Beyond the incorporation items, the update prompted three grove-methodology
questions the user wants driven as their own planning leaves: PRD → spec
reframing; a brainstorm on moving the `.grove/` structure onto GitHub issues
(wayfinder's substrate); and enriching task kinds beyond planning/work, wired
to per-kind model selection.

## Done when

- grove's `content/grilling.md` carries the upstream self-grilling fix, a
  grove-worded confirmation gate, and a refreshed provenance header.
- The decomposition-craft enrichments (vertical slice, expand→contract,
  horizon note, durable briefs, no-fog exit, seam-sketching, glossary rule)
  have landed in grove's `driving.md` / `BRIEF-FORMAT.md` / `CONTEXT-FORMAT.md`.
- `Linkuistics/skills` `authoring-conventions` carries Negation,
  context/cognitive-load + router, and the sentence-level no-op hunt; survey
  citations refreshed; `codebase-design` carries the design-it-twice subagent
  workflow.
- The three planning questions (PRD→spec, issues substrate, task kinds) are
  each grilled to a decision, with follow-on work leaves grown as needed.

## Decomposition

Sequenced so the sure, cheap fixes land first and the substrate brainstorm
(the biggest open fork) precedes the two leaves whose implementation surface
it could reshape (task kinds; decomposition-craft prose). Confirmed by the
user 2026-07-09:

1. `grilling-fixes-k2` [work] — G1 defect + G2 gate + G3 provenance.
2. `skills-authoring-enrichments-k3` [work] — L1–L5, commits in
   `~/Development/skills`.
3. `prd-to-spec-k4` [planning] — PRD→spec reframe; seam-sketching folded in.
4. `issues-substrate-brainstorm-k5` [planning] — `.grove/` onto GitHub issues,
   on merit.
5. `task-kinds-model-selection-k6` [planning] — kinds beyond planning/work +
   per-kind models; after k5.
6. `task-kinds-impl-k9` [work] — the CLI + docs for k6's five-kind taxonomy;
   inserted *ahead* of k7 so the prose lands on a settled taxonomy.
7. `decomposition-craft-k7` [work] — G4+G5 prose enrichments; after k5.
8. `concepts-adr-refresh-k8` [work] — `docs/concepts.md` ADR section; surfaced
   during k4.
9. `review-provider-research-k10` [research] — can a `review` leaf run on
   another model family (GLM via `ANTHROPIC_BASE_URL`, or codex-as-harness)?
10. `review-provider-design-k11` [planning] — grill k10 to a route, or to a
    recorded rejection.

## Pointers

- `docs/research/mattpocock-skills-v1.1-incorporation.md` — the incorporation
  analysis; every leaf below cites its items from there.
- Upstream clone (session-scratch, re-clone if gone):
  `mattpocock/skills` @ `d574778`.
- Prior survey: `~/Development/skills/docs/research/skill-repo-prior-art.md`
  and `grove-recommendations.md` (2026-06-25) — this workstream is the delta
  on top of those.

## Notes

- **One system:** cross-repo leaves commit directly in
  `~/Development/skills` (user decision 2026-07-09; memory
  `grove-skills-one-system`).
- Deliberate non-actions (do NOT revisit without new evidence): no
  CONTEXT-FORMAT re-sync (upstream delta purely subtractive); no ADR-FORMAT
  change (byte-identical move; numbering rejected per
  `linkuistics:decision-records`); no vendoring of wayfinder / research /
  implement / prototype skills — note this is unchanged by `k6` adopting
  `research` and `prototype` as **task kinds**: the disciplines earned a place,
  the skills did not; **no move of `.grove/` onto GitHub issues or
  any tracker** — grilled to a `stay` in `issues-substrate-brainstorm-k5`, so
  `task-kinds-model-selection-k6` and `decomposition-craft-k7` proceed against
  the directory-tree substrate exactly as briefed.
