# grove-process-prose-k4

**Kind:** work

## Goal
Teach grove's methodology prose the **revisit-and-rework** ADR behavior and
declare the linkuistics prerequisite: edits to `content/SKILL.md`,
`content/driving.md`, and `README.md`.

## Context
Mandate: **`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` — Part 2**
(the `SKILL.md`, `driving.md`, `README.md` bullets).
- `content/SKILL.md`: (a) add revisit-and-rework to the **Plan** and **Retire**
  steps — as understanding changes, rework the ADR set *in place*
  (merge/split/delete) to keep it minimal and coherent, and reconcile the BRIEFs;
  never append a superseding ADR. (b) Update the Artifacts-table row for ADRs
  (`docs/adr/<slug>.md`; current-state, minimum coherent set). (c) Update the
  Reference-files list entry for `ADR-FORMAT.md`. (d) Note the linkuistics-plugin
  prerequisite.
- `content/driving.md`: add a short field-guide subsection — reworking ADRs and
  BRIEFs as understanding shifts (edit in place, keep the set minimal, fix
  dangling citations); align the existing "when research retires into ADRs"
  material with edit-in-place.
- `README.md`: declare the linkuistics-plugin prerequisite (grove now depends on
  `linkuistics:decision-records`).

## Done when
- `SKILL.md` Plan+Retire carry the revisit-and-rework behavior; Artifacts row and
  reference-file entry updated; prerequisite noted.
- `driving.md` has the rework subsection; research-retire material aligned.
- `README.md` declares the prerequisite.
- Committed in `grove`.

## Notes
- Depends on `decision-records-skill-k2` existing (prose points at it).
- **Defer** the `ADR-NNNN`→slug self-citation conversion in `SKILL.md` and
  `driving.md` to `citation-reconcile-k7` (slugs not final until `corpus-rework-k6`).
  Do the behavioral/prose edits here; leave the citation sweep to leaf k7.
