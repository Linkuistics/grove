# shapes-k23

## Goal

Classify **`content/TASK-FORMAT.md` from `## Composing the kinds — the two shapes`
to the line before `## The three design kinds — extra guidance`** (baseline
L193–472, 15,904 bytes): `## Composing the kinds — the two shapes`, `### The review
chain — each session cuts the next step`, `### The vendor pair — one eager call`,
`### What the shapes are not`, `## Suggested shape`, and `## A leaf never names a
harness`.

This is batch 3 of 12, and the largest single region in the plan. It **owns the
corpus-wide condition for the two shapes** (family F) — see *The pre-decided call
in this region*.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L193–472 is a baseline coordinate.** Carve from `##
  Composing the kinds — the two shapes` to the line **before** `## The three design
  kinds — extra guidance`, consuming `pending-task-shapes` in full.
  `TASK-FORMAT.md` is finished after this batch — mint **no** residual.
- The tail beyond your end anchor was already carved by `kinds-k22`; leave it
  alone.

### Edge inventory rows owned: none — and you are the source of three

Your region cites `driving.md` (L204 and elsewhere), and the guides. **Write no
cross-file `defers=`**: no target outside `TASK-FORMAT.md` is carved yet, so under
the ownership rule the later endpoint's batch owns every one of those edges.

You are the *source* of three rows, all written by later batches:

| row | edge | written by |
|---|---|---|
| 32 | your family-F owner → `SKILL.md` `**Cut the next step…**` | #10 `shape-cutting-k30` |
| 33 | your family-F owner → `driving.md` §*The review chain — when doubt earns its own leaves* | #7 `doubt-moves-k27` |
| 26–27 | `SKILL.md` *Cut the next step* / *bare stem* / *grammar is five fields* → **your procedural bodies** | #10 |

Rows 26–27 are conditional on how you split: if the prose `SKILL.md` cites for "the
full reasoning" ends up **triggering**, #10 declines the row with that reason rather
than writing an illegal edge. Either way, **name the unit holding the step-suffix
reasoning in your leaf body** so #10 reads it instead of re-deriving it.

### The pre-decided call in this region

**Family F owner — `## Composing the kinds — the two shapes`'s opening (baseline
L193–212).** *"Reach for them by default, and argue yourself out of one rather than
into it"* plus *"they are built in opposite ways, and the asymmetry is the design"*
is the **owner**: `class=triggering kinds=*`, and the corpus-wide statement of the
condition. It is the earliest of the rule's three sites, which is why it has to
carry it — the two later statements (`driving.md` §*The review chain*, #7; and
`SKILL.md` `**Cut the next step…**`, #10) become procedural bodies rooted from your
unit, and a procedural unit must be reachable at the end of its own batch.

Keep the opening's two shape bullets **with** the asymmetry sentence: the bullets
name the shapes and the sentence states the choice, and a session given one without
the other cannot act.

The **bodies** are everything after that — the exact `leaf-add` invocations, the
`leaf-insert` targeting rule for an integration, who cuts what and when — and they
are rooted from your owner in this same batch.

`### What the shapes are not` (7,946 bytes) is the section to think hardest
about. It is mostly *rejected alternatives* and *why the grammar infers no
relationship*. Rejected-alternative prose is neither a condition nor a
procedure — the node brief says to say so rather than force it into a class. But
"the grammar infers no relationship between leaves" **is** a condition: a session
that assumes an `X` requires a `review-X` after it will cut leaves it does not
need. Split that section rather than classifying it whole.

### Traps specific to this region

- **Splitting mid-fence.** L257–258 and the `leaf-add-pair` example are indented
  code blocks and fenced blocks; the parser forbids a marker inside a fence and
  will say so, but the authoring rule behind it is the one no build checks — a
  unit must read correctly standing alone.
- `## Suggested shape` contains a fenced markdown template with `#` headings
  inside it. Do not let a heading scan mistake those for section boundaries.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-task-shapes` is gone and no `pending-task-*` unit remains.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The family-F owner is in place as `class=triggering kinds=*`, with its shape
  bullets and its asymmetry sentence in one unit.
- The unit holding the step-suffix reasoning, and the family-F owner's id, are both
  named in this leaf's body — #7 and #10 read them rather than re-deriving them.

## Notes

- If any prose in `### What the shapes are not` is genuinely narrative — there to
  make the document readable and neither condition nor procedure — record that as
  a **finding about the design** in this leaf's body. Do not force it into a
  class, and do not silently leave it in a triggering unit to make the build pass.
- Doubts to carry forward, by id.
