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
| Alloy 6 | `finish.als` | `FN-01`, `FN-05` – `FN-08` — the transaction's **entry surface**; `FN-09` – `FN-13` — the **reserved witness** |
| Quint | — | none yet (`quint-models-k10`) |

**The `--no-coverage` on the run line above is the signal that this column is
still being built**, and it is what leaves it when the column closes. Forty-five
of the scope's sixty-one alloy cells are empty, and that is the truth about the
repository rather than a defect in the instrument: each belongs to a sibling leaf
of `finish-k8` (`commit`, `handoff`, `exits`). The runner prints the matrix in
full on every run whether or not it is asserted.

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
to name. Two of its rows are already decided by the witness slice and are
recorded under *The mutation matrix* below, so `exits` transcribes rather than
re-derives them: **the reserved witness** and **the evacuation manifest's ready
mark**.

## `finish.als`

**Tool.** Alloy 6, `org.alloytools.alloy.dist.jar`, on Corretto
`21.0.12.1+9-LTS`. The measurement host's default `java` is Corretto 16.0.1 —
below Alloy 6's floor — so the runner's own JDK probe is the difference between a
suite and a broken instrument that reports every check green and every witness
missing ([`docs/preservation-baseline.md`](../../../docs/preservation-baseline.md)
§1).

**Solver.** SAT4J, the distribution default. No command depends on a
solver-specific behaviour.

**Fairness.** None assumed, and none needed: every obligation in these two
slices is a safety property or a reachability witness. Nothing here is a liveness
claim, so no command rests on a scheduler ever running anything.

**Bounds.** Stated per command. The common shape is
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, N steps`.
Four parts of it mean something other than "make it bigger":

- **`2 Device`** is `EN-02`'s dimension and nothing else. One device makes
  `FN-08`'s witness — a layout that passes at the lease gate and fails at the
  transaction's own operands — inexpressible rather than false, which is exactly
  what the assumption's *exercise-removal* control asserts.
- **`2 AttemptId`** and **`2 Digest`** are the witness slice's, and both exist so
  that a claim about recording *this* value is not the same statement as a claim
  about recording *some* value. At one atom each, `Man.mAttempt = Txn.attempt`
  and `Man.mDigest = Root.holds <: digest` hold for any manifest that records
  anything at all, and `FN-12.a` would be checking presence rather than content.
- **No `Int` anywhere.** No `FN-` claim in these slices is arithmetic — there are
  no positions and no keys here — so the bitwidth that governs `task-tree.als`
  has no counterpart. The runner still passes `-n`; it simply has nothing to
  exclude.
- **`N steps` now ranges from 3 to 10.** `entry-k39`'s file ran everything at 3
  or 4 because its transactions are two steps long. The witness slice's body is
  six, and every body witness runs the whole of it from a fresh grove, so the
  file's widest command is `FN-13` at ten states. The next section is why each
  command sits where it does.

### Every check runs at or above its own obligation's widest witness

The catalogue asks for the witness bound separately from the check bound because
*a claim whose witness first lands at the bound it was checked at has no margin*.
Measured, by re-running each witness at `2..12 steps` and taking the first that
lands. **All fourteen inherited rows were re-measured under `crash` and none
moved** — which is the answer to the question `entry-k39` left open, and it is
what a reachable-transition addition is *not* expected to do to bounds that never
needed it:

| witness | first lands at |
|---|---|
| `witness_FN_01a_a_transaction_never_entered_for_want_of_confirmation` | 2 |
| `witness_FN_01b_a_confirmed_attempt_refused_for_want_of_the_guard` | 4 |
| `witness_FN_05a_p1_confirmation_absent` | 2 |
| `witness_FN_05a_p2_no_live_finish_leaf_or_live_ordinary_work` | 3 |
| `witness_FN_05a_p3_layout_unsupported` | 4 |
| `witness_FN_05a_p4_quarantine_target_unreachable` | 3 |
| `witness_FN_05a_p5_task_root_identity_unverified` | 3 |
| `witness_FN_05a_p6_empty_deletion_fingerprint` | 3 |
| `witness_FN_05a_p7_an_entry_type_that_cannot_be_digested` | 3 |
| `witness_FN_05b_a_refusal_with_the_tree_unchanged` | 2 |
| `witness_FN_05c_a_refusal_with_the_repository_unchanged` | 4 |
| `witness_FN_06_a_swap_between_two_steps_is_refused` | 4 |
| `witness_FN_07_a_wholly_untracked_tree` | 3 |
| `witness_FN_08_a_layout_that_passes_at_lease_acquisition_and_fails_here` | 3 |
| `witness_FN_09a_the_transaction_is_entered_by_a_preflight` | 4 |
| `witness_FN_09a_an_interruption_immediately_after_publication` | **9** |
| `witness_FN_09b_an_interruption_inside_the_build` | **7** |
| `witness_FN_10a_a_discard` | **7** |
| `witness_FN_10b_a_refusal_to_discard_unclassifiable_content` | 2 |
| `witness_FN_11_the_interval_between_publication_and_commit` | **9** |
| `witness_FN_12a_a_manifest_interrupted_before_its_ready_mark` | **8** |
| `witness_FN_12b_a_refused_entry_type` | 3 |
| `witness_FN_13_a_commit_attempted_while_the_witness_is_tracked_refused` | **10** |

The rule this file adopts, and the one a sibling leaf should carry forward:
**a check runs at a bound at least as large as the widest first-landing bound
among the witnesses of the obligation it answers**, with the file's conventional
minimum of 4 as a floor where that number is smaller. Applied: `FN-09.a` at 9,
`FN-09.b` and `FN-10.a` at 7, `FN-10.b` and `FN-12.b` at 4, `FN-11` at 9,
`FN-12.a` at 8, `FN-13` at 10, and the entry surface's eight unchanged at 4.

**`FN-11` is the file's first interval claim, and it cost exactly what
`task-tree-k7`'s first bound-vacuity predictor said it would.** *The task root
present, unwalkable and holding every entry* is not a state; it is a stretch of
trace with a publication before it and an attempted commit after it. Nine states
is what holds `TxnOpen`, `Preflight`, and all six body steps with a stutter to
close the lasso, and the predictor was applied before the command was written
rather than after a mutation survived.

### Cost

Forty commands, **2 m 13 s** wall-clock for the whole file on the measurement
host, against `entry-k39`'s twenty-three commands in **23 s** in the same
sitting. Two figures separate the transitions' cost from the states':

- **Eight reachable transitions cost the INHERITED commands ~+55%.** The same six
  entry-surface commands, unchanged, run at 0.9 s each on `entry-k39`'s file and
  1.4 s each on this one — an A/B on one host in one sitting. That is squarely
  the "expensive kind" the cost model predicts for reachable-transition
  additions, and the file has eight of them where the model's worst prior data
  point had four.
- **The new commands are 1.6 s – 2.7 s each**, so the suite's 23 s → 133 s is
  mostly the seventeen new commands rather than a blow-up in the old ones.

**A THIRD MEASUREMENT RULE, and it is new.** `task-tree-k7` established that
whole-suite totals do not compare across sessions. This file adds that **a single
command's cost is bimodal within one sitting**:
`witness_FN_11_the_interval_between_publication_and_commit` measured 2.0 s, 10.1 s
and 2.0 s on three consecutive runs of the same bytes — a 5× swing with nothing
changed. SAT4J's search is not a stopwatch. An A/B on **one** command is not
evidence at any granularity; the figures above are each the median of three, and
a slice that reports a single sentinel's before-and-after is reporting noise.

### Abstractions, and what this file deliberately does not model

Beyond the catalogue's own [deliberate
omissions](../../../docs/specs/semantic-contract.md#deliberate-omissions), which
this file adopts unchanged:

- **The tree is coarse: no filename grammar.** An entry is an opaque object with
  a type, a role and a digest. No `FN-` claim in these slices quantifies over
  names, positions, keys or slugs, so the grammar that occupies most of
  `task-tree.als` would be machinery no claim here reads. The two reserved names
  — `PREPARING-FINISH-<handle>-<attempt>/` and `FINISHING-<handle>/` — are
  modelled as one `Slot` with a **class**, which is the only thing `FN-09.a`
  reads about them: they are two names in one directory, so publication is one
  same-directory rename.
- **`Sys.why` is a model-only observable, and it now names post-flight
  conditions too.** The catalogue fixes seven preconditions and seventeen
  refusal reasons and never states the mapping between them. `why` names which
  condition refused. Nothing in the shipped contract corresponds to it, and no
  claim is stated over it that is not also stated over the outcome.
- **The digest is an opaque equality**, which is the catalogue's own abstraction
  (§*Deliberate omissions*): `FN-12` needs digests to distinguish entries, not to
  be collision-resistant. Nothing constructs one and two entries may share one.
- **The manifest's ready mark is also its "written and verified" record.** The
  ADR writes and verifies the manifest and then marks it ready, so the mark is
  the durable evidence that the verification passed. A separate `verified` field
  would carry no state the mark does not, and `FN-11`'s *beneath a manifest that
  has been written and verified* reads `some Man.mReady`.
- **The attempt identity and the repository anchor are opaque pins, not
  machinery.** They are drawn and recorded at `TxnOpen` because `FN-12.a`
  requires the manifest to record them. Nothing in this file reads them back:
  the classification that compares an anchor against an observed topology, and
  the correlation ticket that names an attempt, are the `commit` sibling's.
- **The lease gate is a recorded verdict, not a transition** — unchanged from
  `entry-k39`, and the verdict now explicitly *survives* a transaction that ends.
  It is the driver's, recorded before the transaction opens; a crash or a refusal
  does not un-record it.
- **The transaction's body is six steps and stops at the ATTEMPT.**
  `doCommitAttempt` records that a commit was attempted and mutates nothing: no
  commit, no correlation ticket, no anchor comparison, no disposition, no
  rollback, no quarantine, no reaper, no revalidation table. `FN-11` and `FN-13`
  both need a commit to have been *attempted* and neither needs one to have
  *happened*, which is what let this slice reach them without the `commit`
  sibling's machinery.
- **The body's step order is a phase machine, not a refusal on every out-of-order
  step**, and that is a scoping decision about the totality rule rather than an
  exception to it. The rule — every action returns exactly one outcome, and a
  failed guard is a named refusal — is about what an **invocation** returns. The
  body's six steps are internal control flow inside one invocation, not
  separately invocable operations, so "`WReady` before `WManifest`" is not a
  thing an operator can ask for and not a thing that needs a reason from the
  closed set. The three places a body step really can refuse an operator — the
  reserved name already occupied, the discard of unclassifiable content, the
  commit attempt over a tracked witness — are each reachable and each checked.

### Where a trace starts, and the one place this slice narrowed `EN-11`

`entry-k39` leaves the initial state wholly unconstrained and cites `EN-11` —
*any well-formed tree is reachable by hand edit* — cashed out as a modelling
decision rather than as a `hand-edit` transition. **That licence is about the
tree.** The entry surface could take it whole because its transactions are two
steps long and its witnesses need at most `Txn.phase = Opened`.

A six-step body cannot. An initial state at `Txn.phase = ReadyP` is not a
hand-edited tree; it is a running transaction nobody started, and **three
separate checks failed on one before the narrowing**:

| check | the state that broke it |
|---|---|
| `FN-12.a` | `Manifested` with only the anchor field written — a manifest half-written by no step |
| `FN-11` | `PublishedP` over an **absent task root** |
| `FN-12.b` | `ReadyP` with an undigestible entry in a root the preflight would have refused |

So `fact TransactionsStartWhereAProcessStarts` constrains state 0's
`Txn.phase` to `Fresh + Opened` and **nothing else**. Note the absence of
`always`: it is a statement about where a trace begins, not an invariant. The
disk stays completely free — the slot, its owner, what it holds, the manifest,
the root, the repository — which is what keeps a foreign reserved name
(`witness_FN_10b`) and an interrupted manifest reachable at state 0, and what
makes a crash still leave any body's disk behind at `Fresh` for recovery to read.

The price is that every body witness runs `TxnOpen` and `Preflight` in front of
its own steps, which is two states each. The gain is that they demonstrate the
protocol rather than assume it, and that
`witness_FN_09a_the_transaction_is_entered_by_a_preflight` is the file's **first
`Applied` preflight**: `entry-k39`'s fourteen witnesses are all refusals, so
until this slice the success branch of `doPreflight` was reached by no run in the
file at all. That is the same class of hole as the undemonstrated `Confirm`
transition entry 031 records, found the same way — by asking what the witnesses
actually execute.

### The refusal-reason mapping this file chose

The catalogue does not state which of its seventeen closed refusal reasons each
of `FN-05.a`'s seven members produces. This file chose:

| condition | reason |
|---|---|
| confirmation absent | *none* — the transaction is never entered; `Decline` is not a transaction step |
| layout unsupported | `LayoutUnsupported` |
| quarantine target unreachable | `LayoutUnsupported` |
| no live finish leaf, or live ordinary work | `NotLive` |
| task-root identity unverified | `RootIdentityChanged` |
| empty deletion fingerprint | `NoTrackedDeletion` |
| an entry type that cannot be digested | `UnsupportedEntryType` |
| the reserved name holds this attempt's own artifact | `WitnessPending` |
| the reserved name holds content Grove cannot classify | `ReservedNameOccupied` |
| **the repository has tracked the witness** | `WitnessPending` — **see below** |

**Two members share one reason, and that is not a modelling shortcut.** `SY-03`
says a preflight is never a licence and every gate revalidates against its own
operands, which makes *layout unsupported* and *quarantine target unreachable*
the same question asked at two gates. What follows is that a reason cannot say
which member refused — hence `Sys.why` — and that the two are distinguishable to
an operator only by which gate reported. Whether the shipped diagnostic should
distinguish them is `formal-synthesis-k16`'s, not this file's.

**`FN-13`'s refusal has no reason in the closed set, and that is a finding.**
`FN-13`'s stated witness is *a commit attempted while the witness is tracked,
**refused***, and none of the seventeen closed refusal reasons names a tracked
witness. This file reports it under `WitnessPending`, which is the closest true
statement the set admits — an artifact at a reserved name that Grove can prove is
its own — and keeps the case distinguishable through `Sys.why`
(`W8WitnessTracked`), exactly the device the two `LayoutUnsupported` members
already needed. **The consequence is that an operator cannot be told from the
reason alone that the *repository*, not the filesystem, is what is blocking.**
There are two exits and `formal-synthesis-k16` picks one: add a reason to the
closed set, or restate `FN-13`'s outcome as a `Blocked` — which is what
`task-tree-transactions-fail-closed` says happens ("a different revision,
**tracked witness**, restoration failure … keeps the witness unwalkable as
Recovery pending") and what `TT-24`'s own context table implies for a transaction
that has already mutated. The catalogue says *refused*; the ADR says *blocked*;
this model followed the catalogue, because the catalogue is the sole input.

## What a green run of this file does not prove

- **Not that the seven preconditions are the right seven.** `FN-05.a` is checked
  as a biconditional between what the catalogue states (`pre1`..`pre7`) and what
  the transaction gates on (`gateWork`..`gateEntryType`), which are written
  separately so a divergence is a counterexample. A mutation that removes a
  member from *both* is invisible to it. That is a limit of any model whose
  transition relation is the thing under test, and the matrix below is what
  bounds it.
- **`FN-05.b` and `FN-05.c` are no longer statements about the frame alone —
  and their antecedent narrowed when the body arrived.** `entry-k39` wrote them
  over *every* reported `why`, which was the same set when only `Preflight` and
  `Decline` could report one. The witness slice gives `why` three post-flight
  members, and a check that quantified over those too would be stating `FN-27` —
  *nothing unrelated is mutated, on any outcome* — under `FN-05`'s name, filling
  a cell no command had reached. They now read *at a preflight or a decline*.
  Within that, they are still carried mostly by frame conditions: the entry
  surface contains no step that mutates anything.
- **Not that the manifest is revalidated at the digest step.** `Root.holds`
  changes only by evacuation in this slice, because `EN-11` is cashed out as a
  free initial state and not as a `hand-edit` transition, so a manifest-time
  re-check of the entry types has **no reachable antecedent** and this file does
  not write one — writing an unreachable branch is how `entry-k39` produced three
  mutations that were not controls. `SY-03` would ask for one. `FN-12.b`'s check
  is stated over the whole body so that it would catch a violation if the world
  could ever produce it, and the third conjunct is currently discharged by the
  preflight rather than by a second gate.
- **Not that `evacuationComplete`'s `some Root.rid` is enforced by the
  transaction.** The claim requires the task root still to be present at the
  commit attempt (the ADR's *`.grove/` stays visibly present and unwalkable*);
  `gateEvacuated` does not check it. The check passes anyway, because the
  preflight's identity gate guarantees it upstream and nothing in this slice
  removes a root. The two sides are written apart precisely so that the day a
  step *does* remove the root — the quarantine rename, which is `handoff`'s —
  the divergence becomes a counterexample rather than a silence.
- **Not anything about the lane.** The lane is in the signature from the first
  command and no obligation in either slice distinguishes the three. `EN-16`'s
  collapse control — which is what makes a lane-blind model visible — is
  `exits`'.
- **Not that the step list is complete.** `FN-24.b` is the obligation that asks
  whether every step has at most one persistent effect and whether that effect is
  a same-directory rename or is itself decomposed. It is `exits`', it quantifies
  over `bodySteps`, and this file writes that set as one named thing so the
  question has something to be asked of. Until then, the six steps are this
  file's *proposal* for the crash boundaries, not a checked claim about them.
- **Nothing outside the bounds.** A successful bounded check is evidence about
  the stated bounds, not proof about arbitrary executions. With three entries,
  two devices, two attempt identities, two digests and ten states, a protocol
  defect that needs a fourth entry or an eleventh state is outside what any green
  above says.

## The mutation matrix

One mutation per obligation, run **before** the green was believed, each
restored afterwards. `KILLED` means the mutation's own check found a
counterexample. Every row carries **evidence that the mutation fires** — an
existing witness re-run under it, still landing — because a mutation the model
cannot execute reports exactly as a surviving one does.

Rows 1–9 are `entry-k39`'s and are unchanged. Rows 10–17 are the witness slice's,
and **all eight landed as first written**, which is what the three retained
lessons below were for.

| # | obligation | mutation | fires (witness still landing) | result |
|---|---|---|---|---|
| 1 | `FN-01.a` | `doTxnOpen` drops `some Op.confirmed` — a transaction step runs unconfirmed | — | KILLED |
| 2 | `FN-01.a` | `doDecline` sets `Op.confirmed'` — the transaction attests its own confirmation | — | KILLED |
| 3 | `FN-01.b` | `preflightGates` reads `gateWork or some Op.confirmed` — confirmation substitutes for the guard | — | KILLED |
| 4 | `FN-05.a` | `preflightGates` drops `gateQuarantine` while `pre4Quarantine` stays | — | KILLED |
| 5 | `FN-05.b` | `doPreflight`'s frame is removed and its refusal branch occupies the reserved slot | `witness_FN_05a_p1` | KILLED |
| 6 | `FN-05.c` | `doPreflight`'s frame is removed and its refusal branch moves the repository | `witness_FN_05a_p1` | KILLED |
| 7 | `FN-06` | `preflightGates` drops `gateIdentity` — the pin is never rechecked | — | KILLED |
| 8 | `FN-07` | `preflightGates` drops `gateFingerprint` | — | KILLED |
| 9 | `FN-08` | `gateQuarantine` reads `wtDev = qDev` — the transaction consults the lease gate's operands | — | KILLED |
| 10 | `FN-09.a` | `doWPublish` drops `rootSame` — the publishing step moves an entry too, so publication is not exactly one rename | `witness_FN_09a_an_interruption_immediately_after_publication` | KILLED |
| 11 | `FN-09.b` | `doWPrepare` builds the preparing witness already holding the root's entries | `witness_FN_10a_a_discard` | KILLED |
| 12 | `FN-10.a` | `doDiscard`'s branch condition gains `some Man.mReady` — the ready mark becomes a second input to the discard | `witness_FN_10b_a_refusal_to_discard_unclassifiable_content` | KILLED |
| 13 | `FN-10.b` | `doDiscard`'s refusal branch discards the unclassifiable content anyway | `witness_FN_10a_a_discard` | KILLED |
| 14 | `FN-11` | `doCommitAttempt` drops `gateEvacuated` — a commit is attempted over a half-evacuated root | `witness_FN_11_the_interval_between_publication_and_commit` | KILLED |
| 15 | `FN-12.a` | `doWManifest` leaves the entries' digests unwritten | `witness_FN_12a_a_manifest_interrupted_before_its_ready_mark` | KILLED |
| 16 | `FN-12.b` | `preflightGates` drops `gateEntryType` — the undigestible entry is not refused before mutation | `witness_FN_09b_an_interruption_inside_the_build` | KILLED |
| 17 | `FN-13` | `doCommitAttempt` drops `gateWitnessUntracked` — the candidate committed tree may include the witness | `witness_FN_11_the_interval_between_publication_and_commit` | KILLED |

**Rows 14 and 17 are the two removal-matrix rows `exits` inherits.** Removing the
`gateEvacuated` half of the commit attempt breaks `FN-11` first; removing the
`gateWitnessUntracked` half breaks `FN-13` first. Both are *incumbent mechanics*
claims, so neither is yet an answer to Q4 — the matrix `exits` writes needs the
first **shared-safety** obligation each removal breaks, and `FN-13` is the only
shared-safety claim either slice reaches.

**Three of `entry-k39`'s nine did not land as first written, and none of the three
was a fact about a check.** Retained because the *rules* are worth more than the
fixes, and because rows 10–17 were written against them:

- **A mutation added underneath a frame condition is unsatisfiable, and an
  unsatisfiable branch reports exactly as a surviving mutation does.** Mutations 5
  and 6 first added `Slot.occ' = Reserved` and `Repo.rev' != Repo.rev` inside
  `doPreflight`'s refusal branch, which already sat under `treeSame and
  repoSame`. The branch became unreachable, the check stayed green for want of an
  antecedent, and the report read *SURVIVED*. The fix is to **remove** the frame,
  not to contradict it — and the general form is the one `selection-k34` and
  `ownership-k38` each met from a different direction: **a mutation the model
  cannot execute is not a control.** Row 10 is that lesson applied: it removes
  `rootSame` from `doWPublish` rather than adding a contradicting conjunct
  underneath it.
- **A mutation can be a semantic no-op and look like a survivor.** Mutation 2 was
  first written as `doTxnOpen` setting `Op.confirmed' = Confirmation`. Its guard
  already requires `some Op.confirmed`, and `Op.confirmed` is `lone Confirmation`,
  so the assignment changes nothing whatever. It was moved to `doDecline`, whose
  guard is `no Op.confirmed`, where it is a real change. Row 13 was written with
  this in mind and *not* as "the discard returns the witness's contents to the
  root": a preparing witness holds nothing (`FN-09.b`), so
  `Root.holds' = Root.holds + Slot.wHolds` would have been the identity.
- **The no-op mutation also found a real hole in the file.** Nothing in it had
  demonstrated the `Confirm` transition at all: every witness could satisfy *some
  confirmation is present* from the unconstrained initial state, so
  `FN-01.a`'s second conjunct — confirmation changes only by the world's own
  action — was checked over a transition no command ever exercised.
  `witness_FN_01b` now requires the `Confirm` action, at a cost of one state.
  The same question asked of the witness slice found the same shape of hole: no
  command in the file reached `doPreflight`'s **success** branch, which is why
  `witness_FN_09a_the_transaction_is_entered_by_a_preflight` exists.

## Counterexamples retained

**Four, all from the witness slice, and all four are about the model rather than
about the protocol** — which is itself the observation, because a slice that adds
eight transitions and finds no protocol defect has still learned something about
what its instrument was licensing.

1. **`FN-09.b`, written the obvious way, fails at state 0.**
   `always (Slot.occ = Preparing implies no Slot.wHolds)` has a counterexample: a
   free initial state that hand-edits a preparing witness with something inside
   it. Under `EN-11`-as-a-free-initial-state, **every "never" claim about tree
   *shape* is false unless it is restated as a claim about what the protocol
   *does***. A `fact` would make the check vacuous and its mutation
   unsatisfiable — the trap this file records twice above — so `FN-09.b` is
   stated as two conjuncts over the transition relation instead: nothing is ever
   moved into a witness that is not published, and the witness this transaction
   builds is built empty. **This is a reusable rule, not an incident**, and it is
   the third bound/vacuity predictor this corpus now carries: *a shape claim
   under a free initial state must be restated over the transition relation.*
2. **`FN-12.a` fails on a manifest half-written by no step** — `Manifested` at
   state 0 with only `mAnchor` set.
3. **`FN-11` fails on a published witness over an absent task root** —
   `PublishedP` at state 0 with `no Root.rid`.
4. **`FN-12.b` fails on an undigestible entry inside an entered transaction** —
   `ReadyP` at state 0 holding an `OpaqueT` entry the preflight would have
   refused.

The last three are one counterexample wearing three hats, and the fix is one
line: `fact TransactionsStartWhereAProcessStarts`. Recording them separately is
deliberate — each was found by a different check, and a reader who meets only the
fact would not know how much it is load-bearing for.

**No command in either slice found a counterexample that was a defect in the
catalogue or in the protocol.** The two catalogue-level findings this file
carries — the seven-preconditions/six-reasons mismatch (entry 031) and `FN-13`'s
missing refusal reason (this slice) — were both found by trying to write a
*witness*, not by a check going red. That is now twice, and it is the strongest
methodological signal either slice has produced.
