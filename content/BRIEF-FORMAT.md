<!-- grove reference file — the BRIEF.md shape -->

# BRIEF-FORMAT — the node briefing

Every node in a grove is a **directory**, and each carries a brief as the
`BRIEF.md` inside it — the root node `.grove/` as `.grove/BRIEF.md`, every other
node as the `BRIEF.md` in its directory `NN-<slug>-k<key>/` (the directory the
leaf became when it was decomposed, keeping its permanent key). It is **process
scaffolding** — neither the glossary (`CONTEXT.md`) nor a decision log
(`docs/adr/`). It exists so that a session executing a leaf can read *three*
ADRs, not fifty: the brief chain, root→leaf, is the curated path into the
project's documented decisions.

A brief is written by the planning task that creates its node. Because a brief
is context, not a task, it is **never** marked done: a node is done
*implicitly*, when no live leaf remains in its subtree, and its brief stays in
place. On that completion, anything still live in the brief is promoted upward
(see SKILL.md, "Retire").

## Suggested shape

A guide, not a schema (constraint 3). Nothing validates a brief; nothing breaks
if a section is missing, reordered, or renamed. Include a section only when it
earns its place (constraint 4).

```markdown
# <slug>-k<key> — brief        (the root brief is titled `# <grove name> — brief`)

## Goal
One or two sentences: what this subtree delivers, and why.

## Done when
The done-criteria rollup for the subtree — the conditions under which every
child is complete and the node retires.

## Decomposition
Why this node is split the way it is, and what the child ordering (the per-level
positions) encodes (dependencies, natural sequence). One line per child is enough.

## Pointers
- ADRs a session here must read: docs/adr/<slug>.md, …
- Glossary terms in play: <term>, <term> (see CONTEXT.md)
- Specs covering this area: docs/specs/<slug>.md
- Test seams this subtree's leaves share: <seam> (see SPEC-FORMAT.md)

## Notes
Anything a session needs that is not yet an ADR or a glossary entry. On
retirement, anything still live here is promoted upward (see SKILL.md, "Retire").
```

## Briefs inherit

A session reads the **whole brief chain**, root→leaf. A child brief states only
what is *new* at its level — it does not repeat the parent. Pointers accumulate
down the chain.
