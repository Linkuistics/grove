# exits-k46 — brief


## Goal

Close the alloy column of the finish scope. Answer `FN-02`, `FN-23` – `FN-30` in
`crates/grove-finish/models/finish.als` — the `Blocked` diagnosis partition, the
full step-boundary crash sweep, hook suppression and the single successful exit —
fourteen obligations, the last fourteen empty cells. Own `EN-08` and `EN-16`.
Write **Q4's artifact/transition removal matrix**.

`finish-k8`'s `Done when` says what closing the column looks like: coverage
**asserted**, zero empty alloy cells for the finish scope, and `--no-coverage`
gone from the README's run line. That last edit is the visible signal, and it is
this leaf's.


## Context

`disposal-k45` left the file green at **118 commands, 7 m 39 s, fourteen empty
alloy cells**, and `handoff-k42`'s subtree complete. Read
`crates/grove-finish/models/README.md` before writing a command; six of its
sections are specifically this leaf's inheritance and are named below.

**What now exists that the exits build on.**

- **The transaction runs end to end, including its disposal.** Twelve body
  steps, two filesystem handoffs with four revalidation points around them, a
  ten-row table written as **data**, a quarantine with an inverse, and a
  three-step marker-guarded disposal with a **reaper** that resumes it. `Reap`
  is deliberately in neither `bodySteps` nor `txnActs` — a sweep is not a step of
  the transaction, takes no confirmation, and never had a disposition to
  revalidate. **`FN-24.b` quantifies over `bodySteps`, so read that exclusion
  before you ask the question of it**; whether a sweep owes the one-persistent-
  effect discipline is `FN-24.b`'s to decide, not something the set already
  answered.
- **`bodySteps` holds sixteen members.** The six body steps, `Recover`,
  `Classify`, `QuarRename`, `Settle`, `Revalidate`, `QuarReturn`, and disposal's
  `MarkerCreate`, `MarkerReplace`, `Dispose`, `MarkerRemove`. **One of them
  plainly has more than one persistent effect and is named as such**:
  `doDispose` clears the quarantine, the reserved slot and the manifest together,
  because in this model they are one directory. `EN-03` says the shipped removal
  is entry-by-entry. That is recorded as an abstraction and it is `FN-24.b`'s to
  judge; `doSettle`'s restore branch is the other one `commit-k41` left you.
- **`exits` inherits FOUR model-only `why` values its partition has to absorb,
  not one.** `W14QuarantineOccupied`, `W15CommittedAfterRestore`,
  `W16ReturnIncomplete` and `W17OwnershipConflict`. Three of the four are
  `Blocked` branches the catalogue diagnoses `RecoveryPending`; the fourth is
  `OwnershipConflict` and is the only one the **catalogue itself names** — it
  serves both `FN-21.c`'s sweep decline (which is a `NoOp`, not a block) and
  `FN-31.d`'s replacement decline (which is a block). **So `FN-25`'s partition is
  over `Blocked` outcomes and `W17` appears on a non-block**, which is exactly
  the trap the catalogue warns about in as many words: *reading `OwnershipConflict`
  onto a refusal would make the partition neither disjoint nor exhaustive over
  anything*. Do not let the `why` set's shape decide the outcome set's.
- **`FN-25` is still deliberately unwritten and `BlockedOutcome` still carries no
  diagnosis.** Four slices have now declined to extend it, each recording why.
  That accumulated abstinence is what makes `FN-25.a`'s totality, disjointness
  and exhaustiveness a *finding* rather than a construction — do not spend it.
- **The catalogue's three-context table is already half-modelled.** *A foreign
  entry at a name Grove reserves* is `Refused(ReservedNameOccupied)` before a
  transaction (`doWPrepare`, `doDiscard`), `Blocked(OwnershipConflict)` inside
  one (`doMarkerReplace`), and a sweep's decline (`doReap`). `TT-24.c`'s and
  `TT-24.d`'s content is therefore present under `FN-` prefixes, and the
  README says so; the placement question is `formal-synthesis-k16`'s and this
  leaf's job is to leave it a citation change.

**Four things `disposal-k45` measured or learned that change how this leaf works.**

- **The cost law was finally NOT pessimistic, and the reason is a shape you are
  about to add more of.** Three variants of one file in one sitting: the four
  disposal transitions plus two phases plus a scope dimension cost **+7%** on the
  widest command; the single **sweep** at a dwell phase cost **+11%** on top of
  that; un-narrowing its guard cost a further **+4%**. So a transition enabled at
  a phase a trace can *rest* in is worth roughly four pass-through transitions at
  this bound. **`FN-24`'s full step-boundary crash sweep is the same shape at a
  much larger scale** — `crash` is already enabled at every boundary, but a claim
  quantified over all sixteen `bodySteps` is the widest antecedent this file will
  have had. Budget it as a dwell claim, take the ordering (a static scope switch,
  then a narrowed antecedent, then a smaller bound), and **do not take the
  multiplier** — it has been wrong three times and right once.
- **A THIRD BOUND-VACUITY PREDICTOR, and it fires on obligations shaped exactly
  like yours.** The file's rule — *a check runs at the widest first-landing bound
  among its obligation's witnesses* — is a **floor that can sit below the real
  floor** whenever an obligation's witnesses posit a disk while its antecedent is
  a deep transition. `FN-31.c`'s witnesses land at 3 and 4 and its antecedent
  first occurs at 10; run at 4 the check would have been green and **empty**.
  Read every check's antecedent for the deepest transition it names and take the
  larger of the two numbers. `FN-23`'s *re-running recovery reaches the same
  terminal state* and `FN-28`'s single successful exit are both cheap to witness
  from a posited disk and both quantify over deep machinery.
- **A seventh bound-register entry, and it is entry 2 sharpened.** A step
  inserted at the **end** of a path costs nothing to a witness whose final
  assertion re-anchors onto it: the disposal split moved two witnesses and left
  three where they were. Sweep all seventy-nine witnesses, not only the ones you
  touch — but expect fewer movements than the arithmetic predicts, and ask of
  each whether it needs to reach **past** your insertion or only **to** it.
- **A fourth rule about mutation aim, for overlapping SUBJECTS.** Two of
  `disposal-k45`'s mutations killed `FN-22.i` alongside their targets, because
  that check had been written to assert the cleanup marker's own content — true,
  and `FN-31`'s. **When two obligations describe the same artifact from two
  directions, the one whose subject it is not should not describe it at all.**
  `FN-27`'s *nothing unrelated is mutated, on any outcome* is the widest-subject
  claim in the scope and will overlap almost every frame condition in the file;
  expect this trap and check what each mutation leaves **green**.


## Done when

- Every obligation of `FN-02` and `FN-23` – `FN-30` is answered by a `check` and
  its required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy` with **coverage asserted** and
  **zero empty alloy cells** for the finish scope — and `--no-coverage` is gone
  from `crates/grove-finish/models/README.md`'s run line.
- The two exercise-removals are present as their own named commands with the
  expected result the assumption table states: **`EN-08`** (`crash` removed —
  `FN-09`, `FN-10`, `FN-24`, `FN-31.c`, `SY-12` named witnesses become
  unreachable and the run fails on zero work rather than reporting green) and
  **`EN-16`** (the lane collapsed to one), each run against the named witness
  sets rather than the whole file. `EN-16` is what separates *the lane is a model
  parameter* from *this model is lane-blind*, and until it runs the README's own
  caveat says the claim is a property of the signature rather than a measured
  fact.
- **Q4's removal matrix is recorded in the family `README.md`** — one row per
  removable artifact or transition, each naming the **first shared-safety
  obligation** its removal breaks, or `none`. **Five rows are already decided and
  are transcribed rather than re-derived** (under *The mutation matrix*): the
  reserved witness, the evacuation manifest's ready mark, the correlation ticket
  (`FN-04`), the deletion fingerprint (`FN-14`) and the recorded anchor
  (`FN-16.a`). **Two rows are recorded as undecidable-from-here and say why** —
  the quarantine's and the cleanup marker's: both are *incumbent mechanics*, so
  every mutation that breaks them is evidence about the incumbent and about
  nothing else, and the shared-safety claims that could name them are `FN-24`'s
  and `FN-27`'s, which are yours. Read both notes before writing either row.
- Whether `models/run.sh` grows a matrix reader is **decided here and recorded**.
  The catalogue calls the matrix *a runner obligation like any other: a removable
  artifact with no row fails the run*, and `run.sh` reads the README for declared
  gaps and for nothing else. If the answer is yes, it is a leaf, not inline work.
- Every check runs at a bound at least as large as **both** the widest
  first-landing bound among its obligation's witnesses, measured by sweep over
  every witness in the file, **and** the bound at which the deepest transition its
  own antecedent names first occurs.
- One mutation per obligation, each with evidence that it fires and a note of
  what it left green.
- The family `README.md` gains the new bounds, abstractions, witness-bound rows,
  mutation-matrix rows, retained counterexamples, and the *what a green run does
  not prove* entries the closing slice can finally remove or add.
- Material observations are appended to Experiment 2 as entry 037 onward.
- **`finish-k8`'s node close is checked, not assumed.** This is the last child of
  that node; its `Done when` names coverage asserted, the Q4 matrix, the four
  assumption mutations, `FN-15.d`/`FN-31.a`'s instrument, and the per-command
  README record. Walk it.



## Decomposition

Fourteen obligations, and **four new machineries** where every sibling slice of
`finish-k8` added one or two: a stable-state classification of the disk
(`FN-24.a`), a persistent-effect enumeration over all sixteen `bodySteps`
(`FN-24.b`), the `Blocked` diagnosis partition four slices deliberately did not
build (`FN-25`), and hook suppression (`FN-30`). On top of those sit two
exercise-removals whose named witness sets span the whole file, Q4's removal
matrix, the runner's matrix-reader question, and this node's own close. That is
more than one focused session, and the cut is along **machinery** — which is how
`finish-k8` cut its own level and how `handoff-k42` cut its.

1. `crash` — `FN-24.a`, `FN-24.b` (2 obligations). Adds the **stable-state
   classification** of the disk, written as data the way `observed` and
   `tableAction` are, and the **persistent-effect enumeration** the step list is
   worth. `FN-24.a`'s antecedent quantifies over all sixteen `bodySteps` and is
   the widest this file will have had; `FN-24.b`'s job is to name the steps that
   have more than one persistent effect and declare them. Owns **`EN-08`** — the
   `crash` exercise-removal — because `FN-24` is the obligation it controls that
   this file did not already have.
2. `blocked` — `FN-25.a` – `FN-25.c`, `FN-26` (4). Adds the **diagnosis
   partition** over `Blocked` and the four model-only `why` values it has to
   absorb, plus *history is never rewritten to clear a block*. Owns **`EN-16`** —
   the lane collapsed to one — because `FN-25.c`'s per-lane witnesses are what
   the control makes visible.
3. `exits` — `FN-02`, `FN-23`, `FN-27.a` – `FN-27.c`, `FN-28` – `FN-30` (8),
   plus **Q4's removal matrix**, the runner question, and the node close. Adds
   hook suppression and the single successful exit. It is last because the matrix
   needs `FN-24` (child 1) and `FN-27` (its own), because `FN-02`'s witness is a
   decline followed by a *successful* attempt and no success exists before
   `FN-28`, and because the visible signal — `--no-coverage` leaving the README's
   run line — is the last child's by construction.

Only the first child is cut now, which is the rule `finish-k8` set: each session
cuts the next one as its last act, once the model's actual shape at that point is
known.

## Notes

**On review.** No leaf in this subtree has cut a `review-prototype` step, and the
reason has been reusable each time: the artifact is adversarially verified by one
mutation per obligation with named fire-evidence and a neighbour sweep, plus a
runner that fails on zero work and on an unnamed command. `revalidation-k44`
decided against one for `FN-22`'s table because the table is written as **data**
and a deleted row makes a check red. **This leaf has the first plausible
exception the subtree has produced**: Q4's removal matrix is prose, the runner
does not check it, and a row naming the wrong first-broken obligation reports
identically to a right one. If the matrix-reader question above is answered *no*,
say explicitly what discipline reaches the matrix instead — and if nothing does,
that is what a `review-prototype` is for.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
