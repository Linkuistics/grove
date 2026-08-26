// =============================================================================
// The controls and the focused scenarios for `lifecycle.qnt`, in a file of its
// own.
//
// WHY A SECOND FILE, and it is not tidiness.  `quint verify` loads and
// serialises every module in the file it is given, and one file holding the
// library, its instances and twenty-odd control modules exceeds quint 0.32.0's
// own result reporter — the whole model is then beyond model checking at any
// depth, for a reason that has nothing to do with the model.
// `crates/grove-task-tree/models/` learned that the hard way and this column
// inherits the remedy rather than rediscovering it.
//
// Every module here is prefixed `relax_`, `mutant_` or `scenario_`, so each
// carries ONLY the commands written inside it — see `models/run.sh`,
// *THE MODULE RULE*, which is where that convention is defined.
//
// WHAT THE MUTANTS ARE FOR, and it is the single most important thing about
// this column.  An executable model of a DETERMINISTIC LOOP satisfies most of
// its own ordering claims by being written in that order — the
// pre-registration's *vacuous invariant* hazard in the form an executable model
// takes.  Nineteen MODEL dials exist so that each such claim has somewhere to
// die: a mutant that leaves every obligation green is a finding that the claim
// was resting on the way the model happened to be written.
//
// WHAT THE SCENARIOS ARE FOR.  Every `SY-` PROPERTY is checked unfocused in
// `base`.  Four WITNESSES are not reachable there at an affordable sample
// count, and each has a `scenario_` instance that narrows the ENVIRONMENT —
// how many actions the world may take (`ENV_BUDGET`) and of which kind
// (`ENV_KINDS`).  A scenario removes no behaviour from the model: it is a
// search strategy over exactly the space the environment assumptions grant.
// =============================================================================

/// THE UNINTERRUPTED LIFECYCLE, end to end and then some: scaffold, run,
/// exhaust, finish, and scaffold again on the far side of a proven teardown.
/// No environment action at all, because `SY-05.a` needs twenty-one driver
/// moves inside a twenty-four-step trace and every world action spends one.
module scenario_teardown {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 0,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set()
  ).* from "./lifecycle"

  /// `SY-05.a`.  A completed teardown whose driver never observed the signal,
  /// followed by a fresh scaffold — which is the whole content of "absence
  /// means START A NEW GROVE and is never read as evidence about an earlier
  /// one".
  val wit_SY_05a_a_fresh_scaffold_after_an_unobserved_teardown: bool =
    hist.freshAfterUnobservedTeardown
}

/// CRASH, AND NOTHING ELSE.  `EN-08` grants interruption between any two
/// steps, and `SY-12`'s witness is one crash point per lifecycle step — which
/// needs a trace that spends its whole environment budget on crashes.
module scenario_crash_points {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 8,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0)
  ).* from "./lifecycle"

  /// `SY-12`.  Four distinct lifecycle points, each crashed at and each
  /// continued from.
  val wit_SY_12_a_crash_at_each_lifecycle_point: bool =
    hist.crashPoints.size() >= 4
}

/// ONE CRASH, LANDING INSIDE THE TRANSACTION.  A finish interrupted between
/// two of its steps is what `recover` is for, and a recovery that cannot prove
/// the attempt either way is what a BLOCK is — which is the state `SY-14`'s
/// exhaustive sweep is run against.
module scenario_recovery {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 1,
    ENV_PHASES = Set(6, 7),
    ENV_KINDS = Set(0)
  ).* from "./lifecycle"

  /// `SY-14.a`.  The whole admitted action set, swept against a blocked tree in
  /// one transition, through the same classifier the real actions use.
  val wit_SY_14a_an_exhaustive_sweep_against_a_blocked_tree: bool =
    hist.sweepDone

  /// `SY-14.b`.  The same sweep, with every refusal naming the block.
  val wit_SY_14b_the_same_sweep_each_refusal_naming_the_block: bool =
    hist.sweepDone and hist.sweepAllNamed
}

/// `EN-08` — interruption may occur between any two steps.  EXERCISE-REMOVAL:
/// the mutation is `crash`'s removal, and the control is UNREACHABILITY rather
/// than failure.  With no crash there is no lifecycle point to restart from,
/// and `SY-12`'s witness has nowhere to land — which is precisely why a green
/// run under a collapsed dimension is false confidence.
module relax_EN_08 {
  import lifecycle(
    CRASHES = false,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val wit_unreach_EN_08_a_crash_at_a_lifecycle_point: bool =
    hist.crashPoints.size() > 0
}

/// `EN-11` — any well-formed tree is reachable by hand edit.
/// EXERCISE-REMOVAL.  `Legacy`, `Foreign` and `Malformed` are states Grove's
/// OWN actions never build, so removing `hand-edit` removes all three — which
/// is also the evidence behind this column's narrowing of `SY-13`: the three
/// sink classes are exactly the hand-edit-reached ones.
module relax_EN_11 {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = false,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val wit_unreach_EN_11_a_legacy_tree_refused: bool = hist.legacyRefused
}

/// MODEL mutation, `SY-01.a`.  A second driver that QUEUES is the whole of what `SY-01.a` forbids, and an
  /// executable model refuses it by simply not having a queue — so the queue has
  /// to exist somewhere for the claim to be falsifiable at all.
module mutant_lease_queue {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = true,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_01a_a_second_driver_is_queued: bool =
    inv_SY_01a_a_second_driver_is_refused_never_queued
}

/// MODEL mutation, `SY-01.b`.  Ownership released by return but not by death: the successor never runs.
module mutant_death_holds {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = false,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_01b_the_lease_survives_a_death: bool =
    inv_SY_01b_death_releases_ownership
}

/// MODEL mutation, `SY-02`.  The layout is proved at the first gate that needs it rather than at lease
  /// acquisition, so configuration validation and the first observation of the
  /// tree both run in front of it.
module mutant_late_layout {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = false,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_02_the_tree_is_touched_before_the_layout_is_proved: bool =
    inv_SY_02_the_layout_is_proved_at_lease_acquisition
}

/// MODEL mutation, `SY-03`.  Every later gate consults what the lease saw instead of its own operands —
  /// which is exactly a preflight used as a LICENCE.
module mutant_preflight_licence {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = true,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_03_a_later_gate_honours_the_lease_time_check: bool =
    inv_SY_03_no_gate_consults_an_earlier_layout_check
}

/// MODEL mutation, `SY-04.a`.  The iteration's transition cap removed.
module mutant_many_transitions {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = false,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_04a_two_transitions_in_one_iteration: bool =
    inv_SY_04a_at_most_one_lifecycle_transition_per_iteration
}

/// MODEL mutation, `SY-04.b`.  A lifecycle transition may run before the configuration has been validated.
module mutant_unvalidated {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = false,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_04b_a_transition_runs_without_validation: bool =
    inv_SY_04b_configuration_validation_precedes_every_transition
}

/// MODEL mutation, `SY-05.a`.  A missing task root is read as evidence about an EARLIER grove: the driver
  /// consults the completed teardown instead of starting a new one.
module mutant_absence_is_evidence {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = false,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 0,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set()
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_05a_absence_is_read_as_evidence: bool =
    inv_SY_05a_absence_is_never_read_as_evidence
}

/// MODEL mutation, `SY-05.b`.  The task root is removed one step before the deletion is proven, which is
  /// the composition failure `SY-05` exists to exclude: `SY-05.a`'s inference
  /// becomes unsound the moment `FN-11`/`FN-19` stop holding.
module mutant_early_deletion {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = false,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_05b_absence_before_the_deletion_is_proven: bool =
    inv_SY_05b_no_trace_exposes_absence_before_the_deletion_is_proven
}

/// MODEL mutation, `SY-06.a`.  A fresh grove indistinguishable from a finished one.
module mutant_charter_only {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = false,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_06a_a_scaffold_writes_only_a_charter: bool =
    inv_SY_06a_a_completed_scaffold_carries_a_live_leaf
}

/// MODEL mutation, `SY-06.b`.  `PartialScaffold` recognised by the MERE ABSENCE of the format witness
  /// rather than by the catalogue's exact known subset — which is the one
  /// sentence `SY-06.b` spends most of its words on.
module mutant_absent_witness {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = false,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_06b_a_legacy_tree_is_completed_as_a_scaffold: bool =
    inv_SY_06b_a_legacy_tree_is_never_completed_as_a_scaffold
}

/// MODEL mutation, `SY-07.b`.  A session may create the finish leaf, which is also how a second one appears.
module mutant_session_finish {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = false,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_07b_a_session_creates_a_finish_leaf: bool =
    inv_SY_07b_no_session_creates_a_finish_leaf
}

/// MODEL mutation, `SY-08`.  Selection recomputed at launch, so a leaf added during the launch window
  /// preempts the iteration's own work instead of becoming the next one's.
module mutant_recompute_selection {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = false,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_08_a_leaf_added_during_the_window_preempts: bool =
    inv_SY_08_a_leaf_added_during_the_window_does_not_preempt
}

/// MODEL mutation, `SY-09.c`.  A session that returned no signal is recorded as DONE — including one that
  /// had committed a teardown, which is the case the claim names.
module mutant_no_signal_is_done {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = true,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_09c_no_signal_is_inferred_as_done: bool =
    inv_SY_09c_no_signal_stops_and_is_never_inferred_as_done
}

/// MODEL mutation, `SY-10.a`.  An ambient operation touches the tree before matching the live launch
  /// generation.
module mutant_no_gen_check {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = false,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_10a_a_stale_session_touches_the_tree: bool =
    inv_SY_10a_a_stale_operation_is_refused_before_it_touches_the_tree
}

/// MODEL mutation, `SY-10.b`.  A contended generation parks instead of timing out into a visible stop.
module mutant_silent_park {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = false,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_10b_a_contended_generation_parks_silently: bool =
    inv_SY_10b_a_contended_generation_never_parks_silently
}

/// MODEL mutation, `SY-12`.  The resume restarts the transaction from its first step, repeating effects
  /// the interrupted attempt had already completed.
module mutant_restart_repeats {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = false,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 4,
    ENV_PHASES = Set(6, 7, 8),
    ENV_KINDS = Set(0)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_12_a_restart_repeats_a_completed_effect: bool =
    inv_SY_12_a_restart_never_silently_repeats_a_completed_effect
}

/// MODEL mutation, `SY-14.a`.  An admitted action clears a block, so the operator's two restorable exits
  /// are not the only way out and a blocked tree stops being terminal.
module mutant_block_clears {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = false,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 1,
    ENV_PHASES = Set(6, 7),
    ENV_KINDS = Set(0)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_14a_an_admitted_action_clears_a_block: bool =
    inv_SY_14a_no_admitted_action_clears_a_block
}

/// MODEL mutation, `SY-11.a` and `SY-11.b`.  The guard ORDER is not enforced:
/// a caller may take the tree guard before the launch generation, and may wait
/// for a generation while holding a tree guard.  The second is the edge that
/// closes the cycle [`bulk-marks-are-not-atomic`] records, and it is why the
/// order is a claim rather than a convention.  `FOCUS = 2` is the contention
/// menu: `SY-11`'s obligations are not reached by a loop that never contends.
module mutant_guard_disorder {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = false,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = false,
    PROCS = Set(0, 1),
    FOCUS = 2,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 4,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_11a_the_guard_order_is_violated: bool =
    inv_SY_11a_lease_then_generation_then_tree

  val inv_fail_MUT_SY_11b_a_generation_wait_under_a_tree_guard_closes_a_cycle: bool =
    inv_SY_11b_no_cycle_and_no_generation_wait_under_a_tree_guard
}

/// MODEL mutation, `SY-13.b`, AND THE ONE THAT IS A FINDING ABOUT THE
/// CATALOGUE RATHER THAN ABOUT THE MODEL.
///
/// `SY-13` quantifies over "any stable state" and names exactly TWO terminal
/// dispositions, explicitly excluding `Malformed` from them.  `Legacy`,
/// `Foreign` and `Malformed` are reached by a hand edit and left by a hand
/// edit, and a hand edit is not an admitted action — `SY-13`'s own note puts
/// operator actions outside the admitted set by construction.  So under the
/// LITERAL text every one of the three is a sink and `SY-13.a`/`SY-13.b` are
/// false.  `base` narrows the sweep to the classes Grove's own actions can
/// produce and declares the narrowing; this instance runs the literal text and
/// the claim dies, which is what turns "the catalogue is wrong here" from a
/// remark into a fired control.
module mutant_literal_sy13 {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    SWEEP_ALL_STABLE = true,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_13b_a_hand_edited_refusal_state_is_a_sink: bool =
    inv_SY_13b_no_stable_state_is_a_sink
}

