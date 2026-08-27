// =============================================================================
// models/system/ — the system-lifecycle scope, Quint column
//
// The JOINT.  `SY-` claims are stated over sessions, exhaustion, finish,
// interruption and recovery TOGETHER, and this file is what
// `docs/specs/semantic-contract.md`, *Claims — system lifecycle (`SY`)*, owns.
//
// WRITTEN UNDER THE INDEPENDENCE BARRIER.  This column's session opened no
// `.als` file, no model-directory `README.md` and no experiment entry from the
// Alloy column's range.  It is derived from the catalogue alone.
//
// WHAT THIS MODEL IS, AND WHAT IT IS ABOVE.  `crates/grove-task-tree/models/`
// models a TREE and `crates/grove-finish/models/` models a TRANSACTION.  This
// model composes their OBSERVATIONS and models neither: the task tree is a
// summary (present, witness, format, reserved class, live counts, a byte
// token), and the finish transaction is a phase cursor plus the two facts the
// rest of the lifecycle reads off it — whether the deletion is proven and
// whether the ticket is persistent.  Growing a third tree model, or a second
// twenty-step transaction, would make this scope a copy of the other two and
// would measure their internals rather than the joint.
//
// THE MODULE RULE IS DEFINED IN ONE PLACE — `models/run.sh`, *THE MODULE RULE*
// — and cited rather than restated here.
//
// REVIEWED AND REPAIRED BY `system-k59` / `system-k60`.  The review's subject
// was this INSTRUMENT rather than its findings, and six defects were real: the
// search chain removed real finish-session endings and reversed the epoch
// handoff; the launch generation was modelled as a lock the session holds
// continuously, which Grove has no such thing as; a control documented as a
// bundle over three claims killed one; the `SY-14.a` sweep read "succeeded" as
// "cleared the block"; the flat-menu comparison could not be re-run; and
// `SY-05.b` checked two values one setter manufactures together.  All six are
// applied.  Each site carries a `system-k59 finding N` note, and the README's
// Quint section carries the whole ledger.
// =============================================================================

module lifecycle {

  // ---------------------------------------------------------------------------
  // Assumption dials (`EN-`) and MODEL dials (`MUT`).
  //
  // An `EN_` dial relaxes an environment assumption the catalogue GRANTS.  A
  // MODEL dial breaks something the model would otherwise satisfy BY BEING
  // WRITTEN THAT WAY — the executable-model form of the pre-registration's
  // *vacuous invariant* hazard.  Every MODEL dial below names the obligation it
  // must kill, and `lifecycle-controls.qnt` fires each one.
  // ---------------------------------------------------------------------------

  /// `EN-08` — interruption may occur between any two steps.  The mutation is
  /// `crash`'s REMOVAL (exercise-removal), and its control is unreachability.
  const CRASHES: bool

  /// `EN-11` — any well-formed tree is reachable by hand edit.  Removal makes
  /// every hand-edit-reached refusal state unreachable.
  const HAND_EDITS: bool

  /// MODEL, `SY-01.a` — a second driver QUEUES instead of being refused.
  const LEASE_QUEUES: bool

  /// MODEL, `SY-01.b` — process death does NOT release ownership.
  const DEATH_RELEASES: bool

  /// MODEL, `SY-02` — the layout is proved AFTER configuration validation and
  /// after the first observation of the tree.
  const LAYOUT_FIRST: bool

  /// MODEL, `SY-03` — a later gate consults the layout check the lease made.
  const PREFLIGHT_LICENCE: bool

  /// MODEL, `SY-04.a` — more than one lifecycle transition per iteration.
  const ONE_TRANSITION: bool

  /// MODEL, `SY-04.b` — a transition runs without full configuration validation.
  const CONFIG_FIRST: bool

  /// MODEL, `SY-05.b` — the task root is removed before the deletion is proven.
  const DELETION_PROVEN_FIRST: bool

  /// MODEL, `SY-05.a` — a missing task root is read as EVIDENCE about an
  /// earlier grove instead of meaning *start a new one*.
  const ABSENCE_IS_FRESH: bool

  /// MODEL, `SY-06.a` — a scaffold writes a charter and no first leaf.
  const SCAFFOLD_LEAF: bool

  /// MODEL, `SY-06.b` — a partial scaffold is recognised by the MERE ABSENCE of
  /// the format witness rather than by the catalogue's exact subset.
  const PARTIAL_BY_SUBSET: bool

  /// MODEL, `SY-07.b` — a session may create the finish leaf.
  const DRIVER_OWNS_FINISH: bool

  /// MODEL, `SY-07.a` — the driver appends a SECOND finish leaf instead of
  /// reusing the one already there.  Isolating for `SY-07.a`: session creation
  /// stays refused, so `SY-07.b` is untouched.
  const ONE_FINISH_LEAF: bool

  /// MODEL, `SY-08` — selection is recomputed at launch.
  const SELECT_ONCE: bool

  /// MODEL, `SY-09.c` — no signal is inferred as done.  ISOLATING: it changes
  /// only the `no signal` arm of the interpretation, which is why the two
  /// dials below exist rather than a claim that this one covers all three.
  const NO_SIGNAL_IS_DONE: bool

  /// MODEL, `SY-09.a` — a RELAUNCH stops the loop.  Only the relaunch arm
  /// moves; `done` and `no signal` are untouched.
  const RELAUNCH_ENDS_LOOP: bool

  /// MODEL, `SY-09.b` — a DONE continues the loop.  Only the done arm moves;
  /// `relaunch` and `no signal` are untouched.
  const DONE_CONTINUES_LOOP: bool

  /// MODEL, `SY-10.a` — an ambient operation touches the tree before matching
  /// the live launch generation.
  const GEN_CHECK: bool

  /// MODEL, `SY-10.b` — a contended generation parks silently instead of
  /// timing out into a visible stop.
  const GEN_TIMEOUT: bool

  /// MODEL, `SY-10.b` — the visible stop BORROWS `SY-10.a`'s reason instead of
  /// naming the contention.  This is the isolating control for the restated
  /// half of `SY-10.b`: with it off the admission still stops visibly and
  /// still leaves nobody parked, and the only thing wrong is the NAME it
  /// returns — which is the thing the old history flag could not ask about,
  /// and which entry 041 found the closed refusal set has no member for.
  const GEN_STOP_NAMED: bool

  /// MODEL, `SY-11` — the guard order is not enforced: a caller may take the
  /// tree guard and then wait for the launch generation.
  const GUARD_ORDER: bool

  /// MODEL, `SY-12` — a restart repeats an effect the crashed attempt had
  /// already completed.
  const RESUME_IS_IDEMPOTENT: bool

  /// MODEL, `SY-14.a` — an admitted action clears a block.
  const BLOCK_STICKY: bool

  /// MODEL, `SY-14.b` — a blocked tree refuses, but with a reason that does
  /// NOT name the block.  Isolating for `SY-14.b`: the block stays sticky, so
  /// nothing clears it and `SY-14.a` is untouched.
  const BLOCK_NAMED: bool

  /// The two cooperating processes the catalogue's omission fixes.
  const PROCS: Set[int]

  /// THE FLAT MENU, COMMITTED so that the comparison in the README's *The
  /// driver is deterministic, and that is the search dial* can be RE-RUN rather
  /// than quoted.  `system-k59` finding 5: the producer's 5-of-25 figure came
  /// from an instrument that was never committed, and a reconstruction
  /// disagreed with it by a factor of three.  With `FLAT_MENU` on, `step` is a
  /// uniform choice over the whole action vocabulary — the null hypothesis
  /// `driverStep` is measured against — and `scenario_flat_menu` is the
  /// instance the README's replay line names.
  const FLAT_MENU: bool

  /// The search dial.  0 is the unfocused menu; a `scenario_` instance narrows
  /// it onto one situation.  It removes no behaviour from the world — every
  /// claim is still checked unfocused in `base`.
  ///
  /// 2 is the guard-contention menu.  3 is *the session takes its own work
  /// while it has any* — a narrowing on the SEARCH, added because
  /// `system-k59` finding 1's repair makes a live session's ending nondeter-
  /// ministic at every step, and a twenty-step march through a coin flip per
  /// step lands nowhere.  It is in a `scenario_` and nowhere else, which is the
  /// discipline the review found `driverStep` had broken by narrowing in
  /// `base`.
  const FOCUS: int

  /// `SY-13`'s escape bound: the length of the bounded sequence the claim
  /// quantifies over.
  const ESCAPE_BOUND: int

  /// THE SEARCH DIAL, and it is a dial on the SEARCH rather than on the model.
  /// The driver's loop is DETERMINISTIC — that is what a loop is — so the only
  /// nondeterminism this subject has is the world's: when it acts, and how
  /// often.  With every environment action enabled at every step, a lifecycle
  /// that takes fifteen driver moves to reach a proven finish is sampled at
  /// `(1/k)^15` and every deep witness is reported green on a trace that never
  /// landed.  `ENV_BUDGET` caps how many environment actions ONE TRACE may
  /// take; `base` still checks every property with a budget large enough to
  /// perturb any point of the march.
  const ENV_BUDGET: int

  /// WHICH environment actions a trace may take.  The second half of the same
  /// dial: 0 crash, 1 hand-edit, 2 foreign write, 3 topology change,
  /// 4 configuration change, 5 a leaf inserted in the launch window,
  /// 6 an ambient session operation, 7 a session creating a finish leaf,
  /// 8 the guard-contention actions.  `base` grants all nine.
  const ENV_KINDS: Set[int]

  /// AT WHICH lifecycle point the world may interrupt.  The third half of the
  /// same dial, and the one `SY-14` needs: a block only exists after a
  /// transaction was interrupted between two of its steps and the recovery
  /// could not prove the attempt either way, so a scenario that wants one has
  /// to be able to say WHERE the crash lands.  `base` grants every point.
  const ENV_PHASES: Set[int]

  // ---------------------------------------------------------------------------
  // The task tree, as OBSERVATIONS
  // ---------------------------------------------------------------------------

  type RClass = RNone | RPreparing | RPublished | RMigrating

  type Tree = {
    present: bool,
    witnessPresent: bool,
    fmt: int,
    exactSubset: bool,
    malformed: bool,
    reserved: RClass,
    charter: bool,
    liveOrdinary: int,
    liveFinish: int,
    // An artifact sits at a name Grove reserves and Grove cannot classify it
    // as its own (`TT-24`, `ReservedNameOccupied`).
    foreignReserved: bool,
    // A byte token.  Every write bumps it; `SY-04.b`'s "byte-identical" is
    // this and nothing else, which is all this scope needs of file contents.
    bytes: int,
  }

  pure val NO_TREE: Tree = {
    present: false, witnessPresent: false, fmt: 0, exactSubset: false,
    malformed: false, reserved: RNone, charter: false,
    liveOrdinary: 0, liveFinish: 0, foreignReserved: false, bytes: 0,
  }

  type RootState =
    | RSAbsent
    | RSReserved(RClass)
    | RSPartialScaffold
    | RSLegacy
    | RSForeign(int)
    | RSMalformed
    | RSCurrentLive
    | RSCurrentFinishOnly
    | RSCurrentSpent

  /// The catalogue's PartialScaffold subset test.  With `PARTIAL_BY_SUBSET`
  /// off, the state is recognised by the mere absence of the format witness,
  /// which is exactly what `SY-06.b` forbids.
  pure def partialScaffold(t: Tree): bool =
    if (PARTIAL_BY_SUBSET) t.exactSubset else true

  /// `TT-18`'s fixed classification order, which is itself a claim.  Reserved
  /// is stated over the CLASS, so removing a member changes nothing.
  pure def classifyTree(t: Tree): RootState =
    if (not(t.present)) RSAbsent
    else if (t.reserved != RNone) RSReserved(t.reserved)
    else if (not(t.witnessPresent) and partialScaffold(t)) RSPartialScaffold
    else if (not(t.witnessPresent)) RSLegacy
    else if (t.fmt != 0) RSForeign(t.fmt)
    else if (t.malformed) RSMalformed
    else if (t.liveOrdinary > 0) RSCurrentLive
    else if (t.liveFinish > 0) RSCurrentFinishOnly
    else RSCurrentSpent

  pure def isCurrent(s: RootState): bool =
    match s {
      | RSCurrentLive => true
      | RSCurrentFinishOnly => true
      | RSCurrentSpent => true
      | _ => false
    }

  // ---------------------------------------------------------------------------
  // Finish and recovery, as OBSERVATIONS
  //
  // Five phases and two facts.  The phases are the boundaries the LIFECYCLE
  // can be interrupted at; what happens inside each is `crates/grove-finish/`'s
  // subject and is not re-derived here.
  // ---------------------------------------------------------------------------

  type FinPhase = FPIdle | FPPreparing | FPPublished | FPDeleting | FPProven

  type Block = BNone | BPending | BConflict

  type Fin = {
    phase: FinPhase,
    // The finish-attempt identity, drawn once per launch.
    attempt: int,
    // The finish handle: stable across the whole attempt and across restarts.
    handle: int,
    // `FN-11`/`FN-19`: the deletion commit is proven to exist and to be this
    // attempt's.  `SY-05` is the composition of that with tree absence.
    deletionProven: bool,
    // The correlation ticket, in version-control history rather than in an
    // artifact the transaction can destroy.
    ticket: bool,
    block: Block,
    // The attempt lost its session.  RECOVERY is the action for one; the
    // transaction's own continuation is not.
    orphaned: bool,
  }

  pure val NO_FIN: Fin = {
    phase: FPIdle, attempt: 0, handle: 0,
    deletionProven: false, ticket: false, block: BNone, orphaned: false,
  }

  pure def finRunning(f: Fin): bool = f.phase != FPIdle and f.phase != FPProven

  // ---------------------------------------------------------------------------
  // Outcomes.  The catalogue's closed set.
  // ---------------------------------------------------------------------------

  type Refusal =
    | RLeaseHeld
    | REpochStale
    | RLayoutUnsupported
    | RRootAbsent
    | RFormatLegacy
    | RFormatForeign
    | RMalformedR
    | RReservedKind
    | RWitnessPending
    | RReservedNameOccupied
    | RNotLive
    | RAlreadyTerminal
    // NO LONGER AN ABSTRACTION — the catalogue's `ConfigurationInvalid`, and
    // this declaration is the finding that put it there.  `SY-04.b` requires an
    // invalid configuration to refuse with the tree byte-identical, and the
    // catalogue's closed refusal set had NO member for one; the model named one
    // and declared it rather than borrowing an unrelated reason and making the
    // taxonomy claim vacuous.  The Alloy column reached the same gap
    // independently.  Disposed by `closed-set-additions-k74`; see
    // `docs/adr/a-refusal-leaves-nothing-standing.md`, clause 2.
    | RConfigInvalid
    // NO LONGER AN ABSTRACTION — the catalogue's `GenerationContended`, and
    // THIS COLUMN'S PLACEMENT IS THE ONE THAT WAS KEPT.  `SY-10.b` requires a
    // contended generation to time out into a VISIBLE STOP, and the closed
    // refusal set had NO member for one: `REpochStale` is `SY-10.a`'s reason and
    // says the operation named a generation that is not live, which is a
    // different fact.  `lifecycle.als` read the same gap as evidence that the
    // closed OUTCOME set was short and declared a seventh outcome, `Stopped`;
    // this column read it as a missing REASON and was right — nothing stands
    // after a timeout, so §*Outcomes*' discriminator makes it a refusal, and
    // widening the six outcomes would have swept `SY-14`'s exhaustive
    // classifier for nothing.  Disposed by `closed-set-additions-k74`; see
    // `docs/adr/a-refusal-leaves-nothing-standing.md`.
    | RGenContended

  type Diagnosis = DRecoveryPending | DOwnershipConflict

  type Outcome =
    | ONone
    | OApplied
    | OReported
    | OEmpty
    | ORefused(Refusal)
    | OBlocked(Diagnosis)

  pure def isRefusal(o: Outcome): bool =
    match o { ORefused(_) => true | _ => false }

  pure def refusalOf(o: Outcome): Refusal =
    match o { ORefused(r) => r | _ => RNotLive }

  pure def isBlocked(o: Outcome): bool =
    match o { OBlocked(_) => true | _ => false }

  /// `TT-24`'s table, read at the lifecycle grain: an artifact Grove CAN prove
  /// is its own refuses `WitnessPending`, and one it cannot classify at all
  /// refuses `ReservedNameOccupied`.  This is what `SY-14.b`'s "naming the
  /// block" means in the catalogue's own closed refusal set — the set has no
  /// member spelled after a diagnosis, and inventing one would put a second
  /// vocabulary beside the contract's.
  pure def namesTheBlock(r: Refusal, b: Block): bool =
    match b {
      | BNone => false
      | BPending => r == RWitnessPending
      | BConflict => r == RReservedNameOccupied
    }

  /// The reason a blocked tree actually refuses with.  With `BLOCK_NAMED` off
  /// it refuses with a reason that names nothing — which is `SY-14.b`'s
  /// falsification and leaves `SY-14.a` alone, since a refusal still clears no
  /// block.
  pure def blockRefusal(b: Block): Refusal =
    if (not(BLOCK_NAMED)) RNotLive
    else match b {
      | BNone => RNotLive
      | BPending => RWitnessPending
      | BConflict => RReservedNameOccupied
    }

  // ---------------------------------------------------------------------------
  // The admitted action set.
  //
  // CLOSED and ENUMERATED, because `SY-13`'s "admitted actions" and `SY-14`'s
  // "exhaustive sweep of the action set" are both quantifications over it.  A
  // model whose action set is implicit in its transitions cannot sweep it.
  //
  // The ENVIRONMENT actions — crash, hand-edit, foreign-write, topology-change,
  // confirm — are the WORLD'S and are deliberately NOT admitted: `SY-13`'s own
  // note says operator actions are outside the admitted set by construction,
  // which is what makes a blocked tree a terminal disposition.
  // ---------------------------------------------------------------------------

  type ActionTag =
    | ATAcquireLease
    | ATValidateConfig
    | ATOpenEpoch
    | ATSelect
    | ATScaffold
    | ATCompleteScaffold
    | ATAppendFinish
    | ATLaunch
    | ATAmbient
    | ATReap
    | ATFinishStep
    | ATRecover
    | ATSessionCreateFinish
    | ATCloseEpoch
    | ATReleaseLease

  pure val ADMITTED: Set[ActionTag] = Set(
    ATAcquireLease, ATValidateConfig, ATOpenEpoch, ATSelect, ATScaffold,
    ATCompleteScaffold, ATAppendFinish, ATLaunch, ATAmbient, ATReap,
    ATFinishStep, ATRecover, ATSessionCreateFinish, ATCloseEpoch,
    ATReleaseLease
  )

  /// A LIFECYCLE TRANSITION is a change to the grove's own lifecycle stage —
  /// not every loop step.  `SY-04.a` is stated over these.
  pure def isLifecycleTransition(a: ActionTag): bool =
    a == ATScaffold or a == ATCompleteScaffold or a == ATAppendFinish or
    a == ATFinishStep or a == ATRecover

  /// The gates `SY-02`/`SY-03` are about, and the ones `SY-04.b` requires a
  /// validated configuration in front of.
  /// `reap` collects a session's ENDING; it reads and writes no tree, which is
  /// why a completed teardown does not make it unreachable.  Getting this wrong
  /// is not a detail: with `reap` refused `RootAbsent`, the loop that just
  /// proved its own teardown can never start the fresh grove `SY-05.a` says a
  /// missing task root MEANS, and the claim becomes unwitnessable rather than
  /// false.
  pure def touchesTree(a: ActionTag): bool =
    a != ATAcquireLease and a != ATValidateConfig and a != ATReleaseLease and
    a != ATOpenEpoch and a != ATCloseEpoch and a != ATReap

  // ---------------------------------------------------------------------------
  // Guards, and the wait-for graph `SY-11` is about
  // ---------------------------------------------------------------------------

  type Wait = WNothing | WLease | WGen | WTree

  type World = {
    layoutOk: bool,
    configValid: bool,
    tree: Tree,
    fin: Fin,
    // `EN-14`: the working-tree root exists before the task root and outlives
    // its deletion.  Granted here; the mutation is Alloy's.
    rootIdentity: int,
  }

  type Session = { live: bool, proc: int, gen: int, handle: int }

  pure val NO_SESSION: Session = { live: false, proc: -1, gen: -1, handle: -1 }

  type Driver = {
    lease: int,
    // `SY-02`: set only by a lease acquisition that PROVED the layout.
    layoutProved: bool,
    // `SY-11.a`: 0 nothing, 1 lease, 2 lease+generation, 3 lease+generation+tree.
    orderStage: int,
    // `SY-09`: 0 running, 1 ended done, 2 stopped.
    loopEnd: int,
    // What the lease-time preflight saw.  `SY-03` is the claim that NO later
    // gate reads this.
    leaseLayoutOk: bool,
    gen: int,
    genOpen: bool,
    // THE EPOCH IS A SHARED/EXCLUSIVE LOCK, NOT A TOKEN ONE PROCESS CARRIES.
    // `system-k59` finding 2.  Grove gives each ambient `grove-llm` operation
    // its own SHARED admission, held through that operation and its later tree
    // guard (`src/driver_lease.rs`, `admit_session` and `SessionEpochGuard`),
    // and the driver takes the lock EXCLUSIVELY for pre-spawn activation and
    // for post-reap invalidation (`activate_session_epoch`,
    // `invalidate_session_epoch`).  The launched session holds nothing
    // continuously; modelling it as if it did made `SY-11.b`'s counterexample
    // an artifact of the model rather than a fact about Grove's wait graph.
    genShared: Set[int],
    genExclusive: int,
    // The child has been collected AND the epoch invalidated, and the
    // completion signal has not been interpreted yet.  `SY-09`'s three endings
    // are read in this window and never before it.
    reaped: bool,
    treeGuard: int,
    waits: int -> Wait,
    loopRunning: bool,
    iterOpen: bool,
    configValidated: bool,
    // -2 nothing selected, -1 exhausted, otherwise the selected handle.
    selected: int,
    launchedHandle: int,
    session: Session,
    transitions: int,
    soloTag: ActionTag,
    ending: int,
    lastAction: ActionTag,
    lastOutcome: Outcome,
  }

  pure val NO_DRIVER: Driver = {
    lease: -1, layoutProved: false, orderStage: 0, loopEnd: 0,
    leaseLayoutOk: false, gen: 0, genOpen: false,
    genShared: Set(), genExclusive: -1, reaped: false,
    treeGuard: -1, waits: Map(), loopRunning: true, iterOpen: false,
    configValidated: false, selected: -2, launchedHandle: -2,
    session: NO_SESSION, transitions: 0, soloTag: ATSelect, ending: 0,
    lastAction: ATSelect, lastOutcome: ONone,
  }

  /// WHO A WAITER AT THE EPOCH IS WAITING FOR.  An acquisition conflicts with
  /// the exclusive holder if there is one, and otherwise with any in-flight
  /// SHARED holder — which is the wait an exclusive acquisition actually makes.
  /// Two shared holders do not wait for each other, which is why this is a
  /// function of the waiter and not a single `genHolder` field.
  pure def genHolderFor(d: Driver, p: int): int =
    if (d.genExclusive != -1 and d.genExclusive != p) d.genExclusive
    else d.genShared.exclude(Set(p)).fold(-1, (acc, q) => if (acc == -1) q else acc)

  pure def waitEdge(d: Driver, p: int): int =
    if (not(d.waits.keys().contains(p))) -1
    else match d.waits.get(p) {
      | WNothing => -1
      | WLease => d.lease
      | WGen => genHolderFor(d, p)
      | WTree => d.treeGuard
    }

  /// A cycle in the wait-for graph, within the two-process bound the
  /// catalogue's omission fixes: `p` waits for `q` and `q` waits back.  Stated
  /// as a bounded closure rather than as a traversal, because the bound IS the
  /// claim's content (`SY-11.b`).
  pure def waitCycle(d: Driver): bool =
    PROCS.exists(p =>
      val q = waitEdge(d, p)
      q != -1 and q != p and waitEdge(d, q) == p)

  /// `SY-11.b`'s own text: no path waits for a generation while holding a tree
  /// guard.
  pure def genWaitUnderTreeGuard(d: Driver): bool =
    PROCS.exists(p =>
      d.waits.keys().contains(p) and d.waits.get(p) == WGen and d.treeGuard == p)

  // ---------------------------------------------------------------------------
  // `SY-13` — the stable classes, and the declared escape map
  //
  // `SY-13` is EXISTENTIAL REACHABILITY, deliberately not liveness, so what it
  // needs is a bounded sequence that EXISTS — and an executable model cannot
  // search for one.  So the map below is DECLARED, the property iterates it
  // exhaustively over the closed class set, and the WITNESS is the
  // corroboration: the run must actually walk the longest declared chain, edge
  // by edge, with the model's own transitions.  A declared map nothing
  // corroborates is fiction with a green tick.
  // ---------------------------------------------------------------------------

  type StableClass =
    | SCAbsent
    | SCPartial
    | SCLegacy
    | SCForeign
    | SCMalformed
    | SCReserved
    | SCCurrentLive
    | SCCurrentFinishOnly
    | SCCurrentSpent
    | SCBlockedPending
    | SCBlockedConflict
    | SCFinished

  pure def stableClassOf(w: World): StableClass =
    if (w.fin.block == BPending) SCBlockedPending
    else if (w.fin.block == BConflict) SCBlockedConflict
    else match classifyTree(w.tree) {
      | RSAbsent => if (w.fin.deletionProven) SCFinished else SCAbsent
      | RSReserved(_) => SCReserved
      | RSPartialScaffold => SCPartial
      | RSLegacy => SCLegacy
      | RSForeign(_) => SCForeign
      | RSMalformed => SCMalformed
      | RSCurrentLive => SCCurrentLive
      | RSCurrentFinishOnly => SCCurrentFinishOnly
      | RSCurrentSpent => SCCurrentSpent
    }

  /// `SY-13`'s two targets: a live leaf to run, and a terminal disposition.
  /// The catalogue names exactly two terminal dispositions and says a
  /// `Malformed` tree is NOT one of them.
  pure def isGoodClass(c: StableClass): bool =
    c == SCCurrentLive or c == SCFinished or
    c == SCBlockedPending or c == SCBlockedConflict

  /// The three hand-edit-reached refusal states.  Grove's own actions never
  /// produce one and no admitted action leaves one — see *A catalogue
  /// contradiction* in the README, and `mutant_literal_sy13`, which fires it.
  pure def isHandEditRefusal(c: StableClass): bool =
    c == SCLegacy or c == SCForeign or c == SCMalformed

  /// The classes the sweep quantifies over.  With `SWEEP_ALL_STABLE` the sweep
  /// is the catalogue's LITERAL "any stable state", and the property dies.
  const SWEEP_ALL_STABLE: bool

  pure val ALL_CLASSES: Set[StableClass] = Set(
    SCAbsent, SCPartial, SCLegacy, SCForeign, SCMalformed, SCReserved,
    SCCurrentLive, SCCurrentFinishOnly, SCCurrentSpent,
    SCBlockedPending, SCBlockedConflict, SCFinished
  )

  pure val SWEEP_CLASSES: Set[StableClass] =
    if (SWEEP_ALL_STABLE) ALL_CLASSES
    else ALL_CLASSES.filter(c => not(isHandEditRefusal(c)))

  /// The declared one-edge escape from each non-good class.
  pure def escapeAction(c: StableClass): ActionTag =
    match c {
      | SCAbsent => ATScaffold
      | SCPartial => ATCompleteScaffold
      | SCReserved => ATFinishStep
      | SCCurrentSpent => ATAppendFinish
      | SCCurrentFinishOnly => ATFinishStep
      | _ => ATReleaseLease
    }

  pure def escapeTo(c: StableClass): StableClass =
    match c {
      | SCAbsent => SCPartial
      | SCPartial => SCCurrentLive
      | SCReserved => SCFinished
      | SCCurrentSpent => SCCurrentFinishOnly
      | SCCurrentFinishOnly => SCReserved
      | _ => c
    }

  /// Iterate the declared map.  Unrolled to `ESCAPE_BOUND`, because Quint has
  /// no recursion and the bound is the claim's own content.
  pure def escapesWithin(c: StableClass, n: int): bool =
    if (isGoodClass(c)) true
    else if (n <= 0) false
    else if (escapeTo(c) == c) false
    else if (isGoodClass(escapeTo(c))) true
    else if (n <= 1) false
    else if (escapeTo(escapeTo(c)) == escapeTo(c)) false
    else if (isGoodClass(escapeTo(escapeTo(c)))) true
    else if (n <= 2) false
    else isGoodClass(escapeTo(escapeTo(escapeTo(c))))

  /// The longest declared chain, which is what `SY-13.a`'s witness must walk.
  pure val LONGEST_ESCAPE: int = 3

  // ---------------------------------------------------------------------------
  // History — what a claim about a TRACE needs recorded at the moment it happens
  //
  // `finish-k12` found two invariants stated over LIVE state that the world
  // could falsify AFTER the moment they are about.  Every flag below is written
  // at the moment its claim is about, for that reason.
  // ---------------------------------------------------------------------------

  type Hist = {
    treeTouchedBeforeLayout: bool,
    configBeforeLayout: bool,
    layoutRefusedOnEmptyTree: bool,
    staleLayoutHonoured: bool,
    layoutChangedBetweenGates: bool,
    soloTransitions: Set[ActionTag],
    twoTransitionsInOneIteration: bool,
    transitionWithoutConfig: bool,
    invalidConfigByteIdentical: bool,
    absenceReadAsEvidence: bool,
    absentBeforeProven: bool,
    freshAfterUnobservedTeardown: bool,
    provenAbsenceReached: bool,
    scaffoldWithoutLeaf: bool,
    freshDistinguishable: bool,
    legacyCompleted: bool,
    partialCompleted: bool,
    legacyRefused: bool,
    finishAppended: bool,
    finishReused: bool,
    twoFinishLeaves: bool,
    sessionCreatedFinish: bool,
    sessionRefusedFinish: bool,
    preempted: bool,
    insertedDuringWindow: bool,
    endings: Set[int],
    // A finish session that found ordinary work left, added it, retired the
    // finish leaf and signalled relaunch — the methodology's reopening exit.
    reopened: bool,
    noSignalInferredDone: bool,
    noSignalWithProvenTeardown: bool,
    staleTouchedTree: bool,
    staleRefused: bool,
    // `SY-10.b`, STATED OVER WHAT THE ADMISSION RETURNS.  The pair-stated
    // predecessor (`silentPark`, set from `parked = contended and
    // not(GEN_TIMEOUT)`) was a RESTATEMENT OF THE DIAL: it discharged the
    // obligation without the model ever writing down what the stop returns,
    // which is entry 048 finding 5's second case and entry 041's finding.
    // `contendedStopUnnamed` is quantified over every contended admission in
    // the trace and is discharged only by a REFUSAL NAMING THE CONTENTION.
    contendedStopUnnamed: bool,
    // THE TRACE HALF WAS TRIED AND IS WITHDRAWN, and what it found is the
    // reason it is named here rather than deleted silently.  Read off the
    // post-state of every step — `dPost.waits` carrying any `WGen` entry — it
    // FAILS in `base`, and not because of the epoch-lock park `SY-10.b` is
    // about: `contendGen` is `SY-11.b`'s apparatus and it constructs a
    // generation wait ON PURPOSE, to have a wait-for edge to check the cycle
    // claim over.  The model's `WGen` therefore carries TWO SENSES — the park
    // `SY-10.b` forbids and the graph edge `SY-11.b` needs — and no predicate
    // over `waits` separates them here, including the narrowing to an
    // exclusive hold (`genExclusive != -1 and genExclusive != q`), which also
    // fails.  Both probes are recorded in the README with their exact
    // commands.  Stating `SY-10.b` over the wait map would report `SY-11.b`'s
    // scaffolding as a protocol defect; the collision is a finding ROUTED BY
    // `routing-and-prose-k73` TO THE MODEL OWNERS — the two obligations' texts do not
    // collide, only their realisations do — rather than a counterexample, and
    // `SY-10.b` is
    // checked over WHAT THE ADMISSION RETURNS instead.
    genTimeoutReported: bool,
    orderViolated: bool,
    fullOrderReached: bool,
    cycleSeen: bool,
    waitChainSeen: bool,
    crashPoints: Set[int],
    repeatedCompletedEffect: bool,
    unstableAfterCrash: bool,
    resumedAfterCrash: bool,
    stableSeen: Set[StableClass],
    escapeEdgesObserved: Set[StableClass],
    walkLen: int,
    longestWalk: int,
    sinkSeen: bool,
    sweepDone: bool,
    sweepCleared: bool,
    sweepAllNamed: bool,
    blockCleared: bool,
    blockedActionNotNaming: bool,
    // `SY-04.b`'s byte-identical clause, STATED OVER THE TRACE and split by
    // WHOSE hands moved — which is the whole of entry 048 finding 5 for this
    // obligation.  The pair-stated predecessor (`refusalMutated`, set from one
    // refusing operation's own before/after pair) is WITHDRAWN: it could not
    // reach a byte change made by any step other than the refusal itself, so
    // the claim's missing scoping was never put under pressure, and no control
    // in this file killed it.  Both flags below are quantified over EVERY step
    // of the trace whose pre-state carries an invalid configuration; the
    // narrowing `SY-04.b` needs is now the SPLIT rather than a silence.
    groveMutatedUnderInvalidConfig: bool,
    // The same, and additionally the driver had NOT recorded a validation.
    // This is the half that is TRUE of the model, and the gap between the two
    // flags is this leaf's own counterexample: `outcomeOn`'s configuration
    // gate reads `d.configValidated`, the driver's RECORDED VERDICT, where the
    // layout gate reads `w.layoutOk` live.  A `configChange` after the
    // validation and before the transition therefore leaves the validation
    // standing as a LICENCE — which is the shape `SY-03` forbids for the
    // layout and which the catalogue states for nothing else.
    groveMutatedUnderUnvalidatedConfig: bool,
    worldMutatedUnderInvalidConfig: bool,
    // The furthest finish phase this trace ever reached.  `SY-12`'s "never
    // silently repeats a completed effect" is stated against it.
    maxPhase: int,
    // Ownership, at the lifecycle grain: a second driver that proceeded while
    // the first still held the lease, and a queued one.
    twoDriversLive: bool,
    // The search dial's counter: how many environment actions this trace has
    // spent.  See `ENV_BUDGET`.
    envUsed: int,
    // A driver died holding the lease and no successor has run yet.
    deathPending: bool,
    leaseSurvivedDeath: bool,
    lateLayoutRefusal: bool,
    launchedAfterInsert: bool,
    relaunchStoppedLoop: bool,
    doneContinuedLoop: bool,
    noSignalContinuedLoop: bool,
    leaseQueued: bool,
    secondDriverRefused: bool,
    successorAfterDeath: bool,
  }

  pure val EMPTY_HIST: Hist = {
    treeTouchedBeforeLayout: false, configBeforeLayout: false,
    layoutRefusedOnEmptyTree: false, staleLayoutHonoured: false,
    layoutChangedBetweenGates: false, soloTransitions: Set(),
    twoTransitionsInOneIteration: false, transitionWithoutConfig: false,
    invalidConfigByteIdentical: false, absenceReadAsEvidence: false,
    absentBeforeProven: false, freshAfterUnobservedTeardown: false,
    provenAbsenceReached: false, scaffoldWithoutLeaf: false,
    freshDistinguishable: false, legacyCompleted: false, partialCompleted: false,
    legacyRefused: false, finishAppended: false, finishReused: false,
    twoFinishLeaves: false, sessionCreatedFinish: false,
    sessionRefusedFinish: false, preempted: false, insertedDuringWindow: false,
    endings: Set(), reopened: false, noSignalInferredDone: false,
    noSignalWithProvenTeardown: false, staleTouchedTree: false,
    staleRefused: false, contendedStopUnnamed: false,
    genTimeoutReported: false,
    orderViolated: false, fullOrderReached: false, cycleSeen: false,
    waitChainSeen: false, crashPoints: Set(), repeatedCompletedEffect: false,
    unstableAfterCrash: false, resumedAfterCrash: false, stableSeen: Set(),
    escapeEdgesObserved: Set(), walkLen: 0, longestWalk: 0, sinkSeen: false,
    sweepDone: false, sweepCleared: false, sweepAllNamed: false,
    blockCleared: false, blockedActionNotNaming: false,
    groveMutatedUnderInvalidConfig: false,
    groveMutatedUnderUnvalidatedConfig: false,
    worldMutatedUnderInvalidConfig: false,
    maxPhase: 0, twoDriversLive: false, envUsed: 0,
    deathPending: false, leaseSurvivedDeath: false, lateLayoutRefusal: false,
    launchedAfterInsert: false, relaunchStoppedLoop: false,
    doneContinuedLoop: false, noSignalContinuedLoop: false, leaseQueued: false, secondDriverRefused: false, successorAfterDeath: false,
  }

  // ---------------------------------------------------------------------------
  // The gate classifier.
  //
  // ONE classifier, used by the real actions AND by `SY-14`'s exhaustive sweep.
  // A model whose sweep computes outcomes a second way is checking that its two
  // spellings agree, which is not what `SY-14` says.
  // ---------------------------------------------------------------------------

  pure def treeGate(w: World, a: ActionTag): Outcome =
    match classifyTree(w.tree) {
      | RSAbsent => if (a == ATScaffold) OApplied else ORefused(RRootAbsent)
      // The transaction's OWN continuation, and recovery, are the two actions a
      // reserved witness admits; every other operation refuses.
      | RSReserved(_) =>
          if (a == ATRecover or a == ATFinishStep) OApplied
          else ORefused(RWitnessPending)
      | RSPartialScaffold =>
          if (a == ATCompleteScaffold) OApplied else ORefused(RWitnessPending)
      | RSLegacy => ORefused(RFormatLegacy)
      | RSForeign(_) => ORefused(RFormatForeign)
      | RSMalformed => ORefused(RMalformedR)
      | RSCurrentLive =>
          if (a == ATScaffold or a == ATCompleteScaffold) ORefused(RAlreadyTerminal)
          else if (a == ATAppendFinish) ORefused(RNotLive)
          else if (a == ATFinishStep) ORefused(RNotLive)
          else if (a == ATRecover) ORefused(RWitnessPending)
          else OApplied
      | RSCurrentFinishOnly =>
          if (a == ATScaffold or a == ATCompleteScaffold) ORefused(RAlreadyTerminal)
          else if (a == ATRecover) ORefused(RWitnessPending)
          else OApplied
      | RSCurrentSpent =>
          if (a == ATScaffold or a == ATCompleteScaffold) ORefused(RAlreadyTerminal)
          else if (a == ATFinishStep) ORefused(RNotLive)
          else if (a == ATRecover) ORefused(RWitnessPending)
          else OApplied
    }

  /// Total.  Every action returns exactly one member of the closed set, and a
  /// failed guard is a NAMED REFUSAL rather than an absent transition.
  pure def outcomeOn(w: World, d: Driver, p: int, g: int, a: ActionTag): Outcome =
    if (a == ATAcquireLease)
      // `SY-02`: the layout is proved AT LEASE ACQUISITION, before
      // configuration validation and before any observation of the tree.
      if (LAYOUT_FIRST and not(w.layoutOk)) ORefused(RLayoutUnsupported)
      else if (d.lease != -1 and d.lease != p) ORefused(RLeaseHeld)
      else OApplied
    else if (a == ATReleaseLease or a == ATCloseEpoch)
      if (d.lease != p) ORefused(RLeaseHeld) else OApplied
    else if (a == ATValidateConfig)
      if (w.configValid) OApplied else ORefused(RConfigInvalid)
    else if (a == ATAmbient)
      if (w.fin.block != BNone and BLOCK_STICKY) ORefused(blockRefusal(w.fin.block))
      else if (GEN_CHECK and (not(d.genOpen) or g != d.gen)) ORefused(REpochStale)
      else if (not(if (PREFLIGHT_LICENCE) d.leaseLayoutOk else w.layoutOk))
        ORefused(RLayoutUnsupported)
      else treeGate(w, a)
    // A session creating the finish leaf is refused `ReservedKind` — but on a
    // BLOCKED tree it is refused the block first, like every other operation
    // that touches the tree (`SY-14.b`).
    else if (a == ATSessionCreateFinish)
      if (w.fin.block != BNone and BLOCK_STICKY) ORefused(blockRefusal(w.fin.block))
      else if (DRIVER_OWNS_FINISH) ORefused(RReservedKind)
      else treeGate(w, a)
    else if (d.lease != p) ORefused(RLeaseHeld)
    // `SY-14.b`: on a blocked tree EVERY action refuses, naming the block.
    else if (w.fin.block != BNone and BLOCK_STICKY)
      ORefused(blockRefusal(w.fin.block))
    // `SY-04.b`: full configuration validation precedes every transition.
    else if (CONFIG_FIRST and isLifecycleTransition(a) and not(d.configValidated))
      ORefused(RConfigInvalid)
    // `SY-03`: each gate revalidates against its OWN operands.  With
    // `PREFLIGHT_LICENCE` the gate consults what the lease saw instead.
    else if (not(if (PREFLIGHT_LICENCE) d.leaseLayoutOk else w.layoutOk))
      ORefused(RLayoutUnsupported)
    // `SY-10.a`: an ambient operation matches the live launch generation
    // BEFORE it may touch the tree.
    else if (a == ATReap) OApplied
    else if (a == ATOpenEpoch)
      if (d.genOpen) ORefused(REpochStale) else OApplied
    else if (a == ATLaunch and not(d.genOpen)) ORefused(REpochStale)
    else treeGate(w, a)

  /// The block a hypothetical application of `a` would LEAVE BEHIND.
  ///
  /// `SY-14.a` says no admitted action CLEARS a block, and clearing is a state
  /// transition rather than a successful invocation: `release-lease` and
  /// `validate-config` succeed on a blocked tree and change no block at all.
  /// A sweep that reads any `Applied` as a clearing is measuring the wrong
  /// thing — `system-k59` finding 4 — so the sweep computes the SUCCESSOR's
  /// block here, from the same effects the real actions apply, through the
  /// same `outcomeOn` classifier that decides whether they apply at all.
  pure def blockAfter(w: World, d: Driver, p: int, a: ActionTag): Block =
    if (outcomeOn(w, d, p, d.gen, a) != OApplied) w.fin.block
    // `scaffoldOp` installs `NO_FIN`, which is the one effect in the model
    // that takes a block back to `BNone`.
    else if (a == ATScaffold) BNone
    else if (a == ATFinishStep and not(BLOCK_STICKY)) BNone
    else if (a == ATRecover)
      if (not(BLOCK_STICKY) and w.fin.block != BNone) BNone
      else if (w.tree.foreignReserved) BConflict
      else if (not(RESUME_IS_IDEMPOTENT)) w.fin.block
      else if (w.fin.ticket) w.fin.block
      else BPending
    else w.fin.block

  pure def phaseNum(f: Fin): int =
    match f.phase {
      | FPIdle => 0 | FPPreparing => 1 | FPPublished => 2
      | FPDeleting => 3 | FPProven => 4
    }

  /// The lifecycle point a crash lands at.  `SY-12`'s witness is one crash
  /// point per lifecycle step, so the points have to be nameable.
  pure def crashPoint(w: World, d: Driver): int =
    if (finRunning(w.fin)) 5 + phaseNum(w.fin)
    else if (d.session.live) 4
    else if (d.selected != -2) 3
    else if (d.genOpen) 2
    else if (d.configValidated) 1
    else 0

  pure def clearWait(m: int -> Wait, p: int): int -> Wait =
    m.keys().exclude(Set(p)).mapBy(q => m.get(q))

  /// `SY-13.b`'s sink test, computed from the model's OWN gate classifier: a
  /// stable state from which no admitted tree-touching action is applicable.
  /// The hypothetical driver holds everything a driver could hold, so a
  /// negative answer is about the WORLD rather than about who happens to hold
  /// the lease.
  pure def sinkAt(w: World, d: Driver): bool =
    val c = stableClassOf(w)
    // The hypothetical holds everything a driver could hold, AND the workspace
    // gates are GRANTED: an unsupported layout and an invalid configuration are
    // refusal states `SY-02` and `SY-04.b` own, and folding them in would make
    // `SY-13` a claim about the workspace rather than about the task root.
    val hw = { ...w, layoutOk: true, configValid: true }
    val hd = { ...d, lease: 0, layoutProved: true, leaseLayoutOk: true,
               genOpen: true, configValidated: true, orderStage: 3, transitions: 0 }
    and {
      SWEEP_CLASSES.contains(c),
      not(isGoodClass(c)),
      not(ADMITTED.filter(a => touchesTree(a))
                 .exists(a => outcomeOn(hw, hd, 0, hd.gen, a) == OApplied)),
    }

  pure def selectHandle(t: Tree): int =
    if (t.liveOrdinary > 0) t.liveOrdinary
    else if (t.liveFinish > 0) 0
    else -1

  // ---------------------------------------------------------------------------
  // State
  // ---------------------------------------------------------------------------

  var w: World
  var d: Driver
  var hist: Hist

  pure val FRESH_WORLD: World = {
    layoutOk: true, configValid: true, tree: NO_TREE, fin: NO_FIN, rootIdentity: 1,
  }

  action init: bool = all {
    w' = FRESH_WORLD,
    d' = NO_DRIVER,
    hist' = EMPTY_HIST,
  }

  // ---------------------------------------------------------------------------
  // The one place a transition is folded into the record.
  //
  // Every claim about a TRACE is written here, at the moment it is about, and
  // never read back off live state afterwards — the `finish-k12` lesson.
  // ---------------------------------------------------------------------------

  /// `byGrove` IS THE DISCRIMINATOR ALLOY HAS AND THIS FOLD DID NOT, and it is
  /// the reason entry 042's counterexample could not be replayed here.  The
  /// world's own actions — `handEditTo`, `foreignWrite`, `topologyChange`, the
  /// leaf inserted during the launch window, and a driver's death — fold
  /// themselves in through this same function under an ADMITTED tag
  /// (`ATAmbient`, `ATReap`) with `ONone`, so no flag written here could tell
  /// an operator's byte from grove's.  Alloy states the same distinction as one
  /// conjunct, `Sys.res' != Environmental`.  Passing it explicitly rather than
  /// inferring it from `o == ONone` keeps the discriminator a declaration
  /// rather than a coincidence of the call sites.
  pure def observe(h: Hist, wPre: World, dPre: Driver, wPost: World,
                   dPost: Driver, a: ActionTag, o: Outcome, byGrove: bool): Hist =
    val c = stableClassOf(wPre)
    val c2 = stableClassOf(wPost)
    val followed = c != c2 and escapeAction(c) == a and escapeTo(c) == c2
    val nwalk = if (followed) h.walkLen + 1 else if (c != c2) 0 else h.walkLen
    val applied = o == OApplied
    val ph = phaseNum(wPost.fin)
    {
      ...h,
      stableSeen: h.stableSeen.union(Set(c2)),
      escapeEdgesObserved:
        if (followed) h.escapeEdgesObserved.union(Set(c)) else h.escapeEdgesObserved,
      walkLen: nwalk,
      longestWalk: if (nwalk > h.longestWalk) nwalk else h.longestWalk,
      sinkSeen: h.sinkSeen or sinkAt(wPost, dPost),
      // `SY-04.b`, OVER THE TRACE.  Every step whose pre-state carries an
      // invalid configuration is examined — grove's applied steps, its
      // refusals, and the world's writes alike — and the answer is split by
      // whose hands moved rather than restricted to the refusing operation's
      // own pair.  `inv_SY_04b` asserts the GROVE half; the WORLD half is
      // asserted by a control that must fire, which is where entry 042's
      // counterexample lands.
      groveMutatedUnderInvalidConfig:
        h.groveMutatedUnderInvalidConfig or
        (byGrove and not(wPre.configValid) and
         wPost.tree.bytes != wPre.tree.bytes),
      groveMutatedUnderUnvalidatedConfig:
        h.groveMutatedUnderUnvalidatedConfig or
        (byGrove and not(wPre.configValid) and not(dPre.configValidated) and
         wPost.tree.bytes != wPre.tree.bytes),
      worldMutatedUnderInvalidConfig:
        h.worldMutatedUnderInvalidConfig or
        (not(byGrove) and not(wPre.configValid) and
         wPost.tree.bytes != wPre.tree.bytes),
      treeTouchedBeforeLayout:
        h.treeTouchedBeforeLayout or (applied and touchesTree(a) and not(dPre.layoutProved)),
      configBeforeLayout:
        h.configBeforeLayout or (a == ATValidateConfig and applied and not(dPre.layoutProved)),
      staleLayoutHonoured:
        h.staleLayoutHonoured or (applied and touchesTree(a) and not(wPre.layoutOk)),
      transitionWithoutConfig:
        h.transitionWithoutConfig or
        (applied and countsAsTransition(wPre, a) and not(dPre.configValidated)),
      invalidConfigByteIdentical:
        h.invalidConfigByteIdentical or
        (isRefusal(o) and refusalOf(o) == RConfigInvalid and
         wPost.tree.bytes == wPre.tree.bytes),
      soloTransitions:
        if (applied and countsAsTransition(wPre, a) and dPost.transitions == 1)
          h.soloTransitions.union(Set(a))
        else h.soloTransitions,
      twoTransitionsInOneIteration:
        h.twoTransitionsInOneIteration or dPost.transitions > 1,
      absentBeforeProven:
        h.absentBeforeProven or
        (wPre.tree.present and not(wPost.tree.present) and not(wPost.fin.deletionProven)),
      provenAbsenceReached:
        h.provenAbsenceReached or (not(wPost.tree.present) and wPost.fin.deletionProven),
      maxPhase: if (ph > h.maxPhase) ph else h.maxPhase,
      repeatedCompletedEffect:
        h.repeatedCompletedEffect or
        (applied and (a == ATFinishStep or a == ATRecover) and
         wPost.fin.phase != FPIdle and ph < h.maxPhase),
      // LITERAL over the admitted set, deliberately: `SY-14.a` is about a
      // block being CLEARED, and once that is a state transition there is
      // nothing left for a `touchesTree` narrowing to protect.
      blockCleared:
        h.blockCleared or
        (wPre.fin.block != BNone and wPost.fin.block == BNone and
         ADMITTED.contains(a)),
      blockedActionNotNaming:
        h.blockedActionNotNaming or
        (wPre.fin.block != BNone and ADMITTED.contains(a) and touchesTree(a) and
         not(isRefusal(o) and namesTheBlock(refusalOf(o), wPre.fin.block))),
      fullOrderReached: h.fullOrderReached or dPost.orderStage >= 3,
      orderViolated:
        h.orderViolated or (applied and touchesTree(a) and dPre.orderStage < 2),
      legacyRefused:
        h.legacyRefused or (isRefusal(o) and refusalOf(o) == RFormatLegacy),
      lateLayoutRefusal:
        h.lateLayoutRefusal or
        (isRefusal(o) and refusalOf(o) == RLayoutUnsupported and dPre.lease != -1),
      cycleSeen: h.cycleSeen or waitCycle(dPost) or genWaitUnderTreeGuard(dPost),
      waitChainSeen:
        h.waitChainSeen or PROCS.exists(p => waitEdge(dPost, p) != -1),
      secondDriverRefused:
        h.secondDriverRefused or
        (a == ATAcquireLease and isRefusal(o) and refusalOf(o) == RLeaseHeld),
      leaseQueued:
        h.leaseQueued or
        dPost.waits.keys().exists(p => dPost.waits.get(p) == WLease),
      layoutRefusedOnEmptyTree:
        h.layoutRefusedOnEmptyTree or
        (a == ATAcquireLease and isRefusal(o) and
         refusalOf(o) == RLayoutUnsupported and not(wPost.tree.present) and
         wPost.tree.bytes == wPre.tree.bytes),
    }

  // ---------------------------------------------------------------------------
  // The lease (`SY-01`, `SY-02`)
  // ---------------------------------------------------------------------------

  action acquireLease(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATAcquireLease)
    val queued = isRefusal(o) and refusalOf(o) == RLeaseHeld and LEASE_QUEUES
    val d2 =
      if (o == OApplied)
        { ...d, lease: p, layoutProved: LAYOUT_FIRST, leaseLayoutOk: w.layoutOk,
          orderStage: 1, configValidated: false, selected: -2, launchedHandle: -2,
          transitions: 0, iterOpen: true, ending: 0,
          lastAction: ATAcquireLease, lastOutcome: o }
      else if (queued)
        { ...d, waits: d.waits.put(p, WLease),
          lastAction: ATAcquireLease, lastOutcome: o }
      else { ...d, lastAction: ATAcquireLease, lastOutcome: o }
    all {
      d.lease != p,
      w' = w,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w, d2, ATAcquireLease, o, true),
        successorAfterDeath:
          hist.successorAfterDeath or (o == OApplied and hist.deathPending),
        deathPending: hist.deathPending and o != OApplied,
      },
    }

  action releaseLease(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATReleaseLease)
    val d2 =
      if (o == OApplied)
        { ...d, lease: -1, layoutProved: false, orderStage: 0, genOpen: false,
          genShared: Set(), genExclusive: -1,
          treeGuard: -1, waits: clearWait(d.waits, p),
          iterOpen: false, lastAction: ATReleaseLease, lastOutcome: o }
      else { ...d, lastAction: ATReleaseLease, lastOutcome: o }
    all {
      d.lease == p,
      w' = w,
      d' = d2,
      hist' = observe(hist, w, d, w, d2, ATReleaseLease, o, true),
    }

  // ---------------------------------------------------------------------------
  // Configuration and the epoch (`SY-04`, `SY-11.a`)
  // ---------------------------------------------------------------------------

  action validateConfig(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATValidateConfig)
    val d2 = { ...d, configValidated: o == OApplied,
               lastAction: ATValidateConfig, lastOutcome: o }
    all {
      d.lease == p,
      w' = w,
      d' = d2,
      hist' = observe(hist, w, d, w, d2, ATValidateConfig, o, true),
    }

  action openEpoch(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATOpenEpoch)
    val violated = GUARD_ORDER and d.orderStage < 1
    val d2 =
      if (o == OApplied)
        // Pre-spawn ACTIVATION: the driver takes the epoch lock exclusively,
        // writes the record and lets go.  Nothing holds it afterwards.
        { ...d, gen: d.gen + 1, genOpen: true, genExclusive: -1,
          orderStage: if (d.orderStage >= 1) 2 else d.orderStage,
          lastAction: ATOpenEpoch, lastOutcome: o }
      else { ...d, lastAction: ATOpenEpoch, lastOutcome: o }
    all {
      d.lease == p,
      not(violated),
      w' = w,
      d' = d2,
      hist' = observe(hist, w, d, w, d2, ATOpenEpoch, o, true),
    }

  action closeEpoch(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATCloseEpoch)
    val d2 =
      if (o == OApplied)
        // Closing the epoch is an exclusive rewrite, so it returns only once
        // every in-flight shared admission has drained.
        { ...d, genOpen: false, genShared: Set(), genExclusive: -1,
          treeGuard: -1,
          orderStage: if (d.orderStage > 1) 1 else d.orderStage,
          iterOpen: true, configValidated: false, selected: -2,
          launchedHandle: -2, transitions: 0,
          lastAction: ATCloseEpoch, lastOutcome: o }
      else { ...d, lastAction: ATCloseEpoch, lastOutcome: o }
    all {
      d.lease == p,
      w' = w,
      d' = d2,
      hist' = observe(hist, w, d, w, d2, ATCloseEpoch, o, true),
    }

  // ---------------------------------------------------------------------------
  // The tree transitions (`SY-05`, `SY-06`, `SY-07`)
  // ---------------------------------------------------------------------------

  /// A finish transaction is ONE lifecycle transition, not one per step: its
  /// steps are the transaction's, and `SY-04.a` is about the loop's iteration
  /// rather than about the protocol's internals — which are
  /// `crates/grove-finish/models/`'s subject and not re-derived here.
  /// The finish TRANSACTION is not a lifecycle transition at all: its steps
  /// belong to the finish leaf's own session, and `crates/grove-finish/models/`
  /// is where they are checked.  What the ITERATION does is ENTER it — append
  /// the finish leaf — and recover an interrupted one.
  pure def countsAsTransition(wPre: World, a: ActionTag): bool =
    a != ATFinishStep and isLifecycleTransition(a)

  pure def afterTree(wPre: World, dPre: Driver, a: ActionTag, o: Outcome): Driver =
    { ...dPre,
      orderStage: if (o == OApplied and dPre.orderStage >= 2) 3 else dPre.orderStage,
      transitions:
        if (o == OApplied and countsAsTransition(wPre, a)) dPre.transitions + 1
        else dPre.transitions,
      lastAction: a, lastOutcome: o }

  /// The guard the LOOP imposes, as against the guards the CONTRACT imposes.
  /// `SY-04.a` is a property of the driver's iteration rather than an
  /// invocation's outcome, so it is an enabling condition here and not a
  /// refusal: the catalogue's closed refusal set has no member for "deferred to
  /// the next iteration", and inventing one would put a second vocabulary
  /// beside the contract's.
  pure def loopAdmits(wPre: World, dPre: Driver, a: ActionTag): bool =
    (not(ONE_TRANSITION) or not(countsAsTransition(wPre, a)) or dPre.transitions == 0)
      and (not(GUARD_ORDER) or not(touchesTree(a)) or dPre.orderStage >= 2)

  action scaffoldOp(p: int): bool =
    val readsEvidence = not(ABSENCE_IS_FRESH) and w.fin.deletionProven
    val o = if (readsEvidence) ORefused(RRootAbsent)
            else outcomeOn(w, d, p, d.gen, ATScaffold)
    val t2 = { ...NO_TREE, present: true, charter: true, witnessPresent: false,
               exactSubset: true,
               liveOrdinary: if (SCAFFOLD_LEAF) 1 else 0,
               bytes: w.tree.bytes + 1 }
    val w2 = if (o == OApplied) { ...w, tree: t2, fin: NO_FIN } else w
    val d2 = afterTree(w, d, ATScaffold, o)
    all {
      d.lease == p,
      loopAdmits(w, d, ATScaffold),
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATScaffold, o, true),
        absenceReadAsEvidence: hist.absenceReadAsEvidence or readsEvidence,
        freshAfterUnobservedTeardown:
          hist.freshAfterUnobservedTeardown or
          (o == OApplied and w.fin.deletionProven and
           (hist.crashPoints.size() > 0 or hist.endings.contains(3))),
      },
    }

  action completeScaffoldOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATCompleteScaffold)
    val notTheSubset = not(w.tree.exactSubset)
    val t2 = { ...w.tree, witnessPresent: true, fmt: 0,
               liveOrdinary: if (SCAFFOLD_LEAF) 1 else w.tree.liveOrdinary,
               bytes: w.tree.bytes + 1 }
    val w2 = if (o == OApplied) { ...w, tree: t2 } else w
    val d2 = afterTree(w, d, ATCompleteScaffold, o)
    all {
      d.lease == p,
      loopAdmits(w, d, ATCompleteScaffold),
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATCompleteScaffold, o, true),
        legacyCompleted:
          hist.legacyCompleted or (o == OApplied and notTheSubset),
        partialCompleted:
          hist.partialCompleted or (o == OApplied and not(notTheSubset)),
        scaffoldWithoutLeaf:
          hist.scaffoldWithoutLeaf or
          (o == OApplied and classifyTree(t2) == RSCurrentSpent),
        freshDistinguishable:
          hist.freshDistinguishable or
          (o == OApplied and classifyTree(t2) == RSCurrentLive),
      },
    }

  action appendFinishOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATAppendFinish)
    // `SY-07.a`'s REUSE, and its dial.  With `ONE_FINISH_LEAF` off the driver
    // appends a second leaf beside the one already there, which is how "exactly
    // one driver-owned finish leaf" fails WITHOUT a session creating anything —
    // the isolating control `SY-07.a` did not have.
    val reuse = ONE_FINISH_LEAF and w.tree.liveFinish > 0
    val t2 = { ...w.tree, liveFinish: if (reuse) w.tree.liveFinish
                                      else w.tree.liveFinish + 1,
               bytes: w.tree.bytes + 1 }
    val w2 = if (o == OApplied) { ...w, tree: t2 } else w
    val d2 = afterTree(w, d, ATAppendFinish, o)
    all {
      d.lease == p,
      loopAdmits(w, d, ATAppendFinish),
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATAppendFinish, o, true),
        finishAppended: hist.finishAppended or (o == OApplied and not(reuse)),
        finishReused: hist.finishReused or (o == OApplied and reuse),
        twoFinishLeaves: hist.twoFinishLeaves or w2.tree.liveFinish > 1,
      },
    }

  /// `SY-07.b`.  A session's attempt to create the finish leaf.  Total: the
  /// refusal is `ReservedKind`, which is the catalogue's own reason for it.
  action sessionCreateFinishOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATSessionCreateFinish)
    val t2 = { ...w.tree, liveFinish: w.tree.liveFinish + 1,
               bytes: w.tree.bytes + 1 }
    val w2 = if (o == OApplied) { ...w, tree: t2 } else w
    val d2 = afterTree(w, d, ATSessionCreateFinish, o)
    all {
      d.lease != -1,
      w.tree.present,
      hist.envUsed < ENV_BUDGET,
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATSessionCreateFinish, o, true),
        envUsed: hist.envUsed + 1,
        sessionCreatedFinish: hist.sessionCreatedFinish or o == OApplied,
        sessionRefusedFinish:
          hist.sessionRefusedFinish or
          (isRefusal(o) and refusalOf(o) == RReservedKind),
        twoFinishLeaves: hist.twoFinishLeaves or w2.tree.liveFinish > 1,
      },
    }

  // ---------------------------------------------------------------------------
  // Selection, launch, the session, and the reap (`SY-08`, `SY-09`, `SY-10`)
  // ---------------------------------------------------------------------------

  action selectOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATSelect)
    val h = selectHandle(w.tree)
    val d2 = { ...afterTree(w, d, ATSelect, o),
               selected: if (o == OApplied) h else d.selected }
    all {
      d.lease == p,
      d.selected == -2,
      loopAdmits(w, d, ATSelect),
      w' = w,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w, d2, ATSelect, o, true),
        // `SY-07.a`'s REUSE.  Appending is `appendFinishOp`; reusing is
        // selecting a finish leaf this iteration did not append.
        finishReused:
          hist.finishReused or (o == OApplied and h == 0 and d.transitions == 0),
      },
    }

  action launchOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATLaunch)
    // `SY-08`: the driver selects ONCE per iteration and does not recompute
    // before launching.  With `SELECT_ONCE` off it recomputes, and a leaf
    // added during the launch window preempts the running one.
    val h = if (SELECT_ONCE) d.selected else selectHandle(w.tree)
    // The launched session is a different process from the driver, and it
    // holds NO epoch lock of its own — `system-k59` finding 2.  What it does is
    // run `grove-llm` operations, and each of those takes its own shared
    // admission for its own duration; see `ambientAdmit`.
    val sp = if (p == 0) 1 else 0
    val d2 = { ...afterTree(w, d, ATLaunch, o),
               launchedHandle: if (o == OApplied) h else d.launchedHandle,
               session: if (o == OApplied)
                          { live: true, proc: sp, gen: d.gen, handle: h }
                        else d.session }
    all {
      d.lease != -1,
      d.selected != -2,
      not(d.session.live),
      loopAdmits(w, d, ATLaunch),
      w' = w,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w, d2, ATLaunch, o, true),
        preempted: hist.preempted or (o == OApplied and h != d.selected),
        launchedAfterInsert:
          hist.launchedAfterInsert or (o == OApplied and hist.insertedDuringWindow),
      },
    }

  /// The world adds a leaf during the LAUNCH WINDOW — between the iteration's
  /// selection and its launch.  `SY-08`'s witness.
  action insertDuringWindow: bool =
    val t2 = { ...w.tree, liveOrdinary: w.tree.liveOrdinary + 1,
               bytes: w.tree.bytes + 1 }
    val w2 = { ...w, tree: t2 }
    all {
      d.selected != -2,
      not(d.session.live),
      w.tree.present,
      w.tree.witnessPresent,
      hist.envUsed < ENV_BUDGET,
      w' = w2,
      d' = d,
      hist' = {
        // `ONone`: this is the WORLD's write, not an admitted action's
        // outcome.  Recording it as `OApplied` would make Grove answerable for
        // a leaf a human added, which is the opposite of what `SY-08` says.
        ...observe(hist, w, d, w2, d, ATAmbient, ONone, false),
        envUsed: hist.envUsed + 1,
        insertedDuringWindow: true,
      },
    }

  /// An ambient operation from a session.  It is NOT the lease holder, and
  /// `SY-10.a` is the claim that it matches the live launch generation before
  /// it may touch the tree.
  /// ADMISSION.  An ambient operation asks for the epoch lock SHARED and holds
  /// it through the operation and its later tree guard.  It conflicts only with
  /// the driver's EXCLUSIVE hold — the post-reap handoff — and `SY-10.b` is the
  /// claim that such a conflict times out visibly rather than parking.
  action ambientAdmit(p: int): bool =
    val contended = d.genExclusive != -1 and d.genExclusive != p
    val timedOut = contended and GEN_TIMEOUT
    val parked = contended and not(GEN_TIMEOUT)
    // ADMISSION TOUCHES NO TREE, and the tag says so.  It is an acquisition of
    // the epoch lock, so it is folded into the record as an epoch action with a
    // REPORTED outcome — not as an applied `ATAmbient`, which would tell
    // `observe` that a tree was touched and make `SY-03` and `SY-14.b` answer
    // for a lock acquisition.  The tree is touched in `ambientOp`, under the
    // admission this step won.
    // WHAT THE STOP RETURNS, written down.  `REpochStale` was borrowed here
    // and it is `SY-10.a`'s reason — *this operation named a generation that
    // is not live* — which is a different fact from *the epoch lock is held
    // exclusively and the wait timed out*.  `RGenContended` is declared beside
    // `RConfigInvalid` for the same reason and with the same disposition
    // owner.
    val o = if (timedOut and GEN_STOP_NAMED) ORefused(RGenContended)
            else if (timedOut) ORefused(REpochStale)
            else if (parked) ONone
            else OReported
    val d2 = { ...d,
               genShared: if (contended) d.genShared else d.genShared.union(Set(p)),
               waits: if (parked) d.waits.put(p, WGen) else d.waits,
               lastAction: ATOpenEpoch, lastOutcome: o }
    all {
      // NO `genOpen` PRECONDITION, deliberately.  An arriving operation blocks
      // on the lock FIRST and reads the record afterwards
      // (`src/driver_lease.rs`, `admit_session`), so whether the epoch is
      // active decides the operation's outcome — `SY-10.a` — and not whether it
      // may queue at all.  Requiring an open epoch here made the driver's own
      // exclusive window, which is exactly when the record is inactive, the one
      // window in which nothing could contend.
      d.lease != -1,
      p != d.lease,
      not(d.genShared.contains(p)),
      hist.envUsed < ENV_BUDGET,
      w' = w,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w, d2, ATOpenEpoch, o, true),
        envUsed: hist.envUsed + 1,
        genTimeoutReported: hist.genTimeoutReported or timedOut,
        // `SY-10.b`, DISCHARGED BY THE OUTCOME RATHER THAN BY THE DIAL.  The
        // predecessor asked `parked`, which is `contended and
        // not(GEN_TIMEOUT)` — the dial spelled backwards — so the obligation
        // was met without the closed outcome set ever being consulted.  This
        // asks what the admission RETURNED, over every contended admission in
        // the trace, and a park (`ONone`) fails it because `ONone` is not a
        // refusal naming anything.
        contendedStopUnnamed:
          hist.contendedStopUnnamed or
          (contended and not(isRefusal(o) and refusalOf(o) == RGenContended)),
      },
    }

  /// THE OPERATION ITSELF, under the admission it already holds, and it RELEASES
  /// that admission when it returns.  It costs no environment budget, because
  /// the budget was spent admitting it and a model that could admit an operation
  /// it can never let finish would stall the loop rather than perturb it.
  action ambientOp(p: int, g: int): bool =
    val o = outcomeOn(w, d, p, g, ATAmbient)
    val stale = not(d.genOpen) or g != d.gen
    val t2 = { ...w.tree, bytes: w.tree.bytes + 1 }
    val w2 = if (o == OApplied) { ...w, tree: t2 } else w
    val d2 = { ...afterTree(w, d, ATAmbient, o),
               genShared: d.genShared.exclude(Set(p)),
               waits: clearWait(d.waits, p) }
    all {
      d.genShared.contains(p),
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATAmbient, o, true),
        staleTouchedTree: hist.staleTouchedTree or (o == OApplied and stale),
        staleRefused:
          hist.staleRefused or
          (stale and isRefusal(o) and refusalOf(o) == REpochStale),
      },
    }

  /// A session that RELAUNCHES has retired its leaf and committed — that is
  /// what `grove-llm complete` signals, and it is where exhaustion comes from.
  /// `done` and `no signal` retire nothing.
  action sessionEnds(k: int): bool =
    // THE METHODOLOGY'S REOPENING EXIT, and `system-k59` finding 1 is why it is
    // here: a finish session that finds ordinary work left adds it, retires the
    // finish leaf and signals RELAUNCH.  The model used to be unable to
    // represent it, because a launched finish session could not end at all.
    val finishSession = d.launchedHandle == 0
    val reopens = k == 1 and finishSession and w.fin.phase == FPIdle and
                  w.tree.liveFinish > 0 and w.tree.witnessPresent
    val retires = k == 1 and not(reopens) and
                  w.tree.liveOrdinary > 0 and w.tree.witnessPresent
    val t2 =
      if (reopens) { ...w.tree, liveOrdinary: w.tree.liveOrdinary + 1,
                     liveFinish: 0, bytes: w.tree.bytes + 1 }
      else if (retires) { ...w.tree, liveOrdinary: w.tree.liveOrdinary - 1,
                          bytes: w.tree.bytes + 1 }
      else w.tree
    val w2 = if (reopens or retires) { ...w, tree: t2 } else w
    val d2 = { ...d, ending: k, session: { ...d.session, live: false },
               lastAction: ATReap, lastOutcome: OReported }
    all {
      d.session.live,
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATReap, OReported, true),
        endings: hist.endings.union(Set(k)),
        reopened: hist.reopened or reopens,
        noSignalWithProvenTeardown:
          hist.noSignalWithProvenTeardown or (k == 3 and w.fin.deletionProven),
      },
    }

  /// WHAT A LIVE SESSION DOES NEXT, and it is not always its own work.
  ///
  /// `system-k59` finding 1: the chain used to force a launched FINISH session
  /// through teardown, so decline, early failure, no signal and the reopening
  /// exit were all unreachable — while the Rust loop accepts any of the three
  /// signal dispositions after ANY launched session
  /// (`src/loop_driver.rs:162`, `src/loop_driver.rs:190`).  So a live session
  /// may end at ANY point, in any of the three ways, and the finish session's
  /// own work is one alternative among them rather than a forced march.
  action sessionMove(p: int): bool =
    val cls = classifyTree(w.tree)
    val reserved = match cls { RSReserved(_) => true | _ => false }
    val finishOnly = match cls { RSCurrentFinishOnly => true | _ => false }
    val finishSession = d.launchedHandle == 0
    // SEARCH ONLY, and only under `FOCUS = 3`: the session has work of its own
    // here, so the search takes it rather than ending.  Every ending remains
    // reachable in `base`, which is where every property is checked.
    val hasWork = finishSession and (reserved or finishOnly)
    any {
      all { FOCUS != 3 or not(hasWork),
            nondet k = oneOf(Set(1, 2, 3)) sessionEnds(k) },
      all { finishSession, reserved, w.fin.orphaned, recoverOp(p) },
      all { finishSession, reserved, not(w.fin.orphaned), finishStepOp(p) },
      all { finishSession, finishOnly, finishStepOp(p) },
    }

  /// COLLECT THE CHILD, THEN INVALIDATE THE EPOCH — AND INTERPRET NOTHING.
  ///
  /// `system-k59` finding 1's second half.  Grove reaps, then acquires the
  /// exclusive epoch handoff and invalidates the epoch, and only THEN reads the
  /// signal file (`src/loop_driver.rs:170`, `src/loop_driver.rs:353`,
  /// `complete_post_reap_epoch_handoff`).  The model used to interpret the
  /// ending here while leaving the epoch open, which is that order backwards.
  /// The invalidation and the interpretation are two STEPS now, so the order is
  /// a fact about the trace rather than a claim about the source.
  action reapOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATReap)
    val d2 = { ...afterTree(w, d, ATReap, o),
               reaped: if (o == OApplied) true else d.reaped,
               genOpen: if (o == OApplied) false else d.genOpen,
               gen: if (o == OApplied) d.gen + 1 else d.gen,
               // The driver holds the exclusive handoff from here until it has
               // read the signal, which is the window an ambient admission
               // contends with (`SY-10.b`).
               genExclusive: if (o == OApplied) p else d.genExclusive,
               treeGuard: if (o == OApplied) -1 else d.treeGuard,
               orderStage:
                 if (o == OApplied and d.orderStage > 1) 1 else d.orderStage }
    all {
      d.lease == p,
      d.ending != 0,
      not(d.session.live),
      not(d.reaped),
      w' = w,
      d' = d2,
      hist' = observe(hist, w, d, w, d2, ATReap, o, true),
    }

  /// READ THE SIGNAL, in the window the invalidation opened.
  ///
  /// THREE ARMS, THREE DIALS, and that is the correction `system-k59` finding 3
  /// forced.  The arms used to be two constants and one dial, so
  /// `mutant_no_signal_is_done` could not touch `SY-09.a` or `SY-09.b` at all
  /// and the "bundle control" the README claimed did not exist.
  action interpretSignalOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATReap)
    val le = if (d.ending == 1) (if (RELAUNCH_ENDS_LOOP) 2 else 0)
             else if (d.ending == 2) (if (DONE_CONTINUES_LOOP) 0 else 1)
             else if (NO_SIGNAL_IS_DONE) 1
             else 2
    val d2 = { ...afterTree(w, d, ATReap, o),
               loopEnd: if (o == OApplied) le else d.loopEnd,
               reaped: if (o == OApplied) false else d.reaped,
               genExclusive: if (o == OApplied) -1 else d.genExclusive,
               ending: if (o == OApplied) 0 else d.ending,
               selected: if (o == OApplied) -2 else d.selected,
               launchedHandle: if (o == OApplied) -2 else d.launchedHandle,
               transitions: if (o == OApplied) 0 else d.transitions }
    all {
      d.lease == p,
      d.reaped,
      not(d.genOpen),
      w' = w,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w, d2, ATReap, o, true),
        noSignalInferredDone:
          hist.noSignalInferredDone or (o == OApplied and d.ending == 3 and le == 1),
        relaunchStoppedLoop:
          hist.relaunchStoppedLoop or (o == OApplied and d.ending == 1 and le != 0),
        doneContinuedLoop:
          hist.doneContinuedLoop or (o == OApplied and d.ending == 2 and le != 1),
        noSignalContinuedLoop:
          hist.noSignalContinuedLoop or (o == OApplied and d.ending == 3 and le == 0),
      },
    }

  // ---------------------------------------------------------------------------
  // Finish, interruption and recovery (`SY-05`, `SY-12`, `SY-14`)
  // ---------------------------------------------------------------------------

  action finishStepOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATFinishStep)
    val f = w.fin
    // MODEL, `SY-14.a`: the block is NOT sticky, so the transaction's own next
    // step walks straight through it and clears it.  That is an ADMITTED action
    // clearing a block, which is the whole of what the claim forbids — and it
    // is the mutation the old dial only *appeared* to make, because the sweep
    // read any successful invocation as a clearing (`system-k59` finding 4).
    val unblocked = if (BLOCK_STICKY) f.block else BNone
    val f2 =
      match f.phase {
        | FPIdle => { ...f, block: unblocked, phase: FPPreparing,
                      attempt: f.attempt + 1, handle: 1 }
        | FPPreparing => { ...f, block: unblocked, phase: FPPublished }
        | FPPublished => { ...f, block: unblocked, phase: FPDeleting, ticket: true }
        | FPDeleting => { ...f, block: unblocked, phase: FPProven, deletionProven: true }
        | FPProven => { ...f, block: unblocked }
      }
    val t2 =
      match f.phase {
        | FPIdle => { ...w.tree, reserved: RPreparing, bytes: w.tree.bytes + 1 }
        | FPPreparing => { ...w.tree, reserved: RPublished, bytes: w.tree.bytes + 1 }
        // `SY-05.b` lives here.  The root becomes absent at the moment the
        // deletion is PROVEN and at no earlier one; `DELETION_PROVEN_FIRST`
        // off removes the root one step early.
        | FPPublished =>
            if (DELETION_PROVEN_FIRST) { ...w.tree, bytes: w.tree.bytes + 1 }
            else { ...NO_TREE, bytes: w.tree.bytes + 1 }
        | FPDeleting => { ...NO_TREE, bytes: w.tree.bytes + 1 }
        | FPProven => w.tree
      }
    val w2 = if (o == OApplied) { ...w, fin: f2, tree: t2 } else w
    val d2 = afterTree(w, d, ATFinishStep, o)
    all {
      d.lease == p,
      f.phase != FPProven,
      loopAdmits(w, d, ATFinishStep),
      w' = w2,
      d' = d2,
      hist' = observe(hist, w, d, w2, d2, ATFinishStep, o, true),
    }

  action recoverOp(p: int): bool =
    val o = outcomeOn(w, d, p, d.gen, ATRecover)
    val f = w.fin
    val f2 =
      // MODEL, `SY-14.a`: the block is NOT sticky — a retried recovery clears
      // it, which is an ADMITTED action clearing a block and is the whole of
      // what the claim forbids.  The dial used to be read off the sweep's
      // "some action succeeded" test instead, which is what `system-k59`
      // finding 4 showed was measuring nothing.
      if (not(BLOCK_STICKY) and f.block != BNone)
        { ...f, block: BNone, orphaned: false }
      else if (w.tree.foreignReserved) { ...f, block: BConflict, orphaned: false }
      else if (not(RESUME_IS_IDEMPOTENT))
        // MODEL: the resume restarts the transaction from its first step,
        // repeating effects the crashed attempt had already completed.
        { ...f, phase: FPPreparing, orphaned: false }
      else if (f.ticket)
        { ...f, phase: FPProven, deletionProven: true, orphaned: false }
      else { ...f, block: BPending, orphaned: false }
    val t2 =
      if (not(w.tree.foreignReserved) and RESUME_IS_IDEMPOTENT and f.ticket)
        { ...NO_TREE, bytes: w.tree.bytes + 1 }
      else w.tree
    val w2 = if (o == OApplied) { ...w, fin: f2, tree: t2 } else w
    val d2 = afterTree(w, d, ATRecover, o)
    all {
      d.lease == p,
      loopAdmits(w, d, ATRecover),
      w' = w2,
      d' = d2,
      hist' = observe(hist, w, d, w2, d2, ATRecover, o, true),
    }

  /// `SY-14`'s instrument: the EXHAUSTIVE sweep of the admitted action set
  /// against a blocked tree, in one transition.  It is an observation — it
  /// mutates nothing — and it is classified through the same `outcomeOn` the
  /// real actions use, so it cannot agree with them by being a second spelling.
  action sweepBlocked(p: int): bool =
    // THE TWO HALVES ARE SWEPT OVER DIFFERENT SETS, and that asymmetry is the
    // corrected experiment `system-k59` finding 4 forced.
    //
    // `SY-14.a` — "no admitted action SHALL clear a block" — is swept over the
    // LITERAL admitted set.  Once clearing is measured as a state transition
    // rather than as a successful invocation, `release-lease` and
    // `validate-config` succeeding on a blocked tree stop being counter-
    // examples, because neither changes the block.  The narrowing this half
    // used to carry was hiding a faulty measurement, not a wide quantifier.
    //
    // `SY-14.b` — "every action on a blocked tree SHALL refuse naming it" — IS
    // narrowed, and the narrowing is declared: taken literally it reaches
    // `acquire-lease`, `validate-config` and `release-lease`, so a blocked tree
    // could not even release its own lease and `FN-26`'s two operator-
    // restorable exits would be unreachable.  See the README's *Narrowings*.
    val clearSet = ADMITTED
    val namingSet = ADMITTED.filter(a => touchesTree(a))
    val cleared =
      clearSet.exists(a => blockAfter(w, d, p, a) == BNone)
    val allNamed = namingSet.forall(a =>
      val o = outcomeOn(w, d, p, d.gen, a)
      isRefusal(o) and namesTheBlock(refusalOf(o), w.fin.block))
    val o = ORefused(blockRefusal(w.fin.block))
    val d2 = { ...d, lastAction: ATAmbient, lastOutcome: o }
    all {
      w.fin.block != BNone,
      d.lease == p,
      clearSet.size() > 0,
      namingSet.size() > 0,
      w' = w,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w, d2, ATAmbient, o, true),
        sweepDone: true,
        sweepCleared: hist.sweepCleared or cleared,
        sweepAllNamed: allNamed,
      },
    }

  // ---------------------------------------------------------------------------
  // Guards and contention (`SY-11`)
  // ---------------------------------------------------------------------------

  action holdTree(p: int): bool = all {
    d.lease == p,
    d.treeGuard == -1,
    hist.envUsed < ENV_BUDGET,
    not(GUARD_ORDER) or d.orderStage >= 2,
    // The order is lease, then launch generation, then tree.  A caller still
    // WAITING for the generation has not got it, so it may not take the tree —
    // which is the same rule `contendGen` enforces from the other side.
    not(GUARD_ORDER) or
      not(d.waits.keys().contains(p) and d.waits.get(p) == WGen),
    w' = w,
    d' = { ...d, treeGuard: p, orderStage: if (d.orderStage >= 2) 3 else d.orderStage },
    hist' = { ...hist,
      envUsed: hist.envUsed + 1,
      fullOrderReached: hist.fullOrderReached or d.orderStage >= 2,
      orderViolated: hist.orderViolated or d.orderStage < 2 },
  }

  action releaseTree(p: int): bool = all {
    d.treeGuard == p,
    hist.envUsed < ENV_BUDGET,
    w' = w,
    d' = { ...d, treeGuard: -1, waits: clearWait(d.waits, p) },
    hist' = { ...hist, envUsed: hist.envUsed + 1 },
  }

  /// A caller waits for the TREE guard.  Legal at every point: Grove's tree
  /// lock blocks, and a wait is not a return.
  action contendTree(p: int): bool = all {
    d.treeGuard != -1,
    d.treeGuard != p,
    not(d.waits.keys().contains(p)),
    hist.envUsed < ENV_BUDGET,
    w' = w,
    d' = { ...d, waits: d.waits.put(p, WTree) },
    hist' = { ...hist, envUsed: hist.envUsed + 1, waitChainSeen: true,
              cycleSeen: hist.cycleSeen or waitCycle({ ...d, waits: d.waits.put(p, WTree) }) },
  }

  /// A caller waits for the LAUNCH GENERATION.  `SY-11.b` forbids doing so
  /// while holding a tree guard, which is the edge that closes the cycle
  /// `bulk-marks-are-not-atomic` records.
  action contendGen(p: int): bool =
    val d2 = { ...d, waits: d.waits.put(p, WGen) }
    all {
      d.genOpen or d.genExclusive != -1,
      genHolderFor(d, p) != -1,
      not(d.waits.keys().contains(p)),
      not(GUARD_ORDER) or d.treeGuard != p,
      hist.envUsed < ENV_BUDGET,
      w' = w,
      d' = d2,
      hist' = { ...hist, envUsed: hist.envUsed + 1, waitChainSeen: true,
                cycleSeen: hist.cycleSeen or waitCycle(d2) or genWaitUnderTreeGuard(d2) },
    }

  // ---------------------------------------------------------------------------
  // The environment (`EN-08`, `EN-11`, and the operator)
  // ---------------------------------------------------------------------------

  action crashNow(p: int): bool =
    val pt = crashPoint(w, d)
    val d2 =
      if (DEATH_RELEASES)
        { ...NO_DRIVER, gen: d.gen, loopEnd: d.loopEnd, treeGuard: -1 }
      else
        { ...d, session: NO_SESSION, genShared: Set(), genExclusive: -1 }
    val w2 = { ...w, fin: { ...w.fin, orphaned: finRunning(w.fin) } }
    all {
      CRASHES,
      d.lease != -1,
      hist.envUsed < ENV_BUDGET,
      ENV_PHASES.contains(pt),
      w' = w2,
      d' = d2,
      hist' = {
        ...observe(hist, w, d, w2, d2, ATReap, ONone, false),
        envUsed: hist.envUsed + 1,
        crashPoints: hist.crashPoints.union(Set(pt)),
        deathPending: true,
        leaseSurvivedDeath: hist.leaseSurvivedDeath or d2.lease != -1,
      },
    }

  /// `EN-11`: any well-formed tree is reachable by hand edit.  The family is
  /// enumerated rather than composed from single edits, which is a SEARCH
  /// STRATEGY over exactly the space the assumption grants — removing
  /// `hand-edit` removes the whole family with it.
  action handEditTo(k: int): bool =
    val t2 =
      if (k == 1) { ...NO_TREE, present: true, charter: true, exactSubset: false,
                    liveOrdinary: 1, bytes: w.tree.bytes + 1 }
      else if (k == 2) { ...NO_TREE, present: true, charter: true,
                         witnessPresent: true, fmt: 7, liveOrdinary: 1,
                         bytes: w.tree.bytes + 1 }
      else if (k == 3) { ...NO_TREE, present: true, charter: true,
                         witnessPresent: true, malformed: true, liveOrdinary: 1,
                         bytes: w.tree.bytes + 1 }
      else { ...NO_TREE, present: true, charter: true, witnessPresent: true,
             liveOrdinary: 0, liveFinish: 0, bytes: w.tree.bytes + 1 }
    val w2 = { ...w, tree: t2 }
    all {
      HAND_EDITS,
      not(finRunning(w.fin)),
      hist.envUsed < ENV_BUDGET,
      w' = w2,
      d' = d,
      hist' = { ...observe(hist, w, d, w2, d, ATAmbient, ONone, false),
                envUsed: hist.envUsed + 1 },
    }

  action handEdit: bool =
    nondet k = oneOf(Set(1, 2, 3, 4))
    handEditTo(k)

  /// An artifact appears at a name Grove reserves and Grove cannot classify it
  /// as its own.
  action foreignWrite: bool =
    val w2 = { ...w, tree: { ...w.tree, foreignReserved: true,
                             bytes: w.tree.bytes + 1 } }
    all {
      w.tree.present,
      not(w.tree.foreignReserved),
      hist.envUsed < ENV_BUDGET,
      w' = w2,
      d' = d,
      hist' = { ...observe(hist, w, d, w2, d, ATAmbient, ONone, false),
                envUsed: hist.envUsed + 1 },
    }

  /// The layout changes under the running driver.  `SY-03`'s witness.
  action topologyChange: bool =
    val w2 = { ...w, layoutOk: not(w.layoutOk) }
    all {
      hist.envUsed < ENV_BUDGET,
      w' = w2,
      d' = d,
      hist' = {
        ...observe(hist, w, d, w2, d, ATAmbient, ONone, false),
        envUsed: hist.envUsed + 1,
        layoutChangedBetweenGates:
          hist.layoutChangedBetweenGates or d.lease != -1,
      },
    }

  action configChange: bool = all {
    hist.envUsed < ENV_BUDGET,
    w' = { ...w, configValid: not(w.configValid) },
    d' = d,
    hist' = { ...hist, envUsed: hist.envUsed + 1 },
  }

  // ---------------------------------------------------------------------------
  // The action menu
  //
  // TWO DISJUNCTS, not twenty-seven, and the reason is the subject rather than
  // the budget.  Grove's driver loop is DETERMINISTIC: given a lease, a
  // validated configuration, an open epoch and a classified root, exactly one
  // move is next.  Modelling the loop as a uniform choice over every action it
  // could ever take does not model a driver, it models a random walk that
  // happens to share its vocabulary — and it puts every deep witness at
  // `(1/k)^15`, which is the *vacuous invariant* hazard in the form an
  // executable model takes.  `flatStep` and `scenario_flat_menu` are that
  // random walk, committed, so the comparison is a measurement rather than a
  // remembered number; the README carries the replay line and the result.
  //
  // So `driverStep` IS the loop, written as the chain of conditionals a loop
  // is, and every action it can reach is still TOTAL: the branch a failed
  // guard reaches returns a named refusal rather than being absent.
  //
  // "AND IT REMOVES NO BEHAVIOUR" WAS NOT TRUE, AND `system-k59` FOUND WHERE.
  // The chain used to force a launched FINISH session through teardown, so a
  // finish session that declines, fails early, gives no signal, or takes the
  // methodology's reopening exit was unreachable — and every `SY-` property was
  // therefore checked over a world in which none of those happens.  A live
  // session's move is now `sessionMove`, and the ending is one alternative
  // among its own work at EVERY point.  What stays nondeterministic is what
  // genuinely is: which of the three endings a session takes, WHEN it takes it,
  // and what the world does between two steps.
  // ---------------------------------------------------------------------------

  /// Whether the next lifecycle move is a TRANSITION, and therefore whether the
  /// iteration has to close before it may be taken (`SY-04.a`).
  pure def nextIsTransition(wNow: World, dNow: Driver): bool =
    match classifyTree(wNow.tree) {
      | RSAbsent => not(dNow.session.live)
      | RSPartialScaffold => true
      | RSReserved(_) => wNow.fin.orphaned
      | RSCurrentSpent => true
      // Only with `ONE_FINISH_LEAF` off: the driver's next move on a
      // finish-only tree is a second APPEND, which is a transition and must
      // therefore wait for its own iteration like every other one.
      | RSCurrentFinishOnly =>
          not(ONE_FINISH_LEAF) and wNow.fin.phase == FPIdle and
          dNow.selected == -2 and not(dNow.session.live)
      | _ => false
    }

  action driverStep(p: int): bool =
    if (d.lease != p) acquireLease(p)
    // The reap window, in the order the Rust loop takes it: collect, invalidate,
    // and only then read the signal.
    else if (d.reaped) interpretSignalOp(p)
    else if (d.ending != 0 and not(d.session.live)) reapOp(p)
    else if (CONFIG_FIRST and not(d.configValidated)) validateConfig(p)
    else if (not(d.genOpen)) openEpoch(p)
    // The sweep IS the loop's next move on a blocked tree, which is what makes
    // a block terminal.  With `BLOCK_STICKY` off there is no block to be
    // terminal about, so the driver walks on and the transaction clears it.
    else if (w.fin.block != BNone and BLOCK_STICKY) sweepBlocked(p)
    // A LIVE SESSION IS THE FOREGROUND PROCESS, and it may end at any point —
    // whatever the tree looks like underneath it.  Hoisting this above the
    // classification is `system-k59` finding 1's repair.
    else if (d.session.live) sessionMove(p)
    else if (nextIsTransition(w, d) and ONE_TRANSITION and d.transitions >= 1)
      closeEpoch(p)
    else
      match classifyTree(w.tree) {
        | RSAbsent => scaffoldOp(p)
        | RSPartialScaffold => completeScaffoldOp(p)
        | RSReserved(_) => if (w.fin.orphaned) recoverOp(p) else finishStepOp(p)
        | RSCurrentSpent => appendFinishOp(p)
        | RSCurrentFinishOnly =>
            if (not(ONE_FINISH_LEAF) and w.fin.phase == FPIdle and
                d.selected == -2) appendFinishOp(p)
            else if (w.fin.phase != FPIdle) finishStepOp(p)
            else if (d.selected == -2) selectOp(p)
            else if (d.launchedHandle == -2) launchOp(p)
            else finishStepOp(p)
        | RSCurrentLive =>
            if (d.selected == -2) selectOp(p)
            else if (d.launchedHandle == -2) launchOp(p)
            else closeEpoch(p)
        // `Legacy`, `Foreign` and `Malformed` refuse every operation.  The
        // loop's own next move is still an invocation, and it still returns —
        // which is what makes `SY-13.b`'s sink test a MEASUREMENT here rather
        // than an absent transition.
        | _ => selectOp(p)
      }

  action envStep: bool = any {
    all { ENV_KINDS.contains(0), nondet p = oneOf(PROCS) crashNow(p) },
    all { ENV_KINDS.contains(1), handEdit },
    all { ENV_KINDS.contains(2), foreignWrite },
    all { ENV_KINDS.contains(3), topologyChange },
    all { ENV_KINDS.contains(4), configChange },
    all { ENV_KINDS.contains(5), insertDuringWindow },
    all { ENV_KINDS.contains(6), nondet p = oneOf(PROCS) ambientAdmit(p) },
    all { ENV_KINDS.contains(6),
          nondet p = oneOf(PROCS) (nondet g = oneOf(Set(0, 1, 2, 3)) ambientOp(p, g)) },
    all { ENV_KINDS.contains(7), nondet p = oneOf(PROCS) sessionCreateFinishOp(p) },
    all { ENV_KINDS.contains(8), nondet p = oneOf(PROCS) holdTree(p) },
    all { ENV_KINDS.contains(8), nondet p = oneOf(PROCS) releaseTree(p) },
    all { ENV_KINDS.contains(8), nondet p = oneOf(PROCS) contendTree(p) },
    all { ENV_KINDS.contains(8), nondet p = oneOf(PROCS) contendGen(p) },
  }

  /// The guard-contention menu.  `SY-11`'s two obligations are about the ORDER
  /// guards are taken in and the wait-for graph that order forbids, and neither
  /// is reached by a loop that never contends.
  action guardDance: bool = any {
    // Six driver disjuncts against four contention ones, and the driver is
    // pinned to one process: `SY-11`'s cycle needs a LAUNCH before it needs a
    // contention, and a launch is ten driver moves in.
    driverStep(0), driverStep(0), driverStep(0),
    driverStep(0), driverStep(0), driverStep(0),
    nondet p = oneOf(PROCS) ambientAdmit(p),
    nondet p = oneOf(PROCS) holdTree(p),
    nondet p = oneOf(PROCS) releaseTree(p),
    nondet p = oneOf(PROCS) contendTree(p),
    nondet p = oneOf(PROCS) contendGen(p),
  }

  /// THE NULL HYPOTHESIS, COMMITTED.  A uniform choice over the whole action
  /// vocabulary — twenty-seven disjuncts, the same actions and the same
  /// totality as `driverStep`, arranged as a random walk rather than as a loop.
  /// It exists so that the README's search-dial comparison is a REPLAYABLE
  /// measurement rather than a remembered number (`system-k59` finding 5).
  action flatStep: bool = any {
    nondet p = oneOf(PROCS) acquireLease(p),
    nondet p = oneOf(PROCS) releaseLease(p),
    nondet p = oneOf(PROCS) validateConfig(p),
    nondet p = oneOf(PROCS) openEpoch(p),
    nondet p = oneOf(PROCS) closeEpoch(p),
    nondet p = oneOf(PROCS) scaffoldOp(p),
    nondet p = oneOf(PROCS) completeScaffoldOp(p),
    nondet p = oneOf(PROCS) appendFinishOp(p),
    nondet p = oneOf(PROCS) sessionCreateFinishOp(p),
    nondet p = oneOf(PROCS) selectOp(p),
    nondet p = oneOf(PROCS) launchOp(p),
    nondet p = oneOf(PROCS) reapOp(p),
    nondet p = oneOf(PROCS) interpretSignalOp(p),
    nondet p = oneOf(PROCS) finishStepOp(p),
    nondet p = oneOf(PROCS) recoverOp(p),
    nondet p = oneOf(PROCS) sweepBlocked(p),
    nondet k = oneOf(Set(1, 2, 3)) sessionEnds(k),
    nondet p = oneOf(PROCS) ambientAdmit(p),
    nondet p = oneOf(PROCS) (nondet g = oneOf(Set(0, 1, 2, 3)) ambientOp(p, g)),
    nondet p = oneOf(PROCS) holdTree(p),
    nondet p = oneOf(PROCS) releaseTree(p),
    nondet p = oneOf(PROCS) contendTree(p),
    nondet p = oneOf(PROCS) contendGen(p),
    nondet p = oneOf(PROCS) crashNow(p),
    handEdit,
    foreignWrite,
    topologyChange,
    configChange,
    insertDuringWindow,
  }

  action step: bool =
    if (FLAT_MENU) flatStep
    else if (FOCUS == 2) guardDance
    else any {
      nondet p = oneOf(PROCS) driverStep(p),
      nondet p = oneOf(PROCS) driverStep(p),
      nondet p = oneOf(PROCS) driverStep(p),
      envStep,
    }

  // ===========================================================================
  // The claims.
  //
  // Every `inv_` is a PROPERTY command and every `wit_` is a REACHABILITY
  // command; both live in the library so that `base` checks all of them
  // unfocused and `verify_small` inherits exactly the properties.  The
  // `scenario_` instances in `lifecycle-controls.qnt` carry no commands of
  // their own except where a witness does not land at an affordable sample
  // count unfocused.
  //
  // Each flag is written at the MOMENT its claim is about — see `observe` —
  // rather than read back off live state, which is `finish-k12`'s lesson about
  // an invariant the world can falsify after the fact.
  // ===========================================================================

  // --- `SY-01` — one live driver per working tree -----------------------------

  val inv_SY_01a_a_second_driver_is_refused_never_queued: bool =
    and {
      not(hist.leaseQueued),
      not(PROCS.exists(p =>
        d.waits.keys().contains(p) and d.waits.get(p) == WLease)),
    }

  val wit_SY_01a_a_second_driver_refused: bool = hist.secondDriverRefused

  val inv_SY_01b_death_releases_ownership: bool = not(hist.leaseSurvivedDeath)

  val wit_SY_01b_a_successor_proceeds_after_a_death: bool =
    hist.successorAfterDeath

  // --- `SY-02` — the layout is proved before any tree exists ------------------

  val inv_SY_02_the_layout_is_proved_at_lease_acquisition: bool =
    and {
      not(hist.treeTouchedBeforeLayout),
      not(hist.configBeforeLayout),
    }

  val wit_SY_02_a_refusal_leaves_an_empty_working_tree_untouched: bool =
    hist.layoutRefusedOnEmptyTree

  // --- `SY-03` — a preflight is never a licence -------------------------------

  val inv_SY_03_no_gate_consults_an_earlier_layout_check: bool =
    not(hist.staleLayoutHonoured)

  val wit_SY_03_a_layout_that_changes_between_the_two_gates: bool =
    hist.layoutChangedBetweenGates and hist.lateLayoutRefusal

  // --- `SY-04` — one transition per iteration, behind a validated config ------

  val inv_SY_04a_at_most_one_lifecycle_transition_per_iteration: bool =
    d.transitions <= 1 and not(hist.twoTransitionsInOneIteration)

  val wit_SY_04a_each_transition_taken_alone: bool =
    hist.soloTransitions.size() >= 3

  val inv_SY_04b_configuration_validation_precedes_every_transition: bool =
    not(hist.transitionWithoutConfig)

  /// `SY-04.b`'s SECOND HALF, RESTATED OVER THE TRACE — entry 048 finding 5.
  ///
  /// The claim is *an invalid configuration leaves the working tree
  /// byte-identical*.  It used to be checked as `not(refusalMutated)`, set
  /// from ONE refusing operation's own before/after pair, so the only step
  /// that could ever falsify it was the refusal itself.  Nothing in this file
  /// killed it, and — the point — the claim's missing scoping could not be
  /// discovered from that shape, because the operator's hand never entered the
  /// predicate at all.
  ///
  /// Stated over the trace, EVERY step under an invalid configuration is
  /// examined — applied steps included — and the claim needs TWO narrowings,
  /// each declared and each with a control that fires.
  ///
  /// 1. **The operator is excluded.** The unscoped sentence is false: a hand
  ///    edit moves a byte under an invalid configuration and grove did
  ///    nothing.  `inv_fail_MUT_SY_04b_the_operator_writes_under_an_invalid_configuration`
  ///    is entry 042's Alloy counterexample, replayed here.
  /// 2. **A configuration that went invalid AFTER the driver validated it is
  ///    excluded** — and this one is THIS LEAF'S OWN COUNTEREXAMPLE, which no
  ///    pair-stated form could have reached.  `outcomeOn` gates transitions on
  ///    `d.configValidated`, a recorded verdict, while the layout gate beside
  ///    it reads `w.layoutOk` live.  So `configChange` after the validation
  ///    leaves the validation standing as a licence and the next transition
  ///    writes the tree under an invalid configuration.
  ///    `inv_fail_MUT_SY_04b_a_validated_configuration_is_a_licence` fires on
  ///    it, under `base`'s own constants with no dial moved.
  ///
  /// The catalogue has an obligation for exactly this shape — `SY-03`, *a
  /// preflight is never a licence* — and states it for the LAYOUT only.
  /// Whether `SY-04.b` owes the same, or whether the configuration is meant to
  /// be read once per iteration, is `lifecycle-scope-k72`'s to disposition.
  val inv_SY_04b_grove_never_mutates_the_tree_under_a_configuration_it_has_not_validated: bool =
    not(hist.groveMutatedUnderUnvalidatedConfig)

  val wit_SY_04b_an_invalid_configuration_leaves_the_tree_byte_identical: bool =
    hist.invalidConfigByteIdentical

  // --- `SY-05` — task-root absence is the fresh-tree discriminator ------------

  val inv_SY_05a_absence_is_never_read_as_evidence: bool =
    not(hist.absenceReadAsEvidence)


  /// AN INTERNAL CONSISTENCY CHECK, AND THE NAME SAYS SO.  `system-k59`
  /// finding 6 is right and the claim is downgraded here rather than defended.
  ///
  /// What `SY-05.b` wants is that `FN-11`/`FN-19` and task-root absence
  /// COMPOSE — that the inference `SY-05.a` makes is sound. What this model can
  /// answer is smaller, because `finishStepOp` chooses BOTH operands: it
  /// decides when `deletionProven` becomes true and when the summarised tree
  /// becomes absent, and `inv_SY_05b` then asks whether that one setter ever
  /// recorded the opposite ordering. `mutant_early_deletion` proves the history
  /// flag can notice a deliberately reordered setter; it cannot notice an
  /// `FN-11` or `FN-19` path the five-phase cursor omitted, because the cursor
  /// is the only thing there is.
  ///
  /// The stronger claim needs a seam that IMPORTS independently established
  /// component outcomes rather than manufacturing both operands together, and
  /// that is an abstraction change this leaf does not make: it would grow a
  /// second twenty-step transaction inside the scope whose whole point is not
  /// to have one. ROUTED BY `routing-and-prose-k73` TO THE MODEL OWNERS as a
  /// modelling-cost decision rather than a catalogue one; the README and
  /// entry 046 carry the downgrade.
  val inv_SY_05b_the_summary_never_records_absence_before_the_deletion_is_proven: bool =
    not(hist.absentBeforeProven)

  val wit_SY_05b_absence_reached_with_the_deletion_proven: bool =
    hist.provenAbsenceReached

  // --- `SY-06` — a fresh root carries a first live leaf -----------------------

  val inv_SY_06a_a_completed_scaffold_carries_a_live_leaf: bool =
    not(hist.scaffoldWithoutLeaf)

  val wit_SY_06a_a_fresh_root_distinguishable_from_a_spent_one: bool =
    hist.freshDistinguishable

  val inv_SY_06b_a_legacy_tree_is_never_completed_as_a_scaffold: bool =
    not(hist.legacyCompleted)

  val wit_SY_06b_a_partial_completed_and_a_legacy_refused: bool =
    hist.partialCompleted and hist.legacyRefused

  // --- `SY-07` — exhaustion yields exactly one finish leaf --------------------

  val inv_SY_07a_exactly_one_driver_owned_finish_leaf: bool =
    w.tree.liveFinish <= 1 and not(hist.twoFinishLeaves)

  val wit_SY_07a_an_append_and_a_reuse: bool =
    hist.finishAppended and hist.finishReused

  val inv_SY_07b_no_session_creates_a_finish_leaf: bool =
    not(hist.sessionCreatedFinish)

  val wit_SY_07b_a_refused_creation: bool = hist.sessionRefusedFinish

  // --- `SY-08` — selection is authoritative once per iteration ----------------

  val inv_SY_08_a_leaf_added_during_the_window_does_not_preempt: bool =
    not(hist.preempted)

  val wit_SY_08_a_leaf_inserted_during_the_launch_window: bool =
    hist.insertedDuringWindow and hist.launchedAfterInsert

  // --- `SY-09` — a session ends in exactly one of three ways ------------------

  val inv_SY_09a_relaunch_continues_the_loop: bool =
    not(hist.relaunchStoppedLoop)

  val wit_SY_09a_a_relaunch: bool = hist.endings.contains(1)

  /// THE REOPENING EXIT, witnessed rather than asserted.  `system-k59` finding
  /// 1 found this path unreachable; a witness is how the repair stays honest.
  val wit_SY_09a_a_finish_session_reopens_the_grove: bool = hist.reopened

  val inv_SY_09b_done_ends_the_loop: bool = not(hist.doneContinuedLoop)

  val wit_SY_09b_a_done: bool = hist.endings.contains(2)

  val inv_SY_09c_no_signal_stops_and_is_never_inferred_as_done: bool =
    not(hist.noSignalInferredDone) and not(hist.noSignalContinuedLoop)

  // --- `SY-10` — a stale session cannot act -----------------------------------

  val inv_SY_10a_a_stale_operation_is_refused_before_it_touches_the_tree: bool =
    not(hist.staleTouchedTree)

  val wit_SY_10a_a_stale_session_refused: bool = hist.staleRefused

  /// `SY-10.b`, RESTATED OVER WHAT THE ADMISSION RETURNS — entry 048 finding 5.
  ///
  /// It used to be `not(hist.silentPark)`, and `silentPark` was set from
  /// `parked = contended and not(GEN_TIMEOUT)`.  That is the dial spelled
  /// backwards: the obligation was discharged by a boolean the model wrote for
  /// itself, so the model never had to say what the visible stop RETURNS —
  /// which is entry 041's finding, and which the closed refusal set has no
  /// member for.
  ///
  /// It is quantified over the trace: of EVERY contended admission, it asks
  /// whether that admission returned a refusal naming the contention.  A park
  /// returns `ONone` and fails it, and so does a stop that borrows
  /// `SY-10.a`'s `REpochStale` — which is what `mutant_borrowed_stop_reason`
  /// establishes and what the old flag could not have asked.
  ///
  /// A SECOND trace conjunct over the wait map was tried and WITHDRAWN; see
  /// the `Hist` field comment above and the README's *Narrowings*.
  val inv_SY_10b_a_contended_generation_never_parks_silently: bool =
    not(hist.contendedStopUnnamed)

  // --- `SY-11` — the guard order admits no cycle ------------------------------

  val inv_SY_11a_lease_then_generation_then_tree: bool =
    not(hist.orderViolated)

  val wit_SY_11a_the_full_order_reached: bool = hist.fullOrderReached

  /// BOUNDED, and the bound is the claim's content: two cooperating processes,
  /// per the catalogue's own omission.  A cycle among more would not be found
  /// here, and no `SY-` obligation rests on one.
  val inv_SY_11b_no_cycle_and_no_generation_wait_under_a_tree_guard: bool =
    and {
      not(waitCycle(d)),
      not(genWaitUnderTreeGuard(d)),
      not(hist.cycleSeen),
    }

  val wit_SY_11b_a_non_empty_wait_for_graph_checked_clean: bool =
    hist.waitChainSeen

  // --- `SY-12` — restart is ordinary continuation -----------------------------

  val inv_SY_12_a_restart_never_silently_repeats_a_completed_effect: bool =
    not(hist.repeatedCompletedEffect)


  // --- `SY-13` — no stable state is a sink ------------------------------------

  /// EXISTENTIAL REACHABILITY, and stated as one.  The property iterates the
  /// DECLARED escape map exhaustively over the closed class set; the witness is
  /// the corroboration that the model's own transitions actually walk the
  /// longest declared chain, edge by edge.  A declared map nothing corroborates
  /// is fiction with a green tick, which is why the pair is inseparable here.
  val inv_SY_13a_every_stable_class_escapes_within_the_bound: bool =
    SWEEP_CLASSES.forall(c => escapesWithin(c, ESCAPE_BOUND))

  val wit_SY_13a_the_longest_declared_escape_chain_was_walked: bool =
    hist.longestWalk >= LONGEST_ESCAPE

  val inv_SY_13b_no_stable_state_is_a_sink: bool =
    and {
      not(hist.sinkSeen),
      SWEEP_CLASSES.forall(c => isGoodClass(c) or escapeTo(c) != c),
    }

  val wit_SY_13b_the_stable_sweep_reached_its_bound: bool =
    hist.stableSeen.intersect(SWEEP_CLASSES).size() >= 5

  // --- `SY-14` — a blocked tree stays blocked until an operator acts ----------

  val inv_SY_14a_no_admitted_action_clears_a_block: bool =
    not(hist.blockCleared) and not(hist.sweepCleared)


  val inv_SY_14b_every_action_on_a_blocked_tree_refuses_naming_it: bool =
    not(hist.blockedActionNotNaming) and (hist.sweepDone implies hist.sweepAllNamed)

}

// =============================================================================
// Instances
//
// `lifecycle` above carries `const` declarations and is therefore a LIBRARY.
// THE MODULE RULE IS DEFINED IN `models/run.sh`, *THE MODULE RULE*, and cited
// rather than restated: a `relax_`, `mutant_` or `scenario_` instance carries
// only its own commands; a `verify_` instance is model-checked and inherits the
// library's PROPERTY commands only; every other instance inherits all of them.
// =============================================================================

module base {
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
    ENV_KINDS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8),
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).*
}

/// The VERIFICATION instance: the same library with the search dial narrowed
/// onto the finish march, so that a symbolic backend has a chance of reaching
/// it.  Every assumption is still granted and `crash` is still present.  What
/// it buys is the direction simulation cannot give; what it costs is stated in
/// the README, and a green here is not a green over `base`'s world.
module verify_small {
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
    ENV_KINDS = Set(0),
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6, 7, 8)
  ).*
}
