# decision-records-skill-k2

**Kind:** work

## Goal
Author the new `linkuistics:decision-records` skill — the single source of truth
for the ADR philosophy (minimum coherent set; current-state not changelog; edit
in place / merge / split / delete; identity by slug not number; the 3-part
when-to-write test; the minimal template). Commit it in the `../skills` repo.

## Context
Mandate: **`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` — Part 1**
(the authoritative bullet list for the skill's content).
- Location: `../skills/plugins/linkuistics/skills/decision-records/SKILL.md`, a
  peer of `codebase-design` / `cli-tool-design`. **Model-invoked** (no
  `disable-model-invocation`). Follow `authoring-conventions` (description =
  capability + "Use when …"; cite sources; progressive disclosure).
- Provenance: surviving good material originates from `mattpocock/skills` (MIT) —
  preserve attribution consistent with grove's bundled copies.
- Light-touch update to the linkuistics plugin manifest keywords/description if
  the new skill warrants it.

## Done when
- The `decision-records` SKILL.md exists at the Part-1 path, covering every
  Part-1 bullet, with mattpocock/MIT attribution preserved.
- Plugin manifest updated if warranted.
- Committed in `../skills` as its **own** commit (separate repo — see the spec's
  "Cross-cutting: two repos, two commits").

## Notes
Independent of the other leaves and the natural **first** in the chain: leaves
`grove-adr-note-k3` and `grove-process-prose-k4` defer *to* this skill, so it
should exist and be stable before they point at it. This grove's other five
leaves all commit in `grove`; this one alone commits in `../skills`.
