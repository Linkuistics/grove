# finish-k12


## Goal

Model finish execution, interruption, and recovery as explicit Quint actions and outcomes.



## Context

Cover the same shared contract as the Alloy finish model but derive the Quint state/action design independently. The environment must be able to fail, restart, race ownership, and expose Git/native-jj/colocated-jj differences.

## Done when

- Typed state covers confirmation, attempt identity, correlation ticket, witness/evacuation/quarantine, `.grove`, VCS lane, owned and unrelated repository state, and both terminal exits.
- Actions cover each protocol boundary plus injected failure, restart, recovery, ownership ambiguity, classified refusal, preserve, merge, and owned cleanup.
- Invariants and scenarios test no mutation without proof, evacuation before root deletion, persistent recovery correlation, monotonic evidence, idempotent recovery, correct error taxonomy, and common outcomes across all VCS lanes.
- Seeds/traces reproduce every counterexample; verification limits and fairness/environment assumptions are explicit.
- Material observations and implementation-test candidates are appended to Experiment 2.

## Notes

A helper that makes an unsafe state unconstructable is useful only if the real environment is constrained the same way. Keep environmental nondeterminism visible.

## Decisions (running log)

**The model is a protocol step machine, not a filesystem.** `task-tree-k11`
sits above the `ordinal-fs-tree` seam and models a tree; this leaf's subject is
a *transaction*, and what every `FN-` claim is about is the order of its steps,
what each step makes persistent, and what a crash between two of them leaves.
So the state here is the transaction's own — a program counter over an
enumerated step list, the witness, the manifest, the quarantine, the repository
anchor and the commit history — and the task tree is abstracted to the set of
entries the transaction evacuates. Growing a second tree model would be the
thing `task-tree-k11`'s own instruction forbids, one scope over.

**`FN-24.b` is why the step list is explicit rather than implicit.** The
obligation demands that every step have at most one persistent effect and that
the effect be a same-directory rename (`EN-01`) or be decomposed, with anything
else *declared*. A model whose steps are implicit in its actions cannot answer
that at all. So `Step` is a closed type, `persistentEffect: Step -> Effect` is
total, and the obligation is checked over the enumeration rather than over a
trace.

**The search dial is the price of an executable model of a twenty-step
transaction, and it is a dial on the SEARCH rather than on the model.** While
the transaction runs, every environment action is enabled at every step, so an
unfocused simulation reaches the end with probability `(1/k)^20` and every deep
witness would be reported green on a trace that never landed — the
pre-registration's *vacuous invariant* hazard in the form an executable model
takes. So `base` checks every PROPERTY unfocused with an environment budget no
trace can spend, and twelve `scenario_` instances carry the WITNESSES, each
capping how many environment actions a trace may take, at which program points,
and of which kind. A budget of one turned out to be the sweet spot: the world
acts at a roughly uniform point of the march and the rest is deterministic, so
every crash boundary and every revalidation point lands in a few percent of
traces. Four scenarios exist for the witnesses that genuinely need two
perturbations.

**Eight MODEL mutations, not two.** `task-tree-k11` needed two dials on the
model itself (`ONE_SNAPSHOT`, `BULK_TARGET_IDEMPOTENT`) for claims that were
otherwise true by construction. This scope needs eight, and the count is itself
an observation: an executable model of a protocol satisfies most of its own
ordering claims by being written in that order. `mutant_short_preflight` kills
four obligations at once — `FN-06`, `FN-07`, `FN-08`, `FN-12.b` all rest on the
ORDER the preflight happens to check things in — and four obligations resting on
one coding habit is worth knowing before any of them is cited as evidence.

**The two blocked diagnoses are classified from the state, independently of the
ten call sites that produce blocks, and `FN-25.b` is the AGREEMENT between
them.** A model whose `blockNow` simply writes the right word satisfies the
partition by definition. Here `diagnose` is a predicate over the world and the
transaction, `blockNow` derives the diagnosis from it, and what the obligation
checks is that the classifier is total over the blocked states — a blocked state
the predicates do not cover is a counterexample. It found one immediately: a
mid-transaction root swap blocks with a diagnosis the catalogue's partition did
not cover, and the resolution (the artifact now sits inside a root Grove did not
pin, so Grove cannot prove it is its own) is `OwnershipConflict`'s own
definition rather than an extension of it.

**Model checking is measured, not assumed — and the measurement is better than
`task-tree-k11`'s and smaller than it sounds.** `quint verify` completes on the
reduced `verify_small` instance and returns a verdict: `--max-steps=3` over
`FN-24.a`, `FN-25.a` and `FN-25.b` finished in 373s with no violation, where the
task-tree column could not finish depth 3 at all. The reason is the same one
that makes this suite's *run* an order of magnitude cheaper: this model has no
tree walk to unroll. What the result is not is a green over `base`'s world —
the incumbent protocol's shortest path from entry to a settled refusal is eleven
steps, so a depth-3 check reaches the published witness and stops. Both halves
are in the README's `VERIFY` line, because quoting "model-checked, no
counterexample" without the depth beside it is the *scope trap* stated as a
result.

**A `review-prototype` chain was cut, and inserted ahead of the sibling model
leaf rather than appended.** `cross-model-replay-k15` already reads this model
adversarially and re-derives every finding, so a review that only re-read the
claims would duplicate it. What replay will not read is the two places where a
false green and a true one produce identical output: the search dial and its
twelve `scenario_` instances, where a witness's ghost may be set by the
scenario's own construction rather than by the protocol; and the eight model
mutations, where "the obligation was true by construction" and "I mutated until
it died" look the same from the runner. Inserted rather than appended for the
timing reason `task-tree-k11`'s was: appending would put the review after
`prototype-system-k13`, which inherits the search-dial idiom.

**Two invariants were caught stated over LIVE state rather than over the moment
they are about, and the distinction is the model's own subject.** `FN-04` and
`FN-15.b` were first written as "if the disposition is `Committed` then the
ticket is present" — which an operator dropping the result *after* the
classification falsifies. That is a claim about what the world may do, not about
what Grove concluded, and it is precisely the situation `FN-22`'s four
revalidation points exist for: reading it as a violation would make the model
contradict the claim it is modelling. Both are now recorded at the moment a
disposition is formed. Five other obligations were restated the same way for the
same reason (`FN-06`, `FN-07`, `FN-08`, `FN-12.b`, `FN-16`), and the
`mutant_short_preflight` control exists so that the restatement did not quietly
make four of them unfalsifiable.
