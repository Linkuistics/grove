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
