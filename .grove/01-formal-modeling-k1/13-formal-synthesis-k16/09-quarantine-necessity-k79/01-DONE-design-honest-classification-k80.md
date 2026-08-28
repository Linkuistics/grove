# honest-classification-k80

## Goal

Decide, in the catalogue and in the Quint column, whether a task root being
disposed of **in place** can be honestly classified — and by what artifact. That
is the question `finish-verdicts-k65` answered by argument (*§States has no
member for a partially removed root*) and the one the node brief says is where
the case for `keep` actually lives. No verdict on Q1 is this leaf's.

## Context

**The apparatus defect, located and identical in both families.** `FN-24.a` is
**shared safety** — the claim a candidate protocol is judged against, and the
catalogue says in as many words that it *names no artifact of the incumbent one*.
Its two *failable* conjuncts do:

- Quint — `groveReservationStands(w) = w.witness != WNone or w.quar.present`
  ([`finish.qnt`](../../../../crates/grove-finish/models/finish.qnt),
  `groveReservationStands`), which `classifiesHonestly` guarded **both** of its
  failable conjuncts on.
- Alloy — `classified = SAbsent implies (no Slot.occ and no Quar.qRid)` and the
  same consequent over `currentStates`
  ([`finish.als`](../../../../crates/grove-finish/models/finish.als):4770-4771).

A candidate with neither witness nor quarantine makes both vacuous. **That is the
`FN-32` vacuity `finish-verdicts-k78` found, in a second place and in both
columns**, and it is why the node brief says the strategy dial alone returns a
false green.

**The neutral form is already in the catalogue, one section away, and it was
written deliberately.** §*States* states the `Reserved` class as *an artifact at
a name Grove reserves says Grove has work outstanding at that name*, and its
load-bearing property as *no transient state may be observable as a different
stable state*. `FN-20` was given exactly that shape — *stated over the role
rather than over the quarantine, so Q1 can be decided against it*. `FN-24.a` is
the one place the discipline was not applied.

**Why the restatement cannot be landed on its own.** With the model's only
reserved-name artifacts being the witness and the quarantine, a textually neutral
`FN-24.a` is **extensionally identical** to the current one: nothing changes, no
control can kill it, and the repair is a green tick over no evidence — the exact
defect this node has been burned by twice (`TT-24.c`'s uncontrolled
transcription; `SY-06.b`'s history flag). **So the strategy dial is this leaf's
instrument, not its subject.** Build enough of the candidate to give the
restatement content, and read nothing off its green.

**What the candidate is, and what the encoding already has.** Non-atomic in-place
disposal: the task root's contents removed entry by entry at its own name, no
quarantine rename, no marker. `Place = AtRoot | InWitness | Disposed` is already
per-entry and `SDisposeEntry` already removes one at a time
with a resumption point. What is missing is a transition: a `const` that is **not**
`ATOMIC_DISPOSAL`, one or two step arms, and the bookkeeping in
`persistentEffect`, `ALL_STEPS`, `DECLARED_STEPS`, `phaseOf` and a candidate
branch list.

**A prediction to measure, not to inherit.** Reading the `SQuarantineRename`
and `SDisposeEntry` arms, a stepwise disposal that retires the
published witness **last** classifies `Reserved(Published)` throughout, and then
passes through a root that is present, witness-free and empty — a `Current(*)`
row `classifiesHonestly` accepts. If that holds, the interesting window is two
steps wide and the incumbent has no counterpart to it, because `FN-19` frees the
root's name in one rename. **This is a reading of the source and not a run**, it
is the order-dependence the node brief says not to assume, and this node has been
wrong about exactly this kind of prediction twice. Build it and measure it.

**Two traps the catalogue names, and both are live here.**

- **`FN-24.b` is not the rejector.** Its invariant is
  `ALL_STEPS.forall(s => ... or DECLARED_STEPS.contains(s))`, so whether the
  candidate's removal steps pass is a modeller's choice about a static table.
  Declaring them is honest; reading their green as evidence about the protocol is
  *a claim stated over a model's own classifier*.
- **A narrowed `Absent` arm, or a state vector that declines to carry the
  candidate's disk, answers the conjunct by construction.** `finish-scope-k71`
  tried both and rejected both. `stableStates` keeps every arm the catalogue's
  row verbatim and the ORDER carries the claim; whatever member this leaf lands
  must keep that property.

**Budget.** `models/run.sh --scope finish --family quint` is **exit 0, 254
commands, 63 of 63 cells, 5 m 25 s** on the tree this leaf opens on — re-measured
at `wkzptosn` rather than recalled. A state-table member and a definition leave
`models/run.sh --list`'s **130** unchanged while still costing both families a
matching outcome, so `--list` is the cheap check that is silent about this leaf's
expensive half.

## Done when

- A strategy dial in `finish.qnt`, independent of `ATOMIC_DISPOSAL`, selects
  stepwise in-place disposal with no quarantine and no marker, reusing the
  per-entry `Disposed` place and `SDisposeEntry`'s resumption point; `ALL_STEPS`,
  `DECLARED_STEPS`, `persistentEffect`, `phaseOf` and the candidate branch list
  all account for it, and `FN-24.b`'s enumeration is asked of the candidate's own
  step list.
- The disk the candidate exposes between its last content removal and the removal
  of the root is **measured** — what `classify` returns, whether
  `classifiesHonestly` accepts it, and at what frequency — and recorded with its
  trace counts, not predicted.
- §*States* has a decided answer on a member for the partially disposed root:
  admitted with its artifact named and its row written the catalogue's way, or
  refused with the reason, and either way the argument is stated over the
  `Reserved` class sentence rather than over the table's current membership.
- `FN-24.a`'s two failable conjuncts are restated in the catalogue over *Grove
  has work outstanding at a name it reserves* rather than over the witness and
  the quarantine, **and the restatement has content**: the incumbent stays green
  and a control naming the candidate kills it. A restatement with no kill is not
  landed.
- The Quint finish cell is green — command count, cell count and wall time
  recorded beside the baseline above — and every new command names an obligation
  the catalogue defines. Whatever Alloy cell or outcome this opens is named for
  `alloy-candidate-k82` rather than left for the runner to find.
- Nothing here classifies Q1, and the leaf says explicitly what its own green
  does not establish — beginning with `FN-32`, which has no candidate-reachable
  site until `sweep-ownership-k81`, so the retained set is **incomplete by
  construction** in this leaf.

## Notes

**The provenance rule applies in full.** `models/run.sh` reads its obligation
manifest out of `docs/specs/semantic-contract.md` and each scope README for its
`GAP` lines, so all of them are subjects of a run. Freeze, record digests, run,
record again; a run line written after its run is a record, and the GAP-line
count either side is what tells the two apart.

**Spend the in-session reviewer on the restatement, if on anything.** The narrow,
unexpected claim here is *this restatement has content* — that the kill control
fires for the reason claimed and not because some neighbouring narrowing made it
fire. That is the shape `finish-verdicts-k78`'s own reviewer caught a second
vacuity in.

## Decisions (running log)

**The available candidate is expressible, it runs, and the prediction was right —
measured, at `scenario_in_place_march`.** `IN_PLACE_DISPOSAL` is a `const` that
grants nothing: every `EN-` assumption the module imports is `base`'s,
`ATOMIC_DISPOSAL` included. The candidate empties the published witness one entry
at a time at the task root's own name (`SDisposeRootEntry`), releases the witness
(`SRemoveRootWitness`) and releases the root (`SRemoveRoot`). It reaches its own
successful exit in **3266** traces, `FN-24.b`'s enumeration over its own branch
in **3266**, and — the measurement this leaf exists for — it exposes **a present
task root with no witness, no quarantine and its disposal outstanding, classified
as a `Current(*)` row, in 3410 traces**.

**The order is not a modeller's choice and the ADR's caveat dissolves.** The ADR
predicted the violation only *"for a candidate that retires the witness first"*.
After `SEvacuate` every entry is **inside** the published witness, and `EN-03`
denies the recursive removal that would take witness and contents together — so
no available candidate *can* retire the witness first. Witness-last is the
candidate's best case and its only case, and the dishonest disk appears in it.

**`FN-24.a` as both families encoded it accepts that disk, and that is the
apparatus defect made falsifiable rather than argued.**
`wit_FN_24a_the_artifact_guarded_encoding_accepts_it` lands. The claim classed
*shared safety* — the one the catalogue says *names no artifact of the incumbent*
— had both of its failable conjuncts guarded on `w.witness != WNone or
w.quar.present`, so a candidate holding neither made them vacuous. **This is the
second instance of `finish-verdicts-k78`'s `FN-32` shape**, in a claim retained by
the same criterion, reached independently by both columns.

**The repair is the role-form, and it keeps both realisations.**
`groveWorkOutstanding(w, t) = groveReservationStands(w) or unsettledRootWork(w, t)`,
where the second disjunct is *an entry of the tree a live transaction has moved or
removed and not settled* — stated over `Place`, which belongs to the **user's**
tree rather than to Grove's machinery. **Both disjuncts are kept deliberately**:
dropping the first loses `mutant_no_quarantined_state`'s kill, whose disk is
`EN-11`'s orphaned quarantine beside a live root with **no transaction running at
all**, which a transaction-shaped predicate cannot see. The second is guarded on
the entry having *moved* rather than on the transaction having *written*, because
the incumbent's restoration branch legitimately reads as `Current(*)` once every
entry is back and the witness is released — a `t.persisted`-shaped predicate would
turn the incumbent's own refusal path red.

**The restatement has content, and the A/B is one disk rather than two findings.**
`inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree` is
**violated**, and the three inherited `FN-24.a` kills —
`mutant_no_quarantined_state`, `mutant_absent_classified_first` and
`relax_EN_01` — are green beside it, with `base` green. So the claim was
**strengthened**, not moved. **And the new kill needs no interruption**:
`scenario_in_place_march` runs at `ENV_BUDGET = 0` with `ENV_PHASES` and
`ENV_KINDS` empty, where every other `FN-24.a` counterexample in this suite needs
a crash.

**A measurement of a defect must survive the fix, and the first draft did not.**
`wit_FN_24a_and_the_claim_as_encoded_accepts_it` was written as
`… and classifiesHonestly(w, t)` and went from reached in 3410 traces to
**unreached in 8000 samples** the moment the repair landed — not because the
defect went away but because the sentence had come to mean something else. The
witness now writes the four pre-repair conjuncts out inline and is renamed
`wit_FN_24a_the_artifact_guarded_encoding_accepts_it`. **This is the node brief's
provenance rule one grain down**: a run whose subject moved under it is not a
measurement of the thing it reports on, and a *predicate* is a subject too.

**§*States* gains no member, and the reason is recorded instead of the row.**
`finish-verdicts-k65` rejected the candidate because the table has no member for a
partially removed root; `finish-verdicts-k78` withdrew that as an argument from a
table this experiment authored. **Both are answered by the class sentence rather
than by the table**: membership is *an artifact at a name Grove reserves says
Grove has work outstanding at that name*, so a protocol leaving a document at a
reserved name **does** have `Reserved(Disposing)` on exactly `Reserved(Quarantined)`'s
terms — nothing about the table refuses it. What the table cannot supply is a row
whose condition tests nothing, and **the candidate that can actually be run puts
no artifact at any reserved name at all.** So no row is added; the catalogue
records why, and the question that remains — *is a candidate keeping a
reserved-name progress document still one that has removed the cleanup layer?* —
is `q1-q4-verdict-k83`'s and not §*States*'.

**A second instrument landed on the same protocol and it is sharper than the
first.** `FN-24`'s own premise is *a crash does not hand the next invocation a
program counter, it hands it a tree, and what the tree says has to be enough*.
`resumePoint` is that reading. Under the candidate it returns
**`SRevalBeforeRestore`** while the entries are half gone and the commit is proven
(reached, 3676 traces) — the protocol would enter the *restoration* path on a
finish it has already committed — and then **`SIdle`**, *nothing of Grove's is
outstanding*, while the task root still stands and the disposal is unfinished
(reached, 3410 traces). **No command in this suite constrains either reading**,
because the shared-safety claim that could — an ownership proof over a resumed
sweep — is exactly the hole `sweep-ownership-k81` owns. Recorded, routed, not
acted on here.

**The manifest did not move.** `models/run.sh --list` prints **130** obligations
before and after. Every catalogue edit here is prose, a table decision or an
obligation's own text; no obligation was added, removed or re-scoped, so no
`(family, obligation)` cell opened. What the edits *do* cost is a matching
outcome in the Alloy column, which is `alloy-candidate-k82`'s and is named there
rather than left for the runner to find.

**What this leaf does NOT establish, stated because a red `FN-24.a` under an
incomplete retained set is not a verdict.** Q1's retained set is `FN-20`,
`FN-24`, `FN-27`, `FN-32`, and **`FN-32` still has no candidate-reachable site** —
the defect `finish-verdicts-k78` found at `relax_EN_03` is unrepaired for this
candidate too. So no run yet checks the candidate against a *complete* retained
set, and nothing here classifies Q1 or Q4's three rows.

**The durable record is
[`a-shared-safety-guard-names-the-role-not-the-artifact`](../../../../docs/adr/a-shared-safety-guard-names-the-role-not-the-artifact.md).**
It earns a record on all three legs of `ADR-FORMAT.md`'s AND test: hard to
reverse (it changes what every candidate protocol is judged against, in both
families), surprising without context (both independently built columns chose the
artifact guard), and a real trade-off with rejected alternatives (replace the
artifact disjunct — loses a kill; narrow the `Absent` arm — answers by
construction, and `finish-scope-k71` rejected it once already; add a §*States*
member for the candidate — it has no artifact to test).

**The frozen run, and the provenance rule honoured in both directions.** The four
subjects `models/run.sh` reads — `docs/specs/semantic-contract.md`, the finish
scope README and both `.qnt` files — were digested before the run and re-digested
after and are **byte-identical either side**; the runner's own declared-gap count
is **0** both times. `models/run.sh --scope finish --family quint` is **exit 0,
261 commands, 63 of 63 cells, 0 declared gaps, 0 empty, Q4 matrix 10 of 10, 5 m
47 s wall / 413 s user** (`2026-08-28T06:17:02Z` → `06:22:48Z`), against the
baseline's 254 commands and 5 m 25 s. Seven more commands, all in
`scenario_in_place_march`. The README row was written **after** the run.

**And the row nearly documented a claim with a count of itself.** Its provenance
sentence first read *GAP-line count 5 both times*, which a `grep -c GAP` then
made 6 — because the sentence contains the word. It now cites the runner's own
`-- cells:` line instead, and says why. `references/execute.md` names this
exactly: *never document a claim with a count of itself; state the structural
fact*.

**The sharpest single line this leaf produced, and it is a contrast rather than a
result.** Under `relax_EN_03` — the **counterfactual** candidate, one atomic
`settle` with no intermediate disk — `inv_FN_24a_one_stable_state_under_the_candidate`
still **holds** after the restatement. Under the **available** candidate the same
claim is red. The admissible-but-unavailable protocol passes and the available one
does not, which is the ADR's headline rule arriving as a measurement instead of an
argument. **It is still not a verdict**, for the reason above: `FN-32` is blind to
this candidate, so no run has yet checked it against a complete retained set.

**The ADR set was reconciled, in place, and three of its sentences were
falsified rather than extended.**
[`finish-keeps-a-cleanup-layer-it-has-not-proved-forced`](../../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
kept its `defer` verdicts untouched — those are `q1-q4-verdict-k83`'s — and had
corrected: *no command runs the candidate* (one does now, in Quint); *the
predicted failure is order-dependent … only for a candidate that retires the
witness first* (no available candidate **can** retire it first, and the leak is
after the witness rather than before it); and the commission's second half, which
asked for a §*States* member **with the manifest cascade both families pay** and
actually needed a claim's guard restated, at a cost of **zero** cells. Leaving a
half-executed commission standing for three more sessions is the failure mode the
node has already paid for twice.

**One defect this leaf introduced and fixed.** `grove-llm leaf-decompose` moved
the task body one directory deeper as the node's `BRIEF.md`, and every
depth-relative link in it went one `../` short — seven dangling links, silent, in
the file every remaining child reads first. The verb's contract is right that
**position-free headers** need no rewrite; a relative link is not position-free.
Repointed here; worth knowing before the next decompose.

**`docs/formalism-findings.md` gets one forward annotation and no entry, which is
the frozen record's own rule applied rather than bent.**
`experiment-synthesis-k62` froze Experiment 2 against the two columns **as
independently built**, so appending an entry here would move M5's checked-claim
counts and M7's run costs after the measurement they belong to. What it does get
is the move `finish-verdicts-k78` already established: a `> **[annotated by …]**`
blockquote at entry 043, where *"`FN-24` was settled by the crash slice"* would
otherwise read as a clearance for a candidate protocol. The annotation adds no
command and moves no count, and Q4-5's `none` stands as the measurement that
slice made.

**And the durable set was registered rather than only written.** `CONTEXT-MAP.md`
requires every record under `docs/adr/` to name a maintaining context — *a record
added later joins this list* — so the new ADR is listed under the **grove**
context. Checked by enumeration rather than by pattern: every file in
`docs/adr/` is now named in the map, none unlisted.

**No glossary entry, deliberately.** *Work outstanding over the task root* is a
spec-local role name that the catalogue defines and the ADR cites; `CONTEXT.md`'s
own scope note is *definitions only … this glossary names those seams rather than
restating them*, and a third statement of a rule with two owners is what
`restatement-declares-its-class` forbids one register over.

**The one thing about this leaf that looks like `finish-verdicts-k78`'s defect
and is not, pre-empted because it will look like it again.**
`scenario_in_place_march` carries `ENV_BUDGET = 0` with `ENV_PHASES` and
`ENV_KINDS` empty — the same three `const`s `k78` had to repair in
`relax_EN_03`. The rule that leaf left is precise: *a control may narrow to its
own site; what `relax_EN_03` may not do is narrow the world an **invariant set**
is checked in.* This module asserts no retained set — six witnesses, which belong
at their own site, and one kill. And the narrowing makes the kill **strictly
stronger** rather than weaker, on an argument that is checkable: with no
environment action admitted the module's traces are a **subset** of `base`'s, so
a violation found here is a violation there. Checked rather than asserted — the
module differs from `base` in exactly **one** non-search `const`,
`IN_PLACE_DISPOSAL`, which is the property `k78` had to establish for
`relax_EN_03` by hand.

**The run reproduces, and the confirmation is command-by-command rather than a
matching total.** A second `models/run.sh --scope finish --family quint` after
the remaining prose edits is **exit 0, 261 commands, 63 of 63 cells, 5 m 45 s**,
and diffing the two runs' `PASS`/`FAIL` lines by command name gives **no
difference at all** — which is a stronger statement than two equal counts, since
two runs can agree on a total while disagreeing on which command did what.

## The in-session reviewer, and the three things it found that the suite could not

The leaf's `Notes` said to spend the allowance on *this restatement has content*.
It was spent there, on one fresh context given the artifact and the contract with
the conclusion stripped, and it came back with eight findings. **Three were
substantive, and the suite was green through all three.** Every finding below was
re-measured here before being acted on; none was taken on the reviewer's word.

**1. The change falsified `FN-28`, and nothing reported it — the same defect at a
third claim.** `inv_FN_28`'s second operand read `(ATOMIC_DISPOSAL or
hist.appliedAfterQuarantine)`: **an enumeration of the two protocols that
happened to exist.** The available candidate satisfies `FN-28`'s role — *the task
root leaves its own name only on a proven result* — and broke the claim.
Reproduced: `--main=scenario_in_place_march --invariant=inv_FN_28_one_successful_exit`
→ violated. **Why the suite could not see it**: a `scenario_` module carries only
the commands written inside it, and this one declared `wit_FN_28_…` while never
checking `inv_FN_28` — **crediting `FN-28`'s coverage cell from a world in which
`FN-28` was false.** That is `models/run.sh`'s own obligation-3 hazard, one grain
down, produced by me. Repaired by stating the claim over two facts each
root-freeing step records for itself (`rootTakenAway`,
`rootTakenWithoutCommitted`) and asserting it inside the candidate's module.
Verified behaviour-preserving: green in `base` and in twelve `scenario_`/`relax_`
modules; `relax_EN_13`'s violation is **pre-existing** — it fires under the old
operand too, and that module is a premise break declaring only its two kills.
**Adding `or IN_PLACE_DISPOSAL` was available and would have been the wrong
repair**, which is what makes the rule worth an ADR rather than a fix.

**2. My stated justification for keeping `groveReservationStands` was measurably
false, in both halves, and four documents repeated it.** I wrote that dropping
the disjunct *loses `mutant_no_quarantined_state`'s kill, whose disk is `EN-11`'s
orphaned quarantine beside a live root with no transaction running*. Measured:
all three inherited `FN-24.a` kills fire with `unsettledRootWork` **alone**,
`base` stays green with it alone, and no reachable state in `base` has the first
disjunct true, the second false, and the classification in the biting set. The
disk I named is **not admitted in the module I attributed it to** —
`mutant_no_quarantined_state` runs `ENV_KINDS = Set(0)`, crash only. **I asserted
a measurement I never made**, which is the failure this node has a rule against.
The disjunct is kept, on the reason that survives — the class sentence has two
realisations and a guard narrower than the role errs the wrong way — and it is
now **declared an uncontrolled conjunct** wherever it is stated, on the precedent
this README already sets for the ticket's attempt-binding.

**3. `nextInPlaceDisposable` walked the wrong set, and the candidate could delete
un-evacuated user entries.** It read `leastOf(evacuated(w))` — entries `InWitness`
only — while both existing disposals are **total**. `SRevalBeforeQuarantine` is
reachable with evacuation incomplete (measured in `base`: crash after `SPublish`,
recover, a late ticket makes it `DCommitted`, divert forward), and under the
candidate that state fell straight through to `SRemoveRoot` and settled `OApplied`
with the root gone and a user entry still `AtRoot`. **It was latent only because
`scenario_in_place_march` runs `ENTRIES = 1` with no environment action** — the
narrowing I had just argued was safe was also what hid a bug from me. Repaired to
walk every entry not yet `Disposed`, which is what `nextDisposable` does.

**Two findings qualified claims rather than breaking them, and both are now
declared.** `unsettledRootWork` reads `t.running and t.persisted`, which is the
model's program state — so on a **post-crash** disk it is false and the guard
falls back to the artifact form, with `hist.crashMisclassified` (computed from
the pre-crash state) carrying the boundary; and no environment action presents a
half-disposed root to a *later* invocation. My text called the second realisation
*stated over the user's tree rather than over Grove's machinery*, which is true of
its `Place` half and not of the whole. And the catalogue's `FN-24.a` is
strengthened for **one family**, so both columns will report it green while
checking different claims and the runner's contested-cell line cannot see it —
disclosed in the catalogue now, not only in the README.

**Three were mechanical and are fixed**: a rename that missed three of five sites
(`wit_FN_24a_and_the_claim_as_encoded_accepts_it`, which does not exist, cited in
the three sentences that turn the repair from an argument into a measurement);
`wit_FN_18_a_half_disposed_root_…`, whose guard did not say what its name said and
could not at `ENTRIES = 1`, now stated over what disposal has actually removed and
renamed; and a comment justifying `SRemoveRootWitness` from the recursive removal
`EN-03` withholds, when the honest statement is that `SRemoveWitness` has the same
shape. **One was noise for this leaf**: `EFFECTFUL_STEPS` is referenced nowhere,
which is pre-existing and unchanged by the three new members.

**The reviewer's own controls are worth carrying.** It swept all 63 library
`inv_` commands against the candidate's module and found exactly two violated —
the intended kill and `FN-28` — which is how the second one surfaced at all. That
sweep is a cheap general obligation for any leaf that adds a protocol: **a module
that changes what the model does must be run against every claim the model has,
not only the ones the module declares.**

**A 212-link resolve over every file this leaf touched finds exactly one broken
target, and it is the one already owned elsewhere.** `docs/formalism-findings.md`'s
`](../adr/bulk-marks-are-not-atomic.md)`, one `../` too many — named by
`finish-verdicts-k78` and routed to `handoff-audit-k66`. **Not fixed here**: it is
a one-character change, and taking it would remove a checkable item from another
leaf's list without that leaf knowing. What this leaf contributes is a fresh
measurement that it is still the *only* one, over a link set that now includes
every file this subtree wrote.

**The run after the review integration, frozen the same way.** `models/run.sh
--scope finish --family quint` is **exit 0, 262 commands, 63 of 63 cells, 0
declared gaps, 0 empty, Q4 matrix 10 of 10, 5 m 49 s wall / 416 s user**
(`2026-08-28T06:52:00Z` → `06:57:49Z`), with all four run subjects digested before
and after and **byte-identical either side**. One command more than the pre-review
run: `inv_FN_28_still_holds_under_the_available_candidate` — which exists because
the pre-review run was green **while `FN-28` was false under the candidate**. The
baseline chain for this leaf is therefore three figures, not two: 254 commands on
the tree as `finish-verdicts-k78` left it, 261 with the apparatus, 262 with the
claim the apparatus turned out to have broken.
