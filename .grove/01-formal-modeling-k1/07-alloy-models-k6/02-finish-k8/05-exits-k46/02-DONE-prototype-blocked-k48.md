# blocked-k48


## Goal

Answer `FN-25.a` – `FN-25.c` and `FN-26` in
`crates/grove-finish/models/finish.als` — the closed partition of `Blocked` over
`RecoveryPending` and `OwnershipConflict`, each diagnosis reachable on each lane,
and *history is never rewritten to clear a block*. Own **`EN-16`**, the lane
collapsed to one. Leave the file green under
`models/run.sh --scope finish --family alloy --no-coverage` with **eight** empty
alloy cells rather than twelve.


## Context

Read the node brief above and `crates/grove-finish/models/README.md` first.
`crash-k47` left the file green at **147 commands, 10 m 33 s, 49 of 61 alloy
cells**, and the twelve empty ones are this leaf's four and the third child's
eight.

**Four slices' abstinence is the whole value of `FN-25`, and it is spendable
exactly once.** `BlockedOutcome` still carries no diagnosis. `commit-k41`,
`quarantine-k43`, `revalidation-k44` and `disposal-k45` each reached a condition
the catalogue diagnoses and each recorded, in as many words, why it gave the
condition a model-only `Sys.why` rather than extending the outcome: a slice that
named the partition would answer `FN-25.a`'s totality, disjointness and
exhaustiveness **by construction**, which is the false-confidence shape rather
than a finding. That accumulated abstinence is what makes the claim a finding
now. Do not spend it by encoding the partition in the signature.

**Four model-only `why` values the partition has to absorb, and one of them
appears on a non-block.** `W14QuarantineOccupied`, `W15CommittedAfterRestore` and
`W16ReturnIncomplete` are `Blocked` branches the catalogue diagnoses
`RecoveryPending`. `W17OwnershipConflict` is the only one the **catalogue itself
names**, and it serves two gates: `FN-31.d`'s replacement decline, which is a
block, and `FN-21.c`'s sweep decline, which is a **`NoOp`**. The catalogue warns
in as many words that reading `OwnershipConflict` onto a refusal would make the
partition neither disjoint nor exhaustive over anything. **The `why` set's shape
is not the outcome set's** — state `FN-25` over `Blocked` outcomes and nothing
else, and let the reaper's `NoOp` sit outside it.

**What `crash-k47` learned that bears directly on this leaf.**

- **Write the partition as DATA, apart from every transition, and let a check ask
  whether it is total and unambiguous.** That is the move that produced both of
  the crash slice's catalogue findings, it costs **no state** — `fun`s over static
  atoms — and it is exactly the shape `FN-25.a` and `.b` need: a mapping made
  disjoint by a `lone` field or by an outcome split answers the claim by
  construction, in the same way a classification made disjoint by its own guards
  would have.
- **A mutation to one row of a total order is not a mutation to the order** — the
  crash slice's fourth mutation-aim rule. A partition stated as data has the same
  hazard in its own form: check what a mutation to one arm leaves standing, and
  prefer a mutation that offers the claim a whole **alternative** partition.
- **Static atoms cost a roughly CONSTANT amount per command, not a percentage.**
  The crash slice added sixteen and measured **+8% / +9% on the two tight
  sentinels and +1% on the widest** — the inverse of every slice before it. If
  this leaf likewise adds no transition and no `var` field, budget it at
  approximately nothing and re-measure the **tight** sentinels rather than only
  the wide ones.
- **The inherited-bound sweep can be argued rather than run — but only if you add
  no `var` field, no transition and no `fact`.** `crash-k47` states that argument
  and runs an eight-witness control behind it. `FN-25.c` may need something the
  argument does not cover; read the note above the witness-bound table before
  deciding.

**`FN-25.c` is where the cost is, and it is per-lane witnesses at depth.** Each
diagnosis on each lane is six commands, and every `Blocked` state in this file is
reached by running the protocol — `fact TransactionsStartWhereAProcessStarts`
confines state 0 to `Fresh + Opened`, so no block can be posited. `W17`'s only
blocking gate is `doMarkerReplace` against a foreign marker, which
`witness_FN_31d` reaches at **10**. Expect three witnesses near there and three
cheaper ones.

**`FN-26` is stated over recorded history and the file already carries the rule
it needs.** `FN_03`'s first conjunct was narrowed to Grove's own steps after the
`doSwap` counterexample — *a claim about what a protocol never does is never a
claim about what the world never does* — and `doCommitMoves` is the world taking
a ticket back out of history. Read the *fourth finding* section of the family
`README.md` before writing `FN_26`'s antecedent; the over-stated form of that
conjunct made two catalogue rows unreachable for two slices.

**`EN-16` is the control that separates *the lane is a model parameter* from
*this model is lane-blind*.** The assumption table's named set is `FN-15.b`,
`FN-15.c`, `FN-15.d`, `FN-17` and `FN-25.c`, and the expected result is that
`FN-25.c`'s per-lane witnesses become unreachable and `FN-17`'s
working-copy-as-commit obligation has no instance **while every `FN-` property
stays green** — which is what makes the collapse invisible without the control.
Run it against those named witness sets, not the whole file. `crash-k47` found
that `EN-08`'s named set overstated its reach by one obligation; ask the same
question of this one and record the answer either way.


## Done when

- `FN-25.a`, `FN-25.b`, `FN-25.c` and `FN-26` are each answered by a `check` and
  its required `witness_` runs, all green under `models/run.sh --scope finish
  --family alloy --no-coverage`, and the runner reports **eight** empty alloy
  cells for the finish scope.
- **The partition is stated over `Blocked` outcomes and nothing else**, and the
  reaper's `NoOp` carrying `W17OwnershipConflict` is explicitly outside it — with
  `README.md` saying so, because that is the trap the catalogue names by hand.
- `FN-25.a`'s witness is *a state that nearly satisfies both* — a Grove-owned,
  correlated artifact at a name Grove also reserves — resolved to exactly one.
- **`EN-16` is present as its own named command** with the expected result the
  assumption table states, run against the named witness sets. If its named set
  overstates or understates its reach, that is a finding and is recorded.
- One mutation per obligation, each with evidence that it fires and a note of
  what it left **green**.
- Every check runs at a bound at least as large as both its obligation's widest
  first-landing witness bound and the bound at which the deepest transition its
  own antecedent names first occurs.
- `README.md` gains the new bounds, abstractions, witness-bound rows, mutation
  rows, retained counterexamples and *what a green run does not prove* entries;
  the inherited-bound sweep is either run or argued, and if argued the argument
  is stated and controlled.
- Material observations are appended to Experiment 2 as entries 038 onward.
- The third child of `exits-k46` is cut as this session's last act, its body
  carrying what this session learned that the node brief could not state in
  advance.


## Notes

No `review-prototype` step is cut by default; the node brief records why, and
where the subtree's first plausible exception is (Q4's prose matrix, which is the
third child's, not this one's).

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
