# honest-classification-k84

**Reviews:** honest-classification-k80

## Goal

Attack what `honest-classification-k80` landed **after** its own reviewer had
already moved it. Three substantive repairs went in without a second adversarial
read, and one of them restated a shared-safety claim and added two history
fields to the library that every module in the finish scope now carries.

## Context

**Why this leaf exists rather than another in-session pass.** `k80` spent its one
allowance, and the four-step pass returned three substantive findings — not
trivia, not a visible trade-off, not something an executable seam covers. A
second need after a substantive non-mechanical fix is the mechanical signal that
review has become tree-sized work, and this is the third time this node has taken
it (`obligation-placement-k67`, `finish-verdicts-k77`). It is **inserted** at
`sweep-ownership-k81`'s slot rather than appended, for the reason this node has
also used three times: `k81` is chartered to edit the very artifacts under review,
and a gate checked after everything it gates is not a gate. Keys are unchanged —
`k81`, `k82`, `k83` are now positions 03, 04 and 05.

**The specific doubt, and it is not a general request for a second opinion.**
`k80`'s own reviewer found that a green suite was hiding a falsified
shared-safety claim, a justification that measurement contradicted, and a
disposal that could delete un-evacuated user entries. **Everything that fixed
those went in unreviewed.** In particular:

- **`inv_FN_28` was restated and two `Hist` fields were added.** `rootTakenAway`
  and `rootTakenWithoutCommitted` are set at three sites (`SQuarantineRename` in
  **both** arms, `SDisposeInPlace`, `SRemoveRoot`). Is the claim now satisfiable
  by construction — a flag set by the very step whose licence it asserts? Is
  there a control that kills it, or is `FN-28` now green for the reason the
  previous encoding was green? Is setting `rootTakenAway` in the torn-rename arm
  right, or does it launder `EN-01`'s failure into a success? `relax_EN_13`
  violates `inv_FN_28` under **both** the old and the new operand — `k80`
  measured that and called it pre-existing; check that reading.
- **`nextInPlaceDisposable` now walks every entry not `Disposed`.** Does the
  candidate now dispose entries it should refuse — an entry `AtRoot` that
  evacuation never claimed, which the incumbent only ever touches from *inside*
  the quarantine, under a manifest that proves ownership? `FN-32` and `FN-21.c`
  are about exactly that, and the candidate has no document.
- **`groveReservationStands` is retained as a declared-uncontrolled conjunct.**
  Is *declared uncontrolled* the honest disposition, or is the disjunct simply
  dead weight that should go — and if it should go, does the catalogue's role
  sentence go with it?

**What is already measured and should be attacked rather than repeated.** The
`k80` leaf body carries every figure with its command. The frozen runs are
recorded in `crates/grove-finish/models/README.md`'s run table with digests
either side.

**One general obligation `k80`'s reviewer produced, worth applying here.** *A
module that changes what the model does must be run against **every** claim the
model has, not only the ones the module declares.* That sweep is what surfaced
`FN-28`; `scenario_in_place_march` has changed since.

## Done when

- Every substantive repair `k80` made after its own review is either upheld with
  the control that would falsify it named and run, or reported as a finding with
  the state or trace that shows it.
- The `FN-28` restatement is specifically attacked for construction-satisfaction:
  a claim stated over flags its own steps set is the shape this node has been
  burned by twice, and a green with no available kill is not a pass.
- The findings are reported without fixing them — this is the adversarial read,
  and repairs are `integrate-review-design honest-classification`'s if this leaf
  cuts one.

## Notes

**Read the models, not only the prose.** Four documents restate `k80`'s
conclusions — the catalogue, both the ADR and the finish ADR, and the model
README — and a defect that reached all four reads as corroboration. The two
`.qnt` files are the subject; `finish.als` is **unrepaired by design** and is
`alloy-candidate-k82`'s, so a finding that Alloy disagrees is not this leaf's
unless the disagreement is about what `k80` wrote.

**Spend no in-session reviewer.** A `review-*` session *is* the adversarial read.
