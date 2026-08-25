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
| Alloy 6 | `finish.als` | `FN-01`, `FN-05` – `FN-08` — the transaction's **entry surface**; `FN-09` – `FN-13` — the **reserved witness**; `FN-03`, `FN-04`, `FN-14` – `FN-18` — the **commit and its disposition**; `FN-19`, `FN-20` — the **quarantine and the atomic root rename**; `FN-22` — the **four revalidation points and the ten-row table** |
| Quint | — | none yet (`quint-models-k10`) |

**The `--no-coverage` on the run line above is the signal that this column is
still being built**, and it is what leaves it when the column closes. **Twenty-one**
of the scope's sixty-one alloy cells are empty, and that is the truth about the
repository rather than a defect in the instrument: `FN-21` and `FN-31` belong to
`handoff-k42`'s remaining child (`disposal`) and the rest to the `exits` sibling
of `finish-k8`. The runner prints the matrix in
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
to name. **Five of its rows are now decided** and are recorded under *The mutation
matrix* below, so `exits` transcribes rather than re-derives them: the reserved
witness, the evacuation manifest's ready mark, and — this slice's — the
**correlation ticket** (`FN-04` first, a shared-safety claim), the **recorded
anchor** (`FN-16.a`, shared safety) and the **deletion fingerprint** (`FN-14`,
shared safety). The commit slice's three are the first rows whose first-broken
obligation is a *shared-safety* claim rather than incumbent mechanics, which is
what makes them answers to Q4 rather than notes toward one.

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
- **The commit slice adds no scope dimension**, and that is worth one line
  because it might have. `Disposition`, `Report` and `Reproducible` are all
  `one sig`s under the default `3`, and the correlation ticket is a relation over
  the `Entry` and `AttemptId` atoms that already existed. The two atoms the
  witness slice bought so a claim about recording *this* value would not collapse
  into one about recording *some* value — `2 AttemptId`, `2 Rev` — are exactly
  what `FN-04` and `FN-16.a` need, and at one atom each both claims would be
  unstateable rather than false.
- **`N steps` now ranges from 2 to 12, and THE REVALIDATION SLICE IS THE FIRST
  TO MOVE THE CEILING SINCE `witness-k40`.** It stood at ten from the witness
  slice through the handoff slice, and three separate things pushed it:
  `witness_FN_22h` — the return that cannot complete — needs the rename, the
  world taking the task-root name, the commit moving and the return that meets
  it, and lands at **twelve**; five more `FN-22` commands land at **eleven**; and
  `witness_FN_22a_the_posited_recovery_disk_is_reachable` runs the whole body
  from a fresh grove to a crash mid-evacuation and lands at eleven, which is
  exactly why no earlier slice could have run it. The ceiling was the reason the
  debt existed, and paying it is what moved the ceiling.
- **`Restored` is a new phase and `RevPoint` is a new static signature, and
  neither is a scope dimension.** `Restored` is one more `Phase` atom under the
  default `3`, and the four revalidation points are `one sig`s used only by the
  claims. What the slice does add to the state space is `doCommitMoves`' freedom
  over `Repo.tickets`, `Repo.tracked` and `Repo.rev` at three phases — see
  *Cost*.

### Every check runs at or above its own obligation's widest witness

The catalogue asks for the witness bound separately from the check bound because
*a claim whose witness first lands at the bound it was checked at has no margin*.
Measured, by re-running each witness at `2..14 steps` and taking the first that
lands; **all fifty-nine rows below have been re-measured under the revalidation
slice**, not only the sixteen it added. **Thirteen inherited rows moved, twelve
of them for one reason, and it is a SIXTH entry in this corpus's bound
register** — see beneath the table:

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
| `witness_FN_11_the_interval_between_publication_and_commit` | **10** |
| `witness_FN_12a_a_manifest_interrupted_before_its_ready_mark` | **8** |
| `witness_FN_12b_a_refused_entry_type` | 3 |
| `witness_FN_13_a_commit_attempted_while_the_witness_is_tracked_refused` | **10** |
| `witness_FN_03_a_retry_with_no_local_trace_settling_forward_on_the_ticket_alone` | **10** |
| `witness_FN_04_two_attempts_on_one_handle_the_earlier_ticket_rejected` | **8** |
| `witness_FN_14_unrelated_modified_work_present_across_a_successful_finish` | **8** |
| `witness_FN_15a_a_failure_reported_after_the_classification_over_an_exact_commit` | **9** |
| `witness_FN_15b_git_committed_reached_from_a_fresh_grove` | **11** |
| `witness_FN_15b_nativejj_committed_reached` | **8** |
| `witness_FN_15b_colocatedjj_committed_reached` | **8** |
| `witness_FN_15c_git_notcommitted_reached` | **8** |
| `witness_FN_15c_nativejj_notcommitted_reached` | **8** |
| `witness_FN_15c_colocatedjj_notcommitted_reached` | **8** |
| `witness_FN_15d_git_indeterminate_reached` | **9** |
| `witness_FN_15d_nativejj_indeterminate_reached` | **9** |
| `witness_FN_15d_colocatedjj_indeterminate_reached` | **9** |
| `witness_FN_16a_a_settle_with_the_recorded_anchor_moved_restores_nothing` | **10** |
| `witness_FN_16b_a_settle_with_the_attempt_bound_result_present_restores_nothing` | **10** |
| `witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit` | **10** |
| `witness_FN_17b_a_restoration_that_cannot_reproduce_it_blocks` | **9** |
| `witness_FN_18_a_proven_commit_reached_after_an_interruption_mid_evacuation` | **10** |
| `witness_FN_19_an_interruption_immediately_after_the_rename` | **10** |
| `witness_FN_20_a_leftover_artifact_present_while_the_tree_is_classified_fresh` | **8** |
| `witness_FN_22a_the_point_after_the_quarantine_rename_is_reached` | **10** |
| `witness_FN_22a_the_point_after_the_restoration_is_reached` | **10** |
| `witness_FN_22a_the_point_before_the_quarantine_rename_is_reached` | **9** |
| `witness_FN_22a_the_point_before_the_restoration_is_reached` | **9** |
| `witness_FN_22a_the_posited_recovery_disk_is_reachable` | **11** |
| `witness_FN_22b_a_late_landing_observed_before_the_restoration` | **10** |
| `witness_FN_22c_a_late_landing_observed_after_the_restoration` | **11** |
| `witness_FN_22d_a_rollback_that_completes_as_a_refusal` | **10** |
| `witness_FN_22e_an_undone_commit_observed_before_the_rename` | **10** |
| `witness_FN_22f_a_committed_becomes_notcommitted_after_the_rename` | **11** |
| `witness_FN_22g_a_committed_becomes_indeterminate_after_the_rename` | **11** |
| `witness_FN_22h_the_task_root_name_taken_while_the_quarantine_holds_the_root` | **12** |
| `witness_FN_22i_an_unchanged_committed_disposes_after_the_rename` | **10** |
| `witness_FN_22j_indeterminate_observed_after_the_restoration` | **11** |
| `witness_FN_22j_indeterminate_observed_before_the_rename` | **10** |
| `witness_FN_22j_indeterminate_observed_before_the_restoration` | **10** |

The rule this file adopts, and the one a sibling leaf should carry forward:
**a check runs at a bound at least as large as the widest first-landing bound
among the witnesses of the obligation it answers**, with the file's conventional
minimum of 4 as a floor where that number is smaller. Applied, after the
revalidation slice's sweep: `FN-09.a` at 9, `FN-09.b` and `FN-10.a` at 7,
`FN-10.b` and `FN-12.b` at 4, `FN-11` at 10, `FN-12.a` at 8, `FN-13` at 10, the
entry surface's eight unchanged at 4, the commit slice's twelve at 8 (`FN-04`,
`FN-14`, `FN-15.c`), 9 (`FN-15.a`, `FN-15.d`, `FN-17.b`), 10 (`FN-03`,
`FN-16.a`, `FN-16.b`, `FN-17.a`, `FN-18`) and 11 (`FN-15.b`), the handoff
slice's two at 10 (`FN-19`) and 8 (`FN-20`), and the revalidation slice's ten at
10 (`FN-22.b`, `.d`, `.e`, `.i`), 11 (`FN-22.a`, `.c`, `.f`, `.g`, `.j`) and 12
(`FN-22.h`).

**`FN-20` NO LONGER RUNS ABOVE ITS OWN RULE, AND NOTHING ABOUT `FN-20` CHANGED.**
The handoff slice set it at 8 against a witness that landed at 7, deliberately,
because it is that slice's only shared-safety claim and a bound equal to its
witness's would have given it no margin. Its witness now lands at 8 — it ends on
a classification, and `doClassify` lost an enabling point — so the deliberate
margin has been absorbed by a bound movement that has nothing to do with the
claim. **A margin taken above a measured floor is not stable across slices**,
which is worth one line for whoever wants one next: state the margin as a
number and re-state it after every sweep, or it quietly becomes the floor.

**A SIXTH ENTRY IN THE BOUND REGISTER, AND IT IS FOUND BY A THIRD QUESTION.**
Twelve of the thirteen moved rows went up by exactly one — `witness_FN_04`,
`witness_FN_14`, `witness_FN_20`, and all nine of `FN-15.b`, `FN-15.c` and
`FN-15.d` — and every one of them is a witness whose **last transition is a
`Classify`**. Nothing about those witnesses changed and nothing about
`doClassify`'s *effect* changed; what changed is that it is no longer enabled at
`Classified`, so a trace that ended on a classification can no longer close its
lasso by running the same classification again and needs a state to stutter into.

The register now carries three shapes, and they are worth keeping apart because
each is found by a different question:

1. **A step that stops being a no-op costs a state to every witness that ENDED on
   it** (`commit-k41`, `witness_FN_11` 9 → 10). Ask: what did a mutating step
   used to leave alone?
2. **A step inserted into a path costs a state to every witness that PASSES
   THROUGH it** (`quarantine-k43`, three witnesses 9 → 10). Ask: which witnesses
   traverse the point a new step was spliced into?
3. **A step that stops being ENABLED at a phase costs a state to every witness
   that CLOSED ITS LASSO on it** (this slice, twelve witnesses +1). Ask: which
   witnesses' final transition is one whose enabling surface you narrowed?

The thirteenth moved row is the second shape again: `witness_FN_17a` went 9 → 10
because the restoration was split into a restore and a release, and it passes
through both.

**EXACTLY ONE INHERITED ROW MOVED, and the reason is a shape change rather than a
state-space one.** All twenty-three of the witness slice's rows were re-measured
under the commit slice and twenty-two are unchanged;
`witness_FN_11_the_interval_between_publication_and_commit` went 9 → **10**, and
`FN-11`'s check with it. `doCommitAttempt` used to leave every field alone, so
the state it produced differed from its predecessor in `Sys.act` alone and the
trace could close its lasso on the spot. It now advances the phase to
`Attempted`, so the trace needs one more state to stutter into. **A step that
stops being a no-op costs a state to every witness that ended on it** — which is
a fourth entry in this corpus's bound register, and the cheapest of them to
check for: re-measure any witness whose last transition you made mutating.

**THREE INHERITED ROWS MOVED UNDER THE HANDOFF SLICE, ALL THREE FOR ONE REASON,
AND IT IS A FIFTH ENTRY IN THIS CORPUS'S BOUND REGISTER.**
`witness_FN_03`, `witness_FN_16b` and `witness_FN_18` each went 9 → **10**, and
their three checks with them. All three are the file's only witnesses that run
through the **forward settle**, and the handoff slice put a step in front of it:
a `Committed` classification now renames the task root into the quarantine and
settles after, so every trace that reaches a forward settle is one transition
longer than it was. Nothing about those three witnesses changed and nothing
about their last transition changed.

The register already carried *a step that stops being a no-op costs a state to
every witness that ENDED on it* (`witness_FN_11`, 9 → 10, above). This is the
other half and it is cheaper still to predict: **a step inserted into a path
costs a state to every witness that PASSES THROUGH it.** The two are worth
keeping apart because they are found by different questions — the first by
asking what a mutating step used to leave alone, the second by asking which
witnesses traverse the point a new step was spliced into. Forty-three witnesses
were swept from 2 to 14 states to establish that exactly three did.

**`FN-11` is the file's first interval claim, and it cost exactly what
`task-tree-k7`'s first bound-vacuity predictor said it would.** *The task root
present, unwalkable and holding every entry* is not a state; it is a stretch of
trace with a publication before it and an attempted commit after it. Nine states
is what holds `TxnOpen`, `Preflight`, and all six body steps with a stutter to
close the lasso, and the predictor was applied before the command was written
rather than after a mutation survived.

### Cost

**THE REVALIDATION SLICE COST 9–15%, AND THE REFINED COST LAW WAS AGAIN
PESSIMISTIC — THIS TIME BY ABOUT FOUR.** Four new reachable transitions (two
Grove's — `Revalidate` and `QuarReturn` — and two the world's — `CommitMoves`
and `RootNameTaken`), one new phase, branch expansion on `doSettle` and
`doQuarRename`, and **one enabling point removed** (`doClassify` at
`Classified`). Medians of three, one host, one sitting, both files present, and
a clean A/B: no bound moved on any of the four sentinels.

| command | handoff slice | revalidation slice | |
|---|---|---|---|
| `FN_08` (4 steps, entry surface) | 1.75 s | 1.90 s | +9% |
| `FN_07` (4 steps, entry surface) | 1.90 s | 2.11 s | +11% |
| `FN_13` (10 steps, the widest inherited) | 6.07 s | **6.93 s** | **+14%** |
| `witness_FN_11` (10 steps) | 2.93 s | 3.38 s | +15% |

`quarantine-k43`'s law says to budget by **(phase, guard) points × the bound they
are reachable at**. This slice adds six such points (`Revalidate` 1,
`QuarReturn` 1, `CommitMoves` 3, `RootNameTaken` 1) and removes one, for a net
five, against `commit-k41`'s twelve at +128% on `FN_13`. Pro rata that predicts
about **+53%**. Measured: **+14%**.

**THE VARIABLE THAT KEEPS BEING OVER-COUNTED IS DWELL, NOT COUNT.** All five of
this slice's net points sit at phases a trace passes through **once** —
`Classified`, `Quarantined`, `Restored` — whereas `commit-k41`'s `Classify` and
`ResultArrives` were enabled at phases a trace can *rest* in, each contributing a
successor at many depths. So the operative form from here, and the third
statement of one law:

> **Budget by the number of STATES OF A TRACE at which a transition is enabled —
> not by transitions, not by depth, and not by (phase, guard) pairs. A phase a
> trace passes through once is one state, however many guards select it.**

Two consecutive slices have now found the arithmetic pessimistic — by six, then
by four — while the *advice* it carries has been right every time. **A sibling
should take the ordering (a static scope switch, then a narrowed antecedent,
then a smaller bound) and should not take the multiplier**, and `disposal` is
the third chance to measure it: `FN-21`'s reaper is a sweep rather than a
transaction, so it is enabled at states no phase machine constrains at all, and
it is the first thing in this scope that the dwell form predicts will be
expensive.

**Where the suite's time actually went.** 101 commands, **5 m 46 s**, against 75
in 3 m 05 s. The four sentinels account for +14%; the rest is twenty-six new
commands at 9–12 states and thirteen inherited commands whose bounds rose. **A
whole-suite total still does not compare across sessions** — the third
measurement rule below — and this pair is quoted only because both halves of it
are recorded with their command counts.

---

**THE HANDOFF SLICE'S QUARANTINE RENAME COST 4–7%, AND THAT IS THE MOST USEFUL
NUMBER THIS FILE HAS PRODUCED — because the cost law predicted far worse.**
One new reachable transition (`QuarRename`), one new `one sig` with one `var
lone` field, one new phase atom, and a forward settle that now disposes what the
rename produced. Seventy-five commands, **3 m 05 s**, against seventy-one in
**2 m 40 s** for `commit-k41`'s file — and the four new commands are most of the
difference.

| command | commit slice | handoff slice | |
|---|---|---|---|
| `FN_08` (4 steps, entry surface) | 1.62 s | 1.69 s | +4% |
| `FN_07` (4 steps, entry surface) | 1.81 s | 1.89 s | +4% |
| `FN_13` (10 steps, the widest inherited) | 5.73 s | **6.01 s** | **+5%** |
| `witness_FN_11` (10 steps) | 2.80 s | 3.01 s | +7% |

Medians of three, one host, one sitting, both files present. This is a clean A/B:
no bound moved on any of the four.

**THE COST LAW NEEDED A SECOND VARIABLE, AND THIS SLICE IS WHERE IT SHOWED.**
`commit-k41` measured four transitions at **+128%** on `FN_13` and concluded that
the marginal cost of a transition is superlinear in the trace length it is
reachable at — and budgeted this slice as the worst case that law describes: a
transition reachable only at the far end of a ten-state trace. Per-transition,
that prediction was about +30% on the widest command. Measured: **+5%**, a factor
of six low.

What separates them is not depth but **how many places the transition is
enabled**. `commit-k41`'s four included a `Classify` re-runnable at three phases
and a world-driven `ResultArrives` enabled at three more; `QuarRename` is enabled
at exactly one phase *and* one disposition (`Classified` with `Committed`), and
nothing else in the file reaches it. So the refinement, and it is the operative
form of the law from here on:

> **Budget by the number of (phase, guard) points a transition is enabled at,
> multiplied by the bound they are reachable at — not by counting transitions,
> and not by depth alone.**

This is the same advice the corpus already gave — *prefer a narrowed antecedent*
— arriving from the other side with a number on it. A deep transition with one
narrow enabling point is nearly free; a shallow one enabled everywhere is not.
**`revalidation` should read this before it writes `FN-22`**: four revalidation
points at two handoffs is the opposite shape, and the ten-row table is enabling
conditions rather than transitions.

---

**The commit slice, measured against `witness-k40`'s file in one sitting.** Four
new reachable transitions — `Recover`, `Classify`, `Settle`, `ResultArrives` —
plus a `CommitAttempt` that mutates where it used to record. Seventy-one
commands, **2 m 40 s**, against forty commands in **1 m 04 s** for the same
inherited file in the same sitting.

| command | witness slice | commit slice | |
|---|---|---|---|
| `FN_08` (4 steps, entry surface) | 1.3 s | 1.7 s | +31% |
| `FN_07` (4 steps, entry surface) | 1.4 s | 1.8 s | +29% |
| `FN_13` (10 steps, the widest inherited) | 2.5 s | **5.7 s** | **+128%** |
| `witness_FN_11` | 2.0 s | 2.8 s | +40% at 9 → 10 states |

Every figure is a median of three, and the last row is **not** a clean A/B: its
bound moved with it, so it measures the state as well as the transitions.

**THE TWO SENTINELS DISAGREE BY A FACTOR OF FOUR, AND THAT IS THE POINT.**
`task-tree-k7` established that one sentinel is not enough and that the tightest
command is nearly blind to new state; this is the sharpest instance yet.
Four reachable transitions cost the *tight* entry-surface commands ~+30% —
less than the witness slice's eight cost them (+55%), roughly in proportion — and
cost the file's *widest* inherited command +128%, which is more than twice what
proportionality predicts. **The marginal cost of a transition is superlinear in
the trace length it is reachable at.** A transition enabled only in the last two
phases of a ten-state trace is cheap for a four-state command and dear for a
ten-state one, and a slice that measured only the entry surface would have
reported +30% and been wrong about its own file by a factor of four.

**The corollary, for the siblings still to come — and it was half right.**
`handoff` adds nineteen obligations over a quarantine, an atomic root rename and
a nested crash-safe protocol, and `exits` adds a full step-boundary crash sweep.
Both are long-trace slices. Budget them by counting **transitions × the bound
they are reachable at**, not by counting transitions, and prefer — as ever — a
static scope switch, then a narrowed antecedent, then a smaller bound. **The
quarantine slice measured this prediction at a factor of six too pessimistic and
says why above**; the advice survives, the arithmetic does not.

---

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

**A THIRD MEASUREMENT RULE, from the witness slice.** `task-tree-k7` established that
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
- **The attempt identity and the repository anchor are no longer pins.** The
  witness slice drew them at `TxnOpen` and never read them back. The commit slice
  is where they become operands: the classification compares `Repo.rev` against
  `Txn.anchor` and looks a ticket up by `Txn.attempt`. **The finish handle joined
  them, and had to** — after `WEvacuate` the task root holds nothing, so
  `finishLive` is empty and a classification that read the tree for its handle
  would have nothing to read. `Txn.handle` is pinned at `TxnOpen` and adopted
  from the manifest by a recovery, which is what makes it a *live session's*
  operand rather than an artifact, and that in turn is what `FN-03` needs it to
  be. It is `set Entry` rather than `lone Entry` so that a tree with two live
  finish leaves makes the **preflight refuse** instead of making `TxnOpen`
  silently unavailable.
- **The repository anchor is lane-blind, and that is a finding rather than a
  shortcut.** The catalogue gives the three lanes three different anchors — a head
  revision; a working-copy change identity with its parents and the exact
  preflight commit; both plus the user's index image — and states the rollback
  licence over the **role** each plays. `Repo.rev` is that role and nothing
  finer. **Exactly one obligation in this slice reads the lane at all**: `FN-17.a`,
  whose *the exact recorded preflight commit is reproduced* is stated only of the
  working-copy-as-commit lanes, and which the file carries as `Repo.reproduced`
  and `Repo.canReproduce`. Twelve obligations, one lane split, and `EN-16`'s
  collapse control — which is what would make a lane-blind model visible — is
  still `exits`'. See *What a green run does not prove*.
- **The commit lands or it does not, and the model does not say why.**
  `doCommitAttempt`'s success branch is a free choice between `commitLands` and
  `commitDoesNotLand`, and the immediate result is a separate `lone Report` left
  wholly free. That is `EN-05` taken seriously: the commit is outside the
  filesystem transaction, so what happened to it is not something the transaction
  can know, and modelling the landing as a branch of the step rather than as a
  consequence of it is what makes `Indeterminate` **reachable** instead of
  argued about.
- **The quarantine is a SECOND PLACE A ROOT CAN BE, not a copy of what the root
  holds.** `Quar.qRid` is the whole signature. The catalogue's rename moves the
  task root *witness and evacuated tree intact*, so the step's only persistent
  effect is the identity's move and everything else is framed — which is also
  what keeps it to one persistent effect for `FN-24.b`. What the model therefore
  cannot say anything about is the quarantine's own **name**: the catalogue
  gives it a per-handle, per-attempt reserved name and this file has no filename
  grammar, so *the quarantine target is occupied* is one condition here where the
  shipped protocol has a family of them. That is `disposal`'s to widen if
  `FN-21.b`'s cleanup manifest needs it.
- **The forward settle now disposes the quarantine, and it is STILL an
  abstraction of disposal.** `commit-k41` released the witness and the manifest
  in place as a stand-in; the handoff slice replaced the first half of that with
  the real rename, and what follows is a settle that clears the quarantine, the
  slot and the manifest in one step. Nothing here claims that disposal is
  re-entrant, marker-guarded, or bounded to Grove's own — `FN-21` and `FN-31`
  are `disposal`'s. `FN-18` requires only that a proven commit is never followed
  by a reconstruction, and it says nothing about how the artifacts go.
- **`doClassify` IS NO LONGER RE-RUNNABLE AT `Classified`, and `FN_22j` is why.**
  `quarantine-k43` deliberately kept it out of `Quarantined` so that two of
  `FN-22`'s rows would not be answered by construction; the revalidation slice
  found that leaving it at `Classified` was the same mistake one phase earlier.
  `Classified` with a disposition is a state where a **handoff is pending**, and
  a classification there re-derives the disposition and takes **no corrective
  action** — a fifth revalidation point the catalogue does not have, at which the
  protocol can observe a change and do nothing about it. `FN_22j`'s
  counterexample is exactly that trace. It is now enabled at `Attempted` and
  `Settled`; the second is `FN-03`'s retry-with-no-artifacts and is unchanged.
- **THE REVALIDATION POINTS ARE STATES, NOT EVENTS, AND `Sys` GAINED NO FIELD.**
  The four points are `(Classified, NotCommitted)`, `(Classified, Committed)`,
  `Restored` and `Quarantined`, read by `atRevPoint`. A `var rp: lone RevPoint`
  naming the current point would have been the obvious encoding and would have
  cost a fifth of the state space at a bound of eleven; the two before-points are
  the same moment in the trace distinguished by which handoff the classification
  pointed at, which is exactly what makes the table's two *divert* rows
  meaningful. `Txn.disp = Indeterminate` at `Classified` is deliberately **not**
  a point — no handoff was ever pending there — and the classification's own
  block is what `witness_FN_16a` still reaches.
- **The ten-row table is written as DATA, and that is what makes a missing row a
  counterexample.** `tableAction` and `tableOutcome` are total functions over
  four points and three dispositions, written apart from every transition;
  `FN_22a` binds every Grove step taken at a point to them. Delete a row and the
  function goes partial, `Sys.act' = tableAction[..]` is false, and the check is
  red. What that construction **cannot** catch is a combination reachable in the
  world that enables no Grove step at all — that is a silence, and the sixteen
  witnesses are what fill it.
- **`observed` and `doClassify` compute the same function and are written
  apart.** Both are built out of `resultProven` and `anchorHolds` and out of
  nothing else, and they are bound to each other *through* those two predicates:
  `FN-15.a` checks that the classification is exactly that function of them, and
  `FN-22`'s rows are stated over `observed`. The separation buys mutation
  isolation — a mutation aimed at the classification leaves `FN-22` standing and
  one aimed at a corrective action leaves `FN-15` standing — which a shared
  definition would have destroyed. What is **not** checked is that the two agree
  directly; see *What a green run does not prove*.
- **THE WORLD CAN NOW MOVE A COMMIT, AND WITHOUT IT HALF THE TABLE IS
  UNREACHABLE.** `doCommitMoves` lets this attempt's own commit land *late* —
  `EN-09`'s *a result may arrive late*, at the grain where the thing arriving is
  the commit rather than its report — or be undone, which is an operator's `jj
  undo` between two of Grove's steps and is `EN-11` at the repository. Before it,
  `resultProven` was **monotone**: tickets grew only by `commitLands`, `tracked`
  shrank only by it, and `doTopologyChange` framed both, so a `Committed`
  observation could never become anything else and the catalogue's two
  `Committed` departures were unreachable **by construction**. It is enabled at
  `Classified`, `Quarantined` and `Restored` only — a cost narrowing, and the
  honest one: a ticket that moved earlier is observed by the classification
  itself, which is `FN-15`'s subject.
- **`doRootNameTaken` is the narrowest guard in the file**, and it exists so that
  `FN-22.h`'s *a return that cannot complete* has a reachable antecedent rather
  than an argued one. `doSwap` cannot serve: it requires `some Root.rid`, because
  it is the world swapping a root that is **there**.
- **THE RETURN'S DIAGNOSTIC IS A STATE, NOT A STRING.** `FN-22.h` asks that a
  failed return *report both the change and the quarantine, both named in the
  diagnostic*. This file has no strings, so what is checked is that both are
  **observable** in the state the attempt ends in: the quarantine still holds the
  root, the witness still stands, and `observed` still differs from the recorded
  disposition. Whether the shipped diagnostic names both is
  `formal-synthesis-k16`'s.
- **The forward settle still passes through `FN-22.i`'s stable state and out of
  it in one step.** The catalogue's stable state after *complete: dispose* is
  *task root `Absent`, quarantine holding the root* — the state from which
  `FN-21`'s disposal proceeds. This file's settle disposes in the same step that
  revalidates, because disposal is still an abstraction here and `FN-21` is
  `disposal`'s. `FN_22i` therefore checks the action, the outcome, that the
  quarantine was holding a root when it ran, and that the task root is left
  exactly as the protocol left it.
- **The occupied quarantine target BLOCKS rather than refuses, and no diagnosis
  is named.** At the rename the transaction has a proven commit, so ending it as
  a refusal would report that the finish did not happen while the ticket in
  history says it did. It is reported as `Blocked` with the model-only
  `Sys.why = W14QuarantineOccupied`; the closed partition over `RecoveryPending`
  and `OwnershipConflict` is `FN-25`'s and `exits`', and naming
  `OwnershipConflict` here would answer `FN-25.a`'s totality by construction —
  the same reason `commit-k41` left `BlockedOutcome` bare.
- ~~**`FN-17.a`'s *before the witness is removed* is a conjunction, not an
  ordering.**~~ **AN ABSTRACTION REMOVED RATHER THAN RESTATED, AND THE CLAIM
  FORCED IT.** `commit-k41` restored the tree, reproduced the exact preflight
  commit and released the witness in one step, and could only state *before the
  witness is removed* as a condition. `FN-22`'s *after restoration* row cannot be
  stated without a state after the restoration — a one-step settle can only
  observe what it observed before its own effect, and the restore branch frames
  the whole repository, so that observation is the **same** one and the row would
  be unreachable by construction. So the step is split: `doSettle` restores and
  stops at `Restored` with the witness standing empty over the restored tree, and
  `doRevalidate` decides what becomes of the witness. `FN_17a`'s second conjunct
  now reads the **unprimed** state for both halves, which is what makes it an
  ordering. Whether the *remaining* steps must be decomposed is still
  `FN-24.b`'s, and `Settle`, `Classify`, `Recover`, `Revalidate` and `QuarReturn`
  are all in `bodySteps` so that `exits` can ask it of them.
- **SPLITTING THE RESTORATION OPENED A LANE-CHANGE WINDOW, AND `FN_17a` FOUND
  IT.** `World.lane` is `var` because `SY-03` requires it, so the layout can move
  between the restoration and the release: a tree restored on a Git lane, where
  `FN-17.a` asks for no reproduction, can be released on a jj lane, where it
  does — and the reproduction was never performed. The answer is `SY-03`'s own
  rule applied to the new gate (`reproductionStands`): the release revalidates
  against **its** operands and blocks if it cannot show the reproduction. This is
  a consequence of the model's decomposition rather than a defect in the shipped
  protocol, whose settle is one step — but it is exactly the hazard the
  implementation inherits if `FN-24.b` ever forces the same split, and
  `formal-synthesis-k16` should read it that way.
- **`Blocked` carries no diagnosis here, and the omission is deliberate.** The
  catalogue's closed partition over `RecoveryPending` and `OwnershipConflict` is
  `FN-25`'s, which is `exits`'. A commit slice that named the two would have
  answered `FN-25.a`'s totality and disjointness **by construction**, which is the
  shape of a false-confidence incident rather than a finding. What this slice
  needs is an outcome atom for `Indeterminate` and for `FN-17.b`, and that is all
  it takes.
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

**THE COMMIT SLICE PAYS THE SAME PRICE ONCE AND THEN STOPS, AND SAYS SO HERE
RATHER THAN LEAVING IT TO BE NOTICED.** Reaching a *settled* disposition from a
fresh grove is ten transitions and a retry that has lost its artifacts is twelve,
against a file whose widest command was already ten states. So fifteen of this
slice's eighteen witnesses start from `interruptedMidEvacuation` — **the disk an
interruption mid-evacuation leaves**: a published witness this attempt owns, a
ready manifest inside it, part of the tree already moved, the task root still
present and still holding the live finish leaf, and `Txn.phase = Fresh`. They
then run `TxnOpen`, `Preflight` and `Recover` for real and go forward.

That is the witness slice's rule applied rather than weakened: the predicate
constrains **tree state only**, at the one phase `doCrash` produces, which is
exactly what a later launch finds. What it does **not** demonstrate is the
six-step body followed by a commit in one trace, and one command exists to close
that: `witness_FN_15b_git_committed_reached_from_a_fresh_grove` runs `TxnOpen`,
the preflight, all six body steps and the classification, at ten states, and it
is the file's widest command. The other fourteen are demonstrably shortcuts of
something rather than of nothing.

**THAT DEBT IS PAID, AND THE ANSWER IS YES.**
`witness_FN_22a_the_posited_recovery_disk_is_reachable` runs `TxnOpen`, the
preflight, all six body steps with a partial evacuation, a `crash`, and the
confirmation a later launch supplies — nine transitions — and then asserts
`interruptedMidEvacuation` itself. **It first lands at eleven states and finds
nothing at ten**, which is precisely why no earlier slice could have run it: ten
was the ceiling from `witness-k40` onward, and the ceiling was the reason the
debt existed. Every witness in this file that rests on the predicate is therefore
testifying about a disk an execution reaches, and the fifteen inherited ones did
not have to be re-examined.

It is filed under `FN-22.a` because that is the obligation whose subject is *the
four points are performed*, and a point performed over a disk no execution
reaches is not performed at all — the catalogue named this table as the check for
it. The filing is recorded here rather than left to be inferred.

The price of the witness slice's own choice is that every body witness runs
`TxnOpen` and `Preflight` in front of its own steps, which is two states each. The gain is that they demonstrate the
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
| **a finish that was rolled back** | *no member fits* — **see below** |

**Two members share one reason, and that is not a modelling shortcut.** `SY-03`
says a preflight is never a licence and every gate revalidates against its own
operands, which makes *layout unsupported* and *quarantine target unreachable*
the same question asked at two gates. What follows is that a reason cannot say
which member refused — hence `Sys.why` — and that the two are distinguishable to
an operator only by which gate reported. Whether the shipped diagnostic should
distinguish them is `formal-synthesis-k16`'s, not this file's.

**A ROLLED-BACK FINISH HAS NO REFUSAL REASON EITHER, AND THIS IS THE SECOND
INSTANCE OF ONE SHAPE.** The catalogue maps the `NotCommitted` disposition to
*rolls back and yields `Refused`*, and none of the seventeen closed reasons names
it: `NoTrackedDeletion` and `RootIdentityChanged` are each **false** of a
transaction whose fingerprint was fine and whose root never moved. Reporting it
under one of them would be a lie the model could not be caught in, so
`finish.als` adds **one** refusal atom of its own — `RefRollbackNotCommitted`,
named for what it is and reported alongside the model-only
`Sys.why = W11NotCommitted`. It is the only atom this file adds to the
catalogue's set, and it is recorded here rather than smuggled in.

Two of the three post-flight outcomes this slice reaches therefore have no name
in the shipped contract, and `formal-synthesis-k16` has the same two exits for
both: widen the closed reason set, or restate the outcome. **That is now three
findings of one kind in this scope** — seven preconditions against six reasons
(entry 031), a tracked witness with no reason (entry 032), and a rolled-back
finish with no reason (this one). The pattern is not three accidents: **the
catalogue fixes closed sets and never states the map between them**, and every
place a model has to write down the outcome of a branch is a place that map is
missing.

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
  transaction — and THE STEP THAT REMOVES THE ROOT DID NOT CLOSE THE
  DIVERGENCE.** The claim requires the task root still to be present at the
  commit attempt (the ADR's *`.grove/` stays visibly present and unwalkable*);
  `gateEvacuated` does not check it. The two have been written apart since
  `witness-k40` against the day a step removed the root, and the handoff slice is
  that day: `FN_11` was expected to have something to say and **it stayed green,
  at ten states, with the quarantine rename in the file.**

  The reason is worth more than the green. The rename is reachable only from
  `Classified`, which is two transitions **past** the commit attempt, and
  `doCommitAttempt` is enabled only at `PublishedP` and `Evacuated`. So the
  root's removal is strictly downstream of `FN-11`'s antecedent and no trace can
  put the two in the wrong order. The one way back — a crash after the rename,
  then a later launch — is closed by `doTxnOpen`'s own `some Root.rid`.

  **THE REVALIDATION SLICE WAS THE PREDICTED DAY AND THE DIVERGENCE STILL DID
  NOT GO LIVE — AND THE REASON IS BETTER THAN THE GREEN.** `quarantine-k43`
  named `FN-22`'s revalidation *after the quarantine rename* as the first thing
  that would re-enter a transaction over a rootless disk. It does: `doQuarReturn`
  and the forward settle both run at `Quarantined`, where `Root.rid` is empty.
  Neither of them attempts a commit, so `FN-11`'s antecedent
  (`Sys.act' = CommitAttempt and Sys.res' = Applied`) is still never met on such
  a disk — `doCommitAttempt` is enabled at `PublishedP` and `Evacuated` and at no
  phase downstream of the rename, and the return lands at `Classified`, which is
  also downstream. **`FN_11` stayed green at ten states with the return, the
  revalidation and `doRootNameTaken` in the file.**

  Neither side has been edited across three slices now, and the honest reading
  has hardened rather than changed: the divergence is unreachable because
  `doCommitAttempt`'s own enabling surface is confined to the two phases before
  the root can move, not because `gateEvacuated` checks anything. **The thing
  that would fire it is a protocol that re-attempts a commit after a handoff** —
  `FN-23`'s idempotent recovery is the nearest candidate and it is `exits`'. The
  file's answer remains *the protocol's ordering makes it unreachable*, and it
  is now checked over four more transitions than it was.
- **Not the strongest form of `FN-20`.** *No classification READS the
  quarantine* is a non-interference property: two traces differing only in what
  the transaction left behind reach the same disposition. Alloy quantifies over
  traces one at a time, so it cannot be stated as a check at all, and the file's
  two conjuncts — presence is never sufficient, presence is never necessary and
  never obstructs — are the reachable approximation. A candidate protocol that
  read a leftover only in a way that changed **nothing** about the outcome would
  satisfy both conjuncts and violate the claim's plain reading.
- **Not that a quarantine this file leaves behind can ever be cleaned up.**
  `witness_FN_19` is the catalogue's own witness — an interruption immediately
  after the rename — and what it leaves is a complete quarantine over an absent
  task root. `doTxnOpen` requires `some Root.rid`, so **no transaction in this
  file can be opened on that disk**, and the forward settle that would dispose of
  the quarantine is unreachable from it. That is not a defect: the catalogue's
  answer is the **reaper** (`FN-21`), which is a sweep rather than a transaction
  over the task root, and it is the `disposal` sibling's. Until it lands, the
  file demonstrates a state it cannot leave.
- **Almost nothing about the lane, still.** Twelve more obligations have landed
  and **exactly one of them reads the lane**: `FN-17.a`'s *on a
  working-copy-as-commit lane the exact recorded preflight commit is reproduced*.
  Everything else — the anchor, the licence, the three dispositions, the scoping
  of the commit — is stated over a role the three lanes share, which is what the
  catalogue asks for and is *also* exactly what a lane-blind model would look
  like. `FN-15.b`, `FN-15.c` and `FN-15.d` each have a witness per lane, so the
  three are demonstrably *reachable* under all three; that is weaker than the
  claims *differing* under them. **`EN-16`'s collapse control is what separates
  the two and it is `exits`'**, and until it runs, "the lane is a model parameter"
  is a property of the signature rather than a measured fact about the commands.
- ~~**Not that `interruptedMidEvacuation` is reachable.**~~ **ANSWERED — see
  *Where a trace starts*.** It is reachable, first landing at eleven states, and
  the command that says so runs the body up to it rather than positing it. This
  is the only entry ever removed from this list, and it is worth one line on how:
  the check that discharged it was a **witness**, not a property, and the thing
  that had prevented it for three slices was a bound rather than an argument.
- **Not that `observed` and `doClassify` agree.** The revalidation's observation
  and the classification are written apart on purpose, for the mutation isolation
  described under *Abstractions*, and no command states a biconditional between
  them. They are bound *through* `resultProven` and `anchorHolds` — `FN-15.a`
  pins the classification to exactly that function of the two, and `FN-22`'s rows
  are stated over `observed`, which is built from the same two — so a divergence
  would require one of them to stop reading the evidence predicates, which
  `FN-15.a` would catch on the classification's side and `FN_22a` would catch on
  the table's. That is an argument, not a check, and it is recorded as one.
- **Not that the four points are the right four.** `atRevPoint` says where they
  are and the catalogue says there are exactly four because there are exactly two
  filesystem handoffs. A protocol with a third handoff would need a fifth and
  sixth point and nothing here would notice; `FN_22a`'s *none is skipped*
  conjuncts quantify over the two handoffs this file has (`Root.holds` growing,
  `Quar.qRid` moving) and over the two completions, and a handoff by some third
  mechanism would be outside all four antecedents. This is the same limit
  `FN-05.a` has against a mutation that removes a precondition from both sides.
- **Not that a `Blocked` produced by the table is `RecoveryPending`.** The
  catalogue diagnoses all four of the table's `Blocked` rows `RecoveryPending`,
  and this file names none of them as an outcome — `W15CommittedAfterRestore`,
  `W16ReturnIncomplete` and `W12Indeterminate` are model-only `why` values. The
  closed partition over `RecoveryPending` and `OwnershipConflict` is `FN-25`'s
  and is `exits`', and naming it here would answer `FN-25.a`'s totality and
  exhaustiveness by construction — the same reason `commit-k41` left
  `BlockedOutcome` bare and `quarantine-k43` used a `why` for the occupied
  target. **`exits` inherits three `why` values that its partition has to
  absorb**, not one.
- **Not that `Indeterminate` is the *only* irreducible gap.** `FN-15.d` is
  answered by a witness on each lane rather than by the bounded-unreachability
  branch, so this file's evidence for Q2 is *`Indeterminate` is reachable under
  the incumbent protocol at these bounds*. Whether it becomes unreachable under
  `EN-05`'s counterfactual — commit and evacuation as one step — is **Quint's**
  side of the assumption table and is not evidence this file can produce.
- **Not that the classification's evidence is complete.** `resultProven` is the
  ticket **and** the expected deletions being gone from the tracked set — but the
  second conjunct goes **vacuously true once the manifest is released**, because
  there is then no recorded fingerprint to compare against. That is `FN-03`'s own
  content (*the ticket ... SHALL survive the destruction of every artifact the
  transaction owns*) rather than a hole, and it is written down because a reader
  meeting the conjunction would take it for a stronger test than it is at exactly
  the moment it matters most.
- **Not that the commit is scoped at the level a lane cares about.** `FN-14` is
  checked as *the fingerprint leaves the tracked set, nothing else does, and the
  unrelated working-copy work is untouched*. The lanes table's mechanisms — a
  pathspec, a jj fileset, the index image, and native jj's **partial-commit
  hazard**, where the deletion stays in the change and the unselected witness
  moves into a *successor* — are none of them modelled. The hazard in particular
  would need a notion of successor revisions this file does not have, and it is
  the sharpest thing `formal-synthesis-k16` should not read this green run as
  covering.
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

Rows 1–9 are `entry-k39`'s and rows 10–17 the witness slice's; both sets are
unchanged. Rows 18–29 are the commit slice's, and **two of the twelve did not
land as first written** — after eight that did. Both failures are new instances
of rules this file already carried, which is recorded below rather than treated
as bad luck. Rows 30–31 are the handoff slice's, and **one of the two did not
land as first written either**, for a reason that is neither of the commit
slice's two — see *A mutation that kills a neighbour* below.

Rows 32–41 are the revalidation slice's, and **three of the ten did not land as
first written — one in each of the three ways this file had already recorded,
which is the first time all three have appeared in one slice.** Every one of the
ten was also run against the other nine `FN-22` checks and against `FN-03`,
`FN-16.a`, `FN-16.b`, `FN-17.a`, `FN-18` and `FN-19`; the *left green* column is
that sweep, not an assertion.

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
| 18 | `FN-03` | `resultProven` also requires `some Man.mReady` — the classification reads an artifact the transaction owns, not the ticket | `witness_FN_15b_nativejj_committed_reached` | KILLED |
| 19 | `FN-04` | `resultProven` reads `some ticketedAttempts` — any ticket on the handle settles, whoever wrote it | `witness_FN_15b_nativejj_committed_reached` | KILLED |
| 20 | `FN-14` | the commit step stops framing the unrelated working-copy work, **in both the places it is framed** | `witness_FN_14_unrelated_modified_work_present_across_a_successful_finish` | KILLED |
| 21 | `FN-15.a` | `doClassify`'s `Committed` arm gains `Txn.report != FailReport` — an exit status read as a receipt | `witness_FN_15a_a_failure_reported_after_the_classification_over_an_exact_commit` | KILLED |
| 22 | `FN-15.b` | `doClassify` reaches `Committed` on `resultProven or some Repo.tickets` | `witness_FN_15c_git_notcommitted_reached` | KILLED |
| 23 | `FN-15.c` | `doClassify` stops comparing the recorded anchor — everything unproven is `NotCommitted` | `witness_FN_15c_git_notcommitted_reached` | KILLED |
| 24 | `FN-15.d` | `doClassify` reaches `Indeterminate` with the anchor intact | `witness_FN_15d_git_indeterminate_reached` | KILLED |
| 25 | `FN-16.a` | `doSettle` restores on an `Indeterminate` too | `witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit` | KILLED |
| 26 | `FN-16.b` | `rollbackLicensed` drops `not resultProven`, and `doSettle` branches on the licence rather than on the disposition | `witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit` | KILLED |
| 27 | `FN-17.a` | `doSettle`'s restore branch drops `preflightCommitReproduced` | `witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit` | KILLED |
| 28 | `FN-17.b` | `doSettle`'s restore branch proceeds whether or not it can reproduce the commit | `witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit` | KILLED |
| 29 | `FN-18` | the forward settle unpacks the witness back into the task root | `witness_FN_15b_nativejj_committed_reached` | KILLED |
| 30 | `FN-19` | `doQuarRename` drops `no Root.rid'` — the root is **copied** into the quarantine rather than renamed into it, so both places hold it | `witness_FN_18_a_proven_commit_reached_after_an_interruption_mid_evacuation` | KILLED |
| 31 | `FN-20` | `doClassify`'s `Committed` arm gains `no Slot.occ` — the transaction's own witness, still sitting there, withholds a finish the ticket proves | `witness_FN_15c_git_notcommitted_reached` | KILLED |
| 32 | `FN-22.a` | the forward settle at `Quarantined` drops `observed = Committed` — it disposes on the disposition the classification wrote, so the fourth point is **skipped** | `witness_FN_22i` | KILLED |
| 33 | `FN-22.b` | the before-restoration divert writes `Txn.disp' = NotCommitted` — it restores nothing and also goes nowhere, so it never takes the forward path | `witness_FN_22d` | KILLED |
| 34 | `FN-22.c` | the `Committed`-after-restoration arm re-evacuates the restored tree back into the witness instead of leaving it standing | `witness_FN_22a_the_point_after_the_restoration_is_reached` | KILLED |
| 35 | `FN-22.d` | the completed refusal restores `Root.holds - Man.mHandle` — everything the manifest recorded except the finish leaf | `witness_FN_22a_the_point_after_the_restoration_is_reached` | KILLED |
| 36 | `FN-22.e` | the before-rename divert writes `Txn.disp' = Indeterminate` — it renames nothing, and sends the attempt to a block rather than to the restoration path | `witness_FN_22i` | KILLED |
| 37 | `FN-22.f` | the successful return puts *some* root back rather than the one that left (`some Root.rid'` for `Root.rid' = Quar.qRid`) | `witness_FN_22g` | KILLED |
| 38 | `FN-22.g` | the block at `Classified` with `Indeterminate` reports `RefRollbackNotCommitted` — a block reported as a refusal, which is the collapse the catalogue names | `witness_FN_22f` | KILLED |
| 39 | `FN-22.h` | the incomplete return clears the quarantine while blocking — it reports the change and not the quarantine | `witness_FN_22g` | KILLED |
| 40 | `FN-22.i` | the forward settle frames the quarantine instead of disposing it — the artifacts go and the quarantine stays | `witness_FN_18` | KILLED |
| 41 | `FN-22.j` | the `Indeterminate` block at a point stops framing `Repo.rev` — it performs no handoff and moves the repository | `witness_FN_22d` | KILLED |

**THE REVALIDATION SLICE DECIDES NO `Q4` ROW, AND THE REASON IS THE SAME ONE
`quarantine-k43` GAVE.** All ten of `FN-22`'s obligations are *incumbent
mechanics* — the class register says so — so a mutation that breaks one is
evidence about the incumbent protocol and about nothing else. The removal matrix
still stands at five decided rows, all inherited, and the shared-safety claims
that could add to it (`FN-24`, `FN-27`) are still `exits`'.

**Rows 14, 17, 19, 20 and 25 are the removal-matrix rows `exits` inherits.**
Removing the `gateEvacuated` half of the commit attempt breaks `FN-11` first and
the `gateWitnessUntracked` half breaks `FN-13` first; both of those are
*incumbent mechanics* claims and neither is yet an answer to Q4. The commit
slice's three are the first that are not:

| removable artifact | first **shared-safety** obligation its removal breaks |
|---|---|
| the correlation ticket (as an *attempt-naming* record) | `FN-04` — row 19 |
| the deletion fingerprint (as the commit's scope) | `FN-14` — row 20 |
| the recorded anchor (as the rollback licence) | `FN-16.a` — row 25 |

Each row is stated over the artifact's **role**, which is what the catalogue's
class register requires: a candidate protocol satisfies the claim by supplying
the role, not by keeping the artifact. `exits` transcribes these three rather
than re-deriving them.

**THE QUARANTINE'S OWN ROW CANNOT BE DECIDED FROM THIS SLICE, AND THE REASON IS
EVIDENCE FOR Q1 RATHER THAN AN ABSENCE.** Rows 30 and 31 are the quarantine's
two mutations and neither decides it: row 30 breaks `FN-19`, which is *incumbent
mechanics* and is exactly what Q1 asks about, so it is not an answer. Row 31
breaks `FN-20`, which **is** shared safety — and `FN-20` survives the
quarantine's removal *by construction*, because the catalogue states it over the
role (*no artifact a transaction leaves behind is a receipt for it*) and a
protocol that leaves nothing behind satisfies it vacuously. So the one
shared-safety claim this slice reaches says nothing against disposal-in-place.
The shared-safety obligations that could name the quarantine — `FN-24` and
`FN-27` — are not in the file yet, and they are `exits`'. **The row is `exits`'
to write and this note is what it should read first**: the quarantine's
first-broken shared-safety obligation is not `FN-20`.

**A MUTATION THAT KILLS A NEIGHBOUR HAS NOT ISOLATED ITS TARGET — row 31, and a
third way for a mutation to fail its aim.** The obvious mutation for `FN-20` is
the one the claim is named for: `doClassify`'s `Committed` arm gains
`no Quar.qRid`, so a leftover **quarantine** withholds the finding. It was run,
it was satisfiable, and it killed `FN-20` — **and it killed `FN-03` with it**,
because `FN-03`'s third conjunct (*a classification with no witness and no
manifest left still settles forward on the ticket alone*) says nothing about a
quarantine, so a quarantine that withholds falsifies it too.

A mutation that takes down its target and a neighbour is evidence that the two
overlap, not that the target is checked. It reports as a kill either way, which
is what makes it the same family as the two failures below. The row as run
mutates on `no Slot.occ` instead — the antecedent `FN-03` **already carries** and
`FN-20`'s deliberately drops — so it kills `FN-20` alone and leaves `FN-03`,
`FN-04` and `FN-15.b` green. **The general form: check what a mutation leaves
standing, not only what it takes down.** `commit-k41` learned to check that a
mutation *can fire* and that it *aims at the right thing*; this adds that a
mutation must be *specific*, and all three report identically when they are not.

**TWO OF THE TWELVE DID NOT LAND AS FIRST WRITTEN, and both are old rules in new
clothes.** Neither is a defect in the model, and recording them is the point —
each reported *exactly* as a surviving mutation does.

- **A frame stated in two places must be removed from both.** Row 20 first
  dropped `World.wcWork' = World.wcWork` from `commitLands` alone. `FN-14` stayed
  green, because `doCommitAttempt`'s common part frames the world too and
  `worldSame` carries the same conjunct. The mutation was a **semantic no-op** —
  the same class as the witness slice's mutation 2, reached from the opposite
  direction: there a conjunct that changed nothing was *added*, here a conjunct
  that constrained nothing was *removed*. The general form is one sentence: **the
  frame you must remove is every frame, and a redundantly-stated one hides the
  other copy.** The row as run removes both.
- **A mutation must be able to reach the state its check is about.** Row 26 first
  reordered `doClassify` so the anchor was consulted before the result. `FN-16.b`
  stayed green — because `doSettle`'s rollback branch reads `rollbackLicensed`,
  which is written apart from the classification and still carried
  `not resultProven`. The mutation changed which disposition was written and
  changed nothing about whether a restoration could happen with a result present.
  **Writing the claim apart from the transition protected the claim from a
  mutation aimed at the transition**, which is the house style working exactly as
  intended and is also why the *aim* has to be checked. The row as run mutates the
  licence itself and lets the settle branch on it.

**A FOURTH RULE ABOUT MUTATION AIM, AND IT IS A CONSEQUENCE OF WRITING THE TABLE
DOWN.** `FN-22.a` binds the **action and the outcome** of every Grove step taken
at every revalidation point. That makes it strictly stronger than the
action-and-outcome half of every other row in the table — so **any mutation that
changes which arm runs, or what it returns, kills `FN-22.a` as well.** Row 36 was
first written as *remove the before-rename divert arm so the rename happens
anyway*; it killed `FN-22.e` and killed `FN-22.a` with it, because the
occupied-target sub-branch then reported `Blocked` where `tableOutcome` says
`Applied`. The general form, and it is the one a sibling with a table of its own
should read first:

> **When one obligation states a table's totality, an isolating mutation for the
> table's other rows must aim at the STABLE-STATE column — the one the totality
> claim does not carry.**

All nine of rows 33–41 do, and that is why each of them leaves `FN-22.a` green.

**AND THE OTHER TWO FAILURES WERE THE OLD RULES, VERBATIM.**

- **Row 34 was first written as the `Committed` arm releasing the witness while
  blocking** — which reads as the sharpest possible falsification of *leaves the
  witness blocking*. It **SURVIVED**, because `fact EmptySlotHoldsNothing` makes
  an empty slot beside a written manifest impossible, so the branch was
  unsatisfiable and reported exactly as a survivor. That is `entry-k39`'s trap:
  **a mutation the model cannot execute is not a control**, met for the first
  time against a *fact* rather than against a frame condition. The row as run
  re-evacuates instead, which the facts permit.
- **Row 37 was first written as the return losing what the root held**
  (`no Root.holds'` for `rootSameHolds`). It **SURVIVED**, and it is the witness
  slice's *semantic no-op* met from a new direction: at `Quarantined` the root
  holds nothing — the evacuation emptied it and the rename framed it — so
  `no Root.holds'` and `Root.holds' = Root.holds` are the **same constraint**.
  A mutation whose two sides coincide in every reachable state changes nothing.
  The row as run mutates the identity the return puts back.

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

**Ten: four from the witness slice, one from the handoff slice and FIVE from the
revalidation slice — and all ten are about the model rather than about the
protocol** — which is itself the
observation, because a slice that adds eight transitions and finds no protocol
defect has still learned something about what its instrument was licensing.

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

**THE HANDOFF SLICE'S QUARANTINE ADDED ONE, and it is the witness slice's first
counterexample met from the other side.**

5. **`FN-19`'s *no partial or empty task root is ever observable*, written over
   every step, fails on a `Swap`.** The invariant *one root identity is never in
   two places at once* is preserved by the rename by construction — the identity
   arrives in `Quar.qRid'` in the same state it leaves `Root.rid'`. It is not
   preserved by `doSwap`, which is **the world** swapping the task root and is
   constrained only by `Root.rid' != Root.rid`: the solver picks the
   quarantine's own identity, and the model has no way to know that moving the
   quarantine directory back over `.grove/` took the quarantine with it.

   The witness slice's first retained counterexample is the same lesson under a
   free **initial state**: *a shape claim under `EN-11` must be restated over the
   transition relation*. This is `EN-11` as a **transition** — `doSwap` and
   `doTopologyChange` are the hand edit made first-class — and the restatement it
   forces is one word: the invariant is preserved by `Sys.act' in txnActs` and
   is not a property of the disk. **A claim about what a protocol never does is
   never a claim about what the world never does**, and this file now carries
   that at both grains. It is also, incidentally, the cost model's *narrowed
   antecedent* arriving as a correctness requirement rather than as a saving.

**THE REVALIDATION SLICE ADDED FIVE, WHICH IS AS MANY AS THE FOUR EARLIER SLICES
TOGETHER, AND THAT IS WHAT A TABLE COSTS.** Nine of the ten `FN-22` checks were
green as first written; the fifth counterexample below is `FN-17.a`'s and the
other four are the table meeting the file's existing licences. Two of the five
changed a **transition** rather than a check, which no earlier counterexample in
this file had done.

6. **`FN-22.j` fails on a `Classify` at a pending handoff — and this one changed
   the protocol.** `Classified` with a disposition is a revalidation point;
   `doClassify` was re-runnable there, so a trace could observe `Indeterminate`
   with the rename pending, re-derive the disposition, report `Applied`, and take
   no corrective action at all. The fix is not to the check: the classification
   is now enabled at `Attempted` and `Settled` only. **A step that re-derives a
   decision without acting on it is a revalidation point the catalogue did not
   authorise**, and it is invisible until something states how many points there
   are. `quarantine-k43` had reached the same conclusion one phase later by
   refusing to open the step to `Quarantined`; this is the general form.

7. **`FN-17.a`'s second conjunct fails on a lane change between the restoration
   and the release — and this one changed the protocol too.** Recorded in full
   under *Abstractions*: the split opened a window `SY-03` says every gate must
   close against its own operands, and `reproductionStands` is that gate. It is a
   consequence of the model's decomposition rather than a defect in the shipped
   one-step settle, and it is the hazard the implementation inherits if
   `FN-24.b` forces the same split.

8. **`FN-22.d`'s *and the finish leaf live* fails on a hand-edited manifest.**
   Written as `one finishLiveNext`, it has a counterexample: under `EN-11` as a
   free initial state, state 0 may record a manifest naming no live finish leaf —
   or two — and a restoration that puts back exactly what such a manifest
   recorded leaves no leaf live. No protocol step produced that manifest. The
   conjunct came **out** of the check and the live leaf is demonstrated in
   `witness_FN_22d` instead, over a disk this slice checks is reachable. This is
   the witness slice's first retained counterexample at a fourth grain — *a shape
   claim under a free initial state is a claim about what the protocol does* —
   and the protocol step it belongs to is `doWManifest`'s, which `FN-12.a`
   already checks.

9. **`FN-17.b` fails on a divert.** Its antecedent was *a settle over a
   `NotCommitted` disposition*, which was the same set of steps while a recorded
   disposition was all a settle could act on. `FN-22`'s before-restoration row
   **diverts** a settle whose recorded disposition is `NotCommitted` and whose
   fresh observation is `Committed`, and a divert restores nothing, so whether
   the exact preflight commit could be reproduced is not a question it asks. The
   antecedent now reads the observation. **Adding a revalidation narrows every
   claim whose antecedent was a recorded disposition**, and `FN-16` is the other
   one — it survives unchanged only because `rollbackLicensed` and
   `observed = NotCommitted` are the same condition.

10. **`FN-22.i`'s *task root `Absent`* fails on `doRootNameTaken`.** The world
    may put something at the task-root name while the quarantine holds the root —
    that is `FN-22.h`'s own antecedent — and the forward settle then disposes
    over a root that is not absent. The catalogue's *`Absent`* is a statement
    about what the rename left, not a promise about what the world does next, so
    the check reads `rootSame`. `doSwap`'s lesson at a third grain: **a claim
    about what a protocol leaves is not a claim about what stays there.**

**THE COMMIT SLICE ADDED NONE, and that is a fact about the slice rather than a
gap in it.** Twelve obligations, thirty-one commands, every check green as first
written and every one of the twelve mutations killed. What it did produce is two
*mutation* failures, recorded under the matrix above, and three findings that no
check could have reported: the missing refusal reason for a rolled-back finish,
the anchor's lane-blindness, and `Indeterminate` being reachable rather than
positively excluded.

**No command in any of the five slices has found a counterexample that was a
defect in the catalogue or in the shipped protocol.** Ten retained
counterexamples, all ten about the model's own licence rather than about the
finish process. The three catalogue-level findings this file carries — the
seven-preconditions/six-reasons mismatch (entry 031), `FN-13`'s missing refusal
reason (entry 032) and the rolled-back finish's missing reason (entry 033) —
were each found by trying to write down **what a branch returns**, never by a
check going red. That is now three times in one scope, and it is the strongest
methodological signal this family has produced: the instrument's value here has
been the discipline of totality, not the solver.

## A fourth finding, and it is of a new kind

**A CHECK WRITTEN STRONGER THAN ITS CLAIM MADE TWO CATALOGUE ROWS UNREACHABLE,
AND NOTHING WOULD HAVE REPORTED IT.** `FN_03`'s first conjunct read
`always Repo.tickets in Repo.tickets'` — history never shrinks, under **any**
step, the world's included. The claim it answers is narrower: the ticket *SHALL
survive the destruction of every artifact the transaction owns*. The comment
beside it has read *under Grove's own steps* since `commit-k41`; the check said
more than the comment, and more than the catalogue.

The cost was invisible for two slices and is not small. With history append-only
under every step, `resultProven` is **monotone**, so a `Committed` observation
can never become anything else — and the catalogue's revalidation table has two
rows that are exactly that transition, which it goes out of its way to say must
not be collapsed (*collapsing them would let a block be reported as a refusal,
which is exactly the distinction `FN-29` requires the operator to be able to
make*). A revalidation slice that had not noticed would have written `FN-22.f`
and `FN-22.g`, found their witnesses would not land, widened the bound, given
up, and reported them **green by construction**.

The narrowing — `(Sys.act' in txnActs) implies Repo.tickets in Repo.tickets'` —
is the same one `FN-19`'s third conjunct took after the `doSwap` counterexample,
and the same rule the witness slice's first counterexample states about a free
initial state: **a claim about what a protocol never does is never a claim about
what the world never does.** This file now carries it at three grains — a free
initial state, a world transition over the **tree**, and a world transition over
**history**.

What is new is the failure mode rather than the rule. The three findings above
are all *the catalogue fixes closed sets and never states the map between them*.
This one is: **an over-stated check does not fail; it removes states, and the
claim that needed those states is answered by construction two slices later.**
An over-statement is invisible to every command in the file that contains it,
and visible only to a claim that needs the states it deleted — which is an
argument for reading a sibling's checks against the catalogue's own wording
before writing over them, and it is the reason this slice read `FN-03`'s comment
and its check against each other at all.
