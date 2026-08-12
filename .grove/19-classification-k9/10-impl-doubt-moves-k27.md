# doubt-moves-k27

## Goal

Classify **`content/driving.md` from `## Doubting inside a picked Grove leaf` to the
line before `## Externalizing surfaced work`** (baseline L415–586, 11,128 bytes):
`## Doubting inside a picked Grove leaf` (2,188 bytes) and `## The review chain —
when doubt earns its own leaves` (8,940 bytes).

This is batch 7 of 12. `## The review chain` is the **single largest section in
`driving.md`**, which is why these two sections get a batch of their own.

**Both of your sections are procedural bodies of rules whose conditions are owned
elsewhere.** That is the whole shape of this batch, and it is settled rather than
yours to decide — see below.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L415–586 is a baseline coordinate.** Carve from `##
  Doubting inside a picked Grove leaf` to the line **before** `## Externalizing
  surfaced work`, consuming the front of `pending-driving-doubt`.
- Mint exactly one residual, **`pending-driving-decompose`**, covering `##
  Externalizing surfaced work` to end of file, as `class=triggering kinds=*` **with
  no `defers=`**.
- **There is nothing to inherit from `pending-driving-doubt`.** A residual never
  carries `defers=`, so there is no list to redistribute.

### The pre-decided calls in this region

`batches-k13` asked this batch to *decide* two cross-file overlaps and hand the
calls to `execute-k29` and `shape-cutting-k30`. `batches-k33` F4 found that
backwards — you would be deciding before the hub batches had classified, and from an
incomplete list of sites. Both calls are now in the node brief:

- **Family A body — `## Doubting inside a picked Grove leaf` (L415–453).**
  **Procedural.** The owner is `TASK-FORMAT.md` §*In-session doubt is budgeted across
  the whole picked leaf* (#2), which carries the predicate and the five-row
  allowance table. **Root your unit from it (row 11)** — that edge is what makes
  your section reachable at the end of *this* batch, so it is not optional.
  This section states the predicate's **negative half** that the owner does not —
  *merely finding `.grove/` or inheriting Grove control variables does not activate
  it*. Keep it together with the rest of the section, and **record in your body that
  the shipped condition lacks this sharpening**; the aggregate reviewer decides
  whether that matters, and #2 records the same doubt from its own side.
- **Family F body — `## The review chain — when doubt earns its own leaves`
  (L455–586).** **Procedural.** The owner is `TASK-FORMAT.md` §*Composing the kinds
  — the two shapes*'s opening (#3). **Root it from that unit (row 33).** Read
  `shapes-k23`'s body for the owner's id rather than re-deriving it.
- Do **not** expect `execute-k29` or `shape-cutting-k30` to defer into your units on
  their own reading. Rows 17 and 32 are the edges from the two owners into *their*
  restatements, and those batches own them.

Both sections are still large enough to need several units each, and the grain
inside them is entirely yours: expect the mechanics of cutting each step, writing
its body, and placing an integration to split several ways.

### Edge inventory rows owned: 11, 33 and 36

| row | edge | note |
|---|---|---|
| 11 | `TASK-FORMAT.md` family-A owner → `## Doubting inside a picked Grove leaf` | Not optional — it is this section's only root |
| 33 | `TASK-FORMAT.md` family-F owner → `## The review chain…` | Not optional — same reason |
| 36 | `## The review chain…` → `TASK-FORMAT.md` chain-mechanics bodies | Conditional: only where the citation is a genuine trigger→body reference and the target is procedural. Decline with a reason otherwise |

`SKILL.md` references in this region still land in `pending-skill-*` units. **No edge
may have a `pending-*` source** — do not park a `defers=` there, and do not treat the
reference as an obligation of yours. Report those hits as *not yours* (rows 17 and 32
are their owners' work); silence is what F2 made indistinguishable from a miss, and a
report is not silence.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-driving-decompose` covers the rest of the file and nothing else, and
  carries no `defers=`.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- **Rows 11, 33 and 36 are reported** — 11 and 33 written, 36 written or declined
  with a reason.
- The ids of both body units are named in this leaf's body, so #9 and #10 can write
  rows 17 and 32 without re-deriving them.
- The missing negative half of the family-A owner's predicate is recorded as a doubt.

## Notes

- This region contains fenced `grove-llm leaf-add` / `leaf-insert` examples. Do
  not split mid-fence.
- Doubts to carry forward, by id. The condition/body split between this file and
  `SKILL.md` is no longer yours to make, but whether the *pre-decided* split is
  right is exactly what the aggregate reviewer is for — so record what you saw that
  makes you doubt it, if anything does.
