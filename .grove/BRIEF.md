# grove.doubt-vs-grove-review-mechanics — brief

## Goal

Make doubt-driven development and Grove's review mechanics compose without
duplicate or runaway in-session reviews. Small unexpected doubt may stay inside
one picked leaf; substantial adversarial review must become Grove-managed tree
work so a fresh session and Grove's routing policy choose the reviewer.

## Done when

- The canonical Grove and doubt-driven guidance encode one consistent boundary
  across all producer kinds, review and integration leaves, research pairs, and
  sessions outside Grove.
- A picked plain producer can be promoted atomically into a structurally grouped
  review chain while preserving its stable handle; the producer then finishes
  only to a reviewable boundary and the fresh review leaf runs next.
- A picked leaf spends at most one in-session reviewer. A second review need, or
  a substantive actionable finding that needs re-review after a non-mechanical
  fix, triggers promotion to Grove-managed review.
- Grove owns review routing. A review still launches but warns unless both its
  effective harness and model differ from the producer's.
- Tests cover atomicity, ordering, key/handle preservation, routing warnings,
  task-kind behavior, and the unchanged standalone doubt-driven behavior.
- The glossary, methodology, architecture, CLI help, and marketplace skill tell
  the same current-state story.

## Decomposition

- `doubt-grove-design-k3` → `doubt-grove-design-review-k4` →
  `doubt-grove-design-integrate-k5`: specify and adversarially verify the
  cross-skill contract, promotion semantics, routing warning, and test seams.
- `doubt-grove-implementation-k7` → `doubt-grove-implementation-review-k8` →
  `doubt-grove-implementation-integrate-k9`: implement the agreed design,
  adversarially review it, and apply verified findings.

The design chain runs first because the atomic promotion and stateless routing
semantics determine whether implementation remains one focused vertical slice.
If `doubt-grove-implementation-k7` proves too large, it must decompose itself
rather than absorb multiple sessions of work.

## Pointers

- Glossary: `CONTEXT.md`, especially **Review chain / vendor pair**, **Task
  kind**, **Kind routing**, and **Pick**.
- Grove methodology: `content/SKILL.md`, `content/driving.md`, and
  `content/TASK-FORMAT.md`.
- Doubt guidance: `plugins/linkuistics/skills/doubt-driven-development/SKILL.md`.
- Architecture: `docs/ARCHITECTURE.md#task-kind-taxonomy` and the loop/routing
  sections adjacent to it.
- Existing CLI seams: tree growth in `src/tree_grow.rs`, LLM command dispatch in
  `src/llm_cli.rs`, and effective launch routing in `src/loop_driver.rs`.

## Notes

- The special composition rule applies only while executing the leaf returned
  by Grove's pick step, not merely because a checkout contains `.grove/`.
- If a producer is already in a review chain, its scheduled `review-*` and
  `integrate-review-*` leaves satisfy doubt-driven development's fresh-context
  review requirement. The producer spawns no duplicate doubt reviewer.
- A `review-*` leaf never invokes doubt-driven development: it is already the
  fresh-context adversarial read. An integration leaf may spend one narrow
  in-session reviewer; substantial redesign becomes follow-up Grove work.
- The one-review allowance is leaf-wide, not per decision or artifact.
- After the allowed reviewer finds a substantive actionable issue, incorporating
  the fix normally creates a second review need and therefore promotion. Trivial
  findings, noise, accepted visible trade-offs, or fixes conclusively covered by
  an executable test seam do not force it.
- Research-pair leaves use their two-corpus and adversarial-combine disciplines,
  never in-session doubt reviewers. A load-bearing decision derived from the
  research belongs in its own reviewed producer chain.
- Outside a picked Grove leaf, doubt-driven development keeps its existing
  behavior, including optional cross-model review.
- Different harness and model are preferred, not gated: warn if either matches
  the producer, then continue.
