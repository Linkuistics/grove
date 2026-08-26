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
| `lifecycle.als` | Alloy 6 | `SY-01`, `SY-02`, `SY-03`, `SY-04`, `SY-08`, `SY-10`, `SY-11` — eleven obligations |

Fourteen of the twenty-five `(alloy, obligation)` cells are empty, and the
runner names each: `SY-05` – `SY-07`, `SY-09`, and `SY-12` – `SY-14`. They are
the `roots` and `sessions` sibling leaves', not gaps.

## The composition boundary

This scope's one discipline: **compose at observations, never at machinery.** A
`SY-` claim reads a task-tree or finish fact through the smallest observation
that decides it, and never through the machinery that produces it.

What this model does **not** have, and which model owns each:

| absent here | owner |
|---|---|
| filenames, positions, keys, slugs, species, digests | `crates/grove-task-tree/models/` |
| the eleven-state task-root classification (`TT-18`) | `crates/grove-task-tree/models/` — this file reads *present or absent* only, and `roots` is where absence becomes load-bearing |
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
- **`EN-14`'s `SY-05` half is `roots`'.** The assumption table's row names both
  `SY-01` and `SY-05`; only the `SY-01` half is answered here, because `SY-05`'s
  machinery — task-root absence as an established-and-preserved fact — arrives
  with `roots`.
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
| control | `expect_fail_EN_07_SY_11b_…` | 5 | — | 1.11 s |
| control | `expect_fail_EN_14_SY_01a_…` | 5 | — | 1.23 s |

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

**Cost measurement is still not meaningful at this size, and the second
measurement confirms the first.** The JVM-plus-parse floor on this host is
**0.58 s**, so the thirty-command file spends about **20 s** actually solving
out of **37 s** serial (**37 s wall** under `models/run.sh`, 355% CPU, 126 s
user). The slice added **six reachable transitions**, **seven `var` fields** and
**two new signatures**, and the file's dearest command moved from 1.43 s to
2.37 s — but that command's bound also rose from 5 to 7, so the two are not
separable and no percentage is offered. Every other command sits between 0.96 s
and 1.43 s, which is within 2.5× the floor, exactly as the admission slice
reported. **The reason is still the placement**: a model that reads two
contracts at their observations does not pay for their machinery.

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
- **A green run of this file is not evidence.** Three of its eleven obligations
  have no firing protocol-level mutation or none that is isolating (below), one
  check was green and vacuous for two rounds before the mutations found it, and
  one was green at a bound too small to reach its own mutation.

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

**`EN-14`, the root removed.** Command
`expect_fail_EN_14_SY_01a_ownership_has_nothing_to_be_held_on`: a driver holds a
lease on `WtId$0`, `doRemoveRoot` puts `WtId$1` at the same path, and a second
driver acquires on `WtId$1`. Two live drivers on one working tree.

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

All three share one shape: **a check that is green because nothing can reach it
looks exactly like a check that is green because the design is right.** The
mutations are the only thing that told them apart — and the third needed the
mutations run at more than one bound.
