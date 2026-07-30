<!-- grove reference file — the BRIEF.md shape -->

# BRIEF-FORMAT — the node briefing

Every node in a grove is a **directory**, and a *decomposition* node carries a
brief as the `BRIEF.md` inside it — the root node `.grove/` as `.grove/BRIEF.md`,
every other one as the `BRIEF.md` in its directory `NN-<slug>-k<key>/` (the
directory the leaf became when it was decomposed, keeping its permanent key). It
is **process scaffolding** — neither the glossary (`CONTEXT.md`) nor a decision
log (`docs/adr/`). It exists so that a session executing a leaf can read *three*
ADRs, not fifty: the brief chain, root→leaf, is the curated path into the
project's documented decisions.

**Not every node has one.** A **chain node** — the directory `leaf-add-chain` /
`leaf-add-pair` writes to hold a review chain or a vendor pair — is brief-less by
rule: it means *these steps compose one artifact*, a shape declared whole at
construction with no context anyone is in a position to write, and a stub emitted
because a step demanded it is what constraint 4 forbids. So the presence of
`BRIEF.md` is the **discriminator between the two node species**, which is why
`brief-chain` skipping a level with no brief is load-bearing rather than merely
tolerant, and why the Retire cascade's close has **work to do on a brief-carrying
node and nothing to do on a chain node** — a `Done when` to check and a brief to
promote, against neither (`TASK-FORMAT.md`; ADRs *task-tree-scheme* and
*confirmation-boundary*). Nothing is enforced:
writing a `BRIEF.md` into a chain node simply makes it brief-carrying.

A brief is written by whichever session creates its node — a `planning` task
cutting the tree, or a leaf of any kind that proved bigger than its brief and
decomposed itself (`leaf-decompose` inherits the parent's kind). Because a brief
is context, not a task, it is **never** marked done: a node is done
*implicitly*, when no live leaf remains in its subtree, and its brief stays in
place. On that completion, anything still live in the brief is promoted upward
(see SKILL.md, "Retire").

**Durable content.** The same durability that governs a node's *identity*
(the permanent `-k<key>`, unmoved by renumber or reorder) should govern a
node's *content*: state behavioural contracts and named types — what a
component does, what it guarantees — rather than file paths or line numbers,
which go stale within a session or two as the tree and the code around it
both move.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/triage/AGENT-BRIEF.md, "Durability over precision")
     — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

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

## On the horizon
Foreseen work too dim to leaf yet — a question you can already state
precisely, not one you can already answer (see driving.md, "Recording fog
without pre-slicing it"). Once precise, `leaf-add` it and drop the line here.

## Notes
Anything a session needs that is not yet an ADR or a glossary entry. On
retirement, anything still live here is promoted upward (see SKILL.md, "Retire").
```

## Briefs inherit

A session reads the **whole brief chain**, root→leaf. A child brief states only
what is *new* at its level — it does not repeat the parent. Pointers accumulate
down the chain.
