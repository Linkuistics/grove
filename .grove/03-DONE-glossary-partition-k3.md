# glossary-partition-k3

**Kind:** work

## Goal

Split the repo's Ubiquitous Language into two bounded contexts behind a root
`CONTEXT-MAP.md`, and rewrite `plugins/CONTEXT.md` into an actual skills-domain
glossary.

## Context

`content/CONTEXT-FORMAT.md` documents the multi-context shape this task adopts —
read its "Multiple contexts" section for the map's format and for how a session
infers which context a topic belongs to. grove has shipped that machinery without
ever using it; this is the first exercise of it.

The file arriving from the skills repo is **not** a domain glossary. It is
titled *"skills — jj adoption context"* and holds four terms from one finished
workstream — `jj-enabled`, `Symmetric VCS rule`, `using-jujutsu`,
`git-to-jj-mapping` — plus an example dialogue. Only `Symmetric VCS rule` is
plausibly durable, and it already has `docs/adr/symmetric-vcs-rule.md`.

## Done when

- `CONTEXT-MAP.md` exists at the repo root, listing both contexts and stating the
  relationship between them — at minimum that grove's methodology cites
  `linkuistics:decision-records` and `linkuistics:codebase-design` as
  documentation-level prerequisites, a dependency that is now intra-repo.
- `CONTEXT.md` (root) is unchanged and remains the **grove** context.
- `plugins/CONTEXT.md` is a glossary of the **skills** domain — what a skill is
  here, how plugins/marketplace/`install.sh` relate, the per-harness install
  story, `paths:` frontmatter vs description-triggered loading, the
  user-invoked-only skills (`authoring-conventions`, `guardrail`). Terms that are
  jj-workstream leftovers are dropped or folded, not preserved out of politeness.
- Every surviving term follows `content/CONTEXT-FORMAT.md`: terse definition,
  `_Avoid_` lines for aliases, **no implementation detail**.
- A skim of both files finds no term defined in both, and no term whose meaning
  differs between them.

## Notes

- Resist letting `plugins/CONTEXT.md` become a table of contents for the skills —
  `README.md` already lists them with their trigger conditions. A glossary
  defines the *language*, not the inventory.
- Root `CONTEXT.md` carries a `## Flagged ambiguities` entry noting that "grove"
  is overloaded three ways. Check whether the merge adds a fourth reading (the
  *repo* now containing both products) and, if so, extend that entry rather than
  starting a new section.
- If the two contexts turn out not to be genuinely distinct once written — if
  the skills glossary keeps reaching for grove terms to explain itself — that is
  worth surfacing rather than forcing: it would mean one glossary was right after
  all, and `docs/adr/skills-monorepo.md` should be edited in place to say so.
