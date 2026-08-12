# research-moves-k25

## Goal

Classify **`content/driving.md` from its body start to the line before `## When to
retire research into ADRs versus leave it`** (baseline L1–263, 13,580 bytes): the
file's framing, `## In this guide`, `## When not to start a grove`, `## When to
commission prior-art research`, `## How to write a research leaf brief`, `##
Running the vendor pair`, `## When to invoke a design discussion (grilling)` and its
four move subsections (WDYT, pushback, don't merge questions, record decisions
inline).

This is batch 5 of 12, and the first of four over `driving.md`. Its framing unit is
**the root three later batches depend on** — see below.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L1–263 is a baseline coordinate.** Carve from the
  file's body start to the line **before** `## When to retire research into ADRs
  versus leave it`.
- The seed unit `driving` is **consumed**. Mint exactly one residual,
  **`pending-driving-evidence`**, covering that heading to end of file, as
  `class=triggering kinds=*` **with no `defers=`** — a residual is a coverage
  placeholder, never an edge ledger.

### `driving.md` roots much of itself, and that is why it comes before `SKILL.md`

The node brief's corpus table calls this file "mostly procedural", and that is
true of its *bodies* — but its `## When to …` headings are **genuine
conditions**, and they are what makes this file batchable without `SKILL.md`.
Three in this region:

- **`## When to commission prior-art research`** — plainly triggering. A session
  never told this condition exists never commissions research, and never learns
  there was a question. Its bodies are `## How to write a research leaf brief`
  and `## Running the vendor pair`.
- **`## When to invoke a design discussion (grilling)`** — triggering; the four
  `###` moves beneath it are its procedural body.
- **`## When not to start a grove`** — read this one carefully before assuming it
  is triggering. The condition it states is faced by *a human deciding whether to
  start a grove*, not by a session already inside one. If you conclude it reaches
  no session's mandate honestly, it is procedural and must be reached from
  somewhere — the file's framing unit is the natural root. **Say which you chose
  and why**; it is the first genuinely debatable call in this file.

### The framing unit, and what later batches will hang off it

`driving.md`'s opening (L1–16) and `## In this guide` (L18–35) are the file's
catch-all entry: *"the moves a human collaborator makes that turn the loop into
productive design work"*, followed by an index naming **every** section of the file.
That is what makes it an honest root rather than a convenience: the opening names
externalizing, doubting and source-grounding outright, and the index names the rest
by title.

**Carve a framing unit that can serve as this file's root, and name its id in your
leaf body.** Three later obligations depend on it:

- **row 12** — `decompose-moves-k28` roots `## Externalizing surfaced work` here,
  because that section is a **body** whose semantic owner (`SKILL.md`
  `**Decompose.**`) is not carved until #9;
- **row 13** — `decompose-moves-k28` roots `## Anti-patterns` and `## The shortest
  version` here;
- **row 23** — `execute-k29` deferring `SKILL.md` L224–227 (*"See `driving.md` for
  the field-guide habits…"*) into the grilling-moves bodies.

Note the licence comment at L37–40 (mattpocock/skills attribution for the no-fog
early exit). It belongs with `## When not to start a grove` and must not be split
away from the prose it attributes.

### Edge inventory rows owned: 34 and 35

| row | edge | note |
|---|---|---|
| 34 | `## When to invoke a design discussion (grilling)` (L182, L189) → `grilling.md` bodies | `guides-k24` carved the target, so this is writable and looks genuine: the condition is *invoke a design discussion*, the body is the interview procedure |
| 35 | `## Running the vendor pair` (L141, L156) → `TASK-FORMAT.md` §*The vendor pair* / §*What the shapes are not* bodies | Read them twice. Both are parenthetical `(`TASK-FORMAT.md`)` **citations supporting a claim** — *"There is no node directory"*, *"a fact the filename already carries"* — not trigger→body edges. **Declining with that reason is the expected outcome**, and a decline recorded is a different act from an edge silently not written |

`SKILL.md` references in this region point at conditions, not bodies, and they sit
inside `pending-skill-*` anyway. **No edge may have a `pending-*` source**, and a
`defers=` naming a triggering unit is a build error. Report those hits as *not
yours*; park nothing.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-driving-evidence` covers the rest of the file and nothing else, and
  carries no `defers=`.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit: `driving` removed, the new
  `driving-*` ids added, `pending-driving-evidence` added.
- **Rows 34 and 35 are reported** — written, or declined with the reason.
- The framing unit's id is named in this leaf's body, and the `## When not to
  start a grove` call is stated with its reasoning.

## Notes

- `## Running the vendor pair` (3,182 bytes) contains a fenced `leaf-add-pair`
  example. Do not split mid-fence; the parser forbids it and the build will say
  so, but the authoring rule behind it is the one no build checks.
- Doubts to carry forward, by id.

## Batch record

Region carved between the two anchors. **11 units minted, the seed id `driving`
consumed, one residual minted** (`pending-driving-evidence`); `EMBEDDED_UNITS`
76 → 87. `cargo build` and `cargo test` green (0 failures, every suite).

**The framing unit's id is `driving-field-guide`** — that is the answer batches
#6, #7 and #8 need, and it is what inventory rows 12 and 13 root from.

### The units, in document order

| id | region (baseline) | class | scope | defers |
|---|---|---|---|---|
| `driving-field-guide` | L1–36: title, framing, `## In this guide` | triggering | `*` | — |
| `driving-when-not-to-start-a-grove` | L37–52, licence comment included | triggering | `requirements` | — |
| `driving-when-to-commission-prior-art-research` | L53–78 | triggering | `*` | the two below |
| `driving-how-to-write-a-research-leaf-brief` | L79–125 | procedural | — | — |
| `driving-running-the-vendor-pair` | L126–171 | procedural | — | — |
| `driving-the-combine-step` | L172–181 | procedural | — | — |
| `driving-when-to-invoke-grilling` | L182–193 | triggering | `*` | `grilling-interrogate` + the four moves |
| `driving-ask-wdyt` | L194–219 | procedural | — | — |
| `driving-ask-for-pushback` | L220–231 | procedural | — | — |
| `driving-dont-merge-questions` | L232–244 | procedural | — | — |
| `driving-record-decisions-inline` | L245–263 | procedural | — | — |
| `pending-driving-evidence` | L264–EOF | triggering | `*` | — (never) |

Four triggering units in the region, seven procedural — the inverse of
`guides-k24`'s shape, and what the node brief predicted for the one file whose
`## When to …` heads let it root itself.

### Coverage proved by reconstruction, not by arithmetic

The plan's byte sums are orientation. What was actually checked, against
`jj file show -r '@-' content/driving.md`:

- the 11 units, fetched in one `grove-llm methodology` call and stripped of
  their marker lines, are **byte-identical to baseline L2–263**;
- `pending-driving-evidence`, likewise stripped, is **byte-identical to baseline
  L264–EOF**;
- the whole file with every `<!-- unit: ` line removed is **byte-identical to the
  baseline with its seed marker removed** — so no prose, filename or fence moved,
  and the trailing newline and fence balance are untouched.

Together those are a partition proof for this region: no gap, no overlap, nothing
edited. **A first attempt at this check was vacuous and is worth flagging to the
reviewer as a method note:** `jj file show '@-' <path>` reads *both* arguments as
paths, so it silently returned the working copy and diffed it against itself. The
`-r` form is the one that fetches a revision. A later batch reusing this check
should confirm the baseline's byte count against the node brief's corpus table
first — 42,744 here — which is what caught it.

The per-unit byte sum came to 13,532 against an expected 13,534 and that gap was
in the expectation, not the marking: the seed marker line is 48 bytes, not 46, and
13,580 − 48 = 13,532 exactly.

### The two debatable calls the leaf asked me to state

**`## When not to start a grove` is `class=triggering kinds=requirements`.**

The leaf brief was right that most of the section addresses a human deciding
whether to run `grove` — the closing sentence about bare `grove` scaffolding a
tree is squarely that. But the middle of the paragraph is addressed to a session
outright: *"If a **first bootstrap session** surfaces no real fog … do the work
directly instead"*. A first bootstrap session is a session in this loop, it is
`requirements` by construction (`task-bootstrap-leaf-is-requirements` — the driver
mints it with no `--kind` to change it), and it is HITL, so telling the human
"this does not need a grove" is a move it can actually make. That is a condition
reaching a mandate honestly, so it is not procedural.

Scope is `requirements` rather than `*` on the asymmetry test read in both
directions: withholding it from a bootstrap session yields the unasked question in
its purest form — a session that dutifully builds a tree for work that needed
none — while the other eighteen kinds cannot act on it at all, because by the time
they run the grove exists and the question has become pruning, which is a different
rule with its own home. The accepted residue is that a *mid-grove* `requirements`
leaf receives advice about starting that it will usually not need; scopes have no
finer granularity than the kind, and the noise is one paragraph.

The paragraph could not have been split even if the calls differed: it is
contiguous prose, markers are whole unindented lines, and this pass edits no prose.

**The licence comment (baseline L37–40) opens the unit, not the one before it.**
The marker sits above the comment, so the attribution travels with the prose it
attributes, and the blank line above the marker belongs to `driving-field-guide` —
the marker-placement convention applied, not a judgement.

**`## When to invoke a design discussion (grilling)` is `kinds=*`, not
`kinds=requirements`.** Its own first sentence names a `requirements` leaf, which
is the pull toward the narrow scope, but the condition is *questions interdepend
and a decision is about to be made on the human's behalf*, and the sessions that
most need it are the ones that would otherwise decide unilaterally: a `planning`
session choosing what kind of leaf to cut, or any kind hitting interdependent
design questions mid-session and reaching for Decompose. That is grove's primary
failure mode, so the asymmetry decides it. The clause *"without the LLM making
decisions on the human's behalf"* ships at `kinds=*` nowhere else —
`task-producer-requirements` carries the mechanism but not that reason.

Consequence the reviewer should weigh: with this unit at `kinds=*` and deferring
to `grilling-interrogate`, `grilling.md`'s procedure becomes **reachable from every
kind's mandate**, where the plan's rooting table had it reachable from
`requirements` alone (rows 1 and 4). Nothing is put *into* another kind's mandate
and no check objects — reachability is per kind and additive — but it widens what
the plan described, so it is stated rather than left to a diff.

### The one boundary this batch chose that the plan did not specify

**`## Running the vendor pair` is two units, split at *"The combine step's job…"*
(baseline L172).**

The seam is an **addressee change**, not an edge: everything above L172 is
addressed to the session *commissioning* a pair — cut it with one call, two kinds
not one, the bare stem, give both researchers the same brief, do not run them
adversarially — and L172–180 is addressed to the `combine-research` session
*executing* the union. `guides-k24` warned against letting "owns an edge" become
the boundary criterion, so the split is justified on the document's own structure
first; the edge below is a consequence, not the reason.

The fence at baseline L135–137 sits well inside the first unit; nothing was split
mid-fence.

**Everything else was left whole.** `## How to write a research leaf brief` has
four bold-lead paragraphs and one blockquote but one addressee and one subject, so
it is one unit; `## When to commission prior-art research` keeps its *Signs you
want a research leaf* list and its placement paragraph, because the signs **are**
the condition. The four `###` grilling moves are four units, following
`guides-k24`'s rule and its `grilling.md` precedent — they are sections, not items
in one list.

### Edge inventory rows owned: 34 and 35

| row | source | target | outcome |
|---|---|---|---|
| 34 | `driving-when-to-invoke-grilling` (triggering `*`) | `grilling-interrogate` | **written** |
| 35 | `driving-running-the-vendor-pair` | `TASK-FORMAT.md` §*The vendor pair* / §*What the shapes are not* | **declined — citations, not edges** |

**Row 34** points at `grilling.md`'s entry rather than at a move, because the prose
cites the file (*"The grilling skill (`grilling.md`) says it briefly…"*) and the
entry chains to all eight moves. That is `guides-k24`'s rows 2/5 precedent applied.

**Row 35 declined, as the leaf predicted.** Both hits are parenthetical
`` (`TASK-FORMAT.md`) `` citations propping up a claim the sentence is already
making — L141 *"There is no node directory"*, L156 *"a fact the filename already
carries"* — not "the rest is over there". Independently, `task-what-shapes-are-not`
is `class=triggering`, so half of row 35 was never a legal `defers=` target.

**Inventory addition — one.**

| # | source | target | why |
|---|---|---|---|
| A2 | `task-combine-research` (triggering, `kinds=combine-research`) | `driving-the-combine-step` | Later-carved endpoint is mine, so the edge is mine. `task-combine-research` already states the condition — *"This kind, not either producer, carries the **adversarial** move: agreement without independent primary sourcing is a red flag"* — and `driving.md` L172–180 is the operational expansion of exactly that sentence: ask, per agreed claim, whether the two surveys reached it through *different* primary sources. An honest condition→body edge, and it is `driving-the-combine-step`'s **only** inbound edge, so there is no second path hiding a dropped one. |

**A repeated-rule family the inventory does not list**, recorded per the node
brief's default rather than decided freehand:

> **G — the combine step's adversarial move.** Sites: `TASK-FORMAT.md`
> `task-combine-research` (#2) — states it completely and earliest → **Owner**,
> `kinds=combine-research`; `driving.md` L172–180 (#5) — later complete statement
> → **Body**. `driving.md` L167–170 (*"Do not run the researchers
> adversarially"*) is a **mention** inside `driving-running-the-vendor-pair`: it
> states the *converse* rule about the producers, and its subject is commissioning.
> No third site.

**One edge considered and declined**, recorded because the reviewer would
otherwise have to re-derive why it is absent: `task-two-shapes` (family-F owner,
`kinds=*`) → `driving-running-the-vendor-pair` would mirror row 33, which gives the
same owner an address for `driving.md`'s review-chain expansion. It is genuine but
**redundant in delivery**: row 33 is the review-chain section's *only* root,
whereas the vendor-pair section is already rooted from
`driving-when-to-commission-prior-art-research`, which is itself `kinds=*` — so no
session lacks an address, and the brief's own justification for multiple inbound
edges (*"or the second condition's session has no address for it"*) does not fire.
Writing it would have bought nothing and created the two-paths-one-dropped-silently
shape the plan warns about. If the reviewer disagrees, adding it is the decoupling
lemma's easy direction.

### The sweeps

```
grep -rn 'driving\.md' content/          # inbound
```

Seven hits, **none of them mine**:

- **`TASK-FORMAT.md` L108** — `task-producer-impl` (`kinds=impl`): *"(`driving.md`
  carries the habits: cite framework decisions to the source, doubt a
  hard-to-reverse decision before it stands, and externalize surfaced work…)"*.
  Three targets, in **#6, #7 and #8**'s regions respectively. Not mine on the
  later-endpoint rule, and not writable by me at all — see the design finding
  below. **Added to the node brief's inventory as rows 38–40**, one per owning
  batch, because a note in this body is not something #6, #7 and #8 read.
- **`TASK-FORMAT.md` L224** — inside `task-two-shapes`: *"a one-file change wants a
  mid-session subagent instead (`driving.md`)"*. Points at §*Doubting inside a
  picked Grove leaf* — the same target as **row 11**, which already roots that
  section from the family-A owner — so this is a redundancy call for **#7**, of the
  same shape as the `task-two-shapes` → vendor-pair edge I declined above. Note it
  is *not* row 33: row 33's target is §*The review chain*, a different section.
- **`SKILL.md` L236, L252, L280** — all inside `pending-skill-loop`. **No edge may
  have a `pending-*` source**, so nothing was written and nothing was parked. They
  are rows 23 (#9), 19 (#9) and #10's territory.
- **`SKILL.md` L755** — the `## Reference files` index row, which names this file's
  contents almost unit for unit (*"when to commission prior-art research, how to
  write a research-leaf brief, grilling moves (WDYT, pushback, running log)"*).
  **Standing sweep exclusion**, already settled in the node brief.
- **`BRIEF-FORMAT.md` L81** — *"see driving.md, 'Recording fog…'"*. Both endpoints
  are **#8**'s.

Outbound, inside the region — four cross-file references, one written:

- **L4** — *"The grove `SKILL.md` and `grilling.md` files state *what* the loop
  is"*, in the framing unit. **Declined**: an orientation contrast (*what* vs
  *how*), not "the rest is over there". `guides-k24` flagged this hit as mine to
  decide; this is the decision.
- **L39** — inside the licence comment, and it names an *upstream* path
  (`skills/engineering/wayfinder/SKILL.md`). A sweep false positive, not a corpus
  reference at all.
- **L141, L156** — row 35, declined above.
- **L189** — row 34, written.
- **L256** — *"(`SKILL.md` constraint 1)"* supporting the claim that phase files are
  forbidden. **Declined**: a citation, and `skill-spine-constraints` is
  `class=triggering`, so it was never a legal target.

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

1. **`driving-when-not-to-start-a-grove` (scope).** The section is genuinely
   mixed-addressee and unsplittable, and I scoped it to the one kind that can act.
   The competing readings are `kinds=*` (cheap, and lets any session say "this
   didn't need a grove") and `class=procedural` rooted from the framing unit (which
   the leaf brief offered). If the reviewer prefers either, this is the unit to
   change; nothing defers to it, so the change is local.
2. **`driving-when-to-invoke-grilling` (scope, and its knock-on).** `kinds=*` is
   the call I am least sure about in the batch, because it widens `grilling.md`'s
   reachability beyond the two `kinds=requirements` roots the plan's table
   described. Narrowing it to `kinds=requirements` would keep the plan's shape and
   is a one-word edit; what would be lost is the planning/impl session that never
   learns interdependent questions are a signal to cut a `requirements` leaf.
3. **`driving-the-combine-step` (existence).** The split is defensible on
   addressee, but it is the one boundary in this batch the plan did not name, and
   it exists to carry addition A2. If the reviewer rejects the split, re-fusing it
   into `driving-running-the-vendor-pair` also requires moving A2's target — or
   dropping A2, which would leave `task-combine-research`'s mandate with no address
   for the expansion of its own rule.
4. **`driving-field-guide` (class, and the `## Reference files` parallel).** It is
   `triggering` because rows 12 and 13 root from it and a root must be triggering —
   but it fuses a genuine framing statement with an 18-line **index of section
   anchors**, and the node brief made `SKILL.md`'s `## Reference files`
   **procedural** on the reasoning that an index promises navigation the delivery
   path cannot honour. The distinction I relied on: `## Reference files` names
   *files*, which `grove-llm methodology` cannot address at all, whereas this index
   names *subjects*, which a session can carry to the listing and turn into unit
   ids. That is a real difference but a thin one, and it is worth the reviewer's
   attention because the fused unit ships ~1.9 kB into all nineteen mandates.
   Splitting the index off is **not** available without cost: row 13
   (`## Anti-patterns`, `## The shortest version`) has no honest root in the opening
   prose — the index is what names those sections — so a split would strand #8.

### Design findings

- **A file-level citation into `driving.md` cannot use `guides-k24`'s
  file-entry indirection, and that is a structural asymmetry between the two
  files.** `guides-k24` resolved *"see `ADR-FORMAT.md`"*-shaped references by
  deferring to the file's **entry** unit (`adr-placement-note`, procedural) and
  letting the chain reach the sections. `driving.md`'s entry is
  `driving-field-guide`, which is **triggering**, and a `defers=` naming a
  triggering unit is a build error — so `TASK-FORMAT.md` L108's *"`driving.md`
  carries the habits: …"* must name the three section bodies **individually**,
  across three different batches (#6, #7, #8), or lose the edge. No batch owns all
  three, and the later-endpoint rule splits it: #6 for the source-grounding body,
  #7 for doubting, #8 for externalizing. **This is a coordination hazard the
  inventory does not list**, and the failure is silent in the usual way — each of
  the three bodies is reachable by another route, so dropping any one of these
  three members leaves `cargo build` green. Flagged here so #6, #7 and #8 each
  write their member of `task-producer-impl`'s `defers=` rather than each assuming
  another will. **Recorded as inventory rows 38–40 in the node brief**, not only
  here: the inventory is the net that a leaf body is not, and this is precisely the
  shape it exists for — a source carved earlier, targets carved later, and no
  single owner.
- **Prose that is neither condition nor procedure: one instance, and it is small.**
  Baseline L14–16 (*"The examples are stated as reusable shapes rather than as the
  history of the workstream that first produced them. A finished grove deletes its
  task tree; the lesson belongs here only when it still helps a future session."*)
  is an **authoring** note — a rule for whoever edits `driving.md`, not for any
  session receiving a mandate. It rides inside `driving-field-guide` and ships in
  all nineteen mandates as a consequence of the framing unit's class. It is three
  lines, so it was not worth a boundary; recording it because the node brief asked
  for exactly this to be said rather than forced into a class, and because it is
  the same species of narrative residue as the `## Reference files` index.
- **The `## In this guide` anchors are the corpus's only self-referential
  *addresses*, and mandate delivery makes them dead links.** Every entry is a
  markdown anchor (`#externalizing-surfaced-work`) that resolves only if the reader
  holds the whole file. A session holding the slice holds neither the file nor an
  anchor resolver. The index is still useful as a *subject* list (doubt 4), but the
  link syntax promises something delivery cannot do. Whether it survives — and in
  what form — is the successor grove's call, alongside `## Reference files`.
