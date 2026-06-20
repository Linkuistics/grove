# 050-dotted-decimal-numbering

**Kind:** planning

## Goal

Settle the exact **flat dotted-decimal** task-id scheme (direction set in 010 D4/D5)
and change the grow/walk verbs to implement it: a node `1.2.5` carries
`1.2.5.BRIEF.md`; its children are `1.2.5.1.<task>.md`, `1.2.5.2.<task>.md`, …; the
`pick` DFS falls out of a numeric per-segment **version-sort comparator**.

## Context

Read the **010 running log D4/D5** (the settled direction) first. Settled there:
legible sequential dotted integers (not fractional/LexoRank keys); ordering by a
numeric per-segment comparator (true infinite width + DFS order); **renumber-on-
reorder accepted**; **mark-done-in-place** (a done marker, no `done/` directory).
What remains is genuinely open mechanics — hence **planning**, not work.

## Done when

- The exact scheme is settled and recorded (ADR if it clears the bar): the
  comparator's edge semantics; the **done-marker** representation (filename suffix
  vs frontmatter — weigh `pick`/sort cost and walk-away legibility); how
  `leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` operate on the flat
  dotted namespace (esp. **insert's renumber** of a subtree, and cross-reference
  surfacing); how a node's `BRIEF.md` is named (`1.2.5.BRIEF.md`).
- The verbs are reimplemented (or extended) to the new scheme, with tests for the
  comparator and the renumber.
- Coordinates with **060** (backwards-compat/migration reads the *old* `NNN-slug/`
  format; this leaf defines the *new* target the migration writes).

## Notes

- Open sub-decisions to grill: done-marker form; whether `pick` skips done items
  by marker (replacing the `done/`-directory skip); insert-renumber mechanics and
  whether numeric cross-references are auto-rewritten or only surfaced (today
  `leaf-insert` surfaces, does not rewrite); the `BRIEF.md` naming convention in
  a flat namespace.
- The user noted prior art for the numbering mechanism (010 D4) — find and cite it.
- Keep the scheme legible on `ls` (constraint 6): the flat sorted list *is* the
  tree.
