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

## Findings

### 1. `FN-28` is still satisfied by construction, and the torn-rename arm records a removal that did not happen

`rootTakenAway` and `rootTakenWithoutCommitted` are written only by the three
root-removal sites and are then read directly by
`inv_FN_28_one_successful_exit` (`finish.qnt`:1265-1273, 1882-1914,
2037-2046, 2082-2092, 3320-3343). There is no independent control that can make
one of those sites omit or falsify either flag. The candidate module merely
asserts the repaired invariant (`finish-controls.qnt`:1108-1120), and the run
record reports that assertion green; it does not kill the new operand.

The strongest counterexample is already in the source. `SQuarantineRename`
sets `rootTakenAway = true` *before* branching on `RENAME_ATOMIC`. In the false
arm it leaves `rootPresent` true, creates no quarantine, and sets only
`rootTorn = true` (`finish.qnt`:1889-1914). A proven-result trace can then carry
`OApplied` through the disposal tail with both new flags satisfying `FN-28`, even
though Grove never established that it took the task root away. The ghost has
laundered `EN-01`'s torn rename into the fact the claim is meant to test.

The recorded `relax_EN_13` violation does not control this repair. That module
changes only the reaper's sweep premise, and the producer records that the old
and new predicates both fail there. It therefore attacks the unchanged
`deletionProvenFor` conjunct, not either new history field. A failure common to
both formulations is evidence that the new operand was not exercised.

### 2. The total-disposal repair deletes an unevacuated entry without proving it is still the manifest's entry

`nextInPlaceDisposable` now selects every initial entry whose place is not
`Disposed`, including `AtRoot` (`finish.qnt`:1397-1418), and
`SDisposeRootEntry` immediately changes the selected entry to `Disposed`
without checking its place, type, digest, manifest membership or ownership
(`finish.qnt`:2059-2070). The producer itself records the reachable case that
motivated this change: a crash after publication, followed by a late exact
result, reaches `SRevalBeforeQuarantine` with evacuation incomplete
(`finish.qnt`:1411-1414). In that case the candidate now unlinks the entry at
the user-visible root instead of refusing or proving it.

The existing model cannot falsify that move. `entryIds` is fixed at `init`, and
`foreignWriteAt` can introduce foreign content only at the witness,
quarantine, or marker reserved names (`finish.qnt`:2247-2277, 2821-2836); it
cannot introduce or replace an ordinary child at the task root. The manifest is
written with the original digests (`finish.qnt`:1696-1703), but the candidate
never consults them when deleting. Consequently the green `FN-27`/candidate
measurements exclude the environment action that would distinguish an original
entry from foreign bytes occupying its path.

This lands ahead of the design decision it depends on. `sweep-ownership-k81`
exists to decide the candidate-reachable ownership site and explicitly says the
candidate currently has no ownership proof. Treating all remaining entries as
disposable makes that later proof irrelevant to the transition already doing
the deletion, and makes the candidate's successful exit include an unproved
cleanup path.

### 3. The catalogue and the repaired Quint invariant state different `FN-28` claims

The semantic contract still says the *only* transaction transition that takes
the root away is the quarantine rename and that the rename does so only on a
proven result (`semantic-contract.md`:2321-2328). The available candidate instead
uses `SRemoveRoot`, and the repaired Quint invariant accepts any protocol whose
own removal site sets the two generic history flags (`finish.qnt`:2082-2092,
3340-3343). The edit to the catalogue adds a note under `FN-24.a` saying the same
problem occurred at `FN-28`, but it does not restate `FN-28` itself.

This is not the disclosed, temporary Alloy divergence: the Quint column is now
green against a role-form that its source-of-truth obligation does not contain.
The README and the new ADR call `FN-28` repaired while the catalogue still names
the incumbent mechanism, so the runner's 63-of-63 coverage claim is over two
different propositions with no declared gap.

### 4. `groveReservationStands` remains an unchecked branch while the cell is reported complete

`groveWorkOutstanding` retains `groveReservationStands` as one of two
realisations (`finish.qnt`:915-940), but the producer measured that every existing
`FN-24.a` kill still fires with `unsettledRootWork` alone and that no current
control reaches a biting state unique to the artifact branch. Calling the branch
"declared uncontrolled" is an honest limitation; it is not verification of the
shared-safety obligation.

The missing control is not an expressivity limit. `handEditTo(12)` already
constructs an orphaned, Grove-owned quarantine with no transaction live
(`finish.qnt`:2807-2812), exactly the first-realisation-only state, while
`mutant_no_quarantined_state` admits only crashes (`finish-controls.qnt`:1920-1950).
No command combines the state with the classifier mutation. The README therefore
cannot simultaneously say the conjunct is uncontrolled and report the `FN-24.a`
cell complete with zero gaps. The catalogue's two-realisation role remains a
design decision; the finding is that this column has not checked one of them.

### 5. The new ADR preserves review chronology instead of the minimum current decision

`a-shared-safety-guard-names-the-role-not-the-artifact.md` retains the sequence
of discoveries by `finish-verdicts-k78`, `honest-classification-k80`, and its
reviewer (lines 17-67), then records the obsolete justification and its later
correction inside the alternatives table (line 102: *"not for the reason first
recorded here"*). That is the producer/reviewer history, already present in the
task body, README and VCS, rather than the current decision and the reason that
now binds.

The record also uses `FN-28` as the third instance that allegedly establishes a
rule about *guards*, although `FN-28`'s changed expression is a consequent
operand, not the antecedent deciding whether the claim applies (lines 3-15,
38-50). The minimum coherent record needs either the broader rule its evidence
actually supports or only the guard decision; the current document mixes the
two and carries the experiment chronology needed to bridge them.

## Review limits

This was the inspection-only review required by `references/review.md`. No model,
test, build, lint or format command was run. The recorded 262-command run and
its digest provenance were inspected in `crates/grove-finish/models/README.md`.
The codebase-memory generation predates the producer commit and reports the two
`.qnt` files as untracked and `docs/` as excluded, so every cited range above was
read directly from the committed source at `04ea4af5`.
