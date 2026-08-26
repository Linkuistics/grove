# `models/system/` — the system-lifecycle scope

The joint of sessions, exhaustion, finish, interruption and recovery. The claims
are **not** here: they are
[`docs/specs/semantic-contract.md`](../../docs/specs/semantic-contract.md)
§*Claims — system lifecycle*, and this directory holds instruments rather than
statements.

```sh
models/run.sh --scope lifecycle --family alloy
```

**`--no-coverage` is gone from that line, and its absence is the signal that the
`SY-` column closed.** It stayed there for the three slices while the scope's
first family was mid-build; the `sessions` slice filled the last eight cells, so
the run now asserts coverage over the whole catalogue subset and fails if any
`(alloy, obligation)` cell is empty.

| file | family | covers |
|---|---|---|
| `lifecycle.als` | Alloy 6 | `SY-01` – `SY-14` — **all twenty-five obligations** |

**Zero empty alloy cells for the lifecycle scope, and zero declared gaps.** The
Quint column is `quint-models-k10`'s and is not this directory's to fill.

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
| `Empty` and `Ambiguous` as distinguishable observation outcomes (`TT-15`) | `crates/grove-task-tree/models/` — `doSelect` branches on `some World.live` and reads no value off either arm; **which** live leaf, and the `Empty`/`Ambiguous` distinction, are the task-tree model's |
| the configuration's contents — kinds, templates, the personal file and the delta | `complete-session-configuration` and `untracked-configuration-delta`. `SY-04.b` reads exactly one bit of it: whether full validation passes |
| the witness slot, the evacuation manifest, the correlation ticket, the quarantine, the cleanup marker | `crates/grove-finish/models/` |
| the three lanes and every lane-specific mechanism | `crates/grove-finish/models/` — no `SY-` obligation in this slice differs by lane |
| the block's **diagnosis** — `FN-25`'s `RecoveryPending`/`OwnershipConflict` partition, its disjointness, its exhaustiveness and its per-lane reachability | `crates/grove-finish/models/`. **`SY-14` reads one bit and one outcome**: *the tree is blocked*, and every tree action on it returns `Blocked`. `World.blocked` is `lone Flag` and carries no diagnosis, because no `SY-` obligation reads which one. A `Blocked` with internals here would be a third copy of the finish contract |
| what PUT the tree in a block — the recorded and observed topology, the artifact holding the transaction, the two operator-restorable exits (`FN-26`) | `crates/grove-finish/models/`. `doRecover` is **one opaque step** carrying only *recovery ran and could not settle*; which interruptions it can settle is `FN-20`'s classification. **The operator action that CLEARS a block is absent here and that is a limit rather than a gap**: §*Actions* puts operator actions outside the admitted set by construction, so at this scope *until an operator acts* is exactly *never, by anything this file has* |
| the signal file — its path, its bytes, its collision-resistance, and the loop control channel it lives on | `one-live-driver-per-working-tree`, and `crates/grove-finish/models/` for the correlation between a finish commit and its attempt. `SY-09` reads **two flags and their absence** (`World.signal`), and the driver's conclusion (`World.ending`) is a second field so that the inference `SY-09.c` forbids is statable at all |
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
- **`SY-14`'s OPERATOR EXIT is owed to `crates/grove-finish/models/` and is a
  limit rather than a gap.** *Until an operator acts* names an action this file
  does not have: §*Actions* puts operator actions outside the admitted set by
  construction, and `FN-26`'s two restorable exits are the finish model's.
  `SY-14.a`'s content is that no **admitted** action clears a block, and that is
  checked here over all twenty-two of them. **This is the sixth instance of the
  `TT-24` placement shape** — a claim whose remainder belongs to a sibling
  directory — and it joins `SY-06.b`'s ordering clause (the fourth) and
  `SY-05.b`'s other half (the fifth) in the closed table below.
- **`SY-09`'s SIGNAL FILE is `one-live-driver-per-working-tree`'s.** What
  crosses is two flags and their absence. The path, the bytes, the
  collision-resistance and the loop control channel are the ADR's, and the
  correlation between a finish commit and its attempt is the finish model's.
  **Not a gap** — `SY-09`'s three obligations are about what the driver
  CONCLUDES, and that is `World.ending` and is entirely here.

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
| `SY-09.a` | `SY_09a_…` | 5 | — | 1.60 s |
| `SY-09.a` | `witness_SY_09a_relaunch_reached` | 5 | **4** | 1.61 s |
| `SY-09.b` | `SY_09b_…` | 5 | — | 1.60 s |
| `SY-09.b` | `witness_SY_09b_done_reached` | 5 | **4** | 1.61 s |
| `SY-09.c` | `SY_09c_…` | 5 | — | 1.59 s |
| `SY-09.c` | `witness_SY_09c_no_signal_after_a_proven_teardown` | 5 | **4** | 1.61 s |
| `SY-12` | `SY_12_…` | 5 | — | 2.01 s |
| `SY-12` | `witness_SY_12_crash_after_acquire_lease` | 5 | **4** | 1.53 s |
| `SY-12` | `witness_SY_12_crash_after_layout_preflight` | 5 | **4** | 1.58 s |
| `SY-12` | `witness_SY_12_crash_after_open_epoch` | 5 | **4** | 1.58 s |
| `SY-12` | `witness_SY_12_crash_after_launch` | 5 | **4** | 1.53 s |
| `SY-12` | `witness_SY_12_crash_after_reap` | 5 | **4** | 1.57 s |
| `SY-12` | `witness_SY_12_crash_after_close_epoch` | 5 | **4** | 1.53 s |
| `SY-12` | `witness_SY_12_crash_after_release_lease` | 5 | **4** | 1.57 s |
| `SY-13.a` | `SY_13a_…` | 5 | — | 1.62 s |
| `SY-13.a` | `witness_SY_13a_the_longest_admitted_sequence_within_the_bound` | **9** | **8** | 5.20 s |
| `SY-13.b` | `SY_13b_…` | 5 | — | 1.58 s |
| `SY-13.b` | `witness_SY_13b_no_grove_is_not_a_sink` | 5 | **4** | 1.57 s |
| `SY-13.b` | `witness_SY_13b_a_partial_scaffold_is_not_a_sink` | 4 | **3** | 1.40 s |
| `SY-13.b` | `witness_SY_13b_a_spent_tree_is_not_a_sink` | 4 | **3** | 1.39 s |
| `SY-13.b` | `witness_SY_13b_a_transaction_part_way_is_not_a_sink` | 4 | **3** | 1.43 s |
| `SY-14.a` | `SY_14a_…` | 5 | — | 1.65 s |
| `SY-14.a` | `witness_SY_14a_a_block_reached_and_surviving_an_admitted_action` | 5 | **3** | 1.42 s |
| `SY-14.b` | `SY_14b_…` | 5 | — | 1.69 s |
| `SY-14.b` | `witness_SY_14b_an_action_on_a_blocked_tree_refuses_naming_the_block` | 4 | **2** | 1.31 s |
| control | `expect_fail_EN_07_SY_11b_…` | 5 | — | 1.11 s |
| control | `expect_fail_EN_14_SY_01a_…` | 5 | — | 1.23 s |
| control | `expect_fail_EN_14_SY_05a_…` | 5 | — | 1.31 s |
| control | `expect_unreachable_EN_08_no_lifecycle_step_is_a_crash_point` | 7 | — | 1.71 s |
| control | `expect_unreachable_EN_08_no_property_fails_when_crash_is_removed` | 6 | — | 2.81 s |

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
unchanged at **0.58 s**. *(The figures in this paragraph are the `roots` slice's and are
kept as its record; the `sessions` slice's A/B is the table below it.)* The forty-six-command file ran in **105 s and 113 s wall** across two
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

**THE `sessions` SLICE MADE THE FILE BIGGER AND THE SUITE BARELY DEARER, WHICH
CONFIRMS THE `roots` SLICE'S COST FINDING FROM THE OTHER SIDE.** Measured as an
A/B on one host in one sitting, which is the only way a suite total means
anything:

| | before this slice | after |
|---|---|---|
| commands | 46 | **73** (+59%) |
| wall | 104 s | **124 s** (+19%) |
| CPU | 251% | **361%** |

The `roots` slice was 53% more commands for **190%** more wall. This one is 59%
more commands for **19%**. The difference is exactly the thing entry 042 named:
`roots` added `Grove`, a **free `var`-referenced signature** at scope 3, and paid
a factor of five in the three commands whose search ranges over its atoms across
states. **This slice added no signature.** What it added is five **static** atoms
(`Sig`, `Ending`, and `Blocked`), four `var lone` fields on the `one sig World`,
and three reachable transitions — and the static-atom law (~10 ms per atom per
command) predicts ~50 ms per command, which is within the noise it was measured
in. **The prediction held.** The rule to carry: budget a static enumeration by
counting atoms, budget a `var` field on a `one sig` at nearly nothing, and budget
a free signature by measuring, because it is the only one of the three that costs
search rather than translation.

**AND THE CHEAPEST THING THIS SLICE DID TO THE FILE'S RUN TIME WAS TO MAKE A
TRANSITION MORE CORRECT.** The `doTreeOp` repair — one conjunct, `no
World.partial and no World.legacy` — is a **narrowing** of the step that
everything in the file reaches, and it moved the two dearest inherited commands
more than any bound change ever has:

| command | `roots` | before the repair | after |
|---|---|---|---|
| `witness_SY_10a_a_stale_session_refused` | 8.62 s | — | **2.28 s** |
| `SY_05a_…` | 4.16 s | — | **1.72 s** |
| `SY_09b_…` (new) | — | 9.68 s | **1.60 s** |
| `SY_12_…` (new) | — | 9.89 s | **2.01 s** |
| `SY_13a_…` (new) | — | 3.25 s | **1.62 s** |
| `SY_10a_…` | 3.23 s | — | 3.43 s (flat) |

`witness_SY_10a` was the file's dearest command after `roots` and is now an
ordinary one. **A guard on a widely-reached transition buys more than a bound
does**, because it removes traces from every command at once rather than states
from one — and the corpus's budgeting advice (prefer a static switch, then a
narrowed antecedent, then a smaller bound) has been about the *claim's* operands.
This is the same move applied to a **transition**, and it is a fourth entry on
that list. It arrived here as a correctness repair and the speed was a
side-effect, which is the honest order to report it in.

**ONE MEASUREMENT VARIANCE WORTH A LINE, because it would otherwise read as a
result.** `witness_SY_14b` measured **9.93 s** once and **1.31 s** on each of two
re-runs of the identical command and file. The table records 1.31 s. Per-command
wall on this host is not reliable to better than a factor of a few on a single
reading, which sharpens the sibling scopes' rule — *whole-suite totals do not
compare across sessions* — into: **a single per-command reading does not compare
with itself.** Take two.

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
- **`Blocked` IS THE CATALOGUE'S OWN OUTCOME AND IS THIS FILE'S FIRST IMPORT
  THAT IS NEITHER AN ABSTRACTION NOR A COPY.** §*Outcomes* lists it beside
  `Applied` and `Refused`, and its two neighbours in `Result` — `Deferred` and
  `Stopped` — are the two this file had to invent. It enters as a `Result`
  member carrying **no diagnosis**, and that is the composition decision:
  `SY-14` is stated over *a blocked tree* and `FN-25` over *which block*, so
  only the first crosses.
- **`doRecover` AND `doBlockedRefusal` ARE ONE OPAQUE STEP EACH, AND THE SECOND
  IS THE OPPOSITE OF THE `doAllocFinish` RULE.** That rule — *prefer splitting
  out a named transition to widening an opaque one, wherever a claim is about
  **which** mutation* — is stated above and this slice is the case it does not
  cover. `SY-14.b` is about **every** action on a blocked tree, and a per-action
  refusal branch would be twenty-six copies of one sentence with twenty-six
  chances to omit it. What carries the claim is a **single conjunct** in
  `mayTouchTree`, and M24 is its removal. **The rule generalises: split when the
  claim distinguishes actions, fold when it quantifies over them.**
- **`SY-13` AND `SY-14.a` ARE EXISTENTIAL-REACHABILITY AND SWEEP INSTRUMENTS,
  NOT LIVENESS PROPERTIES, AND A READER MUST NOT READ FAIRNESS INTO THEM.** The
  catalogue is explicit and gives the reason: *the loop WILL reach one* needs a
  fairness or admission premise these models have no grounds to grant, because
  nothing schedules the operator and `EN-15` says Grove cannot verify a
  confirmation. Concretely, in this file:
  - **the existential half of `SY-13` is carried by `run` commands**, because a
    `run` **is** an existential over traces. `witness_SY_13a_…` exhibits the
    longest admitted sequence within the bound; the four `witness_SY_13b_…`
    runs are the sweep, one per stable class, each reaching a goal.
  - **the two `SY-13` checks carry the half a run cannot** — that no admitted
    action of Grove's own destroys the escape (`SY_13a`), and that the four
    classes are all the non-goal states there are (`SY_13b`).
  - **`SY_14a`'s sweep is exhaustive BY BEING A CHECK**: `Sys.act' in
    AdmittedAct` ranges over all twenty-two admitted actions in every state
    within the bound, which no finite list of runs would give. The run beside it
    lands the non-vacuity — the division of labour `SY-05.b` uses.
  - **no command in this slice contains an `eventually` inside an `always`.**
    That is the mechanical form of the rule and it is checkable by reading.
- **`AdmittedAct` IS A SET AND ITS COMPLEMENT IS LOAD-BEARING IN BOTH `SY-13`
  AND `SY-14`.** §*Actions* puts `crash`, `hand-edit`, `foreign-write`,
  `topology-change` and `confirm` in an Environment group whose guard column
  reads *none — these are the world's*. A sweep that counted a hand edit as an
  exit would find no sink anywhere, and `SY-14`'s *until an operator acts* would
  have nothing to mean. **`IterA` is in neither set**: it is this file's own
  boundary abstraction, not something the loop *does*, which is why `SY-13.a`'s
  sequence length is reported twice — five admitted actions, six transitions.
- **`World.ending` IS A `set` AND `World.halted` IS ITS OWN FIELD, both to keep
  a claim from being a declaration.** This is the `World.fin` lesson applied to
  a third and fourth field. A `lone Ending` would have made `SY-09`'s *exactly
  one of three* true by construction; a `halted` derived from `ending` would
  have made `SY-09.a`'s *the loop continues* and `SY-09.b`'s *the loop ends*
  true by construction. Declared as they are, both are things `SY_09a` and
  `SY_09b` **establish** and things M17 and M18 break.
- **`World.signal` AND `World.ending` ARE TWO FIELDS BECAUSE `SY-09.c` IS ABOUT
  AN INFERENCE.** One field for both would make the inference the obligation
  forbids — reading *done* off something other than the flag — literally
  unstatable, and the check would be a restatement of a branch. Two fields make
  M19 (a reap that concludes *done* from a committed teardown) a
  counterexample instead of an impossibility.
- **THE EPOCH RECORD'S THIRD WRITE POINT IS STILL COLLAPSED, AND `doReap` NOW
  HAS A REASON TO WANT IT.** `reap` writes the record inactive in the ADR and
  does not here — see the entry above. Nothing in `SY-09` reads it: the loop's
  halt is `World.halted` and the session's ending is `World.ending`, and neither
  is the epoch. Recorded again because this is the slice where a reader would
  expect it to have changed.
- **A green run of this file is not evidence, and this slice is the sharpest
  demonstration of it in the corpus.** All twenty-one of its new commands were
  green on their **first** run, before a single mutation was written — and one
  of the two things the slice found was a defect that had been green in every
  command of the three preceding slices. Across the file: four of its
  twenty-five obligations have no firing protocol-level mutation or none that is
  isolating, one check was green and vacuous for two rounds before the mutations
  found it, one was green at a bound too small to reach its own mutation, and
  one was green because a transition let the loop out of a state it cannot
  really leave.

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
| M17 | `SY-09.a` | the relaunch branch halts the loop anyway | **fires** |
| M18 | `SY-09.b` | the done branch leaves the loop running | **fires** |
| M19 | `SY-09.c` | the reap **infers** `done` from a committed teardown (`some World.retired`) rather than from the flag | **fires** |
| M20 | `SY-12` | the reap does not consume the signal slot — a restart re-reads it | **fires** |
| M21 | `SY-13.a` | `initialise-root` marks the fresh root `legacy` instead of `partial` — Grove scaffolds into a sink | **fires** |
| M22 | `SY-13.b` | `prove-commit` writes a proof about a grove that is **not** the one at the name | **fires** |
| M23 | `SY-14.a` | the attempt on a blocked tree **clears** the block | **fires** |
| M24 | `SY-14.b` | `mayTouchTree` drops its `no World.blocked` conjunct | **fires** |
| M23b | `SY-14.a` | the **iteration boundary** blocks the tree — a block arriving from something other than recovery's decline | **fires**, and it is the ISOLATING one for `SY-14.a` |

**NINE MUTATIONS FOR THIS SLICE'S EIGHT OBLIGATIONS, NINE FIRINGS, NO
SURVIVORS — AND THE ISOLATION MATRIX IS PART OF THAT CLAIM RATHER THAN AN
EXTRA.** Every one of M17 – M24 was run against **all eight** of the slice's
checks, not only its own, and seven of the eight are **isolating**: they fire
exactly the obligation they are aimed at and nothing else. That is a better
result than either sibling scope's and it is worth saying why — this slice's
obligations are about four different fields (`ending`/`halted`, `signal`,
`legacy`/`live`, `blocked`), where `SY-05`'s four conjuncts and `SY-11`'s two
halves were about one apiece.

**M23 is the exception and M23b is the repair.** Clearing the block inside the
blocked-tree attempt fires `SY-14.a` **and** `SY-14.b`, because `SY_14b`'s
second conjunct asserts the attempt's frame and clearing the block breaks it.
That is the finish scope's sixth failure mode again. Rather than record the
overlap and stop — which the fourth attempt rule permits — a second mutation was
written against `SY-14.a`'s **other** conjunct, *a block arrives from recovery's
decline and from nowhere else*: the iteration boundary blocks the tree. It fires
`SY-14.a` alone. **A claim with two conjuncts about different things has an
isolating mutation even when one of its conjuncts does not**, and looking for it
cost one run.

**THE BOUND SWEEP.** Every one of the nine was swept from 2 states upward.
**Seven first fire at 3, M24 and M23b at 2, and every check runs at 5** — two or
three states of margin apiece. The number is in the record because *no
survivors* is only worth something beside it, and because entry 041's `M8`
incident was a mutation that survived at two bounds and fired at a third.

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

## The composition boundary, closed — a table for `formal-synthesis-k16`

The `SY-` column is complete, so this is the whole of what this scope imported
from its two siblings and the whole of what it declined to. **A reader of
`formal-synthesis-k16` should be able to use this table without opening
`lifecycle.als`.**

Twenty-five obligations, and **every one of them reads its sibling contracts
through an observation rather than through machinery.** The count that matters:
this file has **eleven `World` observations** standing for two contracts whose
own models carry, between them, filenames, positions, keys, slugs, species,
digests, an eleven-state classification, a witness slot, an evacuation manifest,
a correlation ticket, a quarantine, a cleanup marker, three lanes and a
repository. None of those appears here.

| what a `SY-` claim needs to be true | how it enters this file | who owns it | obligations |
|---|---|---|---|
| a task root exists / does not | `World.rooted`, an identity with no contents | `crates/grove-task-tree/models/` (`TT-18`) | `SY-02`, `SY-05`, `SY-06`, `SY-10` |
| a proven deletion is durable | `World.retired`, monotone; `World.proven`, one opaque step | `crates/grove-finish/models/` (`FN-11`, `FN-19`, `FN-28`) | `SY-05` |
| a root is a partial scaffold / is legacy | `World.partial`, `World.legacy` — two opaque marks, **not** two of eleven states | `crates/grove-task-tree/models/` (`TT-18`, `TT-20`) | `SY-06` |
| there is work to run | `World.live`, an **unordered** set of opaque handles | `crates/grove-task-tree/models/` (`TT-11`, `TT-12`, `TT-14`) | `SY-07`, `SY-08`, `SY-13` |
| exactly one finish leaf | `World.fin`, a **`set`** so the count is checked | `crates/grove-task-tree/models/` (`TT-13`) for the malformity; `SY-07` for the allocation | `SY-07` |
| the tree is blocked | `World.blocked`, `lone Flag`, **no diagnosis** | `crates/grove-finish/models/` (`FN-25`, `FN-26`) | `SY-13`, `SY-14` |
| a session reported / did not | `World.signal`, two flags and their absence | `one-live-driver-per-working-tree` | `SY-09` |
| the workspace layout is supported | `World.layout`, one bit, re-read at every gate | `supported-workspace-layouts` | `SY-02`, `SY-03` |
| the configuration validates | `World.cfg`, one bit | `complete-session-configuration` | `SY-04.b` |
| a launch generation is live | `World.gen`, an opaque identity | `one-live-driver-per-working-tree` | `SY-10` |
| the working-tree root's identity | `WtId`, an open directory and not a path | `one-live-driver-per-working-tree`, `EN-14` | `SY-01` |

**What this file added of its own, and each is an abstraction rather than a
contract**: `Proc.waits`/`Deferred` (a guard wait as an observable state),
`Stopped` and `RefConfigInvalid` (two situations the closed outcome and refusal
sets cannot name — a **finding**, named for `formal-synthesis-k16`), `IterA` (an
iteration boundary, which is not a catalogue action because a boundary is not
something the loop *does*), `World.ending`/`World.halted` (the driver's
conclusion and the loop's halt, two fields so that the inference `SY-09.c`
forbids is statable), and `initialise-root` as **two** steps.

**Three clauses are imported and unchecked here, and each is the `TT-24`
placement shape.** They are the fourth, fifth and sixth instances of it and
`formal-synthesis-k16` inherits the set:

- **`SY-06.b`'s ordering clause** — *completed **before** any format
  classification runs*. This file reads `partial` and `legacy` as marks already
  made and has no classification step; the order is `TT-18`'s.
- **`SY-05.b`'s other half** — the catalogue says `SY-05` and `FN-11`/`FN-19`
  SHALL be checked together, and an `FN_`-prefixed command here is a placement
  error. This file states the observation; the finish model states the steps.
- **`SY-14`'s operator exit** — *until an operator acts*. `FN-26` names the two
  restorable exits and they are the finish model's; §*Actions* puts operator
  actions outside the admitted set, so at this scope the phrase is exactly
  *never, by anything this file has*. **Not a gap** — `SY-14.a`'s content is
  that no *admitted* action clears a block, and that is checked.

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

**A `Legacy` TREE IS A SINK, AND THE DIFFERENTIAL PROBE IS WHAT MAKES THAT A
RESULT RATHER THAN A GUESS.** The un-narrowed `SY-13.b` — the same case analysis
with `no World.legacy` removed from its antecedent — reports a counterexample
immediately. Its cheapest form is a free initial state, so the probe was run
against a **reached** legacy tree instead, and against a reached partial scaffold
as the positive control:

```text
run  ... eventually (hand-edit puts a Legacy root at the name
                     and after (always only admitted actions
                                and eventually atGoal))
                                            6 states       9 states
  from a hand-edited Legacy root            NO INSTANCE    NO INSTANCE
  from an initialise-root PartialScaffold   instance       instance
```

The control is the whole point: *no instance* alone would be a statement about
the probe. Side by side, the pair says the escape exists for one stable class
and does not exist for the other, at two bounds — which is the third of the
three survivor causes told from the first, and it is the same instrument
`admission-k51` used on M5/M5b.

**AND IT WAS MASKED BY A MODEL DEFECT UNTIL THE REPAIR LANDED.** Before
`doTreeOp` learned the classification (the fourth incident below), the same
probe found an instance: a plain tree operation appended a live leaf to the
`Legacy` root, so the sink was not there to find. **A model defect that makes a
state escapable reports exactly as a design in which it is escapable.**

**A DRIVER HOLDING A LEASE UNDER AN INVALID CONFIGURATION IS A SECOND DEAD END,
AND IT IS RECORDED RATHER THAN FOLDED IN.** `SY-04.b` gates every Lifecycle
transition but `acquire-lease` on a valid configuration, so **`release-lease` is
unreachable while the configuration is invalid**: the driver can neither
release, nor open an epoch, nor launch, nor reap, nor close. A probe confirms it
— a driver holding a lease under `InvalidCfg` takes no Lifecycle action for the
whole trace.

**Whether that is a sink turns on one question and the answer decides it:
process death is NOT an admitted action.** §*Actions* puts `crash` in the
Environment group whose guard column reads *none — these are the world's*, so
the exit the shipped driver actually takes — die, and let the kernel release the
lease (`SY-01.b`) — is outside the set `SY-13` quantifies over. Under the
catalogue's own definitions the state is therefore a dead end.

**It is nonetheless not a counterexample to `SY-13` as this file states it, and
the reason is a scoping decision worth writing down.** §*States*' stable /
transient distinction is defined **over task-root states** — *a stable state is
one an ordinary invocation may observe and act on*, and every state in the table
it introduces is a root classification. A driver's own process state is not a
§*States* state at all, so `SY-13`'s antecedent is the root classification and
the loop-side state is out of scope. **The decision is recorded because the
alternative reading is available and would make `SY-13` false a third way.**

What the finding really is, and it is a **design** one rather than a modelling
one: `SY-04.b`'s gate is over-applied. `acquire-lease` is already exempt because
it runs before configuration validation; **`release-lease` deserves the same
exemption for a stronger reason** — a release touches no tree and launches
nothing, so there is nothing for a configuration to be valid *for*. Gating it
means an invalid personal configuration strands a lease that the loop then can
only escape by dying. Named for `formal-synthesis-k16` with the two available
repairs — exempt the release, or admit process death — and not fixed here.

**A `PartialScaffold` IS STABLE AND THE SWEEP CONFIRMS IT, which is a
calibration the next reader can use.** `roots-k53` created the interval between
`doInitRoot` and `doCompleteScaffold` and `CONTEXT.md` is explicit that it is
**stable** and not transient: an ordinary invocation observes it and acts on it.
`witness_SY_13b_a_partial_scaffold_is_not_a_sink` is that sentence as an
instrument — the state is reached, an ordinary `complete-scaffold` leaves it, and
a live leaf follows. **`Reserved(*)` is the transient class and it is
`crates/grove-finish/models/`'s**; this file never enumerates stability, it
reads the four classes its own observations distinguish.

## Five incidents worth carrying forward

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

**AN OPAQUE STEP THAT NEVER LEARNED A CLASSIFICATION THREE OF ITS NEIGHBOURS
DID — AND A SWEEP IS WHAT WALKED INTO IT.** `doTreeOp` is *any observation,
creation or mutation of the task tree*, written by the `admission` slice when the
task root was present-or-absent and nothing more. The `roots` slice introduced
`World.partial` and `World.legacy` and taught `doInitRoot`, `doAllocFinish` and
`doProveCommit` about them. **It did not come back to `doTreeOp`**, so in this
file a plain tree operation would append a live leaf to a `Legacy` root — a tree
with no format witness, which the shipped `grove-llm` refuses with
`FormatLegacy` and which §*States* says a whole-tree classification *stops every
read and mutation* of.

**No inherited obligation could see it, and that is the part worth carrying.**
None of the seventeen reads a tree operation against a classification: `SY-06.b`
owns the legacy refusal and owns it at `complete-scaffold`. Every check was
green, every witness landed, and the defect was invisible to all of them. What
found it was `SY-13`'s sweep, which asks of **every stable state** what leaves
it — and got the wrong answer for `Legacy` because the model let an ordinary
tree operation out of it. **A sweep over a state space finds transitions a
per-claim check set is not shaped to reach**, and it found this one on its first
run.

**It masked a design fact, which is the second half.** With `doTreeOp` open, the
`Legacy` sink below did not exist in the model; the differential probe that
established it only reported *no instance* after the repair. **A model defect
that makes a state escapable reports exactly as a design in which it is
escapable**, and the two are told apart only by asking whether the escape is one
the shipped system would take. The repair is one conjunct
(`no World.partial and no World.legacy`), written as a **guard** rather than as
a refusal branch, because a refusal branch would be a second statement of
`SY-06.b` in a step no obligation reads.

**A CLAIM QUANTIFIED OVER EVERY STABLE STATE IS QUANTIFIED OVER THE OPERATOR'S
WHOLE IMAGINATION.** `SY-13` says *from any stable state there SHALL exist a
bounded sequence of admitted actions reaching either a live leaf to run or a
terminal disposition*. `EN-11` says any well-formed tree is reachable by hand
edit. Put together, the antecedent ranges over states the loop never produced,
and **a `Legacy` tree is one of them and is a sink**: every admitted action
refuses `FormatLegacy`, and neither terminal disposition is reachable. Under the
un-narrowed check this reports immediately, and the differential probe
establishes it is the design rather than the bound — *from a hand-edited
`Legacy` root, no goal is reachable by admitted actions at six states or at
nine, while from a `PartialScaffold` one is at both.*

**The catalogue knows the shape and declines both repairs.** Its own note says a
`Malformed(reason)` tree is not a terminal disposition because folding it in
"would let the claim be satisfied by a tree nobody can act on" — which is right,
and which leaves the claim **false** rather than weak, on `Malformed` and on
`Legacy` and on `Foreign` alike. The repair it does not consider is the one this
file takes: **quantify over the stable states the loop's own admitted actions
reach**, and make *Grove never manufactures one of the others* a checked claim
(`SY_13a` conjunct 1, M21) rather than an assumption. The class is entry 042's
`SY-04.b` class again — **a claim stated over a system when it is true only of
what Grove does** — and this is its second instance in this file. Named for
`formal-synthesis-k16`; not fixed here, because the catalogue is both families'
shared subject and the independence protocol holds.

**The general shape, and it is worth stating once for `formal-synthesis-k16`:**
in a model with a free initial state, every checked invariant must be classified
as *establishable* or *preserve-only*, and a preserve-only invariant needs a
state-0 fact justified by what the steps already require. Written as an `always`
fact it asserts the claim and every mutation against it survives; written as
nothing at all it reports a counterexample that is about the initial state rather
than about the design. **Both failure modes look like a result.**

All six share one shape: **a check that is green because nothing can reach it —
or red because something no step reaches can — looks exactly like a check whose
verdict is about the design.** The mutations are the only thing that told the
green ones apart, the third needed them run at more than one bound, and the
fourth was told apart by reading the counterexample instead of the verdict.

# The Quint column — `lifecycle.qnt`

Written from [`docs/specs/semantic-contract.md`](../../docs/specs/semantic-contract.md)
alone, under the independence protocol
([`docs/formalism-findings.md`](../../docs/formalism-findings.md), *Experiment 2 —
pre-registration*, **Independence protocol**): this column's session opened no
`.als` file, no Alloy section of a model-directory `README.md` and no experiment
entry from the Alloy column's range. Everything below is what a second family
reached on its own, and entry 046 carries the one disclosure that qualifies it.

## Run line

```sh
models/run.sh --scope lifecycle --family quint
```

Two files. [`lifecycle.qnt`](lifecycle.qnt) carries the parameterised library,
the `base` instance and the `verify_small` model-checking instance;
[`lifecycle-controls.qnt`](lifecycle-controls.qnt) carries three focused
scenarios, two assumption controls and eighteen model mutations. The split is
not tidiness — see *Verification*, and
[`crates/grove-task-tree/models/README.md`](../../crates/grove-task-tree/models/README.md),
which found the reason the hard way.

Coverage is asserted: all 25 `SY-` obligations are answered by a property
command and a witness, and there are no declared gaps. Knobs, all environment
variables read by [`models/run.sh`](../run.sh): `QUINT_SAMPLES` (8000),
`QUINT_STEPS` (24), `QUINT_SEED` (fixed, so a green run is replayable),
`QUINT_VERIFY` (0 — see below).

## Verification

- **VERIFY** quint (depth-limited) — `quint verify` **completes and returns a
  verdict** on this subject, which is more than the task-tree column got and
  less than it sounds. Measured on the reduced `verify_small` instance (one
  process, crash-only environment, budget 1) with Apalache 0.56.1 and
  `JVM_ARGS=-Xmx16G`: three state invariants at `--max-steps=4`, **`NoError` in
  983s**, on the revision immediately before this column's last five model
  fixes. **Two things that measurement is not.** It is not the runner's own
  `QUINT_VERIFY=1` path, which batches all 25 properties into one call: that run
  was started and did not complete inside the session that built this column, so
  **the cost of the full batch is unmeasured and the line says so rather than
  implying it is affordable.** And it is not a green over `base`'s world. Depth 4
  reaches the lease, the configuration gate and the open epoch — and stops
  there: the shortest path to a completed scaffold is eight driver moves and the
  shortest to a proven finish is nineteen, so **no obligation about the tree,
  the finish, the block or the escape map is model-checked at all.** Every `SY-`
  property in this column is established by bounded randomized simulation, and
  no green run here is a proof over reachable states. What would change the
  answer is a smaller world, not a bigger heap: the transition relation branches
  over roughly a hundred Apalache transitions per step because `driverStep` is a
  chain of conditionals over a twelve-member classification, and each step costs
  about five minutes at that width. `QUINT_VERIFY=1` runs it;
  `QUINT_VERIFY_STEPS` and `JVM_ARGS` set the depth and the heap.

## What the model is, and what it is above

This scope is the **joint**, and the whole modelling decision is what it
declines to re-derive.

- `crates/grove-task-tree/models/` owns what a task tree **is**. Here the tree
  is a SUMMARY — present, format witness, format tag, reserved class, the exact
  scaffold subset, malformity, two live counts, a foreign artifact at a reserved
  name, and one byte token — and `classifyTree` is `TT-18`'s fixed order over
  it. No positions, no keys, no walk, no ordinal algebra.
- `crates/grove-finish/models/` owns the finish **transaction**. Here it is a
  five-phase cursor plus the two facts the rest of the lifecycle reads off it:
  whether the deletion is proven, and whether the correlation ticket is
  persistent. Its twenty steps, its manifest, its quarantine and its three VCS
  lanes are not represented, because no `SY-` claim quantifies over them.
- What this file DOES own is the driver: the lease, the layout gate, the
  configuration gate, the iteration, the launch generation, the three session
  endings, the guard order, and the composition of all of that with the two
  summaries above.

`SY-05` is the clearest case of why the scope exists at all. `FN-11` and
`FN-19` are checked next door; `SY-05.b` is the claim that they COMPOSE into a
sound inference — that no trace exposes an absent task root before the deletion
is proven, so `SY-05.a` may read absence as *start a new grove*. The model has
a dial (`DELETION_PROVEN_FIRST`) that removes the root one step early, and both
obligations die together.

**Every action is total**: each computes an outcome from one closed set through
one classifier (`outcomeOn`) and transitions in every case, so a refusal is a
value rather than an absent transition. That is what makes the refusal claims
falsifiable at all, and it is why `SY-14`'s exhaustive sweep can run through the
*same* classifier the real actions use rather than through a second spelling of
it.

## The driver is deterministic, and that is the search dial

The other two Quint columns dial the SEARCH because their subjects are a tree
of unknown shape and a twenty-step transaction. This subject is a **loop**, and
a loop is deterministic: given a lease, a validated configuration, an open epoch
and a classified root, exactly one move is next. Modelling it as a uniform
choice over every action it could ever take does not model a driver — it models
a random walk that happens to share the vocabulary, and it puts every deep
witness at `(1/k)^15`.

So `driverStep` **is** the loop, written as the chain of conditionals a loop is.
What stays nondeterministic is what genuinely is: which of the three endings a
session takes, and what the world does between two steps. The world's share is
capped three ways — `ENV_BUDGET` (how many actions one trace may take),
`ENV_KINDS` (of which kind) and `ENV_PHASES` (at which lifecycle point) — and
`base` grants all three wide open.

Measured: with the menu flat and unfocused, 5 of 25 witnesses landed at 2000
samples and 4 of those 5 were the shallow ones. With `driverStep`, 23 of 25 land
in `base` and the remaining two have scenarios. Nothing was removed from the
model to get there.

## Instances

| module | what it is | why |
|---|---|---|
| `base` | every assumption granted, every environment kind and point, budget 3 | the run all 25 properties and 21 witnesses are checked in |
| `scenario_teardown` | no environment action at all, one process | `SY-05.a` needs twenty-two consecutive driver moves inside a twenty-four-step trace, and every world action spends one |
| `scenario_crash_points` | crash and nothing else, budget 8 | `SY-12`'s witness is one crash point per lifecycle step |
| `scenario_recovery` | one crash, landing inside the transaction (`ENV_PHASES = Set(6, 7)`) | `SY-14`'s sweep needs a BLOCK, and a block needs an interrupted attempt a recovery cannot prove either way |
| `relax_EN_08` | `crash` removed | exercise-removal |
| `relax_EN_11` | `hand-edit` removed | exercise-removal, and the evidence behind this column's `SY-13` narrowing |
| eighteen `mutant_*` | one MODEL dial each | see below |
| `verify_small` | one process, crash-only environment, budget 1 | the model-checking instance |

**Which module a command runs in** is decided by one rule, defined in
[`models/run.sh`](../run.sh) under *THE MODULE RULE* and cited rather than
restated here.

## The controls, and what they establish

**Eighteen model mutations, and the count is itself the observation.** The
task-tree column needed two and the finish column eleven. This one needs
eighteen, and the reason is structural rather than incidental: **an executable
model of a deterministic loop satisfies almost every ordering claim it is
written to satisfy.** "The layout is proved at lease acquisition", "validation
precedes every transition", "at most one transition per iteration", "selection
is not recomputed", "a stale operation is refused before it touches the tree" —
each of those is true of `driverStep` because `driverStep` is written in that
order, and each would carry a green tick over no evidence at all. Every dial
below exists so that one named obligation has somewhere to die.

| control | obligation | result |
|---|---|---|
| `inv_fail_MUT_SY_01a_a_second_driver_is_queued` | `SY-01.a` | violated |
| `inv_fail_MUT_SY_01b_the_lease_survives_a_death` | `SY-01.b` | violated |
| `inv_fail_MUT_SY_02_the_tree_is_touched_before_the_layout_is_proved` | `SY-02` | violated |
| `inv_fail_MUT_SY_03_a_later_gate_honours_the_lease_time_check` | `SY-03` | violated |
| `inv_fail_MUT_SY_04a_two_transitions_in_one_iteration` | `SY-04.a` | violated |
| `inv_fail_MUT_SY_04b_a_transition_runs_without_validation` | `SY-04.b` | violated |
| `inv_fail_MUT_SY_05a_absence_is_read_as_evidence` | `SY-05.a` | violated |
| `inv_fail_MUT_SY_05b_absence_before_the_deletion_is_proven` | `SY-05.b` | violated |
| `inv_fail_MUT_SY_06a_a_scaffold_writes_only_a_charter` | `SY-06.a` | violated |
| `inv_fail_MUT_SY_06b_a_legacy_tree_is_completed_as_a_scaffold` | `SY-06.b` | violated |
| `inv_fail_MUT_SY_07b_a_session_creates_a_finish_leaf` | `SY-07.b` | violated |
| `inv_fail_MUT_SY_08_a_leaf_added_during_the_window_preempts` | `SY-08` | violated |
| `inv_fail_MUT_SY_09c_no_signal_is_inferred_as_done` | `SY-09.c` | violated |
| `inv_fail_MUT_SY_10a_a_stale_session_touches_the_tree` | `SY-10.a` | violated |
| `inv_fail_MUT_SY_10b_a_contended_generation_parks_silently` | `SY-10.b` | violated |
| `inv_fail_MUT_SY_11a_the_guard_order_is_violated` | `SY-11.a` | violated |
| `inv_fail_MUT_SY_11b_a_generation_wait_under_a_tree_guard_closes_a_cycle` | `SY-11.b` | violated |
| `inv_fail_MUT_SY_12_a_restart_repeats_a_completed_effect` | `SY-12` | violated |
| `inv_fail_MUT_SY_13b_a_hand_edited_refusal_state_is_a_sink` | `SY-13.b` | violated — **and this one is a finding about the catalogue**; see below |
| `inv_fail_MUT_SY_14a_an_admitted_action_clears_a_block` | `SY-14.a` | violated |

**Four obligations have no isolating mutation of their own, and each is stated
here rather than left to be inferred.** `SY-07.a` dies with
`mutant_session_finish` (a second finish leaf is how "exactly one" fails);
`SY-09.a` and `SY-09.b` die with `mutant_no_signal_is_done` (the same reap
classifier decides all three endings, so a dial on one is a bundle control over
the three); `SY-13.a` and `SY-14.b` are checked by construction over a declared
map and a shared classifier, and their falsification is `mutant_literal_sy13`
and `mutant_block_clears` respectively. **Those are BUNDLE controls, not
isolating ones**, and a reader should not read a green `SY-09.a` as separately
evidenced.

**The two assumption controls.** `EN-08` (`crash` removed) makes `SY-12`'s
witness unreachable and leaves every property green — which is exactly why a
green run under a collapsed dimension is the false confidence the
pre-registration names. `EN-11` (`hand-edit` removed) makes a `Legacy` tree
unreachable, which is also the measured evidence behind the `SY-13` narrowing
below: the three sink classes are precisely the hand-edit-reached ones.

## Abstractions

Beyond the catalogue's own [deliberate
omissions](../../docs/specs/semantic-contract.md#deliberate-omissions), which
this model takes as written:

- **Two processes**, per the catalogue's own omission, and the wait-for cycle
  test (`waitCycle`) is a bounded closure over exactly two. A cycle among more
  would not be found here, and no `SY-` obligation rests on one.
- **The task tree is a summary and the finish transaction is a phase cursor**;
  see *What the model is, and what it is above*.
- **`hand-edit` installs one of four enumerated well-formed trees** — legacy,
  foreign, malformed, spent — rather than composing single edits. `EN-11` grants
  that any well-formed tree is reachable by hand edit, so this is a search
  strategy over exactly the granted space; `relax_EN_11` removes the whole family
  with it.
- **A "lifecycle transition" is scaffold, complete-scaffold, append-finish or
  recover, and not a finish STEP.** The transaction's steps belong to the finish
  leaf's own session. `SY-04.a` is read as a property of the driver's iteration,
  which is what its own words say.
- **`SY-04.a` is an enabling condition, not an outcome.** See *Narrowings*.
- **Bounds**: two processes, four generations, five finish phases, twelve stable
  classes, trace depth 24, 8000 samples, environment budget 3.

## Narrowings and qualifications, each declared

**Three** obligations are checked over less than their literal text, and in all
three the gap is a **finding about the catalogue** rather than a gap in the
model. None is a declared `GAP`: each obligation is answered, and what is
narrowed is recorded here, in entry 046, and — for the first — in a control that
fires.

### 1. `SY-13.a` and `SY-13.b` are checked over the ADMITTED-REACHABLE stable classes

`SY-13` quantifies over "any stable state" and names exactly two terminal
dispositions, explicitly excluding `Malformed` from them. But `Legacy`,
`Foreign` and `Malformed` are reached by a hand edit and left by a hand edit,
and a hand edit is **not an admitted action** — `SY-13`'s own note puts operator
actions outside the admitted set by construction. So under the literal text
every one of the three is a sink and both obligations are FALSE.

`base` narrows the sweep to the classes Grove's own actions can produce
(`SWEEP_ALL_STABLE = false`). `mutant_literal_sy13` runs the literal text and
`inv_SY_13b_no_stable_state_is_a_sink` dies — which is what turns "the catalogue
is wrong here" from a remark into a fired control. `formal-synthesis-k16` owns
the disposition.

### 2. `SY-04.a`'s cap is a loop-control guard, not a refusal

"At most one lifecycle transition per iteration" is enforced as an enabling
condition (`loopAdmits`), because the catalogue's closed refusal set has **no
member for "deferred to the next iteration"** and inventing one would put a
second vocabulary beside the contract's. The consequence is that this half of
`SY-04.a` is not falsifiable through the outcome vocabulary at all — it is
checked over the iteration counter instead, and `mutant_many_transitions` is
what makes that check non-vacuous.

### 3. `SY-14.b`'s "naming the block" is read through `TT-24`'s table

The closed refusal set carries no reason spelled after a diagnosis. This model
reads `WitnessPending` as `RecoveryPending`'s refusal and `ReservedNameOccupied`
as `OwnershipConflict`'s, which is the mapping `TT-24`'s three-context table
makes for the same two artifacts — but the catalogue never states it **for
`SY-14.b`**, and a family that read it differently would check a different
claim.

## The green run this column stands on

```sh
models/run.sh --scope lifecycle --family quint
```

**72 commands, 25 of 25 cells complete, 0 declared gaps, 0 empty. Exit 0.**
2m 05s wall, 149s CPU on a 16-core host; quint 0.32.0, 8000 samples, depth 24,
seed `0x5e0a51d3c0ffee01`.

52 of those commands are the claims — 25 properties and 21 witnesses in `base`,
plus four witnesses in scenarios — and 20 are inverted controls that must go RED.

Witness trace counts, out of 8000, because a witness that lands once is a
different fact from one that lands everywhere and the *scope trap* hazard is
about exactly that difference:

| obligation | traces | | obligation | traces |
|---|---:|---|---|---:|
| `SY-01.a` | 6975 | | `SY-09.a` | 381 |
| `SY-01.b` | 2328 | | `SY-09.b` | 376 |
| `SY-02` | 2041 | | `SY-09.c` | 59 |
| `SY-03` | 2457 | | `SY-10.a` | 3361 |
| `SY-04.a` | 134 | | `SY-10.b` | 353 |
| `SY-04.b` | 1975 | | `SY-11.a` | 4059 |
| `SY-05.a` | 904 *(scenario)* | | `SY-11.b` | 772 |
| `SY-05.b` | 232 | | `SY-12` | 1295 *(scenario)* |
| `SY-06.a` | 1807 | | `SY-13.a` | 232 |
| `SY-06.b` | **16** | | `SY-13.b` | 380 |
| `SY-07.a` | **18** | | `SY-14.a` | 1161 *(scenario)* |
| `SY-07.b` | 1012 | | `SY-14.b` | 1161 *(scenario)* |
| `SY-08` | 36 | | | |

**The two bolded rows have almost no margin.** They are deterministic under the
fixed seed and therefore not flaky, but 16 traces in 8000 is a thin instrument
and a reader should treat `SY-06.b` and `SY-07.a` as the first two obligations
to re-check after any change to the model or the sample budget.

## What a green run here does not prove

- Every property is established by **bounded randomized simulation** — 8000
  samples, depth 24, two processes, an environment budget of three. `quint
  verify` completes only at a depth that does not reach a scaffold.
- The **three narrowings above are narrowings**: `SY-13`'s sweep, `SY-14`'s
  quantifier and `SY-04.a`'s cap are each checked over less than their literal
  text.
- **Four obligations have no isolating mutation** — `SY-07.a`, `SY-09.a`,
  `SY-09.b` and `SY-14.b` die only inside a bundle control.
- **Nothing here is evidence about the Alloy column**, which this session did not
  read. The `(family, obligation)` matrix is what settles that, and
  `cross-model-replay-k15` is where the barrier comes down.

Entry 046 of [`docs/formalism-findings.md`](../../docs/formalism-findings.md)
carries the five findings, the three observations, the false-confidence ledger,
and the independence disclosure that qualifies all of it.
