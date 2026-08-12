# classification-k35

**Reviews:** `spine-k21`, `kinds-k22`, `shapes-k23`, `guides-k24`,
`research-moves-k25`, `evidence-moves-k26`, `doubt-moves-k27`,
`decompose-moves-k28`, `execute-k29`, `shape-cutting-k30`, `lifecycle-k31`,
`finish-cycle-k32` — **all twelve**, not the closing commit alone.

## Goal

Audit the **whole classification** of `content/` — 145 kB of embedded markdown,
now 143 units across nine files, 68 `class=triggering` and 75 `class=procedural`.

This is the artifact the successor grove's mandate composer and its golden
per-kind snapshots are built on. **A misclassification that survives this review
is baked in behind bytes that look stable**, and the failure it hides is silent:
a triggering condition misfiled as procedural yields an *unasked question* — no
error, no exit code, no diff. That is why the node brief made this review
non-optional in practice.

**You are inspecting, not fixing.** Read the committed classification, the
briefs, and the twelve batch records. Do not run `cargo build`/`test`, edit
`content/`, or re-mark anything: the build is already green and the paired
`integrate-review-impl` leaf owns every repair. Findings only.

## Context

### The baseline, and the diff to read

**Pre-classification baseline: commit `1956e2d05e50`** (change id
`yutlkywutlqq`), *"plan: repair classification batching contract (batches-k34)"*.
It is the last commit before `spine-k21` touches `content/`.

**Not `batches-k13`'s**, which `batches-k13` itself named — `batches-k33` and
`batches-k34` both land after it, so a diff from there carries planning churn.
The corpus bytes are identical across all three, so this is about giving you a
clean diff, not about which bytes were classified.

The twelve implementation commits run consecutively from `1d6f802867e0`
(`spine-k21`) to this leaf's producer.

### What is *already proved*, so you need not re-derive it

Every batch proved coverage by reconstruction with the instrument controlled
first (`research-moves-k25` found and documented a vacuous-check trap:
`jj file show '@-' <path>` reads both arguments as paths and silently diffs the
working copy against itself — the `-r` form is the one that fetches a revision).

- **Total partition holds.** Each batch's units, marker lines stripped, are
  byte-identical to the residual they consumed. Every boundary chains.
- **No prose moved, anywhere.** Every changed line in `content/` across all
  twelve commits is a marker line; each batch's `cmp` of stripped files is clean.
- **Zero residuals remain.** `grep -rc '<!-- unit: pending-' content/` returns 0,
  and — the check `lifecycle-k31` warned is *not* implied by it — all nine seed
  ids (`skill`, `task-format`, `driving`, `grilling`, `spec-format`,
  `brief-format`, `context-format`, `adr-format`, `continue`) are consumed too.
- **The whole-embed gate passes from the linked binary**, and `grove-llm
  methodology` was verified out of a **rebuilt, installed** binary in the final
  batch: 143 rows listed, and `skill-finish` → `skill-finish-steps` spot-fetched
  to show a `defers=` target that answers its condition.

**So the mechanical half is settled. What is left is the half no build checks**,
and that is the whole of your brief.

## The six pre-decided verdicts — audit them, do not reconcile twelve calls

`batches-k34` moved the repeated-rule decisions **out of** the batches and into
the node brief, so no batch decided a family. You are auditing **six settled
verdicts** plus what the batches recorded when a verdict looked wrong with the
prose open — a narrower and more answerable question than *"did twelve sessions
agree with each other?"*

| family | owner | landed as settled? |
|---|---|---|
| **A** in-session reviewer budget | `task-in-session-doubt-budget` (`*`) | yes — bodies `driving-doubting-inside-a-picked-leaf` (#7), `skill-review-ownership` (#9) |
| **B** externalize rather than absorb | `skill-decompose` (`*`) | yes — and the corpus's own designation held: `driving.md` L615 names it, and the inbound sweep found it cited **only by procedures**, which is what a family-B owner should look like |
| **C** ADR set is current-state | `skill-adrs-and-specs` (`*`) | yes — fused, see design finding 1 |
| **D** spec at an agreement point | `skill-adrs-and-specs` (shared) | yes — bodies `spec-set-is-current-state` (#4), `skill-specs` (#12) |
| **E** raise ADRs sparingly | `skill-adrs-and-specs` (shared) | yes |
| **F** the two shapes are built oppositely | `task-two-shapes` (`*`) | yes — `skill-cut-the-next-step` is its body (#10) |

**One verdict-level residue the plan already knew about, now confirmed from all
three sides** — `kinds-k22`, `doubt-moves-k27` and `execute-k29` each recorded
it independently:

> **Family A's owner states its predicate positively and never states the
> negative half.** It says the budget applies *"once the current session has run
> Bootstrap and adopted the driver's selected-leaf mandate"*. What it never says
> is what **both bodies** say outright: merely finding `.grove/` in the checkout,
> or inheriting Grove control variables, does **not** activate it.
>
> The owner is the only `kinds=*` statement of the rule, so **the half that
> grants the allowance ships everywhere and the half that withholds it ships
> nowhere.** `execute-k29` reads that as an inference from a clearly stated
> positive rather than an unasked question; `doubt-moves-k27` reads it as a false
> positive on a rule whose whole job is to stop a second reviewer being spawned.
> **This is yours to rule on.** It cannot be fixed by marking — the guard is one
> sentence inside a paragraph — so a finding here is a prose recommendation.

**Eleven further families were unlisted and decided by batches under the node
brief's default** (earliest complete statement owns; later complete statements
are bodies; everything else is a mention): `shapes-k23` decided six (bare stem,
no-node-for-a-shape, declare-the-relationship, nothing-in-a-body-is-metadata,
grammar-is-five-fields, leaf-never-names-a-harness), `research-moves-k25` one
(the combine step's adversarial move), `evidence-moves-k26` one (citation
discipline), `decompose-moves-k28` three (horizon notes, leaf-place-in-doubt,
no-session-log), and `shape-cutting-k30` one — **family M, the chain-gap
asymmetry, which is the only family decided *against* the default's earliest-site
rule** and is flagged in the doubts below.

**A bookkeeping hazard worth one finding:** batches assigned family letters
independently, so the letters **collide** — there are three different "I"s, three
"J"s, two "K"s and two "G"s across the records. Nothing depends on the letters,
but a reader reconciling records by letter will be misled.

## The assembled edge inventory — all 42 rows, plus 11 additions

Each batch reported its own rows; this is assembly, not reconstruction. **44
outcomes** (row 27 resolved in three halves), **37 written and 7 declined.**

### Written (37)

| rows | owner | targets |
|---|---|---|
| 1, 2, 3, 4, 5, 6, 7, 8 | #4 | `grilling.md` → `ADR-FORMAT.md` / `CONTEXT-FORMAT.md` / `SPEC-FORMAT.md` closure, rooted at `TASK-FORMAT.md`'s kind-scoped bullets |
| 9 | #6 | `driving-when-to-retire-research-into-adrs` → `driving-reworking-adrs-and-briefs` |
| 38 | #6 | `task-producer-impl` → `driving-cite-framework-decisions-to-the-source` |
| 11, 33, 39, 41, 42 | #7 | into `driving-doubting-inside-a-picked-leaf` and the chain bodies |
| 12, 13, 14, 16, 40 | #8 | `driving-field-guide` roots, `BRIEF-FORMAT.md` roots |
| 17, 18, 19, 20, 21, 22, 23, 25 | #9 | the fused C/D/E owner's eight-member list, plus `skill-decompose`'s three |
| 26, 27a, 32 | #10 | `skill-cut-the-next-step` ↔ `task-*` mechanics |
| 28, 37 | #11 | `skill-node-close-steps` → `BRIEF-FORMAT.md`; `driving-prune-…` → `skill-leaf-prune-mechanics` |
| 29, 30, 31 | #12 | `context-structure`; `skill-specs`; `skill-reference-files` |
| 34 | #5 | `driving-when-to-invoke-grilling` → `grilling-interrogate` |

### Declined (7) — each with the reason to check

| row | owner | reason given |
|---|---|---|
| 10 | #6 | **the citation the row is predicated on does not exist** — there is no `ADR-FORMAT.md` reference anywhere in `driving.md`, at baseline or now (verified with a cross-tree control so the grep was not blind) |
| 15 | #8 | **no reference in either direction** between §*What a good child leaf looks like* and `BRIEF-FORMAT.md`; the row described a co-occurrence, not a citation |
| 24 | #9 | conditional on #2 having made a per-kind discipline body procedural. **It made none** — all ten are triggering, so the edge is a build error. The plan's own expected outcome |
| 27b, 27c | #10 | targets `task-declare-the-relationship` and `task-grammar-is-five-fields` are **`class=triggering`** — illegal `defers=` |
| 35 | #5 | both hits are parenthetical citations propping up claims already made in full; half the row's target was triggering besides |
| 36 | #7 | same test — a parenthetical, contrasted explicitly against `SKILL.md`'s *"(`TASK-FORMAT.md` **carries the full reasoning**)"*, which **is** an edge (27a) |

### Additions (11 written, ~20 candidates declined)

Unlisted edges a batch found inside its own region and owned as the later
endpoint: `task-leaf-filename` → `task-suggested-shape` (#3);
`grilling-domain-awareness` → `adr-where-adrs-live` (#4);
`task-combine-research` → `driving-the-combine-step` (#5);
`task-producer-planning` → `driving-find-working-increments` +
`driving-what-a-good-child-leaf-looks-like` (#8); the three
`task-*` → `skill-*` restatement roots (#10); `skill-decompose` →
`skill-directory-tree-and-grow-verbs` and `task-in-session-doubt-budget` →
`skill-cutting-a-review-leaf` (#11); `skill-specs` → `spec-suggested-shape` +
`spec-test-seams` (#12).

**The declined candidates are listed in each batch record with a reason**, and
most were declined on the same ground: the target was already rooted from a
`kinds=*` condition, so *"the second condition's session has no address for it"*
did not fire, and an unlisted second inbound path is the drop-the-real-one hazard
without the inventory's visibility.

### The question only you can ask about the inventory

**Does each written edge address a body its source's condition actually raises,
and does each declined row deserve its decline?** The build cannot ask either.
Two specific pressures:

- **Redundancy that launders itself.** `driving-doubting-inside-a-picked-leaf`
  ends with **four** inbound edges (rows 11, 39, 41, 42) — the highest in-degree
  in the corpus. `context-structure` has three (4, 6, 29).
  `adr-placement-note` has several. Every one is a *listed* row, which is the
  node brief's stated safety condition — **confirm that condition is doing real
  work here rather than laundering redundancy.** If row 11 were dropped, three
  paths keep the build green and only the inventory shows it.
- **Rows that are addressing, not reachability.** Rows 20 (second member), 23
  (all four), 26, 38–40 and my A1 are edges whose targets were already reachable.
  Each was written deliberately, on the reasoning that a promise of three or four
  addresses is only honoured if all land. A reviewer may read some as noise.

## The assembled doubts, by unit id

Every batch was asked to record what it was least sure about precisely so this is
assembly. Grouped by the **kind** of doubt, as the node brief asks.

### (a) Scope calls — `kinds=` narrowing, where the failure is silent in 18 kinds

- **`task-producer-*` and the other kind-scoped units (#2), as one decision.**
  Ten units narrowed on one argument: a kind's discipline has two consumers (the
  executing session and a session *choosing* a kind), and narrowing is safe only
  because `task-nineteen-kinds` and `skill-execute` ship the **gloss** at
  `kinds=*`. **Attack the argument, not the ten units.** Most exposed:
  `task-producer-design` (its drift clause is arguably what a *planning* session
  needs) and `task-review-kinds` (what a producer is buying when it cuts
  `review-X`).
- **`skill-starting-a-new-grove` (#1) vs `task-bootstrap-leaf-is-requirements`
  (#2) — decided opposite ways; read as a pair.** #1 declined a
  `kinds=requirements` narrowing; #2 accepted one on nearly the same shape.
- **`driving-when-to-invoke-grilling` (#5) — `kinds=*`, the batch's least
  confident call.** It widens `grilling.md`'s reachability beyond the plan's
  rooting table (which had it reachable from `requirements` alone). Nothing
  objects — reachability is per kind and additive — but it is a departure.
- **`driving-when-not-to-start-a-grove` (#5)** — `kinds=requirements` on a
  genuinely mixed-addressee, unsplittable section.
- **`driving-when-code-depends-on-a-framework-version` (#6)** — `kinds=*` on a
  709 B unit whose prose names `impl`.
- **`spec-when-a-spec-is-written` (#4)** — `kinds=*`, not `kinds=design`.
- **`skill-finish` (#12)** — `kinds=*`, on the reading that *"you do not discover
  that a grove is finished"* tells a **non**-`finish` session not to go looking.

### (b) Condition/body splits — the class calls, where a misfile is the silent direction

- **`skill-loop-diagram` (#1, 1,851 B)** — triggering, though it states no
  condition completely. Decisive ground: **there is no honest root for it as a
  body**; rooting it from `skill-what-a-grove-is` would be the artificial root the
  plan warns against.
- **`task-name-reading-is-strict` (#2)**, **`task-what-shapes-are-not` (#3)**,
  **`task-suggested-shape` (#3)**, **`skill-bare-grove-dispatch` (#1)**,
  **`skill-no-exception-to-check` (#10)** — five close calls, each one marker to
  flip, each with its reasoning recorded.
- **`driving-field-guide` (#5)** — triggering, and it **fuses** a genuine framing
  statement with an 18-line index of section anchors, shipping ~1.9 kB to all
  nineteen mandates. #5's own distinction from `## Reference files` (subjects vs
  filenames) is real but thin, and it cannot be split without stranding row 13.
- **`driving-recording-fog` (#8)** — the batch's most consequential departure
  from its leaf's framing. Made triggering because `grep -rn 'horizon' content/`
  returns exactly two sites, so if it were procedural **the fact that grove has a
  place for foreseen work would ship in no mandate at all.**
- **`skill-chain-gap-asymmetry` / `skill-integration-placement` (#10) — family M,
  the one un-pre-decided call, and the one to attack first.** #10 made `SKILL.md`
  the owner **against** the default's earliest-site rule, because the earliest
  site (`task-chain-contiguity`) is already procedural — so treating `SKILL.md`
  as a body too would leave *an integration must be cut adjacent, because a gap
  silently corrupts its citations* shipping **nowhere**. **Check the premise, not
  just the conclusion:** if `task-chain-contiguity` ought to have been triggering,
  the repair belongs there instead.
- **`skill-commit-boundary-in-git-and-jj` (#11) — the most consequential withheld
  body in the corpus.** The jj sealing rule ships nowhere, and its failure is
  silent: a session that runs `jj describe` and never `jj new` puts the *next*
  task's first edit into *this* task's change, and nothing errors. Three
  mitigations are recorded (the driver states the VCS, `skill-commit` carries the
  address, `linkuistics:using-jujutsu` triggers independently). **If you promote
  exactly one procedural unit to triggering, this is the candidate.**
- **`skill-review-ownership` (#9)** — procedural, with the escalated-review
  routing note fused in. Consequence: `doubt-grove-review-mechanics.md`'s *"grove
  records no producer target, compares none, warns about none"* has **exactly one
  site in the embed**, and it now reaches a session only by following `defers=`.
- **`skill-finish-nothing-after` (#12, 298 B)** — *"integrating the branch is not
  grove workflow"* is a scope prohibition, which is the triggering shape, and it
  was made procedural on the ground that only a finish session is ever at that
  point and it holds the address.
- **`skill-artifacts` (#12)** — triggering on the asymmetry, though every path it
  names is separately reachable.

### (c) Grain — fusion and splitting, where the cost is a fetch rather than a mandate

Each of these names its **honest seam** if you want the split; all are marker-only
edits with no prose consequence.

`task-nineteen-kinds` (#2) · `task-declare-the-relationship` (#3) ·
`task-chain-contiguity` (#3) · `grilling-interrogate`'s eight-member `defers=`
(#4) · `spec-set-is-current-state` as one unit rather than three (#4) ·
`driving-turning-a-sweep-into-evidence` (#6, 3,377 B — seam above *"Never document
a claim with a count of itself"*) · **`driving-review-chain-habits` (#7, 7,574 B —
the largest procedural unit in the corpus, 2.2× the previous; seam above bullet 8,
*"The reviewer produces findings, not fixes"*)** · `driving-the-review-chain` (#7,
the fusion in the other direction) · `brief-every-node-carries-one` (#8) ·
`skill-execute` (#9) · `skill-cut-the-next-step` (#10) ·
`skill-kind-on-the-tree-verbs` (#11 — **parity with `TASK-FORMAT.md`'s grain
argues for splitting**) · `skill-briefs-vs-glossary` (#12).

**One grain rule was inherited and applied nine times: *never carve inside one
list*** (`guides-k24`), overridden only where a **class or scope difference
forces it** (`kinds-k22`, `shapes-k23`). Confirm the rule is right, because it is
what makes several of the largest units large.

### (d) The dangling-colon pattern — decided inconsistently, and worth one ruling

Three batches met a condition whose last line ends in a colon whose list is
deferred. **`evidence-moves-k26` rejected the shape** (it had a cleaner boundary
one line up); **`decompose-moves-k28` accepted it** for
`driving-when-a-leafs-place-is-in-doubt`, arguing the colon's referent under
mandate delivery *is* the `defers=` on the same marker; **`finish-cycle-k32`
accepted it** for `skill-finish` → `skill-finish-steps`. `execute-k29` and
`shape-cutting-k30` declined to split at all partly on this ground.

**One ruling settles all of them.** The `skill-finish` pair is the cleanest
instance to rule on — and it was verified end-to-end out of an installed binary,
so you can see exactly what a session receives.

### (e) Standing-alone defects the build cannot see

The fence half of *"a unit must read correctly standing alone"* is mechanical and
holds. The **prose half** is yours. Known instances:

- Units opening on a back-reference: **"So"** (`skill-integration-placement`,
  #10), **"Instead"** (`skill-node-close-steps`, #11), **"The counterpart to the
  section above"** (`driving-when-asserting-a-repo-wide-claim`, #6). #11 notes
  this is a *pattern*: condition-then-procedure prose reliably opens the procedure
  with a back-reference, and mandate delivery is what makes it visible.
- **`grilling.md`'s `<supporting-info>` wrapper opens in one unit and closes six
  units later** (#4) — read standing alone, six units sit inside an unclosed tag.
  Harmless (the parser tracks fences, not HTML blocks) and the file is vendored.
- **`## In this guide`'s anchors are dead links under mandate delivery** (#5) —
  every entry is a markdown anchor that resolves only for a reader holding the
  whole file.

## What the build cannot check — stated plainly, because it is what you are for

1. **Whether a triggering condition was misfiled as procedural.** The silent
   direction. It yields an unasked question, no error and no diff, and it is the
   single failure this whole review exists to catch. The corpus-wide count of
   `kinds=*` triggering bytes is the budget; the doubts in (b) are where to look.
2. **Whether each unit reads correctly standing alone** — the prose half, per (e).
3. **Whether a `defers=` addresses a body its source's condition actually
   raises**, and whether the declines deserve their reasons.
4. **Whether a rule that should ship, ships.** Reachability proves a body can be
   *reached*; nothing proves a **condition** arrives. See finding 4 below, which
   is the corpus's one proven instance of a rule that ships nowhere.

## The design findings — adjudicate these rather than rediscover them

**1 — the `skill-adrs-and-specs` fusion, confirmed and now measurable.** One
paragraph states **four** rules (raise ADRs sparingly; write a spec only at an
agreement point; the ADR set is current-state; the same rule governs
`docs/specs/`), and its sentence boundaries fall **mid-line**, so a whole-line
marker cannot split it and this pass edits no prose. It therefore owns families C,
D **and** E at once, and rows 20–23 plus row 30 all leave from that one marker.

The cost `execute-k29` made legible: **a 300-byte marker over 787 bytes of prose,
with a nine-member `defers=`, shipping to all nineteen mandates.** The embed's
only comparable marker is `grilling-interrogate`'s 314-byte one — and *that* unit
is procedural, so its address list is paid for once by the session that fetches
it. **A fused owner is where an address list is most expensive, and this is the
corpus's one instance.** De-fusing is a prose edit for a later grove.

**2 — the unsplittable-paragraph pattern is systemic, not occasional, and this is
the finding with the largest leverage.** Nine instances across four batches:
`spine-k21`'s three single-line paragraphs (F1), `shapes-k23`'s 3.9 kB bullet
(F7), the `skill-adrs-and-specs` fusion, `shape-cutting-k30`'s bare-stem paragraph
(**where a leaf's explicit instruction to split was defeated by it**),
`lifecycle-k31`'s `**Signal.**`, node-close and retire-mechanics paragraphs, and
`finish-cycle-k32`'s `**Finish.**` framing.

`shape-cutting-k30` supplied the arithmetic: **the corpus is reflowed to ~78
columns, so a sentence boundary landing on a line boundary is chance at roughly
one in twelve.** The classification's grain is therefore capped by *reflow*
everywhere rather than by judgement. **The honest repair is a reflow pass in a
later grove** — mechanical, reviewable, and it would raise the achievable grain
across the whole embed. Recommend it or reject it; it should not stay implicit.

**3 — narrative residue: four instances, now enough to be one recommendation.**
`## Reference files` (#12), `driving.md`'s `## In this guide` anchors (#5),
`## The shortest version` (#8), `spine-k21`'s constraint-2 parenthesis (#1, which
is **also stale** — it describes re-provisioning that
`mandate-delivers-the-methodology` retires, and no stage-4 item currently repairs
it), plus `research-moves-k25`'s authoring note and
`skill-finish-no-signal-stop`'s second half.

They are one species: **narrative addressed to a reader of a *file*, in a corpus
that no longer delivers files.** `## Reference files` is the sharpest case — its
rows name files and a session cannot fetch a file — and #12 adds that **its root
expires at the same moment its content does**: `continue.md`'s *"use the grove
skill"* becomes false when provisioning retires. Whether any of them survives is
the successor grove's call; this review is where four observations become one
recommendation.

**4 — the `leaf-insert` adjacency obligation ships in no mandate** (`doubt-moves-k27`
F11). The corpus states it **four** times and every statement is procedural. What
ships at `kinds=*` is `task-two-shapes`'s *"cut lazily, one at a time, by the
session that needs the next one"* plus a `defers=`.

**This is the sharpest test case the review has for *keep the `if`, defer the
`then`***: the `if` here is not a situation a session recognises, it is a **verb
choice inside a procedure**. And the failure it guards is the design's own worst
shape — an integration cut with `leaf-add`, landing after intervening work,
consuming `path:line` citations that have drifted, *nothing errors, the finding
just points somewhere slightly wrong*. No site isolates the rule at a line
boundary outside a list, so the fix is prose.

**5 — the largest deletable prose surface** (`shape-cutting-k30` finding 2). Five
of seven paragraphs in `SKILL.md`'s shape-cutting region are the hub **restating**
a rule `TASK-FORMAT.md` owns, and three of them (additions A1–A3) are
**near-verbatim duplicates rather than expansions**. Their roots are on-topic, so
they are not artificial — but a session following them gets the rule it already
holds, said again in the hub's voice. **Reading the two files as documents, the
repetition looks like emphasis; reading them as a unit graph, it is three edges
that carry no information.** The honest repair is a prose deletion.

**6 — a bidirectional prose relationship has no representation in a DAG**
(`decompose-moves-k28` F15). §*Recording fog* names `BRIEF-FORMAT.md`'s On the
horizon note; that note's template row names §*Recording fog* by title. As prose,
a helpful round-trip; as a deferral graph, a **(T) violation** the moment both are
written. What resolved it was **accidental** — an unrelated split gave the pair
distinct endpoints. Nothing in the plan anticipates the shape, and it is the first
case where *which unit boundary you choose* decides *whether an edge is legal at
all*.

**7 — licence attribution is separated from the prose it attributes, three
times** (#6 F12-precursor, #7 F12, #8 F16). A licence comment is one contiguous
block, so it travels with the first section it attributes; `driving.md`'s
addyosmani comment attributes a **non-adjacent** pair, and its to-tickets comment
attributes material in two sections. **A session fetching
`driving-doubting-inside-a-picked-leaf` receives adapted MIT-licensed prose with
no attribution attached.** `LICENSES/` still carries the licence, so this is
attribution *locality* rather than compliance — but it is the first place unit
delivery makes provenance a classification concern, and the fix in every case is
the same prose edit: duplicate the comment onto the second section.

**8 — the first mid-paragraph marker, a corpus-wide precedent set by one batch**
(`lifecycle-k31` finding 1). `skill-leaf-prune-mechanics` opens *inside* a
paragraph — no blank line above the marker, because inserting one would edit prose
and break the byte-identity invariant every coverage proof rests on. Nothing
forbids it, and the node brief's own unsplittable reasoning is framed in terms of
**line** granularity, which only makes sense if this is available. But no earlier
batch did one. **Ratify or revert it deliberately**: #12 and every future
re-marking inherits it. Rejecting it costs 1,052 B × 19 mandates and inventory
row 37.

**9 — plan-coordinate defects the batches found, worth a note on the method.**
Five, all caught the same way — *by executing against the anchors and the
inventory rather than against the plan's prose*: `evidence-moves-k26` found three
(a citation row 10 was predicated on does not exist; a fence warning pointing at
the wrong region; family-C ranges one line short at the tail);
`doubt-moves-k27` found its leaf's *Done when* naming two rows that touch none of
its units; `decompose-moves-k28` found the node brief's **row 37 prose
contradicting its own table** (F17); `shape-cutting-k30` found **row 27 stated
two incompatible ways** between the node brief and its own leaf body, and reported
all three halves rather than choosing. **The last one is the one to generalise:
does any other row carry the same divergence?** A batch reading only one of the
two tables would have reported the row complete while leaving the other source
unexamined.

## Done when

- The **six pre-decided verdicts** are audited, and family A's missing negative
  half is **ruled on** — it is the one residue the plan explicitly deferred here.
- The **class calls in (b)** are checked, with `skill-commit-boundary-in-git-and-jj`,
  family M and `skill-finish-nothing-after` given particular attention — those are
  the three where a wrong call fails silently.
- The **dangling-colon pattern** gets **one ruling** covering all three batches
  that met it.
- The **edge inventory's** written rows and declines are spot-checked against the
  question no build asks, with the four-inbound-edge and three-inbound-edge bodies
  looked at specifically.
- The **nine design findings** are adjudicated — accepted, rejected, or referred
  to the successor grove — rather than rediscovered. Findings 2, 3, 5 and 7 are
  recommendations about *prose* and belong to a later grove either way; saying so
  explicitly is a valid outcome.
- Findings are recorded as findings. **If there are any worth acting on, cut the
  `integrate-review-impl` leaf** with them written into its body verbatim; if
  there are none, retire and create nothing.

## Notes

- **Retiring this leaf closes `classification-k9`** — it is the node's last live
  leaf, so the Retire cascade fires. Check the node brief's `Done when` against
  what the subtree delivered, promote anything still relevant upward, and name
  both handles in the commit message.
- **The classification is green and complete; that is not the claim under
  review.** Reachability going green is not the same claim as the inventory being
  right: a body reached by a second path stays green with a semantic edge missing.
- **You have one in-session reviewer of your own and should not need it.** A
  `review-*` leaf spawns none by default; this brief is already the diverse-lens
  pass.
- The twelve batch records are the evidence base and are dense but well
  structured — each has a `## Batch record` section with the same shape (units,
  coverage proof, rows owned, doubts by id, design findings). **Read them; they
  are why this review is an audit rather than a re-derivation.**

## Review outcome

The mechanical classification is accepted, but the semantic classification is
not accepted as-is. The integration leaf owns these findings verbatim:

1. **[high] Row 23 sends “citation discipline” to the wrong body.**
   `skill-adrs-and-specs` promises field-guide habits for grilling,
   research-leaf commissioning, and the review chain. Its fourth field-guide
   address is currently `driving-cite-framework-decisions-to-the-source`, whose
   condition and procedure are specifically about code depending on a framework
   version. The promised research-leaf habit is instead stated by
   `driving-how-to-write-a-research-leaf-brief`: demand a citation for every
   failure-mode claim, prefer primary sources, and record missing sources. Point
   the edge at the body the source actually promises; existing reachability is
   not evidence that the current edge is semantically valid.
2. **[high] Narrative units have been made artificial triggering roots.**
   `skill-loop-diagram` contains an overview diagram, not a condition a session
   can recognize. `driving-field-guide` contains file-level framing and an
   anchor index, not a condition either; its generic `kinds=*` root also hides
   the concrete prohibitions in procedural `driving-anti-patterns`. Rework or
   remove this file-reader prose and give each rule that must ship an honest
   triggering statement. Do not preserve narrative reachability by calling the
   narrative a trigger, and do not leave the anti-pattern conditions reachable
   only through a generic table of contents.
3. **[medium] `driving-when-to-invoke-grilling` has a scope/prose mismatch.**
   Its marker says `kinds=*`, but its complete trigger says only “a
   `requirements` leaf's brief lists three or more questions whose answers
   interdepend.” Either narrow the marker to the condition actually written or
   rewrite the condition to state the intended all-kind behaviour, including
   what a non-requirements session does when it encounters those questions.
   The batch rationale cannot silently broaden semantics that are absent from
   the delivered bytes.
4. **[medium] Several slices still fail the prose half of “reads correctly
   standing alone.”** Rewrite `driving-when-asserting-a-repo-wide-claim` so it
   names its subject instead of opening with “The counterpart to the section
   above”; remove or localize `grilling.md`'s cross-unit
   `<supporting-info>` wrapper; and attach the relevant licence attribution
   comment to each separated adapted unit. In the same focused prose cleanup,
   remove file-reader-only residue and near-verbatim hub restatements that have
   no mandate-delivery job. Preserve genuinely useful rules by restating their
   conditions, not by retaining a document-navigation shell.

### Rulings and adjudications

- The six settled families stand. Family A has no missing negative half in the
  committed owner: `task-in-session-doubt-budget` explicitly says that outside
  the Bootstrap-and-mandate predicate doubt keeps its standalone bounded cycle.
  Merely finding `.grove/` remains usefully explicit in its deferred bodies, but
  delivery of the owner itself already establishes the positive predicate.
- Family M is correctly owned by triggering `skill-chain-gap-asymmetry`; the
  earlier claim that the adjacency obligation ships nowhere is stale.
  `skill-integration-placement` is its addressed procedure.
- `skill-commit-boundary-in-git-and-jj` and `skill-finish-nothing-after` remain
  procedural. Their respective triggering owners give the session a direct
  address at the only point the mechanics are needed. `skill-no-exception-to-check`
  remains a defensible triggering prohibition.
- A dangling colon is valid only when the marker's `defers=` is the explicit
  address for the grammatical procedure promised by that colon and there is no
  cleaner honest boundary. The accepted decompose and finish cases meet that
  rule; the evidence case correctly used its cleaner boundary.
- The four inbound doubt edges, three inbound context-structure edges, and the
  multiple ADR-placement paths correspond to distinct source conditions. The
  listed declines are justified. Row 23's fourth target is the one semantic edge
  defect found by the spot-check.
- The current list-grain rule and the mid-paragraph marker precedent are
  ratified. A corpus-wide reflow solely to create more boundaries is not
  justified; targeted reflow is appropriate where it enables one of the repairs
  above, especially de-fusing `skill-adrs-and-specs`.
- Design findings 1, 3, 5, and 7 are accepted as focused prose cleanup covered
  by the integration finding above. Finding 2's blanket reflow is rejected;
  finding 4 was resolved by family M; finding 6 needs no bidirectional edge
  mechanism because `defers=` represents control flow rather than every prose
  citation; finding 8 is ratified. For finding 9, the anchor-and-inventory method
  worked as intended, and no second row divergence was found.
