# honest-classification-k85

**Integrates:** honest-classification-k84

## Goal

Verify and integrate the five findings from `review-design
honest-classification-k84` before `sweep-ownership-k81` edits the same catalogue,
Quint library and control module. Repair the instrument, its source-of-truth
claim and its decision record; rerun the verification the review session was
forbidden to run.

## Context

The reviewed producer is commit `04ea4af5` (`honest-classification-k80`). The
review is the immediately preceding sibling and carries exact evidence ranges:
[`02-DONE-review-design-honest-classification-k84.md`](02-DONE-review-design-honest-classification-k84.md)
after that review retires. Its five findings are the work list, not a summary to
replace with a fresh general review:

1. `FN-28`'s new history fields are written only by the same root-removal sites
   the invariant asks about, have no isolating kill, and `SQuarantineRename`
   sets `rootTakenAway` in the torn-rename arm where `rootPresent` remains true.
   `relax_EN_13`'s old-and-new failure attacks the unchanged proof conjunct and
   is not a control for the new operand.
2. `nextInPlaceDisposable` selects an unevacuated `AtRoot` entry and
   `SDisposeRootEntry` deletes it without checking the manifest identity/digest
   or modelling a foreign ordinary child. The reachable late-result/incomplete-
   evacuation path therefore succeeds by disposing bytes it has not proved.
3. The catalogue's `FN-28` still says the quarantine rename is the only
   root-removal transition, while the repaired Quint invariant accepts
   `SRemoveRoot` through generic flags. The source of truth and the green cell
   are different propositions.
4. `groveReservationStands` is retained as a declared-uncontrolled branch while
   the `FN-24.a` cell is reported complete. `handEditTo(12)` supplies the
   first-realisation-only disk, but no classifier mutation admits it.
5. `a-shared-safety-claim-names-the-role-not-the-artifact.md` preserves the
   producer/reviewer chronology and an obsolete rationale, and uses a
   consequent operand at `FN-28` to establish a rule stated only about guards.
   It is not yet a minimum current-state decision record.

`sweep-ownership-k81` remains the owner of the durable candidate/reaper
ownership-proof decision. Integrating finding 2 must leave it an honest starting
instrument — not silently decide its either/or and not leave an unsafe success
for it to inherit. `alloy-candidate-k82` still owns the Alloy implementation of
the available candidate; reconcile its task body if the corrected contract
changes what it must match.

## Done when

- `FN-28`'s role-form is stated once in the semantic contract and implemented by
  the Quint predicate without claiming an attempted or torn rename as a
  completed root removal. A control independently falsifies each new
  load-bearing operand; an unrelated failure common to the old and new
  predicates is not credited as that control.
- The available in-place candidate never reports success after deleting an
  `AtRoot` entry it has not proved still matches the recorded manifest. The
  late-result/incomplete-evacuation path and a foreign or substituted ordinary
  entry are expressible and distinguish the safe response from deletion, or the
  model declares the precise gap and removes that path from evidence about a
  successful candidate until `k81` supplies the proof.
- The catalogue, Quint model, control module, model README, finish ADR and
  shared-safety ADR state the same current `FN-28` and candidate limits. Any
  deliberate Alloy divergence is explicit and remains routed to `k82`.
- The artifact branch of `groveWorkOutstanding` has an isolating witness and
  kill, or the runner/README records a real gap instead of a complete cell. The
  catalogue's role is changed only on design evidence, not to fit the current
  model's reachability.
- The shared-safety ADR is reworked in place to the minimum present decision and
  trade-off. Obsolete reasoning and task/reviewer chronology are left to the VCS
  and experiment evidence; the title and decision cover the same class of
  expression.
- The affected Quint commands, the complete finish-family Quint run, and every
  control introduced for these findings are green with command count, cell
  count, gaps and subject digests recorded under the existing provenance rule.
  No green is claimed from a narrowed module that cannot reach its subject.

## Notes

This integration is adjacent by construction: it was inserted at
`sweep-ownership-k81`'s former slot, so no sibling has changed the cited source
since review commit `04ea4af5`.

## Outcome

All five findings verified and integrated; the sweep required to verify them
returned two more. What the remaining children inherit is in this node's
`BRIEF.md` rather than here, because a retired task body is not in anyone's brief
chain.

| finding | verdict | what it became |
|---|---|---|
| 1 — `FN-28` self-certifying, torn arm launders a removal | **real issue** | flags moved into the completing arm; two isolating kills, each measured green on the *other* operand; `inv_FN_22i`'s long-hidden failure in `relax_EN_01` declared |
| 2 — total in-place disposal deletes an unproved entry | **real issue** | the walk is `evacuated(w)`; an unevacuated entry makes the candidate **block** (`RecoveryPending`), measured in `scenario_in_place_late_result`. `k81`'s either/or deliberately not decided |
| 3 — catalogue and Quint state different `FN-28`s | **contract stated unclearly** | `FN-28` restated over the role, an attempted removal excluded in the catalogue's words, `*Class*: shared safety` added |
| 4 — `groveReservationStands` uncontrolled while the cell reads complete | **real issue** | `mutant_orphan_is_not_a_reserved_state`: an isolating witness (7230/8000) and a kill; the declared gap retired |
| 5 — the ADR keeps chronology, and uses a consequent to establish a rule about guards | **real issue** | reworked in place to the broader rule its evidence supports; title and slug changed together, every live citation repointed |

**Two findings the review did not have.** `inv_FN_28` was violated with **no
model mutation at all**, found by *narrowing* `base`'s environment rather than
widening it — a strict subset of `base`'s traces, so the counterexample was
always `base`'s. And `inv_FN_25b` is red under the in-place candidate,
pre-existing at `6d0188dd`, routed to `sweep-ownership-k81` with its measurement.

**One gap externalised**: `FN-22.e` is green because nothing can falsify it —
`quarantine-gate-control-k86`.

**The run**: exit 0, **272 commands**, 63 of 63 cells, 0 declared gaps, Q4 matrix
10 of 10, 6m 39s wall / 481s user; baseline re-measured this session at
`6d0188dd` was 262 commands / 5m 55s / 419s user. Four subjects digested
byte-identical either side; `--list` prints 130 before and after.
