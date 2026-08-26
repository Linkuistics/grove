# finish-k57

**Reviews:** finish-k12

## Goal

Read the Quint finish/recovery model adversarially and decide whether its green
run is evidence or construction.

## Context

The producer is the adjacent leaf `finish-k12`; the reviewed artifact is the
commit that retires it —
`crates/grove-finish/models/finish.qnt`,
`crates/grove-finish/models/finish-controls.qnt`, the Quint section of
`crates/grove-finish/models/README.md`, and entry 045 of
`docs/formalism-findings.md`.

**The independence barrier still applies.** Do not open any `.als` file, the
Alloy sections of a model-directory `README.md`, or entries 026 – 043.
`cross-model-replay-k15` is where the barrier comes down; this is not it.

**Why this chain exists, and it is not that the artifact is large.**
`cross-model-replay-k15` already reads this model adversarially and re-derives
every finding, so a review that only re-read the claims would duplicate it. Two
things replay will not read, and both are places where a false green and a true
one produce identical output:

1. **The search dial and the twelve `scenario_` instances.** Every witness in
   this column lives in a focused instance, because an unfocused search reaches
   the end of a twenty-step transaction with probability `(1/k)^20`. That is a
   real problem with a defensible remedy — and it is also exactly the shape of
   an instrument that can be narrowed until it passes. The question a reviewer
   owns is whether each witness's ghost is set by something the **protocol**
   does or by something the **scenario's own construction** guarantees. A ghost
   that only a hand edit can set, witnessed in a scenario whose only admitted
   environment action is that hand edit, is a tautology wearing a percentage.
2. **The eight model mutations.** "The obligation was true by construction and
   this dial proves it" and "I mutated the model until the claim died" produce
   the same runner output. `mutant_short_preflight` kills four obligations at
   once; whether that means four obligations rest on one coding habit, or that
   one dial was drawn wide enough to catch four unrelated things, is a judgement
   the producing session is the wrong context to make.

## Done when

- Each of the 129 witnesses is classified: **protocol-established** (the ghost is
  set by a transition the claim is about), **construction-established** (the
  ghost is set by the scenario's own setup and the witness would land whatever
  the protocol did), or **unclear**. Every non-protocol classification is a
  finding, and the count is reported whatever it is.
- The eight model mutations are each judged against the obligation they kill:
  does the mutation remove the mechanism the obligation names, or something
  wider? `mutant_short_preflight` and `mutant_unproven_ownership` are the two
  drawn widest and should be read first.
- The six material findings in entry 045 are each checked against the catalogue
  text they cite, and any that overstates what the model establishes is reported.
  Finding 1 (`EN-01` narrower than the protocol's own steps) is the load-bearing
  one and rests on a reading of "same-directory"; finding 6 rests on a
  counterexample and should be the easiest to confirm or refute.
- The declared abstractions are checked for one thing only: does any obligation
  read a field the abstraction makes vacuous? The index image is already
  declared as such; the question is whether anything else is.
- The Q4 removal matrix's ten rows are checked in the direction no runner can:
  is the obligation each row names actually the **first** shared-safety
  obligation its artifact's removal breaks? Three rows read `none` on the
  strength of one control (`relax_EN_03`) and they are what Q1 will be decided
  on.

## Notes

Inspection only: no runs, no edits, no re-derivation of the model. Findings are
the output, and the paired `integrate-review-prototype` step owns every fix and
all post-fix verification.

Two conclusions are worth stating explicitly if they stand, because a review
that reports only defects leaves the next session unable to tell what was
checked: that the `FN-25.b` instrument (two independent encodings of one
partition, checked for agreement) is sound, and that the step-list encoding
answers `FN-24.b` rather than restating it.

## Review findings

### Verdict

The search dial is not manufacturing the reachability record: all 129 `wit_`
commands are **protocol-established**, with **0 construction-established** and
**0 unclear**. The scenario modules bind constants only; none assigns `w`, `t`
or `hist`. Every positive target is set by a transition in `finish`, and every
negative control names a target set by the transition or dimension that the
control removes. Exhaustive accounting by module:

| module | witnesses | classification |
|---|---:|---|
| `scenario_march` | 38 | protocol-established |
| `scenario_gates` | 30 | protocol-established |
| `scenario_crash` | 10 | protocol-established |
| `scenario_crash_late` | 6 | protocol-established |
| `scenario_crash_disposal` | 9 | protocol-established |
| `scenario_edit_txn` | 6 | protocol-established |
| `scenario_reval` | 8 | protocol-established |
| `scenario_foreign_marker` | 1 | protocol-established |
| `scenario_unclassifiable` | 1 | protocol-established |
| `scenario_orphan` | 1 | protocol-established |
| `scenario_return_blocked` | 1 | protocol-established |
| `scenario_return_crash` | 1 | protocol-established |
| `relax_EN_03` | 1 | protocol-established |
| `relax_EN_05` | 4 | protocol-established (three are negative controls) |
| `relax_EN_08` | 6 | protocol-established negative controls |
| `relax_EN_15` | 2 | protocol-established |
| `relax_EN_16` | 4 | protocol-established negative controls |

That classification does **not** make every property meaningful. Six findings
below separate a real protocol-fed witness from a property that still passes by
construction or checks less than its prose claims.

### B1 — `FN-25.b` does not compare the two encodings it claims to compare

**High.** The model says the diagnosis carried at each block site is checked for
agreement with the independent state classifier, but `blockNow` obtains the
carried diagnosis directly from `pickDiagnosis`, which obtains it from that same
classifier ([`finish.qnt`:1138](../../../crates/grove-finish/models/finish.qnt#L1138),
[`finish.qnt`:1606](../../../crates/grove-finish/models/finish.qnt#L1606),
[`finish.qnt`:1619](../../../crates/grove-finish/models/finish.qnt#L1619)). The
invariant checks only that `diagnose` has size one; it never compares
`diagnosisOf(t.outcome)` with `diagnose(w, t)`
([`finish.qnt`:2571](../../../crates/grove-finish/models/finish.qnt#L2571)).

The one site that does carry a diagnosis independently makes the miss concrete:
the tracked-witness commit hard-codes `Blocked(OwnershipConflict)`
([`finish.qnt`:1862](../../../crates/grove-finish/models/finish.qnt#L1862)),
while the state has a correlated, classifiable published witness and therefore
`diagnose` returns `RecoveryPending`. `inv_FN_25b` remains green because both are
single diagnoses. Its witness is weaker again: “an exhaustive sweep” is only
`blockedStatesSeen > 0`
([`finish-controls.qnt`:126](../../../crates/grove-finish/models/finish-controls.qnt#L126)).
The claimed soundness conclusion therefore does not stand.

### B2 — finding 6 moves the root-swap case into `FN-25` by contradicting `FN-06`

**High.** The catalogue says a mid-transaction root swap is a **refusal**, and
its witness explicitly requires “refused”
([`semantic-contract.md`:971](../../../docs/specs/semantic-contract.md#L971)).
The model sends every identity mismatch after a persistent step to `blockNow`,
yielding `Blocked`
([`finish.qnt`:1135](../../../crates/grove-finish/models/finish.qnt#L1135)).
Because `FN-25` is expressly a partition over blocks and not refusals
([`semantic-contract.md`:1233](../../../docs/specs/semantic-contract.md#L1233)),
entry 045 finding 6 cannot yet count the swap as evidence that the diagnosis
partition is incomplete. It first exposes a catalogue/model outcome conflict:
either `FN-06` must become a block (then the partition needs the case), or the
model must refuse it (then it is outside `FN-25`).

### B3 — the rootless ticket instrument drops the attempt identity

**High.** `FN-03` and `FN-04` require the deletion ticket to name the finish
handle **and the attempt identity**
([`semantic-contract.md`:925](../../../docs/specs/semantic-contract.md#L925)).
`deletionProven`, however, accepts any ticket with the handle and ignores its
attempt ([`finish.qnt`:1167](../../../crates/grove-finish/models/finish.qnt#L1167)).
The rootless recovery path has no manifest or quarantine, sets `settling = -1`,
and reports `Applied` from that handle-only predicate
([`finish.qnt`:1974](../../../crates/grove-finish/models/finish.qnt#L1974)).
The same predicate is used by `FN-03` and `FN-28`, so their properties and the
“ticket alone” witness can accept another attempt's ticket. The exact-result
classifier elsewhere already provides the right attempt-bound shape.

### B4 — `Current(Live)` is an undeclared vacuous classification branch

**Medium.** The catalogue includes `Current(Live)` as a stable task-root state
([`semantic-contract.md`:384](../../../docs/specs/semantic-contract.md#L384)),
and `FN-24.a` quantifies over stable classification after every interruption.
The model's classifier reads `liveOrdinaryWork`, but that function is hard-coded
to `false`
([`finish.qnt`:657](../../../crates/grove-finish/models/finish.qnt#L657),
[`finish.qnt`:688](../../../crates/grove-finish/models/finish.qnt#L688)). The
separate `guardsOk` flag can make entry refuse, but it never makes the stable
state `Current(Live)`. This abstraction is not declared beside the index-image
abstraction and makes a branch read by `FN-24.a` unreachable.

### B5 — three shared-safety greens still have no falsifying mechanism

**Medium.** `FN-14`, `FN-26` and `FN-30` are asserted over fields that no
transition or mutant can make bad. `unrelatedMutated` and `historyRewritten` are
initialised false and only read; the commit sets `hooksRan` to false regardless
of the installed-hook input
([`finish.qnt`:1634](../../../crates/grove-finish/models/finish.qnt#L1634),
[`finish.qnt`:1891](../../../crates/grove-finish/models/finish.qnt#L1891),
[`finish.qnt`:2380](../../../crates/grove-finish/models/finish.qnt#L2380),
[`finish.qnt`:2584](../../../crates/grove-finish/models/finish.qnt#L2584),
[`finish.qnt`:2620](../../../crates/grove-finish/models/finish.qnt#L2620)).
Their witnesses are protocol-established, but the safety property is true by
construction. Add focused model mutations, or narrow the evidence claim.

### B6 — Q4's three `none` rows establish only a bundled replacement

**Medium.** Q4 says each row names the first shared-safety obligation broken by
that artifact's removal, but Q4-105 through Q4-107 all cite row 902
([`README.md`:3261](../../../crates/grove-finish/models/README.md#L3261)). Row
902 simultaneously grants atomic recursive deletion and removes the quarantine,
cleanup marker and replace transition
([`README.md`:3243](../../../crates/grove-finish/models/README.md#L3243)). It
establishes that the **three-part mechanism can be replaced as a bundle** while
retained obligations stay green; it does not establish `none` for each artifact
removed independently. Q4-101–104, Q4-109 and Q4-110 name the first direct
shared-safety obligation their roles support; Q4-108 is honestly marked
abstracted. Reword 105–107 as one bundled candidate result, or supply
artifact-specific removals.

### Checks that stood

- The `FN-24.b` step-list encoding does answer rather than merely restate the
  obligation: `Step` is closed, `persistentEffect` is total, non-rename effects
  are enumerated in `DECLARED_STEPS`, and both branch witnesses are transition-fed
  ([`finish.qnt`:361](../../../crates/grove-finish/models/finish.qnt#L361),
  [`finish.qnt`:2557](../../../crates/grove-finish/models/finish.qnt#L2557),
  [`finish-controls.qnt`:120](../../../crates/grove-finish/models/finish-controls.qnt#L120)).
- Entry 045 findings 1–5 stand against the catalogue text. Finding 1 is the
  load-bearing same-directory mismatch; findings 2 and 3 identify missing closed
  members; finding 4 correctly exposes `FN-13`'s refusal/block conflict; finding
  5 correctly exposes the `Indeterminate`/diagnosis tension. Finding 6 needs the
  B2 reframe before it can stand.
- The eight model mutations are causally useful. `mutant_status_classifier`,
  `mutant_no_revalidation`, `mutant_nonatomic_marker`, `mutant_eager_recovery`,
  `mutant_eager_preflight` and `mutant_receipt_reading` remove the named
  mechanism. `mutant_short_preflight` validly kills all four named checks but is
  deliberately wider—it also skips the layout precondition.
  `mutant_unproven_ownership` validly kills `FN-10.b` in its focused scenario but
  is globally wider—it also disables the root-identity stop. These are bundle
  controls, not eight independent minimal mutations.
- Of the declared abstractions, the index-image limitation is honest and the
  step-list abstraction is useful. B4 is the additional vacuous field read by
  an obligation.
