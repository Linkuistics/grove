# define-and-explain-process-terms — root brief

## Goal
Anchor the imported methodology vocabulary grove leans on — DDD, Ubiquitous Language, Bounded Context, Context Map, ADR, PRD — in a single canonical doc at `docs/concepts.md`, so `content/SKILL.md`, `docs/grove.md`, and the FORMAT files can reference it instead of redefining or hand-waving these terms each time.

## Done when
- `docs/concepts.md` exists, with all six terms entered: DDD, Ubiquitous Language, Bounded Context, Context Map, ADR, PRD.
- Each entry has three parts: terse **definition**, **how grove uses it** (concrete tie-in to grove's files/flow), **canonical references** (book + canonical web doc).
- `docs/grove.md`'s existing inline glosses for UL and Bounded Context link to `docs/concepts.md` (gloss kept; link added).
- `content/SKILL.md` and the relevant FORMAT files link to `docs/concepts.md` where they first introduce these terms in body text — lightly, only where a reader would benefit.
- No entries added to `CONTEXT.md` (excluded by `CONTEXT-FORMAT.md`'s "no general programming concepts" rule).

## Decomposition
Split by **source-lineage cluster**, not by term count:
- `010-scaffold-and-ddd-cluster.md` — create the doc skeleton and write the DDD trio (DDD, UL, Bounded Context). Shared source (Evans, Vernon), shared vocabulary, written together so tone and cross-references settle.
- `020-context-map-adr-prd.md` — add Context Map, ADR, PRD. Different lineages each, written against the template established in 010.

## Pointers
- Files this grove touches: `docs/concepts.md` (new), `docs/grove.md` (cross-links), `content/SKILL.md` and `content/BRIEF-FORMAT.md` / `content/CONTEXT-FORMAT.md` / `content/ADR-FORMAT.md` (light cross-links where terms first appear in body text).
- Glossary terms in play (`CONTEXT.md`): **grove** is overloaded — the *methodology* sense is what's in play here, not the CLI or workstream senses.
- ADRs cited: none yet — no decision in this grove is hard-to-reverse, surprising, or a real trade-off.

## Notes
- These terms are deliberately **not** added to `CONTEXT.md`. `CONTEXT-FORMAT.md` excludes "general programming concepts even if the project uses them extensively"; DDD, UL, BC, Context Map, ADR, PRD are exactly that exclusion case — imported methodology vocabulary, not grove-specific domain terms. Anchoring them in `docs/concepts.md` resolves the start-prompt template's instinct to put them in `CONTEXT.md` without violating the format rule.
- "How grove uses it" is the load-bearing section of each entry — it's where the imported concept meets grove's concrete artifacts (`CONTEXT.md` *is* the UL; `CONTEXT-MAP.md` *is* the Context Map; `docs/adr/` is where ADRs live; `docs/prd/` is where PRDs live).
- Canonical references default: cite the canonical **book with edition** (not page numbers — pages drift between printings) plus the canonical web doc where one exists. Keep it terse: 1-3 lines of references per term.
- Term list is **closed at six**. Resist drift into DDD building blocks (Aggregate, Entity, Value Object) or Anti-corruption Layer — those aren't load-bearing for grove. If a future session feels the pull to add them, it's a new planning task, not a scope creep on this one.
- Length budget per entry: 2-5 lines for definition, 3-6 lines for "how grove uses it", 1-3 lines for references. If a paragraph is forming, cut.
