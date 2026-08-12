# decompose-moves-k28

## Goal

Classify **`content/driving.md` from `## Externalizing surfaced work` to end of
file** (baseline L587–754, 9,508 bytes) and **`content/BRIEF-FORMAT.md` whole**
(4,568 bytes) — 14,076 bytes together.

`driving.md`: `## Externalizing surfaced work`, `## Find working increments before
child leaves`, `## What a good child leaf looks like`, `## Recording fog without
pre-slicing it`, `## Prune, reorder, or file an issue`, `## Anti-patterns`,
`## The shortest version`.

This is batch 8 of 12. It **finishes `driving.md`** and is the last batch before
`SKILL.md`'s middle.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L587–754 is a baseline coordinate.** Carve from `##
  Externalizing surfaced work` to end of file, consuming `pending-driving-decompose`
  in full. **`driving.md` is finished after this batch — mint no residual for it.**
- Carve `content/BRIEF-FORMAT.md` whole; the seed id `brief-format` is consumed
  and no residual is minted.
- **There is nothing to inherit from `pending-driving-decompose`.** A residual never
  carries `defers=`.

### The pre-decided call: `## Externalizing surfaced work` is a *body*

`batches-k13` called this section "the paradigm triggering unit for the entire
design". The corpus disagrees with that in its own words, and the node brief follows
the corpus: L595 reads *"`SKILL.md`'s Decompose step **states the rule**; this is the
**habit that honours it**."*

So family B's **owner** is `SKILL.md` `**Decompose.**` (#9), and this section is its
**body** — `class=procedural`. The rule is no less load-bearing for that; what
changes is only which byte span ships it.

Because your batch runs **before** its owner, you cannot root it from the owner.
**Root it from `driving.md`'s framing unit** (`research-moves-k25` carved it; its id
is in that leaf's body) — inventory **row 12**. That is an honest root, not a
reachability crutch: the framing unit's opening names *"externalizing surfaced work
into new leaves rather than absorbing it"* outright and `## In this guide` indexes
this section by name. #9 later adds the owner's address as row 18.

### Why `BRIEF-FORMAT.md` is in this batch

`BRIEF-FORMAT.md` carries **no condition of its own** — it opens on a statement
("Every node in a grove is a **directory**, and it carries a brief…"), not a
question a session could fail to ask. It needs roots, and two are in this batch's own
`driving.md` region.

### Edge inventory rows owned: 12–16

| row | edge |
|---|---|
| 12 | `driving.md` framing unit → `## Externalizing surfaced work` |
| 13 | `driving.md` framing unit → `## Anti-patterns`, `## The shortest version` |
| 14 | `## Recording fog without pre-slicing it` (L670–684) → `BRIEF-FORMAT.md` §*On the horizon* body — it cites that note explicitly, a clean trigger→body edge |
| 15 | `## What a good child leaf looks like` (L644–669) → `BRIEF-FORMAT.md` bodies — the condition a `planning` session faces when about to write a child brief |
| 16 | `TASK-FORMAT.md`'s `planning` bullet (L485–486, carved by `kinds-k22`) → `BRIEF-FORMAT.md` bodies |

Run the sweep as evidence alongside them:

```
grep -rn 'BRIEF-FORMAT\.md' content/
```

`SKILL.md`'s `**Decompose.**` is a fourth inbound path to `BRIEF-FORMAT.md`, and it
is **row 19, owned by `execute-k29`**. It still sits in a `pending-skill-*` unit, and
**no edge may have a `pending-*` source** — park nothing, and report the hit as *not
yours*.

### The two orphan sections, and the root they need

`## Anti-patterns` (1,112 bytes) and `## The shortest version` (608 bytes) state
no condition of their own — they are a summary and a digest. They must still be
reachable, and row 13 is how. If you conclude they are
narrative rather than procedure — there to make the document readable and neither
condition nor body — **say so as a finding about the design** rather than forcing
a class; the node brief asks for exactly that, and a summary of a document that is
delivered in slices is a fair candidate.

### The judgement this batch exists for

`## Externalizing surfaced work` (2,331 bytes) states grove's **primary failure
mode** — a session quietly absorbing work that should have been its own leaf. The
asymmetry argument in the node brief's *The rule* is written about exactly this case,
which is why its *grain* matters even though its class is settled: split the
condition-shaped opening from the two-triggers mechanics only if each half reads
correctly standing alone.

`## Prune, reorder, or file an issue` (2,233 bytes) states a condition (*a leaf's
place in the tree is in doubt*) and then a triage. Note that pruning is **HITL** —
an agent never prunes on its own — which makes the condition load-bearing in a way
the triage is not.

## Done when

- The `driving.md` region and `content/BRIEF-FORMAT.md` are subdivided into real
  units. **No `pending-driving-*` unit remains**, and `brief-format` is gone.
- Every procedural unit in `BRIEF-FORMAT.md` is reachable, and the sweep for
  `BRIEF-FORMAT.md` is run and its outcome stated.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- **Rows 12–16 are each reported** — written, or declined with a reason. Rows 14, 15
  and 16 all reach `BRIEF-FORMAT.md`, so the build stays green if one is dropped;
  the inventory is what catches that, not `cargo build`.
- The id of the `## Externalizing surfaced work` body unit is named in this leaf's
  body, so `execute-k29` can write row 18 without re-deriving it.
- The orphan-section call (`## Anti-patterns`, `## The shortest version`) is
  recorded in this leaf's body with its reasoning.

## Notes

- After this batch, `driving.md`, `TASK-FORMAT.md`, `BRIEF-FORMAT.md`,
  `grilling.md`, `SPEC-FORMAT.md`, `CONTEXT-FORMAT.md` and `ADR-FORMAT.md` are all
  finished. Only `SKILL.md` and `prompts/continue.md` remain, and every cross-file
  target the four `SKILL.md` batches need now exists. **State that in your commit
  message** — it is the gate condition for batches 9–12.
- Doubts to carry forward, by id.
