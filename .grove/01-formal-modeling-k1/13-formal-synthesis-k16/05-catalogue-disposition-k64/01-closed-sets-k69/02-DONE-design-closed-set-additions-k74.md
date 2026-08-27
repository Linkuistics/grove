# closed-set-additions-k74


## Goal

Decide the five closed-set additions and the two refuse-or-block questions that
turn on them, land them in `docs/specs/semantic-contract.md`, and get both
families answering every changed obligation with every scope the decisions reach
green under a coverage-asserting run.

## Context

**Read the pattern before the five members.** Three of the additions are one
shape recorded three times in the finish scope alone — *seven preconditions
against six reasons* (entry 031), *a tracked witness with no reason* (entry 032),
*a rolled-back finish with no reason* — and
`crates/grove-finish/models/README.md` says in as many words that **the pattern
is not three accidents**. The lifecycle scope then produced two more. Both
families had to invent the same device twice to work around it: `Sys.why` in
Alloy and its Quint counterpart exist because *a refusal reason names the
question asked, not the gate that refused*. So the first question is not "which
five members" but **whether the closed refusal set is under-populated or whether
its reason vocabulary is answering the wrong question**. If that resolves into a
rule, the rule is one ADR rather than five.

**The strongest single piece of evidence on the list is an agreement, not a
disagreement.** Both columns reached the rolled-back-finish gap independently and
each *declared* an atom rather than smuggling one in — `RefRollbackNotCommitted`
in `finish.als`, `RRolledBack` in `finish.qnt`. Two independently built families
inventing the same missing member is the catalogue being wrong, not a modelling
convenience.

**The seven items.**

- **1 · `RRolledBack` / `RefRollbackNotCommitted`.** The catalogue maps the
  `NotCommitted` disposition to *rolls back and yields `Refused`*, and none of
  the seventeen closed reasons names it: `NoTrackedDeletion` and
  `RootIdentityChanged` are each **false** of a transaction whose fingerprint was
  fine and whose root never moved. Reporting it under one of them "would be a lie
  the model could not be caught in". Two exits: widen the closed set, or restate
  the outcome. Sites: `finish.als:567`, `finish/README:1405`.
- **2 · `RConfigInvalid`.** `SY-04.b` requires an invalid configuration to refuse
  with the tree byte-identical, and no closed reason names one. Declared in
  `lifecycle.qnt` and `lifecycle.als`, recorded in `models/system/README.md`.
- **3 · `RGenContended` / `Stopped`.** `SY-10.b` requires a contended generation
  to time out into a **visible stop**, and the closed *outcome* set cannot name
  one. It is not a `Refused` — no reason covers a handoff timeout, and
  `EpochStale` is `SY-10.a`'s *mismatch*, a different fact — and not a `Blocked`,
  because §*Outcomes* scopes blocks to a transaction stopped part-way while
  `FN-25`'s two diagnoses are both about finish ownership. **It is shipped**:
  `one-live-driver-per-working-tree` says the driver "stops `blocked`" on a
  post-reap invalidation timeout. This is the one addition that would touch the
  **outcome** set rather than the reason set, which is a wider blast radius —
  `SY-14`'s exhaustive sweep runs through the same classifier the real actions
  use.
- **4 · `ONotEntered`.** `FN-01`'s first preflight member "produces no refusal at
  all" and the catalogue's closed set deliberately has no member for it — but a
  total action must return something, and if want-of-confirmation produced an
  *absent* transition then `FN-01.a` would be true by construction and
  unfalsifiable. Declared at `finish/README:3392`, which names no owner: **this
  item has no `formal-synthesis-k16` site and reaches you only through this
  body.**
- **5 · `W8WitnessTracked`.** `FN-13`'s stated witness is *a commit attempted
  while the witness is tracked, refused*, and no closed reason names a tracked
  witness. `finish.als` reports it under `WitnessPending` — the closest true
  statement the set admits — and keeps it distinguishable through `Sys.why`. The
  consequence is that **an operator cannot be told from the reason alone that the
  repository, not the filesystem, is what is blocking.**
- **6 · `FN-13` refuse-or-block.** `finish.als` **refuses**, following the
  catalogue "because the catalogue is the sole input to the formal phase";
  `finish.qnt` **blocks**, on the three-contexts rule. The two *documents*
  disagree too: `task-tree-transactions-fail-closed` says a tracked witness
  "keeps the witness unwalkable as **Recovery pending**" — a `Blocked` — and the
  catalogue says refused. **Note the coupling: resolving 6 as *block* may retire
  item 5 entirely**, since a block carries a diagnosis rather than a reason.
- **7 · `FN-10.b` / `FN-32` refuse-or-block.** `finish.als` refuses where
  `finish.qnt` blocks at the same step, on an unclassifiable artifact at a
  reserved name met inside a transaction, and **both are green** against text
  that says only *fails closed*. `FN-32` states only what both agree on (the
  artifact is not mutated). The catalogue's *one artifact, three contexts, one
  decided outcome* table now carries a `checked by` column and a second row that
  deliberately fixes no outcome — **that row is where this answer lands**, and
  `semantic-contract.md` names **this leaf by handle** there, so the row is
  addressed to you rather than to an ancestor. Quint's
  `wit_FN_32` is worded on the blocking half and **moves with the answer**.

**Two arguments the catalogue already supplies, and they point opposite ways.**
For *widening*: `finish.als`'s state-table member note observes that adding a
member is licensed in as many words — *`TT-18`/`TT-19` are stated over the
reserved CLASS rather than over its members so that removing one member changes
no claim*. Against: §*Outcomes* refuses a guard wait a member on the explicit
ground that *the set covers what a completed invocation returns*, and tells a
model needing one to declare an abstraction instead. Both are about closed sets
and they are not obviously reconcilable; deciding which governs is most of this
leaf.

## Done when

- Items 1 – 7 are each decided and landed in `docs/specs/semantic-contract.md`,
  and each is marked manifest-changing at the moment it is decided.
- **Both** families answer every new or changed obligation with a property
  command plus its required witnesses, or with that family's own declared gap.
- Every scope the decisions reach is green with coverage asserted under
  `models/run.sh --scope <scope> --family <family>`, with a run line recorded per
  scope. Expect finish and lifecycle at minimum; check whether the task-tree
  scope sweeps any set that moved before assuming it is untouched.
- `models/run-controls.sh` passes.
- `finish.als`, `finish.qnt`, `lifecycle.als`, `lifecycle.qnt` and their three
  READMEs say **in place** how each declared addition was disposed, so a later
  reader meeting the declaration finds the decision beside it. Every one of them
  currently names this leaf.
- The findings log carries an appended disposition beside each affected entry —
  appended, never rewritten.

## Notes

**Budget the runs rather than discovering them.** Alloy's finish cell is
14 m 33 s for 180 commands; Quint's is 4 m 25 s for 228; the lifecycle cells are
73 and 93 commands at ~4 m 27 s. The finish scope **can** afford
`QUINT_VERIFY=1` and the repository default stays 0 because the task-tree scope
cannot.

**A disposition is a decision about the contract, so the ADR test applies**
(`content/ADR-FORMAT.md`). These seven are the items on the whole disposition
list most likely to earn a record, because a later reader will want the **cost**
of widening a closed set — a matching outcome imposed on every column that
sweeps it — and not only the outcome. If the pattern paragraph above resolves
into a rule about what a refusal reason names, that rule is **one** record rather
than five.

**Sibling `routing-and-prose-k73` already landed a rule you will want.** The
catalogue's assumption table now states that *an exercise-removal row's controls
column is a claim of unreachability, to be established by running the removal
rather than by reading the witness*, and names the two different failures the
shape produces. Any new member that changes an assumption's reachability inherits
that obligation.


## Decisions (running log)

### The pattern question came first, as the body asked, and it splits in two

The body's first question is whether the closed refusal set is **under-populated**
or whether its **reason vocabulary is answering the wrong question**. Both are
true, of different halves, and separating them is most of the value here.

**The vocabulary answers the right question, and the twice-invented device is
evidence about models rather than about the contract.** The catalogue already
rests on the rule at `FN-05.a`: an unsupported layout and an unreachable
quarantine operand share `LayoutUnsupported` because `SY-03` makes them one
question asked at two gates — *the reason names the question, not the gate*.
`Sys.why` and its Quint counterpart exist to tell **gates** apart, which is a
modelling need `FN-05.a` explicitly asks for. So the rule is now stated rather
than merely relied on, and `Sys.why` stays an abstraction in both columns.

**The set is under-populated, and not randomly.** The seventeen were drawn over
the questions the **task-tree** scope asks — preconditions and guards on a tree —
and the set is swept by **three** scopes. Every one of the granted members is a
question a *later* scope asks: a commit's disposition, a configuration, a launch
generation. That is checkable, it explains why three "separate accidents" arrived
in one scope and two more in another, and it predicts where the next gap is
rather than listing the closed ones. **It predicted correctly inside this
session**: a fourth instance turned up in the task-tree scope
(`PartialScaffold` has no reason either), found by reading the model rather than
the enumeration, and is routed to `task-tree-scope-k70` with the frozen set and
the rule rather than as an open question.

### Seven items, and the tally is 3 granted · 1 dissolved · 1 declined · 2 decided by one rule

The mark in the third column is the one the node brief asks for **at the moment
the item is decided**, and it is given in both senses, because they differ here
and the cheap check is silent about the expensive one — see *Manifest-changing
has two meanings* below. **MC-cascade** = both families must answer something
new or changed. **MC-count** = `models/run.sh --list` moves.

| # | item | mark | disposition |
|---|---|---|---|
| 1 | `RRolledBack` / `RefRollbackNotCommitted` | MC-cascade, MN-count | **granted** — `DeletionNotCommitted`; both columns rename to the member |
| 2 | `RConfigInvalid` | MC-cascade, MN-count | **granted** — `ConfigurationInvalid`; both columns already carried the atom |
| 3 | `RGenContended` / `Stopped` | MC-cascade, MN-count | **granted as a REASON, refused as an OUTCOME** — `GenerationContended`; the six outcomes stand and `lifecycle.als`'s `Stopped` is gone |
| 4 | `ONotEntered` | **MN both** | **declined** — the catalogue gains nothing; the abstraction is licensed by a generalised rule |
| 5 | `W8WitnessTracked` | **MN both** | **dissolved by 6** — a block carries a diagnosis, not a reason; `W8` stays a model-only observable |
| 6 | `FN-13` refuse-or-block | MC-cascade, MN-count | **block**, `RecoveryPending`; `finish.als` moves, `finish.qnt` was right |
| 7 | `FN-10.b` / `FN-32` refuse-or-block | MC-cascade, MN-count | **refuse** at that step — and a block at a later one; `finish.qnt` moves, `finish.als` was right |
| — | the rule the last two imply | **MC-count** | `FN-29` gains `.a` and `.b`; 128 → 129, and the only count that moved in the whole leaf |

**Two of the seven turned out to cost nothing at all**, against a task file that
marked all seven manifest-changing. That is not a smaller job than the one
chartered — establishing 4 and 5 took reading the shipped product and applying
6's rule — but it is the honest tally, and a later reader who inherits "seven
manifest-changing items" from the brief chain should meet the correction here.

Closed **refusal reasons** 17 → 20. Closed **outcomes** 6 → 6. Closed **blocked
diagnoses** 2 → 2. Obligations 128 → **129**, and the one added is `FN-29.b`.

### 6 and 7 resolved in OPPOSITE directions from ONE rule, which is why they are one record and not two coin-flips

The two sharpest items were never a choice between columns. **The catalogue fixed
`Refused` and `Blocked` and never said what separates them**, so each family
invented a discriminator and they invented different ones:

- **Alloy read it step-locally.** `treeSame` is a frame condition on one
  transition. At `FN-13` the failing step mutates nothing, so `Refused`.
- **Quint read it action-locally.** The transaction has published and evacuated,
  so `Blocked`.

Both are correct readings of *fails closed*, which is all `FN-10.b` said, and
**both stayed green** — the failure mode a suite cannot report about itself. The
repair is not to pick a column but to state the predicate both were guessing at:

> `Refused` and `Blocked` are separated by **what the action leaves**, never by
> where it stopped. An action returns `Refused` when the tree it hands back
> equals the tree it was given — whether no step ran, or every step that ran was
> undone. It returns `Blocked` when an effect stands it could neither complete
> nor reverse.

Applied, that settles the two items **in opposite directions**: `FN-13` is
post-evacuation, so a block; `FN-10.b`'s discard has applied nothing, so a
refusal — and `ReservedNameOccupied` has named that exact case since before
either model existed. The three-contexts table's second row therefore fixes a
**function of the step** rather than one outcome, which is what a claim
quantified over every step of a transaction needed all along.

**Three independent confirmations, none of them a preference.** The shipped
post-commit verification rejects a result that still tracks `.grove/` with the
evacuation done; `task-tree-transactions-fail-closed` says a tracked witness
"keeps the witness unwalkable as **Recovery pending**"; and `FN-25`'s own first
arm gives the diagnosis, since the witness is provably this attempt's. The
diagnosis argument deliberately rests on `FN-25`'s **first** sentence, not its
third — the third is `finish-scope-k71`'s item 9, and leaning on a sentence a
sibling is chartered to repair would have coupled the two for nothing.

### The falsifier was run before the rule was written, and the catalogue was already obeying it

`FN-22`'s ten-row revalidation table applies the rule **row by row** — `Refused`
wherever the tree returns to what it was (including after a rollback that reached
the commit), `Blocked` wherever an effect stands — and never names the predicate
that generates the rows. `FN-16.a`/`.b` say *refused* of a step **declining**,
and both families read it as a block anyway, because `FN-17.b` says *blocks
rather than proceeds* three lines below. **Where the catalogue happened to supply
the missing predicate in a neighbouring sentence the two columns agreed without
discussing it; where it did not, they diverged.** That is the sharpest single
piece of evidence that the omission was the defect, and it is why the rule earned
an obligation rather than a paragraph.

### `FN-29.b` exists because nothing could check the rule, and it was controlled against the exact defect

A rule carried only by prose is a rule a model can contradict without a
counterexample. So `FN-29` gained letters: `.a` is its old content, `.b` is the
discriminator.

**Placement is one obligation, not one per scope, and that is forced.**
`obligations-follow-context-not-artifact` clause 4 turns a claim over *every
action* into one obligation per scope — but only for scopes that can execute its
context. `Blocked` is produced by the finish and recovery protocol and nothing
else: §*Outcomes* scopes it to a transaction, `FN-25`'s diagnoses are both about
finish ownership, the task-tree scope has no block to be distinguished from, and
`models/system/` reads *blocked* as a mark already made. One scope, one
obligation, two cells.

**The control fired.** Reverting `finish.als`'s `FN-13` to a refusal in a scratch
copy makes `FN_29b_a_refusal_leaves_nothing_standing` produce a counterexample.
**A rule that took a cross-model replay to find is now one a single family's own
run would catch** — which is the most transferable thing this leaf produced.

### `FN-29.b`'s first draft was FALSE, and the counterexample is worth more than the fix

Written as *a refusal implies not `gateEvacuatedNext`*, the check failed on its
first run. `EN-11`'s free initial state supplies a world that **starts**
`Published` with an empty root — an earlier attempt's evacuation, standing at
state 0 — and a fresh transaction refusing over it is a counterexample to a rule
about what **this** action left. `FN-29.a`'s own note records the identical trap
on the quarantine name; this is its third instance.

The lesson generalises past this file: **in a model whose world may start
anywhere, *the tree is not in state X* and *this action did not put it in state
X* are different claims, and only the second is what a refusal promises.** The
conjunct `Slot.owner' = Txn.attempt'` is the repair, and it is this file's own
existing idiom for *this attempt's*. The check keeps its teeth — the mutation
control above was run **after** the repair.

### The SAME counterexample then fired in the other column, on a different artifact, and three instances make it the finding

`inv_FN_29b`'s first Quint draft was the mirror of the Alloy one and failed the
same way in the suite that introduced it — but on the **quarantine** rather than
the witness slot. A quarantine standing in the free initial state, created by no
transaction, survives a refusal at `SCreatePreparing` and is reported as an
incomplete outcome because of bytes that were never this action's.

**That artifact is the one `finish.als`'s `FN-29.a` note already named**, for the
identical reason, before this session existed. So the count is three: `FN-29.a`
on the quarantine (recorded), `FN-29.b` on the witness slot (Alloy, this
session), `FN-29.b` on the quarantine again (Quint, this session). **A defect
found three times on three artifacts is a property of the claim's shape, not of
any one encoding**, and it is why the catalogue's wording is *the tree it hands
back equals the tree it was **given*** rather than *the tree is untouched* — the
operand carries the whole rule.

The repair is the same in both columns and reads differently in each because
each expresses ownership its own way: Alloy conjoins `Slot.owner' =
Txn.attempt'`, Quint conjoins `w.mani.handle == HANDLE and w.mani.attempt ==
t.attempt` onto its existing `evacuationComplete`. **What is left after the
repair is the evacuation, owned** — which is the effect the discriminator
actually turns on, because it is what makes a stop recoverable rather than
forgettable.

**Both columns are controlled, and the two mutations are different because each
column's defect was in a different place.** Alloy: revert `FN-13` to a refusal →
counterexample. Quint: make `stopReserved` always refuse → counterexample. Each
kills the obligation against the exact branch that column got wrong, which is a
stronger pair than one mutation run twice.

**And the two columns divide the claim rather than duplicating it.** `finish.als`
states `FN-29.b` two-sidedly (*and every block leaves an artifact standing*)
because it has no neighbour that does; `finish.qnt` states only the refusal half,
because `inv_FN_22c` already carries the block half there and restating it would
make each mutation unable to kill the other — the hazard `FN-32` and `FN-21.c`
record beside `foreignAtReservedName`. The division is declared at both sites.

### 3 was a level error, and one WORD caused it

`SY-10.b`'s visible stop was declared as a seventh **outcome** in `lifecycle.als`
and as a **refusal reason** in `lifecycle.qnt`. Quint was right. Alloy's argument
was *not a `Refused`, because no refusal reason covers a handoff timeout* — which
is circular the moment the reason set is the thing under review — and *not a
`Blocked`*, which holds. So the residue is a refusal missing a reason, and
widening the six outcomes would have swept `SY-14`'s exhaustive classifier for
nothing.

**What made the wider reading look necessary was a word collision.**
`one-live-driver-per-working-tree` says the driver "stops `blocked`", and that
`blocked` is not the catalogue's `Blocked(b)` — it is the *epoch invalidation*
being blocked. Verified in the shipped code rather than inferred:
`complete_post_reap_epoch_handoff` in `src/loop_driver.rs` returns an error, the
loop stops, the completion signal is left unconsumed, and the ADR's own next
sentence is that a timeout performs no tree access or epoch rewrite. Nothing
stands, so it is a refusal. **A collision between a diagnostic's English and a
closed set's vocabulary cost a proposed widening of the most load-bearing set in
the catalogue**, and that is worth more to a later reader than the member itself.

### 4 was decided by the product, and the product has no gate at all

`ONotEntered` was declared because `FN-01`'s first preflight member "produces no
refusal at all" while a total action must return something. Checked against the
shipped code rather than argued: **Grove has no confirmation gate anywhere.**
Constraint 5 is *grove guides, it does not gate*; `finish_commit`'s own contract
is that "whether a human confirmed teardown is the calling finish session's
responsibility"; no Grove binary reads standard input; `EN-15` already grants
that Grove cannot verify a confirmation.

So a transaction not entered for want of confirmation is **a call that was never
made**, and the closed set — which covers what a *completed invocation* returns —
cannot name it without ceasing to be that set. The catalogue already had the
answer and had written it for a different case: §*Outcomes*' guard-wait paragraph
licenses a model to name an unobservable as its own abstraction. That paragraph
is generalised from a one-off to a rule, and `ONotEntered` stands as the rule
being followed. **One of the five "additions" adds nothing, and establishing that
took reading the product rather than the catalogue.**

### Manifest-changing has two meanings here and only one of them moves a number

`models/run.sh` reads its obligation list out of the catalogue by matching claim
headings and `- \`XX-nn.x\` —` bullets. **A closed-set addition matches neither**,
so `--list` printed `128` across every one of items 1 – 7 and moved to `129` only
when `FN-29` gained letters. That is not a defect in the classification the briefs
use — a member of a set an obligation *sweeps* still costs both families a
matching outcome — but the two must not be conflated, because the cheap check
(`--list`) is silent about exactly the expensive kind. Recorded because the node
brief's `Done when` asks each item to be marked at the moment it is decided, and
a reader who marks by the count alone will mark five of these wrong.

### The task-tree scope was checked rather than assumed, and owes no re-run

The `Done when` asks explicitly. `crates/grove-task-tree/models/task-tree.qnt`
**transcribes all seventeen** reason members, so it does read the set — but
nothing quantifies over the type exhaustively, no `TT-` obligation reaches the
three new members, and `task-tree.als` already restricts its own set and says so.
The three are therefore deliberately **not** transcribed, with the reason stated
at the type, so a later reader meeting 17 of 20 finds the argument rather than a
trap.

No re-run is owed and that is established rather than asserted: the whole diff is
inside comments (`jj diff` filtered for non-comment changed lines is empty, 24
insertions all `//`), `quint typecheck` passes, one real command runs — `quint run
--main=base --invariants inv_TT_01a_…` reports `[ok] No violation found`, exit 0 —
and the **negative control**, the same command on a deliberately broken copy,
exits 1. Clean-here plus dirty-there is what makes the pair evidence.

### One item was routed to each of the two scope siblings, and neither is evasion

- **`task-tree-scope-k70`** gets the `PartialScaffold` reason gap: a fourth
  instance of the shape, decidable only after that leaf's own items 20 and 21
  (what `PartialScaffold` *is*, and whether the state table gains the shipped
  ambiguity member). It inherits a **frozen set and a stated rule**, so the
  collision hazard that put this vocabulary child ahead of the scopes does not
  apply — and a member survives a redefinition of the state it reports on,
  because it names the question rather than the state's extension.
- **`finish-scope-k71`** gets `doCommitAttempt`'s `W9SlotPending` branch, which
  refuses with a **published** witness standing. It survives `FN-29.b` as written,
  because that antecedent is the *completed* evacuation — a fact about where the
  line was drawn, not evidence the branch is on the right side of it. `FN-29.b` is
  what turns it from invisible into a question, which is the argument for routing
  it rather than for having decided it.

Neither was referred to this leaf by any session, and re-deciding unreferred
branches under cover of a disposition is how a vocabulary child becomes a scope
child. `routing-and-prose-k73` set the precedent on its third `EN-11` instance:
state the rule, route the repair.

### One ADR, and the AND test was applied to the alternative as well as to the decision

[`a-refusal-leaves-nothing-standing`](../../../../../docs/adr/a-refusal-leaves-nothing-standing.md)
carries both clauses. Each leg of `content/ADR-FORMAT.md`'s test:

- **Hard to reverse** — clause 1 fixes the meaning of two of six outcomes, and
  every `FN-` obligation, ten rows of `FN-22`'s table and both families' commands
  are stated against it. Clause 2's members impose a matching outcome on every
  column that sweeps the set.
- **Surprising without context** — *this step mutated nothing, therefore refused*
  is the intuitive reading, and a green suite held it for the whole Alloy column.
- **A real trade-off with a rejected alternative** — and both alternatives are
  the ones that were actually taken, not straw men. The step-local reading is
  *cheaper to check* (a frame condition over one transition, which is why Alloy
  could state it at all); reporting under the closest true member is *cheaper to
  land* (one declaration against a two-family cascade), and a model has no other
  move while the set is closed against it. Each was rejected on a stated cost,
  and each carries a **Reopen** condition naming what would make it admissible
  again.

Five members were referred and one record was written, which is the task file's
own prediction met: three are members of one rule, one dissolves under the other,
and one is not a gap. A record per member would have recorded the symptoms.

### Nothing else in the ADR set needed reworking, and two records were deliberately left alone

`task-tree-transactions-fail-closed` and `one-live-driver-per-working-tree` are
the two `routing-and-prose-k73` refused to pre-adjust, on the ground that they are
evidence this leaf weighs. Both are **cited** here and neither is edited: the
first turned out to be *right* against the catalogue, so there was nothing to
change; the second's "stops `blocked`" is accurate English about epoch
invalidation and only misleads when read as the catalogue's outcome name — which
is a fact for §*Outcomes* to carry, not a defect in the ADR. That sibling's
restraint paid: pre-adjusting either would have put a guess in the way of the
argument.

### The reported run is the one that started after the last edit, and two earlier ones were discarded

Two green-so-far finish runs were thrown away, both for the same reason and both
on this session's own mistake: a model file was edited **while the run was
reading it**. Neither edit changed a single line of semantics — one added a
routing comment, the other corrected the phrase *declared gap* to *declared
narrowing*, since `GAP` is a term `models/run.sh` parses and this obligation is
answered rather than gapped — but a run line whose subject has moved is exactly
the provenance failure `routing-and-prose-k73` discarded two of its own control
runs over, and *comment-only* is an argument for not re-running, not for
back-dating a run that already started.

So the model files were frozen first, their digests recorded, and the reported
runs started after. The digests are recorded again at each run's end, and they
match. The individual commands the disposition changed were each executed on
their own before the suite — `FN_13`, `FN_29a`, `FN_29b`, both `FN_29b`
witnesses, `FN_10b`, `SY_10b`, `SY_04b` — so the suite is the record rather than
the debugger.

### Run lines — five cells, every one exit 0, coverage asserted

Run as four per-family cells plus the runner controls, concurrently, started
`2026-08-27T19:47:45Z` and ending `20:06:13Z`. The model files and both scope
READMEs were **frozen before the batch started**, their digests recorded, and
recorded again at the end unchanged — the runner reads the catalogue as its
manifest and each scope README for declared gaps, so all of them are subjects of
the run rather than bystanders.

```sh
models/run.sh --scope finish    --family alloy   # 186 commands, 63 of 63 cells, exit 0
models/run.sh --scope finish    --family quint   # 236 commands, 63 of 63 cells, exit 0
models/run.sh --scope lifecycle --family alloy   #  73 commands, 25 of 25 cells, exit 0
models/run.sh --scope lifecycle --family quint   #  93 commands, 25 of 25 cells, exit 0
models/run-controls.sh                           # 10 passed, 0 failed, exit 0
```

Every cell reports `0 declared gaps, 0 empty`, and no cell is **contested** —
neither family declared a gap where the other answered, which is the line the
runner added after `TT-24.c` and the one that would expose a transcription here.

**The finish scope's cell count moved 61 → 63**, which is `FN-29.a` and
`FN-29.b` standing where `FN-29` did, answered by both families with a property
and its witnesses. Alloy gained six commands (180 → 186) and Quint eight
(228 → 236): the `FN-29` split, `FN-29.b`'s two witnesses per family, and — in
Quint — the moved `wit_FN_32` reading both arms.

**The task-tree scope is not in the batch and owes no run**, established rather
than assumed, above.

**Cost, recorded because the next session should budget from it rather than from
the inherited figure.** The finish Alloy cell took **~18 min** here against the
14 m 33 s the README records, and a single witness command measured 23 s wall
against 7.5 s of CPU — **34% CPU**, so these runs are startup- and IO-bound
rather than solver-bound on this host. That is why the four cells were run
CONCURRENTLY: they barely contend (load stayed near 3 on 16 cores), and the
batch's wall time is the finish Alloy cell alone rather than the sum. A
sequential invocation of the same work was on track for well over an hour.

**The run lines above were written into the two scope READMEs *after* the batch,
and that is not the provenance failure the discarded runs were.** A record of a
run is necessarily written after it. The distinction that matters is whether the
edit changed a **subject** of the run, and `models/run.sh` reads each scope
README for exactly one thing — its `- **GAP** <family> …` declarations. Checked
rather than asserted: the GAP-line count in all three model READMEs is
**identical to the parent commit's**, so this session declared and removed no gap
anywhere, and every post-run edit is inert to the runner. The model files and the
catalogue were not touched after the batch started at all; their digests are
recorded either side and match.
