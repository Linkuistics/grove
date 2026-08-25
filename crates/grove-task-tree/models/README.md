# grove-task-tree — models

The task-tree scope of [the semantic
contract](../../../docs/specs/semantic-contract.md): the `TT-` claims, checked
independently by each model family. This directory exists before the crate does,
which is deliberate — the model is what the crate will be cut against.

Run them from the repository root:

```sh
models/run.sh --scope task-tree --family alloy --no-coverage
```

## What is covered, and what is not

| family | file | obligations |
|---|---|---|
| Alloy 6 | `task-tree.als` | `TT-01` – `TT-23` |
| Quint | — | none yet (`quint-models-k10`) |

`TT-24` and `TT-25` are the `ownership` sibling leaf's work — fail-closed
ownership and derived done-ness. The runner reports their cells empty, which is
the truth about this directory rather than a defect in it: they are **not**
declared gaps, because a declared gap means *cannot express*, and nothing here
has tried yet.

**One question `ownership` inherits, stated here because this slice is where it
became concrete.** `TT-24.c`'s outcome is `Blocked(OwnershipConflict)`, and
`Blocked` is not in this model's `Result` set. Whether the task-tree model
represents it or declares it a gap belonging to `crates/grove-finish/models/` is
`ownership`'s to answer explicitly — a declared gap **with a reason** is a
legitimate answer, and the runner reads it from this file.

**Declared gaps** — none. The runner reads them from this file, in one shape:

```md
- **GAP** alloy `TT-nn.x` (inexpressible|abstracted|out-of-bounds|tool-limited) — reason.
```

## `task-tree.als`

**Tool.** Alloy 6, `org.alloytools.alloy.dist.jar`, on Corretto
`21.0.12.1+9-LTS`. The measurement host's default `java` is Corretto 16.0.1 —
below Alloy 6's floor — so the runner's own JDK probe is the difference between
a suite and a broken instrument that reports every check green and every witness
missing ([`docs/preservation-baseline.md`](../../../docs/preservation-baseline.md)
§1).

**Solver.** SAT4J, the distribution default. No command depends on a
solver-specific behaviour.

**Bounds.** Stated per command. The common shape is
`for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps`.
Two of its parts mean something other than "make it bigger":

- **`4 Int`** is a bitwidth, not a count: positions and keys are `Int` because
  allocation is `max + 1` and a shift is `+ 1`. Every command runs with Alloy's
  `-n` (no-overflow), so a counterexample that exists only because `plus[7, 1]`
  wrapped is excluded — that is a fact about the bitwidth, not about the claim. The
  usable range is 1..7 for both, and no command needs more.
- **`3 steps`** is the minimum for any behavioural command, and the reason is
  the lasso. An Alloy 6 trace is infinite, so its last state must loop; a state
  reached by a tree-changing action can loop neither back to the idle initial
  state (the tree differs) nor to itself (repeating the action would change the
  tree again). **At `2 steps` no applied mutation exists at all**, and every
  check conditioned on `Applied` is vacuously true. Three states admit one
  mutation followed by a stutter; the vacuity guard, which needs two mutations,
  runs at four. The purely static name claims stay at `1 steps`.

- **`2 steps` for an observation, and the same lasso argument is why.** The
  three-state argument above is about a *tree-changing* action. `select` and
  `resolve` change nothing, so the state one of them reaches loops to **itself**:
  repeat the observation on an unchanged tree and every component of the state
  recurs. Two states therefore admit an applied observation, which is what makes
  the selection commands roughly an order of magnitude cheaper than the
  mutation ones. It is checked rather than argued — `witness_TT_09a_append` finds
  no instance at `2 steps` while every observation witness does.

  Three exceptions run wider. `TT-12` and `TT-13.c`'s **checks** quantify over
  grove *mutations* (a terminal entry is never taken off disk; every read and
  mutation refuses on a malformed tree), so they need the mutation bound of
  `3 steps`. `witness_TT_14` needs `4 steps`, because two orderings of the same
  work selecting differently is *select · hand-edit · select* and nothing
  shorter: it finds no instance at 2 or at 3.

  What makes three *enough* is `EN-11` — any well-formed tree is reachable by
  hand edit — cashed out as a modelling decision: the initial state is
  unconstrained beyond the filesystem facts, so every single transition is
  reachable from state 0 and a one-step property needs no run-up. A claim about
  two *consecutive* grove actions would need more, and none of `TT-01` – `TT-10`
  is one.

The **guarding** slice (`TT-21` – `TT-23`) runs narrower — `2 FileObj, 1 DirObj,
4 Filename` — because every one of its obligations is flat: `1 DirObj` is the
task root and nothing else, and no claim about a guard, a listing or a plan reads
a subtree. The four commands that need four *entry* names run at `6 Filename`,
and the reason is an atom rather than a claim: `one sig CharterF in Filename`
consumes one `Filename` atom, so `4 Filename` leaves three for entries.
`witness_TT_23b` needs a live and a marked spelling for each of two leaves, and
was one atom short of **expressible**, not one short of true.

- **The lasso closes on the ACTION, not only on the tree, and the guarding
  witnesses are where that bites.** `Sys.act` is part of the state, so a trace's
  last state loops only if repeating its action reproduces it exactly. A
  `Classify` and a refused `Mark` are idempotent — same listing, same answer,
  same `Sys` — so those witnesses close on themselves and need no run-out state.
  An `Open` does not (a second open by a holder is `Deferred`) and an applied
  `Mark` does not (the entry it renamed is no longer live), so each of those
  costs **one extra state purely to close the loop**:
  `witness_TT_22a` at `4 steps`, `witness_TT_22b` at `6`, `witness_TT_23b` at
  `5`. It is checked rather than argued — each finds no instance one state
  shorter.

- **`4 steps` for both `TT-21` checks, and the reason is what the claim is
  about.** `TT-21` is a claim about what happens *between* an operation taking
  its guard and the classifications it makes from it, so the shortest violating
  trace is **open · interleave · classify** — four states. Both mutations aimed
  at `TT-21` **survived at `3 steps`** and were caught at `4`; see the mutation
  matrix. This is the `TT-19` incident's cause, met twice more, and it is now the
  standing question for any claim whose subject is an interval rather than a
  step: *how many states does the interval need before anything can happen inside
  it?*

- **`1 steps` for a root classification, and `3` for the interruption.** A
  classification is a function of the state it classifies, so `TT-17` and
  `TT-18` are one-state claims and run where `TT-01`'s do. `TT-19` needs two,
  because a refusal is a transition — and then **three**, because its exception
  clause is about the matching *recovery*, and a recovery that settles a witness
  is a tree change. At `2 steps` that clause is vacuous and the mutation aimed at
  it survives; the lasso argument above is the same one, met from a direction
  where most of the claim looks like a read. `TT-20` runs its witness at `3` —
  scaffold, crash, and the stable root the crash left — and its check at `4`,
  which is where an initialisation followed by an ordinary mutation gives a
  premature witness somewhere to appear.

**Runtime.** The Alloy scope of this directory is **88 commands** and costs
**4002s CPU** end to end on the measurement host (1h 02m wall; CPU is the fairer
number). One command is a large fraction of it:
`witness_TT_07_shift_across_every_species` runs about **nine minutes**, because
`TT-07`'s witness obligation asks for a level carrying every species at once —
a node, that node's charter one level down, a terminal leaf, a foreign entry and
the shift's target, which is five files and two directories before the insert
adds its own. It is kept whole rather than split into cheaper witnesses, because
"a directory containing every species" is the obligation as the catalogue states
it. Two checks are the other heavy ones, at three to four minutes each; both are
recorded on their commands.

The **selection** slice (`TT-11` – `TT-16`, 22 commands) costs about **4m 40s**
of that, and the reason is the `2 steps` bound: its two most expensive commands
are `TT-11`'s check at 61s and `TT-15.a`'s at 40s, and eleven of its twenty-two
finish in five seconds each. Its mutation pass is another **~7 minutes**. The
cheapness is a fact about observations, not about the claims: a read that cannot
change the tree gives the solver a two-state lasso to search instead of three.

The **guarding** slice (`TT-21` – `TT-23`, 12 commands plus 5 assumption
controls) costs under **a minute and a half** of the total, and it is the only
slice so far whose arrival did not visibly move the ones before it. Five new
transitions, an eighth `Result` atom, two `Sys` fields and seven per-process
`var` fields took the whole scope from **3648s CPU over 71 commands to 4002s over
88** — **+10% CPU for +24% more commands**, against the root-identity slice's
+41% for +20%. `TT-03`, the file's standing sentinel, went from **132s to 158s**:
a real +20%, not nothing, and the honest number to carry. The difference is
`Env.concOn` being **static**, so the whole concurrent branch of `step` is a
constant the translator folds away for the sixty-odd commands that pin it off;
the root slice's four transitions were reachable disjuncts of `ordinaryStep` and
had to be encoded into every trace whether their guards could fire or not.

The **root-identity** slice (`TT-17` – `TT-20`, 9 commands plus 2 assumption
controls) costs under **two minutes** of the total — and that number is
misleading in a way worth stating, because **the slice's real cost was paid by
the commands that came before it.** Adding four transitions (`InitScaffold`,
`InitPublish`, `Crash`, and three recoveries) took the whole scope from 2581s CPU
for 59 commands to 3648s for 71: **+41% for +20% more commands.** In a temporal
relational model a new action is encoded into every trace the solver searches,
including traces of commands whose guards it can never pass. `TT-03` — already
the tightest command here, run one filename short of its neighbours — went from
**68s to not finishing at all**, and came back to **122s** only once
`CurrentRootThroughout` pinned the new state away from it. The rule that follows
is a standing one rather than an optimisation reached for when something stalls:
**each slice pins the state the earlier slices did not know about.**

**Temporal, not structural.** State is `var`: `onDisk`, each object's name,
parent and digest, and the `Sys` record of the action and outcome that produced
the state. Every action is total — a failed guard yields a named refusal and a
byte-identical tree, never an absent transition — which is what lets a refusal
be checked at all.

**The root's own identity, and why it sits beside the entries rather than among
them.** `TT-17` – `TT-20` are the first claims here about the task root itself,
and the model gives it three pieces of state that are **not** `Obj`s: the format
witness's content (`Fmt.fmt`), what sits at the reserved name (`Slot.occ`), and
whether an initialisation transaction is open (`inFlight`). A witness holds no
position, carries no permanent key and is never ordered, so nothing any of those
four obligations says about one reads a `Filename` — what they read is its
**presence** and its **content**. Keeping witnesses out of `Obj` is also what
keeps the earlier slices' bounds where they are: a witness costs no `FileObj` and
no `Filename` atom, and the nine-minute command did not get a sixth file.

The classification itself is a `fun` (`rootState`), not a `var` field, so it adds
**no free state** for the solver to search. `TT-18` is nevertheless a claim and
not a definition, because the mutation the matrix runs is a **reordering of that
function's body** — the same relationship `ParseIsCanonical` has to `Grammar`.

**`TT-01` – `TT-16` say out loud what they were always about.** Every command in
those slices now carries `CurrentRootThroughout` — a current-format root, no
reserved witness, no transaction in flight — and that is a narrowing rather than
a change: a claim about `add-leaf` appending was never a claim about a root
`add-leaf` refuses to touch. Saying it explicitly does two things. It keeps the
new state **pinned** rather than free, which is why those thirty commands cost
what they cost before this layer arrived; and it makes visible that four of them
— `TT-02.a`, `TT-02.b`, `TT-03` and `TT-13.c`, the *malformed halts* family —
were implicitly assuming it. They had to be: `Malformed` is **walk-derived**, and
`TT-18` orders it behind reserved-witness and format classification, so on a tree
carrying a reserved witness the refusal is `WitnessPending` and not
`RefMalformed`. The arrival of root classification exposed that assumption; it
did not create it.

**Two process scopes in one file, and the switch that separates them.**
`TT-01` – `TT-20` are stated over ONE cooperating process, which this README
already recorded as a bound; `TT-21` – `TT-23` are about what happens *during* an
operation and about what a second process may do while it runs, so they need a
second scope. `Env.concOn` is a **static** switch that selects between them:
`CurrentRootThroughout` pins it off, the root-identity commands pin it off
explicitly, and the guarding commands turn it on. Static rather than `var`
deliberately — the translator can fold the whole unselected branch away, which is
the only reason five new transitions cost the earlier fifty-nine commands
nothing. The pinning is not optional: without it the solver may pick the
concurrent scope for a command about `add-leaf`, and in that scope `add-leaf`
does not exist — every witness unreachable, every check vacuous.

**What an operation IS in the concurrent scope, and why the two guards are not
symmetric.** An observation is `Open(Shared) · Classify* · Release` and holds its
guard across states; a mutation is a single `Mark` that acquires and consumes an
exclusive guard within one step. That asymmetry is
[`bulk-marks-are-not-atomic`](../../../docs/adr/bulk-marks-are-not-atomic.md)
rather than a simplification: *a mutating method consumes its `WriteGuard`*, so N
marks are N critical sections and an exclusive guard never spans a state
boundary. `holds` therefore only ever carries `Shared`, and `TT-22.b` is checked
over the mark's **acquisition** rather than over a held exclusive guard. It costs
two states off every `TT-23` command, and it is the shape the ADR describes.

**Assumptions about the ordinal component.** `ordinal-fs-tree` is an imported
boundary, not a dependency this model reimplements
([`docs/ordinal-fs-tree/ARCHITECTURE.md`](../../../docs/ordinal-fs-tree/ARCHITECTURE.md)).
Ordering, shifting and allocation are its properties; what this model states is
grove's *domain precondition in front of them*, and `TT-10` is the claim that
the seam holds. The algebra's own refusal is one opaque `AlgebraicRefusal` atom
and nothing produces it — the content of `TT-10` is in its witness, which
exhibits an argument the algebra would have refused being pre-empted by a
refusal this catalogue names.

**Abstractions and deliberate omissions**, beyond the catalogue's own fixed list:

| omitted | modelled instead as | why |
|---|---|---|
| the nineteen session kinds | `FinishK`, `OrdinaryK`, `UnknownK` | no `TT-` claim distinguishes more: the driver-reserved one, an ordinary one, and one this build does not know (`TT-03`) |
| a filename's internal grammar | a tuple of six `lone` fields plus a `canon` pointer | the catalogue says a name **is** the tuple of its parts and nothing else; `canon` is what a refusal names |
| an entry's bytes | an opaque `Digest` equality | the catalogue's *entry digest*, which the deliberate-omission row grants. It is what makes `TT-07`'s "never any file's bytes" checkable at all rather than a clause no model can reach |
| promotion's non-atomicity | one step | `EN-04` is the control on that, and the root-identity slice ran it |
| `Refused(DestinationOccupied)` | not represented | no `TT-01` – `TT-10` obligation states it; a name collision at a reserved name is `TT-24.b`'s, in `ownership` |
| a directory the walk does not enter | present on disk, but outside `visited` | the walk descends into the task root and into nodes only; nothing beneath a foreign directory is an entry, holds a position, or contributes a key |
| a resolution's reference syntax | `one sig Query` — an optional key and an optional slug | the CLI's `[n]`, a bare slug and `<slug>-k<key>` differ only in which of the two is present, and no `TT-` obligation reads a slug's content. **One** atom, so a trace carries one resolution argument: every command here needs at most one `resolve`, and the existing `TT-01` – `TT-10` scopes are unchanged by its arrival |
| what an observation reported | `Sys.got` and `Sys.gotTerm`, written by the transition | *derived* terminality could not be got wrong, and `TT-16` is precisely the claim that the report carries it. Modelling the report as state is what makes the mutation *"a resolved `Done` entry is not reported terminal"* expressible at all |
| `brief-chain` and `kind` | not represented | both are observations, but no `TT-11` – `TT-16` obligation states anything about either beyond what `select` and `resolve` already carry |
| the witnesses' filesystem placement | `Fmt.fmt` and `Slot.occ`, beside `Obj` rather than in it | see above: no `TT-17` – `TT-20` obligation reads a witness's name, position or key. `TT-24.b` will need a foreign entry **at** a reserved name; `Slot.occ = Unowned` was the seat kept for it, and the `ownership` leaf is cut carrying this slice's answer that it is **not enough on its own** — `TT-24.b`'s reason carries the *entry*, and a `SlotContent` cannot name an `Obj` |
| the fifteen other task-tree operations, in the concurrent scope | `Open`/`Classify`/`Release` for an observation, `Mark` for a mutation | no `TT-21` – `TT-23` obligation distinguishes *which* mutation holds the exclusive guard, and a second one is a transition every command in the file pays for. A single mutation is a bulk mark with a one-member plan, so `Mark` is both |
| a bulk mark's target subtree | the task root | no `TT-23` obligation reads a narrower subtree, and a narrower one costs a second `DirObj` every command in the slice would pay for |
| the plan's validation and its first rename | one step | the exclusive guard the first mark holds is the guard the plan was validated under, so nothing **cooperating** can interleave between them (`TT-22`); a `foreign-write` that does is `TT-21.b`'s subject and is modelled |
| `Refused(DestinationOccupied)` in a bulk plan | not represented | the ADR names three ways a plan member is invalid — unaddressable, a `finish` leaf, a destination already occupied. The second is modelled and is enough for `TT-23.a`; the third is the same omission the name slice already records |
| the halt cascade, in the concurrent scope | not represented; `Guarding` pins the tree walkable instead | halting is `TT-02`/`TT-03`'s subject and is already checked there over every read and every mutation. A fifth copy of it across four more transitions would be paid for by every command in the file and answers no `TT-21` – `TT-23` obligation; pinning is what keeps the omission from licensing a mark on a tree grove would refuse — the same narrowing, and the same reason, as `CurrentRootThroughout`. |
| a lock wait | the outcome `Deferred` | **not** one of the catalogue's outcomes, and deliberately so: the closed set covers what a completed invocation returns, and a wait is not a return. The alternative — a guard failure as an *absent* transition — would make `TT-22` true by construction and break this file's totality rule besides |
| a listing's contents | `snapOn` + `snapNm` for an observation's, `planNm` for a plan's own | `TT-21` is a claim about WHICH listing a classification came from. Two rather than one because a plan **outlives** the guard that validated it while an observation's listing dies with its guard — a counterexample this file found and retained. The two sets of fields are **disjointly owned**: `Open`/`Classify`/`Release` write only the first, `Mark` only the second, and each frames the other. That is not tidiness — an under-framed transition is a latent inconsistency *and* a search-space multiplier, and completing the separation took the slice's four slowest commands from 88s, 42s and 37s to under five seconds each |
| the root state `Absent` | not represented | no `TT-` obligation reads it; `SY-05` owns the absent task root, and this model's `TaskRoot` is always on disk |
| the three reserved classes' mechanisms | `Preparing`, `Published`, `Migrating`, each with its own recovery | `TT-18`/`TT-19` are stated over the reserved **class**, deliberately, so that removing migration changes no claim. Three atoms rather than one because *the matching recovery* is `TT-19`'s content and one atom cannot express a mismatch |
| the bytes a fresh scaffold writes | one `ScaffoldD` atom for both the charter and the first leaf | the digest is an opaque equality; no obligation distinguishes the two, and one atom keeps the `2 Digest` bound |
| the `requirements` session kind | `OrdinaryK` | the same row as the nineteen kinds: `PartialScaffold`'s subset needs *one positioned live leaf at position 1 with key 1*, and no obligation reads which ordinary kind it is |
| interruption's cause | one `Crash` action that ENDS an open transaction | `EN-08` grants exactly this and excludes power loss and storage-cache loss. What makes `crash` load-bearing rather than decorative is that it is the only way to leave a transaction incomplete, which is what turns a transient state into a **stable** one |
| a name's rendering to a path | `Rendering.collide`, pinned empty by `EN_12` inside `GroveGrammar` | there is no path in this model, so `EN-12` had nowhere to be false and nothing to control. `collide` is the one place it is given one, and it rides inside the grammar bundle because an unpinned free static relation is paid for by all sixty-eight commands rather than by the one that drops it |

**`TT-11`'s "depends on no state outside the tree" is answered by construction,
not by a command**, and this is the honest place to say so. `precedes` and
`selected` are written as functions of `loc` and `nm` and of nothing else, so
there is no scheduler state for a check to quantify over — a model cannot check
the absence of a variable it does not have. What *is* checked is the falsifiable
half: `TT-14` pins position as the mechanism that orders siblings, independently
of whatever `precedes` happens to say, which is why re-defining `precedes` to
order by key breaks `TT-14` and leaves `TT-11` green.

**What a green run does not prove.** Every result is about the stated bounds: at
most 5 objects, at most 6 filenames, two to six states, one working tree, and —
in the guarding scope — **two** cooperating processes, one process everywhere
else. Two is enough for every `TT-21` – `TT-23` obligation, each of which is
about one operation and one other party; a claim needing a third would need a
third atom and is not one this catalogue makes. A **non**-cooperating writer is
excluded by nothing here and is not meant to be: that is `EN-06`, and `TT-21.b`
exists to say so. Nothing here is a proof about arbitrary trees. In
particular a green `check` at bound *n* says nothing about a defect that needs
*n+1* objects to express, which is why every obligation's witness records the
bound at which it **first** lands, separately from the bound its check ran at.

**The bound at which each witness first lands.** Recorded separately from the
bound its check ran at, which is the pre-registered control on the scope trap. A
check green at bound *n* says nothing about a defect needing *n+1*.

| obligation | witness first lands at | what the extra room buys |
|---|---|---|
| `TT-11` | `3 FileObj, 2 DirObj, 6 Filename, 2 steps` | a node, its charter and a leaf inside it — the walk cannot descend without one |
| `TT-12` | as above | a node whose whole subtree is terminal |
| `TT-13.a` | `2 FileObj, 1 DirObj, 4 Filename, 2 steps` | nothing: two leaves on one level suffice |
| `TT-13.b` | as `TT-13.a` | — |
| `TT-13.c` | `3 FileObj, 2 DirObj, 6 Filename, 2 steps` | *different subtrees*, which needs a second directory |
| `TT-14` | `3 FileObj, 2 DirObj, 6 Filename, **4 steps**` | the second observation: it finds no instance at 2 or at 3 |
| `TT-15.a` – `TT-16.b` | `2 FileObj, 1 DirObj, 4 Filename, 2 steps` | nothing; every one of them is flat |
| `TT-17` | `1 FileObj, 1 DirObj, 3 Filename, **1 steps**` | nothing: a classification is a function of one state, and a legacy root needs one live entry to be *mistakable* for a current one |
| `TT-18` | as `TT-17` | — |
| `TT-19` | `2 FileObj, 1 DirObj, 4 Filename, 2 steps` | the refusal: a refusal is a transition, so the witness needs the second state the classification witnesses do not |
| `TT-20` | `2 FileObj, 1 DirObj, 4 Filename, **3 steps**` | the interruption. Scaffold, crash, and the stable root the crash left behind; it finds no instance at 2. The *uninterrupted* companion witness needs **4**, because the publish must be followed by a state in which the published root is observed |
| `TT-21.a` | `2 FileObj, 1 DirObj, 4 Filename, **5 steps**` | the interval: open · classify · a deferred cooperating mark · classify. Nothing shorter contains "between two classifications" |
| `TT-21.b` | as `TT-21.a` | the same interval, with the `foreign-write` in the deferred mark's place |
| `TT-22.a` | `2 FileObj, 1 DirObj, 4 Filename, **4 steps**` | the run-out state. Two opens reach the situation at three; a third is needed only so the trace can close on an idle self-loop, since a repeated `Open` by a holder is `Deferred` |
| `TT-22.b` | `2 FileObj, 1 DirObj, 4 Filename, **6 steps**` | the whole serialization: open · deferred mark · release · applied mark, plus the run-out state an applied mark cannot supply itself |
| `TT-23.a` | `2 FileObj, 1 DirObj, 4 Filename, **2 steps**` | nothing: a refused mark changes nothing, so it closes on itself, and the invalid plan is reachable at state 0 by `EN-11` |
| `TT-23.b` | `2 FileObj, 1 DirObj, **6 Filename**, **5 steps**` | the interruption **and** the re-run: mark · crash · mark, plus the run-out state. The sixth filename is `CharterF`'s atom, not a claim |

**Symmetry, exact scopes, fairness.** No command uses an `exactly` scope, so
Alloy's symmetry breaking is free to collapse isomorphic instances — which is
sound for the existential witnesses (an instance is an instance) and for the
universal checks (a counterexample has an isomorph). No command states a
fairness assumption, and the guarding slice is where that is worth a sentence
rather than a clause: `TT-22`'s `Deferred` is a wait, so a **liveness** claim
that a waiting operation eventually acquires would need one. The catalogue makes
no such claim — every `TT-21` – `TT-23` obligation is safety or reachability —
so a trace in which one process defers forever is admitted here and refutes
nothing.

## The mutation matrix

The pre-registered control on a green suite: **one mutation per obligation —
break the mechanism, watch *that* check fail, restore**. A suite that stays green
under a deliberate break is measuring nothing, and this model has already
reported itself green while checking nothing at all (below), so the pass is run
**before** the green is believed rather than after.

| obligation | mutation | check falls |
|---|---|---|
| `TT-01.a` | canonicity dropped from the grammar | ✓ |
| `TT-01.b` | canonicity dropped from the grammar | ✓ |
| `TT-02.a` | species mismatch no longer halts | ✓ |
| `TT-02.b` | a node name at a file no longer mismatches | ✓ |
| `TT-03` | `add-leaf` loses its halt guard | ✓ |
| `TT-04` | an insert may rename a foreign sibling | ✓ |
| `TT-05` | promotion issues a fresh key | ✓ |
| `TT-06.a` | the append leaves a gap | ✓ |
| `TT-06.b` | the target itself is not shifted | ✓ |
| `TT-07` | a shift may change the slug | ✓ |
| `TT-08` | promotion issues a fresh key | ✓ |
| `TT-09.a` | the append no longer frames the other names | ✓ |
| `TT-09.b` | the inserted object need not be fresh | ✓ |
| `TT-09.c` | promotion leaves the leaf in place beside its node | ✓ |
| `TT-09.d` | a rewrite may move the entry | ✓ |
| `TT-10` | the algebra's refusal reaches the operator | ✓ |
| `TT-11` | the walk returns the pre-order **maximum** | ✓ |
| `TT-12` | promotion accepts a terminal leaf, taking it off disk | ✓ (see below) |
| `TT-13.a` | eligibility is plain liveness — finish is not reserved | ✓ |
| `TT-13.b` | eligibility is `liveOrdinary` — finish is never returned | ✓ |
| `TT-13.c` | two live finish leaves no longer halt the tree | ✓ |
| `TT-14` | `precedes` orders siblings by **key** instead of position | ✓ |
| `TT-15.a` | a spent tree refuses instead of reporting `Empty` | ✓ |
| `TT-15.b` | a resolution matching nothing refuses instead of `Empty` | ✓ |
| `TT-15.c` | several matches are reported as one | ✓ |
| `TT-16.a` | a resolved `Done` entry is not reported terminal | ✓ |
| `TT-16.b` | a resolved `Abandoned` entry is not reported terminal | ✓ |
| `TT-17` | the format classification reads the **walk**: a halted current-format root reports `Foreign` | ✓ |
| `TT-18` | format classification ordered **before** reserved-witness classification | ✓ |
| `TT-19` | any recovery settles any reserved witness — the *matching* is dropped | ✓ (see below) |
| `TT-20` | the scaffold step publishes the format witness in the same step | ✓ |
| `TT-21.a` | the mark's acquisition ignores another process's guard | ✓ (at `4 steps`; see below) |
| `TT-21.b` | a classification answers from the **live tree** instead of the listing | ✓ (at `4 steps`; see below) |
| `TT-22.a` | a shared open conflicts with another **shared** holder | ✓ |
| `TT-22.b` | the plan is validated **before** the guard is taken, so a refusing mark never acquires | ✓ |
| `TT-23.a` | only the member about to be renamed is validated, not the whole plan | ✓ |
| `TT-23.b` | a run with nothing left to mark refuses `AlreadyTerminal` instead of reporting `Empty` | ✓ |

**The guarding six were each run against every other check in the slice, and
every neighbour stayed green — with one deliberate exception, recorded rather
than contrived away.** `TT-21.a`'s mutation (the acquisition drop) breaks
`TT-22.b` as well, and **no mutation was found that breaks `TT-21.a` alone**. The
reason is structural: every cooperating tree change under a held guard is an
applied mark, and an applied mark has acquired — so `TT-21.a`'s violations are a
subset of `TT-22.b`'s. They are separated from the *other* side instead:
`TT-22.b`'s own mutation, which lets a **refusing** mark skip the acquisition,
breaks `TT-22.b` and leaves `TT-21.a` green, because a refusal changes no bytes.
Two obligations, one mechanism, and the pair of mutations is what shows which
half each claims.

**Fire evidence for each of the six.** `witness_TT_21b` lands under `TT-21.b`'s
mutation, `witness_TT_22b` under `TT-22.a`'s and `TT-22.b`'s, `witness_TT_23b`
under `TT-23.a`'s, `witness_TT_23a` under `TT-23.b`'s, and `witness_TT_22a` under
`TT-21.a`'s. Every one of the six is additionally its own evidence by the
asymmetry below: a failing check has the situation in hand.

**Two of the six survived their first run, and both for the same reason — the
sixth and seventh incidents of the family below, and the third and fourth whose
cause is the BOUND.** `TT-21.a`'s and `TT-21.b`'s checks were written at
`3 steps` like their neighbours, and both mutations reported green exactly as a
real survivor would. `TT-21` is a claim about an **interval** — what happens
between an operation taking its guard and the classifications it makes from it —
and the shortest violating trace is *open · interleave · classify*, which is four
states. At three there is no room for the interleave, the antecedent is
unreachable, and the check is vacuous. Both are caught at `4 steps` in seconds.
The `TT-19` incident said the fix for a bound vacuity is to widen the command
rather than re-aim the mutation; this pair says **when to suspect one**: whenever
the claim's subject is an interval rather than a step, count the states the
interval needs before anything can happen inside it.

**Each of the eleven was run against its neighbour as well as its target, and
every neighbour stayed green.** Two of those pairings are worth more than the
bookkeeping:

- **`TT-11` and `TT-14` are not the same claim, and the mutation pass is what
  shows it.** Re-defining `precedes` to order siblings by key leaves `TT-11`
  green — the walk still returns the `precedes`-minimum, and the check is stated
  in terms of `precedes` — while `TT-14` fails, because `TT-14` names `fPos`.
  That is the whole content of *selection is not a scheduler*: it is the command
  that would catch a scheduler `precedes` had been taught to respect.
- **`TT-13.a` and `TT-13.b` are the two halves of the reservation rule and fail
  independently.** Making finish never selectable breaks `.b` alone; making it
  always selectable breaks `.a` alone.

**The root-identity four were each run against their neighbours, and every
neighbour stayed green.** Two of those pairings carry more than bookkeeping:

- **`TT-17` and `TT-18` are not the same claim.** A classification that consults
  the walk *inside* the format decision breaks `TT-17` — a current-format halted
  root reports `Foreign`, so the witness's content stopped deciding — while
  `TT-18` stays green, because nothing about the *order* of the three
  classifications changed. Reordering them breaks `TT-18` and leaves `TT-17`
  green, for the mirror-image reason.
- **`TT-19`'s mutation survived first time, and the diagnosis is the lasso
  again.** Most of `TT-19` is refusals, and a refusal changes nothing, so the
  check was written at `2 steps` like the observation commands. Its exception
  clause is not a refusal: *the matching recovery* is admitted, and an admitted
  recovery **settles the witness**, which is a tree change. At two states no
  applied recovery exists, that clause is vacuous, and the mutation reported
  green exactly as a real survivor would. At `3 steps` it is caught in 10s.
  This is the fifth incident of the family below and the first whose cause is the
  *bound* rather than an unsatisfiable transition — worth separating, because the
  fix is different: an unsatisfiable mutation must be re-aimed, while this one
  was correct and the command around it was too narrow.

**Fire evidence for each.** `TT-17` and `TT-19`'s witnesses both still land under
their own mutations. `TT-18`'s and `TT-20`'s do not, and cannot: each witness
asserts the very thing its mutation removes. For those two the evidence is the
counterexample itself — a failing `check` has found a real instance, so the
mutated transition demonstrably fired, and `TT-20`'s trace shows it directly
(`InitScaffold`, `Applied`, and `CurrentFmt` in the same state). The vacuity
hazard is asymmetric: a check that *passes* can pass for want of a reachable
situation, while a check that *fails* has the situation in hand.

**Three first attempts had to be replaced, and the reason is a hazard of the
control itself.** Two of them — *insert may drop an object*, *promotion leaves
the leaf on disk without a name* — are **unsatisfiable** against the filesystem
facts: an object cannot leave `onDisk` while keeping a parent, and cannot stay on
it without a name. The mutated transition therefore never fires, its check is
vacuously true, and the run reports green exactly as a *surviving* mutation does.
**A mutation the model cannot execute is not a control**; it is a second vacuity
wearing the first one's clothes, and it has to be told apart from a real survivor
by reading, since the runner cannot see the difference. The third — dropping
species mismatch from the halting reasons — left `TT-02.b` green **honestly**: a
file carrying a node name is in `nodeDirs`, has no charter child, and the tree
halts under `NodeWithoutCharter` instead. That one is a fact about the model
worth keeping: the two reasons overlap on exactly this tree.

**A fourth unsatisfiable mutation, and it is the same hazard as the first two.**
`TT-12`'s first attempt widened promotion's applied guard from `t in liveLeaves`
to *any leaf* — and **survived**. It was not a survivor: `doDecompose` still
carried its `RefAlreadyTerminal` clause, so on a terminal target both
implications fired at once against a `one Result` field, the transition was
unsatisfiable, and the check passed vacuously. The runner reports that exactly as
it reports a real survivor. The mutation that works removes the refusal clause
*as well*, and it is checkable that the mutated transition still fires:
`witness_TT_09c_promotion` finds an instance under it. **A mutation should come
with evidence that it executes** — one existing witness re-run under the mutation
is enough, and it is cheap.

## Counterexamples retained

**None about grove's shipped behaviour**, from any slice. The `TT-01` – `TT-10`
slice produced three findings about the **catalogue**, and all three landed
there; the `TT-11` – `TT-16` slice produced none of either kind — its result is
the mutation matrix, and its one incident is the vacuous `TT-12` mutation above.
The `TT-17` – `TT-20` slice produced one model defect and one bound vacuity; the
`TT-21` – `TT-23` slice produced one model defect (the orphaned plan listing,
below) and two more bound vacuities. See
[`docs/formalism-findings.md`](../../../docs/formalism-findings.md) entries 026,
027, 028 and 029.

**One counterexample, retained.** `TT-06.b`, at the standard bound:

```sh
models/run.sh --scope task-tree --family alloy --no-coverage   # or, for the trace:
java -jar "$ALLOY_JAR" exec -q -n -t text   -c TT_06b_insert_shifts_every_later_sibling crates/grove-task-tree/models/task-tree.als
```

Trimmed to the transition that matters: the task root holds a directory whose
**own** name is foreign; inside it sits one task-shaped entry at position 2. An
`insert-leaf` targets that entry, the shift is performed correctly — the target
moves 2 → 3 and the new leaf lands at 2 — and the level is still not gapless
afterwards, because it never started at 1.

The defect was the model's `entries`, which counted any parseable name
transitively beneath the task root. Grove's walk descends into the task root and
into **nodes** only, so the level does not exist. `visited` is the fix, and the
catalogue's *Entry* now says *reached* rather than *beneath*.
`witness_TT_04_a_task_name_under_a_foreign_directory_is_not_work` keeps the
situation reachable.

**One counterexample about the model, retained, and it is the guarding slice's
only finding.** `TT-21.b`'s check found it in 9s, at `4 steps`, on the first run
after the bound was widened. A bulk mark's plan and an observation's listing were
sharing one field. The trace is three transitions long: a `Mark` sets the plan and
the listing it was validated against; an `Open` and then a `Release` clear the
listing — correctly, since an observation's listing dies with its guard — and the
plan is left alive with nothing to have been validated against.

```sh
java -jar "$ALLOY_JAR" exec -q -n -t text \
  -c TT_21b_every_classification_and_every_plan_member_comes_from_the_one_listing \
  crates/grove-task-tree/models/task-tree.als
```

The defect is a lifetime, and the ADR states it in one clause the model had not
represented: **a plan outlives the guard that validated it.** An observation's
listing does not. `planNm` is the fix — the plan's own frozen record of what the
first guard's listing showed for each member — and it is written and cleared only
by the steps that change the plan, so an `Open` or a `Release` cannot orphan one.
`TT-23.a` gained a conjunct from it: the whole-plan validation **stands** for the
rest of the run, across the guards the ADR gives each mark of its own.

**One counterexample about the model, retained, and it is what `TT-19` is for.**
`InitScaffold` — root initialisation's first step — was written with its own
guard and no root cascade at all, so on a tree holding a reserved witness it
refused with `RefNotAnEntry`: a refusal that names nothing, recovers nothing, and
tells the operator only that its operand was not an entry. `TT-19`'s check found
it in 7s. The fix is the reason `rootRefusal` is split in two: initialisation
runs the **reserved** half and not the **format** half, because a witnessless
root is what it is *for* and an operation that refused there could never create
one. That split is `TT-18`'s ordering made operational — reserved classification
runs for every operation, format classification only for operations that need a
format — and it is a distinction the catalogue states and a model is free to
miss.

**One false-confidence incident, retained because it is the more useful
result.**

An earlier revision of this model ran every behavioural command at
`2 steps` and reported the whole suite green *including every witness*. Both
halves were wrong at once, and each hid the other:

1. `doRewrite` left `Sys.act'` unconstrained when the mark written was `Live`,
   so the predicate admitted a **no-op rename labelling itself any action** —
   `AddLeaf`, `Applied`, and a tree that did not change.
2. At `2 steps` the lasso admits no applied mutation, so every check
   conditioned on `Applied` was vacuous.

The witnesses exist precisely to catch (2). They did not, because (1) let the
solver forge the action label they were looking for, and a forged `AddLeaf`
that changes nothing satisfies the lasso. **The witness discipline is sound only
if the transition relation cannot lie about which action fired**, which is an
argument for writing every action's outcome as a total function of its guard
before writing any command. Found by re-reading the transitions, not by the
suite; it stood for roughly half an hour.

## The assumption mutations this file runs

Separate from the mutation matrix above, and a different control. The matrix
asks whether the model's own mechanisms are load-bearing; these ask whether the
[environment assumptions](../../../docs/specs/semantic-contract.md#environment-assumptions)
are — the pre-registration's control on *agreement mistaken for proof*, since
both families descend from one document and an error smuggled in as an
assumption produces two models that agree.

The Alloy-owned rows this file has run are `EN-04`, `EN-07`, `EN-08`, `EN-12` and
`EN-14`. `EN-11` is `ownership`'s.

| id | class | command | result |
|---|---|---|---|
| `EN-04` | counterfactual-capability | `expect_unreachable_EN_04_promotion_is_never_observed_half_applied` | no instance, as expected |
| `EN-07` | premise-break | `expect_unreachable_EN_07_an_outer_guard_is_never_held_across_a_mark` | no instance, as expected — and **no `TT-` obligation depends on it** |
| `EN-07` (fire evidence) | — | `witness_EN_07_the_outer_guard_is_admitted_once_two_descriptions_share_a_lock` | instance found |
| `EN-08` | exercise-removal | `expect_unreachable_EN_08_the_interrupted_initialisation_witness_needs_crash` | no instance, as expected |
| `EN-08` | exercise-removal | `expect_unreachable_EN_08_the_interrupted_bulk_mark_witness_needs_crash` | no instance, as expected |
| `EN-12` | premise-break | `expect_fail_EN_12_TT_01a_a_name_that_renders_as_two_components` | counterexample found, as expected |
| `EN-14` | premise-break | `expect_fail_EN_14_TT_22b_with_no_root_to_guard_a_mark_lands_mid_observation` | counterexample found, as expected |

**`EN-04` — there is no atomic replacement of a file by a differently named
directory.** This model already carries the *candidate* rather than the
incumbent: promotion is one step, which the abstraction table records and which
the assumption table names as the Alloy-owned mutation. The control is therefore
that the capability is really in force — the half-applied promotion the
incumbent would expose is unreachable — and the retained obligations `TT-07`,
`TT-08` and `TT-09` are green **in this same file, at no wider bound**, which is
what the counterfactual class asks for. The *exercised* half is `TT-02.b`, whose
witness lands by hand edit (`EN-11`) rather than through a half-promoted entry.

**The finding is that no `TT-` obligation depends on `EN-04`**, which is what the
assumption table predicts in its own expected-result column: *`EN-04` buys step
count, not safety*. An assumption carrying no weight is a legitimate result of
this control and not a defect in it, and the row is recorded rather than dropped
so the next family has the same claim to check against.

**`EN-12` — a name renders as exactly one path component.** This assumption had
**nowhere to be false** in the model as it stood, which is itself worth writing
down: there is no path here, only a `Filename` in a directory, so the filesystem
fact supplied the assumption for free and no command could control it. The
mutation therefore needed a mechanism built for it — `Rendering.collide`, a
rendering under which two distinct spellings reach one entry, which is what a
separator inside a name's part buys — and `TT-01.a` is restated over
*denotation* rather than over *reading* so that it has somewhere to fail. It
fails, which is the assumption table's stated expected result.

`collide` rides **inside** `GroveGrammar` (as `EN_12`) rather than beside it, and
the reason is arithmetic rather than taste: it is a free static relation over
`Filename -> Filename`, so leaving it unpinned would be paid for by all
sixty-eight commands instead of by the one command that drops it.

**`EN-07` — two open descriptions of one directory do not share a lock.** It is
the assumption behind
[`bulk-marks-are-not-atomic`](../../../docs/adr/bulk-marks-are-not-atomic.md)'s
third rejected option: hold Grove's own exclusive guard around the whole bulk run
and let the library take its guard inside it. Both `flock` the directory holding
the tree root, so the inner acquisition deadlocks against the outer one — and the
control is that the deadlock is really in force, which is what
`expect_unreachable_EN_07_an_outer_guard_is_never_held_across_a_mark` says. Its
fire evidence is the companion witness: once the two descriptions **do** share a
lock, the nested acquisition is admitted, so the command above is unreachable
because of the assumption rather than because the situation cannot be built.

**The finding is that no `TT-` obligation depends on `EN-07`**, and the assumption
table predicts it: its own expected-result column names `SY-11.b`, the lifecycle
scope's. Every `TT-21` – `TT-23` check leaves `EN_07` **free**, so all six are
checked over the broken assumption as well as over the incumbent, and all six are
green either way. That is the same class of result as `EN-04`'s — an assumption
carrying no weight in this scope — and it is a legitimate outcome of the control
rather than a defect in it. What the assumption *does* buy is the window the ADR
names: the interval between two marks in which another writer may arrive, which
is `SY-11.b`'s subject and not this file's.

**`EN-08` — interruption may occur between any two steps.** Exercise-removal, run
against the two **named witness sets** it controls rather than against the whole
file: `TT-20`'s interrupted initialisation and `TT-23.b`'s interrupted bulk mark.
With `crash` removed both are unreachable, which is what the two
`expect_unreachable_EN_08_*` commands report.

The other half of the assumption table's expected result — *every property check
stays green* — needs no run of its own, and the argument is worth stating because
it is cheaper than eleven more commands. **No check in this file asserts
`EN_08`**, so each is already checked over the traces that contain `crash` and
over the traces that do not; green over the superset is green over the subset.

**`EN-14` — the working-tree root exists before the task root and outlives its
deletion.** It is what the guard is held *on*, and it cannot be the task root:
finish deletes that, and the lease outlives it. Remove the working-tree root and
there is nothing to `flock` — no guard is taken, the compatibility test has no
subject, and a mutation lands while an observation is mid-flight. `TT-22.b`
fails, in 3s. That is the `TT-` half of the expected result; the column's own
entry names `SY-01`, whose second driver is the lifecycle scope's concern.
