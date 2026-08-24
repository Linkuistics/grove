# guards-k37


## Goal

Extend `crates/grove-task-tree/models/task-tree.als` to the **guarding** and
**bulk-mark** obligations `TT-21` – `TT-23`, and run the three Alloy-owned `EN-`
mutations this leaf holds: `EN-07`, `EN-14` and `EN-08`. The model stays green
for `TT-01` – `TT-20` throughout.


## Context

**Read `crates/grove-task-tree/models/README.md` before writing a command**, and
read the `roots` sibling's `Decisions (running log)` — several of them are
constraints on this leaf rather than history.

What `roots` left you, and what it costs you:

- **`CurrentRootThroughout`.** Every `TT-01` – `TT-16` command carries it, and it
  excludes the root-lifecycle actions *and* `Crash`. Any new action this leaf
  adds must either be excluded there too or be shown not to cost `TT-03` — which
  went from 68s to over ten minutes when four transitions arrived, and came back
  to 122s only when the bundle was narrowed. **In a temporal relational model the
  cost of a new action is paid by every command in the file**, so budget for it
  before writing the transition, not after.
- **`crash` exists**, as an action that ENDS an open transaction. `TT-23.b` needs
  exactly that: a bulk mark interrupted mid-run. Copy `doInitScaffold` /
  `doInitPublish` / `inFlight` rather than inventing a second transaction shape.
- **Refusals name things.** `Sys.pending` and `Sys.recov` are fields written by
  the transition, not derived, because a derived value could not be got wrong.

What this leaf has to build, and none of it exists:

- **Concurrency.** `TT-22` is about two operations at once, and the model has
  exactly one `Sys` and one operation per step. An operation needs a *duration* —
  something in flight across states — before "two concurrent observations are
  admitted" and "an observation and a mutation are serialized" are expressible at
  all. This is the layer to design first and the one this leaf will mostly cost.
- **The guard's holder.** `TT-22` says guards are taken on the **working-tree
  root**, which this model does not have: `TaskRoot` is the task root, and the
  working-tree root is what outlives its deletion (`EN-14`). `EN-14`'s mutation
  is *a scope in which the root itself is removed*, so the holder has to be
  something that can vanish.
- **One listing per operation** (`TT-21`). The claim is about **internal
  consistency, not about excluding the world**: `EN-06` grants only that
  cooperating processes are serialized, so a `foreign-write` may land
  mid-operation and the operation may act on a world that has already moved.
  `TT-21.b`'s witness is exactly that interleaving, with the operation's
  classifications still mutually consistent — so the model needs an operation's
  *snapshot* as state distinct from the live tree.
- **A bulk mark's plan** (`TT-23`), validated whole against one snapshot before
  the first rename, and convergent on re-run after a partial application. The
  ADR is `bulk-marks-are-not-atomic`.


## Done when

- `TT-21.a`, `TT-21.b`, `TT-22.a`, `TT-22.b`, `TT-23.a` and `TT-23.b` each have a
  `check` and the `witness_` run the catalogue names, all green under
  `models/run.sh --scope task-tree --family alloy --no-coverage`, with
  `TT-01` – `TT-20` still green beside them.
- `EN-07`, `EN-14` and `EN-08` run as their own named commands in the runner's
  two inverted forms, with the result the assumption table states — or a
  recorded finding where it differs. Note that the table's *expected result*
  column for `EN-07` and `EN-14` names `SY-11.b` and `SY-01`, which are the
  lifecycle scope's: what this leaf owes is the `TT-` half (`TT-22.b`, `TT-22`),
  and an assumption that turns out to carry no weight there **is** the finding.
- `EN-08` is run against the **named witness sets** rather than the whole file:
  it controls `TT-20`'s witness (which `roots` landed) and `TT-23.b`'s (which
  this leaf lands), and with `crash` removed both must become unreachable while
  every property check stays green.
- **One mutation per new obligation**, run before the green is believed, each
  breaking exactly the check it is aimed at and each run against a neighbour that
  stays green, **with evidence that it fires**. Read the README's *fire evidence,
  and its asymmetry* paragraph first: a witness that cannot survive its own
  mutation is not a failure of the mutation, and a failing check is its own
  evidence.
- The family `README.md` records the new bounds and any that differ from the
  standard shape, the concurrency layer in the abstraction table, the bound at
  which each new witness **first** lands, and the six new mutation-matrix rows.
- Material observations are appended to Experiment 2 as entry 029.
- The `ownership` sibling is cut with `grove-llm leaf-add` as this session's last
  act before commit, its body carrying what the model's shape at that point makes
  concrete about `TT-24` and `TT-25` — in particular whether `Slot.occ = Unowned`
  is still the right seat for `TT-24.b`'s foreign entry at a reserved name.


## Notes

`TT-23.b` needs **two consecutive** grove actions with an interruption between
them, so budget a wider trace bound and record it. `roots` found that `TT-20`'s
interruption needed only three states after all — the README's `3 steps` argument
reached it — so probe rather than assume; the probe is cheap and the README's
*first lands at* table is where the answer goes.

`TT-21` is the one claim in this scope that **cannot** be strengthened into
excluding a non-cooperating writer, and `TT-21.b` exists to say so. A model that
serializes `foreign-write` has answered `EN-06` by construction, which is the
shape of a false-confidence incident rather than a finding.

Two commands in this file cost minutes each and one costs nine. Prefer
**narrowing the antecedent** over shrinking the bound.

Do not read the Quint side of Experiment 2. The independence protocol holds
until both families are green.
