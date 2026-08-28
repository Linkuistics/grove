# corpus-rule-ownership

## Problem

The installed prompt corpus states the same rule in several files with no
recorded owner. That is not verbosity, and compressing sentences would not touch
it. The cause is structural: **the corpus has four competing filing axes**, and a
rule usually belongs to all of them.

Take the placement rule for an `integrate-review-*` step. It is a **loop-step**
rule (it governs Decompose), a **kind** rule (it binds the five `review-*`
kinds), a **filename-grammar** rule (it is about positions among siblings), and a
**habit** (it is a judgement a session exercises). Filed by topic, it lands in
every file that owns one of those topics — and it has. It is stated at length in
`content/SKILL.md`, `content/references/decompose.md`, `content/driving.md`,
`content/TASK-FORMAT.md`, `docs/specs/doubt-grove-review-mechanics.md` and
`CONTEXT.md`: six statements of one rule, each carrying its own reasoning, each
free to drift.

The same shape produced the corpus's live contradictions. As the corpus stood
when this design was written, `references/design.md` said an ADR is raised for a
decision "hard to reverse, surprising, **or** a real trade-off" while
`grilling.md` said all three must hold, and nothing decided between them because
nothing said which file owned the test. `references/planning.md` stated the
working-increments rule **twice in its own body**, with `references/execute.md`
stating it a third time — a file duplicating itself is what "no placement rule"
looks like at its clearest.

The only shape check standing over any of this, when this design was written, was
a 500-line ceiling on `SKILL.md` whose own doc comment conceded it "establishes
nothing semantic". *Test seams* below replaces it with per-kind loaded-path
budgets, which is the measure a rule-ownership design can actually be held to.

Two constraints bound any answer. Normative operational material must stay
**embedded under `content/` and reachable by an installed session** — only
`content/` is swept into the harness skill directories, so a rule moved to
`docs/` is unreachable to every session outside this repository and has been
deleted rather than rehomed. And `src/prompt.rs`'s three-part runtime is fixed:
this design changes the prose the guaranteed core points *at*, never the runtime
that points.

## Solution

**File a rule by when a session meets it, not by what it is about.**

**Two ADRs hold the normative statements this area rests on, and this spec states
neither of them.** [Every normative rule has one
owner](../adr/corpus-rules-have-one-owner.md) is the canonical source for the
recorded pair `Bound(R)` · `Occasion(R)`, the closed occasion domain, the seven
ordered placement rules and the reachability edge. [A restatement declares its
class](../adr/restatement-declares-its-class.md) is the canonical source for the
`own` / `trigger` / `none` classes, the trigger grammar and the sharing test.
**Read both before using the inventory below** — the rule numbers, occasion values
and class names used throughout this spec are theirs, and are not reproduced here.
They are two records because either can change without the other: the function can
gain occasion values or a new precedence while the classes stand, and the classes
can be relaxed — or made free by a generator — while every canonical owner stays
put.

What this spec owns is everything downstream of those two decisions: the
**derivation for particular rows**, the **inventory**, the **canonical trigger
sentences**, the **budget arithmetic**, the **contradiction resolutions**, the
**deferral policy**, the **relocation table**, and the **enforcement seams**. Where
a row's owner is surprising, the derivation is stated at the row.

### Row-level consequences of the ordering

Three consequences of the ordered function decide rows the superseded design got
wrong, and each is recorded here rather than re-argued:

- **`escalation-names-the-tradeoff` records `{step:Execute, step:Retire,
  step:Finish}`** — handing a question back to a human happens at all three — and
  the earliest-step rule derives `references/execute.md`.
- **`finish-is-the-drivers-to-discover` records `{step:Retire, step:Finish}`.**
  Its trigger is *the last live leaf retires*, which is an event in Retire; Finish
  is what the driver may launch afterwards. The earliest-step rule therefore
  derives `references/retire.md`, and the rule is **not** the `finish` kind's.
  Filing it at Finish left the eighteen non-finish kinds — the only kinds that can
  commit the error it forbids — pointed at a file they are never routed to.
- **`records-are-current-state` splits in two.** Several `artifact:A` values make
  one row per artifact, so it becomes `adr-set-is-minimum-coherent` and
  `spec-set-is-current-state`, owned by the two format files, and
  `references/execute.md` states neither.

### Why the input had to change

The predecessor of this function took `Bound(R)` alone and mapped "all nineteen
kinds" to *the loop-step reference for the step R governs*. The ADR rejects that
input; two of its concrete failures are inventory facts and belong here.

**It contradicted itself on the ADR test.** `adr-when-to-write` binds all
nineteen kinds, so the old table sent it to a loop-step reference; the design's
own resolution put it in `ADR-FORMAT.md`, an artifact format file. Under the
ordered function, rule 3 fires before rule 6 and the answer is `ADR-FORMAT.md` with
nothing left to decide — the same ordering that splits `records-are-current-state`
above.

**And it was inconsistent with the fixed runtime.** `always(K)` was defined as a
*static* path whose file is `SKILL.md` or `reference_file(k)`, while rows owned by
`execute.md`, `decompose.md`, `retire.md` and `commit.md` were labelled
`always(19)`. `src/prompt.rs:136` maps kinds only to the ten kind references, so a
loop-step reference is never on a static path. Read literally, therefore, "the
narrowest file every bound session already opens" sends **every** all-nineteen
rule to `SKILL.md` — which is the opposite of what the design intends and what
the word budget permits. Rules 4–7 are what actually resolve those rows, and the
load notation below states static and conditional paths as the different things
they are.

### Load predicate notation

- `static(K)` — on the static loaded path of every kind in `K`. **Only three
  things can be static**: the guaranteed core — whose one embedded part is the
  kind's signal file — `SKILL.md`, and `reference_file(k)`. Nothing else may carry
  this value.

  **`K` is written one of six ways, and the list is closed.** A test asserting
  `static(K)` against the runtime has to resolve `K` to a kind set, and a
  notation whose only definition is that resolver is a second source of truth
  hiding in a test:

  | spelling | the kinds it names |
  |---|---|
  | `19` | all nineteen |
  | `18` | all except `finish` — the eighteen whose ending is `content/SIGNAL.md` |
  | `{<label>}` | the one kind with that label, e.g. `{impl}` |
  | `research` | `research-a` and `research-b` |
  | `review-*` | the five `review-<producer>` kinds |
  | `integrate-review-*` | the five `integrate-review-<producer>` kinds |

  A seventh spelling is a change to this table, not a free coinage: an
  unrecognised one resolves to the empty set, which would assert the row against
  nothing.
- `on(<trigger>) @ <file>` — conditional. `<file>` is the file whose sentence
  fires the trigger, and it is part of the predicate, not a note. **`<trigger>` is
  non-empty and is parsed, not skipped**: for an in-file row it *is* the whole
  predicate — which condition inside an open file applies — and for a cross-file
  row it names the situation the realising sentence has to state. A reader that
  looked only for ` @ ` accepted `anything @ F` and `on() @ F` alike and kept the
  suffix, which deletes the trigger silently. The notation is closed: a value that
  is neither form is refused rather than coerced into one.

**The `@` column carries two relations, and only one of them is an edge.** Where
`<file>` is *not* the row's owner, it records **what sends a session to another
file** — a transition, and the thing reachability is about. Where `<file>` **is**
the owner, it records **which condition inside an already-open file applies** — an
in-file condition, which moves the session nowhere. 50 of the inventory's 97
conditional rows are the second kind, including every follow-on row in
`references/driver.md`, `execute.md`, `decompose.md`, `retire.md` and the format
files. Reading those as edges makes each a self-loop and the graph uniformly
cyclic, which would condemn the ordinary case.

> **Cross-file rows only.** For a conditional row with `on(t) @ F` and owner `O`
> where **`F ≠ O`**, the pointer graph carries the edge `F → O`, realised by a
> sentence **in F that names O's path**. The assertions are: (1) `F` is static or
> is the owner of a reachable row; (2) `F` actually names `O`; (3) every chain
> terminates at a static path with no cycles; and (4) **every owner file that is
> not static has at least one incoming edge**.
>
> **A row with `F = O` records no edge and is asserted only on its face**: `F` must
> equal the grouping heading. It contributes nothing to the graph, cannot create a
> cycle, and cannot discharge assertion 4 for its own file.

Assertion 2 is what a file-loadability test cannot express, and assertion 4 is
what catches an owner nothing points at even when every individual row's `@`
resolves. Assertion 4 is also what makes the reflexive carve-out safe: **what is
reached is a file, not a rule**, so once one cross-file edge lands a session in
`decompose.md`, every in-file condition there is available to it — and a file with
*no* incoming edge fails assertion 4 no matter how many reflexive rows it carries.
`BRIEF-FORMAT.md` is legitimately reached from `decompose.md` rather than from
`SKILL.md`, and the chain is what keeps `SKILL.md` a page. A row whose chain does
not terminate — or whose source file names no path to it — is an **unreachable
rule**: present in `content/`, deleted in effect. That failure is what `driving.md`
already does to two `impl` rules today, inside `content/` rather than at its edge.

**The cross-file graph is fourteen edges, realised by 47 rows.** Ten run out of
`SKILL.md` — to `references/grove.md`, `driver.md`, `bootstrap.md`, `execute.md`,
`decompose.md`, `retire.md`, `commit.md`, and to `ADR-FORMAT.md`, `SPEC-FORMAT.md`
and `CONTEXT-FORMAT.md` — and four do not: `references/requirements.md` →
`grilling.md`, `references/decompose.md` → `TASK-FORMAT.md` and → `BRIEF-FORMAT.md`,
and `references/retire.md` → `BRIEF-FORMAT.md`. Every chain terminates at a static
path — `SKILL.md` and `references/requirements.md` are static, and `decompose.md`
and `retire.md` are themselves reached from `SKILL.md` — and none revisits a file.
Every non-static owner appears as a target. `references/finish.md` and
`content/SIGNAL.md` need no incoming edge: both are static for the kinds their
remaining rows bind, which is what the move of
`finish-is-the-drivers-to-discover` out of `finish.md` leaves behind.

The `@` file and the declared class constrain each other; *How this inventory
records the three classes* below states how.

### Where the hard boundary is applied

`docs/` is not provisioned, so *normative material stays embedded* needs no rule of
its own: it is placement rule 1 read backwards, which the owner ADR settles. What
this spec does with it is **apply it per relocation**. Every move authorised below
names the rule it is moving, states why that rule's `Bound` is empty or its
`Occasion` is `none`, and is refused otherwise; the discharge is in *What moves,
and where*, and in the `Relocate →` notes under the inventory's file groups.

### How this inventory records the three classes

The two registers, the `own` / `trigger` / `none` classes, the ≤25-word trigger
grammar and the same-situation-same-owner sharing test are [the restatement
record](../adr/restatement-declares-its-class.md)'s and are not restated here.
What the inventory adds is the per-row application, and three facts about it are
worth stating once:

- **Every row declares a class**, in the `mirror` column, and the cell is the
  whole declaration — `` `own` ``, `` `none` ``, or `` `trigger` `` followed by
  exactly one parenthesised citation, `(sentence N)` or `(shares sentence N)`,
  either optionally carrying a trailing `— <note>` inside the parentheses. The two
  byte-frozen signal rows spell `none` as prose and are the one exception. **The
  grammar is closed and is parsed whole**, because a substring test for the word
  `trigger` accepts `not-trigger` and any prose containing it; and the **plural is
  not in the grammar**, because a reader that half-read `(sentences 1 and 999)`
  declared a citation of 1 and hid an invalid citation of 999 behind it. Admitting
  a plural is a change to the sharing rule, made here first. The declaration is
  checkable against the row's own `@` file. A row whose `@` file is `SKILL.md` must
  declare `trigger` or share another row's sentence, because the sentence realising
  that edge *is* a trigger; `none` is legal only when the `@` file is some other
  file. That pairing is what left `references/driver.md` with no incoming sentence
  at all — two of its rows claimed a `SKILL.md` trigger while declaring that
  `SKILL.md` says nothing.
- **Where two rows share a sentence, the inventory names it in both rows**, and the
  canonical wording is in *The trigger sentences* below. Four superseded pairings
  fail the sharing test and are split there.
- **Per-kind rows carry `none` by construction.** That is most of what `SKILL.md`
  states today, and most of why the file shrinks without anything leaving the
  corpus.

### What `SKILL.md` can hold, arithmetically

A word target is only credible with the classes counted, so this design states the
budget rather than assuming it — and states it over **measured sentences**, not
over a per-sentence ceiling multiplied by a guessed count. The superseded text
multiplied instead, and the product disagreed with its own test seam (19 sentences
against at most 18) and with its own grammar (three of its eight sharing pairs
were compounds the `trigger` class forbids). `skill-router-k4` lands the budget as
an assertion.

| part | measured | budget |
|---|---|---|
| frontmatter `description`, title, intro, section headings | not drafted | ≤120 words |
| the **8 `own` rows** (routing table, spine, one-task-one-session, bootstrap order, mandate, no second pick, stated VCS, HITL/AFK mark) | 192 for seven; the eighth not drafted | ≤240 words |
| the **26 `trigger` sentences** below, each ≤25 words | 302 | ≤480 words |
| **total** | 494 drafted, two parts outstanding | **at most 900 words** |

**There is no floor, and the reason is that nothing has measured one.** The
superseded arithmetic reported a total of "~613" and derived a 600-word floor from
it — but 613 is `120 + 212 + 281`, and the 120 is this table's *ceiling* on a part
nobody had written. Adding a budget to two measurements does not produce a
measurement, and a floor asserted over the sum can only be discharged by writing
words to reach it. That is the padding the previous correction was trying to
prevent, one layer down.

What a floor was for is caught directly and better by the three exact assertions
below: **exactly 26 trigger sentences**, each ≤25 words, and the **eight `own` rows
present**. A silently dropped row fails one of those by name, and says which. A
word count could only report that the file is smaller than expected, and would
pass a file that dropped a row and gained a paragraph of prose.

So the budget is a **ceiling of 900 words** plus the three exact assertions. The
per-sentence ceiling of 25 words binds differently: it is a *shape* rule keeping a
trigger from growing into a procedure. The parts' ceilings sum to 840, so no part
can be spent to its own limit and still break the total. On the drafted content the
file should land near 500–620 words; that is a **projection, not an assertion**,
and `skill-router-k4` records what it actually measured.

**Thirty-one `trigger` rows resolve to twenty-six sentences.** Five sentences
cover two rows each, and each pair is recorded here because it passes the
restatement record's sharing test:

- `retire-before-commit` + `retirement-is-filename-only` (retire.md)
- `triage-picks-the-verb` + `no-fourth-status` (retire.md) — *not*
  `pruning-is-hitl` with `no-fourth-status`, which the superseded prose said while
  the inventory said this; the inventory was right, because these two share the
  situation *a leaf's place is in doubt* and pruning's is *the path looks decided
  against*
- `node-close-is-implicit` + `cascade-is-silent` (retire.md)
- `one-focused-commit` + `name-by-handle` (commit.md)
- `pick-walk-order` + `one-configuration` (driver.md) — the pair that gives
  `references/driver.md` its incoming edge

That is five pairs, so 31 rows − 5 shared = 26 sentences. Four superseded pairings
are **split** rather than kept: the durable artifacts from the plugin prerequisite
(two situations), the ADR test from the spec agreement point (two files), the ADR
set's current-state rule from the spec set's (two files), and
`externalize-by-default` from `bigger-than-brief-decomposes` (two situations —
work that does not serve this goal, and work that does but outgrew it).

#### The trigger sentences

This is the canonical set. `skill-router-k4` writes these words, and the budget
above is measured over them.

| # | sentence | words | rows |
|---|---|---|---|
| 1 | When how this session was launched matters, read `references/driver.md`. | 9 | `pick-walk-order`, `one-configuration` |
| 2 | When the mandated handle resolves to nothing or to a terminal leaf, stop as `references/bootstrap.md` directs. | 16 | `stale-launch-stops` |
| 3 | When considering an in-session reviewer, apply the budget in `references/execute.md`. | 10 | `review-budget` |
| 4 | When about to make a repo-wide claim, verify it as `references/execute.md` requires. | 12 | `verify-repo-claims-with-controls` |
| 5 | When a decision settles, record it as `references/execute.md` directs. | 9 | `decisions-land-as-they-settle` |
| 6 | When handing a question back to a human, frame it as `references/execute.md` directs. | 13 | `escalation-names-the-tradeoff` |
| 7 | When work surfaces that does not serve this leaf's goal, externalise it through `references/decompose.md`. | 14 | `externalize-by-default` |
| 8 | When this leaf proves bigger than its brief, decompose it through `references/decompose.md`. | 12 | `bigger-than-brief-decomposes` |
| 9 | When cutting an integration step, place it by the rule in `references/decompose.md`. | 12 | `integration-placement` |
| 10 | When you foresee work you cannot yet state precisely, follow `references/decompose.md`. | 11 | `fog-or-ticket` |
| 11 | When a design needs lessons this codebase cannot show, follow `references/decompose.md`. | 11 | `prior-art-research-is-its-own-leaf` |
| 12 | When the work is done, retire the leaf as `references/retire.md` directs, before committing. | 13 | `retire-before-commit`, `retirement-is-filename-only` |
| 13 | When this leaf's path looks decided against, stop and ask, as `references/retire.md` directs. | 13 | `pruning-is-hitl` |
| 14 | When a leaf's place is in doubt, choose the verb `references/retire.md` names. | 12 | `triage-picks-the-verb`, `no-fourth-status` |
| 15 | When a node has no live leaf left, close it as `references/retire.md` directs. | 13 | `node-close-is-implicit`, `cascade-is-silent` |
| 16 | When the leaf is retired, commit it as `references/commit.md` directs. | 10 | `one-focused-commit`, `name-by-handle` |
| 17 | When the last live leaf retires, leave finishing to the driver, as `references/retire.md` directs. | 14 | `finish-is-the-drivers-to-discover` |
| 18 | When deciding where a durable artifact belongs, follow `references/grove.md`. | 9 | `durable-artifact-set` |
| 19 | When a rule cites the `linkuistics` plugin, read what binds without it in `references/grove.md`. | 14 | `plugin-prerequisite` |
| 20 | When considering an ADR, apply the test in `ADR-FORMAT.md`. | 9 | `adr-when-to-write` |
| 21 | When changing a recorded decision, rework the set as `ADR-FORMAT.md` directs. | 11 | `adr-set-is-minimum-coherent` |
| 22 | When this increment may be an agreement point, consult `SPEC-FORMAT.md`. | 10 | `spec-at-an-agreement-point` |
| 23 | When changing a spec, keep its set current as `SPEC-FORMAT.md` directs. | 11 | `spec-set-is-current-state` |
| 24 | When a term is resolved, record it as `CONTEXT-FORMAT.md` directs. | 10 | `glossary-is-the-forcing-function` |
| 25 | When an artifact may need review, decide it as `references/decompose.md` directs. | 11 | `review-chain-when-load-bearing` |
| 26 | When a question may need two independent surveys, decide it as `references/decompose.md` directs. | 13 | `vendor-pair-when-load-bearing` |

Total 302 words, longest 16, mean 11.6, and none carries a threshold, a branch, an
enumeration or a step. `skill-router-k4` may reword within the grammar; it may not
change the set without a design change, because the set **is** the reachability
graph's edge list out of `SKILL.md`.

**Sentence 1 states one situation and does not enumerate it.** Its superseded
wording — *launched, picked or configured* — listed the three members of the
`launch` occasion, which the trigger grammar forbids exactly because a list is how a
condition grows back into a test. `launch` is one occasion by the placement
function's own account, so the situation has a name and the sentence uses it; the
two rows still share it legitimately, on one situation and one owner file.

**Sentences 25 and 26 are new, and they close a reachability gap the inventory had
left open.** `chain-is-lazy` and `pair-is-eager` say how the steps of each shape
land once the shape is chosen, and both are reached from inside
`references/decompose.md`. Nothing said **when a session should reach for either
shape in the first place** — and none of sentences 7–11 fires for *my artifact is
load-bearing and finished*, so a producer holding no such condition never opens the
file and never asks. That is the unasked question the condition register exists to
prevent, and it costs 24 words. Both sentences name the situation and defer the
criterion, as sentence 3 does for the reviewer budget.

The superseded inventory carried 39 `SKILL.md` mirror rows whose rule text alone
ran to about 670 words before any router prose. That is what the classes bought,
and it is why the target was reachable at all.

**Why the spine is `own` rather than a pointer.** Six other corpus files —
`BRIEF-FORMAT.md`, `SPEC-FORMAT.md`, `references/driver.md`,
`references/finish.md`, `references/decompose.md` and `references/grove.md` —
cite the constraints **by number** (11 citations, verified with a normalised
sweep whose cross-tree control finds the same class in `docs/` and `.grove/`).
Every one of those files is conditionally loaded, so a session reading
"constraint 4" in one of them can only resolve the number from a static path. The
numbered list is therefore on `SKILL.md`'s static path by derivation, and it is
**one row, not seven** — the list is cited as a unit, so seven rows would be
seven mirrors of something that is one rule.

### A command fact is owned by the CLI

A fifth placement case is not a case: **what a verb does, what a flag defaults
to, what the config schema accepts** is owned by the CLI that generates it. Prose
may name a verb and say *when* to reach for it; transcribing *what it does*
creates a second source that changes on a release the corpus does not see. Such
material is marked **relocate → CLI** in the inventory and is not given a rule ID.

### The inventory is a document, not a grammar

The inventory below gives every rule a stable ID that prose and tests may cite.
**Those IDs are not markers, and none is written into `content/`.** The corpus
carried 140 HTML-comment unit markers with a build gate over them; that
classification was scaffolding, it did its work, and it was deleted. Reviving it
under a new name would re-buy a parser for a claim a parser cannot decide.

Enforcement is therefore **per rule, by the instrument that fits that rule** —
a behavioural eval for a rule about what a session does, a targeted
phrase-uniqueness sweep for a rule whose wording is distinctive, a word budget
for a loaded path — and never a universal partition check. The `test` column
names the instrument; it does not name a grammar.

## Decisions

### The rule grain

One row per rule **a session could violate**, stated at the coarsest grain at
which the violation is still nameable. Finer than that and the inventory becomes
a parser's input; coarser and a restatement hides inside a row.

Completeness is checkable **per owner file**: every normative sentence in a
`content/` file resolves to a row whose canonical source is that file, or to a
row that names it as a declared mirror, or is marked for relocation.

**The audit runs over sentences, not over sections.** The superseded audit walked
each file's headings and missed three classes of rule in three separate files, all
found by a reviewer reading the prose: a global rule stated in a *format* file
(`one-task-is-one-session` and the pruning duplicate in `TASK-FORMAT.md:21-22`), an
imperative whose nearest row decided a *different* question (the bare stem, beside
`name-step-kind-off-the-producer`), and two rules inside a file whose rows had been
enumerated once and treated as closed (`references/finish.md`'s promotion and
decline rules). A section-level audit cannot see any of them, because in each case
the section it belongs to was already accounted for. So a file is complete only when
**every imperative sentence in it** has been dispositioned, and the count of rows a
file had before the audit is not evidence about the count it should have after.

**The audit was promised and then not re-run, so it was re-run.** Announcing the
sentence-level rule and continuing to work from the section list is how a
methodology change becomes decorative; the second pass over the three nominated
files — `content/driving.md`, `content/TASK-FORMAT.md`,
`content/references/finish.md` — found five more rules with no row, every one of
them the same shape as the three that motivated the rule:

| rule | source sentence | why no row existed |
|---|---|---|
| `review-chain-when-load-bearing` | `driving.md:399-402` | its section was in the relocation table with six other rules under it |
| `vendor-pair-when-load-bearing` | `driving.md:92-94` | as above, with four |
| `sweep-scope-is-the-claim` | `driving.md:330-336` | three named narrowing failures, inside a section whose four other rules were listed |
| `no-kind-prefix-in-commit-subject` | `TASK-FORMAT.md:237-239` | a **Commit**-step imperative inside a *filename-grammar* file's argument for the bare stem |
| `finish-resume-reruns-the-same-command` | `finish.md:32-38`, `:57-73` | reads as mechanics beside `finish-commit`, and its negative half is a *different* rule (`absent-tree-proves-nothing`) |

Two of the five are **selection** rules whose construction counterpart *was*
inventoried — `chain-is-lazy` says how a chain's steps land, `pair-is-eager` says
how a pair's three do, and neither says when a session should reach for the shape.
Following the inventory as a worklist would have deleted the criteria with
`driving.md` while leaving the unowned duplicates in `decompose.md:93-96` and
`:115-118` standing. The generalisation worth keeping is that **a construction rule
and a selection rule over the same shape are two rules**, and finding one is no
evidence of having found the other.

### Every row carries five columns, in every table

The requirements ask each normative concept for a rule ID, a canonical source,
permitted mirrors, a load predicate and behavioural tests. That obligation is
schema, not prose: the superseded inventory gave the ten kind references and the
six format files a **two-column** listing — a file and a semicolon-separated rule
name list — which left roughly half the named rules with no mirror class, no load
predicate and no test class at all. **Every table below carries the full record.**
Canonical source is the grouping heading; the four remaining columns are `rule`,
`Bound · Occasion`, `mirror`, `load`, `test`.

#### The inventory's shape, stated exactly

The reader over these tables asserts the totals below as **equalities**, and this
table is where it reads them. They were a floor and two lower bounds in the test
itself, which is a control that gains permanent slack: `corpus-split-k30`
legitimately added a static row, so the inventory rose above the floor and
deleting some *other* static row returned it to the old count with no edge
changed and nothing red. A floor cannot distinguish a row added from a row
swapped.

| total rows | `static(K)` | conditional, in-file | conditional, cross-file |
|---|---|---|---|
| 146 | 49 | 50 | 47 |

Adding, removing or repointing a row therefore edits this table in the same
commit. That is the cost of the equality, and it is the point of it: the number
is a claim about the inventory that a reviewer can check by counting, rather than
a bound that quietly stops meaning anything.

Rule IDs are unique across the whole inventory. A duplicate is invisible to every
assertion over the graph — edge sets deduplicate — so it is refused at the reader
instead.

### Test class

- **B** — behavioural: observable in a session's conduct.
- **S** — structural: a single-source or budget assertion over the corpus, green
  only **after** the rewrite that lands it.
- **B+S** — both apply, and they are different instruments over the same rule.
- **—** — the row is non-normative and is leaving `content/`; nothing asserts it.

Conduct is the property far more often than the superseded classification
allowed. A uniqueness sweep can show that one sentence owns a rule; it cannot
show that a session obeys it, so `no-adjacency-exception`, `no-fourth-status` and
`nothing-after-finish` are **B+S**, not S-only. **S alone is reserved for a rule
whose property genuinely is the corpus's shape** — the spine's freeform-shape and
walk-away clauses, the one-page budget, the plugin-deferral sweep — and for the
uniqueness half of a B+S pair.

**Which B rows belong to `behavior-evals-k3`, and which do not.** `k3` is
chartered green-before-and-after, so it owns the rows inside **eight of the nine
areas the requirements name**, marked **B★** below: no second pick, no VCS
reprobe, stale launch, the decomposition boundary, human-only pruning,
retire → commit → complete, the review budget, all three finish-signal outcomes.
An area may span several rows — *retire → commit → complete* is three — and only
the rows inside an area are starred.

**The ninth area is the exception, and it had to be.** *The interview threshold*
is `grilling-threshold`, and it is a contradiction being resolved rather than a
behaviour being preserved: while the corpus stated both forms, a test for the
threshold was **red until `kind-references-k5` landed the fix**. Giving it to `k3`
would have chartered a green-before-and-after leaf to ship red across three
leaves. It is therefore marked **B (k5)**, and `k5` landed it — as a ninth area
in `tests/lifecycle_invariants.rs`, beside the eight, pinning the *resolution*:
the deliverable is unconditional, the interview is not, and below three
interdependent open questions the session records the decisions and proceeds.

Every other **B** row is landed by the leaf that rewrites its owner file, with
the rewrite it describes. Handing `k3` the whole B set would balloon it; handing
it any **S** row would charter it to fail.

### A rule is never homeless between two commits

Eight leaves rewrite this corpus in sequence, and a rule whose owner changes is
stated in two files that two different leaves own. **The leaf that adds the new
statement is the leaf that removes the old one.** When the two files belong to
different leaves, the move belongs to the **later** leaf, and the earlier one
leaves the old statement in place and untouched. A rule with no statement in
`content/` between two commits is deleted for every session launched in that
window, and in a meta-grove that window is real sessions.

**The later-leaf convention yields when the earlier leaf cannot keep its copy.**
It assumes the earlier leaf is free to leave the old statement where it is, and
`skill-router-k4` is not: its own contract is a `SKILL.md` of eight `own` rows,
26 trigger sentences and nothing else, so every procedure it holds has to go in
that commit whether or not the owner is ready for it. Where the owner file
already stated the rule the removal is free — which was true of most of them, the
duplication being what this design exists to remove. Where it did not, **the
earlier leaf performs the whole move**: the invariant is the point and the
convention is only the cheapest way to satisfy it.

**Five** rules were in that position, each stated in `SKILL.md` and nowhere else
in `content/`. `skill-router-k4` landed four of them in their owners:
`one-focused-commit`'s scope (the artifact, the grow-verb writes, the `DONE`
rename and the cascade's promotions, and why Retire precedes) →
`references/commit.md`; `node-close-is-implicit` and the close's *asks the human
nothing* clause → `references/retire.md`; and `pruning-is-hitl`'s agent-side
half — *an agent never prunes on its own*, which the file gated on human
confirmation without ever forbidding — → `references/retire.md`. Each owner cell
was right; what the inventory could not say is that five owners named a file
which did not yet carry the rule.

The fifth was `durable-artifact-set`, and it was **not** landed: `SKILL.md`'s
artifact table was the corpus's only statement of the four members and of which
of them outlive the grove, and `references/grove.md` — the owner sentence 18
names — argued the constraints, the glossary and the spec grain without ever
listing them. `skill-router-k20` found it and `skill-router-k21` repaired it,
adding *The four artifacts, and which of them outlive the grove* to
`references/grove.md`. The gap is the reason this section's audit is now stated
as an exact count rather than a summary: a rule whose owner does not carry it is
indistinguishable, from inside `SKILL.md`, from one whose owner does.

**A procedure register states the rule, not the condition.** The pruning move
first landed in `references/retire.md` in the condition's own words — *finds its
leaf's path decided against* — which put the situation and the procedure in one
file and made `SKILL.md`'s trigger redundant to a delivery check. That is the
duplication in miniature, and `tests/lifecycle_invariants.rs`'s
condition-severing control caught it: an owner file that restates the situation
cannot fail when its trigger is deleted.

---

### The inventory

Grouped by canonical source, so completeness is checkable a file at a time.

#### `content/SKILL.md` — the condition register (`own` rows)

These eight are the only rules `SKILL.md` owns. Everything else it says is one
`trigger` sentence for a rule owned elsewhere.

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `kind-routing-table` — ten files serve nineteen kinds; open the one your kind names before acting | 19 · orientation | `own` | `static(19)` | B |
| `spine-seven-constraints` — the numbered non-negotiables, cited by number from six other corpus files; lazy means just-in-time, not few | 19 · orientation | `own` | `static(19)` | B+S |
| `one-task-is-one-session` — one leaf is one session's whole work; a leaf too big for that decomposes rather than running long | 19 · orientation | `own` | `static(19)` | B+S |
| `bootstrap-order` — resolve the handle, then glossary → cited ADRs → brief chain root→leaf → task file, and nothing else by reflex | 19 · orientation | `own` | `static(19)` | B |
| `mandate-is-authoritative` — the driver's single pre-session pick is this session's mandate; nothing modulates it | 19 · orientation | `own` | `static(19)` | B★ |
| `no-second-pick` — `grove-llm pick` is a diagnostic, not this session's dispatcher; on disagreement the mandate wins | 19 · orientation | `own` | `static(19)` | B★ |
| `stated-vcs-is-definitive` — the driver's statement wins; do not re-derive it, and disregard a harness banner | 19 · orientation | `own` | `static(19)` | B★ |
| `hitl-afk-mark-predicts` — the mark predicts who is present; it does not permit or forbid, and any kind may stop and ask | 19 · orientation | `own` | `static(19)` | B |

`one-task-is-one-session` is the eighth `own` row, and it was missing from the
superseded inventory while being stated in two files — `content/SKILL.md` and
`content/TASK-FORMAT.md:21-22`. It passes the `own` test exactly as the spine
does: `Occasion = orientation`, because a session that learns it after deciding how
much to absorb has learned it too late, and its whole content is its trigger, so no
procedure remains to defer. `TASK-FORMAT.md`'s statement is a procedure-register
duplicate and is removed by `corpus-split-k6`; the **S** half of its test is that
one file states it.

`bootstrap-order`'s Occasion is `orientation` rather than `step:Bootstrap`
because Bootstrap is the *first* thing a session does: there is no earlier
condition that could send it to a file, so a pointer would arrive after the
moment it governs. `references/bootstrap.md` therefore keeps the procedure that
is more than the list — the `brief-chain` walk, the silently skipped level, what
a stale launch looks like — and does not restate the order.

#### `content/references/grove.md` — what a grove is

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `durable-artifact-set` — the glossary, the ADR set, the spec set and the task tree, and which of them outlives the grove | 19 · context | `trigger` (sentence 18) | `on(deciding where a durable artifact belongs) @ SKILL.md` | B |
| `plugin-prerequisite` — grove requires but does not provision `linkuistics`; every deferral states what binds without it | 19 · context | `trigger` (sentence 19) | `on(a rule cites the plugin) @ SKILL.md` | S |
| `build-boundary-is-the-binary` — editing `content/` reaches no session until the binary is rebuilt and installed | 19 · context | `none` | `on(the grove's subject is grove) @ references/grove.md` | B |

All three are `context`, not `step:Execute`. The superseded rows said
`step:Execute` while naming `references/grove.md` as owner — and rule 6 maps
`step:Execute` to `references/execute.md`, so **the function derived no owner that
was `grove.md` at all** and these cells were hand-assignments. What a session
actually meets here is a standing fact about the grove rather than a moment in its
loop, which is what rule 5 is for; the two rows with a `SKILL.md` trigger now carry
one sentence each, because *deciding where an artifact belongs* and *meeting a
plugin citation* are two situations and one sentence covering both needed the
branch the class forbids.

**Relocate → `docs/ARCHITECTURE.md`:** *The seven constraints, argued* and *Why
the glossary is the forcing function* are argument, not rule — the normative
statements are `spine-seven-constraints` (`SKILL.md`) and
`glossary-is-the-forcing-function` (`CONTEXT-FORMAT.md`), and `Occasion = none`
for the argument itself. **Relocate → `SPEC-FORMAT.md`:** grove.md's *Specs*
section restates `spec-membership-test` and `spec-grain-rule`; `Bound ≠ ∅`, so
they collapse into their owner and grove.md keeps a pointer.

#### `content/references/driver.md` — how this session was launched

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `pick-walk-order` — first live leaf in pre-order; briefs, terminal leaves and foreign files skipped; `finish` passed over while ordinary work is live | 19 · launch | `trigger` (sentence 1) | `on(how this session was launched matters) @ SKILL.md` | B |
| `one-configuration` — one complete command template per kind; nothing else routes a session | 19 · launch | `trigger` (shares sentence 1) | `on(how this session was launched matters) @ SKILL.md` | S |
| `config-edit-lands-next-session` — the file is revalidated before every mutation and launch; an edit reaches the *next* session, and an invalid file leaves this leaf live | 19 · launch | `none` | `on(the configuration is wrong or edited) @ references/driver.md` | B |
| `restart-equals-continuation` — the loop holds no state; a task that dies before its commit boundary is simply redone | 19 · launch | `none` | `on(resuming) @ references/driver.md` | B |
| `scaffold-is-the-drivers` — a session never scaffolds `.grove/`; it starts at Bootstrap like every other, and its commit folds the scaffold in | 19 · launch | `none` | `on(the tree was just created) @ references/driver.md` | B |
| `session-name-suggested-once` — suggest `/rename` once if the template passed no `${session_name}`, then move on; derive `<name>` from the workspace root and `<repo-basename>` from the main repo | 19 · launch | `none` | `on(the session name does not match) @ references/driver.md` | B |
| `migration-is-the-drivers` — the one migratable legacy shape is converted by bare `grove` before your session; the two older layouts are refused, and neither is yours to convert by hand | 19 · launch | `none` | `on(the tree looks legacy) @ references/driver.md` | B |

**Sentence 1 is what makes this file reachable.** Its first two rows previously
declared `mirror = none` while claiming `@ SKILL.md`, and no other row anywhere
named `references/driver.md`, so after the rewrite no session had a sentence
sending it here — the unreachable-rule failure, in the file that explains how the
session was launched. The two rows share one sentence legitimately: both are
`Occasion = launch`, both are owned by this file, and the situation *a question
about how this session was launched, picked or configured* is one situation, which
is exactly what rule 4 treats as one moment. The remaining five rows chain from
here.

**Relocate → `references/requirements.md`:** driver.md's *what the scaffold
creates* paragraph tells the bootstrap session to grill, record the outcome in the
root brief, and then either cut the leaves itself or add a `planning` leaf. That
is `requirements`-kind conduct (`Bound = {requirements}`, so rule 2 fires) and it
is stated in `references/requirements.md` too — the file keeps the scaffold's
*mechanics* and drops the session's instructions.
**Relocate → CLI / `docs/USAGE.md`:** the `brew install` line, "no subcommand, no
flags", and the dispatch table are command facts and human-operator material.

#### `content/references/bootstrap.md` — assembling the mandate

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `stale-launch-stops` — a handle resolving to nothing or to a terminal leaf is a stale launch, not work to redo | 19 · step:Bootstrap | `trigger` (sentence 2) | `on(the handle does not resolve to a live leaf) @ SKILL.md` | B★ |
| `brief-chain-tolerates-gaps` — `brief-chain` walks ancestor directories root→leaf and skips a level with no brief silently, so an uncharted node still bootstraps | 19 · step:Bootstrap | `none` | `on(reading the brief chain) @ references/bootstrap.md` | B |

#### `content/references/execute.md` — doing the work

`execute.md` sheds *What each kind produces* entirely: it is a nineteen-kind
summary of material the ten kind references own, and the driver already routed
the session to its own.

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `review-budget` — a picked plain producer may materialise at most **one** in-session reviewer across the whole leaf; a second need is a `review-*` leaf | 19 · step:Execute | `trigger` (sentence 3) | `on(considering a reviewer) @ SKILL.md` | B★ |
| `review-budget-predicate` — the allowance applies only to a session the driver mandated **and** that adopted the mandate by running Bootstrap | 19 · step:Execute | `none` | `on(considering a reviewer) @ references/execute.md` | B |
| `review-budget-by-kind` — the per-kind allowances (reviewed producer: none; `review-*`: none; `integrate-review-*`: one narrow; research trio: none) | 19 · step:Execute | `none` | `on(considering a reviewer) @ references/execute.md` | B |
| `doubt-pass-procedure` — state the claim, strip the conclusion, give one fresh context the artifact and contract adversarially, then classify every finding four ways | 19 · step:Execute | `none` | `on(spending the allowance) @ references/execute.md` | B |
| `escalated-review-routes-through-config` — once review is a leaf, grove owns the route; do not add a competing in-session reviewer | 19 · step:Execute | `none` | `on(a review leaf exists) @ references/execute.md` | B |
| `verify-repo-claims-with-controls` — a repo-wide claim needs a positive **and** a cross-tree control; clean-here alone proves nothing | 19 · step:Execute | `trigger` (sentence 4) | `on(about to make a repo-wide claim) @ SKILL.md` | B |
| `enumerate-then-classify` — extract every candidate from the whole surface and classify each; never sweep a pattern list | 19 · step:Execute | `none` | `on(making a repo-wide claim) @ references/execute.md` | B |
| `sweep-scope-is-the-claim` — grep the claim, not a file list; a path-or-directory scope goes stale and never reaches a file in no tree; a finding against a section does not reach the summary layer | 19 · step:Execute | `none` | `on(making a repo-wide claim) @ references/execute.md` | B |
| `no-self-invalidating-count` — never document a claim with a count of itself; state the structural fact | 19 · step:Execute | `none` | `on(making a repo-wide claim) @ references/execute.md` | B |
| `check-the-rescued-clause` — before deleting a false clause, check whether the true one beside it only reads as true in its company | 19 · step:Execute | `none` | `on(deleting prose) @ references/execute.md` | B |
| `control-must-be-seen-to-fail` — a control that has never come back dirty against a subject known to be wrong is not a control; show it failing before crediting the clean read | 19 · step:Execute | `none` | `on(making a repo-wide claim) @ references/execute.md` | B+S |
| `measure-a-frozen-subject` — finish every edit, then measure; digest every subject either side, reads as well as executables; a run whose subject moved under it is not a measurement | 19 · step:Execute | `none` | `on(reporting a measured result) @ references/execute.md` | B+S |
| `one-measurement-one-writer` — two runs appending to one output produce a self-contradictory reading, and a launcher's return is not a background job's completion | 19 · step:Execute | `none` | `on(reporting a measured result) @ references/execute.md` | B+S |
| `re-run-is-confirmed-item-by-item` — matching totals are consistent with two items moving in opposite directions; compare the per-item record, and without one the re-run confirms nothing | 19 · step:Execute | `none` | `on(reporting a measured result) @ references/execute.md` | B+S |
| `decisions-land-as-they-settle` — append each settled decision to the task file's running log as it settles; never reconstruct them at the end, in a summary file or in the commit message | 19 · step:Execute | `trigger` (sentence 5) | `on(a decision settles) @ SKILL.md` | B |
| `escalation-names-the-tradeoff` — an escalation names the specific trade-off, proposes a recommended answer, and gives the evidence; a general invitation to ask questions is not a prompt | 19 · {step:Execute, step:Retire, step:Finish} | `trigger` (sentence 6) | `on(handing a question back to a human) @ SKILL.md` | B |

#### `content/references/decompose.md` — growing the tree

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `externalize-by-default` — work that does not serve this leaf's stated goal goes to the tree, never inline; the bar is "fits this session", not "I can finish it" | 19 · step:Decompose | `trigger` (sentence 7) | `on(work surfaces that does not serve this goal) @ SKILL.md` | B★ |
| `bigger-than-brief-decomposes` — a leaf that proves bigger becomes a node; do only the first child | 19 · step:Decompose | `trigger` (sentence 8) | `on(the leaf is bigger than its brief) @ SKILL.md` | B★ |
| `vertical-slice` — a child leaf cuts a narrow complete path, demoable without waiting on a sibling | 19 · step:Decompose | `none` | `on(cutting children) @ references/decompose.md` | B |
| `wide-refactor-expand-contract` — a fan-out refactor sequences expand → migrate → contract, one leaf per stage | 19 · step:Decompose | `none` | `on(cutting children) @ references/decompose.md` | B |
| `review-chain-when-load-bearing` — an artifact others will build on — a landed spec, a decomposition, a subsystem — earns a review chain; artifact size and vendor preference are not the test | 19 · step:Decompose | `trigger` (sentence 25) | `on(an artifact may need review) @ SKILL.md` | B |
| `chain-is-lazy` — each step of a review chain is cut by the session before it, only if required, and decided at that session's end | 19 · step:Decompose | `none` | `on(an artifact may need review) @ references/decompose.md` | B |
| `vendor-pair-when-load-bearing` — a question earns a vendor pair only when it is load-bearing enough to pay for two corpora; one survey is the default | 19 · step:Decompose | `trigger` (sentence 26) | `on(a question may need two surveys) @ SKILL.md` | B |
| `pair-is-eager` — a vendor pair lands in one call or not at all | 19 · step:Decompose | `none` | `on(a question needs two corpora) @ references/decompose.md` | B |
| `creating-session-writes-the-body` — the session that knows why a step is needed writes its body | 19 · step:Decompose | `none` | `on(cutting a step) @ references/decompose.md` | B |
| `integration-body-carries-the-handle` — an integration's body names the review by handle and never transcribes its findings; a body that is the finding list leaves the integration no place to reject one, and the rule moves the hazard rather than removing it | 19 · step:Decompose | `none` | `on(cutting a step) @ references/decompose.md` | B+S |
| `name-step-kind-off-the-producer` — `review-<producer>` for the producer that actually ran | 19 · step:Decompose | `none` | `on(cutting a step) @ references/decompose.md` | B |
| `steps-share-the-producers-stem` — every step of a composed shape carries the producer's bare stem as its whole slug; no `-review`, no `-a`, no leading kind word | 19 · step:Decompose | `none` | `on(cutting a step) @ references/decompose.md` | B+S |
| `integration-placement` — `leaf-insert` at the first sibling **entry** after the review whose subtree still holds live work; `leaf-add` when nothing blocks | 19 · step:Decompose | `trigger` (sentence 9) | `on(cutting an integration) @ SKILL.md` | B |
| `no-adjacency-exception` — there is no check an exception could perform; a session that departs owns the drift | 19 · step:Decompose | `none` | `on(cutting an integration) @ references/decompose.md` | B+S |
| `diversity-is-the-configs` — whether a review reaches a different target is two config entries' business; grove compares nothing and warns about nothing | 19 · step:Decompose | `none` | `on(cutting a review step) @ references/decompose.md` | B |
| `fog-or-ticket` — a question you can state precisely earns a leaf now; one you cannot stays a horizon note (`BRIEF-FORMAT.md`) | 19 · step:Decompose | `trigger` (sentence 10) | `on(foreseeing work you cannot yet state) @ SKILL.md` | B |
| `prior-art-research-is-its-own-leaf` — a design depending on lessons the codebase does not show earns a research leaf ahead of it, not a tangent inside it | 19 · step:Decompose | `trigger` (sentence 11) | `on(the codebase cannot answer it) @ SKILL.md` | B |
| `research-brief-names-downstream-questions` — the brief names, leaf by leaf, the downstream questions the survey must answer, and biases the search toward post-mortems | 19 · step:Decompose | `none` | `on(cutting a research leaf) @ references/decompose.md` | B |
| `grow-verbs-are-working-tree-only` — the enclosing task's commit folds them in | 19 · step:Decompose | `none` | `on(growing the tree) @ references/decompose.md` | B |
| `task-and-brief-shape` — a cut leaf's name and body, and a node's brief, follow `TASK-FORMAT.md` and `BRIEF-FORMAT.md` | 19 · step:Decompose | `none` | `on(cutting a leaf or a node) @ references/decompose.md` | B |

`steps-share-the-producers-stem` is a row the superseded inventory did not carry,
and it is not `name-step-kind-off-the-producer` under another name: that row
decides *which kind word* the new leaf takes (`review-design` rather than
`review-impl`), while this one decides the **slug**, independently. It is
imperative in `content/driving.md:469-479` — *give every step the producer's bare
stem* — and discussed as convention-not-grammar in
`content/TASK-FORMAT.md:25-31`. With no row, deleting `driving.md` and shedding
`TASK-FORMAT.md`'s policy would have deleted it outright, which is why the
conditional-deletion table below now carries it. The conduct is `decompose.md`'s;
the grammar half — that a stem is convention and nothing parses it — stays with
`convention-not-grammar` in `TASK-FORMAT.md`, and the split is the same
grammar-here / conduct-there line `suggested-body-shape` sits on.

`task-and-brief-shape` is the row that makes `TASK-FORMAT.md` and
`BRIEF-FORMAT.md` reachable, and it is why neither needs a `SKILL.md` sentence:
the chain `SKILL.md` → `decompose.md` → the two format files terminates at a
static path, which is all the reachability rule asks.

**Relocate → CLI:** decompose.md's transcription of what `leaf-decompose`,
`leaf-add`, `leaf-insert` and `leaf-add-pair` *do* — what each moves, retitles,
creates, gates and prints. `Bound ≠ ∅` for *when* to run them, so the when stays.

#### `content/references/retire.md` — ending a leaf

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `retire-before-commit` — the rename must land inside the task's own commit, and retirement touches one filename and nothing else | 19 · step:Retire | `trigger` (sentence 12) | `on(the work is done) @ SKILL.md` | B★ |
| `retirement-is-filename-only` — not the body, not a sibling, not an ancestor; a waiting review reads the committed artifact | 19 · step:Retire | `trigger` (shares sentence 12) | `on(the work is done) @ SKILL.md` | B |
| `pruning-is-hitl` — an agent never prunes on its own; an AFK session that finds its path decided against says so and stops | 19 · step:Retire | `trigger` (sentence 13) | `on(the path looks decided against) @ SKILL.md` | B★ |
| `prune-scopes-to-the-whole-path` — pruning a reviewed producer leaves its review live and uncheckable; a rejected path is pruned step by step, a chain having no enclosing directory | 19 · step:Retire | `none` | `on(pruning a reviewed producer) @ references/retire.md` | B |
| `no-fourth-status` — no `blocked`, `deferred` or `superseded`; a leaf in doubt gets no status word | 19 · step:Retire | `trigger` (shares sentence 14) | `on(a leaf's place is in doubt) @ SKILL.md` | B+S |
| `triage-picks-the-verb` — *not now* → reorder; *not ours* → an issue; *decided against* → prune | 19 · step:Retire | `trigger` (sentence 14) | `on(a leaf's place is in doubt) @ SKILL.md` | B |
| `node-close-is-implicit` — a node is never marked; its done-ness is the absence of a live child, and the cascade asks the human nothing | 19 · step:Retire | `trigger` (sentence 15) | `on(a node has no live leaf left) @ SKILL.md` | B |
| `cascade-is-silent` — the close recurses upward without stopping | 19 · step:Retire | `trigger` (shares sentence 15) | `on(a node has no live leaf left) @ SKILL.md` | B |
| `node-close-four-steps` — check `Done when`, `leaf-add` the named gap, escalate an unnameable one, promote and report; skip step 1 for an uncharted node | 19 · step:Retire | `none` | `on(a node has no live leaf left) @ references/retire.md` | B |
| `reconcile-records-at-retire` — retirement is where the record sets are reworked and every dangling citation is fixed | 19 · step:Retire | `none` | `on(retiring) @ references/retire.md` | B |
| `finish-is-the-drivers-to-discover` — retiring the last live leaf is an ordinary retirement; a session never concludes the grove is finished, and the driver says so by launching `finish` | 19 · {step:Retire, step:Finish} | `trigger` (sentence 17) | `on(the last live leaf retires) @ SKILL.md` | B |

#### `content/references/commit.md` — the task boundary

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `one-focused-commit` — artifact + grow-verb writes + `DONE` rename + whatever the cascade promoted, together, named by handle | 19 · step:Commit | `trigger` (sentence 16) | `on(the leaf is retired) @ SKILL.md` | B★ |
| `name-by-handle` — name the work item, and each closed node, by `<slug>-k<key>`, never by position or path | 19 · step:Commit | `trigger` (shares sentence 16) | `on(the leaf is retired) @ SKILL.md` | B |
| `no-kind-prefix-in-commit-subject` — do not compensate for the bare stem with a `review:` / `impl:` subject convention; the kind-bearing filename is already in the diff forever | 19 · step:Commit | `none` | `on(writing the commit subject) @ references/commit.md` | B |
| `jj-seal` — in a jj tree, `jj new` **after** describing, once the rename has landed; this is sufficient without the plugin | 19 · step:Commit | `none` | `on(the stated VCS is jj) @ references/commit.md` | B |

#### `content/references/finish.md` and `content/SIGNAL-FINISH.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `finish-confirmation-gate` — propose, and wait for explicit human confirmation before any teardown; with no human, report the plan | {finish} · step:Finish | `none` | `static({finish})` | B |
| `finish-promotes-before-teardown` — promote everything in the briefs that should outlive the grove — ADRs, docs, glossary entries — as working-tree edits, **before** teardown; a near no-op when decisions landed inline | {finish} · step:Finish | `none` | `static({finish})` | B |
| `teardown-via-finish-commit` — never delete `.grove/` by hand; `grove-llm finish-commit <handle>` | {finish} · step:Finish | `none` | `static({finish})` | B |
| `declined-finish-stays-live` — a declined finish leaves its leaf live for a later resume; no session retires the `finish` leaf | {finish} · step:Finish | `none` | `static({finish})` | B |
| `absent-tree-proves-nothing` — an absent `.grove/` never proves teardown succeeded | {finish} · step:Finish | `none` | `static({finish})` | B |
| `recovery-pending-stops` — hand a `Recovery pending` diagnostic to the human; never rewrite history to clear it | {finish} · step:Finish | `none` | `static({finish})` | B |
| `finish-resume-reruns-the-same-command` — if step 2's result is lost, rerun `finish-commit` with the same handle and let the repository answer; a refusal means teardown did not complete, and a no-signal stop after step 2 leaves nothing to resume | {finish} · step:Finish | `none` | `static({finish})` | B |
| `nothing-after-finish` — branch integration and worktree teardown are not grove workflow | {finish} · step:Finish | `none` | `static({finish})` | B+S |
| `finish-three-endings` — teardown → `complete --done`; externalised work → `complete`; declined → no signal | {finish} · step:Finish | **none — byte-frozen and inlined into `${prompt}`** | `static({finish})` | B★ |

#### `content/SIGNAL.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `signal-is-the-last-action` — `grove-llm complete` last, then nothing else; ending without signalling stops the loop | 18 · step:Commit | **none — byte-frozen and inlined into `${prompt}`** | `static(18)` | B★ |

**`finish-is-the-drivers-to-discover` is not this file's, and never was.** It is
tabulated with `references/retire.md` above. Its occasion is `{step:Retire,
step:Finish}` — its trigger is *the last live leaf retires*, an event in Retire,
with Finish being what the driver may launch afterwards — so the earliest-step rule
derives `retire.md`. Filing it at Finish inverted its audience: the rule forbids an
error only the **eighteen non-finish kinds** can commit, and it pointed them at the
one reference file none of them is ever routed to. A `finish` session, which is the
only kind `references/finish.md` is static for, is the one session that cannot
violate it. The row's `SKILL.md` trigger already sent every kind to the right place
in principle; it named the wrong file, and did so for both channels at once.

`finish-promotes-before-teardown` (`content/references/finish.md:18-20`),
`declined-finish-stays-live` (`:75-78`) and
`finish-resume-reruns-the-same-command` (`:32-38`, `:57-73`) were absent from the
superseded seven rows.
All three are **session conduct**, not CLI mechanics: promotion is a judgement
about which brief material is durable, what a decline leaves behind is the rule
that makes a later resume legitimate rather than a lapse, and recovery is what a
session does when it cannot see whether teardown landed. None is derivable from
`finish-confirmation-gate` or `teardown-via-finish-commit`, and a `finish` session
that skipped promotion would destroy the briefs' durable residue in a commit that
cannot be partially undone. `nothing-after-finish` is the boundary *after*
teardown and says nothing about any of them, and `absent-tree-proves-nothing`
forbids the wrong *inference* while saying nothing about the right *action* — which
is the third row.

Both signal files are **out of scope for every rewriting leaf**. They are the one
surface where a wording change is unrecoverable mid-loop, and the guaranteed core
inlines their bytes verbatim, so an edit ships to `${prompt}` and the skill at
once with no channel left to correct it.

#### The ten kind references

Incremental by construction under rule 2: a kind reference states what is true of
**that kind and no sibling**, states nothing a loop-step or format file owns, and
gets **no `SKILL.md` presence at all**.

##### `content/references/requirements.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `requirements-establishes-what` — the deliverable is *what* should be built, in the human's own words | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `grilling-threshold` — the full one-question-at-a-time procedure runs **only** at three or more interdependent open questions; below it, record the decisions and proceed | {requirements} · step:Execute | `none` | `static({requirements})` | B (k5) |
| `sequence-interdependent-questions` — ask the foundational question first and carry its answer into the derived one; never batch two | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `pre-decided-is-not-a-grilling-question` — record a settled answer and move on; all-settled means the work is an `impl` leaf | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `agree-the-seams-during-grilling` — put the sketched test seams to the human before the design is committed (`SPEC-FORMAT.md` records them) | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `small-workstream-may-fuse-the-three` — a bootstrap session may resolve requirements, design and planning in one leaf, or add a `planning` leaf instead | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `when-not-to-start-a-grove` — no session-to-session fog means do the work directly; the scaffold's existence is not the signal | {requirements} · step:Execute | `none` | `static({requirements})` | B |

##### `content/references/design.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `design-deliverable` — a spec, an ADR set, or both | {design} · step:Execute | `none` | `static({design})` | B |
| `design-does-not-cut-impl-leaves` — a `design` session cutting implementation leaves has drifted into planning's job and externalises a `planning` leaf | {design} · step:Execute | `none` | `static({design})` | B |

The OR-form ADR test currently in this file is **deleted**, not rehomed:
`adr-when-to-write` is `ADR-FORMAT.md`'s, and this file may not restate it.

##### `content/references/planning.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `planning-grows-generatively` — the only kind that grows the tree generatively; the deliverable is more tree | {planning} · step:Execute | `none` | `static({planning})` | B |
| `working-increments-before-slices` — find the smallest independently useful working increments and order them by dependency before slicing; stated **once** | {planning} · step:Execute | `none` | `static({planning})` | B+S |
| `planning-writes-the-briefs` — the child briefs and ordered leaf files for any node it grows | {planning} · step:Execute | `none` | `static({planning})` | B |

##### `content/references/prototype.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `prototype-is-throwaway` — the reaction is the deliverable; polish is a defect | {prototype} · step:Execute | `none` | `static({prototype})` | B |

##### `content/references/impl.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `impl-ships` — the deliverable is code, docs or tests that ship | {impl} · step:Execute | `none` | `static({impl})` | B |
| `cite-framework-decisions-to-source` — read the manifest, fetch the official source, cite at the decision site, flag what you could not verify; version-invariant logic, renames and plumbing are exempt | {impl} · step:Execute | `none` | `static({impl})` | B+S |
| `hard-to-reverse-pairs-with-doubt` — where such a decision is also hard to reverse, cite the source **and** spend the leaf's review allowance on a fresh context trying to break it | {impl} · step:Execute | `none` | `static({impl})` | B+S |

`verify-repo-claims-with-controls` does **not** belong here, and the superseded
inventory placed it wrongly. A `review-*` session re-running a producer's sweep, a
`design` session counting occurrences, and a `combine-research` session checking a
claim are all bound by it, so `Bound` is all nineteen and rule 2 does not fire.
It is `execute.md`'s. Only the framework-source discipline is genuinely
`{impl}`-bound.

##### `content/references/review.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `review-is-inspection-only` — inspect the committed diff, source, requirements and recorded evidence; run no test, build, lint or format command and edit nothing | review-\* family · step:Execute | `none` | `static(review-*)` | B |
| `review-output-is-findings-only` — the paired integration owns every fix and all post-fix verification | review-\* family · step:Execute | `none` | `static(review-*)` | B |
| `the-five-reads-differ` — what each of the five reads looks for | review-\* family · step:Execute | `none` | `static(review-*)` | B |

##### `content/references/integrate-review.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `triage-four-ways` — verify, then classify as unclear contract, real issue, real trade-off, or noise; never performative agreement | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B |
| `findings-read-from-the-reviews-commit` — the findings live in the review's commit, not in this task's body; a restatement is a pointer, and grading is against the artifacts | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B+S |
| `what-each-integration-may-change` — the five differ only in what the session may edit | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B |
| `integration-escalates-redesign` — substantial redesign is a new producer review chain beside the leaf being integrated, not this session's work | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B |

##### `content/references/research.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `citation-per-failure-mode-claim` — a primary source per failure-mode claim; a claim without one is mood, not evidence | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `silence-is-a-finding` — record "no primary source found" explicitly; the absence is a confidence signal | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `walk-away-check-per-system` — for each prior tool, answer what stays legible with the tool uninstalled | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `both-researchers-get-one-brief` — the pair buys breadth from differing corpora, not differing questions | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `researchers-are-not-adversarial` — both run breadth-seeking; the adversarial move is the combiner's | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `research-output-path-per-kind` — `research-a` writes `<slug>-a.md`, `research-b` `-b.md`; the kind is the discriminator that stops the second clobbering the first | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |

`research-output-path-per-kind`'s Occasion was `artifact:research doc` — a value
outside the closed artifact domain. A research document is durable, but it has no
format file, so rule 3 has nothing to name and `artifact:` cannot take it; the
domain is exactly the five artifacts a `*-FORMAT.md` file exists for. Rule 2 fired
first for these two rows and masked the invalid value, which is why the owner cells
were right while the schema was not. Should a `RESEARCH-FORMAT.md` ever be written,
the domain gains a sixth member and these rows are recomputed — the domain is
closed by *what format files exist*, which makes it checkable rather than a list to
remember.

##### `content/references/combine-research.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `agreement-without-independent-primary-sourcing-is-a-red-flag` — union the coverage, flag every disagreement, and ask whether an agreed claim reached the two surveys through *different* primary sources; stated **once** | {combine-research} · step:Execute | `none` | `static({combine-research})` | B+S |
| `combine-writes-the-union` — the unadorned `docs/research/<slug>.md` is the union | {combine-research} · step:Execute | `none` | `static({combine-research})` | B |

##### `content/references/finish.md`

Its rows are tabulated above with `SIGNAL-FINISH.md`; the file doubles as the
`finish` kind's own reference.

#### The format files

##### `content/TASK-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `leaf-name-grammar` — five fields, all parsed; position, outcome infix, kind, slug, key | 19 · artifact:task | `none` | `on(cutting or reading a leaf) @ references/decompose.md` | B+S |
| `kind-set-is-closed-at-nineteen` — the closed set, and a missing or unknown kind is malformed rather than an `impl` default | 19 · artifact:task | `none` | `on(cutting a leaf) @ references/decompose.md` | B |
| `handle-is-slug-key` — `<slug>-k<key>` is the work-item identity; a key is never reused | 19 · artifact:task | `none` | `on(naming a work item) @ references/decompose.md` | B |
| `header-is-position-free` — the body's `# <slug>-k<key>` header carries no position and no kind | 19 · artifact:task | `none` | `on(writing a task body) @ references/decompose.md` | B |
| `body-carries-no-launch-metadata` — no kind, harness, model, or record of how a past session ran | 19 · artifact:task | `none` | `on(writing a task body) @ references/decompose.md` | B |
| `suggested-body-shape` — Goal, Context, Done when, Notes, and a `## Decisions (running log)` section when one is being kept | 19 · artifact:task | `none` | `on(writing a task body) @ references/decompose.md` | B |
| `convention-not-grammar` — the shared stem, the relative ordering and the two `**Reviews:**` / `**Integrates:**` lines are written by hand and parsed by nothing; nothing reconstructs a relationship from a name, a position or a body | 19 · artifact:task | `none` | `on(cutting a step) @ references/decompose.md` | B+S |

**`TASK-FORMAT.md` sheds its policy.** It is format grammar, so the composition
shapes, the doubt budget table, the kind disciplines and *A leaf never names a
harness* leave it for `decompose.md`, `execute.md`, the kind references and
`driver.md` respectively. Two smaller passengers leave with them, both found by the
re-run audit: the research **output-path table** (`:69-81`) is
`references/research.md`'s `research-output-path-per-kind`, and the
**commit-subject** imperative (`:237-239`) is `references/commit.md`'s
`no-kind-prefix-in-commit-subject` — a Commit-step rule that reached a
filename-grammar file by riding inside the argument for the bare stem.
`leaf-decompose`'s first-child kind inheritance (`:83-86`) and the `work`
legacy-spelling paragraph (`:88-92`) are **command facts, relocate → CLI**. What
remains is what constrains bytes on disk.
`suggested-body-shape` is where the running log's *section* is described; the
obligation to append to it as decisions settle is `execute.md`'s
`decisions-land-as-they-settle`, and the split is deliberate — grammar here,
conduct there.

`declaration-lines-are-convention` is **widened** to `convention-not-grammar`. The
narrow row named only the two declaration lines, while the file's own sentence
covers the shared stem and the relative ordering in the same breath
(`content/TASK-FORMAT.md:25-31`) — so the row was finer-grained than the rule and a
restatement could hide in the gap, which is exactly what the grain rule forbids.
The conduct half (*give every step the producer's bare stem*) is
`decompose.md`'s `steps-share-the-producers-stem`.

**Two global rules leave this file, and neither was in the superseded inventory.**
`content/TASK-FORMAT.md:21-22` states *one task is one session* and *pruning is
HITL and never an agent's own call* in a single sentence, in a **format-grammar**
file, about neither bytes nor names. The first is `SKILL.md`'s
`one-task-is-one-session` (`own`, `orientation`); the second is
`references/retire.md`'s `pruning-is-hitl`, and this statement is a
procedure-register duplicate of it — a defect under the mirror rule even though the
two agree. `corpus-split-k6` removes both from this file in the commit that lands
them, and by then both owners already state them, so neither is homeless.

##### `content/BRIEF-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `every-node-carries-a-brief` — one species; a node is a leaf that decomposed, and a brief-less node is a lapse a reader still tolerates | 19 · artifact:brief | `none` | `on(cutting a node) @ references/decompose.md` | B |
| `brief-is-process-scaffolding` — neither glossary nor decision log; it exists so a leaf reads three ADRs, not fifty | 19 · artifact:brief | `none` | `on(writing a brief) @ references/decompose.md` | B |
| `briefs-inherit` — a child brief states only what is new at its level; pointers accumulate down the chain | 19 · artifact:brief | `none` | `on(writing a brief) @ references/decompose.md` | B |
| `brief-content-is-durable` — behavioural contracts and named types, not paths or line numbers | 19 · artifact:brief | `none` | `on(writing a brief) @ references/decompose.md` | B |
| `horizon-note-shape` — a line or two of foreseen work, dropped once it graduates to a leaf | 19 · artifact:brief | `none` | `on(recording fog) @ references/decompose.md` | B |
| `brief-is-never-marked-done` — a brief is context, not a task; promotion upward is what a close does with it | 19 · artifact:brief | `none` | `on(a node closes) @ references/retire.md` | B |

##### `content/ADR-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `adr-when-to-write` — **all three**: hard to reverse **and** surprising without context **and** the result of a real trade-off; stated locally, not cited | 19 · artifact:adr | `trigger` (sentence 20 — naming the file, **never** the test) | `on(considering an ADR) @ SKILL.md` | B+S |
| `adr-minimal-template` — the local minimal template | 19 · artifact:adr | `none` | `on(writing an ADR) @ content/ADR-FORMAT.md` | B |
| `adr-placement-and-slug-identity` — `docs/adr/<slug>.md`, the slug is the identity, cite by slug and never by number, directory created lazily | 19 · artifact:adr | `none` | `on(writing an ADR) @ content/ADR-FORMAT.md` | B |
| `adr-set-is-minimum-coherent` — a session that changes a recorded decision reworks the set in place (merge / split / delete) and never appends a superseding record | 19 · artifact:adr | `trigger` (sentence 21) | `on(changing a recorded decision) @ SKILL.md` | B+S |
| `adr-split-is-conditional-on-repo-shape` — split only when the contexts are peers; otherwise one flat root set with ownership recorded in `CONTEXT-MAP.md` | 19 · artifact:adr | `none` | `on(writing an ADR in a multi-context repo) @ content/ADR-FORMAT.md` | B |
| `research-to-adr-bridge` — an adopted finding gets a bridge pointing both ways: the ADR cites the survey by primary source, the survey names the ADRs its findings landed in | 19 · artifact:adr | `none` | `on(adopting a research finding) @ content/ADR-FORMAT.md` | B |

`records-are-current-state` was a single row owned by `references/execute.md`,
restating what `ADR-FORMAT.md` and `SPEC-FORMAT.md` each already say about their
own set. Rule 3 fires per artifact, so it becomes two rows —
`adr-set-is-minimum-coherent` and `spec-set-is-current-state` — and
`references/execute.md` states neither.

**They get two sentences, not one.** The superseded text had them share a single
*records-current-state* trigger naming both files, and a trigger may name **one**
owner path: naming two requires the enumeration the class forbids, and the "or" that
carries it is the same shape as the paraphrase that produced the AND/OR
contradiction. The same applies to `adr-when-to-write` and
`spec-at-an-agreement-point`, which the superseded text also paired: two files, two
situations, two sentences (20 and 22). Splitting costs four sentences and 41 words
across the four rows, and the measured total absorbs it with room to spare.

##### `content/SPEC-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `spec-at-an-agreement-point` — written only at a genuine agreement point, by `design`; most increments write none | 19 · artifact:spec | `trigger` (sentence 22) | `on(the increment may be an agreement point) @ SKILL.md` | B |
| `spec-membership-test` — would a session on an unrelated future grove need to read this? if not it is a `BRIEF.md` | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B |
| `spec-grain-rule` — an ADR records one decision; a spec describes how an area works and **cites** the ADRs in its area rather than restating them | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B+S |
| `spec-set-is-current-state` — edited, merged, split and deleted in place; never dated, numbered or superseded | 19 · artifact:spec | `trigger` (sentence 23) | `on(changing a spec) @ SKILL.md` | B+S |
| `spec-synthesises-never-re-interviews` — the grilling already happened upstream; a spec synthesises its running log | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B |
| `spec-is-behavioural-not-procedural` — interfaces, types and contracts; no paths, no line numbers, no code but a decision-encoding snippet | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B |
| `test-seams-agreed-and-recorded` — prefer existing seams, propose a new one at the highest point, drive the count toward one, and record the agreement in the spec's `## Test seams` or the node's brief | 19 · artifact:spec | `none` | `on(the increment covers tested code) @ content/SPEC-FORMAT.md` | B |

##### `content/CONTEXT-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `glossary-is-the-forcing-function` — a term is resolved into `CONTEXT.md` **inline**, never batched | 19 · artifact:glossary | `trigger` (sentence 24) | `on(a term is resolved) @ SKILL.md` | B+S |
| `glossary-is-only-a-glossary` — terse definitions and aliases to avoid; no implementation detail, no spec, no scratch pad | 19 · artifact:glossary | `none` | `on(writing a glossary entry) @ content/CONTEXT-FORMAT.md` | B |
| `terms-are-context-specific` — only terms unique to this context; general programming concepts do not belong | 19 · artifact:glossary | `none` | `on(writing a glossary entry) @ content/CONTEXT-FORMAT.md` | B |
| `context-map-when-multiple` — a root `CONTEXT-MAP.md` means multiple contexts; a term is defined in its owning context's glossary and never both | 19 · artifact:glossary | `none` | `on(the repo has a context map) @ content/CONTEXT-FORMAT.md` | B |
| `challenge-and-sharpen-terms` — call out a term conflicting with the glossary, propose a precise canonical term for a fuzzy one, and cross-check a stated behaviour against the code | 19 · artifact:glossary | `none` | `on(a term is used loosely) @ content/CONTEXT-FORMAT.md` | B |

`glossary-is-the-forcing-function` moves here from `references/grove.md`. Rule 3
fires on `artifact:glossary`, and the file every session about to write a glossary
entry opens is this one; the argument for it has relocated to
`docs/ARCHITECTURE.md` and `grove.md` keeps neither half.
`challenge-and-sharpen-terms` moves here from
`grilling.md`, whose statements of it and of `glossary-is-only-a-glossary` are
procedure-register duplicates.

##### `content/grilling.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `grilling-procedure` — one question at a time, a recommended answer for each, walk the design tree, facts looked up and decisions put to the human | {requirements} · step:Execute | `none` | `on(the grilling threshold is met) @ references/requirements.md` | B |
| `no-writes-before-shared-understanding` — do not commit decisions or grow the tree until the human confirms shared understanding | {requirements} · step:Execute | `none` | `on(the grilling threshold is met) @ references/requirements.md` | B |
| `probe-with-concrete-scenarios` — stress-test domain relationships with invented edge-case scenarios that force precision about boundaries | {requirements} · step:Execute | `none` | `on(the grilling threshold is met) @ references/requirements.md` | B |

`grilling.md` is a **procedure reached on a condition**, not a standing
instruction, and its entry condition is stated above the bundled body in Grove's
own voice — as its provenance comments already are — leaving the `<what-to-do>`
block byte-intact. `no-writes-before-shared-understanding` lives *inside* that
block, so the row records the bundled sentence as the canonical statement rather
than authorising a Grove-voiced second one.

Its remaining sections are duplicates and leave: the *File structure* /
lazy-creation material restates `context-map-when-multiple`,
`adr-split-is-conditional-on-repo-shape` and `spine-seven-constraints`
(constraint 4); *Update CONTEXT.md inline* and the glossary-only sentence restate
`CONTEXT-FORMAT.md`'s rows; *Offer ADRs sparingly* becomes a pointer to
`adr-when-to-write`; *Agree the test seams* keeps only the grilling-timing half
(`agree-the-seams-during-grilling`, owned by `references/requirements.md`) and
points at `SPEC-FORMAT.md` for the rest.

---

### The two contradictions, resolved

#### 1. The grilling threshold

**Canonical statement.** A `requirements` session **always** establishes *what*
should be built. The **full one-question-at-a-time grilling procedure** runs
**only** when three or more interdependent questions are open. Below that
threshold the session records the decisions and proceeds; staging an interview
over settled ground costs a human's attention and returns nothing.

- **Canonical source:** `content/references/requirements.md`, as
  `grilling-threshold`. Rule 2 fires — `Bound = {requirements}`.
- **Load predicate:** `static({requirements})`.
- **Mirror class:** `none`. The driver already routed that kind to this file.
- **Stated once.** The file carried the always-form bullet and the three-question
  trigger as two independent statements — and the always-form bullet twice.
  `kind-references-k5` left one statement, which subordinates the procedure to
  the threshold while keeping the deliverable unconditional, and removed
  `references/execute.md`'s always-form restatement in the same commit.

#### 2. The ADR test

**Canonical statement.** An ADR is raised only when **all three** hold: hard to
reverse **and** surprising without context **and** the result of a real
trade-off.

- **Canonical source:** `content/ADR-FORMAT.md`, as `adr-when-to-write`. Rule 3
  fires on `artifact:adr` **before** rule 6 could send it to a loop-step file —
  which is the ambiguity the superseded text left open and this ordering closes.
- **Load predicate:** `on(considering an ADR) @ SKILL.md`.
- **Mirror class:** `trigger`, naming the file and never paraphrasing the test.
  "Raised sparingly" is not a legal trigger — it is a looser test wearing a
  condition's clothes, and it is what let the OR-form survive.
- **The test is stated locally**, not only cited (deferral row 1 below). A cited
  test is no bar of Grove's own, and the OR-form is what a cited-only test
  produced.

### Plugin deferral policy

Grove requires the `linkuistics` plugin and does not provision it, so a corpus
that cites it without saying what binds in its absence depends on it
**silently**. At the time this policy was written the corpus carried **14
distinct (file, skill) deferrals across 9 files** — enumerated below, and the
requirements' "7 files" is a miscount. The rewrites since have reduced that to
**6 across 4 files**; the count that binds is the audit under *What
`plugin-fallback-k9` found*, and this table is its reasoning rather than its
inventory.

The generating question is per deferral and has one form: **does the absence
change what a session writes, or only how well it writes it?** Absence that
changes *what* is owned locally; absence that changes *how well* is deferred,
and the deferring sentence must say so.

| # | file | skill | deferred capability | decision |
|---|---|---|---|---|
| 1 | `ADR-FORMAT.md` | `decision-records` | when-to-write test | **own locally** — three bullets |
| 2 | `ADR-FORMAT.md` | `decision-records` | minimum-coherent-set discipline | **own locally** (already stated); citation becomes attribution |
| 3 | `SKILL.md` | `decision-records` | ADR philosophy (artifact table row) | **defer**, stating that `ADR-FORMAT.md` holds what binds |
| 4 | `SKILL.md` | `codebase-design` | what a test seam is | **defer** with a one-line local gloss |
| 5 | `SKILL.md` | `using-jujutsu` | working-copy-as-commit lane | **defer**, stating `commit.md` is sufficient alone |
| 6 | `references/grove.md` | `decision-records` | ADR philosophy | **defer** (same as 3) |
| 7 | `references/grove.md` | `codebase-design` | seam vocabulary | **defer** (same as 4) |
| 8 | `references/grove.md` | `using-jujutsu` | the jj lane | **defer** (same as 5) |
| 9 | `references/execute.md` | `decision-records` | philosophy, format, template | **own the template locally**; defer the philosophy |
| 10 | `references/commit.md` | `using-jujutsu` | the jj lane | **own locally** — `jj describe` + `jj new` and why, already there; add that this is sufficient without the plugin |
| 11 | `references/retire.md` | `decision-records` | never-append-a-superseding-record | **own locally** (already stated) |
| 12 | `SPEC-FORMAT.md` | `codebase-design` | what a seam is / how to judge one | **defer** with a one-line local gloss; the operative rules (prefer existing, propose highest, drive the count to one) are already Grove-local |
| 13 | `grilling.md` | `decision-records` | when-to-write test | **own locally** — cite `ADR-FORMAT.md`, not the plugin |
| 14 | `grilling.md` | `codebase-design` | seam vocabulary | **defer** (same as 12) |

Two of these are load-bearing enough to name individually. **Row 1** is the
sharpest: with the test only cited, Grove had *no* bar of its own, and the
AND/OR contradiction is exactly what a cited-only test produces. **Row 10** is
the one whose absence is unrecoverable — a session that commits with git in a jj
tree bypasses the operation log — and today a reader cannot tell whether
`commit.md` is complete or a teaser. Saying so is the whole fix; the commands are
already there.

**The discharge test for `plugin-fallback-k9`:** every deferring sentence states
what binds without the plugin. A citation that leaves that unstated is the
silent dependency, whichever way the row was decided.

#### What `plugin-fallback-k9` found, and how it discharged

**The table above is superseded as an inventory and kept as the reasoning.** The
audit is re-derived against the corpus as it stands, per the leaf's own charter,
and the corpus moved underneath the table: `loop-step-references-k11` and
`corpus-split-k6` repointed four citations at grove-local files, so
`references/execute.md`, `references/retire.md` and `grilling.md` no longer name
the plugin at all. **Rows 9, 11, 13 and 14 discharged themselves** as a side
effect of homing their rules — which is the placement function doing what it is
for, not a coincidence.

What the corpus carries is **6 distinct `(file, skill)` citations across 4
files**, enumerated in `tests/plugin_fallback.rs` as `CITATIONS`:

| file | skill | what binds without it |
|---|---|---|
| `ADR-FORMAT.md` | `decision-records` | "what binds without it is stated here" — the AND test, the minimal record and placement, all local |
| `SPEC-FORMAT.md` | `codebase-design` | the three operative seam rules are grove's own; `references/grove.md` carries the gloss |
| `references/commit.md` | `using-jujutsu` | the boundary above is the whole of what a session needs, on either lane |
| `references/grove.md` | `decision-records` | `ADR-FORMAT.md` |
| `references/grove.md` | `codebase-design` | the seam gloss, plus `SPEC-FORMAT.md`'s operative rules |
| `references/grove.md` | `using-jujutsu` | `references/commit.md` |

Rows 3–5 have no successor because `SKILL.md` no longer cites a skill: it carries
**trigger sentence 19**, which routes a session meeting any citation to
`references/grove.md`. That is the structural change the design's table predates,
and it changes the answer for rows 4 and 12.

**The two rows that were genuinely open, and why neither took a mirror.** Both
`references/commit.md` and `SPEC-FORMAT.md` failed the discharge test the same
way: their deferring sentences **understated their own file**. `commit.md` said
grove states "only where its boundary falls" and `SPEC-FORMAT.md` said it "only
says where the agreement gets recorded" — while both files carried the complete
operative rules already. A reader could not tell a complete statement from a
teaser, which is the silent dependency in its purest form. The fix in each case
is the sentence, not new content.

Row 12's prescribed "one-line local gloss" is therefore **not taken**. With the
hub in `references/grove.md` and trigger 19 reaching it from the always-loaded
condition register, a gloss in `SPEC-FORMAT.md` would be a second statement of
`plugin-prerequisite`'s content — the duplication this spec exists to remove, and
the failure the leaf's charter names ("a *fallback* that restates the whole plugin
skill locally makes the plugin pointless"). The deferring sentence names the hub
instead.

**Permitted mirrors, with their reasons.** The mirror ledger for this policy has
exactly one class of entry — a rule grove states locally that
`linkuistics:decision-records` or `linkuistics:codebase-design` also states — and
each is admitted only because absence changes *what* a session writes:

| local statement | owner in `content/` | mirrored from | why it is admitted |
|---|---|---|---|
| the ADR when-to-write AND test | `ADR-FORMAT.md` (`adr-when-to-write`) | `linkuistics:decision-records` | a cited-only test is no bar of grove's own; this is the defect that let the OR-form survive |
| minimum-coherent-set discipline | `ADR-FORMAT.md`, `SPEC-FORMAT.md` (`records-are-current-state`) | `linkuistics:decision-records` | it decides whether a session appends or reworks — a *what*, and unrecoverable once appended |
| the seam gloss (a seam is a place behaviour can be replaced without editing the code under test) | `references/grove.md` (`plugin-prerequisite`) | `linkuistics:codebase-design` | one sentence, stated **once**, so a session can apply the operative rules that are already grove's |
| the jj commit lane (`jj describe` then `jj new`) | `references/commit.md` (`jj-seal`) | `linkuistics:using-jujutsu` | a session that cannot commit cannot end, and git-in-a-jj-tree bypasses the operation log |

Everything else about those skills stays deferred. **No row needed explicit
provisioning verification instead of a fallback.** The requirement offered that
as the alternative where a fallback cannot stand in, and no citation turned out
to be such a case: each of the three deferred capabilities has a part whose
absence changes *what* a session writes, and that part is small enough to state.
The nearest thing to a verification is the hub itself, which tells a session
plainly that the plugin is required and separately installed — legible, and not a
dangling reference stepped over silently.

The standing decision is recorded as
`docs/adr/grove-binds-without-the-plugin.md`, which clears the AND
test: reversing it means revisiting every citation and either install-enforcing a
context the binary does not own or dropping the local statements; the mirrors are
surprising against this spec's own thesis without the reason; and the rejected
alternatives are real — a hard install dependency, and pure deferral, which is the
prior state that produced the OR-form.

**Enforcement.** `tests/plugin_fallback.rs` asserts both halves. Each citation's
binding sentence must be present in its file, and the registry must be
**exhaustive** over a sweep of the embedded corpus — so a citation added later
without a binding sentence fails there rather than reaching a session. The sweep
runs over normalised text and carries a positive, a normalisation and a negative
control, because every way it can break produces a clean-looking result.
`an_ordinary_task_completes_without_reading_a_plugin_skill` is the acceptance
test the brief names: bootstrap, produce, retire, commit and signal each assert
the grove-local statement they need, and the four files a session cannot choose
to leave (`bootstrap.md`, `retire.md`, and both signal files) must cite no skill
at all.

### Where `content/driving.md`'s rules went

The function's sharpest output concerned `content/driving.md`, the largest file
in the corpus and the one no other corpus file named. **Its rules did not all
have owners elsewhere, and the superseded text's claim that the remainder had
`Bound(R) = ∅` was false.** Direct inspection found eight sections still
carrying imperatives that alter session conduct, so the file's deletion was made
conditional on each one landing an embedded, reachable owner first. This table
was that condition. It is **discharged**: `loop-step-references-k11` landed the
loop-step rows and `corpus-split-k6` landed the format-file rows, repointed the
`SKILL.md` conditions and deleted the file. The table stays as the map from a
rule to the owner that now carries it.

**It was a condition only while it was complete**, and three times it was not. The *review
chain* section's bare-stem imperative (`content/driving.md:469-479`) had no row,
and then the same section's selection criterion (`:399-402`) and route-through-config
sentence (`:501-508`) had none either, alongside the vendor pair's selection
criterion (`:92-94`) and three sweep-narrowing rules (`:330-336`). The third gap
survived the deletion itself and was found by `corpus-split-k6`'s review:
`hard-to-reverse-pairs-with-doubt` (`:284-285`), the trailing paragraph under
*Verifying framework decisions against the source*, went with the file and reached
`references/impl.md` in neither the table nor the corpus. Each time the
table authorised a deletion that would have taken a live rule with it, and each time
the section *was* listed with several rules under it. The lesson is the
sentence-level audit above, and the second failure is why it is now recorded as
having been re-run rather than promised — the third says the re-run was still
section-anchored where a section's rules were a list *plus* the sentence after it.
Every row below is a **rule**, not a
section; a section appearing here is not evidence that everything in it is
accounted for, and a *count* of rules under a section is not evidence either.

| `driving.md` section | rule | new owner |
|---|---|---|
| When to commission prior-art research | `prior-art-research-is-its-own-leaf` | `references/decompose.md` |
| How to write a research leaf brief | `research-brief-names-downstream-questions` | `references/decompose.md` |
| ” | `walk-away-check-per-system`, `citation-per-failure-mode-claim`, `silence-is-a-finding` | `references/research.md` |
| Running the vendor pair | `vendor-pair-when-load-bearing`, `pair-is-eager`, `both-researchers-get-one-brief`, `researchers-are-not-adversarial`, `research-output-path-per-kind` | `decompose.md`, `research.md` |
| Name the trade-off you want input on | `escalation-names-the-tradeoff` | `references/execute.md` |
| Record decisions inline | `decisions-land-as-they-settle` (+ `suggested-body-shape`) | `references/execute.md` (+ `TASK-FORMAT.md`) |
| Retiring research into ADRs | `research-to-adr-bridge` | `content/ADR-FORMAT.md` |
| Reworking ADRs and briefs | `adr-set-is-minimum-coherent`, `reconcile-records-at-retire` | `ADR-FORMAT.md`, `references/retire.md` |
| Verifying framework decisions against the source | `cite-framework-decisions-to-source` | `references/impl.md` |
| ” | `hard-to-reverse-pairs-with-doubt` (`:284-285`) | `references/impl.md` |
| Verifying a claim about the repo itself | `verify-repo-claims-with-controls`, `enumerate-then-classify`, `sweep-scope-is-the-claim`, `no-self-invalidating-count`, `check-the-rescued-clause` | `references/execute.md` |
| Doubting inside a picked Grove leaf | `review-budget`, `review-budget-predicate`, `review-budget-by-kind`, `doubt-pass-procedure` | `references/execute.md` |
| The review chain | `review-chain-when-load-bearing`, `chain-is-lazy`, `creating-session-writes-the-body`, `name-step-kind-off-the-producer`, `integration-placement`, `no-adjacency-exception`, `diversity-is-the-configs` | `references/decompose.md` |
| ” | `steps-share-the-producers-stem` | `references/decompose.md` |
| ” | `escalated-review-routes-through-config` (`:501-508`) | `references/execute.md` |
| ” | `retirement-is-filename-only` (`:484-486`) | `references/retire.md` |
| ” | `review-is-inspection-only`, `review-output-is-findings-only` | `references/review.md` |
| ” | `triage-four-ways`, `integration-escalates-redesign` | `references/integrate-review.md` |
| Externalizing surfaced work | `externalize-by-default`, `bigger-than-brief-decomposes` | `references/decompose.md` |
| What a good child leaf looks like | `vertical-slice`, `wide-refactor-expand-contract` | `references/decompose.md` |
| Recording fog without pre-slicing it | `fog-or-ticket`, `horizon-note-shape` | `decompose.md`, `BRIEF-FORMAT.md` |
| Prune, reorder, or file an issue | `triage-picks-the-verb`, `no-fourth-status`, `prune-scopes-to-the-whole-path` | `references/retire.md` |

**What genuinely had `Occasion = none`, and where it went.** Two sections were
addressed to the *human* driving a grove rather than to a session — *Ask the LLM
"WDYT" before committing* and *Ask for pushback when the LLM agrees too easily*.
A session is the LLM; it cannot obey either. Their session-facing residue is
already `escalation-names-the-tradeoff` (never withhold a recommendation out of
deference) and `grilling-procedure` (a recommended answer per question), so what
remained is operator guidance, and the human-facing home already existed:
**relocated → `docs/USAGE.md`**, as *Two habits for the human in the loop*.

Everything else left over was argument, worked example and provenance. **It was
deleted, not relocated.** A worked example anchored on the sync-semantics grove
cites a work item that resolves nowhere — which the glossary's own handle rule
forbids in provisioned content — and the arguments that are still binding are in
the ADRs. The VCS holds the rest. `docs/` earns a relocation only where a real
human-facing document already wants the material, which is the `USAGE.md` case
and nothing else here.

**And eight conditions in `SKILL.md` pointed at `driving.md`.** Every one was
repointed by `skill-router-k4` when it wrote the canonical trigger set; a
`trigger` sentence naming a deleted file is the reachability failure this design
exists to prevent, and it would have shipped silently.

## Test seams

- **Per-rule, never universal.** No parser over the corpus and no marker grammar.
  Each rule's instrument is named in its row and lands with the rewrite that
  homes it.
- **The loaded path is computed from `src/prompt.rs`**, not transcribed. The
  guaranteed core comes from `prompt::compose` and the reference file from
  `prompt::reference_file`, both already exhaustive over the kind set; a budget
  computed by a second notion of what a session reads drifts from the real one
  and then lies. It costs no new production code: the kind's **signal file** is
  identified by matching `prompt::ending_of`'s bytes against the embed, so the
  seam is read rather than widened.
- **The budget is a table of two halves, in words.** The **static** path is what
  every session of a kind reads unconditionally — core, `SKILL.md`,
  `reference_file(kind)`; the **reachable** path adds the transitive closure of
  the pointer graph below, which is the worst case if every condition fires. Both
  are asserted from **both sides**, and the two sides are different numbers. A
  ceiling is **set** at the measurement it is fitted to plus 10%, rounded up to 25
  words — and each row records that measurement beside the ceiling, so the set
  point is an equality a test checks rather than a convention a comment states. A
  ceiling is **allowed to stand** while it is within +25% of what the corpus
  measures now, which is what lets the corpus move without re-fitting 38 numbers,
  and it fails once the corpus has shrunk ~12% below the measurement it was
  fitted to (`1 - 1.10/1.25`). So a ceiling nothing approaches fails as loudly as
  a path that outgrew one, which is what the superseded 500-line ceiling could
  not do. **A single number for both was a defect and so was a band with only an
  outer edge**: with no set point checked, a ceiling could be fitted at zero width
  — failing on the next word — or raised straight to the outer edge without ever
  being fitted. Words rather than tokens: a reproducible token count needs a vendored
  tokenizer, and a budget that needs a download stops running. The limit is
  stated rather than hidden — a word count cannot price a register change, so the
  reading is "this path grew", never "this path costs N tokens".
- **Reachability is an edge test over cross-file rows, not a loadability test.**
  Partition the conditional rows first: a row whose `@` file **is** its owner is an
  in-file condition and is asserted only to agree with its grouping heading. For
  every remaining `on(<trigger>) @ F` row with owner `O` (`F ≠ O`): `F` is static or
  the owner of a reachable row; **`F` contains a literal reference to `O`'s path**;
  every chain terminates at a static path with no cycles; and every non-static owner
  file has at least one incoming edge. The middle assertion is the one a loadability
  check cannot make, and the last is what catches an owner nothing points at — the
  state `references/driver.md` was left in. **A path in `F` is necessary and not
  sufficient**, because a provenance note, a worked example or a sentence about the
  file's own history all contain the path. The sufficient half is split by source:
  `SKILL.md`'s 26 edges are pinned to their situations by the canonical trigger
  audit, and the four edges out of other files — `requirements.md` → `grilling.md`,
  `decompose.md` → `TASK-FORMAT.md` and → `BRIEF-FORMAT.md`, `retire.md` →
  `BRIEF-FORMAT.md` — are pinned by requiring the sentence carrying the path to
  carry the row's situation as well. Without that second half those four edges had
  no sufficient check at all, and deleting the conditional sentence while naming
  the same file in an unrelated one left every assertion green. **Running the cycle check over the
  unpartitioned set fails on roughly half the inventory**, every reflexive row being
  a self-loop, so the partition is part of the test rather than a reading of it. Two
  schema checks ride along: a row whose `@` file is `SKILL.md` must declare `trigger`
  or share one, and every `trigger` row's sentence number must exist in *The trigger
  sentences*.
- **`static(...)` is asserted against the runtime.** A row claiming `static(K)`
  fails unless **every** file its grouping heading names is on `k`'s static path,
  for every `k ∈ K`. Every, not any: the one heading that names two files claims
  both are static, and an existential reading would let it name a file that does
  not exist while its real neighbour discharged the check for every row under it. That single
  check is what the `always(19)` labelling would not have survived.
  **Three files can be static, not two**: `SKILL.md`, `reference_file(k)`, and
  the **signal file the guaranteed core inlines** — `content/SIGNAL.md` for the
  eighteen, `content/SIGNAL-FINISH.md` for `finish`. The third is why
  `signal-is-the-last-action`'s `static(18)` is a correct predicate and not a
  violation: the core *is* part of the loaded path, and its one embedded part is
  that file.
- **The `SKILL.md` budget is asserted, not hoped for — and it has no floor.**
  Total words **at most 900**, exactly the **26** `trigger` sentences of the
  canonical set, each at most 25 words, and the eight `own` rows present. Owned by
  `skill-router-k4`. The count is `=26` rather than `≤`, because a *missing* trigger
  is a rule that stops being reachable and is the failure this design exists to
  prevent; the reachability assertion above catches it from the other direction, and
  the two agree by construction only if both name the same number. A lower bound is
  deliberately absent: it would have to be derived from a measurement nobody has
  taken, and the three exact assertions detect a dropped row **by name**, which a
  word count cannot.
- **Single-source assertions are phrase-scoped, with controls.** A rule whose
  wording is distinctive gets a normalised sweep (emphasis stripped, whitespace
  collapsed) asserting exactly one procedure-register file states it — with a
  positive control that the sweep finds a phrase known present, and a
  cross-tree control that it still finds the class where it legitimately lives.
  An unnormalised sweep silently misses a wrapped or emphasised match; that
  failure was reproduced while writing this spec.
- **Behavioural evals assert conduct, not contents** — and what a `cargo test` can
  actually assert about conduct is settled by [a behavioural rule is covered by
  asserting its delivery](../adr/behavioural-coverage-asserts-delivery.md): the
  rule is present and reachable on the composed loaded path of every kind it
  binds, deterministically, with a near-miss fixture per rule.
  `behavior-evals-k3` owns
  the eight **B★** areas: *no second pick*, *no VCS reprobe*, *stale launch
  stops*, *the decomposition boundary*, *human-only pruning*,
  *retire → commit → complete*, *the review budget*, *all three finish-signal
  outcomes*. The ninth required area, *the interview threshold*, landed with
  `kind-references-k5` because it could not be green before that fix, in the same
  table and on the same terms. Every other **B** row lands with its own rewrite.
- **The two signal files are asserted byte-identical** to their state at this
  grove's start, for the whole workstream.
- **The existing embed checks stay as they are** — the linked embed matching
  `content/` on disk, the routing table resolving, and the instructed-verb
  comparison. The last becomes *more* load-bearing as prose stops transcribing
  command facts and starts naming verbs.

## Out of scope

- **`src/prompt.rs`.** Its three-part architecture — load instruction, bare
  runtime facts, byte-exact terminal signal — is a fixed constraint. This design
  changes the prose it points at.
- **Mechanising command facts beyond naming them.** Which facts qualify is
  decided above (verb mechanics, flag defaults, config schema); *building* the
  generation is a separate decision with its own leaf, if it is ever worth it.
- **A marker grammar, a build gate, or any universal partition check** over the
  corpus. That was tried, did its work, and was deleted.
- **The plugin skills themselves** — `plugin-skills-k7` and `harness-compat-k8`
  own those; this design touches only how `content/` cites them.
- **Any change to what `${prompt}` carries.** The too-late test owns that
  boundary and is unchanged.
