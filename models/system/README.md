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
| `lifecycle.als` | Alloy 6 | `SY-01`, `SY-02`, `SY-03`, `SY-11` — six obligations |

Nineteen of the twenty-five `(alloy, obligation)` cells are empty, and the
runner names each: `SY-04` – `SY-10` and `SY-12` – `SY-14`. They are the
`iteration`, `roots` and `sessions` sibling leaves', not gaps.

## The composition boundary

This scope's one discipline: **compose at observations, never at machinery.** A
`SY-` claim reads a task-tree or finish fact through the smallest observation
that decides it, and never through the machinery that produces it.

What this model does **not** have, and which model owns each:

| absent here | owner |
|---|---|
| filenames, positions, keys, slugs, species, digests | `crates/grove-task-tree/models/` |
| the eleven-state task-root classification (`TT-18`) | `crates/grove-task-tree/models/` — this file reads *present or absent* only, and `roots` is where absence becomes load-bearing |
| the witness slot, the evacuation manifest, the correlation ticket, the quarantine, the cleanup marker | `crates/grove-finish/models/` |
| the three lanes and every lane-specific mechanism | `crates/grove-finish/models/` — no `SY-` obligation in this slice differs by lane |
| the finish transaction's own preflight | `crates/grove-finish/models/`, `FN-05.a` member 3. This file's `doLayoutPreflight` is **the later gate as such**, standing for every subsequent revalidation; `SY-03` is the claim that connects them, and `witness_FN_05a_p3_layout_unsupported` is the same claim stated inside the finish scope |

A signature here that grows one of those rows is this file becoming a third copy
of two contracts rather than the joint of them.

## What is owed elsewhere, and is not a gap

- **`SY-02`'s session half is `SY-10`'s.** The check's fourth conjunct — a tree
  operation happens only under a lease — is stated **for a driver only**. A
  session reaches the tree by matching a live generation, and a live generation
  exists only because a driver holding a lease opened one; but `launch` is not
  modelled in this slice, so the session half would be stated over machinery
  this file does not have. Written unqualified it is false at state 0, and the
  check reported exactly that. `SY-10` — *a stale session cannot act* — is where
  it lands.
- **`EN-14`'s `SY-05` half is `roots`'.** The assumption table's row names both
  `SY-01` and `SY-05`; only the `SY-01` half is answered here, because `SY-05`'s
  machinery — task-root absence as an established-and-preserved fact — arrives
  with `roots`.
- **`SY-04.b`'s configuration content is `iteration`'s.** `doValidateConfig` is
  opaque here and exists only so `SY-02`'s *before configuration validation* has
  something to be before.

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
**`N steps` is N STATES**, an Alloy 6 trace being a lasso whose last state loops.

| obligation | command | steps | witness first lands at | wall |
|---|---|---|---|---|
| `SY-01.a` | `SY_01a_…` | 4 | — | 1.61 s |
| `SY-01.a` | `witness_SY_01a_a_second_driver_refused_while_the_first_holds` | 3 | **2** | 0.82 s |
| `SY-01.b` | `SY_01b_…` | 4 | — | 0.88 s |
| `SY-01.b` | `witness_SY_01b_a_crashed_driver_whose_successor_proceeds` | 5 | **4** | 0.92 s |
| `SY-02` | `SY_02_…` | 4 | — | 0.89 s |
| `SY-02` | `witness_SY_02_a_refusal_leaving_an_empty_working_tree_untouched` | 3 | **2** | 0.82 s |
| `SY-03` | `SY_03_…` | 4 | — | 0.87 s |
| `SY-03` | `witness_SY_03_a_layout_that_changes_between_the_two_gates` | 5 | **4** | 1.43 s |
| `SY-11.a` | `SY_11a_…` | 5 | — | 0.97 s |
| `SY-11.a` | `witness_SY_11a_the_full_order_reached` | **6** | **5** | 0.98 s |
| `SY-11.b` | `SY_11b_…` | 5 | — | 1.00 s |
| `SY-11.b` | `witness_SY_11b_a_real_wait_that_is_not_a_cycle` | 4 | **2** | 0.83 s |
| control | `expect_fail_EN_07_SY_11b_…` | 5 | — | 0.87 s |
| control | `expect_fail_EN_14_SY_01a_…` | 5 | — | 0.93 s |

`witness_SY_11a` is run at 6 and first lands at 5 **because it first landed at
the bound it was run at**. A witness whose first-landing bound equals its check
bound has no margin, which is the pre-registration's *scope trap* stated as a
number; the other five carry one or two states of it.

**Cost measurement is not meaningful at this size, and that is itself a
finding.** The JVM-plus-parse floor on this host is **0.58 s** (measured three
times on a one-signature file), so the whole fourteen-command file spends about
**4.6 s** actually solving. The sibling scopes' cost law — states-at-which-a-
transition-is-enabled × the bound, and ~10 ms of translation per static atom per
command — is unmeasurable here: every command sits within 2× the floor and the
run-to-run spread swamps any slice's imposition. Do not carry a percentage out
of this file. The reason is the composition boundary above: a model that reads
two contracts at their observations is *cheap in exactly the way a third copy of
them would not be*.

## Abstractions, and what a green run does not prove

- **`Proc.waits` is this file's own abstraction, not the catalogue's outcome
  set.** §*Outcomes* is explicit that a guard wait is not an outcome — Grove's
  tree lock blocks and no invocation returns while it is held — but `SY-11.b` is
  a claim *about* waiting, and a model in which a failed guard is an absent
  transition makes it true by construction. The sibling task-tree model met the
  same wall and introduced `Deferred`; this is the same move under a different
  name, and `Deferred` appears in `Result` for the same reason. **It is an
  abstraction, never a contract**: nothing in Grove returns it.
- **One process role distinction, and no more.** A driver and an ambient
  session, because with one role only the lease-holder ever reaches the epoch,
  no two processes contend, and `SY-11.b` would be checked over an empty
  wait-for graph. Nothing here reads a generation *value*; that is `SY-10`'s.
- **`seen` is never reset.** Within these bounds a process runs one admission
  cycle. A slice that models the loop's iteration will have to reset it, and
  that is `SY-04`'s.
- **`launch` and `reap` are not modelled.** Two of the catalogue's seven
  Lifecycle actions are absent because no obligation in this slice reads a
  spawn. They arrive with `iteration` and `sessions`.
- **A green run of this file is not evidence.** Two of its six obligations have
  no firing protocol-level mutation (below), and one check was green and vacuous
  for two rounds before the mutations found it.

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
| M5 | `SY-11.a` | the **grant** site drops the order clause | **survives** — see below |
| M5b | `SY-11.a` | the **take-tree** site drops the order clause | **survives** — see below |
| M6a | `SY-11.a` | the **open-epoch** site drops the order clause | **fires** |
| M6 | `SY-11.b` | the same open-epoch mutation | **fires**, and is **not isolating** |

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

**Two order clauses are unexercised within these bounds, and they stay.** M5 and
M5b survive for a reason worth stating, because it is easy to mistake for a
defect: a **grant** cannot violate an order the wait already satisfied — `seen`
does not change while a process is blocked — and the **take-tree** site's clause
can only be violated by a re-acquisition, which nothing else in this slice
admits. Both clauses become load-bearing the moment a slice resets `seen` per
iteration. They are belt on fastened braces today and are recorded as such.

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

## Two incidents worth carrying forward

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

Both incidents share one shape: **a check that is green because nothing can
reach it looks exactly like a check that is green because the design is right.**
The mutations are the only thing that told them apart.
