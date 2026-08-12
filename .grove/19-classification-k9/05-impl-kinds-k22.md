# kinds-k22

## Goal

Classify **two regions of `content/TASK-FORMAT.md`** (13,189 bytes together): the
file's body start down to the line before `## Composing the kinds — the two
shapes` (baseline L1–192), **plus** `## The three design kinds — extra guidance` to
end of file (baseline L473–501). That is the file's framing and filename grammar,
`## The nineteen kinds` with its per-kind discipline bullets, and the three-design-
kinds guidance.

This is batch 2 of 12, and it is **where the `kinds=`-scoped units live if they
live anywhere**. It is the highest-value classification judgement in the corpus,
and it now also **owns the corpus-wide condition for the in-session reviewer
budget** — see *The pre-decided calls in this region*.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; the line ranges are pre-classification baseline
  coordinates.** Carve from the file's body start to the line **before** `##
  Composing the kinds — the two shapes`, and from `## The three design kinds —
  extra guidance` to end of file. The two regions are one closure: the tail is
  extra guidance on three of the nineteen kinds the head introduces.
- The seed unit `task-format` is **consumed**. Mint exactly one residual,
  **`pending-task-shapes`**, covering the middle — `## Composing the kinds — the
  two shapes` to the line before `## The three design kinds — extra guidance` — as
  `class=triggering kinds=*` **with no `defers=`**. `shapes-k23` consumes it.

### Edge inventory rows owned: none — but you are the *source* of five

Your region names `grilling.md` (L77, L478), `driving.md` (L99–101),
`ADR-FORMAT.md`, `SPEC-FORMAT.md` and `BRIEF-FORMAT.md` (L481–486). **Write no
`defers=` for any of them.** Those files are still single trivial triggering units,
so the edge is not yet writable — and under the ownership rule the later-carved
endpoint's batch owns it, which is never you:

| row | your unit is the source of an edge to | written by |
|---|---|---|
| 1 | `grilling.md` bodies, from your `requirements` bullet | #4 `guides-k24` |
| 2 | `ADR-FORMAT.md` bodies, from your `design` bullet (L481–484) | #4 |
| 3 | `SPEC-FORMAT.md` bodies, from the same `design` bullet | #4 |
| 4 | `grilling.md` and `CONTEXT-FORMAT.md` bodies, from the *three design kinds* `requirements` bullet (L478–480) | #4 |
| 16 | `BRIEF-FORMAT.md` bodies, from your `planning` bullet (L485–486) | #8 `decompose-moves-k28` |
| 11, 17 | `driving.md` §*Doubting…* and `SKILL.md` *Review ownership…*, from your family-A owner unit | #7, #9 |

**Name every one of those units so a later batch finds it by `grep -n` on a
heading or bold-lead, and list their ids in your leaf body.** Five batches reach
back into your markers; an id they can only locate by line number costs each of
them a re-derivation against a file you have already shifted.

Nothing here is an exception to the decoupling lemma or a deferred obligation of
yours: nothing has moved yet, so the "body moved but no `defers=`" trap is not
live, and there is no residual to park an edge on.

### The pre-decided calls in this region

The node brief settles these; apply them rather than re-deciding. Your region
carries **six** sites across four families.

- **Family A owner — `**In-session doubt is budgeted across the whole picked
  leaf**` (baseline L164–177).** This is the **owner**: `class=triggering
  kinds=*`, the predicate plus the five-row allowance table, and it is the
  corpus-wide statement of the reviewer budget that every mandate carries. Both
  other statements of the rule — `driving.md` §*Doubting inside a picked Grove
  leaf* (#7) and `SKILL.md` *Review ownership inside a picked leaf* (#9) — become
  procedural bodies rooted from **your** unit, which is why it has to be here: a
  procedural unit must be reachable at the end of its own batch, and yours is the
  earliest batch of the three.
  **Do not split the predicate from the table.** The predicate ("once the current
  session has run Bootstrap and adopted the driver's selected-leaf mandate") is
  what makes the allowance readable; a table alone states a budget with no
  precondition.
  **One residue to record, not to fix:** your prose states the predicate
  positively and does not state its negative half — that merely finding `.grove/`
  or inheriting Grove control variables does not activate it — which both bodies
  do. Record that in your leaf body for the aggregate reviewer; whether the
  predicate ships completely enough is a review question, not yours.
- **Family B second condition — the `design` bullet's drift clause (L80–83),**
  *a `design` session that finds itself cutting implementation leaves has drifted
  into planning's job and should externalize a `planning` leaf instead.* This is a
  **second condition**, not a restatement: a kind-scoped drift detector. It ships
  on its own account, and it owes no edge to family B's owner
  (`SKILL.md` `**Decompose.**`, #9).
- **Family B mention — the `impl` bullet's parenthesis (L99–101).** A clause inside
  a unit whose subject is the `impl` kind's deliverable. It takes the bullet's
  class, owes no edge, and is **not a site to decide**.
- **Families D and E second condition — the `design` bullet in `## The three design
  kinds` (L481–484).** *Raises ADRs sparingly… and MAY write a spec when the
  increment is a genuine agreement point.* A **second condition**, and a genuine
  `kinds=design` candidate — one of the very few honest explicit scopes in the
  corpus. It is also the planned inbound root for both format guides (rows 2–3).
- **Family C mentions — L144 (`review-design`'s "are the ADRs a minimum coherent
  set?") and L157–158 (`integrate-review-design`'s in-place discipline).** Clauses
  inside the review-kind and integrate-kind bullets. No decision, no edge.

### The judgement this batch exists for

`kinds=*` is the overwhelming default; an explicit list is for guidance genuinely
about **one kind's discipline**. This region is the densest concentration of such
guidance in the corpus, and there is **no family shorthand and no negation** — a
scope that wants to say "every producer" is spelled out in full or is `*`.

Weigh each of these separately rather than as a block:

- The **per-kind bullets** (L74–115: five producers, the research trio) — each is
  a candidate `kinds=<that kind>` unit. But a kind's discipline is also what a
  session of *another* kind needs when deciding what to cut next, which argues
  some of them are `kinds=*`. Decide per bullet, and say why.
- The **HITL/AFK paragraph** (L64–72) — it is about all nineteen and ends with
  "the mark **predicts, it does not permit**", a condition every kind needs. Very
  likely `kinds=*`.
- The **kind table and the "closed set" paragraph** (L37–62) — the fact that the
  set is closed and that `finish` is driver-reserved is a condition; the table
  itself may be its procedural body.
- The **filename-grammar framing** (L6–33) — `pruning is HITL, never an agent's
  own call` is a condition with real teeth; the round-tripping rules read
  procedural.

### Traps specific to this region

- **Classifying by size.** A long conditional is still a condition; the `planning`
  bullet (L84–94) is the longest and is plainly triggering for `planning`.
- **`kinds=` is required on triggering and forbidden on procedural.** A scoped
  procedural unit is a build error, and it is the easiest mistake to make here
  because the scope feels informative.

## Done when

- Both regions are subdivided into real units; `pending-task-shapes` covers the
  middle region and nothing else, and carries no `defers=`.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit: `task-format` removed, the new
  `task-*` ids added, `pending-task-shapes` added — each named deliberately.
- The ids five later batches must reach back into are listed in this leaf's body:
  the `requirements`, `design`, `planning` and `impl` bullets, the three
  design-kind bullets, and **the family-A owner unit**.
- The family-A owner is in place as `class=triggering kinds=*`, predicate and table
  together, and the missing negative half of its predicate is recorded as a doubt.

## Notes

- Follow the id-naming convention `spine-k21` recorded; do not invent a second.
- Doubts to carry forward, by id — the scope decisions here are the ones most
  likely to be wrong in a way no build sees, so be generous about naming them.
