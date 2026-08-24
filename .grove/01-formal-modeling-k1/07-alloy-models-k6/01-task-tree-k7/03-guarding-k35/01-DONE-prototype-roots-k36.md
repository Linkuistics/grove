# roots-k36


## Goal

Extend `crates/grove-task-tree/models/task-tree.als` to the **root-identity**
obligations `TT-17` – `TT-20`, and run the two Alloy-owned `EN-` mutations this
leaf holds: `EN-04` and `EN-12`. The model stays green for `TT-01` – `TT-16`
throughout; when this leaf retires, `guards` and `ownership` are the only empty
Alloy cells left in the task-tree scope.


## Context

**Read `crates/grove-task-tree/models/README.md` before writing a command.** It
carries the bounds and their arguments, the abstraction table, the two retained
false-confidence incidents, and the mutation matrix whose discipline this leaf
continues. The subtree brief above says what the whole of `guarding-k35` owes;
this leaf owes the first third of it.

What exists and is green: the filesystem facts, the parse trichotomy, the walk,
six whole-tree halting reasons, total transitions with named refusals, and the
two observations. What none of it has is any notion of the **task root's own
identity**: `TaskRoot` is a bare `one sig` that is always on disk and carries no
name, no content and no classification.

What this leaf has to build:

- **A `Witness` species.** The format witness and the reserved witnesses are
  entries the walk must *not* read as work, and the format witness's **content**
  — not its presence, and not any task entry's text — is what `TT-17` says the
  classification depends on. That needs the witness to carry a value the model
  can vary independently of every name in the tree.
- **A classification, in a fixed order** (`TT-18`): reserved-witness first,
  then format, then walk-derived — with `PartialScaffold` ordered before
  `Legacy`. The states are the catalogue's §*States* table and are not
  inventable here; `PartialScaffold`'s definition is the exact closed subset
  the catalogue spells out, not "present and witnessless".
- **The reserved-witness refusal** (`TT-19`): while any reserved witness exists,
  every observation and mutation except the matching recovery refuses, naming
  the witness and the recovering operation. `WitnessPending(class)` is the
  refusal reason, and it is not in this file's `Refused` set yet.
- **`crash`, and a root initialisation that is more than one step** (`TT-20`).
  The format witness lands last, by an atomic same-directory rename, so an
  interruption before it lands leaves a `PartialScaffold`. Promotion is modelled
  as one step; root initialisation *cannot* be, because the claim is precisely
  about the state between two of its filesystem steps.

`EN-04` (counterfactual-capability: promotion made atomic) and `EN-12`
(premise-break: a name that renders as more than one path component) both bear
on claims that are already green and need no new machinery — they are here
because they are cheap where the model already is, not because `TT-17` – `TT-20`
depend on them. `EN-04`'s expected result is that `TT-07`, `TT-08`, `TT-09` stay
green and `TT-02.b`'s witness still lands by hand edit; `EN-12`'s is that
`TT-01.a` fails.


## Done when

- `TT-17`, `TT-18`, `TT-19` and `TT-20` each have a `check` and the `witness_`
  run the catalogue names, all green under
  `models/run.sh --scope task-tree --family alloy --no-coverage`, with
  `TT-01` – `TT-16` still green beside them.
- `EN-04` and `EN-12` run as their own named commands in the runner's two
  inverted forms (`expect_fail_<EN>_<OB>_<mnemonic>`,
  `expect_unreachable_<EN>_<mnemonic>`), with the result the assumption table
  states — or a recorded finding where the result differs, since an assumption
  carrying no weight is itself what the table asks about.
- **One mutation per new obligation**, run before the green is believed, each
  breaking exactly the check it is aimed at and each run against a neighbour
  that stays green — **with evidence that the mutation actually fires**, one
  existing witness re-run under it. `selection-k34` and `names-k33` between them
  produced four mutations that were unsatisfiable and therefore reported exactly
  as survivors; that is the hazard this clause exists for.
- The family `README.md` records: the new bounds and the argument for any that
  differ from the standard shape, `Witness` and `crash` in the abstraction
  table, the bound at which each of the four witnesses **first** lands, and the
  four new mutation-matrix rows.
- Material observations are appended to Experiment 2 as entry 028.
- The `guards` sibling is cut with `grove-llm leaf-add` as this session's last
  act before commit, its body carrying what the model's shape at that point
  makes concrete about `TT-21` – `TT-23`.


## Notes

`TT-20` is the first claim in this scope needing an action **interrupted
part-way**, so the README's `3 steps` argument does not cover it: budget a wider
trace bound for its commands and record the bound and the reason beside them.

`TT-19` refuses *observations* as well as mutations, which is the shape
`TT-13.c` already has — copy it rather than inventing a second whole-tree
refusal mechanism.

`TT-17`'s witness is a **legacy** tree whose slug text would read as a current
kind. The model abstracts slugs to opaque atoms and nothing reads one, so the
witness has to be built out of what the model does represent: a tree whose
entries are perfectly well-formed current-format names and whose format witness
says something else. If that turns out to make `TT-17` unfalsifiable by
construction — the classification cannot read a slug because the model has no
slug content to read — say so explicitly in the README the way `TT-11`'s
"answered by construction" is said, rather than writing a check that cannot fail.

Two commands in this file already cost minutes each and one costs nine. If a new
command stops finishing, prefer **narrowing the antecedent** over shrinking the
bound; that trade is recorded on `TT-05`'s four commands.

Do not read the Quint side of Experiment 2. The independence protocol holds
until both families are green.


## Decisions (running log)

**The witnesses are modelled beside `Obj`, not in it.** `Fmt.fmt` and `Slot.occ`
are the format witness's content and what sits at the reserved name; neither is a
filesystem object. No `TT-17` – `TT-20` obligation reads a witness's name,
position or key — what they read is presence and content — and keeping them out
of `Obj` is what kept the earlier slices' bounds where they were. `TT-24.b`
(`ownership`'s) needs *a foreign entry at a reserved name*, and `Slot.occ =
Unowned` is the seat kept for it; if that turns out not to be enough, re-seating
it is that leaf's cost and the alternative was paying it here for four claims
that do not need it.

**`rootState` is a `fun`, not `var` state.** A classification is a function of
the state it classifies, so making it a variable would add eight free values per
state to every command in the file for no gain. `TT-18` is still a claim rather
than a definition, because the mutation the matrix runs is a **reordering of that
function's body** — the same relationship `ParseIsCanonical` has to `Grammar`.

**`Absent` is omitted from the root states, and recorded as an omission.** No
`TT-` obligation reads it; `SY-05` owns the absent task root. Representing it
would mean letting `TaskRoot` leave `onDisk`, which is a rewrite of the
filesystem fact every earlier command rests on.

**`TT-01` – `TT-16` now carry `CurrentRootThroughout` explicitly.** Three reasons,
and only the first was the intent. It is a **narrowing, not a change**: a claim
about `add-leaf` appending was never a claim about a root `add-leaf` refuses to
touch. It **pins** the new state instead of leaving it free, which is what keeps
those thirty commands affordable. And writing it down **exposed** that four of
them — the *malformed halts* family — were assuming it all along, since
`Malformed` is walk-derived and `TT-18` orders it last.

The bundle also excludes the root-lifecycle actions and `Crash`, and that clause
is not cosmetic: without it `TT-03` — already the tightest command in the file —
went from 68s to not finishing. Nothing is lost by it, because `Crash` under `no
inFlight` is a pure stutter that `doIdle` already supplies, and the four root
actions all refuse on a current-format root.

**`TT-17` is checked as two conjuncts, and the second is the falsifiable one.**
*Classification depends only on the witness's content* is true by construction of
any classification written from the witness; what can fail is *a hand edit that
changes every name and leaves the witness alone does not move the root between
format families*. Without that conjunct `TT-17` would have joined `TT-11` in the
README's *answered by construction* paragraph, which was the outcome this leaf's
notes told it to name explicitly rather than fake.

**`EN-12` was given a mechanism rather than declared untestable.** *A name
renders as exactly one path component* was free in a model with no paths, so it
had nothing to control. `Rendering.collide` is the mechanism, `TT-01.a` is
restated over *denotation* rather than *reading* so it has somewhere to fail, and
`EN_12` rides inside `GroveGrammar` so the free relation is pinned for the other
sixty-seven commands.

**`EN-04`'s finding is that nothing depends on it**, which is what the assumption
table predicts. This model already carries the candidate — promotion is one step
— so the control is `expect_unreachable`: the half-applied promotion the
incumbent would expose does not exist, and `TT-07`/`TT-08`/`TT-09` are green
beside it at no wider bound.

**`TT-19`'s check runs at `3 steps`, not `2`.** Found by its mutation surviving.
Most of the claim is refusals, which change nothing, but its exception clause is
about the matching *recovery* — and a recovery that settles a witness is a tree
change. The lasso argument the README already documents, met where most of the
claim looks like a read.

**`InitScaffold` runs the reserved half of the cascade and not the format half.**
Found by `TT-19`'s check, which caught the action refusing `RefNotAnEntry` under
a reserved witness — naming nothing and recovering nothing. A witnessless root is
what initialisation is *for*, so a format refusal there would make a
current-format root uncreatable; the split is `TT-18`'s ordering made
operational.
