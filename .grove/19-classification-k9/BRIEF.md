# classification-k9 — brief

## Goal

Replace the trivial marking with the **real** one: subdivide `content/`'s
one-unit-per-file partition into the actual triggering/procedural split, with
`kinds=` scopes and `defers=` targets, across ~145 kB of embedded markdown.

This is the load-bearing, judgement-heavy work the grilling earmarked — wrong in
a way no compiler sees. It is a **pure `content/` edit**: no Rust changes, and
every existing unit is only ever subdivided, never merged or moved between files.

**This was a single leaf until `increments-integrate-k12`.** `increments-review-k11`
B5 found it larger than one focused session — one context asked to classify
139,136 bytes across nine files, make every unit-boundary and scope/deferral
judgement, preserve cross-file reachability, update the pinned complete id set,
verify the installed listing, and author the aggregate review handoff. The three
largest files alone are 51,524, 41,954 and 24,894 bytes. It is now a node, and its
first child is the `planning` leaf that derives the batches.

## The rule, and it is the only rule

**Keep the `if`, defer the `then`.** A rule's *condition* — that a situation
exists calling for something other than what this session is doing — is
`class=triggering` and ships in every mandate its scope admits. Its *body* — how
to act once that is decided — is `class=procedural`, ships in no mandate, and is
reached through `defers=`.

The asymmetry is the whole argument, so classify against it directly: withholding
a procedural body costs a lookup the session knows to make; withholding a
triggering condition yields an **unasked question**, which is grove's primary
failure mode — a session quietly absorbing work that should have been its own
leaf. When a unit is genuinely ambiguous, that asymmetry is the tie-breaker, not
size and not frequency.

`docs/adr/mandate-delivers-the-methodology.md` and the spec's *Keep the `if`,
defer the `then`* carry the argument in full; `CONTEXT.md`'s
[[Triggering unit]] / procedural unit entry carries the two `_Avoid_` traps.

### The corpus, largest first

Byte counts restated by `batches-k13` against the corpus as it stands after
`drop-step-suffix-k18` — the figures this table first carried predate that work.

| file | bytes | what to expect |
|---|---|---|
| `content/SKILL.md` | 52,323 | the spine and the loop; the highest density of conditions, and the finest grain — a kind's discipline is one bullet out of nineteen |
| `content/driving.md` | 42,744 | mostly procedural: habits and moves, entered once a session has decided to do the thing — but its `## When to …` heads are genuine conditions, so it roots much of itself |
| `content/TASK-FORMAT.md` | 29,093 | the kind taxonomy and the filename grammar; `kinds=`-scoped units live here if anywhere |
| `content/SPEC-FORMAT.md` | 5,364 | format guide — largely procedural bodies behind a triggering "when you need one" |
| `content/grilling.md` | 4,735 | vendored; procedural, entered from `requirements` |
| `content/BRIEF-FORMAT.md` | 4,568 | format guide |
| `content/CONTEXT-FORMAT.md` | 3,502 | vendored; format guide |
| `content/ADR-FORMAT.md` | 2,066 | format guide, mostly a pointer to `linkuistics:decision-records` |
| `content/prompts/continue.md` | 838 | still live and still true under provisioning; leave its text alone |

`kinds=*` is the overwhelming default. An explicit list is for guidance genuinely
about one kind's discipline — and there is no family shorthand and no negation, so
a list that wants to say "every producer" is spelled out or is `*`.

### What will be tempting and is wrong

- **Classifying by size.** A long conditional is still a condition.
- **Classifying by frequency.** The test is whether the session could know to
  *ask*, not how often it needs the answer.
- **Leaving a triggering unit with no `defers=` when its body moved.** Absence of
  `defers=` is meaningful — it tells the session the unit is complete as
  delivered. A condition whose procedure went to another unit and does not name it
  is exactly the "unasked question" in miniature.
- **Reading a unit's own id as the address of its deferred body.** One namespace
  covers both classes, so that id fetches the unit again.
- **Splitting mid-fence.** The parser forbids it, so the build will say so — but
  the authoring rule behind it is the one no build checks: a unit must read
  correctly standing alone, because a mandate is units joined by a blank line and
  nothing else.

## Why this is a node, and what the batching has to respect

**A blind one-file-per-child split is not automatically green.** The gate
`embed-wide-gate-k8` lands enforces `defers=` resolution, target class, and
procedural reachability *across the whole embed*. So a newly classified triggering
unit in one file may defer to a procedure whose file is still represented by a
single trivial `class=triggering kinds=*` unit — and the build rejects that twice
over: the target does not exist, and if a same-named one did it would be of the
wrong class. `content/SKILL.md` defers heavily into `driving.md`, `grilling.md`
and `TASK-FORMAT.md`, so this is the normal case, not a corner.

Batch by **deferral closure and session size**, not by filename. Every child must
leave `cargo build` and `cargo test` green on its own commit — that is the whole
reason the batching is planned before any of it is executed.

## The batching contract — settled by `batches-k13`

`batches-k13` mapped the closure with the files open and cut the twelve batch
children below. What follows is what those children share; each child's own body
carries its region, its edges and its doubts. **Read this section before writing a
single marker** — it is what makes a batch green without a trial build.

### The two lemmas that make incremental classification safe

**The greenness lemma.** An un-carved region stays covered by a
`class=triggering kinds=*` unit. A triggering unit carries **no** reachability
obligation (it ships in every mandate its scope admits) and `defers=` is optional
on it. So **the un-carved residue can never fail the gate**, and a batch's
greenness depends only on what that batch *creates*:

- **(D)** every `defers=` the batch writes names a unit that exists at end of
  batch and is `class=procedural`;
- **(R)** every `class=procedural` unit the batch creates is reachable, at end of
  batch, by following `defers=` from a triggering unit;
- **(T)** no `defers=` chain the batch creates returns to a unit already passed
  through.

**The decoupling lemma.** A later batch may add `defers=` to a marker an earlier
batch wrote, **without changing that unit's id or its boundaries** — a marker line
is one line. So a condition and its body need not be carved together. The
consequence is the rule the whole order rests on:

> **Procedural units drag their inbound edge into their own batch; triggering
> units drag nothing.**

A batch may therefore carve a triggering unit whose body is still sitting,
un-carved, inside another file's residue, and write **no** `defers=` for it. That
is not the "body moved but no `defers=`" trap in the node brief above: nothing has
moved yet. The trap fires only if a batch moves a body and leaves no edge — and
the inbound sweep below is what prevents that.

### The inbound sweep — how a batch finds every edge it owes

Before finishing, a batch that carves file `F` runs

```
grep -rn '<F>' content/
```

and, for **every** hit outside `F`, decides whether that reference is a
trigger→body edge. Where it is, the batch adds `defers=` to the referring unit.
This is mechanical and complete, and it is why no ledger of pending edges is
needed anywhere.

Two cautions:

- **A prose cross-reference is not automatically a `defers=`.** `driving.md`
  names `SKILL.md` eleven times, and most of those point at a *constraint* or a
  *condition* — a `class=triggering` unit, which is an illegal `defers=` target.
  The build catches it, but do not spend a cycle discovering that.
- **If the referring text is still inside a `pending-` unit**, add the `defers=`
  to the `pending-` unit itself (legal: a triggering unit may defer). The batch
  that later carves that region **redistributes** the list onto the real units it
  creates. The list sitting on the marker being replaced *is* the checklist, so
  nothing has to be remembered across sessions.

### The `pending-` convention — an id prefix that makes "not finished" greppable

Every region not yet carved is covered by a unit whose id begins **`pending-`**,
`class=triggering kinds=*`. It is an ordinary unit: it is pinned in
`EMBEDDED_UNITS` like any other, and it may carry `defers=` per the caution above.

- A batch carving the front of a `pending-` unit **consumes** that id and mints
  **one** new `pending-` id for whatever remains. Carving a file's head *and* tail
  in one batch leaves a middle residual — also fine, also one mint.
- A batch that carves a whole file mints none.
- **The final batch leaves zero.** `grep -rc '<!-- unit: pending-' content/`
  returning 0 is the mechanical statement that the classification is complete, and
  it belongs in the final batch's verification.

The nine ids that exist today (`skill`, `task-format`, `driving`, `grilling`,
`spec-format`, `brief-format`, `context-format`, `adr-format`, `continue`) are the
seed residuals. Real units take a file-scoped id prefix — `skill-`, `task-`,
`driving-`, `grilling-`, `spec-`, `brief-`, `context-`, `adr-`, `continue-` — which
makes cross-batch id collision impossible without coordination.

### Which files can root themselves, and which cannot

This is what actually fixes the order. Four files carry **no condition of their
own** — they are procedures entered from elsewhere — so their batch must reach
back to a root another batch already carved:

| file | own conditions? | rooted from |
|---|---|---|
| `driving.md` | yes — its `## When to …` heads | itself, plus `SKILL.md` for its orphan sections |
| `TASK-FORMAT.md` | yes — its framing and per-kind discipline | itself |
| `SPEC-FORMAT.md` | yes — "when the increment is a genuine agreement point" | itself, plus `grilling.md` §*Agree the test seams* |
| `SKILL.md` | it is the hub | itself |
| `grilling.md` | **no** | `TASK-FORMAT.md`'s `requirements` bullet (`kinds=requirements`) |
| `ADR-FORMAT.md` | **no** | `grilling.md` §*Offer ADRs sparingly*, or `SKILL.md`'s ADR paragraph |
| `CONTEXT-FORMAT.md` | **no** | `grilling.md` §*Update CONTEXT.md inline*, or `SKILL.md`'s glossary paragraph |
| `BRIEF-FORMAT.md` | **no** | `driving.md` §*Recording fog…* / §*What a good child leaf looks like* |
| `prompts/continue.md` | it is the framing; all triggering | needs no root — it creates no procedural unit |

Reachability is **per kind**, not universal: a procedure reached only from a
`kinds=requirements` condition is reachable from the `requirements` mandate and
from no other, and that is correct, not a gap.

### The batch order, and why each boundary is green

Twelve children, in this order. It satisfies every
dependency; the sizes are the body bytes each session must *judge*, not merely
read. Corpus total is 145,233 bytes (grown from the 139,136 this brief first
recorded, by `drop-step-suffix-k18`).

| # | leaf | region | bytes | green because |
|---|---|---|---|---|
| 1 | `spine` | `SKILL.md` L5–166 | 12,024 | self-contained; no cross-file `defers=` at all |
| 2 | `kinds` | `TASK-FORMAT.md` L1–192 + L473–501 | 13,189 | self-rooted; its outward pointers stay un-deferred (decoupling lemma) |
| 3 | `shapes` | `TASK-FORMAT.md` L193–472 | 15,904 | self-rooted from §*Composing the kinds* |
| 4 | `guides` | `grilling.md` + `ADR-FORMAT.md` + `SPEC-FORMAT.md` + `CONTEXT-FORMAT.md` | 15,667 | roots `grilling.md` at #2's `requirements` unit, then chains the other three off `grilling.md`'s own move sections |
| 5 | `research-moves` | `driving.md` L1–263 | 13,580 | self-rooted from its `## When to …` heads |
| 6 | `evidence-moves` | `driving.md` L264–414 | 8,528 | self-rooted |
| 7 | `doubt-moves` | `driving.md` L415–586 | 11,128 | self-rooted |
| 8 | `decompose-moves` | `driving.md` L587–754 + `BRIEF-FORMAT.md` | 14,076 | roots `BRIEF-FORMAT.md` at `driving.md` sections carved in the same batch |
| 9 | `execute` | `SKILL.md` L167–245 | 5,454 | every cross-file target it defers to exists after #2–#8 |
| 10 | `shape-cutting` | `SKILL.md` L247–406 | 10,067 | targets in `driving.md` (#7) and `TASK-FORMAT.md` (#3) exist |
| 11 | `lifecycle` | `SKILL.md` L408–608 | 13,711 | targets in `BRIEF-FORMAT.md` (#8) and `ADR-FORMAT.md` (#4) exist |
| 12 | `finish-cycle` | `SKILL.md` L610–760 + `prompts/continue.md` | 11,621 | everything exists; **final** — consumes the last `pending-` unit |

The hard ordering edges are only these: **#4 after #2**; **#9 after #2, #4, #5–#8**;
**#10 after #3 and #7**; **#11 after #4 and #8**; **#12 last**. Position order
satisfies all of them with slack.

### Two obligations in every batch, restated because they are easy to skip

- **Update `EMBEDDED_UNITS` in `tests/methodology.rs` in the same commit**, naming
  each new id deliberately — that set equality is the design's confirmation point,
  and it fails in both directions, so the `pending-` churn shows up there too.
- **Record the units you were least sure about, by id, in your leaf body** before
  retiring, so #12 assembles the aggregate review handoff rather than
  reconstructing it from twelve diffs.

## Done when

- Every embedded markdown file is subdivided into real units; `cargo build` and
  `cargo test` are green, including reachability across the whole embed.
- **No `pending-` unit remains** — `grep -rc '<!-- unit: pending-' content/`
  returns 0. That is the mechanical statement that the subdivision is finished
  rather than merely green (see *The batching contract* below).
- The pinned complete id set in the test constant is updated deliberately — every
  new id named by a human decision, which is exactly the moment the design
  intends the classification to be confirmed.
- `grove-llm methodology` (from `addressable-embed-k7`) lists the real
  classification, and spot-fetching a triggering unit shows a `defers=` target
  that answers it.
- An **aggregate `review-impl` leaf** has run over the whole classification —
  cut inside this node, after the final batch, with a body naming the pre-
  classification baseline commit and **every** batch child's `<slug>-k<key>`
  handle, so the reviewer inspects the whole classification rather than only the
  closing commit. This is the leaf the design earmarked; the final batch's
  producer cuts it, and decides *what* to point the reviewer at, not *whether* to.

## Notes

- The review is not optional here in practice: the classification is the artifact
  the successor grove's composer and golden snapshots are built on, and a
  misclassification that survives is baked in behind bytes that look stable. Give
  the reviewer the units each batch was least sure about, by id — carry those
  doubts forward from child to child so the final batch can assemble them rather
  than reconstruct them.
- The single in-session reviewer a picked producer may spend is for one narrow,
  unexpected doubt — not for a second opinion on a batch. That is what the
  aggregate `review-impl` leaf is for.
- Verification here is structural by design; behavioural evaluation was
  considered and rejected during grilling. The honest behavioural check is the
  first real grove run after the successor grove ships, with a human watching —
  which is also why this classification wants to be *released and auditable*
  before a composer is written over it.
- If subdividing turns up prose that is neither a condition nor a procedure —
  narrative that exists only to make the document readable — say so rather than
  forcing it into a class. That is a finding about the design, and it belongs in
  a leaf, not in a marker.
- `content/SKILL.md` and `content/TASK-FORMAT.md` are edited by
  `step-suffix-redundancy-k10` and whatever it cuts, both of which run **before**
  this node. Classify the prose as it then stands; if either file still has
  uncommitted suffix work outstanding when a batch reaches it, that is a
  sequencing fault worth stopping on rather than classifying around.
