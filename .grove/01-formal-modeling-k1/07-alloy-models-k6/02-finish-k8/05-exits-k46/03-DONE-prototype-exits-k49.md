# exits-k49

## Goal

Close the alloy column of the finish scope. Answer `FN-02`, `FN-23`,
`FN-27.a` – `FN-27.c`, `FN-28`, `FN-29` and `FN-30` in
`crates/grove-finish/models/finish.als` — the single successful exit, hook
suppression, *nothing unrelated is mutated on any outcome*, a refusal as a
complete outcome, and recovery's idempotence — eight obligations, the last eight
empty cells. Write **Q4's artifact/transition removal matrix**. Decide and record
whether `models/run.sh` grows a matrix reader. Leave the file green under
`models/run.sh --scope finish --family alloy` with **coverage asserted**, zero
empty alloy cells, and **`--no-coverage` gone from the family `README.md`'s run
line** — which is the visible signal that the column closed and is this leaf's by
construction. Then walk `finish-k8`'s node close.


## Context

Read the node brief above and `crates/grove-finish/models/README.md` first.
`blocked-k48` left the file green at **164 commands, 12 m 08 s, 53 of 61 alloy
cells**, and the eight empty ones are all yours.

**What the last two slices settled that this one inherits.**

- **The partition exists, as data, and `FN-27` is the only shared-safety claim
  left that can move Q4's two undecided rows.** `crash-k47` gave the file a
  stable-state classification and a persistent-effect enumeration; `blocked-k48`
  gave it `Diagnosis`, `BlockField`, a two-element precedence and six clause
  predicates, none of them read by a guard. Five removal-matrix rows are decided
  and transcribed under *The mutation matrix*; two — the quarantine's and the
  cleanup marker's — are recorded as undecidable-from-there **with the reason**,
  and `blocked-k48` added a line to each saying why `FN-25` and `FN-26` do not
  move them. `FN-27` is the widest-subject claim in the scope and is the last
  chance either row has.
- **`FN-27` WILL OVERLAP ALMOST EVERY FRAME CONDITION IN THE FILE**, which the
  node brief predicted and two slices have now confirmed at a smaller scale.
  `blocked-k48`'s four mutations produced **two neighbour kills**, both because a
  conjunct was written *about* the predicate the mutation edited. Expect that at
  `FN-27`'s scale and check what each mutation leaves **green** rather than only
  what it kills.
- **A sixth vacuity grain, and it is not about a bound.** Every `FN-25` and
  `FN-26` command reads its subject **unprimed against a primed `Sys.res'`**,
  because `doSettle`, `doRevalidate` and `doQuarReturn` block through `txnGone`
  and the state a block LANDS in has no attempt identity — at which point
  `resultProven`'s `Txn.attempt in ticketedAttempts` reads `none in ...` and is
  vacuously **true**, and every block classifies alike, greenly. `FN-28`'s *a
  finish succeeds exactly when the exact attempt-bound commit is proven and the
  task root is absent* reads exactly the same operands, and `FN-29`'s *`NotCommitted`
  leaves the grove exactly as it was* is stated over a state a refusal produces.
  **Ask of each: is the state my claim reads still holding the operands it
  names?** Four antecedents were probed for reachability in `blocked-k48` and two
  would otherwise have been vacuous.
- **A fifth way for a mutation to miss its aim.** A mutation aimed at a
  transition the claim's antecedent cannot reach *after* the antecedent holds
  reports exactly as a survivor. `FN-26`'s first mutation freed `doRecover`'s
  repository frame and `doRecover` guards on `Entered`, which a block never
  leaves. Aim a mutation for a claim stated ACROSS a state boundary at a
  transition inside the antecedent.
- **Every witness whose subject is an OUTCOME costs the run-up and nothing
  else.** All ten of `blocked-k48`'s land at **nine** — not one at eight and not
  one at ten — because no block can be posited (`fact
  TransactionsStartWhereAProcessStarts` confines state 0 to `Fresh + Opened`) and
  the route from `interruptedMidEvacuation` to any blocking gate is the same four
  steps. `FN-28`'s and `FN-29`'s witnesses are the same shape and should be
  budgeted the same way; `FN-02`'s — *a decline followed by a later successful
  attempt* — is the one that is not, because it needs two attempts.
- **Static structure still costs approximately nothing, twice measured.** Two
  consecutive slices have added no transition, no `var` field, no `fact` and no
  scope dimension, and both were entitled to `crash-k47`'s inherited-bound
  ARGUMENT rather than a full sweep, each running a small control behind it.
  **`FN-30`'s hook suppression is the first thing this node adds that may need a
  `var` field**; if it does, the argument lapses and the full witness sweep is
  owed.

**What is still open and is yours.**

- **The runner does not read the removal matrix**, and the catalogue calls the
  matrix *a runner obligation like any other: a removable artifact with no row
  fails the run*. Decide it, record the decision, and cut a leaf if the answer is
  yes. The node brief's *On review* note turns on this: the matrix is prose, a row
  naming the wrong first-broken obligation reports identically to a right one, and
  **if the answer is no, say explicitly what discipline reaches the matrix
  instead** — and if nothing does, that is what a `review-prototype` is for. It is
  the first plausible exception this subtree has produced and it is still open.
- **`EN-08`'s named set overstates its reach by one obligation and `EN-16`'s is
  exact.** Both answers are recorded. No assumption control is owed by this leaf;
  what is owed is the node close, which checks that all four of `finish-k8`'s are
  present with the expected results the assumption table states.
- **`TT-24.c` and `TT-24.d` are still unfillable from either directory**, and
  `blocked-k48` left the answer a citation change: `FN-25` now states
  `Blocked(OwnershipConflict)` inside a transaction under an `FN-` prefix, and
  `FN-21.c` states the reaper's decline. Say so in the family README if it does
  not already read that way; the placement decision itself is
  `formal-synthesis-k16`'s.
- **Three catalogue findings are unpromoted.** `blocked-k48` found that
  §*Outcomes* states one condition under both diagnoses, that
  `RecoveryPending`'s third sentence is false of two rows of its own table, and
  that `OwnershipConflict`'s first clause is a general sentence with examples
  the model needed the sentence of. All three are recorded as evidence in the
  family README and Experiment 2 entry 038, deliberately — they are
  `formal-synthesis-k16`'s to settle and no ADR was written. Do not re-derive
  them; do check whether `FN-27`'s frame conditions make a fourth.


## Done when

- `FN-02`, `FN-23`, `FN-27.a` – `FN-27.c`, `FN-28`, `FN-29` and `FN-30` are each
  answered by a `check` and its required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy` with **coverage asserted** and
  **zero empty alloy cells** for the finish scope.
- **`--no-coverage` is gone from `crates/grove-finish/models/README.md`'s run
  line.** That edit is the visible signal that the column closed.
- **Q4's removal matrix is recorded in the family `README.md`** for the alloy
  family — one row per removable artifact or transition, each naming the first
  **shared-safety** obligation its removal breaks, or `none`. Five rows are
  transcribed rather than re-derived; the two recorded as undecidable carry notes
  saying why, and both notes have been extended once already. Read them before
  writing either row.
- Whether `models/run.sh` grows a matrix reader is **decided and recorded**, and
  is a leaf rather than inline work if the answer is yes. If the answer is no,
  what discipline reaches the matrix instead is stated — or a `review-prototype`
  is cut.
- Every check runs at a bound at least as large as both the widest first-landing
  bound among its obligation's witnesses and the bound at which the deepest
  transition its own antecedent names first occurs. The inherited-bound sweep is
  run, or argued on `crash-k47`'s argument with a control behind it — and the
  argument lapses if this slice adds a `var` field, a transition or a `fact`.
- One mutation per obligation, each with evidence that it fires and a note of
  what it left **green**. `FN-27`'s overlap with the file's frame conditions is
  the expected trap.
- The family `README.md` gains the new bounds, abstractions, witness-bound rows,
  mutation rows, retained counterexamples, and the *what a green run does not
  prove* entries the closing slice can finally add or remove.
- Material observations are appended to Experiment 2 as entries 039 onward.
- **`exits-k46`'s node close and then `finish-k8`'s are both checked, not
  assumed.** This is the last child of `exits-k46`, which is the last child of
  `finish-k8`, so the cascade runs two levels. `finish-k8`'s `Done when` names
  coverage asserted, the Q4 matrix, the four assumption mutations
  (`EN-02`, `EN-08`, `EN-09`, `EN-16` — all four now present), `FN-15.d`'s and
  `FN-31.a`'s instrument, and the per-command README record. Walk both.


## Notes

No `review-prototype` step is cut by default; the node brief records why, and
this leaf holds the subtree's one plausible exception — Q4's prose matrix. The
decision is yours and is charged above.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
