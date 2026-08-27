// =============================================================================
// The controls for `task-tree.qnt`, in a file of their own.
//
// WHY THIS IS A SECOND FILE, and it is not tidiness.  `quint verify` loads and
// serialises every module in the file it is given, and one file holding the
// library, its base instance, six assumption mutations, a model mutation and
// four focused scenarios exceeds V8's maximum string length in quint 0.32.0's
// own result reporter — the whole model is then beyond model checking, at any
// depth, for a reason that has nothing to do with the model.  Splitting the
// controls out is what puts `task-tree.qnt` back within reach of Apalache.
//
// Every module here is prefixed `relax_`, `mutant_` or `scenario_`, so each
// carries ONLY the commands written inside it — see `models/run.sh`,
// *THE MODULE RULE*, which is where that convention is defined.
//
// The controls themselves are NOT verified and do not need to be: a premise
// break is a claim that some obligation FAILS, and simulation finding the
// counterexample is exactly the evidence wanted.  What model checking buys is
// the other direction, and that direction lives in the file this one imports.
// =============================================================================

/// `EN-01`, premise-break.  A same-directory rename observable half-applied.
/// Expected: `TT-20` fails — the torn witness classifies as a DIFFERENT stable
/// state, which is exactly what an atomic rename is bought to prevent.
module relax_EN_01 {
  import taskTree(
    RENAME_ATOMIC = false,
    FOREIGN_WRITES = true, CRASHES = true, ENTRIES_REMOVABLE = false,
    HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_EN_01_TT_20_a_torn_witness_is_not_a_partial_scaffold: bool =
    inv_TT_20_an_interrupted_scaffold_is_a_partial_scaffold
}

/// `EN-06`, exercise-removal.  The non-cooperating writer removed.  Expected:
/// `TT-21.b`'s witness becomes unreachable while `TT-22` stays green — which is
/// the content of the assumption, and the reason `TT-21` cannot claim to
/// exclude a direct edit.
module relax_EN_06 {
  import taskTree(
    FOREIGN_WRITES = false,
    RENAME_ATOMIC = true, CRASHES = true, ENTRIES_REMOVABLE = false,
    HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_unreach_EN_06_a_foreign_write_between_two_classifications: bool =
    wit_TT_21b_a_foreign_write_between_two_classifications

  val inv_TT_22a_still_admits_two_observations_without_a_foreign_writer: bool =
    inv_TT_22a_two_observations_are_admitted

  val inv_TT_22b_still_serializes_without_a_foreign_writer: bool =
    inv_TT_22b_observation_and_mutation_are_serialized
}

/// `EN-08`, exercise-removal.  `crash` removed.  Expected: every witness that
/// posits an interruption becomes unreachable, and the run fails on zero work
/// rather than reporting green.
module relax_EN_08 {
  import taskTree(
    CRASHES = false,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, ENTRIES_REMOVABLE = false,
    HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_unreach_EN_08_an_interruption_before_the_witness_lands: bool =
    wit_TT_20_an_interruption_before_the_witness_lands

}

/// `EN-08` again, in the focused search where the witness it removes actually
/// lands.  Run in the unfocused instance this control would pass VACUOUSLY: the
/// witness is unreachable there whether `crash` exists or not, which is the
/// exercise-removal control proving nothing.
module relax_EN_08_bulk {
  import taskTree(
    CRASHES = false,
    FOCUS = 1,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, ENTRIES_REMOVABLE = false,
    HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_unreach_EN_08_a_bulk_mark_interrupted_and_repaired: bool =
    hist.bulkRepaired
}

/// `EN-10`, premise-break.  An entry removed.  Expected: `TT-05` fails —
/// allocation re-issues a key a removed entry held.
module relax_EN_10 {
  import taskTree(
    ENTRIES_REMOVABLE = true,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_EN_10_TT_05_allocation_reissues_a_removed_entrys_key: bool =
    inv_TT_05_no_key_is_ever_issued_twice

  val inv_fail_EN_10_TT_12_a_terminal_entry_is_removed: bool =
    inv_TT_12_terminal_entries_are_never_removed
}

/// `EN-11`, exercise-removal.  `hand-edit` removed.  Expected: every witness
/// that posits a tree Grove's own actions cannot build becomes unreachable —
/// and the catalogue's own note is checked here too: a resolved TERMINAL entry
/// is something Grove's actions DO build, so `TT-16`'s witnesses still land.
module relax_EN_11 {
  import taskTree(
    HAND_EDITS = false,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_unreach_EN_11_a_leaf_name_at_a_directory: bool =
    wit_TT_02a_leaf_name_at_a_directory

  val wit_unreach_EN_11_a_node_name_at_a_file: bool =
    wit_TT_02b_node_name_at_a_file

  val wit_unreach_EN_11_a_malformed_node_whose_subtree_holds_live_work: bool =
    wit_TT_03_a_malformed_node_whose_subtree_holds_live_work

  val wit_unreach_EN_11_two_live_finish_leaves_in_different_subtrees: bool =
    wit_TT_13c_two_live_finish_leaves_in_different_subtrees

  /// NOT an `expect_unreachable` — a FINDING, and the second of its kind.
  ///
  /// The catalogue lists `TT-24.b` in `EN-11`'s controls column, so removing
  /// `hand-edit` should make its witness unreachable.  It does not: `EN-13`
  /// grants that foreign entries may appear AT ANY NAME, so `foreign-write`
  /// alone reaches a foreign artifact at a reserved name and the ordinary
  /// operation refuses exactly as `TT-24.b` says.  The dependency is on
  /// `EN-13`, not on `EN-11`.  This is structurally the same mistake the
  /// catalogue already caught and annotated for `TT-16`, found here on a
  /// sibling entry.
  val wit_finding_EN_11_does_not_gate_TT_24b: bool =
    wit_TT_24b_an_ordinary_operation_meets_a_foreign_reserved_name
}

/// `EN-11`'s positive half, in the focused search that reaches it.  The
/// catalogue's own note says a resolved TERMINAL entry is something Grove's own
/// actions build — allocate, retire, resolve — so its witness never needed the
/// assumption.  Checking that in Quint needs the action menu narrowed to those
/// three, because random simulation over the full menu reaches the sequence in
/// under one trace in five thousand.
module relax_EN_11_grove_built {
  import taskTree(
    HAND_EDITS = false,
    FOCUS = 5,
    // The foreign writer is off here for a reason: with it on, a foreign
    // artifact at a reserved name makes every ordinary operation refuse, and
    // the sequence this instance exists to reach never completes.  What the
    // instance asks is whether GROVE'S OWN ACTIONS build a resolved terminal
    // entry, so the world's are out of the way.
    FOREIGN_WRITES = false,
    // Interruption is off for the same reason: an interrupted scaffold refuses
    // every later operation, so a trace that crashes during `init` never gets
    // to build anything.  `EN-08` has its own controls elsewhere.
    CRASHES = false,
    RENAME_ATOMIC = true,
    ENTRIES_REMOVABLE = false, REAPER_SWEEPS_RESERVED = false, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_EN_11_a_resolved_terminal_entry_needs_no_hand_edit: bool =
    wit_TT_16a_a_resolved_done_entry
}

/// `EN-13`, premise-break.  The reaper sweeps the reserved namespace.
/// Expected: `TT-04` fails — the sweep deletes bytes a refusal exists to
/// preserve.
///
/// `inv_fail_EN_13_TT_24d_the_reaper_stops_declining` WAS THE ROW'S SECOND
/// CONTROL AND WENT WITH ITS OBLIGATION.  `TT-24.d` is retired: the quarantine
/// reaper is `grove-finish`'s action, so the obligation is `FN-21.c`
/// (`docs/adr/obligations-follow-context-not-artifact.md`).  The premise-break
/// is not lost — `crates/grove-finish/models/finish-controls.qnt`'s own
/// `relax_EN_13` runs the same dial against the same sweep and carries
/// `inv_fail_EN_13_FN_21b_the_reaper_stops_asking_whose_it_is` and
/// `inv_fail_EN_13_FN_27a_the_sweep_deletes_foreign_bytes`.  The sweep stays in
/// this file because `TT-04` needs it.
module relax_EN_13 {
  import taskTree(
    REAPER_SWEEPS_RESERVED = true,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_EN_13_TT_04_the_sweep_deletes_foreign_bytes: bool =
    inv_TT_04_foreign_subtrees_are_invisible_and_untouched
}

/// A control on the MODEL rather than on the world: later steps classify from
/// the LIVE tree instead of the operation's one listing.  `TT-21` is otherwise
/// true by construction here, and a claim true by construction is the
/// vacuous-invariant hazard with a green tick on it.
module mutant_two_listings {
  import taskTree(
    ONE_SNAPSHOT = false,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_MUT_TT_21a_two_listings_disagree: bool =
    inv_TT_21a_every_classification_comes_from_one_listing
}

/// A control on the MODEL, and the second of its kind.  `TT-23.b`'s
/// convergence rests on a requirement the catalogue implies and does not
/// state — that a bulk member already in the plan's TARGET state is admissible
/// and a no-op.  With `BULK_TARGET_IDEMPOTENT` off, a bulk plan is validated
/// exactly the way a single mark is, so re-running the identical invocation
/// after a partial application is refused `AlreadyTerminal` and the plan can
/// never converge.  Expected: `TT-23.b` FAILS.  Without this instance the
/// requirement is asserted only by the way the model happens to be written,
/// which is the pre-registration's vacuous-invariant hazard wearing a green
/// tick — the same reason `mutant_two_listings` exists.
///
/// Focused (`FOCUS = 1`) for the same reason `scenario_bulk` is: the situation
/// needs a bulk mark interrupted between its two renames and the identical
/// request re-run, which unfocused simulation reaches far too rarely to be a
/// control.
module mutant_bulk_strict {
  import taskTree(
    BULK_TARGET_IDEMPOTENT = false, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    FOCUS = 1,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_MUT_TT_23b_strict_validation_cannot_converge: bool =
    inv_TT_23b_re_running_after_a_partial_application_converges
}

/// A focused search, not a different world.  `wit_TT_23b` needs a bulk mark
/// interrupted between its two renames AND a second bulk mark that completes;
/// the unfocused instance reaches that in far under one trace in four thousand,
/// which is flaky rather than slow.  Focusing the action menu and the hand-edit
/// family onto that one situation makes it common.  Nothing is removed: every
/// invariant is still checked in `base`.
module scenario_bulk {
  import taskTree(
    FOCUS = 1,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_TT_23b_a_bulk_mark_interrupted_and_repaired: bool = hist.bulkRepaired

  val inv_TT_23b_re_running_after_a_partial_application_converges_here: bool =
    inv_TT_23b_re_running_after_a_partial_application_converges
}

/// The same, for `TT-07`'s witness: a shift across a directory holding a live
/// leaf, a node, a `Done` leaf and an `Abandoned` leaf at once.
module scenario_species_shift {
  import taskTree(
    FOCUS = 2,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_TT_07_a_shift_across_a_directory_containing_every_species: bool =
    atRest and isApplied(op.outcome) and op.alg == AInsert and op.applied.length() > 1 and
    val d = parentOf(fs, effTarget(op.applied.head()))
    val sibs = positionedIn(op.beforeFs, d)
    and {
      sibs.exists(i => nameAt(op.beforeFs, i).species == SNode),
      sibs.exists(i => nameAt(op.beforeFs, i).state == SDone),
      sibs.exists(i => nameAt(op.beforeFs, i).state == SAbandoned),
      sibs.exists(i => nameAt(op.beforeFs, i).state == SLive and
                       nameAt(op.beforeFs, i).species == SLeaf),
    }

  val inv_TT_07_a_shift_changes_position_and_nothing_else_here: bool =
    inv_TT_07_a_shift_changes_position_and_nothing_else
}

/// `TT-08` and `TT-09.c`: promotion of a leaf carrying the tree's maximum key.
module scenario_promote {
  import taskTree(
    FOCUS = 3,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_TT_08_promotion_of_the_maximum_key: bool =
    atRest and isApplied(op.outcome) and op.alg == APromote and
    match op.tag {
      | TDecompose(k) => k == keysOf(op.beforeFs).fold(0, (a, x) => if (x > a) x else a)
      | _ => false
    }

  val wit_TT_09c_promotion: bool = atRest and isApplied(op.outcome) and op.alg == APromote

  val inv_TT_08_promotion_preserves_the_key_here: bool =
    inv_TT_08_promotion_preserves_the_key
}

/// `TT-04`: a foreign entry sitting beside siblings an insert renumbers.
module scenario_foreign_sibling {
  import taskTree(
    FOCUS = 4,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val wit_TT_04_a_foreign_entry_survives_a_sibling_renumbering: bool =
    atRest and isApplied(op.outcome) and op.alg == AInsert and
    op.applied.length() > 1 and
    fs.objs.keys().exists(i =>
      isForeign(fs, i) and has(op.beforeFs, i) and
      fs.objs.get(i).digest == op.beforeFs.objs.get(i).digest)

  val inv_TT_04_foreign_subtrees_are_invisible_and_untouched_here: bool =
    inv_TT_04_foreign_subtrees_are_invisible_and_untouched

  /// THE FINDING THIS COMMAND USED TO CARRY IS DISPOSED, AND IN THE OPPOSITE
  /// DIRECTION FROM THE ONE IT ARGUED FOR.  It witnessed an ORDINARY mutation
  /// that had applied an effect, met a destination a non-cooperating writer had
  /// taken, and returned `Blocked(OwnershipConflict)` — reported as a missing
  /// row in the catalogue's *one artifact, three contexts* table, on the ground
  /// that the tree was not byte-identical so no refusal was honest.
  ///
  /// The tree IS byte-identical.  Every mutation runs through the delegated
  /// boundary's one interpreter, which unwinds what it applied on any reported
  /// error (`EN-17`, and `crates/ordinal-fs-tree`'s `Error::Failed`: "the tree
  /// is as it was found").  The model had built an interpreter without that
  /// capability because nothing granted it, and then read the consequence as a
  /// gap in the outcome table.  The command is now the POSITIVE witness of the
  /// repaired claim, in the same instance and reaching the same interleaving.
  val wit_TT_24b_a_mid_flight_collision_unwinds_and_refuses_here: bool =
    wit_TT_24b_a_mid_flight_collision_unwinds_and_refuses
}

/// A MODEL MUTATION, and it is the control for this leaf's own disposition.
/// `SCAFFOLD_AMBIGUITY_CLASSED = false` restores the pre-disposition
/// definition of a witnessless root: the exact subset or `Legacy`, nothing
/// between.  Expected: `TT-20`'s second conjunct fails — an interrupted
/// initialisation whose first `requirements` leaf had already landed, met by
/// one `foreign-write`, classifies `Legacy` with a root-init-exclusive entry
/// on disk, which is what the repaired claim forbids.
///
/// It exists because the conjunct is TRUE BY CONSTRUCTION of `formatStage` and
/// a property no mutation kills is a transcription of the code that satisfies
/// it, which is the failure `obligations-follow-context-not-artifact` records
/// at `TT-24.c`.  `FOCUS = 5` is the init-focused menu, and it is load-bearing:
/// this control was first written at `FOCUS = 4` and reported **no violation**
/// over 4000 traces, because that dial's action menu is inserts only and
/// `root-init` therefore never runs.  A mutation that cannot reach the
/// situation it mutates reads exactly like a mutation that failed to break
/// anything, which is the same silence this control exists to remove.
module mutant_scaffold_absence_only {
  import taskTree(
    SCAFFOLD_AMBIGUITY_CLASSED = false, MUTATION_UNWINDS = true,
    FOCUS = 5,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_MUT_TT_20_absence_only_reads_grove_s_own_work_as_legacy: bool =
    inv_TT_20_an_interrupted_scaffold_is_a_partial_scaffold
}

/// A focused search, not a different world — the fourth of them, and the one
/// this leaf's disposition needed.  `TT-20`'s two new witnesses both require an
/// interrupted `root-init` with a `foreign-write` inside the same open
/// initialisation, which the unfocused menu reaches in about one trace in two
/// thousand.  `FOCUS = 5` narrows the action menu to `root-init`, `retire` and
/// `resolve`; nothing is removed and every invariant is still checked in
/// `base`.
///
/// The pair is deliberate and it is the whole point of the split state: the
/// SAME interleaving lands on two different classifications depending on
/// whether a root-init-exclusive entry had already landed, and both are
/// witnessed rather than one being fenced off as a narrowing.
module scenario_scaffold {
  import taskTree(
    FOCUS = 5,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true, MUTATION_UNWINDS = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_TT_20_an_interrupted_scaffold_is_a_partial_scaffold_here: bool =
    inv_TT_20_an_interrupted_scaffold_is_a_partial_scaffold

  /// `TT-20`'s second witness, declared HERE and not in the library, because in
  /// the unfocused instance it lands in about one trace in two thousand.  The
  /// disposed finding's positive replacement: a stray foreign entry beside a
  /// half-written scaffold now classifies `PartialScaffold(Ambiguous)` — which
  /// refuses — rather than `Legacy`, which migrates.
  val wit_TT_20_a_concurrent_foreign_write_is_ambiguous_not_legacy: bool =
    hist.initCrashUnderForeign.contains(PartialScaffold(SAmbiguous))
}

/// A MODEL MUTATION, and the second control this leaf's disposition needed.
/// `MUTATION_UNWINDS = false` restores the interpreter this model built for
/// itself while `EN-17` was ungranted: an ordinary operation whose next step is
/// no longer licensed stops where it is, leaving its own shifts standing.
/// Expected: `TT-24.b`'s second clause fails — the operation still returns a
/// refusal (that is the catalogue's answer and the mutation does not change it)
/// while the ids it touched are NOT back to the state it found them in, which
/// is a refusal that leaves something to recover.
///
/// It exists because without it the clause is satisfied by the way this file
/// happens to be written, and the whole content of the disposition is that the
/// unwind is load-bearing.  `FOCUS = 4` is the insert-and-foreign-write dial,
/// which is where a mid-flight collision is common.
module mutant_no_unwind {
  import taskTree(
    MUTATION_UNWINDS = false,
    FOCUS = 4,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true, SCAFFOLD_AMBIGUITY_CLASSED = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_fail_MUT_TT_24b_a_refusal_that_leaves_its_own_shifts_standing: bool =
    inv_TT_24b_an_ordinary_operation_refuses_reserved_name_occupied
}
