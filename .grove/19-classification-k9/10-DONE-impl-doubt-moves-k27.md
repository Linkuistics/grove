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

## Batch record

**Anchors executed against**, both re-verified unique (`grep -Fc` returned 1 for
each): `## Doubting inside a picked Grove leaf` and `## Externalizing surfaced
work`. **The baseline coordinates agreed with the anchors exactly** — the region
between them measured **11,128 B** before marking, and its two sections **2,188 B**
and **8,940 B**, which are this leaf's three figures to the byte. #5 and #6 had
inserted 17 markers above, so baseline L415–586 opened this session as L432–603.

**3 units minted, `pending-driving-doubt` consumed, one residual minted**
(`pending-driving-decompose`); `EMBEDDED_UNITS` 93 → 96. `cargo build` green;
`cargo test` green — 0 failures in every one of the 39 test binaries plus
doc-tests (595 lib unit tests; `tests/methodology.rs` 14 passed, including
`the_embedded_unit_set_is_pinned_complete`).

**The two ids this leaf's *Done when* asks for, first:**

- **`driving-doubting-inside-a-picked-leaf`** — the family-A body (§*Doubting
  inside a picked Grove leaf*), `class=procedural`.
- **`driving-review-chain-habits`** — the operative half of the family-F body.
  §*The review chain* became **two** units, not one; the other is
  **`driving-the-review-chain`** (its definition, when-to-decide, and the
  each-session-cuts-the-next mechanics with the `leaf-add` fence).

*A plan defect, not a gap:* the *Done when* says these ids let **#9 and #10 write
rows 17 and 32 without re-deriving them**. They do not — **row 17 targets
`SKILL.md` *Review ownership inside a picked leaf* and row 32 targets `SKILL.md`
`**Cut the next step…**`**, so neither row touches a unit of mine, and both
sources (`task-in-session-doubt-budget`, `task-two-shapes`) were published by
`kinds-k22` and `shapes-k23`. What #9 and #10 genuinely need these ids for is the
**converse**: to see that both rules already have a `kinds=*` address into this
file, and so to decide deliberately rather than by reflex whether to add a second
one. Same species as `evidence-moves-k26`'s three wrong baseline coordinates —
caught by executing against the anchors and the inventory rather than the prose.

### The units, in document order

Ranges are pre-classification baseline coordinates; the byte column is the exact
slice a fetch carries (marker line included), taken from
`./target/debug/grove-llm methodology <id>` rather than measured by hand.

| id | region (baseline) | class | defers | bytes |
|---|---|---|---|---|
| `driving-doubting-inside-a-picked-leaf` | L415–454 | procedural | — | 2,258 |
| `driving-the-review-chain` | L455–482 | procedural | — | 1,483 |
| `driving-review-chain-habits` | L483–586 | procedural | — | 7,574 |
| `pending-driving-decompose` | L587–754 | triggering `*` | — (never) | 9,574 |

**Nothing in this region is triggering, and that is the pre-decided call, not a
finding of mine** — both sections are bodies of rules owned in `TASK-FORMAT.md`.
The consequence is worth stating once, because it is unique in the corpus so far:
**this batch adds 0 bytes to every mandate and 11,315 bytes of addressable
procedure**, the widest asymmetry any batch has produced.

### Coverage proved by reconstruction, with the instrument controlled first

The stripped pre-batch file measured **42,696 B** — the figure `evidence-moves-k26`
independently recorded — so the instrument was proved non-blind before any
comparison, per §*Verifying a claim about the repo itself*'s own positive-control
rule.

- the 3 units, fetched in one call and stripped of their marker lines, are
  **byte-identical to baseline L415–586** (11,128 B — this leaf's figure exactly);
- `pending-driving-decompose`, likewise stripped, is **byte-identical to baseline
  L587–EOF** (9,508 B);
- 11,128 + 9,508 = **20,636** = the consumed `pending-driving-doubt`'s coverage to
  the byte, so there is no gap and no overlap;
- the whole file with every `<!-- unit: ` line removed is **byte-identical** to the
  pre-batch file likewise stripped (42,696 B both sides) — no prose, filename or
  fence moved, trailing newline and fence balance untouched. `jj diff --git
  content/ | grep -E '^[+-][^+-]' | grep -v '<!-- unit: '` is empty: all 15 changed
  lines are marker lines.

The fence at §*The review chain*'s `leaf-add` examples is intact and unsplit — the
nearest marker sits 9 lines below its close, at neutral fence state.

### The grain calls, and the one that departs from this leaf's expectation

**§*Doubting inside a picked Grove leaf* is one unit, where the leaf expected
"several".** Its five paragraphs form an unbroken chain of back-references —
*"Inside it"* → the predicate, *"Use that one pass"* → the allowance, *"If the pass
finds"* → the pass, *"The other kinds are deliberate exceptions"* → *a picked plain
producer*. There is no heading and no bold lead anywhere in the section, so
`guides-k24`'s boundary rule (*carve at a heading or a distinct block*) offers no
seam, and every candidate split produces the standing-alone defect no build checks.
At 2,188 B it is smaller than four units this classification already left whole.

**§*The review chain* is two units, where the leaf expected "the mechanics of
cutting each step, writing its body, and placing an integration to split several
ways".** It did not, and this is the batch's most consequential judgement:

- The opening paragraph and the `**Its steps are flat siblings…**` block **fused**.
  Split, the second unit opens *"**Its** steps"* with no antecedent — the same
  dangling-reference defect `evidence-moves-k26` recorded as doubt 4 and declined to
  create where it had the choice. Fusing costs one address nothing distinct uses:
  the section has exactly one inbound source.
- The twelve habit bullets stayed **one 7,574 B unit**, because they are items in a
  **single list** and `guides-k24`'s rule — *never carve inside one list*, applied
  again by `evidence-moves-k26` to §*Reworking ADRs and briefs* — forbids it absent
  a class or scope difference. There is none: everything here is procedural.
  `kinds-k22` and `shapes-k23` did carve inside lists, and in both cases a **scope
  or class change forced it**.

That makes `driving-review-chain-habits` the largest procedural unit in the corpus,
2.2× the previous largest. **The cost is bounded and lands in the right
place**: a procedural unit ships in no mandate, so the whole of it is a fetch-time
cost paid by a session that has already decided it is cutting a chain step — and
that session wants most of the twelve. Doubt 1 names the honest seam.

### Edge inventory rows owned — 11, 33 and 36 per this leaf; 39, 41 and 42 per the node brief

**This leaf's table lists only 11, 33 and 36.** Rows 38–42 were added to the node
brief *after* the leaf bodies were written — 38–40 by `research-moves-k25`, 41–42 by
`evidence-moves-k26` — and three of those five name **#7** as owner. The node brief
is authoritative, so all six are reconciled here.

| row | source | target | outcome |
|---|---|---|---|
| 11 | `task-in-session-doubt-budget` (triggering `*`) | `driving-doubting-inside-a-picked-leaf` | **written** — this section's root |
| 33 | `task-two-shapes` (triggering `*`) | `driving-the-review-chain`, `driving-review-chain-habits` | **written**, both members |
| 36 | `driving-review-chain-habits` | `task-bare-stem-reasoning` | **declined — a citation, not an edge** |
| 39 | `task-producer-impl` (triggering `kinds=impl`) | `driving-doubting-inside-a-picked-leaf` | **written** |
| 41 | `driving-reworking-adrs-and-briefs` (procedural) | `driving-doubting-inside-a-picked-leaf` | **written** |
| 42 | `driving-cite-framework-decisions-to-the-source` (procedural) | `driving-doubting-inside-a-picked-leaf` | **written** |

**Row 33 names both units rather than chaining them.** `task-two-shapes` states
both halves on its own account — *"reach for them by default, and argue yourself
out of one"* is what `driving-the-review-chain` expands, and *"cut **lazily, one at
a time, by the session that needs the next one**"* is what the habits
operationalise — so each unit gets exactly **one** honest inbound edge from the same
honest source. The rejected alternative (owner → definition → habits) would have
put the `leaf-insert` adjacency obligation behind a second hop for no gain.

**Row 36 declined, on `research-moves-k25`'s own test.** The region's single
cross-file reference is baseline L533, *"the kind beside it is what says which step
you are looking at, so the slug does not restate it (`TASK-FORMAT.md`)"* — a
parenthetical propping up a claim the sentence has **already made in full**, not
"the rest is over there". The contrast is exact and `shapes-k23` supplied it: the
`SKILL.md` twin of this same rule reads *"(`TASK-FORMAT.md` **carries the full
reasoning**)"*, which is why row 27 is writable and this is not. Independently,
`task-bare-stem-reasoning` already has two planned inbound paths, so the decline
costs no reachability.

**Rows 41 and 42 written, and they are the reason the inventory exists.** Both
sources end by naming *"the doubt pass (below)"* / *"the doubt pass below"* — a
**positional** reference that means nothing in a unit fetched alone, so the
`defers=` is precisely what replaces it. This is row 9's move applied twice more.
Neither is visible to `grep -rn 'driving\.md' content/`: they name no file.

**Row 39 written**, completing `task-producer-impl`'s three-way parenthesis
alongside `evidence-moves-k26`'s row 38; #8 owns the third member. Its reach is
redundant with row 11 (`kinds=*` already covers the `impl` mandate), and it is
written anyway for the reason the node brief lists rows 38–40 at all: each of the
three is reachable by another route, so **dropping a member leaves `cargo build`
green**, and the sentence's promise is only honoured if all three land.

**Two unlisted candidate edges, both declined.** Each is one word in one marker to
reverse, and both are recorded here rather than silently not written.

1. **`task-two-shapes` → `driving-doubting-inside-a-picked-leaf`** — `shapes-k23`'s
   inventory addition #2, handed to me as an explicit call. Its clause is *"a
   one-file change wants a mid-session subagent instead (`driving.md`)"*, which
   genuinely points at the alternative branch's procedure rather than merely
   propping up a claim — so it is closer to an edge than row 36 is. **Declined on
   reach:** `task-two-shapes` and `task-in-session-doubt-budget` are **both
   triggering `kinds=*`**, so every mandate carrying the citation already carries
   row 11's address to the identical audience. The node brief's justification for a
   second inbound edge — *"or the second condition's session has no address for
   it"* — does not fire, and an **unlisted** fifth path into the corpus's
   most-addressed body is the drop-the-real-one hazard without the inventory's
   visibility. I checked the one argument that would have overturned this: whether a
   session can act on a bare filename. It half can — `grove-llm methodology` with no
   argument lists a `<file>` column — so the citation is degraded navigation, not
   dead navigation, which is weaker than the node brief's `## Reference files`
   argument assumes.
2. **`driving-the-review-chain` → `driving-doubting-inside-a-picked-leaf`** — my own
   intra-batch reference, *"when a picked plain producer reaches the second-review
   boundary **above**"*. **Declined as orientation, not instruction.** It is the
   distinction that separates it from rows 41–42, which I wrote: those say *run the
   doubt pass*, an imperative to go execute a named procedure; this cites where a
   boundary was *defined*, and the sentence's claim is complete without following it.

**(D), (R) and (T) hold at end of batch.** Every `defers=` written names a declared
`class=procedural` unit; all three new procedural units are reachable from a
triggering one — `driving-doubting-inside-a-picked-leaf` from
`task-in-session-doubt-budget` (`*`), both chain units from `task-two-shapes` (`*`);
and my units defer to nothing, so no chain this batch created can return anywhere.

### The sweeps

```
grep -rn 'driving\.md' content/          # inbound
```

Seven hits, the same seven the last two batches enumerated. **One is mine** —
`TASK-FORMAT.md` L108, `task-producer-impl`, row 39, written. Six are not, and none
is missed:

- **`TASK-FORMAT.md` L224** — inside `task-two-shapes`, pointing at §*Doubting*.
  Mine to call, **declined** above.
- **`SKILL.md` L236, L252, L280** — all inside `pending-skill-loop`. **No edge may
  have a `pending-*` source**, so nothing is parked; they are rows 23/19 (#9) and
  #10's territory. L280 is the node brief's family-A **mention** (*"may use its
  single in-session reviewer instead"*) and its target is a unit of mine, so #10 is
  the later endpoint and owns the call.
- **`SKILL.md` L755** — the `## Reference files` index, standing sweep exclusion.
- **`BRIEF-FORMAT.md` L81** — both endpoints are #8's.

Outbound, over L432–603: **one** hit, `TASK-FORMAT.md` at baseline L533 — row 36,
declined above. **No `SKILL.md` reference exists anywhere in this region**, which is
worth stating because this leaf's *Context* predicted several and told me how to
report them: there are none to report.

A second sweep for the region's **subjects** rather than its filename, since the
filename grep is a cross-file instrument and rows 41–42 proved it blind to intra-file
edges:

```
grep -rn 'doubt pass\|fresh-context\|adversarial\|in-session review\|leaf-insert\|Reviews:\|Integrates:' content/
```

It surfaced no relationship the inventory does not already carry, and it is how rows
41–42 were confirmed present at their recorded addresses.

### Repeated-rule families this region restates — all bodies by construction

The node brief asks for unlisted families to be recorded with a call. **This region
can own none**, and the reason is structural rather than a judgement: the
pre-decided call makes every unit here `class=procedural`, and an owner must be
triggering. So each family below is reported with the site that *does* own it, and
the interesting question is whether that owner ships.

| rule restated here | owner, and does it ship? |
|---|---|
| the reviewer produces findings, not fixes; review is inspection-only | `task-review-kinds` (#2), triggering, scoped to the five `review-*` kinds — **ships to exactly the sessions that need it** |
| the `integrate-review-*` step triages, it does not capitulate | `task-integrate-review-kinds` (#2), triggering, scoped — ships |
| write the relationship line yourself | `task-declare-the-relationship` (#3), triggering `*` — ships (family I) |
| route the review through configuration, not the tree; diversity is yours | `skill-one-configuration` (#1), triggering `*` — ships (family L) |
| give every step the producer's bare stem | `SKILL.md`'s bare-stem paragraph (#10) — **ships only if #10 makes it triggering**, which is `shapes-k23`'s family-G dependency, not a new one |
| decide at the end of the session; each session cuts the next | `task-two-shapes` (#3), triggering `*` — ships (family F) |
| **cut the integration adjacent to its review — `leaf-insert`** | **nowhere at `kinds=*` — see finding F11** |

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

1. **`driving-review-chain-habits` (7,574 B) — the largest procedural unit in the
   corpus, and the call most likely to be overturned.** The case for the fusion is
   the inherited never-carve-inside-one-list rule; the case against is that this
   leaf's own body expected "the mechanics of cutting each step, writing its body,
   and placing an integration to split several ways", and twelve bullets is a lot to
   address as one id. **The honest seam, if the reviewer wants one, is above bullet
   8, *"The reviewer produces findings, not fixes"*** (baseline L547): bullets 1–7
   are *how to cut the steps*, bullets 8–12 are *how each step's session then
   behaves*, and the split is ~3,900/3,600 with no back-reference across it. It is
   one marker to add — but it splits a rendered list into two, which is `shapes-k23`'s
   F10 trade repeated in a section of multi-paragraph bullets.
2. **`driving-the-review-chain` (1,483 B) — the fusion, in the other direction.**
   The opening paragraph and the flat-siblings block are two distinct blocks by the
   document's own structure, and only the dangling *"Its steps"* argued them
   together. If the reviewer weights document structure over the standing-alone rule,
   this splits into a ~700 B definition and a ~780 B mechanics unit, and row 33 gains
   a third member. I would not: the standing-alone rule is the one no build checks.
3. **The whole region being procedural (the pre-decided call).** This leaf asked me
   to record what I saw that makes me doubt the pre-decided split, so: **one clause
   made me look twice.** §*Doubting*'s opening states the predicate's **negative
   half** — *merely finding `.grove/` or inheriting Grove control variables does not
   activate it* — and that is a false-positive guard on an allowance, which is the
   triggering shape. It is delivered to nobody. See the required record below; I did
   not split it out, because it is one sentence inside a paragraph whose remainder is
   the predicate's positive half, restated from the owner, and carving it would need a
   prose edit rather than a marker.
4. **Row 39's redundancy, and rows 41–42's.** `driving-doubting-inside-a-picked-leaf`
   ends the batch with **four** inbound edges, all four written. That is the highest
   in-degree in the corpus, and every one of them is a listed inventory row, which is
   the node brief's stated safety condition. The reviewer should confirm that
   condition is doing real work here rather than laundering redundancy: if row 11 were
   dropped, three paths would keep the build green, and only the inventory would show
   it.

### Design findings

**F11 — the `leaf-insert` adjacency obligation ships in no mandate, and this batch
is where that becomes provable.** The corpus states it **four** times and every
statement is procedural or planned-procedural: `task-review-chain-mechanics`
(#3, procedural), `task-chain-contiguity` (#3, procedural),
`driving-review-chain-habits` (mine, procedural), and `SKILL.md`
`**Cut the next step…**`, which row 32 makes a family-F **body** (#10). What ships
at `kinds=*` is `task-two-shapes`'s *"cut **lazily, one at a time, by the session
that needs the next one**"* plus a `defers=` — which is the design working as
intended if you accept that *"I am cutting a chain step"* is a trigger the session
cannot miss, and a silent defect if you do not. **The failure it guards is the
design's own worst shape**: an integration cut with `leaf-add`, landing after
intervening work, consuming `path:line` citations that have drifted — *nothing
errors, the finding just points somewhere slightly wrong*. `shapes-k23`'s doubt 5
saw one face of this from inside `TASK-FORMAT.md`; the corpus-wide count is the part
no single batch could see. **The fix is a prose edit, not a marker**: no site
isolates the two-sentence rule at a line boundary outside a list. This is the
sharpest test case the aggregate review has for *keep the `if`, defer the `then`* —
the `if` here is not a situation the session recognises, it is a verb choice inside
a procedure.

**F12 — the doubt units carry no attribution, exactly as the node brief predicted.**
`driving.md`'s addyosmani licence comment attributes a **non-adjacent** pair, and a
comment is one contiguous block, so it travelled with
`driving-when-code-depends-on-a-framework-version` (#6). §*Doubting inside a picked
Grove leaf* is the other half of that attribution and now ships as
`driving-doubting-inside-a-picked-leaf` with none. Recorded rather than
rediscovered, per the node brief. Nothing to fix while marking — `LICENSES/` still
carries the licence, so this is attribution *locality*, not compliance — but it is
now concrete rather than predicted: **a session fetching this unit receives adapted
MIT-licensed prose with no attribution attached**, and the fix is a prose edit that
duplicates the comment onto the second section.

**F13 — the sole cross-file citation in 11 kB, and what that says about the sweep.**
This region is 7.7% of the corpus and contains **one** `grep`-visible cross-file
reference, which turned out to be a decline. Its three *real* inbound edges came
from the inventory and its two intra-file edges (rows 41–42) were invisible to the
filename grep by construction. That is the strongest evidence any batch has produced
for the node brief's own claim that **the filename grep is evidence, not
completeness** — in this region the sweep's hit rate against genuine edges was
**1 in 6, and the one hit was declined**.

**F14 — no prose in this region was neither condition nor procedure.** The node
brief asks for that to be said if it turns up. The closest call was §*The review
chain*'s closing paragraph (*"grove does not require a review after every producer…"*),
which reads like narrative closure — but it states two rules with teeth (deciding
against review is normal; a chain is not a scheduling unit and not contiguous by
construction) and ends by pointing back at the `leaf-insert` obligation, so it is
procedure, and it is why it stayed inside `driving-review-chain-habits` rather than
becoming its own unit.

### The required record — the family-A owner's missing negative half

This leaf's *Done when* requires it, and `kinds-k22` recorded the same doubt from
the owner's side.

**`task-in-session-doubt-budget` states its predicate positively and never states
its negative half.** The owner says the budget applies *"once the current session
has run Bootstrap and adopted the driver's selected-leaf mandate"* and closes with
*"Outside that Bootstrap-and-mandate predicate, doubt-driven development keeps its
standalone bounded cycles."* What it never says is what my body says outright:
**merely finding `.grove/` in the checkout, or inheriting Grove control variables,
does not activate it.**

**Confirmed from both sides now, and the asymmetry is the point.** The owner is the
only `kinds=*` statement of the rule, so it is the only one every mandate carries —
and the half that ships is the half that *grants* the allowance, while the half that
**withholds it from a session that merely smells a grove** ships nowhere. A session
that reads the shipped text and finds `.grove/` beside it has no delivered reason
not to claim a budget it was never given. That is a false positive on a rule whose
whole job is to stop a second reviewer being spawned, and it is silent.

Both bodies state the guard — mine and `SKILL.md` *Review ownership inside a picked
leaf* (#9) — so a session that follows the `defers=` gets it. Whether that is
sufficient is the reviewer's call, not this batch's. **It cannot be fixed by
marking**: the guard is one sentence inside a paragraph, and lifting it into the
owner is a prose edit.
