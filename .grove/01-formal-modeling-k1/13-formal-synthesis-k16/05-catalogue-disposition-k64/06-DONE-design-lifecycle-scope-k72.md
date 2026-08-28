# lifecycle-scope-k72


## Goal

Decide and land the five `SY-` scoped catalogue findings, get
`models/run.sh --scope lifecycle` green with coverage asserted in both columns,
and close this node by re-running the enumeration instrument that opened it.

## Context

**Last of the three scope children, and the one that carries the node's
whole-node obligation.** `task-tree-scope-k70` and `finish-scope-k71` ran first
and either may have re-scoped an item upward into this one — the rule moves
obligations up, never down, so an item arriving here arrives as a new `SY-`
obligation with an empty cell in both families. Read both retired bodies before
starting.

**`closed-sets-k69` froze the vocabulary, and two of its five closed-set
additions were this scope's** — `RConfigInvalid` and `RGenContended`/`Stopped`.
Items 15, 16 and 17 below all sit against `SY-04.b`, whose refusal is what
`RConfigInvalid` was added (or declined) for, so read those decisions first.

**Run cost:** the lifecycle cell is the cheapest of the three — Alloy 73
commands, Quint 93, 4 m 27 s wall at the revision entry 046 recorded.

**The five items, with the evidence each already has.** Numbering is the node
brief's item table.

- **10 · `SY-13` is false over the `Legacy`/`Foreign`/`Malformed` sinks** (`MN`).
  `SY-13` excludes `Malformed` from the terminal dispositions, but all three of
  `Legacy`, `Foreign` and `Malformed` are reached by a hand edit and left by a
  hand edit — and **a hand edit is not an admitted action**, which `SY-13`'s own
  note establishes. So under the literal text every one of the three is a sink
  and both obligations are FALSE. **The catalogue knows the shape and declines
  both repairs it considered**; the repair it does not consider is the one the
  model takes — quantify over the stable states the loop's own admitted actions
  reach, and make *Grove never manufactures one of the others* a checked claim
  (`SY_13a` conjunct 1, M21) rather than an assumption. `mutant_literal_sy13`
  runs the literal text and `inv_SY_13b_no_stable_state_is_a_sink` dies, which
  is what turns "the catalogue is wrong here" from a remark into a fired
  control. **Two independent readings agree** — the model's and `system-k59`'s.
  Sites: `system/README:1078`, `1081`, `1531`; `findings:7821`.
- **15 · Does `SY-04.b` owe `SY-03`'s *a preflight is never a licence*?** (`MC`).
  `outcomeOn` gates transitions on `d.configValidated`, the driver's **recorded
  verdict**, while the layout gate three lines below reads `w.layoutOk` **live**.
  So a `configChange` between the validation and the transition leaves the
  validation standing as a licence and the transition writes the tree under an
  invalid configuration — **with the operator's hands out of it**.
  `inv_fail_MUT_SY_04b_a_validated_configuration_is_a_licence` fires on it under
  `base`'s own constants with no dial moved. The catalogue has an obligation for
  exactly this shape and states it for the layout only. Either `SY-04.b` owes the
  same, or the configuration is deliberately read once per iteration — and that
  second reading must be *stated* if it is chosen, because it is currently
  neither stated nor checked. Retained counterexample and reproduce command at
  `system/README:819`. Sites: `system/README:819`, `1584`; `lifecycle.qnt:2165`;
  `findings:8730`.
- **16 · `SY-04.b`/`SY-14.b` are over-applied to `release-lease`** (`MN`).
  `acquire-lease` is already exempt because it runs before configuration
  validation; **`release-lease` deserves the same exemption for a stronger
  reason** — a release touches no tree and launches nothing, so there is nothing
  for a configuration to be valid *for*. Gating it means an invalid personal
  configuration strands a lease the loop can then only escape by dying. Two
  repairs are available and the choice is this leaf's: exempt the release, or
  admit process death. **The second is not free**: `CONTEXT.md`'s *Admitted
  action* is explicit that process death is `crash` and therefore the world's, so
  admitting it as Grove's exit changes what every reachability claim quantifies
  over. Sites: `system/README:969`; `findings:6626`, `6737`.
- **17 · `SY-04.b`'s byte-identical clause is stated over a system when it is
  true only of Grove** (`MN`). *An invalid configuration leaves the working tree
  byte-identical* is true of **Grove's own transitions** and false of the
  world's, while §*Actions* puts `hand-edit` and `foreign-write` in the same
  table as the transitions the claim is about. Unqualified, the conjunct reads
  *a bad configuration stops the operator editing their own directory*, which is
  false and is not what the obligation says. **This is entry 042's class and its
  second instance in the same file** — the same shape as item 10 — so decide
  the two together and consider whether the catalogue owes one general
  qualification rather than two local ones. One clause is owed.
  Sites: `system/README:474`; `findings:6361`, `6452`.
- **18 · `SY-14.b`'s *every action* must be read as every action ON THE TREE**
  (`MN`). The literal quantifier reaches `acquire-lease`, `validate-config` and
  `release-lease`, so a blocked tree could not release its own lease and
  `FN-26`'s two operator-restorable exits would be unreachable. Note this is the
  same over-application as item 16 seen from the other obligation, so the two
  should not be decided in opposite directions.
  Site: `system/README:1531` (§2).

**A vocabulary question `closed-sets-k69` noticed and left here, because it
sits directly against items 15 – 17.** The catalogue's Lifecycle action group
names `layout-preflight`, and that string appears **exactly once in the whole
catalogue** — in the `Actions` table. Quint models the action as
`ATValidateConfig`, which is what `SY-04.b`'s validated-configuration gate reads,
while `SY-02`/`SY-03` are about the **layout**. So either the catalogue has one
action doing two jobs, or it owes a `validate-config` beside the preflight.
Decide it with 15 – 17 rather than separately: all four are about what
`SY-04.b`'s gate is over.

**Routed, not disposed — four model findings this scope raised.** Items 31 (the
`SY-10.b`/`SY-11.b` collision over `WGen`), 32 (`SY_11a` blind to a repeat
acquisition at an existing site), 33 (`SY-05.b`'s stronger claim needs an
importing seam) and 35 (an `EN-` assumption row for process death) belong to the
model owners rather than to the catalogue. `closed-sets-k69` landed the routes;
this leaf must not absorb them.

## Done when

- Items 10, 15, 16, 17 and 18 are each decided and landed in
  `docs/specs/semantic-contract.md`, each marked manifest-neutral or
  manifest-changing at the moment it is decided.
- For every manifest-changing one, **both** families answer the new or changed
  obligation with a property command plus its required witnesses, or with that
  family's own declared gap.
- `models/run.sh --scope lifecycle --family alloy` and `--family quint` are each
  green with coverage asserted, and both run lines are recorded here.
- `models/system/README.md` says, in place, how each finding it recorded rather
  than fixed was disposed.
- **The node's own closing obligation, which is this leaf's alone:** re-run the
  enumeration instrument and confirm **no catalogue finding is left saying
  `formal-synthesis-k16` owns it**. Three things `routing-and-prose-k73` learned
  the hard way and which this sweep must therefore carry:
  - **The subject sweep is not the evidence.** Run the positive control
    (`cross-model-replay-k15` must still find its own sites) and the negative one
    (an invented handle must find none) — and run the **cross-tree** control,
    which is what actually caught the defect: every *node* handle
    (`formal-synthesis-k16`, `catalogue-disposition-k64`, `closed-sets-k69`) must
    find **zero** live sites, while every *leaf* handle finds its own. A node
    handle naming a live artifact is a pointer to a directory rather than to an
    owner, and a clean subject sweep hides it completely.
  - **`docs/formalism-findings.md` is a log and its 31 sites are correct.** The
    clean condition over the log is *every site carries an appended
    `> **[disposed by …]**` line*, never *the name is gone* — the entries' own
    prose was true when written and is not rewritten.
  - **Every decomposition this node performed added handles.** Sweep the leaf
    handles this node created — `k69` – `k74` — not only the ones it inherited.
- **Three items reach this node through prose and not through any site** — 4, 7
  and 19, all `closed-sets-k69`'s — so the closing sweep must check them by name
  rather than trusting the grep. A future re-run will report clean whether or not
  they were decided.

## Notes

**`closed-set-additions-k74` LANDED TWO REFUSAL REASONS INSIDE YOUR TERRITORY,
AND THEY CHANGE THE STARTING TEXT OF THREE OF YOUR FIVE ITEMS.** The closed
reason set is now **twenty**; the closed outcome set and the two blocked
diagnoses are unchanged.

- **`ConfigurationInvalid`** — `SY-04.b`'s refusal now has a name, and the
  catalogue says so beside the obligation. Items 15, 16 and 17 are all `SY-04.b`
  and each now argues against a claim whose refusal is nameable.
- **`GenerationContended`** — `SY-10.b`'s visible stop is
  `Refused(GenerationContended)`, **not** a seventh outcome.
  `models/system/lifecycle.als`'s `Stopped` is gone and its uses are the refusal;
  `lifecycle.qnt`'s placement was the one kept. If any of your repairs quantify
  over the outcome set, it is still six.
- **A word collision is recorded at `SY-10.b` and is worth reading before item
  18.** `one-live-driver-per-working-tree`'s "stops `blocked`" is the *epoch
  invalidation* being blocked, not the catalogue's `Blocked(b)`. Item 18 is about
  `SY-14.b`'s quantifier reaching `release-lease`, which sits in the same
  neighbourhood of the same ADR.

**And `FN-29.b` is new, which bears on item 18 more than it looks.** *Every
`Refused` is returned with the tree byte-equal to the tree that action received;
an effect that stands and can be neither completed nor undone is `Blocked`.* It
is `grove-finish`'s alone — placement argued at the obligation — but the rule it
states is the catalogue's, so a `SY-14.b` repair that makes an action on a blocked
tree do something other than refuse must be consistent with it. See
[`a-refusal-leaves-nothing-standing`](../../../../docs/adr/a-refusal-leaves-nothing-standing.md).

**Your closing sweep gains two handles.** `closed-set-additions-k74` and
`routing-and-prose-k73` both retired, and both are named in live artifacts as
**attributions of work done** rather than as owners of undecided work — which
`routing-and-prose-k73` established is the correct clean condition, not the
absence of the name. Check `closed-sets-k69` too: it was a leaf, then a node, and
a sweep that finds a node handle naming an owner has found the defect `k73`
caught one level up.

**The whole-repository run is not owed here.** The node brief's `Done when` asks
for a green **per-scope** run for every scope a child touched; the repository-wide
invocation with its `ordinal-fs-tree` positive control is
`formal-synthesis-k16`'s own `Notes` obligation ("rerun from a clean
checkout-equivalent state") and belongs to `handoff-audit-k66`. Do not absorb a
~3 h measurement that a later sibling is chartered for.

**A disposition is a decision about the contract, so the ADR test applies**
(`content/ADR-FORMAT.md`). Item 16 is the likeliest to earn a record if the
chosen repair is *admit process death*, because that changes the admitted-action
set every lifecycle reachability claim quantifies over, and `CONTEXT.md`'s
*Admitted action* entry would need reworking with it.

## Handed forward by `task-tree-scope-k70` — one cell, and it is not a decision

**`SY-06.b` is a declared cross-scope citation of `TT-18`/`TT-20`, and both
narrowings it carries changed under it.**
[`obligations-follow-context-not-artifact`](../../../../docs/adr/obligations-follow-context-not-artifact.md)
clause 2 makes a citation carry the cited obligation's declared narrowings, so
this is a cell you owe rather than a question you must answer — **the decision is
landed**, in
[`a-witnessless-root-refuses-what-it-cannot-account-for`](../../../../docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md)
and in the catalogue's §*States*.

**What changed.** The catalogue had **one** state for a witnessless root where
the product has three. It now has an ordered three-way test:
`PartialScaffold(Exact)` (nothing but the fresh scaffold's own byte-exact
entries — **completed**), `PartialScaffold(Ambiguous)` (a root-init-exclusive
entry standing beside something else — **refused**, mutating nothing), and
`Legacy`. `SY-06.b`'s text is updated in place, because leaving a stale citation
in one document is worse than the split; its **model cell and its run** are
yours.

**The concrete work, as far as `k70` could see it.**

- `models/system/lifecycle.qnt` carries `RSPartialScaffold` and a
  `PARTIAL_BY_SUBSET` const whose comment quotes the old subset test. The state
  needs the class, and `classify`'s `RSPartialScaffold` arm splits.
- `SY-06.b`'s witness is *an interrupted scaffold, completed; and a `Legacy`
  tree, refused rather than completed*. There is now a **third** case that must
  be refused rather than completed, and it is the sharp one: `Ambiguous` is
  refused **while carrying proof that Grove's own initialisation ran**, which is
  precisely the case a completing implementation gets wrong. `wit_SY_06b_...`
  and `hist.partialCompleted` are where that lands.
- The citation inherits a **different** strength rather than the same one:
  `TT-20` no longer excludes an initialisation the world touched, and instead
  narrows its `Legacy` prohibition to the window in which a root-init-exclusive
  entry has landed. Carry that narrowing, not the retired one.
- `models/system/lifecycle.als` has no `Partial` state and may owe nothing;
  `k70` did not verify this beyond a grep and it is yours to establish.

**The new refusal reason costs this scope nothing, and that is checked rather
than assumed.** The closed set gained `ScaffoldIncomplete(class)` (reasons
20 → 21). `lifecycle.qnt`'s `type Refusal` spells out only the members this
scope reaches and no `SY-` obligation quantifies over the type exhaustively, so
no re-run is owed **for the reason**. The state refinement above is a separate
matter and is the cell.

## Handed forward by `finish-scope-k71` — one cell to establish or declare, and one correction to a record you inherit

**Nothing is re-scoped upward.** No `FN-` obligation moved to `SY-`, and no
`SY-` obligation's text changed. What travels is a **§*Vocabulary*-level change
that all three scopes sweep**, exactly as `closed-sets-k69`'s did, plus one ADR
edit in your reading chain.

### §*States* gained a row and changed its order

The task-root state table now reads: the **four** `Reserved` classes first —
`Preparing`, `Published`, **`Quarantined`** (new), `Migrating` — then `Absent`,
then the format rows, then the three `Current` rows. Both halves are one repair
and either alone leaves the other's disk misclassified.

**The argument that decided it is one you will meet again, because `SY-05` is
yours.** `FN-22`'s **fourth** revalidation point runs *after* the quarantine
rename, and two of its three rows return the quarantine — so between the rename
and that point the task-root name is free and the disposition is **unsettled**.
The shipped protocol has the same shape (`proof.revalidate()` after
`cleanup.handoff()`, `cleanup.restore()` on failure,
`src/finish_transaction.rs:1949-1969`). With `Absent` classified first, that disk
reads `Absent` — which is exactly the trace **`SY-05.b`** says does not exist
(*no trace exposes an absent task root before the deletion is proven (`FN-11`,
`FN-19`)*) and over which **`SY-05.a`** would scaffold a fresh grove. The
reorder is what makes `SY-05.b` true rather than aspirational.

### The member's MEANING was repaired after that handoff was written — read this half too

`finish-scope-k75` reviewed the change above and `finish-scope-k76` integrated
its findings, and one of them lands squarely in your scope.

**A `Reserved` state is a fact about a NAME, not about a disposition.** The class
sentence `finish-scope-k71` landed read *an artifact at a name Grove reserves
says a Grove transaction is **incomplete***, and it is false past `FN-22`'s
fourth revalidation point: a `Committed` returned unchanged makes the finish
`Applied` with the quarantine still standing, and the shipped protocol returns
success there even when disposal fails (`src/finish_transaction.rs:1953-1974`).
Under that sentence one disk was a proven success and evidence of an unfinished
transaction at once. The sentence now reads *says Grove has **work outstanding at
that name***, and §*States*' `Reserved(Quarantined)` row says in place that it is
reached on **both** sides of the fourth point.

**Why this is yours and not merely context.** `SY-05` is your obligation and the
paragraph above hands you the ORDER argument; the order is unchanged and still
rests on the window between the rename and the fourth point. What changed is that
you may not infer *the transaction is unfinished* from the classification —
`SY-05.a`'s *a missing task root means start a new grove* is sound because the
disk is not `Absent`, not because the disk proves an attempt is outstanding. The
disposition is `FN-28`'s and is proved by the correlation ticket
([`success-is-proved-by-the-ticket-not-the-tree`](../../../../docs/adr/success-is-proved-by-the-ticket-not-the-tree.md),
whose own paragraph carried the same error and is repaired).

**Both families now witness the coexistence** rather than arguing it:
`witness_FN_28_a_success_whose_cleanup_is_still_outstanding` requires
`classified in reservedClass` beside `finishSucceeded`, and Quint's
`successWithCleanupOutstanding` records `isReserved(classify(w))`. If a `SY-`
claim you write reads a reserved classification as evidence about an outcome,
that is the counterexample.

**One other repair may reach you and is recorded so you do not re-derive it.**
`FN-25.a` no longer claims the two blocked diagnoses are disjoint — the
definitions overlap reachably and the obligation is now that the **carried**
diagnosis is the one precedence selects, `OwnershipConflict` winning. Alloy's
check carried the overlap as an *exemption* and was measurably green under a
fully reversed precedence; it is not any more (`crates/grove-finish/models/README.md`,
matrix row 65). Quint's classifier moved to meet the same rule.

**Two `TT-` consequences were checked and both are that nothing changed**, so do
not re-derive them: `TT-19`'s refusal is stated over a reserved **witness** and
does not reach a standing quarantine (whose recovery is the reaper's sweep, which
refuses nothing); and `TT-18`'s three stages are unchanged, because its reserved
stage reads a reserved **witness**, which lives beneath the task root, so a free
task-root name has none — and the order over a disk only the finish protocol
creates is `FN-24.a`'s
([`obligations-follow-context-not-artifact`](../../../../docs/adr/obligations-follow-context-not-artifact.md)).
`FN-19`'s own witness was reworded from *an absent task root* to *a free
task-root name* for the same reason.

### The cell, and it may well be a declaration rather than model work

**`models/system/lifecycle.qnt`'s `classifyTree` returns `RSAbsent` first and
returns it unconditionally on `not(t.present)`**, so a tree whose root name is
free while a reserved name is occupied classifies `RSAbsent` there — the same
defect the finish column had, and the one `SY-05.b` is stated against. Whether
that disk is **representable** in this scope is what you owe, and `k71` looked
far enough to say the question is real and not far enough to answer it:

- `NO_TREE` sets `reserved: RNone`, and every transition that clears the root
  (`finishStepOp`'s `FPDeleting` branch, `FPPublished` under
  `DELETION_PROVEN_FIRST = false`, `recoverOp`'s idempotent resume) assigns
  `NO_TREE` whole. So the summary appears to conflate the root's presence with
  the reserved name's, which would make the disk unrepresentable and `SY-05.b`
  true **as a modelling fact rather than as a claim**. If that holds it is a
  declared narrowing, not a green run — say so in the README rather than leaving
  the order reading as the catalogue's.
- **`SY-05.b` is the obligation to look at first.** It is *the exhaustive absence
  of such a trace within the bound*, and a model that cannot represent the trace
  discharges it by construction. That is precisely the shape the corpus keeps
  finding (`crates/grove-finish/models/README.md`, the fourth finding), so it is
  worth a sentence either way.
- If the disk *is* representable, the repair is the finish column's: the arm
  verbatim, the order carrying the claim, and a mutant that restores the old
  ranking. `crates/grove-finish/models/finish.qnt`'s `classifiesHonestly`,
  `groveReservationStands` and rows 919/920 of its mutation matrix are the worked
  example.
- **`models/system/lifecycle.als` has no root-state classification at all** — it
  abstracts to `rooted`/`partial`/`legacy` and imports `TT-18` as the bare fact
  that the states differ (`fact TheClassificationIsOfTheRootThatIsThere`). It
  most likely owes nothing; `k71` established that by reading, not by running.
- **The task-tree scope owes nothing and that was checked in both files**:
  `task-tree.als` carries no `Absent` state at all ("no `TT-` obligation reads it
  and `SY-05` owns it"), and `task-tree.qnt`'s `classify` puts `Absent` first
  over arms that cannot both hold, since a reserved witness lives *beneath* the
  root.

### `FN-20`'s witness was clarified, and it is worth reading before you cite it

`FN-20`'s subject is the **commit's disposition** and never the task root's
state, and the catalogue's witness now says so. The two finish columns were green
on two different claims: `finish.als` stated it over the disposition, `finish.qnt`
compared the *task-root classification* with the leftover and without it — which
is *never observed* rather than *never a receipt*, and which the new state-table
row makes false, because the task-root classification **must** read the
quarantine. The wide reading would also forbid `FN-21.b`'s reaper reading its own
cleanup marker. `models/system/README.md` cites `FN-20` as a scope boundary
(*which interruptions recovery can settle is `FN-20`'s classification*); that
citation is about the finish scope's own reach and is unaffected, but read it
against the clarified wording before relying on it.

### One record in your reading chain changed

[`root-lifecycle-stays-with-its-receipt`](../../../../docs/adr/root-lifecycle-stays-with-its-receipt.md)
lost its open question. The general form of *once the caller grades an effect
applied it never ungrades it* was **declined**, and the argument is now stronger
than the one that record carried: `FN-22`'s table has two rows that are exactly
the transition the obligation forbids, so Grove could not honour it even if it
wanted to. A new record,
[`success-is-proved-by-the-ticket-not-the-tree`](../../../../docs/adr/success-is-proved-by-the-ticket-not-the-tree.md),
carries `FN-28`'s restated operands and is cited from both.


## Decisions (running log)

### The five items are not five: 15, 16, 17 and the vocabulary question are one word doing two jobs

The task file predicted that items 15 – 17 and the `layout-preflight` vocabulary
question are "all about what `SY-04.b`'s gate is over". They are, and the answer
is smaller and sharper than four dispositions: **the catalogue used *lifecycle
transition* for two sets and defined neither.** `models/system/lifecycle.als`
read it as §*Actions*' seven-member Lifecycle **group** and witnessed all seven
of it, one *alone in an iteration* each; `models/system/lifecycle.qnt` read it as
the stage-changing steps, which that group contains **none** of. Two green
columns, two sets with **no member in common**, and no enumeration had flagged
`SY-04` as underdetermined — this is entry 048's `FN-13` shape found by reading
the two columns against each other rather than by reading either.

**The catalogue's own *so that* clause decides it.** `SY-04` ends *so an invalid
configuration leaves the working tree byte-identical*, and a gate in front of
`close-epoch` or `release-lease` buys that consequence nothing, because neither
writes a tree. A claim whose justification reaches only part of its own
quantifier is stated too wide. §*Claims — system lifecycle* now defines the term
and `CONTEXT.md` carries it with three `_Avoid_` lines.

### The tally — five items, one vocabulary question, two inherited cells

**MC-cascade** = both families must answer something new or changed ·
**MN-count** = `models/run.sh --list` does not move. It did not move: **130
before and after**, which is `closed-set-additions-k74`'s measurement holding a
third time — a definition is not an obligation, and neither is a state class.

| # | item | mark | disposition |
|---|---|---|---|
| 10 | `SY-13` false over the `Legacy`/`Foreign`/`Malformed` sinks | MC-cascade, MN-count | **granted** — the repair the catalogue did not consider is now its text, with the companion as a checked conjunct |
| 15 | does `SY-04.b` owe `SY-03`'s *a preflight is never a licence*? | MC-cascade, MN-count | **YES, and this column was the defect** — the shipped driver revalidates twice an iteration |
| 16 | `SY-04.b`/`SY-14.b` over-applied to `release-lease` | MC-cascade, MN-count | **neither offered repair** — `release-lease` is not a transition; the dead end dissolves with the definition |
| 17 | the byte-identical clause is stated over a system when true only of Grove | MC-cascade, MN-count | **granted, and generalised once** rather than patched twice |
| 18 | `SY-14.b`'s *every action* is every action ON THE TREE | MC-cascade, MN-count | **granted** — both columns had reached it independently; now the text, with a control on the disposed reading |
| — | `layout-preflight` doing two jobs | MC-cascade, MN-count | **the catalogue owed `validate-config`** — §*Actions* was a row short |
| — | `SY-06.b`'s cell (`task-tree-scope-k70`) | MC-cascade, MN-count | **answered** — the class split, its reason, its witness and its control |
| — | `SY-05.b`'s cell (`finish-scope-k71`) | MN | **declared, and the declaration is a command** |

### 15 · The product is the referee, and it had already written the answer down

`outcomeOn` gated transitions on `d.configValidated`, the driver's **recorded
verdict**, while the layout gate three lines below read `w.layoutOk` live — so a
`configChange` between the validation and the transition left the validation
standing as a licence. This column retained the counterexample rather than
repairing it, because repairing it meant deciding what `SY-04.b` means.

**It means what `SY-03` means, and `src/loop_driver.rs` says so.** That file is
the **sole** caller of `SessionConfig::load` and calls it **twice an iteration** —
once before the tree mutation, once before the launch — which
[`complete-session-configuration`](../../../../docs/adr/complete-session-configuration.md)
states as *validated in full, before every tree mutation and again before every
launch*. So the licence was never the design; it was the obligation not saying
what `SY-03` says. **`lifecycle.als` read the configuration live and was green on
the stronger claim the whole time.** No ADR needed changing: the record that
settles it already said it, one document away from the obligation that did not.

`base` now closes the licence and the retained counterexample became
`mutant_config_licence` — the configuration twin of the file's own
`PREFLIGHT_LICENCE` dial, and **isolating, asserted in the same run**: the weak
conjunct stays green under it, so the control kills the clause the disposition
added rather than the one that was always there.

### 16 · Both offered repairs are refused, and the second is refused explicitly because it was the expensive one

The task file named two: exempt the release, or admit process death; and it
predicted the second would earn an ADR. Neither is owed. `release-lease`
advances no stage and writes no tree, so it was never in `SY-04.b`'s quantifier —
the gate that appeared to reach it was `lifecycle.als` reading *transition* as
the group. **The dead end dissolves with the definition and no reachability
quantifier moves.**

**Admitting process death is refused with its cost stated rather than merely not
chosen.** `crash` is the world's (§*Actions*; `CONTEXT.md`'s *Admitted action*),
so admitting it changes what **every** reachability claim quantifies over — and a
sweep in which the loop may always die finds no dead end anywhere, which is the
argument this catalogue already makes for refusing to count a hand edit as an
exit. `CONTEXT.md`'s entry stands unchanged, and the refusal is in the ADR with
a **Reopen** condition.

### 10 and 17 are one class and get ONE general clause plus two local ones, and the general one nearly went wrong

Both are entry 042's class — *a claim stated over a system when it is true only
of what Grove does*. The task file asked whether one general qualification beats
two local ones. **Both, and the split matters**: the general rule is at the head
of §*Claims — system lifecycle*, and each claim still carries its own
qualification, because §*Actions*' own established rule is that an obligation
ranges over what its scope admits **and says so in its own text**. A silent
default would make every under-specified claim quietly narrower in exactly the
direction nobody is looking.

**The obvious general rule is FALSE and `SY-05.b` is the counterexample.**
*`SY-` claims are about Grove only* would have been the tempting sentence, and
`SY-05.b` binds the world deliberately — it rests on `EN-14` and on §*States*'
*a task root whose deletion is not yet proven is never `Absent`*. So the rule is
about **naming what bounds the world**, not about excluding it. That distinction
is the whole content of the clause and it was found by looking for a
counterexample to the draft rather than by generalising the two instances.

### 18 · Both columns had already agreed, which is the cheap case, and it still earned a control

`lifecycle.als` states `SY-14.b` over `TreeAct` and calls the restriction *a
reading rather than an economy*; `lifecycle.qnt` uses
`ADMITTED.filter(touchesTree)`. Same argument, reached independently: a block is
a property of the **tree**, so an action that reads and writes no task tree
cannot name a block it never read. The catalogue adopts it, keeps `SY-14.a` over
the literal set, and states in place why `release-lease` returning `Applied` on a
blocked tree is consistent with `FN-29.b` — *every refusal leaves nothing
standing* is not *everything leaving nothing standing is a refusal*.

**`mutant_block_named_by_all` runs the disposed reading and kills the
obligation**, with `SY-14.a` asserted green beside it. An agreement between two
columns is the weakest evidence on this list precisely because neither had to
argue, so it is the one that most needed the literal text run rather than
described.

### The two inherited cells, and one of them is a declaration with teeth

**`SY-06.b` (`task-tree-scope-k70`).** `lifecycle.qnt` carries
`RSPartialScaffold(ScaffoldClass)` with the catalogue's ordered three-way test,
`RScaffoldIncomplete(class)` in place of the `WitnessPending` borrow, the third
witness, and a control restoring the fall-through. **`lifecycle.als` owes
nothing, and that is established rather than asserted**: `SY_06b` is a
**biconditional** — the completion applies *iff* `some World.partial` — so it
already forbids completing anything that is not the exact subset, and a third
non-completable mark changes nothing it says. A one-sided *refuse if legacy*
would have needed a new arm. **A biconditional survived a refinement of its own
subject's state space**, which is worth more than the cell it saved.

**`SY-05.b` (`finish-scope-k71`).** The disk §*States*' new order exists to
classify — root name free, quarantine standing — is **not representable** here:
`NO_TREE` sets `reserved: RNone` and every transition that frees the root assigns
`NO_TREE` whole. So `SY-05.b` is true as a **modelling fact** rather than as a
claim over that disk, and the order is checked where its subject lives
(`FN-24.a`, matrix row 49). Written as
`inv_SY_05b_absence_and_a_reserved_name_are_not_separable_at_this_scope` rather
than as a README sentence, because **a declared narrowing that cannot be broken
by a later edit is not a declaration, it is a hope**: a slice that gives this
scope an independent quarantine breaks that command loudly.

### A fourth member joined `SY-13`'s excluded class, and neither leaf that produced it could have seen it

`task-tree-scope-k70` split the witnessless root and does not own `SY-13`;
`SY-13`'s exclusion list was written before the class existed.
**`PartialScaffold(Ambiguous)` meets the same test as `Legacy`, `Foreign` and
`Malformed`**: Grove's scaffold writes `Exact`, only a world write into an open
scaffold makes it `Ambiguous`, and Grove then refuses everything **including the
completion** — so the loop has no admitted exit. It is in the catalogue's
`SY-13.a` with the other three, and `isHandEditRefusal` was renamed
`isWorldOnlyRefusal` because a foreign write is not a hand edit and the fourth
member arrives by one. **A decomposition that splits a state must be swept
against every claim quantified over states, and neither the splitting leaf nor
the claim's owner is positioned to notice.**

### Three drafts were refuted by instruments rather than by reading, and the third is the most transferable

Recorded together because the pattern is the leaf's main methodological output.

- **`SY-04.b`'s consequence, drafted too wide.** Written *no step of Grove's
  changes a byte under an invalid configuration*, the invariant failed on its
  first run: an **ambient** operation writes the tree, correctly, because the
  session configuration is the driver's launch policy and `grove-llm` never
  reads it. The catalogue now scopes the clause to the driver's transitions and
  says why. The wider flag is kept beside the asserted one so a later reader sees
  where the boundary is rather than being told there is none.
- **`SY-06.b`'s completion flags, drafted over the classification.** Stated as
  `classifyTree(...) != RSPartialScaffold(PSExact)` they are a restatement of the
  gate the obligation is about, and `mutant_absent_witness` went **green** under
  them — it breaks the classification, so a legacy tree classified `Exact` and a
  classification-derived flag could not see that it had been completed. Restated
  over the **disk** (`exactSubset`), which is what the catalogue's own test is
  over. **An existing control caught a new defect in the claim it was written to
  defend**, which is the case for keeping controls green-asserted rather than
  merely present.
- **QUINT'S `match` HAS NO NESTED CONSTRUCTOR PATTERNS, AND IT TYPECHECKS.**
  `| RSPartialScaffold(PSExact) => …` binds a **variable** named `PSExact`,
  matches every `RSPartialScaffold(_)`, and leaves the arm below it dead. The
  suite then reported `SCPartialAmbiguous` unreachable while `classifyTree` was
  returning it — a green run describing a state it could not see. Confirmed on a
  four-line repro (`f(Y(B))` returns the `Y(A)` arm's value). **The trap had
  never been reached in this corpus because the other parameterised state,
  `Reserved(class)`, is matched `RSReserved(_)` everywhere**: a parameterised
  state is safe exactly while nobody branches on the parameter, and the first
  branch is where the trap is.

### The Alloy `SY-04.a` restatement needed a second flag, and the frame-hole conjunct paid for itself four times

`spent` could never have carried `SY-04.a`: it is set by the Lifecycle **group**,
which contains none of the transitions, and the group and the transitions must
both be available in one iteration — `open-epoch` spends the group's turn and the
iteration still has to take its transition. So `Proc` gained `moved`, spent by
the three steps that **complete** a transition and guarded by all four. Root
initialisation is one catalogue action modelled as two steps, so the count is
taken where the format witness lands: the uninterrupted pair counts once, and a
completion finishing an earlier iteration's initialisation is that iteration's
own — the interrupted case `SY-06.b` exists for. A **refused** attempt
transitions nothing and does not spend, which is `lifecycle.qnt`'s counting rule
reached independently.

**`SY_04a`'s third conjunct is not a third claim.** *Nothing but a counted
transition or the iteration boundary moves the flag* exists because a `var` added
to `Proc` and forgotten at one site is left **free** — which does not error and
does not fail, it quietly weakens the two conjuncts above into statements about
an unconstrained field. It found four holes in one sitting: `doSelect`,
`doAcquireLease`'s applied branch, the `doReleaseLease`/`doCloseEpoch`/
`doOpenEpoch` frames, and — the one no reading would have found — `doLaunch`
framing the **launched session's** fields by hand (`s.spent'`, not `p.spent'`),
so the new field was free on the wrong process. **A field-addition checklist is
not the instrument; a claim that the field only moves where it should, is.**

**And the old `SY_04a` command is gone rather than kept.** The pair *a group
action needs `fresh` and sets `spent`* is true of this file's admission machinery
and is that machinery read back to itself — the transcription shape
[`obligations-follow-context-not-artifact`](../../../../docs/adr/obligations-follow-context-not-artifact.md)
records at `TT-24.c`. The machinery is untouched; what is gone is the command
that credited a coverage cell for checking it, and the seven witnesses that
offered *layout-preflight alone in an iteration* as evidence for a claim whose
own justification is about the working tree.

### One ADR, and the AND test was applied to the alternatives as well as to the decision

[`a-lifecycle-claim-says-what-it-is-over`](../../../../docs/adr/a-lifecycle-claim-says-what-it-is-over.md)
carries both clauses — what a claim is over, and what a transition is — as one
record rather than two, because both answer *what is this claim quantified over*,
both were decided by the same referee (the shipped product), and both rest on the
same failure mode. `content/ADR-FORMAT.md`'s legs:

- **Hard to reverse** — §*Actions* gained a row; `SY-04.a`, `SY-04.b`, `SY-13`
  and `SY-14.b` all have restated text; both families' `SY-04` commands moved;
  `lifecycle.als` gained a `var` on `Proc` with a total frame; four Quint
  controls exist against the disposed readings.
- **Surprising without context** — two independently built families instantiated
  one word as sets with no member in common and both stayed green, and two
  claims were **false as literally worded** with both families narrowing both.
- **A real trade-off, with alternatives that were actually taken rather than
  straw men** — the Lifecycle-group reading is one column's; *admit process
  death* is one of the two repairs the record inherited; a silent default and a
  blanket *`SY-` claims are about Grove* are the two general rules that look
  right and are not. Each is rejected on a stated cost and carries a **Reopen**.

**No other record needed reworking, and that is a conclusion rather than a
skipped step.** `complete-session-configuration` already said what `SY-04.b` did
not and is cited rather than edited; `one-live-driver-per-working-tree` is
untouched for the reason `closed-set-additions-k74` gave;
`CONTEXT.md`'s *Admitted action* is checked and stands, which is item 16's
disposition rather than an omission. `CONTEXT.md` gains **Lifecycle transition**,
because the term was resolved here and was doing two jobs before.
`CONTEXT-MAP.md`'s ADR-ownership list gains the record, verified by enumerating
`docs/adr/*.md` against the map rather than by reading the list — 24 of 24.

### Run lines — both cells, coverage asserted, and one run discarded on this leaf's own mistake

The catalogue, both model files, the control file and `models/system/README.md`
were **frozen before each reported run started**, their SHA-256 digests recorded,
and recorded again at the end **unchanged** — `models/run.sh` reads the catalogue
as its manifest and the scope README for declared gaps, so all five are subjects
rather than bystanders.

```sh
models/run.sh --scope lifecycle --family quint   # 103 commands, 25 of 25 cells, exit 0
models/run.sh --scope lifecycle --family alloy   #  71 commands, 25 of 25 cells, exit 0
models/run-controls.sh                           # 10 passed, 0 failed, exit 0
```

Both report `-- cells: 25 complete, 0 declared gaps, 0 empty, of 25`. **No cell
is contested** — neither family declared a gap where the other answered, and the
GAP-line count in all three model READMEs is unchanged, so this leaf declared and
removed no gap anywhere.

- **Quint — 103 commands, 5m 45s wall, 395s CPU**, quint 0.32.0, 8000 samples,
  depth 24, seed `0x5e0a51d3c0ffee01`. Ten commands are new and every one is a
  control or a witness: four `inv_fail_MUT_` controls with their four
  asserted-green neighbours, `SY-06.b`'s third witness (444 of 8000),
  `inv_SY_06b_a_root_carrying_grove_s_own_proof_is_never_legacy`, and
  `SY-05.b`'s representability declaration.
- **Alloy — 71 commands, 12m 20s wall, 416s CPU, run ALONE.** The cell went
  73 → 71 and got **three times dearer** (4m 27s → 12m 20s). The commands did not
  multiply, so the cost is the state space: `Proc` gained a `var moved: lone
  Flag`, doubling per-process state at every step of every temporal command.
  **A one-bit field on a signature is not a one-bit change to a bounded temporal
  search**, and that is the figure the next session should budget from rather
  than the inherited 4m 27s.
- `models/run-controls.sh` was run because commands of a shape the runner
  classifies (`inv_fail_MUT_<OB>_…`) entered the suite, and the controls are what
  assert the classification rather than the commands.

**One Alloy run was discarded and it is recorded rather than tidied away.** A
`models/system/README.md` edit — the Quint run line — landed **while the Alloy
run was reading that file**. The edit added no GAP line and was inert to the
runner, and that is not the test: `closed-set-additions-k74` killed a run over
the same shape and stated the rule this leaf then broke anyway — *comment-only is
an argument for not re-running, not for back-dating a run that already started*.
**Third instance in this node.** The run was killed, all remaining edits were
finished, the subjects were re-frozen, and the reported Alloy cell ran alone
afterwards. The Alloy run line itself was written after that run, which is a
record of a run rather than a moved subject, and the GAP-line count is verified
identical either side — `finish-scope-k76`'s distinction, applied.

### The closing sweep — the node's own obligation, and the negative control CONTAMINATED ITSELF

```sh
grep -rn "<handle>" . --exclude-dir=.jj --exclude-dir=target --exclude-dir=.grove \
  --exclude-dir=_apalache-out --exclude-dir=.review-tmp
```

**No catalogue finding is left saying `formal-synthesis-k16` owns it.** Every
site of that handle, and of `catalogue-disposition-k64`, is inside
`docs/formalism-findings.md`; **zero live artifacts name either.**
`closed-sets-k69` finds nothing anywhere.

| control | requirement | result |
|---|---|---|
| subject | node handles find no LIVE site | `formal-synthesis-k16` 0 · `catalogue-disposition-k64` 0 · `closed-sets-k69` 0 |
| positive | a live sibling finds its own | `cross-model-replay-k15` finds its own sites |
| negative | an invented handle finds none | see below |
| cross-tree | every LEAF handle finds its own | `k70` `k71` `k72` `k73` `k74` `k75` `k76` all non-zero |

**The log's clean condition is a disposition beside the finding, never the
absence of the name** (`routing-and-prose-k73`'s rule), and it holds: the
disposition-line count is 23, and the five sites this leaf owed — entries 042,
043 and 046 — each carry one. Entry 049's *To `catalogue-disposition-k64`*
hand-off carries the node's own closing disposition, so a reader meeting the
hand-off finds what became of all 93 sites beside it.

**THE NEGATIVE CONTROL IS NOW INVALID AND THE REASON IS WORTH MORE THAN THE
CONTROL.** `formal-synthesis-k99` has been the invented handle for three sessions
and it went from **0 sites to 1** the instant a durable record said it found 0.
**A control handle, once named in a durable artifact inside the swept tree, stops
being a negative control** — and the same self-reference applies to every count:
writing *31 sites* makes it 32. So the log's disposition quotes no counts, this
sweep used a fresh handle (`lifecycle-scope-k98`, 0), and the trap is written
into the log for the next sweep. **An instrument whose report lives inside its
own subject measures itself**, which is a general hazard for any grep-based
sweep this corpus runs.

**The three items that reach this node through prose and not through any site
were checked BY NAME**, as the task file required, because a future re-run
reports clean whether or not they were decided:

- **Item 4 (`ONotEntered`)** — declined, and the rule generalised:
  `semantic-contract.md` says *Grove has no confirmation gate at all*, so a
  transaction not entered for want of confirmation is a call that was never made.
- **Item 7 (`FN-10.b`/`FN-32` refuse-or-block)** — landed in the *one artifact,
  three contexts* table, whose second row now fixes a **function of the step**
  (`Refused(ReservedNameOccupied)` while nothing stands, `Blocked(OwnershipConflict)`
  once something does) and names no owner.
- **Item 19 (`EN-11`'s twice-wrong controls row)** — `EN-11` reads `TT-02`,
  `TT-03`, `TT-13.c`, `TT-25`; `TT-24.b` is on `EN-13`, where its dependency is.

### What this leaf did NOT do

- **The whole-repository run is not owed here** and was not attempted. The node
  brief asks for a green **per-scope** run for every scope a child touched; the
  repository-wide invocation with its `ordinal-fs-tree` positive control is
  `formal-synthesis-k16`'s own `Notes` obligation and belongs to
  `handoff-audit-k66`. **Two scopes are untouched and that is checked rather
  than assumed**: no `TT-` or `FN-` obligation's text moved, §*Outcomes*' closed
  sets are unchanged, and the only shared-vocabulary edit is §*Actions* gaining
  `validate-config` — a Lifecycle-group row that no `TT-` or `FN-` obligation
  quantifies over.
- **The four routed model findings were not absorbed.** Items 31 (`SY-10.b` /
  `SY-11.b` over `WGen`), 32 (`SY_11a` and a repeat acquisition), 33 (`SY-05.b`'s
  importing seam) and 35 (an `EN-` row for process death) still name the model
  owners. Item 33 was the closest call, because `SY-05.b`'s cell landed here —
  but the cell was *representability*, and the stronger claim is still the
  abstraction change that leaf declined.
- **No obligation was added, removed or re-scoped.** `models/run.sh --list`
  prints **130** before and after. Every disposition is a restatement of an
  existing obligation, a §*Vocabulary* row, or a definition.
- **`docs/formalism-findings.md` was not revised**, only appended to: five
  `> **[disposed by …]**` lines beside findings that named a disposition as owed,
  and the node's closing disposition beside entry 049's hand-off. No entry's own
  prose is rewritten.
