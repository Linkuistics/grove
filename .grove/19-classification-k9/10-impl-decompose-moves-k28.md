# decompose-moves-k28

## Goal

Classify **`content/driving.md` lines 587–754** (9,508 bytes) and
**`content/BRIEF-FORMAT.md` whole** (4,568 bytes) — 14,076 bytes together.

`driving.md`: `## Externalizing surfaced work`, `## Find working increments before
child leaves`, `## What a good child leaf looks like`, `## Recording fog without
pre-slicing it`, `## Prune, reorder, or file an issue`, `## Anti-patterns`,
`## The shortest version`.

This is batch 8 of 12. It **finishes `driving.md`** and is the last batch before
`SKILL.md`'s middle.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/driving.md` **L587–L754**, consuming `pending-driving-decompose`
  in full. **`driving.md` is finished after this batch — mint no residual for it.**
- Carve `content/BRIEF-FORMAT.md` whole; the seed id `brief-format` is consumed
  and no residual is minted.
- Redistribute any `defers=` inherited from `pending-driving-decompose`.

### Why `BRIEF-FORMAT.md` is in this batch

`BRIEF-FORMAT.md` carries **no condition of its own** — it opens on a statement
("Every node in a grove is a **directory**, and it carries a brief…"), not a
question a session could fail to ask. It needs a root, and two of them are in this
batch's own `driving.md` region:

- **`## Recording fog without pre-slicing it`** (L670–684) explicitly cites
  `BRIEF-FORMAT.md`'s **On the horizon** note — a clean trigger→body edge.
- **`## What a good child leaf looks like`** (L644–669) is the condition a
  `planning` session faces when it is about to write a child brief.

`TASK-FORMAT.md`'s `planning` bullet (L486–487, carved by `kinds-k22`) is the
third inbound edge and should also be written here — the inbound sweep will find
it:

```
grep -rn 'BRIEF-FORMAT\.md' content/
```

`SKILL.md`'s *Decompose* bullet is a fourth; it still sits in a `pending-skill-*`
unit and belongs to `execute-k29`. Leave it or defer from the pending unit, and
**say which**.

### The two orphan sections, and the root they need

`## Anti-patterns` (1,112 bytes) and `## The shortest version` (608 bytes) state
no condition of their own — they are a summary and a digest. They must still be
reachable. Hang them off the framing unit `research-moves-k25` carved at the top
of `driving.md` (its id is recorded in that leaf's body). If you conclude they are
narrative rather than procedure — there to make the document readable and neither
condition nor body — **say so as a finding about the design** rather than forcing
a class; the node brief asks for exactly that, and a summary of a document that is
delivered in slices is a fair candidate.

### The judgement this batch exists for

`## Externalizing surfaced work` (2,331 bytes) states grove's **primary failure
mode** — a session quietly absorbing work that should have been its own leaf.
That is the paradigm triggering unit for the entire design, and the asymmetry
argument in the node brief's *The rule* is written about exactly this case.
Whatever else is debatable here, this one is not.

`## Prune, reorder, or file an issue` (2,233 bytes) states a condition (*a leaf's
place in the tree is in doubt*) and then a triage. Note that pruning is **HITL** —
an agent never prunes on its own — which makes the condition load-bearing in a way
the triage is not.

## Done when

- `content/driving.md` L587–754 and `content/BRIEF-FORMAT.md` are subdivided into
  real units. **No `pending-driving-*` unit remains**, and `brief-format` is gone.
- Every procedural unit in `BRIEF-FORMAT.md` is reachable, and the inbound sweep
  for `BRIEF-FORMAT.md` is run and its outcome stated.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The orphan-section call (`## Anti-patterns`, `## The shortest version`) is
  recorded in this leaf's body with its reasoning.

## Notes

- After this batch, `driving.md`, `TASK-FORMAT.md`, `BRIEF-FORMAT.md`,
  `grilling.md`, `SPEC-FORMAT.md`, `CONTEXT-FORMAT.md` and `ADR-FORMAT.md` are all
  finished. Only `SKILL.md` and `prompts/continue.md` remain, and every cross-file
  target the four `SKILL.md` batches need now exists. **State that in your commit
  message** — it is the gate condition for batches 9–12.
- Doubts to carry forward, by id.
