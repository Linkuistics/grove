# finish-cycle-k32

## Goal

Classify the **last of the corpus** (11,621 bytes) and close the classification:

- `content/SKILL.md` **from `**Finish.**` to end of file** (baseline L610–760,
  10,783 bytes) — `**Finish.**` with its three numbered steps,
  `**Resume is state-checked, never a marker file**`, `**Ending after step 2 but
  before step 3…**`, `## Artifacts`, `## Specs`, `## Reference files` and the
  `linkuistics` prerequisite note.
- `content/prompts/continue.md` **whole** (838 bytes).

This is batch 12 of 12. Its **last act is to cut the aggregate `review-impl`
leaf** the node brief requires.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L610–760 is a baseline coordinate — four batches have
  inserted markers above your region by now.** Carve from `**Finish.**` to end of
  file, consuming `pending-skill-finish` in full. Yours is the one `SKILL.md` region
  the F7 separator correction does not touch: it ends at EOF, and the blank line
  before `**Finish.**` belongs to `lifecycle-k31`.
- **There is nothing to inherit from `pending-skill-finish`.** A residual never
  carries `defers=`.
- Carve `content/prompts/continue.md`. It is the launcher framing and it is **all
  triggering** — but it is **not** edge-free: it is the root for `## Reference files`
  (row 31, below). The node brief says to leave its text alone; that still holds.
  One or two units at most.
- **Mint no residual.** After this batch,
  `grep -rc '<!-- unit: pending-' content/` must return **0**.

### `## Reference files` — settled, and no longer your call

`batches-k13` left this to you as a free choice between eight unconditional
`defers=` members and none. `batches-k33` F6 found that a false binary — both options
evade the classification question — and the node brief settles it:

**`## Reference files` (L735–744) is `class=procedural`, rooted from
`prompts/continue.md`'s framing unit (row 31), and it writes no `defers=` of its
own.** The decisive argument is neither size nor redundancy: **the index's rows name
files, and a session cannot fetch a file.** `grove-llm methodology` addresses units
by id, so an index of filenames delivered into a mandate promises navigation the
delivery path cannot honour — while every genuine trigger→body edge for those guides
was written at its point of use by batches #4–#12 anyway.

Its eight filename mentions are a **standing sweep exclusion**: they will show up in
every `grep -rn '<F>' content/` and none of them is a trigger→body edge.

**Carry it to the aggregate review as a design finding**, which the node brief asks
for in these words: the index is narrative residue of the provisioned-skill era — it
exists so a reader of a skill *directory* knows what sits beside `SKILL.md`, and
mandate delivery replaces that job with `grove-llm methodology`'s listing. Whether it
survives at all is the successor grove's call.

**The `linkuistics` prerequisite note (L746–760) is a separate unit,
`class=triggering kinds=*`, with no `defers=`.** It states a genuine condition — a
session raising an ADR, sketching a spec's seams, or driving a jj-enabled tree should
consult the matching plugin skill — and its three targets are not embedded, so none
can be a `defers=` target. That the note is unfollowable from the embed is a fact
about the design, not a defect; if you think it reads as a dangling promise, that is
a **finding** for your body.

### `prompts/continue.md`'s Decompose reference is deliberately edgeless

`batches-k33` F3 read `continue.md` L6 — *"see the skill's Decompose step"* — as a
lost edge, on the grounds that it is a cross-file trigger→body reference the filename
grep cannot find. **The mechanism is real and is why the edge inventory exists; this
particular reference owes nothing.** Its target is `SKILL.md` `**Decompose.**`, which
`execute-k29` carves as family B's **owner** — a triggering unit. A `defers=` naming a
triggering unit is a build error, and none is needed: the condition ships in every
mandate. Record that reasoning rather than writing the edge.

The same test applies to `continue.md`'s other references (*"use the grove skill"*,
`grove-llm complete`, the handle rule): a reference to a condition needs no address.

### Edge inventory rows owned: 29, 30 and 31

| row | source | target | note |
|---|---|---|---|
| 29 | `## Artifacts`'s glossary paragraph (L707–713, *Keep it a glossary and nothing else*) | `CONTEXT-FORMAT.md` bodies | `guides-k24` carved the target; `guides-k24` reported this hit as *not yours* |
| 30 | `SKILL.md` L217–227 (family C/D/E owner, `execute-k29`) | your `## Specs` body | not optional; its only root |
| 31 | `prompts/continue.md`'s framing unit | your `## Reference files` body | not optional; its only root |

L703's *philosophy per `linkuistics:decision-records`* is **not embedded** and can
never be a `defers=` target.

### The judgement this batch exists for

- **`**Finish.**`** is HITL and states the loop's **only routine human gate**:
  the session proposes teardown and waits for explicit confirmation, and a
  headless run reports and stops. That is triggering and cannot be anywhere else.
  The three numbered steps — `finish-commit`, the `Recovery pending` handling,
  `complete --done` — are procedural, and `kinds=finish` is a genuine candidate
  scope for them, which makes this one of the very few places an explicit scope
  list is honest. But read `**Finish.**`'s opening again before scoping: *"You do
  not discover that a grove is finished — the driver does"* is addressed to every
  kind, because it tells a non-`finish` session **not** to go looking.
- **`## Artifacts`** is the four-row table plus *the glossary is load-bearing*.
  The glossary paragraph states terminology drift as the acute failure mode of
  multi-session work — a condition every session needs.
- **`## Specs`** (L720–733) restates `SPEC-FORMAT.md`'s membership and grain rules
  more tersely. **Settled: it is a family-D body, `class=procedural`**, rooted from
  `SKILL.md` L217–227 (row 30). Its opening sentence restates the condition
  ("produced lazily by a `design` task *when the increment is a genuine agreement
  point*"), which ships from three other places — the owner, `TASK-FORMAT.md`'s
  `design` bullet, and `SPEC-FORMAT.md`'s opening — so do not carve it out as a
  fourth. Its *grain* is still yours.
- **`prompts/continue.md`** is the launcher the driver prepends to every mandate
  today. Under the successor grove it becomes `content/MANDATE.md`'s framing unit,
  but that is not this grove's work — classify it as it stands. Note that its framing
  unit is the root for `## Reference files`, which is exactly the job
  `MANDATE.md`'s framing unit inherits: *here is what you are holding, and here is how
  the rest is served*.

## Done when

- `content/SKILL.md` from `**Finish.**` to end of file, and
  `content/prompts/continue.md`, are subdivided into real units.
- **Rows 29, 30 and 31 are each reported** — 30 and 31 written (they are their
  targets' only roots), 29 written or declined with a reason.
- The `## Reference files` and `## Specs` verdicts are applied as settled, and the
  `## Reference files` design finding is carried into the aggregate review handoff.
- **`grep -rc '<!-- unit: pending-' content/` returns 0.** Run it and paste the
  result into the commit message; it is the mechanical statement that the
  classification is finished rather than merely green.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately and
  the last `pending-` id removed.
- **`grove-llm methodology` is verified out of a rebuilt, installed binary**: the
  listing shows the real classification, and spot-fetching a triggering unit shows
  a `defers=` target that answers it. This is the node brief's `Done when` and
  this batch is where it is checked — the eleven batches before it verified
  through the module seam.
- **The full edge inventory is assembled from all twelve leaf bodies** — every row
  written, declined with a reason, or added by a batch that found an unlisted edge —
  and carried into the review handoff. This is assembly, not reconstruction: each
  batch reported its own rows.
- **The aggregate `review-impl` leaf is cut** — see below.

## The aggregate review, which is this leaf's last act

The node brief requires it, and it is not optional in practice: this
classification is the artifact the successor grove's composer and golden snapshots
are built on, and a misclassification that survives is baked in behind bytes that
look stable.

Cut it beside this leaf:

```
grove-llm leaf-add classification-k9 classification --kind review-impl
```

`leaf-add` is correct here — a `review-*` step re-derives its citations from the
producer's commit, so it needs no `leaf-insert` care, and no sibling entry
after this leaf holds live work.

**Write its body yourself, and give it six things:**

1. **`**Reviews:**`** naming all twelve batch handles — `spine-k21`, `kinds-k22`,
   `shapes-k23`, `guides-k24`, `research-moves-k25`, `evidence-moves-k26`,
   `doubt-moves-k27`, `decompose-moves-k28`, `execute-k29`, `shape-cutting-k30`,
   `lifecycle-k31`, `finish-cycle-k32` — so the reviewer inspects the **whole
   classification** rather than only the closing commit.
2. **The pre-classification baseline commit**, by id. It is the commit that retires
   **`batches-k34`** — the last commit before `spine-k21` touches `content/`. **Not
   `batches-k13`'s**, which `batches-k13` itself named: `batches-k33` and
   `batches-k34` both land after it, so a diff from there carries planning churn.
   The corpus bytes are identical across all three, so this is about giving the
   reviewer a clean diff, not about which bytes were classified. Resolve it (`jj log`
   for the commit whose description names `batches-k34`) and write the actual change
   id into the body, not the handle alone.
3. **The assembled doubts**, by unit id, gathered from all twelve leaf bodies.
   Every batch was asked to record what it was least sure about precisely so this
   step is assembly rather than reconstruction. Group them by the kind of doubt —
   scope calls, condition/body splits, and any prose flagged as neither condition nor
   procedure. **The six repeated-rule families are a different kind of item now**: no
   batch decided them, so what the reviewer needs is not twelve calls to reconcile
   but the *pre-decided* verdicts plus every note a batch recorded that made a verdict
   look wrong with the prose open.
4. **The assembled edge inventory** — all thirty-plus rows with their outcomes, and
   any row a batch added. The reviewer's question there is the one no build asks: does
   each written edge address a body its source's condition actually raises, and does
   each *declined* row deserve its decline?
5. **What the build cannot check**, stated plainly, because that is what the
   reviewer is for: whether each unit **reads correctly standing alone** (the
   fence half is mechanical; the prose half is not), and whether a triggering
   condition was misfiled as procedural — the silent direction, which yields an
   unasked question and no diff.
6. **The three design findings this plan already knows about**, so the reviewer
   adjudicates them rather than rediscovering them: the `SKILL.md` L217–227 **fusion**
   (four rules in one unsplittable paragraph, which is why one unit owns families C, D
   and E), the **`## Reference files` index** as narrative residue of the
   provisioned-skill era, and whatever the twelve batches flagged as neither condition
   nor procedure.

## Notes

- If the twelve batches surfaced prose that is **neither a condition nor a
  procedure** — narrative that exists only to make the document readable — collect
  those findings here too. That is a finding about the *design*, and the review
  leaf is where it gets adjudicated.
- Do **not** retire `classification-k9` yourself. Retiring this leaf leaves the
  review leaf live, so the node stays open and the cascade does not fire — which
  is correct: the node's `Done when` includes the review having run.

## Batch record

**Eleven real units minted in `SKILL.md`, one in `prompts/continue.md`;
`pending-skill-finish` and the `continue` seed both consumed; no residual minted.**
`EMBEDDED_UNITS` 133 → 143. `cargo build` green; `cargo test` green — 0 failures in
every suite, including `the_embedded_unit_set_is_pinned_complete`.

**The classification is complete.** `grep -rc '<!-- unit: pending-' content/`
returns 0 on every file, and — per `lifecycle-k31`'s warning that the check is not
sufficient alone — **all nine seed ids are gone too**
(`grep -rhoE '<!-- unit: (skill|task-format|driving|grilling|spec-format|brief-format|context-format|adr-format|continue) '`
returns nothing). Corpus: **143 units, 68 triggering and 75 procedural.**

**Anchor executed against:** `**Finish.**` → EOF, re-verified unique. Zero fences in
the region, so every marker sits at neutral state without a judgement call. The
baseline offset was exactly **37 lines** at every checkpoint (baseline L610 →
current L647, L720 → L757, L735 → L772), so the plan's coordinates agreed with the
anchor throughout and no departure is reported.

### The units, in document order

Slice bytes are marker-line-inclusive — what a mandate actually carries — fetched
through the **installed** `grove-llm methodology <id>` rather than measured by hand.

| id | class | scope | slice | prose | defers |
|---|---|---|---|---|---|
| `skill-finish` | triggering | `*` | 1,476 | 1,320 | the four bodies below |
| `skill-finish-steps` | **procedural** | — | 2,535 | 2,484 | — |
| `skill-finish-nothing-after` | **procedural** | — | 357 | 298 | — |
| `skill-finish-resume` | **procedural** | — | 880 | 828 | — |
| `skill-finish-no-signal-stop` | **procedural** | — | 1,178 | 1,118 | — |
| `skill-artifacts` | triggering | `*` | 816 | 760 | — |
| `skill-glossary-is-load-bearing` | triggering | `*` | 576 | 480 | `context-structure` (row 29) |
| `skill-briefs-vs-glossary` | triggering | `*` | 311 | 246 | — |
| `skill-specs` | **procedural** | — | 953 | 863 | `spec-suggested-shape`, `spec-test-seams` (addition A1) |
| `skill-reference-files` | **procedural** | — | 1,315 | 1,261 | — |
| `skill-linkuistics-prerequisite` | triggering | `*` | 1,196 | 1,125 | — |
| `continue-launcher-framing` | triggering | `*` | 884 | 789 | `skill-reference-files` (row 31) |

Two markers edited in place under the decoupling lemma: `skill-adrs-and-specs`
(#9) gains `skill-specs` (**row 30**, its ninth member).

**Per-mandate effect: every one of the nineteen mandates loses 6,423 bytes.**
Before, `pending-skill-finish` shipped 10,844 B as `triggering kinds=*`; after,
4,375 B of real conditions ship and 7,218 B is addressable procedure, against 46 B
of growth in `continue.md`'s marker (838 → 884, the `defers=` it gains). That is
the second-largest saving of any batch, behind `lifecycle-k31`'s 9,278.

### Coverage proved by reconstruction, instrument controlled first

Following `research-moves-k25`'s vacuous-check trap, the strip was proved non-blind
before any comparison: `grep -v '^<!-- unit: '` removes 3,117 B from the pre-batch
`SKILL.md` and 3,878 B from the post-batch one.

- **The consumed `pending-skill-finish`'s prose is 10,783 B; my eleven `SKILL.md`
  units, marker lines stripped, are 10,783 B.** Byte-identical — no gap, no overlap,
  and it is `lifecycle-k31`'s recorded figure to the byte.
- **Both edited files with every `<!-- unit: ` line removed are byte-identical to
  their pre-batch selves** (`cmp` clean, 52,277 B and 837 B). No prose, filename or
  fence moved; trailing newline present.
- All 143 corpus markers match `^<!-- unit: .* -->$` — unindented whole lines, every
  one at neutral fence state — and the count equals the listing's row count exactly.

### Edge inventory rows owned: 29, 30 and 31 — all three written

| row | source → target | outcome |
|---|---|---|
| 29 | `skill-glossary-is-load-bearing` → `context-structure` | **written** — to the file's entry, `guides-k24`'s rows 2/5 precedent |
| 30 | `skill-adrs-and-specs` → `skill-specs` | **written** — this body's only root; without it the unit fails (R) |
| 31 | `continue-launcher-framing` → `skill-reference-files` | **written** — likewise its only root |

**Row 29 targets `context-structure`, not the sections.** The prose is a bare file
citation — *"no implementation detail (`CONTEXT-FORMAT.md`)"* — and `guides-k24`
answers those with the file's entry unit, which is procedural and chains to
`context-rules` and `context-single-vs-multi-repos`. Note the consequence
`guides-k24`'s doubt 2 asked to be confirmed: `context-structure` now has **three**
inbound edges (rows 4, 6 and 29). All three are listed inventory rows, which is the
node brief's stated safety condition, but this is the corpus's third-highest
in-degree and the reviewer should confirm the condition is doing real work rather
than laundering redundancy.

**Inventory addition — one.**

| # | source → target | why |
|---|---|---|
| A1 | `skill-specs` → `spec-suggested-shape`, `spec-test-seams` | `## Specs` closes *"Shape and the seam-sketching rule: `SPEC-FORMAT.md`."* — a genuine "the rest is over there", and the one sentence in my region that promises a procedure it does not carry. Unlisted because family D's table records `SKILL.md` L704 and L741 as its mentions and never reached L733. Both members are named because the sentence names two things; `spec-suggested-shape` chains to `spec-test-seams` anyway, so the second is an address rather than a reachability need |

**A1 is a procedural→procedural hop**, the same shape as `guides-k24`'s A1, and it
is the one edge in this batch a reviewer could cut without breaking anything.

**Six references declined**, each recorded with its reason rather than silently not
written:

| candidate | reason |
|---|---|
| `continue-launcher-framing` → `skill-decompose` (*"see the skill's Decompose step"*) | **the pre-decided edgeless case.** Target is family B's **owner**, `class=triggering` — an illegal `defers=`, and none is needed: the condition ships in every mandate. `batches-k33` F3 read this as a lost edge; the mechanism is real and is why the inventory exists, but this reference owes nothing |
| `skill-artifacts`'s three table cells (`CONTEXT.md`, `linkuistics:decision-records`, `SPEC-FORMAT.md`) | **pre-decided mentions** (family C's L703, family D's L704). The first two name artifacts and a non-embedded plugin skill; the third is an index row |
| `skill-briefs-vs-glossary` → `BRIEF-FORMAT.md` bodies | a **mention** — it names the *artifact* (*"a node that carries anything carries a `BRIEF.md`, not a glossary"*), not a procedure. `BRIEF-FORMAT.md` is already reached by rows 14, 16, 19 and 28 |
| `skill-reference-files`'s eight filename rows | the **standing sweep exclusion** the node brief settles. The index's rows name files and a session cannot fetch a file |
| `skill-linkuistics-prerequisite` → its three plugin skills | **not embedded**; no id exists to name |
| `skill-linkuistics-prerequisite` → `adr-placement-note` / `spec-suggested-shape` (*"`ADR-FORMAT.md` and `SPEC-FORMAT.md` keep only grove's placement and recording conventions"*) | a **claim about the files**, not an instruction to fetch them. Both are already addressed from `skill-adrs-and-specs` at `kinds=*` |

### The sweeps

- **Outbound**, over `**Finish.**` → EOF and `continue.md` whole — fifteen
  embedded-file citations. Two are edges (row 29, addition A1); eight are the
  `## Reference files` index (standing exclusion); five are the mentions and
  non-embedded targets declined above. Nothing unlisted.
- **Inbound** — `grep -rn 'SKILL\.md' content/` returns the same fourteen hits the
  last three batches enumerated, and **none points into this region**: they land in
  #1's spine, #9's `**Decompose.**`, #11's `**Retire.**`, whole-file framing, or are
  external-skill attribution comments. `grep -rn 'continue\.md' content/` returns
  one hit — `SKILL.md`'s own `## Reference files` row, inside my own region and a
  standing exclusion besides.
- **Intra-file, the grep-invisible class** (rows 41–42's shape) — one candidate,
  `skill-finish`'s *"the **Finish** cycle below"* forward reference from
  `skill-signal` (#11). Target is `skill-finish`, `class=triggering` — illegal, and
  the condition ships in every mandate anyway.

### (D), (R), (T) at end of batch

- **(D)** — all seven `defers=` members written name declared `class=procedural`
  units, every one read back from the **installed** binary's listing.
- **(R)** — all six procedural units I created are reachable from a `triggering
  kinds=*` root: the four finish bodies from `skill-finish`; `skill-specs` from
  `skill-adrs-and-specs` (row 30); `skill-reference-files` from
  `continue-launcher-framing` (row 31).
- **(T)** — the deepest chain is two hops (`skill-adrs-and-specs` → `skill-specs` →
  `spec-suggested-shape` → its four terminal children, three hops at most). No chain
  can re-enter: `skill-finish` and `continue-launcher-framing` are triggering and
  therefore illegal targets, and the whole-embed gate confirms it.

### The pre-decided verdicts, applied not re-decided

All four landed as the node brief settled them, and none looked wrong with the prose
open.

- **`## Reference files` → `class=procedural`, rooted from `continue.md`'s framing,
  writing no `defers=`.** Applied. The decisive argument held up with the file
  open: the index's rows name *files*, and `grove-llm methodology` addresses units by
  id. See design finding 2 for what I would add to it.
- **The `linkuistics` prerequisite note → `class=triggering kinds=*`, no `defers=`.**
  Applied. It states a genuine condition and its three targets are unembeddable.
- **`## Specs` → family-D **body**, `class=procedural`, rooted by row 30.** Applied,
  and its opening condition was **not** carved out as a fourth statement, exactly as
  instructed.
- **`skill-finish` → `kinds=*`, not `kinds=finish`.** Applied on the leaf's own
  reasoning: *"You do not discover that a grove is finished — the driver does"* is
  addressed to every kind, because it tells a non-`finish` session **not** to go
  looking. The three numbered steps are procedural, so the `kinds=finish` scope the
  leaf floated for them is not merely unnecessary but **ungrammatical** — `kinds=` is
  forbidden on a procedural unit. Reachability does the scoping instead, and does it
  better: the steps are reached only by a session that fetched them.

### Doubts, by id — carried into the aggregate review

1. **`skill-finish-nothing-after` (298 B) — the weakest class call in the batch.**
   *"Nothing after: integrating the grove's branch … are **not** grove workflow"* is
   a **scope prohibition**, which is the triggering shape, and I made it procedural.
   The reasoning: only a session running the finish cycle is at the point where
   integration is conceivable, and that session holds `skill-finish`, whose marker
   names this unit. So withholding it costs a lookup the session knows to make. If
   the reviewer reads "don't do X" as always-triggering, this is the one marker to
   flip, and it is 298 B.
2. **`skill-finish-no-signal-stop` (1,118 B) — procedural, and it is closer to
   narrative than anything else I classified.** Most of it describes what the
   *driver* and the *next invocation* do. Its procedural payload is genuine but
   negative — *do not treat an interrupted finish as pending, and do not kill the
   30-second guard* — and a reader could fairly call it explanation. Recorded rather
   than forced; see design finding 3.
3. **`skill-artifacts` (760 B) — triggering, and the call was close.** Against it:
   every path it names is reachable from a condition that already ships
   (`adr-placement-note` from `skill-adrs-and-specs`, the glossary from my own next
   unit). For it: this is the corpus's only statement of *which four artifacts exist
   and which of them is ephemeral*, and a session that must decide where a durable
   fact goes cannot ask for an index it does not know exists. The asymmetry decided
   it. One marker to flip, and it would then need a root.
4. **`skill-briefs-vs-glossary` (246 B) — kept separate from the glossary unit.**
   Both are `triggering kinds=*`, so under `spine-k21`'s inherited rule 3 (*split
   finer only when the split changes a class or a scope*) they arguably should be one
   unit. I split them because the glossary unit carries row 29's `defers=` and this
   one states a different rule (domain partition vs process partition). It is the
   batch's clearest instance of a split that buys an address nothing uses. One marker
   to delete.
5. **`skill-finish` / `skill-finish-steps` — the dangling colon, third instance.**
   The condition ends *"On confirmation, run:"* and the three steps are deferred.
   `decompose-moves-k28`'s doubt 2 and `evidence-moves-k26`'s rejected boundary are
   the precedents, and I take `decompose-moves-k28`'s line: under mandate delivery
   the colon's referent is the `defers=` on the same marker. This is the corpus's
   **cleanest** demonstration of that claim — verified out of the installed binary —
   and if the reviewer rejects it here, the alternative is 2,484 B of finish
   mechanics in all nineteen mandates.

### Design findings for the aggregate review

**1 — the finish region is the corpus's densest run of unsplittable paragraphs, and
it is the seventh through ninth instance of the systemic pattern.** `**Finish.**`'s
framing (1,320 B) states **three** things — *the driver discovers it, not you*; the
sentinel's resumability and non-preemption mechanics; and the loop's only routine
human gate — with no line-aligned boundary between any of them, so one class covers
all three. The three numbered steps are likewise one unit: a marker between items 2
and 3 would restart the rendered list, and there is no blank line to sit on.
`shape-cutting-k30`'s reflow arithmetic (~78 columns, so a sentence boundary lands
on a line boundary roughly one time in twelve) explains it, and `lifecycle-k31`'s
finding 2 already called it systemic. **This batch closes the corpus-wide count: the
pattern appears in every `SKILL.md` batch and in `TASK-FORMAT.md`, and the honest
repair is a reflow pass in a later grove**, which would be mechanical and reviewable
and would raise the achievable grain everywhere.

**2 — `## Reference files` is narrative residue, and the successor grove makes it
doubly so.** Carried forward as the node brief asks, with one addition it does not
make: the index's **root** expires at the same moment its content does.
`continue-launcher-framing` opens *"use the grove skill"*, which the design records
as becoming false when provisioning retires — so under the successor grove the index
names files of a skill directory that no longer exists, rooted from a sentence that
is no longer true. Whether it survives is the successor's call; this classification
should not make it look load-bearing. **It joins `driving.md`'s `## In this guide`
anchors and `## The shortest version` as the fourth member of the species
`decompose-moves-k28` named** — narrative addressed to a reader of a *file*, in a
corpus that no longer delivers files — and the aggregate review is where that stops
being four separate observations and becomes one recommendation.

**3 — prose that is neither condition nor procedure: one instance, and it is
`skill-finish-no-signal-stop`'s second half.** *"Likewise if the driver dies after
the commit, the `done` it never got to read is coordination debris…"* through the
30-second guard is **mechanism description addressed to a reader reasoning about the
design**, not to a session acting. It rides in a procedural unit and ships nowhere,
which is the harmless direction. Recorded because the node brief asks for it to be
said rather than forced, and because it is the first instance found in `SKILL.md`
itself rather than in `driving.md`.

**4 — the region contains the design's own best worked example, and it is worth
keeping.** `skill-finish` (`triggering *`, ending mid-sentence at *"On confirmation,
run:"*) and `skill-finish-steps` (`procedural`, beginning *"1. **Promote**…"*) are
*keep the `if`, defer the `then`* in its most literal form anywhere in the corpus,
and the pair was verified end-to-end out of a rebuilt, installed binary. If the
successor grove wants a golden snapshot that demonstrates the mechanism to a human,
this is the pair to use.
