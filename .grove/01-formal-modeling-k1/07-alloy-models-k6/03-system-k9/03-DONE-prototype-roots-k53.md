# roots-k53


## Goal

Add the task root's own lifecycle to `models/system/lifecycle.als`: `SY-05`,
`SY-06` and `SY-07` — six obligations. The root-state classification this scope
reads, scaffolding and its interrupted subset, exhaustion, the driver-owned
finish leaf, and absence as an **established and preserved** fact.


## Context

- `docs/specs/semantic-contract.md` §*Claims — system lifecycle* `SY-05` –
  `SY-07`; §*States* for the `PartialScaffold` subset and for the `Legacy`
  classification `SY-06.b` must refuse rather than complete; §*Actions* for
  `initialise-root` and `allocate-finish-leaf`.
- `models/system/README.md` — read *Three incidents worth carrying forward* and
  *The mutation matrix* before writing a fact. The third incident is new and is
  the one this leaf is most exposed to: **a check green at a bound too small to
  reach its own mutation**, which survived two rounds of mutation before a bound
  sweep found it.
- `CONTEXT.md` *Partial scaffold*, *root-init* / *fresh-grove start*,
  *Correlation ticket*, *Terminal disposition*.
- ADR `one-live-driver-per-working-tree`, the paragraph beginning *Consequently,
  at a driver lifecycle transition an absent `.grove/` is always a fresh-tree
  fact* — that is `SY-05.a` in prose, and it names both halves of why absence
  cannot be read as a receipt.
- The measuring invocation is
  `models/run.sh --scope lifecycle --family alloy --no-coverage`.


## Done when

- `SY-05.a`, `SY-05.b`, `SY-06.a`, `SY-06.b`, `SY-07.a` and `SY-07.b` each have
  a `check` and their required `witness_` runs, all green, and the runner
  reports exactly eight empty alloy cells for the lifecycle scope.
- **`SY-05` models absence as something Grove ESTABLISHES AND PRESERVES, never
  as something that HOLDS.** `finish-k8` falsified three formulations of `FN-28`
  on one trace: after the quarantine rename the task-root name is free, the
  world can occupy it, and it can give what it put there the quarantined root's
  own identity. The correlation ticket is the only durable evidence a finish
  succeeded (`docs/formalism-findings.md` entry 039). A file that states absence
  as an invariant will re-derive that counterexample at its own cost.
- **`SY-05.b` is a joint claim by construction and a placement question.** The
  catalogue says `SY-05` and `FN-11`/`FN-19` "SHALL be checked together", and an
  `FN_`-prefixed command in `models/system/` is a placement failure the runner
  refuses. What this file owes is the *observation* — no trace exposes an absent
  task root before the deletion is proven — stated over its own transitions,
  with the finish model named as the owner of the underlying steps. This is the
  subtree's sharpest composition question and the node brief says so.
- **`EN-14`'s `SY-05` half is answered here**, and it is the half
  `admission-k51` declared owed rather than answered. The assumption table's row
  names both `SY-01` and `SY-05`; `expect_fail_EN_14_SY_01a_…` exists, and its
  `SY-05` sibling does not yet.
- One mutation per reported obligation, each with evidence that it actually
  fires — **and a surviving mutation investigated to one of three named causes
  rather than recorded as a survivor**. The three are in the family README: an
  unsatisfiable mutation (check it against the transition's own *frame*
  predicates, which is where a model states the opposite of a mutation without
  naming the field), a bound too small to build the mutation's antecedent, and a
  live mutation the check's shape cannot see. Only a **differential probe** —
  satisfiable in the mutant, unsatisfiable in the original — tells the third
  from the first, and it costs four lines.
- `models/system/README.md` extended: bounds and witness first-landing per new
  obligation, any new abstraction, and any new declared gap in the shape the
  runner parses.
- Experiment 2 entry 042 is appended with the six required fields plus the
  pre-registration's four additions, including the M5 authoring figure — this
  scope is the H8 comparison's system-level arm and it now has two points
  (0.42 and 0.60 h/obligation, mean 0.50).
- The next leaf (`sessions`, `SY-09`, `SY-12` – `SY-14`) is cut as a sibling,
  its body carrying the machinery question this session's file actually leaves
  open.


## Notes

**The machinery this session's file actually leaves open, and it is one
question.** `World.rooted` is `lone TaskRoot` — the task root is *present or
absent* and nothing more — and every claim so far has been satisfied by that.
`SY-06.b` is the first obligation that cannot be: it distinguishes a
`PartialScaffold` (by the exact known subset, never by the mere absence of the
format witness) from a `Legacy` tree, and refuses the second where it completes
the first. **That is two of `TT-18`'s eleven states, and importing two is not
importing eleven.** Decide, and record it in the README's composition-boundary
table: either `World.rooted` grows into a small classification of exactly the
states `SY-` obligations read, or the classification is imported as opaque
predicates over the observation with `crates/grove-task-tree/models/` named as
the owner. The node brief's rule points at the second, and the second is also
what keeps `SY-06.b` from becoming a third copy of `TT-18`.

**`doTreeOp` is one opaque step and `SY-06`/`SY-07` are the first claims that
care which one.** Scaffolding produces a root *and a first live leaf*
(`SY-06.a`); exhaustion appends or reuses *exactly one driver-owned finish leaf*
and no session may create one (`SY-07`). Both are about a specific tree
mutation's identity, where `doTreeOp` deliberately has none. **Prefer splitting
out named transitions to widening `doTreeOp`'s content** — `SY-07.b`'s *no
session creates one* is a claim about the actor of a specific mutation, and a
single opaque step cannot carry an actor rule for one mutation and not another.

**`doSelect` is guarded on `some World.live`, and `SY-07` is where that guard
comes off.** This slice declared exhaustion out of scope by making selection on
a spent tree unreachable rather than by branching on it — the catalogue's
`Empty` (`TT-15.a`) and the finish-leaf yield are both this leaf's. Expect to
replace the guard with a branch, and expect that to change
`witness_SY_04a_launch_alone`'s reachable set, so re-run the `SY-04` witnesses
after it.

**One design observation is waiting for `SY-13`, and it is `sessions`', not
this leaf's — but it was found here and would otherwise be lost.** `SY-04.b`
gates every Lifecycle transition but `acquire-lease` on a valid configuration,
so **`release-lease` is unreachable under an invalid configuration**: a driver
whose configuration goes invalid mid-loop cannot release its lease by returning.
It is not a sink — `SY-01.b` makes process death an ordinary release, and that
is the exit the shipped driver takes — but it is the first concrete state
`SY-13` will have to classify, and `roots` should carry it forward to
`sessions` if it does not land before then.

**Two `TT-` obligations remain declared gaps unfillable from either sibling
directory, and `finish-k8` found a third instance in Q4's removal matrix.**
Wherever a `SY-` claim's content is really a `TT-` or `FN-` one, record it in
the family README in the same shape rather than solving it —
`formal-synthesis-k16` inherits the whole set, and a fourth instance is cheap
for it only if it is *stated*. `SY-05.b` is the likeliest candidate in this
slice.
