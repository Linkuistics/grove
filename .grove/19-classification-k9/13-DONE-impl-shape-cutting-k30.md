# shape-cutting-k30

## Goal

Classify **`content/SKILL.md` from `**Cut the next step, when it is needed.**` to the
line before `**When a picked producer needs fresh review**`** (baseline L247–407,
10,068 bytes): `**Cut the next step, when it is needed.**` with the review chain and
vendor pair bullets, `**Neither shape gets a node directory.**`, `**Every step of a
shape carries the same bare stem**`, `**Declare the relationship in the body, by
hand.**`, `**The grammar is five fields; no relationship is one of them.**`, `**A
chain is not contiguous by construction…**`, and `**There is no exception to
check.**`

This is batch 10 of 12.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L247–407 is a baseline coordinate — batches #1 and #9
  have inserted markers above your region by the time you open the file.** Carve from
  `**Cut the next step, when it is needed.**` to the line **before** `**When a picked
  producer needs fresh review**`, consuming the front of `pending-skill-shapes`. Your
  region ends **including** the blank separator before that anchor, per the
  marker-placement convention — the F7 correction, which is why the baseline range is
  L247–407 and the size 10,068 rather than the L247–406 / 10,067 `batches-k13`
  recorded.
- Mint exactly one residual, **`pending-skill-lifecycle`**, covering `**When a picked
  producer needs fresh review**` to end of file, as `class=triggering kinds=*` **with
  no `defers=`**.
- **There is nothing to inherit from `pending-skill-shapes`.** A residual never
  carries `defers=`.

### The pre-decided call: `**Cut the next step…**` is a *body*

`batches-k13` called this paragraph triggering and asked you to reconcile it with two
sibling batches' recorded calls. The node brief settles it as **family F**:

- **The owner is `TASK-FORMAT.md` §*Composing the kinds — the two shapes*'s opening**
  (#3), which states *"reach for them by default, and argue yourself out of one rather
  than into it"* and *"they are built in opposite ways"*.
- **`**Cut the next step, when it is needed.**` (L247–268) is the body** for its
  restatement of that asymmetry — `class=procedural`, rooted from the owner, which is
  **row 32, yours to write**. Read `shapes-k23`'s body for the owner's id.
- What is *not* a restatement is still your judgement: *when* to decide for review
  ("the artifact is load-bearing — a spec, a decomposition you will build on for
  months, a subsystem") and the in-session-reviewer pointer at L266–268. If either
  reads as a condition in its own right, carve it as one and say why.

**The L266–268 in-session-reviewer pointer is a mention** of family A, not a site to
decide: its owner is `TASK-FORMAT.md` §*In-session doubt is budgeted…* (#2). An edge
from it into `driving.md` §*Doubting…* is legal and harmless — that unit is
procedural and already rooted by row 11 — but it is **not an inventory obligation**,
so write it only if the reference reads as a genuine trigger→body address.

### Edge inventory rows owned: 26, 27 and 32

| row | source (in your region) | target | note |
|---|---|---|---|
| 32 | `TASK-FORMAT.md` family-F owner | your `**Cut the next step…**` body | not optional; its only root |
| 26 | your `**Cut the next step…**` body | `TASK-FORMAT.md` §*The review chain* / §*The vendor pair* mechanics bodies | conditional: only where those units are procedural |
| 27 | `**Every step of a shape carries the same bare stem**` (L323), `**Declare the relationship…**` (L327) | `TASK-FORMAT.md` §*What the shapes are not* bodies | conditional the same way — `shapes-k23`'s body names the unit holding the step-suffix reasoning; **read it rather than re-deriving it** |

### The judgement this batch exists for

This region is unusually condition-rich, and the temptation is to classify the
whole thing triggering because it is all rules. Resist that — several of these
paragraphs are *justifications* of a rule stated elsewhere, and a justification is
not a condition.
- **`**Every step of a shape carries the same bare stem**`** is a naming rule
  (triggering: a session that does not know it will suffix its slugs) *plus* a
  long justification of why the step suffix was deleted (not a condition). Split
  it.
- **`**The grammar is five fields; no relationship is one of them.**`** is the
  paragraph most worth getting right. *Grove infers no relationship between
  leaves* is a genuine condition: a session that assumes an `X` requires a
  `review-X` after it cuts leaves it does not need, and nothing errors. The
  five-field enumeration behind it is procedural.
- **`**A chain is not contiguous by construction…**`** (3,314 bytes) is the
  largest block here. Its condition is *an integration consumes, so a gap
  corrupts*; its body is the directory-local placement rule with the three
  clarifying bullets and the fenced `leaf-insert` line. The asymmetry is stark and
  the split should be clean.
- **`**There is no exception to check.**`** is 464 bytes of pure justification for
  the preceding rule. If it is neither condition nor procedure, **say so as a
  finding** rather than forcing it — but note that *"a session that departs anyway
  owns the drift"* reads as a condition, so read it twice.

### Scope

`kinds=*` throughout, almost certainly. The chain and pair rules are addressed to
whichever session is deciding to cut the next step, which can be any kind.
`**A chain is not contiguous…**` speaks most directly to `review-*` and
`integrate-review-*` sessions, but a producer cutting the first step needs the
same condition — scoping it to the review kinds would withhold it from exactly the
session that opens the chain.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-skill-lifecycle` covers the rest of the file and nothing else, and carries
  no `defers=`.
- **Rows 26, 27 and 32 are each reported** — written, or declined with a reason.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- Whatever inside `**Cut the next step…**` you carved as a condition in its own
  right — rather than as part of the family-F body — is named with its reasoning.

## Notes

- Fenced blocks at L257–258 (indented `leaf-add` lines), L292 and L379–381
  (`text` fence). Do not split mid-fence.
- Doubts to carry forward, by id.

## Batch record

**Eight real units minted, one residual, `pending-skill-shapes` consumed.**
`EMBEDDED_UNITS` 112 → 120. `cargo build` green; `cargo test` green — 40 test
binaries, **1,023 tests, 0 failures**, including
`the_embedded_unit_set_is_pinned_complete`. `content/SKILL.md` still carries the
one remaining residual (`pending-skill-lifecycle`), and
`content/prompts/continue.md` is still a seed residual; those two are #11's and
#12's.

**Anchors executed against, both re-verified unique** (`grep -Fc` returned 1 for
each): `**Cut the next step, when it is needed.**` and `**When a picked producer
needs fresh review**`. **The baseline range agreed with the anchors exactly** —
pre-batch L263–423 (baseline L247–407 plus the sixteen markers batches #1 and #9
inserted above) measures **10,068 bytes**, this leaf's figure to the byte,
including the one-byte blank separator the F7 correction added. No disagreement
to report and no departure from the coordinates.

### The units, in document order

Slice bytes are marker-line-inclusive — what a mandate actually carries — fetched
through `./target/debug/grove-llm methodology <id>` rather than measured by hand.

| id | post-marking | class | scope | slice | prose | marker |
|---|---|---|---|---|---|---|
| `skill-cut-the-next-step` | L262 | **procedural** | — | 3,507 | 3,387 | 120 |
| `skill-no-node-for-a-shape` | L320 | **procedural** | — | 541 | 483 | 58 |
| `skill-bare-stem-rule` | L329 | triggering | `*` | 1,053 | 960 | 93 |
| `skill-declare-the-relationship` | L343 | **procedural** | — | 464 | 401 | 63 |
| `skill-grammar-is-five-fields` | L351 | **procedural** | — | 1,120 | 1,059 | 61 |
| `skill-chain-gap-asymmetry` | L367 | triggering | `*` | 1,835 | 1,710 | 125 |
| `skill-integration-placement` | L393 | **procedural** | — | 1,664 | 1,604 | 60 |
| `skill-no-exception-to-check` | L423 | triggering | `*` | 532 | 464 | 68 |
| `pending-skill-lifecycle` | L431 | triggering | `*` | 24,559 | 24,495 | 64 |

Three triggering (3,420 B ships in every mandate), five procedural (7,296 B ships
in none). `defers=` written, in full:

- `skill-cut-the-next-step` → `task-review-chain-mechanics`,
  `task-vendor-pair-mechanics` (row 26)
- `skill-bare-stem-rule` → `task-bare-stem-reasoning` (row 27, bare-stem half)
- `skill-chain-gap-asymmetry` → `skill-integration-placement`,
  `task-chain-contiguity`
- `task-two-shapes` (edited in place, decoupling lemma) → **+
  `skill-cut-the-next-step`** (row 32)
- `task-no-node-for-a-shape`, `task-declare-the-relationship`,
  `task-grammar-is-five-fields` (edited in place) → **+ their `skill-` bodies**
  (three inventory additions, below)
- `pending-skill-lifecycle` → **none**, as the convention requires

**Per-mandate effect: every one of the nineteen mandates loses 6,514 bytes.**
Before, one `triggering kinds=*` residual shipped 34,624 B from this region;
after, 27,979 B ships (3,420 B of real conditions + the 24,559 B residual #11 and
#12 will carve) and 7,296 B is addressable procedure, against 131 B of growth in
the four `TASK-FORMAT.md` triggering markers that now carry the new addresses.
This is the largest per-mandate saving of any batch so far, and the reason is
structural rather than lucky: **five of this region's seven paragraphs are the
hub restating a rule `TASK-FORMAT.md` already owns**, so the classification's
answer is "one ships, the other is addressable" five times over. See design
finding 2.

### The pre-decided call, applied not re-decided

**Family F — `**Cut the next step, when it is needed.**` is a body.** Landed as
the node brief settled it: `skill-cut-the-next-step`, `class=procedural`, rooted
by row 32 from `task-two-shapes`. Kept whole, all 56 lines: the intro paragraph
ends in a colon and the two shape bullets are its list, so splitting the lead
from the list leaves a condition pointing at nothing — `execute-k29`'s grain call
on `**Execute.**` and `guides-k24`'s boundary rule (*never inside one list*),
neither of which a scope difference overrides here since both bullets are the
same class.

**What I carved out of it as a condition in its own right: nothing, and the
reason is not judgement.** This leaf licensed carving *when to decide for review*
("the artifact is load-bearing — a spec, a decomposition you will build on for
months, a subsystem") and the in-session-reviewer pointer. Both sit at
**two-space indentation inside bullet 1**, and a marker is an unindented whole
line, so neither is separable — the same mechanical bound as `shapes-k23`'s F7
and the node brief's L217–227 fusion, in its third instance.

**It costs nothing here, and that is worth stating rather than assuming.**
`task-two-shapes` is `triggering kinds=*` and its own review-chain bullet already
reads *"Decide for it when the artifact is load-bearing (a spec, a decomposition
you will build on for months, a subsystem); a one-file change wants a mid-session
subagent instead"* — the same condition, in the owner, shipping to all nineteen
mandates. So the family-F verdict withholds no condition; the hub's copy is a
restatement in the strict sense, which is exactly what the pre-decided call
claims. Checked rather than trusted, because if it had been false the verdict
would have buried a live condition in a body.

### Families this inventory did not list — three applied, one decided

`shapes-k23` decided families **J** (no node for a shape), **I** (declare the
relationship) and **G′** (the grammar infers no relationship) against `SKILL.md`
twins **in this region**, taking its own earlier site as owner in each. Its
doubt 3 states the consequence: *"#10's two paragraphs must become bodies, or
every mandate carries each rule twice."* **All three are applied.**
`skill-no-node-for-a-shape`, `skill-declare-the-relationship` and
`skill-grammar-is-five-fields` are procedural, each rooted from the
`TASK-FORMAT.md` unit that owns its rule. Same-rule roots, not adjacent-subject
ones: `task-grammar-is-five-fields`'s closing sentence and
`skill-grammar-is-five-fields`'s are the same argument in the same words.

**G — the bare stem — applied, and `shapes-k23`'s dependency is discharged.** Its
doubt 2 states the stake: *"`SKILL.md`'s bare-stem paragraph has to be
`class=triggering`, or the rule ships nowhere"*, since `task-bare-stem-reasoning`
is the corpus-designated body (*"`TASK-FORMAT.md` carries the full reasoning"*).
`skill-bare-stem-rule` is **`triggering kinds=*`**, and it carries row 27's
address. The rule ships; the 3,957 B of reasoning does not.

**M — a chain's gap is asymmetric, and this is the one call this batch had to
make.** Sites: `TASK-FORMAT.md` §*What the shapes are not* bullets 3–4
(`task-chain-contiguity`, #3, earliest) and `SKILL.md` `**A chain is not
contiguous by construction…**` (mine). **I carved my site as the owner, against
the default's earliest-site rule, and the reason is that the default's owner is
already procedural.** `task-chain-contiguity` is `class=procedural`,
`task-review-chain-mechanics`'s bullet 4 (the one-line prescription) is
procedural, and `task-two-shapes` says nothing about gaps — so treating my site
as a body too would leave the rule *an integration must be cut adjacent, because
a gap silently corrupts its citations* shipping in **no mandate at all**. That is
the silent direction the node brief's asymmetry tie-breaker exists to settle, and
the failure is concrete: a review session reaches for `leaf-add`, the integration
lands after intervening work, and the finding coordinates it consumes point
somewhere slightly wrong with nothing erroring.

The split is the clean one this leaf predicted. `skill-chain-gap-asymmetry`
(triggering, 1,710 B of prose) is the diagnosis — *which hop a gap costs and
why* — and stops at the colon-and-bullets that state it. `skill-integration-
placement` (procedural, 1,604 B) is the prescription — the directory-local
condition, the fenced `leaf-insert` line and the three clarifying bullets. The
condition names the problem and defers the answer, which is *keep the `if`, defer
the `then`* in its most literal form anywhere in this batch.

**`**There is no exception to check.**` — triggering, and I read it twice as
asked.** The leaf offered it as 464 B of pure justification. It is not, and the
operative reading is the one the leaf flagged: *"Adjacency is unconditional
guidance"* plus *"a session that departs anyway owns the drift"* is a
**prohibition against a session acting on its own unverifiable reasoning** — and
the reasoning it forbids is exactly what a capable session will attempt
("the intervening leaf only touches one file, so my citations are safe"). The
paragraph's own middle explains why that attempt cannot succeed: the intervening
leaf has not run, and grove makes no leaf's eventual file set part of its
contract. Withheld, it leaves `skill-chain-gap-asymmetry` looking like a
risk to weigh rather than a rule to follow. The only other site is inside
procedural `task-chain-contiguity`, so this is family M's shape again: body here
means the door is never closed in any mandate. 464 bytes to close it.

### Edge inventory rows owned: 26, 27 and 32 — reported row by row

| row | source → target | outcome |
|---|---|---|
| 32 | `task-two-shapes` → `skill-cut-the-next-step` | **written** — this body's only root; without it the unit fails (R) |
| 26 | `skill-cut-the-next-step` → `task-review-chain-mechanics`, `task-vendor-pair-mechanics` | **written**, both members — both procedural, so the plan's condition holds |
| 27a | `skill-bare-stem-rule` → `task-bare-stem-reasoning` | **written** — `shapes-k23` pre-determined it writable and named the id |
| 27b | `skill-declare-the-relationship` → `task-declare-the-relationship` | **declined** — the target is `class=triggering`; a `defers=` naming it is a build error |
| 27c | `skill-grammar-is-five-fields` → `task-grammar-is-five-fields` | **declined** — same, and family G′ is what makes it triggering |

**Row 27 is reported in three halves because its two statements of the row
disagree**, and I could not tell which was meant without resolving both. The node
brief's inventory names the sources *bare stem* / *grammar is five fields*; this
leaf's own table names them *bare stem* / *declare the relationship*. `shapes-k23`
had already determined all three outcomes from the target side, so both readings
are reported rather than one chosen — 27a written, 27b and 27c declined as
illegal edges. That is the plan's own conditional resolving, not a gap. **Worth
the aggregate reviewer's note as a defect in the plan text**, since a batch that
read only one of the two tables would have reported the row complete while
leaving the other source unexamined.

**Row 26 has no filename citation to find it by**, which is the inventory earning
its keep on its own stated terms: `**Cut the next step…**` names no embedded file
except the `(`driving.md`)` mention, so the outbound sweep does not see the edge
at all. Same shape as `execute-k29`'s row 18. Note also that it is an
**addressing** edge and not a reachability one — `task-two-shapes` already defers
to both mechanics bodies at `kinds=*`, so dropping either member leaves
`cargo build` green. Reported member by member for that reason.

### Three inventory additions, all written, all required for (R)

Each is a genuine cross-file trigger→body edge whose later-carved endpoint is
mine, and each is the **only** root its body has. They are the direct mechanical
consequence of `shapes-k23`'s J / I / G′ calls, which the node brief's inventory
predates.

| addition | edge | why it is honest |
|---|---|---|
| A1 | `task-no-node-for-a-shape` → `skill-no-node-for-a-shape` | family J's owner addressing family J's restatement |
| A2 | `task-declare-the-relationship` → `skill-declare-the-relationship` | family I's owner addressing family I's restatement |
| A3 | `task-grammar-is-five-fields` → `skill-grammar-is-five-fields` | family G′'s owner addressing family G′'s restatement |

**These are same-rule roots, which is the test, but they are weaker than the
usual body-answers-condition shape** — see design finding 2. Each owner is
`triggering kinds=*`, so all three bodies are reachable from all nineteen
mandates.

### (D), (R), (T) at end of batch

- **(D)** — every one of the eight `defers=` members written names a declared
  `class=procedural` unit; the four `TASK-FORMAT.md` targets were read back from
  the listing (`task-review-chain-mechanics`, `task-vendor-pair-mechanics`,
  `task-bare-stem-reasoning`, `task-chain-contiguity` — all procedural, all with
  empty `defers=`).
- **(R)** — all five procedural units I created are reachable, each from a
  `triggering kinds=*` root: `skill-cut-the-next-step` from `task-two-shapes`
  (row 32); `skill-no-node-for-a-shape`, `skill-declare-the-relationship` and
  `skill-grammar-is-five-fields` from their owners (A1–A3);
  `skill-integration-placement` from `skill-chain-gap-asymmetry`.
- **(T)** — the deepest chain is two hops (`task-two-shapes` →
  `skill-cut-the-next-step` → the two mechanics bodies, which defer to nothing);
  every other chain is one hop into a unit with an empty `defers=`. No chain can
  re-enter a source: all four `TASK-FORMAT.md` sources are triggering and
  therefore illegal targets, and `skill-chain-gap-asymmetry` is triggering too.

### Sweeps: both run, both recorded

- **Outbound** — three embedded-file citations inside the region.
  `(`TASK-FORMAT.md` carries the full reasoning)` = row 27a, **written**;
  `(`TASK-FORMAT.md`)` on the declaration lines = row 27b, **declined**;
  `(`driving.md`)` on the in-session-reviewer sentence = **declined**, below.
- **Inbound** — `grep -rn 'SKILL\.md' content/` returns the same fourteen hits
  `execute-k29` enumerated, and **none of them points into this region**. They
  land in #1's spine (`driving.md` L266, L639, L695 — constraints 1 and 4), #9's
  `**Decompose.**` (L615, L674), #11's `**Retire.**` (`BRIEF-FORMAT.md` L40 and
  L89, `driving.md` L320 and L728), whole-file framing (`driving.md` L4), or are
  external-skill attribution comments (`grilling.md` L2, `driving.md` L40, L645,
  L690). No inbound edge to write; the three roots I needed came from the
  *target* side instead, as additions A1–A3.
- **Intra-file, the grep-invisible class** (rows 41–42's shape) — one hit:
  `skill-declare-the-relationship` cites *"constraint 3"*. Target is
  `skill-spine-constraints`, `class=triggering` — illegal, and a supporting
  citation rather than a trigger→body address. Nothing written.

**The `driving.md` mention, declined with the node brief's own reasoning.**
`**Cut the next step…**`'s *"may use its single in-session reviewer instead
(`driving.md`)"* is family A's **Mention**, and the brief says an edge from it is
legal but *"not an inventory obligation, so write it only if the reference reads
as a genuine trigger→body address."* It does not: the citing unit is itself a
body rather than a trigger, and `task-in-session-doubt-budget` (`triggering
kinds=*`) already carries `driving-doubting-inside-a-picked-leaf` into every
mandate. Writing it would add a second inbound path to an already-rooted body —
the shape the plan warns lets a dropped real edge stay green — for no address a
session lacks.

### Coverage, proved by reconstruction with the instrument controlled first

Following `research-moves-k25`'s vacuous-check trap, the strip was proved
non-blind before any comparison: `grep -v '^<!-- unit: '` removes 1,400 B from
the pre-batch `SKILL.md` and 2,051 B from the post-batch one.

- **The consumed `pending-skill-shapes`' prose is 34,563 B; my eight units plus
  `pending-skill-lifecycle`, marker lines stripped, are 34,563 B.** Byte-identical
  — no gap, no overlap at either boundary.
- **My eight units' prose is 10,068 B**, this leaf's planned region size to the
  byte, so the residual boundary is where the plan put it.
- **Both edited files with every `<!-- unit: ` line removed are byte-identical to
  their pre-batch selves** (`cmp` clean). No prose, filename or fence moved;
  trailing newline present; four fence lines in `SKILL.md`, all outside the
  region's marker positions.
- All nine markers are unindented whole lines at neutral fence state.

### Design findings for the aggregate review

**1 — the third unsplittable paragraph, and the leaf asked for the split.** This
leaf instructs: *"`**Every step of a shape carries the same bare stem**` is a
naming rule … *plus* a long justification of why the step suffix was deleted (not
a condition). **Split it.**"* **It cannot be split.** Every sentence boundary in
the paragraph falls mid-line — the rule ends at *"…does not restate the kind.**"*
inside post-marking L333, and the justification opens on the same line — so no
whole-line marker separates them, and this pass edits no prose. The whole
paragraph therefore takes one class, **triggering**, and 960 B ships where ~370 B
of rule would have. Same mechanism as `shapes-k23`'s F7 and the node brief's
L217–227 fusion; **the third instance, and the first where a leaf's explicit
instruction was defeated by it.** That is worth the reviewer's attention as a
pattern rather than three coincidences: the corpus's paragraphs are reflowed to
~78 columns, so a sentence boundary landing on a line boundary is chance at
roughly one in twelve, and the classification's grain is capped by that
everywhere. De-fusing is a prose edit for a later grove.

**2 — five of seven paragraphs in this region are the hub restating a rule
`TASK-FORMAT.md` owns, and the classification is now the evidence.** After this
batch, `skill-cut-the-next-step`, `skill-no-node-for-a-shape`,
`skill-declare-the-relationship`, `skill-grammar-is-five-fields` and (in the
opposite direction) `task-bare-stem-reasoning` are all bodies whose owner states
the same rule in another file. **Three of them —
A1–A3 — are near-verbatim duplicates rather than expansions**, and that makes
their roots weaker than the usual shape: a session that follows
`task-no-node-for-a-shape`'s `defers=` gets the rule it already holds, said again
in the hub's voice. The root is *on-topic* (same rule, same subject), which is
the test the plan sets and not an artificial root — but it is an address into
redundancy, and the honest repair is a **prose deletion**, not a re-marking.
Recorded here because mandate delivery is what surfaced it: reading the two files
as documents, the repetition looks like emphasis; reading them as a unit graph,
it is three edges that carry no information. This is the largest *deletable*
prose surface the classification has found so far, and it belongs to whoever owns
`SKILL.md`'s size after the successor grove ships.

**3 — `skill-integration-placement` opens with "So", the standing-alone defect
`shapes-k23` avoided by fusing.** Its doubt 5 refused to split
`task-chain-contiguity`'s bullets 3 and 4 for exactly this reason — bullet 4
opens *"**So** an integration is cut…"*. I made the equivalent split anyway,
because here the alternative is not a grain trade but family M's silent failure:
un-split, either the condition ships nowhere (procedural) or 3.3 kB of procedure
ships to nineteen mandates (triggering). The cost is real and bounded — the body
reads *"So an integration is cut where `pick` reaches it next, and the condition
for that is mechanical and directory-local: …"* and then restates the whole
condition in its own terms, so it is comprehensible standing alone; only the
opening conjunction dangles. **The two files now differ**: `TASK-FORMAT.md` keeps
one procedural unit and `SKILL.md` splits condition from procedure. That is
defensible (their owners differ), but the reviewer should confirm it rather than
find it.

**4 — no prose in this region resisted classification.** The node brief asks for
narrative that is neither condition nor procedure to be named rather than forced.
There is none here. `**There is no exception to check.**` was the candidate this
leaf flagged and it resolved as a genuine condition (family M, above); everything
else is a rule or the mechanics of one.

### The ids #11 and #12 need

Locatable by `grep -n '<!-- unit: <id>'` — no line numbers.

| batch | needs | which is |
|---|---|---|
| #11 `lifecycle-k31` | `pending-skill-lifecycle` | the residual it consumes; `triggering kinds=*`, no `defers=`, covering `**When a picked producer needs fresh review**` → EOF |
| #11 | row 37's source | `SKILL.md` `**Retire.**`'s pruning body, its own; `driving-prune-reorder-or-file-an-issue` is the target, #8's |
| #11 | family C's L550–554 mention | inside its own region; the owner `skill-adrs-and-specs` (#9) already names retirement |
| #12 `finish-cycle-k32` | rows 29–31, and `skill-adrs-and-specs` for row 30 | none of them touch anything this batch created |
| #11–#12 | all eight of my units | **complete as delivered — none is a target for any remaining inventory row.** The only thing either batch could want from here is `skill-chain-gap-asymmetry`'s address if a later region cites integration placement, and nothing does |

### Doubts to carry forward, by id

1. **`skill-chain-gap-asymmetry` / `skill-integration-placement` — family M is
   this batch's one un-pre-decided call, and it is the one to attack first.** I
   made `SKILL.md` the owner against the default's earliest-site rule, on the
   ground that `shapes-k23` had already made the earliest site procedural. **The
   reviewer should check the premise, not just the conclusion**: if
   `task-chain-contiguity` ought to have been triggering, the right repair is
   there and mine should be a body, and the two together currently ship 1,710 B
   where one of them should ship. If the premise holds, the remaining question is
   my boundary — whether the diagnosis really stops where I put it.
2. **`skill-no-exception-to-check` (532 B slice) — triggering by the asymmetry
   tie-breaker, and a genuine coin-flip on the leaf's own framing.** It is the
   only unit in the batch whose class I would call unforced. Read as
   justification it is 464 B of prose in nineteen mandates for nothing; read as a
   prohibition it closes a door `skill-chain-gap-asymmetry` leaves ajar. I went
   with the prohibition because the asymmetry says to. One marker to flip, and if
   flipped it needs a root — `skill-chain-gap-asymmetry` is the obvious one.
3. **`skill-cut-the-next-step` (3,387 B of prose, one unit) — the grain.** Kept
   whole on the colon-and-list rule. The two shape bullets are separately
   addressable in `TASK-FORMAT.md` (`task-review-chain-mechanics` /
   `task-vendor-pair-mechanics`) and not here, so a session wanting only the pair
   fetches 3.5 kB of chain first. It ships in no mandate, so the cost is a
   fetch's size rather than a mandate's; splitting is two markers and no prose
   edit if the reviewer wants parity with `TASK-FORMAT.md`'s grain.
4. **A1–A3 — the three near-duplicate bodies** (finding 2). Not a doubt about the
   class — triggering would duplicate three rules in nineteen mandates, which
   `shapes-k23`'s doubt 3 already rules out — but about whether an edge into a
   restatement is an address worth having at all. If the reviewer's answer is
   "delete the prose instead", these three edges and three units go with it, and
   that is a prose change for the successor grove rather than a re-marking.
5. **Row 27's two incompatible statements in the plan** (reported above). Not a
   doubt about my outcome — all three halves are determined by target class — but
   the reviewer should decide whether other rows carry the same divergence between
   the node brief's inventory and a child body's restatement of it.
