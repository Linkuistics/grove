# skill-delivered-methodology-k4

## Goal

Cut `docs/specs/skill-delivered-methodology.md` into the smallest independently
useful working increments, ordered by dependency, and grow the tree.

## Context

Read the spec first; it is the whole input. `plan-k1` and the root `BRIEF.md`
carry the decisions behind it. If `integrate-review-design` ran, its changes are
already in the spec — read the spec, not the findings.

The work is a **rewrite plus a deletion**, and the deletion is the larger half by
line count and the smaller half by risk.

## Done when

The tree carries the increments, each leaving the product working and delivering
something verifiable for its successor. The spec's own material to work from:

- **The one ordering constraint that is a design fact, not a preference:**
  `${prompt}` must not shrink before `SKILL.md` is short. Either half alone
  reproduces one of the two measured failures — a core against an unrewritten
  corpus hands back a ~50 KiB `SKILL.md` in one gulp; a rewritten corpus behind a
  full mandate keeps the wall. Whether that forces one increment or two with a
  flag is the planning call.
- **The rewrite of `content/`** into `SKILL.md` plus a flat `references/`. The 140
  unit markers are the scaffolding: `class=triggering` yields a condition line,
  the remainder joins the procedures, and the ten narrowed scopes are the per-kind
  file set. This is the largest piece and is plausibly several increments — the
  spec's arithmetic (~51 condition lines, ~200 lines total) is the bound to hold
  each one against.
- **Restoring the delivery path** on the `launch.rs` / `loop_driver.rs` seam it
  never left, plus the new `prompt` module seam replacing `methodology`'s parse
  and compose.
- **The deletions**: composer, marker grammar, fence-state parser, build gate,
  completeness invariant, file-ordering directive, `grove-llm methodology`. Two
  checks in that neighbourhood must **survive** — the instructed-verb scan and the
  flat-verb-surface pin — and the spec says why.
- **The record rework**: `mandate-delivers-the-methodology` → renamed and reworked
  as `skill-delivers-the-methodology`; `one-build-owns-a-session` targeted rework;
  `CONTEXT-MAP.md`'s shared-target clause; `docs/ARCHITECTURE.md`'s embedded
  methodology section; and the deletion of
  `docs/specs/mandate-delivered-methodology.md` by the increment that removes the
  last machinery it describes. `CONTEXT.md`'s remaining mandate-era entries
  (Methodology unit, Mandate slice, Triggering unit / procedural unit, Build
  pairing) are reworked by the increments that remove what they describe — that
  was `plan-k1`'s call and it stands.
- **Citation reconciliation** is real work with a known surface: `grep -rn
  'mandate-delivers-the-methodology\|mandate-delivered-methodology'` finds it. The
  prose sites and the code doc-comments do **not** move at the same time — the
  code ones cite live mechanism and move with it.
- **The verification run** is part of the grove's `Done when`, not an afterthought:
  a real Grove run with a human watching, showing sessions both **ending** and
  **reading the skill**. One without the other is a swap. Whether that is a leaf
  or a gate on the last increment is the planning call.

## Notes

`grove-llm methodology <id>` still works while the machinery is live and is the
fastest way to read a procedural body while cutting the rewrite increments.

Nothing here re-opens *what* is being built. If the spec looks wrong, that is a
finding for a review chain, not a planning decision.
