# `models/system/` — the system-lifecycle scope

The joint of sessions, exhaustion, finish, interruption and recovery. The claims
are **not** here: they are
[`docs/specs/semantic-contract.md`](../../docs/specs/semantic-contract.md)
§*Claims — system lifecycle*, and this directory holds instruments rather than
statements.

```sh
models/run.sh --scope lifecycle --family alloy --no-coverage
```

`--no-coverage` stays on that line until the whole `SY-` column closes. It is
the visible signal that a scope's first family is still mid-build, and dropping
it is what says the column closed.

| file | family | covers |
|---|---|---|
| `lifecycle.als` | Alloy 6 | `SY-01` – `SY-08`, `SY-10`, `SY-11` — seventeen obligations |

Eight of the twenty-five `(alloy, obligation)` cells are empty, and the runner
names each: `SY-09` and `SY-12` – `SY-14`. They are the `sessions` sibling
leaf's, not gaps.

## The composition boundary

This scope's one discipline: **compose at observations, never at machinery.** A
`SY-` claim reads a task-tree or finish fact through the smallest observation
that decides it, and never through the machinery that produces it.

What this model does **not** have, and which model owns each:

| absent here | owner |
|---|---|
| filenames, positions, keys, slugs, species, digests | `crates/grove-task-tree/models/` |
| the eleven-state task-root classification (`TT-18`) — its members, the subset comparison that decides `PartialScaffold`, the byte equality, and the ORDER that puts `PartialScaffold` before `Legacy` | `crates/grove-task-tree/models/`. **Two of the eleven reach this file, and as OPAQUE OBSERVATIONS rather than as states**: `World.partial` and `World.legacy` are marks whose content is entirely `TT-18`'s and `TT-20`'s. What this file owns is the difference between the observation that ENABLES a completion — their union, which at this scope is exactly *the format witness is absent* — and the one that DECIDES it, `partial` alone. That difference is `SY-06.b`, and it is checkable with no format witness in the signature. Importing two is not importing eleven; importing the classification would have made this file a third copy of `TT-18` |
| the format witness itself, and the exact known subset that defines a partial scaffold | `crates/grove-task-tree/models/`, `TT-18`/`TT-20`. This file never reads a witness or a byte: *no format witness* is `partial + legacy` and nothing else |
| a leaf's state, and the retirement that spends a tree | `crates/grove-task-tree/models/`, `TT-17`. A `Current(Spent)` root is reached here from the free initial state, which `EN-11` is what licenses |
| the eleven-state classification's `Reserved` members, the quarantine, the correlation ticket, the manifest, the atomic rename | `crates/grove-finish/models/`. `SY-05.b`'s two underlying steps are `FN-11` and `FN-19`, and they reach this file as `doProveCommit` and `doSettleDeletion` — **one opaque step each**, carrying only the observation the obligation reads: the deletion is proven, and the name frees |
| entry names, positions, kinds, terminality, and the pre-order walk that decides **which** live leaf `select` returns (`TT-11`, `TT-12`, `TT-14`) | `crates/grove-task-tree/models/` — `SY-08` cannot be stated without something for selection to *return*, so `Leaf` is an opaque handle and `World.live` is an **unordered set**. The choice among live leaves is imported as non-determinism, and a signature that grew an order would be this file re-stating the selection contract |
| `Empty` and `Ambiguous` as distinguishable observation outcomes (`TT-15`) | `crates/grove-task-tree/models/` — `doSelect` is guarded on `some World.live` rather than branching on it, because selection on a spent tree and the finish leaf exhaustion yields (`SY-07`) are both the `roots` sibling's |
| the configuration's contents — kinds, templates, the personal file and the delta | `complete-session-configuration` and `untracked-configuration-delta`. `SY-04.b` reads exactly one bit of it: whether full validation passes |
| the witness slot, the evacuation manifest, the correlation ticket, the quarantine, the cleanup marker | `crates/grove-finish/models/` |
| the three lanes and every lane-specific mechanism | `crates/grove-finish/models/` — no `SY-` obligation in this slice differs by lane |
| the finish transaction's own preflight | `crates/grove-finish/models/`, `FN-05.a` member 3. This file's `doLayoutPreflight` is **the later gate as such**, standing for every subsequent revalidation; `SY-03` is the claim that connects them, and `witness_FN_05a_p3_layout_unsupported` is the same claim stated inside the finish scope |

A signature here that grows one of those rows is this file becoming a third copy
of two contracts rather than the joint of them.

## What is owed elsewhere, and is not a gap

- **`SY-02`'s session half — ANSWERED, and not by widening `SY_02`.** The
  admission slice stated `SY-02`'s fourth conjunct (a tree operation happens
  only under a lease) **for a driver only**, because a session reaches the tree
  through a generation rather than a lease and `launch` was not modelled. It is
  now `SY_10a`'s third conjunct: an ambient tree operation happens only while
  the session holds an epoch guard, and `SY_10a`'s second conjunct is that a
  session acquires one only at a matching generation. Widening `SY_02` instead
  would have been **false**: a driver may release its lease while a generation
  it opened is still live, so *some driver holds a lease* is not an invariant,
  and the seam belongs where the catalogue put it.
- **`EN-14`'s `SY-05` half — ANSWERED.**
  `expect_fail_EN_14_SY_05a_absence_stops_discriminating_a_fresh_tree` is the
  row's second command. The working-tree root is replaced under the loop, the
  task root goes with it — a new directory at the path has no task tree in it —
  and the grove is untouched, sitting where Grove can no longer reach it.
  `SY-05.a`'s third conjunct fails: absence stops discriminating a fresh tree,
  and Grove would scaffold a new grove over a live one. **`SY-05.b` fails under
  the same scope and for the same step**, and the control names only `SY-05.a`
  deliberately — a control naming both would report one counterexample as two.
- **`SY-05.b` is the joint claim, and its placement is recorded rather than
  solved.** The catalogue says `SY-05` and `FN-11`/`FN-19` SHALL be checked
  together, and an `FN_`-prefixed command here is a placement failure the runner
  refuses. What this file states is the OBSERVATION — no trace exposes an absent
  task root before the deletion is proven — over its own transitions;
  `crates/grove-finish/models/` owns the steps underneath, and its `FN_11_…` and
  `FN_19_…` commands are the other half of the pair. **This is not a declared
  gap**: the obligation is answered, by the half that belongs here.
- **`SY-06.b`'s ORDERING CLAUSE IS IMPORTED, AND IT IS THE FOURTH INSTANCE OF THE
  `TT-24` PLACEMENT SHAPE.** The obligation ends *and is completed **before** any
  format classification runs*. This file has no format classification step — it
  reads `partial` and `legacy` as observations that are already made — so what it
  checks is the ordering's CONSEQUENCE (a `Legacy` tree is never completed as
  though Grove had scaffolded it) and not the ordering itself. The order that
  puts `PartialScaffold` before `Legacy` is `TT-18`'s and is
  `crates/grove-task-tree/models/`'s to check. **Recorded rather than solved**,
  in the shape the two `TT-` gaps and `finish-k8`'s Q4 instance are recorded in:
  `formal-synthesis-k16` inherits the set, and a fourth instance is cheap for it
  only because it is stated. It is **not** a declared gap — `SY-06.b` has a check
  and a firing mutation here; what is imported is one clause of it.
- **`SY-04.b`'s configuration content — ANSWERED.** `doValidateConfig` was
  opaque in the admission slice and now reads the live configuration, with the
  same biconditional instrument `SY-03` uses on the layout.

## Tool, solver, bounds

| | |
|---|---|
| Alloy | 6.2.0.202501090817 (git `794226d`), `~/.local/share/alloy/org.alloytools.alloy.dist.jar` |
| Java | Corretto 21.0.12.1+9-LTS — the host's default `java` is 16.0.1, below Alloy 6's floor, and `models/run.sh` probes past it |
| Solver | SAT4J, Alloy's bundled pure-Java default, not overridden |
| Runner flags | `-q -n -t text` — `-n` excludes overflow instances, `-t text` because the default table renders a temporal trace as an empty grid |

Every command's scope is `for 3 but 2 WtId, N steps`. `3 Proc` is not
decoration: `SY-11.b`'s cycle needs two blocked processes and its non-vacuity
witness needs a third holder. `2 WtId` exists for `EN-14` and for nothing else.
`Leaf` and `Gen` take the default 3 — two would serve every witness, and the
third is the margin the *scope trap* asks for. **`N steps` is N STATES**, an
Alloy 6 trace being a lasso whose last state loops.

| obligation | command | steps | witness first lands at | wall |
|---|---|---|---|---|
| `SY-01.a` | `SY_01a_…` | 4 | — | 1.29 s |
| `SY-01.a` | `witness_SY_01a_a_second_driver_refused_while_the_first_holds` | 4 | **3** | 1.14 s |
| `SY-01.b` | `SY_01b_…` | 4 | — | 1.16 s |
| `SY-01.b` | `witness_SY_01b_a_crashed_driver_whose_successor_proceeds` | 5 | **4** | 1.14 s |
| `SY-02` | `SY_02_…` | 4 | — | 1.17 s |
| `SY-02` | `witness_SY_02_a_refusal_leaving_an_empty_working_tree_untouched` | 4 | **3** | 1.05 s |
| `SY-03` | `SY_03_…` | 4 | — | 1.05 s |
| `SY-03` | `witness_SY_03_a_layout_that_changes_between_the_two_gates` | 6 | **5** | 1.22 s |
| `SY-04.a` | `SY_04a_…` | 5 | — | 1.17 s |
| `SY-04.a` | `witness_SY_04a_acquire_lease_alone` | 6 | **3** | 1.05 s |
| `SY-04.a` | `witness_SY_04a_layout_preflight_alone` | 6 | **3** | 1.04 s |
| `SY-04.a` | `witness_SY_04a_open_epoch_alone` | 6 | **3** | 1.05 s |
| `SY-04.a` | `witness_SY_04a_launch_alone` | 6 | **5** | 1.25 s |
| `SY-04.a` | `witness_SY_04a_reap_alone` | 6 | **4** | 1.20 s |
| `SY-04.a` | `witness_SY_04a_close_epoch_alone` | 6 | **4** | 1.20 s |
| `SY-04.a` | `witness_SY_04a_release_lease_alone` | 6 | **4** | 1.14 s |
| `SY-04.b` | `SY_04b_…` | 5 | — | 1.26 s |
| `SY-04.b` | `witness_SY_04b_a_configuration_that_goes_invalid_…` | 5 | **3** | 1.18 s |
| `SY-08` | `SY_08_…` | 5 | — | 1.39 s |
| `SY-08` | `witness_SY_08_a_leaf_inserted_during_the_launch_window` | 6 | **5** | 1.22 s |
| `SY-10.a` | `SY_10a_…` | **7** | — | 2.37 s |
| `SY-10.a` | `witness_SY_10a_a_stale_session_refused` | 7 | **6** | 1.59 s |
| `SY-10.b` | `SY_10b_…` | 5 | — | 1.40 s |
| `SY-10.b` | `witness_SY_10b_a_contended_generation_times_out_…` | 5 | **4** | 1.16 s |
| `SY-11.a` | `SY_11a_…` | 5 | — | 1.24 s |
| `SY-11.a` | `witness_SY_11a_the_full_order_reached` | 7 | **6** | 1.43 s |
| `SY-11.b` | `SY_11b_…` | 5 | — | 1.37 s |
| `SY-11.b` | `witness_SY_11b_a_real_wait_that_is_not_a_cycle` | 4 | **2** | 0.96 s |
| `SY-05.a` | `SY_05a_…` | 5 | — | 4.16 s |
| `SY-05.a` | `witness_SY_05a_a_completed_teardown_and_then_a_fresh_scaffold` | 6 | **4** | 1.40 s |
| `SY-05.a` | `witness_SY_05a_the_name_reoccupied_with_the_retired_identity` | 6 | **4** | 1.37 s |
| `SY-05.b` | `SY_05b_…` | 5 | — | 1.42 s |
| `SY-05.b` | `witness_SY_05b_absence_is_reached_and_only_after_a_proven_deletion` | 5 | **4** | 1.35 s |
| `SY-06.a` | `SY_06a_…` | 5 | — | 1.44 s |
| `SY-06.a` | `witness_SY_06a_a_fresh_root_distinguishable_from_a_spent_one` | 7 | **5** | 1.53 s |
| `SY-06.b` | `SY_06b_…` | 5 | — | 1.36 s |
| `SY-06.b` | `witness_SY_06b_an_interrupted_scaffold_completed_by_a_successor` | 8 | **7** | 2.02 s |
| `SY-06.b` | `witness_SY_06b_a_legacy_tree_refused_rather_than_completed` | 4 | **3** | 1.27 s |
| `SY-07.a` | `SY_07a_…` | 5 | — | 1.56 s |
| `SY-07.a` | `witness_SY_07a_an_append` | 4 | **3** | 1.29 s |
| `SY-07.a` | `witness_SY_07a_a_reuse` | 4 | **3** | 1.27 s |
| `SY-07.b` | `SY_07b_…` | 5 | — | 1.68 s |
| `SY-07.b` | `witness_SY_07b_a_refused_creation` | 4 | **2** | 1.84 s |
| control | `expect_fail_EN_07_SY_11b_…` | 5 | — | 1.11 s |
| control | `expect_fail_EN_14_SY_01a_…` | 5 | — | 1.23 s |
| control | `expect_fail_EN_14_SY_05a_…` | 5 | — | 1.31 s |

**EVERY INHERITED WITNESS STILL LANDS EXACTLY WHERE IT DID**, and the whole sweep
was re-run from 2 to its declared bound to establish it rather than assumed. That
is the expected result and it is worth the paragraph anyway: this slice's one
change to an existing transition is that `doSelect`'s `some World.live` guard
became a branch, and **removing a guard only ADDS traces**, so no witness could
have lost its instance by it. The three checks that were edited — `SY_02` and
`SY_10a` restated over `TreeAct`, `SY_04b` conjunct 3 given its environmental
exclusion — are dealt with below.

**ONE INHERITED WITNESS WAS CORRECTED, AND THE CAUSE IS A NEW CONSTRUCTION FACT.**
`witness_SY_10a_a_stale_session_refused` said `always no World.rooted`, which is
how the iteration slice spelled *before it touches the tree*: no task root exists
in any state, so the refusal cannot be one that arrived after a read. This slice
added `AnAbsentRootHasNoEntries` — a tree that is not there has no entries in it —
and that reading stopped being available: a rootless trace now has no live leaf,
so it has no selection, so it has no **launch**, and the witness needs one. It
failed as *no instance*. The replacement is `always Sys.act not in TreeAct`, which
is the stronger statement and the one the claim actually makes — no tree action
anywhere in the trace, whether or not a tree is there to act on. **A construction
fact added for one obligation can make another obligation's witness unreachable,
and unreachable reports exactly as wrong.**

**`witness_SY_10a` ALSO WENT FROM 1.59 s TO 8.62 s**, and that is this slice's
price rather than a measurement error: the witness is a seven-state satisfiable
search and the state it searches now carries six more `World` fields. It is the
file's second-dearest command and the dearest witness.

**A SATISFIABLE SEARCH CAN COST 8× FOR ONE EXTRA STATE OF MARGIN, AND THAT IS
NEW.** `witness_SY_06b_a_legacy_tree_refused_rather_than_completed` lands at 3.
At 5 states it takes **10.42 s**; at 4 it takes **1.27 s**. Nothing about the
instance changes — the extra state is pure search space. The bound is 4, which is
the margin the file's other sixteen witnesses carry, and the measurement is
recorded because the *scope trap* discipline pushes bounds upward and this is the
first command where that push had a price worth naming. The rule it suggests:
**margin is bought one state at a time and priced per command**, so raise the
bound where a mutation needs it and not by reflex.

**`SY-10.a` RUNS AT SEVEN AND THAT NUMBER IS A CORRECTION.** It was written at
five, was green, and its conjunct-2 mutation (**M10b** below) *survived*. At six
it still survived. The defect needs a rotation to occur **while a session is
blocked**, which is six transitions: the wait, the holder's release, the
driver's iteration boundary, the driver's rotation, the driver's death, and the
grant. This is the sibling scopes' second vacuity predictor met head-on — **the
bound must hold the machinery of the transitions the obligation quantifies
over**, not only the objects it names — and it is an `M8` incident, because the
five-state run was a believed green over a defect it could not reach.

**FOUR OF THE SIX INHERITED WITNESSES NOW LAND ONE STATE LATER**, and that is
this slice's price rather than a measurement error. `SY-01.a` went 2 → 3,
`SY-02` 2 → 3, `SY-03` 4 → 5 and `SY-11.a` 5 → 6. Two causes, both structural:
`Proc.spent` makes a two-state lasso unable to close (the return edge would have
to be a `doIdle` that leaves `spent` unchanged, and the transition that set it
did not), and `acquire-lease` and `open-epoch` are **both** Lifecycle actions,
so `witness_SY_11a`'s full order now needs an iteration boundary between them.
**Six witnesses had lost their margin entirely and their bounds were raised to
restore it** — the *scope trap* is the pre-registration's hazard and a witness
sitting exactly at its own first-landing bound is that hazard as a number. The
whole sweep was re-run from 1 to 8, which is what the finish scope's rule
requires of a slice that touches a field a guard reads.

**COST STOPPED BEING UNINTERESTING WITH THIS SLICE, AND THE THIRD MEASUREMENT IS
THE FIRST ONE THAT SAYS ANYTHING.** The JVM-plus-parse floor on this host is
unchanged at **0.58 s**. The forty-six-command file runs in **105 s and 113 s wall** across two
runs under `models/run.sh` (222–241% CPU, ~240 s user both times) against the
thirty-command file's 37 s — so **53% more commands cost roughly 190% more
wall**, and the gap is where the finding is. **The parallel utilisation fell from
355% to ~230%**, which is most of it: the runner's concurrency is unchanged, so
what dropped is how much of the suite is short enough to overlap. The slice added **eight reachable transitions**, **six `var` fields on
`World`**, **one new signature** (`Grove`, at the default scope 3) and two
`Result` members. Three commands account for most of the growth: `SY_05a` at
4.16 s, `SY_10a` at 3.23 s (from 2.37 s) and `witness_SY_10a` at 8.62 s (from
1.59 s). **The rest of the file did not move**: thirty-odd commands still sit
between 1.27 s and 1.68 s, within 3× the floor.

**What the shape says is that `Grove` is the expensive addition and the
observations are not.** Six `var` fields on a `one sig` cost almost nothing —
they are six relations of arity ≤ 2 over singletons. `Grove` at scope 3 is a
`var`-referenced free sig, and the static-atom law the node brief carries (~10 ms
of translation per atom per command) understates it: what it costs is not
translation but the SEARCH, in exactly the two commands that quantify over grove
identities across states. **The placement is still doing its job** — this file
pays for `Grove` because `SY-05` is about identity, and it pays for nothing about
witnesses, manifests, quarantines or lanes.

## Abstractions, and what a green run does not prove

- **`Proc.waits` is this file's own abstraction, not the catalogue's outcome
  set.** §*Outcomes* is explicit that a guard wait is not an outcome — Grove's
  tree lock blocks and no invocation returns while it is held — but `SY-11.b` is
  a claim *about* waiting, and a model in which a failed guard is an absent
  transition makes it true by construction. The sibling task-tree model met the
  same wall and introduced `Deferred`; this is the same move under a different
  name, and `Deferred` appears in `Result` for the same reason. **It is an
  abstraction, never a contract**: nothing in Grove returns it. `SY-10.b`
  **reuses it rather than introducing a second wait** — the timeout is an exit
  from `Proc.waits`, which is the answer the `iteration` leaf was asked to give
  explicitly.
- **`Stopped` IS THE SECOND ABSTRACTION, AND ITS EXISTENCE IS A FINDING ABOUT
  THE CATALOGUE.** `SY-10.b` requires a contended generation to time out into a
  *visible stop*, and the closed outcome set cannot name one. It is not a
  `Refused`: the closed refusal-reason list has `EpochStale`, which is
  `SY-10.a`'s **mismatch**, and nothing for a handoff timeout. It is not a
  `Blocked`: §*Outcomes* scopes blocks to *a transaction stopped part-way*, and
  `FN-25`'s two diagnoses are both about finish ownership. `one-live-driver-per-
  working-tree` says the driver "stops `blocked`" on a post-reap invalidation
  timeout, so the situation is real and shipped. **`RefConfigInvalid` is the
  same finding's second instance**: `SY-04.b` requires full configuration
  validation ahead of every transition, and no refusal reason names its failure
  either. Both are declared here and named for `formal-synthesis-k16`; neither
  is this leaf's to add to the catalogue, because a new reason imposes a
  matching outcome on the Quint column.
- **`IterA` is the third, and it is a boundary rather than an action.**
  §*Actions* has no iteration boundary in it, correctly — a boundary is not
  something the loop *does*. But `SY-04.a` says *at most one lifecycle
  transition **per iteration***, and an iteration with no observable edge has
  nothing for the count to be taken between. `IterA` takes no guard, returns no
  catalogue outcome, and touches no part of the world.
- **`seen` IS NOW RESET, and to `p.holds` rather than to nothing.** The
  admission slice never reset it and named this slice as the owner.
  `HeldImpliesTaken` is a construction fact, so emptying `seen` while the driver
  still holds its lease across the boundary would make the boundary
  unsatisfiable for exactly the process whose loop it is. The honest reading is
  that `seen` records the guards taken **in this iteration**, and an iteration
  begins holding whatever the last one did not release.
- **`launch` and `reap` are now modelled**; all seven of the catalogue's
  Lifecycle actions are present, and `LifecycleAct` is the set both `SY-04`
  obligations quantify over so that a slice adding an eighth reaches them
  without either command being edited.
- **The epoch record models only its ROTATION write.** The ADR has three write
  points — inactive after lease acquisition, active before spawn, inactive after
  reap — and this file has one: `open-epoch` writes the record active with a
  fresh identity. The two *inactive* writes are collapsed away, on the
  catalogue's own grounds that rotation is the stronger mechanism (it "catches
  stale sessions between every launch as well as after finish plus root
  recreation"). `no World.gen` remains reachable as a free initial state, so an
  inactive record is not unrepresentable — only unwritten.
- **`SY-10.b` IS NOT A LIVENESS PROPERTY AND MUST NOT BE READ AS ONE.**
  §*Deliberate omissions* models clocks, timeouts and retry counts as
  non-determinism, because a bounded handoff wait is a liveness property of the
  *implementation* and not of the protocol. `doTimeout` therefore carries no
  clock and is non-deterministically enabled, and nothing in this file says the
  timeout *will* fire. What `SY_10b` says is that a generation wait ends only in
  the waiter's own step and that that step reports something — which is what
  *never a silent park* can mean with no fairness premise to hand. This is the
  same discipline `SY-13` will need and for the same reason.
- **`SY-04.b`'s *byte-identical* is READ AT THIS SCOPE'S GRAIN, and the reading
  is part of the claim rather than beneath it.** This file's task root is
  present or absent and its leaves are opaque handles; there is no byte here to
  compare. Conjunct 3 says the strongest thing the composition boundary admits:
  under an invalid configuration the tree's presence does not change and no
  entry appears. A model that could see a byte would say more.
- **One process role distinction, and no more.** A driver and an ambient
  session, because with one role only the lease-holder ever reaches the epoch,
  no two processes contend, and `SY-11.b` would be checked over an empty
  wait-for graph.
- **`initialise-root` IS ONE CATALOGUE ACTION MODELLED AS TWO STEPS, and the
  split is `SY-06.b`'s whole subject.** §*States* says root initialisation makes
  the format witness visible **last**, and `PartialScaffold` is defined by the
  interval that leaves open. A model in which scaffolding is one indivisible
  mutation has no such interval, so the obligation's subject does not exist in it
  and the claim is answered by construction — which is this file's most-recorded
  failure mode. `doInitRoot` writes the root and stops; `doCompleteScaffold` is
  the append, and it is **the same step whether it finishes a scaffold Grove just
  started or one an interruption left behind**. That is §*States*' own reasoning —
  every value the completion would write is fixed in advance, so completing is a
  comparison followed by at most one append — and modelling them as two steps
  would make `SY-06.b`'s witness a different operation from `SY-06.a`'s when the
  claim is that they are not.
- **`allocate-finish-leaf` IS SPLIT OUT OF `doTreeOp` FOR THE SAME CLASS OF
  REASON, and the reason is stated as a rule.** `SY-07.b` is a claim about the
  **actor of one specific mutation**, and one opaque tree step cannot carry an
  actor rule for one mutation and not another: written inside `doTreeOp`, *no
  session creates a finish leaf* would have been false on every trace where a
  session touched the tree at all. Prefer splitting out a named transition to
  widening an opaque one, wherever a claim is about **which** mutation rather
  than about mutation.
- **`World.fin` IS A `set` AND `World.rooted` IS AN IDENTITY, both to keep a
  claim from being a declaration.** `lone fin` would have made `SY-07.a`'s
  *exactly one* true by construction; a `one sig TaskRoot` would have made
  `SY-05`'s identity questions unstatable. Each costs a state-0 fact
  (`TheTreeStartsWithAtMostOneFinishLeaf`, `GrovesStartAtTheirName`) and neither
  fact carries `always`, which is the distinction the whole file turns on.
- **`Empty` ARRIVES BECAUSE `SY-07` NEEDS `doSelect`'s GUARD TO COME OFF.** The
  iteration slice guarded selection on `some World.live` and named `roots` as the
  owner. `SY-07`'s antecedent is exactly that state — *when no live leaf remains*
  — so a guard that deletes it deletes the obligation's subject. `Empty` is a
  **success** (§*Outcomes*, `TT-15.a`), not a refusal, and it is the catalogue's
  own outcome rather than an abstraction of this file's. **The change removed a
  guard, so it only added traces**, and the whole inherited witness sweep was
  re-run to establish that nothing moved.
- **THE WORLD CAN WRITE THE TREE NOW, AND THAT CORRECTED AN INHERITED CHECK.**
  `hand-edit` (`EN-11`) puts a `Legacy` tree at the name; `foreign-write`
  (`EN-13`) re-occupies a freed name, with the retired identity. `SY-04.b`'s
  third conjunct — an invalid configuration leaves the tree unchanged — was
  written before either existed and is now qualified by `Sys.res' !=
  Environmental`. **This is a correction, not a weakening**: unqualified, the
  conjunct reads *a bad configuration stops the operator editing their own
  directory*, which is false and is not what the obligation says.
- **WHAT THE MODEL GRANTS ABOUT THE WORLD AND THE ROOT, stated because it is
  load-bearing.** Neither environment action DELETES the task root. §*States* is
  the ground: *a task root whose deletion is not yet proven is never `Absent`* —
  that is the contract, and `EN-14` is the one place its negation is exercised.
  A `doHandEdit` that could empty the name would put `EN-14`'s counterexample
  inside the assumed scope and `SY-05.b` would report it as an ordinary defect.
- **THE `SY-04.b` SCOPING ERROR IS A CATALOGUE FINDING AND IS NAMED FOR
  `formal-synthesis-k16`, NOT FIXED HERE.** The catalogue is the shared subject
  of both families, and this leaf does not edit it under the independence
  protocol — the same rule that left `Stopped` and `RefConfigInvalid` declared
  rather than added. What the catalogue owes is one clause: *an invalid
  configuration leaves the working tree byte-identical* is true of **Grove's own
  transitions** and is false of the world's, and §*Actions* puts `hand-edit` and
  `foreign-write` in the same table as the transitions the claim is about.
  Recorded in `docs/formalism-findings.md` entry 042 with its counterexample.
- **A green run of this file is not evidence.** Three of its seventeen
  obligations have no firing protocol-level mutation or none that is isolating
  (below), one check was green and vacuous for two rounds before the mutations
  found it, and one was green at a bound too small to reach its own mutation.

## The mutation matrix

One mutation per reported obligation, run before the green was believed. **A
mutation the model's own facts make unsatisfiable reports exactly as a survivor
does**, so a survivor is investigated rather than recorded.

| # | obligation | mutation | result |
|---|---|---|---|
| M1 | `SY-01.a` | the contended lease queues (`waits' = LeaseG`) instead of refusing | **fires** |
| M2 | `SY-01.b` | `doCrash` leaves the lease standing — death as a cleanup path rather than a kernel release | **survives** — see below |
| M2b | `SY-01.b` | the construction fact stops clearing `leaseOn` on death | **survives** — see below |
| M3 | `SY-02` | the lease gate stops reading the layout | **fires** |
| M4 | `SY-03` | the later gate consults the recorded verdict instead of the layout | **fires** |
| M5 | `SY-11.a` | the **grant** site drops the order clause | **survives** — and the reason is now structural, see below |
| M5b | `SY-11.a` | the **take-tree** site drops the order clause | **survives**, but it is now a LIVE mutation — see below |
| M6a | `SY-11.a` | the **open-epoch** site drops the order clause | **fires** |
| M6 | `SY-11.b` | the same open-epoch mutation | **fires**, and is **not isolating** |
| M7 | `SY-04.a` | the **launch** site stops checking the iteration is fresh | **fires** |
| M7b | `SY-04.a` | the **reap** site takes its transition without spending the iteration | **fires** |
| M8 | `SY-04.b` | a tree operation stops revalidating the configuration | **fires** |
| M8b | `SY-04.b` | `open-epoch` stops revalidating the configuration | **fires** |
| M9 | `SY-08` | the launch recomputes from `World.live` instead of launching the selection | **fires** |
| M9b | `SY-08` | selection stops being taken once (`no p.sel` dropped) | **fires** |
| M10 | `SY-10.a` | the stale ambient **queues** instead of being refused | **fires** |
| M10b | `SY-10.a` | the **grant** site stops re-checking the generation | **fires at 7 states; SURVIVED at 5 and at 6** — see the bound note above |
| M10c | `SY-10.a` | an ambient tree operation stops needing an admission it holds | **fires** |
| M11 | `SY-10.b` | the timeout rewrites the epoch it is forbidden to touch | **fires** — the first attempt did not; see below |
| M11b | `SY-10.b` | the iteration boundary silently un-parks a blocked process | **fires** |
| M12 | `SY-05.a` | `initialise-root` reuses a **retired** identity | **fires** |
| M12b | `SY-05.a` | a foreign write **un-retires** the grove whose name it re-occupied | **fires** |
| M12c | `SY-05.b` | the settle runs **without** the proof | **fires**, and is **not isolating** — see below |
| M13 | `SY-06.a` | the completion appends no leaf — a charter and nothing else | **fires** |
| M14 | `SY-06.b` | the completion **decides** on the union: the mere absence of the format witness | **fires** |
| M15 | `SY-07.a` | the allocation always **appends**, never reuses | **fires** |
| M16 | `SY-07.b` | a session allocates the finish leaf exactly as a driver does | **fires** |

**SEVEN MUTATIONS, SEVEN FIRINGS, NO SURVIVORS — AND THE BOUND SWEEP IS PART OF
THAT CLAIM RATHER THAN AN EXTRA.** The third incident below is a check green at a
bound too small to reach its own mutation, and it is indistinguishable from a
correct green without re-running the mutation at more than one bound. Every one
of the seven was swept from 2 states upward: **all seven first fire at 3, and
every check runs at 5, so each carries two states of margin.** The number is in
the record because *no survivors* is only worth something beside it.

**M12b IS THE ONE WORTH READING, AND IT IS ENTRY 039 TURNED INTO AN INSTRUMENT.**
The finish scope met the trace as the thing that killed three formulations of
`FN-28`: after the rename the name is free, the world occupies it, and it gives
what it put there the quarantined root's own identity. Here the same trace is a
**witness** — `witness_SY_05a_the_name_reoccupied_with_the_retired_identity` —
because absence is stated as a fact Grove established rather than as a state that
holds. What the mutation does is make the re-occupation *un-retire* the grove,
which is exactly the implementation that reads a tree at the name as evidence the
finish did not happen. It fires at 3 states. **The formulation is what turned a
counterexample into an ordinary case, and the mutation is what proves the
formulation is not merely quiet.**

**M12c IS NOT ISOLATING, and the neighbour is named rather than left implicit.**
Dropping the proof guard from `doSettleDeletion` fires `SY-05.b` and also fires
`SY-05.a`'s first conjunct, which says `retired` is written by a proven deletion
and by nothing else. That is the finish scope's sixth failure mode again — a
claim one of whose conjuncts is another claim's subject has no isolating
mutation. `SY-05.a`'s other three conjuncts have isolating mutations (M12, M12b)
or an isolating control (`expect_fail_EN_14_SY_05a`), so the overlap costs
nothing; recording it is what keeps the next reader from believing two
independent instruments where there is one and a half.

**`SY-01.b` has no protocol-level mutation, and this is a finding rather than a
gap.** Its release half — *ownership is released by process death* — is a
property of the platform, not of Grove: the kernel releases an advisory lock
when the holder ceases to exist. In any model that represents that honestly it
is a construction fact (`TheDeadHoldNothing`, chained through
`TheLeaseIsAGuard`), and a mutation of the *protocol* cannot reach it — both M2
and M2b are made unsatisfiable by the facts, which is why they read as
survivors. What the model does check is the half that is not construction: that
a return releases as ordinarily as a death, and that a successor proceeds, which
is what `witness_SY_01b` lands. Recorded for `formal-synthesis-k16`: an
obligation whose content is a platform property has no protocol-level mutation,
and the honest record is this paragraph rather than a fourth attempt.

**`SY-11.a` and `SY-11.b` share their only firing mutation**, which is the
finish scope's sixth failure mode — *a claim every one of whose conjuncts is
another claim's subject has no isolating mutation*. The neighbour list is the
honest record: M6a/M6 fires both; `expect_fail_EN_07_SY_11b` is isolating for
`SY-11.b`'s back-edge conjunct and touches `SY-11.a` not at all.

**M11's FIRST FORM WAS UNSATISFIABLE, AND ITS SHAPE IS NEW.** The mutation added
`World.gen' != World.gen` to `doTimeout` — beside the `launchSame` frame, which
says `World.gen' = World.gen`. The transition became unreachable, and an
unreachable transition reports exactly as a survivor does. **A mutation that
ADDS a conjunct to a transition must be checked against that transition's own
FRAME predicates**, because a frame is the one place a model states the opposite
of a mutation without naming the field. Rewritten to replace the frame rather
than contradict it, it fires immediately. This is a seventh entry for the finish
scope's *six ways for a mutation to fail its aim*.

**M5 AND M5b BOTH STILL SURVIVE AFTER THE `seen` RESET, AND THEY SURVIVE FOR
OPPOSITE REASONS.** The admission slice recorded both as *belt on fastened
braces, load-bearing the moment a slice resets `seen` per iteration*. That
prediction is half right, and the half that is wrong is the more interesting.

- **M5b (take-tree) is now a LIVE mutation that `SY-11.a` cannot see.** A
  differential probe settles it: *a `TakeTreeA` applied in a state whose
  predecessor already had `TreeG in p.seen`* is **unsatisfiable in the original
  and satisfiable in the mutant**. The clause is doing real work — the reset did
  create the re-acquisition the admission slice said nothing admitted. But
  `SY_11a` is stated over `p.seen' - p.seen`, the guards **newly** seen, and a
  re-acquisition adds nothing to `seen`; the check's antecedent is empty exactly
  on the traces the mutation newly admits. **The shape chosen to be robust
  against a sixth acquisition SITE is blind to a repeat at an existing one.**
  That is a property of the check rather than of the design, no `SY-` obligation
  states it, and inventing one is not this file's to do —
  `formal-synthesis-k16` inherits it.
- **M5 (grant) is still not live, and now for a stated structural reason.** The
  admission slice's reason was that `seen` does not change while a process is
  blocked. The reset does not change that, and the mechanism is now nameable:
  **`doIter` is guarded on `no p.waits`, so a blocked process cannot cross an
  iteration boundary** — the one thing that could have changed `seen` under a
  wait is exactly what the boundary's own guard excludes. The same differential
  probe is unsatisfiable in the mutant as well as in the original.

**Both answers came from a probe rather than from the check**, which is the
general lesson and the reason this section is longer than its table: a surviving
mutation is one of three things — an unsatisfiable mutation, a bound too small,
or a live mutation the check's shape cannot see — and **only a differential
probe (satisfiable in the mutant, unsatisfiable in the original) tells the third
from the first**. This slice met all three in one session.

## Retained counterexamples

**`EN-07`, the shared-lock scope — and it is exactly the option
[`bulk-marks-are-not-atomic`](../../docs/adr/bulk-marks-are-not-atomic.md)
rejected.** Command
`expect_fail_EN_07_SY_11b_a_shared_lock_scope_reintroduces_the_cycle`, three
states:

```text
state 0   session S: holds {Tree},          seen {Epoch, Tree}
          driver  D: holds {Lease, Epoch},  seen {Lease, Epoch}
state 1   S nested-acquires Epoch           -> Deferred, S.waits = Epoch
state 2   D takes Tree                      -> Deferred, D.waits = Tree
          (loop)   S waits on D, D waits on S
```

The ADR's third rejected option is *hold Grove's own exclusive guard around the
whole run and let the library take its guard inside it*, rejected because two
open file descriptions on one directory do not share a lock. `EN-07` is that
fact; removing it removes the reason Grove's architecture is *two locks, one at
a time*, and the back edge — a tree guard held across a generation acquisition —
becomes admissible. Across two processes it closes.

**`EN-14`, the root removed — TWO CONSEQUENCES, AND THE SECOND ARRIVES WITH THIS
SLICE.** Command `expect_fail_EN_14_SY_01a_ownership_has_nothing_to_be_held_on`:
a driver holds a lease on `WtId$0`, `doRemoveRoot` puts `WtId$1` at the same
path, and a second driver acquires on `WtId$1`. Two live drivers on one working
tree.

Command `expect_fail_EN_14_SY_05a_absence_stops_discriminating_a_fresh_tree` is
the same step read at the task root instead of at the lease:

```text
state 0   World.rooted = Grove$0,  World.extant = {Grove$0}
state 1   remove-root                -> World.wt = WtId$1
          World.rooted = none,       World.extant = {Grove$0}
```

The grove is still there. It is in the directory Grove can no longer reach, and
Grove now observes an empty working tree — which `SY-05.a` says means *start a
new grove*. **The next scaffold would run over a live grove.** This is what the
assumption buys and it is why `World.extant` exists as a field: with `extant`
pinned to `rooted` by an `always` fact the two could never part company, the
mutation would be unsatisfiable, and an unsatisfiable mutation reports exactly as
a survivor. The correspondence is a **state-0** fact (`GrovesStartAtTheirName`)
for precisely that reason — the same lesson `OneLeaseHolderPerRoot` taught this
file one slice earlier.

**THE NAME RE-OCCUPIED WITH THE RETIRED IDENTITY, retained as the shape rather
than as a defect** — entry 039's trace, and the reason `SY-05` is formulated the
way it is. Command
`witness_SY_05a_the_name_reoccupied_with_the_retired_identity`:

```text
state k     settle-deletion   -> World.rooted = none
                                 World.extant = {},  World.retired = {Grove$0}
state k+1   foreign-write     -> World.rooted = Grove$0     <- THE RETIRED ONE
                                 World.retired = {Grove$0}  <- UNCHANGED
```

Read as *the task root is absent*, state k+1 falsifies the finish having
succeeded — which is what killed three formulations of `FN-28` in the finish
scope. Read as *Grove established and preserves the fact of a proven deletion*,
it changes nothing: the world owns the name, and a tree at a name is not evidence
about a grove. Under **M12b** the same two states produce `World.retired = {}`,
which is the implementation that reads presence as a receipt.

**This control caught a claim made true by construction.** The file's first pass
had `always all g: Guard | lone holds.g`, which says *one live driver per
working tree* — `SY-01` — as a fact. The mutation could not fire, and an
unsatisfiable mutation reports exactly as a survivor. The construction fact is
now one holder **per root** (`OneLeaseHolderPerRoot`), and the lease is
deliberately outside `OneLockOneHolder`.

**A LEAF INSERTED DURING THE LAUNCH WINDOW, retained as the shape rather than as
a defect.** Command `witness_SY_08_a_leaf_inserted_during_the_launch_window`,
the three consecutive states that matter:

```text
state k     select   by driver D        -> D.sel = a,  World.live = {a}
state k+1   tree-op  by session S       -> World.live = {a, b}
state k+2   launch   by driver D        -> World.running = a,  b still live
```

It is a witness rather than a counterexample because the model is right. What it
retains is the **window as a state a trace passes through**: a model in which
selection and launch are one transition has no state `k+1` to insert into, and
answers `SY-08` by construction. Under **M9** — the launch recomputing from
`World.live` — the same three states produce `World.running = b`, which is the
claim's *preempting the running one* exactly.

## Three incidents worth carrying forward

**A `lone` field under `not in` is false when the field is empty.**
`p.waits not in p.holds` reads as *nobody waits for what they already hold*, and
it is **false** exactly when `p.waits` is empty, because `none in X` is true for
every `X`. Written that way the fact said *every process is blocked in every
state*, and the whole file was unsatisfiable — `some Proc` had no instance. It
presents as a total, silent unsatisfiability with no error and no diagnostic:
every check green, every witness empty. The guard is `some p.waits implies`, and
the general rule is that a negated containment over a `lone` field needs a `some`
antecedent.

**A construction fact that states a claim makes the claim vacuous and makes
every mutation against it survive.** `SY-11.a`'s check was green for two rounds
while `StateZeroIsAStateTheStepsCouldHaveProduced` asserted `TreeG in p.seen
implies EpochG in p.seen` — which *is* the order the claim states. Three
mutations survived before the cause was found. The residue of the fact is the
two clauses about what is **held**, which rest on `needs` plus reverse release
rather than on any prohibition, and the requirement that pushed the order into
`seen` moved out of `needs` and into the discipline where it belongs.

**A check green at a bound too small to reach its own mutation.** `SY_10a` ran
at five states, reported no counterexample, and **M10b survived at five and at
six**. The defect it exists to catch — a grant admitting a session whose
generation rotated while it was blocked — needs six transitions to build, and
the check saw none of them. It is the *scope trap* the pre-registration names,
and it is distinguishable from the other two survivor causes only by re-running
the mutation at a larger bound. **The cheap question to ask before the fact is
the sibling scopes' second vacuity predictor**: does the bound hold the
machinery of the transitions the obligation quantifies over? `every acquisition
by a session` quantifies over the grant site, and the grant site's antecedent
is five transitions deep.

**A COUNTEREXAMPLE AT STATE 0, WITH NO TRANSITION IN THE TRACE AT ALL.**
`SY_07a`'s first run reported one immediately: a free initial state holding **two**
finish leaves. Nothing was wrong with the design, the transitions or the claim —
`doAllocFinish` is the only site that writes `World.fin` and it writes exactly
one. What was wrong is that *exactly one finish leaf* is an invariant the
transitions **preserve** and cannot **establish**, and this file's rule is that
state 0 stays wide open. The fix is the file's own idiom, a state-0-only fact
with no `always` on it — the fourth such, beside `TracesStartWithNobodyBlocked`,
`LeasesStartBoundToTheLiveRoot` and `SelectionsStartInsideTheTree`.

**The general shape, and it is worth stating once for `formal-synthesis-k16`:**
in a model with a free initial state, every checked invariant must be classified
as *establishable* or *preserve-only*, and a preserve-only invariant needs a
state-0 fact justified by what the steps already require. Written as an `always`
fact it asserts the claim and every mutation against it survives; written as
nothing at all it reports a counterexample that is about the initial state rather
than about the design. **Both failure modes look like a result.**

All four share one shape: **a check that is green because nothing can reach it —
or red because something no step reaches can — looks exactly like a check whose
verdict is about the design.** The mutations are the only thing that told the
green ones apart, the third needed them run at more than one bound, and the
fourth was told apart by reading the counterexample instead of the verdict.
