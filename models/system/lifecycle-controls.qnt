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
// takes.  Twenty-three dials exist so that each such claim has somewhere to
// die: a mutant that leaves every obligation green is a finding that the claim
// was resting on the way the model happened to be written.
//
// AND A MUTANT THAT KILLS ITS NEIGHBOUR IS A DIFFERENT FACT FROM ONE THAT KILLS
// ITS TARGET.  `system-k59` found that `mutant_no_signal_is_done` was DOCUMENTED
// as a bundle control over all three session endings and in fact changed only
// one arm of the interpretation, leaving `SY-09.a` and `SY-09.b` with no control
// at all.  So the five ISOLATING mutants below each carry their neighbours as
// must-hold commands: "this target fails while its neighbour stays green" is a
// runner result here rather than a sentence in a README.
//
// WHAT THE SCENARIOS ARE FOR.  Every `SY-` PROPERTY is checked unfocused in
// `base`.  Six WITNESSES are not reachable there at an affordable sample
// count, and each has a `scenario_` instance that narrows the SEARCH — how many
// actions the world may take (`ENV_BUDGET`), of which kind (`ENV_KINDS`), at
// which lifecycle point (`ENV_PHASES`), and whether a live session takes its own
// work before ending (`FOCUS = 3`).  A scenario removes no behaviour from the
// model: it is a search strategy over exactly the space the assumptions grant,
// and every property stays checked unfocused in `base`, which is the discipline
// `system-k59` found `driverStep` had broken by narrowing in `base` itself.
// =============================================================================

/// THE FINISH MARCH AND THE FAR SIDE OF IT: enter a spent grove, append the
/// finish leaf, run it, prove the teardown, end with no signal, and scaffold
/// again on the far side.
///
/// ONE environment action, and it is a hand edit that installs the SPENT tree
/// the march starts from.  `EN-11` grants that any well-formed tree is
/// reachable by hand edit and `relax_EN_11` removes the whole family as a
/// control, so this is a search strategy over exactly the granted space — and
/// it buys the six driver moves the ordinary-session iteration would have
/// spent.  It has to: `system-k59` finding 1's repair costs two moves per reap
/// (collect, invalidate, then interpret in the next step, which is the order
/// `src/loop_driver.rs` takes), and the end-to-end march no longer fits inside
/// a twenty-four-step trace.  `FOCUS = 3` narrows the SEARCH so the finish
/// session takes its own work while it has any; every ending stays reachable
/// in `base`, which is where every property is checked.
module scenario_teardown {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0),
    FOCUS = 3,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 1,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(1)
  ).* from "./lifecycle"

  /// `SY-05.a`.  A completed teardown whose driver never observed the signal,
  /// followed by a fresh scaffold — which is the whole content of "absence
  /// means START A NEW GROVE and is never read as evidence about an earlier
  /// one".
  val wit_SY_05a_a_fresh_scaffold_after_an_unobserved_teardown: bool =
    hist.freshAfterUnobservedTeardown
  /// `SY-09.c`.  The same march reaches it: a teardown this driver PROVED,
  /// and a session that then ended with no signal at all.  It moved here from
  /// the library because `system-k59` finding 1's repair — a live session may
  /// end at any point — makes a twenty-step march a coin flip per step, and a
  /// witness that lands in one trace of eight thousand is not an instrument.
  val wit_SY_09c_no_signal_with_a_proven_teardown: bool =
    hist.noSignalWithProvenTeardown

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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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

/// AMBIENT ADMISSION AND NOTHING ELSE.  `SY-10.b`'s situation is an ambient
/// `grove-llm` operation arriving while the DRIVER holds the epoch lock
/// exclusively — the post-reap handoff — and that window is two steps wide in a
/// twenty-four-step trace, so a trace that wants one has to spend its whole
/// environment budget arriving.
module scenario_epoch_handoff {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 6,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(6)
  ).* from "./lifecycle"

  /// `SY-10.b`.  A contended generation reported as a visible timeout rather
  /// than parked.  It moved here from the library once `system-k59` finding 2's
  /// repair made the contention window the driver's real exclusive hold rather
  /// than a session's imagined continuous one.
  val wit_SY_10b_a_timeout_reported: bool = hist.genTimeoutReported
}

/// THE NULL HYPOTHESIS, AND IT IS COMMITTED SO THE COMPARISON CAN BE RE-RUN.
///
/// `system-k59` finding 5: the producer's "5 of 25 witnesses under a flat menu"
/// came from an instrument that reached no commit, and a reconstruction
/// disagreed with it by a factor of three.  This module IS that instrument now.
/// It differs from `base` in one constant — `FLAT_MENU` — so the comparison is
/// between two arrangements of the SAME actions with the same totality and the
/// same environment budget.
///
/// The MEASUREMENT is not a runner command, because a runner command is a claim
/// and this is an experiment: the number it produces is *how many witnesses
/// fail to land*, and a runner whose obligation is that witnesses land cannot
/// carry it.  The exact replay line is in the README's *The driver is
/// deterministic, and that is the search dial*, and it names this module.  The
/// one command below is the module's own positive control: a witness shallow
/// enough that the flat menu still reaches it, so a run that reached NOTHING is
/// distinguishable from a search this arrangement genuinely cannot make.
module scenario_flat_menu {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = true,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  /// `SY-01.a`, and shallow: two lease acquisitions, which any arrangement of
  /// the menu reaches.
  val wit_SY_01a_the_flat_menu_still_reaches_a_shallow_witness: bool =
    hist.secondDriverRefused
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = false,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = false,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_04b_a_transition_runs_without_validation: bool =
    inv_SY_04b_configuration_validation_precedes_every_transition

  /// THE RESTATED HALF, killed by the same dial — and NOT isolating from the
  /// conjunct above, which is recorded rather than hidden.  It is here because
  /// the restatement's whole point is that this half now HAS somewhere to die:
  /// its pair-stated predecessor (`refusalMutated`) was killed by no control in
  /// this file, because the only step it could look at was the refusal's own.
  /// With `CONFIG_FIRST` off a grove transition APPLIES under an invalid
  /// configuration and writes the tree, which the trace form sees and the pair
  /// form did not.
  val inv_fail_MUT_SY_04b_grove_writes_under_an_invalid_configuration: bool =
    inv_SY_04b_no_transition_runs_under_an_invalid_configuration
}

/// MODEL mutation, `SY-05.a`.  A missing task root is read as evidence about an EARLIER grove: the driver
  /// consults the completed teardown instead of starting a new one.
  ///
  /// SEARCHED LIKE `scenario_teardown`, and it has to be: the dial can fire only
  /// on a scaffold attempted on the far side of a PROVEN teardown, and
  /// `system-k59` finding 1's repair — a live session may end at any point —
  /// turned that march into a coin flip per step.  The mutation is unchanged;
  /// what changed is the search that reaches it.
module mutant_absence_is_evidence {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = false,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0),
    FOCUS = 3,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 1,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(1)
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = false,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_05b_absence_before_the_deletion_is_proven: bool =
    inv_SY_05b_the_summary_never_records_absence_before_the_deletion_is_proven
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = false,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = false,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = false,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = false,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = true,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_09c_no_signal_is_inferred_as_done: bool =
    inv_SY_09c_no_signal_stops_and_is_never_inferred_as_done

  /// THE TWO NEIGHBOURS, asserted rather than assumed.  The README used to say
  /// this dial killed all three endings as a bundle; `system-k59` finding 3
  /// showed it kills only this one, and these two commands are the evidence
  /// that it is ISOLATING rather than the evidence it was claimed to be.
  val inv_SY_09a_the_neighbour_stays_green: bool =
    inv_SY_09a_relaunch_continues_the_loop

  val inv_SY_09b_the_neighbour_stays_green: bool =
    inv_SY_09b_done_ends_the_loop
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = false,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = false,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = false,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = false,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = false,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
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
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = true,
    FLAT_MENU = false,
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

/// MODEL mutation, `SY-07.a`, and ISOLATING.  The DRIVER appends a second
/// finish leaf beside the one already there.  Session creation is left refused,
/// so `SY-07.b` is untouched and asserted green below — which is what
/// `system-k59`'s "each target fails while its neighbour remains green" asks
/// for, and what the old bundle could not give.
module mutant_two_finish_leaves {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = false,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_07a_two_driver_owned_finish_leaves: bool =
    inv_SY_07a_exactly_one_driver_owned_finish_leaf

  /// THE NEIGHBOUR, asserted rather than assumed.
  val inv_SY_07b_the_neighbour_stays_green: bool =
    inv_SY_07b_no_session_creates_a_finish_leaf
}

/// MODEL mutation, `SY-09.a`, and ISOLATING.  Only the RELAUNCH arm of the
/// signal interpretation moves: a relaunch stops the loop.  `done` and
/// `no signal` are untouched and both are asserted green below.
module mutant_relaunch_stops {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = true,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_09a_a_relaunch_stops_the_loop: bool =
    inv_SY_09a_relaunch_continues_the_loop

  val inv_SY_09b_the_neighbour_stays_green: bool =
    inv_SY_09b_done_ends_the_loop

  val inv_SY_09c_the_neighbour_stays_green: bool =
    inv_SY_09c_no_signal_stops_and_is_never_inferred_as_done
}

/// MODEL mutation, `SY-09.b`, and ISOLATING.  Only the DONE arm moves: a
/// done continues the loop.  `relaunch` and `no signal` are untouched and both
/// are asserted green below.
module mutant_done_continues {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = true,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_09b_a_done_continues_the_loop: bool =
    inv_SY_09b_done_ends_the_loop

  val inv_SY_09a_the_neighbour_stays_green: bool =
    inv_SY_09a_relaunch_continues_the_loop

  val inv_SY_09c_the_neighbour_stays_green: bool =
    inv_SY_09c_no_signal_stops_and_is_never_inferred_as_done
}

/// MODEL mutation, `SY-14.b`, and ISOLATING.  The block stays STICKY — so
/// nothing clears it and `SY-14.a` is untouched and asserted green below — but
/// every refusal on the blocked tree carries `NotLive`, which names no block.
/// Configured like `scenario_recovery`, because a `SY-14` control needs a
/// BLOCK, and a block needs one crash landing inside the transaction.
module mutant_block_unnamed {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = false,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 1,
    ENV_PHASES = Set(6, 7),
    ENV_KINDS = Set(0)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_14b_a_blocked_action_does_not_name_the_block: bool =
    inv_SY_14b_every_action_on_a_blocked_tree_refuses_naming_it

  val inv_SY_14a_the_neighbour_stays_green: bool =
    inv_SY_14a_no_admitted_action_clears_a_block
}

/// THE UNSCOPED CLAIM, WHICH IS THE CATALOGUE'S OWN WORDS, AND IT FIRES.
///
/// `SY-04.b` says an invalid configuration *leaves the working tree
/// byte-identical* and does not scope that to grove.  This module changes NO
/// dial — every constant is `base`'s — and asserts exactly that sentence: no
/// step at all, grove's or the operator's, moves a byte under an invalid
/// configuration.  A hand edit falsifies it, which is entry 042's Alloy
/// counterexample replayed in Quint, three-and-a-bit states long and needing no
/// protocol.
///
/// It is what makes the narrowing in `inv_SY_04b_grove_never_mutates…` a
/// DECLARATION rather than a silence: the model states the wide claim, watches
/// it die, and checks the half that is true.  The pair-stated predecessor could
/// not have run this control at all — `refusalMutated` never looked at a step
/// the operator took.
module mutant_unscoped_bytes {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_04b_the_operator_writes_under_an_invalid_configuration: bool =
    not(hist.worldMutatedUnderInvalidConfig)

  /// THE NEIGHBOUR, asserted green in the same run, so that the control above
  /// is killing the world half and not the claim itself.
  val inv_SY_04b_the_neighbour_stays_green: bool =
    inv_SY_04b_no_transition_runs_under_an_invalid_configuration
}

/// MODEL mutation, `SY-04.b`.  A VALIDATED CONFIGURATION IS A LICENCE.
///
/// **THIS MODULE IS WHERE A RETAINED COUNTEREXAMPLE BECAME A CONTROL**, which
/// is what a disposition costs when the finding was in the catalogue rather
/// than in the model.  Before `lifecycle-scope-k72` this counterexample fired
/// under `base`'s own constants with no dial moved, and it was RETAINED rather
/// than repaired because repairing it meant deciding what `SY-04.b` means —
/// a catalogue change the independence protocol forbade.  It is decided:
/// `SY-04.b` owes `SY-03`'s *a preflight is never a licence* for the
/// configuration as well as for the layout, the shipped driver already
/// revalidates (`SessionConfig::load` before the tree mutation and again before
/// the launch, `src/loop_driver.rs`), and `lifecycle.als` — which read the
/// configuration LIVE — was the column that had it right.
///
/// So `base` closes the licence and this dial reopens it.  `configChange` fires
/// after the driver recorded its validation, the gate consults that RECORDED
/// VERDICT rather than the live configuration, and the next transition writes
/// the tree with the operator's hands out of it.
///
/// **IT IS ISOLATING AND THAT IS ASSERTED IN THE SAME RUN.** The weak conjunct
/// — grove never mutates under a configuration it has NOT validated — stays
/// green under this dial, because the driver did validate; only the strong one
/// dies.  A control that killed both would not show which clause the disposition
/// added.
module mutant_config_licence {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = true,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_04b_a_validated_configuration_is_a_licence: bool =
    inv_SY_04b_no_transition_runs_under_an_invalid_configuration

  /// THE ISOLATION, asserted green in the same run: the driver DID validate, so
  /// the weak conjunct is untouched and the dial kills only the clause the
  /// disposition added.
  val inv_SY_04b_the_unvalidated_half_stays_green: bool =
    not(hist.groveMutatedUnderUnvalidatedConfig)
}

/// MODEL mutation, `SY-10.b`.  The visible stop borrows `SY-10.a`'s reason.
///
/// This is the ISOLATING control for the restated half.  `mutant_silent_park`
/// turns the timeout off entirely, so the admission parks and returns `ONone`;
/// under THIS dial the admission still stops, still stops visibly, and still
/// leaves nobody parked — the single thing wrong is that it answers
/// `REpochStale`, which is the reason for *this operation named a generation
/// that is not live*.  The old `not(hist.silentPark)` was green under it,
/// because a flag derived from `parked` cannot see a name.
module mutant_borrowed_stop_reason {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = false,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_10b_the_visible_stop_borrows_a_reason: bool =
    inv_SY_10b_a_contended_generation_never_parks_silently

  /// THE NEIGHBOUR, asserted green: `SY-10.a`'s own claim is untouched by a
  /// reason borrowed at the admission, which is exactly why borrowing it was
  /// invisible.
  val inv_SY_10a_the_neighbour_stays_green: bool =
    inv_SY_10a_a_stale_operation_is_refused_before_it_touches_the_tree
}

/// MODEL mutation, `SY-06.b`.  The witnessless arm falls through to `Legacy`.
///
/// **THE NARROWING THIS OBLIGATION *INHERITS*, MADE RUNNABLE IN THE SCOPE THAT
/// CITES IT.** `SY-06.b` is a declared cross-scope citation of `TT-18`/`TT-20`,
/// and `obligations-follow-context-not-artifact` clause 2 makes a citation carry
/// the cited obligation's declared narrowings.  `task-tree-scope-k70` changed
/// both of them, so this citation inherits a DIFFERENT strength rather than the
/// same one, and a citation whose strength moved is worth a command rather than
/// a sentence.
///
/// With the dial off, `classifyTree`'s ambiguous arm disappears and a root
/// carrying Grove's own root-init-exclusive entry beside a foreign one reads as
/// somebody else's legacy tree — which is exactly what `TT-20`'s narrowed
/// prohibition forbids past the boundary, and exactly the defect the split
/// exists to remove.
module mutant_ambiguity_unclassed {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = false,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_06b_grove_s_own_interrupted_work_reads_as_legacy: bool =
    inv_SY_06b_a_root_carrying_grove_s_own_proof_is_never_legacy

  /// THE NEIGHBOUR, green under the same dial: refusing to COMPLETE a legacy
  /// tree is untouched, so the control kills the classification and not the
  /// completion gate.
  val inv_SY_06b_the_completion_gate_stays_green: bool =
    inv_SY_06b_a_legacy_tree_is_never_completed_as_a_scaffold
}

/// MODEL mutation, `SY-13.a`.  GROVE MANUFACTURES ONE OF THE EXCLUDED STATES.
///
/// **THIS IS THE CONTROL THAT MAKES THE NARROWED SWEEP A CLAIM RATHER THAN A
/// CONVENIENCE, AND IT IS WHY THE COMPANION IS A CONJUNCT AND NOT A PREMISE.**
/// `SY-13` now quantifies over the stable states Grove's own admitted actions
/// reach, which is sound only while Grove reaches none of the others.  Left as
/// an assumption that is unfalsifiable: a Grove that walked the loop into
/// `Legacy` would satisfy the sweep by putting the tree somewhere the sweep
/// stopped looking.  With this dial the scaffold writes a root with neither the
/// exact subset nor a root-init-exclusive entry, so `initialise-root` itself
/// produces `Legacy` and the first conjunct dies.
///
/// The ESCAPE half is asserted green in the same run: the declared map is
/// untouched, so the control kills the boundary and not the reachability.
module mutant_grove_manufactures_legacy {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = true,
    BLOCK_NAMED_BY_ALL = false,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_13a_grove_manufactures_a_refusal_state: bool =
    inv_SY_13a_every_stable_class_escapes_within_the_bound

  val inv_SY_13b_the_sink_sweep_stays_green: bool =
    inv_SY_13b_no_stable_state_is_a_sink
}

/// MODEL mutation, `SY-14.b`.  The naming sweep runs over the LITERAL admitted
/// set.
///
/// **THE DISPOSED READING, RUN — which is what turns *the literal text is false*
/// from an argument into a fired control.** `lifecycle-scope-k72` decided that
/// `SY-14.b` is *every admitted action ON THE TREE*, on the argument both
/// families reached independently: a block is a property of the tree, so an
/// action that reads and writes no task tree cannot name a block it never read.
/// Under this dial the sweep demands that `acquire-lease`, `validate-config` and
/// `release-lease` refuse naming the block too, and the obligation dies —
/// meaning a blocked tree could not release its own lease, and `FN-26`'s two
/// operator-restorable exits would be unreachable.
///
/// `SY-14.a` is asserted green in the same run, which is the asymmetry the
/// disposition rests on: the CLEARING half is swept over the literal set and is
/// right to be.
module mutant_block_named_by_all {
  import lifecycle(
    CRASHES = true,
    HAND_EDITS = true,
    LEASE_QUEUES = false,
    DEATH_RELEASES = true,
    LAYOUT_FIRST = true,
    PREFLIGHT_LICENCE = false,
    CONFIG_LICENCE = false,
    SCAFFOLD_AMBIGUITY_CLASSED = true,
    GROVE_MANUFACTURES_LEGACY = false,
    BLOCK_NAMED_BY_ALL = true,
    ONE_TRANSITION = true,
    CONFIG_FIRST = true,
    DELETION_PROVEN_FIRST = true,
    ABSENCE_IS_FRESH = true,
    SCAFFOLD_LEAF = true,
    PARTIAL_BY_SUBSET = true,
    DRIVER_OWNS_FINISH = true,
    ONE_FINISH_LEAF = true,
    SELECT_ONCE = true,
    NO_SIGNAL_IS_DONE = false,
    RELAUNCH_ENDS_LOOP = false,
    DONE_CONTINUES_LOOP = false,
    GEN_CHECK = true,
    GEN_TIMEOUT = true,
    GEN_STOP_NAMED = true,
    GUARD_ORDER = true,
    RESUME_IS_IDEMPOTENT = true,
    BLOCK_STICKY = true,
    BLOCK_NAMED = true,
    SWEEP_ALL_STABLE = false,
    FLAT_MENU = false,
    PROCS = Set(0, 1),
    FOCUS = 0,
    ESCAPE_BOUND = 3,
    ENV_BUDGET = 3,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).* from "./lifecycle"

  val inv_fail_MUT_SY_14b_the_literal_quantifier_reaches_the_lease: bool =
    inv_SY_14b_every_action_on_a_blocked_tree_refuses_naming_it

  val inv_SY_14a_the_clearing_half_stays_green: bool =
    inv_SY_14a_no_admitted_action_clears_a_block
}
