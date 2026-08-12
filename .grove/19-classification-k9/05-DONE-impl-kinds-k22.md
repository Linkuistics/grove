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

## Batch record

**Anchors executed against**, both verified unique (`grep -Fc` returned 1 for
each): `## Composing the kinds — the two shapes` at L193 and `## The three design
kinds — extra guidance` at L473. **The baseline ranges agreed with the anchors
exactly** — L1–192 = 11,596 B and L473–501 = 1,593 B, summing to the 13,189 B this
leaf records, and the middle L193–472 = 15,904 B matching `shapes-k23`'s figure.
11,596 + 15,904 + 1,593 = 29,093 = the whole file: this file has **no `---`
preamble**, so unlike `SKILL.md` there is no unassigned residue at all. Batches
after this one still cannot assume their ranges agree.

### The units, in document order

**26 real units and one residual.** Byte counts are post-marking, fetched
through `grove-llm methodology <id>` rather than measured by hand, so each is the
exact slice a mandate would carry (marker line included).

| id | class | scope | bytes |
|---|---|---|---|
| `task-leaf-filename` | triggering | `*` | 1,011 |
| `task-kind-in-the-filename` | triggering | `*` | 636 |
| `task-name-reading-is-strict` | **procedural** | — | 495 |
| `task-nineteen-kinds` | triggering | `*` | 1,674 |
| `task-hitl-afk` | triggering | `*` | 658 |
| `task-producer-requirements` | triggering | `requirements` | 377 |
| `task-producer-design` | triggering | `design` | 331 |
| `task-producer-planning` | triggering | `planning` | 897 |
| `task-producer-prototype` | triggering | `prototype` | 240 |
| `task-producer-impl` | triggering | `impl` | 362 |
| `task-research-pair` | triggering | `research-a research-b` | 626 |
| `task-combine-research` | triggering | `combine-research` | 427 |
| `task-research-write-paths` | **procedural** | — | 623 |
| `task-finish-session` | triggering | `finish` | 390 |
| `task-review-kinds` | triggering | the five `review-*` | 1,192 |
| `task-integrate-review-kinds` | triggering | the five `integrate-review-*` | 1,056 |
| `task-in-session-doubt-budget` | triggering | `*` | 1,219 |
| `task-too-big-is-planning` | triggering | `*` | 202 |
| `task-decompose-inherits-kind` | **procedural** | — | 369 |
| `task-work-is-not-a-kind` | **procedural** | — | 440 |
| `pending-task-shapes` | triggering | `*` | 15,964 |
| `task-three-design-kinds` | triggering | `*` | 232 |
| `task-deliverable-requirements` | triggering | `requirements` | 258 |
| `task-deliverable-design` | triggering | `design` | 318 |
| `task-deliverable-planning` | triggering | `planning` | 188 |
| `task-deliverable-split-not-a-gate` | triggering | `*` | 433 |
| `task-bootstrap-leaf-is-requirements` | triggering | `requirements` | 612 |

**Coverage, verified rather than assumed:** the 27 slices sum to **31,230 bytes =
the whole marked file**, so the partition is total with nothing unassigned.
Independently, by line arithmetic: 192 + 19 markers = L211 (region A), + 280 + 1 =
L492 (residual), + 29 + 6 = L527 = the file. `pending-task-shapes` is 15,904
baseline bytes + its own 60-byte marker, carries `class=triggering kinds=*` and
**no `defers=`**, and covers the middle region and nothing else.

### The ids five later batches must reach back into

Required by this leaf's *Done when*. Every one is locatable by `grep -n '<!-- unit:
<id>'` — no line number needed.

| batch | row(s) | reaches into | which is |
|---|---|---|---|
| #4 `guides-k24` | 1 | `task-producer-requirements` | the `requirements` producer bullet, `kinds=requirements` |
| #4 | 2, 3 | `task-deliverable-design` | the *three design kinds* `design` bullet, `kinds=design` |
| #4 | 4 | `task-deliverable-requirements` | the *three design kinds* `requirements` bullet, `kinds=requirements` |
| #8 `decompose-moves-k28` | 16 | `task-deliverable-planning` | the *three design kinds* `planning` bullet, `kinds=planning` |
| #7 `doubt-moves-k27` | 11 | `task-in-session-doubt-budget` | the **family-A owner**, `kinds=*` |
| #9 `execute-k29` | 17 | `task-in-session-doubt-budget` | the same owner — two inbound edges, both planned |

The remaining three ids this leaf's *Done when* names, listed for completeness
even though no inventory row targets them — they are the bullets a later batch
would reach for if it wanted an extra address into a per-kind discipline, and #9
needs them by name to **decline row 24** (see below): `task-producer-design`,
`task-producer-planning`, `task-producer-impl`. All three are
`class=triggering`, kind-scoped, and therefore illegal `defers=` targets.

**A warning for #4, #7 and #8, because it is the one way these edges can go
silently wrong.** Four of those six sources are **kind-scoped, not `kinds=*`**. A
procedural body rooted *only* from one of them is reachable from that one kind's
mandate and from no other — correct by the plan's own "reachability is per kind",
but only because the plan gives each of those bodies a `kinds=*` root as well
(rows 20–21 root `ADR-FORMAT.md` and `SPEC-FORMAT.md` bodies from #9's fused
C/D/E owner; row 19 roots `BRIEF-FORMAT.md` bodies from #9's `**Decompose.**`
owner; row 29 roots `CONTEXT-FORMAT.md` bodies from #12). The build cannot tell
the difference between a body honestly reachable from one kind and a body whose
`kinds=*` root was forgotten. **If you write my scoped root and #9's `kinds=*`
root never lands, the narrowing is silent.** Only `task-in-session-doubt-budget`
is `kinds=*` and needs no backstop.

### Edge inventory rows owned: none — reported as planned

**Zero cross-file `defers=` written**, exactly as this leaf's body predicted. Both
sweeps run and recorded:

- **Outbound** — over my two regions: six hits, and they are precisely the five
  files the leaf body enumerated (`grilling.md` twice, `driving.md`,
  `ADR-FORMAT.md`, `SPEC-FORMAT.md`, `BRIEF-FORMAT.md`). Every target is still a
  single trivial `class=triggering kinds=*` unit, so no edge is **writable** yet —
  a `defers=` naming a triggering unit is a build error — and under the
  later-carved-endpoint rule none of them is mine in any case.
- **Inbound** — `grep -rn 'TASK-FORMAT\.md' content/`: eight hits, and **only one
  points into my region**. `SKILL.md` L183 (*"the driver-reserved `finish`
  (`TASK-FORMAT.md` for every kind's discipline and its HITL/AFK mark)"*) is
  **inventory row 24**, owned by #9. Five hits (`BRIEF-FORMAT.md` L23,
  `driving.md` L141/L156/L533, `SKILL.md` L335/L339) target the **residual**
  region — §*The vendor pair*, §*What the shapes are not*, the bare-stem
  reasoning — so they are `shapes-k23`'s targets, not mine, and belong to rows 26,
  27 and 35. `SKILL.md` L750 is the `## Reference files` index, the plan's standing
  sweep exclusion.

**Row 24 should be declined by #9, and here is the reason to quote.** The row is
conditional on *"if #2 made any procedural"* per-kind discipline body. **It did
not**: all ten per-kind units (`task-producer-*`, `task-research-pair`,
`task-combine-research`, `task-finish-session`, `task-review-kinds`,
`task-integrate-review-kinds`) are `class=triggering`, kind-scoped. A `defers=`
from `SKILL.md`'s `**Execute.**` naming any of them is a build error. My four
procedural units are not per-kind discipline and are each already honestly rooted
in-file. So row 24 is **declined, not missing** — the plan's own expected outcome.

**One inventory addition, for #5, #7 and #8.** `task-producer-impl` names three
`driving.md` habits in its parenthesis — cite-to-source, doubt-before-it-stands,
externalize-rather-than-absorb — which are bodies in #5, #7 and #8 respectively.
Under the later-endpoint rule those batches own the edge if it is one. **Weigh it
against the `kinds=impl` warning above**: the plan already roots all three from
`kinds=*` conditions (row 11 from my family-A owner, rows 12/18 from #5's framing
and #9's `**Decompose.**`), so an edge from my `impl`-scoped bullet is an extra
*address* at best and never the load-bearing root. The plan does not list it, and
declining it costs nothing.

**Zero `defers=` chains cross a file boundary**, so (D), (R) and (T) hold
in-file: four procedural units, each reached from a triggering unit in this same
region — `task-name-reading-is-strict` from `task-kind-in-the-filename`,
`task-research-write-paths` from **both** `task-research-pair` and
`task-combine-research`, `task-decompose-inherits-kind` from
`task-too-big-is-planning`, `task-work-is-not-a-kind` from `task-nineteen-kinds`.
Every chain is one hop, so termination is trivial.

### The six pre-decided calls, applied not re-decided

All six sites landed as the node brief settled them.

- **Family A owner** — `task-in-session-doubt-budget`, `class=triggering kinds=*`,
  **predicate and allowance table in one unit** (L164–177 baseline, including the
  closing *"Outside that Bootstrap-and-mandate predicate…"* paragraph, which is the
  predicate's negative complement and the N-fresh-contexts-spends-N counting rule).
  Not split. No `defers=` — rows 11 and 17 are #7's and #9's to add, and the
  decoupling lemma makes that a one-line edit that moves no prose and no id.
- **Family B second condition** — `task-producer-design` carries the drift clause
  (*a `design` session cutting implementation leaves has drifted into planning's
  job*) and ships on its own account with no edge to #9's owner.
- **Family B mention** — the `impl` bullet's parenthesis rides inside
  `task-producer-impl` and takes its class. Not a site decided here.
- **Families D and E second condition** — `task-deliverable-design`,
  `kinds=design`, the honest explicit scope the plan flagged, and the planned
  inbound root for rows 2–3.
- **Family C mentions** — the `review-design` question and the
  `integrate-review-design` in-place discipline ride inside `task-review-kinds`
  and `task-integrate-review-kinds`. No units, no edges.

### The judgement this batch exists for — scope, per bullet

The whole region is `kinds=` territory, and the call came out the same way for
every per-kind bullet: **triggering, scoped to that kind.** The reasoning is one
argument, and it is the argument the reviewer should attack if any of this is
wrong.

A kind's discipline has two consumers: the session *executing* that kind, and a
session *choosing* a kind when it cuts a leaf. The second is real and frequent —
every producer decides whether to cut `review-X`; every `planning` session picks
kinds. If the per-kind bullets were the only statement of what each kind is
**for**, narrow scoping would withhold exactly the thing a chooser needs, and
that failure is silent.

**They are not the only statement**, and that is what makes narrowing safe:
`task-nineteen-kinds` is `kinds=*` and carries the full kind table plus the
parameterised shape, and `SKILL.md`'s `**Execute.**` (#9) names all nineteen and
glosses the branch. So the **gloss** ships everywhere and the **discipline** is
scoped. Per bullet:

- `task-producer-requirements` → `kinds=requirements`. **The node brief fixes this
  one** (*Which files can root themselves* names it as `grilling.md`'s root, "(`kinds=requirements`)"),
  and its content is the grilling procedure — the executing session's discipline.
- `task-producer-design` → `kinds=design`. The drift detector is only actionable
  by a `design` session; the deliverable statement is restated at `kinds=*` by
  `task-deliverable-split-not-a-gate`.
- `task-producer-planning` → `kinds=planning`. The leaf body called it "plainly
  triggering for `planning`" and the content agrees: increments, dependency order,
  vertical slices, *the deliverable is more tree*.
- `task-producer-prototype` → `kinds=prototype`. Pure discipline, 240 bytes.
- `task-producer-impl` → `kinds=impl`. Deliverable plus the `driving.md` pointer.
- `task-research-pair` → `kinds="research-a research-b"`; `task-combine-research` →
  `kinds=combine-research`. Split because the scope differs and because the
  combiner alone carries the adversarial move (*agreement without independent
  primary sourcing is a red flag*).
- `task-review-kinds` / `task-integrate-review-kinds` → the five members each,
  **spelled out in full**: no family shorthand and no negation exist, which is
  why these two markers are the longest in the corpus. Whether review is
  *required* is `kinds=*` guidance and lives in `SKILL.md` *Cut the next step*
  (#10) and §*Composing the kinds* (#3); what a review **does** is the
  reviewer's.
- `task-finish-session` → `kinds=finish`. "No session creates one" is already
  stated at `kinds=*` by `task-nineteen-kinds` (the grow verbs refuse it), so
  nothing load-bearing is withheld from the other eighteen.

**Four units in the region stayed `kinds=*` deliberately**, and they are the
backstop the narrowing leans on: `task-leaf-filename`,
`task-kind-in-the-filename`, `task-nineteen-kinds`, `task-hitl-afk` — plus
`task-in-session-doubt-budget`, `task-too-big-is-planning`,
`task-three-design-kinds` and `task-deliverable-split-not-a-gate`.

### Four procedural units, and why each root is honest

The plan warns that reachability is satisfiable by any inbound edge, so each root
is stated with the condition it answers:

- **`task-name-reading-is-strict`** ← `task-kind-in-the-filename`. The condition
  says the kind lives in the filename and the handle stays `<slug>-k<key>`; this
  body answers *how that name is then parsed* — "no kind label plus `-` prefixes
  another, so a name always separates unambiguously and round-trips without
  touching the slug". That is the same subject, one grain down. It is also
  mechanism a session is **told** about when it needs it: a malformed name stops a
  tree verb with the path and the valid set, so the lookup is one the session
  knows to make. Rooting it from `task-leaf-filename` was considered and rejected
  as looser — that unit's teeth are the pruning-HITL clause, which this body does
  not answer.
- **`task-research-write-paths`** ← `task-research-pair` **and**
  `task-combine-research`. Two inbound edges, both honest and both necessary: the
  body covers all three kinds' write paths, and rooting it from the pair alone
  would leave it unreachable from the `combine-research` mandate — the exact
  silent narrowing warned about above, inside one file.
- **`task-decompose-inherits-kind`** ← `task-too-big-is-planning`. The condition
  is *a task too big for one focused session is a planning task*; this body is
  what `leaf-decompose` then does with the kind. A textbook if/then pair, and the
  reason `task-too-big-is-planning` is carved as its own 202-byte unit rather than
  absorbed above.
- **`task-work-is-not-a-kind`** ← `task-nineteen-kinds`. The condition is that the
  set is closed and "not a free-text label a leaf may coin"; this body is the one
  label that used to be in the set and is not. Rooting it from
  `task-name-reading-is-strict` would also have been honest but makes a two-hop
  chain on a procedural first hop for no gain.

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

Generous, as this leaf asked, because the scope calls fail in a direction no
build sees.

1. **`task-in-session-doubt-budget` — the residue this leaf was told to record,
   not fix.** The owner states its predicate **positively** ("once the current
   session has run Bootstrap and adopted the driver's selected-leaf mandate") and
   **never states the negative half** — that merely finding `.grove/` in the
   checkout, or inheriting Grove control variables, does **not** activate it.
   Both bodies do state it (`driving.md` §*Doubting inside a picked Grove leaf*,
   #7; `SKILL.md` *Review ownership inside a picked leaf*, #9). So the corpus-wide
   condition every mandate carries is the *only* statement of the rule that omits
   the false-positive guard, and the guard is the half that stops a session
   claiming the allowance it was never given. Whether the predicate ships
   completely enough is the reviewer's call, not this batch's. **Fixing it is a
   prose edit**, so it cannot be done by a marking pass at all.
2. **The ten kind-scoped units, as one decision.** The argument above is sound
   only if the `kinds=*` gloss really is sufficient for a chooser. If the reviewer
   thinks a `planning` session cutting leaves needs the *full* discipline of the
   kind it is choosing, the fix is to flip some or all of these to `kinds=*` —
   each a one-marker edit with no prose consequence. The two most exposed:
   **`task-producer-design`** (whose drift clause is arguably what a *planning*
   session needs, to recognise it has handed a design leaf implementation work)
   and **`task-review-kinds`** (whose five reads tell a producer what it is buying
   when it cuts `review-X`).
3. **`task-name-reading-is-strict`** — the only unit here whose **class** was
   genuinely close. It is defensible as `kinds=*` triggering: "foreign non-task
   files in the tree stay ignored" is a permission, and the strictness is a
   standing fact about the tree. It went procedural on the asymmetry as applied to
   *arrival*: the session that needs it is told by a verb diagnostic naming the
   path and the valid set, so withholding it costs a lookup the session cannot
   fail to know it needs. One marker to flip.
4. **`task-bootstrap-leaf-is-requirements`** (`kinds=requirements`, 612 B) — the
   same shape as `spine-k21`'s declined narrowing on `skill-starting-a-new-grove`,
   and **decided the other way**, which the reviewer should look at as a pair. The
   difference relied on: that unit's subject is *how to start a grove* (needed by
   any session that concludes a stage deserves its own grove), whereas this one's
   subject is *the bootstrap leaf's own fusion of the three design kinds* and its
   operative half ("may go on to cut the leaves itself; a larger one adds a
   `planning` leaf") is addressed to that session. If the reviewer reads the two as
   the same site, this one should follow `spine-k21` and become `kinds=*`.
5. **`task-nineteen-kinds`** (1,674 B, `kinds=*`) — the kind table and the
   "closed set" paragraph were **weighed for a split** as this leaf's *judgement*
   section invited ("the table itself may be its procedural body") and kept as one
   triggering unit. Two reasons: the annotation paragraph after the table opens
   *"Five producer rows of three, the research row's three, and one driver-owned
   step"*, so a triggering annotation over a deferred table states a shape whose
   table the session does not have — violating "a unit must read correctly standing
   alone" — and that paragraph carries a real condition (`finish` is
   driver-reserved; `leaf-insert` may target it only to put ordinary work *before*
   teardown). Table-and-annotation are therefore one unit, and its class is set by
   the condition. Under the inherited convention's rule 3 that is also the right
   grain: nothing splits without a class or scope change.

### Design findings

**F4 — `**Producers**` is a section header with no rule of its own, and it lands
in the preceding unit.** The inherited convention (`spine-k21` rule 2: an un-led
paragraph "joins the unit above it" unless it states a rule that stands alone) puts
it at the tail of `task-hitl-afk`, which is `kinds=*`, so it ships everywhere. The
visible residue: an `integrate-review-impl` mandate carries "**Producers**"
immediately before the `integrate-review-*` bullet, since the five producer bullets
it introduces are scoped away. Harmless, unfixable by marking (the header is its own
line but has no content to classify), and worth one sentence to the reviewer because
it is the first place the classification makes a *composed mandate* read slightly
oddly. `**Research**` (L103–104) is **not** the same case — it states a rule ("a
vendor pair, not a review chain") so it heads `task-research-pair` rather than
joining `task-producer-impl`, which would have shipped it to `impl` alone.

**F5 — the `## Composing the kinds` heading now sits under a `pending-` id, and
that is the one intermediate state a reader can misread.** `pending-task-shapes` is
15,964 bytes of `class=triggering kinds=*` covering both shapes in full. Same shape
as `spine-k21`'s F3 and recorded for the same reason: it is a coverage placeholder,
not a classification, and `shapes-k23` consumes it next.

**F6 — this region confirms `spine-k21`'s F1 rather than adding to it.** No unit
here was forced coarser by a mid-line boundary; every boundary this classification
wanted was available at line granularity. The fusion problem is real but is
concentrated in `SKILL.md` (F1, and the node brief's L217–227), not here. The one
place it nearly bit is `task-nineteen-kinds` (doubt 5), and there the obstacle was
**readability standing alone**, not line granularity — a different and weaker
constraint, and one a prose edit could not fix either.
