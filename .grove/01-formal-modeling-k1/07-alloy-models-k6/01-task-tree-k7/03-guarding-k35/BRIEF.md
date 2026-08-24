# guarding-k35 — brief


## Goal

Extend `crates/grove-task-tree/models/task-tree.als` to the root-identity and
guarding claims `TT-17` – `TT-25`, and run the Alloy-owned `EN-` assumption
mutations that control them. This is the last subtree of `task-tree-k7`: when it
closes, the Alloy column of the task-tree scope is complete and only the Quint
column (`quint-models-k10`) is empty.


## Context

`names-k33` and `selection-k34` left the file green for `TT-01` – `TT-16` and
built `models/run.sh`. **Read `crates/grove-task-tree/models/README.md` before
writing a command** — it carries the bounds, the abstractions, the two retained
false-confidence incidents, and the mutation matrix whose discipline this leaf
continues. What exists that you can build on:

- **The filesystem, and the halting condition.** Six whole-tree reasons live in
  `halted`, five entry-local and one — `rMultipleLiveFinish` — about the tree.
  `TT-13.c` is the shape to copy for any further whole-tree classification.
- **Total transitions.** Every action returns exactly one outcome and a failed
  guard yields a named refusal with a byte-identical tree. `TT-24` is stated
  over refusals, so this is the machinery it needs; do not add an action whose
  guard failure is an *absent* transition.
- **Observation actions.** `select` and `resolve` exist, with `Sys.got` /
  `Sys.gotTerm` and the `Reported` / `Empty` / `Ambiguous` outcomes. `TT-21` and
  `TT-22` are about how many *snapshots* an operation takes and which guard it
  holds, which is a distinction the model does not draw yet.

What this leaf has to build, and none of it exists:

- **Root classification**, in a fixed order (`TT-17`, `TT-18`): the format
  witness's *content* decides, reserved-witness classification runs first, and
  `PartialScaffold` is ordered before `Legacy`. The states are in the
  catalogue's §*States*, not inventable here.
- **The reserved witness class** (`TT-19`, `TT-20`), including *the format
  witness lands last* — which needs `crash` between two filesystem steps, so
  root initialisation cannot be one step the way promotion is.
- **Guards** (`TT-21`, `TT-22`): shared for observation, exclusive for mutation,
  and one snapshot per operation. `TT-21.b` is explicitly about a
  **non-cooperating** writer, which is `EN-06`'s content and the reason `TT-21`
  cannot claim to exclude one.
- **Bulk marks** (`TT-23`), which validate before moving and converge on
  re-run after a partial application — the `bulk-marks-are-not-atomic` ADR.
- **Fail-closed ownership** (`TT-24`), the one claim whose three obligations
  are one artifact met in three contexts; the catalogue fixes the outcome of
  each and a model that lets itself choose has answered the question by
  construction.
- **Derived done-ness** (`TT-25`): a node is never marked.

The `EN-` mutations this leaf owns are `EN-04`, `EN-07`, `EN-12`, `EN-14`, and
the exercise-removals `EN-08` / `EN-11`. Their command spellings are
`expect_fail_<EN>_<OB>_<mnemonic>` (must find a counterexample) and
`expect_unreachable_<EN>_<mnemonic>` (must find none) — the runner's two
inverted forms, and `models/run.sh`'s header states them. `EN-08` removes
`crash`; `EN-11` removes `doHandEdit`, and **removing `doHandEdit` will make a
great many existing witnesses unreachable**, which is exactly its stated
control. Run those two against the named witness sets rather than the whole file.


## Done when

- Every obligation of `TT-17` – `TT-25` is answered by a `check` and its
  required `witness_` runs, all green under
  `models/run.sh --scope task-tree --family alloy --no-coverage`, and the run
  reports **zero empty alloy cells** for the task-tree scope.
- The six `EN-` mutations run with the expected result the assumption table
  states, as their own named commands.
- **One mutation per obligation added**, run before the green is believed, with
  each mutation breaking exactly the check it is aimed at — and **evidence that
  each mutation actually fires**, since `selection-k34` produced one that was
  unsatisfiable and therefore reported as a survivor. One existing witness
  re-run under the mutation is enough.
- The family `README.md` records the new bounds, any new abstraction, and the
  witness bound at which each new obligation first lands.
- Material observations are appended to Experiment 2 as entry 028.


## Decomposition

Three sessions, cut along the **machinery** each claim group needs rather than
along the claim numbers, exactly as `task-tree-k7` cut this level. Each child
leaves `task-tree.als` green for the obligations it claims and the runner able
to say which cells are still empty, so no child is dead until its siblings land.
Only the first is cut now; each session cuts the next one as its last act, once
the model's actual shape at that point is known.

1. `roots` — root identity: `TT-17` – `TT-20`. Adds the `Witness` species, the
   witness's *content* as what classification reads, the fixed classification
   order, and `crash` between two filesystem steps (which is why root
   initialisation cannot be one step the way promotion is). Owns `EN-04` and
   `EN-12`, both of which are premise/counterfactual mutations over claims that
   are already green and need no new machinery.
2. `guards` — guards and bulk marks: `TT-21` – `TT-23`. Adds concurrency: an
   operation with a duration, a shared/exclusive lock taken on the working-tree
   root, the single listing an operation classifies from, and a bulk mark's
   plan. Owns `EN-07`, `EN-14`, and `EN-08` — the last because `crash`'s removal
   controls `TT-20`'s witness *and* `TT-23.b`'s, so it runs once both exist.
3. `ownership` — fail-closed ownership and derived done-ness: `TT-24`, `TT-25`.
   Adds `Blocked`, and whatever the reserved-name-occupancy question settles to.
   Owns `EN-11`, which runs last because removing `doHandEdit` makes a great
   many of the file's witnesses unreachable and that is its stated control.

The `Notes` below are the whole subtree's, and each names the child it lands on:
the wider trace bound is `roots`' (`TT-20`) and `guards`' (`TT-23.b`); the
`Blocked` question is `ownership`'s; the antecedent-narrowing trade is every
child's.


## Notes

`TT-20` and `TT-23.b` are the first claims in this scope that need **two
consecutive** grove actions, or an action interrupted part-way. The `3 steps`
argument in the README is explicitly *not* sufficient for those: it holds only
because `EN-11` makes every single transition reachable from an unconstrained
state 0. Budget for a wider trace bound on those commands and record it.

`TT-24`'s three contexts include one — inside a finish or recovery transaction —
whose outcome is `Blocked(OwnershipConflict)`. `Blocked` is not in this model's
`Result` set. Whether the task-tree model represents it or declares it a gap
belongs to `crates/grove-finish/models/`'s scope is a real question this leaf
must answer explicitly rather than by omission; a **declared gap with a reason**
is a legitimate answer and the runner reads it from `README.md`.

Two commands in this file already cost minutes each and one costs nine. Root
classification adds a `Witness` species and an ordering over classifications;
if a command stops finishing, prefer **narrowing the antecedent** over shrinking
the bound — that trade is recorded on `TT-05`'s four commands and the reason is
that a smaller bound buys the green run at the cost of what the run was evidence
about.

Do not read the Quint side of Experiment 2. The independence protocol holds
until both families are green.
