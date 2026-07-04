# corpus-rework-k6

**Kind:** work

## Goal
Execute the **approved** disposition from `corpus-disposition-k5`: perform the
keep / delete / merge, and rename every survivor to a slug-only filename. The
result is grove's `docs/adr/` reduced to a minimum coherent set describing
grove's *current* design.

## Context
Mandate: **`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` — Part 3**
(Method steps 2 and 4).
- Read the approved disposition table produced by `corpus-disposition-k5`
  (`grove-llm resolve corpus-disposition-k5` to find it, then its persisted
  table).
- **Keep** → `git mv NNNN-slug.md slug.md` (slug-only, no number) and edit the
  content to be self-contained and current-state (drop `superseded by`, decision
  history, and any narrative of how the team arrived).
- **Delete** → `git rm` (git holds the history).
- **Merge** → fold the live lesson into its surviving ADR, then `git rm` the source.
- Slug uniqueness within `docs/adr/` (slugs are now the handle).

## Done when
- The corpus matches the approved disposition: survivors only, each slug-only and
  self-contained/current-state; deletes and merges executed.
- No `Status: superseded` framing remains in any surviving ADR.
- Committed in `grove`.

## Notes
- **Depends on `corpus-disposition-k5`'s approved table** — do not start without
  it. If the table is not yet approved, that leaf is not done; stop.
- Citation reconciliation across the repo is **not** done here — it is
  `citation-reconcile-k7`, which runs *after* this leaf so the survivor slugs are
  final.
