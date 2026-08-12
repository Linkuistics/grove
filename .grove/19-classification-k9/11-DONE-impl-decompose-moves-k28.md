# decompose-moves-k28

## Goal

Classify **`content/driving.md` from `## Externalizing surfaced work` to end of
file** (baseline L587–754, 9,508 bytes) and **`content/BRIEF-FORMAT.md` whole**
(4,568 bytes) — 14,076 bytes together.

`driving.md`: `## Externalizing surfaced work`, `## Find working increments before
child leaves`, `## What a good child leaf looks like`, `## Recording fog without
pre-slicing it`, `## Prune, reorder, or file an issue`, `## Anti-patterns`,
`## The shortest version`.

This is batch 8 of 12. It **finishes `driving.md`** and is the last batch before
`SKILL.md`'s middle.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L587–754 is a baseline coordinate.** Carve from `##
  Externalizing surfaced work` to end of file, consuming `pending-driving-decompose`
  in full. **`driving.md` is finished after this batch — mint no residual for it.**
- Carve `content/BRIEF-FORMAT.md` whole; the seed id `brief-format` is consumed
  and no residual is minted.
- **There is nothing to inherit from `pending-driving-decompose`.** A residual never
  carries `defers=`.

### The pre-decided call: `## Externalizing surfaced work` is a *body*

`batches-k13` called this section "the paradigm triggering unit for the entire
design". The corpus disagrees with that in its own words, and the node brief follows
the corpus: L595 reads *"`SKILL.md`'s Decompose step **states the rule**; this is the
**habit that honours it**."*

So family B's **owner** is `SKILL.md` `**Decompose.**` (#9), and this section is its
**body** — `class=procedural`. The rule is no less load-bearing for that; what
changes is only which byte span ships it.

Because your batch runs **before** its owner, you cannot root it from the owner.
**Root it from `driving.md`'s framing unit** (`research-moves-k25` carved it; its id
is in that leaf's body) — inventory **row 12**. That is an honest root, not a
reachability crutch: the framing unit's opening names *"externalizing surfaced work
into new leaves rather than absorbing it"* outright and `## In this guide` indexes
this section by name. #9 later adds the owner's address as row 18.

### Why `BRIEF-FORMAT.md` is in this batch

`BRIEF-FORMAT.md` carries **no condition of its own** — it opens on a statement
("Every node in a grove is a **directory**, and it carries a brief…"), not a
question a session could fail to ask. It needs roots, and two are in this batch's own
`driving.md` region.

### Edge inventory rows owned: 12–16

| row | edge |
|---|---|
| 12 | `driving.md` framing unit → `## Externalizing surfaced work` |
| 13 | `driving.md` framing unit → `## Anti-patterns`, `## The shortest version` |
| 14 | `## Recording fog without pre-slicing it` (L670–684) → `BRIEF-FORMAT.md` §*On the horizon* body — it cites that note explicitly, a clean trigger→body edge |
| 15 | `## What a good child leaf looks like` (L644–669) → `BRIEF-FORMAT.md` bodies — the condition a `planning` session faces when about to write a child brief |
| 16 | `TASK-FORMAT.md`'s `planning` bullet (L485–486, carved by `kinds-k22`) → `BRIEF-FORMAT.md` bodies |

Run the sweep as evidence alongside them:

```
grep -rn 'BRIEF-FORMAT\.md' content/
```

`SKILL.md`'s `**Decompose.**` is a fourth inbound path to `BRIEF-FORMAT.md`, and it
is **row 19, owned by `execute-k29`**. It still sits in a `pending-skill-*` unit, and
**no edge may have a `pending-*` source** — park nothing, and report the hit as *not
yours*.

### The two orphan sections, and the root they need

`## Anti-patterns` (1,112 bytes) and `## The shortest version` (608 bytes) state
no condition of their own — they are a summary and a digest. They must still be
reachable, and row 13 is how. If you conclude they are
narrative rather than procedure — there to make the document readable and neither
condition nor body — **say so as a finding about the design** rather than forcing
a class; the node brief asks for exactly that, and a summary of a document that is
delivered in slices is a fair candidate.

### The judgement this batch exists for

`## Externalizing surfaced work` (2,331 bytes) states grove's **primary failure
mode** — a session quietly absorbing work that should have been its own leaf. The
asymmetry argument in the node brief's *The rule* is written about exactly this case,
which is why its *grain* matters even though its class is settled: split the
condition-shaped opening from the two-triggers mechanics only if each half reads
correctly standing alone.

`## Prune, reorder, or file an issue` (2,233 bytes) states a condition (*a leaf's
place in the tree is in doubt*) and then a triage. Note that pruning is **HITL** —
an agent never prunes on its own — which makes the condition load-bearing in a way
the triage is not.

## Done when

- The `driving.md` region and `content/BRIEF-FORMAT.md` are subdivided into real
  units. **No `pending-driving-*` unit remains**, and `brief-format` is gone.
- Every procedural unit in `BRIEF-FORMAT.md` is reachable, and the sweep for
  `BRIEF-FORMAT.md` is run and its outcome stated.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- **Rows 12–16 are each reported** — written, or declined with a reason. Rows 14, 15
  and 16 all reach `BRIEF-FORMAT.md`, so the build stays green if one is dropped;
  the inventory is what catches that, not `cargo build`.
- The id of the `## Externalizing surfaced work` body unit is named in this leaf's
  body, so `execute-k29` can write row 18 without re-deriving it.
- The orphan-section call (`## Anti-patterns`, `## The shortest version`) is
  recorded in this leaf's body with its reasoning.

## Notes

- After this batch, `driving.md`, `TASK-FORMAT.md`, `BRIEF-FORMAT.md`,
  `grilling.md`, `SPEC-FORMAT.md`, `CONTEXT-FORMAT.md` and `ADR-FORMAT.md` are all
  finished. Only `SKILL.md` and `prompts/continue.md` remain, and every cross-file
  target the four `SKILL.md` batches need now exists. **State that in your commit
  message** — it is the gate condition for batches 9–12.
- Doubts to carry forward, by id.

## Batch record

**`driving.md` is finished.** Region carved between the anchor and EOF;
`content/BRIEF-FORMAT.md` carved whole. **14 units minted, `pending-driving-decompose`
and the seed `brief-format` consumed, no residual minted**; `EMBEDDED_UNITS`
96 → 108. `cargo build` green; `cargo test` green — **40 test binaries, 1,023
tests, 0 failures**, including `the_embedded_unit_set_is_pinned_complete`.

**The id this leaf's *Done when* asks for, first: the `## Externalizing surfaced
work` body is `driving-externalizing-surfaced-work`** — that is what `execute-k29`
needs for row 18, and it needs no re-derivation.

Only `content/SKILL.md` and `content/prompts/continue.md` remain unclassified;
**every cross-file target batches #9–#12 need now exists.**

### The units, in document order

Ranges are pre-classification baseline coordinates; the byte column is prose only,
excluding the marker line, measured from `grove-llm methodology` rather than by hand.

`driving.md` — nine:

| id | region (baseline) | class | scope | defers |
|---|---|---|---|---|
| `driving-externalizing-surfaced-work` | L587–621 | procedural | — | — |
| `driving-find-working-increments` | L622–642, licence comment included | procedural | — | — |
| `driving-what-a-good-child-leaf-looks-like` | L644–663 | procedural | — | — |
| `driving-recording-fog` | L665–676, licence comment included | **triggering** | `*` | `driving-the-fog-or-ticket-test`, `brief-suggested-shape` |
| `driving-the-fog-or-ticket-test` | L678–683 | procedural | — | — |
| `driving-when-a-leafs-place-is-in-doubt` | L685–691 | **triggering** | `*` | `driving-prune-reorder-or-file-an-issue` |
| `driving-prune-reorder-or-file-an-issue` | L693–719 | procedural | — | — |
| `driving-anti-patterns` | L721–740 | procedural | — | — |
| `driving-the-shortest-version` | L742–754 | procedural | — | — |

`BRIEF-FORMAT.md` — five, **all procedural**, exactly as the node brief's rooting
table predicted (*own conditions? **no***):

| id | region | bytes | defers |
|---|---|---|---|
| `brief-the-node-briefing` | L2–13: reference comment, title, what a brief is and where it lives | 664 | the other four |
| `brief-every-node-carries-one` | L15–39: one node species, nothing is enforced, who writes it and that it is never marked done | 1,633 | — |
| `brief-durable-content` | L41–51, licence comment included | 668 | — |
| `brief-suggested-shape` | L53–87, the fenced template | 1,351 | — |
| `brief-briefs-inherit` | L89–93 | 199 | — |

Those five sum to **4,515 B**, the whole file.

**This batch is the corpus's largest net *reduction* in mandate size.** The two
consumed placeholders were both `class=triggering kinds=*` and shipped 9,508 B
(`pending-driving-decompose`) and 4,515 B (`brief-format`) into all nineteen
mandates. What replaces them is **1,062 B of condition** — `driving-recording-fog`
649 B and `driving-when-a-leafs-place-is-in-doubt` 413 B — and **12,961 B of
addressable procedure**. Net: every mandate loses ~12.9 kB.

### Coverage proved by reconstruction, with the instrument controlled first

`research-moves-k25`'s vacuous-check trap and `evidence-moves-k26`'s positive
control both applied: the stripped pre-batch `driving.md` measured **42,696 B**,
the figure the last two batches independently recorded, and the stripped
`BRIEF-FORMAT.md` **4,515 B**, which is the node brief's 4,568 B corpus figure
minus the 53-byte seed marker. The instrument was proved non-blind before any
comparison.

- the 9 `driving.md` units, fetched in one call and stripped of their marker lines,
  are **byte-identical to the consumed `pending-driving-decompose`'s coverage —
  9,508 B**, this leaf's figure exactly, so there is no gap and no overlap;
- the 5 `BRIEF-FORMAT.md` units, likewise, **reconstruct the whole file byte-for-byte**
  (4,515 B);
- both whole files with every `<!-- unit: ` line removed are **byte-identical to
  their pre-batch selves** — no prose, filename or fence moved, trailing newline and
  fence balance untouched. `jj diff --git content/ | grep -E '^[+-][^+-]' | grep -v
  '<!-- unit: '` is empty.
- `grep -rc '<!-- unit: pending-' content/` returns a single non-zero line,
  `content/SKILL.md:1`.

`BRIEF-FORMAT.md`'s fenced template (L59–87) sits whole inside `brief-suggested-shape`;
nothing was split mid-fence, and `driving.md`'s region contains no fence.

### The two grain calls that departed from this leaf's framing

**Two sections split into a condition and a body, where the leaf named neither as a
split candidate.** Both are the *keep the `if`, defer the `then`* shape, and both
are the reason this batch ships any condition at all.

**§*Recording fog without pre-slicing it* → `driving-recording-fog` (triggering `*`,
649 B) + `driving-the-fog-or-ticket-test` (procedural, 413 B).**

The opening paragraph is a complete `if`: *there is nowhere to keep the dim view of
work you can see coming but can't yet leaf-shape — a brief's **On the horizon** note
is that place*. The second paragraph is the **fog-or-ticket test**, a decision
procedure for acting once the situation is recognised. Applying the asymmetry
directly settled the class: **`grep -rn 'horizon' content/` returns exactly two
sites** — this section and the template row inside `brief-suggested-shape` — so if
this section were procedural, the fact that grove *has* a place for foreseen work
would ship in **no mandate at all**, and the only sessions that could discover it
are the ones already fetching `BRIEF-FORMAT.md`. `SKILL.md` `**Decompose.**` states
the *don't pre-slice* half (*"never speculatively"*) and not the *there is a place*
half. That is the unasked question in its purest form, and it is what makes this
triggering rather than another `driving.md` body.

**§*Prune, reorder, or file an issue* → `driving-when-a-leafs-place-is-in-doubt`
(triggering `*`, 413 B) + `driving-prune-reorder-or-file-an-issue` (procedural,
1,776 B).** The leaf predicted this one (*"states a condition … and then a
triage"*), and the pair sums to 2,233 B — this leaf's figure to the byte. The
condition is the prohibition, complete on its own: *a leaf whose place is in doubt
tempts a status word; resist it — the doubt resolves to one of three existing
mechanisms*. The triage, the prune-scoping paragraph and the misfiling paragraph are
the `then`.

**The boundary leaves the condition ending in a colon, and that was decided, not
overlooked.** `evidence-moves-k26` rejected exactly this shape once — *"would have
left the condition ending in a colon pointing at nothing"* — but there a cleaner
boundary existed one line higher. Here the colon closes the same line as the
condition's last sentence (*"live. The doubt always resolves to one of three existing
mechanisms:"*), markers are whole lines, and this pass edits no prose, so the only
alternative was one 2,233 B triggering unit in all nineteen mandates. Under mandate
delivery the colon's referent is not nothing: it is the `defers=` on the same marker.
Recorded as doubt 2 — it is one marker to remove.

### The grain calls that did not split

**§*Externalizing surfaced work* is one unit (2,005 B), where the leaf licensed a
split.** The licence was conditional — *"only if each half reads correctly standing
alone"* — and they do not. The four paragraphs are a single chain: the rule → *"Two
triggers, two verbs"* → *"How to tell inline from externalize"*, which back-references
both → *"Externalizing is cheap"*, which back-references the verbs. It is smaller than
`driving-turning-a-sweep-into-evidence` (3,377 B) and `driving-doubting-inside-a-picked-leaf`
(2,188 B), both of which stayed whole for the same reason.

**§*Find working increments* and §*What a good child leaf looks like* stayed two
units rather than fusing**, despite sharing one licence comment (see the finding
below). They are two subjects at two granularities, and the second says so in its own
first sentence: *"Externalizing tells you **when** to split; this is what the split
should **produce**."* Increments are grove-level staging; vertical slices are
leaf-level shape, and the first section's title (*"**before** child leaves"*) exists
to keep them apart. Fusing would also have blunted row 19, which names
§*What a good child leaf looks like* individually as a target.

**`BRIEF-FORMAT.md`'s three middle paragraphs fused into `brief-every-node-carries-one`.**
*One node species* → *Nothing is enforced* → *who writes it and it is never marked
done* are one subject — the brief's lifecycle in the tree — and the middle one
explicitly continues the first (*"a lapse to fix, **not a second kind of node**"*).
They are bold-lead paragraphs with one addressee, which is the shape
`research-moves-k25` and `evidence-moves-k26` both kept whole. Five units for
4,515 B is proportionate to `guides-k24`'s three for `CONTEXT-FORMAT.md`'s 3,502 B.

### The orphan-section call, which this leaf required

**`## Anti-patterns` (1,112 B) and `## The shortest version` (608 B) are both
`class=procedural`, rooted from `driving-field-guide` (row 13), and they are *not*
the same call.**

**`## Anti-patterns` is procedure, comfortably.** Its four bullets are rules with
teeth, not narrative: *capture must be one non-interactive gesture*, *don't
reconstruct decisions in a commit message or session-summary file*, *ask about the
specific trade-off*, *a pre-decided question is an `impl` task, not a grilling*.
Three of the four are grilling habits and belong beside the moves `research-moves-k25`
carved; the fourth (the decision summary) restates a rule whose owner already ships
— see family K below. It is one unit because they are items in a single list, which
`guides-k24`'s rule forbids carving inside.

**`## The shortest version` (608 B) is where the leaf's prediction lands, and I am
recording it as the finding rather than forcing the class to carry the argument.**
It is a **digest of a document that is no longer delivered as a document**: *"If you
remember one paragraph of this doc, remember this"* addresses a reader holding all
42 kB, and under mandate delivery no session ever holds `driving.md`. Its blockquote
compresses six sections — commissioning research, the citation discipline, the
grilling moves, the running log, ADR retirement — every one of which the session
either already holds as a condition or can fetch by id. So it duplicates addressable
content in a form that cannot be addressed *to* anything.

I classified it `procedural` rather than inventing a third state, which is what the
node brief asks for (*"say so rather than forcing it into a class"* — the saying is
here, the class is the mechanical residue). **It is a candidate for deletion in the
successor grove**, alongside `SKILL.md`'s `## Reference files` index and
`research-moves-k25`'s `## In this guide` anchors, and it is the third member of the
same species: narrative that exists to orient a reader of a *file*.

### Edge inventory rows owned: 12–16 per this leaf, plus 40 per the node brief

**Row 40 is not in this leaf's table.** It was added to the node brief by
`research-moves-k25` after the leaf bodies were written, and it names #8 as owner;
`evidence-moves-k26` wrote row 38 and `doubt-moves-k27` row 39 from the same
sentence. The node brief is authoritative, so all six are reconciled here.

| row | source | target | outcome |
|---|---|---|---|
| 12 | `driving-field-guide` (triggering `*`) | `driving-externalizing-surfaced-work` | **written** — this section's root, and not optional |
| 13 | `driving-field-guide` | `driving-anti-patterns`, `driving-the-shortest-version` | **written**, both members |
| 14 | `driving-recording-fog` (triggering `*`) | `brief-suggested-shape` | **written** — see the target correction below |
| 15 | `driving-what-a-good-child-leaf-looks-like` | `BRIEF-FORMAT.md` bodies | **declined — no such reference exists, in either direction** |
| 16 | `task-deliverable-planning` (triggering `kinds=planning`) | `brief-the-node-briefing` | **written**, to the file's entry |
| 40 | `task-producer-impl` (triggering `kinds=impl`) | `driving-externalizing-surfaced-work` | **written** — the third member of the three-way parenthesis |

**Row 40 completes `task-producer-impl`'s sentence, and this is the batch that could
finally close it.** `research-moves-k25` found the parenthesis, could write none of
its three members, and recorded rows 38–40 in the node brief precisely because each
of the three targets is reachable by another route — so **dropping a member leaves
`cargo build` green**. All three are now written: 38 by #6, 39 by #7, 40 here. Mine
is redundant with row 12 in reach (`driving-field-guide` is `kinds=*`, so the `impl`
mandate already carries an address), and it is written anyway for the reason the
inventory lists it: the sentence promises three addresses and is only honoured if all
three land.

**Row 14's target is `brief-suggested-shape`, not a §*On the horizon* unit — because
no such unit can exist.** The plan names *"`BRIEF-FORMAT.md` §*On the horizon* body"*,
but `## On the horizon` is **a heading inside the fenced markdown template** at
`BRIEF-FORMAT.md` L79–82, not a section of the file. Splitting to it would be splitting
mid-fence, which the parser forbids. The honest target is the unit that contains it.
Pointing at the section rather than the file's entry follows `guides-k24`'s row 7
precedent and its stated test: the prose names the note (*"A brief's **On the horizon**
note (`BRIEF-FORMAT.md`)"*), so the section-level address is one the prose actually
offers — unlike its rows 2 and 5, where a bare file citation went to the entry.

**Row 15 declined, and the reason is factual before it is a judgement.**
§*What a good child leaf looks like* contains **no reference to `BRIEF-FORMAT.md`**,
and `BRIEF-FORMAT.md` contains none to it — the full corpus sweep is four hits and
neither endpoint appears (see *The sweeps*). The row's justification (*"the condition a
`planning` session faces when about to write a child brief"*) describes a
**co-occurrence**: a planning session both cuts leaves and writes a node brief, but
this section is about the *shape of a leaf's work* — a vertical slice, the independence
test, the expand→contract exception — and a child leaf has no brief. Its own
cross-reference is to `SKILL.md`'s Decompose step. The node brief's own caution applies
in the direction it did not anticipate: *a prose cross-reference is not automatically a
`defers=`* — and here there is not even a cross-reference. Declining costs no
reachability: rows 14 and 16 both reach `BRIEF-FORMAT.md`, and row 16's source is a
triggering unit.

**Consequence the reviewer should weigh:** the node brief's *Which files can root
themselves* table lists `BRIEF-FORMAT.md` as rooted from *"§*Recording fog…* /
§*What a good child leaf looks like*"*. Half of that table row is now declined, and
what replaced it is **stronger** than what the plan described — row 16 roots the file
from a `TASK-FORMAT.md` triggering unit, and row 14's source became triggering. The
file is reachable from `kinds=*` and from `kinds=planning`, where the plan had it
reachable only through two procedural `driving.md` sections.

**Inventory addition — one row, two members.**

| # | source | target | why |
|---|---|---|---|
| A3 | `task-producer-planning` (triggering, `kinds=planning`) | `driving-find-working-increments`, `driving-what-a-good-child-leaf-looks-like` | Later-carved endpoint is mine, so the edge is mine. The source states **both** halves of the rule and nothing more — *"first find the **smallest independently useful working increments** and order them by dependency … Then cut the current increment into **vertical slices**"* — and my two sections are the operational expansion of exactly those two clauses: what counts as an increment (*"The boundary is product behavior, not code location"*, with the worked negative case), and what a good slice looks like (the independence test, the wide-refactor exception). Two honest condition→body edges from one condition, and they are these two sections' **only** inbound edges, so no second path hides a dropped one. |

A3 is unlisted because the plan expected `BRIEF-FORMAT.md` to be these sections'
downstream neighbour and did not look upstream. It has no filename citation in either
direction, so **no sweep could have found it** — it is the same intra-corpus,
filename-free shape as rows 41–42, one file apart.

**Three edges considered and declined**, recorded because each is one marker edit to
reverse and none is visible as an absence.

1. **`brief-suggested-shape` → `driving-the-fog-or-ticket-test`** — the reverse of
   row 14, and **the corpus's only mutual cross-file citation**. `BRIEF-FORMAT.md`
   L81 reads *"a question you can already state precisely, not one you can already
   answer (see driving.md, "Recording fog without pre-slicing it")"*, which is a
   verbatim paraphrase of the fog-or-ticket test. **Declined on reach, but note it
   was constrained first:** written alongside row 14 it would have been legal only
   because the split gave §*Recording fog* a procedural half — had that section
   stayed one triggering unit, the reverse edge would have been an illegal
   `defers=` target, and had row 14 pointed at `brief-the-node-briefing` it would
   have closed a **(T)** cycle and failed the build. What decides it is the node
   brief's own test: `driving-recording-fog` is `kinds=*`, so **every** session
   holding `brief-suggested-shape` already holds the address, and *"the second
   condition's session has no address for it"* does not fire. An unlisted second
   path into a body would be the drop-the-real-one hazard without the inventory's
   visibility.
2. **`brief-suggested-shape` → `spec-test-seams`** — `guides-k24` handed me this
   call explicitly (*"that batch owns the edge and decides whether the template's
   parenthesis is an edge or a citation"*). The template row is *"Test seams this
   subtree's leaves share: `<seam>` (see SPEC-FORMAT.md)"*. **Declined**, and the
   structural reason is `research-moves-k25`'s finding in a second file: the
   parenthesis is a **bare file citation**, `guides-k24`'s answer to a bare file
   citation is the file's *entry* unit, and `SPEC-FORMAT.md`'s entry
   (`spec-when-a-spec-is-written`) is **`class=triggering`** — an illegal `defers=`
   target. So the only writable edge would assert a section-level address the prose
   does not offer. It also costs nothing: that entry is `kinds=*`, so every session
   reaches `spec-suggested-shape` → `spec-test-seams` in two hops from a unit it
   already holds.
3. **`brief-every-node-carries-one` → `TASK-FORMAT.md`** — its
   *"(flat-lazy-review; `TASK-FORMAT.md`)"* parenthesis. **Declined**: a citation
   propping up a claim the sentence has already made in full, and its target
   `task-no-node-for-a-shape` is `class=triggering` besides. This is
   `doubt-moves-k27`'s row 36 test applied unchanged.

**(D), (R) and (T) hold at end of batch.** Every `defers=` written names a declared
`class=procedural` unit; all twelve new procedural units are reachable —
`driving-externalizing-surfaced-work` from `driving-field-guide` (`*`) and
`task-producer-impl` (`impl`); `driving-find-working-increments` and
`driving-what-a-good-child-leaf-looks-like` from `task-producer-planning` (`planning`);
`driving-the-fog-or-ticket-test` from `driving-recording-fog` (`*`);
`driving-prune-reorder-or-file-an-issue` from `driving-when-a-leafs-place-is-in-doubt`
(`*`); `driving-anti-patterns` and `driving-the-shortest-version` from
`driving-field-guide` (`*`); `brief-the-node-briefing` from `task-deliverable-planning`
(`planning`) and its four children from it; `brief-suggested-shape` additionally from
`driving-recording-fog` (`*`). No chain returns anywhere: every target this batch names
either has no `defers=` or (in `brief-the-node-briefing`'s case) points only at units
that have none.

### The sweeps

```
grep -rn 'BRIEF-FORMAT\.md' content/          # inbound, for the file this batch carves
```

**Four hits, two of them mine, and neither of the other two is missed:**

- **`TASK-FORMAT.md` L521** — `task-deliverable-planning`'s *"writes the child
  `BRIEF.md`(s) and ordered leaf files for any node it grows (`BRIEF-FORMAT.md`)"*.
  Row 16, **written** to the entry.
- **`driving.md` L694** — §*Recording fog*'s *"A brief's **On the horizon** note"*.
  Row 14, **written** to `brief-suggested-shape`.
- **`SKILL.md` L250** — inside `pending-skill-loop`, `**Decompose.**`'s *"turn the leaf
  into a node (a brief, `BRIEF-FORMAT.md`, and ordered child leaves)"*. **No edge may
  have a `pending-*` source**; nothing written, nothing parked. This is **row 19**,
  `execute-k29`'s.
- **`SKILL.md` L749** — the `## Reference files` index row. **Standing sweep
  exclusion**, settled in the node brief.

```
grep -rn 'driving\.md' content/               # inbound, for the file this batch finishes
```

Seven hits, the same seven the last three batches enumerated. **One is mine** —
`TASK-FORMAT.md` L108, `task-producer-impl`, row 40, **written**. Six are not:
`TASK-FORMAT.md` L224 (`task-two-shapes` → §*Doubting*, #7's, declined there);
`SKILL.md` L236, L252, L280, all inside `pending-skill-loop` (rows 23 and 19 for #9,
and #10's family-A mention); `SKILL.md` L755, the `## Reference files` index. **The
one hit inside `BRIEF-FORMAT.md` — L81 — is now both endpoints mine**, and it is
decline 1 above. `research-moves-k25` predicted exactly that (*"both endpoints are
#8's"*), three batches early.

Outbound, over the region — **three cross-file references, one written:**

- **L615** (`driving-externalizing-surfaced-work`) — *"`SKILL.md`'s Decompose step
  states the rule; this is the habit that honours it"*. This is the sentence the node
  brief used to settle family B, so it is **not mine to write**: it points at the
  family-B **owner**, a triggering unit, and #9 supplies the edge in the other
  direction as **row 18**. Reported as *not mine*; nothing parked.
- **L639** (same unit) — *"lazy means just-in-time, not few (`SKILL.md` constraint
  4)"*. **Declined**: a citation propping up a claim already made, and
  `skill-spine-constraints` is `class=triggering`. `research-moves-k25`'s L256 call,
  identical shape.
- **L672** (`driving-what-a-good-child-leaf-looks-like`) — *"a second axis alongside
  'fits this session' (`SKILL.md`'s Decompose step)"*. Same target, same class,
  **declined** on the same two grounds; and it is half of **row 19**, which is #9's.
- **L722** (`driving-prune-reorder-or-file-an-issue`) — *"a **prune**
  (`grove-llm leaf-prune`, HITL, `SKILL.md` "Retire")"*. This is **row 37**, and the
  node brief's table assigns it to **#11**. Reported as *not mine*; nothing parked.
  **See the plan defect below** — the brief's prose about row 37 contradicts its own
  table.

A second sweep for the region's *subjects*, since A3 and rows 41–42 both proved the
filename grep blind to references that name no file:

```
grep -rn 'horizon\|fog\|status word\|blocked\|superseded\|vertical slice\|working increment\|anti-pattern' content/
```

It is what found **A3** (`task-producer-planning`'s two clauses, cited nowhere by
filename) and what proved `driving-recording-fog`'s condition ships nowhere else. It
surfaced one further relationship, recorded as a family rather than an edge:
`driving.md` L309 (*"No `superseded by`, no status line"*, inside
`driving-reworking-adrs-and-briefs`) states the no-status-word rule about **ADRs**, a
different subject, and rides as a mention in #6's unit.

### Repeated-rule families this region states — three unlisted, recorded per the default

**I — foreseen work goes to a horizon note, not a speculative leaf.**

| site | batch | verdict |
|---|---|---|
| `driving.md` §*Recording fog without pre-slicing it* opening | #8 | **Owner**, `kinds=*` — the only complete statement in the corpus, and the only `kinds=*` one |
| `driving.md` the fog-or-ticket test | #8 | **Body**, rooted from the owner |
| `BRIEF-FORMAT.md` template row `## On the horizon` (L79–82) | #8 | **Mention** — a template line inside a fence, unsplittable and taking `brief-suggested-shape`'s class |
| `SKILL.md` `**Decompose.**` *"never speculatively"* | #9 | **Mention** — it states the prohibition, not the alternative. Worth #9's eye: it is the half that ships today |

**J — a leaf whose place is in doubt resolves to reorder, issue, or prune; never a
status word.**

| site | batch | verdict |
|---|---|---|
| `driving.md` §*Prune, reorder, or file an issue* condition | #8 | **Owner**, `kinds=*` — the corpus's only statement of the triage, and of *"not ours at all → a GitHub issue, not a leaf"* |
| `driving.md` the three bullets, the prune-scoping and misfiling paragraphs | #8 | **Body** |
| `SKILL.md` `**Retire.**` pruning paragraph | #11 | **Second condition** — a different trigger (*the path is decided against*), and it carries the HITL rule and the `ABANDONED` mechanics. Not duplication |
| `driving.md` *"Scope the prune to the decision…"* (L727–732) | #8 | **Mention** — a near-verbatim restatement of `SKILL.md` L513–519, inside my body and unsplittable from the triage it qualifies |

**K — no session log, no decision summary; the inline log and the ADRs are the
record.** Owner: **`skill-spine-constraints`** (#1, triggering `kinds=*`, constraint
1 — *"No phase file, no session log, no status file"*) — **it ships**.
`driving-record-decisions-inline` (#5, procedural) is the body;
`driving-anti-patterns`' second bullet (*"Don't reconstruct decisions in the commit
message or a session-summary file"*) is a **mention** inside a body. No call to make
— recorded so the aggregate reviewer does not read the anti-pattern as an orphaned
rule.

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

1. **`driving-recording-fog` (class).** Making it triggering is this batch's most
   consequential judgement and the one that departs furthest from the leaf's framing,
   which treated the section only as an edge source. The case is the two-site grep:
   nothing else in the corpus tells a session a horizon note exists. The case against
   is that the situation — *foreseen work too dim to leaf* — may be one the session
   recognises anyway, in which case a `kinds=*` address from `SKILL.md`
   `**Decompose.**` (a unit #9 has not written yet) would do the same job for fewer
   shipped bytes. If the reviewer disagrees, the repair is `class=procedural` plus a
   root — but note it would then need one, and its only honest candidate is
   `SKILL.md` `**Decompose.**`, which is #9's.
2. **`driving-when-a-leafs-place-is-in-doubt` (the dangling colon).** The condition
   ends *"…resolves to one of three existing mechanisms:"* and the three are in the
   deferred body. I argue the colon's referent becomes the `defers=`; a reviewer
   holding `evidence-moves-k26`'s line will read it as the standing-alone defect no
   build checks. Re-fusing is deleting one marker, and it costs 1,776 B in nineteen
   mandates.
3. **`driving-the-shortest-version` (existence).** Classified `procedural` and flagged
   as narrative residue in the same breath. The reviewer's real question is not its
   class but whether the aggregate review should recommend its **deletion** to the
   successor grove, alongside `## Reference files` and the `## In this guide` anchors.
   I think it should; that is a prose edit, not a marking decision.
4. **A3's scope, and the `requirements` bootstrap gap.** Rooting both increment
   sections from `task-producer-planning` (`kinds=planning`) is honest but narrow:
   `task-bootstrap-leaf-is-requirements` says outright that *"a small workstream's
   bootstrap session may go on to cut the leaves itself"*, and such a session receives
   no address for either body. Row 19 closes half of it if #9 writes it (§*What a good
   child leaf looks like* gains a `kinds=*` source); §*Find working increments* would
   remain planning-only. I did **not** reach for `driving-field-guide` to paper over
   this, because the node brief limits that root to rows 12 and 13 by name. **This is
   the one gap in the batch that another batch can still close**, and #9 is where.
5. **`brief-every-node-carries-one` (grain, 1,633 B).** Three paragraphs fused on
   subject cohesion. If the reviewer wants them apart, the honest seam is above
   *"**Nothing is enforced**"* — the first paragraph is *what the tree guarantees*, the
   remaining two are *what is not enforced and who writes it* — and splitting is the
   decoupling lemma's easy direction, since nothing addresses this unit but the entry.
6. **Row 15's decline, in the direction it might be wrong.** I declined on the absence
   of any citation in either direction. A reviewer could hold that the *semantic* edge
   survives the absence — that a `planning` session reading about vertical slices is
   exactly the session about to write a brief. I read that as co-occurrence, and note
   that accepting it would put a **fourth** inbound edge on `BRIEF-FORMAT.md`, which
   is the redundancy shape the node brief warns about and which rows 14, 16 and 19
   already cover between them.

### Design findings

**F15 — the corpus's only *mutual* cross-file citation, and mandate delivery forces a
choice between its two halves.** §*Recording fog* names `BRIEF-FORMAT.md`'s On the
horizon note; that note's template row names §*Recording fog* by title. As prose this
is a helpful round-trip. As a deferral graph it is a **(T)** violation the moment both
are written at the file's natural granularity, and the build would have said so. What
resolved it here was accidental — splitting §*Recording fog* on unrelated grounds
happened to give the pair distinct endpoints — and the outcome (decline 1) is a
judgement, not a rule. **The general shape is worth the reviewer's attention because
nothing in the plan anticipates it:** the inventory's redundancy discipline assumes
edges flow one way, and a genuinely bidirectional prose relationship has no
representation in a DAG. It is also the first case where *which unit boundary you
choose* determines *whether an edge is legal at all*.

**F16 — a licence comment attributes two sections and can only travel with one; this
is the third instance and the first with a named cost.** `driving.md`'s to-tickets
comment attributes *"vertical-slice-rules **and** the wide-refactor expand-contract
exception"* — the first belongs to §*Find working increments* and §*What a good child
leaf looks like*, the second only to the latter. The comment sits above the former, so
`driving-what-a-good-child-leaf-looks-like` ships the wide-refactor material with **no
attribution attached**, exactly as `driving-doubting-inside-a-picked-leaf` does (F12)
and for the same structural reason. `LICENSES/` still carries the licence, so this is
locality rather than compliance — but three instances in one file make it a pattern,
and the fix in every case is the same prose edit: duplicate the comment onto the second
section. **`BRIEF-FORMAT.md` is the counter-example that proves the mechanism**: its
licence comment sits *after* the paragraph it attributes, so the marker-placement
convention keeps them together with no loss.

**F17 — a plan defect in the node brief's row 37 prose, which contradicts its own
table.** The brief reads *"Row 37 is the one edge whose source is carved **after** its
target, so it is written by the source's batch"*. The source is §*Prune, reorder…*
(#8, mine) and the target is `SKILL.md` `**Retire.**` (#11), so the source is carved
**before** the target — and the table's owner column says **#11**, which is what the
later-endpoint rule gives. The table is right and the sentence is wrong. I acted on the
table and reported the hit as *not mine*. Recorded because it is the third plan-coordinate
defect the batches have found (`evidence-moves-k26`'s three, `doubt-moves-k27`'s rows
17/32), all caught the same way — by executing against the rule rather than the prose.

**F18 — one plan byte-figure moves, and the licence-comment convention is why.** This
leaf sizes §*Externalizing surfaced work* at 2,331 B; the unit measures **2,005 B**.
The 326-byte difference is the to-tickets licence comment, which the plan's region
arithmetic assigned to the preceding section and which the marker-placement convention
— applied by `research-moves-k25` and unchallenged since — assigns to the section it
attributes. **The region total is unaffected** (9,508 B, proved above), so this is a
boundary attribution, not a coverage error. The other three figures land exactly: the
§*Prune…* pair sums to 2,233 B, `## Anti-patterns` to 1,112 B, `## The shortest
version` to 608 B.

**F19 — prose that is neither condition nor procedure: one instance, and it is the
largest yet.** `## The shortest version` (608 B), argued above. It joins
`research-moves-k25`'s authoring note (L14–16) and the two indexes as the fourth member
of the same species — narrative addressed to a reader of a *file*, in a corpus that no
longer delivers files. That is now enough instances to be a **finding about the corpus
rather than about any batch**, and the aggregate review is the right place to say so
once.
