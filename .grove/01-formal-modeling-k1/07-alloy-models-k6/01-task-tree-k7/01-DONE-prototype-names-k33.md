# names-k33


## Goal

Build the repository model runner, and the first slice of the Alloy 6 task-tree
model: the naming, identity and mutation-algebra claims `TT-01`–`TT-10`.


## Context

- `docs/specs/semantic-contract.md` §*Model paths and the runner* is the
  runner's whole contract; §*Claims — task tree* `TT-01`–`TT-10` is this leaf's
  claim scope.
- `docs/ordinal-fs-tree/models/run-alloy.sh` is the convention the new runner
  adopts and the dead-tool probe it must keep. `models/run.sh` delegates to it
  and to `run-quint.sh` rather than absorbing them, which is also its positive
  control.
- `docs/ordinal-fs-tree/models/structure.als` is the house style for an Alloy
  model in this repository: nothing the document merely *claims* is a `fact`,
  claims are named predicates, and every command says which ones it assumes.
  Its machinery is Alloy 5-static and does not carry over.


## Done when

- `models/run.sh` exists, runs every `.als`/`.qnt` model in the repository,
  aborts on a dead tool, fails on zero work, and reports the
  `(family, obligation)` coverage matrix in both directions against the
  catalogue read as a manifest.
- `models/README.md` maps the model paths and states what each scope owns.
- `crates/grove-task-tree/models/task-tree.als` answers every obligation of
  `TT-01`–`TT-10` with a `check` and its required `witness_` runs, all green.
- `crates/grove-task-tree/models/README.md` records tool version, bounds,
  solver, abstractions, deliberate omissions, what a green run does not prove,
  the obligations covered so far, and any declared gap.
- Experiment 2 entry 026 is appended with the six fields and the
  pre-registration's four additions.
- The next leaf (`selection`, `TT-11`–`TT-16`) is cut as a sibling.


## Notes

`TT-10` is a claim about the *boundary*: no algebraic refusal reaches an
operator, because grove's own preconditions run in front of `ordinal-fs-tree`.
Model the algebra as an assumed layer with its own refusals, not as an
implementation of it.


## Decisions (running log)

**`task-tree-k7` was decomposed before this leaf ran.** Twenty-five claims and
forty-three obligations of temporal Alloy, plus the repository runner the
catalogue makes the first model family responsible for, is not one focused
session. The cut is by the machinery each claim group needs, not by the file:
names/identity/algebra needs no walk and no root classification, selection needs
the walk, guarding needs the root states and `crash`. The rationale and the
remaining two children are in the node's `BRIEF.md`.

**The runner asserts coverage over exactly the subset a run names, and
`--no-coverage` is what a scope under construction uses.** The catalogue
requires coverage in both directions per `(family, obligation)`, which makes a
bare `models/run.sh` red until both families cover all three scopes — correctly,
since that is the phase's remaining work. Three ways to make that liveable were
open: excuse a family that has no model directory for a scope (rejected — it
excuses exactly the omission the per-family rule exists to catch), keep a
declared-gap list of not-yet-written obligations (rejected — a declared gap means
*cannot express*, and conflating it with *not yet* is how the two stop looking
different), or make the caller **name** the subset it is asserting. The third is
the only one where nothing is silently excused: the coverage matrix is always
computed and printed in full, and `--no-coverage` suppresses only its
fatality, printing the empty-cell count as the remaining work.

**A command whose obligation belongs to another scope is a runner failure, not a
warning.** The root brief's placement rule — component models beside the
component they constrain — is otherwise unenforced prose, and a `SY_` command
that drifts into `crates/grove-task-tree/models/` would satisfy coverage while
putting a lifecycle claim in a component model. It costs one comparison.

**Two command conventions added, both inverted.** `expect_fail_<EN>_<OB>_` must
find a counterexample and `expect_unreachable_<EN>_` must find none. The
catalogue's assumption table has three control classes and two of them expect a
*named obligation to fail* or a *named witness to become unreachable*; a runner
that treats every failing check as a defect cannot report either, so the
controls would have had to live outside the suite. They are the runner's
conventions rather than Alloy's, so both families inherit them.

**Alloy runs with `-n` and `-t text`.** `-n` excludes arithmetic-overflow
models: positions and keys are `Int` because allocation is `max + 1` and a shift
is `+ 1`, and a counterexample that exists only because `plus[7, 1]` wrapped is
a fact about the bitwidth. `-t text` because the default `-t table` renders a
temporal trace as an **empty grid** — the tool reports a counterexample and
shows nothing of it, which is an M3-score-0 counterexample produced by a flag
rather than by the defect.

**A step-relation check runs at `2 steps`, and the licence is `EN-11`.** The
initial state is unconstrained beyond the filesystem facts — which is what *any
well-formed tree is reachable by hand edit* means as a modelling decision — so
every single transition is reachable from state 0 and a two-state trace can
exhibit any counterexample to a one-step property. Nothing in `TT-01`–`TT-10`
needs two consecutive grove actions. The first attempt ran everything at 3
steps and `TT-03` alone did not finish in three minutes.

**Gaplessness is stated relationally, not by counting.** `#positions !=
#entries` is the obvious spelling and it is what took `TT-03` past three
minutes: set cardinality over `Int` translates badly to SAT. No repetition, a
position 1 present, and no position whose predecessor is absent says the same
thing with no arithmetic in it, and runs in seconds. The append's own position
moved from `#entries + 1` to `max + 1` for the same reason; on a gapless level
they agree, and gaplessness is exactly `not halted`.

**Promotion replaces the object rather than mutating its species.** A leaf is a
file and a node a directory, and an inode does not become the other — which is
why `EN-04` exists at all. So `decompose` removes the leaf object and introduces
a directory object, and what `TT-08` preserves is the **key**, not the atom.
Modelled as one step here; its non-atomicity is `EN-04`'s control, which is
`guarding`'s to run.

**Three corrections landed in the catalogue, none of them about grove.** The
obligation manifest is the obligation lines *outside fenced blocks* — the
document demonstrates the shape by showing it, and its own example is otherwise
byte-identical to a real obligation. `TT-01.b`'s refusal is fixed as
`Malformed(MalformedEntry(entry))` rather than left to each family to choose,
since a spelling refusal and an unknown-kind refusal are the same reason and a
model inventing a second one has widened a closed set. `TT-07`'s byte clause is
discharged by the **entry digest** the vocabulary already defines; read against
the deliberate-omission row alone it is a clause neither family can reach, which
is not a claim.

**The suite reported itself green — witnesses included — while checking
nothing, and that is the session's most useful result.** Two defects compounded:
`doRewrite` left `Sys.act'` unconstrained for a `Live` mark, so a no-op rename
could label itself any action; and an Alloy 6 trace is a lasso, so at `2 steps`
no applied mutation exists at all and every check conditioned on `Applied` was
vacuous. The witnesses exist to catch the second and were defeated by the first.
Every behavioural command now runs at `3 steps`, every action's outcome is a
total function of its guard, and the incident is retained in the family
`README.md` because the *rule* it produced is worth more than the fix: a paired
witness proves reachability only if the transition relation cannot lie about
which action fired.

**An entry is what the walk *reaches*, not what sits beneath the task root.**
`TT-06.b` produced a real counterexample: an insert into a directory whose own
name is foreign, on a level whose positions began at 2. Grove descends into the
task root and into nodes only, so a well-formed task name inside a foreign
directory is invisible — not an entry, no position, no key in the counter, and a
malformity in it does not halt the tree. The catalogue said "anything directly or
transitively beneath the task root", which is the opposite; it now says
*reached*, `TT-04` extends the foreign rule to whole subtrees, and `TT-06` says
which directories it quantifies over. The correction propagated to `TT-02.a`,
`TT-02.b` and `TT-03`'s witness, which is itself evidence the ambiguity was
load-bearing rather than cosmetic.

**The mutation pass runs before the green is believed, not after.** Sixteen
mutations, thirteen killing their check first time. Two of the three failures
were mutations the model's own filesystem facts make **unsatisfiable** — the
mutated transition never fires, so its check is vacuously green and the run looks
exactly like a surviving mutation. A mutation the model cannot execute is not a
control. The third survived honestly: dropping species mismatch from the halting
reasons left `TT-02.b` green because `NodeWithoutCharter` halts the same tree,
which is an overlap in the catalogue's reason table that only a mutation could
have surfaced.

**Bounds are per command and recorded on it, and a check that will not finish is
narrowed at the antecedent rather than at the bound.** `TT-05` became four
commands, one per action, rather than one over `groveActs`; `TT-06`'s gaplessness
consequent is about the level the action touched rather than every level. The
temptation to shrink the bound instead is worth naming: it buys the green run at
the cost of what the run was evidence about, and it can put a check below the
bound at which its own witness first lands, at which point the pair still reports
green and means nothing.

**The runner skips fenced blocks wherever it reads a manifest, not only in the
catalogue.** The same defect recurred within the hour: the family `README.md` is
also a manifest — declared gaps are read out of it — and a worked example of the
gap line went straight into it. It failed to fire only because the placeholder
had letters where the pattern wants digits. The rule generalises rather than the
fix.
