// =============================================================================
// The controls and the focused scenarios for `finish.qnt`, in a file of their
// own.
//
// WHY THIS IS A SECOND FILE, and it is not tidiness.  `quint verify` loads and
// serialises every module in the file it is given, and one file holding the
// library, its instances, six assumption mutations, seven model mutations and
// ten focused scenarios exceeds quint 0.32.0's own result reporter — the whole
// model is then beyond model checking, at any depth, for a reason that has
// nothing to do with the model.  `crates/grove-task-tree/models/` learned that
// the hard way (entry 044) and this column inherits the remedy rather than
// rediscovering it.
//
// Every module here is prefixed `relax_`, `mutant_` or `scenario_`, so each
// carries ONLY the commands written inside it — see `models/run.sh`,
// *THE MODULE RULE*, which is where that convention is defined.
//
// WHAT THE SCENARIOS ARE FOR, stated plainly, because it is the one thing about
// this column a reader should not have to infer.  Every `FN-` PROPERTY is
// checked unfocused in `base`.  Every `FN-` WITNESS lives in a `scenario_`
// instance, because the subject is a twenty-step transaction and an unfocused
// search reaches its end with probability (1/k)^20: the deep claims would be
// reported green on witnesses that never landed, which is precisely the
// pre-registration's *vacuous invariant* hazard.  A scenario removes NO
// behaviour — every action stays in the action set — it caps how many
// environment actions one trace may take (`ENV_BUDGET`), at which program
// points (`ENV_PHASES`) and of which kind (`ENV_KINDS`).  That is a search
// strategy over exactly the space the environment assumptions already grant.
// =============================================================================

// -----------------------------------------------------------------------------
// The uninterrupted protocol.  No environment action at all: both branches of
// the transaction, end to end, and everything the nondeterminism INSIDE the
// commit can produce.
// -----------------------------------------------------------------------------
module scenario_march {
  import finish(
    RENAME_ATOMIC = true, ATOMIC_DISPOSAL = false, ATOMIC_COMMIT = false,
    CRASHES = true, FOREIGN_WRITES = true, REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false, LANES = Set(0, 1, 2),
    REVALIDATE = true, CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true, OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true, PREFLIGHT_PURE = true, PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-01.a`.  A transaction never entered for want of confirmation.
  val wit_FN_01a_never_entered_for_want_of_confirmation: bool =
    hist.neverEnteredForConfirmation

  /// `FN-02`.  A decline followed by a later successful attempt on the same
  /// handle.
  val wit_FN_02_a_decline_then_a_success_on_the_same_handle: bool =
    hist.declinedThenSucceeded

  /// `FN-03`.  A retry with no local trace of the attempt, settling forward on
  /// the ticket alone.
  val wit_FN_03_a_retry_with_no_local_trace: bool =
    hist.settledForwardOnTicketAlone

  /// `FN-31.a`'s companion in the same trace: disposal ran to the end, so the
  /// marker's whole life — create, replace, remove — is on one trace.
  val wit_FN_21a_disposal_ran_to_completion: bool =
    hist.stepsVisited.contains(SRemoveQuarantine)

  /// `FN-11`.  The interval between publication and commit, with the task root
  /// present, unwalkable and holding every entry.
  val wit_FN_11_the_interval_between_publication_and_commit: bool =
    hist.evacuationCompleteBeforeCommit

  /// `FN-15.a`.  A result reported as failure while the exact commit exists —
  /// classified `Committed`.
  val wit_FN_15a_a_lost_result_classified_committed: bool =
    hist.lostResultClassifiedCommitted

  /// `FN-15.b`, `FN-15.c`, `FN-15.d` — reached on EACH lane.
  val wit_FN_15b_committed_on_git: bool = hist.committedOnLane.contains(0)
  val wit_FN_15b_committed_on_native_jj: bool = hist.committedOnLane.contains(1)
  val wit_FN_15b_committed_on_colocated_jj: bool = hist.committedOnLane.contains(2)
  val wit_FN_15c_notcommitted_on_git: bool = hist.notCommittedOnLane.contains(0)
  val wit_FN_15c_notcommitted_on_native_jj: bool = hist.notCommittedOnLane.contains(1)
  val wit_FN_15c_notcommitted_on_colocated_jj: bool = hist.notCommittedOnLane.contains(2)
  val wit_FN_15d_indeterminate_on_git: bool = hist.indeterminateOnLane.contains(0)
  val wit_FN_15d_indeterminate_on_native_jj: bool = hist.indeterminateOnLane.contains(1)
  val wit_FN_15d_indeterminate_on_colocated_jj: bool = hist.indeterminateOnLane.contains(2)

  /// `FN-17.a`.  A restoration that reproduces the manifest exactly.
  val wit_FN_17a_a_restoration_that_reproduces_the_manifest: bool =
    hist.restorationExact

  /// `FN-22.a`.  Each of the four revalidation points, reached.
  val wit_FN_22a_before_restoration: bool = hist.revalPerformed.contains(SRevalBeforeRestore)
  val wit_FN_22a_after_restoration: bool = hist.revalPerformed.contains(SRevalAfterRestore)
  val wit_FN_22a_before_the_quarantine_rename: bool = hist.revalPerformed.contains(SRevalBeforeQuarantine)
  val wit_FN_22a_after_the_quarantine_rename: bool = hist.revalPerformed.contains(SRevalAfterQuarantine)

  /// `FN-22.d`.  An unchanged `NotCommitted` after restoration settles to a
  /// manifest-matching root, witness removed, finish leaf live.
  val wit_FN_22d_notcommitted_after_restoration_settles: bool =
    hist.settledAfterRestore

  /// `FN-22.i`.  An unchanged `Committed` after the rename applies.
  val wit_FN_22i_committed_after_the_rename_applies: bool =
    hist.appliedAfterQuarantine

  /// `FN-22.j`.  `Indeterminate` observed somewhere other than after the
  /// quarantine rename: no handoff, and a block.
  val wit_FN_22j_indeterminate_blocks_with_no_handoff: bool =
    hist.indeterminateBlockedAt.size() > 0

  /// `FN-23`.  Three consecutive recoveries, the second and third changing
  /// nothing.
  val wit_FN_23_three_consecutive_recoveries: bool = hist.noopRecoveries >= 2

  /// `FN-24.b`.  THE STEP LIST, ENUMERATED — each step's persistent effect
  /// named, on each of the transaction's two branches.  Two commands rather
  /// than one because no single trace takes both branches within the runner's
  /// depth, and the enumeration is the union of the two.
  val wit_FN_24b_the_refusal_branchs_step_list_enumerated: bool =
    REFUSAL_BRANCH_STEPS.subseteq(hist.stepsVisited)
  val wit_FN_24b_the_success_branchs_step_list_enumerated: bool =
    SUCCESS_BRANCH_STEPS.subseteq(hist.stepsVisited)

  /// `FN-25.b`.  THE BOUNDED SWEEP, reported as what it is.  The obligation asks
  /// for an exhaustive sweep of the blocked states; what randomized simulation
  /// supplies is a block SITE reached, carrying the diagnosis the independent
  /// classifier gives its own state.  The previous form was
  /// `blockedStatesSeen > 0` — one reached block, named as an exhaustive sweep,
  /// and asserting nothing about the diagnosis it carried.
  ///
  /// A TRACE REACHES AT MOST ONE SITE, because a block is terminal; the sweep's
  /// width is a property of the 8000 samples and of the twelve scenarios, not of
  /// any one trace.  Which is the honest statement, and it is why this is a
  /// witness that a site was swept rather than a claim that all of them were.
  val wit_FN_25b_the_blocked_states_swept_within_the_bound: bool =
    hist.blockSitesSwept.size() > 0 and
    not(hist.blockedWithoutDiagnosis) and
    not(hist.carriedDisagreed)

  /// `FN-25.c`.  `RecoveryPending`, on each lane.
  val wit_FN_25c_recovery_pending_on_git: bool = hist.recoveryPendingOnLane.contains(0)
  val wit_FN_25c_recovery_pending_on_native_jj: bool = hist.recoveryPendingOnLane.contains(1)
  val wit_FN_25c_recovery_pending_on_colocated_jj: bool = hist.recoveryPendingOnLane.contains(2)

  /// `FN-26`.  A block whose diagnostic names the artifact holding the
  /// transaction and the recorded and observed topology.
  val wit_FN_26_a_block_naming_the_artifact_and_the_topology: bool =
    hist.blockNamedArtifactAndTopology

  /// `FN-27`.  Nothing unrelated changed, on each of the three outcomes.
  val wit_FN_27a_a_success_with_nothing_unrelated_changed: bool =
    hist.appliedAfterQuarantine and not(hist.unrelatedMutated)
  val wit_FN_27b_a_refusal_with_nothing_unrelated_changed: bool =
    hist.settledAfterRestore and not(hist.unrelatedMutated)
  val wit_FN_27c_a_block_with_nothing_unrelated_changed: bool =
    hist.blockedStatesSeen > 0 and not(hist.unrelatedMutated)

  /// `FN-28`.  A success whose cleanup is still outstanding.
  val wit_FN_28_a_success_whose_cleanup_is_outstanding: bool =
    hist.successWithCleanupOutstanding

  /// `FN-29`.  A refused attempt followed by a successful one.
  val wit_FN_29a_a_refusal_then_a_success: bool = hist.refusedThenSucceeded

  /// `FN-31.a`.  The source state from which disposal must REPLACE an existing
  /// marker rather than create or remove one.  This is Q3's evidence, and it is
  /// a reachability fact rather than a construction: the marker is present,
  /// owned, and carries a value the next disposal step must supersede.
  val wit_FN_31a_a_state_requiring_a_replacement: bool = hist.replaceRequired

  /// `FN-31.b`.  An observation interleaved with the replacement — the staged
  /// name beside the live one, and exactly one marker visible.
  val wit_FN_31b_an_observation_during_the_replacement: bool =
    hist.markerObservedDuringReplace
}

// -----------------------------------------------------------------------------
// The gates.  Perturbations while the tree is at rest, and nothing during a
// transaction: the seven preflight preconditions, the reaper, and the ordinary
// operation's own view.
// -----------------------------------------------------------------------------
module scenario_gates {
  import finish(
    RENAME_ATOMIC = true, ATOMIC_DISPOSAL = false, ATOMIC_COMMIT = false,
    CRASHES = true, FOREIGN_WRITES = true, REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false, LANES = Set(0, 1, 2),
    REVALIDATE = true, CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true, OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true, PREFLIGHT_PURE = true, PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-01.b`.  A transaction refused for want of the DETERMINISTIC GUARD,
  /// distinct from a transaction never entered for want of confirmation.
  val wit_FN_01b_refused_for_want_of_the_deterministic_guard: bool =
    hist.refusedByDeterministicGuard

  /// `FN-04`.  Two attempts on one handle, the earlier ticket rejected by the
  /// later.
  val wit_FN_04_an_earlier_ticket_rejected_by_a_later_attempt: bool =
    hist.earlierTicketRejected

  /// `FN-24.a`.  `Current(Live)` — the stable task-root state the catalogue
  /// names for a root with live ordinary work — REACHED and classified to
  /// exactly one state.  It was previously unreachable: the classifier read a
  /// function hard-coded to `false`, so a branch `FN-24.a` quantifies over could
  /// not be exercised.  The state is set by the environment's own hand edit,
  /// the same edit the `PCGuards` refusal is stated over.
  val wit_FN_24a_current_live_is_a_reached_stable_state: bool =
    stableStates(w) == Set(RCurrentLive)

  /// `FN-05.a`.  EACH OF THE SEVEN, reached.  Seven commands rather than one:
  /// the catalogue's own wording is "each of the seven", and the seven are
  /// mutually masking — the preflight returns the FIRST failure — so no single
  /// trace reaches them all.
  val wit_FN_05a_confirmation_absent: bool = hist.preconditionsHit.contains(PCConfirmation)
  val wit_FN_05a_deterministic_guards: bool = hist.preconditionsHit.contains(PCGuards)
  val wit_FN_05a_layout_unsupported: bool = hist.preconditionsHit.contains(PCLayout)
  val wit_FN_05a_quarantine_operand: bool = hist.preconditionsHit.contains(PCQuarantineOperand)
  val wit_FN_05a_root_identity: bool = hist.preconditionsHit.contains(PCRootIdentity)
  val wit_FN_05a_empty_fingerprint: bool = hist.preconditionsHit.contains(PCFingerprint)
  val wit_FN_05a_undigestible_entry: bool = hist.preconditionsHit.contains(PCEntryType)

  /// `FN-05.b`.  Each of the seven, with the TREE unchanged.
  val wit_FN_05b_confirmation_absent: bool =
    hist.preconditionsHit.contains(PCConfirmation) and not(hist.preflightMovedTree)
  val wit_FN_05b_deterministic_guards: bool =
    hist.preconditionsHit.contains(PCGuards) and not(hist.preflightMovedTree)
  val wit_FN_05b_layout_unsupported: bool =
    hist.preconditionsHit.contains(PCLayout) and not(hist.preflightMovedTree)
  val wit_FN_05b_quarantine_operand: bool =
    hist.preconditionsHit.contains(PCQuarantineOperand) and not(hist.preflightMovedTree)
  val wit_FN_05b_root_identity: bool =
    hist.preconditionsHit.contains(PCRootIdentity) and not(hist.preflightMovedTree)
  val wit_FN_05b_empty_fingerprint: bool =
    hist.preconditionsHit.contains(PCFingerprint) and not(hist.preflightMovedTree)
  val wit_FN_05b_undigestible_entry: bool =
    hist.preconditionsHit.contains(PCEntryType) and not(hist.preflightMovedTree)

  /// `FN-05.c`.  Each of the seven, with the REPOSITORY unchanged.
  val wit_FN_05c_confirmation_absent: bool =
    hist.preconditionsHit.contains(PCConfirmation) and not(hist.preflightMovedRepo)
  val wit_FN_05c_deterministic_guards: bool =
    hist.preconditionsHit.contains(PCGuards) and not(hist.preflightMovedRepo)
  val wit_FN_05c_layout_unsupported: bool =
    hist.preconditionsHit.contains(PCLayout) and not(hist.preflightMovedRepo)
  val wit_FN_05c_quarantine_operand: bool =
    hist.preconditionsHit.contains(PCQuarantineOperand) and not(hist.preflightMovedRepo)
  val wit_FN_05c_root_identity: bool =
    hist.preconditionsHit.contains(PCRootIdentity) and not(hist.preflightMovedRepo)
  val wit_FN_05c_empty_fingerprint: bool =
    hist.preconditionsHit.contains(PCFingerprint) and not(hist.preflightMovedRepo)
  val wit_FN_05c_undigestible_entry: bool =
    hist.preconditionsHit.contains(PCEntryType) and not(hist.preflightMovedRepo)

  /// `FN-07`.  A wholly untracked tree.
  val wit_FN_07_a_wholly_untracked_tree: bool =
    hist.preconditionsHit.contains(PCFingerprint)

  /// `FN-08`.  A layout that PASSES at lease acquisition and fails at the
  /// transaction's own rename operands.
  val wit_FN_08_a_layout_that_passes_the_lease_and_fails_here: bool =
    hist.operandGateCaughtWhatLeaseMissed

  /// `FN-12.b`.  A refused entry type.
  val wit_FN_12b_a_refused_entry_type: bool =
    hist.preconditionsHit.contains(PCEntryType)

  /// `FN-14`.  Unrelated modified work present across a successful finish.
  val wit_FN_14_unrelated_work_survives_a_successful_finish: bool =
    hist.unrelatedSurvivedSuccess

  /// `FN-17.b`.  A restoration that cannot reproduce the exact preflight commit
  /// blocks rather than proceeds.
  val wit_FN_17b_a_restoration_that_cannot_reproduce_blocks: bool =
    hist.restorationBlocked

  /// `FN-21.c`.  A reaper declining a FOREIGN entry at a reserved name,
  /// mutating nothing, while the sweep continues over what it can prove is
  /// Grove's.
  val wit_FN_21c_the_reaper_declines_a_foreign_entry: bool =
    hist.reaperDeclinedForeign

  /// `FN-30`.  A hook that would have run, shown suppressed.
  val wit_FN_30_a_hook_shown_suppressed: bool = hist.hookSuppressed
}

// -----------------------------------------------------------------------------
// One interruption inside the witness build or at the commit.
// -----------------------------------------------------------------------------
module scenario_crash {
  import finish(
    RENAME_ATOMIC = true, ATOMIC_DISPOSAL = false, ATOMIC_COMMIT = false,
    CRASHES = true, FOREIGN_WRITES = true, REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false, LANES = Set(0, 1, 2),
    REVALIDATE = true, CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true, OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true, PREFLIGHT_PURE = true, PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-09.a`.  An interruption immediately after publication.
  val wit_FN_09a_an_interruption_immediately_after_publication: bool =
    hist.crashPoints.contains(SEvacuate)

  /// `FN-09.b`.  An interruption inside the build.
  val wit_FN_09b_an_interruption_inside_the_build: bool =
    hist.crashPoints.contains(SWriteManifest) or hist.crashPoints.contains(SPublish)

  /// `FN-10.a`.  A discard of an unpublished witness.
  val wit_FN_10a_an_unpublished_witness_discarded: bool = hist.unpublishedDiscarded

  /// `FN-12.a`.  A manifest interrupted before its ready mark, recovered as NOT
  /// READY.
  val wit_FN_12a_a_manifest_recovered_as_not_ready: bool = hist.notReadyRecovered

  /// `FN-24.a`.  ONE CRASH POINT PER STEP — the witness-build and commit half.
  /// Eighteen commands, one per step with a persistent effect: a crash at a
  /// revalidation point makes nothing persistent and is therefore
  /// indistinguishable from a crash at the boundary before it, which is why the
  /// enumeration is over the effectful steps rather than over all of them.
  val wit_FN_24a_crash_at_create_preparing: bool = hist.crashPoints.contains(SCreatePreparing)
  val wit_FN_24a_crash_at_write_manifest: bool = hist.crashPoints.contains(SWriteManifest)
  val wit_FN_24a_crash_at_publish: bool = hist.crashPoints.contains(SPublish)
  val wit_FN_24a_crash_at_evacuate: bool = hist.crashPoints.contains(SEvacuate)
  val wit_FN_24a_crash_at_mark_ready: bool = hist.crashPoints.contains(SMarkReady)
  val wit_FN_24a_crash_at_commit: bool = hist.crashPoints.contains(SCommit)
}

// -----------------------------------------------------------------------------
// One interruption in the handoffs or in disposal.
// -----------------------------------------------------------------------------
module scenario_crash_late {
  import finish(
    RENAME_ATOMIC = true, ATOMIC_DISPOSAL = false, ATOMIC_COMMIT = false,
    CRASHES = true, FOREIGN_WRITES = true, REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false, LANES = Set(0, 1, 2),
    REVALIDATE = true, CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true, OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true, PREFLIGHT_PURE = true, PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(3, 4, 5, 6), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-19`.  An interruption immediately after the rename, leaving a complete
  /// quarantine and an absent task root.
  val wit_FN_19_an_interruption_immediately_after_the_rename: bool =
    hist.crashPoints.contains(SRevalAfterQuarantine)

  /// `FN-20`.  A leftover artifact present while the tree is classified FRESH.
  val wit_FN_20_a_leftover_artifact_while_the_tree_reads_fresh: bool =
    hist.leftoverWhileFresh

  /// `FN-21.a`.  A resumed disposal.

  /// `FN-31.c`.  An interruption at each step of the replacement, resumed to
  /// the same terminal state.

  /// `FN-24.a`, continued: the handoff and disposal half.
  val wit_FN_24a_crash_at_restore: bool = hist.crashPoints.contains(SRestore)
  val wit_FN_24a_crash_at_reproduce_preflight: bool = hist.crashPoints.contains(SReproducePreflight)
  val wit_FN_24a_crash_at_remove_witness: bool = hist.crashPoints.contains(SRemoveWitness)
  val wit_FN_24a_crash_at_quarantine_rename: bool = hist.crashPoints.contains(SQuarantineRename)
}

// -----------------------------------------------------------------------------
// One hand edit inside the transaction: the root swapped between two steps, and
// the witness put under version control.
// -----------------------------------------------------------------------------
module scenario_edit_txn {
  import finish(
    RENAME_ATOMIC = true, ATOMIC_DISPOSAL = false, ATOMIC_COMMIT = false,
    CRASHES = true, FOREIGN_WRITES = true, REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false, LANES = Set(0, 1, 2),
    REVALIDATE = true, CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true, OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true, PREFLIGHT_PURE = true, PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2), ENV_KINDS = Set(1),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-06`.  A swap between two steps, whose mutation is not applied
  /// elsewhere.
  val wit_FN_06_a_root_swap_between_two_steps: bool = hist.identitySwapCaught

  /// `FN-06`, IN THE CATALOGUE'S OWN WORD: the swap REFUSED.  Available exactly
  /// while the transaction has made nothing persistent; past that point
  /// *One artifact, three contexts* takes over and the outcome is a block.  The
  /// catalogue states only the first, which is the content of
  /// `docs/formalism-findings.md` entry 045, finding 6.
  val wit_FN_06_a_root_swap_between_two_steps_refused: bool =
    hist.identitySwapRefused

  /// `FN-13`.  A commit attempted while the witness is tracked, refused.
  val wit_FN_13_a_commit_attempted_with_the_witness_tracked: bool =
    hist.trackedWitnessRefused

  /// `FN-25.a`.  The NEAR MISS resolved to exactly one diagnosis: a Grove-owned,
  /// correlated artifact sitting at a name Grove also reserves.
  val wit_FN_25a_a_near_miss_resolved_to_one: bool = hist.nearMissResolvedToOne

  /// `FN-25.c`.  `OwnershipConflict`, on each lane.
  val wit_FN_25c_ownership_conflict_on_git: bool = hist.ownershipConflictOnLane.contains(0)
  val wit_FN_25c_ownership_conflict_on_native_jj: bool = hist.ownershipConflictOnLane.contains(1)
  val wit_FN_25c_ownership_conflict_on_colocated_jj: bool = hist.ownershipConflictOnLane.contains(2)
}

// -----------------------------------------------------------------------------
// One topology change at a revalidation point.  This is where the four points
// of `FN-22` earn their existence: the disposition CHANGES between two of them.
// -----------------------------------------------------------------------------
module scenario_reval {
  import finish(
    RENAME_ATOMIC = true, ATOMIC_DISPOSAL = false, ATOMIC_COMMIT = false,
    CRASHES = true, FOREIGN_WRITES = true, REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false, LANES = Set(0, 1, 2),
    REVALIDATE = true, CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true, OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true, PREFLIGHT_PURE = true, PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(3), ENV_KINDS = Set(3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-16.a`.  Restoration refused because the recorded anchor no longer
  /// holds.
  val wit_FN_16a_restoration_refused_for_want_of_the_anchor: bool =
    hist.rollbackRefusedAnchor

  /// `FN-16.b`.  Restoration refused because the attempt-bound result is
  /// present.
  val wit_FN_16b_restoration_refused_for_a_present_result: bool =
    hist.rollbackRefusedResult

  /// `FN-18`.  A proven commit reached after an interruption mid-evacuation —
  /// and the tree not reconstructed.
  val wit_FN_18_a_proven_commit_after_an_interrupted_evacuation: bool =
    hist.divertedBeforeRestore

  /// `FN-22.b`.  Before restoration, `Committed` diverts and restores nothing.
  val wit_FN_22b_committed_before_restoration_diverts: bool =
    hist.divertedBeforeRestore

  /// `FN-22.c`.  After restoration, `Committed` leaves the witness blocking the
  /// restored tree.
  val wit_FN_22c_committed_after_restoration_blocks: bool =
    hist.blockedAfterRestore

  /// `FN-22.e`.  Before the quarantine rename, `NotCommitted` diverts and
  /// renames nothing.
  val wit_FN_22e_notcommitted_before_the_rename_diverts: bool =
    hist.divertedBeforeQuarantine

  /// `FN-22.f`.  After the rename, `Committed -> NotCommitted` returns the
  /// quarantine atomically and completes as a refusal.
  val wit_FN_22f_the_quarantine_returned_and_refused: bool = hist.returnedRefused

  /// `FN-22.g`.  After the rename, `Committed -> Indeterminate` returns the
  /// quarantine and blocks.
  val wit_FN_22g_the_quarantine_returned_and_blocked: bool = hist.returnedBlocked
}

// -----------------------------------------------------------------------------
// One foreign write at a name Grove reserves, in each of the places this scope
// meets one.  Three scenarios rather than one, because the three situations
// need different companions: an unclassifiable PREPARING witness needs a crash
// to have left one, an orphaned quarantine needs a hand edit to have installed
// one, and a foreign cleanup marker needs the quarantine to exist first —
// `SQuarantineRename` builds a fresh quarantine, so a marker written before it
// is overwritten and the write has to land after.
// -----------------------------------------------------------------------------
module scenario_foreign_marker {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(6), ENV_KINDS = Set(2),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-31.d`.  A foreign marker, declined: a replacement is never performed
  /// against a marker Grove cannot prove is its own.
  val wit_FN_31d_a_foreign_marker_declined: bool = hist.replaceDeclinedForeign

  /// `FN-32`'s MARKER HALF, WITNESSED WHERE A STEP COULD ACTUALLY MUTATE IT.
  /// The library witness lands beside a `Discard`, which frames the marker
  /// unconditionally and so cannot probe the ownership gate at all; this one is
  /// the replacement meeting a marker it cannot prove is its own and leaving it
  /// standing.  Same state `wit_FN_31d` reaches — deliberately, because that is
  /// the ONE reachable site where the marker's ownership bit is read, and
  /// `FN-32` says so as shared safety where `FN-31.d` says it as incumbent
  /// mechanics.
  val wit_FN_32_a_transaction_step_meets_an_unprovable_marker: bool =
    hist.replaceDeclinedForeign and not(hist.mutatedUnprovenMarker)
}

module scenario_unclassifiable {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 2, ENV_PHASES = Set(0, 1), ENV_KINDS = Set(0, 2),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-10.b`.  A refusal to discard content the discard cannot classify as
  /// Grove's own — the unpublished witness is never interpreted and never
  /// removed.
  val wit_FN_10b_a_refusal_to_discard_unclassifiable_content: bool =
    hist.unpublishedRefused
}

module scenario_orphan {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 2, ENV_PHASES = Set(0, 1), ENV_KINDS = Set(0, 1),
    ENV_EDITS = Set(12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-21.b`.  A reaper declining an entry whose in-tree witness still owns
  /// it: an orphaned quarantine beside a live, correlated witness.
  val wit_FN_21b_the_reaper_declines_an_owned_entry: bool = hist.reaperDeclinedOwned
}

// -----------------------------------------------------------------------------
// The quarantine's RETURN, which only exists once the disposition has changed
// under a completed rename.  Two scenarios: one that makes the return fail, and
// one that interrupts it.
// -----------------------------------------------------------------------------
module scenario_return_blocked {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 2, ENV_PHASES = Set(0, 3), ENV_KINDS = Set(1, 3),
    ENV_EDITS = Set(8), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(1), ENTRIES = 1
  ).* from "./finish"

  /// `FN-22.h`.  A return that cannot complete reports both the change and the
  /// quarantine, and blocks.
  val wit_FN_22h_a_return_that_cannot_complete: bool = hist.returnIncomplete
}

module scenario_return_crash {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 2, ENV_PHASES = Set(3, 5), ENV_KINDS = Set(0, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(1), ENTRIES = 1
  ).* from "./finish"

  /// `FN-24.a`, the last crash boundary: inside the quarantine's return.
  val wit_FN_24a_crash_at_return_quarantine: bool =
    hist.crashPoints.contains(SReturnQuarantine)
}

// -----------------------------------------------------------------------------
// One interruption inside DISPOSAL, which is where the cleanup marker's
// replacement lives and therefore where `FN-31.c` is.
// -----------------------------------------------------------------------------
module scenario_crash_disposal {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(6), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  /// `FN-21.a`.  A resumed disposal.
  val wit_FN_21a_a_resumed_disposal: bool = hist.disposalResumed

  /// `FN-31.c`.  An interruption at each step of the replacement, resumed to
  /// the same terminal state an uninterrupted replacement reaches.
  val wit_FN_31c_an_interrupted_replacement_resumed: bool =
    hist.replaceResumedTerminal

  /// `FN-24.a`, the disposal crash boundaries.
  val wit_FN_24a_crash_at_create_marker: bool = hist.crashPoints.contains(SCreateMarker)
  val wit_FN_24a_crash_at_replace_stage: bool = hist.crashPoints.contains(SReplaceStage)
  val wit_FN_24a_crash_at_replace_exchange: bool = hist.crashPoints.contains(SReplaceExchange)
  val wit_FN_24a_crash_at_replace_cleanup: bool = hist.crashPoints.contains(SReplaceCleanup)
  val wit_FN_24a_crash_at_dispose_entry: bool = hist.crashPoints.contains(SDisposeEntry)
  val wit_FN_24a_crash_at_remove_marker: bool = hist.crashPoints.contains(SRemoveMarker)
  val wit_FN_24a_crash_at_remove_quarantine: bool = hist.crashPoints.contains(SRemoveQuarantine)
}

// =============================================================================
// THE ASSUMPTION MUTATIONS.
//
// `docs/specs/semantic-contract.md`, *Environment assumptions*, assigns each
// mutation to a family and states its EXPECTED RESULT, and the three classes do
// not expect the same thing:
//
//   premise-break            a NAMED OBLIGATION FAILS.  `inv_fail_` is inverted
//                            on purpose — a control whose whole content is
//                            "this claim dies" cannot be reported by a runner
//                            that treats every failing check as a defect.
//   exercise-removal         NAMED WITNESSES BECOME UNREACHABLE and the
//                            properties stay green.  `wit_unreach_`.
//   counterfactual-capability  every SHARED-SAFETY obligation stays green, and
//                            the incumbent-mechanics claims are out of scope for
//                            the candidate.  Ordinary `inv_` commands, and their
//                            greenness is the evidence.
// =============================================================================

/// `EN-01`, premise-break.  A rename observable half-applied.  Expected: the
/// publication is observable torn, so `FN-09.a` fails — and `FN-24.a` fails with
/// it, because the torn state is classifiable as TWO stable states, which is
/// the exact thing an atomic rename is bought to prevent.
module relax_EN_01 {
  import finish(
    RENAME_ATOMIC = false,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_EN_01_FN_09a_a_torn_publication_is_observable: bool =
    inv_FN_09a_publication_is_one_atomic_rename

  val inv_fail_EN_01_FN_24a_a_torn_rename_is_two_stable_states: bool =
    inv_FN_24a_every_interruption_lands_in_exactly_one_state
}

/// `EN-03`, COUNTERFACTUAL CAPABILITY — and this is Q1's whole instrument.
/// Granting an atomic recursive directory deletion lets a proven commit be
/// settled by disposing of the evacuation witness IN PLACE: no quarantine, no
/// handoff, no cleanup marker, and the three nested transactions
/// `docs/adr/finish-layers-are-forced-not-chosen.md` measures collapse to one.
///
/// What is checked here is the RETAINED shared-safety set and nothing else.
/// `FN-19`, `FN-21` and `FN-31` are the incumbent mechanics the candidate
/// REPLACES; their failure under it would not be evidence, and their absence
/// from this module is deliberate rather than an omission.
///
/// THE RETAINED SET NAMES `FN-32` WHERE IT ONCE NAMED `TT-24`, and the swap is
/// `finish-verdicts-k65`'s.  `TT-24`'s transaction context IS `FN-32` — the
/// letters `c` and `d` are retired — and a `TT-` obligation cannot be
/// discharged from this scope at all, so a retained set naming it was a
/// criterion this module could never meet.  Nothing is dropped: the claim is
/// asserted where its context lives.
module relax_EN_03 {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = true,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_FN_20_a_leftover_artifact_is_never_a_receipt_under_the_candidate: bool =
    inv_FN_20_a_leftover_artifact_is_never_a_receipt
  val inv_FN_24a_one_stable_state_under_the_candidate: bool =
    inv_FN_24a_every_interruption_lands_in_exactly_one_state
  val inv_FN_24b_one_declared_effect_per_step_under_the_candidate: bool =
    inv_FN_24b_every_step_has_one_declared_effect
  val inv_FN_27a_nothing_unrelated_on_success_under_the_candidate: bool =
    inv_FN_27a_nothing_unrelated_changes_on_success
  val inv_FN_27b_nothing_unrelated_on_refusal_under_the_candidate: bool =
    inv_FN_27b_nothing_unrelated_changes_on_refusal
  val inv_FN_27c_nothing_unrelated_on_a_block_under_the_candidate: bool =
    inv_FN_27c_nothing_unrelated_changes_on_a_block
  val inv_FN_32_ownership_still_proven_under_the_candidate: bool =
    inv_FN_32_a_transaction_never_mutates_what_it_cannot_prove_is_its_own

  /// The candidate's own successful exit, reached — which is what makes the
  /// green above mean something rather than being green over a world in which
  /// nothing happened.
  val wit_FN_28_the_candidate_reaches_its_successful_exit: bool =
    hist.stepsVisited.contains(SDisposeInPlace)
}

/// `EN-05`, COUNTERFACTUAL CAPABILITY — Q2's instrument.  With the commit inside
/// the filesystem transaction, the transaction's own atomicity is the receipt:
/// it applied or it did not, and there is no third answer to read.
///
/// `FN-15.d`'s BOUNDED-UNREACHABILITY branch is what this module supplies, per
/// lane.  It is evidence that `Indeterminate` is unreachable WITHIN 8000 samples
/// at depth 24 — never a proof that it is unreachable, and the README says so
/// where it is read.
module relax_EN_05 {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = true,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_FN_03_the_ticket_still_carries_the_record: bool =
    inv_FN_03_the_ticket_is_the_durable_record
  val inv_FN_15a_still_never_reads_an_exit_status: bool =
    inv_FN_15a_the_classification_never_reads_an_exit_status
  val inv_FN_24a_one_stable_state_under_an_atomic_commit: bool =
    inv_FN_24a_every_interruption_lands_in_exactly_one_state
  val inv_FN_25a_the_precedence_still_selects_the_diagnosis: bool =
    inv_FN_25a_the_carried_diagnosis_is_the_one_precedence_selects
  val inv_FN_25b_the_diagnoses_stay_exhaustive: bool =
    inv_FN_25b_the_diagnoses_are_exhaustive
  val inv_FN_27a_nothing_unrelated_on_success_under_an_atomic_commit: bool =
    inv_FN_27a_nothing_unrelated_changes_on_success

  val wit_unreach_EN_05_indeterminate_on_git: bool = hist.indeterminateOnLane.contains(0)
  val wit_unreach_EN_05_indeterminate_on_native_jj: bool = hist.indeterminateOnLane.contains(1)
  val wit_unreach_EN_05_indeterminate_on_colocated_jj: bool = hist.indeterminateOnLane.contains(2)

  /// …and the commit is still REACHED, so the three unreachability results
  /// above are about the disposition rather than about a world in which nothing
  /// was ever committed.
  val wit_FN_15b_committed_is_still_reached_on_git: bool = hist.committedOnLane.contains(0)
}

/// `EN-08`, exercise-removal.  `crash` removed.  Expected: every witness that
/// posits an interruption becomes unreachable, and the run fails on ZERO WORK
/// rather than reporting green.
module relax_EN_08 {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = false,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2, 3, 4, 5, 6), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val wit_unreach_EN_08_an_interruption_after_publication: bool =
    hist.crashPoints.contains(SEvacuate)
  val wit_unreach_EN_08_an_interruption_inside_the_build: bool =
    hist.crashPoints.contains(SWriteManifest)
  val wit_unreach_EN_08_an_unpublished_witness_discarded: bool =
    hist.unpublishedDiscarded
  val wit_unreach_EN_08_a_manifest_recovered_as_not_ready: bool =
    hist.notReadyRecovered
  val wit_unreach_EN_08_a_resumed_disposal: bool = hist.disposalResumed
  val wit_unreach_EN_08_an_interrupted_replacement_resumed: bool =
    hist.replaceResumedTerminal
}

/// `EN-13`, premise-break.  The reaper sweeps its reserved namespace instead of
/// touching only what carries Grove's own cleanup manifest.  Expected: the sweep
/// deletes bytes a refusal exists to preserve, so `FN-27` fails — and `FN-21.b`
/// fails with it, because the same relaxation stops the reaper asking whether a
/// live in-tree witness still owns the entry.
module relax_EN_13 {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = true,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_EN_13_FN_27a_the_sweep_deletes_foreign_bytes: bool =
    inv_FN_27a_nothing_unrelated_changes_on_success

  val inv_fail_EN_13_FN_21b_the_reaper_stops_asking_whose_it_is: bool =
    inv_FN_21b_the_reaper_touches_only_groves_own
}

/// `EN-15`, COUNTERFACTUAL CAPABILITY.  A machine-attested confirmation.
/// Expected, and it is the unusual one: **no obligation becomes stronger and
/// none fails.**  An attestation replaces nothing — `FN-01.a` still forbids
/// running without confirmation and `FN-01.b` still refuses the deterministic
/// guard as a substitute.  A run in which some obligation DOES strengthen is
/// the finding, because it would mean a claim was resting on the attestation
/// rather than on the guard.
module relax_EN_15 {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = true,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_FN_01a_no_step_runs_without_confirmation_under_attestation: bool =
    inv_FN_01a_no_step_runs_without_confirmation
  val inv_FN_01b_guards_are_still_not_a_substitute: bool =
    inv_FN_01b_guards_are_not_a_substitute

  val wit_FN_01a_still_never_entered_for_want_of_confirmation: bool =
    hist.neverEnteredForConfirmation
  val wit_FN_01b_still_refused_by_the_deterministic_guard: bool =
    hist.refusedByDeterministicGuard
}

/// `EN-16`, exercise-removal.  The lane collapsed to one.  Expected: the
/// per-lane witnesses of `FN-15.b`, `FN-15.c`, `FN-15.d` and `FN-25.c` become
/// unreachable while every `FN-` property stays green — which is exactly what
/// makes a collapsed lane dimension INVISIBLE without this control, and is the
/// reason the catalogue exercises `EN-16` rather than relaxing it.
module relax_EN_16 {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val wit_unreach_EN_16_committed_on_native_jj: bool = hist.committedOnLane.contains(1)
  val wit_unreach_EN_16_notcommitted_on_colocated_jj: bool = hist.notCommittedOnLane.contains(2)
  val wit_unreach_EN_16_indeterminate_on_native_jj: bool = hist.indeterminateOnLane.contains(1)
  val wit_unreach_EN_16_recovery_pending_on_colocated_jj: bool =
    hist.recoveryPendingOnLane.contains(2)

  val inv_FN_15a_still_never_reads_an_exit_status_on_one_lane: bool =
    inv_FN_15a_the_classification_never_reads_an_exit_status
  val inv_FN_17a_rollback_is_still_exact_on_one_lane: bool =
    inv_FN_17a_rollback_is_exact
  val inv_FN_25c_a_diagnosis_is_still_its_own_lanes: bool =
    inv_FN_25c_a_diagnosis_is_recorded_against_its_own_lane
}

// =============================================================================
// THE MODEL MUTATIONS.
//
// A control on the MODEL rather than on the world.  Each exists because some
// obligation would otherwise be true BY CONSTRUCTION — satisfied by the way an
// executable model is naturally written rather than by anything the protocol
// does — and a claim true by construction is the pre-registration's *vacuous
// invariant* hazard wearing a green tick.  Every one of these is inverted:
// the named obligation MUST die.
// =============================================================================

/// The disposition read off the reported exit status rather than off the
/// evidence.  `FN-15.a` is otherwise satisfied by an evidence-classifying model
/// simply because that is how one writes it.
module mutant_status_classifier {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = false,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_15a_the_exit_status_read_as_a_receipt: bool =
    inv_FN_15a_the_classification_never_reads_an_exit_status
}

/// The four revalidation points not performed: the handoff acts on the
/// disposition it classified before it started.
module mutant_no_revalidation {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = false,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(3), ENV_KINDS = Set(3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_22b_a_committed_result_restored_anyway: bool =
    inv_FN_22b_committed_before_restoration_diverts
}

/// The cleanup marker replaced by a remove followed by a create, so a reader
/// can observe the marker ABSENT.  Without this instance `FN-31.b` is a
/// statement about the way the model happens to write one line.
module mutant_nonatomic_marker {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = false,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_31b_a_reader_observes_the_marker_absent: bool =
    inv_FN_31b_the_replacement_is_atomic_for_readers
}

/// The transaction mutating what it cannot prove is its own.
module mutant_unproven_ownership {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = false,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(0, 1, 2), ENV_KINDS = Set(2),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_10b_unclassifiable_content_mutated_anyway: bool =
    inv_FN_10b_unclassifiable_content_fails_closed

  /// `FN-32`, THROUGH THE WITNESS SLOT AND ONLY THROUGH IT.  This module's
  /// environment (`ENV_PHASES = Set(0, 1, 2)`, `ENV_FOREIGN = Set(0)`) never
  /// reaches a foreign cleanup marker, so the kill this control records is the
  /// slot site's — which is exactly why it is stated over the slot flag rather
  /// than over the aggregate.  Stated over the aggregate it read as evidence
  /// about both artifacts `FN-32` names while probing one, which is the hole
  /// `obligation-placement-k67` found.  The marker's own control is
  /// `mutant_unproven_marker` below.
  val inv_fail_MUT_FN_32_a_transaction_mutates_a_slot_it_cannot_prove: bool =
    not(hist.mutatedUnprovenSlot)
}

/// THE SAME DIAL, IN THE ONE ENVIRONMENT THAT REACHES A FOREIGN CLEANUP MARKER.
/// `scenario_foreign_marker` is what demonstrates that state is reachable at all
/// (`wit_FN_31d`), and it needs `ENV_PHASES = Set(6)` and `ENV_FOREIGN = Set(2)`
/// to get there; this module is that scenario with `OWNERSHIP_PROVEN` turned
/// off, so the replacement that would have DECLINED against a marker it cannot
/// prove is its own performs it instead.
///
/// WHAT IT BUYS, STATED AS THE THING IT WOULD CATCH.  `FN-32`'s marker half is
/// green in both families, and the only step whose marker mutation is gated on
/// ownership is the replacement — every other step frames the marker
/// unconditionally.  So the half could be satisfied by an antecedent nothing
/// reaches and no run of the suite would say so.  This control is the run that
/// says so: if the flag is never set, the command does not fail and the RUNNER
/// goes red on it, because an `inv_fail_` that holds is a control that proved
/// nothing.
module mutant_unproven_marker {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = false,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(6), ENV_KINDS = Set(2),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_32_a_transaction_mutates_a_marker_it_cannot_prove: bool =
    not(hist.mutatedUnprovenMarker)
}

/// A recovery that finds nothing outstanding and REPAIRS anyway.
module mutant_eager_recovery {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = false,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 0, ENV_PHASES = Set(), ENV_KINDS = Set(),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_23_a_recovery_at_rest_changes_the_tree: bool =
    inv_FN_23_recovery_is_idempotent
}

/// A preflight that builds the witness and records its intent BEFORE it has
/// finished checking.
module mutant_eager_preflight {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = false,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_05b_a_failing_precondition_moved_the_tree: bool =
    inv_FN_05b_a_failing_precondition_leaves_the_tree
  val inv_fail_MUT_FN_05c_a_failing_precondition_moved_the_repository: bool =
    inv_FN_05c_a_failing_precondition_leaves_the_repository
}

/// A preflight that checks only confirmation and the deterministic guards.
/// Four obligations die at once, and that is the point: each of them is
/// otherwise satisfied by the ORDER the model happens to check things in.
module mutant_short_preflight {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = false,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_06_the_identity_is_never_verified: bool =
    inv_FN_06_the_root_identity_is_pinned_and_rechecked
  val inv_fail_MUT_FN_07_an_untracked_tree_is_evacuated: bool =
    inv_FN_07_an_untracked_tree_is_refused_before_evacuation
  val inv_fail_MUT_FN_08_the_operand_gate_is_skipped: bool =
    inv_FN_08_the_operand_gate_is_never_satisfied_by_the_lease
  val inv_fail_MUT_FN_12b_an_undigestible_entry_is_evacuated: bool =
    inv_FN_12b_an_undigestible_entry_is_refused_before_mutation
}

/// The fresh-tree classification reading a leftover artifact as evidence that a
/// finish happened.  `FN-20` is checked as a TWO-WORLD comparison, and a
/// two-world comparison proves nothing unless there is a world in which the
/// classifier actually does it.
module mutant_receipt_reading {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = true,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(3, 4, 5, 6), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_20_a_leftover_artifact_read_as_a_receipt: bool =
    inv_FN_20_a_leftover_artifact_is_never_a_receipt
}

/// An operator hook that RUNS during the internal commit.  Without this
/// instance `FN-30` is green because the model writes `hooksRan: false` — a
/// claim about one line rather than about the protocol.  The hook has to be
/// installed for the mutation to bite, so the instance perturbs at rest.
module mutant_hooks_run {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = false,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_30_a_hook_ran_on_an_internal_commit: bool =
    inv_FN_30_internal_commits_run_without_hooks
}

/// A commit that is NOT scoped: it takes the unrelated working-copy bytes with
/// it.  `FN-14` and `FN-27.a` are both asserted over a field no transition
/// otherwise sets, so without this instance both are green by construction.
module mutant_unscoped_commit {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = false,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 8, ENV_PHASES = Set(0), ENV_KINDS = Set(1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_14_the_commit_took_unrelated_work: bool =
    inv_FN_14_the_commit_is_scoped
}

/// A block "cleared" by rewriting the recorded history out from under it.
/// `FN-26` forbids exactly that, and nothing else in the model can perform it —
/// so without this instance the obligation is green on the absence of a
/// transition rather than on the protocol refusing to make one.
module mutant_history_rewritten {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = false,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2), ENV_KINDS = Set(1),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_26_a_block_cleared_by_rewriting_history: bool =
    inv_FN_26_history_is_never_rewritten
}


/// §*States* WITHOUT `Reserved(Quarantined)` — the six rows this scope reaches
/// without it, as the table stood before `finish-scope-k71`.  This is
/// `FN-24.a`'s fourth conjunct's own alternative, and what it produces is the
/// finding: a disposal that has released its reserved witness while its
/// quarantine still stands, and `EN-11`'s orphaned quarantine beside a task root
/// that is still present, both read as an ORDINARY grove.  That is what
/// §*States*' load-bearing property forbids, and without the row nothing in this
/// column said so.
module mutant_no_quarantined_state {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = false,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2, 3, 4, 5, 6), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_24a_a_standing_quarantine_reads_as_an_ordinary_grove: bool =
    inv_FN_24a_every_interruption_lands_in_exactly_one_state
}

/// §*States*' FORMER table ORDER, taken literally: `Absent` first and the whole
/// `Reserved` class after it.  It is a mutation to the ranking rather than to
/// one edge of it, which is the point — a precedence relation is transitively
/// redundant by construction, so a disk's classification survives the loss of
/// any single pair on a neighbouring one.  What it produces is `FN-24.a`'s third
/// conjunct: the disk an interruption immediately after the quarantine rename
/// leaves reads `Absent` while `FN-22`'s fourth revalidation point is still
/// ahead of it — which is the trace `SY-05.b` says does not exist and which
/// `SY-05.a` would scaffold a fresh grove over.
module mutant_absent_classified_first {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = false,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = true,
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2, 3, 4, 5, 6), ENV_KINDS = Set(0),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_24a_the_post_rename_disk_reads_as_absent: bool =
    inv_FN_24a_every_interruption_lands_in_exactly_one_state
}

/// `FN-25.a`'s PRECEDENCE, REVERSED — correlation wins the declared overlap, so
/// a disk carrying an artifact Grove cannot classify beside a correlated attempt
/// is reported as a recovery the operator should go and run.
///
/// THIS IS THE CONTROL THE OLD STATEMENT COULD NOT HAVE.  Until
/// `finish-scope-k76` this column's `FN-25.a` said *the two diagnoses are
/// disjoint*, which `diagnose`'s if/else chain made true by construction — no
/// mutation could move it, because no mutation could make the classifier return
/// two answers.  The claim is now the precedence, and the precedence is a dial,
/// so this module is what says the check tests something.  The Alloy column ran
/// the same mutation against ITS old form and it was green (`README.md`, matrix
/// row 65), which is the same finding reached from the other side.
module mutant_correlation_wins_the_overlap {
  import finish(
    RENAME_ATOMIC = true,
    ATOMIC_DISPOSAL = false,
    ATOMIC_COMMIT = false,
    CRASHES = true,
    FOREIGN_WRITES = true,
    REAPER_SWEEPS_RESERVED = false,
    ATTESTED_CONFIRMATION = false,
    LANES = Set(0, 1, 2),
    REVALIDATE = true,
    CLASSIFY_FROM_EVIDENCE = true,
    MARKER_REPLACE_ATOMIC = true,
    OWNERSHIP_PROVEN = true,
    RECOVERY_IDEMPOTENT = true,
    PREFLIGHT_PURE = true,
    PREFLIGHT_COMPLETE = true,
    QUARANTINE_IS_A_RESERVED_STATE = true,
    RESERVED_BEFORE_ABSENT = true,
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    OWNERSHIP_WINS_THE_OVERLAP = false,
    // `scenario_edit_txn`'s ENVIRONMENT, and it is not interchangeable with the
    // other mutants'.  The overlap `FN-25.a` is about is reached only by an
    // in-transaction hand edit (`ENV_KINDS = Set(1)`) in the two phases that
    // module opens; `wit_FN_25a_a_near_miss_resolved_to_one` lands there and
    // nowhere else in this file.  A mutant that copies the neighbouring modules'
    // environment reports GREEN — measured, on the first attempt at this module
    // — and a green mutant reads as a surviving claim when it is really an
    // unreached one.
    ENV_BUDGET = 1, ENV_PHASES = Set(1, 2), ENV_KINDS = Set(1),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), ENV_FOREIGN = Set(0, 1, 2), ENV_TOPOS = Set(0, 1, 2, 3, 4, 5), ENTRIES = 1
  ).* from "./finish"

  val inv_fail_MUT_FN_25a_correlation_wins_the_overlap: bool =
    inv_FN_25a_the_carried_diagnosis_is_the_one_precedence_selects
}
