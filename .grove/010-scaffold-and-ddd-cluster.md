# 010-scaffold-and-ddd-cluster

**Kind:** work

## Goal
Create `docs/concepts.md` with its overall structure and a short preface, then write the first three entries — Domain-Driven Design, Ubiquitous Language, Bounded Context — and link from `docs/grove.md`'s existing inline glosses.

## Context
- Existing inline glosses to cross-link in `docs/grove.md` (kept in place, just linked):
  - "DDD's term for the project's shared domain vocabulary" — paragraph that introduces `CONTEXT.md`.
  - "DDD's term for distinct domain partitions" — paragraph that introduces `CONTEXT-MAP.md`.
- The doc's per-entry shape (lock this in 010; 020 just applies it):
  - **Definition** (2-5 lines) — terse, the meaning. Not a tutorial.
  - **How grove uses it** (3-6 lines) — concrete tie-in to grove's artifacts/flow.
  - **Canonical references** (1-3 lines) — book (with edition, no page numbers) and canonical web doc.
- Canonical sources for this trio:
  - Evans, *Domain-Driven Design: Tackling Complexity in the Heart of Software* (Addison-Wesley, 2003) — the Blue Book; the source.
  - Vernon, *Implementing Domain-Driven Design* (Addison-Wesley, 2013) — the Red Book; the pragmatic companion.
- The doc's preface should make these exclusions explicit:
  - DDD building blocks (Aggregate, Entity, Value Object, Repository) — not load-bearing for grove.
  - grove-local terms (task, leaf, node, planning task, work task, brief, grilling, retirement) — those are defined in `content/SKILL.md` and its sibling files.
  - Codebase-specific domain terms (install scope, path-scoped commit, lifecycle walkthrough) — those live in `CONTEXT.md`.

## Done when
- `docs/concepts.md` exists with a short preface (what the doc is for, what's deliberately excluded) and entries for DDD, Ubiquitous Language, Bounded Context.
- Each entry uses the three-part shape, within the length budget.
- `docs/grove.md`'s inline glosses for UL and Bounded Context link to the corresponding `docs/concepts.md` anchors. The inline gloss stays — it's the first hit; the link backs it.
- A reader can land on `docs/concepts.md`, read the preface, read the three entries, and walk away knowing what each term means and how grove operationalises it — without needing the canonical books on hand.

## Notes
- Do not over-explain DDD as a whole. The DDD entry's job is to frame the other two, not to summarise the Blue Book.
- The "how grove uses it" section is what makes this doc grove's, not a generic glossary. Be concrete: name the file, the format, the moment in the session loop.
- If the preface or any entry starts to grow into a paragraph beyond the budget, that's a signal to cut, not to expand the budget.
