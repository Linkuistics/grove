# finish-verdicts-k78

**Integrates:** finish-verdicts-k77

## Goal

Integrate the two findings from `finish-verdicts-k77`: Q1's `keep` verdict is
not established by the candidate that was run, and Q4's three cleanup-layer
`keep`s contradict the catalogue's still-binding classifier. Reconcile the
model, catalogue, README and ADR from one evidence set; do not preserve a
verdict by changing only its explanation.

## Context

### R1 — high: Q1's retained ownership check is vacuous, and “no control can” is a model coupling

The new retained-claim command does not exercise the claim it names.
`relax_EN_03` sets `ENV_BUDGET = 0` and both `ENV_PHASES` and `ENV_KINDS` empty
([`finish-controls.qnt`](../../../crates/grove-finish/models/finish-controls.qnt):824-865).
`envAdmitted` requires all three gates, so no foreign artifact can be installed
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):2220-2225).
The asserted `FN-32` invariant is only the absence of two mutation flags
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):3226-3234),
and this module carries no companion witness that reaches either unprovable
artifact. Its one reachability command reaches `SDisposeInPlace`, not the
ownership antecedent
([`finish-controls.qnt`](../../../crates/grove-finish/models/finish-controls.qnt):864-871).
The reported green therefore does not close the retained-set gap.

The reason given for declining the missing candidate is likewise about the
model, not the protocol. The state machine selects `SDisposeInPlace` exactly
when `ATOMIC_DISPOSAL` is true and `SQuarantineRename` otherwise
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):1706-1708);
the in-place action is itself defined as one atomic recursive deletion
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):1862-1869).
That encoding cannot express `ATOMIC_DISPOSAL = false` together with a
no-quarantine, resumable disposal strategy, but a separate strategy/model dial
can. The README's inference that no control can exist because one `const` ties
the two together is therefore circular
([`README.md`](../../../crates/grove-finish/models/README.md):4031-4044).
Whether that candidate violates `FN-24` is the missing measurement, not a result
already produced by the current parameterisation.

This matters to the verdict rather than only the suite. The catalogue still says
Q1 is classified by the candidate retaining every shared-safety claim and
reaching the `FN-24` witnesses
([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):296-307),
while the producer records that those witnesses were not run. Declaring that
classifier mis-typed invalidates it; it does not invert missing evidence into a
`keep`. Unless a replacement criterion and its evidence are landed coherently,
Q1 is `defer`, with the non-atomic no-quarantine control and the reachable
`FN-32` scenario as the commission.

### R2 — high: Q4's three `keep`s contradict the catalogue and consume the same missing control

The catalogue says a `none` row in both families is Q4 evidence for
`delete/replace`
([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):344-355).
The Quint matrix records exactly that result for the quarantine, cleanup marker
and replace transition, but also says the three are one bundled control and
explicitly asks `finish-verdicts-k65` to commission artifact-specific removals
if the decision needs separation
([`README.md`](../../../crates/grove-finish/models/README.md):3962-4002).
The later `keep` text declines the commission solely because the model couples
the mechanism to `ATOMIC_DISPOSAL`; R1 shows that is not a protocol result.

Q3 does not close this gap. Its witnesses prove the replace transition is
reachable in the incumbent protocol, which confirms Q3 on its own evidence, but
they do not prove that the whole cleanup mechanism is irreplaceable. Until the
matrix rule is deliberately revised and re-measured, or the missing candidate
controls are run, the cleanup trio's Q4 disposition is `defer`, not `keep`.
The current catalogue is internally inconsistent where its classifier says
`delete/replace` and the paragraph immediately above it says all four are
`keep` ([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):302-330).

### Claims that survived the review

- Q2 `keep` is independently supported by reached `Indeterminate` witnesses on
  Git, native jj and colocated jj in both families; it does not need the
  counterfactual-capability reinterpretation.
- Q3 `keep` is supported within the incumbent by the reached replacement source
  state and by the trace that produces the stale marker rather than positing it.
- `FN-13` remains shared safety. Total removal satisfying it vacuously means it
  cannot block that candidate; it does not make the class assignment false.
- `EN-03` and `EN-05` remain sound environment constraints for the targeted
  Unix/VCS implementation. The review found no available atomic recursive
  deletion or filesystem/VCS transaction construction.
- Root creation remains Grove-owned on depth: ignoring the quoted line counts,
  moving it would expose three new library concepts for one consumer while the
  Grove-specific format classification stays outside the seam. The ADR's
  second-consumer reopener is the right test.

## Done when

- The model can express a no-quarantine strategy independently of
  `ATOMIC_DISPOSAL`, and the intended non-atomic candidate is either run against
  the retained set or rejected by a runnable control rather than by the current
  branch shape.
- The Q1 candidate reaches an unprovable witness/marker case while asserting
  `FN-32`, and carries the `FN-24` witness coverage its classifier requires.
- Q1 and the three Q4 cleanup rows are reclassified from the resulting evidence:
  `keep` only if the replacement criterion positively establishes it, otherwise
  `defer` with the exact remaining commission. Q2 and Q3 remain `keep` unless
  that new candidate makes Q3 moot.
- `docs/specs/semantic-contract.md`, the finish model README, the finish-layer
  ADR, the formal-synthesis brief and implementation hand-off say one coherent
  thing. An ADR whose title says every layer is forced must not survive if Q1 or
  Q4 remains deferred.
- The finish-family model commands and coverage checks are rerun after the
  integration changes; the whole-repository run remains `handoff-audit-k66`'s.

## Notes

This integration task owns fixes and post-fix verification. The review ran no
model, test, build, lint or format command.

## Decisions (running log)

**R1 is confirmed, and by two independent routes rather than the one it names.**
`relax_EN_03` sets `ENV_BUDGET = 0` with `ENV_PHASES`/`ENV_KINDS` empty, and
`envAdmitted` is `envUsed < ENV_BUDGET and ...`
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):2224), so
`foreignWriteAt` — whose guard is `envAdmitted(2)` — can never fire: no foreign
artifact exists at any name in that module. That is the route the review names.
The second is stronger and it makes the first redundant: `relax_EN_03` sets
`OWNERSHIP_PROVEN = true`, and every one of `noteUnproven{,Slot,Marker}` is
guarded on `not(OWNERSHIP_PROVEN)`
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):1470-1490), so
`mutatedUnprovenSlot` and `mutatedUnprovenMarker` — the whole content of
`inv_FN_32` — are unsettable in **every** module that leaves the dial true.
`inv_FN_32` is therefore non-vacuous only in the two modules that turn it off,
`mutant_unproven_ownership` and `mutant_unproven_marker`, and **both of those run
`ATOMIC_DISPOSAL = false`**: the candidate has no `FN-32` control at all.

**R1's model-coupling half is confirmed as read.** `ATOMIC_DISPOSAL` is one
`const` on one arm — `go(f1, if (ATOMIC_DISPOSAL) SDisposeInPlace else
SQuarantineRename)` ([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):1707)
— and `SDisposeInPlace` is a single `settle` that removes the root, the witness,
the manifest and every entry in one step (1862-1869). A no-quarantine strategy
that is *not* atomic is unexpressible in this encoding, so "no control can exist"
is a fact about the parameterisation. It is not a fact about the protocol, and
the README states it as one.

**R2 is confirmed, and the producer README states the criterion MET in as many
words.** Q4-105/106/107 read `none` in Quint and Q4-5/6/7 read `none` in Alloy,
which is the catalogue's pre-registered `delete/replace` evidence
([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):352-355). The
Alloy scope README does not merely record the rows; it says *"That is Q4's
delete/replace criterion met for the first time in either family"* and, for the
marker and the replace transition, *"met a second and third time"*
([`README.md`](../../../crates/grove-finish/models/README.md):3205-3266).

**And one load-bearing sentence of the ADR is false on the artifact.** It says
*"Alloy's `none` rows are `argument` rows"*
(the record as `k65` left it, now
[`finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md),
§Q4). Only Q4-5 is `argument`. Q4-6 is `mutation — row x1` and Q4-7 is
`mutation — row 45`, and both are **artifact-specific removals run in the
available world**, with no counterfactual capability anywhere near them. So the
claim that no control *can* separate the three is not merely a statement about
Quint's parameterisation — it is contradicted by two controls the neighbouring
family already ran.

**The instrument is repaired, and Q1's pre-registered criterion is COMPLETED
rather than abandoned — which is what turns `k65`'s argument into a
measurement.** Four changes, all Quint, none touching the manifest:

- `relax_EN_03` now differs from `base` in **exactly one `const`**. It carried
  `ENV_BUDGET = 0` with empty `ENV_PHASES`/`ENV_KINDS`, which narrows the
  *world* rather than the candidate; the criterion says the candidate is checked
  "at the bounds the incumbent reached them at", and the incumbent's bound is
  `base`'s environment.
- `wit_FN_32_the_candidate_meets_an_unprovable_artifact` — the antecedent, now
  reached (**317 traces**), so the retained `FN-32` is asserted over a world in
  which it has something to be about.
- `mutant_unproven_ownership_under_the_candidate` — the candidate with
  `OWNERSHIP_PROVEN` off. It is the first `FN-32` kill anywhere in the suite that
  runs under `ATOMIC_DISPOSAL = true`, and it **fires**. The claim is falsifiable
  under the candidate, which it demonstrably was not.
- `FN-24`'s witness coverage, which the README named as the outstanding
  commission and declined to run: ten `wit_FN_24a_crash_at_<step>_under_the_candidate`
  over the candidate's own effectful step list, all reached; and `FN-24.b`'s two
  branch enumerations in `scenario_march_under_the_candidate`, the candidate's
  counterpart of the uninterrupted module the incumbent's own pair lives in
  (**1660** and **3649** traces). They do not live in `relax_EN_03` for the
  reason the incumbent's do not live in `base`: a branch enumeration needs a
  trace that walks a whole branch, which a crash-at-every-point world does not
  produce.

**And this is the result that decides Q1, in the direction nobody wanted.** The
criterion is now **met in full** — and met, it returns **`delete/replace`** for a
protocol that needs an atomic recursive directory deletion `EN-03` says does not
exist. That is the demonstration that the criterion is admissibility-typed:
running it to completion licenses removing a mechanism in favour of a protocol
that cannot be built. `k65` asserted this; it is now measured.

**A mis-typed criterion yields no verdict in either direction, so Q1 is
`defer`.** `keep` was `k65`'s inference from the criterion's defect, and missing
evidence has no sign. Under an availability-typed replacement — *every candidate
requiring no absent capability either retains the shared-safety set (⇒ delete/
replace) or is shown to break one (⇒ keep)* — the only available no-quarantine
strategy is non-atomic in-place disposal, and no command in either family runs
it. Quint cannot express it; Alloy runs no counterfactual-capability mutation at
all.

**Q4's three cleanup rows are `defer` for a reason that is not a re-reading of
them.** The rows are real, the criterion is met, and the Alloy column says so.
What stops them being a licence is the coverage hole that column itself
records — *no shared-safety obligation in this repository is stated over the
quarantine reaper's actions* — with a Quint face: `OWNERSHIP_PROVEN` is a free
`const` rather than something the marker's presence derives, so no control can
make removing the marker cost a proof of ownership. A `none` returned by a set
that declines to look at the sweep is silent, not permissive. Neither verdict is
established.

**Q2 and Q3 stay `keep`**, on the review's own finding: both rest on witnesses
reached under the incumbent and neither needs the counterfactual reading.

## The in-session reviewer, and every finding classified

The leaf's one narrow reviewer was spent on the finished record, against the
model artifacts, with an adversarial *find what is wrong* prompt. It confirmed
the direction — Q1 and Q4's three cleanup rows are not `keep` — and found six
things wrong with the reasons. All six are verified against the artifacts.

**Valid and actionable, and the sharpest of them.** *`inv_FN_32` under the
candidate is still shape rather than content.* The pure tautology is gone — the
antecedent is reached and the kill fires — but both land at `SCreatePreparing`,
the witness slot, which the candidate inherits from the incumbent unchanged.
`stopReserved`'s other sites are `SQuarantineRename` and `SCreateMarker`, both
unreachable under `ATOMIC_DISPOSAL = true`. So `FN-32` has **no content over
anything the candidate changes**, and it cannot be given any: the candidate
removes every artifact the claim's other sites are about. Verified — and it does
not weaken the verdict, it sharpens it. The criterion is met **as written** and
met while one of its four retained claims is trivial over the difference it is
meant to judge. That is a second, independent defect in the same criterion, and
it is a better demonstration than the one this session first wrote down.
Recorded as such; "this is now a measurement rather than an argument" is
withdrawn.

**Valid and actionable.** *"A mis-typed criterion yields no verdict" needs a
suppressed premise — only measurements decide — that this repository's own
practice contradicts.* Five of Quint's ten Q4 rows are `argument` rows read as
evidence. The record cannot count an argument when it points at `delete` and
discount one when it points at `keep`. The reason Q1's `EN-03` + `EN-08` +
§*States* argument does not entail `keep` is concrete rather than
methodological, and the reviewer supplied it: it silently fixes **the state
table** — §*States* gained `Reserved(Quarantined)` mid-experiment, so a candidate
proposing `Reserved(Disposing)` is not refuted by *our table has no row* — and it
silently fixes **the removal order** — `classifiesHonestly` guards both failable
conjuncts on `groveReservationStands`, so a disposal that retires the published
witness *last* keeps that true throughout and never leaks to `Current(*)`.

**Valid and actionable, and it would have cost the commission its answer.** *The
commission as first written returns green for structural reasons.* With no
quarantine and no in-tree witness, `groveReservationStands` is false, so
`classifiesHonestly`'s two failable conjuncts are vacuous and `inv_FN_32` is
trivial — the commissioned control would have manufactured exactly the false
green this leaf exists to stop. The commission now requires a §*States* member
for the partially disposed root and a reachable `FN-32` site, in the ADR and in
`quarantine-necessity-k79`'s body.

**Valid and actionable.** *"The model cannot express it" is the same species of
overstatement the record condemns.* `Place = AtRoot | InWitness | Disposed` is
already per-entry and `SDisposeEntry` already removes entry at a time; what is
missing is a **transition**, not expressivity. "No command runs it" is the true
sentence and it was already one line away.

**Valid and actionable, and it re-founds Q4's `defer` on better evidence.**
*Quint's three `none`s are one bundled result from row 902, which is
`relax_EN_03` — the counterfactual-capability module.* By this record's own
headline rule those cells measure admissibility and say nothing about the shipped
world, so the Quint column supplies **zero** qualifying per-row cells rather than
three, and "reads `none` in both families" is false for the Quint half. The
record applied its new rule to Q1 and forgot to apply it to the Q4 rows produced
by the same module.

**Valid and actionable, against an inherited sentence rather than a new one.**
*"No shared-safety obligation in this repository is stated over the quarantine
reaper's actions" is false.* `fun groveActs: set Action { txnActs + Decline +
Discard + Reap }` ([`finish.als`](../../../crates/grove-finish/models/finish.als):5224),
and `FN-27` — one of Q4's own retained shared-safety claims — is quantified over
`groveActs` three times (5501-5503, 5518-5520, 5529-5531); row x1 left it green.
Only `FN-32` excludes `Reap`, deliberately. The true statement is the narrow one:
no shared-safety obligation constrains the reaper's **ownership proof**. The
over-broad sentence is `finish-scope-k71`'s, in the model README, and is
corrected there as well as here. It also does not reach `Q4-7` at all — the
replace transition is a `txnAct` inside `groveActs - Reap` that `FN-32` does
examine — so `Q4-7` gets the reason that actually holds: `MarkerReplace` is *the
only `groveActs - Reap` member whose marker mutation is gated on ownership*
([`finish.als`](../../../crates/grove-finish/models/finish.als):5930), so row 45's
green is a vacuity artifact of its own mutation.

**A visible trade-off, accepted.** *Alloy's `Q4-3` (the ready mark) names
`FN-10.a`, which is incumbent mechanics and which that row itself calls "not yet
a Q4 answer".* So "six rows protect the user" is a reading across both columns
rather than a per-family fact. The ADR says so now; splitting the count per
family would say less, not more.

**Noise for want of context.** *`10,366 / 31,423` is 33.0%, not the 34% the cost
table says.* The figure is inherited from `docs/preservation-baseline.md`, whose
own table lists test modules under `src/`; whether the denominator is all of
`src/` or `src/` net of tests is that file's to say, and 34% is right against the
smaller one. Changing an inherited figure on a denominator this session has not
established would introduce an error rather than remove one.

**No re-review is cut, and the reason is the direction of every fix.** Each
change above **removes** a claim or narrows one; the only new inferences are the
reviewer's own, which arrived from a fresh context. The work that *adds* claims
is `quarantine-necessity-k79`'s, and its body already instructs it to cut a
`review-design` beside itself.
