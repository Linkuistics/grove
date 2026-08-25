# alloy-models-k6 — brief


## Goal

Build an independent Alloy 6 behavioural account of the shared claims using mutable relational state and temporal operators.



## Context

Follow `model-contract-k5`. Alloy 6 is not limited to static structure: model transitions with `var` relations, primed state, temporal formulas, and lasso traces where appropriate. Do not inspect the Quint implementation before Alloy's three models are complete and green.

## Done when

- Runnable `.als` models exist for task tree, finish/recovery, and end-to-end lifecycle at the agreed component/system paths.
- Every claim has a named assertion/check or an explicitly documented reason it is not represented; every important state/action has a satisfiable witness.
- Commands pin tool/solver details, use meaningful scopes and trace bounds, and fail when the runner, assertion set, or witnesses silently execute zero work.
- Counterexample traces and resulting claim/design changes are retained in compact, reproducible form and logged in the formalism experiment.

## Notes

Record symmetry, exact-scope, liveness/fairness, and boundedness caveats. A successful bounded check is evidence about the stated bounds, not proof about arbitrary executions.


## What the task-tree subtree established, and what the remaining two inherit

`task-tree-k7` closed with the Alloy column of the task-tree scope complete
(`TT-01` – `TT-25`, 103 commands). Four things it settled are this node's, not
that subtree's, and `finish-k8` and `system-k9` should not rediscover them.

**How a half-built phase is run green.** `models/run.sh --scope <scope> --family
<family>` — a **named subset that still asserts coverage**, not an expected-red
whole run. The rule and its reason now live in
`docs/specs/semantic-contract.md` §*Model paths and the runner*; `--no-coverage`
is what a scope uses while its own first family is mid-build, and dropping it
from a model README's run line is the visible signal that a column closed.

**Two of `TT-24`'s four obligations are declared gaps in the task-tree model, and
they are `finish-k8`'s to answer or to leave declared.** `TT-24.c`'s outcome is
`Blocked(OwnershipConflict)` and its antecedent is *inside a finish or recovery
transaction*; `TT-24.d`'s subject is the quarantine reaper (`FN-21`). Both are
`TT-`-prefixed obligations stated over `FN-` contexts, which is the first place
the catalogue's claim numbering and the runner's placement rule disagree —
`formal-synthesis-k16` inherits whether they should be re-stated as `FN-`.
A finish model that represents `Blocked` and the reaper can express what the
task-tree model cannot, and the two gaps are where to look for the cells.

**The cost model, now at three points — and how to measure it.** Four reachable
transitions cost the whole file +41% CPU; five behind a **static** switch cost
+10%; a slice adding **no** transition but two `lone` fields and an outcome atom
costs ~+15% overall and **+48% on the file's widest command**. Two measurement
rules came out of that last one and both are new:

- **One sentinel is not enough.** `TT-03` is the file's *tightest* command, which
  makes it sensitive to new transitions and nearly blind to new **state** — it
  went *down* 156s → 138s in the same slice that made `witness_TT_07` 48% dearer.
  Measure the largest command as well as the tightest.
- **Whole-suite totals do not compare across sessions.** `TT-11`, untouched,
  costs 61s in one session's figures and 77s in another's on the *same file*. A
  slice's imposition is an **A/B on one host in one sitting**, and the absolute
  numbers in a model README carry that caveat.

Budgeting still goes by counting transitions, preferring in order: a static scope
switch, a narrowed antecedent, a smaller bound. One refinement to the switch
rule: **pin a switch that deletes state, leave free a switch that admits an
action** — pinning `EN-11` (which enables `hand-edit`) made the sentinel 50%
slower, where pinning `EN-12` (which empties a relation) made the file cheaper.

**Two predictors for bound vacuity, both cheap to apply before the fact.** An
**interval** claim needs interval-many *states*; and the bound must hold the
**machinery of the transitions the obligation quantifies over**, not only the
objects the obligation names. Five of the file's nine incidents were a check
written too narrow, so these are the two questions to ask while writing a
command rather than after a mutation survives.
