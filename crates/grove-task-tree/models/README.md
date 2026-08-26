# grove-task-tree — models

The task-tree scope of [the semantic
contract](../../../docs/specs/semantic-contract.md): the `TT-` claims, checked
independently by each model family. This directory exists before the crate does,
which is deliberate — the model is what the crate will be cut against.

Run them from the repository root:

```sh
models/run.sh --scope task-tree --family alloy
```

## What is covered, and what is not

| family | file | obligations |
|---|---|---|
| Alloy 6 | `task-tree.als` | `TT-01` – `TT-25` |
| Quint | `task-tree.qnt`, `task-tree-controls.qnt` | `TT-01` – `TT-25` |

**Both columns of the task-tree scope are complete**, and each family's own
invocation is what says so: with `--family <fam>` the coverage matrix holds only
that family's cells, so it runs **with coverage asserted** and exits 0. The
unqualified `models/run.sh` stays red until the **finish** and **lifecycle**
Quint columns land, and that redness is the truth about the repository rather
than a gate to be tolerated — which is why a phase's green is spelled as a named
subset rather than as an expected-red run. A suite anyone is told to ignore the
colour of is not a suite.

**Declared gaps** — two, both `TT-24`'s, and both because the context the
obligation names is not this model's. The runner reads them from this file, in
one shape:

```md
- **GAP** alloy `TT-nn.x` (inexpressible|abstracted|out-of-bounds|tool-limited) — reason.
```

- **GAP** alloy `TT-24.c` (out-of-bounds) — the obligation's antecedent is *inside a finish or recovery transaction* and its outcome is `Blocked(OwnershipConflict)`. This model has no finish transaction and no `Blocked` in its `Result` set; adding a ninth outcome so the cell could be filled would answer `TT-24` by construction, which is what the catalogue's *one artifact, three contexts, one decided outcome* table exists to prevent. It belongs to `crates/grove-finish/models/`, beside `FN-22` and `FN-25`.
- **GAP** alloy `TT-24.d` (out-of-bounds) — the obligation's subject is the quarantine reaper, which is `FN-21`'s machinery: a sweep, a quarantine, and a per-entry ownership proof, none of which this model has. Stated separately from `TT-24.c` rather than folded into it, because the two are out of reach for different reasons — one lacks an outcome, the other lacks a subject.

**Both gaps are an observation about the catalogue, not only about this model.**
`TT-24` is the one `TT-` claim whose obligations are stated over `FN-` contexts,
so two of its four cells can only ever be filled by the finish scope while the
runner's placement rule sends every `TT-` command to this directory. Whether
`TT-24.c` and `TT-24.d` should be re-stated as `FN-` obligations is
`formal-synthesis-k16`'s to settle; the gaps are honest either way, and the
quint column will meet the same wall for the same reason.

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

The **ownership** slice (`TT-24` – `TT-25`) is the one slice that runs *wider*
than the file's common shape rather than narrower, and both widenings were asked
for by a mutation rather than by a claim.

- **`3 DirObj` for `TT-25.b`**, where nothing else in the file has needed more
  than two. The claim is *a node with a live leaf **anywhere** beneath it*, so a
  counterexample needs a node, a node beneath it, and the task root. At
  `2 DirObj` the live leaf is necessarily a direct child, the subtree reading and
  the children reading of done-ness agree, and the mutation aimed at this
  obligation survives for want of a place to differ. It is the depth analogue of
  the guarding slice's interval rule: **count the depth the claim quantifies
  over before writing the bound**.
- **`3 FileObj` for `TT-24.a`'s check**, where the claim mentions two objects —
  an actor and something unprovable. The bound is set by the transition most
  likely to violate it rather than by the claim: `InitScaffold` introduces a
  charter and a first leaf of its **own** before it can trample anything, which
  is three files before the tree holds one. At `2 FileObj` the mutation could not
  fire and reported green exactly as a survivor would. The generalisation is a
  second predictor beside the interval one: **an obligation's bound must hold the
  machinery of the transitions it quantifies over, not only the objects it
  names.**

`TT-24.b`'s commands stay at the guarding slice's narrow `2 FileObj, 1 DirObj,
4 Filename`: an occupant and a live leaf is the whole situation, and it is flat.

**Every `TT-24` and `TT-25` command pins the process scope**, and it is a
correctness control rather than a habit. Both claims are single-process, and in
the concurrent scope no grove mutation exists at all — so an unpinned witness is
unreachable and an unpinned check vacuous. `TT-25`'s two carry
`CurrentRootThroughout` (which carries `SingleProc`); `TT-24`'s carry
`SingleProc` alone, because `TT-24` is stated over roots `CurrentRootThroughout`
excludes — an occupied reserved name is not a root grove may act on.

**Runtime, and a caveat that governs every absolute number in this section.**
The Alloy scope of this directory is **103 commands** and costs **6888s CPU**
end to end (1h 57m wall; CPU is the fairer number).

**Do not compare a figure here against one measured in another session.** The
numbers below were taken across five slices and the host is not the same
instrument each time: `TT-11`'s check, unchanged since the selection slice,
costs **61s** in the measurement that produced this section's earlier figures and
**77s on that same unmodified file** when re-run during the ownership slice —
~24% drift, independently confirmed by `witness_TT_07` landing at 668s against a
drift-adjusted prediction of ~680s. A slice's real imposition is therefore an
**A/B on one host in one sitting**, old file against new, and that is how the
ownership slice's figures below were taken. Corrected for drift, the whole scope
went from ~4970s to 6888s: about **+15%** on the pre-existing commands plus
~860s for the fifteen new ones.

**And one sentinel is not enough — the ownership slice is what established
that.** `TT-03` was adopted as the file's standing sentinel because it is the
*tightest* command here, run one filename short of its neighbours. That makes it
sensitive to a new **transition**, which is what the two previous slices added,
and nearly blind to new **state**. The ownership slice added no transition and
two `lone Obj` fields plus a ninth `Result` atom — state present in every state of
every trace — and the A/B says so plainly:

| command | pre-slice | with ownership | Δ |
|---|---|---|---|
| `TT-03` (tightest) | 156s | 138s | −12% |
| `TT-11` | 77s | 75s | −3% |
| `TT-15.a` | 51s | 56s | +10% |
| `witness_TT_07` (largest) | 668s | **987s** | **+48%** |

Read `TT-03` alone and this slice looks free. It is not: the file's single
largest command is half again as expensive, because state is paid where the trace
is **widest**. **Measure the largest command as well as the tightest** — the
first measures the cost of state, the second the cost of transitions.

One command is a large fraction of the total: `witness_TT_07_shift_across_every_species` runs about **sixteen minutes** (nine
before this slice, and see the A/B above), because
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
| the reserved NAME, as a spelling | `Slot.occAt`, a pointer from the slot to the object sitting there | `TT-24.b`'s refusal reason carries the **entry**, so the occupant must be a real filesystem object — but what makes it *reserved* need not be a `Filename`. What makes a witness reserved is that the slot holds it; what makes an occupant reserved is that the slot points at it, and the symmetry is the argument. `one sig ReservedF in Filename` — the reserved spelling as an atom — would consume a `Filename` atom in **every** command in the file, and the nine-minute `TT-07` witness runs at six with nothing spare |
| the root state `Blocked`, and a finish or recovery transaction | not represented; `TT-24.c` is a declared gap | there is no transaction of that kind here, and a ninth `Result` atom invented to fill the cell would answer `TT-24` by construction. See *Declared gaps* |
| the quarantine reaper | not represented; `TT-24.d` is a declared gap | `FN-21`'s machinery — a sweep, a quarantine, a per-entry ownership proof. The gap is stated separately from `TT-24.c`'s because the two are out of reach for different reasons: one lacks an outcome, the other lacks a subject |
| a marked node | **not spellable**: `isShaped` gives a `NodeSp` name no `fOut` field at all | `TT-25`'s prohibition is therefore answered by construction, and the paragraph below says so rather than a command pretending to check it |
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

**`TT-25`'s "a node is never marked" is answered by construction too, and it is
the second entry in this paragraph's family.** A node name carries no outcome
infix — `isShaped` says `f.fSpec = NodeSp implies (no f.fKind and no f.fOut)` —
so this model cannot spell a marked node, and no mutation of an *action* can
produce one. What is checked instead is what "derived" actually forbids, and both
halves are falsifiable: the transition that makes a node done writes **nothing**
to the node (`TT-25.a`), and done-ness reads the **whole** subtree (`TT-25.b`,
stated over `d.^(~loc)` rather than over `nodeDone`, exactly as `TT-14` names
`fPos` rather than `precedes`, so that re-defining the mechanism has somewhere to
fail).

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
| `TT-24.a` | `2 FileObj, 1 DirObj, 4 Filename, **2 steps**` | nothing: a refused mutation changes no bytes, so the trace closes on itself. Its **check** runs at `3 FileObj` for the mutation's sake, not the witness's |
| `TT-24.b` | `2 FileObj, 1 DirObj, 4 Filename, **2 steps**` | nothing, and for the same reason: the occupant and one live leaf is the whole situation, and the refusal is idempotent |
| `TT-25.a` | `3 FileObj, 2 DirObj, 6 Filename, **3 steps**` | the retirement: a node, its charter, the leaf inside it, and the state the applied rename reaches |
| `TT-25.b` | `3 FileObj, **3 DirObj**, 6 Filename, **1 steps**` | the **depth**. A node, a node beneath it, and a live leaf in the second — which is the only shape a done-ness reading only the node's children gets wrong |

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
| `TT-24.a` | `initialise-root` clears a directory it did not create | ✓ (at `3 FileObj`; see below) |
| `TT-24.b` | the refusal treats someone else's bytes as a recoverable witness — `WitnessPending`, naming a recovery | ✓ |
| `TT-25.a` | the retire that empties a node records done-ness in the node's own bytes | ✓ |
| `TT-25.b` | done-ness reads only the node's **children** instead of its subtree | ✓ |

**The ownership four each fired, and each has POSITIVE evidence that it did** —
no reliance on the failing-check asymmetry this time, which is worth recording
because the two earlier slices leaned on it for four of their ten mutations. Each
mutant carries a `fire_` probe exhibiting the mutated transition in an instance:
an `InitScaffold` applied over a non-empty root, a `groveAct` refusing
`RefWitnessPending` under an `Unowned` slot, a retire that changes its node's
digest, and a node reported done with a live leaf beneath it. All four land.

**`TT-25.a` and `TT-09.d` are one framing seen from two sides, and the pair of
mutations says which half each claims.** `TT-25.a`'s mutation — the retire that
empties a node records done-ness in the node's own bytes — breaks `TT-09.d` as
well, and it must: in this model the only transition that can make a node done is
a rewrite, and `TT-09.d` already says a rewrite touches nothing but its target.
They separate from the other side, which is what makes the pair a control rather
than a duplicate: `TT-09.d`'s own mutation — *a rewrite may move the entry* —
breaks `TT-09.d` and leaves `TT-25.a` **green**, because moving the target writes
nothing to the node above it. This is the second such pairing in the file, after
`TT-21.a`/`TT-22.b`, and it is recorded rather than contrived away for the same
reason.

**Each was run against its neighbours, and one pairing carries more than
bookkeeping.** `TT-24.a` and `TT-04` are both about bytes grove must not touch,
and they separate cleanly *because of the scope*: `TT-04` carries
`CurrentRootThroughout`, which excludes the root-lifecycle actions, so an
`initialise-root` that tramples a foreign entry is invisible to it and visible to
`TT-24.a`. That is the whole of what `TT-24.a` adds to a file that already
had `TT-04` — not a wider class of protected object, but a wider class of
**action**, and the roots `TT-01` – `TT-16` are not stated over.

**`TT-24.a`'s mutation survived its first run, and it is the ninth incident and
the FIFTH whose cause is the bound.** The check was written at the guarding
slice's `2 FileObj`, which is what the claim mentions: an actor and something
unprovable. The transition that violates it is `InitScaffold`, which brings a
charter and a first leaf of its own — so the situation needs **three** files
before the tree holds one, the mutated transition could not fire, and the check
reported green exactly as a real survivor would. Caught at `3 FileObj` in 22s.
Entry 029's predictor was about *states*; this one is the same shape in the other
dimension, and the two combine into one authoring rule: **the bound must hold the
machinery of the transitions the obligation quantifies over, not only the objects
the obligation names.**

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
below) and two more bound vacuities; the `TT-24` – `TT-25` slice produced one
model defect (the subset antecedent, below), one bound vacuity, and one finding
about the catalogue's own assumption table. See
[`docs/formalism-findings.md`](../../../docs/formalism-findings.md) entries 026,
027, 028, 029 and 030.

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

**One model defect the ownership slice introduced and its own witnesses caught,
retained because the failure mode is the file's oldest one wearing new
clothes.** `reservedRefusal` had to split — a reserved *witness* refuses with
`WitnessPending`, an occupant grove cannot classify with `ReservedNameOccupied` —
and the witness half was first written:

```alloy
Slot.occ in WitnessClass implies { Sys.res' = RefWitnessPending ... }
```

`in` is **subset**, and `Slot.occ` is a `lone` field whose empty value is a
subset of every set. On a root with no reserved artifact at all the antecedent is
therefore **true**, every ordinary transition was forced to refuse
`RefWitnessPending` against its own applied branch, and the transition relation
became **unsatisfiable**. All four of the slice's new checks reported green while
checking nothing whatever, and `TT-19`, `TT-24.a`, `TT-24.b`, `TT-25.a` and
`TT-25.b` would all have shipped that way.

What caught it was not a check but the two **witnesses that did not land** — and
then a probe run against the pre-change file, which found the same situation in
7s and localised the change that had removed it. The spelling that says what was
meant is `some (Slot.occ & WitnessClass)`, which is what `doRecover` had said
correctly all along. The lesson is narrower and more useful than *be careful with
`in`*: **a `lone` field's emptiness makes `in` an antecedent that fires when
nothing is there**, and the symptom is not a failing check but a file that has
quietly stopped being able to do anything.

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

The Alloy-owned rows this file has run are `EN-04`, `EN-07`, `EN-08`, `EN-11`,
`EN-12` and `EN-14` — all five of them, with `EN-11` the last to land.

| id | class | command | result |
|---|---|---|---|
| `EN-04` | counterfactual-capability | `expect_unreachable_EN_04_promotion_is_never_observed_half_applied` | no instance, as expected |
| `EN-07` | premise-break | `expect_unreachable_EN_07_an_outer_guard_is_never_held_across_a_mark` | no instance, as expected — and **no `TT-` obligation depends on it** |
| `EN-07` (fire evidence) | — | `witness_EN_07_the_outer_guard_is_admitted_once_two_descriptions_share_a_lock` | instance found |
| `EN-08` | exercise-removal | `expect_unreachable_EN_08_the_interrupted_initialisation_witness_needs_crash` | no instance, as expected |
| `EN-08` | exercise-removal | `expect_unreachable_EN_08_the_interrupted_bulk_mark_witness_needs_crash` | no instance, as expected |
| `EN-12` | premise-break | `expect_fail_EN_12_TT_01a_a_name_that_renders_as_two_components` | counterexample found, as expected |
| `EN-14` | premise-break | `expect_fail_EN_14_TT_22b_with_no_root_to_guard_a_mark_lands_mid_observation` | counterexample found, as expected |
| `EN-11` | exercise-removal | `expect_unreachable_EN_11_a_species_mismatch_needs_a_hand_edit` | no instance, as expected |
| `EN-11` | exercise-removal | `expect_unreachable_EN_11_a_malformed_node_hiding_live_work_needs_a_hand_edit` | no instance, as expected |
| `EN-11` | exercise-removal | `expect_unreachable_EN_11_two_live_finish_leaves_need_a_hand_edit` | no instance, as expected |
| `EN-11` | exercise-removal | `expect_unreachable_EN_11_an_occupied_reserved_name_needs_a_hand_edit` | no instance, as expected |
| `EN-11` | exercise-removal | `expect_unreachable_EN_11_a_leaf_two_levels_deep_needs_a_hand_edit_at_this_bound` | no instance, as expected **at this bound** |
| `EN-11` (fire evidence) | — | `witness_EN_11_groves_own_actions_still_build_a_tree_without_a_hand_edit` | instance found |
| `EN-11` (**finding**) | — | `witness_EN_11_a_resolved_terminal_entry_needs_no_hand_edit` | instance found — the assumption does **not** control `TT-16` |

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

**`EN-11` — any well-formed tree is reachable by hand edit.** Exercise-removal,
and it is the row that took two attempts to *remove anything*.

**The assumption is realised in this file in two places, not one.** The
`hand-edit` action is the obvious one. The other is the **unconstrained initial
state**, which the bounds paragraph above rests on in as many words — *every
single transition is reachable from state 0, so a one-step property needs no
run-up*. That argument **is** `EN-11`, cashed out as a modelling decision. A
switch that guarded only the action left every named witness reachable at state
0 and reported green while removing nothing at all. `not EN_11` takes both: no
`hand-edit`, and a world that starts as Grove would have found it before it had
done anything — an empty task root, no format witness, nothing at a reserved
name.

**The scope is `SingleProc`, not `CurrentRootThroughout`, and `5 steps`.** Under
`CurrentRootThroughout` the root-lifecycle actions are excluded, so an empty
start could never be populated at all and every command would be unreachable for
the trivial reason. The commands admit `initialise-root` instead, and five states
is the shortest run-up in which Grove builds anything: scaffold, publish, one
mutation, and the state that closes the lasso.
`witness_EN_11_groves_own_actions_still_build_a_tree_without_a_hand_edit` is what
shows the scope is not simply dead — an applied `add-leaf` from an empty root,
with `hand-edit` gone.

**Five of the six named witness sets are controlled, and `TT-16` is not — which
is this row's finding.** A species mismatch, a malformed node hiding live work,
two live finish leaves, an occupant at a reserved name and a live leaf two levels
deep are all unreachable with `hand-edit` removed. A **resolved terminal entry**
is not: Grove's own actions build one — allocate, retire, resolve — so
`TT-16`'s witness never needed the assumption. The catalogue's `EN-11` row listed
it and has been corrected; the model ships the positive control rather than an
`expect_unreachable_` written to the table instead of to the model.

One of the five is weaker than the other four and says so in its own name.
`expect_unreachable_EN_11_a_leaf_two_levels_deep_needs_a_hand_edit_at_this_bound`
is unreachable because two promotions need two nodes, two charters and two leaves
against a scope of three files — not because Grove could not build such a tree
given room. It is recorded as an unreachability **at this bound**, which is the
distinction the whole `expect_unreachable_` form rests on.

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

---

# The Quint column — `task-tree.qnt`

Written from [`docs/specs/semantic-contract.md`](../../../docs/specs/semantic-contract.md)
alone, under the independence protocol
([`docs/formalism-findings.md`](../../../docs/formalism-findings.md), *Experiment 2 —
pre-registration*, **Independence protocol**): this column's session opened no
`.als` file, no model-directory `README.md` and no experiment entry from the
Alloy column's range. Everything below is what a second family reached on its
own.

## Run line

```sh
models/run.sh --scope task-tree --family quint
```

Two files: [`task-tree.qnt`](task-tree.qnt) carries the parameterised library,
the `base` instance and the `verify_small` model-checking instance;
[`task-tree-controls.qnt`](task-tree-controls.qnt) carries every assumption
mutation and focused scenario. The split is not tidiness — see *Verification*.

Coverage is asserted: all 43 `TT-` obligations are answered by a property
command and at least one witness, and there are no declared gaps. **In both
directions**: an `inv_`/`wit_` command naming an obligation the catalogue does
not define is fatal, and a command citing a *claim* rather than one of its
sub-identities is reported as crediting no cell. That direction is not a claim
about the runner to be taken on trust — see *The runner's own controls* below.

The controls on the runner itself:

```sh
models/run-controls.sh
```

Knobs, all environment variables read by `models/run.sh`: `QUINT_SAMPLES`
(default 8000), `QUINT_STEPS` (default 24), `QUINT_SEED` (a fixed default, so a
green run is replayable and a red one reproduces), `QUINT_VERIFY` (default 0 —
see below).

## Verification

- **VERIFY** quint (cost-limited) — model checking is REACHABLE and NOT
  AFFORDABLE for this subject, and the two are different findings. `quint verify`
  reaches Apalache's `BoundedChecker` and checks state invariants correctly;
  what it cannot do is finish. Measured on a 16-core / 128 GB host: the full
  `base` instance exhausts a 4 GB JVM heap at `--max-steps=3`, and the
  deliberately tiny `verify_small` instance below — 6 objects, depth 2, a
  three-action menu, no hand-edit — reached `State 3` and then ran past 25
  minutes without completing `--max-steps=3`. The cost is in the encoding, not
  the bounds: every transition quantifies over `reached`, a bounded unrolling of
  set union/filter/flatten that a simulator skips and a symbolic backend must
  encode in full at every step. **Every `TT-` property in this column is
  therefore established by bounded randomized simulation, and no green run here
  is a proof over reachable states.** What would change the answer: an encoding
  of tree reachability that is not an unrolled fixpoint — which is a modelling
  change with its own cost, and one `formal-synthesis-k16` should weigh rather
  than this leaf. `QUINT_VERIFY=1` runs it anyway; `QUINT_VERIFY_STEPS` and
  `JVM_ARGS` (default `-Xmx16G`) set the depth and the heap.

Two barriers were removed on the way to that measurement, and both are durable:

- **The file split.** In one file holding the library, `base`, six assumption
  mutations, a model mutation and four focused scenarios, `quint verify` 0.32.0
  died in its own result reporter — `RangeError: Invalid string length` out of
  `json-bigint/stringify`, at every depth including `--max-steps=2` — because the
  intermediate JSON exceeded V8's maximum string length. That is a reporting
  failure with nothing to do with the model, and it made the whole subject look
  unverifiable. The controls now live in
  [`task-tree-controls.qnt`](task-tree-controls.qnt), and `task-tree.qnt` is
  within Apalache's reach.
- **`gapless` no longer writes `1.to(n)`.** Apalache refuses a non-constant
  integer range outright (`Expected a constant integer range in [ .. ]`), so one
  natural spelling of one predicate put the entire model beyond model checking.
  It is now stated as a cardinality plus a bound — `n` distinct positions all in
  `[1, n]` ARE `1..n` — which is equivalent and constant-range-free.

`models/run.sh` reads the `VERIFY` line above and prints it on every Quint run,
and **fails** a scope whose Quint models exist and which declares nothing — so a
limit on model checking names itself rather than passing as silence.

## The runner's own controls

```sh
models/run-controls.sh
```

A suite that has never been shown to fail is not evidence, and neither is a
runner. `models/run.sh` claims four things beyond running commands, and each is
a claim that it goes RED in a situation nobody normally creates. Seven controls
mutate a **copy** of the repository — the real catalogue, the real models — and
assert the named fatal diagnostic:

| control | mutation | asserted |
|---|---|---|
| `invented-obligation` | a command named `inv_TT_99_…` | `names TT-99, which the catalogue does not define`, run red |
| `claim-level-citation` | a command named `inv_TT_24_…`, a claim with sub-identities | reported as crediting no cell, and **not** as invented |
| `deleted-witness` | `TT-19`'s last witness removed | `TT-19  quint:NO-WITNESS`, run red |
| `deleted-property` | `TT-19`'s last property removed | `TT-19  quint:NO-CHECK`, run red |
| `dead-quint-launch` | a `quint` on `PATH` that cannot start | `quint failed to launch`, **exit 2** |
| `dead-quint-run` | a `quint` whose `--version` answers and whose `run` dies | `tool failure, not a result`, **exit 2** |
| `dead-backend` | Apalache under `JVM_ARGS=-Xmx6m` | `tool failure, not a result`, **exit 2** |

Three of them — `dead-backend`, `dead-quint-run` and `invented-obligation` —
are worth reading twice, because each was a **false green** before this chain's
review found it:

- a non-zero `quint verify` whose wording was not on a five-string list was
  recorded as "model-checked … no counterexample", so a JVM too small to read
  its own jar reported 43 verified properties;
- a non-zero `quint run` was read as "violated", which in a **premise-break**
  control is recorded as *passing* — a dead tool reporting that an assumption
  was carrying weight;
- a syntactically valid invented obligation was counted under a matrix key
  nothing reads, so the model-to-catalogue direction the runner advertised was
  not asserted at all.

Now a verdict requires Apalache's own counterexample report (`[violation] Found
an issue`, `❌ <invariant>`) or `error: Invariant violated` from the simulator,
and **every other non-zero exit aborts**. The default for an unrecognised
failure is death, not green.

**What these controls do not show.** They run at `QUINT_SAMPLES=1
QUINT_STEPS=1`, so a full accounting pass takes about a minute rather than
twenty; at that budget the unmutated suite's witnesses do not land, so each
assertion is on the exact diagnostic line rather than on the exit code alone.
The unmutated baseline is the full-budget run recorded below, which exits 0 and
prints none of those lines.

**And they do not show anything about the other family's column.** The two
drivers share `resolve_ob`, so the reverse-coverage direction now binds Alloy
commands too — and the session that wrote these controls was under the
independence barrier and opened no `.als` file, so it could neither run
`--family alloy` nor inspect what it declares. The check is deliberately as
lenient as the Q4 matrix already is (an exact manifest entry, or a claim whose
sub-identities the catalogue defines), so an Alloy command can only newly fail
if it names an obligation the catalogue does not define at all — which is the
defect the check exists to catch rather than a regression in it.
**`cross-model-replay-k15` is where the barrier comes down, and re-running
`models/run.sh --family alloy` under this check is its first obligation.**

## What the model is, and what it is above

`ordinal-fs-tree`'s ordered-tree algebra — append, insert, promotion, rewrite,
and the shifting and key allocation they imply — is **assumed**, not re-derived;
it is checked in [`docs/ordinal-fs-tree/models/operations.qnt`](../../../docs/ordinal-fs-tree/models/operations.qnt).
`TT-09` is the seam itself, stated as the claim that every mutation Grove makes
is one of those four operations plus a domain precondition. No second filesystem
model is grown, which is this leaf's own instruction.

**Every action is total.** No action does nothing when its guard is false: each
computes a `Decision` from its snapshot and transitions in every case, so a
refusal is a value rather than an absent transition. That is what makes the
refusal claims falsifiable at all.

## Instances

| module | what it is | why |
|---|---|---|
| `base` | every assumption granted | the run every `TT-` obligation is checked in |
| `scenario_bulk` | action menu narrowed to bulk marks | `TT-23.b`'s witness lands in under 1 trace in 4000 unfocused |
| `scenario_species_shift` | narrowed to inserts into a mixed-species directory | `TT-07`'s witness, 0.03% unfocused |
| `scenario_promote` | narrowed to decomposition | `TT-08`, `TT-09.c`, 0.06% unfocused |
| `scenario_foreign_sibling` | narrowed to inserts beside a foreign entry | `TT-04`'s renumbering witness, 0.08% unfocused |
| `relax_EN_01` | rename observable half-applied | premise-break |
| `relax_EN_06` | non-cooperating writer removed | exercise-removal |
| `relax_EN_08`, `relax_EN_08_bulk` | `crash` removed | exercise-removal |
| `relax_EN_10` | an entry removed | premise-break |
| `relax_EN_11`, `relax_EN_11_grove_built` | `hand-edit` removed | exercise-removal, plus its positive half |
| `relax_EN_13` | reaper sweeps the reserved namespace | premise-break |
| `mutant_two_listings` | later steps classify from the LIVE tree | a control on the **model**, not the world |
| `mutant_bulk_strict` | bulk validation without target-state idempotence | a control on the **model**: `TT-23.b` must die without the requirement the catalogue implies and does not state |
| `verify_small` | 6 objects, depth 2, a three-action menu | the model-checking instance; see *Verification* for what it can and cannot finish |

A `scenario_` instance removes no behaviour: it narrows the *search*, and every
claim is still checked unfocused in `base`.

**Which module a command runs in** is decided by one rule, defined in
[`models/run.sh`](../../../models/run.sh) under *THE MODULE RULE* and cited
rather than restated here: a `relax_`, `mutant_` or `scenario_` instance carries
only the commands written inside it; a `verify_` instance is model-checked and
inherits the library's **property** commands only; every other instance inherits
all of the library's commands. So `base` runs 83 inherited commands,
`verify_small` owns none of its own and inherits 43 properties — which is
correct rather than zero-work — and each control module runs exactly what is
written in it.

## The controls, and what they establish

Each **premise-break** control names an obligation that must DIE when its
assumption is removed. All six do:

| control | obligation | result |
|---|---|---|
| `inv_fail_EN_01_TT_20_a_torn_witness_is_not_a_partial_scaffold` | `TT-20` | violated |
| `inv_fail_EN_10_TT_05_allocation_reissues_a_removed_entrys_key` | `TT-05` | violated |
| `inv_fail_EN_10_TT_12_a_terminal_entry_is_removed` | `TT-12` | violated |
| `inv_fail_EN_13_TT_04_the_sweep_deletes_foreign_bytes` | `TT-04` | violated |
| `inv_fail_EN_13_TT_24d_the_reaper_stops_declining` | `TT-24.d` | violated |
| `inv_fail_MUT_TT_21a_two_listings_disagree` | `TT-21.a` | violated |
| `inv_fail_MUT_TT_23b_strict_validation_cannot_converge` | `TT-23.b` | violated |

`mutant_two_listings` is the one that matters most for reading the rest. `TT-21`
is otherwise true **by construction** in an executable model — every
classification is computed from `op.snap` because that is how the model is
written — and a claim true by construction is the pre-registration's
*vacuous invariant* hazard wearing a green tick. The mutant makes later steps
classify from the live tree, and the claim dies. Without that instance, `TT-21`
would be reported green on no evidence.

`mutant_bulk_strict` is the second of that kind, and it exists for the same
reason at a different claim. `TT-23.b`'s convergence rests on a requirement the
catalogue implies and does not state — that a bulk member already in the plan's
**target** state is admissible and a no-op. With `BULK_TARGET_IDEMPOTENT` off, a
bulk plan is validated exactly the way a single mark is, the identical re-run is
refused `AlreadyTerminal`, and `TT-23.b` dies. Until that instance existed the
requirement was asserted only by the way the model happened to be written.

**Bounded unreachability, stated as what it is.** The `wit_unreach_` controls
are randomized simulation, so a zero count is evidence that the witness is
unreachable *within* 8000 samples at depth 24 — never a proof that it is
unreachable. Where the catalogue needs the stronger instrument it says so
(`FN-15.d`, `FN-31.a`), and no `TT-` obligation here rests on one.

## Abstractions

Beyond the catalogue's own [deliberate
omissions](../../../docs/specs/semantic-contract.md#deliberate-omissions), which
this model takes as written:

- **`hand-edit` installs one of an enumerated family of well-formed trees**
  rather than composing single-object edits. `EN-11` grants that any well-formed
  tree is reachable by hand edit, so this is a *search strategy over exactly the
  space the assumption already grants*, not a second assumption. It is faithful
  in the direction that matters for the controls: removing `hand-edit`
  (`relax_EN_11`) removes the whole family with it.
- **A guard wait is not an outcome.** The catalogue's outcome set has no member
  for one, deliberately, because Grove's tree lock blocks and a wait is not a
  return. The waiting caller is modelled as membership of `pend`, which keeps
  `TT-22` falsifiable — a failed guard is a real transition — without inventing
  a tree-level twin of `LeaseHeld`.
- **Two cooperating processes**, per the catalogue's own omission.
- **`preorderKey`**: depth-first pre-order is encoded as a positional number
  over the path from the root, so `TT-11` is checkable as a minimum rather than
  as a traversal. Quint has no recursion; every tree walk here is a bounded
  unrolling to `MAX_DEPTH`, and `MAX_DEPTH >= MAX_OBJECTS` makes the bound
  unreachable.
- **Bounds**: `MAX_OBJECTS = 14`, `MAX_DEPTH = 6`, `MAX_POS = 6`, trace depth 24,
  8000 samples.

## Narrowings and qualifications, each declared

**Three** obligations are checked over less than their literal text, and in all
three the gap between the text and what is checkable is a **finding about the
catalogue** rather than a gap in the model. None is a declared `GAP`: the
obligation is answered, and the narrowing is recorded here and in the experiment
log. The catalogue is not edited, because it is frozen under the independence
barrier; `formal-synthesis-k16` owns the disposition of every one of them.

A fourth item is listed below them and is **not** a narrowing — it is a
statement qualification, and it is here rather than in a quieter place because
a guard nobody can find is indistinguishable from a guard nobody declared.

- **`TT-17`** is checked over the Current/Legacy/Foreign decision only. Its
  literal text — "the classification SHALL depend only on the format witness,
  never on any task entry's text" — is contradicted by the catalogue's own
  `PartialScaffold`, which is defined by an exact comparison against a task
  entry's name *and* bytes. `TT-17`'s own witness is about the format decision,
  which is what `formatDecision` isolates and `perturbText` attacks.
- **`TT-20`** collects only interruptions with no `foreign-write` during the
  initialisation, and only those with work still pending. See the counterexample
  below for why the first narrowing exists; the second is not a narrowing of the
  claim at all — a crash after the last effect landed leaves a complete current
  root, which is not "the root an interruption leaves behind".
- **`TT-15.a`** is guarded by `walkStageReached`. Its literal text requires every
  snapshot classifying `CurrentSpent` to report `Empty`. A current root with no
  live task and a **foreign artifact at a name Grove reserves** classifies
  `CurrentSpent` — `classify` reaches its walk stage and finds nothing live —
  while `TT-24.b` requires that same tree to refuse
  `Refused(ReservedNameOccupied(entry))`, and `TT-18` puts that refusal two
  stages ahead of anything the walk says. Both statements are the catalogue's;
  under one tree they are inconsistent, and `TT-24.b` is the one whose whole
  purpose is to win. The guard states the staging premise `TT-15.a` leaves
  implicit — *on a tree the gate actually walks*, selection on a spent tree
  reports `Empty` — and without it the obligation is a claim about the
  classification **order** wearing selection's name.

And the qualification, which is not one of the above:

- **`TT-10`** is checked over `op.decided` — what the operation concluded from
  its own arguments and its one listing — rather than over the outcome the
  operator finally sees. Read over the final outcome it would be a claim about
  the world as well as about arguments: a non-cooperating writer can take a
  create's destination *after* the listing, and the refusal that follows is
  caused by `EN-06` rather than by an argument the domain failed to pre-empt.
  `TT-10`'s own text is "no algebraic refusal reaches an operator **from an
  ordinary argument**", so this is the claim rather than less than it. The
  mid-flight case is not left unclaimed: it has `TT-21.b` and `TT-24.a`, and its
  outcome is decided by `collisionOutcome` (see *Counterexamples*).

## The green run this column stands on

Every claim above is read off one pair of runs, and both are reproducible from
this file rather than from a screenshot.

```sh
models/run.sh --scope task-tree --family quint     # the suite
models/run-controls.sh                             # the controls on the runner
```

| run | result |
|---|---|
| `--scope task-tree --family quint` | **exit 0** — 19m 46s wall / 1209s CPU on a 16-core host, 111 commands, 43 skipped model-checking properties, `-- cells: 43 complete, 0 declared gaps, 0 empty, of 43` |
| `models/run-controls.sh` | **exit 0** — 7 of 7 controls, each mutation producing its named fatal diagnostic |

Both figures are from an **unmodified** runner — `models/run.sh`'s checksum was
taken before the run and again after it, and matches. `bash` reads a script
incrementally, so a run that overlapped an edit to its own runner is not
evidence of anything, and a chain whose subject is false greens does not get to
record one.

The suite's 111 commands are the 83 the library declares and `base` inherits,
plus the 28 written inside the fourteen control instances. `verify_small` owns
none of its own and inherits the 43 properties, which report `SKIP` under the
default `QUINT_VERIFY=0` — a skipped verification that names itself on every
line, and whose reason is the `VERIFY` declaration above. **No module loses
commands under the runner's parser**: every `val inv_…`/`val wit_…` declaration
in both files is accounted for, and the ten `val`s that are neither are
helpers (`atRest`, `allTags`, `tagChoices`, and the like), which is what makes
the counts above check.

## Counterexamples

Replayable; the runner prints the same line on any failure.

### `PartialScaffold` is not robust to a foreign write — `TT-20`, `EN-13`

```sh
quint run crates/grove-task-tree/models/task-tree.qnt --main=base \
  --witnesses wit_finding_partial_scaffold_is_not_robust_to_a_foreign_write \
  --max-steps=24 --max-samples=8000 --seed=0x5e0a51d3c0ffee01 --verbosity=1
```

Trace — **six transitions**, and this is the count every description of it
uses (the experiment log's entry 044 and the derived test below included):

1. `beginOp(TInitRoot)` — the task root, the root charter and the first
   `requirements` leaf are planned; the format witness is planned last.
2. `stepOp` — the task root lands.
3. `stepOp` — the root charter lands.
4. `stepOp` — the first `requirements` leaf lands. The format witness has not.
5. `foreignWrite` — a non-cooperating writer creates one foreign entry beneath
   the task root. `EN-13` grants exactly this.
6. `crashNow` — interruption, which `EN-08` grants between any two steps.

The root now contains the charter, the first leaf, no format witness, and one
foreign entry. `isPartialScaffold` requires the root's entries to be a subset of
`{charter, first leaf}` **and nothing else**, so the extra entry drops the tree
through to `Legacy` — the classification `TT-20` names as forbidden, in the
sentence "never as `Current(*)` and never as `Legacy`".

**Why it is a defect rather than a modelling artifact.** The two statements are
both the catalogue's: `PartialScaffold` is defined by an *exact closed subset* of
the root's contents, and `EN-13` grants that a foreign entry may appear *at any
name*. Under interleaving they are inconsistent, and the interleaving is one
`crash` and one `foreign-write` deep. In the product it is an interrupted
`root-init` plus any stray file in `.grove/` — an editor swap file, a
`.DS_Store`, a partially-synced artifact — after which Grove reads its own
interrupted work as somebody else's legacy tree.

**What a fix would have to decide** (it belongs to `formal-synthesis-k16`, not
here): whether `PartialScaffold` is defined by the *presence* of the scaffold's
own entries with their fixed bytes, ignoring entries outside the task grammar,
rather than by the *absence* of everything else. The safety argument for the
exact subset — every value a completion writes is fixed in advance — survives
that change unchanged, because a foreign entry is not something completion
writes.

### `EN-11` does not gate `TT-24.b`

```sh
quint run crates/grove-task-tree/models/task-tree.qnt --main=relax_EN_11 \
  --witnesses wit_finding_EN_11_does_not_gate_TT_24b \
  --max-steps=24 --max-samples=8000 --seed=0x5e0a51d3c0ffee01 --verbosity=1
```

The catalogue lists `TT-24.b` in `EN-11`'s controls column, whose stated expected
result is that with `hand-edit` removed "every witness that posits a tree
Grove's own actions cannot build is unreachable". `TT-24.b`'s witness — an
ordinary operation meeting a foreign entry at a name Grove reserves — is reached
in ~2% of traces with `hand-edit` gone, because `EN-13` grants that foreign
entries may appear **at any name** and `foreign-write` alone supplies one. The
dependency is on `EN-13`, not on `EN-11`.

This is structurally the same mistake the catalogue already caught and annotated
for `TT-16` in the same row. Finding a second instance of it is what a second
independently built family is for; the correction is one word in a controls
column and it changes no claim.

### A bulk mark cannot converge if `AlreadyTerminal` refuses its plan

```sh
quint run crates/grove-task-tree/models/task-tree-controls.qnt \
  --main=mutant_bulk_strict \
  --invariant inv_fail_MUT_TT_23b_strict_validation_cannot_converge \
  --max-steps=24 --max-samples=8000 --seed=0x5e0a51d3c0ffee01 --verbosity=3
```

`TT-23.b` requires that re-running a bulk mark after a partial application
reaches the same result. `AlreadyTerminal` is a refusal for a single mark, and
`TT-23.a` requires the *whole plan* validated before the first rename. Validate
a bulk plan the way a single mark is validated and the re-run refuses on the
member the interrupted run already marked — so the plan can never converge, and
the property [`bulk-marks-are-not-atomic`](../../../docs/adr/bulk-marks-are-not-atomic.md)
exists to buy is unreachable. A bulk member already in the plan's **target**
state must therefore be admissible and a no-op, which is what `bulkMemberOk`
implements under `BULK_TARGET_IDEMPOTENT`.

**What the instrument establishes, and it is a stronger statement than the
inference.** The convergence ghost retains the interrupted **request** —
`hist.bulkPlanKeys`, the key list itself — and not a sticky boolean, so:

- only the **identical** request repairs the interrupted one. `[1, 2]` and
  `[2, 1]` are two requests; a model that accepted either as the other's repair
  would be establishing that *a* bulk mark converges, which is a different and
  much weaker claim.
- "the same result" is every member of the interrupted plan in the target
  state — not merely "no member still live", which a member someone abandoned
  in between would satisfy while the re-run plainly did not reach the first
  run's result.
- a **refused retry falsifies the property**, and it has to be caught where the
  refusal is decided: a refusal never becomes a running operation, so a ghost
  that only watches completions cannot see the failure `TT-23.b` is about. Only
  a refusal the bulk validator itself produced counts — a gate refusal
  (`ReservedNameOccupied`, `WitnessPending`, a root that is no longer current)
  refuses *every* operation, and reading those onto `TT-23.b` would make it a
  statement about what the world did to the tree. `TT-24.b` and `TT-19` own
  those, and they do.
- the operator installing a different tree (`hand-edit`) clears the pending
  plan: a plan interrupted against a tree that is gone is not a plan anyone can
  re-run, and keeping it pending would falsify `TT-23.b` on the hand-edit
  abstraction rather than on anything a bulk mark did.

With all four in place `inv_TT_23b` holds in `base` and in `scenario_bulk`,
`wit_TT_23b` is reached in ~1.5% of `scenario_bulk`'s traces, and
`mutant_bulk_strict` — the same model with idempotence off — **violates** it.
That last run is what turns the requirement from an argument into a control.

**Derived test** (for the implementation phase, in the existing black-box
binaries rather than a new seam): interrupt a bulk mark between two of its
renames, re-run the **identical** invocation, and assert it succeeds and leaves
every named entry marked — not that it refuses `AlreadyTerminal`.

### An ordinary mutation blocked after it had already mutated — the outcome set's real gap

```sh
quint run crates/grove-task-tree/models/task-tree-controls.qnt \
  --main=scenario_foreign_sibling \
  --witnesses wit_finding_an_ordinary_mutation_blocked_after_a_partial_mutation \
  --max-steps=24 --max-samples=8000 --seed=0x5e0a51d3c0ffee01 --verbosity=1
```

Reached in ~12% of that instance's traces. An insert shifts its later siblings,
and then a non-cooperating writer takes the name the create was going to use
between the listing and the step (`EN-06`, `EN-13`). The create is no longer
licensed — proceeding would mutate an entry whose ownership the operation cannot
prove (`TT-24.a`) — and the tree is **not** byte-identical, because the shifts
already landed.

The catalogue's [*One artifact, three contexts, one decided
outcome*](../../../docs/specs/semantic-contract.md#outcomes) table fixes an
ordinary operation **before any transaction**, a finish or recovery transaction,
and the reaper. This is none of the three: no refusal is honest once an effect
has landed (`Refused` means "nothing happened; the tree is byte-identical"), and
the row that licenses `Blocked` is about a transaction. The model returns
`Blocked(OwnershipConflict)` on the table's own reasoning — a caller who has
already mutated is owed a block rather than a refusal — and **that** is the gap.

**What is NOT a gap, and the distinction is the whole finding.** The same
collision *before* any effect has landed is answered perfectly well by the
closed set: the tree is byte-identical, so it is a refusal, and the reason is
`ReservedNameOccupied(entry)` exactly when the occupant is the artifact
`TT-24.b` is about and `DestinationOccupied` otherwise. `collisionOutcome` draws
that line. An earlier version of this model returned
`Blocked(OwnershipConflict)` for both, which overstated the finding: it made a
pre-effect collision look like evidence of a missing outcome when it was
evidence of a model that had not read the table's own partition.
