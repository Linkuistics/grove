# task-tree-k7 — brief


## Goal

Model Grove's task-tree semantics in Alloy 6 without reimplementing generic
ordinal-tree algebra, and build the repository model runner the first model
family to run owes the phase.


## Context

The model belongs with `grove-task-tree`, at `crates/grove-task-tree/models/`.
Treat `ordinal-fs-tree` properties as an imported/assumed algebraic boundary and
concentrate on Grove names, format ownership, legal selection, growth,
retirement, and terminality.

`docs/specs/semantic-contract.md` is the sole input: every state, action,
outcome, refusal reason and claim this subtree models is defined there, and
nothing here invents a semantic decision. The catalogue's unit is the
**obligation** — a claim with no sub-identities, or one lettered sub-identity of
a claim that has them — and the subtree owes an Alloy command per obligation, or
a declared gap in the family `README.md`.

Alloy 6 must be **temporal**: `var` state, primed transitions, and traces that
include interruption. A static relational snapshot does not answer `TT-20`,
`TT-21`, `TT-22` or `TT-23`.


## Done when

- The model represents current-format roots, Grove-owned entries, task kind/key
  identity, ordinal sibling order, active-leaf selection, decomposition,
  insertion/addition, retirement, and empty/root-terminal states.
- Assertions cover uniqueness, valid naming/format, stable selection,
  preservation of unrelated/opaque entries, legal mutation preconditions,
  fail-closed foreign roots, and terminal-state correctness — that is, every
  `TT-` obligation the catalogue defines is answered by an Alloy command or by a
  declared gap.
- Temporal traces include normal progress and refused/invalid operations;
  satisfiable witnesses demonstrate every transition family.
- Bounds, assumptions about the ordinal component, runner command, claims, and
  at least one useful instance or counterexample are documented.
- The Alloy-owned assumption mutations that control `TT-` obligations are
  present as their own named scopes or commands, with the expected result the
  catalogue states.
- Material observations are appended to Experiment 2 using the required six
  fields plus the pre-registration's four additions.


## Decomposition

One `.als` file, three sessions, cut along the machinery each claim group needs
rather than along the file. Each child leaves the model **green for the
obligations it claims** and the runner able to say exactly which cells are still
empty, so no child is dead until its siblings land.

1. `names` — the repository runner, and the naming/identity/mutation-algebra
   claims (`TT-01`–`TT-10`). Needs filenames, the parse trichotomy, the tree,
   and the four algebraic operations; needs no walk and no root classification.
2. `selection` — the walk (`TT-11`–`TT-16`). Adds pre-order, terminality, the
   reserved finish leaf, and the empty/ambiguous observation outcomes.
3. `guarding` — root identity, guards and fail-closed ownership
   (`TT-17`–`TT-25`). Adds root classification order, reserved witnesses,
   shared/exclusive guards, `crash`, and the Alloy-owned `EN-` mutations that
   control `TT-` obligations (`EN-04`, `EN-07`, `EN-12`, `EN-14`, and the
   exercise-removals `EN-08`/`EN-11`).

The later two are cut as leaves by the session before them, once the model's
actual shape is known — the claim groups are fixed by the catalogue, but which
machinery each needs is not knowable until the file exists.


## Pointers

- `docs/specs/semantic-contract.md` — the claim catalogue; §*Claims — task tree*
  is this subtree's whole scope, and §*Model paths and the runner* is the
  runner's contract.
- `docs/formalism-findings.md` — *Experiment 2 — pre-registration* fixes the
  recording protocol, the material-finding rule, and the named false-confidence
  hazards each command is a control for.
- `docs/ordinal-fs-tree/ARCHITECTURE.md` — the delegated boundary: the
  ordinal/key split, the parse trichotomy, and the seven seam obligations. This
  subtree **consumes** it and restates none of it.
- `docs/ordinal-fs-tree/models/run-alloy.sh` — the pass/fail conventions the new
  runner adopts (`witness_*` must find an instance, every `check` must find
  none) and the dead-tool probe it must keep.
- ADRs: `task-names-are-canonical`, `entries-are-never-removed`,
  `entry-name-is-the-only-seam`, `bulk-marks-are-not-atomic`,
  `task-tree-transactions-fail-closed`.
- Glossary: *Tree format witness*, *Node directory*, *Leaf*, *Pick*, *Position*,
  *Permanent key*, *Work-item handle*, *Tree access lock*, *DONE infix*,
  *Pruning*, *Partial scaffold*, *Obligation*.


## Notes

If the model needs raw path manipulation to express a Grove rule, treat that as
evidence that the semantic seam is wrong; do not silently pull filesystem
mechanics into this component.

Do not read the Quint side of Experiment 2 — the independence protocol holds
until both families are green. `docs/ordinal-fs-tree/models/operations.qnt` is a
different subject and a different experiment, but there is no reason to open it
either.


## On the horizon

- The runner asserts coverage over the **whole** catalogue, so a full
  `models/run.sh` run stays red until `quint-models-k10`'s leaves land. Whether
  that is stated as an expected-red phase gate or as a named subset invocation is
  a question `formal-synthesis-k16` inherits if these leaves do not settle it.
- Nothing here creates `crates/grove-task-tree/Cargo.toml`. The catalogue says
  the `models/` child exists before the crate does; whether the workspace should
  gain a stub member so the directory is not invisible is an implementation-phase
  question, not this subtree's.
