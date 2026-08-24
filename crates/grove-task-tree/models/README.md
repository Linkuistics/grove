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
| Alloy 6 | `task-tree.als` | `TT-01` – `TT-16` |
| Quint | — | none yet (`quint-models-k10`) |

`TT-17` – `TT-25` are the `guarding` sibling leaf's work. The runner reports
their cells empty, which is the truth about this directory rather than a defect
in it: they are **not** declared gaps, because a declared gap means *cannot
express*, and nothing here has tried yet.

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

**Runtime.** The Alloy scope of this directory takes roughly **half an hour**
end to end on the measurement host, and one command is a third of it:
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

**Temporal, not structural.** State is `var`: `onDisk`, each object's name,
parent and digest, and the `Sys` record of the action and outcome that produced
the state. Every action is total — a failed guard yields a named refusal and a
byte-identical tree, never an absent transition — which is what lets a refusal
be checked at all.

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
| promotion's non-atomicity | one step | `EN-04` is the control on that, and it is `guarding`'s to run |
| `Refused(DestinationOccupied)` | not represented | no `TT-01` – `TT-10` obligation states it; a name collision at a reserved name is `TT-24.b`'s, in `guarding` |
| the reserved and format witnesses | not represented | `TT-17` – `TT-20`, in `guarding` |
| a directory the walk does not enter | present on disk, but outside `visited` | the walk descends into the task root and into nodes only; nothing beneath a foreign directory is an entry, holds a position, or contributes a key |
| a resolution's reference syntax | `one sig Query` — an optional key and an optional slug | the CLI's `[n]`, a bare slug and `<slug>-k<key>` differ only in which of the two is present, and no `TT-` obligation reads a slug's content. **One** atom, so a trace carries one resolution argument: every command here needs at most one `resolve`, and the existing `TT-01` – `TT-10` scopes are unchanged by its arrival |
| what an observation reported | `Sys.got` and `Sys.gotTerm`, written by the transition | *derived* terminality could not be got wrong, and `TT-16` is precisely the claim that the report carries it. Modelling the report as state is what makes the mutation *"a resolved `Done` entry is not reported terminal"* expressible at all |
| `brief-chain` and `kind` | not represented | both are observations, but no `TT-11` – `TT-16` obligation states anything about either beyond what `select` and `resolve` already carry |

**`TT-11`'s "depends on no state outside the tree" is answered by construction,
not by a command**, and this is the honest place to say so. `precedes` and
`selected` are written as functions of `loc` and `nm` and of nothing else, so
there is no scheduler state for a check to quantify over — a model cannot check
the absence of a variable it does not have. What *is* checked is the falsifiable
half: `TT-14` pins position as the mechanism that orders siblings, independently
of whatever `precedes` happens to say, which is why re-defining `precedes` to
order by key breaks `TT-14` and leaves `TT-11` green.

**What a green run does not prove.** Every result is about the stated bounds: at
most 5 objects, at most 5 filenames, two to four states, one working tree, and
one cooperating process. Nothing here is a proof about arbitrary trees. In
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

**Symmetry, exact scopes, fairness.** No command uses an `exactly` scope, so
Alloy's symmetry breaking is free to collapse isomorphic instances — which is
sound for the existential witnesses (an instance is an instance) and for the
universal checks (a counterexample has an isomorph). No command states a
fairness assumption: `TT-01` – `TT-10` are safety and reachability claims, and
none of them is an eventuality.

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

**None about grove's shipped behaviour**, from either slice. The `TT-01` – `TT-10`
slice produced three findings about the **catalogue**, and all three landed
there; the `TT-11` – `TT-16` slice produced none of either kind — its result is
the mutation matrix, and its one incident is the vacuous `TT-12` mutation above.
See [`docs/formalism-findings.md`](../../../docs/formalism-findings.md) entries
026 and 027.

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
