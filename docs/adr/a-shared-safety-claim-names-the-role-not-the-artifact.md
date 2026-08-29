# A shared-safety claim names the role, not the incumbent's artifact

A claim classed **shared safety** exists to judge a *candidate* protocol, so
**every operand that decides whether it bites or is satisfied** — the antecedent
that guards it and the consequent that discharges it alike — SHALL name the
**role** an artifact or a transition plays, and never the incumbent's artifact
that plays it, nor an enumeration of the protocols that happen to exist. Both
forms are satisfied by construction under a candidate that differs exactly there,
and so report green over the one difference the claim was retained to judge.

## The trade-off this settles

**Vacuity is invisible from inside a green suite, and it reached three of the
four claims in one retained set.** Q1's retained shared-safety claims are
`FN-20`, `FN-24`, `FN-27` and `FN-32`. Three could not see the candidate they
were retained to judge, and the three sit at three different places in a claim's
expression — which is what makes this a rule about expression rather than three
incidents:

- **`FN-32` — the reached antecedent.** Its transaction-side sites are
  `SCreatePreparing`, `SQuarantineRename` and `SCreateMarker`; a candidate with
  neither quarantine nor marker reaches only the first, which it inherits from
  the incumbent unchanged. Both the antecedent and the kill landed at a step the
  candidate does not change.
- **`FN-24.a` — the guard.** Its two *failable* conjuncts were guarded on the
  incumbent's own artifacts in **both independently built families** —
  `groveReservationStands(w) = w.witness != WNone or w.quar.present` in
  `finish.qnt`, and `classified = SAbsent implies (no Slot.occ and no
  Quar.qRid)` with the same consequent over `currentStates` in `finish.als`. A
  protocol holding neither makes both vacuous.
- **`FN-28` — the consequent.** Its second operand read `(ATOMIC_DISPOSAL or
  hist.appliedAfterQuarantine)`: an **enumeration of the two protocols that
  happened to exist**, the incumbent's quarantine handoff and `EN-03`'s
  counterfactual. A candidate satisfying the role — *the task root leaves its own
  name only on a proven result* — falsified a shared-safety claim, and the suite
  reported green. This is the operand that shows adding a disjunct is **not** the
  repair: a third mechanism named is a fourth one missed.

**Two families reaching the same encoding independently is what makes this the
contract's defect and not a modeller's.** It is the same evidence-shape
[`a-lifecycle-claim-says-what-it-is-over`](a-lifecycle-claim-says-what-it-is-over.md)
rests on, one register across: there, a word with no definition; here, an operand
the catalogue never said what to state over.

## The role-form, and what it costs

*Grove has work outstanding over the task root* has **two** realisations, and
`FN-24.a`'s failable conjuncts are guarded on their disjunction: an artifact of
Grove's standing at a name it reserves — the incumbent's witness or quarantine;
and an entry of the tree a live transaction has moved or removed and not settled
— which any protocol leaves, including one that reserves no name at all. The
second is stated over `Place`, which belongs to the **user's** tree rather than
to Grove's machinery, and it is what gives the claim content over a candidate. It
is guarded on the entry having moved rather than on the transaction having
written, because the incumbent's own restoration branch legitimately reads as
`Current(*)` once every entry is back and the witness is released.

*The task root leaves its own name only on a proven result* is stated the same
way: each step that frees the name records that it did and that the result was
proven when it did, so the claim is over Grove's own steps rather than over which
protocol is running — and an **attempted** removal is not one, because a torn
rename leaves the root answering at its own name.

**One narrowing rides with the guard and is declared rather than discovered
later.** The moved-tree realisation also reads *a live transaction*, which is the
model's program state and not the disk — and a crash resets it. So on a
**post-crash** disk that disjunct is false and the guard collapses back to the
artifact form; what carries the crash boundary is a flag the crash records from
the pre-crash state. Nothing in the Quint column presents a half-disposed root to
a *later* invocation, so the strengthened claim is checkable inside the trace
that creates the state and not from outside it. The disk-only half cannot stand
alone either, because *disposed* is terminal and would be true forever after any
successful finish. Closing that gap needs an environment action that produces the
disk — owed, and named where it is owed rather than papered over here.

## The alternatives, each rejected on a stated cost

| | what it does | why rejected |
|---|---|---|
| **name the new mechanism too** — add a disjunct per protocol | repairs the instance | this is the form, not an instance of it. `FN-28`'s operand already enumerated two and a third was available; the repair that keeps working is the one that stops enumerating. It also hides: a green after such a repair is a green over the protocols someone thought of |
| **replace the artifact realisation outright** | states the guard only over the moved-tree fact | rejected: §*States*' `Reserved` class sentence — *an artifact at a name Grove reserves says Grove has work outstanding at that name* — has two realisations, and a model guard narrower than the contract's role errs the wrong way. It is not retained on faith: `mutant_orphan_is_not_a_reserved_state` reaches, in 90% of its traces, a disk that only the artifact realisation catches |
| **narrow §*States*' `Absent` row instead** — *no task root, and nothing of Grove's at a reserved name either* | makes the failing disk unclassifiable rather than misclassified | rejected once already by `finish-scope-k71`, for a reason that binds here too: it satisfies the conjunct **by construction** and makes the departure invisible to the check that exists to catch it. Declining to carry the artifact in the state vector is the same defect wearing a different hat |
| **add a §*States* member for the candidate's partially disposed root** | gives the disk a row of its own | a member is admitted by the `Reserved` class sentence, and the candidate under measurement puts **no** artifact at any reserved name. A row whose condition tests nothing is not a member. A candidate that *does* place such a document is admissible, and has kept the cleanup layer's document under another name |
| **leave it and read the green** | nothing | this is what a criterion met over a blind claim already produced once, at `relax_EN_03`. A retained set cannot classify a difference one of its members is blind to, and reading such a green is how a `delete/replace` gets returned for a protocol nobody checked |

## What checks it

- `finish.qnt` — `groveWorkOutstanding`, which `classifiesHonestly`'s two
  failable conjuncts are stated over; `inv_FN_28_one_successful_exit`, stated
  over the two facts each root-freeing step records for itself;
  `IN_PLACE_DISPOSAL`, the strategy dial that grants no capability.
- `finish-controls.qnt` — `scenario_in_place_march`, whose
  `inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree` is the
  kill and whose `wit_FN_24a_the_artifact_guarded_encoding_accepts_it` is the
  same disk under the pre-repair encoding, frozen inline: an A/B on one world.
  `mutant_orphan_is_not_a_reserved_state` is the artifact realisation's own
  witness and kill. `FN-28`'s two operands have one isolating kill each —
  `relax_EN_01`'s `inv_fail_EN_01_FN_28_a_torn_rename_is_not_a_removal` and
  `mutant_status_classifier`'s
  `inv_fail_MUT_FN_28_the_root_is_taken_away_before_the_result_is_proven` — each
  measured green on the *other* operand over its own module, which is what makes
  them isolating rather than merely red.

**A green with no available kill is not a pass, and that is the rule's second
half.** A role-form operand is a fact the model's own steps record, so it is one
edit away from being satisfied by construction; every one of them therefore owes
a control that makes a step omit or falsify it.

## What would reopen this

A shared-safety claim whose role genuinely has one realisation, where naming it
is naming the role. `FN-13`'s *no candidate committed tree contains the reserved
witness* is the near case, and it is not an instance: the witness there is the
thing being excluded rather than the operand deciding whether the claim applies.

The Alloy column has not been restated and owes the same repair for `FN-24.a` and
`FN-28`; it is `alloy-candidate-k82`'s. Until it lands, both families report those
two green while checking **different claims**, and the runner's contested-cell
line cannot see it — it fires only when one family *declares a gap*. The
catalogue says so where it states the obligations.

**The catalogue and both model families have been retired.** The catalogue was
`docs/specs/semantic-contract.md`; the two families were the Quint and Alloy
columns under `models/` and the per-scope model directories. All of it was
deleted with the campaign's apparatus — `delete-formal-models-k29`, and
`delete-finish-models-k30` for the finish column, which took every file
§*What checks it* names. So the repair that section leaves outstanding for
`FN-24.a` and `FN-28` is not work anyone will do, and `alloy-candidate-k82` will
not be chartered: the divergence it describes is a finding about how two
independently built columns read one underdetermined catalogue, and that finding
is what survives. The decision survived the instrument that found it, which is
the outcome that campaign was run to test, and `docs/formalism-findings.md` keeps
the record of how it was found.
