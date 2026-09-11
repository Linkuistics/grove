<!-- grove reference file — the node-file and brief shape -->

# BRIEF-FORMAT — the node briefing

Every node in a grove is a **directory** with exactly one regular **node file**:
the root has `.grove/_BRIEF.md`; a positioned node is `NN-k<key>/` with
`_<slug>.md` inside it. The directory carries position and permanent key; the
node file's name supplies the title. Its body is the **brief**. A node's handle
combines that filename's slug with the directory's key: `<slug>-k<key>`.
Titles are always extracted from names, never from headings or other contents.
`_BRIEF.md` belongs only at the root, and `BRIEF` is reserved, not a slug.

```text
.grove/
  _BRIEF.md
  01-k7/
    _extract.md
    01-impl--extract-k8.md
    02-DONE-review-impl--extract-k9.md
```

The brief is **process scaffolding** — neither the glossary (`CONTEXT.md`) nor a
decision log (`docs/adr/`). It exists so that a session executing a leaf can read *three*
ADRs, not fifty: the brief chain, root→leaf, is the curated path into the
project's documented decisions.

**There is one node species, and it always has one node file.** A node is a leaf that
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

**The node file is mandatory; its body is freeform.** Missing, competing or
misplaced node files make a level malformed. Tree reads and mutations refuse
that level by name and give the canonical form. A `_`-prefixed entry belongs to
this grammar even when malformed. Create the node file with its node; add brief
sections only when they earn their place (constraints 3 and 4).

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

A guide to the body, not a schema (constraint 3). Nothing validates its sections;
nothing breaks if a section is missing, reordered, or renamed. Include a section
only when it earns its place (constraint 4).

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

## Series                                       (a pass series node only)
The step sequence, the exit condition, the cap, and what happens at the cap —
written before the first pass runs. Plus, if a pass reached the cap, which one
and what it left.

## On the horizon
Foreseen work too dim to leaf yet — a question you can already state
precisely, not one you can already answer (the fog-or-ticket test, in
references/decompose.md). Once precise, `leaf-add` it — with its `--kind` — and drop the line here.

## Notes
Anything a session needs that is not yet an ADR or a glossary entry. On
retirement, anything still live here is promoted upward (see SKILL.md, "Retire").
```

## A series brief declares the repetition

A **pass series** is a node whose children are its *passes* — work that was
completed and is deliberately done again over one subject
(`references/decompose.md`). Its brief carries what nothing else in a grove can,
and it carries it **before the first pass runs**:

1. **The step sequence** — the kinds, in order, that one pass runs.
2. **The exit condition** — a fact a session can check at the end of a pass.
3. **The cap** — how many passes, as a number.
4. **What happens at the cap** — stop and say so, which is the only answer there
   is. It is written down anyway, so that the session which meets the cap is not
   the one deciding what a cap means from inside the series.

A node whose children happen to repeat, with none of those four written down, is
not a series. Writing them down is the whole of the difference: nothing parses a
brief and nothing checks a declaration against what the passes did, so these are
discipline this file records rather than a shape the tree enforces.

**The exit condition is a fact about the subject, never a trend in what the
passes are finding.** A gate that passes, a threshold fixed in advance, a
checkable state — each is a fact a later session can check and get the same
answer you would. *Stop once a pass finds little* is not: it reads the series'
own yield as its stopping rule, so the number that ends the series is chosen by a
session already inside it, which is what declaring the condition in advance
exists to prevent.

**The cap bounds the series, not the work.** Exit means *this shape stops here*,
and the subject may well go on being worked as ordinary leaves somewhere with no
cap over them. A brief whose cap is written as a claim that the subject will be
finished has promised something the shape cannot deliver.

**A session that reaches the cap writes what happened here before it stops** —
which pass reached it, and what was still unfinished. That is context and not a
log: it says what the series *is* now, stopped at its cap, which is a state the
tree does not carry. A satisfied exit and a reached cap both cut nothing, so
without this entry the two leave a **byte-identical** tree, a stalled series is
indistinguishable from a cleanly exited one, and nothing will detect it. Say it
in that session's commit message too — a brief is the durable place *in the
tree*, and `.grove/` itself does not outlive the finish cycle.

## Briefs inherit

A session reads the **whole brief chain**, root→leaf. A child brief states only
what is *new* at its level — it does not repeat the parent. Pointers accumulate
down the chain.
