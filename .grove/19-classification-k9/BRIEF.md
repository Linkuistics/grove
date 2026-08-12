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

Whole-file byte counts against the **pre-classification baseline**: `content/` as
it stands at the commit retiring `batches-k34`, which is byte-identical to the
corpus `drop-step-suffix-k20` left (nothing between them touches `content/`).
Whole-file counts do not move under marking; the *region* counts further down are
baseline coordinates and do — see *Coordinates are orientation; anchors are
authoritative*.

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
  is exactly the "unasked question" in miniature. The **edge inventory** below is
  what makes this checkable per batch instead of hoped for.
- **Satisfying reachability with a root that answers nothing.** Any inbound edge
  makes a procedural unit reachable, so an artificial root turns a build check into
  a formality — and once a body has two inbound paths, dropping the *real* one
  leaves the build green. Root a body from a condition it genuinely answers, or
  treat the missing root as a sequencing fact.
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

## The batching contract — settled by `batches-k13`, repaired by `batches-k34`

`batches-k13` mapped the closure with the files open and cut the twelve batch
children below. `batches-k33` found seven defects in that contract and
`batches-k34` repaired them: boundaries are now semantic rather than positional,
every cross-file edge has exactly one named owner, the repeated-rule calls are
pre-decided here instead of delegated across twelve sessions, and the greenness
lemma is narrowed to what it actually proves. What follows is what all twelve
children share; each child's own body carries its region, the edges it owns, and
its doubts. **Read this section before writing a single marker** — it is what
makes a batch green without a trial build.

### Coordinates are orientation; anchors are authoritative

Every line number and every region byte count in this brief and in the twelve
child bodies is a **pre-classification baseline coordinate**, measured against
`content/` at the commit retiring `batches-k34`. They are for orientation and
sizing only, and they are **stale by construction at execution time**: marking
inserts marker lines, so batch #1 moves every later line in `SKILL.md` before
batch #9 opens it.

What a batch executes against, in this order of authority:

1. **The `pending-*` unit it consumes.** That is a *unit*, so it moves with its
   prose and cannot go stale. It is the region's real definition.
2. **The semantic anchor** naming where its region ends and the next begins — the
   heading or bold-lead line quoted in its body (`**Execute.**`,
   `## Composing the kinds — the two shapes`, and so on). Find them with
   `grep -Fn`. **All nine anchors were verified unique in their own file** by
   `batches-k34` (`grep -Fc` returns 1 for each, mermaid fence included), so an
   anchor match is unambiguous and needs no line number to disambiguate.
3. **Baseline line ranges and byte counts** — orientation only. A batch that finds
   a baseline range disagreeing with its anchor follows the **anchor** and says so
   in its body.

**The marker-placement convention**, because it decides which unit owns a blank
separator line: a marker line goes **immediately above the first prose line of its
unit**, so the blank line above the marker belongs to the *preceding* unit. That
makes separator ownership mechanical rather than a judgement, and it is what the
coverage arithmetic below assumes.

### The greenness lemma, narrowed — and the local obligations it does not cover

An un-carved region stays covered by a `class=triggering kinds=*` unit. A
triggering unit carries **no** reachability obligation (it ships in every mandate
its scope admits) and `defers=` is optional on it. So **the un-carved residue can
never fail the gate**, and the *deferral graph* stays consistent as long as each
batch satisfies, at end of its own batch:

- **(D)** every `defers=` the batch writes names a unit that exists and is
  `class=procedural`;
- **(R)** every `class=procedural` unit the batch creates is reachable by
  following `defers=` from a triggering unit;
- **(T)** no `defers=` chain the batch creates returns to a unit already passed
  through.

**(D), (R) and (T) are the complete set of *cross-unit deferral-graph*
obligations, and nothing more.** They are not the complete set of obligations a
subdivision creates. Subdividing *adds markers and ids*, so every batch also owns
these **local** rules, each of which `cargo build` catches on that batch's own
commit (`docs/specs/mandate-delivered-methodology.md`, *A malformed embed fails
the build*):

- the marker parses, with attributes in the fixed order `id`, `kinds`, `class`,
  `defers`, and no unknown attribute;
- `class` is present; `kinds=` is **required** on triggering and **forbidden** on
  procedural;
- every `kinds=` member is one of the nineteen session kinds; the scope is `*` or
  a quoted space-separated list, with no family shorthand and no negation;
- every id is kebab-case and **unique across the whole embed** — held by the
  file-scoped prefix convention, which is why no two batches can collide;
- the marker is an unindented whole line at **neutral fence state**: not indented,
  not inside a fence, not inside the leading `---` block;
- the file still declares at least one unit, and no body text precedes its first
  marker.

What genuinely *is* preserved by construction, because a batch inserts marker
lines and edits no prose, no filename and no fence: the trailing newline, fence
balance at end of file, the absence of control characters in embedded paths, and
`content/SKILL.md`'s leading `---` block. Do not extend that list to the rules
above; the plan once did, and it was wrong.

### The decoupling lemma

A later batch may add a `defers=` member to a marker an earlier batch wrote,
**without changing that unit's id, its class or its boundaries** — a marker line
is one line, and adding a member to it moves no bytes of prose and no id in
`EMBEDDED_UNITS`. So a condition and its body need not be carved together.

Two things the lemma does **not** license, both of which the repair forbids
outright: it does not license changing an earlier batch's `class` (that would make
every batch's classification provisional), and it does not license parking an edge
on a residual for a later batch to move.

### Edge ownership — one edge, one owner, computable from the batch table

**The later-carved endpoint's batch owns the edge.** For an edge from triggering
site `S` to procedural body `T`, the owning batch is whichever of `S`'s batch and
`T`'s batch appears later in the order below; if both are in one batch, that batch
owns it. That is the whole rule, and it makes ownership a fact about the table
rather than a hand-off between sessions.

Two prohibitions make it total:

- **No edge may have a `pending-*` source.** A residual is not a unit anyone
  reasoned about, so a `defers=` on it asserts an edge from prose that may not
  contain the condition — and a member parked there can be silently dropped later
  while the target stays reachable by another path, with no build failure and no
  diff. If the source region is still pending, the edge is simply **not yet
  writable**, and the batch that carves that region writes it.
- **No procedural unit is rooted from a unit that does not state a condition it
  answers.** Reachability is satisfiable by any inbound edge, which makes it easy
  to satisfy dishonestly; an artificial root turns a checked property into a
  formality. Where a body has no honest root yet, that is a **sequencing fact**,
  and the order below is built to respect it (see *Which files can root
  themselves* and the *edge inventory*).

Multiple inbound edges into one body are **legal and often correct** — `defers=`
is the addressing device as much as the reachability device, so two genuine
conditions sharing a body must both name it, or the second condition's session has
no address for it. What makes redundancy safe here is that every edge is a
**listed obligation** in the inventory below, so a dropped edge is visible against
the plan even when the build stays green.

Each batch runs **two sweeps**, and neither is claimed complete on its own:

```
grep -rn '<F>' content/          # for every file F this batch carves
```

- **Inbound** — for every hit outside `F` referring to a body this batch created,
  decide whether the reference is a genuine trigger→body edge, and where it is and
  the source is already carved, add the `defers=`.
- **Outbound** — for every cross-file reference *inside* this batch's own region
  pointing at a body an earlier batch already carved, write the `defers=`.

**The filename grep is evidence, not completeness.** A cross-file relationship the
prose expresses *without naming a file* is invisible to it. The corpus's one live
example is `content/prompts/continue.md`, which says *"see the skill's Decompose
step"* without spelling `SKILL.md` — and it is instructive in **both** directions:
the grep cannot see it, *and* it turns out to owe no edge, because its target is a
condition rather than a body (see the note under the inventory). A sweep that cannot
see a reference cannot classify it either way, which is why the net is the **edge
inventory** below: each batch reconciles the inventory rows it owns *in addition to*
running the sweeps, and records the outcome row by row.

One caution survives unchanged: **a prose cross-reference is not automatically a
`defers=`.** `driving.md` names `SKILL.md` eleven times and most of those point at
a *constraint* or a *condition* — a `class=triggering` unit, which is an illegal
`defers=` target. The build catches it; do not spend a cycle discovering that.

### The `pending-` convention — a coverage placeholder and nothing else

Every region not yet carved is covered by a unit whose id begins **`pending-`**,
`class=triggering kinds=*`, **with no `defers=`, ever**. It is pinned in
`EMBEDDED_UNITS` like any other unit, and it carries no obligation and no
information beyond "these bytes are not classified yet". There is no
redistribution protocol, because nothing is ever parked on a residual.

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
| `prompts/continue.md` | it is the framing; all triggering | needs no root itself, and **is** the root for `SKILL.md` §*Reference files* (below) |

Reachability is **per kind**, not universal: a procedure reached only from a
`kinds=requirements` condition is reachable from the `requirements` mandate and
from no other, and that is correct, not a gap.

`driving.md`'s **framing unit** (its opening plus `## In this guide`, carved by
batch #5) is the file's designated self-root: `## In this guide` indexes every
section of the file by name, and the opening names externalizing, doubting and
source-grounding outright, so it is an honest root for any `driving.md` section
whose semantic owner is carved later. It is used that way twice below and nowhere
else; do not reach for it to paper over a body with no condition.

### The five repeated-rule families, pre-decided here

The corpus states several load-bearing rules two or three times, in files carved by
different batches. Getting one wrong is consequential in **both** directions:
classify the condition on two sides and every mandate carries it twice; classify it
on neither and the mandate carries it nowhere — the silent direction, with no diff.
`batches-k13` delegated four of these calls to a sibling-body hand-off, which let
the *first* batch to reach a family decide it without knowing every site existed.
They are settled here instead, from the whole site list, and each batch applies the
verdict for the sites in its own region rather than making a call.

Four verdicts are used:

- **Owner (triggering)** — the site whose prose states the condition completely,
  and which every session must receive. Exactly one per rule, and it must sit in a
  batch **no later than** every batch creating one of that rule's bodies, because a
  procedural unit must be reachable at the end of its own batch.
- **Body (procedural)** — a site that expands the rule into how to act. Reached by
  `defers=` from the owner or from another genuine condition that addresses it;
  the edge belongs to the later-carved endpoint's batch.
- **Second condition (triggering)** — a site stating a *different trigger* for the
  same subject: another moment in the loop, or one kind's specialization. It ships
  too, and that is not duplication, because the condition differs.
- **Mention (no unit of its own)** — a clause inside a unit whose subject is
  something else: a per-kind bullet, an index row, a table cell, a forward pointer.
  It takes its host unit's class, owes no edge, and is **not a site to decide**.
  Most of the extra sites are this, and saying so is what stops eight batches
  re-litigating the same grep hit.

**A — the in-session reviewer budget.** The condition is *there is a leaf-wide
allowance, this session may already have spent it, and a second need means cutting
a leaf.*

| site | batch | verdict |
|---|---|---|
| `TASK-FORMAT.md` §*In-session doubt is budgeted across the whole picked leaf* (baseline L164–177) | #2 `kinds-k22` | **Owner**, `kinds=*` — predicate plus the allowance table keyed by picked session kind: the most operational statement, and the earliest, so both bodies below are rooted from something that exists |
| `driving.md` §*Doubting inside a picked Grove leaf* (L415–453) | #7 `doubt-moves-k27` | **Body** — the field-guide expansion: what counts, the four-step pass, the per-kind exceptions. Edge owned by #7 |
| `SKILL.md` *Review ownership inside a picked leaf* (L198–215) | #9 `execute-k29` | **Body** — the hub's restatement plus the escalated-review routing note. Edge owned by #9 |
| `SKILL.md` *Cut the next step* — "may use its single in-session reviewer instead (`driving.md`)" (L266–268) | #10 | Mention |
| `driving.md` §*The review chain* — "do not add a competing doubt reviewer beside" (L567) | #7 | Mention |

*Known residue, for the aggregate review:* the owner states the predicate
positively ("once the current session has run Bootstrap and adopted the driver's
selected-leaf mandate") but not its **negative half** — that merely finding
`.grove/` or inheriting Grove control variables does not activate it — which both
bodies do state. #2 records this; the reviewer decides whether the predicate ships
completely enough.

**B — externalize rather than absorb** (grove's primary failure mode).

| site | batch | verdict |
|---|---|---|
| `SKILL.md` `**Decompose.**` (L229–245) | #9 `execute-k29` | **Owner**, `kinds=*`. The corpus itself designates it: `driving.md` L595 reads *"`SKILL.md`'s Decompose step states the rule; this is the habit that honours it"* |
| `driving.md` §*Externalizing surfaced work* (L587–613) | #8 `decompose-moves-k28` | **Body** — carved *before* its owner, so #8 roots it from `driving.md`'s framing unit (#5), which names externalizing outright; #9 then adds the owner's address. Two inbound edges, both honest, both inventory rows |
| `TASK-FORMAT.md` `design` bullet — *a `design` session cutting implementation leaves has drifted into planning's job* (L80–83) | #2 | **Second condition** — a kind-scoped drift detector, not the general rule |
| `TASK-FORMAT.md` `impl` bullet parenthesis (L99–101) | #2 | Mention |
| `driving.md` framing and `## In this guide` (L10–11, L32) | #5 | Mention |
| `prompts/continue.md` (L3–6) | #12 | **Second condition** — the launcher's framing statement of the rule. Its "see the skill's Decompose step" points at the **owner**, a triggering unit, so **no edge is legal and none is needed**: the condition already ships in every mandate |

**C — the ADR set is current-state: rework in place, never supersede.**

| site | batch | verdict |
|---|---|---|
| `SKILL.md` *Whichever kind is running…* (L217–227) | #9 `execute-k29` | **Owner**, `kinds=*` — and see the fusion note below |
| `driving.md` §*Reworking ADRs and briefs as understanding shifts* (L285–310) | #6 `evidence-moves-k26` | **Body** — carved before its owner, so #6 roots it from `driving.md` §*When to retire research into ADRs…* in the **same batch**, which the corpus itself points there (*"see *Reworking ADRs and briefs* below"*); #9 adds the owner's address |
| `driving.md` §*When to retire research into ADRs versus leave it in docs/research/* (L264–283) | #6 | **Second condition** — a different trigger (a research finding becoming binding). Its own "you are editing the ADR **in place**" clause rides along as a mention |
| `ADR-FORMAT.md` §*Why the set stays minimal* (L32–40) | #4 `guides-k24` | **Body** — rooted from `grilling.md` §*Offer ADRs sparingly* and `TASK-FORMAT.md`'s `design` bullet, both already planned; #9 adds the owner's address |
| `SKILL.md` retirement reconciliation (L550–554) | #11 `lifecycle-k31` | Mention — a clause inside the node-close cascade's procedural prose, and unsplittable from it at line granularity. The owner already names retirement as a checkpoint |
| `TASK-FORMAT.md` L144 (`review-design` question), L157–158 (`integrate-review-design` in-place discipline) | #2 | Mentions |
| `SKILL.md` `## Artifacts` ADR row (L703) | #12 | Mention |

**D — a spec is written at an agreement point; the set is current-state, with the
membership and grain tests.**

| site | batch | verdict |
|---|---|---|
| `SKILL.md` *Whichever kind is running…* (L217–227) | #9 | **Owner** (shared with C), `kinds=*` — *"write a spec only at a genuine agreement point"* and *"the same rule governs `docs/specs/`, one grain coarser"* |
| `TASK-FORMAT.md` `design` bullet in *The three design kinds* (L481–484) | #2 | **Second condition** — the `design` kind's own deliverable statement, and a genuine `kinds=design` candidate. It is also the planned inbound root for both format guides |
| `SPEC-FORMAT.md` opening (L6–15) | #4 | **Second condition** — *"Most increments write no spec at all"* is a condition with teeth, and it is how `SPEC-FORMAT.md` self-roots |
| `SPEC-FORMAT.md` §*The set is current-state*, the membership test, the grain rule (L17–36) | #4 | **Body** — rooted from `SPEC-FORMAT.md`'s own opening in the same batch; #2's `design` bullet and #9's owner add addresses |
| `SKILL.md` `## Specs` (L720–733) | #12 `finish-cycle-k32` | **Body** — a terser restatement, addressed by the owner alongside `SPEC-FORMAT.md`'s body. Edge owned by #12 |
| `BRIEF-FORMAT.md` L76, `SKILL.md` L704 and L741 | #8, #12 | Mentions |

**E — raise ADRs sparingly (the when-to-write test).**

| site | batch | verdict |
|---|---|---|
| `SKILL.md` *Whichever kind is running…* (L217–227) | #9 | **Owner** (shared with C and D), `kinds=*` — the *only* `kinds=*` statement of it |
| `TASK-FORMAT.md` `design` bullet (L481–482) | #2 | **Second condition**, `kinds=design` candidate |
| `grilling.md` §*Offer ADRs sparingly* | #4 | **Body** — already the `ADR-FORMAT.md` root chain |
| `ADR-FORMAT.md` head (the redirect to `linkuistics:decision-records`) | #4 | **Body** — a redirect, not a condition, exactly as `guides-k24` already reads it |

**F — the two shapes are built in opposite ways** (a chain lazily, a pair eagerly).
`batches-k33` did not list this one, but `shape-cutting-k30`'s body already flagged
it as an overlap, and it has the same three-file shape and the same failure
directions as A–E, so it is settled here too.

| site | batch | verdict |
|---|---|---|
| `TASK-FORMAT.md` §*Composing the kinds — the two shapes* opening (L193–212) | #3 `shapes-k23` | **Owner**, `kinds=*` — *"reach for them by default, and argue yourself out of one rather than into it"* plus *"they are built in opposite ways"*, stated completely in twenty lines, and the earliest of the three sites |
| `TASK-FORMAT.md` §*The review chain — each session cuts the next step*, §*The vendor pair — one eager call* (L214–~330) | #3 | **Bodies** — the mechanics: the `leaf-add` invocations, who cuts what and when, the integration placement rule. Rooted from the owner in the same batch |
| `driving.md` §*The review chain — when doubt earns its own leaves* (L455–586) | #7 `doubt-moves-k27` | **Body** — the field-guide expansion. Edge owned by #7 (row 33) |
| `SKILL.md` `**Cut the next step, when it is needed.**` (L247–268) | #10 `shape-cutting-k30` | **Body** for its restatement of the asymmetry. Edge owned by #10 (row 32). What is *not* a restatement — *when* to decide for review, and the in-session-reviewer pointer — is #10's own judgement and is not governed by this call |

**The fusion note, and it is a finding.** `SKILL.md` L217–227 states **four**
rules — raise ADRs sparingly, write a spec only at an agreement point, the ADR set
is current-state, and the same rule governs `docs/specs/` — and the sentence
boundaries fall **mid-line** (L220 carries the end of the spec clause and the start
of the ADR-set clause). Markers are whole unindented lines, and this pass edits no
prose, so the paragraph **cannot be split** and takes one class for all four rules.
It is triggering, because two of the four ship nowhere else at `kinds=*` and
withholding them yields an unasked question. Consequence: the paragraph is the
owner of C, D and E at once, which is why the bodies of all three defer from the
same unit. **Record this in #9's body as a design finding**: the grain of the
classification is bounded by line boundaries, and de-fusing this paragraph is a
prose edit for a later grove, not a marking decision.

**A family this inventory does not list.** Apply the default rather than inventing
a call: the site that states the condition **completely and earliest** is the owner,
every later complete statement is a body, everything else is a mention — and record
the family, its sites and your call in your leaf body so the aggregate review sees
it. If no site states it completely, that is a finding, not a call to make alone.

### The edge inventory — the completeness net the filename grep is not

Every cross-file edge the classification must create, with its single owning batch.
A batch reconciles the rows it owns and reports each one; a row that turns out not
to be a genuine trigger→body edge is **declined in the body with a reason**, which
is a different act from silently not writing it.

| # | source (triggering) | target (procedural body) | owner |
|---|---|---|---|
| 1 | `TASK-FORMAT.md` `requirements` bullet (L76–79) | `grilling.md` bodies | #4 |
| 2 | `TASK-FORMAT.md` `design` bullet (L481–484) | `ADR-FORMAT.md` bodies | #4 |
| 3 | `TASK-FORMAT.md` `design` bullet (L481–484) | `SPEC-FORMAT.md` §*current-state* / membership / grain | #4 |
| 4 | `TASK-FORMAT.md` *three design kinds* `requirements` bullet (L478–480) | `grilling.md`, `CONTEXT-FORMAT.md` bodies | #4 |
| 5 | `grilling.md` §*Offer ADRs sparingly* | `ADR-FORMAT.md` bodies | #4 |
| 6 | `grilling.md` §*Update CONTEXT.md inline* | `CONTEXT-FORMAT.md` bodies | #4 |
| 7 | `grilling.md` §*Agree the test seams* | `SPEC-FORMAT.md` bodies | #4 |
| 8 | `SPEC-FORMAT.md` opening (L6–15) | `SPEC-FORMAT.md` §*current-state* / membership / grain | #4 |
| 9 | `driving.md` §*When to retire research into ADRs…* (L264–283) | `driving.md` §*Reworking ADRs and briefs…* | #6 |
| 10 | `driving.md` §*When to retire research into ADRs…* | `ADR-FORMAT.md` bodies | #6 |
| 11 | `TASK-FORMAT.md` §*In-session doubt is budgeted…* (L164–177) | `driving.md` §*Doubting inside a picked Grove leaf* | #7 |
| 12 | `driving.md` framing unit (#5) | `driving.md` §*Externalizing surfaced work* | #8 |
| 13 | `driving.md` framing unit (#5) | `driving.md` §*Anti-patterns*, §*The shortest version* | #8 |
| 14 | `driving.md` §*Recording fog without pre-slicing it* | `BRIEF-FORMAT.md` §*On the horizon* body | #8 |
| 15 | `driving.md` §*What a good child leaf looks like* | `BRIEF-FORMAT.md` bodies | #8 |
| 16 | `TASK-FORMAT.md` `planning` bullet (L485–486) | `BRIEF-FORMAT.md` bodies | #8 |
| 17 | `TASK-FORMAT.md` §*In-session doubt is budgeted…* (L164–177) | `SKILL.md` *Review ownership inside a picked leaf* | #9 |
| 18 | `SKILL.md` `**Decompose.**` (owner, B) | `driving.md` §*Externalizing surfaced work* | #9 |
| 19 | `SKILL.md` `**Decompose.**` | `BRIEF-FORMAT.md` bodies, `driving.md` §*What a good child leaf looks like* | #9 |
| 20 | `SKILL.md` L217–227 (owner, C/D/E) | `ADR-FORMAT.md` bodies | #9 |
| 21 | `SKILL.md` L217–227 | `SPEC-FORMAT.md` §*current-state* / membership / grain | #9 |
| 22 | `SKILL.md` L217–227 | `driving.md` §*Reworking ADRs and briefs…* | #9 |
| 23 | `SKILL.md` L224–227 (*See `driving.md` for the field-guide habits*) | `driving.md` grilling-moves bodies | #9 |
| 24 | `SKILL.md` `**Execute.**` (L167–174) | `TASK-FORMAT.md` per-kind discipline bodies, if #2 made any procedural | #9 |
| 25 | `SKILL.md` `**Execute.**` `requirements` bullet (L176–179) | `grilling.md` bodies | #9 |
| 26 | `SKILL.md` *Cut the next step* (L247–268) | `TASK-FORMAT.md` §*Composing the kinds* bodies | #10 |
| 27 | `SKILL.md` *bare stem* / *grammar is five fields* (L323, L327) | `TASK-FORMAT.md` §*What the shapes are not* bodies | #10 |
| 28 | `SKILL.md` `**Retire.**` node-close (L533) | `BRIEF-FORMAT.md` bodies | #11 |
| 29 | `SKILL.md` `## Artifacts` glossary paragraph (L707–713) | `CONTEXT-FORMAT.md` bodies | #12 |
| 30 | `SKILL.md` L217–227 (owner, D) | `SKILL.md` `## Specs` (L720–733) | #12 |
| 31 | `prompts/continue.md` framing unit | `SKILL.md` `## Reference files` (L735–744) | #12 |
| 32 | `TASK-FORMAT.md` §*Composing the kinds* opening (owner, F) | `SKILL.md` `**Cut the next step…**` (L247–268) | #10 |
| 33 | `TASK-FORMAT.md` §*Composing the kinds* opening (owner, F) | `driving.md` §*The review chain — when doubt earns its own leaves* | #7 |
| 34 | `driving.md` §*When to invoke a design discussion (grilling)* (L182, L189) | `grilling.md` bodies | #5 |
| 35 | `driving.md` §*Running the vendor pair* (L141, L156 citations) | `TASK-FORMAT.md` §*The vendor pair* / §*What the shapes are not* bodies | #5 |
| 36 | `driving.md` §*The review chain…* (its `TASK-FORMAT.md` citations) | `TASK-FORMAT.md` chain-mechanics bodies | #7 |
| 37 | `driving.md` §*Prune, reorder, or file an issue* (L702, its `SKILL.md` citation) | `SKILL.md` `**Retire.**` pruning body | #11 |
| 38 | `TASK-FORMAT.md` `task-producer-impl` (L106–109, `kinds=impl`) | `driving.md` §*Verifying framework decisions against the source* | #6 |
| 39 | `TASK-FORMAT.md` `task-producer-impl` (L106–109) | `driving.md` §*Doubting inside a picked Grove leaf* | #7 |
| 40 | `TASK-FORMAT.md` `task-producer-impl` (L106–109) | `driving.md` §*Externalizing surfaced work* | #8 |
| 41 | `driving-reworking-adrs-and-briefs` (procedural, #6) | `driving.md` §*Doubting inside a picked Grove leaf* — the doubt-pass body | #7 |
| 42 | `driving-cite-framework-decisions-to-the-source` (procedural, #6) | `driving.md` §*Doubting inside a picked Grove leaf* — the doubt-pass body | #7 |

**Rows 41–42 are an addition by `evidence-moves-k26`, and they are the mirror
image of rows 38–40.** Two of #6's bodies end by naming the doubt pass as the
paired procedure — *"when the rework is big enough that you might miss a caller,
run the doubt pass (below) over the reconciled set"* and *"When the decision is
also hard to reverse, this pairs with the doubt pass below"* — so the source is
carved and the target is not, and the later-endpoint rule makes both #7's. They
are **intra-file**, so #7's `grep -rn 'driving\.md' content/` sweep cannot see
them at all: the filename grep is a *cross-file* instrument and these references
name no file. That is the same silent shape as rows 38–40 — §*Doubting* is already
reachable by rows 11 and 39, so dropping either member leaves `cargo build`
green — which is why they are rows here rather than a note in a leaf body. #7
picks the target id (it decides §*Doubting*'s own grain) and may decline either
row as a citation with that reason.

**A note #7 needs and no row carries.** `driving.md`'s addyosmani licence comment
(*"The two sections below are adapted … source-driven-development and
doubt-driven-development"*) attributes a **non-adjacent pair**: §*Verifying
framework decisions against the source* and §*Doubting inside a picked Grove
leaf*, with grove's own §*Verifying a claim about the repo itself* between them.
The comment can only travel with the first, so it is now inside
`driving-when-code-depends-on-a-framework-version` and #7's doubt units will carry
no attribution. Nothing to fix while marking — the fix is a prose edit — but #7
should record it rather than rediscover it, and it is worth the aggregate
reviewer's attention as the first case where mandate delivery separates adapted
prose from its attribution.

**Rows 38–40 are an addition by `research-moves-k25`, and they are one sentence
split three ways.** `task-producer-impl`'s parenthesis — *"(`driving.md` carries
the habits: cite framework decisions to the source, doubt a hard-to-reverse
decision before it stands, and externalize surfaced work into new leaves rather
than absorbing it.)"* — names **three** bodies living in three different batches,
so the later-endpoint rule gives each batch one member of one `defers=` list and
no batch owns the whole edge. They are listed here rather than left to three
independent sweeps because the failure is the silent one: each of the three
sections is reachable by another planned route (rows 11, 12 and #6's own
self-rooting), so **dropping any one member leaves `cargo build` green**. Each
owning batch writes its member and reports the row; a target that turns out
triggering is a decline with that reason, as anywhere else.

`research-moves-k25` could not write any of the three: it owns none of the
targets, and the indirection `guides-k24` used for file-level citations — defer to
the file's **entry** unit and let the chain reach the sections — is unavailable
here, because `driving.md`'s entry (`driving-field-guide`) is `class=triggering`
and a `defers=` naming a triggering unit is a build error.

Rows 24, 26, 27 and 35–37 are **conditional on an earlier batch's own split, or on
the reference turning out to be a citation rather than an edge** — a parenthetical
`(`TASK-FORMAT.md`)` supporting a claim is not a trigger→body relationship. The
owning batch reads the earlier leaf's body, and where the target is triggering or
the reference is a citation it **declines the row with that reason**, which is the
expected outcome rather than a gap. Row 37 is the one edge whose source is carved
*after* its target, so it is written by the source's batch.

**The inventory is extensible, and it is not claimed exhaustive.** A batch that
finds a genuine cross-file trigger→body edge inside its own region pointing at an
already-carved body **owns that edge** (it is the later endpoint), writes it, and
records it as an inventory addition in its leaf body — so #12 assembles the final
list for the aggregate review rather than the reviewer reconstructing it from twelve
diffs.

**`prompts/continue.md`'s Decompose reference is deliberately edgeless.** It is the
one semantic, filename-free cross-file reference in the corpus, and it points at
`SKILL.md` `**Decompose.**` — the family-B **owner**, a triggering unit. A
`defers=` naming a triggering unit is a build error, and none is needed: the
condition ships in every mandate. `batches-k33` read this as a lost edge; the
mechanism it found is real and is why this inventory exists, but this particular
reference owes nothing.

### `## Reference files` is procedural, behind the launcher framing

`batches-k13` left the class of `SKILL.md`'s `## Reference files` index (L735–744)
to the final and most loaded batch, framed as a free choice between eight
unconditional `defers=` members and none. Both options evade the classification
question, so the call is made here.

**It is `class=procedural`, rooted from `prompts/continue.md`'s framing unit
(row 31), and it writes no `defers=` of its own.** The decisive argument is not
size and not redundancy: **the index's rows name files, and a session cannot fetch
a file.** `grove-llm methodology` addresses units by id, so an index of filenames
delivered into a mandate promises navigation the delivery path cannot honour — while
every genuine trigger→body edge for those guides is written at its point of use by
batches #4–#12 anyway. As a procedural unit behind the framing that tells a session
*what it is holding and how the rest is served*, it costs nothing and sits where
its one honest condition is.

The index writes no outbound edges, and its eight filename mentions are a
**standing sweep exclusion**: every batch will hit them in `grep -rn '<F>'
content/` and none of them is a trigger→body edge.

**The `linkuistics` prerequisite note (L746–760) is a separate unit and is
`class=triggering kinds=*`**, with no `defers=`. It states a genuine condition — a
session raising an ADR, sketching a spec's seams, or driving a jj-enabled tree
should consult the matching plugin skill — and its three targets are not embedded,
so none can be a `defers=` target.

**Carry it to the aggregate review as a design finding.** The index is narrative
residue of the provisioned-skill era: it exists so a reader of a skill *directory*
knows what sits beside `SKILL.md`, and mandate delivery replaces that job with
`grove-llm methodology`'s listing. Whether it survives at all is the successor
grove's call, and this classification should say so rather than quietly making it
look load-bearing.

### The batch order, and why each boundary is green

Twelve children, in this order. The **anchors** are what a batch executes against;
the line ranges and byte counts beside them are pre-classification baseline
coordinates, for sizing only. Sizes are the body bytes each session must *judge*,
not merely read.

| # | leaf | region — anchors are authoritative | baseline lines | bytes | green because |
|---|---|---|---|---|---|
| 1 | `spine` | `SKILL.md`: file body start → line before `**Execute.**` | L5–166 | 12,024 | self-contained; writes no cross-file `defers=` at all |
| 2 | `kinds` | `TASK-FORMAT.md`: file body start → line before `## Composing the kinds — the two shapes`; **plus** `## The three design kinds — extra guidance` → EOF | L1–192 + L473–501 | 13,189 | self-rooted; owns the family-A owner, and its outward pointers stay un-deferred until their targets exist |
| 3 | `shapes` | `TASK-FORMAT.md`: `## Composing the kinds — the two shapes` → line before `## The three design kinds — extra guidance` | L193–472 | 15,904 | self-rooted from §*Composing the kinds*; owns the family-F owner; writes no cross-file `defers=` |
| 4 | `guides` | `grilling.md` + `ADR-FORMAT.md` + `SPEC-FORMAT.md` + `CONTEXT-FORMAT.md`, all whole | — | 15,667 | roots `grilling.md` at #2's `requirements` unit, then chains the other three off `grilling.md`'s own move sections and `SPEC-FORMAT.md`'s own opening; owns inventory rows 1–8 |
| 5 | `research-moves` | `driving.md`: file body start → line before `## When to retire research into ADRs versus leave it` | L1–263 | 13,580 | self-rooted from its `## When to …` heads; carves the framing unit later batches root from |
| 6 | `evidence-moves` | `driving.md`: `## When to retire research into ADRs versus leave it` → line before `## Doubting inside a picked Grove leaf` | L264–414 | 8,528 | self-rooted; §*Reworking ADRs…* is rooted from §*When to retire research…* in this same batch; owns rows 9–10 |
| 7 | `doubt-moves` | `driving.md`: `## Doubting inside a picked Grove leaf` → line before `## Externalizing surfaced work` | L415–586 | 11,128 | §*Doubting…* is a **body** rooted from #2's family-A owner and §*The review chain* a **body** rooted from #3's family-F owner; owns rows 11 and 33 |
| 8 | `decompose-moves` | `driving.md`: `## Externalizing surfaced work` → EOF; **plus** `BRIEF-FORMAT.md` whole | L587–754 | 14,076 | §*Externalizing…* is a **body** rooted from #5's framing unit; `BRIEF-FORMAT.md` from `driving.md` sections in this same batch; owns rows 12–16 |
| 9 | `execute` | `SKILL.md`: `**Execute.**` → line before `**Cut the next step, when it is needed.**` | L167–246 | 5,455 | every cross-file target exists after #2–#8; owns the family-B owner, the fused C/D/E owner, and rows 17–25 |
| 10 | `shape-cutting` | `SKILL.md`: `**Cut the next step, when it is needed.**` → line before `**When a picked producer needs fresh review**` | L247–407 | 10,068 | `TASK-FORMAT.md` targets (#3) exist; owns rows 26–27 and 32 |
| 11 | `lifecycle` | `SKILL.md`: `**When a picked producer needs fresh review**` → line before `**Finish.**` | L408–609 | 13,712 | `BRIEF-FORMAT.md` (#8) and `ADR-FORMAT.md` (#4) exist; owns row 28 |
| 12 | `finish-cycle` | `SKILL.md`: `**Finish.**` → EOF; **plus** `prompts/continue.md` whole | L610–760 | 11,621 | everything exists; owns rows 29–31; **final** — consumes the last `pending-` unit |

The hard ordering edges are only these: **#4 after #2**; **#6 after #4**; **#7
after #2 and #3**; **#8 after #2 and #5**; **#9 after #2, #4, #6 and #8**; **#10
after #3**; **#11 after #4 and #8**; **#12 last**. Position order satisfies all of them
with slack. Every one of them is now a consequence of a *listed* edge or a
pre-decided ownership call, rather than of a batch's own reading.

### The coverage arithmetic, and what it proves

The twelve regions sum to **144,952 bytes** against a corpus of **145,233**. The
difference is exactly **281 bytes**: `content/SKILL.md`'s leading `---` block,
which the parser skips uninterpreted and no unit covers by design.

`batches-k13` recorded a 284-byte residue and called it the preamble. It is not:
the preamble is 281 bytes, and the other three are the **one-byte blank separator
lines** at baseline `SKILL.md` L246, L407 and L609, which its region ranges left
unassigned. Under the marker-placement convention above a separator belongs to the
**preceding** unit, so each is assigned to the region before it — which is why #9,
#10 and #11 read L167–246, L247–407 and L408–609 here rather than the ranges the
child bodies first carried, and why their byte counts are one greater than
`batches-k13` recorded.

That correction is not a choice between conventions: `batches-k13` **already applied
this one** at the first `SKILL.md` boundary — baseline L166 is a blank line and it
sits inside `spine`'s L5–166 — and then dropped it at the other three. The four
boundaries are now consistent with each other and with how the parser actually
attributes bytes.

With that correction the arithmetic is a genuine coverage proof: **every non-preamble
byte of the corpus is assigned to exactly one batch.** A region assigned to nobody
would be classified by nobody, caught by no build — it would simply stay inside a
`pending-` unit — and the final batch's zero-`pending-` check would fail twelve
sessions late.

### Three obligations in every batch, restated because they are easy to skip

- **Update `EMBEDDED_UNITS` in `tests/methodology.rs` in the same commit**, naming
  each new id deliberately — that set equality is the design's confirmation point,
  and it fails in both directions, so the `pending-` churn shows up there too.
- **Reconcile the edge-inventory rows you own, row by row, and report each one** —
  written, or declined with a reason. The sweeps are evidence; the inventory is the
  net.
- **Record the units you were least sure about, by id, in your leaf body** before
  retiring, so #12 assembles the aggregate review handoff rather than
  reconstructing it from twelve diffs.

## Done when

- Every embedded markdown file is subdivided into real units; `cargo build` and
  `cargo test` are green, including reachability across the whole embed.
- **No `pending-` unit remains** — `grep -rc '<!-- unit: pending-' content/`
  returns 0. That is the mechanical statement that the subdivision is finished
  rather than merely green (see *The batching contract* above).
- **Every row of the edge inventory is accounted for** — written, or declined in
  the owning batch's body with a reason. Reachability going green is not the same
  claim: a body reached by a second path stays green with a semantic edge missing.
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
- **What the reviewer is asked changed shape with `batches-k34`.** No batch decides a
  repeated-rule family any more, so the reviewer is not reconciling twelve
  independent calls; it is auditing **six pre-decided verdicts**, the **edge
  inventory's** row-by-row outcomes, and the **three design findings** the plan
  already knows about (the `SKILL.md` L217–227 fusion, the `## Reference files`
  index, and whatever prose the batches flag as neither condition nor procedure).
  That is a narrower and more answerable brief than "check twelve sessions agreed
  with each other".
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
