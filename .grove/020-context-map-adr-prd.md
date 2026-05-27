# 020-context-map-adr-prd

**Kind:** work

## Goal
Extend `docs/concepts.md` with the three remaining entries — Context Map, ADR, PRD — using the template established in 010, and add cross-links from `content/SKILL.md` and the relevant FORMAT files where these terms first appear in body text.

## Context
- Three different lineages, each with its own canonical anchor:
  - **Context Map** — Evans, Blue Book, Part IV (Strategic Design). grove materialises it as `CONTEXT-MAP.md` at the repo root when multiple bounded contexts exist.
  - **ADR** — Michael Nygard, "Documenting Architecture Decisions" (cognitect.com, 2011, often republished). MADR (`adr.github.io/madr/`) is a useful secondary reference for template style. grove uses `docs/adr/NNNN-*.md` and offers ADRs sparingly (per `grilling.md`: hard to reverse, surprising, or a real trade-off).
  - **PRD** — no single canonical source. Reasonable references: Cagan, *Inspired* (Wiley, 2017) for the framing of product-discovery artifacts; Atlassian's PRD template page for a concrete shape. grove uses `docs/prd/` at human-facing agreement points only.
- Cross-link targets (light touch — only where the term first appears in body text, not in every heading):
  - `content/SKILL.md` — introduces ADR (the "Artifacts" table and the "PRDs" section), PRD (same), Ubiquitous Language ("the glossary is load-bearing").
  - `content/BRIEF-FORMAT.md` — references ADRs in the "Pointers" example.
  - `content/CONTEXT-FORMAT.md` — references Bounded Context throughout.
  - `content/ADR-FORMAT.md` — defines the ADR file shape; a single backlink to `docs/concepts.md#adr` is enough.

## Done when
- Three new entries in `docs/concepts.md` (Context Map, ADR, PRD), same three-part shape and length budget as 010.
- Light cross-links added from `content/SKILL.md` and the FORMAT files at the first body-text mention of each term. Don't link every occurrence; once per file is plenty.
- Whole-doc read-through: tone consistent across all six entries; the references list resolves (no dead URLs, edition info present for books).

## Notes
- PRD is the entry with the weakest canonical anchor. That's fine — say so plainly in the references section ("no single canonical source; grove's usage is shaped more by the practice than by a definitive text"). Don't fake authority by citing something marginal.
- The MADR reference is genuinely useful for ADR-curious readers; include it. But don't conflate "MADR template" with "the ADR concept" — the concept comes from Nygard.
- If during writing this leaf you discover one of these terms wants its own grove (e.g. PRD's grove-specific usage needs a real spec), don't expand it inline — flag it in `.grove/` as a follow-up note for the user. This grove's job is six terse entries.
