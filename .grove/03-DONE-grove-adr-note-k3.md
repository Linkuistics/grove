# grove-adr-note-k3

**Kind:** work

## Goal
Reduce `content/ADR-FORMAT.md` to a **thin grove note** that keeps only
grove-specific placement conventions and defers philosophy/format/template to the
`linkuistics:decision-records` skill. Update `content/grilling.md` to match.

## Context
Mandate: **`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` — Part 2**
(the `ADR-FORMAT.md` and `grilling.md` bullets).
- `content/ADR-FORMAT.md`: **keep** `docs/adr/<slug>.md` naming (slug-only, no
  number), per-context placement under `CONTEXT-MAP.md`, lazy creation, and the
  brief-chain curation rationale ("a session reads three ADRs, not fifty").
  **Remove** the template, the 3-part test, the numbering/superseded machinery.
  **Add** a pointer to `linkuistics:decision-records` for philosophy/format/test.
- `content/grilling.md` "Offer ADRs sparingly": collapse the duplicated 3-part
  test to a one-line reference to the skill; update the file-tree examples
  (`0001-event-sourced-orders.md`, `0002-postgres-for-write-model.md`) to
  slug-only names.

## Done when
- `ADR-FORMAT.md` is the thin note per the Part-2 bullets, pointing to the skill.
- `grilling.md`'s duplicated test is a one-line skill reference; its file-tree
  examples are slug-only.
- Committed in `grove`.

## Notes
- Depends on `decision-records-skill-k2` existing (this file points at it).
- **Defer** any numbered-citation→slug conversion to `citation-reconcile-k7`:
  surviving ADR slugs are not final until `corpus-rework-k6`. Make the prose
  changes here; leave the final citation sweep to leaf k7.
