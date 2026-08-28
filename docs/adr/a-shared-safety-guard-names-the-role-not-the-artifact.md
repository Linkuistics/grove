# A shared-safety guard names the role, not the incumbent's artifact

A claim classed **shared safety** exists to judge a *candidate* protocol, so its
guard — the antecedent that decides whether the claim has anything to say about a
disk — SHALL name the **role** an artifact plays and never the incumbent's
artifact that plays it. A guard naming the artifact is satisfied vacuously by any
candidate that does not have it, and reports green over exactly the difference it
was retained to judge.

The semantic contract already applies this to what a shared-safety claim *says*:
`FN-20` is deliberately *"stated over the role rather than over the quarantine,
so Q1 can be decided against it"*, and `FN-28`'s conjunct naming `QuarRename`
carries a role-form beside it. This record extends the same discipline to what a
shared-safety claim is *guarded by*, which is where two claims had it and nobody
had noticed.

## The trade-off this settles

**Vacuity is invisible from inside a green suite, and it happened twice in the
same retained set.** Q1's retained shared-safety claims are `FN-20`, `FN-24`,
`FN-27` and `FN-32`. Two of the four could not see the candidate they were
retained to judge:

- **`FN-32`**, found by `finish-verdicts-k78`. Its transaction-side sites are
  `SCreatePreparing`, `SQuarantineRename` and `SCreateMarker`; a candidate with
  neither quarantine nor marker reaches only the first, which it inherits from
  the incumbent unchanged. Both the reached antecedent and the kill landed at a
  step the candidate does not change.
- **`FN-24.a`**, found by `honest-classification-k80`, and this is what makes the
  first one a pattern rather than an incident. Its two *failable* conjuncts were
  guarded on the incumbent's own artifacts in **both independently built
  families** — `groveReservationStands(w) = w.witness != WNone or w.quar.present`
  in [`finish.qnt`](../../crates/grove-finish/models/finish.qnt), and
  `classified = SAbsent implies (no Slot.occ and no Quar.qRid)` with the same
  consequent over `currentStates` in
  [`finish.als`](../../crates/grove-finish/models/finish.als). A protocol holding
  neither makes both vacuous.
- **`FN-28`**, found by that leaf's own in-session reviewer **after** the repair
  above had landed, and it is the finding that settles the shape as a rule.
  `inv_FN_28`'s second operand read `(ATOMIC_DISPOSAL or
  hist.appliedAfterQuarantine)` — an **enumeration of the two protocols that
  happened to exist**, the incumbent's quarantine handoff and `EN-03`'s
  counterfactual. The available candidate satisfies the claim's role (*the task
  root leaves its own name only on a proven result*) and **falsified the claim**,
  and nothing reported it: a `scenario_` module carries only the commands written
  inside it, and the candidate's module declared `FN-28`'s **witness** while never
  checking its property — crediting a coverage cell from a world in which the
  claim was false. **A guard that enumerates mechanisms is the same defect as one
  that names an artifact**, and the third instance is what shows that adding a
  disjunct is not the repair; stating the role is.

**Two families reaching the same encoding independently is what makes this the
contract's defect and not a modeller's.** It is the same evidence-shape
[`a-lifecycle-claim-says-what-it-is-over`](a-lifecycle-claim-says-what-it-is-over.md)
rests on, one register across: there, a word with no definition; here, a guard
the catalogue never said what to state over.

**The measurement, rather than the argument.** `finish.qnt` now carries the
available candidate — stepwise in-place disposal, granted no capability
`EN-03` withholds — and
`scenario_in_place_march`'s `wit_FN_24a_the_artifact_guarded_encoding_accepts_it`
lands: the candidate exposes a present task root with no witness, no quarantine
and its disposal outstanding, `classify` returns a `Current(*)` row, and
`FN-24.a` **as it was encoded** accepts it. Restated over the role, the same
world kills it
(`inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree`) — with no
model mutation and no interruption anywhere in the trace.

## The role-form, and why it keeps both disjuncts

*Grove has work outstanding over the task root* has **two** realisations, and the
claim is guarded on their disjunction:

- an artifact of Grove's standing at a name it reserves — the incumbent's
  witness or quarantine; and
- an entry of the tree a live transaction has moved or removed and not settled —
  which any protocol leaves, including one that reserves no name at all.

The second is stated over `Place`, which belongs to the **user's** tree rather
than to Grove's machinery, and it is what gives the claim content over a
candidate. It is guarded on the entry having moved rather than on the
transaction having written, because the incumbent's own restoration branch
legitimately reads as `Current(*)` once every entry is back and the witness is
released.

**One narrowing rides with it and is declared rather than discovered later.** The
second realisation also reads *a live transaction*, which is the model's program
state and not the disk — and a crash resets it. So on a **post-crash** disk the
disjunct is false and the guard collapses back to the artifact form; what carries
the crash boundary is a flag the crash records from the pre-crash state. Nothing
in the Quint column presents a half-disposed root to a *later* invocation, so the
strengthened claim is checkable inside the trace that creates the state and not
from outside it. The disk-only half cannot stand alone either, because *disposed*
is terminal and would be true forever after any successful finish. Closing that
gap needs an environment action that produces the disk — owed, and named where it
is owed rather than papered over here.

## The alternatives, each rejected on a stated cost

| | what it does | why rejected |
|---|---|---|
| **replace the artifact disjunct outright** | states the guard only over the moved-tree fact | kept, but **not** for the reason first recorded here. This row claimed dropping it would lose `mutant_no_quarantined_state`'s kill; `honest-classification-k80`'s reviewer measured that false — all three inherited `FN-24.a` kills fire with the moved-tree disjunct alone, `base` stays green, and no reachable state has the first disjunct true, the second false, and the classification in the biting set. It is kept because §*States*' `Reserved` class sentence has two realisations and a model guard narrower than the contract's role is the wrong direction to err — **and it is an uncontrolled conjunct, declared as such** rather than defended |
| **narrow §*States*' `Absent` row instead** — *no task root, and nothing of Grove's at a reserved name either* | makes the failing disk unclassifiable rather than misclassified | rejected once already by `finish-scope-k71`, for a reason that binds here too: it satisfies the conjunct **by construction** and makes the departure invisible to the check that exists to catch it. Declining to carry the artifact in the state vector is the same defect wearing a different hat |
| **add a §*States* member for the candidate's partially disposed root** | gives the disk a row of its own | a member is admitted by the `Reserved` class sentence — *an artifact at a name Grove reserves says Grove has work outstanding at that name* — and the candidate under measurement puts **no** artifact at any reserved name. A row whose condition tests nothing is not a member. A candidate that *does* place such a document is admissible, and has kept the cleanup layer's document under another name |
| **leave it and read the green** | nothing | this is what a criterion met over a blind claim already produced once, at `relax_EN_03`. A retained set cannot classify a difference one of its members is blind to, and reading such a green is how a `delete/replace` gets returned for a protocol nobody checked |

## What checks it

- [`finish.qnt`](../../crates/grove-finish/models/finish.qnt) —
  `groveWorkOutstanding`, which `classifiesHonestly`'s two failable conjuncts are
  stated over; `IN_PLACE_DISPOSAL`, the strategy dial that grants no capability.
- [`finish-controls.qnt`](../../crates/grove-finish/models/finish-controls.qnt) —
  `scenario_in_place_march`, whose
  `inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree` is the
  kill, whose `inv_FN_28_still_holds_under_the_available_candidate` is the
  repaired third instance asserted where it can fail, and whose `wit_` commands
  are the measurement. The three inherited
  `FN-24.a` kills — `mutant_no_quarantined_state`,
  `mutant_absent_classified_first` and `relax_EN_01` — are asserted green beside
  it, which is what says the restatement strengthened the claim rather than
  moving it.

## What would reopen this

A shared-safety claim whose role genuinely has one realisation, where naming it
is naming the role. `FN-13`'s *no candidate committed tree contains the reserved
witness* is the near case, and it is not an instance: the witness there is the
thing being excluded rather than the guard deciding whether the claim applies.

The Alloy column has not been restated and owes the same repair; it is
`alloy-candidate-k82`'s. Until it lands, both families will report `FN-24.a` and
`FN-28` green while checking **different claims**, and the runner's contested-cell
line cannot see it — it fires only when one family *declares a gap*. The
catalogue says so where it states the obligation.
