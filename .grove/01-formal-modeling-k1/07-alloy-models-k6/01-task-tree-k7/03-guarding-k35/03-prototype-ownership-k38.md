# ownership-k38


## Goal

Extend `crates/grove-task-tree/models/task-tree.als` to the **fail-closed
ownership** and **derived done-ness** obligations `TT-24` and `TT-25`, and run the
one Alloy-owned `EN-` mutation this leaf holds: `EN-11`. The model stays green for
`TT-01` – `TT-23` throughout. When it closes, the Alloy column of the task-tree
scope is complete and only the Quint column (`quint-models-k10`) is empty.


## Context

**Read `crates/grove-task-tree/models/README.md` before writing a command**, and
read the `roots` and `guards` siblings' `Decisions (running log)` — several
entries in each are constraints on this leaf rather than history.

What the three siblings left you, and what it costs you:

- **Two process scopes, and yours is the single one.** `Env.concOn` is a static
  switch; `SingleProc` pins it off and `CurrentRootThroughout` carries it.
  `TT-24` and `TT-25` are single-process claims, so **every command you write
  must pin the scope** — in the concurrent scope no grove mutation exists at all,
  so an unpinned witness is unreachable and an unpinned check is vacuous. That is
  the file's retained incident in a third set of clothes and the guards session
  avoided it only by having written the rule down first.
- **Refusals name things.** `Sys.pending` and `Sys.recov` are fields written by
  the transition, not derived, because a derived value could not be got wrong.
  `TT-24.b` is a claim about what a refusal **names**, so this is the machinery
  it needs, and `Sys.occupant` is the shape to copy.
- **`doRewrite` already refuses a node** with `RefNotLive`, and a bulk mark's
  plan contains only live *leaves*. `TT-25`'s "no action writes a node's state"
  therefore has its mechanism already; what it lacks is a derived-done-ness
  function and two witnesses. This is the cheap half of the leaf.
- **A total transition is what `TT-24` needs.** Every action returns exactly one
  outcome and a failed guard yields a named refusal with a byte-identical tree.
  Do not add an action whose guard failure is an *absent* transition.

What this leaf has to build, and none of it exists:

- **`Slot.occ = Unowned` is NOT enough for `TT-24.b`, and re-seating it is this
  leaf's first cost.** `roots` kept the seat and said so; `guards` did not need
  it and left it untouched; the guarding layer's own experience with
  `Sys.pending` is what makes the answer clear. `TT-24.b`'s obligation is
  `Refused(ReservedNameOccupied(entry))` — the reason **carries the entry** — and
  `Slot.occ` is a `SlotContent`, not an `Obj`. There is no way to name an entry
  from it. The two candidate answers, with their costs:
  - **`Sys.occupant: lone Obj`**, written by the refusal exactly as `pending` is.
    One `lone` field, and it follows a precedent the file already argues. But the
    occupant must then be an `Obj` sitting at a reserved *name*, and this model
    has no reserved `Filename` — reserved names live beside `Obj` precisely so
    that a witness costs no `Filename` atom. Introducing one is paid for by the
    nine-minute command.
  - **Declare `TT-24.b`'s naming half a gap** and check only the byte-identity
    and no-recovery halves. Legitimate only with a reason, and the reason would
    have to be better than *it was expensive*.
  Decide it explicitly and record the decision; a model that lets itself choose
  has answered `TT-24` by construction, which is what the catalogue's *one
  artifact, three contexts, one decided outcome* table exists to prevent.
- **`reservedRefusal` must split**, and the split changes a green command.
  It currently returns `RefWitnessPending` for **any** `some Slot.occ`. `TT-24.b`
  needs `Unowned` to return a different reason naming no recovery, so the
  predicate splits on `Slot.occ in WitnessClass`. `TT-19`'s check says
  `some Slot.occ implies Sys.res' = RefWitnessPending` and **will fail** the
  moment you do it: narrow it to `Slot.occ in WitnessClass`, which is what
  `TT-19` says in words (*a reserved **witness***) and what `roots` could not
  distinguish because `Unowned` had no consumer yet.
- **`Blocked` is not in this model's `Result` set**, and `TT-24.c`'s outcome is
  `Blocked(OwnershipConflict)`. Whether the task-tree model represents it or
  declares it a gap belonging to `crates/grove-finish/models/`'s scope is a real
  question this leaf must answer **explicitly rather than by omission**. A
  declared gap with a reason is a legitimate answer and the runner reads it from
  `README.md`. Note that `TT-24.c`'s antecedent is *inside a finish or recovery
  transaction*, and this file has no finish transaction — which is an argument
  for the gap, not against representing it.
- **`TT-24.d`** is the quarantine reaper, which this file has no notion of at
  all. The same question, and probably the same answer; state it separately
  rather than folding the two.
- **Derived done-ness** (`TT-25`): a node is done when no live leaf is beneath
  it, and nothing writes it. `nodeDirs`, `entries` and `liveLeaves` exist and
  `precedes` already reasons over `^loc`.


## Done when

- Every obligation of `TT-24` and `TT-25` is answered by a `check` and its
  required `witness_` run — or by a **declared gap with a reason** in the family
  `README.md`, in the runner's one shape — all green under
  `models/run.sh --scope task-tree --family alloy --no-coverage`, and the run
  reports **zero empty alloy cells** for the task-tree scope.
- `EN-11` runs as its own named command in the runner's inverted form, against
  the **named witness sets** it controls (`TT-02`, `TT-03`, `TT-13.c`, `TT-16`,
  `TT-24.b`, `TT-25`) rather than against the whole file. Removing `doHandEdit`
  makes a great many of the file's witnesses unreachable, and that is its stated
  control rather than a side effect — run it last, for that reason.
- **One mutation per obligation added**, run before the green is believed, each
  breaking exactly the check it is aimed at and each run against a neighbour that
  stays green, **with evidence that it fires**. Read the README's *fire evidence,
  and its asymmetry* paragraph and its *bound vacuity* paragraphs first: three of
  the file's seven incidents are a check written too narrow, and the predictor is
  now written down — an **interval** claim needs interval-many states.
- The family `README.md` records the new bounds, any new abstraction, the bound
  at which each new witness **first** lands, the new mutation-matrix rows, and
  the `EN-11` row; and its *what is covered* table reads `TT-01` – `TT-25`.
- Material observations are appended to Experiment 2 as entry 030.
- `task-tree-k7`'s Alloy column being complete, this leaf's last act before
  commit is to settle — or hand on, in writing — the one question
  `task-tree-k7`'s brief left *on the horizon*: a full `models/run.sh` run stays
  red until `quint-models-k10` lands, and whether that is an expected-red phase
  gate or a named subset invocation is a decision `formal-synthesis-k16` should
  not have to rediscover.


## Notes

`TT-24` is the one claim in this scope whose three obligations are **one artifact
met in three contexts**. The catalogue fixes the outcome of each — `Refused` for
an ordinary operation, `Blocked` inside a transaction, *declined and continued*
for the reaper — and a model that lets itself choose has answered the question by
construction. Where a context is out of this file's reach, the honest move is the
declared gap, not a fourth outcome invented to make the cell green.

Two commands in this file cost minutes each and one costs nine. Prefer
**narrowing the antecedent** over shrinking the bound — and prefer a **static
scope switch** over both where the claims permit it, which is the guards slice's
result and the cheapest of the three.

Do not read the Quint side of Experiment 2. The independence protocol holds
until both families are green.
