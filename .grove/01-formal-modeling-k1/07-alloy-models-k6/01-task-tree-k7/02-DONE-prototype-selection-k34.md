# selection-k34


## Goal

Extend `crates/grove-task-tree/models/task-tree.als` to the selection claims
`TT-11` – `TT-16`: the pre-order walk, terminality, the reserved finish leaf,
and the empty/ambiguous observation outcomes.


## Context

`names-k33` left the file green for `TT-01` – `TT-10` and built
`models/run.sh`. What it did **not** build, and what this leaf needs:

- **The walk itself.** Pre-order needs no rank relation: `a` precedes `b` iff
  `a` is an ancestor of `b`, or some `x` in `a.*loc` and `y` in `b.*loc` share a
  parent with `x.nm.fPos < y.nm.fPos`. Selection is then the `precedes`-minimal
  live leaf, and `TT-14` — *selection is not a scheduler* — is the claim that
  nothing else enters that definition.
- **Observation actions and their outcomes.** `TT-15` needs `Empty` and
  `Ambiguous` as **successes**, not refusals, and the model's `Result` set does
  not carry them yet. `TT-16` needs `resolve` to report terminality alongside
  the entry.
- **`FinishK` already exists** in the kind vocabulary but nothing uses it;
  `TT-13` is where it earns its place, including `Malformed(MultipleLiveFinish)`
  as a **whole-tree** reason, which `halted` must gain.

Read `crates/grove-task-tree/models/README.md` before writing a command: it
carries the bounds, the abstractions, and the retained false-confidence
incident. Read the model's own `CLAIMS` header for why every behavioural command
runs at `3 steps` and not `2`.


## Done when

- Every obligation of `TT-11` – `TT-16` is answered by a `check` and its
  required `witness_` runs, all green under
  `models/run.sh --scope task-tree --family alloy --no-coverage`.
- **One mutation per obligation added**, run before the green is believed, with
  each mutation breaking exactly the check it is aimed at. This is not optional
  here: `names-k33` reported a fully green suite with every witness landing while
  checking nothing at all, and mutation is the only control that separated the
  two.
- The family `README.md` records the new bounds, any new abstraction, and the
  witness bound at which each new obligation first lands.
- Material observations are appended to Experiment 2 as entry 027.
- The `guarding` leaf (`TT-17` – `TT-25`, plus the Alloy-owned `EN-` mutations)
  is cut as a sibling.


## Notes

`TT-13.c` classifies the **tree** rather than either entry, so it needs a
whole-tree reason and not an entry-local one — the same shape `rKeyReissued`
already has, not the shape `rSpeciesMismatch` has.

`TT-11`'s "depends on no state outside the tree" is a claim a model can only
make by construction: state the walk as a function of `loc`, `nm` and nothing
else, and say so in the README rather than pretending a command checks it.


## Decisions (running log)

- **The walk is a `precedes` predicate, not a rank relation.** `a` precedes `b`
  iff `a` is an ancestor of `b`, or ancestor-or-self of each are siblings ordered
  by `fPos`. `selected` is the `precedes`-minimum of `eligible`, and `eligible`
  is `liveOrdinary` unless it is empty, in which case `liveFinish`. No new state.
- **`TT-11`'s "depends on no state outside the tree" is answered by construction
  and no command claims to check it.** Recorded under its own heading in the
  family `README.md`. A model cannot check the absence of a variable it does not
  have; what *is* checked is `TT-14`, which names `fPos` and so survives a
  `precedes` that has been taught a scheduler.
- **Observation commands run at `2 steps`.** The lasso argument that forces
  mutations to `3` does not apply to a read: the state a read reaches loops to
  itself. Verified, not assumed — `witness_TT_09a_append` finds no instance at
  `2 steps` while every observation witness does. `TT-12` and `TT-13.c`'s checks
  quantify over mutations and stay at `3`; `witness_TT_14` needs `4`.
- **What an observation reported is modelled as state** (`Sys.got`,
  `Sys.gotTerm`) rather than derived from the tree. A derived terminality could
  not be got wrong, and `TT-16` is exactly the claim that the report carries it.
- **A resolution's reference is `one sig Query`** — an optional key and an
  optional slug, one atom for the whole trace. Every command needs at most one
  `resolve`, and one atom leaves the existing `TT-01` – `TT-10` scopes untouched.
- **`MultipleLiveFinish` is a whole-tree halting reason** (`rMultipleLiveFinish`
  in `halted`), the shape `rKeyReissued` has and not the shape
  `rSpeciesMismatch` has, per `TT-13.c`.
- **A mutation is a control only with evidence that it fires.** `TT-12`'s first
  mutation was unsatisfiable and therefore reported as a survivor; the rule added
  to the family `README.md` is to re-run one existing witness under the mutation.
- **The `guarding` sibling is `03-prototype-guarding-k35.md`**, carrying
  `TT-17` – `TT-25` and the Alloy-owned `EN-` mutations.
