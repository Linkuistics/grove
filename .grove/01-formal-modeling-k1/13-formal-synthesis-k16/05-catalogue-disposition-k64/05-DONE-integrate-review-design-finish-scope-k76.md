# finish-scope-k76

**Integrates:** finish-scope-k75

## Goal

Integrate the four findings from `finish-scope-k75` before
`lifecycle-scope-k72` consumes the changed state vocabulary and model claims.

## Context

Read the review's `## Findings` verbatim; its `path:line` citations are the
handoff. The producer is `finish-scope-k71`, and its committed change is the
review baseline.

The findings are coupled around the difference between a state predicate and
the outcome selected from it:

1. `FN-25.a` says the diagnosis definitions are disjoint, while the contract
   also declares reachable overlap and Alloy exempts it
   (`docs/specs/semantic-contract.md:891-904`, `946-955`, `1950-1959`;
   `crates/grove-finish/models/finish.als:1339-1382`, `4906-4931`).
2. Every standing quarantine is classified `Reserved(Quarantined)` and described
   as unfinished, even though the fourth proof can already have returned
   `Committed` and the finish can be `Applied` with cleanup outstanding
   (`docs/specs/semantic-contract.md:414-418`, `1845-1848`, `2023-2034`;
   `crates/grove-finish/models/finish.qnt:756-767`, `1722-1731`;
   `src/finish_transaction.rs:1953-1974`).
3. Alloy's `EN-08`/`FN-31.c` cell is declared unmeetable from an estimated
   seventeen-state trace against a thirteen-state bound, without the deeper run
   that would distinguish cost from impossibility
   (`docs/specs/semantic-contract.md:1130-1150`;
   `crates/grove-finish/models/README.md:1565-1582`).
4. The `W9SlotPending` behavior is `NoOp`, but `FN-11`'s comments still call the
   early branch a reachable refusal
   (`crates/grove-finish/models/finish.als:1841-1859`, `2940-2950`).

## Done when

- `FN-25.a` makes one coherent, falsifiable claim: either its two state
  predicates are genuinely disjoint, or it explicitly claims that precedence
  selects exactly one diagnosis from overlapping predicates. The contract,
  glossary, Alloy and Quint classifiers, checks, witnesses, controls and README
  all state and test that same claim.
- The state vocabulary distinguishes the unsettled post-rename window from a
  proven success whose disposal is merely outstanding, or gives one state a
  meaning and consequences that are valid for both. `FN-22`, `FN-24`, `FN-28`,
  `SY-05`, both finish models, both ADRs, `CONTEXT.md`, and the handoff in
  `06-design-lifecycle-scope-k72.md` are reconciled; a disposal failure after
  the fourth successful proof cannot turn `Applied` back into unfinished.
- The Alloy `EN-08`/`FN-31.c` cell is supported by the deeper positive witness
  and crash-removed negative control, or is recorded consistently as a bounded
  gap rather than a logical incompatibility. The exact command, bound, result
  and cost are recorded in the finish README and reflected in the assumption
  table.
- `doCommitAttempt`, `FN-11` and the finish README consistently describe the
  early ordering guard as an internal `NoOp`; the command still has reached
  antecedents and the applied-after-evacuation witness remains reachable.
- Every affected obligation has the required property, witness and isolating
  control in both families, or that family's precise declared gap. Run the
  finish-scope verification and any changed control/mutation cells against the
  final files, and record the commands, bounds and results here.
- Reconcile `finish-controls.qnt`, both durable handoffs and the ADR set after
  the four repairs. The product question already routed to `handoff-audit-k66`
  — whether the shipped reaper should re-read repository disposition — remains
  there unless a repair makes it moot.

## Notes

This leaf was inserted at the first live sibling after the review. Do not let
`lifecycle-scope-k72` absorb the repairs: it consumes both the state order and
the meaning of `Reserved(Quarantined)`, so it must start from the integrated
version.

## Decisions (running log)

### Every finding was verified before it was acted on, and all four stand

`references/integrate-review.md` asks for verification rather than performative
agreement, and then for one of four classifications. All four findings are
**real issues**; none is noise, a trade-off to accept visibly, or a contract
merely stated unclearly — though findings 1 and 2 each turned out to be a
contract that was *false* rather than unclear, which is the same repair site and
a stronger verdict.

**Findings 1 and 2 are one failure mode seen twice: a predicate over disk shape
made to carry a claim about a disposition.** `FN-25.a` asserted a partition of
two *definitions* while the same document declared their overlap reachable and
fixed a precedence over it; §*States*' `Reserved` class asserted *a Grove
transaction is incomplete* of a disk shape, one section after `FN-28` and its
ADR established that the tree answers no question about a disposition. That the
two arrived from one producer session is not a coincidence — both are the
cheapest available way to make a claim look total.

### 4 · `W9SlotPending` — the comments moved to the step grain, and nothing else did

Confirmed exactly as the review states, and it is the smallest of the four.
`doCommitAttempt`'s branch comment said `gateEvacuated` *still refuses* the early
attempt and the `FN_11` command called it a *REACHABLE refusal*; the branch has
returned `NoOp` since `finish-scope-k71`. **The semantic decision is untouched.**
What keeps `FN-11` off being true by construction is restated as the step's
**enabledness**: `doCommitAttempt` is enabled at `PublishedP`, so the early step
occurs and its guard declines to apply it, which makes `Sys.res' = Applied` an
antecedent the file could have satisfied early and does not. The reachability
evidence is `witness_FN_11`'s applied-after-evacuation trace, retained unchanged
and green in the run below. `MN`; no obligation, closed set or command moves.
The finish README carries the restatement in place.

### 3 · `EN-08` / `FN-31.c` — the estimate was run, and it was wrong

**The disposition `finish-scope-k71` landed was an impossibility argued from an
unrun state count**: *a model that POSITS a disk under `EN-11` cannot also
EXERCISE `EN-08` at that disk*, on about seventeen states against `finish.als`'s
thirteen-state maximum. The review said the estimate had never been run. It has
now been.

- **The mid-disposal disk is reached through `crash` in FOURTEEN states** from a
  pristine root — `Idle · Confirm · TxnOpen · Preflight · WPrepare · WManifest ·
  WReady · WPublish · WEvacuate · CommitAttempt · Classify · QuarRename ·
  MarkerCreate · Crash` — found in **8.9 s** at a 16-step bound.
- **And the run-up need not start from a pristine root at all.** `EN-11`'s
  licence still covers the tree at `Fresh`, so starting at
  `interruptedMidEvacuation` reaches both of `FN-31.c`'s boundaries at 14 steps
  in 6 – 7 s. **Two commands already in the file did exactly that, at eleven and
  twelve states** (`witness_FN_21a_the_interrupted_disposal_disk_is_reachable`,
  `witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves`) — so
  the general claim was contradicted twice over inside the file that made it.

**Landed:** both `witness_FN_31c_*` commands now run the protocol instead of
positing the disk, and a fourth control,
`expect_unreachable_EN_08_no_resumption_of_an_interrupted_disposal_is_reachable`,
asserts that they stop landing with `crash` removed (measured: no instance, 6 s;
the earlier boundary measured separately, likewise empty, 5 s). **The row is now
met in both columns and the cell is uncontested.** The catalogue's assumption
table gains the general rule this leaves — *a family's failure to meet an
exercise-removal row is established by running the deeper attempt, never by
costing it in prose; a bound too dear to pay is a declared and **measured** gap,
an unpaid estimate is neither* — as a **third** failure mode beside the two that
section already named. `MN`: no obligation moves, one control command is added.

### 1 · `FN-25.a` — the definitions overlap, and the claim is now about the outcome

**Decided: precedence, not disjointness.** The review's two admissible outcomes
were genuine disjointness or an explicit precedence claim, and disjointness is
not available: making the definitions complementary would narrow one arm until it
cannot meet the other, which is `finish.als`'s own named trap — *an arm narrowed
until it cannot meet its neighbour is an arm that answers `FN-25.a` by
construction*. It trades a false claim for a vacuous one.

So `FN-25.a` now reads: **every block carries exactly one diagnosis, and where
both definitions hold the one it carries is the one precedence selects**, with
the two definitions declared non-disjoint in as many words. `MN` — no obligation
is added or removed, and `--list` does not move.

**What the exempted check cost, measured.** `finish.als`'s `FN_25a` read
`lone diagnosedRaw or declaredDiagnosisOverlap`, which is green whatever the
model does at the two declared overlap classes. Reversing the precedence relation
entirely — `earlierDiagnosis` becomes `DRecoveryPending -> DOwnershipConflict`,
so a disk carrying an artifact Grove cannot classify is reported as a recovery
the operator should go and run — leaves the **old check green** and the repaired
check **red**. Both runs are in `README.md`'s matrix as row 65.

**The repaired check has FOUR conjuncts, three with their own killer**, which is
what turns the exemption's content into obligation: `one diagnosed` by row 52,
the precedence by row 65, and the **floor** — a correlated block with nothing
unaccountable beside it carries `RecoveryPending` — by row 51. The floor is not
decoration: without it the precedence conjunct absorbs row 51, and dropping
`dgTopologyUnmatched`'s proviso would have become invisible in the very edit that
made the precedence checkable.

**The fourth conjunct is the old clause put back, and it is a correction this
leaf made against its own first edit.** `lone diagnosedRaw or
declaredDiagnosisOverlap` was deleted as *the false disjointness claim* — it
carried that label and the label was false. Read as a check it says something
else and true: **the arms meet only where the file says they meet.** Without it a
third overlap class appearing later satisfies `one diagnosed`, falls outside the
precedence conjunct's antecedent, and passes in silence, because
`earlierDiagnosis` resolves any pair to one atom. **A clause can be load-bearing
for a claim other than the one it is labelled with** — the sibling of
`references/execute.md`'s *a clause rescued by its neighbour is not correct*,
read from the other end, and worth carrying as a general obligation of any
repair that deletes a clause it has just proved false. The kept form is strictly
stronger than either the exempted original or this leaf's first replacement, and
all three mutations still kill it (measured: base green, rows 51, 52 and 65 each
one counterexample). It is why the Alloy cell below is a **second** run.

**The Quint column had the same defect wearing the other hat and answered the
same disks differently.** Its `inv_FN_25a` was *the two diagnoses are disjoint*,
which `diagnose`'s if/else chain made **true by construction** — no mutation
could have moved it. And its classifier gave `RecoveryPending` wherever a
correlated attempt met an `unprovable` state outside its narrow `cannotClassify`
predicate, which is where `finish.als` gave `OwnershipConflict`. The old
argument for that — *of an artifact Grove has just proved is its own, "Grove
cannot classify it as its own" is FALSE* — is sound about the witness slot and
does not generalise: `unprovable` also holds of an undigestible entry in the root
and of a root swapped under the transaction, about which proving one artifact is
Grove's says nothing. Landed: the correlation proviso on the topology clause, the
precedence behind a new dial `OWNERSHIP_WINS_THE_OVERLAP`, the invariant restated
over a ghost that the reversal fires, the witness strengthened from *resolved to
exactly one* (satisfied by either winner) to *resolved to `OwnershipConflict`*,
and `mutant_correlation_wins_the_overlap` as the isolating control.

**Two measurements from that work are worth more than the edits.**

- **A symmetry the model refused.** `not(w.returnCanComplete)` was given the same
  proviso as the topology clause, on the reading that `FN-22.h`'s row is a
  correlated Grove-owned block — and `inv_FN_25b` went **red** in
  `scenario_return_blocked`: the state that row reaches is *not*
  `groveOwnedCorrelated`, so with a proviso it fell through both diagnoses and
  exhaustiveness died. The catalogue deliberately names no diagnosis on that row
  (*it is the classifiers' and not this document's*), so the two columns are left
  disagreeing and the divergence is **declared** rather than smoothed over. Only
  one clause carries the proviso, and the count is measured.
- **A mutant module's ENVIRONMENT is part of the control.** The first
  `mutant_correlation_wins_the_overlap` copied its neighbours' environment
  (`ENV_KINDS = Set(0)`) and reported **green** — the overlap is reached only by
  an in-transaction hand edit, which is `scenario_edit_txn`'s environment and
  nowhere else in the file. A green mutant reads as a surviving claim when it is
  really an unreached one. The module now carries that environment and the reason
  in place.

### 2 · `Reserved(Quarantined)` — a reserved state is a fact about a NAME

**Decided: one state, one meaning, valid for both windows** — the second of the
review's two admissible outcomes, and it is cheaper *and* truer than splitting
the member. The class sentence `finish-scope-k71` landed was *an artifact at a
name Grove reserves says a Grove transaction is **incomplete***. It is false past
`FN-22`'s fourth revalidation point: a `Committed` returned unchanged makes the
finish `Applied` with the quarantine standing, and the shipped protocol returns
success there even when disposal fails (`src/finish_transaction.rs:1953-1974`).
Under that sentence one disk was a proven success and evidence of an unfinished
transaction at once.

The sentence now reads **says Grove has *work outstanding at that name***, and
the repair is `FN-28`'s own rule applied one section earlier: **the tree answers
questions about names, and the ticket answers questions about outcomes.** That
is why splitting the member is the wrong repair — a second member would put the
disposition back into the classification, which is the error rather than the
fix, and the disk cannot distinguish the two windows anyway.

**Nothing about the ORDER moves, and that is the point of separating the two.**
The window between the rename and the fourth point is real, is what `SY-05.b`
needs, and is now stated as an argument for the **order** rather than as a
reading of the member. `MC` in the sense `closed-set-additions-k74` measured — a
closed set an obligation sweeps changes meaning without `--list` moving.

**Reconciled, each in place:** §*States*' class sentence, the
`Reserved(Quarantined)` row (which now says it is reached on both sides of the
fourth point), `FN-22`'s `Applied` row (whose stable state said *task root
`Absent`, quarantine holding the root* — two things the repaired table cannot
both say), `CONTEXT.md`'s **Quarantine** and diagnosis entries,
[`success-is-proved-by-the-ticket-not-the-tree`](../../../../docs/adr/success-is-proved-by-the-ticket-not-the-tree.md)
(whose own paragraph carried the error), both model files, the finish README, and
the handoff in `06-design-lifecycle-scope-k72.md`, which gains the half it was
missing.

**And the coexistence is witnessed rather than argued, in both columns.**
`witness_FN_28_a_success_whose_cleanup_is_still_outstanding` now requires
`classified in reservedClass` beside `finishSucceeded` — written over the CLASS,
because §*States*' own discipline is that removing a member changes no claim, and
because the state it lands in is `Reserved(Published)` rather than
`Reserved(Quarantined)` (the witness is still inside the quarantine at that
point, which is a fact worth knowing and was not written down anywhere).
Quint's `successWithCleanupOutstanding` was the literal `true` — it witnessed the
branch, not the state — and now records `isReserved(classify(w))`. **A model in
which that conjunction is unreachable has the collision back.**

### `finish-verdicts-k65` and `handoff-audit-k66` are unaffected but for one note

The product question `finish-scope-k71` routed to `handoff-audit-k66` — whether
the shipped reaper should re-read the disposition before disposing — **is not
moot** and stays there. It is sharpened, and the sharpening is written into that
leaf: with the contract now saying that nothing on the disk carries a disposition
(not the classification, not the quarantine, and by `FN-20` not the marker), the
*in favour* argument can no longer be stated as "the marker tells the reaper the
finish succeeded" but only as "the marker is an instruction, not evidence" —
a narrower claim. Nothing else is routed anywhere.

### One ADR is written and one is edited in place, and the AND test decides both

**Written: [`a-closed-partition-is-over-outcomes-not-states`](../../../../docs/adr/a-closed-partition-is-over-outcomes-not-states.md).**
All three legs hold. *Hard to reverse*: `FN-25.a`, both classifiers, three
mutation-matrix rows and the routed shipped-diagnostic question rest on it.
*Surprising without context*: a closed set of two diagnoses reads as a partition
of states, and both the contract and both models said so. *A real trade-off with
a rejected alternative*: genuine disjointness by narrowing `RecoveryPending` is
statable and is rejected, because it answers the obligation by construction —
the trap `finish.als` names in place. The record carries the generalisable half a
later reader needs the cost of: **a check that exempts its own declared
counterexamples tests nothing where the claim is hardest**, with the measurement
that says so, and the corollary about deleting a clause you have just proved
false.

**Edited in place, not appended to:**
[`success-is-proved-by-the-ticket-not-the-tree`](../../../../docs/adr/success-is-proved-by-the-ticket-not-the-tree.md).
Finding 2's rule passes the AND test too — but it is **the same rule that record
already states**, applied one section earlier, and `ADR-FORMAT.md` asks for a
minimum coherent set rather than two places to look for one rule. Its own
paragraph carried the error, so the record is where the repair belongs. No
superseding record was appended, which that file forbids.

**And `CONTEXT-MAP.md`'s ADR-ownership list was two records short** — the ticket
ADR `finish-scope-k71` created was never added to it. Both it and the new record
are now listed, checked by enumerating `docs/adr/*.md` against the map rather
than by reading the list.

### Run lines — three cells and the runner's own controls, all exit 0

The catalogue, both model files, the control file and the finish README were
**frozen before the batch started** at `2026-08-28T01:19:45Z`, their digests
recorded, and recorded again at the end **unchanged** — the runner reads the
catalogue as its manifest and the scope README for declared gaps, so all five are
subjects rather than bystanders. The Alloy cell was then re-run alone against its
own recorded digest after the `FN_25a` correction below; the freeze held across
that too, and the catalogue's one prose addition during it was checked against
`--list` rather than assumed harmless.

```sh
models/run.sh --scope finish --family alloy                 # 187 commands, 63 of 63 cells, exit 0 (run twice; see below)
models/run.sh --scope finish --family quint                 # 239 commands, 63 of 63 cells, exit 0
QUINT_VERIFY=1 models/run.sh --scope finish --family quint  # 302 commands, 63 of 63 cells, exit 0
models/run-controls.sh                                      # 10 passed, 0 failed, exit 0
```

All three report `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63` and
`10 of 10 rows` of Q4's removal matrix. **No cell is contested** — neither family
declared a gap where the other answered, and the `FN-31.c` cell that was
one-sided in evidence is now answered with a control in both columns.

**The Alloy cell is 187, one more than `finish-scope-k71` left it**, and the one
is the `EN-08` control. Three commands got dearer rather than more numerous: the
two `FN-31.c` witnesses moved from 3 and 4 steps to 14 (7 s and 6 s), and
`FN_25a` from the exempted form to the four-conjunct one (9 s). **18 m 48 s
wall** (`02:03:05Z` → `02:21:53Z`), run alone, `finish.als`'s digest identical
either side.

**That is the SECOND run of this cell, and the first is recorded rather than
overwritten.** The first — 187 commands, exit 0, 20 m 12 s, run concurrently with
the non-verify Quint cell — measured the three-conjunct `FN_25a`. Re-reading my
own edit found that the deleted clause was checking something true, so the clause
went back and the cell was re-run rather than the difference argued away. **A
comment-only edit landed while a third, earlier attempt was in flight and that
attempt was killed rather than kept**: the freeze is the point of the digest, and
a run whose subject moved under it is not a measurement of the file it reports
on, however harmless the edit looks.

**The catalogue gained one `*See*:` line during the Alloy re-run, and the effect
on the manifest was CHECKED rather than assumed.** `models/run.sh --list` prints
`-- 130 obligations in scope` against both the mid-run text and the final text,
byte-identical output, so the manifest that run asserted coverage against is the
one in the tree. A prose line inside a bullet's paragraph is not an obligation —
but *is not* and *was checked not to be* are different claims, and this section
is where the second one belongs.

**The Quint cell is 239 non-verify and 302 with `QUINT_VERIFY=1`**, one more than
k71's in each case — `inv_fail_MUT_FN_25a_correlation_wins_the_overlap`, which
reports `violated, as the control requires`. **14 m 35 s wall**
(`01:37:28Z` → `01:52:03Z`), about 1 m 40 s over k71's, and the difference is
`models/run-controls.sh` overlapping it — which is that leaf's own measurement
holding a second time: budget an Apalache cell alone.

**All 63 properties are `model-checked to depth 4, no counterexample`**, the
restated `inv_FN_25a_the_carried_diagnosis_is_the_one_precedence_selects` among
them. So the precedence is model-checked rather than only simulated, which
matters here because it is a claim about states the simulator reaches rarely: the
witness lands in 1243 traces, and the mutant's counterexample in a handful.

`models/run-controls.sh` was run because three commands of shapes the runner
classifies entered or changed in the suite (`inv_fail_MUT_<OB>_…` and
`expect_unreachable_<EN>_…`), and the controls are what assert the classification
rather than the commands.

### What this leaf did NOT do

- **No obligation was added, removed or re-scoped.** `models/run.sh --list`
  arithmetic is untouched; all three repairs to claims are restatements of
  existing obligations, and the fourth is comments plus one control command.
- **`docs/formalism-findings.md` is not revised.** No entry named any of these
  four as owed — the dispositions live in the catalogue and the finish README —
  and the node brief says the log is a log.
- **The two confirmed dispositions were not reopened.** Item 14 / `FN-28` and
  item 28 / the declined general form were both confirmed by the review with
  their own evidence, and nothing here disturbs them; the `FN-28` work in this
  leaf strengthens its *witness*, not its claim.
- **`SY-05` was not edited.** Finding 2 changes what may be inferred from a
  reserved classification and not the order that classification is in, and the
  order is what `SY-05.b` rests on. The consequence is written into
  `lifecycle-scope-k72`'s body, which owns that obligation.
