# grove-finish — models

The finish/recovery scope of [the semantic
contract](../../../docs/specs/semantic-contract.md): the `FN-` claims, checked
independently by each model family. This directory exists before the crate does,
which is deliberate — the model is what the crate will be cut against.

Run them from the repository root:

```sh
models/run.sh --scope finish --family alloy --no-coverage
```

## What is covered, and what is not

| family | file | obligations |
|---|---|---|
| Alloy 6 | `finish.als` | `FN-01`, `FN-05` – `FN-08` — the transaction's **entry surface** |
| Quint | — | none yet (`quint-models-k10`) |

**The `--no-coverage` on the run line above is the signal that this column is
still being built**, and it is what leaves it when the column closes. Fifty-three
of the scope's sixty-one alloy cells are empty, and that is the truth about the
repository rather than a defect in the instrument: each belongs to a sibling leaf
of `finish-k8` (`witness`, `commit`, `handoff`, `exits`). The runner prints the
matrix in full on every run whether or not it is asserted.

**Declared gaps** — none. The runner reads them from this file, in one shape:

```md
- **GAP** alloy `FN-nn.x` (inexpressible|abstracted|out-of-bounds|tool-limited) — reason.
```

**Two obligations of the *task-tree* scope are waiting on this directory, and
neither can be filled from either side as the placement rule stands.**
`crates/grove-task-tree/models/README.md` declares `TT-24.c` and `TT-24.d`
`out-of-bounds`, both because the context each names is a finish context: `TT-24.c`
is `Blocked(OwnershipConflict)` inside a finish or recovery transaction, and
`TT-24.d`'s subject is the quarantine reaper. This model will have both machineries
— `FN-25` and `FN-21` are exactly their subjects — but the runner's placement rule
sends every `TT_`-prefixed command to the task-tree directory, so a `TT_24c`
command *here* is a placement failure rather than a filled cell. Whether the two
should be re-stated as `FN-` obligations is `formal-synthesis-k16`'s to settle;
the re-statement would be a citation change rather than new modelling, because
`FN-21.c` and `FN-25` already carry the same content under `FN-` prefixes.

**Q4's artifact/transition removal matrix is not here yet.** The catalogue
requires one, in this file, per family — one row per removable artifact naming
the first shared-safety obligation its removal breaks, or `none`. It belongs to
the `exits` sibling, which is the leaf that has every shared-safety claim in
front of it; a matrix written before `FN-24` and `FN-27` exist would have nothing
to name.

## `finish.als`

**Tool.** Alloy 6, `org.alloytools.alloy.dist.jar`, on Corretto
`21.0.12.1+9-LTS`. The measurement host's default `java` is Corretto 16.0.1 —
below Alloy 6's floor — so the runner's own JDK probe is the difference between a
suite and a broken instrument that reports every check green and every witness
missing ([`docs/preservation-baseline.md`](../../../docs/preservation-baseline.md)
§1).

**Solver.** SAT4J, the distribution default. No command depends on a
solver-specific behaviour.

**Fairness.** None assumed, and none needed: every obligation in this slice is a
safety property or a reachability witness. Nothing here is a liveness claim, so
no command rests on a scheduler ever running anything.

**Bounds.** Stated per command. The common shape is
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, N steps`. Three parts of it mean
something other than "make it bigger":

- **`2 Device`** is `EN-02`'s dimension and nothing else. One device makes
  `FN-08`'s witness — a layout that passes at the lease gate and fails at the
  transaction's own operands — inexpressible rather than false, which is exactly
  what the assumption's *exercise-removal* control asserts.
- **No `Int` anywhere.** No `FN-` claim in this slice is arithmetic — there are no
  positions and no keys here — so the bitwidth that governs `task-tree.als` has
  no counterpart. The runner still passes `-n`; it simply has nothing to exclude.
- **`N steps`, and every `check` runs at 4.** The lasso argument
  `crates/grove-task-tree/models/README.md` records applies unchanged: at
  `2 steps` no applied transition exists, so every check conditioned on an
  outcome is vacuous. Three states admit one transition and a stutter. **Four is
  the file's check bound because four is the largest bound at which any witness
  of any obligation first lands** — see the next section, which is why.

### Every check runs at or above its own obligation's widest witness

The catalogue asks for the witness bound separately from the check bound because
*a claim whose witness first lands at the bound it was checked at has no margin*.
Measured, by re-running each witness at `1..5 steps` and taking the first that
lands:

| witness | first lands at |
|---|---|
| `witness_FN_01a_a_transaction_never_entered_for_want_of_confirmation` | 2 |
| `witness_FN_01b_a_confirmed_attempt_refused_for_want_of_the_guard` | **4** |
| `witness_FN_05a_p1_confirmation_absent` | 2 |
| `witness_FN_05a_p2_no_live_finish_leaf_or_live_ordinary_work` | 3 |
| `witness_FN_05a_p3_layout_unsupported` | **4** |
| `witness_FN_05a_p4_quarantine_target_unreachable` | 3 |
| `witness_FN_05a_p5_task_root_identity_unverified` | 3 |
| `witness_FN_05a_p6_empty_deletion_fingerprint` | 3 |
| `witness_FN_05a_p7_an_entry_type_that_cannot_be_digested` | 3 |
| `witness_FN_05b_a_refusal_with_the_tree_unchanged` | 2 |
| `witness_FN_05c_a_refusal_with_the_repository_unchanged` | **4** |
| `witness_FN_06_a_swap_between_two_steps_is_refused` | **4** |
| `witness_FN_07_a_wholly_untracked_tree` | 3 |
| `witness_FN_08_a_layout_that_passes_at_lease_acquisition_and_fails_here` | 3 |

**Four of the fourteen first land at 4, and the file's checks were originally
written at 3.** `FN-05.a` is the one where that mattered rather than merely
looked untidy: at `3 steps` the layout-unsupported member is unreachable, so a
check over "the preflight refuses exactly when some member fails" would have run
green having never once seen the third member fail. The two predictors
`task-tree-k7` left hold here and both would have caught it before the fact — the
bound must hold *the machinery of the transitions the obligation quantifies
over*, and an obligation whose member needs an intervening environment action
needs a state for it.

The rule this file adopts, and the one a sibling leaf should carry forward:
**a check runs at a bound at least as large as the widest first-landing bound
among the witnesses of the obligation it answers.** Here that is uniformly 4.

### Cost

Twenty-three commands, 38 s wall-clock for the whole file on the measurement
host, of which two commands are 9–10 s and the other twenty-one are under one
second each. The two are `witness_FN_05a_p5` and
`expect_unreachable_EN_02_…`; the second is the expensive shape by
construction, since establishing that no instance exists means exhausting the
space rather than stopping at the first model.

**These figures do not compare across sessions.** `task-tree-k7` measured the
same untouched command at 61 s in one sitting and 77 s in another. A slice's
imposition is an A/B on one host in one sitting, and the absolute numbers above
carry that caveat.

### Abstractions, and what this file deliberately does not model

Beyond the catalogue's own [deliberate
omissions](../../../docs/specs/semantic-contract.md#deliberate-omissions), which
this file adopts unchanged:

- **The tree is coarse: no filename grammar.** An entry is an opaque object with
  a type and a role. No `FN-` claim quantifies over names, positions, keys or
  slugs, so the grammar that occupies most of `task-tree.als` would be machinery
  no claim in this scope reads. What the seven preconditions actually need is
  leaf liveness, the finish/ordinary distinction, an undigestible entry type, and
  a tracked/untracked split — and nothing finer.
- **`Sys.why` is a model-only observable.** The catalogue fixes seven
  preconditions and seventeen refusal reasons and never states the mapping
  between them. `why` names which member refused. Nothing in the shipped contract
  corresponds to it, and no claim is stated over it that is not also stated over
  the outcome.
- **The lease gate is a recorded verdict, not a transition.** `SY-02` is the
  lifecycle scope's claim; this file consumes it as the fact that a recorded
  verdict cannot have been recorded by a gate that did not pass, and models only
  the part `FN-08` is about — that the verdict never licenses the transaction's
  own operands.
- **The transaction's body does not exist.** `TxnOpen` and `Preflight` are its
  only steps. There is no witness build, no evacuation, no commit, no
  disposition, no quarantine and no recovery; `Slot` is in the signature but is
  never occupied. Everything from `FN-09` onward is a sibling leaf's.
- **The initial state is unconstrained** beyond the two well-formedness facts,
  which is `EN-11` — *any well-formed tree is reachable by hand edit* — cashed out
  as a modelling decision rather than as a `hand-edit` transition. That is why
  this file needs no `HandEdit` action and why most witnesses need no run-up.
  `EN-11`'s own exercise-removal control belongs to the `exits` sibling, and it
  will have to remove this licence rather than an action.

### The refusal-reason mapping this file chose

The catalogue does not state which of its seventeen closed refusal reasons each
of `FN-05.a`'s seven members produces. This file chose:

| member | reason |
|---|---|
| confirmation absent | *none* — the transaction is never entered; `Decline` is not a transaction step |
| no live finish leaf, or live ordinary work | `NotLive` |
| layout unsupported | `LayoutUnsupported` |
| quarantine target unreachable | `LayoutUnsupported` |
| task-root identity unverified | `RootIdentityChanged` |
| empty deletion fingerprint | `NoTrackedDeletion` |
| an entry type that cannot be digested | `UnsupportedEntryType` |

**Two members share one reason, and that is not a modelling shortcut.** `SY-03`
says a preflight is never a licence and every gate revalidates against its own
operands, which makes *layout unsupported* and *quarantine target unreachable*
the same question asked at two gates. What follows is that a reason cannot say
which member refused — hence `Sys.why` — and that the two are distinguishable to
an operator only by which gate reported. Whether the shipped diagnostic should
distinguish them is `formal-synthesis-k16`'s, not this file's.

## What a green run of this file does not prove

- **Not that the seven preconditions are the right seven.** `FN-05.a` is checked
  as a biconditional between what the catalogue states (`pre1`..`pre7`) and what
  the transaction gates on (`gateWork`..`gateEntryType`), which are written
  separately so a divergence is a counterexample. A mutation that removes a
  member from *both* is invisible to it. That is a limit of any model whose
  transition relation is the thing under test, and the matrix below is what
  bounds it.
- **Not that preflight mutates nothing — only that the check would catch it if
  it did.** `FN-05.b` and `FN-05.c` are, in this slice, carried entirely by the
  frame conditions on `doPreflight`, because the entry surface contains no step
  that mutates anything. They separate two reachable behaviours only once
  evacuation exists. The `witness` sibling is where they stop being statements
  about the frame and start being statements about the protocol; until then, read
  them as *the claim is stated and the instrument works*, not as *the protocol
  was tested*.
- **Not anything about the lane.** The lane is in the signature from the first
  command and no obligation here distinguishes the three. `EN-16`'s collapse
  control — which is what makes a lane-blind model visible — is `exits`'.
- **Nothing outside the bounds.** A successful bounded check is evidence about
  the stated bounds, not proof about arbitrary executions. With three entries,
  two devices and four states, a protocol defect that needs a fourth entry or a
  fifth state is outside what any green above says.

## The mutation matrix

One mutation per obligation, run **before** the green was believed, each
restored afterwards. `KILLED` means the mutation's own check found a
counterexample.

| # | obligation | mutation | result |
|---|---|---|---|
| 1 | `FN-01.a` | `doTxnOpen` drops `some Op.confirmed` — a transaction step runs unconfirmed | KILLED |
| 2 | `FN-01.a` | `doDecline` sets `Op.confirmed'` — the transaction attests its own confirmation | KILLED |
| 3 | `FN-01.b` | `preflightGates` reads `gateWork or some Op.confirmed` — confirmation substitutes for the guard | KILLED |
| 4 | `FN-05.a` | `preflightGates` drops `gateQuarantine` while `pre4Quarantine` stays | KILLED |
| 5 | `FN-05.b` | `doPreflight`'s frame is removed and its refusal branch occupies the reserved slot | KILLED |
| 6 | `FN-05.c` | `doPreflight`'s frame is removed and its refusal branch moves the repository | KILLED |
| 7 | `FN-06` | `preflightGates` drops `gateIdentity` — the pin is never rechecked | KILLED |
| 8 | `FN-07` | `preflightGates` drops `gateFingerprint` | KILLED |
| 9 | `FN-08` | `gateQuarantine` reads `wtDev = qDev` — the transaction consults the lease gate's operands | KILLED |

**Three of the nine did not land as first written, and none of the three was a
fact about a check.** Retained because the *rules* are worth more than the fixes:

- **A mutation added underneath a frame condition is unsatisfiable, and an
  unsatisfiable branch reports exactly as a surviving mutation does.** Mutations 5
  and 6 first added `Slot.occ' = Reserved` and `Repo.rev' != Repo.rev` inside
  `doPreflight`'s refusal branch, which already sat under `treeSame and
  repoSame`. The branch became unreachable, the check stayed green for want of an
  antecedent, and the report read *SURVIVED*. The fix is to **remove** the frame,
  not to contradict it — and the general form is the one `selection-k34` and
  `ownership-k38` each met from a different direction: **a mutation the model
  cannot execute is not a control.** Every mutation above therefore carries
  evidence that it fires, which for 5 and 6 is one existing witness re-run under
  the mutation and still landing.
- **A mutation can be a semantic no-op and look like a survivor.** Mutation 2 was
  first written as `doTxnOpen` setting `Op.confirmed' = Confirmation`. Its guard
  already requires `some Op.confirmed`, and `Op.confirmed` is `lone Confirmation`,
  so the assignment changes nothing whatever. It was moved to `doDecline`, whose
  guard is `no Op.confirmed`, where it is a real change.
- **The no-op mutation also found a real hole in the file.** Nothing in it had
  demonstrated the `Confirm` transition at all: every witness could satisfy *some
  confirmation is present* from the unconstrained initial state, so
  `FN-01.a`'s second conjunct — confirmation changes only by the world's own
  action — was checked over a transition no command ever exercised.
  `witness_FN_01b` now requires the `Confirm` action, at a cost of one state.

## Counterexamples retained

None. No command in this slice found a counterexample that was a defect in the
catalogue or in the protocol; the only counterexamples produced were the nine
mutations' own, which are the matrix above. The two model defects this session
found — the `3 steps` check bound and the undemonstrated `Confirm` transition —
were both found by the mutation pass and by the witness-bound measurement rather
than by a check, which is itself the observation Experiment 2 entry 031 records.
