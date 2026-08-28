# quarantine-necessity-k79 — brief

## Goal

Decide `TODO.finish_process.md` Q1 and Q4's three cleanup rows against a control
for the only no-quarantine strategy the environment table actually permits.
`finish-verdicts-k78` left both at `defer` and commissioned this; it is the last
thing standing between the formal phase and a verdict on 10,366 lines and 31
`unsafe` blocks.

## Context

**What is already settled, and must not be re-derived.**
[`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
carries the four questions, their dispositions and the four binding constraints.
Q2 and Q3 are `keep` on witnesses reached under the incumbent, and this leaf does
not reopen them. Q1's pre-registered criterion is **met in full and cannot decide
anything**: it is stated over `relax_EN_03`, a counterfactual-capability control,
and running it to completion returns `delete/replace` for a protocol that needs
the atomic recursive deletion `EN-03` says does not exist. The replacement
criterion is availability-typed and is in
[`semantic-contract.md`](../../../../docs/specs/semantic-contract.md), *What the
models must be able to decide*.

**The one measurement that is missing.** The available candidate is **in-place
disposal that is non-atomic**: the task root's contents removed entry by entry at
its own name, with no quarantine rename and no cleanup marker. **No command in
either family runs it** — which is not the same as *cannot express it*, and the
difference is this leaf's first economy. Quint ties the quarantine's existence to
`ATOMIC_DISPOSAL` in one `const` — its true branch replaces `SQuarantineRename`
and every step after it with a single `SDisposeInPlace`, itself one atomic
`settle` ([`finish.qnt`](../../../../crates/grove-finish/models/finish.qnt), the
`SRevalBeforeQuarantine` arm and `SDisposeInPlace`) — but `Place = AtRoot |
InWitness | Disposed` is already per-entry and `SDisposeEntry` already removes
entry at a time with a resumption point. **What is missing is a transition, not
expressivity**: one `const`, one or two step arms, and the bookkeeping in
`persistentEffect`, `ALL_STEPS`, `DECLARED_STEPS` and `phaseOf`. Alloy runs no
counterfactual-capability mutation at all. `k65` read the coupling as a fact
about the protocol; it is a fact about the encoding.

**THE STRATEGY DIAL ALONE RETURNS A FALSE GREEN. THIS IS THE LEAF'S WHOLE RISK.**
Built and run as it stands, the retained shared-safety set is green over the new
candidate **for structural reasons**, and the run would read as `delete/replace`:

- `classifiesHonestly` guards both of its failable conjuncts on
  `groveReservationStands(w)` = `w.witness != WNone or w.quar.present`
  ([`finish.qnt`](../../../../crates/grove-finish/models/finish.qnt), `groveReservationStands`, which `classifiesHonestly` guarded both failable conjuncts on).
  With no quarantine and the witness gone, both conjuncts are vacuous — so
  `FN-24.a` cannot fail. And with the witness retired **last**, the disk reads
  `Reserved(Published)` throughout and `FN-24.a` does not fail even in the
  interesting order. **The predicted violation is order-dependent and the model
  says so; do not assume it.**
- `FN-32`'s three transaction-side sites are `SCreatePreparing`,
  `SQuarantineRename` and `SCreateMarker`. A candidate with neither quarantine
  nor marker reaches only the first, which it inherits from the incumbent
  unchanged — so `FN-32` is trivial over everything the candidate changes, the
  same defect `finish-verdicts-k78` found in `relax_EN_03`'s retained set and
  could not repair there either.

**So the apparatus is the expensive half and it must be built first.** Two
additions, and both are the kind whose cost this brief warns about:

- **A §*States* member for the partially disposed root** — the candidate's
  analogue of `Reserved(Quarantined)`, e.g. `Reserved(Disposing)`: the root at
  its own name with a document at a reserved name. **This is where the argument
  for `keep` actually lives**: `k65` argued the candidate inadmissible because
  §*States* has no member for a partially removed root, and §*States* is a table
  this experiment authored and already extended once. Deciding whether such a
  member is admissible **is** deciding Q1, and it is a catalogue change with the
  manifest cascade both families pay.
- **An `FN-32` site the candidate can reach.** Alloy solved the mirror problem —
  `MarkerReplace` is *the only `groveActs - Reap` member whose marker mutation is
  gated on ownership*, "so this is where the claim has content"
  ([`finish.als`](../../../../crates/grove-finish/models/finish.als):5930). The
  candidate has no marker, so the site has to be the resumed sweep's, which is
  the same question Q4's reaper hole asks. **The two commissions are one
  question**; do not solve them separately.

**And `FN-24.b` is not the rejector, however much it looks like one.** Its
invariant is `ALL_STEPS.forall(s => ... or DECLARED_STEPS.contains(s))`, so
whether the candidate's removal step passes is a modeller's choice about a static
table. That is *a claim stated over a model's own classifier*, which this node
has already been burned by twice.

**Q4's three rows are blocked in three different ways, and the ADR names each.**
Quint's three are one bundled result from `relax_EN_03`, so by the record's own
rule they are counterfactual and supply **no** qualifying cell. Alloy's Q4-6 is a
real available-world `none` bounded by the reaper's ownership-proof hole — which
must be settled one way or the other before the row can be read: either an
obligation is stated over the sweep's ownership proof (a manifest change, both
families, cascading) or the catalogue records the silence and the `none` cells
are annotated as such. Alloy's Q4-7 needs neither: row 45's green is a vacuity
artifact of its own mutation, and what would replace it is a control that
narrows the replace transition away **while keeping an `FN-32` site with
content**. Do not carry the ADR's old universal sentence about "the reaper's
actions" forward; `FN-27` is quantified over a set containing `Reap` and stayed
green, and the true statement is the narrow one.

## Done when

- Both families can express a no-quarantine strategy independently of the
  capability dial, and the non-atomic in-place candidate is run against the
  retained shared-safety set at the incumbent's bounds, with reachable
  antecedents and a kill control for every retained claim asserted.
- Q1 is classified from that run: `delete/replace` if the candidate retains the
  set, `keep` if a runnable control shows it breaking one, `defer` only if the
  run itself is blocked by something this leaf names.
- Q4's reaper-coverage question is settled — an obligation stated, or the silence
  recorded — and the three cleanup rows are classified from the result.
- The ADR is reworked **in place** to what now binds — the grove skill's
  `ADR-FORMAT.md` rule: edit, merge, split or delete, and never append a
  superseding record — and the catalogue, both model READMEs, this brief and the
  implementation brief say one thing. If the verdict changes the ADR's title, the
  slug changes with it and every citation is repointed; `k78` found 31 across 17
  files.
- If the answer is `delete/replace`, the narrowly named `impl` leaf is inserted
  immediately before `collapse-application-k27` and
  `.grove/03-implementation-k3/BRIEF.md`'s *Promoted from `TODO.finish_process.md`*
  paragraph is corrected; if it is `keep`, that paragraph's no-op stays and says
  why on evidence.
- The finish-family commands are green in both families after the changes. The
  whole-repository run is `handoff-audit-k66`'s and is not this leaf's.

## Decomposition

The order encodes one dependency, one deliberate refusal to split, and one
independence.

1. **`honest-classification-k80`** — the apparatus, in Quint, and the one
   question whose answer changes what every later child does: *can a root being
   disposed of in place be honestly classified at all, and by what?* It builds
   the strategy dial because a restatement of `FN-24.a` with nothing to be about
   is unfalsifiable, and it decides §*States*' member for a partially disposed
   root. **First, because the apparatus is what the record says must be built
   before the strategy is read**, and because both of the later columns' cells
   are shaped by what this child lands in the catalogue.
2. **`sweep-ownership-k81`** — the `FN-32` site the candidate can reach and Q4's
   reaper-coverage hole, **as one question**, which is the brief's own
   instruction. Its either/or — state a shared-safety obligation over the sweep's
   ownership proof, or record the silence and annotate the `none` cells — is the
   one manifest change in this subtree, and it is separated from child 1 because
   only child 1's answer says what artifact the candidate's ownership proof would
   even be about.
3. **`alloy-candidate-k82`** — the Alloy column's available-world candidate,
   mirroring children 1 and 2. It is its own leaf because `finish.als` runs **no**
   counterfactual-capability mutation at all, so the Alloy half is a new
   instrument rather than a transcription, and because the Alloy finish cell is
   this scope's long pole (180 commands, 14 m 33 s measured; ~18 min alone).
4. **`q1-q4-verdict-k83`** — Q1 and Q4's three cleanup rows classified from what
   children 1 – 3 ran, the ADR reworked **in place**, and the catalogue, both
   READMEs, this brief and `03-implementation-k3`'s reconciled. The `impl` leaf
   before `collapse-application-k27` is inserted here or the no-op paragraph is
   corrected here; neither is any earlier child's.

Children 2 – 4 were cut by child 1's session, which is why their bodies carry the
specific inherited items rather than a generic goal sentence.

**No child before 4 may classify Q1**, and that is the whole point of the order.
Each of 1 – 3 produces evidence and records what its own green does and does not
establish; a child that reads its own run as a verdict has done what
`finish-verdicts-k65` did.

## Notes

**Budget before you cut.** This brief's *Three measurements to budget from*
applies in full: an obligation addition opens an empty `(family, obligation)`
cell both families must fill; `models/run.sh --list`'s count is silent about the
expensive kind of change; and the Alloy finish cell is the long pole. If the
`FN-24.a` predicate change or the reaper obligation turns out to be a manifest
change, this leaf is bigger than one session and should decompose rather than
run long.

**Cut a `review-design` beside this leaf if it lands a verdict.** The producer
that decided these questions the first time spent no reviewer and was wrong in
two of four; a session that reverses or confirms that on new evidence of its own
making is the same shape.

## Decisions (running log)

**This leaf is bigger than its brief, and its own `Notes` named the trigger
before the session did.** The `Notes` say to decompose *"if the `FN-24.a`
predicate change or the reaper obligation turns out to be a manifest change"*.
The reaper obligation is one branch of an either/or the brief itself states —
*state a shared-safety obligation over the sweep's ownership proof, or record the
silence* — and the first branch is a manifest change by construction. That alone
is the named trigger. Two further measurements make it decisive rather than
marginal:

- **The `FN-24.a` predicate change is not a manifest change and is worse than
  one.** `models/run.sh --list` counts claim headings and obligation bullets, so
  restating `FN-24.a`'s two failable conjuncts moves no number — while the
  conjuncts are checked in **both** families and in every `relax_`/`mutant_`
  module that carries an `inv_FN_24a*` command. This is exactly the *expensive
  kind of change the cheap check is silent about* that the node brief's *Three
  measurements to budget from* names.
- **The `Done when` is five deliverables over two families plus an ADR rework.**
  Both families expressing a strategy dial; Q1 classified from a run; Q4's
  reaper question settled; the ADR reworked in place with slug and 31 citations
  at risk; and an `impl` leaf inserted into `03-implementation-k3` or a paragraph
  corrected on evidence. Four of the five are independently verifiable, which is
  the vertical-slice test passing four times over.

Decomposed with `grove-llm leaf-decompose`. The children and their order are in
the brief's *Decomposition*.

**The apparatus defect is located, in both families, and it is one defect rather
than two.** `FN-24.a`'s two *failable* conjuncts were guarded on the incumbent's
own artifacts in both columns — Quint's is repaired by the closing section
below, Alloy's is not:
[`finish.qnt`](../../../../crates/grove-finish/models/finish.qnt) —
`groveReservationStands(w) = w.witness != WNone or w.quar.present`, which
`classifiesHonestly` guarded both conjuncts on — and
[`finish.als`](../../../../crates/grove-finish/models/finish.als):4770-4771 —
`classified = SAbsent implies (no Slot.occ and no Quar.qRid)`, and the same
consequent for `currentStates`. **A shared-safety claim whose failable half is
guarded on the incumbent's artifacts cannot judge a candidate that has none**;
the antecedent goes false and the conjuncts are satisfied vacuously. That is the
`FN-32` vacuity `finish-verdicts-k78` found, in a second place and in both
families, and it is why the brief's warning that the strategy dial alone returns
a false green is structural rather than a matter of degree.

**And the neutral form is already written in the catalogue, one section away.**
§*States* states the `Reserved` class as *an artifact at a name Grove reserves
says Grove has work outstanding at that name*, and states its load-bearing
property as *no transient state may be observable as a different stable state* —
both over a role rather than over the quarantine, which is the form `FN-20` was
deliberately given *so Q1 can be decided against it*. `FN-24.a`'s conjuncts are
the one place that discipline was not applied. Restating them there is what gives
a candidate something it can fail, and it is child 1's.

**What is NOT decided here, and is deliberately left to child 1 to run rather
than to inherit.** Reading the Quint step arms (`SDisposeEntry` at 1832,
`SQuarantineRename` at 1724) predicts that a stepwise in-place disposal which
retires the published witness **last** classifies `Reserved(Published)`
throughout and then passes through a root that is present, witness-free and
empty — a `Current(*)` row that `classifiesHonestly` accepts. That is a reading
of the source and not a run, it is the order-dependence this leaf's own `Context`
says not to assume, and it is exactly the kind of prediction this node has twice
been wrong about. Child 1 builds it and measures it.

**Baseline, frozen before anything moved, and it reproduces.** `models/run.sh
--scope finish --family quint` was run on the unmodified tree at `wkzptosn`
(`finish-verdicts-k78`), working copy clean: **exit 0, 254 commands, 63 of 63
cells, 0 declared gaps, 0 empty, Q4 matrix 10 of 10, 5 m 25 s wall / 387 s user**
on this host at `QUINT_VERIFY=0`. The command count and the cell count are the
ones `k78` recorded, so the column reproduces off a clean checkout and every
later run in this subtree has a figure to be compared against rather than a
recollection. The Alloy column was not run here and has not moved.


## `honest-classification-k80` is closed, and this is what children 2 – 4 inherit

Child 1 retired with the apparatus repaired in the Quint column and two catalogue
decisions landed. What it established is stated here rather than left in a retired
task body, because every remaining child reads it.

**The available candidate exists, it runs, and the ADR's order caveat dissolves.**
`IN_PLACE_DISPOSAL` in
[`finish.qnt`](../../../../crates/grove-finish/models/finish.qnt) grants **no**
capability — every `EN-` assumption is `base`'s, `ATOMIC_DISPOSAL` included — and
selects the strategy: the published witness emptied one entry at a time at the
task root's own name (`SDisposeRootEntry`), the witness released
(`SRemoveRootWitness`), the root released (`SRemoveRoot`). The ADR predicted the
interesting violation only *"for a candidate that retires the witness first"*;
**no available candidate can**, because after `SEvacuate` every entry is inside
the published witness and `EN-03` denies the recursive removal that would take
both together. Witness-last is the candidate's best case *and its only case*.

**`FN-24.a`'s failable half was guarded on the incumbent's artifacts in BOTH
columns, and that is the second instance of `finish-verdicts-k78`'s `FN-32`
shape.** The claim classed *shared safety* — the one the catalogue says *names no
artifact of the incumbent* — accepted a present task root with no witness, no
quarantine and its disposal outstanding. Measured, not argued:
`wit_FN_24a_the_artifact_guarded_encoding_accepts_it`. The rule is
[`a-shared-safety-guard-names-the-role-not-the-artifact`](../../../../docs/adr/a-shared-safety-guard-names-the-role-not-the-artifact.md);
the repair is `groveWorkOutstanding`, which keeps `groveReservationStands` as one
disjunct **deliberately** — dropping it loses `mutant_no_quarantined_state`'s
kill. `inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree` fires
with **no mutation and no interruption** (`ENV_BUDGET = 0`), and all three
inherited `FN-24.a` kills stay green beside it.

**Three things to inherit rather than re-derive.**

- **§*States* gains no member, and the argument is the class sentence rather than
  the table.** A protocol that leaves a document at a reserved name recording its
  disposal's progress **does** have `Reserved(Disposing)`, on exactly
  `Reserved(Quarantined)`'s terms — `finish-verdicts-k65`'s *the table has no
  member* was never the refutation. What the table cannot supply is a row whose
  condition tests nothing, and **the runnable candidate puts no artifact at any
  reserved name at all.** So the question that survives is a question about the
  *protocol* — *is a candidate keeping a reserved-name progress document still
  one that has removed the cleanup layer?* — and it is `q1-q4-verdict-k83`'s.
- **`resumePoint` is a second instrument on the same protocol and it is sharper
  than the first — and it is `sweep-ownership-k81`'s.** Under the candidate the
  disk reads `SRevalBeforeRestore` mid-disposal (3676 traces): the protocol would
  enter the **restoration** path on a finish whose commit is proven. Once the
  witness is released it reads `SIdle` — *nothing of Grove's is outstanding* —
  while the root still stands and disposal is unfinished (3410 traces). **No
  command in the suite constrains either reading**, and the claim that could is
  the ownership proof over a resumed sweep that `k81` owns. The candidate's
  `FN-32` site and its resumption licence are the same artifact; the brief's
  *do not solve them separately* now has a measured second half.
- **The manifest did not move and no cell opened.** `models/run.sh --list` prints
  **130** before and after. Every catalogue edit is prose, a table decision or an
  obligation's own text. What the edits cost is a **matching outcome** in the
  Alloy column — `FN_24a`'s conjuncts (c) and (d) at
  [`finish.als`](../../../../crates/grove-finish/models/finish.als):4770-4771 are
  the same artifact guard, unrepaired — which is `alloy-candidate-k82`'s and is
  named rather than left for the runner to find.

**And nothing is classified.** Q1's retained set is `FN-20`, `FN-24`, `FN-27`,
`FN-32`, and **`FN-32` still has no candidate-reachable site**, so no run yet
checks this candidate against a *complete* retained set. A red `FN-24.a` under an
incomplete set is evidence about the instrument, not a verdict on the protocol.
`q1-q4-verdict-k83` is still where Q1 and Q4's three rows are decided.

**One method result, and it cost a run to learn.** A witness written to measure a
defect **as encoded** must not be written by calling the definition it is about:
`wit_FN_24a_and_the_claim_as_encoded_accepts_it` went from reached in 3410 traces
to unreached in 8000 samples the moment the repair landed in the same session —
the defect had not gone away, the sentence had come to mean something else. Write
the pre-repair predicate out inline. **This is the node's provenance rule one
grain down: a predicate is a subject too**, and the check that catches it is the
same one — measure, freeze, then repair.


## A review was cut and inserted, and three of `k80`'s repairs went in unread

`honest-classification-k80` spent its one in-session reviewer on the narrow claim
its `Notes` named, and the reviewer returned **three substantive findings against
a suite that was green**:

- **`inv_FN_28`'s second operand was an enumeration of the two protocols that
  happened to exist** — `(ATOMIC_DISPOSAL or hist.appliedAfterQuarantine)` — so
  the available candidate falsified a **shared-safety** claim, and the run
  credited `FN-28`'s coverage cell from a world in which `FN-28` was false,
  because a `scenario_` module carries only the commands written inside it and
  that one declared the witness without the property. **This is the third site of
  the shape `a-shared-safety-guard-names-the-role-not-the-artifact` records**, and
  it is what turns that record from an observation into a rule: adding a third
  disjunct was available and would have been the wrong repair.
- **The stated justification for retaining `groveReservationStands` was false in
  both halves** — measured. All three inherited `FN-24.a` kills fire without it,
  and the disk the justification named is not admitted in the module it was
  attributed to. It is retained on a different reason and **declared an
  uncontrolled conjunct**.
- **`nextInPlaceDisposable` walked `evacuated(w)` where both existing disposals
  are total**, so a `SRevalBeforeQuarantine` reached with evacuation incomplete —
  which `base` reaches — let the candidate settle `OApplied` with the root gone
  and a user entry still `AtRoot`. Latent only because the candidate's module runs
  `ENTRIES = 1` with no environment action: **the narrowing `k80` had just argued
  was safe for its kill is also what hid a bug from it.**

**So `review-design honest-classification-k84` holds position 02 and was
inserted, not appended.** `sweep-ownership-k81` is chartered to edit the very
artifacts under review, and this node's own rule — used at
`obligation-placement-k67` and `finish-verdicts-k77` — is that a gate checked
after everything it gates is not a gate. **Keys are unchanged: `k81`, `k82` and
`k83` are now positions 03, 04 and 05.** If `k84` finds nothing it creates nothing
and retires.

**One method result to carry into every later child's own instrument.** *A module
that changes what the model does must be run against **every** claim the model
has, not only the ones it declares.* That sweep — all 63 library `inv_` commands
against the candidate's module — is what found `FN-28` at all, and no suite run
under the module rule can find it for you.
