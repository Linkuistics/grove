<!-- grove reference file — the BRIEF.md shape -->

# BRIEF-FORMAT — the node briefing

Every node in a grove is a **directory**, and it carries a
brief as the `BRIEF.md` inside it — the root node `.grove/` as `.grove/BRIEF.md`,
every other one as the `BRIEF.md` in its directory `NN-<slug>-k<key>/` (the
directory the leaf became when it was decomposed, keeping its permanent key). It
is **process scaffolding** — neither the glossary (`CONTEXT.md`) nor a decision
log (`docs/adr/`). It exists so that a session executing a leaf can read *three*
ADRs, not fifty: the brief chain, root→leaf, is the curated path into the
project's documented decisions.

**There is one node species, and it always has one.** A node is a leaf that
proved bigger than one session, so the charter is exactly the context those extra
sessions need, and every node grove writes gets one: `leaf-decompose` moves the
decomposed leaf's own body in as the brief, and `root-init` scaffolds the root's.
Nothing composes leaves into a node any more — a review chain's steps are flat
siblings each session cuts as its last act, and a vendor pair's are three flat
siblings — so there is no longer a species that means *these steps compose one
artifact* and has no context anyone could write (flat-lazy-review;
`TASK-FORMAT.md`). The Retire cascade's close therefore has the same work at
every node it meets: a `Done when` to check against the subtree and a brief to
promote upward.

**Nothing is enforced**, and `brief-chain` still skips a level with no brief. A
brief is a lazy artifact (constraint 4) and briefs are freeform markdown that
nothing validates (constraint 3), so a reader must not fail on a node whose
charter has not been written yet — but a node without one is a lapse to fix, not
a second kind of node.

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
