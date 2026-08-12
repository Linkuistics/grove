# shapes-k23

## Goal

Classify **`content/TASK-FORMAT.md` from `## Composing the kinds — the two shapes`
to the line before `## The three design kinds — extra guidance`** (baseline
L193–472, 15,904 bytes): `## Composing the kinds — the two shapes`, `### The review
chain — each session cuts the next step`, `### The vendor pair — one eager call`,
`### What the shapes are not`, `## Suggested shape`, and `## A leaf never names a
harness`.

This is batch 3 of 12, and the largest single region in the plan. It **owns the
corpus-wide condition for the two shapes** (family F) — see *The pre-decided call
in this region*.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L193–472 is a baseline coordinate.** Carve from `##
  Composing the kinds — the two shapes` to the line **before** `## The three design
  kinds — extra guidance`, consuming `pending-task-shapes` in full.
  `TASK-FORMAT.md` is finished after this batch — mint **no** residual.
- The tail beyond your end anchor was already carved by `kinds-k22`; leave it
  alone.

### Edge inventory rows owned: none — and you are the source of three

Your region cites `driving.md` (L204 and elsewhere), and the guides. **Write no
cross-file `defers=`**: no target outside `TASK-FORMAT.md` is carved yet, so under
the ownership rule the later endpoint's batch owns every one of those edges.

You are the *source* of three rows, all written by later batches:

| row | edge | written by |
|---|---|---|
| 32 | your family-F owner → `SKILL.md` `**Cut the next step…**` | #10 `shape-cutting-k30` |
| 33 | your family-F owner → `driving.md` §*The review chain — when doubt earns its own leaves* | #7 `doubt-moves-k27` |
| 26–27 | `SKILL.md` *Cut the next step* / *bare stem* / *grammar is five fields* → **your procedural bodies** | #10 |

Rows 26–27 are conditional on how you split: if the prose `SKILL.md` cites for "the
full reasoning" ends up **triggering**, #10 declines the row with that reason rather
than writing an illegal edge. Either way, **name the unit holding the step-suffix
reasoning in your leaf body** so #10 reads it instead of re-deriving it.

### The pre-decided call in this region

**Family F owner — `## Composing the kinds — the two shapes`'s opening (baseline
L193–212).** *"Reach for them by default, and argue yourself out of one rather than
into it"* plus *"they are built in opposite ways, and the asymmetry is the design"*
is the **owner**: `class=triggering kinds=*`, and the corpus-wide statement of the
condition. It is the earliest of the rule's three sites, which is why it has to
carry it — the two later statements (`driving.md` §*The review chain*, #7; and
`SKILL.md` `**Cut the next step…**`, #10) become procedural bodies rooted from your
unit, and a procedural unit must be reachable at the end of its own batch.

Keep the opening's two shape bullets **with** the asymmetry sentence: the bullets
name the shapes and the sentence states the choice, and a session given one without
the other cannot act.

The **bodies** are everything after that — the exact `leaf-add` invocations, the
`leaf-insert` targeting rule for an integration, who cuts what and when — and they
are rooted from your owner in this same batch.

`### What the shapes are not` (7,946 bytes) is the section to think hardest
about. It is mostly *rejected alternatives* and *why the grammar infers no
relationship*. Rejected-alternative prose is neither a condition nor a
procedure — the node brief says to say so rather than force it into a class. But
"the grammar infers no relationship between leaves" **is** a condition: a session
that assumes an `X` requires a `review-X` after it will cut leaves it does not
need. Split that section rather than classifying it whole.

### Traps specific to this region

- **Splitting mid-fence.** L257–258 and the `leaf-add-pair` example are indented
  code blocks and fenced blocks; the parser forbids a marker inside a fence and
  will say so, but the authoring rule behind it is the one no build checks — a
  unit must read correctly standing alone.
- `## Suggested shape` contains a fenced markdown template with `#` headings
  inside it. Do not let a heading scan mistake those for section boundaries.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-task-shapes` is gone and no `pending-task-*` unit remains.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The family-F owner is in place as `class=triggering kinds=*`, with its shape
  bullets and its asymmetry sentence in one unit.
- The unit holding the step-suffix reasoning, and the family-F owner's id, are both
  named in this leaf's body — #7 and #10 read them rather than re-deriving them.

## Notes

- If any prose in `### What the shapes are not` is genuinely narrative — there to
  make the document readable and neither condition nor procedure — record that as
  a **finding about the design** in this leaf's body. Do not force it into a
  class, and do not silently leave it in a triggering unit to make the build pass.
- Doubts to carry forward, by id.

## Batch record

**Anchors executed against**, both re-verified unique (`grep -Fc` returned 1 for
each): `## Composing the kinds — the two shapes` and `## The three design kinds —
extra guidance`. The baseline range agreed with the anchors **exactly** — the
region between them measured 15,904 bytes before marking, the figure this leaf and
the node brief both carry. `kinds-k22` inserted 20 lines ahead of it, so baseline
L193–472 opened this session as L213–492.

**The two ids the leaf's *Done when* asks for, first, because two later batches
read them:**

- **Family-F owner — `task-two-shapes`** (`class=triggering kinds=*`). #7
  `doubt-moves-k27` writes row 33 into it and #10 `shape-cutting-k30` writes row 32.
- **The step-suffix reasoning — `task-bare-stem-reasoning`** (`class=procedural`).
  It is a **legal `defers=` target**, so #10 **writes** row 27's bare-stem half
  rather than declining it. See *Row 27 is writable; its other half is not* below.

### The units, in document order

**Twelve units, no residual.** Byte counts are post-marking, fetched through
`./target/debug/grove-llm methodology <id>` rather than measured by hand, so each
is the exact slice a mandate would carry (marker line included).

| id | class | scope | bytes | defers |
|---|---|---|---|---|
| `task-two-shapes` | triggering | `*` | 1,270 | the three bodies below |
| `task-review-chain-mechanics` | **procedural** | — | 2,845 | — |
| `task-declare-the-relationship` | triggering | `*` | 668 | — |
| `task-vendor-pair-mechanics` | **procedural** | — | 991 | — |
| `task-what-shapes-are-not` | triggering | `*` | 421 | `task-bare-stem-reasoning`, `task-chain-contiguity` |
| `task-bare-stem-reasoning` | **procedural** | — | 3,957 | — |
| `task-no-node-for-a-shape` | triggering | `*` | 452 | — |
| `task-chain-contiguity` | **procedural** | — | 2,100 | — |
| `task-grammar-is-five-fields` | triggering | `*` | 973 | — |
| `task-nothing-in-a-body-is-metadata` | triggering | `*` | 483 | — |
| `task-suggested-shape` | **procedural** | — | 1,487 | — |
| `task-leaf-never-names-a-harness` | **procedural** | — | 1,155 | — |

Seven triggering (5,220 B ships in every mandate), five procedural (11,582 B ships
in none). `task-two-shapes` defers to `task-review-chain-mechanics`,
`task-vendor-pair-mechanics` and `task-leaf-never-names-a-harness`.

**One `defers=` member added to an earlier batch's marker**, under the decoupling
lemma and owned by me as the later-carved endpoint: `task-leaf-filename` (#2)
gains `defers=task-suggested-shape`. It is an **in-file** edge, so it is not the
cross-file kind this leaf forbids. The root is honest rather than convenient: that
unit's last sentence is *"The file is freeform markdown — **a guide follows, not a
schema**"*, and `task-suggested-shape` is the guide it promises. Considered and
declined as a second root: `task-kind-in-the-filename`, whose
`[work-item handle](#suggested-shape)` anchor points at the same section — a
redundant address, and the plan warns that a second inbound path is what lets a
dropped real one stay green.

**Coverage, verified rather than assumed:** the twelve slices sum to **16,802
bytes**, exactly the region's current L212–503, and 16,802 − 898 bytes of marker
lines = **15,904** = the baseline figure. Independently, the whole file's units sum
to 32,096 = the whole file (no `---` preamble here, as `kinds-k22` established).
`grep -c 'unit: pending-task-' content/TASK-FORMAT.md` returns **0**:
`TASK-FORMAT.md` is finished, and `pending-skill-loop` is the only residual left in
the corpus.

### The pre-decided call, applied not re-decided

**Family F owner — `task-two-shapes`.** Landed as the node brief settled it:
`class=triggering kinds=*`, the `## Composing the kinds` heading, the opening
paragraph, **both shape bullets and the asymmetry sentence in one unit**. Not
split — the leaf asked for exactly that, and the reason holds: the bullets name the
shapes and the sentence states the choice, and a session given one without the
other cannot act. Its three deferrals are the shape mechanics the asymmetry
sentence sends a session to.

### Six families this inventory did not list, and the call on each

The node brief's default for an unlisted family — *the site stating the condition
completely and earliest is the owner, every later complete statement is a body,
everything else is a mention* — decided five of these. One is decided against the
default, and that exception is the most consequential judgement in the batch, so
it is stated first with its evidence.

**G — the slug is the bare stem, and it does not restate the kind.** Sites:
`TASK-FORMAT.md` §*What the shapes are not* bullet 1 (**#3, mine**, and the
earliest); `SKILL.md` *Every step of a shape carries the same bare stem* (#10);
`driving.md` L154 (#5) and L529 (#7), both kind-scoped restatements.

**The default would make mine the owner. It is the body instead, because the
corpus designates the split**: `SKILL.md`'s statement ends *"(`TASK-FORMAT.md`
carries the full reasoning)"* — the same designation signal the node brief used to
settle family B from `driving.md` L595. `SKILL.md` states the rule in ~900 B; my
site states it in two sentences and then spends 3.5 kB on *why the marker went*,
*what that costs*, *what it does not cost* and *why a prefix is no better*. That is
the `if` in the hub and the `then` in the guide, which is the design.
**Consequence #10 must know: `SKILL.md`'s bare-stem paragraph has to be
`class=triggering`, or the rule ships nowhere.** It is the one place in this batch
where a rule's survival depends on a later batch's call.

**J — neither shape gets a node directory.** Sites: mine (#3, earliest) and
`SKILL.md`'s *Neither shape gets a node directory* (#10). Neither designates the
other, so the default applies: **mine is the owner, `class=triggering kinds=*`.**
It is a structural prohibition, and the failure it prevents is real rather than
theoretical — a `planning` session that reads three chain steps as one artifact and
decomposes them into a node changes `pick`'s walk and gives the Retire cascade a
`Done when` to check that nobody wrote.

**I — declare the relationship in the body, by hand.** Sites: mine (#3, earliest,
with the worked `**Reviews:**` example); `SKILL.md` (#10); `driving.md` §*The review
chain* *Write the relationship line yourself* (#7). No designation, so the default
applies: **mine is the owner, `class=triggering kinds=*`,** carved out of
`### The review chain` because the class changes. The asymmetry decided it
independently of the default — this is a convention **nothing validates and no verb
prompts**, so a session that does not already hold it has no occasion to look it
up. That is the unasked question in miniature, for 668 bytes.

**K — nothing in a body is metadata.** Sites: mine at the close of §*What the
shapes are not* (#3, earliest); my own `## Suggested shape` restatement (*The body
carries no launch metadata at all*, which rides inside `task-suggested-shape` as a
body); `SKILL.md` `**Retire.**`'s *retirement touches one filename and nothing else*
(#11). **Mine is the owner, `class=triggering kinds=*`.** Its operative half is
addressed to a review session — *exactly one thing to read, its producer's committed
artifact, and never a note the producer left behind* — and to the producer tempted
to leave one.

**G′ — the grammar is five fields and infers no relationship.** Sites: mine (#3,
earliest) and `SKILL.md` *The grammar is five fields; no relationship is one of
them* (#10). No designation, default applies: **mine is the owner,
`class=triggering kinds=*`.** This leaf's own *Context* section asked for exactly
this call — *"a session that assumes an `X` requires a `review-X` after it will cut
leaves it does not need"* — and it is why row 27's *grammar is five fields* half is
**not** writable (below).

**L — a leaf never names a harness.** Sites: `SKILL.md` `**One configuration, no
other launch policy.**` (`skill-one-configuration`, #1, `kinds=*`, and the
**earliest**, which states *"no field in a task file"* outright); mine (#3). By the
default the owner is #1's and **mine is the body** — `class=procedural`, and its
content confirms it: the rule plus the vendor pair's history, which is *why* there
is no per-leaf declaration rather than *that* there is none. Rooted in-batch from
`task-two-shapes`, whose pair bullet ends *"without any per-leaf routing
metadata"* — the clause this section answers.

### Edge inventory rows owned: none — reported as planned

**Zero cross-file `defers=` written**, exactly as this leaf predicted. Both sweeps
run and recorded.

- **Outbound** — over L212–503: **one** hit. `task-two-shapes` L224 ends *"a
  one-file change wants a mid-session subagent instead (`driving.md`)"*, pointing at
  §*Doubting inside a picked Grove leaf*, which is still a single trivial
  `class=triggering kinds=*` unit — not writable, and #7's under the later-endpoint
  rule. **Recorded as an inventory addition for #7** (below); the plan does not list
  it because it is a mention inside a unit whose subject is the two shapes.
- **Inbound** — `grep -rn 'TASK-FORMAT\.md' content/`: eight hits, five of them
  pointing into this region. Their outcomes are now **determined**, and each is
  reported for its owning batch rather than left to be discovered against the build.

| hit | targets | owner | outcome |
|---|---|---|---|
| `SKILL.md` L335 *"carries the full reasoning"* | `task-bare-stem-reasoning` (procedural) | #10 | **row 27 writable** — write it |
| `SKILL.md` L339 *"(`TASK-FORMAT.md`)"* on the declaration lines | `task-declare-the-relationship` (**triggering**) | #10 | **decline** — illegal target |
| `driving.md` L156 (the pair's bare stem) | `task-bare-stem-reasoning` (procedural) | #5 | **row 35 partly writable** — write it |
| `driving.md` L533 (the chain's bare stem) | `task-bare-stem-reasoning` (procedural) | #7 | writable; #7's call under row 36 |
| `driving.md` L141 *"There is no node directory"* | `task-no-node-for-a-shape` (**triggering**) | #5 | **decline** — illegal target, and a parenthetical citation besides |
| `BRIEF-FORMAT.md` L23 *"(flat-lazy-review; `TASK-FORMAT.md`)"* | `task-no-node-for-a-shape` (**triggering**) | #8 | **decline** — same, and not in the inventory |
| `SKILL.md` L183 | `task-*` per-kind bullets (#2) | #9 | row 24, already declined by `kinds-k22` |
| `SKILL.md` L750 | the `## Reference files` index | #12 | standing sweep exclusion |

**Row 27's two halves come out differently, and that is the plan's own conditional
resolving, not a gap.** Its *bare stem* source targets a procedural body and is
**written**; its *grammar is five fields* source targets `task-grammar-is-five-fields`,
which family G′ makes triggering, so that half is **declined as an illegal edge**.
Row 26 (`SKILL.md` *Cut the next step* → §*Composing the kinds* bodies) is
writable at `task-review-chain-mechanics` and `task-vendor-pair-mechanics`, both
procedural.

**Two inventory additions, both declined here with reasons:**

1. **`skill-one-configuration` (#1, triggering `kinds=*`) →
   `task-leaf-never-names-a-harness` (procedural, mine).** A genuine cross-file
   trigger→body edge that I own as the later endpoint — family L's owner addressing
   family L's body. **Declined** because this leaf instructs *write no cross-file
   `defers=`*, and because it is an **address rather than a reachability need**: the
   body is already honestly rooted in-batch from `task-two-shapes`. It is a one-line
   edit for the aggregate review or a later batch if the reviewer wants the address.
2. **`task-two-shapes` → `driving.md` §*Doubting inside a picked Grove leaf*.** The
   outbound sweep's single hit, above. Not writable yet; **#7 owns it** and should
   decide whether *"a one-file change wants a mid-session subagent instead"* is a
   trigger→body edge or a supporting citation.

**(D), (R) and (T) hold at end of batch.** Every `defers=` I wrote names a declared
`class=procedural` unit; all six procedural units are reachable from a triggering
one — `task-review-chain-mechanics`, `task-vendor-pair-mechanics` and
`task-leaf-never-names-a-harness` from `task-two-shapes`; `task-bare-stem-reasoning`
and `task-chain-contiguity` from `task-what-shapes-are-not`; `task-suggested-shape`
from `task-leaf-filename` — and every chain is one hop, so termination is trivial
and no chain crosses a file boundary.

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

1. **`task-what-shapes-are-not` (421 B) — the weakest triggering claim in the
   batch, and the one the reviewer should attack first.** Its prose is a section
   lead: one sentence of rationale (*those names are long, and that is the trade the
   scheme makes*) and one list lead-in (*three things that shape looks like it could
   be and is not:*). It states no rule of its own. It was classified triggering on
   two grounds — it is the honest **in-batch root** for `task-bare-stem-reasoning`
   and `task-chain-contiguity`, and shipping it is what gives `task-no-node-for-a-shape`,
   `task-grammar-is-five-fields` and `task-nothing-in-a-body-is-metadata` their
   section heading in a composed mandate. **If the reviewer reads it as the
   artificial root the plan warns against, the honest alternatives are named in
   finding F8 below.** Note the standing risk either way: with two inbound paths into
   `task-bare-stem-reasoning` after #10 writes row 27, dropping the *real* one leaves
   the build green — which is why row 27 is a listed obligation and not left to the
   sweep.
2. **`task-bare-stem-reasoning` (3,957 B) — the biggest single call here, and it is
   a dependency, not a decision this batch can finish.** Procedural on the corpus's
   own designation (family G). If #10 classifies `SKILL.md`'s bare-stem paragraph
   procedural too, the rule ships in **no** mandate and nothing fails: both units are
   reachable, the build is green, the pinned set is unchanged, and the only evidence
   is a rule that stopped arriving. **The reviewer should check that one edge
   specifically.** Flipping mine to triggering is a one-marker fix, at a cost of
   ~3.9 kB in nineteen mandates for a rule whose operative statement is two
   sentences.
3. **`task-no-node-for-a-shape` (452 B) and `task-grammar-is-five-fields` (973 B) —
   decided by the default rule, against `SKILL.md` twins in #10's region.** Taking
   the earliest site as owner means **#10's two paragraphs must become bodies**, or
   every mandate carries each rule twice. That is the *loud* direction rather than
   the silent one, but it is still a defect, and the pair should be read together
   with row 32 — which already makes `SKILL.md`'s `**Cut the next step…**` a body of
   my family-F owner, so the shape is the plan's, not an invention.
4. **`task-declare-the-relationship` (668 B) — a genuine coin-flip on grain.** The
   alternative was leaving it inside `task-review-chain-mechanics`, where a session
   cutting a chain step would fetch it along with the rest of the mechanics. That was
   rejected because nothing *makes* a session fetch the mechanics — `SKILL.md`'s
   `**Cut the next step**` carries the `leaf-add` invocations inline — so the
   convention would ride on a lookup that may never happen. If the reviewer prefers
   the coarser grain it is one marker to delete, and the two units are adjacent.
5. **`task-chain-contiguity` (2,100 B) fuses bullets 3 and 4 deliberately.** Bullet
   4 opens *"**So** an integration is cut where `pick` reaches it next"*, so splitting
   there leaves a unit that cannot read correctly standing alone — the authoring rule
   no build checks. The residue: the *exact* `leaf-insert` targeting condition, which
   is the most operationally consequential sentence in the section, sits behind
   1.2 kB of contiguity argument in one procedural fetch.
6. **`task-suggested-shape` (1,487 B) — procedural, and the class was close.** It is
   the task-file template every leaf-cutting session's output conforms to. It went
   procedural because `leaf-add` **generates** that shape, so the session receives it
   as an artifact rather than needing to hold it, and the two facts a session must
   hold — the handle is `<slug>-k<key>` and the body carries no launch metadata —
   ship from `task-kind-in-the-filename` (#2) and `task-nothing-in-a-body-is-metadata`
   respectively. If the reviewer disagrees, it is one marker.

### Design findings

**F7 — `### What the shapes are not` bullet 1 cannot be split, and the grain lost
there is the largest in the corpus so far.** The bullet is 3.9 kB carrying **six**
distinct blocks: the rule (two sentences), *Why the marker went*, *What that costs,
exactly*, two exactness notes, *What it does not cost*, *Both spellings stay legal*,
and the prefix rejection. Every one after the first is an **indented continuation
paragraph inside a list item**, and a marker is a whole *unindented* line, so no
boundary is available — inherited convention rule 4, and the same shape as
`spine-k21`'s F1 and the node brief's `SKILL.md` L217–227 fusion. Two of those
blocks carry rules with teeth that are now delivered only behind a deferral: *an
older `…-review-k14` you meet in a live tree is a well-formed leaf; **leave it
alone***, and ***do not compensate with a commit-subject convention***. Both are
prohibitions against a session *acting*, which is the triggering shape, and neither
can be lifted out without a prose edit. **De-fusing this bullet is a prose edit for
a later grove, not a marking decision.**

**F8 — a section lead had to carry a class, and the corpus offers no honest
alternative here.** `### What the shapes are not`'s lead is neither a condition nor
a procedure — it is the narrative the node brief's *Notes* asks to be flagged rather
than forced. It was made triggering because the partition leaves no third option and
because the two alternatives are both worse: folding it into
`task-bare-stem-reasoning` buries the section heading inside a unit that never
ships, leaving three triggering units headless in every mandate; rooting the two
bodies from `task-kind-in-the-filename` instead is an *adjacent-subject* root — that
unit says the kind lives in the filename, not that the slug must not restate
it — which is closer to the artificial root the plan warns against than the lead is.
Recording it as a finding rather than pretending the lead states a rule.

**F9 — the section promises three items and delivers one, in every composed
mandate.** With bullet 1 and bullets 3–4 procedural and bullet 2 triggering, a
mandate reads *"Three things that shape looks like it could be and is not:"*
followed by a single `- **Neither shape gets a node directory.**` bullet. It is not
wrong — the lead's marker carries the other two ids, so the addresses arrive with
the slice — but it is the second place the classification makes a composed mandate
read slightly oddly, after `kinds-k22`'s F4. Same character: harmless, unfixable by
marking, and worth one sentence to the reviewer. A related and pre-existing
inaccuracy, noted because a reader will trip on it either way: the section says
*three* things and the list has **four** bullets, the fourth being the *"So an
integration is cut…"* consequence of the third. That is a prose defect this pass
inherited and did not create.

**F10 — three markers split a markdown list, and this batch is the first place
that changes how the document renders.** The markers above bullets 1, 2 and 3 of
`### What the shapes are not` sit at column 0 between list items, so a strict
CommonMark renderer now sees three consecutive lists where it saw one. `kinds-k22`
established the practice (its five per-kind producer bullets are split the same
way) and nothing in the grammar or the gate objects, but this section's bullets are
multi-paragraph rather than one-liners, so the visual seam is more noticeable.
Recorded because the corpus is still read as a provisioned skill today, and the
reviewer should confirm the trade is accepted rather than unnoticed.
