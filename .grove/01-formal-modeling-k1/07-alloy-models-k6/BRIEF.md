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


## What the finish subtree established, and what `system-k9` inherits

`finish-k8` closed with the Alloy column of the finish scope complete — `FN-01` –
`FN-31`, **180 commands, 61 of 61 cells, coverage asserted**, and
`--no-coverage` gone from `crates/grove-finish/models/README.md`'s run line.
Eight slices, nine sessions, and the four things below are this node's rather
than that subtree's. Everything the task-tree section above carries still
applies unchanged and none of it is restated.

**A design constraint, not a modelling one — and `SY-05` is where it lands.**
`FN-28` says *a finish succeeds exactly when the exact attempt-bound commit is
proven and the task root is absent*, and the second operand is **not a fact the
protocol can hold**: after the quarantine rename the task-root name is free, the
world can occupy it, and it can then give what it put there the quarantined
root's own identity. Three formulations of the claim were falsified by that one
trace. **The only durable evidence a finish succeeded is the correlation
ticket** — a name is not an artifact and the world owns the namespace. `SY-05`
makes *absent root* a first-class lifecycle state; model it as something Grove
establishes and preserves, never as something that holds, or the lifecycle model
will re-derive this at its own cost. Recorded in `docs/formalism-findings.md`
entry 039 and in the family README under *A tenth finding*; no ADR was written,
because settling it is `formal-synthesis-k16`'s.

**The `TT-24` placement disagreement now has THREE consequences, not two.**
`TT-24.c` and `TT-24.d` remain declared gaps in the task-tree model and are
unfillable from either directory; the finish model now states both contents
under `FN-` prefixes (`FN-25` for the in-transaction `Blocked(OwnershipConflict)`,
`FN-21.c` for the reaper's decline), so a re-statement would be a citation change
and nothing more. The third: **Q4's removal matrix has a row whose first-broken
obligation is `TT-24`**, cited from a directory in which no `TT_`-prefixed
command may live. `system-k9` should expect the same shape wherever an `SY-`
claim's content is really a `TT-` or `FN-` one; `formal-synthesis-k16` inherits
all three.

**Two more entries in the bound register, and the second sharpens a rule this
node already carried.**

- **A shape claim under `EN-11` must be restated over the protocol's own steps**
  — the task-tree section's rule — was met **four** more times in the finish
  subtree, at four different granularities: a free initial state, a world
  transition, a claim's own subject, and the operand of a definition. It is the
  single most productive rule in this corpus and it is cheap to apply before the
  fact: **ask of every conjunct whether state 0 or the world could hand-edit it
  false.**
- **A `var` field that adds only free choice owes a proof and a control; a `var`
  field any guard, fact or existing command reads owes the full witness sweep.**
  `exits-k49` added `World.hookRan`, framed by twenty-eight of twenty-nine
  transitions and read by no guard, and discharged the sweep with a
  **monotonicity argument** — every pre-existing instance extends by setting the
  field absent throughout, so no first-landing bound can rise — plus an
  eight-witness control. That replaces the crash slice's blanket rule.

**And a cost correction: a `var` field is NOT static structure.** The static-atom
law measured twice in the finish subtree is uniform — about **10 ms of
translation per atom per command** — so budget static signatures by counting
atoms and stop reading percentages. A `var` field does not obey it: it adds a
boolean per state, its cost tracks trace length rather than a command's
difficulty, and a four-sentinel A/B gave −6.8%, +17.5%, +3.7% and −3.3% with the
**dearest** command moving least. Budget one at a few per cent of the whole file
and measure rather than predict.

**Six ways for a mutation to fail its aim, and the sixth is new in kind.** The
register is in `crates/grove-finish/models/README.md` under *The mutation
matrix* and is worth reading once before writing a control. The sixth: **a claim
every one of whose conjuncts is another claim's subject has no isolating
mutation** — a property of the check set rather than of the model, and the
honest record is the neighbour list rather than a fourth attempt.

**One leaf was cut out of this subtree and it is not under this node.**
`matrix-reader-k50` teaches `models/run.sh` to read each family's Q4 removal
matrix; it sits under `formal-modeling-k1` ahead of `quint-models-k10`, because
the runner is shared by four scopes and two families and the Quint column will
owe a matrix of its own. `system-k9` needs nothing from it.
