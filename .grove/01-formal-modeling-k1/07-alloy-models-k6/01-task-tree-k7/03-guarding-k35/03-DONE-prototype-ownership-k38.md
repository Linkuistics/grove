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


## Decisions (running log)

**The seat for `TT-24.b`'s occupant is a POINTER FROM THE SLOT, not a reserved
`Filename`.** `Slot.occAt: lone Obj` — the object sitting at the reserved name,
present exactly when `Slot.occ = Unowned`. Both candidates the `guards` sibling
handed forward were paid for and neither was taken whole: `Sys.occupant: lone
Obj` is right and is here, but the occupant does **not** need a reserved
`Filename` to sit at. What makes a witness reserved in this file is that the slot
holds it; what makes an occupant reserved is that the slot points at it, and the
symmetry is the argument. The rejected alternative — `one sig ReservedF in
Filename` — consumes a `Filename` atom in **every** command in the file, and
`witness_TT_07_shift_across_every_species` runs nine minutes at six with nothing
spare. The occupant's own name is `foreignName`: grove cannot parse it, which is
what *cannot classify at all* means here.

**`reservedRefusal` split, and `TT-19`'s check narrowed with it.** The witness
half keeps `RefWitnessPending` and names the recovery; the `Unowned` half is
`RefReservedNameOccupied`, names the entry, and names **no** recovery and no
class. `TT-19`'s check now says `some (Slot.occ & WitnessClass)` where it said
`some Slot.occ` — which is what `TT-19` says in words (*a reserved **witness***)
and what `roots` could not distinguish, because `Unowned` had no consumer yet.

**A model defect I introduced, found by a reachability probe rather than by a
check, and it is the file's eighth incident.** The split was first written
`Slot.occ in WitnessClass implies …`. `in` is **subset**, and a `lone` field's
empty value is a subset of every set — so on a root with no reserved artifact at
all the antecedent was true, every ordinary transition was forced to refuse
`RefWitnessPending` against its own applied branch, and the transition relation
was **unsatisfiable**. All four new checks reported green while checking nothing;
the two witnesses that did not land are what caught it. Spelled
`some (Slot.occ & WitnessClass)`, which is what `doRecover` had said correctly
all along.

**`TT-24.c` and `TT-24.d` are declared gaps, with reasons, and that is the
catalogue's own answer rather than a concession.** `TT-24.c`'s antecedent is
*inside a finish or recovery transaction* and its outcome is
`Blocked(OwnershipConflict)`; this file has no transaction of that kind and no
`Blocked` in its `Result` set, and inventing one would answer `TT-24` by
construction — which is exactly what the catalogue's *one artifact, three
contexts, one decided outcome* table exists to prevent. `TT-24.d`'s subject is
the quarantine reaper, which is `FN-21`'s machinery. Both belong to
`crates/grove-finish/models/`.

**`TT-25`'s prohibition is answered by construction, and the checked half is
behavioural.** *A node is never marked* cannot fail in this model: `isShaped`
gives a `NodeSp` name **no `fOut` field at all**, so a marked node is not
spellable. Said out loud rather than faked with a command, exactly as `TT-11`'s
*depends on no state outside the tree* is. What is left is falsifiable and is
what "derived" actually forbids — the transition that makes a node done writes
nothing to the node (`TT-25.a`), and done-ness reads the **whole** subtree
(`TT-25.b`, stated over `d.^(~loc)` rather than over `nodeDone`, so the mechanism
has somewhere to be wrong).

**`TT-25.b` runs at `3 DirObj`, which is wider than anything else in the file.**
A node, a node beneath it, and the task root. At two the live leaf is necessarily
a direct child, both readings of done-ness agree, and the mutation aimed at this
obligation would survive for want of a place to differ. It is the depth analogue
of the guarding slice's interval rule: **a claim about a subtree needs
subtree-deep bounds**, counted before the command is written rather than after
the mutation survives.

**`EN-11` is realised twice in this file, and removing the action removed
nothing.** The first switch guarded `doHandEdit` only; every named witness stayed
reachable, because the README's own `3 steps` argument rests on the
**unconstrained initial state** — *every single transition is reachable from
state 0* — which is `EN-11` cashed out as a modelling decision rather than as an
action. `not EN_11` now takes both, and the scope it leaves needs
`SingleProc` (not `CurrentRootThroughout`, which excludes the root-lifecycle
actions and would make the whole scope trivially dead) and `5 steps`, with a
companion witness showing grove still builds a tree under it. The general form:
**an exercise-removal under which nothing becomes unreachable has not removed
the assumption.**

**`EN-11` does not control `TT-16`, and the catalogue's row is corrected.** Five
of the six named witness sets go dark with `hand-edit` removed; a resolved
*terminal* entry does not, because grove's own actions build one — allocate,
retire, resolve. Shipped as a **positive** control
(`witness_EN_11_a_resolved_terminal_entry_needs_no_hand_edit`), because an
`expect_unreachable_` there would be a command written to the assumption table
instead of to the model. Third assumption row this file has found to carry less
weight than predicted, after `EN-04` and `EN-07` — and the first where the table
was wrong rather than self-aware.

**The horizon question is settled as a NAMED SUBSET, not an expected-red gate**,
and the settlement is recorded where the runner's contract lives
(`docs/specs/semantic-contract.md` §*Model paths and the runner*) rather than
only in a model README. While a family's column is under construction the
unqualified `models/run.sh` is red and that redness is true; what it must not
become is a colour anyone is told to ignore, because a suite whose red is
routinely explained away has stopped being an instrument. So the phase's green is
`models/run.sh --scope <scope> --family <family>`, which restricts the coverage
matrix to that family's cells and therefore still **fails on an empty one** —
and dropping `--no-coverage` from a model README's run line is the visible signal
that its column closed. This directory's line is updated accordingly.

**No ADR, and no ADR needed reworking.** Applying the three-part AND test: the
named-subset run rule and the two declared gaps are both **easily reversible** —
a flag, and a later model that can express what this one cannot — so neither
earns a record, and both are recorded where they land (the catalogue's runner
section, the family README). Reconciling the set against what this leaf
established found nothing to change:
[`task-tree-transactions-fail-closed`](../../../../../docs/adr/task-tree-transactions-fail-closed.md)
already says *every ordinary reader and mutator refuses while any reserved
witness exists*, that the matching recovery runs ahead of format and selection
classification, and — in its rejected alternatives — that sweeping a reserved
namespace would delete the bytes a refusal exists to preserve. `TT-18`, `TT-19`
and `TT-24` are that decision checked, not amended.

**`EN_11` is left FREE rather than pinned into the law bundle, and the measurement
is why.** The file's `EN_12` precedent says a free static switch should ride
inside `GroveGrammar`, since an unpinned free relation is paid for by every
command. It does not transfer, and the sentinel says so: `TT-03` runs at **138s**
with `EN_11` free and **208s** with it pinned into the bundle. The asymmetry is in
what pinning *does*. Pinning `EN_12` **removes** a free relation from the search;
pinning `EN_11` **enables a transition** — `hand-edit`, the most permissive one in
the file — in every trace, where leaving it free lets the solver take the
empty-start branch when that branch settles the command. **Pin a switch that
deletes state; leave free a switch that admits an action.**

**This slice imposed nothing measurable on the rest of the file**, and it is the
first that can say so: `TT-03`, the file's standing sentinel, went from **156s to
138s**. The reason is structural rather than luck — it is the first slice that
added **no transition at all**. `TT-24` and `TT-25` needed no new action, because
the actions that could violate them already existed; what was missing was
something for a refusal to **name** and something for done-ness to be **read
from**, and both are state a command either pins or never mentions. Set beside
entry 028's +41% for four transitions and entry 029's +10% for five behind a
static switch, the file now has a three-point cost model, and it is not about
size: **cost is paid per transition encoded into every trace.**

**A near-miss worth recording, since the discipline is what caught it.** The
whole-suite run was pacing at roughly a third of the previous revision's
throughput, and the obvious reading — this slice regressed the file — was wrong:
the first twenty commands contain the file's heaviest checks, so per-command
pacing across an unequal mix says nothing. Measuring the **sentinel**, which
exists for exactly this comparison, took three minutes and reversed the
conclusion. A repair was drafted and would have made the file 50% slower.

**Correction to the paragraph above, and it is this slice's most transferable
result: ONE SENTINEL IS NOT ENOUGH.** *This slice imposed nothing measurable* was
read off `TT-03` alone and is **wrong**. A same-host, same-session A/B against
the pre-slice file across four commands:

| command | baseline | with this slice | Δ |
|---|---|---|---|
| `TT-03` (the sentinel) | 156s | 138s | −12% |
| `TT-11` | 77s | 75s | −3% |
| `TT-15.a` | 51s | 56s | +10% |
| `witness_TT_07` (the file's largest) | 668s | **987s** | **+48%** |

The three cheap-to-middling commands are unchanged; the single largest one is
half again as expensive. The reason is what was added: a ninth `Result` atom and
two `lone Obj` fields are state present in **every state of every trace**, so
they are paid where the trace is **widest** — and `TT-03`, chosen as the sentinel
because it is the file's *tightest* command (one filename short of its
neighbours), is the command least able to show it. `witness_TT_07` carries five
files and two directories and shows it immediately.

> Measure the file's **largest** command as well as its tightest. A sentinel
> chosen for being tight measures the cost of transitions; only a wide command
> measures the cost of state.

**And whole-suite totals do not compare across sessions.** The scope went 4002s
CPU / 88 commands to **6888s / 103**, which reads as +72%. It is not: `TT-11`
costs **77s on the unmodified baseline today** against the **61s** the README
recorded, so this host is ~24% slower than when those numbers were taken —
independently confirmed by `witness_TT_07`'s baseline landing at 668s against a
drift-adjusted prediction of ~680s. Corrected for drift the pre-existing scope
is ~4970s, so this slice's real imposition on it is ~**+15%**, concentrated
almost entirely in the widest commands, plus ~860s for its own fifteen new ones.
The README's absolute figures are now labelled with that caveat, because a later
reader comparing against them would repeat the same mistake I made.
