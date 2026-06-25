# author-codebase-design-k15

**Kind:** work

## Goal

Author a new **`codebase-design`** skill: a language-neutral *deep-module design
vocabulary* (Ousterhout deep modules + Feathers seams). The survey's top content win.
(Synthesis skills disposition **AUTHOR #1**, source mattpocock-S1.)

## Context

- The gap: we ship language style guides (`coding-style-*`) + `cli-tool-design` but **no
  design-craft vocabulary skill**. This occupies the neutral-craft niche `cli-tool-design`
  already proves works here.
- The model to adapt (NOT fork verbatim): `mattpocock/skills` →
  `codebase-design/SKILL.md` (quoted in survey §mattpocock/skills S1). Keep its precise,
  *scale-agnostic* glossary (Module / Interface / Seam) and its **checkable** principles —
  the deletion test, "the interface is the test surface", "two adapters means a real seam"
  — and its rejection of Ousterhout's depth-as-line-ratio framing (it "rewards padding").
- Make it genuinely **language-neutral** (its source uses illustrative TS; the vocabulary
  is not TS-specific). Model-invoked (should auto-fire on design/architecture tasks).

## Done when

- `plugins/linkuistics/skills/codebase-design/SKILL.md` exists, spec-conformant, following
  the house authoring-conventions from k14, registered in `marketplace.json` + README.
- Self-contained (optionally one-level `references/` per the progressive-disclosure rule
  if it outgrows ~500 lines).

## Notes

- Invoke `brainstorming` + `writing-skills` (superpowers) when authoring — this is genuine
  creative work.
- **`domain-modeling` was deferred** (Synthesis AUTHOR #6, "decided not to author now" —
  carries `CONTEXT.md`/`docs/adr/` coupling). If this leaf finds the pairing essential,
  externalize it as a new leaf (`leaf-add`), don't absorb it.
