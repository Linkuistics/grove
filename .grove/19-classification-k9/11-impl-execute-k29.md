# execute-k29

## Goal

Classify **`content/SKILL.md` lines 167–245** (5,454 bytes): `**Execute.**` and
its three bullets, `**Review ownership inside a picked leaf.**`, the *"Whichever
kind is running"* paragraph, and `**Decompose.**` with its two bullets.

This is batch 9 of 12. It is the **smallest region in the plan and the densest in
cross-file deferral** — six of the nine embedded files are named in these
79 lines.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/SKILL.md` **L167–L245**, consuming the front of
  `pending-skill-loop`.
- Mint exactly one residual, **`pending-skill-shapes`**, covering **L247–L760**.
- Redistribute any `defers=` inherited from `pending-skill-loop` — batches 4–8 may
  have parked edges there.

### Every cross-file target now exists

`decompose-moves-k28` finished the last of them. This region's edges, all of them
now writable:

| `SKILL.md` site | target file | carved by |
|---|---|---|
| L171–172 — *`TASK-FORMAT.md` for every kind's discipline and its HITL/AFK mark* | `TASK-FORMAT.md` | `kinds-k22` |
| L176–179 — *`requirements` opens with a grilling session (`grilling.md`)* | `grilling.md` | `guides-k24` |
| L217–218 — *raise ADRs sparingly (`ADR-FORMAT.md` for placement)* | `ADR-FORMAT.md` | `guides-k24` |
| L219–220 — *write a spec only at a genuine agreement point (`SPEC-FORMAT.md`)* | `SPEC-FORMAT.md` | `guides-k24` |
| L224–227 — *See `driving.md` for the field-guide habits…* | `driving.md` | `research-moves-k25`ff |
| L237–238 — *turn the leaf into a node (a brief, `BRIEF-FORMAT.md`…)* | `BRIEF-FORMAT.md` | `decompose-moves-k28` |
| L240 — *each child shaped as a vertical slice (`driving.md`)* | `driving.md` §*What a good child leaf looks like* | `decompose-moves-k28` |

**L224–227 is the catch-all root** — it is what `decompose-moves-k28` hung
`driving.md`'s orphan sections off, if it chose to leave them. Check that leaf's
body before you carve, and close whatever it left open.

### The judgement this batch exists for

Three of the four paragraphs here are unambiguous conditions, and the fourth is
the one to think about.

- **`**Execute.**`** states that the filename carries the kind and that the closed
  set has nineteen members. Triggering, `kinds=*`. Its three bullets differ: the
  first two describe *other* kinds' disciplines, which `kinds-k22` may already
  hold — check for duplication before carving a second statement of the same rule.
- **`**Review ownership inside a picked leaf.**`** is the in-session reviewer
  budget. `doubt-moves-k27` classified the `driving.md` statement of the same
  rule and recorded which side holds the condition. **Read that call and honour
  it.** Duplicating the condition on both sides puts it in every mandate twice;
  putting it on neither loses it.
- **The *"Whichever kind is running"* paragraph** is three rules fused into one
  paragraph: raise ADRs sparingly, write a spec only at an agreement point, and
  see `driving.md` for the habits. Each has a different target and arguably a
  different scope. Splitting it is probably right; if you keep it whole it needs a
  three-member `defers=` list, which is legal and may be the better reading.
  Decide explicitly.
- **`**Decompose.**`** is grove's primary failure mode stated on the hub side —
  *default to externalizing rather than absorbing*. `decompose-moves-k28`
  classified the `driving.md` statement. Same overlap question, same discipline:
  read the recorded call.

### Scope

Nothing in this region is honestly narrower than `kinds=*`. `**Execute.**`'s
bullets describe per-kind discipline but are addressed to *every* session
deciding what a leaf of some kind will do — resist the pull to scope them, and if
you do scope one, say why. Remember there is no family shorthand: "every producer"
is spelled out in full or is `*`.

## Done when

- `content/SKILL.md` L167–245 is subdivided into real units;
  `pending-skill-shapes` covers L247–760 and nothing else.
- Every edge in the table above is either written or explicitly declined with a
  reason recorded in this leaf's body.
- Any `defers=` inherited from `pending-skill-loop` is redistributed and accounted
  for.
- `cargo build` and `cargo test` are green — this is the batch most likely to
  provoke the reachability, class and termination checks, because it writes the
  most edges.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.

## Notes

- 5,454 bytes is small for a batch by design. The cost here is the six-file edge
  set and three overlap decisions with already-classified files, not the prose.
  **Do not absorb `pending-skill-shapes`** to fill the session — `shape-cutting-k30`
  has its own overlaps to reconcile.
- Doubts to carry forward, by id. Every overlap call you make (Review ownership,
  Decompose) belongs in the aggregate review handoff whether or not you are
  confident.
