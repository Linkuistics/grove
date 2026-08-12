# execute-k29

## Goal

Classify **`content/SKILL.md` from `**Execute.**` to the line before `**Cut the next
step, when it is needed.**`** (baseline L167–246, 5,455 bytes): `**Execute.**` and
its three bullets, `**Review ownership inside a picked leaf.**`, the *"Whichever
kind is running"* paragraph, and `**Decompose.**` with its two bullets.

This is batch 9 of 12. It is the **smallest region in the plan and by far the densest
in cross-file deferral** — six of the nine embedded files are named in these
80 lines, it owns **nine inventory rows**, and it carries the **owner** of four of the
six repeated-rule families.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L167–246 is a baseline coordinate — and by the time
  you open the file, batch #1 has inserted markers above it.** Carve from
  `**Execute.**` to the line **before** `**Cut the next step, when it is needed.**`,
  consuming the front of `pending-skill-loop`. Your region ends **including** the
  blank separator line before that anchor, per the marker-placement convention; that
  is the one-byte correction `batches-k33` F7 found (your baseline range is L167–246,
  not L167–245, and your size is 5,455 bytes).
- Mint exactly one residual, **`pending-skill-shapes`**, covering `**Cut the next
  step, when it is needed.**` to end of file, as `class=triggering kinds=*` **with no
  `defers=`**.
- **There is nothing to inherit from `pending-skill-loop`.** No batch parked an edge
  on it: a residual never carries `defers=`, and no edge may have a `pending-*`
  source. Batches 4–8 reported their `SKILL.md` hits as *not yours* instead, and
  those hits are the inventory rows below.

### The pre-decided calls in this region — four family owners and one body

This region is where the corpus's hub states four of the six repeated rules, and the
node brief settles every one of them. Apply the verdicts; do not re-decide.

- **Family B owner — `**Decompose.**` (L229–245).** **Triggering, `kinds=*`.** The
  corpus designates it: `driving.md` L595 reads *"`SKILL.md`'s Decompose step states
  the rule; this is the habit that honours it."* Its restatement in
  `driving.md` §*Externalizing surfaced work* is the **body**, and **row 18 is
  yours** — the second address into a unit `decompose-moves-k28` already rooted from
  `driving.md`'s framing unit.
- **Families C, D and E owner — the *"Whichever kind is running"* paragraph
  (L217–227).** **Triggering, `kinds=*`**, and **kept whole**. `batches-k13` told you
  to consider splitting it; `batches-k34` established that you cannot. It states four
  rules — raise ADRs sparingly, write a spec only at an agreement point, the ADR set
  is current-state, and the same rule governs `docs/specs/` — and the sentence
  boundaries fall **mid-line** (baseline L220 carries the end of the spec clause and
  the start of the ADR-set clause). Markers are whole unindented lines and this pass
  edits no prose, so the paragraph takes **one** class for all four rules. It is
  triggering because two of the four ship nowhere else at `kinds=*`: withhold *raise
  ADRs sparingly* and *write a spec at an agreement point* from the eighteen non-
  `design` kinds and you have manufactured an unasked question.
  **Record the fusion as a design finding in your body.** The grain of the
  classification is bounded by line boundaries here, and de-fusing the paragraph is a
  prose edit for a later grove, not a marking decision. It is the reason rows 20–22
  all leave from one unit.
- **Family A body — `**Review ownership inside a picked leaf.**` (L198–215).**
  **Procedural.** The owner is `TASK-FORMAT.md` §*In-session doubt is budgeted across
  the whole picked leaf* (#2), which carries the predicate and the five-row allowance
  table. **Row 17 is yours, and it is this paragraph's only root** — without it the
  unit fails reachability at the end of your batch.
  This paragraph states the predicate's **negative half** (*a `.grove/` directory in
  the checkout and inherited Grove control variables do not count*) that the owner
  does not. #2 and #7 both recorded that; add your own note so the aggregate reviewer
  sees it from all three sides.
  Its tail (L211–215, the escalated-review routing note) is about review target
  policy rather than the budget. Splitting it off is your judgement, not the family
  call; if you do, decide its class on its own merits.
- **`**Execute.**` (L167–196)** carries no family site. Triggering, `kinds=*` — the
  filename states the kind, and the closed set has nineteen members. Its three
  bullets describe *other* kinds' disciplines, which `kinds-k22` also states per
  kind; check its body for duplication before carving a second statement, and where
  the per-kind prose there ended up **procedural**, rows 24–25 are the addresses.

### Edge inventory rows owned: 17–25

Every target exists — `decompose-moves-k28` finished the last of them and said so in
its commit message.

| row | source (in your region) | target | note |
|---|---|---|---|
| 17 | `TASK-FORMAT.md` family-A owner | your *Review ownership* body | not optional; its only root |
| 18 | your `**Decompose.**` owner | `driving.md` §*Externalizing surfaced work* | second address into an already-rooted body |
| 19 | your `**Decompose.**` owner | `BRIEF-FORMAT.md` bodies (L237–238), `driving.md` §*What a good child leaf looks like* (L240) | |
| 20 | your L217–227 owner | `ADR-FORMAT.md` bodies (L217–218) | |
| 21 | your L217–227 owner | `SPEC-FORMAT.md` §*current-state* / membership / grain (L219–220) | |
| 22 | your L217–227 owner | `driving.md` §*Reworking ADRs and briefs…* (L220–224) | read `evidence-moves-k26`'s body for the id |
| 23 | your L217–227 owner (L224–227, *See `driving.md` for the field-guide habits…*) | `driving.md` grilling-moves bodies | the hub-side catch-all into `driving.md` |
| 24 | `**Execute.**` (L171–172) | `TASK-FORMAT.md` per-kind discipline bodies | conditional on `kinds-k22` having made any of them procedural; decline with that reason otherwise |
| 25 | `**Execute.**`'s `requirements` bullet (L176–179) | `grilling.md` bodies | |

Rows 20–23 all leave from the **same** unit, so its `defers=` is a multi-member
quoted list. That is legal and is the direct consequence of the fusion above.

### Scope

Nothing in this region is honestly narrower than `kinds=*`. `**Execute.**`'s
bullets describe per-kind discipline but are addressed to *every* session
deciding what a leaf of some kind will do — resist the pull to scope them, and if
you do scope one, say why. Remember there is no family shorthand: "every producer"
is spelled out in full or is `*`.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-skill-shapes` covers the rest of the file and nothing else, and carries
  no `defers=`.
- **Rows 17–25 are each reported** — written, or declined with a reason recorded in
  this leaf's body. Rows 18 and 19 target bodies that already have another inbound
  path, so the build stays green if either is dropped; the inventory is the net.
- `cargo build` and `cargo test` are green — this is the batch most likely to
  provoke the reachability, class and termination checks, because it writes the
  most edges.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The **fusion finding** on L217–227 is recorded, and the ids of the four family
  owners/bodies in this region are named for #10, #11 and #12.

## Notes

- 5,455 bytes is small for a batch by design. The cost here is the six-file edge
  set and four family owners, not the prose.
  **Do not absorb `pending-skill-shapes`** to fill the session — `shape-cutting-k30`
  has its own rows to reconcile.
- Doubts to carry forward, by id. You no longer *make* the overlap calls, so what
  belongs in the handoff is different and sharper: where a pre-decided verdict looked
  wrong once you had the prose open, and the fusion finding.

## Batch record

**Four real units minted, one residual, `pending-skill-loop` consumed.**
`EMBEDDED_UNITS` 108 → 112. `cargo build` green; `cargo test` green — 40 test
binaries, **1,023 tests, 0 failures**, including
`the_embedded_unit_set_is_pinned_complete`. Only `content/SKILL.md` still carries a
`pending-` unit (`grep -rc '<!-- unit: pending-' content/` → the single line
`content/SKILL.md:1`), and `content/prompts/continue.md` is still a seed residual.

**Anchors executed against, both verified unique** (`grep -Fc` returned 1 for each,
after marking): `**Execute.**` and `**Cut the next step, when it is needed.**`. **The
baseline range agreed with the anchors exactly** — pre-batch L179–258 (baseline
L167–246 plus batch #1's twelve inserted markers) measures **5,455 bytes**, this
leaf's figure to the byte, including the one-byte blank separator `batches-k33` F7
added. No disagreement to report and no departure from the coordinates.

### The units, in document order

Slice bytes are marker-line-inclusive — what a mandate actually carries — fetched
through `./target/debug/grove-llm methodology <id>` rather than measured by hand.

| id | region (pre-batch) | class | scope | slice | prose | marker |
|---|---|---|---|---|---|---|
| `skill-execute` | L179–209: `**Execute.**` + its three bullets | triggering | `*` | 2,244 | 2,162 | 82 |
| `skill-review-ownership` | L210–228: the whole paragraph | **procedural** | — | 1,396 | 1,341 | 55 |
| `skill-adrs-and-specs` | L229–240: *"Whichever kind is running"* | triggering | `*` | 1,087 | 787 | **300** |
| `skill-decompose` | L241–258: `**Decompose.**` + bullets + *Continue inline* | triggering | `*` | 1,332 | 1,165 | 167 |
| `pending-skill-shapes` | L259–EOF | triggering | `*` | 34,624 | 34,563 | 61 |

`defers=` written, in full:

- `skill-execute` → `grilling-interrogate`
- `skill-adrs-and-specs` → `adr-placement-note`, `adr-why-the-set-stays-minimal`,
  `spec-set-is-current-state`, `driving-reworking-adrs-and-briefs`,
  `driving-ask-wdyt`, `driving-ask-for-pushback`, `driving-record-decisions-inline`,
  `driving-cite-framework-decisions-to-the-source` — eight members, the direct
  consequence of the fusion
- `skill-decompose` → `driving-externalizing-surfaced-work`,
  `brief-the-node-briefing`, `driving-what-a-good-child-leaf-looks-like`
- `task-in-session-doubt-budget` (edited in place, decoupling lemma) →
  `driving-doubting-inside-a-picked-leaf` **+ `skill-review-ownership`**
- `pending-skill-shapes` → **none**, as the convention requires

**Per-mandate effect: every one of the nineteen mandates loses 790 bytes.** Before,
one `class=triggering kinds=*` residual shipped 40,077 B; after, 39,287 B ships and
1,396 B is addressable procedure. The saving is the 1,341 B procedural body less
551 B of net new marker overhead — modest, because the region is small and three of
its four units are conditions that must ship. The bulk of the file is still
`pending-skill-shapes` and is `shape-cutting-k30`'s, `lifecycle-k31`'s and
`finish-cycle-k32`'s to carve; **it was not absorbed.**

### Coverage proved by reconstruction, instrument controlled first

Following `research-moves-k25`'s vacuous-check trap: the strip was proved non-blind
before any comparison — `grep -v '^<!-- unit: '` removes 794 B from the pre-batch
file and 1,400 B from the post-batch one, so it is not silently matching everything.

- **The five units, fetched in one call and stripped of their marker lines, are
  byte-identical to the consumed `pending-skill-loop`'s coverage — 40,018 B** on both
  sides. No gap, no overlap.
- **My four units' prose is byte-identical to pre-batch L179–258 — 5,455 B**, the
  planned figure, so the residual boundary is where the plan put it.
- **Both edited files with every `<!-- unit: ` line removed are byte-identical to
  their pre-batch selves.** No prose, filename or fence moved; trailing newline
  present, fence balance untouched (4 fence lines, all outside this region).
- All five markers are unindented whole lines at neutral fence state, at
  post-marking L178, L210, L230, L243, L262.

### Edge inventory rows owned: 17–25, eight written and one declined

| row | source → target | outcome |
|---|---|---|
| 17 | `task-in-session-doubt-budget` → `skill-review-ownership` | **written** — this body's only root; without it the unit fails (R) |
| 18 | `skill-decompose` → `driving-externalizing-surfaced-work` | **written** — second address into a body #8 rooted from `driving-field-guide` |
| 19 | `skill-decompose` → `brief-the-node-briefing`, `driving-what-a-good-child-leaf-looks-like` | **written**, both members |
| 20 | `skill-adrs-and-specs` → `adr-placement-note`, `adr-why-the-set-stays-minimal` | **written**, both — see the target note below |
| 21 | `skill-adrs-and-specs` → `spec-set-is-current-state` | **written** |
| 22 | `skill-adrs-and-specs` → `driving-reworking-adrs-and-briefs` | **written** |
| 23 | `skill-adrs-and-specs` → the four habits its parenthesis names | **written**, four members — see the scoping note below |
| 24 | `skill-execute` → `TASK-FORMAT.md` per-kind discipline bodies | **declined**: `kinds-k22` made **none** of them procedural |
| 25 | `skill-execute` → `grilling-interrogate` | **written** |

**Row 24 is declined for the reason `kinds-k22` pre-supplied, verified against the
file rather than taken on trust.** All ten per-kind units — `task-producer-*` (five),
`task-research-pair`, `task-combine-research`, `task-finish-session`,
`task-review-kinds`, `task-integrate-review-kinds` — plus `task-hitl-afk`, which the
same parenthesis cites, are `class=triggering`. A `defers=` naming any of them is a
build error. The row was conditional on #2 having made one procedural; it did not, so
this is the plan's own expected outcome, not a gap.

**Row 20's two members, and why the entry alone was not enough.** The prose makes two
distinct promises. *"`ADR-FORMAT.md` for placement"* is a bare file citation, and
`guides-k24`'s precedent answers those with the file's **entry** unit —
`adr-placement-note`, which is procedural and legal, and whose own `defers=` reaches
the placement body. But the node brief's family-C table asks #9 for the **owner's
address** to §*Why the set stays minimal* specifically, and that section is what the
paragraph's second promise (*minimum coherent set … rework in place … never append a
superseding ADR*) actually expands. Naming only the entry would have supplied an
address for the *file* and not for the *body the family names*, so both are written.
The redundancy is the legal, listed kind: `adr-why-the-set-stays-minimal` is an
inventory row's target, so a later drop is visible against the plan.

**Row 23 is written at the parenthesis's four named habits, and deliberately no
wider.** The sentence is *"See `driving.md` for the field-guide habits that make
grilling, research-leaf commissioning, and the review chain productive (WDYT,
pushback, running decision log, citation discipline)."* Its **entry-unit indirection
is unavailable** — `driving-field-guide` is `class=triggering` — which is the same
obstacle `research-moves-k25` recorded for rows 38–40, and the same answer applies:
name the sections the prose names. The parenthesis names four, and they map exactly
onto `driving-ask-wdyt`, `driving-ask-for-pushback`, `driving-record-decisions-inline`
and `driving-cite-framework-decisions-to-the-source`.

I did **not** extend it to `driving-how-to-write-a-research-leaf-brief`,
`driving-the-review-chain` or `driving-review-chain-habits`. The three subjects the
sentence lists are *topics*, and each already ships its own `kinds=*` condition
carrying those addresses — `driving-when-to-commission-prior-art-research` and
`task-two-shapes` respectively. **Be clear about what row 23 therefore is: an
addressing edge and nothing more.** All four members are already reachable from
`kinds=*` roots (`driving-when-to-invoke-grilling` for the first three,
`driving-when-code-depends-on-a-framework-version` and `task-producer-impl` for the
fourth), so **dropping any member leaves `cargo build` green** — rows 38–40's silent
shape exactly, and why it is reported member by member here.

### (D), (R), (T) at end of batch

- **(D)** — every one of the twelve `defers=` members written names a declared unit
  and every one is `class=procedural`, confirmed by fetching all eight of
  `skill-adrs-and-specs`'s targets and reading their markers back.
- **(R)** — `skill-review-ownership` is the batch's only procedural unit and row 17
  roots it from `task-in-session-doubt-budget`, a `kinds=*` triggering unit. So it is
  reachable from all nineteen mandates, not one.
- **(T)** — the deepest chain this batch creates is two hops:
  `skill-adrs-and-specs` → `driving-reworking-adrs-and-briefs` /
  `driving-cite-framework-decisions-to-the-source` →
  `driving-doubting-inside-a-picked-leaf`, which defers to nothing. Every other chain
  is one hop. No chain can re-enter my units: all three sources are triggering, and a
  triggering unit is an illegal target.

### Sweeps: both run, both recorded

- **Outbound** — seven embedded-file citations inside the region, and they are
  precisely rows 19–25's sources: `TASK-FORMAT.md` once (row 24, declined),
  `grilling.md` once (25), `ADR-FORMAT.md` once (20), `SPEC-FORMAT.md` once (21),
  `driving.md` twice (22/23 and 19), `BRIEF-FORMAT.md` once (19). Nothing unlisted.
  **Row 18 has no filename citation at all** — it exists only because
  `driving.md` L615 points back the other way — which is the inventory earning its
  keep on its own stated terms.
- **Inbound** — `grep -rn 'SKILL\.md' content/` returns fourteen hits; **two point
  into this region**, and both are declined below. The rest land in #1's spine
  (`constraint 1`, `constraint 4` ×2), #11's `**Retire.**` (`BRIEF-FORMAT.md` L40 and
  L89, `driving.md` L728 = row 37), whole-file framing (`driving.md` L4), or are
  external-skill attribution comments (`driving.md` L40, L645, L690, `grilling.md`
  L2).

### Three edges considered and declined, each one marker edit to reverse

1. **`driving-externalizing-surfaced-work` → `skill-decompose`** (`driving.md` L615,
   *"`SKILL.md`'s Decompose step states the rule; this is the habit that honours
   it"*) and **`driving-what-a-good-child-leaf-looks-like` → `skill-decompose`**
   (L674, *"a second axis alongside 'fits this session' (`SKILL.md`'s Decompose
   step)"*). Both **illegal and unnecessary**: the target is `class=triggering
   kinds=*`, so a `defers=` naming it is a build error, and the condition already
   ships in every mandate. This is `prompts/continue.md`'s edgeless-reference shape
   found twice more, in the reverse direction of row 18 — worth the reviewer's note
   that the corpus's most-cited paragraph is cited exclusively *by* procedures, which
   is what a family-B owner should look like.
2. **`skill-review-ownership` → `task-review-chain-mechanics`.** The paragraph says
   *"cut a `review-<producer>` leaf with `leaf-add`"* and #3 carved the mechanics of
   doing so. **Declined** on three counts: the prose carries no citation to
   `TASK-FORMAT.md`, the paragraph's subject is the *budget* rather than the chain,
   and `task-two-shapes` (`kinds=*`) already addresses that body in every mandate.
3. **`skill-decompose` → `task-decompose-inherits-kind`.** Same shape — the bullet
   names `leaf-decompose` and #2 carved the `--kind` inheritance body. **Declined**:
   no citation, and `task-too-big-is-planning` (`kinds=*`) already addresses it.

### The pre-decided verdicts, applied and not re-decided

All four landed as the node brief settled them, and none looked wrong with the prose
open. Two carry additions.

- **Family B owner** — `skill-decompose`, triggering `kinds=*`. The corpus's own
  designation held up: `driving.md` L615 names it as the rule's statement, and the
  inbound sweep found it cited only by procedures.
- **Families C, D and E owner** — `skill-adrs-and-specs`, triggering `kinds=*`, kept
  whole. `batches-k34`'s finding is confirmed exactly: the paragraph's sentence
  boundaries fall mid-line (post-marking L233 carries the end of the spec clause and
  the start of the ADR-set clause), so it cannot be split by a whole-line marker
  without editing prose.
- **Family A body** — `skill-review-ownership`, procedural. Rooted by row 17.
  **The predicate's negative half is stated here and not in the owner**, as #2 and #7
  both recorded: *"a `.grove/` directory in the checkout and inherited Grove control
  variables do not count."* That is now recorded from all three sides. My reading:
  the owner ships the positive predicate to every kind and the disqualifier ships to
  none, so a session that has inherited Grove variables *without* a mandate reads the
  owner's *"once the current session has run Bootstrap and adopted the driver's
  selected-leaf mandate"* and must infer the negative. It is an inference from a
  clearly stated positive rather than an unasked question, which is why I did not
  treat it as a misclassification — but it is the residue the aggregate reviewer was
  told to rule on, and it is one prose sentence in the owner to fix if the reviewer
  disagrees.
- **`**Execute.**`** — triggering `kinds=*`, no family site, **not scoped and not
  split** (see the grain call below).

### The grain call: `**Execute.**` stayed one unit

The task file licensed a second statement of per-kind discipline and I did not carve
one, on `guides-k24`'s established boundary rule — *carve at a heading or a distinct
block; **never inside one list*** — and on its stated exception: `kinds-k22` split
`TASK-FORMAT.md`'s producer bullets individually only because a **scope difference**
forced it. No such force exists here. This leaf's own *Scope* section settles that
nothing in the region is honestly narrower than `kinds=*`, so the three bullets would
have become three units identical in class and scope, differing only in id — and the
paragraph introducing them ends in a colon, so splitting it from its list would leave
a condition pointing at nothing (the shape `evidence-moves-k26` rejected and
`decompose-moves-k28` accepted only where no alternative existed).

Each bullet was nonetheless tested against the asymmetry on its own, because a shared
class is a conclusion and not an assumption: bullet 2's *inspection-only* prohibition
is the strongest condition in the region (withhold it and a `review-*` session runs
tests and edits code), bullet 3 tells a `planning` session what it must do first, and
bullet 1 tells every other kind the loop does nothing special with its artifact.
Three conditions, one list, one unit.

### Design findings for the aggregate review

**1 — the L217–227 fusion, confirmed and now measurable.** One paragraph owns
families C, D and E, so rows 20–23 all leave from one marker: **`skill-adrs-and-specs`
carries a 300-byte marker over 787 bytes of prose, and an eight-member `defers=`.**

The comparison that makes the cost legible is not the ratio — several small
`TASK-FORMAT.md` units run 30–50% marker by slice simply because they are short, so
27.6% is unremarkable there. It is this: **the embed's only other eight-member
`defers=` is `grilling-interrogate`'s, and its 314-byte marker is the only one larger
than this one.** That unit is `class=procedural` — its address list is paid for once,
by the session that fetches it. `skill-adrs-and-specs` is `triggering kinds=*`, so a
comparable marker ships to **all nineteen** mandates. A fused owner is where an
address list is most expensive, and this is the corpus's one instance of it.

The grain here is bounded by line boundaries, so **de-fusing the paragraph is a prose
edit for a later grove, not a marking decision** — and the pairing above is the figure
to quote when arguing for it.

**2 — a second fusion the plan did not predict, in the same region.**
`**Review ownership inside a picked leaf.**`'s tail — the escalated-review routing
note (*"Once review is escalated to the tree grove owns the route…"*) — is about
review-target policy rather than the reviewer budget, and this leaf left splitting it
to my judgement. **It cannot be split.** The sentence boundary falls mid-line at
post-marking L223 (*"integrating. Outside this predicate doubt"*), and the line below
it opens mid-sentence, so no whole-line marker separates them. The whole paragraph
therefore takes the family-A verdict, **procedural**, and one consequence deserves the
reviewer's attention.

`docs/specs/doubt-grove-review-mechanics.md`'s *"Review diversity is personal
configuration policy … Grove … does not record producer targets, compare harnesses or
models, inject review warnings"* has **exactly one site in the embed** —
`grep -rn 'records no producer target\|owns the route' content/` returns
`content/SKILL.md:228` and nothing else — and that site is now inside a procedural
unit. So the fact reaches a session **only** by following `defers=` from
`task-in-session-doubt-budget`. It is a statement about what grove will *not* do, so a
session that never fetches it does not act wrongly; it simply cannot discover that
grove will not warn it about a mismatched review target. If the reviewer judges that a
condition rather than a procedure, the repair is a prose split, not a re-marking.

(The neighbouring `docs/adr/grove-owns-escalated-review.md` covers the budget
predicate and the one-`leaf-add` escalation, both of which ship — the owner is
`kinds=*` — and does **not** carry the routing claim. Checked, because I first
attributed it there.)

**3 — a stale cross-reference in `driving.md`.** L320 cites *"(`SKILL.md`'s **Plan**
and Retire steps)"*. There is no `**Plan.**` step: `grep -nE '^\*\*[A-Z][a-z]+\.\*\*'
content/SKILL.md` returns Pick, Bootstrap, Execute, Decompose, Retire, Commit, Signal,
Finish. The reference predates the lifecycle-verb removals. It is a citation rather
than an edge, so it owed nothing and nothing was written — but it is a prose defect
mandate delivery makes worse, because a session holding
`driving-reworking-adrs-and-briefs` is pointed at a step name that no longer exists
and has no document to browse for the nearest match. **Prose fix for the successor
grove; recorded rather than silently stepped over.**

**4 — no prose in this region resisted classification.** The node brief asks for
narrative that is neither condition nor procedure to be named. There is none here;
all four paragraphs are load-bearing rules. This region has nothing of
`## Reference files`' or `## The shortest version`' character.

### The ids #10, #11 and #12 need

Locatable by `grep -n '<!-- unit: <id>'` — no line numbers.

| batch | needs | which is |
|---|---|---|
| #10 `shape-cutting-k30` | `pending-skill-shapes` | the residual it consumes; `class=triggering kinds=*`, no `defers=` |
| #10 | rows 26–27, 32 | its own; my region writes nothing into `**Cut the next step**` |
| #11 `lifecycle-k31` | `pending-skill-shapes` (after #10 re-mints) | row 28 → `brief-the-node-briefing`, already the target of my row 19 |
| #11 | row 37's target | `driving-prune-reorder-or-file-an-issue`, #8's — untouched by me |
| #12 `finish-cycle-k32` | `skill-adrs-and-specs` | **row 30's source.** The family-D owner is this id, and `SKILL.md` `## Specs` is its body; adding a member to my eight-member list is the decoupling lemma's one-line edit |
| #12 | `skill-decompose` | family-B owner, for `prompts/continue.md`'s **edgeless** reference — a triggering target, so **no edge is legal**, exactly as the node brief settles it |
| #10–#12 | `skill-execute`, `skill-review-ownership` | complete as delivered; neither is a target for any remaining row |

### Doubts to carry forward, by id

1. **`skill-adrs-and-specs`** — the fusion (finding 1). Not a doubt about the class:
   two of its four rules ship at `kinds=*` nowhere else, so triggering is forced. The
   doubt is whether **eight `defers=` members on one marker** is an address list a
   session uses or one it skims, and whether row 23's four already-reachable members
   earn their 150-odd bytes in all nineteen mandates. Declining row 23 was live and I
   went the other way on rows 38–40's precedent.
2. **`skill-review-ownership`** — finding 2. The unit is procedural by the family
   call and the escalated-review routing note is fused into it. Whether that note is
   a condition is the reviewer's to rule on; it is inseparable at line granularity
   either way.
3. **`skill-execute`** — the grain. One unit for 2,162 bytes and three bullets, on
   `guides-k24`'s list rule. If the reviewer prefers per-bullet ids for
   addressability, the split is four markers and no prose edit — but note that all
   four would be `triggering kinds=*`, so it buys nothing a mandate can see.
4. **Row 20's two members** — naming both the entry and the family body is
   redundant by reach (the entry defers to the body). I judged the family-C
   instruction to want an explicit address; a reviewer may read one member as
   correct and the second as noise.
