# lifecycle-k31

## Goal

Classify **`content/SKILL.md` from `**When a picked producer needs fresh review**` to
the line before `**Finish.**`** (baseline L408–609, 13,712 bytes): `**When a picked
producer needs fresh review**`, the *tree is a real directory tree* paragraph
(`leaf-decompose`, `leaf-add`, `leaf-insert`), `**`--kind <kind>` appears on the
grow verbs…**`, `**Reading is strict too**`, `**Retire.**` with the pruning case
and the node-close cascade, `**Commit.**`, and `**Signal.**`

This is batch 11 of 12 — the loop's back half, from the grow verbs through the
session boundary.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L408–609 is a baseline coordinate — three batches have
  inserted markers above your region by now.** Carve from `**When a picked producer
  needs fresh review**` to the line **before** `**Finish.**`, consuming the front of
  `pending-skill-lifecycle`. Your region ends **including** the blank separator before
  that anchor, per the marker-placement convention — the F7 correction, which is why
  the baseline range is L408–609 and the size 13,712 rather than L408–608 / 13,711.
- Mint exactly one residual, **`pending-skill-finish`**, covering `**Finish.**` to end
  of file, as `class=triggering kinds=*` **with no `defers=`**.
- **There is nothing to inherit from `pending-skill-lifecycle`.** A residual never
  carries `defers=`.

### The pre-decided call: the ADR reconciliation clause is a *mention*

`batches-k13` told you to read `evidence-moves-k26`'s three-way overlap call and
honour it. The node brief settles the family instead, and your side of it is the
simplest of the three:

**`SKILL.md` L550–554 — *"Retirement is also the moment to reconcile the ADR set…
never append a superseding ADR"* — is a family-C mention, not a site to decide.** It
is a clause inside the node-close cascade's prose, and it is **unsplittable from it at
line granularity** (baseline L554 carries the end of the ADR clause and the start of
*"That may leave the next ancestor with no live leaf either"*). It takes the cascade
unit's class, which is procedural, and it owes **no** family edge: family C's owner —
`SKILL.md` L217–227, carved by `execute-k29` — already names retirement as a
checkpoint, and so does the owner's body in `driving.md` §*Reworking ADRs and
briefs…*, whose closing paragraph names *"retiring a leaf or node"* explicitly.

So the ADR-reworking rule reaches a retiring session through the mandate it already
holds, not through a second condition here. Do **not** carve L550–554 out as a third
statement of the rule.

### Edge inventory rows owned: 28 and 37

| row | source | target | note |
|---|---|---|---|
| 28 | `**Retire.**`'s node-close step 1 (L533, *Check the node's brief `Done when`*) | `BRIEF-FORMAT.md` bodies | `decompose-moves-k28` carved the target |
| 37 | `driving.md` §*Prune, reorder, or file an issue* (L702, its `SKILL.md` citation) | your `**Retire.**` pruning body | **the one edge in the whole inventory whose source is carved before its target** — `decompose-moves-k28` could not write it, so it is yours, written *into* an earlier batch's marker. Conditional on your pruning prose ending up procedural; decline with that reason if it is triggering |

### The judgement this batch exists for

This region contains grove's **densest procedural prose** and its **two HITL
conditions**. The split should be sharp, and the risk runs one way: classifying
verb mechanics as triggering because they are stated as rules.

- **The *tree is a real directory tree* paragraph** (L417–448, 4,466 bytes) is
  almost entirely the mechanics of three verbs — what `leaf-decompose` moves, what
  `leaf-insert` shifts, why headers rewrite zero file contents. Procedural. The
  condition ahead of it (*the current item proves bigger* / *a new concern must
  sequence ahead*) already lives in `**Decompose.**`, carved by `execute-k29`.
  Check that unit and defer into these bodies from it rather than restating the
  condition.
- **`**Retire.**`** (L471–562, 6,013 bytes) is the largest block in the region and
  is genuinely mixed:
  - *A leaf ends one of two ways* and *retirement touches one filename and nothing
    else* — conditions.
  - **Pruning is HITL — an agent never prunes on its own**, and an AFK session
    that discovers the path is decided against **says so and stops**. That is a
    condition of the first importance: a session never told it would prune on its
    own authority, and the node brief's asymmetry argument applies at full
    strength. Do not bury it in a procedural body.
  - The **node-close cascade** — the four numbered steps, the brief-less-node
    exception, the recursion up the parent chain — reads procedural, but *the
    close asks the human nothing* is a condition, and so is *escalate if the check
    fails and you cannot name the gap*.
  - The `leaf-retire` / `leaf-prune` invocations and the `DONE`/`ABANDONED` infix
    mechanics are procedural.
- **`**Commit.**`** (L564–591) states *one task = one focused commit* and
  *name the work item by its stable handle* — both triggering — and then the
  git/jj asymmetry and the jj sealing mechanics, which are procedural. Note the
  pointer to `linkuistics:using-jujutsu`, which is **not** an embedded file and
  therefore never a `defers=` target.
- **`**Signal.**`** (L593–608) is a condition (*run `grove-llm complete` as your
  last action, then do nothing else*) with a short mechanical body. The
  `complete` / `complete --done` distinction is load-bearing and belongs with the
  condition, not the body.

### Scope

`kinds=*` throughout. The pruning HITL rule names `requirements`, `prototype` and
`finish` as the HITL kinds, which tempts a scope — but the rule is addressed to
*the AFK session that must stop*, so scoping it to the HITL three would deliver it
to precisely the kinds that do not need it. `**Signal.**`'s `--done` clause is
about `finish`, but every session needs to know the plain form is not the
`finish` form.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-skill-finish` covers the rest of the file and nothing else, and carries no
  `defers=`.
- **Rows 28 and 37 are each reported** — written, or declined with a reason.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The pruning-HITL and node-close classifications are recorded with their reasoning —
  they are the two most consequential calls left in this batch now that the ADR
  overlap is settled.

## Notes

- Fenced/indented blocks: the four numbered close steps are a list, not a fence —
  but check fence state before placing any marker, because a marker recognised
  only at neutral state that lands mid-fence silently joins the preceding unit.
- Doubts to carry forward, by id. The pruning-HITL and node-close classifications
  are the two most consequential in this batch.

## Batch record

**Thirteen real units minted, one residual, `pending-skill-lifecycle` consumed.**
`EMBEDDED_UNITS` 120 → 133. `cargo build` green; `cargo test` green — 40 test
binaries, **1,023 tests, 0 failures**, including
`the_embedded_unit_set_is_pinned_complete`. Two residuals remain in the whole
corpus — `pending-skill-finish` (mine) and the `continue` seed — and both are
#12's, so #12 can reach zero.

**Anchors executed against, both re-verified unique** (`grep -Fc` returned 1 for
each): `**When a picked producer needs fresh review**` and `**Finish.**`. **The
baseline range agreed with the anchors exactly** — pre-batch L432–633 measures
**13,712 bytes**, this leaf's figure to the byte, including the one-byte blank
separator the F7 correction added. No disagreement to report, no departure from
the coordinates. Zero fences in the region (`SKILL.md`'s four are all above
L432), so every marker sits at neutral state without a judgement call.

### The units, in document order

Slice bytes are marker-line-inclusive — what a mandate actually carries — fetched
through `./target/debug/grove-llm methodology <id>` rather than measured by hand.

| id | class | scope | slice | prose |
|---|---|---|---|---|
| `skill-cutting-a-review-leaf` | **procedural** | — | 680 | 620 |
| `skill-directory-tree-and-grow-verbs` | **procedural** | — | 2,479 | 2,375 |
| `skill-kind-on-the-tree-verbs` | **procedural** | — | 1,548 | 1,487 |
| `skill-retire` | triggering | `*` | 386 | 298 |
| `skill-leaf-retire-mechanics` | **procedural** | — | 788 | 728 |
| `skill-retirement-touches-one-filename` | triggering | `*` | 362 | 284 |
| `skill-pruning-is-hitl` | triggering | `*` | 442 | 346 |
| `skill-leaf-prune-mechanics` | **procedural** | — | 1,111 | 1,052 |
| `skill-node-close-cascade` | triggering | `*` | 1,041 | 946 |
| `skill-node-close-steps` | **procedural** | — | 2,474 | 2,359 |
| `skill-commit` | triggering | `*` | 901 | 805 |
| `skill-commit-boundary-in-git-and-jj` | **procedural** | — | 1,292 | 1,224 |
| `skill-signal` | triggering | `*` | 1,241 | 1,188 |
| `pending-skill-finish` | triggering | `*` | 10,844 | 10,783 |

Six triggering (4,373 B ships in every mandate), seven procedural (10,372 B ships
in none). `defers=` written, in full:

- `skill-retire` → `skill-leaf-retire-mechanics`
- `skill-pruning-is-hitl` → `skill-leaf-prune-mechanics`
- `skill-node-close-cascade` → `skill-node-close-steps`
- `skill-node-close-steps` → `brief-every-node-carries-one`,
  `brief-suggested-shape` (**row 28**)
- `skill-commit` → `skill-commit-boundary-in-git-and-jj`
- `skill-directory-tree-and-grow-verbs` → `skill-kind-on-the-tree-verbs`
- `skill-decompose` (edited in place, decoupling lemma) → **+
  `skill-directory-tree-and-grow-verbs`** (addition A1)
- `task-in-session-doubt-budget` (edited in place) → **+
  `skill-cutting-a-review-leaf`** (addition A2)
- `driving-prune-reorder-or-file-an-issue` (edited in place) → **+
  `skill-leaf-prune-mechanics`** (**row 37**)
- `pending-skill-finish` → **none**, as the convention requires

**Per-mandate effect: every one of the nineteen mandates loses 9,278 bytes** —
the largest saving of any batch so far, past `shape-cutting-k30`'s 6,514. Before,
one `triggering kinds=*` residual shipped 24,559 B from this region; after,
15,217 B ships (4,373 B of real conditions + the 10,844 B residual #12 carves)
and 10,372 B is addressable procedure, against 64 B of growth in the two
*shipping* markers that now carry new addresses (`skill-decompose` +36,
`task-in-session-doubt-budget` +28; `driving-prune-…`'s +34 is procedural and
ships nowhere). The ratio is structural, not lucky: this region is the loop's
verb-mechanics run, so most of it is *how*, not *whether*.

### The pre-decided call, applied not re-decided

**Family C — `SKILL.md` L550–554 is a mention.** Landed as the node brief settled
it: the ADR-reconciliation clause is inside `skill-node-close-steps` and carved
out as nothing. Verified mechanically rather than accepted: post-marking L594
carries `…never append a superseding ADR (`linkuistics:decision-records`). That
may leave the next ancestor with no` — the end of the ADR clause and the start of
the recursion sentence on one line, so no whole-line marker separates them and
this pass edits no prose. Unsplittable exactly as the brief predicted, and it
takes the cascade body's class, procedural. **No third statement of the rule
carved, and no family-C edge owed** — `skill-adrs-and-specs` (#9) already names
retirement as a checkpoint.

### The two judgement calls this batch existed for

**1 — pruning is HITL, and the paragraph splits cleanly.** This is the batch's
one genuinely new structural move: the split falls **inside a paragraph**, at the
one place a sentence boundary coincides with a line boundary (`…the system
working, not a fault.` | `Only on explicit human confirmation, run `grove-llm
leaf-prune <path>`…`). `skill-pruning-is-hitl` (**triggering**, 346 B of prose)
carries the condition alone — the path is decided against; an agent never prunes
on its own; an AFK session says so and stops; the stall is the system working.
`skill-leaf-prune-mechanics` (**procedural**, 1,052 B) carries the verb, the
node-subtree behaviour, the chain-scoping rule, and where the durable *why* goes.

The leaf's instruction was *do not bury it in a procedural body*, and the
alternative to splitting was to ship all 1,398 B to nineteen mandates. Both
failure directions are real here and the split avoids both: withheld, a session
prunes on its own authority and a human never sees the decision; fused, every
mandate carries the `leaf-prune` invocation it will use once in a grove's life.
346 B is the whole cost of closing that door.

**Scope stayed `kinds=*`, deliberately.** The rule names `requirements`,
`prototype` and `finish` as the HITL kinds, but it is addressed to the AFK
session that must **stop** — scoping it to the named three would deliver it to
precisely the kinds that do not need it, as the leaf warned.

**2 — the node-close cascade splits at the paragraph break, and the trigger is
the walk.** `skill-node-close-cascade` (**triggering**, 946 B) is *"Then walk the
parent chain…"* through *"…why pruning and the finish cycle still do"*;
`skill-node-close-steps` (**procedural**, 2,359 B) is the lead-in, the four
numbered steps, the brief-less-node exception, the ADR reconciliation and the
recursion.

The condition that had to ship is **the walk itself**. A session that never
learns to check the parent chain simply retires its leaf and stops: the node is
never closed, its brief never promoted, its handle never named in a commit
message — and **nothing errors**, which is the silent direction the asymmetry
tie-breaker exists for. *The close asks the human nothing* rides in the same unit
and is the second condition the leaf named.

**The third — *escalate if you cannot name the gap* — is inside the body, and I
checked that rather than assumed it.** It is a condition, but it only *arises*
inside the close procedure, which is entered only from a shipping trigger that
addresses it. The reason to look twice is that the shipped half says *the close
asks the human nothing* while the withheld half says *escalate* — a shape that
could actively suppress a legitimate escalation. It does not: the shipped clause
is scoped to the done-ness inference (*"A question there would have gated an
inference"*), and step 3 calls its escalation *"discretionary and always
legitimate, not a routine gate"*. The two are consistent, and the units agree
with `confirmation-boundary`'s two tests. Recorded as doubt 2 anyway, because it
is the one place in this batch where a body holds something that reads like a
condition.

### Edge inventory rows owned: 28 and 37 — reported row by row

| row | source → target | outcome |
|---|---|---|
| 28 | `skill-node-close-steps` → `brief-every-node-carries-one`, `brief-suggested-shape` | **written**, both members |
| 37 | `driving-prune-reorder-or-file-an-issue` → `skill-leaf-prune-mechanics` | **written** — the condition the plan set (my pruning prose being procedural) is satisfied by the split |

**Row 28 is invisible to the filename sweep, and this is the starkest case in the
corpus so far.** The whole 13,712-byte region contains **zero** references to any
embedded guide filename — `grep` for the eight names returns nothing but
`BRIEF.md`/`NN-….md` hits, which are the *artifact* and the filename grammar, not
`BRIEF-FORMAT.md`. So neither sweep could have found this edge in either
direction; only the inventory carries it. Same shape as `execute-k29`'s row 18
and `shape-cutting-k30`'s row 26, one degree more extreme.

I targeted the two precise bodies rather than the `brief-the-node-briefing` entry
unit `guides-k24` uses for file-level citations. `brief-every-node-carries-one`
states the close's obligations in the corpus's own words — *"The Retire cascade's
close therefore has the same work at every node it meets: a `Done when` to check
against the subtree and a brief to promote upward"* — which is steps 1 and 4
exactly; `brief-suggested-shape` defines the `## Done when` section step 1 reads.
The entry unit would have reached both by chaining, but through 1.5 kB the
closing session does not need.

**Row 37 was writable because of call 1, and it is worth stating why the plan
made it conditional.** Had the pruning paragraph stayed one unit it would have
been triggering (the asymmetry forces that), and a `defers=` naming it is a build
error — the plan's anticipated decline. The split produced a procedural half, so
the row is written to that half and **not** to `skill-pruning-is-hitl`, which
ships in every mandate anyway and needs no address. Note the source is
`class=procedural`, unlike the inventory's *source (triggering)* column header —
legal, and the same shape rows 41–42 established.

### Two inventory additions, both written, both required for (R)

| addition | edge | why it is honest |
|---|---|---|
| A1 | `skill-decompose` → `skill-directory-tree-and-grow-verbs` | the leaf's own instruction: the condition (*the current item proves bigger* / *a new concern must sequence ahead*) already lives in `**Decompose.**`, so defer into the mechanics rather than restate it. `skill-decompose` names all three verbs; this unit is what they do |
| A2 | `task-in-session-doubt-budget` → `skill-cutting-a-review-leaf` | family A's owner. Its table row reads *"`leaf-add` a `review-<producer>` leaf, with the doubt written into its body"*; this unit is that sentence expanded — finish to a reviewable boundary, cut, retire, commit under the producer's handle |

Both are the only root their body has, and both are genuine condition→body edges
rather than the near-verbatim restatement roots `shape-cutting-k30`'s finding 2
flagged.

**`skill-cutting-a-review-leaf` withholds nothing, checked rather than
assumed.** Making the region's opening paragraph procedural is safe only if none
of its rules ships nowhere else. Its one candidate is *"Retiring it is the
filename `DONE` transition and nothing else, and it leaves the review
byte-identical"* — and that rule ships, at `kinds=*`, as
`skill-retirement-touches-one-filename`, whose own text states the general case
and names the review exception outright. Everything else in the paragraph is
sequence and mechanics.

### Five declines, each with its reason

| candidate | reason |
|---|---|
| `brief-every-node-carries-one` (L40, *"promoted upward (see SKILL.md, "Retire")"*) → `skill-node-close-steps` | **would close a cycle** with row 28 and fail the build's `ProceduralCycle` check. Mechanically excluded, not a preference — and row 28 is the listed obligation, so it wins |
| `brief-suggested-shape` (L89, same citation) → `skill-node-close-steps` | same cycle, and the hit is inside the fenced brief template |
| `driving-reworking-adrs-and-briefs` (L320, *"retiring a leaf or node (`SKILL.md`'s Plan and Retire steps)"*) → my cascade units | a **checkpoint** citation — it names *when* to rework, not a procedure to fetch. The node brief pre-decided this reading for family C |
| `skill-review-ownership` → `skill-cutting-a-review-leaf` | legal (both procedural) but **no address the session lacks**: every session reaching `skill-review-ownership` came through `task-in-session-doubt-budget`, which now carries A2. Adding it is the second-inbound-path shape the plan warns lets a dropped real edge stay green. Same reasoning `shape-cutting-k30` used to decline its `driving.md` mention |
| `skill-cut-the-next-step` / `skill-integration-placement` → `skill-directory-tree-and-grow-verbs` | both already carry the exact `leaf-add` / `leaf-insert` invocations they need; the general verb mechanics answer no question they raise |

### (D), (R), (T) at end of batch

- **(D)** — all nine `defers=` members written name declared `class=procedural`
  units; every target was read back from the built binary's listing rather than
  from the source, including the two `BRIEF-FORMAT.md` ones.
- **(R)** — all seven procedural units I created are reachable from a
  `triggering kinds=*` root: `skill-cutting-a-review-leaf` from
  `task-in-session-doubt-budget` (A2); `skill-directory-tree-and-grow-verbs` from
  `skill-decompose` (A1), and `skill-kind-on-the-tree-verbs` through it;
  `skill-leaf-retire-mechanics` from `skill-retire`;
  `skill-leaf-prune-mechanics` from `skill-pruning-is-hitl` (and, second, from
  row 37); `skill-node-close-steps` from `skill-node-close-cascade`;
  `skill-commit-boundary-in-git-and-jj` from `skill-commit`.
- **(T)** — the deepest chain is two hops (`skill-decompose` → grow verbs →
  `--kind`, and `skill-node-close-cascade` → close steps → the two
  `BRIEF-FORMAT.md` bodies, both of which carry an empty `defers=`). The one
  cycle available in this region was row 28 against `BRIEF-FORMAT.md`'s two
  return citations, and it is declined above.

### Sweeps: both run, both recorded

- **Outbound** — **zero** embedded-file citations in the entire region (above).
  Nothing to write, nothing to decline.
- **Inbound** — `grep -rn 'SKILL\.md' content/` returns the same fourteen hits
  `execute-k29` and `shape-cutting-k30` enumerated. **Four point into this
  region**, exactly as `shape-cutting-k30` predicted: `BRIEF-FORMAT.md` L40 and
  L89 and `driving.md` L320 and L728. L728 is row 37, **written**; the other
  three are declined above.
- **Intra-file, the grep-invisible class** (rows 41–42's shape) — two hits, both
  declined: *"the Retire-then-Commit order below"* (L436) points at
  `skill-commit`, `class=triggering` and therefore an illegal target; and
  *"confirmation-boundary carries both"* (L550) names an ADR slug, not an
  embedded unit.

### Coverage, proved by reconstruction with the instrument controlled first

Following `research-moves-k25`'s vacuous-check trap, the strip was proved
non-blind before any comparison: `grep -v '^<!-- unit: '` removes 2,051 B from
the pre-batch `SKILL.md` and 3,117 B from the post-batch one.

- **The consumed `pending-skill-lifecycle`'s prose is 24,495 B; my thirteen units
  plus `pending-skill-finish`, marker lines stripped, are 24,495 B.**
  Byte-identical — no gap, no overlap at either boundary.
- **My thirteen units' prose is 13,712 B**, this leaf's planned region size to
  the byte, so the residual boundary is where the plan put it.
- **All three edited files with every `<!-- unit: ` line removed are
  byte-identical to their pre-batch selves** (`cmp` clean). No prose, filename or
  fence moved; trailing newline present; `SKILL.md`'s four fence lines all sit
  above the region.
- All 38 `SKILL.md` markers match `^<!-- unit: .* -->$` — unindented whole lines,
  every one at neutral fence state.

### Design findings for the aggregate review

**1 — the first mid-paragraph marker in the corpus, and it should be ratified or
reverted deliberately.** `skill-leaf-prune-mechanics` opens *inside* a paragraph:
the marker sits between two lines of continuous prose with no blank line, because
inserting one would have edited prose and broken the byte-identity invariant
every batch's coverage proof rests on. Nothing forbids it — the parser's rule is
*unindented whole line at neutral fence state*, and the node brief's own
unsplittable-paragraph reasoning is framed entirely in terms of **line**
granularity (*"the sentence boundaries fall mid-line … so the paragraph cannot be
split"*), which only makes sense if a line-aligned boundary inside a paragraph is
available. But **no previous batch has done one**: of the ten existing markers not
preceded by a blank line, nine sit between adjacent list items in
`TASK-FORMAT.md` and the tenth sits directly below `SKILL.md`'s frontmatter
terminator. None interrupts running prose. The consequence
is a rendering one — a CommonMark HTML block interrupts the paragraph, so a
reader of the rendered document sees one paragraph become two. Raw-markdown
readers (which is how the corpus is actually consumed) see marker lines and
byte-identical prose. I took it because call 1's stake is large and the seam is
clean; **the reviewer should confirm the precedent rather than discover it**,
since #12 and every future re-marking inherits it.

**2 — the unsplittable-paragraph pattern reaches instances four, five and six,
and it is now clearly systemic.** `shapes-k23` (F7), the node brief's L217–227
fusion and `shape-cutting-k30`'s bare-stem paragraph were the first three. This
region adds: **`**Signal.**`** (1,188 B, *zero* line-aligned sentence boundaries
in sixteen lines — the condition, the driver-watcher mechanics and the
`--done` distinction all take one class); **the node-close paragraph** (946 B,
~330 B of which is the "grove enforces the no-infix half" justification, fused to
two conditions); and **`skill-leaf-retire-mechanics`'s** closing *"Mechanical
bookkeeping, no need to ask"* — a genuine don't-stall condition fused mid-line to
the infix mechanics and therefore shipping nowhere (doubt 4).
`shape-cutting-k30`'s arithmetic explains it: the corpus is reflowed to ~78
columns, so a sentence boundary landing on a line boundary is chance at roughly
one in twelve. **Six instances across three batches means the classification's
grain is capped by reflow everywhere, not occasionally**, and the honest repair is
a prose pass in a later grove — reflowing so that load-bearing sentence
boundaries end lines would be a mechanical, reviewable change that raises the
achievable grain across the whole embed.

**3 — `skill-node-close-steps` opens with "Instead", the standing-alone defect
`shape-cutting-k30` recorded for `skill-integration-placement`'s "So".** Second
instance, same trade and the same reason: the alternative was shipping 2,359 B of
close procedure to nineteen mandates. The damage is bounded — the unit's second
sentence (*"Every node carries a `BRIEF.md` … so every close has the same four
steps:"*) restates its own subject completely, so only the opening conjunction
dangles. Worth the reviewer noting as a **pattern in how this corpus splits**:
condition-then-procedure prose reliably opens the procedure with a back-reference,
and mandate delivery is what makes that visible.

**4 — no prose in this region resisted classification.** The node brief asks for
narrative that is neither condition nor procedure to be named rather than forced.
There is none here; this region is the densest procedural run in the corpus and
every paragraph is either a rule or the mechanics of one.

### The ids #12 needs

Locatable by `grep -n '<!-- unit: <id>'` — no line numbers.

| needs | which is |
|---|---|
| `pending-skill-finish` | the residual it consumes; `triggering kinds=*`, no `defers=`, covering `**Finish.**` → EOF (10,783 B of prose) |
| `continue` | the other seed residual, `content/prompts/continue.md`, untouched by me |
| rows 29–31, and `skill-adrs-and-specs` for row 30 | none of them touch anything this batch created |
| all thirteen of my units | **complete as delivered — none is a target for any remaining inventory row.** `**Finish.**` cites no unit of mine, and `## Artifacts`/`## Specs`/`## Reference files` reach `BRIEF-FORMAT.md`, `CONTEXT-FORMAT.md` and `SPEC-FORMAT.md`, not the loop's verb mechanics |

**One thing #12 should not have to rediscover:** the zero-`pending-` check is two
ids away, and `continue` is a *seed* residual with no `pending-` prefix — so
`grep -rc '<!-- unit: pending-' content/` returning 0 is **not sufficient** on its
own. #12 must also consume `continue`, whose id the convention never renamed.

### Doubts to carry forward, by id

1. **`skill-pruning-is-hitl` / `skill-leaf-prune-mechanics` — the split point, and
   the precedent it sets** (finding 1). Not a doubt about the classes: both
   directions of failure are stark and the asymmetry is unambiguous. The doubt is
   the **mid-paragraph marker**, which is a corpus-wide precedent decided by one
   batch. If the reviewer rejects it, the fallback is one unit, `triggering`,
   shipping 1,398 B to nineteen mandates and declining row 37 — so rejecting it
   costs 1,052 B × 19 and an inventory row, which is the trade to weigh.
2. **`skill-node-close-cascade` / `skill-node-close-steps` — the escalation
   condition sits in the body.** Reasoned through above and I believe it is right,
   but it is the one place in this batch where a body holds something that reads
   like a condition, and the shipped half says *the close asks the human nothing*.
   If the reviewer disagrees, the repair is not a class flip (2,359 B of procedure
   would ship) but a **prose** one: hoisting step 3's escalation into the
   triggering paragraph, which is a later grove's edit.
3. **`skill-commit-boundary-in-git-and-jj` (1,224 B, procedural) — the jj sealing
   rule ships nowhere, and its failure is silent.** A jj session that runs
   `jj describe` and never `jj new` puts the *next* task's first edit into *this*
   task's change, and nothing errors. Three things make procedural defensible: the
   driver already states the VCS to every session (`stated_vcs`), `skill-commit`
   is triggering and carries the address, and `linkuistics:using-jujutsu` triggers
   on "about to commit" independently. The id is deliberately self-describing so
   the address advertises what it holds. **Still the batch's most consequential
   withheld body**, and if the reviewer wants one procedural unit promoted, this
   is the candidate.
4. **`skill-leaf-retire-mechanics` — *"Mechanical bookkeeping, no need to ask"* is
   a don't-stall condition shipping nowhere** (finding 2). Fused mid-line to the
   infix mechanics, so unsplittable; the leaf pre-judged the invocations
   procedural and I followed it. Mitigated by `prompts/continue.md` and
   `skill-commit`, both of which instruct retirement in every mandate. Flagged
   because "may I retire?" is exactly the kind of stall grove is built to avoid.
5. **`skill-kind-on-the-tree-verbs` (1,487 B, one unit) — the grain, deliberately
   against `TASK-FORMAT.md`'s.** I kept `**`--kind <kind>` appears on the grow
   verbs…**` and `**Reading is strict too**` together although a clean
   line-aligned split exists, because the second opens *"Reading is strict
   **too**"* — a back-reference that dangles standing alone, with no compensating
   benefit since both halves are procedural and share one root. `TASK-FORMAT.md`
   separates the same two subjects (`task-decompose-inherits-kind` /
   `task-name-reading-is-strict`), so **parity would argue for splitting**; it is
   two markers and no prose edit if the reviewer wants it.
