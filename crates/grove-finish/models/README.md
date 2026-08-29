# grove-finish — models

> **Stranded.** The catalogue these models were written from
> (`docs/specs/semantic-contract.md`) and the runner that checked them
> (`models/run.sh`) were deleted at `delete-formal-models-k29`, so nothing here
> can be run or re-derived. This directory itself goes at
> `delete-finish-models-k30`. Paths named below are descriptions of where things
> were, not links.

The finish/recovery scope of the semantic contract, once
`docs/specs/semantic-contract.md` and now deleted with the rest of the
formal-methods apparatus (`delete-formal-models-k29`): the `FN-` claims, checked
independently by each model family. This directory exists before the crate does,
which is deliberate — the model is what the crate will be cut against.

Run them from the repository root:

```sh
models/run.sh --scope finish --family alloy
```

**The `--no-coverage` is gone, and that is the signal that this column is
closed.** Every one of the finish scope's sixty-three alloy cells is filled and
coverage is asserted; the flag was on that line from `entry-k39` to
`blocked-k48` and its departure is `exits-k49`'s, by construction.

## What is covered, and what is not

| family | file | obligations |
|---|---|---|
| Alloy 6 | `finish.als` | `FN-01`, `FN-05` – `FN-08` — the transaction's **entry surface**; `FN-09` – `FN-13` — the **reserved witness**; `FN-03`, `FN-04`, `FN-14` – `FN-18` — the **commit and its disposition**; `FN-19`, `FN-20` — the **quarantine and the atomic root rename**; `FN-22` — the **four revalidation points and the ten-row table**; `FN-21`, `FN-31` — **disposal**: its re-entrancy, the cleanup marker's create / replace / remove transitions, and the reaper; `FN-24` — the **crash slice**: what the disk a crash leaves classifies as, and what one step of the transaction may change; `FN-25`, `FN-26` — the **blocked slice**: the closed diagnosis partition over `RecoveryPending` and `OwnershipConflict`, the strict precedence that resolves the two places its arms meet, and what a block's diagnostic names; `FN-02`, `FN-23`, `FN-27` – `FN-30` — the **exits slice**: intent persisting as the finish leaf, recovery as monotone removal with a guard that goes false, the three unrelated-mutation obligations, the single successful exit, the completed refusal and its distinction from a block, and hook suppression; `FN-32` — **fail-closed ownership inside a transaction**, `TT-24`'s second context re-stated under an `FN-` prefix by `obligation-placement-k63` |
| Quint | `finish.qnt`, `finish-controls.qnt` | every `FN-` obligation in the scope, built independently of the Alloy column under the barrier `formal-modeling-k1`'s brief describes. The column is closed; see **The Quint column** below for its run line, its verification depth, its instances and its narrowings |

**Zero of the scope's sixty-three alloy cells are empty**, and coverage is
asserted on every run. The last eight were the `exits` slice's — `FN-02`,
`FN-23`, `FN-27.a` – `FN-27.c`, `FN-28`, `FN-29` and `FN-30`: intent persisting
as the finish leaf, recovery's idempotence, *nothing unrelated is mutated on any
outcome*, the single successful exit, a refusal as a complete outcome, and hook
suppression.

**`exits-k46` decomposed, and the reason is in this file.** (Historical, and
kept because the three slices' costs are measured against the cut.) It was cut as one
leaf for fourteen obligations; four of them need machinery no sibling slice
built — a stable-state classification of the disk, a persistent-effect
enumeration over all sixteen `bodySteps`, the `Blocked` diagnosis partition four
slices deliberately did not build, and hook suppression — where every earlier
slice of `finish-k8` added one or two. The node's three children are `crash`
(`FN-24`, this one, owning `EN-08`), `blocked` (`FN-25`, `FN-26`, owning
`EN-16`) and `exits` (`FN-02`, `FN-23`, `FN-27` – `FN-30`, plus Q4's removal
matrix and the runner question). The visible signal — `--no-coverage` leaving
the run line above — is the last of the three's, by construction: the matrix
needs `FN-24` and `FN-27` both, and `FN-02`'s witness is a decline followed by a
**successful** attempt, which does not exist before `FN-28`.

**All three landed and the cut held.** `crash-k47` took the scope to 147
commands and forty-nine filled cells; `blocked-k48` to **164** and
**fifty-three**; `exits-k49` to **180** and **sixty-one**, and no one of the
three needed machinery another built. What the second did NOT find is worth one
line, because the node brief predicted it would: the `Blocked` partition was
expected to be where four slices' abstinence paid off, and it was — five
counterexamples, three of them about the catalogue's own two definitions rather
than about the model. The third found four, and **three of the four are one
counterexample this file has now met from four directions**: `EN-11` cashed out
as a free initial state and as the world's own transitions makes every claim
about the DISK false unless it is restated over the protocol's own steps. The
fourth is `FN-28`'s second operand, and it is the largest single finding of the
three slices — see *A tenth finding, and it is about the shipped protocol*.

**Declared gaps** — none. The runner reads them from this file, in one shape:

```md
- **GAP** alloy `FN-nn.x` (inexpressible|abstracted|out-of-bounds|tool-limited) — reason.
```

**Two obligations of the *task-tree* scope are waiting on this directory, and
neither can be filled from either side as the placement rule stands — and the
content each needs is now present under an `FN-` prefix, so the re-statement
`obligation-placement-k63` chose is a citation change and nothing more.**
**SETTLED BY `obligation-placement-k63`, AND THE PREDICTION BELOW WAS HALF
RIGHT — WHICH IS WHY IT IS LEFT STANDING.** Both obligations are retired and
re-stated under `FN-` prefixes: `TT-24.d` is `FN-21.c`, a pure citation change
exactly as predicted, and `TT-24.c` is the new `FN-32`, which needed a claim
because `FN-25` is stated over `Blocked` outcomes **and nothing else** — it says
which diagnosis a block carries, never that this situation blocks. See
*`FN-32` — `TT-24`'s second context, arriving from the other scope* below. The
paragraph that follows is the prediction as it stood:

`crates/grove-task-tree/models/README.md` declared `TT-24.c` and `TT-24.d`
`out-of-bounds`, both because the context each names is a finish context: `TT-24.c`
is `Blocked(OwnershipConflict)` inside a finish or recovery transaction, and
`TT-24.d`'s subject is the quarantine reaper. This model will have both machineries
— `FN-25` states `Blocked(OwnershipConflict)` inside a transaction and `FN-21.c`
states the reaper's decline, which are exactly their two subjects — but the runner's placement rule
sends every `TT_`-prefixed command to the task-tree directory, so a `TT_24c`
command *here* is a placement failure rather than a filled cell. Whether the two
should be re-stated as `FN-` obligations is `obligation-placement-k63`'s to settle;
the re-statement would be a citation change rather than new modelling, because
`FN-21.c` and `FN-25` already carry the same content under `FN-` prefixes.

**Q4's artifact/transition removal matrix is not here yet.** The catalogue
requires one, in this file, per family — one row per removable artifact naming
the first shared-safety obligation its removal breaks, or `none`. It belongs to
the **third child of `exits-k46`**, which is the leaf that has every
shared-safety claim in front of it; a matrix written before `FN-24` and `FN-27`
exist would have nothing to name. `FN-24` now exists — the crash slice is the
first of the three children — and **it still decides no row**, for the reason
under *The mutation matrix*: `FN-24.b`'s two multi-effect steps are declared
abstractions of the incumbent's own machinery, and `FN-24.a`'s only mutation
that reaches an artifact is one that reaches the model's classification rather
than the protocol's. The matrix stays at five decided rows and two recorded as
undecidable; `FN-27` is the last shared-safety claim that could move it. **Five of its rows are now decided** and are recorded under *The mutation
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
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark,
N steps`.
Five parts of it mean something other than "make it bigger":

- **`2 Device`** is `EN-02`'s dimension and nothing else. One device makes
  `FN-08`'s witness — a layout that passes at the lease gate and fails at the
  transaction's own operands — inexpressible rather than false, which is exactly
  what the assumption's *exercise-removal* control asserts.
- **`2 AttemptId`** and **`2 Digest`** are the witness slice's, and both exist so
  that a claim about recording *this* value is not the same statement as a claim
  about recording *some* value. At one atom each, `Man.mAttempt = Txn.attempt`
  and `Man.mDigest = Root.holds <: digest` hold for any manifest that records
  anything at all, and `FN-12.a` would be checking presence rather than content.
- **`2 CMark` IS THE DISPOSAL SLICE'S ONE SCOPE DIMENSION, AND IT EXISTS SO
  THAT `FN-31.b` CAN BE FALSE.** *No reader observes the marker absent, **nor
  observes two markers*** is two prohibitions. Modelled as `one sig Mark { var
  there: lone Marker }`, the second is **inexpressible** and half the claim is
  true by construction — the false-confidence shape this file has now recorded
  five times. So a cleanup marker is an **atom** and what is `var` is which
  markers stand at the reserved name (`Cleanup.present: set CMark`), which makes
  `#Cleanup.present = 2` a state the model can be in and a remove-then-create
  replacement a trace it can take. Two atoms is the smallest scope in which a
  replacement has an old marker and a new one to be distinguishable; at one, *the
  replacement superseded the marker* and *the replacement left it alone* are the
  same instance. `cOwner` and `cTarget` are **static**, because a marker's bytes
  are written once and it is the marker's PRESENCE that changes.
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
- **THE CRASH SLICE ADDS NO SCOPE DIMENSION, NO `var` FIELD AND NO TRANSITION —
  AND IT IS THE FIRST SLICE IN THIS SCOPE THAT ADDS NONE OF THE THREE.** What it
  adds is sixteen **static** `one sig` atoms in two new abstract signatures
  (`Stable`, `Effect`) and a handful of `fun`s and `pred`s over them, all of them
  read only by claims. `crash` has been enabled at every step boundary since
  `witness-k40`; `FN-24` is the first claim that asks what the disk it leaves
  classifies as. **It did move the ceiling, from 12 to 13**, and by exactly one
  command: `witness_FN_24a_a_crash_after_the_cleanup_marker_is_removed` needs
  disposal's last step and then an interruption, which is one state past the
  deepest trace the file had.
- **THE BLOCKED SLICE ADDS NONE OF THE THREE EITHER, AND IT IS THE SECOND
  CONSECUTIVE SLICE THAT DOES NOT.** Six static `one sig` atoms in two abstract
  signatures — `Diagnosis` (two members) and `BlockField` (four) — six `pred`s,
  three `fun`s and one two-element precedence relation, every one of them read
  only by claims. `BlockedOutcome` still carries no diagnosis and `Sys.why` is
  still model-only, which is the point: the partition is data the claims range
  over, not a field a guard sets. **The ceiling does not move.** Thirteen holds
  it, where `FN_24a` already sat and where this slice's four checks sit; the
  deepest witness here is **nine**, because every block is reachable within four
  steps of `interruptedMidEvacuation`'s posited disk.
- **`N steps` still ranges from 2 to 12, AND THE DISPOSAL SLICE DID NOT MOVE THE
  CEILING — which is not what its own arithmetic predicted.** Splitting the
  forward settle into three steps put two transitions into a path five inherited
  witnesses run through, and the file's rule says a step inserted into a path
  costs a state to every witness that passes through it. **Two witnesses moved
  and no more**: `witness_FN_03` 10 → **12** and `witness_FN_18` 10 → **11**.
  The three that did not — `witness_FN_16b`, `witness_FN_22i` and
  `witness_FN_22a_the_point_after_the_quarantine_rename_is_reached` — each end
  ON the inserted step rather than past it, and their final assertion was
  re-anchored from `Settle` to `MarkerCreate`, which is the same position in the
  trace. That is a **seventh entry in the bound register** and it is stated
  beneath the table. The two new commands that reach twelve —
  `witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves`, and
  `witness_FN_22h` which was already there — are what hold the ceiling where
  `revalidation-k44` left it.
- ~~**`N steps` now ranges from 2 to 12, and THE REVALIDATION SLICE IS THE FIRST
  TO MOVE THE CEILING SINCE `witness-k40`.**~~ It stood at ten from the witness
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
lands; the ninety-three rows below carry the disposal slice's sweep of
sixty-nine plus the crash slice's twenty-four.

**THE EXITS SLICE ADDS A `var` FIELD AND STILL DOES NOT OWE THE FULL SWEEP, AND
THE REASON IS A STRONGER ARGUMENT RATHER THAN AN EXEMPTION.** The rule the crash
slice left is *a sibling that adds a `var` field, a transition or a fact does not
inherit this argument and owes the full sweep*, and this slice adds one:
`World.hookRan`, a `lone` field over the `one sig Hook`. What replaces the sweep
is a **monotonicity proof** rather than the crash slice's absence argument. No
guard, no `fact` and no witness reads the field; twenty-eight of the file's
twenty-nine transitions frame it through `worldSame` and the twenty-ninth
(`doTopologyChange`) leaves it free within `World.hookInstalled`. So every
instance of the file before the field extends to an instance after it by setting
`hookRan` absent in every state and `hookInstalled` absent throughout — the
transition relation restricted to those instances is unchanged — and therefore
**no pre-existing trace can be removed and no witness's first-landing bound can
rise**. A `var` field that adds only free choice is not the `var` field the rule
was written about; a field a guard reads is. **The rule is sharpened rather than
broken**: *a sibling that adds a `var` field ANY GUARD, FACT OR EXISTING COMMAND
READS owes the full sweep; one that adds free choice alone owes a proof and a
control.*

**EIGHT INHERITED ROWS WERE RE-MEASURED AS THAT CONTROL** — the same eight the
crash slice ran, for the same reason it ran them — and every one still lands at
its recorded bound: `witness_FN_01a` (2), `witness_FN_21c` (2),
`witness_FN_09b` (7), `witness_FN_11` (10), `witness_FN_15b_git` (11),
`witness_FN_22h` (12), `witness_FN_31a_the_stale_marker` (12) and
`witness_FN_03` (12).

**THE CRASH SLICE IS THE FIRST WHOSE INHERITED ROWS COULD NOT MOVE, AND THE
ARGUMENT IS BETTER EVIDENCE THAN THE SWEEP WOULD HAVE BEEN.** It adds no
transition, no `var` field and no `fact`; the transition relation and every
existing signature's state are byte-for-byte what the disposal slice left. Two
new abstract signatures of `one sig` atoms cannot make a trace exist or stop
existing, so no inherited witness's first-landing bound can move — that is a
proof rather than a measurement, and it is stated here because the alternative
was a sixty-nine-witness sweep that could only have confirmed it. **Eight
inherited rows spread across the whole depth range were re-measured anyway, as
the control the argument is worth nothing without**: `witness_FN_01a` (2),
`witness_FN_21c` (2), `witness_FN_09b` (7), `witness_FN_11` (10),
`witness_FN_15b_git` (11), `witness_FN_22h` (12), `witness_FN_31a_the_stale_marker`
(12) and `witness_FN_03` (12) each still land at their recorded bound and at no
smaller one. A sibling that adds a `var` field, a transition or a fact does not
inherit this argument and owes the full sweep.

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
| `witness_FN_02_a_decline_followed_by_a_later_successful_attempt_on_the_same_handle` | **11** |
| `witness_FN_23_three_consecutive_recoveries_the_second_and_third_changing_nothing` | 4 |
| `witness_FN_27a_a_success_with_unrelated_work_present` | **9** |
| `witness_FN_27b_a_refusal_with_unrelated_work_present` | **10** |
| `witness_FN_27c_a_block_with_unrelated_work_present` | **9** |
| `witness_FN_28_a_success_whose_cleanup_is_still_outstanding` | **9** |
| `witness_FN_29a_a_refused_attempt_followed_by_a_successful_one` | **10** |
| `witness_FN_30_a_hook_that_would_have_run_shown_suppressed` | **8** |
| `witness_FN_12a_a_manifest_interrupted_before_its_ready_mark` | **8** |
| `witness_FN_12b_a_refused_entry_type` | 3 |
| `witness_FN_13_a_commit_attempted_while_the_witness_is_tracked_refused` | **10** |
| `witness_FN_03_a_retry_with_no_local_trace_settling_forward_on_the_ticket_alone` | **12** |
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
| `witness_FN_18_a_proven_commit_reached_after_an_interruption_mid_evacuation` | **11** |
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
| `witness_FN_21a_a_disposal_interrupted_mid_disposal_and_resumed` | 4 |
| `witness_FN_21a_the_interrupted_disposal_disk_is_reachable` | **11** |
| `witness_FN_21b_a_reaper_declining_an_entry_its_in_tree_witness_still_owns` | 2 |
| `witness_FN_21c_a_foreign_entry_at_a_reserved_name_is_declined` | 2 |
| `witness_FN_31a_a_source_state_from_which_disposal_must_replace_a_marker` | **10** |
| `witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves` | **12** |
| `witness_FN_31b_an_observation_interleaved_with_the_replacement_sees_one_marker` | **10** |
| `witness_FN_31c_an_interruption_before_the_replacement_is_resumed` | 3 |
| `witness_FN_31c_an_interruption_after_the_replacement_is_resumed` | 4 |
| `witness_FN_31d_a_foreign_marker_is_declined` | **10** |
| `witness_FN_24a_a_crash_after_the_witness_is_prepared` | **6** |
| `witness_FN_24a_a_crash_after_the_manifest_is_written` | **7** |
| `witness_FN_24a_a_crash_after_the_manifest_is_marked_ready` | **8** |
| `witness_FN_24a_a_crash_after_the_witness_is_published` | **9** |
| `witness_FN_24a_a_crash_after_the_tree_is_evacuated` | **7** |
| `witness_FN_24a_a_crash_after_the_commit_is_attempted` | **7** |
| `witness_FN_24a_a_crash_after_a_recovery_adopts_the_witness` | **6** |
| `witness_FN_24a_a_crash_after_the_classification` | **9** |
| `witness_FN_24a_a_crash_after_the_quarantine_rename` | **10** |
| `witness_FN_24a_a_crash_after_the_settle` | **10** |
| `witness_FN_24a_a_crash_after_the_revalidation` | **11** |
| `witness_FN_24a_a_crash_after_the_quarantine_is_returned` | **12** |
| `witness_FN_24a_a_crash_after_the_cleanup_marker_is_created` | **11** |
| `witness_FN_24a_a_crash_after_the_cleanup_marker_is_replaced` | **11** |
| `witness_FN_24a_a_crash_after_the_quarantine_is_disposed` | **12** |
| `witness_FN_24a_a_crash_after_the_cleanup_marker_is_removed` | **13** |
| `witness_FN_24b_a_step_whose_one_effect_is_at_the_reserved_witness_name` | 5 |
| `witness_FN_24b_a_step_whose_one_effect_is_the_manifest` | **6** |
| `witness_FN_24b_a_step_whose_one_effect_is_the_ready_mark` | **7** |
| `witness_FN_24b_a_step_whose_one_effect_moves_entries` | **9** |
| `witness_FN_24b_a_step_whose_one_effect_is_the_commit` | **7** |
| `witness_FN_24b_a_step_whose_one_effect_is_the_atomic_root_rename` | **9** |
| `witness_FN_24b_a_step_whose_one_effect_is_at_the_cleanup_marker_name` | **10** |
| `witness_FN_24b_the_declared_step_with_two_persistent_effects` | **11** |
| `witness_FN_25a_a_correlated_attempt_at_a_name_grove_also_reserves_resolved_to_one` | **9** |
| `witness_FN_25b_a_block_whose_only_arm_is_recovery_pending` | **9** |
| `witness_FN_25b_a_block_whose_only_arm_is_ownership_conflict` | **9** |
| `witness_FN_25c_git_recovery_pending_reached` | **9** |
| `witness_FN_25c_nativejj_recovery_pending_reached` | **9** |
| `witness_FN_25c_colocatedjj_recovery_pending_reached` | **9** |
| `witness_FN_25c_git_ownership_conflict_reached` | **9** |
| `witness_FN_25c_nativejj_ownership_conflict_reached` | **9** |
| `witness_FN_25c_colocatedjj_ownership_conflict_reached` | **9** |
| `witness_FN_26_a_block_whose_diagnostic_names_all_four_with_history_unchanged` | **9** |

**ALL TEN OF THE BLOCKED SLICE'S WITNESSES FIRST LAND AT NINE, AND THE FLATNESS
IS THE MEASUREMENT.** Each was swept from 3 to 10 and none lands at eight. The
reason is structural rather than coincidental: every block in this file is
reached by RUNNING the protocol — `fact TransactionsStartWhereAProcessStarts`
confines state 0 to `Fresh + Opened`, so no block can be posited — and the
shortest route from `interruptedMidEvacuation`'s posited disk to any blocking
gate is the same four steps whichever gate it is. **What that predicts, and what
the third child should not have to rediscover, is that a witness whose subject is
an OUTCOME rather than a step costs the run-up and nothing else.** The node
brief expected three of these near ten and three cheaper; all six are the same
number, and the reason the deep ones are not deep is that a foreign cleanup
marker can be POSITED in the initial state where `witness_FN_31d` had to reach
one through disposal.

**THE INHERITED-BOUND SWEEP IS ARGUED RATHER THAN RUN, ON `crash-k47`'s ARGUMENT
AND UNDER ITS OWN CONDITION.** This slice adds no transition, no `var` field and
no `fact`; the transition relation and every existing signature's state are
byte-for-byte what the crash slice left, so no inherited witness's first-landing
bound can move. **Six inherited rows across the depth range were re-measured as
the control the argument is worth nothing without**: `witness_FN_01a` (2),
`witness_FN_10b` (2), `witness_FN_12a` (8), `witness_FN_11` (10),
`witness_FN_31d_a_foreign_marker_is_declined` (10) and `witness_FN_22h` (12) each
still land at their recorded bound and at no smaller one. `witness_FN_31d` is in
the control set deliberately: it is the one inherited witness whose subject —
a foreign marker — this slice now also reads.

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
(`FN-22.h`). Two inherited checks move with their witnesses: **`FN-03` to 12** and
**`FN-18` to 11**.

**THE CRASH SLICE'S TWO CHECKS, AND THE TWO RULES AGREE ON ONE AND DISAGREE ON
THE OTHER.** `FN-24.a` runs at **13**: its widest witness is the crash after
disposal's last step at 13, and its antecedent — a body step followed by an
interruption — is deepest at exactly that trace, so both rules give 13.
`FN-24.b` runs at **12**, where the two rules disagree by one and the antecedent
rule wins: its widest witness lands at 11, and the antecedent quantifies over
`bodySteps`, whose deepest member (`MarkerRemove`) first occurs at 12. Run at 11
the check would have said nothing about the last step of the transaction — which
is `disposal-k45`'s third predictor in its ordinary, undramatic form, applied
before the fact rather than after a mutation survived.

**AND THE DISPOSAL SLICE IS WHERE THE RULE, APPLIED LITERALLY, WOULD HAVE MADE A
CHECK VACUOUS — WHICH IS A THIRD BOUND-VACUITY PREDICTOR FOR THIS CORPUS.**
`FN-31.c`'s two witnesses land at **3** and **4**, because both posit the disk an
interruption leaves and run the sweep over it; the rule says *the widest
first-landing bound among the obligation's witnesses*, with 4 as the floor. Run
at 4, `FN_31c`'s first conjunct — *both markers the replacement touches are ones
a sweep can act on* — has **no reachable antecedent at all**, because
`MarkerReplace` first occurs at ten states. The check would have been green and
empty, and its mutation would have reported exactly as a survivor.

`task-tree-k7` left two predictors for bound vacuity: an interval claim needs
interval-many states, and the bound must hold *the machinery of the transitions
the obligation quantifies over*, not only the objects it names. The second is
this case — and what is new is **where the wrong number comes from**. It is not
carelessness: it is the file's own witness-bound rule, applied to an obligation
whose witnesses are **cheap posited disks** and whose antecedent is a **deep
transition**. The two had always coincided before, because every witness in this
file ran the protocol up to the thing it was witnessing.

> **The witness-bound rule is a FLOOR, and it is below the real floor whenever an
> obligation's witnesses posit a disk its antecedent has to be run up to.** Read
> the check's own antecedent for the deepest transition it names, and take the
> larger of the two numbers.

Applied: `FN-31.a`, `FN-31.b`, `FN-31.c` and `FN-31.d` all run at **11** — the
witness rule gives 11, 11, 4 and 11, and the antecedent rule gives 11 for all
four, because every one of them quantifies over `MarkerReplace`. `FN-21.a` runs
at **11**, where both rules agree. `FN-21.b` and `FN-21.c` run at **5** — a
stated margin of **three** over the floor of 2, and their antecedent is `Reap`,
which first occurs at **2**, so the two rules agree here as well. Stating the
margin as a number is `FN-20`'s lesson from the previous slice, applied
pre-emptively.

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
   that CLOSED ITS LASSO on it** (`revalidation-k44`, twelve witnesses +1). Ask:
   which witnesses' final transition is one whose enabling surface you narrowed?
4. **A step inserted at the END of a path costs nothing to a witness whose final
   assertion RE-ANCHORS onto it** (the disposal slice, three witnesses +0 where
   the arithmetic said +2). The disposal split put two transitions between the
   after-rename point and `Settled`. Five inherited witnesses run through that
   stretch, and only the two that must reach the far end of it moved —
   `witness_FN_03` +2 and `witness_FN_18` +1. `witness_FN_16b`,
   `witness_FN_22i` and `witness_FN_22a_the_point_after_the_quarantine_rename` each
   asserted *a forward settle happened*, which is now *the first disposal step
   happened*, at the same position in the trace. Ask: does this witness need to
   reach **past** the insertion, or only **to** it? This is entry 2 sharpened
   rather than a new shape — *passes through* is the condition, and a witness
   that ends at the insertion point does not pass through it.

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

**THE EXITS SLICE ADDS ONE `var` FIELD AND ITS TAX IS BETWEEN −7% AND +18%, WHICH
IS A DIFFERENT SHAPE FROM THE STATIC LAW RATHER THAN A LARGER NUMBER.** The
sentinel A/B is the same instrument the last two slices used — the same four
inherited commands, run against this file and against a copy with
`World.hookRan`, its `worldSame` conjunct, its `doTopologyChange` conjunct and
`FN-30`'s two commands removed, best of two:

| sentinel | without the field | with it | delta |
|---|---|---|---|
| `FN_01a` | 2.49 s | 2.32 s | **−0.17 s (−6.8%)** |
| `FN_03` | 8.15 s | 9.57 s | **+1.43 s (+17.5%)** |
| `FN_22a` | 13.82 s | 14.33 s | **+0.51 s (+3.7%)** |
| `FN_24b` | 52.46 s | 50.73 s | **−1.73 s (−3.3%)** |

**Read the absolute column, not the percentage, and notice that it does not
scale.** The static-atom law the last two slices measured is uniform — about
10 ms of translation per atom per command, so a dear command pays proportionally
more in seconds and the same in percent. A `var` field does not behave that way:
the file's **dearest** command moved least, in seconds and in percent, while an
8-second command moved +1.4 s. The reason is that a `var` field adds a boolean
**per state**, so what it costs a command tracks that command's TRACE LENGTH and
the size of its transition-relation encoding rather than the difficulty of its
own claim, and at a bound of thirteen with a field nothing reads the SAT solver's
own variance is the same order as the effect. Four sentinels is not enough to
separate the two, and the honest register entry is the qualitative one:

> **A `var` field is not static structure, even when no guard reads it.** The
> monotonicity argument above says the field cannot change which INSTANCES exist;
> it says nothing about what they cost. Budget a `var` field at a few per cent of
> the whole file, not at zero, and do not expect the static law's per-atom
> arithmetic to apply to it.

**In aggregate the slice is +2 m 08 s on a 12 m 25 s file, and the sixteen new
commands account for essentially all of it.** 180 commands, **14 m 33 s**,
against 164 in 12 m 25 s.

> **After `finish-scope-k71` this file is still 186 commands, `exit 0`, and
> `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`** — that leaf changed
> comments, one branch's `Sys.res'`/`Sys.why'` and nothing else, so the command
> set did not move. It measured **27 m 44 s wall / 1935 s CPU** on a 16-core
> host, run **concurrently with this scope's `QUINT_VERIFY=1` cell**, which is
> where the extra wall time over the ~18 min below went: Apalache is heap- and
> CPU-hungry in a way the simulator is not, and *concurrent cells barely contend*
> stops being true when one of them is a model checker. Budget the two serially,
> or budget the verify cell alone.
>
> **After `finish-scope-k76` this file is 187 commands, `exit 0`, and
> `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`.** The one new command
> is `expect_unreachable_EN_08_no_resumption_of_an_interrupted_disposal_is_reachable`,
> the control that establishes the `FN-31.c` row this column had declared
> unmeetable; the two `FN-31.c` witnesses were rewritten from 3 and 4 steps to 14
> and `FN_25a` was restated, so the command COUNT moved by one and three
> commands got dearer. **18 m 48 s wall** (`2026-08-28T02:03:05Z` →
> `02:21:53Z`) on a 16-core host, run alone; this file's digest is identical
> either side of it. The three changed commands cost 7 s, 6 s and 9 s, so the run
> is within noise of the ~18 min below and the edits are not what moves it.
>
> **It is the SECOND run of this cell in that leaf, and the first is worth
> keeping.** The first (187 commands, exit 0, 20 m 12 s, run concurrently with
> the non-verify Quint cell) measured an `FN_25a` with three conjuncts; re-reading
> the edit found that deleting `lone diagnosedRaw or declaredDiagnosisOverlap`
> along with the false label it carried had dropped a true claim — *no undeclared
> overlap* — so the clause went back and the cell was re-run rather than
> re-argued.
>
> **After `closed-set-additions-k74` this file is 186 commands, `exit 0`, and
> `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`** — the `FN-29` split
> plus `FN-29.b`'s two witnesses. It measured **~18 min** on a 16-core host
> rather than the 14 m 33 s above, and the difference is the host rather than the
> six commands: a single witness command timed there took **23 s wall against
> 7.5 s of CPU (34%)**, so these runs are startup- and IO-bound. Run the four
> per-family cells CONCURRENTLY — they barely contend, and the batch's wall time
> becomes this cell alone. The eight new checks cost 45.0 s (`FN_02`), 11 s
(`FN_23`), 7 s each (`FN_27.a` – `FN_27.c`), 18 s (`FN_28`), 10 s (`FN_29`) and
6 s (`FN_30`) — 111 s — and the eight witnesses about 4–5 s apiece, for roughly
2 m 27 s of new work against a 2 m 08 s aggregate delta. The inherited commands
therefore paid nothing measurable in the aggregate, which is consistent with the
sentinel table's mixed signs and is why the sentinel table is the one that is
reported rather than the aggregate.

**`FN_02` IS THE SLICE'S DEAREST COMMAND AT 45 s, AND ITS FIRST FORM WAS 3 s.**
The 3-second form was the one with a counterexample in it — a check that fails
early is cheap, and a check that must exhaust the space is not, so **a mutation's
timing is evidence about the mutation and a green check's timing is evidence
about the claim.** Worth one line because the reverse mistake is easy: a slice
that budgeted from its first, red run would have budgeted an order of magnitude
low.

**THE BLOCKED SLICE COST 1–6%, AND IT IS THE SECOND MEASUREMENT OF THE FLAT LAW
AND THE ONE THAT GIVES IT A COEFFICIENT.** It adds no transition, no `var` field,
no `fact` and no scope dimension: six static `one sig` atoms in two abstract
signatures, six predicates, three functions and a two-element precedence relation
that only claims read. Medians of three, one host, one sitting, both files
present, and a clean A/B on the four inherited sentinels:

| command | crash slice | blocked slice | | absolute |
|---|---|---|---|---|
| `FN_08` (4 steps, entry surface) | 2.16 s | 2.28 s | **+6%** | +0.12 s |
| `FN_07` (4 steps, entry surface) | 2.41 s | 2.49 s | **+3%** | +0.08 s |
| `FN_13` (10 steps, the widest inherited) | 8.31 s | 8.41 s | **+1%** | +0.10 s |
| `witness_FN_11` (10 steps) | 3.95 s | 4.02 s | +2% | +0.07 s |

(The crash column is re-measured in **this** sitting and reads 8.31 s on `FN_13`
where that slice's own figures said 8.26 s, and 2.16 s on `FN_08` where they said
2.20 s — the third measurement rule doing its job for the third consecutive
slice.)

> **The flat cost has a coefficient, and it is the number of ATOMS.** The crash
> slice added sixteen static atoms and moved the four sentinels by 0.17, 0.20,
> 0.10 and 0.14 s. This one added **six** and moved them by 0.12, 0.08, 0.10 and
> 0.07 s. Same shape — an absolute constant, not a percentage — at roughly a
> third to a half the size, on commands whose totals differ by a factor of four.
> **Budget static structure at about 10 ms of translation per atom per command
> and stop reading the percentage**, which is a large fraction of a small number
> on a tight sentinel and a rounding error on a wide one.

That is the second slice in a row for which *budget by the number of states of a
trace at which a transition is enabled* prices the work at **zero**, and zero was
again very nearly right. The law has now been wrong three times and right three
times, and every one of the three it was right about added no transition.

**Where the suite's time actually went.** 164 commands, **12 m 08 s**, against
147 in 10 m 33 s. Seventeen new commands — four checks, ten witnesses and three
`EN-16` controls — of which the ten witnesses all sit at nine states and the
three controls at eight to ten. The four checks are **7.2 s**, **7.6 s**, **5.2 s**
and **15.4 s**; `FN_26` is the dearest of the four and is nowhere near
`FN_24b`'s 51.8 s, which remains the dearest single command in the file. All four
run at thirteen states with antecedents that quantify over every blocking
transition in the file, and all four are cheap for the reason the crash slice
recorded from the other side: **a check whose antecedent is wide but whose
consequent is a function of static data is cheap at any bound.** `FN_26` costs
twice its siblings because its third conjunct is nested — `always (... implies
always (...))` — which is the one thing in this slice that is not a function of
static data.

**A whole-suite total still does not compare across sessions**, and this pair is
quoted only because both halves carry their command counts; the sentinel A/B puts
this slice's tax on the inherited commands at 1–6%, so essentially all of the
extra 1 m 35 s is the seventeen new commands themselves.

---

**THE CRASH SLICE COST 1–9%, AND THE SHAPE OF THE COST IS INVERTED FROM EVERY
SLICE BEFORE IT.** It adds no transition, no `var` field and no scope dimension:
sixteen static `one sig` atoms in two abstract signatures, and `fun`s over them
that only claims read. Medians of three, one host, one sitting, both files
present, and a clean A/B on the four inherited sentinels:

| command | disposal slice | crash slice | |
|---|---|---|---|
| `FN_08` (4 steps, entry surface) | 2.03 s | 2.20 s | **+8%** |
| `FN_07` (4 steps, entry surface) | 2.24 s | 2.44 s | **+9%** |
| `FN_13` (10 steps, the widest inherited) | 8.16 s | 8.26 s | **+1%** |
| `witness_FN_11` (10 steps) | 3.83 s | 3.97 s | +4% |

(The disposal column is re-measured in **this** sitting and reads 8.16 s on
`FN_13` where that slice's own figures said 8.37 s — the third measurement rule
doing its job, as it did for the slice before.)

**EVERY SLICE BEFORE THIS ONE COST MOST ON THE WIDEST COMMAND AND LEAST ON THE
TIGHTEST. THIS ONE IS THE OTHER WAY ROUND, AND THAT IS THE FINDING.** The
disposal slice was +13% / +11% on the two entry-surface sentinels and **+19%** on
`FN_13`; the revalidation slice +9% / +11% against +14%; the commit slice's ratio
was steeper still. Here the tightest commands moved **eight and nine points** and
the widest moved **one**. The absolute movement is nearly the same number of
milliseconds in all four — 0.17 s, 0.20 s, 0.10 s, 0.14 s — which is what a
**flat** cost looks like on four commands whose totals differ by a factor of four.

> **The cost law is about STATE. Static atoms and static relations cost a
> roughly CONSTANT amount per command, not a percentage of it** — they enlarge
> `univ` and the per-command translation, both of which are paid once whatever
> the bound. Read a percentage on a tight sentinel as a large fraction of a small
> number, and do not extrapolate it to the wide ones.

That is a fourth statement of one law rather than a fifth law: *budget by the
number of states of a trace at which a transition is enabled* prices this slice
at **zero**, and zero was very nearly right. It also means the four-sentinel
convention this scope inherited has been carrying a second job all along — one
sentinel is not enough is `task-tree-k7`'s rule for **state**, and this slice is
the first case where the tight sentinel is the one that saw the movement.

**Where the suite's time actually went.** 147 commands, **10 m 33 s**, against
118 in 7 m 39 s. Twenty-nine new commands — two checks, twenty-four witnesses and
three `EN-08` controls — of which six are at 11–13 states. The two checks are
**5.3 s** and **51.7 s**: `FN_24b` is the dearest single command in the file, and
it is dear for the ordinary reason — a `bodySteps`-wide antecedent at twelve
states — rather than for a new one. `FN_24a` at thirteen states is 5.3 s, which
is the flat-cost result again from the other side: a check whose antecedent is
wide but whose consequent is a function of static data is cheap at any bound.
**A whole-suite total still does not compare across sessions**; this pair is
quoted only because both halves carry their command counts, and note that the
sentinel A/B above puts the crash slice's own contribution at 1–9%, so most of
the 2 m 54 s is the twenty-nine new commands rather than a tax on the old ones.

---

**THE DISPOSAL SLICE COST 11–19%, AND FOR THE FIRST TIME IN THIS SCOPE THE COST
LAW WAS NOT PESSIMISTIC — BECAUSE ONE TRANSITION IS A SWEEP AND THE OTHERS ARE
NOT.** Four new reachable Grove transitions (`MarkerCreate`, `MarkerReplace`,
`Dispose`, `MarkerRemove`), two new phases, **one new scope dimension**
(`2 CMark`), one enabling point removed (the forward settle's `Quarantined`
branch), and **one sweep enabled outside the phase machine altogether** (`Reap`).
Medians of three, one host, one sitting, both files present, and a clean A/B: no
bound moved on any of the four sentinels.

| command | revalidation slice | disposal slice | |
|---|---|---|---|
| `FN_08` (4 steps, entry surface) | 1.97 s | 2.22 s | +13% |
| `FN_07` (4 steps, entry surface) | 2.17 s | 2.41 s | +11% |
| `FN_13` (10 steps, the widest inherited) | 7.04 s | **8.37 s** | **+19%** |
| `witness_FN_11` (10 steps) | 3.45 s | 3.95 s | +14% |

(The revalidation column is re-measured in **this** sitting and reads 7.04 s
where that slice's own figures said 6.93 s — which is the third measurement rule
below doing its job rather than a discrepancy.)

**THE SWEEP WAS ISOLATED AND PRICED, AND IT IS THE MOST USEFUL NUMBER IN THIS
FILE SINCE `quarantine-k43`'s.** `revalidation-k44` predicted that `FN-21`'s
reaper would be the first thing in this scope the dwell form of the law says is
expensive, *because a sweep is enabled at states no phase machine constrains* —
and named this slice as the third chance to measure the law and the first chance
to see it wrong in the **dear** direction. Three variants of the same file, same
sitting, same command, medians of three:

| `FN_13` at 10 states | median | against |
|---|---|---|
| the disposal slice with **no `Reap` at all** | 7.54 s | +7% on the inherited file |
| the disposal slice as it ships (`Reap` guarded on *something at a reserved name*) | 8.34 s | **+11% on top** |
| the same with `Reap`'s antecedent widened to **every** `Fresh` state | 8.68 s | +15% on top |

Three things follow, and the first is the headline:

- **ONE SWEEP AT A DWELL PHASE COSTS MORE THAN FOUR TRANSITIONS AT PASS-THROUGH
  PHASES PUT TOGETHER.** The four disposal steps, the two phases and the new
  scope dimension are +7% between them; `Reap` alone is +11%. The dwell form
  predicted the *direction* and the *ordering* correctly, which is the first time
  in three slices the law has not needed its arithmetic corrected — and it is the
  first slice in this scope whose dominant new cost is a single transition.
- **THE NARROWED ANTECEDENT BOUGHT A QUARTER OF IT.** `some Cleanup.present or
  some Quar.qRid` against an unguarded `Txn.phase = Fresh` is 8.34 s against
  8.68 s: about 4 points of the sweep's 15. A sweep over nothing is a no-op, so
  the narrowing costs no reachable behaviour — which makes it the cheapest
  possible instance of *prefer a narrowed antecedent*, arriving with a number on
  it for the second time in this scope.
- **AND IT IS STILL ONLY +11%, WHICH IS THE SECOND-ORDER RESULT.** A sweep
  enabled at a phase a trace can rest in is the expensive shape the law names,
  and it cost a fifth of what `commit-k41`'s four transitions cost on the same
  sentinel. **The ordering the corpus recommends has now been right four times
  running and the multiplier has been wrong three times; take the ordering, do
  not take the multiplier** — and read *expensive* as *worth narrowing*, not as
  *worth avoiding*.

**Where the suite's time actually went.** 118 commands, **7 m 39 s**, against 101
in 5 m 46 s. Seventeen new commands, six of them at 10–12 states, plus two
inherited bounds rising and the +19% on everything wide. A whole-suite total
still does not compare across sessions — this pair is quoted only because both
halves carry their command counts.

---

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

Beyond the catalogue's own *Deliberate omissions*, which
this file adopts unchanged:

- **THE STABLE-STATE CLASSIFICATION IS EVERY ROW OF THE CATALOGUE'S TABLE A
  FINISH TRANSITION CAN PRODUCE, AND NOTHING ELSE.** §*States* classifies a task
  root in a fixed order over the four `Reserved` classes, `Absent`,
  `PartialScaffold`, `Legacy`, `Foreign`, `Malformed` and three `Current`
  classes. No finish transition produces a `Migrating`, a partial scaffold, a
  legacy tree, a foreign format witness or a malformity — all five are the
  task-tree scope's — so `classifiedRaw` carries `Absent`,
  `Reserved(Preparing)`, `Reserved(Published)`, `Reserved(Quarantined)` and the
  three `Current` rows. **`Reserved(Quarantined)` was this file's own and is now
  the catalogue's**, landed by `finish-scope-k71` along with the order; see *A
  fifth finding* below. Each arm is the catalogue's row verbatim and the arms
  **overlap**; the classification ORDER is what resolves them, and it is written
  as a strict precedence relation so that deleting a pair leaves two survivors
  rather than silently changing which one wins.
- **A step's PERSISTENT EFFECTS are counted at the grain `FN-24.b` states them,
  which is the effect and not the field.** Three consequences, and each of them
  is a case where a field-by-field count would report a correct protocol as a
  defective one: a same-directory rename touches two names and `EN-01` makes it
  **one** effect; removing a directory removes what is inside it, so a step that
  releases the reserved witness has not separately written its manifest; and
  moving entries between two names is one move however many entries move.
  Counted by field, the completed refusal would read as four persistent effects
  and the atomic root rename as two.
- **TWO STEPS HAVE MORE THAN ONE PERSISTENT EFFECT, AND BOTH ARE DECLARED — which
  is what `FN-24.b` asks for in as many words.** They are named in one place
  (`declaredMultiEffect`), so narrowing the check and declaring the abstraction
  are the same edit; a check quietly weakened until it passes and a declaration
  are otherwise indistinguishable in a green run.
  - **`Dispose` clears the quarantine name and the reserved witness name
    together.** In this model they are two `one sig`s; in the shipped protocol
    the witness is **inside** the root the rename moved, so removing the one
    removes the other. *To decompose it* the model needs a containment relation
    between the two names — which is the same abstraction `EN-03` (no atomic
    recursive deletion) already forces the shipped removal to take entry by
    entry, and which `disposal-k45` recorded as *what is not modelled is a
    partial removal within the step*.
  - **`doSettle`'s restore branch puts the tree back and reproduces the exact
    preflight commit**, and on a working-copy-as-commit lane those are two
    persistent effects. *To decompose it* the model needs a phase between the
    restoration and the reproduction — which is exactly what `revalidation-k44`
    did to the settle once already, for `FN-22`'s after-restoration row. What a
    **fifth** revalidation point would cost is `FN-22`'s question rather than
    this claim's, and `FN-24.b` declares rather than answers it.
- **The tree is coarse: no filename grammar.** An entry is an opaque object with
  a type, a role and a digest. No `FN-` claim in these slices quantifies over
  names, positions, keys or slugs, so the grammar that occupies most of
  `task-tree.als` would be machinery no claim here reads. The two reserved names
  — `PREPARING-FINISH-<handle>-<attempt>/` and `FINISHING-<handle>/` — are
  modelled as one `Slot` with a **class**, which is the only thing `FN-09.a`
  reads about them: they are two names in one directory, so publication is one
  same-directory rename.
- **`Sys.why` is a model-only observable, and it now names post-flight
  conditions too.** The catalogue fixes seven preconditions and (now) **twenty**
  refusal reasons, and — at the time this was written — stated the mapping
  between them nowhere. `closed-set-additions-k74` changed half of that: a reason
  now provably names *the question asked, not the gate that asked it*, which is
  why `Sys.why` survives the widening rather than being replaced by it. Two of
  the seven preconditions are still one question at two gates, so six reasons
  still cannot witness seven members. `why` names which condition refused. Nothing in the shipped contract corresponds to it, and no
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
- **THE EXITS SLICE'S THREE, AND THE FIRST IS THE ONE A READER SHOULD NOT SKIP.**
  - ***The task root is absent* is read as `Txn.pinned not in Root.rid`, not as
    `no Root.rid`.** `FN-28`'s second operand looks like a fact about the disk
    and is not one the protocol can hold: `doRootNameTaken` lets the world
    occupy the free name and `doSwap` may then give what it put there the
    **quarantined root's own identity**, which at `2 RootId` is one step. So
    *absent* is read as *the task root — the identity this transaction pinned —
    is no longer at that name*, and the claim is stated as something Grove
    establishes and preserves rather than as something that holds. See *A tenth
    finding*.
  - **A user-supplied hook is one static atom and *has run* is one `var`
    boolean.** `FN-30` is a claim about whether a hook RAN, not about which one,
    what it did, or what it returned; `2 Hook` would say nothing `1 Hook` does
    not, so the slice adds no scope dimension. What the hook would MUTATE is
    deliberately not modelled — that is `FN-27`'s subject and the catalogue's
    reason for `FN-30` rather than its content.
  - **The internal commits are `CommitAttempt` and `Settle`, and there is no
    third.** `FN-30`'s *during an internal commit* is `internalCommitActs`; a
    protocol with a third would need the set to grow, and the check's second
    conjunct — no step of Grove's runs a hook at all — is what would keep the
    claim true meanwhile.
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
  `handoff-audit-k66`'s.
- ~~**The forward settle still passes through `FN-22.i`'s stable state and out of
  it in one step.**~~ **AN ABSTRACTION REMOVED, AND THE SECOND ONE A CLAIM HAS
  FORCED RATHER THAN A SLICE CHOSEN.** `FN-21.a` says disposal is *re-enterable
  from any interruption* and `EN-03` says there is no atomic recursive directory
  deletion, so a disposal that is one transition has no interruption point to be
  re-enterable from and the claim is unstateable. Disposal is now **three steps**
  — write the cleanup marker, remove what it authorises removing, retire the
  marker — and the forward settle is gone from `doSettle` altogether. The
  catalogue's stable state after the rename is still passed through; what has
  changed is that the protocol now stops there and proceeds under a document.
- **THE CLEANUP MARKER'S NAME IS NOT MODELLED, AND `cTarget` IS WHAT STANDS IN
  FOR IT.** The catalogue gives the marker a per-handle, per-attempt reserved
  name and this file has no filename grammar. What the reaper actually reads a
  name **for** is *which quarantine does this document authorise removing*, and
  that is `cTarget`; what it reads an attempt identity for is *can Grove prove
  this is its own*, and that is `cOwner` present or absent — `Slot.owner`'s trick
  applied to the artifact `FN-31.d` is about. `quarantine-k43` flagged the
  quarantine's missing name as **this slice's to widen if `FN-21.b`'s cleanup
  manifest needed it**; it did not need a filename, it needed a **target**, and
  one static field is what that cost.
- **`inTreeWitnessOwns` IS AN ABSTRACTION AND IT ERRS TOWARDS DECLINING.**
  `FN-21.b`'s *only when no matching in-tree witness owns them* distinguishes a
  `FINISHING-<handle>/` still standing in the task root from the one that rode
  into the quarantine with it. This file has **one** `Slot` and no filename
  grammar, so it cannot tell the two apart; the sweep reads *there is a task root
  present to hold a witness, that witness is published, and it names an attempt
  one of the markers names*. On the disk the rename leaves, the task root is
  absent and the sweep proceeds; where the world has put something back at the
  task-root name (`doRootNameTaken`), the sweep **declines** where the shipped
  protocol might proceed. That is the fail-closed direction, which is the one the
  catalogue requires, and it is why `FN-21.b`'s witness is a *decline* rather
  than a proceed. A model that needed the sweep to proceed there would need a
  second place a witness can be, the way `Quar` is a second place a root can be.
- **DISPOSAL'S CONTENT REMOVAL IS ONE STEP, AND `EN-03` SAYS IT IS NOT.** The
  shipped protocol removes the quarantine entry by entry, because there is no
  atomic recursive deletion; this file has one `Quar.qRid` and no filename
  grammar, so it cannot decompose the removal below the marker protocol's own two
  boundaries. What is modelled is that the removal is **marker-guarded** and
  **re-enterable**, which is what `FN-21.a` claims; a partial removal *within*
  the step is not. The two interruption points the model does have —
  `Disposing` and `Disposed` — are the two the marker exists to distinguish.
- **DISPOSAL'S TERMINAL STATE IS THE TWO NAMES DISPOSAL OWNS, NOT THE TREE.**
  Written as *quarantine gone, marker gone, and the reserved witness gone*,
  `FN-21.a` is false, and a counterexample said so: a sweep may retire a stale
  marker beside an **unrelated** preparing witness that is nobody's. The
  predicate reads `no Quar.qRid' and no Cleanup.present'`, and what the release
  of the artifacts *inside* the quarantine is worth is carried separately, by a
  conjunct stated over the removal step. Retained below.
- **THE SWEEP IS NOT IN `bodySteps` OR `txnActs`, AND THREE THINGS FOLLOW.** It
  takes no operator confirmation (`FN-01.a` is stated over `txnActs`, and
  collecting the garbage a crashed finish left is not a second finish); `FN-24.b`
  should not be asked of it, ~~though as written each firing has exactly one
  persistent effect~~ — **and that clause was wrong: the crash slice's effect
  enumeration shows the content-removal branch has two, exactly as `Dispose`
  does**; and `FN-22.a`'s *none is skipped* conjuncts, which quantify over
  `txnActs`, do not reach it — which is correct, because a sweep never had a
  disposition to revalidate. `FN-24.b` landed with that exclusion intact.
- **`tableAction`'s AFTER-RENAME `Committed` ROW NOW READS THE MARKER.** The
  catalogue's corrective action there is *complete: dispose (`FN-21`)* — it names
  another claim group rather than one move — and `FN-31` requires a **create**
  where the reserved name is free and a **replace** where it is not, as distinct
  transitions. So the row is a function of the marker as well as of the point and
  the observation, which is what `tableOutcome` has done since `revalidation-k44`
  for the occupied target and the unreproducible commit. It is still data and
  still total: delete the row and the function goes partial exactly as before.
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
  are all in `bodySteps` so that it can be asked of them. **It was**: `Settle`'s
  restore branch is one of the two steps `FN-24.b` declares.
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
  `handoff-audit-k66` should read it that way.
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

- **THE DIAGNOSIS IS READ IN THE STATE A BLOCK IS DECIDED IN, NOT IN THE STATE IT
  LEAVES.** Every `FN-25` and `FN-26` command reads `diagnosed` unprimed against
  a primed `Sys.res'`. That is forced rather than chosen and it is this slice's
  first counterexample: `doSettle`, `doRevalidate` and `doQuarReturn` all block
  through `txnGone`, so the state a block LANDS in has no attempt identity, no
  handle and no anchor left. `resultProven`'s first conjunct then reads
  `none in ticketedAttempts` and is **vacuously true**, `anchorHolds` is false,
  and every block classifies alike — greenly. An outcome is what an INVOCATION
  returns, and the invocation still holds its operands when it decides.
- **`BObserved` IS TRUE IN EVERY STATE OF THIS FILE.** `FN-26`'s diagnostic names
  four things and one of them — the observed topology — is `Repo.rev`, which is
  `one Rev`. This model has no unreadable repository, so that arm is a fact of
  the signature and not a claim. It is kept as an arm rather than deleted because
  three arms would read as the whole of the catalogue's sentence; it is named
  here and under *what a green run does not prove* so that it reads as declared
  rather than as demonstrated.
- **`FN-25`'s WITNESS-NAME ARM AND ITS CORRELATION CLAUSE ARE COMPLEMENTS, AND
  THE PARTITION HAS NO CONTENT THERE.** `dgWitnessNotProvablyThisAttempts` is the
  negation of `dgCorrelatedIncompleteAttempt`'s first three conjuncts, so at the
  reserved WITNESS name disjointness and exhaustiveness are true by construction.
  What carries `FN-25` is everything else: the marker, the quarantine, the
  topology clause, `dgUndigestibleEntry`, and the precedence that resolves the two
  places the arms meet. The widening that produced the complementarity is the
  slice's second counterexample and is retained below; the alternative — leaving
  the arm at `no Slot.owner` — left `FN-25.b` FALSE, which is worse than an arm
  with no margin.
- **THE PRECEDENCE IS A DESIGN DECISION AND NOT A READING OF THE CATALOGUE.**
  Nothing in §*Outcomes* says which diagnosis wins where both hold; this file
  chooses `OwnershipConflict`, on the fail-closed rule — *Grove never mutates what
  it cannot prove is its own* — and states it as a two-element relation so that
  deleting the edge leaves two survivors rather than silently reversing.
  `handoff-audit-k66` reads it as a proposal, not as a finding.

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

The catalogue does not state which of its **twenty** closed refusal reasons each
of `FN-05.a`'s seven members produces — it was seventeen when this table was
written, and none of the three added by `closed-set-additions-k74` is a preflight
question, so the choice below is unchanged. This file chose:

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
| **the repository has tracked the witness** | **no reason at all** — the stop is a `Blocked` — **see below** |
| **a finish that was rolled back** | `DeletionNotCommitted` — **see below** |

**Two members share one reason, and that is not a modelling shortcut.** `SY-03`
says a preflight is never a licence and every gate revalidates against its own
operands, which makes *layout unsupported* and *quarantine target unreachable*
the same question asked at two gates. What follows is that a reason cannot say
which member refused — hence `Sys.why` — and that the two are distinguishable to
an operator only by which gate reported. Whether the shipped diagnostic should
distinguish them is `handoff-audit-k66`'s, not this file's.

**A ROLLED-BACK FINISH HAD NO REFUSAL REASON EITHER, AND THE CATALOGUE NOW
CARRIES ONE.** The catalogue maps the `NotCommitted` disposition to *rolls back
and yields `Refused`*, and none of the **seventeen** closed reasons named it:
`NoTrackedDeletion` and `RootIdentityChanged` are each **false** of a transaction
whose fingerprint was fine and whose root never moved. Reporting it under one of
them would have been a lie the model could not be caught in, so `finish.als`
added one refusal atom of its own and recorded it here rather than smuggling it
in. `finish.qnt` added the same atom independently, under its own name.

**DISPOSED BY `closed-set-additions-k74`: the reason set gained
`DeletionNotCommitted`**, and both columns now spell the catalogue's member
(`RefDeletionNotCommitted` here, `RDeletionNotCommitted` there) rather than their
own. Two independently built families inventing the same missing member is the
catalogue being wrong, not a modelling convenience, and that agreement was the
strongest single piece of evidence on the whole disposition list.

**The pattern was not three accidents, and naming it was worth more than the
members.** Three findings of one kind stood in this scope — seven preconditions
against six reasons (entry 031), a tracked witness with no reason (entry 032),
and a rolled-back finish with no reason (this one) — and the lifecycle scope
produced two more. The disposition separates them rather than granting all five:
the **reason** vocabulary was answering the right question all along (*a reason
names the question asked, not the gate that asked it*, which is why `Sys.why` is
a modelling need and not a contract gap), while the **set** was drawn over one
scope's questions and swept by three. Three of the five were questions a later
scope asks and became members; one was the wrong half being fixed (see the next
section); and one was no gap at all. The record is
[`a-refusal-leaves-nothing-standing`](../../../docs/adr/a-refusal-leaves-nothing-standing.md).

**`FN-13`'s refusal had no reason in the closed set, and the wrong half was
being fixed.** `FN-13`'s stated witness was *a commit attempted while the witness
is tracked, **refused***, and none of the seventeen closed refusal reasons named
a tracked witness. This file reported it under `WitnessPending` — the closest
true statement the set admitted — and kept the case distinguishable through
`Sys.why` (`W8WitnessTracked`), exactly the device the two `LayoutUnsupported`
members already needed. Two exits were offered: add a reason, or restate the
outcome as a `Blocked` — which is what
[`task-tree-transactions-fail-closed`](../../../docs/adr/task-tree-transactions-fail-closed.md)
says happens ("a different revision, **tracked witness**, restoration failure …
keeps the witness unwalkable as Recovery pending"). The catalogue said *refused*;
the ADR said *blocked*; this model followed the catalogue, because the catalogue
is the sole input.

**DISPOSED BY `closed-set-additions-k74`: the outcome moved and the reason set
gained nothing.** The commit is attempted only after publication and evacuation
(`FN-11`), so an effect stands that the action can neither complete nor undo —
which is a `Blocked` by §*Outcomes*' own definitions, now stated as a
discriminator and checked as `FN-29.b`. The catalogue's `FN-13` witness is
corrected, `finish.qnt` reached the same answer independently, and the shipped
post-commit verification rejects a result that still tracks `.grove/` with the
evacuation already done. **A block carries a diagnosis rather than a reason**, so
`W8WitnessTracked` stays what it always was — a model-only observable naming the
branch — and no longer maps through `reasonOf`. Following the catalogue was the
right method against a wrong sentence, and this file's correction is the
sentence's, not the method's.

**The residue is real and has an owner.** An operator still cannot be told from
the outcome alone that the *repository*, not the filesystem, is what stopped the
attempt; the diagnosis names the class and the message must name the cause. That
is a diagnostic question and is `handoff-audit-k66`'s, beside the four it already
carries.

**The diagnosis half is a declared NARROWING of this column, not a `GAP` in the
runner's sense** — `FN-13` is answered here with a property and a witness, and a
`GAP` line would say the opposite. `BlockedOutcome` carries
no diagnosis here by the composition decision at its declaration — the
`RecoveryPending`/`OwnershipConflict` partition is `FN-25`'s — so *which* block
`FN-13` produces is answered in `finish.qnt`, whose `pickDiagnosis` classifies it
rather than naming it. The catalogue records `RecoveryPending`, on `FN-25`'s own
first arm: the artifact holding the transaction is this attempt's own published
witness, correlated by this handle and this attempt identity.

## What a green run of this file does not prove

- **Not that `FN-28`'s *branch, bookmark and worktree topology SHALL be
  unchanged* is checked at the grain the sentence uses.** This file has one
  `Repo.rev` and no branches, bookmarks or worktrees, so what conjunct (d)
  checks is *Grove moves recorded topology only at an internal commit*. An
  integration or a removal performed BY a commit — a merge commit, a
  `git branch -d` folded into one — is invisible to it. That is the same
  abstraction `FN-14` declares for the lanes' commit mechanisms, met at the
  other end of the protocol.
- **Not that `FN-29`'s second conjunct has an isolating mutation, and the reason
  is a fact about this file rather than about the claim.** *Every block leaves
  an artifact standing* is entailed here by `FN-22.a`'s third conjunct — *the
  witness is only ever released, on the rollback path, at the after-restoration
  point* — so a mutation that makes a block leave nothing kills `FN-22.a` before
  it reaches `FN-29`. Row 61 is aimed at the first conjunct instead and pairs
  with `FN-22.d` by construction. **A conjunct entailed by a neighbour's
  totality claim has no isolating mutation**, and that is a sixth way for a
  mutation to fail its aim — the first that is a property of the CHECK SET
  rather than of the model, of the claim's own data, or of the aim.
- **Not that `FN-30` says anything about what a hook would do.** The catalogue's
  reason — *such a hook may mutate unrelated working-tree bytes that no index
  image restores* — is the motivation and not the claim, and this file
  deliberately does not model the mutation: `World.hookRan` is disjoint from
  `unrelatedUnchanged`, so a green `FN-30` and a green `FN-27` are two
  independent facts rather than one stated twice. What is NOT checked is the
  implication between them, which would need a hook with an effect.
- **Not that `FN-23` covers a recovery that runs to a DIFFERENT terminal state
  from the one an uninterrupted run reaches.** What is checked is that recovery
  only removes and that its guard goes false, which together give *re-running
  reaches the same terminal state* for a cleanup. A recovery whose terminal
  state depended on WHEN it was interrupted would need `FN-21.a`'s third
  conjunct, which is the incumbent's own and is checked there.
- **Not that `FN-24.a`'s totality conjunct is doing any work.** `some
  classifiedRaw` is the claim's *classifies into a stable state* half, and with
  the arm set as it stands it is true by construction: `SAbsent` reads *no task
  root* and the three `Current` rows between them cover *some task root*, so
  every disk matches something. It is kept because a row added later with a
  narrower guard would break it and because its absence would be a silence rather
  than a red — but the conjuncts that carry `FN-24.a` are the **order** ones,
  and the mutation matrix's row 49 is aimed at those.
- **Not that all sixteen step boundaries are witnessed by one trace.** They are
  witnessed by sixteen, one per member of `bodySteps`, because a crash ends the
  transaction and a trace that reached them all would have to restart fifteen
  times. What the file demonstrates is that each boundary is individually
  reachable and that `FN_24a` quantifies over all of them; what it does not
  demonstrate is a single execution visiting every one.
- **Not that `FN-24.b` covers `Reap`, and the exclusion corrected a sentence of
  this file.** `bodySteps` deliberately excludes the sweep — a sweep is not a
  step of the transaction, takes no confirmation, and never had a disposition to
  revalidate — and the note on `doReap` has said since `disposal-k45` that *as
  written each firing has exactly one persistent effect*. **Writing the
  enumeration down shows that is false**: the content-removal branch clears the
  quarantine name and the reserved witness name together, exactly as `Dispose`
  does and for the same reason. The exclusion was right; the reason offered for
  it was one sentence too generous, and the sentence has been corrected in
  `finish.als`.
- **`EN-08`'s control now reaches every obligation the assumption table names,
  and `FN-31.c` — the one it did not — is where this file learned what a
  declaration is worth.** The table names `FN-09`, `FN-10`, `FN-24`, `FN-31.c`,
  `SY-12`, `TT-20` and `TT-23.b` as the witnesses that become unreachable when
  `crash` is removed. `FN-09.a`, `FN-09.b`, `FN-10.a` and all sixteen of
  `FN-24.a`'s always did; **`FN-31.c`'s two did not**, because both **posited**
  the disk an interruption leaves rather than running `crash` to reach it, so
  removing the action left them landing. That is a fact about this file's
  realisation rather than about the assumption, and it is exactly what an
  exercise-removal exists to make visible — it is invisible without one.
  **`routing-and-prose-k73` disposed it**: the catalogue's assumption table
  states that an exercise-removal row's controls column is a claim of
  unreachability to be established by RUNNING the removal, and names this as the
  case where the row is right and a family does not meet it.

  **`finish-scope-k71` then closed it by DECLARING the row unmeetable in this
  column, and the declaration was wrong.** Its argument was that reaching the
  interrupted replacement through `crash` means running the whole six-step body,
  the commit, the classification, the quarantine rename and disposal **from state
  0** before the crash — because `fact TransactionsStartWhereAProcessStarts`
  narrows `EN-11` for the transaction's volatile phase — at about seventeen
  states against this file's thirteen-state maximum, and hence that *a model that
  POSITS a disk under `EN-11` cannot also EXERCISE `EN-08` at that disk*.
  **The seventeen was estimated and never run.** `finish-scope-k75` said so, and
  `finish-scope-k76` ran it.

  **The measurement, and it kills the declaration twice over.** The run-up does
  not have to start from a pristine root: `EN-11`'s licence still covers the TREE
  at `Fresh`, so the honest run-up starts at `interruptedMidEvacuation` — the
  same posit `witness_FN_21a_the_interrupted_disposal_disk_is_reachable` and
  `witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves`
  already use — and runs the rename, the marker and the removal to the crash.
  **Those two commands were already in this file, at eleven and twelve states,
  reaching exactly the two disks the declaration said were out of reach.** And
  even from a genuinely pristine root the estimate was high: the mid-disposal
  disk is reached through `crash` in **fourteen states**, `Idle · Confirm ·
  TxnOpen · Preflight · WPrepare · WManifest · WReady · WPublish · WEvacuate ·
  CommitAttempt · Classify · QuarRename · MarkerCreate · Crash`, found in 8.9 s
  at a 16-step bound.

  | command | bound | result | wall |
  |---|---|---|---|
  | `witness_FN_31c_an_interruption_before_the_replacement_is_resumed` | 14 steps | instance found | 7 s |
  | `witness_FN_31c_an_interruption_after_the_replacement_is_resumed` | 14 steps | instance found | 6 s |
  | `expect_unreachable_EN_08_no_resumption_of_an_interrupted_disposal_is_reachable` | 14 steps | **no instance**, as the control requires | 6 s |
  | the same run over the earlier boundary with `crash` removed (measured, not kept — the later boundary subsumes its run-up) | 14 steps | **no instance** | 5 s |

  Same scope bounds as the rest of the file (`3 but 2 Device, 2 RootId, 2 Rev,
  3 Entry, 2 AttemptId, 2 Digest, 2 CMark`), Alloy 6 on the corretto-21 JVM the
  runner picks. **Both `FN-31.c` witnesses now run the protocol, so the row is
  MET in this column**, and the fourth `expect_unreachable_EN_08_*` command is
  what establishes it — a witness that depends on `crash` is a claim only the
  removal can check. The row is met in the Quint column too, by
  `wit_unreach_EN_08_an_interrupted_replacement_resumed`, so the cell is
  uncontested.

  **What this cost and what it bought.** It cost about 25 s of solver time and
  two rewritten witness preambles; the previous disposition cost nothing and
  asserted an impossibility. **The rule it leaves — now in the catalogue's
  assumption table — is that a family's failure to meet an exercise-removal row
  is established by running the deeper attempt, never by costing it in prose. A
  bound too dear to pay is a declared and MEASURED gap; an unpaid estimate is
  neither.**
- **Not that `FN-09.a`'s *exactly one rename* frames the whole tree.** Its
  `WPublish` branch asserts `Slot.owner`, `Slot.wHolds`, `rootSame`, `manSame`,
  `repoSame` and `worldSame` — and **not** the cleanup marker, which did not
  exist when the check was written. Mutation 50 is a publication that deletes a
  standing marker: it kills `FN-24.b` and leaves `FN-09.a` green. Two readings
  are available and the file takes the second: either `FN-09.a` should grow a
  `markSame`, or the marker is `FN-31`'s subject and `FN-09.a` should not
  describe it — which is `disposal-k45`'s fourth rule about aim, applied to a
  check rather than to a mutation. What follows is that *nothing else moves* in
  this file means *nothing else the claim's own frame names*, and `FN-24.b` is
  now the only check that quantifies over the whole disk at once.

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
- ~~**Not that a quarantine this file leaves behind can ever be cleaned up.**~~
  **ANSWERED, AND IT IS THE SECOND ENTRY EVER REMOVED FROM THIS LIST.**
  `witness_FN_19` leaves a complete quarantine over an absent task root, and
  `doTxnOpen` requires `some Root.rid`, so no *transaction* can be opened on that
  disk. The catalogue's answer is the **reaper**, and it is now in the file:
  `doReap` runs at `Txn.phase = Fresh` over anything at a reserved name, reads
  the cleanup marker for whether Grove can prove the entry is its own, and
  resumes the disposal in the order disposal runs in.
  `witness_FN_21a_a_disposal_interrupted_mid_disposal_and_resumed` is that
  resumption, at four states. **Both entries removed from this list were removed
  by a witness rather than by a property**, which is now twice in two slices.
- **Not that the sweep is safe to run concurrently with anything.** `doReap`
  fires at `Txn.phase = Fresh`, which in this file means *no transaction is
  running in this process*. Nothing here models a second process, a lease, or the
  file lock that `TT-22` is about, so what is checked is that a sweep run when
  Grove believes nothing is in flight touches only what it can prove is its own.
  The shipped reaper is **lease-owned** ([`finish-keeps-a-cleanup-layer-it-has-not-proved-forced`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md),
 
  `src/finish_cleanup/reaper.rs`) and that is exactly the machinery this file
  does not have. `SY-` is where the two would meet.
- **Not that the marker's own byte layout is crash-safe.** The catalogue lists
  *the marker-replacement protocol's own byte layout* under its deliberate
  omissions and requires only that the transition exist and be decided by
  reachability. `MarkerReplace` is one transition here and `EN-01` grants
  same-directory renames atomicity, so *the replacement is atomic* is an
  **assumption discharged by `EN-01`**, not a result. What the model does decide
  is that the transition is **reachable** and **distinct**, which is what Q3
  asked.
- **Not that `FN-21.b`'s in-tree-witness condition is checked at the grain the
  shipped protocol uses.** See *Abstractions*: the model errs towards declining,
  and a shipped reaper that proceeds where this one declines would satisfy every
  check here. The direction is the safe one, and that is all a green run says.
- **Not that disposal's removal is re-enterable *within* a step.** The two
  interruption points the model has are the marker protocol's; `EN-03` says the
  shipped removal has one per entry. A defect that needs a partial recursive
  deletion is outside what any green above says, and it is the sharpest thing
  no reader of this column should read this slice as covering.
- **Almost nothing about the lane, still.** Twelve more obligations have landed
  and **exactly one of them reads the lane**: `FN-17.a`'s *on a
  working-copy-as-commit lane the exact recorded preflight commit is reproduced*.
  Everything else — the anchor, the licence, the three dispositions, the scoping
  of the commit — is stated over a role the three lanes share, which is what the
  catalogue asks for and is *also* exactly what a lane-blind model would look
  like. `FN-15.b`, `FN-15.c` and `FN-15.d` each have a witness per lane, so the
  three are demonstrably *reachable* under all three; that is weaker than the
  claims *differing* under them. **`EN-16`'s collapse control is what separates
  the two, and `blocked-k48` ran it**: every `FN-` property stays green with the
  lane collapsed to `GitL`, and only the named witnesses stop landing. So "the
  lane is a model parameter" is now a measured fact about the commands and not a
  property of the signature — and *a lane-blind model passes every check in this
  file* is measured too, which is the half worth carrying forward.
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
  the sharpest thing no reader should read this green run as
  covering.
- ~~**Not that the step list is complete.**~~ **ANSWERED BY THE CRASH SLICE.**
  `FN-24.b` asks whether every step has at most one persistent effect and whether
  that effect is a same-directory rename or is itself decomposed; it quantifies
  over `bodySteps`, which this file has written as one named thing since
  `witness-k40` precisely so the question would have something to be asked of.
  Sixteen steps, fourteen with at most one effect and **two declared** with what
  decomposing them would take. What a green `FN_24b` still does not prove is that
  the step *list* is the complete set of crash boundaries — that is `FN-24`'s
  first premise, stated by the catalogue rather than checked by it, and no
  command in either family can establish it.
- **Nothing outside the bounds.** A successful bounded check is evidence about
  the stated bounds, not proof about arbitrary executions. With three entries,
  two devices, two attempt identities, two digests and ten states, a protocol
  defect that needs a fourth entry or an eleventh state is outside what any green
  above says.

- **Not that `FN-25.a` and `FN-25.b` have content at the reserved WITNESS name.**
  `dgWitnessNotProvablyThisAttempts` is the negation of the correlation clause, so
  there the two arms partition by construction. The claims' content is at the
  cleanup marker, at the quarantine, at `dgUndigestibleEntry`, at the topology
  clause's proviso, and in the precedence — all five of which a mutation reaches
  and the first of which does not.
- **Not that `FN-26`'s diagnostic names the observed topology as a matter of
  fact.** `BObserved` reads `some Repo.rev`, and `Repo.rev` is `one Rev`. Three
  of the four arms are claims about the state; the fourth is a property of the
  signature, and it is declared rather than demonstrated.
- **Not that `OwnershipConflict`'s SECOND clause ever fires alone.** `FN-25.b`'s
  second conjunct records, as a checked fact, that every state whose topology
  matches neither the anchor nor the result is also a state whose reserved witness
  the running attempt cannot prove is its own. A witness for the topology clause
  alone was sought at twelve states and does not exist. **That is a bounded
  unreachability and not a proof**: what it says is that within these bounds the
  catalogue's second `OwnershipConflict` clause is doing no independent work, and
  a slice that reaches one should expect a RED command rather than a silence.
- **Not that the precedence is the catalogue's.** Where both arms hold, this file
  returns `OwnershipConflict`. Nothing in §*Outcomes* decides that; the fail-closed
  rule is the argument, and `handoff-audit-k66` owns whether the shipped
  diagnostic adopts it.
- **Not that a block is always decided under a supported layout.** `World.lane`
  is `var` because `SY-03` requires a preflight never to be a licence, so the
  world can withdraw the layout between two of Grove's steps and the block that
  follows is decided with **no lane at all**. `FN-25.c`'s first attempted
  property asserted the opposite and was false at nine states; what the check
  states instead is lane-BLINDNESS — a step that changes the lane and nothing
  else moves no atom into or out of `diagnosed` — which is what makes the six
  per-lane witnesses mean *reachable on each lane* rather than *reachable three
  times*.
- **Not that `EN-16`'s control is a failure test.** It is an exercise-removal:
  the collapse makes the named witnesses INEXPRESSIBLE rather than false, exactly
  as `EN-02`'s single-device scope does, and every `FN-` property stays green
  under it. A lane-blind model passes this whole file, which is why the control
  exists and why a green run without it says nothing about the dimension.

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

Rows 49–50 are the crash slice's, and **both landed as first written — but row
49 is the second mutation that was tried**, and the first is a fourth way for a
mutation to fail its aim; see below.

Rows 51–54 are the blocked slice's, and **all four landed as first written after
one that did not** — row 54's first form freed the repository frame of
`doRecover`, and `doRecover` is not reachable within the bound from a blocked
state, because a block leaves `Txn.phase = Fresh` and `Recover` guards on
`Entered`. A mutation aimed at a transition the claim's antecedent cannot reach
after the antecedent holds reports exactly as a survivor does; the fix was to aim
it at `doQuarRename`'s own blocking branch, which is inside the antecedent rather
than after it. **That is a fifth way for a mutation to fail its aim, and it is
specific to claims stated ACROSS a state boundary**: *from a blocked state, no
Grove step rewrites history* has two reachable steps after it and a dozen that
are not. Every one of the four was also run against the other three; the *left
green* column is that sweep.

**`FN-25.a`'s CHECK WAS AN EXEMPTION AND IS NOW AN OBLIGATION, AND ROW 65 IS
WHAT MEASURES THE DIFFERENCE.** `finish-scope-k71` landed the catalogue's
precedence — *where both definitions hold of one disk, `OwnershipConflict` wins*
— beside an `FN-25.a` that still read *the two diagnoses are disjoint: no
blocked state satisfies both*. This file followed the document into the same
shape: `declaredDiagnosisOverlap` named the two reachable overlap classes and
`FN_25a` then **weakened itself by exempting them**, so the command tested least
exactly where the claim was hardest. `finish-scope-k75` found it; the repair is
in `finish-scope-k76`.

**What the exemption cost, measured rather than argued.** Reverse the precedence
relation — `earlierDiagnosis` becomes `DRecoveryPending -> DOwnershipConflict`,
so a disk carrying an artifact Grove cannot classify is reported as a recovery
the operator should go and run — and **the old check is green**. It is satisfied
by the declaration on one conjunct and by either winner on the other, so it did
not test the precedence at all. The repaired check is red on the same mutation
(row 65). The check now carries **four** conjuncts. Three are falsified by their own
mutation: `one diagnosed` by row 52, the precedence by row 65, and the floor — *a
correlated block with nothing unaccountable beside it carries `RecoveryPending`*
— by row 51.

**The fourth is the OLD clause, kept, and keeping it is a correction made against
this leaf's own first edit.** `lone diagnosedRaw or declaredDiagnosisOverlap` was
labelled with the disjointness the catalogue asserted, and that label was false —
so it was deleted with the label attached. Read as a check it says something else
and true: *the arms meet only where this file says they meet*. Without it a
THIRD overlap class appearing later would satisfy `one diagnosed`, fall outside
the precedence conjunct's antecedent, and pass in silence, because
`earlierDiagnosis` resolves any pair to one atom. **A clause can be load-bearing
for a claim other than the one it is labelled with**, which is the sibling of
this file's own rule about a clause rescued by its neighbour. **The floor is not decoration**: without it the
precedence conjunct would absorb row 51 and that mutation would report as a
survivor, so removing `dgTopologyUnmatched`'s proviso would have become
invisible in the same edit that made the precedence checkable.

**Two of the four kill a neighbour, and both neighbours are stated over the arm
the mutation edits.** Row 51 kills `FN-25.b` as well as its target because
`FN-25.b`'s second conjunct is a bounded-unreachability statement ABOUT
`dgTopologyUnmatched`, which is what row 51 edits; row 52 kills `FN-25.a` as well
because the arm it narrows is one of the two `declaredDiagnosisOverlap` names.
That is `disposal-k45`'s fourth rule — *when two obligations describe the same
artifact from two directions* — met for the first time where the shared artifact
is a PREDICATE rather than a disk object, and it is not a defect: a conjunct
written about a clause is supposed to move when the clause does.

Rows 42–48 are the disposal slice's, and **three of the seven did not land as
first written — two of them the same trap, met against a `fact` and against a
contradiction, and one a neighbour kill.** Every one of the seven was run against
the other six disposal checks and against `FN-22.a`, `FN-22.i`, `FN-03`, `FN-18`
and `FN-19`; the *left green* column is that sweep.

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
| 38 | `FN-22.g` | the block at `Classified` with `Indeterminate` reports `RefDeletionNotCommitted` — a block reported as a refusal, which is the collapse the catalogue names | `witness_FN_22f` | KILLED |
| 39 | `FN-22.h` | the incomplete return clears the quarantine while blocking — it reports the change and not the quarantine | `witness_FN_22g` | KILLED |
| 40 | `FN-22.i` | the forward settle frames the quarantine instead of disposing it — the artifacts go and the quarantine stays | `witness_FN_18` | KILLED |
| 41 | `FN-22.j` | the `Indeterminate` block at a point stops framing `Repo.rev` — it performs no handoff and moves the repository | `witness_FN_22d` | KILLED |
| 42 | `FN-21.a` | `doDispose` retires the cleanup marker in the same step that removes what it authorises removing — the evidence goes before the work | `witness_FN_18` | KILLED |
| 43 | `FN-21.b` | `reapable` drops `not inTreeWitnessOwns` — the sweep collects a quarantine whose in-tree witness still owns it | `witness_FN_21a_a_disposal_interrupted_mid_disposal_and_resumed` | KILLED |
| 44 | `FN-21.c` | the sweep's decline reports `Environmental` and no `why` — it passes over a foreign entry **silently**, mutating nothing and reporting nothing | `witness_FN_21a_a_disposal_interrupted_mid_disposal_and_resumed` | KILLED |
| 45 | `FN-31.a` | `doMarkerReplace`'s guard narrows to a **foreign** marker — an owned one is never superseded, so the replacement's source state is never acted on | `witness_FN_22i` | KILLED (the witness stops landing) |
| 46 | `FN-31.b` | the replacement adds its marker beside the one it supersedes — two stand at the reserved name | `witness_FN_22i` | KILLED |
| 47 | `FN-31.c` | the sweep retires the marker in the same firing that removes the quarantine — the two boundaries of the replacement stop being distinguishable | `witness_FN_31c_an_interruption_before_the_replacement_is_resumed` | KILLED |
| 48 | `FN-31.d` | the foreign document is **superseded anyway** while the attempt blocks — it reports the conflict and mutates the thing it could not prove is its own | `witness_FN_31a_a_source_state_from_which_disposal_must_replace_a_marker` | KILLED |
| 49 | `FN-24.a` | the classification order is the catalogue's table order taken **literally** — `Absent` first, the whole `Reserved` class after it | `witness_FN_24a_a_crash_after_the_quarantine_rename` | KILLED |
| 50 | `FN-24.b` | `doWPublish` drops `markSame` and removes a standing cleanup marker — the publication is still exactly one rename and now has two persistent effects | `witness_FN_09a_an_interruption_immediately_after_publication` | KILLED |
| 51 | `FN-25.a` | `dgTopologyUnmatched` drops its `not dgCorrelatedIncompleteAttempt` proviso — the catalogue's second `OwnershipConflict` clause taken literally | `witness_FN_25c_git_recovery_pending_reached` | KILLED (also kills `FN-25.b`; left green: `FN-25.c`, `FN-26`). **Re-run by `finish-scope-k76` against the repaired `FN_25a` and still KILLED — by the new THIRD conjunct rather than by the old first one.** Under the repair's second conjunct alone this mutation would have been absorbed by the precedence and reported as a survivor, which is why the floor clause exists |
| 52 | `FN-25.b` | `dgWitnessNotProvablyThisAttempts` narrows back to `no Slot.owner` — an owned witness whose manifest names another handle falls through both arms | `witness_FN_25b_a_block_whose_only_arm_is_ownership_conflict` | KILLED (also kills `FN-25.a`; left green: `FN-25.c`, `FN-26`) |
| 53 | `FN-25.c` | `dgUndigestibleEntry` is guarded on `World.lane in wcAsCommitLanes` — one clause of the partition reads the lane | `witness_FN_25c_git_ownership_conflict_reached` | KILLED (left green: `FN-25.a`, `FN-25.b`, `FN-26`) |
| 54 | `FN-26` | `doQuarRename`'s blocking branch drops `repoSame` for `Repo.wTracked' = Repo.wTracked` — the transition that blocks may write recorded history | `witness_FN_25c_git_recovery_pending_reached` | KILLED (left green: all three `FN-25` checks) |
| 55 | `FN-02` | `doSettle`'s cannot-reproduce block drops `slotSame` and discards the evacuated tree — the exit takes the finish leaf with it | `witness_FN_17b_a_restoration_that_cannot_reproduce_it_blocks` | KILLED (left green: all 25 others swept) |
| 56 | `FN-23` | `doReap`'s guard drops *there is something at a reserved name* — the sweep is enabled at every `Fresh` state, so it is available at its own terminal state | `witness_FN_21a_a_disposal_interrupted_mid_disposal_and_resumed` | KILLED (left green: all 25 others swept) |
| 57 | `FN-27.a` | `doQuarRename`'s **success branch** stops framing the world — the rename mutates unrelated working-copy bytes. Written as a substitution, not an addition: `worldSame` comes out of the common part and back into the three non-success branches | `witness_FN_18_a_proven_commit_reached_after_an_interruption_mid_evacuation` | KILLED (left green: all 25 others, `FN-27.b` and `FN-27.c` included) |
| 58 | `FN-27.b` | `doRevalidate`'s **completed refusal** stops framing the world, by the same substitution | `witness_FN_22d_a_rollback_that_completes_as_a_refusal` | KILLED (left green: all 25 others) |
| 59 | `FN-27.c` | `doSettle`'s **cannot-reproduce block** stops framing the world, by the same substitution | `witness_FN_17b_a_restoration_that_cannot_reproduce_it_blocks` | KILLED (left green: all 25 others) |
| 60 | `FN-28` | `doMarkerRemove` stops framing `Repo.rev` — disposal's last step performs an integration | `witness_FN_03_a_retry_with_no_local_trace_settling_forward_on_the_ticket_alone` | KILLED (also kills `FN-24.b`; left green: 24 others) |
| 61 | `FN-29` | `doRevalidate`'s completed refusal releases the **task root's own name** along with the witness — the refusal leaves no grove behind | `witness_FN_22a_the_point_after_the_restoration_is_reached` | KILLED (also kills `FN-19`, `FN-22.d`, `FN-24.b` and `FN-28`; left green: 21 others). **The third form tried — see below** |
| 62 | `FN-30` | `doCommitAttempt` stops framing the hook — the internal commit runs the operator's hooks | `witness_FN_14_unrelated_modified_work_present_across_a_successful_finish` | KILLED (left green: all 25 others, `FN-27.a` – `FN-27.c` included) |
| 63 | `FN-32` | `doDiscard`'s else-branch drops `slotSame` and clears the slot — the discard removes exactly what it just refused to classify | `witness_FN_32_a_transaction_step_meets_an_unprovable_artifact_and_it_stands` | KILLED. **`FN-21.c` survives**, which is the row's point: the two obligations are stated over different antecedents — a transaction step here, a sweep there — so neither mutation can kill the other, and `FN-32` is not a restatement of `FN-21.c` |
| 64 | `FN-32` | `doMarkerReplace`'s **foreign branch** stops framing the marker: the replacement still blocks and still carries `W17OwnershipConflict`, and supersedes the document it cannot prove is its own on its way out | `witness_FN_32_a_transaction_step_meets_an_unprovable_marker_and_it_stands` | KILLED. **Also kills `FN-31.d`**, and that is the finding rather than noise — the replacement is the ONLY `groveActs - Reap` member whose marker mutation is gated on ownership, so `FN-32`'s marker half and `FN-31.d`'s second conjunct have the same content at the same place and differ only in class. Left green: all 60 others, `FN-10.b`, `FN-21.b`, `FN-21.c` and `FN-27.a` – `FN-27.c` included. **Paired with row 63**: 63 kills `FN-32` through the witness slot and leaves the marker untouched, 64 through the marker and leaves the slot untouched, so the claim's two conjuncts are separately falsifiable — which is what `obligation-placement-k67` found row 63 alone could not establish |
| 65 | `FN-25.a` | **the precedence itself is reversed**: `earlierDiagnosis` becomes `DRecoveryPending -> DOwnershipConflict`, so correlation wins the declared overlap and a disk carrying an artifact Grove cannot classify is reported as a recovery the operator should run | `witness_FN_25a_a_correlated_attempt_at_a_name_grove_also_reserves_resolved_to_one` | KILLED. **THIS ROW IS THE MEASUREMENT `finish-scope-k76` EXISTS FOR.** Run against the OLD exempted `FN_25a` the same mutation is **GREEN** — `lone diagnosedRaw or declaredDiagnosisOverlap` is satisfied by the declaration and `one diagnosed` by either winner, so the check did not test the precedence the catalogue states, at all. Both runs are recorded above the matrix. Left green: `FN-25.b`, `FN-25.c`, `FN-26` |
| x1 | *(not an obligation's row — Q4-6's evidence)* | `reapable` narrows to *there is a quarantine* — the sweep with no document to read | *the kills are the fire evidence* | KILLED `FN-21.b` and `FN-21.c`; left green: all 26 others |
| x2 | *(not an obligation's row — Q4-2's evidence)* | `doSettle`'s restore branch drops `treeMatchesManifest` for `some Root.holds'` — the restoration puts back something the record did not name | `witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit` | KILLED `FN-02` and `FN-22.d`; **`FN-17.a` survives**; left green: 24 others |

**ROWS 55–62 ARE THE EXITS SLICE'S, AND SIX OF THE EIGHT ISOLATED CLEANLY ON A
TWENTY-SIX-CHECK SWEEP.** Every one of the eight was run against the other seven
and against `FN-01.a`, `FN-03`, `FN-14`, `FN-18`, `FN-19`, `FN-20`, `FN-21.a` –
`FN-21.c`, `FN-22.a`, `FN-22.d`, `FN-22.i`, `FN-22.j`, `FN-24.a`, `FN-24.b`,
`FN-25.a`, `FN-26` and `FN-31.b`; the *left green* column is that sweep and not
an assertion. Two did not isolate, and the two failures are the slice's largest
methodological results.

**ROW 60'S NEIGHBOUR IS `FN-24.b`, AND IT IS THE CRASH SLICE'S PREDICTION COMING
TRUE.** `FN-24.b` enumerates every persistent effect of every step; freeing
`Repo.rev'` at `doMarkerRemove` gives that step a second one — the marker and the
commit — so the enumeration goes red alongside `FN-28`. That is not a failure of
aim: **`FN-24.b` is the only check in the file that quantifies over the whole
disk at once, so any mutation that adds a persistent effect to any step kills it
by construction.** `crash-k47` recorded the shape from the other side (its own row
50 was swept against fifteen neighbours and left all fifteen green); this is the
first slice to meet it from a mutation that was not `FN-24.b`'s own. Expect it, do
not aim around it, and read it as a second confirmation rather than as noise.

**ROW 61 TOOK THREE FORMS AND THE THIRD IS THE ROW, AND WHAT THE THREE ESTABLISH
IS THAT `FN-29` HAS NO ISOLATING MUTATION IN THIS FILE.** That is a SIXTH way for
a mutation to fail its aim, and the first that is a property of the **check set**
rather than of the model, of the claim's own data, or of the aim:

> **A claim every one of whose conjuncts is another claim's subject has no
> isolating mutation.** The claim is still checked and its mutation is still a
> control; what it is not is a control *for that claim alone*, and the honest
> record is the neighbour list rather than a fourth attempt.

The three forms, because the two that failed are worth more than the one that
worked:

- **First form: the classification's own `Indeterminate` block releases every
  artifact it owns**, aimed squarely at `FN-29`'s second conjunct (*every block
  leaves an artifact standing*). It KILLED `FN-29` — and killed `FN-02` and
  `FN-22.a` with it. `FN-22.a`'s third conjunct is *the witness is only ever
  released, on the rollback path, at the after-restoration point*, so **a block
  that releases the witness anywhere else is a counterexample to `FN-22.a`
  before it is one to `FN-29`**. That is `revalidation-k44`'s fourth rule about
  aim — *when one obligation states a table's totality, an isolating mutation
  must aim at the column the totality claim does not carry* — and `FN-29`'s
  second conjunct has no such column: it IS the totality claim's column.
- **Second form: the completed refusal leaves the witness standing over the
  restored tree**, aimed at the first conjunct. It **SURVIVED**, and the reason
  is `fact BodyPhaseMatchesDisk`: the step produces `Txn.phase' = Settled` and
  the fact requires `Settled implies (no Slot.occ and manEmpty)`, so the mutated
  branch was **unsatisfiable**. That is row 42's trap met against the same fact
  for the second time — *a phase machine's own well-formedness facts are a
  mutation surface you did not choose* — and it reported exactly as a survivor,
  except that the fire-evidence witness stopped landing, which is what caught it.
  **Check the fire evidence even when the mutation looks like it landed**: an
  unsatisfiable branch and a working mutation differ only in the witness column.
- **Third form, and the row as run: the completed refusal releases the task
  root's own NAME.** It fires and it kills, and it kills four neighbours —
  `FN-19` (*a task root never simply disappears under a transaction step*),
  `FN-24.b` (a second persistent effect, again), `FN-22.d` (the pair `finish.als`
  documents beside the claim) and `FN-28` (its own conjunct (b), *the task root
  leaves its name only by the quarantine rename*). Four neighbours is what
  `FN-29` costs, and the neighbour list is the finding.

**AND ROW 61'S FIRST TWO FORMS ARE WHY `FN-29` IS WRITTEN AS IT IS.** The claim
could have been narrowed to its second conjunct alone — the half `FN-22.d` does
not carry — which would have made it isolating and would have dropped *leaves the
grove exactly as it was* on the floor. `finish.als` keeps both and records the
pair, because a claim's job is to say what the catalogue says and a mutation's job
is to be evidence about it, and only the second of those is negotiable.

**ROWS 57, 58 AND 59 ARE ONE MUTATION AT THREE SITES, AND THE SHAPE IS THE
FRAME-SUBSTITUTION RULE APPLIED BEFORE IT COULD BITE.** `worldSame` sits in the
COMMON part of all three transitions, so freeing `World.wcWork'` under it would
have been unsatisfiable (`entry-k39`'s trap) and freeing it for the whole
transition would have killed two of the three `FN-27` obligations at once
(`disposal-k45`'s third way to fail an aim). Each row therefore **substitutes**:
`worldSame` comes out of the common part, the two fields `FN-27` does not read
(`World.lane`, `World.hookRan`) are framed unconditionally, and `World.wcWork'`
is framed on every branch except the one the row is about. All three isolate.

**ROW 62 IS WHY `World.hookRan` IS NOT IN `unrelatedUnchanged`.** The mutation
that makes the internal commit run the operator's hooks kills `FN-30` and leaves
`FN-27.a`, `FN-27.b` and `FN-27.c` green, because the two claims read disjoint
fields. Had `FN-27` described the hook, this row and rows 57–59 would each have
killed the other's target and neither would have been evidence about anything.
**The fourth rule about aim is cheaper applied to a claim than to a mutation**,
and this is the first row in the file that demonstrates it prospectively rather
than after a neighbour kill.

**ROWS 49–50 ARE THE CRASH SLICE'S, AND BOTH LANDED AS FIRST WRITTEN — BUT ROW
49 IS THE SECOND THING THAT WAS TRIED, AND THE FIRST IS WORTH MORE THAN THE
ROW.** Row 49 was first written as *delete one pair from the precedence
relation* — `SReservedPublished -> SAbsent`, so that the post-rename disk would
match `Reserved(Published)` and `Absent` with neither classified before the
other. It **SURVIVED**. The reason is that the same disk also matches
`Reserved(Quarantined)`, and `SReservedQuarantined -> SAbsent` was still
standing, so `Absent` was dominated by a **neighbouring** pair the mutation had
not touched. That is a fourth way for a mutation to fail its aim, and it is the
first one in this file that is a property of the *claim's own data* rather than
of the model around it:

> **A mutation to one row of a total order is not a mutation to the order.** A
> precedence relation is transitively redundant by construction, so any single
> pair a disk's classification rests on has understudies. Mutate the ORDER — the
> whole ranking, as an alternative the claim is stated against — not one edge of
> it.

The row as run does exactly that: it restores §*States*' own table order and lets
the check say what is wrong with it. That makes the mutation and the finding the
same object, which is the cheapest form a control can take.

**BOTH ROWS WERE SWEPT AGAINST FIFTEEN NEIGHBOURING CHECKS AND LEFT ALL FIFTEEN
GREEN.** `FN-24.b`'s subject is the widest in the file — it quantifies over every
persistent component of the disk at once — so the *left green* column mattered
more here than anywhere: row 50 was run against `FN-09.a`, `FN-12.a`, `FN-11`,
`FN-19`, `FN-21.a`, `FN-21.b`, `FN-21.c`, `FN-22.a`, `FN-22.i`, `FN-31.a`,
`FN-31.b`, `FN-31.c`, `FN-31.d` and `FN-24.a`, and every one stayed green. Row
49 touches `earlierThan`, which `classified` alone reads and `FN-24.a` alone
reads, so it isolates by construction rather than by measurement — which is
worth saying plainly, because *isolates by construction* is a weaker fact than
*isolated by a sweep* and reads identically in a table.

**AND A SECOND MUTATION WAS RUN AGAINST `FN-24.a` THAT IS NOT A MATRIX ROW,
because it is evidence for a finding rather than a control for the claim.**
Removing the `SReservedQuarantined` arm from `classifiedRaw` — reverting to the
catalogue's six applicable rows — **KILLS** `FN-24.a`, on the disk a disposal
leaves when it has released its reserved witness with the quarantine still
standing: without the row that disk classifies `Current(Spent)`, an ordinary
spent grove. That is what makes the added row load-bearing rather than
decorative, and it is recorded under *A fifth finding* rather than in the matrix
because one mutation per obligation is the rule and row 49 is `FN-24.a`'s.

**ROW 45 IS THE ONLY MUTATION IN THIS FILE WHOSE KILL IS A WITNESS THAT STOPS
LANDING, AND THAT IS WHAT A REACHABILITY OBLIGATION'S CONTROL LOOKS LIKE.**
`FN-31.a` is answered by a witness, so the thing a mutation has to be able to
break is the witness. Narrowing `doMarkerReplace` to foreign markers leaves every
check in the file green — the protocol simply **stops** at `Quarantined` with an
owned marker standing, which is a liveness hole no safety check notices — and
`witness_FN_31a_a_source_state_from_which_disposal_must_replace_a_marker` stops
landing. Worth one line for whoever writes the next reachability-answered
obligation: **a mutation aimed at a witness must be able to make the trace
disappear, and a green suite under it is the expected result rather than a
survivor.** The runner reports the missing instance as a `FAIL`, which is exactly
right.

**AND `witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves`
SURVIVES ROW 45, WHICH IS THE POINT OF HAVING IT.** The mutation removes the
protocol's ability to *act on* a stale marker, not its ability to *produce* one.
Two commands, two different things: one says the source state is reachable, the
other says the transition that needs it exists. The finish questions' Q3 needs
both.

**THE DISPOSAL SLICE DECIDES NO `Q4` ROW EITHER, AND THE CLASS REGISTER IS WHY.**
`FN-21` and `FN-31` are both *incumbent mechanics* — the catalogue says so — so a
mutation that breaks one is evidence about the incumbent protocol and about
nothing else. **The cleanup marker's own removal-matrix row therefore cannot be
decided here, and the reason is the quarantine's reason at one remove**: rows 42
and 47 break `FN-21.a` and `FN-31.c`, both incumbent mechanics; rows 45, 46 and
48 break `FN-31.a`, `.b` and `.d`, likewise. The marker exists to make a
non-atomic removal resumable, and *disposal is resumable* is `FN-21`, which is
what Q1 asks about — so the shared-safety claim the marker's removal would break
first is not in this file. It is `FN-24`'s or `FN-27`'s.

**AND THE CRASH SLICE SETTLED HALF OF THAT: IT IS NOT `FN-24`'s.** `FN-24.b`'s
two multi-effect steps are `Dispose` and `doSettle`'s restore branch, both
declared abstractions of the incumbent's own machinery; `FN-24.a`'s only
mutation that reaches an artifact reaches the model's **classification** rather
than the protocol's. Neither obligation's mutations name the quarantine or the
marker as the thing whose removal breaks them, so **`FN-27` is the last
shared-safety claim that could decide either row**, and both stay recorded as
undecidable until the third child of `exits-k46` writes it. This note is what
the marker's row should read first.

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

**THREE OF THE SEVEN DID NOT LAND AS FIRST WRITTEN, AND TWO OF THEM ARE THE SAME
TRAP MET FROM TWO DIRECTIONS IN ONE SLICE.** All three reported exactly as a
surviving mutation does.

- **Row 42 was first written as `doMarkerRemove` enabled at `Disposing` as well
  as `Disposed`** — retire the marker before the removal it authorises. It
  **SURVIVED**, and the reason is `fact BodyPhaseMatchesDisk`: the step produces
  `Txn.phase' = Settled`, and the fact requires `Settled implies (no Slot.occ and
  manEmpty)`, which at `Disposing` is false because the witness and the manifest
  are still inside the quarantine. The mutated branch was **unsatisfiable**. That
  is row 34's trap — *a mutation the model cannot execute is not a control* — met
  against a `fact` for the second time, and it is worth separating from row 34's
  because the fact in question is one the file added for a completely unrelated
  reason two slices earlier. **A phase machine's own well-formedness facts are a
  mutation surface you did not choose**, and the cheapest check is to ask what
  the mutated step's *successor phase* is obliged to look like.
- **Row 47 was first written as the sweep keeping the quarantine while retiring
  the marker**, and the patch left the original `no Quar.qRid'` in place beside a
  new `quarSame`. Unsatisfiable, and it **SURVIVED** — the same trap in its
  original form, *a mutation added underneath a frame condition*, except that
  here the contradiction was with the conjunct the mutation was meant to replace.
  `entry-k39`'s rule is *remove the frame, do not contradict it*; the disposal
  slice adds the mechanical corollary: **a mutation stated as an addition when it
  should have been a substitution is unsatisfiable and reports green.**
- **Row 46 was first written with `FN-22.i` still asserting the marker's own
  content** — exactly one marker afterwards, naming this quarantine. The
  two-markers mutation killed `FN-31.b` **and** `FN-22.i`, and row 48's killed
  `FN-31.d` and `FN-22.i`. Both are neighbour kills, and the fix was not to the
  mutations: the conjunct came **out** of `FN_22i`. `FN-22.i` is about the
  corrective action and the stable state it is taken from; the document at the
  reserved name is `FN-31`'s subject. **When two obligations describe the same
  artifact from two directions, the one whose subject it is not should not
  describe it at all** — which is the fourth rule about aim, restated for
  overlapping *subjects* rather than for a table's totality.

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

**Twenty-one: four from the witness slice, one from the handoff slice, five from
the revalidation slice, TWO from the disposal slice, FIVE from the blocked slice
and FOUR from the exits slice — and twenty of the twenty-one are about the model
or the catalogue rather than about the protocol. The twenty-first is not, and it
is the tenth finding.** — which is itself the
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

**THE DISPOSAL SLICE ADDED TWO, AND BOTH ARE A CHECK WRITTEN WIDER THAN THE
CLAIM IT ANSWERS.** Five of the slice's seven checks were green as first written.

11. **`FN-21.a`'s terminal state, written over the tree, fails on an unrelated
    preparing witness.** `disposalTerminalNext` first read *the quarantine gone,
    the marker gone, **and the reserved witness gone***, which is what a
    completed disposal does leave. The counterexample is a sweep retiring a
    **stale marker** — Grove's own, its target already removed — while a
    `Slot.occ = Preparing` owned by nobody stands at the reserved name, having
    nothing to do with the disposal being resumed. The sweep is right and the
    predicate was wide: disposal's business is the quarantine and the document
    that authorises removing it, and an unpublished witness at the reserved name
    is `FN-10`'s subject. The predicate now reads `no Quar.qRid' and no
    Cleanup.present'`, and what the release of the artifacts *inside* the
    quarantine is worth is carried by a conjunct stated over the removal step.

    This is the `FN-03` over-statement's shape at a **third** grain, and the
    grain is new: `FN-03`'s was a claim over every actor where the catalogue said
    Grove's own steps; `FN-19`'s was a claim over the disk where the catalogue
    said the protocol; this one is a claim over **every artifact at a reserved
    name** where the catalogue said the ones disposal owns. **A claim about what
    a protocol leaves behind is not a claim about what else is lying around.**

12. **`FN-31.b`'s *one marker before and one after*, written over the pre-state,
    fails on a hand-edited pair.** Under `EN-11` as a free initial state, two
    markers at state 0 is a hand edit, and a replacement performed over them is a
    trace the model permits. The conjunct now constrains only the **primed**
    state, and *the reserved name never comes to hold two* is carried over the
    transition relation by the conjunct beside it. This is the witness slice's
    first retained counterexample at a **fifth** grain — a shape claim under a
    free initial state must be restated over the transition relation — and its
    value is that the rule keeps being violated by a conjunct that reads as
    obviously true, in a file whose header states the rule.

13. **A block classified in the state it LANDS in classifies everything alike,
    greenly.** The first form of every `FN-25` command read `diagnosed` in the
    post-state of `Sys.res = BlockedOutcome`. `doSettle`, `doRevalidate` and
    `doQuarReturn` block through `txnGone`, so that state carries no attempt
    identity — and `resultProven`'s `Txn.attempt in ticketedAttempts` then reads
    `none in ...`, which Alloy makes **vacuously true**. Every arm evaluated to
    the same thing and both `lone` and `one` held for the wrong reason. **This is
    a sixth grain of the vacuity rule this file already carries at five**, and it
    is the sharpest: the others are about a bound too small to reach a state, and
    this one is about a state that is REACHED and EMPTY. An outcome is what an
    invocation returns and the invocation still holds its operands when it
    decides; every command now reads the diagnosis unprimed against `Sys.res'`.
14. **A Grove-owned witness whose manifest names a different handle falls through
    both diagnoses.** With `OwnershipConflict`'s first clause modelled as the
    catalogue's printed example — *no owner* — `FN-25.b` is false at eleven
    states: a published witness with an owner, adopted by a transaction whose
    pinned handle the manifest does not name, is neither correlated nor
    unclassifiable. The catalogue's clause is the general sentence — *state is
    unrelated, ambiguous, or cannot be proved safe to mutate* — and the three
    items printed beneath it are examples. The arm was widened to *not provably
    THIS attempt's*, at the cost recorded under *Abstractions*.
15. **`OwnershipConflict`'s second clause, taken literally, holds at every block
    the catalogue's own table diagnoses `RecoveryPending`.** *The observed
    topology matches neither the recorded anchor nor the expected result* is
    `Indeterminate` written out, and §*Handoff and cleanup*'s ten-row table
    diagnoses three `Indeterminate` rows `RecoveryPending` by name. `FN-25.a` is
    then not nearly false but flatly so, at nine states. **This is a catalogue
    finding and it is the third of its kind in this scope**: the document fixes
    two closed definitions and states one condition under both, and only a table
    printed six hundred lines away disambiguates it. The proviso is that table's
    answer written once, and it is mutation 51.
16. **An entry of a type Grove refuses to touch reaches a CORRELATED block,
    because the seven preconditions are the entry surface's.** The second place
    the arms meet, and it was not foreseen: a recovery adopts a published witness
    and never re-runs the preflight, so a settle that restores a manifest
    recording an undigestible entry reaches a block with `dgUndigestibleEntry`
    true and the attempt perfectly correlated. `declaredDiagnosisOverlap` names
    both meeting places and nowhere else names either, so narrowing the check and
    declaring the overlap are one edit.
17. **A block can be decided with no supported layout selected, and `FN-25.c`'s
    obvious companion property is therefore false.** `World.lane` is `var`
    because `SY-03` requires a preflight never to be a licence; a
    `TopologyChange` between two of Grove's steps withdraws the layout and the
    block that follows carries a diagnosis under no lane. The property the check
    states instead is lane-blindness, which is what *each diagnosis on each lane*
    actually rests on.

**THE COMMIT SLICE ADDED NONE, and that is a fact about the slice rather than a
gap in it.** Twelve obligations, thirty-one commands, every check green as first
written and every one of the twelve mutations killed. What it did produce is two
*mutation* failures, recorded under the matrix above, and three findings that no
check could have reported: the missing refusal reason for a rolled-back finish,
the anchor's lane-blindness, and `Indeterminate` being reachable rather than
positively excluded.

**No command in any of the six slices has found a counterexample that was a
defect in the catalogue or in the shipped protocol.** Twelve retained
counterexamples, all twelve about the model's own licence rather than about the
finish process. The three catalogue-level findings this file carries — the
seven-preconditions/six-reasons mismatch (entry 031), `FN-13`'s missing refusal
reason (entry 032) and the rolled-back finish's missing reason (entry 033) —
were each found by trying to write down **what a branch returns**, never by a
check going red. That is now three times in one scope, and it is the strongest
methodological signal this family has produced: the instrument's value here has
been the discipline of totality, not the solver.

**THE EXITS SLICE ADDED FOUR, AND THREE OF THEM ARE ONE COUNTEREXAMPLE THIS
FILE HAS NOW MET FROM FOUR DIRECTIONS.**

18. **`FN-02`'s *the pinned handle is findable after an exit*, written as a
    presence, fails at state 0.** A hand-edited `Opened` with `Txn.handle`
    naming an entry the tree does not hold, and the preflight's own refusal is
    an exit that never had a leaf to leave. `fact TxnStateWellFormed` permits it
    because the handle is `set Entry` and only its `Fresh` direction is true by
    construction — the deliberate looseness recorded on the field, met from a
    direction its note did not anticipate. Restated as a **preservation** —
    *whatever of the handle was findable stays findable* — it is a claim about
    the transition relation, which is counterexample 1's rule applied rather
    than met again.
19. **`FN-28`'s conjunct (d), written as *some ticket landed*, fails on a commit
    whose handle is empty.** `commitLands` writes
    `Repo.tickets' = Repo.tickets + (Txn.handle -> Txn.attempt)`, and at
    `Txn.handle = none` the product is empty while `Repo.rev' != Repo.rev`
    still holds — a commit that moves recorded topology and correlates nothing.
    Same free initial state, different consequence: this one says something about
    the protocol as well as about the model, and it is the eleventh finding.
20. **`FN-29`'s *the refusal leaves no artifact*, written over the file's own
    `leftoverArtifact`, fails on a quarantine standing at state 0.** A quarantine
    no transaction created and the restoration path never touches survives the
    refusal, and the refusal reads as incomplete because of bytes that were never
    its. The restoration path owns the reserved witness and the manifest, so
    those are stated as absences and the quarantine is stated as **not
    created**. **A claim about what an outcome LEAVES must name the artifacts
    that outcome OWNS**, which is counterexample 1's rule at the granularity of a
    single claim's subject rather than of the whole disk.
21. **`FN-28`'s second operand — *the task root is absent* — fails on a `Swap`
    after a `RootNameTaken`, and this one is about the shipped protocol.** Three
    successive formulations were falsified by one trace: the world occupies the
    free task-root name and then swaps what it put there for the **quarantined
    root's own identity**. This is counterexample 5 — *`EN-11` as a transition;
    a claim about the protocol's shape has to be stated over the protocol's own
    steps* — met at the operand of a definition rather than at an invariant, and
    the consequence is a design constraint rather than a restatement: **a finish
    cannot report success by looking at the task-root name.** See *A tenth
    finding*.

## Q3, answered — and the enumeration it asked for

These are the four questions [`finish-keeps-a-cleanup-layer-it-has-not-proved-forced`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
settles; the sections below are the evidence it reads.

**Q3 asks: *is the marker-replacement sub-transaction reachable?  Enumerate the
states that require replacing rather than creating or removing a marker.*** The
Alloy side's answer is **yes, by witness, at ten states**, and the enumeration is
**one class rather than a list**:

> A cleanup marker left standing by a disposal that completed the removal it
> authorised and was interrupted before retiring it. The document is Grove's own
> and its target is gone. A sweep will collect it; a **new** attempt that reaches
> the after-rename point before the sweep does must supersede it with its own.

Three things about that answer are worth separating, because only the first is a
result and the other two are what make it evidence:

- **It is not an artefact of the encoding.** The stale marker exists because
  `doMarkerRemove` is disposal's **last** step, and it is last because `FN-21.a`
  requires the marker to outlive the work it authorises — a document that records
  that the removal has not happened cannot go before the removal. A protocol that
  retired the marker earlier would have no stale markers and no replacement, and
  it would also not be re-enterable. **Q3's answer falls out of Q1's machinery**,
  which is what makes it a finding about the incumbent protocol rather than about
  this file.
- **The source state is REACHED, not posited.**
  `witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves` runs
  the protocol from the disk an interruption mid-evacuation leaves, through the
  rename, the marker, and the removal, and crashes before the marker is retired —
  **twelve states**. Without it the answer would rest on a hand-edited disk that
  `EN-11` permits, which is precisely the debt `commit-k41` took on and
  `revalidation-k44` paid; this slice does not open a second one.
- **The bounded-unreachability branch was not taken, and no `defer` is
  recorded.** The catalogue offers three instruments and `handoff-k42`'s brief
  names `commit-k41`'s three-lane answer to `FN-15.d` as the bar. This is the
  witness branch, answered once rather than per lane, **because nothing in the
  replacement reads the lane** — `doMarkerReplace`, `doMarkerCreate`,
  `doMarkerRemove`, `doDispose` and `doReap` mention no `World.lane` and no
  `wcAsCommitLanes`, and the witness runs with the lane free, so the instance the
  solver returns is a statement about all three. That is weaker than three
  lane-pinned witnesses and it is what the claim supports; `EN-16`'s collapse
  control, which is what would make a lane-blind model visible, was run by
  `blocked-k48` and its named set was found **exact**.

**What `finish-verdicts-k65` should read this as, and what it should not.** The
Alloy family says the transition is **reachable under the incumbent protocol at
these bounds**, so *delete the replacement* is not available on this evidence.
It does **not** say the sub-transaction earns its 960 lines: the catalogue
explicitly omits the marker's byte layout, `EN-01` grants the atomicity the
replacement rests on rather than the model establishing it, and Q1's counterfactual
— disposal in place, under `relax_EN_03` — is **Quint's** and would remove the
quarantine, the marker and the replacement together. Q3 is answered *within* the
incumbent; Q1 is what could make the question moot.

## A fifth finding, and a sixth — both from writing a classification down

**THE CATALOGUE'S OWN CLASSIFICATION ORDER, TAKEN LITERALLY, CONTRADICTS THE
LOAD-BEARING PROPERTY STATED THREE PARAGRAPHS BELOW IT.** §*States* orders the
task-root states `Absent`, then the three `Reserved` classes, then
`PartialScaffold`, `Legacy`, `Foreign`, `Malformed` and the three `Current`
classes — and says beneath the table that **no transient state may be observable
as a different stable state**, instancing *a task root whose deletion is not yet
proven is never `Absent`*. The two are in tension exactly once a reserved name
can be occupied while the task-root name is **free**, which is a disk the finish
protocol creates — `doQuarRename` moves the whole task root into the quarantine
in one rename — and the task-tree scope never does. With `Absent` classified
first, the disk an interruption immediately after that rename leaves reads
`Absent`: the deletion is recorded in history and can be **undone** by the world
between two of Grove's steps (`doCommitMoves`), so *not yet proven* is reachable
there and not hypothetical.

`finish.als` therefore orders the whole `Reserved` class **before** `Absent`,
states each arm as the catalogue's row verbatim so that the order is what carries
the claim, and `FN-24.a`'s third conjunct is what catches the other choice —
mutation row 49 restores the catalogue's order and the check goes red.

**`finish-scope-k71` LANDED THE REORDER, and it rejected two one-word repairs
rather than one.** Qualifying the `Absent` ROW — *no task root, and nothing of
Grove's at a reserved name either* — is the same repair from the other end, and
so is narrowing the model's own **state vector** to stop classifying the
quarantine at all. Both make `FN-24.a`'s third conjunct true **by construction**,
so the departure this section records becomes invisible to the check that exists
to catch it. Both were tried and both were reverted; the second survived long
enough to reach the spec and both model files before the argument below broke it.

**What decided it is that the rename is not the end of the protocol.** `FN-22`'s
**fourth** revalidation point runs *after* the quarantine rename and two of its
three rows return the quarantine, so between the rename and that point the
task-root name is free and the disposition is unsettled. The shipped protocol has
the same shape — `proof.revalidate()` runs after `cleanup.handoff()` and a
failure calls `cleanup.restore()` (`src/finish_transaction.rs:1949-1969`). And
`SY-05.b` names `FN-19` by identifier for exactly this: *no trace exposes an
absent task root before the deletion is proven*, which is what makes `SY-05.a`'s
*a missing task root means start a new grove* sound. With `Absent` first, that
trace exists.

**The Quint column had departed the same way without declaring it** —
`finish.qnt`'s `classify` already put the two `Reserved` arms ahead of `RAbsent`
under a comment reading *in the catalogue's own order*, which was false of the
catalogue — so both families were on the right side of a document that was on the
wrong one, and only one of them knew.

**AND THE STATE TABLE HAS NO ROW FOR A DISK THE PROTOCOL ROUTINELY PRODUCES.**
A disposal that has released its reserved witness while its quarantine is still
standing — the task root present, nothing at the witness's name, Grove's own
quarantine holding a root — matches no row: it is not `Reserved`, because
`Reserved(Preparing)` and `Reserved(Published)` are about the witness; and every
`Current` row would call it an ordinary grove. `finish.als` added
**`Reserved(Quarantined)`** as a model-only member of the reserved class, which
the catalogue licensed in as many words (*`TT-18`/`TT-19` are stated over the
reserved CLASS rather than over its members so that removing one member changes
no claim*). It is **load-bearing rather than decorative**, and the evidence is a
mutation that is not a matrix row: remove the arm and `FN-24.a` goes red on
exactly that disk.

**`finish-scope-k71` LANDED IT INTO §*States* as the reserved class's fourth
member**, together with the reorder above — the two are one repair, and either
alone leaves the other's disk misclassified.

**The near-miss is worth as much as the landing, and it is recorded rather than
tidied away.** The disposition was provisionally decided the *other* way — that
the quarantine lives in the VCS control directory
(`.git/grove/FINISHED-<handle>-<attempt>`, `src/finish_transaction.rs:322-329`),
that §*States* classifies a task root and so reaches it in neither direction, and
that both of this file's findings were consequences of its own state vector. Two
arguments looked decisive and both were wrong:

- *The deletion is proven when the rename lands, so the post-rename disk is not
  an instance of the load-bearing property.* **False.** `FN-22`'s fourth
  revalidation point runs after the rename and two of its three rows return the
  quarantine; the shipped protocol runs `proof.revalidate()` after
  `cleanup.handoff()` and restores on failure (`src/finish_transaction.rs:1949-1969`).
- *The reserved class has only `TT-18` and `TT-19` over it and neither reaches a
  standing quarantine, so it is not a member.* **The premise is false**:
  `FN-24.a`'s third and fourth conjuncts are stated over *something of Grove's at
  a reserved name*, and `SY-05.b` names `FN-19` by identifier. And the inference
  is wrong: membership is intensional — `SY-06.b` reaches `PartialScaffold(Exact)`
  and must not complete `PartialScaffold(Ambiguous)`, and both are members.

**AND THE MEMBER'S MEANING WAS WRONG WHEN IT LANDED, WHICH IS A THIRD THING THE
EPISODE PRODUCED.** `finish-scope-k71` wrote the class sentence as *an artifact
at a name Grove reserves says a Grove transaction is INCOMPLETE*, on the strength
of the window between the rename and the fourth revalidation point. The window is
real and it is what the ORDER needs; it is not what the MEMBER means. The same
disk is reached on the other side of that point — a `Committed` returned
unchanged makes the finish `Applied` with the quarantine still standing, and the
shipped protocol returns success there even when disposal fails
(`src/finish_transaction.rs:1953-1974`) — so under that sentence one disk was a
proven success and evidence of an unfinished transaction at once.
`finish-scope-k75` found it; `finish-scope-k76` repaired the class sentence to
*says Grove has WORK OUTSTANDING at that name*. **The error is this scope's own
`FN-28` committed one section later**: a disposition read off the tree. Both
families now witness the coexistence rather than arguing it —
`witness_FN_28_a_success_whose_cleanup_is_still_outstanding` requires
`classified in reservedClass` beside `finishSucceeded`, written over the CLASS so
a later member changes no claim, and Quint's `successWithCleanupOutstanding`
records `isReserved(classify(w))` instead of the literal `true` it used to set,
which witnessed the branch rather than the state.

`TT-19` still does not reach the new member, and that is now recorded in
§*States* as a fact about `TT-19` rather than about the membership: a standing
quarantine's recovery is `FN-21`'s sweep, which refuses nothing. **The Quint
column had neither the member nor the order** and classified the post-rename disk
`RAbsent` and `EN-11`'s orphaned quarantine `Current(Spent)`; both are repaired
there, with `mutant_no_quarantined_state` and `mutant_absent_classified_first` as
the controls that kill them (rows 919 and 920).

Both findings are the same shape as the four above it — *the catalogue fixes a
closed set and the model reaches a member of it the set does not name* — and both
were produced by the same move, which is worth naming because it is cheap and
this corpus had not made it before: **write the classification down as data,
apart from every transition, and ask a check whether it is total and
unambiguous.** Six slices of this scope acted on the disk without ever stating
what the disk *is*.

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


## The partition is over `Blocked` outcomes and nothing else — including the reaper's

The catalogue warns about this by hand, and it is the one trap in `FN-25` that a
model can walk into without a check going red: *reading `OwnershipConflict` onto
a refusal would make the partition neither disjoint nor exhaustive over
anything*. This file has a live instance of the hazard rather than a hypothetical
one. **`W17OwnershipConflict` serves two gates and only one of them is a block**
— `FN-31.d`'s replacement decline, which is `Blocked`, and `FN-21.c`'s sweep
decline, which is a **`NoOp`** — so the `why` set's shape is not the outcome
set's, and a partition keyed on `Sys.why` rather than on `Sys.res` would have
absorbed a non-block.

Every `FN-25` command is therefore stated over `Sys.res' = BlockedOutcome` and
over nothing else, and `diagnosed` is never read against a `why`. **The reaper's
decline sits outside the claim entirely.** Two probes, run at thirteen and four
states rather than kept as commands, because the statement is about what the
claim RANGES over rather than about the protocol:

- `always (Sys.act' = Reap implies Sys.res' != BlockedOutcome)` — no
  counterexample. A sweep never blocks; `doReap`'s decline branch is `NoOp` and
  its success branch is `Applied`.
- `eventually (Sys.act' = Reap and Sys.res' = NoOp and Sys.why' =
  W17OwnershipConflict)` — reached at four states. The non-block that carries the
  diagnosis's name is real and is out of scope of `FN-25` by the outcome, not by
  an omission.

`TT-24.c` and `TT-24.d` were the other side of this, and `blocked-k48` judged
them one citation away. **`obligation-placement-k63` settled the placement and
found the citation was enough for one of the two.** `FN-21.c` carries
`TT-24.d` entire. `FN-25` does **not** carry `TT-24.c`, for the reason this very
section gives: it is stated over `Blocked` outcomes and nothing else, so it
fixes the diagnosis a block carries and never that the situation blocks. What
`TT-24.c` actually had that no `FN-` claim did was its shared-safety half, and
that is `FN-32`.

## A seventh finding, an eighth and a ninth — all three from writing one partition down

The corpus already carries the shape: *the catalogue fixes closed sets and never
states the map between them* (findings one to three), and *an over-stated check
does not fail, it removes states* (finding four). The blocked slice adds three,
and the first two are a new instance of the first shape at a grain that has not
appeared before — **the catalogue states one CONDITION under two closed names**,
rather than leaving a map unstated.

**SEVENTH. `OwnershipConflict`'s second clause is `Indeterminate`, and the
catalogue's own revalidation table diagnoses `Indeterminate` `RecoveryPending`.**
§*Outcomes* defines `OwnershipConflict` as *state is unrelated, ambiguous, or
cannot be proved safe to mutate*, and gives three examples; the second is *the
observed topology matches neither the recorded anchor nor the expected result*.
That is the classification's `Indeterminate`, written out — `Committed` is
`resultProven`, `NotCommitted` is the anchor intact with no result, and this is
the negation of both. §*Handoff and cleanup*'s ten-row table then produces three
`Blocked` rows for `Indeterminate` and names `RecoveryPending` on every one of
them. Read literally the two definitions are not a partition at all: every
`RecoveryPending` state the protocol reaches satisfies the other name's
definition. **The disambiguation exists and is six hundred lines away in a table
about something else**, and nothing in either place cross-references the other.
`FN-25.a` is red at nine states without a proviso naming which of the two wins,
and mutation 51 is that proviso removed. **`finish-scope-k71` landed the
proviso** into `OwnershipConflict`'s second instance: it applies when Grove
cannot correlate the state to its own attempt.

**EIGHTH. `RecoveryPending`'s third sentence is false of two of the table's own
rows.** *And the outcome cannot yet be proven either way* reads as a conjunct of
the definition. Two rows of the same table are blocks whose outcome IS proven —
*after restoration, `Committed` leaves the witness blocking the restored tree*,
and *after the rename, a return that cannot complete* — and both are diagnosed
`RecoveryPending`. Taken as a conjunct the sentence makes `FN-25.b` false on both.
This file reads it as the elaboration of the common case rather than as a
condition, and the load-bearing clause is *a correlated Grove-owned attempt is
INCOMPLETE*. **The seventh and the eighth are the same sentence read from two
sides**, which is what made them one finding and two edits: the sentence belongs
to neither definition alone. **`finish-scope-k71` moved it out of
`RecoveryPending`'s definition** and kept it as the elaboration of the common
case, which is how this file had already read it — **and it tightened the
argument while landing it.** Only ONE of the two rows is the catalogue's own
counterexample: the after-restoration `Committed` row settles to
*`Reserved(Published)` carrying `RecoveryPending`*, so the table itself diagnoses
a proven-outcome block `RecoveryPending`, and one such row is enough to make the
conjunct reading contradict the document. `FN-22.h`'s incomplete return is a
second instance in **this file's** reading and in `finish.qnt`'s, but the table
names no diagnosis on that row — so *both diagnosed `RecoveryPending` by the
table*, above, claims more than the table says. The finding stands on the first
row alone.

**NINTH. `OwnershipConflict`'s three printed examples are not exhaustive of its
own general clause, and the file needed the general clause.** *An artifact sits
at a name Grove reserves but Grove cannot classify it as its own* was modelled as
the shipped device — an owner absent — and left a Grove-OWNED artifact whose
manifest names another handle falling through both diagnoses. The general
sentence covers it (*cannot be proved safe to mutate*); the example does not.
What this costs is stated plainly under *Abstractions*: the widened arm is the
complement of the correlation clause, so `FN-25` has no content at the reserved
witness name and all of it elsewhere. **A closed set whose members are defined by
a general sentence plus examples is not a closed set until a model asks which of
the two it is**, and that question is the ninth finding rather than the answer.

### How the three were disposed, and the one that was not this scope's

`finish-scope-k71` landed all three into §*Outcomes* as **one** repair, because
all three are the same defect: **the partition was carried by the diagnoses'
illustrations rather than by their definitions.** The sentence left
`RecoveryPending`; `OwnershipConflict`'s second instance gained the correlation
proviso; and the instances are now declared **non-exhaustive** of the general
sentence, which is the ninth finding answered rather than restated — a
Grove-**owned** artifact whose manifest names another handle is caught by *cannot
be proved safe to mutate* and by no printed instance.

**A fourth edit followed that this slice had not asked for, and `FN-25.a` is why.**
Where a correlated incomplete attempt and an unclassifiable artifact at a
reserved name are both present, both definitions hold and disjointness was a
model's choice; this file returns `OwnershipConflict` on the fail-closed rule and
so does `finish.qnt`, independently. The catalogue now states the precedence —
*the outcome names the strongest thing Grove cannot account for* — because
without it `FN-25.a` is not a claim. The two places the arms meet are reachable
at nine states; both are witnessed.

**What was NOT absorbed is the product question**, and this file was right to
name it as the one that is not citation-sized: whether the **shipped diagnostic**
adopts the precedence, and the two names at all, is `handoff-audit-k66`'s, beside
the other four.

## A tenth finding, an eleventh and a twelfth — and the tenth is about the shipped protocol

The three slices of `exits-k46` produced nine findings between them and eight
were about the catalogue's own words or about this file's instrument. **The tenth
is not**: it is about what the shipped protocol can and cannot read.

**TENTH — `FN-28`'s second operand is not a state predicate, and a finish
therefore cannot report success by looking at the task-root name.** The claim
says *a finish succeeds exactly when the exact attempt-bound commit is proven and
the task root is absent*, and *absent* reads as a fact about the disk. It is a
fact the protocol cannot hold: after the quarantine rename the task-root name is
FREE, `doRootNameTaken` is the world occupying it, and `doSwap` may then give
what the world put there the **quarantined root's own identity** — at `2 RootId`
that is one further step. Three separate formulations of `FN-28`'s conjuncts (a)
and (c) were each falsified by exactly that trace and by nothing else.

What follows for the crates is a sentence, and it was the sharpest thing this
slice had to hand up: **the only durable evidence a finish succeeded is the
correlation ticket.** `finish-scope-k71` landed it in both places it belongs —
`FN-28`'s own text now states its operands over Grove's own steps, and the design
constraint is
[`success-is-proved-by-the-ticket-not-the-tree`](../../../docs/adr/success-is-proved-by-the-ticket-not-the-tree.md),
whose rejected alternative is the cheap one an implementer reaches for: decide
success with a `stat`. `FN-03` says the ticket *SHALL survive the
destruction of every artifact the transaction owns*; this adds that it must also
survive the RE-CREATION of one, because a name is not an artifact and the world
owns the namespace. A `grove finish` that decided success by stat-ing the task
root would report failure on a grove someone had simply started using again — and
would report success on one where the quarantine had been moved back over it. The
model states the operands as things Grove ESTABLISHES and PRESERVES, never as
things that hold, and `finish.als` records the reading beside `taskRootAbsent`.

This is the witness slice's first retained counterexample and the handoff slice's
fifth met a **fourth** time, and it is the first of the four whose consequence
lands on the shipped protocol rather than on the model: *a shape claim under
`EN-11` must be restated over the protocol's own steps* is now a rule this file
has paid for four times, and the fourth payment bought a design constraint.

**ELEVENTH — a commit whose handle is empty lands a revision and no ticket.**
`commitLands` writes `Repo.tickets' = Repo.tickets + (Txn.handle -> Txn.attempt)`,
and at `Txn.handle = none` that product is empty while `Repo.rev' != Repo.rev`
still holds. The state is a hand-edited `Opened` with no handle, which
`fact TxnStateWellFormed` permits because the handle is `set Entry` and only its
`Fresh` direction is true by construction — the deliberate looseness recorded on
the field, met from a direction its note did not anticipate. It killed `FN-28`'s
conjunct (d) in its first form (*some ticket landed*) and the fix was to state
the conjunct over the ACTION. What it says about the protocol is smaller than the
tenth finding but not nothing: **nothing in the transaction asserts, at commit
time, that the record it is about to write can correlate.** `FN-04` is stated
over a ticket that exists; a commit that writes no ticket at all is outside every
`FN-` claim in this file.

**TWELFTH — `FN-13` is *shared safety* in the catalogue's class register, and
this file's own commit-slice note calls it *incumbent mechanics*.** The register
reads `FN-01`–`FN-07`, `FN-13`–`FN-18`, `FN-20`, `FN-23`–`FN-30`, and `TT-24`;
the paragraph under *The mutation matrix* that introduces the inherited
removal-matrix rows says of rows 14 and 17 that *both of those are incumbent
mechanics claims and neither is yet an answer to Q4*. Row 14's is (`FN-11`);
**row 17's is not** (`FN-13`). The consequence is a row of Q4's matrix, and it is
recorded below rather than corrected in place, because the five inherited rows
are transcribed rather than re-derived and a silent edit would lose the fact that
the file disagreed with the register for three slices.

**SETTLED BY `finish-verdicts-k65`: THE REGISTER IS RIGHT AND THIS FILE'S NOTE
WAS WRONG. `FN-13` IS SHARED SAFETY.** Its role-form is *a transaction never
lands its own in-flight artifacts in the user's history, and blocks rather than
proceed if it would* — a property any admissible protocol must have, not a fact
about the incumbent's steps. That it names a concrete artifact decides nothing:
the register's own rule is that a shared-safety claim naming an artifact names
*the incumbent realisation of a role*, exactly as it does for the correlation
ticket, the manifest and the recorded anchor. It also sits with the two claims it
was corrected against — `FN-14` (the commit is scoped) and `FN-29.b` (a refusal
is complete) — both shared safety, and `finish-scope-k71`'s repair restated its
outcome as `Blocked(RecoveryPending)`, which is a statement about the outcome
discriminator rather than about a step list.

**The consequence for Q4-1 is that the row IS a Q4 answer**, where the
commit-slice note said neither of rows 14 and 17 yet was. Row 14's is still not
(`FN-11` is incumbent mechanics); row 17's is. What the row does **not** become
is evidence about a candidate protocol, and the reason is unchanged and is
already in the cell: row 17 mutates the **gate**, and under a *total* removal of
the reserved witness `FN-13` is satisfied vacuously. So Q4-1 reads *the reserved
witness protects the user, under any protocol that still has one* — which is what
it should have read all along, and which Quint's Q4-101 reaches independently by
a different obligation.

## A thirteenth finding and a fourteenth — both produced by landing a disposition

Neither was visible to either column before `finish-scope-k71` edited the
catalogue, and that is the point of recording them here: **a disposition is a
measurement too.** Both are the same shape as the fourth finding above — a check
stated over the wrong subject does not fail, it silently answers a different
claim — met once in each column.

**THIRTEENTH. `FN-20`'s subject is the COMMIT'S DISPOSITION, and the two columns
were green on two different claims.** The obligation is *no classification reads
the quarantine, or any other artifact the transaction owns, as evidence that a
finish **happened***. `finish.als` states it over `Txn.disp` and its witness is a
leftover present while the classification settles `NotCommitted`. `finish.qnt`
built a two-world instrument over the **task-root classification** — the same
question asked with the artifact and without it — which is *never OBSERVED*
rather than *never a RECEIPT*, strictly stronger, and green for as long as no row
of the state table read the quarantine.

`Reserved(Quarantined)` is exactly such a row, and the collision is instructive
rather than awkward: **the task-root classification MUST read the quarantine**,
because that row is how the contract says a finish is *incomplete*, while `FN-20`
forbids reading it as evidence a finish *completed*. Two opposite requirements on
one artifact, and only one of them is `FN-20`'s. The wide reading has a second
cost that settles it independently: it would also forbid `FN-21.b`'s reaper
reading its own cleanup marker, which `FN-21.b` requires. The Quint instrument
now compares the DISPOSITION with the artifact and without it, its witness reads
*a leftover standing while the evidence says no finish of this attempt happened*,
and row 915 still kills it. The catalogue's witness sentence says which reading it
meant, so the next model does not have to guess.

**FOURTEENTH. `doCommitAttempt`'s ordering guard returned an outcome, and the
closed set is over ACTIONS.** The step is enabled from `PublishedP` as well as
from `Evacuated` on purpose, so that `FN-11` has an antecedent an early attempt
could have satisfied rather than being true by construction; the branch that
gate took reported
`Refused(WitnessPending)` while the witness stood published and the root stood
part-evacuated. Two things were wrong and they are one thing. **The transaction
is still live at that branch** (`txnSame`) — nothing was returned to a caller,
and the next step may still complete or unwind it — so a completed-invocation
outcome there is the step/action confusion §*Outcomes*' discriminator forbids,
met from the inside. And it escaped `FN-29.b` only because that check's
antecedent names the **completed** evacuation, which is a fact about where the
line was drawn rather than evidence the branch was on the right side of it.

`closed-set-additions-k74` named the branch and routed it rather than deciding it
under cover of a disposition, which was right: deciding it needed the restoration
path's obligations. The answer is that the branch returns **nothing** — `NoOp`,
the result this file already gives a step that reports and mutates nothing — and
it gained its own `why`, `W18EvacuationIncomplete`, because it had been sharing
`W9SlotPending` with `doWPrepare`'s branch, where that name means a genuinely
different situation: Grove's own artifact already at the reserved witness name,
nothing mutated, the attempt over. **This is the second time the same confusion
has been found at the same step** — the first was `FN-13`, whose outcome moved to
a block — and it is the last branch there. The catalogue carries the general rule
beside *a guard wait is not an outcome*.

**`FN-11`'s own non-vacuity sentence had to move with the outcome, and
`finish-scope-k75` found that it had not.** Both `doCommitAttempt`'s branch
comment and the `FN_11` command still said `gateEvacuated` *refuses* the early
attempt and that the early attempt is a *reachable refusal* — behaviour the model
no longer has. The semantic decision is unaffected; the argument is restated at
the **step** grain by `finish-scope-k76`. What keeps `FN-11` off being true by
construction is that `doCommitAttempt` is **enabled** at `PublishedP`, so the
early step occurs and its guard returns `NoOp` where a different model would have
returned `Applied`; the evidence that the antecedent is satisfiable at all is
`witness_FN_11`'s applied-after-evacuation trace, which is retained unchanged.
**A shrinking antecedent is how a check becomes vacuous**, so the pair
(command, the reachability witness that keeps it non-vacuous) is what a later
reader should check rather than the prose.

## Q4 — the matrix, and three rows that read `none`

**The catalogue names the ten rows**, so the matrix is not this file's to size:
*the reserved witness, the evacuation manifest, its ready mark, the correlation
ticket, the quarantine, the cleanup marker, the replace transition, the index
image, the recorded anchor, the deletion fingerprint*. Each names the **first
shared-safety obligation** its removal breaks under the mutation discipline, or
`none`. Q4's own success criterion is *a row whose artifact or transition can be
removed without breaking any shared-safety claim*, so a `none` is an answer and
an obligation is evidence the artifact protects the user.

**Every row carries its family**, because both families record into this one
file and `models/run.sh` reads the rows per family — the same reason a `GAP`
line names its family. The runner asserts the matrix in both directions
(every catalogue-named artifact has a row; every row names an artifact the
catalogue names, cites an obligation the catalogue defines or `none` or is
declared `abstracted`, and carries an evidence citation that resolves), and the
row shape it reads is documented in its header under *Q4's removal matrix*.

**Every row carries its evidence class**, because the node brief's warning is
exact — *the matrix is prose, and a row naming the wrong first-broken obligation
reports identically to a right one*. `mutation` means a row of the matrix above
fired and killed the named check; `argument` means the claim's own text names the
artifact and no mutation in either direction was available; `abstracted` means
the artifact has no counterpart in this file.

| # | family | removable artifact or transition | first **shared-safety** obligation its removal breaks | evidence |
|---|---|---|---|---|
| Q4-1 | alloy | the **reserved witness**, as the thing every candidate committed tree excludes | `FN-13` | mutation — row 17. **See the twelfth finding**, now settled: the register is right, `FN-13` is shared safety, and this row is therefore a Q4 answer. Under *total* removal the claim is satisfied **vacuously**, which is `FN-20`'s shape and is why the row is evidence about the mutation discipline rather than about a candidate protocol |
| Q4-2 | alloy | the **evacuation manifest**, as the record a restoration restores *to* | `FN-02` | mutation — row x2, and **not the obligation the argument predicted**. A restore branch that stops matching the manifest was expected to kill `FN-17.a` (*a restoration matches the manifest*) and does not; it kills `FN-02` and `FN-22.d`. Read forward: without the record, the restoration cannot put the **finish leaf** back, and *intent persists as the finish leaf* is the first shared-safety claim that notices. `FN-17.a`'s first conjunct is guarded on entries actually returning and its second on the release, so a restoration that returns nothing slips beneath both — which is itself worth a line to whoever writes the Quint row |
| Q4-3 | alloy | its **ready mark** | `FN-10.a` — **incumbent mechanics, so not yet a Q4 answer** | mutation — row 12 |
| Q4-4 | alloy | the **correlation ticket**, as an *attempt-naming* record | `FN-04` | mutation — row 19, transcribed |
| Q4-5 | alloy | the **quarantine** | **`none`** | argument, and it is the row Q1 exists for — see below |
| Q4-6 | alloy | the **cleanup marker**, as the document a sweep reads to know what is Grove's | **`none`** | mutation — row x1, and the row's obligation was `TT-24.a` until `obligation-placement-k68` re-decided it; see below |
| Q4-7 | alloy | the **replace transition** (`doMarkerReplace`) | **`none`** | mutation — row 45, transcribed: narrowing the transition away leaves **every check in the file green** and stops one witness landing. A liveness hole no safety claim sees |
| Q4-8 | alloy | the **index image** | — | abstracted. The three lanes' anchors and commit mechanisms are carried as one `Repo.rev` role and the index image has no counterpart here; declared under *Abstractions* since `commit-k41`. The row exists so that a removable artifact is not silently unrowed |
| Q4-9 | alloy | the **recorded anchor**, as the rollback licence | `FN-16.a` | mutation — row 25, transcribed |
| Q4-10 | alloy | the **deletion fingerprint**, as the commit's scope | `FN-14` | mutation — row 20, transcribed |

**Q4-5 IS THE ROW `exits` WAS CUT TO WRITE, AND IT READS `none`.** Every earlier
slice recorded the quarantine's row as undecidable-from-here, each with the same
reason: its own mutations reached only *incumbent mechanics*, and the
shared-safety claims that could name it — `FN-24` and `FN-27` — did not yet
exist. Both now do, and neither names it:

- **`FN-20`** — survives vacuously. A protocol that leaves nothing behind
  satisfies *no artifact a transaction leaves behind is a receipt* by having no
  artifact. Recorded by `quarantine-k43`.
- **`FN-24`** — settled by `crash-k47`. `FN-24.b`'s two multi-effect steps are
  declared abstractions of the incumbent's own machinery, and `FN-24.a`'s only
  mutation that reaches an artifact reaches the model's classification. Removing
  the quarantine removes `SReservedQuarantined` and the disks that need it in the
  same stroke, so the partition stays total and disjoint over what is left.
- **`FN-27`** — this slice's, and it does not name the quarantine because
  `unrelatedUnchanged` names three fields of the WORKSPACE and nothing on the
  task root's side. Disposal-in-place frames all three exactly as the incumbent
  does; all three of row 57, 58 and 59's kills are frame removals at Grove's own
  steps, none of which is a quarantine.
- **`FN-28`** — this slice's, and its conjunct (b) *does* name `QuarRename`. That
  is the incumbent realisation of a role and the class register says so in as many
  words: *where a shared-safety claim names a concrete artifact, the artifact is
  the incumbent realisation of a role*. (b)'s role-form is **the task root leaves
  its own name only on a proven result**, which disposal-in-place satisfies by
  never moving it at all.
- **`FN-23`** and **`FN-29`** — this slice's. Monotone removal with a guard that
  goes false is satisfiable in place; the blocks that vanish with the rename take
  their own artifacts with them and the witness-side blocks are untouched.
- **`TT-24`** — *Grove never mutates what it cannot prove is its own*. The proof
  is the manifest and the document at the reserved name, neither of which is the
  quarantine.

So the incumbent quarantine handoff protects Grove and not the user, on this
family's evidence. **That is Q4's delete/replace criterion met for the first
time in either family, and it is evidence FOR Q1's candidate rather than a
decision of it** — Q1's own criterion additionally requires disposal-in-place to
be *checked* against every retained claim under `relax_EN_03`, with `FN-24`'s
witnesses reached at a bound no greater than the incumbent's. Nothing here runs
that candidate. `finish-verdicts-k65` reads the row; the Quint column owes its
own.

**Q4-6 READS `none`, AND THE ROW CITED `TT-24.a` UNTIL A REVIEW SHOWED THE
CITATION COULD NOT CARRY IT.** Row x1 strips `reapable` back to *there is a
quarantine* — the sweep with no document to read — and what dies is `FN-21.b` and
`FN-21.c`, both *incumbent mechanics*. `obligation-placement-k63` read past them
to the shared-safety sentence behind them and named `TT-24.a` — *no action
mutates an entry whose ownership it cannot prove* — on the strength of `TT-24`
sitting in the register's shared-safety list, and declared the row a cross-scope
citation.

**`obligation-placement-k67` found the citation invalid, and the argument is one
sentence: `TT-24.a` is not quantified over this mutation's action.** *Action* is
partitioned by the catalogue's vocabulary across five groups belonging to three
scopes, so an obligation stated over it is read over **its own scope's admitted
set**
([`obligations-follow-context-not-artifact`](../../../docs/adr/obligations-follow-context-not-artifact.md),
clause 4). `TT-24.a`'s commands quantify over the task-tree file's own
transitions and over nothing else; row x1 mutates the **quarantine reaper**.
A green over one action set is not evidence about another, and clause 2 says a
citation carries the cited obligation's declared narrowings — which the
prefix-local reading is. The universal reading is no rescue: under it `TT-24.a`
names two scopes above `TT-` and clause 1 moves it out of the task-tree scope
altogether, taking its coverage with it. Either way the row had no obligation.

**So the row reads `none`, and a `none` is an answer rather than a hole.** Q4's
criterion is *a row whose artifact or transition can be removed without breaking
any shared-safety claim*, and row x1 is exactly that measurement, executed
against the action set it changes: the reaper loses its document, and the only
things that die are two incumbent-mechanics obligations. Every other check in
the file — twenty-six of them — stayed green, `FN-27` (*nothing unrelated is
mutated, on any outcome*) included.

**`FN-32` does not rescue it either, and the exclusion is deliberate.** `FN-32`
is stated over `groveActs - Reap`: `Reap` is excluded precisely so that a
mutation aimed at the sweep cannot kill it and one aimed at a transaction step
cannot kill `FN-21.c`. The sweep's own side of fail-closed ownership is
`FN-21.c`, and `FN-21.c` is incumbent mechanics.

**THE CONSEQUENCE IS A FINDING FOR THE Q4 VERDICT RATHER THAN A DEFECT HERE: no
shared-safety obligation in this repository constrains the quarantine reaper's
OWNERSHIP PROOF.** `TT-24.a` covers the task-tree scope's admitted set, `FN-32`
covers a live transaction's steps and excludes `Reap` deliberately, and the
reaper's own fail-closed ownership is `FN-21.b`/`FN-21.c`, both incumbent
mechanics. The cleanup marker therefore joins
the quarantine (`Q4-5`) and the replace transition (`Q4-7`) as an artifact this
family's evidence says protects **Grove's own intermediate machinery** and not
the user — which is Q4's delete/replace criterion met a
second and third time. Whether that licenses removal is Q4's question and not
this file's; `finish-verdicts-k78` answered `defer`.

**THE SENTENCE ABOVE SAID *the reaper's actions* UNTIL `finish-verdicts-k78`'S
REVIEWER FALSIFIED IT, AND THE NARROWING MATTERS.** `fun groveActs: set Action {
txnActs + Decline + Discard + Reap }` ([`finish.als`](finish.als):5224), and
`FN-27` — a shared-safety claim, and one of Q4's own retained set — is quantified
over `groveActs` at all three of its checks (5501-5503, 5518-5520, 5529-5531); so
are `FN-28`'s third and fourth conjuncts and `FN-30`'s second. Row x1 left every
one of them green. **The claim set did look at the sweep, and the sweep passed.**
What it does not look at is whether the sweep can prove what it touches, which is
`FN-32`'s subject and exactly where `Reap` is excluded. The universal reading was
doing work the narrow one does not, and reading a `none` as *silent* rests on the
narrow one alone.

**AND IT DOES NOT REACH `Q4-7`, WHICH NEEDS ITS OWN REASON AND HAS A BETTER
ONE.** The replace transition is `MarkerReplace`, a `txnAct` inside `groveActs -
Reap` that `FN-32` does examine — indeed *"`MarkerReplace` is the only `groveActs
- Reap` member whose marker mutation is gated on ownership … so this is where the
claim has content"* ([`finish.als`](finish.als):5930). Removing it by row 45
therefore removes the only site at which `FN-32`'s marker half has content, so
the row's green is **a vacuity artifact of its own mutation** rather than a
finding that no claim protects the transition. That is why `Q4-7` is `defer`, and
it is family-local and checkable where the reaper argument is neither.

**The other two instances of the shape are gone rather than declared.**
`TT-24.c` and `TT-24.d` were stated over this scope's contexts and are retired;
they are `FN-32` and `FN-21.c`, both answered here.

### What the blocked slice decided, which was nothing (retained)

`FN-25` and `FN-26` are both **shared-safety** claims, so the question is fair
and the answer is no. Neither obligation names a removable artifact of the
incumbent protocol: `FN-25`'s subject is the diagnosis a block carries and
`FN-26`'s is recorded history, and the artifacts the partition READS — the
reserved witness, the manifest, the quarantine, the cleanup marker — are read
for their CORRELATION rather than for their presence. **The two undecided rows
stay undecided.** Removing the cleanup marker would take away one of the four
clauses `OwnershipConflict`'s first arm asks the question of, and
`witness_FN_25b_a_block_whose_only_arm_is_ownership_conflict` reaches the
diagnosis without a marker at all, so the diagnosis survives the removal; what a
markerless model would lose is `FN-31`, which is incumbent mechanics. The
quarantine's row is the same shape. `FN-27` — *nothing unrelated is mutated, on
any outcome* — remains the last shared-safety claim that could move either, and
it is the third child's.

## `FN-32` — `TT-24`'s second context, arriving from the other scope

`obligation-placement-k63` retired `TT-24.c` and `TT-24.d` and re-stated them
under `FN-` prefixes, because both name this crate's actions and `grove-finish`
depends on `grove-task-tree` rather than the reverse
([`obligations-follow-context-not-artifact`](../../../docs/adr/obligations-follow-context-not-artifact.md)).
`TT-24.d` needed no new modelling: `FN-21.c` already states the reaper's decline
and cites it. `TT-24.c` needed a claim, and writing it found two things.

**First, `TT-24.c` was false as literally worded, and this file is where it is
false.** It said the entry met inside a transaction returns
`Blocked(OwnershipConflict)`. `FN_10b_content_the_discard_cannot_classify_fails_closed`
requires `Sys.res' in Refused` at exactly that situation, and is green. The Quint
column **blocks** at the same step (`SRemoveWitness`'s discard branch calls
`blockNow(…, "foreign")`), and is also green — both against `FN-10.b`, whose text
says only *fails closed*. Two independent readers resolved one sentence in
opposite directions, which is stronger evidence that the catalogue is
underdetermined than either column's own account. **The outcome is therefore not
stated in `FN-32`.**

**DECIDED BY `closed-set-additions-k74`, AND THIS COLUMN WAS RIGHT.** The
catalogue's second row now fixes a **function of the step** rather than one
outcome: a recovery meeting unclassifiable content at the witness slot has
applied nothing, so the tree it hands back is the tree it received and the
outcome is `Refused(ReservedNameOccupied)` — a member the closed set has carried
since before either model existed. A later step, over an evacuation that stands,
blocks `OwnershipConflict`. *Inside a transaction* was never the discriminator;
what the action has left standing is, and `FN-29.b` now checks it. `FN_10b`'s
`Sys.res' in Refused` is unchanged and is now the catalogue's answer rather than
a reading of it. See
[`a-refusal-leaves-nothing-standing`](../../../docs/adr/a-refusal-leaves-nothing-standing.md),
clause 1.

**Second, the shared-safety half had no shared-safety home.** *Nothing at a
reserved name Grove cannot prove is its own is mutated by a transaction step* is
proven in this file twice — `FN_10b`'s `treeSame` and `FN_31d`'s
`(Sys.act' in disposalSteps and markerForeign) implies markSame` — and **both
claims are incumbent mechanics**, so neither is evidence about a candidate
protocol. Q1 retains `TT-24`; `TT-24.a` survives, read over the **task-tree
scope's admitted actions**, and this crate's own transaction steps are not in
that set. `FN-32` is that home, classed **shared safety**.

**Two reserved names, and the quarantine is deliberately not one of them.**
A witness slot occupied with no owner, and a cleanup marker with no `cOwner`,
are the two artifacts here that carry an ownership bit.
`foreignAtReservedName`'s second arm — *a quarantine no marker authorises* — is
true on the ordinary forward path between `QuarRename` and `MarkerCreate`, so
the quarantine has no ownership bit of its own and its case is the reaper's.

**Quint's SLOT witness is the only library-level `wit_` in this column, and that
is declared rather than accidental.** (Its marker witness is a scenario one, for
the ordinary reason — see below.) Every other Quint witness here lives in a
`scenario_` instance in `finish-controls.qnt`, because a narrowed environment is
what makes it reachable in 8000 samples. `wit_FN_32` lands in `base` — the
general environment, no narrowing — in **322 of 8000 traces (4.03%)**, which is
stronger evidence than a scenario-reached witness: the situation arises in the
ordinary search rather than only under a contrivance. The cost is that any future
plain instance inherits it and must reach it too.

**And Quint's witness was worded on the contested half, so it moved.**
`hist.transactionBlockedReserved` was set where that column **blocked**, and the
Alloy column refused at the same step. The disposition settled the witness-slot
step as a **refusal**, so `finish.qnt`'s discard branch now routes through
`stopReserved` and `wit_FN_32` reads **both** arms —
`transactionBlockedReserved or transactionRefusedReserved`. That is the repair
rather than a rename: a witness that lands only on a block was reading the
contested outcome into a claim that never carried it. `FN-32` itself did not
move, because it says only that the artifact stands, which both columns agreed on
throughout — which is exactly why it survived the question and `FN-29.b` had to
be written to answer it.

**`Reap` is excluded from the antecedent, and mutation 63 is why that matters.**
Dropping `slotSame` from `doDiscard`'s else-branch kills `FN-32` and leaves
`FN-21.c` green; row x1's mutation kills `FN-21.b` and `FN-21.c` and leaves
`FN-32` green. Neither obligation is a restatement of the other, and the two
columns record it the same way: Quint's `inv_FN_32` reads flags set only at a
transaction step, while `FN-21.c` reads `hist.foreignMutated`, which the sweep
also sets. One flag for both would have made each mutation kill the other.

### `FN-32` NAMES TWO ARTIFACTS AND WAS CONTROLLED ON ONE, WHICH IS WHAT `obligation-placement-k67` FOUND

Everything above was true and none of it was evidence about the **cleanup
marker**. The claim names two reserved names; mutation 63 removes `slotSame`
from `doDiscard`, so it kills the first conjunct and says nothing about the
second. The Alloy witness put both artifacts beside a `Discard` — and `doDiscard`
frames the marker with an **unconditional** `markSame`, so the marker stood there
because no step in the trace could move it, not because an ownership gate held. A
witness whose relevant step cannot mutate its subject is evidence about the
framing. Quint was worse: `inv_FN_32` read `hist.mutatedUnproven`, an **aggregate**
flag set by three call sites, so it was character-for-character `inv_FN_10b` and
covered the quarantine name the catalogue explicitly excludes; its control
(`mutant_unproven_ownership`) flips one global `OWNERSHIP_PROVEN` dial, so the
kill it recorded could have come entirely through the slot.

**Both families could therefore have stayed green with the marker half
tautological**, and no run of either suite could have said so — the failure mode
entry 046's ledger names, one level down. Four repairs, and the second is the
measurement.

**1. The Quint invariant now names its two artifacts.** `noteUnprovenSlot` and
`noteUnprovenMarker` set `mutatedUnprovenSlot` / `mutatedUnprovenMarker` beside
the aggregate, and `inv_FN_32` reads `not(mutatedUnprovenSlot) and
not(mutatedUnprovenMarker)`. That is the catalogue's `FN-32` exactly — the
quarantine name, which carries no ownership bit, is no longer swept in — and it
is no longer a copy of `inv_FN_10b`.

**2. The marker gate is reachable and load-bearing, and this is the run that
says so.** `mutant_unproven_ownership`'s environment
(`ENV_PHASES = Set(0, 1, 2)`, `ENV_FOREIGN = Set(0)`) never reaches a foreign
cleanup marker at all, so its kill was always the slot's. `mutant_unproven_marker`
is `scenario_foreign_marker` — the one environment that demonstrably reaches that
state — with `OWNERSHIP_PROVEN = false`, and
`inv_fail_MUT_FN_32_a_transaction_mutates_a_marker_it_cannot_prove` is
**violated** in it. Had the marker half been vacuous the control would have
*held*, and an `inv_fail_` that holds turns the runner red. `mutant_unproven_ownership`
keeps the slot's half, now stated over the slot flag so it cannot borrow the
marker's.

**3. The Alloy witness for the marker sits at a step that could mutate it.**
`witness_FN_32_a_transaction_step_meets_an_unprovable_marker_and_it_stands`
reaches a `MarkerReplace` meeting a marker with no `cOwner`, which is the only
`groveActs - Reap` member whose marker mutation is gated on ownership — every
other step frames the marker unconditionally. Quint's counterpart is
`wit_FN_32_a_transaction_step_meets_an_unprovable_marker` in
`scenario_foreign_marker`, witnessed in **1659 of 8000 traces (20.74%)**, the same
state and count as `wit_FN_31d`.

**4. An isolating Alloy mutation, row 64.** It is the marker's mutation 63: the
replacement still blocks and still carries `W17OwnershipConflict`, and supersedes
the foreign document on its way out. It killed `FN-32` and `FN-31.d` and nothing
else. **Its *left green* sweep is every `check` command in the file — 62, not a
named subset** — because the earlier slice sweeps ran twenty-six and this claim's
neighbours are spread across the discard, disposal and reaper slices; the row
records the whole set rather than asking a reader to trust the choice of
twenty-six.

**All four are green in a whole-repository run against final files**, recorded in
`models/README.md`: 794 commands, `256 complete, 0 declared gaps, 0 empty, of
256`, Q4 matrix asserted in both families, exit 0.

**THE HONEST RESIDUE, AND IT IS A FINDING RATHER THAN A REPAIR.** The one
reachable site where a non-`Reap` step reads the marker's ownership bit is the
replacement, which is `FN-31.d`'s — so `FN-32`'s marker half and `FN-31.d`'s
second conjunct have the same content at the same place, and mutation 64 kills
both. The difference is entirely the **class**: `FN-31.d` is incumbent mechanics
and cannot be evidence about a candidate protocol; `FN-32` is shared safety and
can. That is the argument the claim was created on, and it is now the argument it
rests on for this artifact — not an independent second proof.

## Whether `models/run.sh` grows a matrix reader — decided: yes, narrowly, and it is a leaf

The catalogue calls the matrix *a runner obligation like any other: a removable
artifact with no row fails the run*. The decision is **yes**, and what a reader
can and cannot do is the whole of it.

**What it can check, and it is worth checking.** The catalogue NAMES the ten
removable artifacts, in one sentence, in one place — so the row set is derivable
from the catalogue exactly as the obligation manifest is, which is the runner's
founding principle (*the catalogue IS the manifest*). A reader can therefore
assert, per family: every named artifact has a row; every row names an artifact
the catalogue names; and every row's cited obligation is one the catalogue
defines, or `none`, or a declared `abstracted`. That is the same two-direction
assertion the coverage matrix already makes about commands, at a second grain,
and it catches a typo'd identifier, an invented obligation, an artifact the
catalogue added with no row, and a matrix that silently disappeared when a
README was rewritten.

**What it cannot check, stated plainly.** A row naming the wrong *but real*
obligation reports identically to a right one. No runner can decide *first
broken*; that is what the mutation discipline is for.

**So what reaches the content is a citation discipline, instituted here and
carried by the table above.** Every row carries an evidence class and a pointer:
`mutation` cites a numbered row of the mutation matrix, which carries its own
fire-evidence and its own *left green* sweep; `argument` cites the claim's text
and says no mutation was available; `abstracted` cites the *Abstractions* entry.
A reader can check that the citation RESOLVES — that row 19 exists, that the
obligation it names is the one the row claims — which is one more direction the
same reader gets for free. What is left unmechanised is whether the cited
mutation is the *first* broken, and that is bounded rather than open: a row whose
mutation killed exactly one check, with the *left green* sweep recorded, has said
as much as this instrument can say.

**No `review-prototype` is cut**, and the node brief's condition is what decides
it: the exception it names is *if nothing reaches the matrix*. Something does —
seven of the ten rows cite a mutation with fire-evidence and a neighbour sweep,
one is `abstracted`, and the two that rest on argument (`Q4-2`, `Q4-5`) each
enumerate the claims they considered rather than asserting a conclusion, which is
the shape a reviewer would have to read anyway. **A review did come, from the
other direction, and it moved a row**: `obligation-placement-k67` read `Q4-6`'s
cross-scope citation rather than its mutation and found the cited obligation
quantified over a different action set, which is the exact failure mode this
paragraph called bounded and is not caught by any *left green* sweep. The row now
reads `none`. The residual risk is the one
stated above and it is the same risk the mutation matrix has carried since
`entry-k39`.

The reader is **a leaf, not inline work**: it touches `models/run.sh`, which is
shared by four scopes and two families, and it must land before the Quint column
closes and owes its own matrix. Cut as `matrix-reader`, inserted ahead of
`quint-models-k10`.


# The Quint column — `finish.qnt`

Written from `docs/specs/semantic-contract.md`
alone, under the independence protocol
([`docs/formalism-findings.md`](../../../docs/formalism-findings.md), *Experiment 2 —
pre-registration*, **Independence protocol**): this column's session opened no
`.als` file, no Alloy section of a model-directory `README.md` and no experiment
entry from the Alloy column's range. Everything below is what a second family
reached on its own.

## Run line

```sh
models/run.sh --scope finish --family quint
```

Two files: [`finish.qnt`](finish.qnt) carries the parameterised library, the
`base` instance and the `verify_small` model-checking instance;
[`finish-controls.qnt`](finish-controls.qnt) carries the fifteen focused
scenarios, the seven assumption mutations and the seventeen model mutations. The
split is not tidiness — see *Verification*.

Coverage is asserted: all 63 `FN-` obligations are answered by a property
command and at least one witness, and there are no declared gaps.

Knobs, all environment variables read by the retired runner `models/run.sh`:
`QUINT_SAMPLES` (default 8000), `QUINT_STEPS` (default 24), `QUINT_SEED` (a
fixed default, so a green run is replayable and a red one reproduces),
`QUINT_VERIFY` (default 0 — see below).

## Verification

- **VERIFY** quint (bounded) — model checking is REACHABLE and AFFORDABLE at the
  runner's own default depth, and this is the strongest statement this column
  makes. `quint verify` (Apalache 0.56.1) checks the `verify_small` instance —
  one lane, one evacuable entry, one environment action per trace — over **all
  61 property commands at `--max-steps=4` in 445s** on a 16-core host with
  `-Xmx16G`, reporting no violation; `--max-steps=3` over three of the hardest
  (`FN-24.a`, `FN-25.a`, `FN-25.b`) finishes in 373s. Depth is what costs, not
  the number of invariants. That is the direction simulation cannot give — no
  counterexample *exists* within the depth, rather than none was sampled — and
  the task-tree column could not reach it at all, which is a difference between
  the two subjects rather than between the two sessions: this model has no tree
  walk to unroll.
  **What it is not is a green over `base`'s world, and the gap is the honest
  reading.** `verify_small` is smaller in every dimension the model has, and
  **four steps is not a transaction**: the incumbent protocol's shortest path
  from entry to a settled refusal is eleven steps, so a depth-4 check reaches
  the published witness and stops. It verifies the beginning of the transaction
  and says nothing about the commit, either handoff, disposal, or recovery.
  Quoting "model-checked, no counterexample" without the depth beside it would
  be the pre-registration's *scope trap* stated as a result. **Every `FN-`
  property from the commit onward is therefore established by bounded randomized
  simulation alone.**
  What would change the answer: splitting the ghost record out of the state — 94
  fields of write-only bookkeeping that a symbolic backend must nonetheless
  encode at every step — which is a modelling change with its own cost, and one
  the model owners should weigh rather than this leaf — ROUTED there by
  `routing-and-prose-k73`, because no catalogue text is contradicted by a column
  established by simulation.
  `QUINT_VERIFY=1` runs it and **this scope can afford it**; the repository
  default stays 0 because the task-tree scope cannot. `QUINT_VERIFY_STEPS` and
  `JVM_ARGS` (default `-Xmx16G`) set the depth and the heap.

The file split is inherited rather than rediscovered: in one file holding the
library, its instances and twenty-seven control and scenario modules, `quint
verify` 0.32.0 dies in its own result reporter — a fact
`crates/grove-task-tree/models/` measured first, and the remedy (controls and
scenarios in a file of their own) is what keeps `finish.qnt` within Apalache's
reach at all.

`models/run.sh` reads the `VERIFY` line above and prints it on every Quint run,
and **fails** a scope whose Quint models exist and which declares nothing — so a
limit on model checking names itself rather than passing as silence.

## What the model is, and what it is above

**A transaction, not a tree.** `crates/grove-task-tree/models/` owns what a task
tree *is*; what this file owns is the ORDER of the steps that end one, what each
step makes persistent, and what a crash between two of them leaves behind. The
task tree is abstracted to the set of entries the transaction evacuates, and
every claim here is about the protocol. Growing a second tree model would be the
thing the task-tree leaf's own instruction forbids, one scope over.

**The step list is EXPLICIT, and `FN-24.b` is why.** That obligation demands
that every step have at most one persistent effect and that the effect be a
same-directory rename (`EN-01`) or be decomposed — with anything else
*declared*, and with what it would take to decompose it. A model whose steps are
implicit in its actions cannot answer that at all. So `Step` is a closed
twenty-six-member type, `persistentEffect` is a total function on it, and the
obligation is checked over the enumeration rather than over a trace.

**Every action is total.** No action does nothing when its guard is false: each
transitions and returns exactly one member of the closed outcome set, so a
refusal is a value rather than an absent transition. That is what makes the
refusal claims falsifiable at all.

**A crash hands the next invocation a TREE, never a program counter.** `crashNow`
resets the transaction to idle; `resumePoint` derives where to rejoin from the
persistent state alone. That is `FN-24`'s content made structural rather than
asserted, and it is what makes `FN-21.a` and `FN-31.c` checkable: a resumed
disposal rejoins by the marker it finds.

## Instances

| module | what it is | why |
|---|---|---|
| `base` | every assumption granted, every model mutation off, all three lanes, an environment budget larger than any trace can spend at every point | the run every `FN-` PROPERTY is checked in, unfocused |
| `verify_small` | one lane, one entry, one environment action | the model-checking instance; see *Verification* |
| `scenario_march` | no environment action at all | both branches of the transaction, end to end |
| `scenario_gates` | perturbations only while the tree is at rest | the seven preflight preconditions, the reaper, the hook, `Current(Live)` |
| `scenario_crash` | one crash in the witness build or at the commit | `FN-09`, `FN-10.a`, `FN-12.a`, six crash boundaries |
| `scenario_crash_late` | one crash in the handoffs | `FN-19`, `FN-20`, and four crash boundaries |
| `scenario_crash_disposal` | one crash inside disposal | `FN-21.a`, `FN-31.c`, seven crash boundaries |
| `scenario_edit_txn` | one hand edit inside the transaction | `FN-06` (both outcomes), `FN-13`, `FN-25.a`, `FN-25.c` |
| `scenario_reval` | one topology change at a revalidation point | `FN-16`, `FN-18`, `FN-22.b`, `.c`, `.e`, `.f`, `.g` |
| `scenario_foreign_marker` | one foreign write during disposal | `FN-31.d` |
| `scenario_unclassifiable` | a crash, then a foreign write at the witness's name | `FN-10.b` |
| `scenario_orphan` | an orphaned quarantine beside a live witness | `FN-21.b` |
| `scenario_return_blocked` | a return that cannot complete | `FN-22.h` |
| `scenario_return_crash` | a crash inside the quarantine's return | the last crash boundary |
| `scenario_march_under_the_candidate` | `EN-03`'s counterfactual candidate, marched | `FN-24.b` over the candidate's step lists |
| `scenario_in_place_march` | the AVAILABLE candidate, no environment action at all | `FN-24.a`'s kill, `FN-28`, `FN-18`'s pair — see *The available candidate* |
| `scenario_in_place_late_result` | the available candidate, one crash and one late result | `FN-29.b`: the candidate blocks rather than disposing what it never evacuated |
| seven `relax_EN_*` | the assumption mutations | see *The controls* |
| seventeen `mutant_*` | the model mutations | see *The controls* |

**Which module a command runs in** is decided by one rule, defined in the
retired runner `models/run.sh` under *THE MODULE RULE* and cited
rather than restated here.

## What a `scenario_` instance is for, and what it is not

Every `FN-` PROPERTY is checked unfocused in `base`. Every `FN-` WITNESS lives in
a `scenario_` instance, and the reason is arithmetic rather than taste: the
subject is a transaction of about twenty steps, and while it runs every
environment action is enabled at every one of them, so an unfocused search
reaches the end with probability `(1/k)^20`. At any affordable sample count the
deep claims would be reported green on witnesses that never landed — which is
exactly the pre-registration's *vacuous invariant* hazard, in the form an
executable model takes.

A scenario **removes no behaviour**. Every action stays in the action set. What
it narrows is the SEARCH, through three model parameters:

- `ENV_BUDGET` — how many environment actions one trace may take;
- `ENV_PHASES` — at which program points they may be taken (idle, the witness
  build, the commit, the revalidation points, the restoration path, the
  quarantine handoff, disposal);
- `ENV_KINDS`, `ENV_EDITS`, `ENV_FOREIGN`, `ENV_TOPOS` — which kind, and which
  member of that kind.

`base` grants a budget larger than any trace can spend, at every phase, of every
kind — so nothing is narrowed where the properties are checked. That is the same
division `crates/grove-task-tree/models/task-tree.qnt` makes, at a larger scale,
because the subject is a longer transaction.

**The dial was audited, and it is not manufacturing the record.**
`review-prototype-finish-k57` classified every one of the 129 `wit_` commands
this column had at the time against one question — is the ghost set by a
transition the claim is about, or by the scenario's own setup? — and returned
**129 protocol-established, 0 construction-established, 0 unclear**. The
scenario modules bind constants only; none assigns `w`, `t` or `hist`, so no
witness here lands because its instance arranged for it to. That is the check a
narrowed instrument most needs and the one a green run cannot supply.

**What it does not establish is that every property is meaningful**, and the
review said so in the same breath: a protocol-fed witness sits happily beside a
safety claim that no transition could falsify. Three of them did — see
*The controls*, rows 916 – 918 — and one instrument claimed a comparison it was
not making, see *Narrowings*. Reachability and non-vacuity are two audits, and
passing the first says nothing about the second.

**A budget of one is the sweet spot, and the reason is worth recording.** With
one perturbation admitted, the world acts at a roughly uniform point of the
march and the rest of the trace is deterministic — so every crash boundary and
every revalidation point is reached in a few percent of traces. With a budget of
two the second spend is itself competing with the march, and a witness needing
one specific pair of perturbations must be given its own scenario with the kinds
narrowed to that pair. Four of the twelve scenarios exist for exactly that.

## The controls, and what they establish

### Premise-break — a named obligation must DIE

| control | obligation | result |
|---|---|---|
| `inv_fail_EN_01_FN_09a_a_torn_publication_is_observable` | `FN-09.a` | violated |
| `inv_fail_EN_01_FN_24a_a_torn_rename_is_two_stable_states` | `FN-24.a` | violated |
| `inv_fail_EN_13_FN_27a_the_sweep_deletes_foreign_bytes` | `FN-27.a` | violated |
| `inv_fail_EN_13_FN_21b_the_reaper_stops_asking_whose_it_is` | `FN-21.b` | violated |
| `inv_fail_MUT_FN_15a_the_exit_status_read_as_a_receipt` | `FN-15.a` | violated |
| `inv_fail_MUT_FN_22b_a_committed_result_restored_anyway` | `FN-22.b` | violated |
| `inv_fail_MUT_FN_31b_a_reader_observes_the_marker_absent` | `FN-31.b` | violated |
| `inv_fail_MUT_FN_10b_unclassifiable_content_mutated_anyway` | `FN-10.b` | violated |
| `inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree` | `FN-24.a` | violated — **and nothing is mutated**, see *The available candidate* below |
| `inv_fail_MUT_FN_23_a_recovery_at_rest_changes_the_tree` | `FN-23` | violated |
| `inv_fail_MUT_FN_05b_a_failing_precondition_moved_the_tree` | `FN-05.b` | violated |
| `inv_fail_MUT_FN_05c_a_failing_precondition_moved_the_repository` | `FN-05.c` | violated |
| `inv_fail_MUT_FN_06_the_identity_is_never_verified` | `FN-06` | violated |
| `inv_fail_MUT_FN_07_an_untracked_tree_is_evacuated` | `FN-07` | violated |
| `inv_fail_MUT_FN_08_the_operand_gate_is_skipped` | `FN-08` | violated |
| `inv_fail_MUT_FN_12b_an_undigestible_entry_is_evacuated` | `FN-12.b` | violated |
| `inv_fail_MUT_FN_20_a_leftover_artifact_read_as_a_receipt` | `FN-20` | violated |
| `inv_fail_MUT_FN_30_a_hook_ran_on_an_internal_commit` | `FN-30` | violated |
| `inv_fail_MUT_FN_14_the_commit_took_unrelated_work` | `FN-14` | violated |
| `inv_fail_MUT_FN_26_a_block_cleared_by_rewriting_history` | `FN-26` | violated |

**Eleven of the nineteen are MODEL mutations, and they are the ones that matter
most for reading the rest.** Each exists because an obligation would otherwise be
true *by construction* — satisfied by the way an executable model is naturally
written rather than by anything the protocol does:

- **`mutant_status_classifier`.** An evidence-classifying model satisfies
  `FN-15.a` because that is how one writes it. The mutant reads the reported
  exit status instead, and the claim dies. Without it, `FN-15.a` would be
  reported green on no evidence at all — and `FN-15.a` is the claim
  Q2 of the finish questions partly turns on.
- **`mutant_short_preflight`** kills four at once, and that is its content:
  `FN-06`, `FN-07`, `FN-08` and `FN-12.b` are each satisfied by the ORDER the
  model happens to check things in. Four obligations resting on one coding habit
  is worth knowing. **It is a BUNDLE control rather than a minimal one**, and
  the distinction matters when it is cited: the mutation skips the layout
  precondition too, so what it establishes is that the four claims fall together
  under a short preflight — not that each rests on a mechanism this dial removes
  and nothing else.
- **`mutant_eager_preflight`.** `FN-05.b` and `FN-05.c` are satisfied by a
  failure branch that assigns nothing. The mutant builds the witness and records
  its intent before it has finished checking.
- **`mutant_receipt_reading`.** `FN-20` is checked as a two-world comparison —
  the classifier is asked the same question with the leftover artifact and
  without it — and a two-world comparison proves nothing unless there is a world
  in which the classifier actually reads the artifact.
- **`mutant_no_revalidation`**, **`mutant_nonatomic_marker`**,
  **`mutant_eager_recovery`** are the same move at `FN-22`, `FN-31.b` and
  `FN-23`, each removing the named mechanism and nothing wider.
- **`mutant_unproven_ownership`** kills `FN-10.b` in its focused scenario, and it
  is **the second BUNDLE control**: globally it also disables the root-identity
  stop, so it is drawn wider than the obligation it is cited for. Read it as
  "ownership proof removed", not as "this one mechanism removed".
- **`mutant_hooks_run`**, **`mutant_unscoped_commit`** and
  **`mutant_history_rewritten`** are the three that were missing, and their
  absence is the sharper version of the same lesson. `FN-30`, `FN-14` and
  `FN-26` were each asserted over a field **no transition in the model could
  make bad**: the commit wrote `hooksRan: false` unconditionally,
  `unrelatedMutated` and `historyRewritten` were initialised false and only ever
  read. Their witnesses landed and were protocol-fed, and the safety claims were
  still true by construction — which is exactly the case where a protocol-fed
  witness and a meaningful property come apart. Each mutant now supplies the
  missing mechanism: a hook that runs, a commit that takes the unrelated bytes,
  and a block "cleared" by rewriting the recorded history.

### Exercise-removal — named witnesses must become UNREACHABLE

| control | witnesses | result |
|---|---|---|
| `relax_EN_08` (`crash` removed) | six, including `FN-09.a`'s, `FN-10.a`'s, `FN-12.a`'s, `FN-21.a`'s and `FN-31.c`'s | 0 traces each |
| `relax_EN_16` (the lane collapsed to one) | `FN-15.b`, `FN-15.c`, `FN-15.d`, `FN-25.c` on the removed lanes | 0 traces each |

`relax_EN_16` is the one to read twice. Every `FN-` property stays green with one
lane — which is precisely what makes a collapsed lane dimension INVISIBLE
without this control, and is why the catalogue exercises `EN-16` rather than
relaxing it.

### Counterfactual capability — every retained obligation must stay GREEN

| control | retained | result |
|---|---|---|
| `relax_EN_03` (atomic recursive deletion — **Q1**) | `FN-20`, `FN-24.a`, `FN-24.b`, `FN-27.a`–`.c`, `FN-32` | all hold; the candidate's own successful exit reached in 46% of traces. **`FN-32` is `finish-verdicts-k65`'s addition**, in place of the `TT-24` Q1's retained set named and this scope cannot discharge |
| `relax_EN_05` (commit inside the transaction — **Q2**) | `FN-03`, `FN-15.a`, `FN-24.a`, `FN-25.a`, `FN-25.b`, `FN-27.a` | all hold; `Indeterminate` unreached on every lane |
| `relax_EN_15` (machine-attested confirmation) | `FN-01.a`, `FN-01.b` | both hold, both witnesses still land; **no obligation strengthened** |

`relax_EN_15`'s result is the unusual one and it is the expected one: an
attestation replaces nothing. A run in which some obligation *did* strengthen
would have been the finding.

### The available candidate — a strategy dial, and what it measured

`IN_PLACE_DISPOSAL` is **not** a counterfactual capability and does not belong in
the table above. It grants nothing: every `EN-` assumption
`scenario_in_place_march` imports is the one `base` imports, `ATOMIC_DISPOSAL`
included. What differs is the **strategy** after a proven commit — the published
witness emptied entry by entry at the task root's own name, the witness released,
the root released — which is the only no-quarantine protocol the environment
table permits and the one Q1's availability-typed criterion is stated over.
`finish-verdicts-k65` read `ATOMIC_DISPOSAL`'s single `const` as proof that no
control could separate the quarantine from the capability; **that was a fact
about this encoding, and this dial is the measurement that says so.**

**The order is the candidate's best case and it is also forced.** After
`SEvacuate` every entry is inside the published witness, and `EN-03` denies the
recursive removal that would take the witness and its contents together — so the
entries go one at a time, the witness is released when it is empty, and the
root's own name is released last. The ADR predicted the interesting violation
only "for a candidate that retires the witness first"; no available candidate
can. If a retained claim fails under this order it fails under every available
order.

| command | what it measures | result |
|---|---|---|
| `wit_FN_28_the_available_candidate_reaches_its_successful_exit` | the candidate runs to `SRemoveRoot`, so everything below is over a world in which it happened | reached, 3266 traces |
| `wit_FN_24b_the_in_place_branch_enumerated` | `FN-24.b`'s enumeration over the candidate's own step list | reached, 3266 traces |
| `wit_FN_24a_the_in_place_candidate_exposes_an_ordinary_current_tree` | a present task root, no witness, no quarantine, disposal outstanding — classified `Current(*)` | reached, 3410 traces |
| `wit_FN_24a_the_artifact_guarded_encoding_accepts_it` | **the false green**: `FN-24.a` as this file encoded it accepts that disk | reached |
| `wit_FN_18_a_root_being_disposed_reads_as_a_restoration` | with at least one entry removed and the witness still standing, `resumePoint` reads the disk as `SRevalBeforeRestore` — a proven-committed finish, read as one to roll back | reached |
| `inv_FN_28_still_holds_under_the_available_candidate` | the repaired `FN-28`, over the world its witness above is counted in | holds |
| `wit_FN_18_and_then_reads_as_nothing_outstanding` | once the witness is released, `resumePoint` reads `SIdle` while the root still stands and disposal is unfinished | reached, 3410 traces |
| `inv_fail_MUT_FN_24a_the_available_candidate_leaves_an_ordinary_tree` | the **repaired** `FN-24.a`, over the same world | violated, as the control requires |

**AND THE CANDIDATE NO LONGER SUCCEEDS OVER BYTES IT HAS NOT PROVED.** This
module admits no environment action, which is what makes its `FN-24.a` kill
strong and is also what hid a second defect from it. `nextInPlaceDisposable`
walked **every** entry not yet `Disposed`, `AtRoot` included, and
`SDisposeRootEntry` unlinked the selected entry without consulting its place,
type, digest or manifest membership. The path is reachable and the model records
it: a crash after publication, recovery, and a late result that makes the
disposition `Committed` reaches `SRevalBeforeQuarantine` with **evacuation
incomplete** and diverts forward into the candidate's disposal, over a task root
whose entries the transaction never moved. The incumbent may walk the whole root
because the rename **moved** it to a name Grove owns; the candidate performs no
such move and can point at nothing for an entry still at a user-visible path.

The walk is now `evacuated(w)` — what this transaction's own evacuation moved —
and the fall-through the first draft had is closed at the other end: with an
entry still `AtRoot` when the evacuated ones are gone, the candidate **blocks**.
Publication and the commit both stand, so `FN-29.b` makes the stop a block rather
than a refusal, and the diagnosis is `RecoveryPending` because the witness is
Grove's and names this attempt. `scenario_in_place_late_result` is that
measurement: the block is reached in **56 of 8000** traces with the published
witness standing and the entry unlinked by nothing, every one of them carrying
`RecoveryPending`, while the candidate's ordinary successful exit is still
reached in **2130** — so the stop discriminates rather than walls. Found by
`honest-classification-k84`, finding 2.

**Deciding whether a resumed sweep may prove ownership over such an entry is not
this section's, and was deliberately not decided here**; it is
`sweep-ownership-k81`'s either/or. What was removed is only the candidate
reporting **success** over a deletion it had not proved, which would have made
that later proof irrelevant to the transition already doing the deleting.

**AND ONE CLAIM IS RED UNDER THE CANDIDATE AND IS DECLARED HERE RATHER THAN LEFT
TO THE MODULE RULE.** `inv_FN_25b` fails in
`scenario_in_place_late_result`'s world: once the candidate has released the task
root, a recovery meeting an absent root with no correlation ticket blocks through
`recoverOp`'s `rootless` site with `diagnose` returning the **empty set**,
because nothing of Grove's is left at a reserved name for `groveOwnedCorrelated`
to read. The incumbent leaves a quarantine there and never reaches it.
**Pre-existing, and measured as such**: the same module run against the committed
model at `6d0188dd` — before this leaf touched anything — violates it
identically. It is the same disk `wit_FN_18_and_then_reads_as_nothing_outstanding`
reads as `SIdle`, so it is the resumption hole `sweep-ownership-k81` owns, and it
is recorded in that leaf's body rather than repaired here.

**A THIRD SITE OF THE SAME SHAPE TURNED UP AFTER THE REPAIR, AND IT IS WHY THE
`FN-28` ROW IS IN THAT TABLE.** `inv_FN_28`'s second operand read
`(ATOMIC_DISPOSAL or hist.appliedAfterQuarantine)` — **an enumeration of the two
protocols that happened to exist**, not a guard on a role. The available
candidate satisfies `FN-28`'s role (*the task root leaves its own name only on a
proven result*) and **falsified the claim**, and nothing reported it: a
`scenario_` module carries only its own commands, and this one declared
`FN-28`'s **witness** while never checking its property — so the run credited a
coverage cell from a world in which the claim was false, which is run.sh's
obligation-3 hazard one grain down. Found by this leaf's in-session reviewer,
after the `FN-24.a` repair had landed. The claim is now stated over two facts
each root-freeing step records for itself (`rootTakenAway`,
`rootTakenWithoutProof`), it is green in `base` and in every `scenario_` and
`relax_` module, and it is asserted inside the candidate's own module so the
witness and the property cannot come apart again. **Adding a third disjunct would
have been the wrong repair**, and that it was available is what makes the rule
worth recording.

**AND THE FIRST REPAIR WAS SELF-CERTIFYING, WHICH IS WHAT `honest-classification-k84`
CAME BACK FOR.** `rootTakenAway` was set in `SQuarantineRename` **before** the
branch on `RENAME_ATOMIC`, on the reading that a torn rename still took the
root's name away. It did not: the false arm leaves `rootPresent` **true**,
creates no quarantine, and sets `rootTorn` — a name free and occupied at once,
which is precisely what `EN-01` is bought to prevent. Recording that as a removal
let a proven-result trace settle `OApplied` with `FN-28` satisfied while Grove had
never taken the task root away, **laundering `EN-01`'s premise break into the fact
the claim exists to test**. `inv_FN_22i` says the same thing about the same arm
one claim over, and had been failing there, unreported, for as long as
`relax_EN_01` has existed — a `relax_` module carries only the commands written
inside it. Both flags now sit in the arm that completes the removal, and both
failures are declared:
`inv_fail_EN_01_FN_28_a_torn_rename_is_not_a_removal` and
`inv_fail_EN_01_FN_22i_a_torn_rename_applies_over_a_present_root`.

**AND EACH OPERAND NOW HAS AN ISOLATING KILL, MEASURED AS ISOLATING RATHER THAN
ASSERTED TO BE.** The review's test is exact — *a control independently falsifies
each new load-bearing operand; an unrelated failure common to the old and new
predicates is not credited as that control* — and `relax_EN_13`, which the
producer offered, fails `FN-28` under the old formulation and the new one alike,
through the operand neither changed.

| operand | control | what makes it isolating |
|---|---|---|
| `rootTakenAway` | `relax_EN_01` | the OLD predicate — `deletionProvenFor and (ATOMIC_DISPOSAL or appliedAfterQuarantine)`, written out inline — is **green** over that module, and so is the new predicate's other operand on its own. Only this one decides it |
| `rootTakenWithoutProof` | `mutant_status_classifier` | the disposition read off an exit status runs the removal with no ticket (the flag is set in **683 of 8000** traces) and the same status settles `OApplied`; `rootTakenAway` alone over that module is **green** |

**AND THE FIRST OPERAND WAS A FACT ABOUT THE REPOSITORY, WHICH THE CATALOGUE
ALREADY SAYS THE PROTOCOL CANNOT HOLD — IT JUST SAID IT ABOUT THE SECOND.**
`inv_FN_28` read `deletionProvenFor(w, t.settling)` over the world the invariant
was **evaluated** in, and the correlation ticket is not Grove's to keep:
`topologyChangeAt(1)` and `(5)` are the operator dropping the result, admitted at
any point. So a finish that did everything right went red the moment the operator
dropped the ticket afterwards. **This is a false green found by narrowing, not by
widening, and it was `base`'s**: with every `EN-` assumption granted and every
model mutation off, in a world whose only difference from `base` is an
environment restricted to topology changes with a budget of 2 — a strict
**subset** of `base`'s traces — `inv_FN_28_one_successful_exit` is violated with
the ticket set empty, `shape` `CSAbsent` and `rootTakenAway` true. `base`'s 8000
samples never drew it. The proof is now recorded at the step that acts on it
(`rootTakenWithoutProof`), and the same narrowed world is green.

**AND THE CATALOGUE NOW STATES THE SAME `FN-28` THE MODEL IMPLEMENTS.** The
semantic contract said *the only transition under a transaction that takes the
task root away SHALL be the quarantine rename* while the repaired predicate
accepted `SRemoveRoot` through generic history flags — the Quint column green
against a role-form its source-of-truth obligation did not contain, with the
runner's coverage claim over two different propositions and no declared gap
(`honest-classification-k84`, finding 3). `FN-28` is restated over the role, an
attempted removal is excluded in the catalogue's own words, and the claim now
carries the `*Class*: shared safety` line every other claim of its class does —
which is what makes
[`a-shared-safety-claim-names-the-role-not-the-artifact`](../../../docs/adr/a-shared-safety-claim-names-the-role-not-the-artifact.md)
visibly apply to it.

**WHAT THE TWO `FN-24.a` ROWS ARE, TOGETHER, IS AN A/B ON THE REPAIR AND NOT TWO
FINDINGS.** One disk; the artifact-guarded encoding accepts it and the
role-guarded one rejects it. That is the whole of what
[`a-shared-safety-claim-names-the-role-not-the-artifact`](../../../docs/adr/a-shared-safety-claim-names-the-role-not-the-artifact.md)
records. All three inherited `FN-24.a` kills — `mutant_no_quarantined_state`,
`mutant_absent_classified_first` and `relax_EN_01` — are green beside the new one,
which is what says the claim was strengthened rather than moved.

**AND `groveReservationStands` NOW HAS ITS OWN WITNESS AND KILL, WHICH RETIRES A
DECLARED GAP RATHER THAN RESTATING ONE.** This paragraph twice said less than it
should have. It first claimed the disjunct was kept because dropping it would
lose `mutant_no_quarantined_state`'s kill; `honest-classification-k80`'s reviewer
measured that false in both halves — all three inherited kills fire with
`unsettledRootWork` **alone**, `base` stays green, and the disk the justification
named is not admitted in that module at all (`ENV_KINDS = Set(0)`, crash only).
It then called the conjunct *declared uncontrolled*, and
`honest-classification-k84`'s finding 4 is that **an honest limitation is not a
verification**, and that the missing control was never an expressivity limit:
`handEditTo(12)` already constructs `EN-11`'s orphaned, Grove-owned quarantine
beside a task root that is still present, with no transaction live over it. What
no module did was combine that disk with the classifier mutation.

`mutant_orphan_is_not_a_reserved_state` does. It admits that hand edit and
**nothing else** — crashes off, one environment action, `ENV_EDITS = Set(12)` —
and it carries a pair rather than a bare red:
`wit_FN_24a_the_artifact_realisation_bites_where_the_moved_tree_one_cannot` is
reached in **7230 of 8000** traces and holds `groveReservationStands` true,
`unsettledRootWork` **false**, and the classification in the set the conjuncts
bite on; `inv_fail_MUT_FN_24a_an_orphaned_quarantine_reads_as_an_ordinary_grove`
is the kill. **The isolation is carried by the witness, deliberately, and the
reason is stated rather than hidden**: with the quarantine no longer a reserved
state every trace reaching the rename also fails through the moved-tree
realisation, so which disjunct the invariant died on is not attributable from the
kill alone — what is attributable is a reached state that only the first
realisation catches. So the disjunct is retained on §*States*' `Reserved` class
sentence having two realisations, and it is no longer retained on faith.

**THE NEW KILL NEEDS NO INTERRUPTION, AND THAT IS THE SHARPEST THING HERE.**
`scenario_in_place_march` runs at `ENV_BUDGET = 0` with `ENV_PHASES` and
`ENV_KINDS` empty — no crash, no hand edit, no foreign write, no topology change.
Every other `FN-24.a` counterexample in this suite needs a crash. The candidate
produces its dishonest disk on its own uninterrupted happy path.

**AND THAT NARROWING IS NOT THE ONE `finish-verdicts-k78` HAD TO REPAIR, THOUGH
IT IS THE SAME THREE `const`s.** What that leaf found in `relax_EN_03` was an
**invariant set** asserted in a world narrowed until its retained claims had no
antecedent to be about; the rule it left is that a control may narrow to its own
site, and may not narrow the world an invariant set is checked in. This module
asserts no retained set — six witnesses, which belong at their own site, and one
kill — and the narrowing makes the kill **strictly stronger**: with no
environment action admitted its traces are a **subset** of `base`'s, so a
violation found here is a violation there. Checked rather than asserted: the
module differs from `base` in exactly one non-search `const`,
`IN_PLACE_DISPOSAL`.

**AND `unsettledRootWork` CARRIES A NARROWING, DECLARED HERE RATHER THAN FOUND
LATER.** Its *a live transaction* half is the model's program state, not the
disk, and `crashNow` resets it — so on a **post-crash** disk the disjunct is
false and `groveWorkOutstanding` falls back to the artifact form. What carries
the crash boundary is `hist.crashMisclassified`, which `crashNow` computes from
the **pre-crash** `w` and `t`, so the judgement is made while the transaction is
live and then frozen. No `handEditTo` in this column presents a half-disposed
root to a *later* invocation, so the strengthened claim is checkable inside the
trace that creates the state and not from outside it; and the disk-only half
cannot stand alone, because `Disposed` is terminal and would be true forever
after any successful finish. Closing that needs an environment action producing
the disk, and it is owed.

**AND THE `FN-18` PAIR IS A SECOND INSTRUMENT ON THE SAME PROTOCOL, WORTH ITS OWN
LINE.** `FN-24`'s header states the premise the whole claim rests on: *a crash
does not hand the next invocation a program counter, it hands it a tree, and what
the tree says has to be enough.* `resumePoint` is this file's reading of that
tree, over the persistent state and nothing else. Under the candidate it says
**restore** while the entries are half gone and the commit is proven, and then
says **nothing is outstanding** while the root still stands. Neither reading is
constrained by any command in this suite today, because the shared-safety claim
that could constrain them — an ownership proof over a resumed sweep — is the hole
`sweep-ownership-k81` owns. Recorded here rather than acted on.

**WHAT THIS SECTION DOES NOT ESTABLISH, AND THE LIST IS SHORT AND LOAD-BEARING.**
Q1 is not classified here and neither are Q4's rows. The retained set is
`FN-20`, `FN-24`, `FN-27`, `FN-32`, and **`FN-32` still has no
candidate-reachable site** — the defect `finish-verdicts-k78` found at
`relax_EN_03` is unrepaired for this candidate too, and repairing it is
`sweep-ownership-k81`'s. So no run in this file yet checks the candidate against
a *complete* retained set, and a red `FN-24.a` under an incomplete set is
evidence about the instrument rather than a verdict on the protocol.
The Alloy column has neither the dial nor the repair and owes both
(`alloy-candidate-k82`).

**Bounded unreachability, stated as what it is.** Every `wit_unreach_` control
here is randomized simulation, so a zero count is evidence that the witness is
unreachable *within* 8000 samples at depth 24 — never a proof. Where the
catalogue needs the stronger instrument it says so, and `FN-15.d`'s
unreachability branch under `relax_EN_05` is read with exactly that
qualification below.

## Abstractions

Beyond the catalogue's own *Deliberate omissions*, which
this model takes as written:

- **The task tree is a set of entries.** Names, positions, keys, session kinds
  and the walk are the task-tree scope's, and nothing here reads them. What this
  model needs of an entry is its identity, its type (digestible or not) and its
  digest.
- **`ONotEntered` is an outcome this model ADDS**, and the catalogue's closed set
  deliberately has no member for it: `FN-01`'s first preflight member "produces
  no refusal at all". But a total action must return something, and if
  want-of-confirmation produced an *absent* transition then `FN-01.a` would be
  true by construction and unfalsifiable. So the model names the non-entry —
  exactly as the catalogue anticipates a model naming the guard wait it also
  refuses to put in the set.
  **`closed-set-additions-k74` disposed this one as a DECLINE, and the
  abstraction stands.** The catalogue gains nothing, and the reason is a fact
  about the product rather than about the set: **Grove has no confirmation gate
  at all.** Constraint 5 is *grove guides, it does not gate*, `finish_commit`'s
  own contract puts the confirmation on the calling session, and no Grove binary
  reads standard input anywhere. A transaction not entered for want of
  confirmation is a call that was never made, so the closed set — which covers
  what a *completed invocation* returns — cannot name it without ceasing to be
  that set. §*Outcomes* now generalises its guard-wait paragraph to govern every
  such non-event, so this bullet describes a rule being followed rather than a
  gap being worked around.
- **`RDeletionNotCommitted` is NO LONGER a refusal reason this model adds** — it
  is the catalogue's `DeletionNotCommitted`, and this model's declaration (with
  `finish.als`'s independent one) is the finding that put it there. Disposed by
  `closed-set-additions-k74`; see *Findings*.
- **A precondition-naming observable.** `FN-05.a`'s seven members are not
  distinguishable by their outcomes — the catalogue says so and asks for exactly
  this — so `Precondition` is a state field the seven witnesses read.
- **`Current(Live)` is NOT one of these, and it used to be.** The classifier
  read a `liveOrdinaryWork` hard-coded to `false`, so one of the catalogue's own
  stable task-root states — and a branch `FN-24.a` quantifies over — could not be
  reached, undeclared. Live ordinary work is now a state of the world set by the
  same environment edit the `PCGuards` refusal is stated over, the classification
  is exercised in 15.4% of `scenario_gates` traces, and the abstraction is gone
  rather than declared.
- **The index image is a flag nothing reads.** The colocated lane's index backup
  and restore are represented as `repo.indexRestored`, set on an uncommitted
  result, and no obligation is stated over it. Q4's row for it is declared
  `abstracted` for that reason rather than answered.
- **Two attempts, one process**, per the catalogue's own omission of more than
  two cooperating processes. The reaper runs as a separate invocation rather
  than as a concurrent one.
- **Bounds**: one evacuable entry, three lanes, trace depth 24, 8000 samples,
  environment budget as each instance declares.

## Narrowings, each declared

- **`FN-24.a`'s witness is enumerated over the EFFECTFUL steps**, eighteen of
  them, one command each — not over all twenty-six. A crash at a revalidation
  point makes nothing persistent, so the state it leaves is byte-identical to
  the state a crash at the boundary before it leaves, and a nineteenth command
  asking for it would be asking for the same reachability twice. The obligation's
  text is "one crash point per step"; this is that, over the steps that have a
  boundary.
- **`FN-24.b`'s witness is the step list enumerated on each of the two
  branches**, in two commands. No single trace takes both the restoration and
  the quarantine branch within the runner's depth of 24, and the enumeration is
  the union of the two.
- **`FN-05.a`, `FN-05.b` and `FN-05.c` get seven witnesses each.** The seven
  preconditions are mutually masking — the preflight returns the FIRST failure —
  so no single trace reaches them all, and the catalogue's own wording is "each
  of the seven, reached".
- **`FN-25.b`'s "exhaustive sweep of the blocked states" is a BOUNDED sweep,
  and the witness now says so.** It reports the block SITES the search reached
  — `blockSitesSwept` — rather than asserting that one reached block was an
  exhaustive sweep, which is what it did before and which was not a sweep at
  all. The obligation asks for exhaustiveness; what this column supplies is
  8000 samples at depth 24.
- **`FN-25.b`'s second half IS a comparison, and it was not one until this
  column was reviewed.** The diagnosis is classified from the STATE by
  `diagnose`, a predicate independent of the block call sites, and the
  obligation compares that against the diagnosis each `Blocked` outcome
  actually CARRIES (`carriedDisagreed`). Until the comparison existed the
  invariant asked only whether the classifier returns one answer — which a
  machine deriving every carried diagnosis from that same classifier satisfies
  by definition. Adding it found a real disagreement immediately: the
  tracked-witness commit named `OwnershipConflict` by hand while the state was a
  correlated, provably Grove-owned artifact, which is `RecoveryPending`. That
  site now classifies like every other.
- **The correlation ticket's attempt-binding is a CONSTRUCTION fact here, not a
  controlled one, and that is the honest statement.** `deletionProvenFor` takes
  the attempt it is proving, so a ticket naming this handle and another attempt
  cannot satisfy `FN-03` or `FN-28` — the catalogue's own reason for putting the
  attempt in the ticket is that it "lets a retry that has lost every local trace
  still tell its own completed attempt from someone else's". What no control in
  this column can show is that the binding BITES: no reachable state offers a
  rootless retry a ticket for an attempt other than the one it is settling,
  because a crash carries `settling` forward and the task root only goes absent
  after this attempt's own commit has landed. The alien ticket is reachable (a
  `topology-change`, `EN-09`), the state in which it would be *believed* is not.
- **`FN-24.a` reads *exactly one* as the ORDER's answer and not as arm
  disjointness, and it used to read the second.** The catalogue fixes an order
  and says the order is itself a claim (`TT-18`), so *exactly one* is what the
  order resolves to; *never into a state indistinguishable from a different one*
  is the separate half, and in this model the only thing that produces it is one
  artifact answering to two **names** at once, which is what `EN-01` buys and
  what `relax_EN_01` takes away (`sameNameAmbiguity`). The disjointness reading
  was stronger than the catalogue and became false the moment the table gained a
  row that overlaps `Absent` by design. Totality and single-valuedness are now
  near-true by construction here exactly as in `finish.als`, kept against a row
  added later with a narrower guard; what carries the obligation is §*States*'
  load-bearing property, one conjunct for `Absent` and one for the `Current`
  class, and rows 919 and 920 are what kill them.
- **`FN-28`'s third sentence is true by construction here, and the reason is a
  world this column cannot build.** *Grove never puts the pinned task root back
  while the commit stands proven* is `SReturnQuarantine`'s own guard
  (`d != DCommitted`), so no trace can violate it. Nothing in this world
  re-occupies a freed task-root name either, which is why the operand
  `finish.als` had falsified three times — *the task root is absent* — was true
  here for a reason that is a property of the model rather than of the protocol.
  The invariant now states what the catalogue states, over Grove's own steps;
  what carries it is `deletionProvenFor`, and the step operand is declared here
  rather than presented as a conjunct that can fail.

## The green run this column stands on

```sh
models/run.sh --scope finish --family quint                 # the suite
QUINT_VERIFY=1 models/run.sh --scope finish --family quint  # …and model-checked
```

| run | result |
|---|---|
| `--scope finish --family quint` | **exit 0** — 4m 25s wall / 317s CPU on a 16-core host, 228 commands, 0 failures, `-- cells: 61 complete, 0 declared gaps, 0 empty, of 61`, Q4 matrix 10 of 10 rows |
| `--scope finish --family quint`, after `closed-set-additions-k74` | **exit 0** — **236 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows. Eight more commands and two more cells: `FN-29` split into `.a` and `.b`, `FN-29.b` carries a witness per arm, and `wit_FN_32` now reads both arms because the witness-slot step moved from a block to a refusal |
| the same with `QUINT_VERIFY=1` | **exit 0** — 11m 51s wall, 289 commands, 0 failures; the 61 `SKIP` lines become `model-checked to depth 4, no counterexample` |
| `--scope finish --family quint`, after `finish-verdicts-k65` | **exit 0** — **240 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows (3 `none`, 1 abstracted). **5m 07s wall** (`2026-08-28T04:33:02Z` → `04:38:09Z`). One more command than the row below: `relax_EN_03` gained `inv_FN_32_ownership_still_proven_under_the_candidate`, which **holds**, so Q1's retained set is now checked in full by the family the assumption table assigns `EN-03`'s mutation to. Provenance: `docs/specs/semantic-contract.md`, this README and both `.qnt` files were digested before the run and re-digested after, byte-identical either side, and the GAP-line count was 4 both times; this row was written after the run |
| `--scope finish --family quint`, after `honest-classification-k85` integrated `k84`'s review | **exit 0** — **272 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows. **6m 39s wall / 481s user** (`2026-08-28T07:54:39Z` → `08:01:18Z`). Ten more commands than the row below, every one accounted for: 2 in `relax_EN_01` (`FN-28`'s isolating kill and `FN-22.i`, which had been failing there unreported), 1 in `mutant_status_classifier` (`FN-28`'s second isolating kill), 5 in the new `scenario_in_place_late_result`, 2 in the new `mutant_orphan_is_not_a_reserved_state`. The obligation manifest did **not** move — `models/run.sh --list` prints **130** before and after — because the catalogue's `FN-28` edit is that obligation's own text plus the `*Class*` line every claim of its class already carries. **The baseline it is measured against was re-measured in this same session on the unmodified tree at `6d0188dd`, not recalled: exit 0, 262 commands, 63 of 63 cells, 5m 55s wall / 419s user.** Provenance: the same four subjects digested before the run and re-digested after, **byte-identical either side**, and the runner's own `-- cells:` line reported 0 declared gaps both times; this row was written after the run |
| `--scope finish --family quint`, after `honest-classification-k80`'s review integration | **exit 0** — **262 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows. **5m 49s wall / 416s user** (`2026-08-28T06:52:00Z` → `06:57:49Z`). One more command than the row below: `inv_FN_28_still_holds_under_the_available_candidate`, which exists because the row below's run was green while `FN-28` was **false** under the candidate — the claim's second operand enumerated two protocols, and a `scenario_` module carries only its own commands, so the witness credited a cell the property would have failed. Provenance: the same four subjects digested before and after, **byte-identical either side**, declared gaps 0 both times; this row was written after the run |
| `--scope finish --family quint`, after `honest-classification-k80` **before its review was integrated** | **exit 0** — **261 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows (3 `none`, 1 abstracted). **5m 47s wall / 413s user** (`2026-08-28T06:17:02Z` → `06:22:48Z`). Seven more commands than the 254 the previous leaf left, all in the new `scenario_in_place_march`: six `wit_` measurements and one `inv_fail_`. The obligation manifest did **not** move — `models/run.sh --list` prints 130 before and after — because every catalogue edit is prose, a table decision or an obligation's own text. Provenance: `docs/specs/semantic-contract.md`, this README and both `.qnt` files were digested before the run and re-digested after, **byte-identical either side**, and the declared-gap count the run itself reports was **0 both times** — the figure to compare is the runner's own `-- cells:` line rather than a grep for the word, which this row would otherwise have moved by containing it. This row was written after the run. The baseline it is measured against is the same command on the unmodified tree at `wkzptosn`: **exit 0, 254 commands, 63 of 63 cells, 5m 25s**, re-measured in the same session rather than recalled |
| `--scope finish --family quint`, after `finish-scope-k76` | **exit 0** — **239 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows. Three more than the row above: `mutant_correlation_wins_the_overlap`'s `inv_fail_MUT_FN_25a_correlation_wins_the_overlap`, plus the two `mutant_*` commands the k71 row already counted under verify |
| the same with `QUINT_VERIFY=1`, after `finish-scope-k76` | **exit 0** — **14m 35s wall** (`2026-08-28T01:37:28Z` → `01:52:03Z`), **302 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows. All **63** properties `model-checked to depth 4, no counterexample`, the restated `inv_FN_25a_the_carried_diagnosis_is_the_one_precedence_selects` among them — so the precedence is model-checked and not only simulated. One more command than k71's 301 and ~1m 40s dearer; the extra wall time is `models/run-controls.sh` running concurrently for part of it, which the k71 note predicts. **`inv_fail_MUT_FN_25a_correlation_wins_the_overlap` reports `violated, as the control requires`**, and its module carries `scenario_edit_txn`'s environment rather than its neighbours' — with the neighbours' it reports green, because the overlap is unreached there |
| `--scope finish --family quint`, after `finish-scope-k71`, with `QUINT_VERIFY=1` | **exit 0** — 12m 53s wall / 376s CPU, **301 commands**, 0 failures, `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows. All 63 properties `model-checked to depth 4, no counterexample`, the restated `inv_FN_24a` among them. Two more commands than the batch above's non-verify count: `mutant_no_quarantined_state` and `mutant_absent_classified_first`, both reported `violated, as the control requires` |

Both figures are post-integration: `integrate-review-prototype-finish-k58` added
three model mutations and two witnesses to what `prototype-finish-k12` left, for
about 8% of the run (the producer's were 4m 05s / 223 commands and 10m 38s / 284).

The second row is the one to quote, **with its depth attached**. See the
`VERIFY` line above for what depth 4 does and does not reach.

**The command count moved four times since; the arithmetic below is the 228 row's
and is kept because it is the only place the accounting method is written down.**
Read it as *how* to reconcile a count, not as the current one.

The 228 are the 61 property commands the library declares and `base` inherits,
plus the 167 written inside `finish-controls.qnt`: **114 witnesses** across the
twelve scenarios, and **53** across the eighteen mutation instances — 17
witnesses (13 of them `wit_unreach_`), 17 retained properties and 19 inverted
`inv_fail_` controls. `verify_small` inherits the same 61 properties and
reports each as `SKIP` under the default `QUINT_VERIFY=0`; the runner does not
count a skipped command as work, which is why 228 rather than 289. **No module
loses commands under the runner's parser**: every `val inv_…`/`val wit_…`
declaration in both files is accounted for, and the `val`s that are neither are
helpers.

## The mutation matrix — Quint

One row per control, and what it killed. Q4's rows cite these by number.

| row | control | class | what it does | result |
|---|---|---|---|---|
| 901 | `relax_EN_01` | premise-break | a rename observable half-applied | `FN-09.a` and `FN-24.a` die |
| 902 | `relax_EN_03` | counterfactual | atomic recursive deletion: disposal in place, no quarantine, no marker, no replace transition | every retained shared-safety obligation holds, `FN-32` included — and since `finish-verdicts-k78` the module differs from `base` in exactly one `const`, carries `FN-24.a`'s ten per-step crash witnesses over the candidate's own step list, and has a reached `FN-32` antecedent (`wit_FN_32_the_candidate_meets_an_unprovable_artifact`, 317 traces). **It is one `const`**: `ATOMIC_DISPOSAL`'s true branch replaces `SQuarantineRename` and every step after it with one `SDisposeInPlace`, which is why rows Q4-105 – 107 cannot be separated **in this file** — see below |
| 902a | `scenario_march_under_the_candidate` | counterfactual | row 902's protocol, uninterrupted | `FN-24.b`'s two branch enumerations over the candidate's own step list, 1660 and 3649 traces. It exists for the reason `scenario_march` does: a branch enumeration needs a trace that walks a whole branch, which a crash-at-every-point world does not produce |
| 902b | `mutant_unproven_ownership_under_the_candidate` | model | `mutant_unproven_ownership`'s environment with `ATOMIC_DISPOSAL` flipped | KILLED `FN-32` through the slot, so the candidate's retained `FN-32` is no longer a pure tautology — before this row it had no control at all. **It kills at `SCreatePreparing`, which the candidate inherits unchanged**: `stopReserved`'s other two transaction-side sites are `SQuarantineRename` and `SCreateMarker`, both unreachable under `ATOMIC_DISPOSAL = true`. So `FN-32` has no content over anything the candidate CHANGES and cannot be given any, since the candidate removes every artifact its other sites are about — which is Q1's second criterion defect rather than this row's |
| 903 | `relax_EN_05` | counterfactual | the commit inside the filesystem transaction | every retained obligation holds; `Indeterminate` unreached on every lane |
| 904 | `relax_EN_08` | exercise-removal | `crash` removed | six interruption witnesses unreachable |
| 905 | `relax_EN_13` | premise-break | the reaper sweeps its reserved namespace | `FN-27.a` and `FN-21.b` die |
| 906 | `relax_EN_15` | counterfactual | a machine-attested confirmation | nothing strengthens, nothing fails |
| 907 | `relax_EN_16` | exercise-removal | the lane collapsed to one | four per-lane witnesses unreachable, every property green |
| 908 | `mutant_status_classifier` | model | the exit status read as a receipt | `FN-15.a` dies |
| 909 | `mutant_no_revalidation` | model | the four revalidation points not performed | `FN-22.b` dies |
| 910 | `mutant_nonatomic_marker` | model | the marker replaced by a remove then a create | `FN-31.b` dies |
| 911 | `mutant_unproven_ownership` | model | the transaction mutates what it cannot prove is its own | `FN-10.b` **and `FN-32`** die. Both read `hist.mutatedUnproven`, which is the transaction-side flag; the reaper's `hist.foreignMutated` is a second field precisely so that `FN-21.c` is not killed here |
| 912 | `mutant_eager_recovery` | model | a recovery at rest repairs anyway | `FN-23` dies |
| 913 | `mutant_eager_preflight` | model | the preflight writes before it has finished checking | `FN-05.b` and `FN-05.c` die |
| 914 | `mutant_short_preflight` | model | the preflight checks only confirmation and the guards | `FN-06`, `FN-07`, `FN-08`, `FN-12.b` die |
| 915 | `mutant_receipt_reading` | model | the fresh-tree classification reads a leftover artifact | `FN-20` dies |
| 916 | `mutant_hooks_run` | model | an installed operator hook RUNS during the internal commit | `FN-30` dies |
| 917 | `mutant_unscoped_commit` | model | the commit takes the unrelated working-copy bytes with it | `FN-14` dies |
| 918 | `mutant_history_rewritten` | model | a block "cleared" by rewriting the recorded history | `FN-26` dies |
| 919 | `mutant_no_quarantined_state` | model | §*States* without `Reserved(Quarantined)` — the six rows this scope reaches without it, as the table stood before `finish-scope-k71` | `FN-24.a` dies: a standing quarantine reads as an ordinary grove |
| 920 | `mutant_absent_classified_first` | model | §*States*' former table ORDER taken literally — `Absent` first, the whole `Reserved` class after it | `FN-24.a` dies: the post-rename disk reads `Absent` while `FN-22`'s fourth revalidation point is still ahead of it |

Rows 914 and 911 are **bundle** controls and the rest are minimal: 914 also
skips the layout precondition, and 911 also disables the root-identity stop.
Rows 916 – 918 exist because `FN-30`, `FN-14` and `FN-26` were asserted over
fields no transition could make bad — see *The controls*.

**Rows 919 and 920 are `finish-scope-k71`'s, and they are two rows against one
obligation on purpose**: the two halves of the catalogue repair they control are
separable defects, and a single combined dial could not say which half a green
run rested on. Row 920 mutates the **ranking** rather than one edge of it, which
is the lesson `finish.als`'s row 49 paid for — a precedence relation is
transitively redundant by construction, so a disk's classification survives the
loss of any single pair on a neighbouring one.


## Q4 — the removal matrix, Quint's rows

One row per removable artifact, naming the **first shared-safety obligation** its
removal breaks under the mutation discipline, or `none`. A row reading `none` in
both families is Q4's evidence for delete/replace; a row naming an obligation is
evidence the artifact is protecting the user rather than Grove.

| row | family | artifact | first shared-safety obligation broken | evidence |
|---|---|---|---|---|
| Q4-101 | quint | the **reserved witness** | `FN-24.a` | argument — without evacuation a crash mid-deletion leaves a partially deleted root, which classifies two ways |
| Q4-102 | quint | the **evacuation manifest** | `FN-17.a` | argument — a rollback with nothing to compare against cannot be exact |
| Q4-103 | quint | its **ready mark** | `FN-24.a` | argument — a manifest interrupted mid-write is otherwise indistinguishable from a complete one |
| Q4-104 | quint | the **correlation ticket** | `FN-03` | argument — `FN-03`'s own witness is a retry with no local trace, which has nothing else to settle on |
| Q4-105 | quint | the **quarantine** | `none`, **as part of one bundle** | mutation — row 902, which removes 105, 106 and 107 together. Verdict `defer`, see below |
| Q4-106 | quint | the **cleanup marker** | `none`, **as part of one bundle** | mutation — row 902, which removes 105, 106 and 107 together. Verdict `defer`, see below |
| Q4-107 | quint | the **replace transition** | `none`, **as part of one bundle** | mutation — row 902, which removes 105, 106 and 107 together. Verdict `defer`, see below |
| Q4-108 | quint | the **index image** | — | abstracted |
| Q4-109 | quint | the **recorded anchor** | `FN-16.a` | argument — the rollback licence is stated over it, and without it no restoration can be refused for want of one |
| Q4-110 | quint | the **deletion fingerprint** | `FN-07` | mutation — row 914 |

**Three rows read `none`, and they are the same row three times — which is why
each of them says so in the cell rather than only in this paragraph.** The
quarantine, the cleanup marker and the replace transition are one mechanism:
`relax_EN_03` removes all three together, because the marker exists to make the
quarantine's disposal resumable and the replace transition exists to advance the
marker. Under the candidate every retained shared-safety obligation holds at a
bound no greater than the incumbent's, and the candidate's own successful exit is
reached in 46% of traces. What that says is that the three protect **Grove's own
intermediate artifacts**, not the user — which is Q4's
question asked precisely.

**What it does NOT say is `none` for each artifact removed on its own**, and the
matrix's own wording ("the first shared-safety obligation its removal breaks")
invites exactly that misreading. This column supplies **one bundled candidate
result**, not three independent removals: no control here removes the quarantine
while retaining the marker, or the marker while retaining the replace
transition. Rows Q4-101–104, Q4-109 and Q4-110 name the first direct
shared-safety obligation their artifact's role supports and are read normally;
Q4-108 is declared `abstracted`. 105 – 107 are one row with three names, and the
artifact-specific removals this paragraph offered to commission are what
`finish-verdicts-k78` left commissioned rather than declined — they are the
reason the three rows are `defer`.

**And `FN-31.a`'s witness DID land.** The replace transition is *reachable* in
the incumbent, and not marginally: the marker records disposal progress, so the
FIRST disposal step already carries a value the next step must supersede.
Measured in `scenario_march`, a state requiring a replacement is reached in
40.9% of traces against 28.0% that run a disposal to completion — the
replacement is forced before the disposal can finish, not occasionally after. So Q3's
answer from this column is not *delete*: the transition is needed by the
incumbent protocol, and what row Q4-107 says is that the incumbent protocol is
what needs it. Those are different findings and `finish-verdicts-k65` owns
which one decides Q3.

## The four verdicts, and what each one leaves this file owing

`finish-verdicts-k65` answered `TODO.finish_process.md` Q1 – Q4 **keep**;
`finish-verdicts-k77` attacked that reading and `finish-verdicts-k78` integrated
the result. **Q2 and Q3 stand. Q1, and rows Q4-105 – 107, are `defer`.** The
durable record is
[`finish-keeps-a-cleanup-layer-it-has-not-proved-forced`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md).
Four things this file handed up are settled here rather than left in a retired
task file.

**Q3 is decided by `FN-31.a`'s witness and not by row Q4-107.** The section above
names the two findings and asks which decides. It is the witness: Q4-107 says the
*incumbent* protocol is what needs the replace transition, which is a
classification of the artifact, while `FN-31.a` says a state requiring
replacement is **reached**, which is the reachability question Q3 actually asks.
The two do not conflict — Q3 is answered *within* the incumbent, and Q1 is the
only thing that could have made it moot. Q1 is `defer`, so Q3 stands until Q1
returns something else.

**The commission this column offered for Q4-105 – 107 was DECLINED, and the
declining argument does not survive.** `finish-verdicts-k65` declined it on the
ground that no control **can** separate the three: `ATOMIC_DISPOSAL` is a single
`const` whose true branch replaces `SQuarantineRename` and every step after it
with one `SDisposeInPlace`, so an artifact-specific removal is not a mutation of
the incumbent but a fourth candidate protocol. **The premise is true of this file
and the conclusion is not true of the protocol.** One `const` is a fact about
this encoding; a strategy dial that is not the capability dial would express
in-place *non-atomic* disposal, and the Alloy column has already run two of the
three separations in the available world (rows x1 and 45). What the coupling
measures is that **this model** cannot ask the question, which is the reason Q1
is `defer` rather than a reason it is `keep`.

**Q1's criterion is no longer open, and completing it is what decided Q1.**
`finish-verdicts-k78` ran the commission this file named rather than declining
it. `relax_EN_03` now differs from `base` in exactly one `const` — it had carried
`ENV_BUDGET = 0` with `ENV_PHASES` and `ENV_KINDS` empty, which narrows the
*world* the candidate is judged in rather than the candidate, and left
`inv_FN_24a`'s crash half with no crash and `inv_FN_32` with no foreign artifact
to meet. It now carries ten `wit_FN_24a_crash_at_<step>_under_the_candidate` over
the candidate's own effectful step list, `FN-32`'s reached antecedent, and a kill
control (row 902b); `scenario_march_under_the_candidate` carries `FN-24.b`'s two
branch enumerations. **All of them land, so the criterion is met as written — and
what running it to completion shows is that it decides nothing, twice over.**
First, met, it returns **`delete/replace`** for a protocol that requires the
atomic recursive deletion `EN-03` says does not exist: it is admissibility-typed.
Second, it is satisfiable while `FN-32`, one of its four retained claims, is
trivial over the difference it is judging — row 902b's own cell says where and
why. A criterion with either defect yields no verdict in **either** direction, so
Q1 is `defer`, and what it waits on is a control for the only *available*
no-quarantine strategy: non-atomic in-place disposal, which neither family runs.

**And rows Q4-105 – 107 are `defer` for a reason that is this column's own.**
Their `none` is **one bundled result from row 902**, which is `relax_EN_03` — the
counterfactual-capability module. By the rule this file's verdict section states
two paragraphs up, a counterfactual measures admissibility and says nothing about
the shipped world, so these three cells are not per-row availability evidence at
all: **this column supplies zero qualifying cells for Q4, not three.** The
paragraph above already said the bundling half in bold; what
`finish-verdicts-k78` adds is that the same rule that voids Q1's criterion voids
these cells, and that not applying it here was an inconsistency rather than a
second finding. Alloy's Q4-6 and Q4-7 are available-world mutations and are read
on their own terms — see the Alloy section — and the Quint face of Q4-6's hole is
real independently: `OWNERSHIP_PROVEN` is a free `const` rather than something
the marker's presence derives, so no control here can make removing the marker
cost a proof of ownership.

**And the Alloy column's `none` for Q4-5 is not a gap either.** *"Nothing here
runs that candidate"* reads as a shortfall and is not one: the assumption table
assigns `EN-03`'s mutation to **Quint**, one family per row, and
[`finish.als`](finish.als) runs no counterfactual-capability mutation at all —
every `EN_`-named command in it is `EN-02`, `EN-08`, `EN-09` or `EN-16`. The
positive control is `crates/grove-task-tree/models/task-tree.als`, which carries
`expect_unreachable_EN_04_promotion_is_never_observed_half_applied`, Alloy's own
assigned counterfactual. Q1's row is also the only one of the four whose
criterion does not say *in both families* — which stopped mattering when Q1 went
`defer`: the commissioned control is for the **available** candidate, and an
availability result Grove would act on is owed by both columns.
