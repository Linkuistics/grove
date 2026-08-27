// =============================================================================
// Grove's finish/recovery protocol, in Quint.  The `FN-` scope of
// `docs/specs/semantic-contract.md`, checked as executable guarded actions with
// total outcomes.
//
// WHAT THIS MODEL IS ABOUT.  A transaction, not a tree.  `crates/grove-task-tree/`
// owns what a task tree IS; what this file owns is the ORDER of the steps that
// end one, what each step makes persistent, and what a crash between two of
// them leaves behind.  The task tree is therefore abstracted to the set of
// entries the transaction evacuates, and every claim here is about the
// protocol.
//
// THE STEP LIST IS EXPLICIT, and `FN-24.b` is why.  That obligation demands
// that every step of the transaction have at most one persistent effect, and
// that the effect be a same-directory rename (`EN-01`) or be decomposed — with
// anything else DECLARED, and with what it would take to decompose it.  A model
// whose steps are implicit in its actions cannot answer that at all.  So `Step`
// is a closed type, `persistentEffect` is a total function on it, and the
// obligation is checked over the enumeration.
//
// TOTALITY.  No action here "does nothing when its guard is false".  Every
// action transitions and returns exactly one member of the catalogue's closed
// outcome set; a refusal is a value, not an absent transition.  That is what
// makes the refusal claims checkable at all.
//
// THE MODEL PARAMETERS ARE THE ASSUMPTION DIAL.  Each `const` below names an
// `EN-` assumption the catalogue grants, or a MODEL mutation — a dial on the
// model rather than on the world, for a claim that would otherwise be true by
// construction.  The base instance grants every assumption and disables every
// model mutation; the `relax_`/`mutant_` modules in `finish-controls.qnt` each
// turn exactly one off.
//
// THE MODULE RULE IS DEFINED IN ONE PLACE — `models/run.sh`, *THE MODULE RULE*.
// =============================================================================

module finish {

  // ---------------------------------------------------------------------------
  // The assumption dial
  // ---------------------------------------------------------------------------

  /// `EN-01`.  A rename is atomic with respect to namespace visibility.  With
  /// this false a rename is observable half-applied: both names present, or
  /// neither.
  const RENAME_ATOMIC: bool

  /// `EN-03`.  There is no atomic recursive directory deletion.  Granting the
  /// opposite is Q1's counterfactual capability: disposal becomes one step and
  /// the quarantine handoff has nothing left to protect.
  const ATOMIC_DISPOSAL: bool

  /// `EN-05`.  No filesystem transaction can include a version-control commit.
  /// Granting the opposite is Q2's counterfactual capability: the commit and
  /// the evacuation settle together, and no window exists in which the result
  /// is unobservable.
  const ATOMIC_COMMIT: bool

  /// `EN-08`.  Interruption may occur between any two steps.
  const CRASHES: bool

  /// `EN-06`/`EN-13`.  A non-cooperating writer exists, and may put a foreign
  /// artifact at a name Grove reserves.
  const FOREIGN_WRITES: bool

  /// `EN-13`, premise-break.  The reaper sweeps its reserved namespace instead
  /// of touching only what carries Grove's own cleanup manifest.
  const REAPER_SWEEPS_RESERVED: bool

  /// `EN-15`, counterfactual capability.  A machine-attested confirmation.
  /// Nothing in the protocol may become stronger for having it.
  const ATTESTED_CONFIRMATION: bool

  /// `EN-16`.  The lanes the model may choose from.  Collapsing this to one is
  /// the exercise-removal control.
  const LANES: Set[int]

  /// MODEL mutation.  With this false the four revalidation points of `FN-22`
  /// are not performed, and the handoff acts on the disposition it classified
  /// before it started.
  const REVALIDATE: bool

  /// MODEL mutation.  With this false the disposition is read off the reported
  /// exit status rather than off the evidence, which is the defect `FN-15.a`
  /// forbids and which an evidence-classifying model would otherwise satisfy by
  /// construction.
  const CLASSIFY_FROM_EVIDENCE: bool

  /// MODEL mutation.  With this false the cleanup marker's replacement is a
  /// remove followed by a create, so a reader can observe the marker absent —
  /// which is what `FN-31.b` forbids and what makes the exchange load-bearing.
  const MARKER_REPLACE_ATOMIC: bool

  /// MODEL mutation.  With this false the transaction may mutate an artifact it
  /// cannot prove is its own, which is what `TT-24`/`FN-21.b` forbid.
  const OWNERSHIP_PROVEN: bool

  /// MODEL mutation.  With this false the preflight checks only confirmation
  /// and the deterministic guards, and the remaining five preconditions are
  /// skipped.  `FN-06`, `FN-07`, `FN-08` and `FN-12.b` are otherwise satisfied
  /// by the ORDER the model happens to check things in, which is a coding habit
  /// and not a claim; with this off, all four die at once.
  const PREFLIGHT_COMPLETE: bool

  /// MODEL mutation.  With this true the fresh-tree classification reads a
  /// leftover artifact as evidence that a finish happened — which is exactly
  /// what `FN-20` forbids, and what a two-world comparison cannot catch unless
  /// there is a world in which the classifier does it.
  const LEFTOVERS_ARE_EVIDENCE: bool

  /// MODEL mutation.  With this false a recovery that finds nothing
  /// outstanding REPAIRS anyway — it rebuilds the witness it has no reason to
  /// build.  `FN-23` is otherwise idempotent because the model's at-rest branch
  /// assigns nothing, which is a coding habit rather than a claim.
  const RECOVERY_IDEMPOTENT: bool

  /// MODEL mutation.  With this false the preflight builds the witness and
  /// records its intent BEFORE it has finished checking, so a failing
  /// precondition leaves the tree and the repository moved.  `FN-05.b` and
  /// `FN-05.c` are otherwise satisfied by the way the model happens to be
  /// written — the failure branch simply assigns nothing — and a claim
  /// satisfied by a coding habit is a green tick over no evidence.
  const PREFLIGHT_PURE: bool

  /// THE SEARCH DIAL, and it removes no behaviour.
  ///
  /// An executable model of a twenty-step transaction has a problem simulation
  /// alone does not solve: while the transaction runs, every environment action
  /// is enabled at every step, so the probability of MARCHING to the end is
  /// exponentially small and the deep claims are never reached at any
  /// affordable sample count.  `ENV_BUDGET` caps how many environment actions
  /// one trace may take, and `ENV_AT` says at which program points they may be
  /// taken.  Together they concentrate the search on the situation a witness is
  /// about.
  ///
  /// The base instance grants a budget larger than any trace can spend at every
  /// point, so nothing is narrowed there and every PROPERTY is checked
  /// unfocused.  Witnesses live in the `scenario_` instances, which is the same
  /// division `crates/grove-task-tree/models/task-tree.qnt` makes.
  const ENV_BUDGET: int
  const ENV_PHASES: Set[int]
  /// Which KINDS of environment action a scenario admits:
  ///   0 `crash` · 1 `hand-edit` · 2 `foreign-write` · 3 `topology-change`
  const ENV_KINDS: Set[int]
  /// …and which member of each family.  A scenario that must reach one
  /// situation narrows to the edit that produces it; `base` admits all of them.
  /// MODEL MUTATION for `FN-30`.  With it off an operator hook installed in the
  /// repository RUNS during the internal commit.  Without the const, "no hook
  /// ran" is true because the model never sets the flag — a green on the way
  /// one line happens to be written.
  const HOOKS_SUPPRESSED: bool
  /// MODEL MUTATION for `FN-14`.  With it off the commit is unscoped and takes
  /// the unrelated working-copy bytes with it.
  const COMMIT_SCOPED: bool
  /// MODEL MUTATION for `FN-26`.  With it off a block is "cleared" by rewriting
  /// the recorded history out from under it, which is the one move `FN-26`
  /// exists to forbid and which nothing in the model could otherwise perform.
  const HISTORY_IMMUTABLE: bool

  const ENV_EDITS: Set[int]
  const ENV_FOREIGN: Set[int]
  const ENV_TOPOS: Set[int]

  /// How many ordinary entries the task root holds.  Two is enough for every
  /// claim here: one entry cannot show a partially evacuated root.
  const ENTRIES: int

  // ---------------------------------------------------------------------------
  // Lanes
  // ---------------------------------------------------------------------------

  /// One abstract outcome, three mechanisms.  The lane is a MODEL PARAMETER,
  /// not three models: every `FN-` claim is checked under all three, and a
  /// claim that holds on only some of them is a finding.
  pure val GIT: int = 0
  pure val NATIVE_JJ: int = 1
  pure val COLOCATED_JJ: int = 2

  /// A lane whose working copy IS a commit.  `FN-17.a`'s second half — the
  /// exact recorded preflight commit reproduced before the witness is removed —
  /// is stated over exactly these.
  pure def workingCopyIsACommit(l: int): bool = l != GIT

  /// A lane that carries the user's index image.  Only the colocated lane can
  /// export into a Git index it did not create.
  pure def hasIndexImage(l: int): bool = l != NATIVE_JJ

  /// A lane on which a partial commit moves the unselected witness into a
  /// SUCCESSOR, so success is the exact ticket-named PARENT of that successor
  /// rather than the ticket appearing somewhere in history.
  pure def hasPartialCommitHazard(l: int): bool = l != GIT

  // ---------------------------------------------------------------------------
  // Identities
  // ---------------------------------------------------------------------------

  /// An evacuable entry beneath the task root.  Identity, not name: a rename
  /// moves a name, and only a stable identity lets a two-state property say
  /// "*this* entry came back".
  type EntryId = int

  /// What an entry is, as far as this scope cares.  `ETOpaque` is the entry
  /// type `FN-12.b` refuses before any mutation — a socket, a device node,
  /// anything the digest cannot be taken over.
  type EntryType = ETOrdinary | ETOpaque

  /// Where an entry is.  The three places are the whole of what the protocol
  /// moves an entry between.
  type Place =
    | AtRoot          // still at its original path beneath the task root
    | InWitness       // evacuated into the published witness
    | Disposed        // unlinked, from inside the quarantine

  type Entry = { etype: EntryType, digest: int, place: Place }

  /// The finish handle — the work-item handle of the finish leaf, stable across
  /// the whole attempt and across restarts.
  pure val HANDLE: int = 7

  /// The attempt identity a `hand-edit` or an unrelated party can produce: an
  /// identity that is not this session's.  `FN-04` is the claim that it never
  /// settles this attempt.
  pure val ALIEN_ATTEMPT: int = 99

  // ---------------------------------------------------------------------------
  // The witness, the manifest, the quarantine
  // ---------------------------------------------------------------------------

  /// The reserved witness beneath the task root.
  ///   `WNone`      — no witness
  ///   `WPreparing` — built under a preparing name, not published
  ///   `WPublished` — published by exactly one atomic rename
  ///   `WTorn`      — `EN-01` relaxed: the rename is observable half-applied
  type WitnessState = WNone | WPreparing | WPublished | WTorn

  /// The evacuation manifest.  Its fields are the catalogue's, and `ready` is
  /// marked LAST — `FN-12.a`.
  type Manifest = {
    present: bool,
    handle: int,
    attempt: int,
    anchor: int,
    fingerprint: Set[EntryId],
    digests: EntryId -> int,
    ready: bool,
  }

  pure val NO_MANIFEST: Manifest = {
    present: false, handle: -1, attempt: -1, anchor: -1,
    fingerprint: Set(), digests: Map(), ready: false,
  }

  /// The cleanup marker's state inside the quarantine.  Its byte layout is a
  /// deliberate omission; what is NOT omitted is the `replace` transition,
  /// because Q3 asks whether replacement is reachable at all and a model that
  /// folds it away answers Q3 by construction.
  type MarkerState =
    | MNone
    | MLive(int)              // a marker Grove owns, carrying a value
    | MStaged({ live: int, staged: int })  // live value, and a staged replacement beside it
    | MAbsentDuringReplace    // `MARKER_REPLACE_ATOMIC` off: the observable gap
    | MForeign                // an artifact at the marker's name Grove cannot own

  /// The quarantine: the whole task root, renamed out of the way once the
  /// commit is proven.
  type Quarantine = {
    present: bool,
    handle: int,
    attempt: int,
    owned: bool,              // carries Grove's own cleanup manifest
    marker: MarkerState,
    entries: Set[EntryId],
  }

  pure val NO_QUARANTINE: Quarantine = {
    present: false, handle: -1, attempt: -1, owned: false,
    marker: MNone, entries: Set(),
  }

  // ---------------------------------------------------------------------------
  // The repository
  // ---------------------------------------------------------------------------

  /// The correlation ticket: the deletion commit's own message, naming the
  /// finish handle and the attempt identity.  This is the attempt's durable
  /// record, and it sits in version-control history rather than in any artifact
  /// the transaction can destroy.
  type Ticket = { handle: int, attempt: int }

  /// What the commit left behind.
  ///   `CSAbsent`     — no commit for this attempt
  ///   `CSExact`      — the exact attempt-bound result: the deletion, scoped
  ///   `CSSuccessor`  — the partial-commit hazard: the deletion is in the
  ///                    change and the unselected witness moved into a
  ///                    successor, so success is the ticket-named PARENT
  type CommitShape = CSAbsent | CSExact | CSSuccessor

  /// The recorded starting topology, whose shape is lane-specific and whose
  /// ROLE is not: it is what a rollback must find unchanged before it is
  /// licensed.
  type Anchor = { head: int, change: int, parents: int, index: int }

  type Repo = {
    lane: int,
    topology: Anchor,
    recorded: Anchor,
    recordedSet: bool,
    tickets: Set[Ticket],
    shape: CommitShape,
    // What the command's exit status SAID, which is not a receipt (`EN-09`).
    reported: int,            // 0 ok, 1 failed, 2 lost/late
    // Whether the EXACT IMMEDIATE RESULT can be read at all.  `EN-09` again,
    // and the state variable that makes `Indeterminate` reachable rather than
    // posited: a result may be lost or arrive late, and a repository read taken
    // across a concurrent change decides nothing.
    observable: bool,
    // The deletion fingerprint: the expected, non-empty set of tracked paths
    // the deletion commit removes.  Empty is refused before any mutation.
    tracked: Set[EntryId],
    hooksRan: bool,
    unrelatedDirty: bool,     // unrelated working-copy work, which must survive
    unrelatedMutated: bool,   // and the ghost that says it did not
    historyRewritten: bool,
    indexRestored: bool,
    preflightCommitReproduced: bool,
  }

  // ---------------------------------------------------------------------------
  // The step list.  `FN-24.b`'s subject, and this model's spine.
  // ---------------------------------------------------------------------------

  type Step =
    // ---- entry and preflight: no persistent effect anywhere in here --------
    | SIdle
    | SPreflight
    // ---- the witness ------------------------------------------------------
    | SCreatePreparing
    | SWriteManifest
    | SPublish
    | SEvacuate
    | SMarkReady
    // ---- the commit -------------------------------------------------------
    | SCommit
    // ---- the restoration handoff -----------------------------------------
    | SRevalBeforeRestore
    | SRestore
    | SReproducePreflight
    | SRevalAfterRestore
    | SRemoveWitness
    // ---- the quarantine handoff ------------------------------------------
    | SRevalBeforeQuarantine
    | SQuarantineRename
    | SRevalAfterQuarantine
    | SReturnQuarantine
    // ---- disposal ---------------------------------------------------------
    | SCreateMarker
    | SReplaceStage
    | SReplaceExchange
    | SReplaceCleanup
    | SDisposeEntry
    | SRemoveMarker
    | SRemoveQuarantine
    // ---- `EN-03` relaxed: Q1's counterfactual, disposal in place ----------
    | SDisposeInPlace
    // ---- terminal ---------------------------------------------------------
    | SSettled

  /// The persistent effect a step makes.  Total by construction, which is what
  /// `FN-24.b` needs: every step names exactly one of these.
  type Effect =
    | ENothing            // a read, a classification, a revalidation
    | ESameDirRename      // granted atomic by `EN-01`
    | ECrossDirRename     // atomic on a POSIX filesystem, NOT granted by `EN-01`
    | ECreate             // a directory or a file brought into existence
    | ERemove             // an unlink or an rmdir
    | ECommit             // a version-control commit — not a filesystem effect

  pure def persistentEffect(s: Step): Effect =
    match s {
      | SIdle                   => ENothing
      | SPreflight              => ENothing
      | SCreatePreparing        => ECreate
      | SWriteManifest          => ECreate
      | SPublish                => ESameDirRename
      | SEvacuate               => ECrossDirRename
      | SMarkReady              => ESameDirRename
      | SCommit                 => ECommit
      | SRevalBeforeRestore     => ENothing
      | SRestore                => ECrossDirRename
      | SReproducePreflight     => ECommit
      | SRevalAfterRestore      => ENothing
      | SRemoveWitness          => ERemove
      | SRevalBeforeQuarantine  => ENothing
      | SQuarantineRename       => ESameDirRename
      | SRevalAfterQuarantine   => ENothing
      | SReturnQuarantine       => ESameDirRename
      | SCreateMarker           => ECreate
      | SReplaceStage           => ECreate
      | SReplaceExchange        => ESameDirRename
      | SReplaceCleanup         => ERemove
      | SDisposeEntry           => ERemove
      | SRemoveMarker           => ERemove
      | SRemoveQuarantine       => ERemove
      | SDisposeInPlace         => ERemove
      | SSettled                => ENothing
    }

  /// The whole step list, enumerated.  `FN-24.b`'s witness is that this
  /// enumeration exists and that every member's effect is named; the invariant
  /// is that every member's effect is a same-directory rename or is DECLARED
  /// below.
  pure val ALL_STEPS: Set[Step] = Set(
    SIdle, SPreflight, SCreatePreparing, SWriteManifest, SPublish, SEvacuate,
    SMarkReady, SCommit, SRevalBeforeRestore, SRestore, SReproducePreflight,
    SRevalAfterRestore, SRemoveWitness, SRevalBeforeQuarantine,
    SQuarantineRename, SRevalAfterQuarantine, SReturnQuarantine, SCreateMarker,
    SReplaceStage, SReplaceExchange, SReplaceCleanup, SDisposeEntry,
    SRemoveMarker, SRemoveQuarantine, SDisposeInPlace, SSettled)

  /// THE DECLARED STEPS, and this is `FN-24.b`'s honest half.
  ///
  /// `EN-01` grants atomicity to a SAME-DIRECTORY rename and to nothing else.
  /// Six of the protocol's steps make a persistent effect that is not one, and
  /// the obligation's own instruction for such a step is to DECLARE it with
  /// what it would take to decompose it.  Each is declared here, in the model,
  /// rather than in prose beside it:
  ///
  ///   `SCreatePreparing`   ECreate    — a directory brought into existence at
  ///     a reserved name.  Decomposable only against a filesystem that offers
  ///     an atomic create-or-fail; POSIX `mkdir` IS that, and the honest
  ///     statement is that this step rests on `mkdir`'s exclusivity rather than
  ///     on `EN-01`.  What would change the answer: an `EN-` assumption
  ///     granting atomic exclusive creation, which the catalogue does not have.
  ///
  ///   `SWriteManifest`     ECreate    — the manifest body, written under a
  ///     not-ready name.  Already decomposed: its readiness is a SEPARATE step
  ///     (`SMarkReady`), which IS a same-directory rename, so a crash inside
  ///     the write leaves a manifest that is not ready and is discarded
  ///     (`FN-10`, `FN-12.a`).  The residual is the create itself.
  ///
  ///   `SEvacuate`          ECrossDirRename — root/<e> -> root/<witness>/<e>.
  ///     Atomic on any POSIX filesystem, and `FN-08` proves the operands are on
  ///     one device before the transaction runs — but `EN-01` as written grants
  ///     SAME-DIRECTORY renames only, so the protocol's most load-bearing move
  ///     is outside the assumption it is supposed to rest on.  This is a
  ///     finding about the catalogue, recorded and not fixed.
  ///
  ///   `SRestore`           ECrossDirRename — the same rename, backwards.
  ///
  ///   `SCommit`            ECommit    — not a filesystem effect at all, which
  ///     is exactly what `EN-05` says.  It cannot be decomposed into filesystem
  ///     steps, and the protocol's answer is not to decompose it but to make
  ///     its result CLASSIFIABLE from evidence (`FN-15`) and its window
  ///     survivable (`FN-24.a`).
  ///
  ///   `SReproducePreflight` ECommit   — as `SCommit`, on the two
  ///     working-copy-as-commit lanes.
  ///
  ///   `SRemoveWitness`, `SReplaceCleanup`, `SDisposeEntry`, `SRemoveMarker`,
  ///   `SRemoveQuarantine`  ERemove    — an unlink or an rmdir.  Each is
  ///     individually atomic and each is IDEMPOTENT — removing what is already
  ///     absent is a no-op — which is what makes disposal re-enterable
  ///     (`FN-21.a`) without any of them being a rename.  `EN-03` is the
  ///     assumption that the recursive whole is not one step; the members are.
  ///
  ///   `SCreateMarker`, `SReplaceStage`  ECreate — as `SCreatePreparing`.
  pure val DECLARED_STEPS: Set[Step] = Set(
    SCreatePreparing, SWriteManifest, SEvacuate, SRestore, SCommit,
    SReproducePreflight, SRemoveWitness, SCreateMarker, SReplaceStage,
    SReplaceCleanup, SDisposeEntry, SRemoveMarker, SRemoveQuarantine,
    SDisposeInPlace)

  /// The steps of the restoration branch, and of the quarantine branch.  Both
  /// are enumerated because `FN-24.b`'s witness is the enumeration and no one
  /// trace takes both branches within the runner's depth.
  pure val REFUSAL_BRANCH_STEPS: Set[Step] = Set(
    SPreflight, SCreatePreparing, SWriteManifest, SPublish, SEvacuate,
    SMarkReady, SCommit, SRevalBeforeRestore, SRestore, SRevalAfterRestore,
    SRemoveWitness)

  pure val SUCCESS_BRANCH_STEPS: Set[Step] = Set(
    SPreflight, SCreatePreparing, SWriteManifest, SPublish, SEvacuate,
    SMarkReady, SCommit, SRevalBeforeQuarantine, SQuarantineRename,
    SRevalAfterQuarantine, SCreateMarker, SDisposeEntry, SRemoveMarker,
    SRemoveQuarantine)

  /// The coarse phase a step belongs to.  `ENV_PHASES` is written over these
  /// rather than over the steps themselves so that an instantiation is a short
  /// set of integers rather than a transcription of the step list — and so that
  /// adding a step does not silently fall outside every scenario's reach.
  ///   0 idle or settled · 1 the witness build · 2 the commit ·
  ///   3 the revalidation points · 4 the restoration path ·
  ///   5 the quarantine handoff · 6 disposal
  pure def phaseOf(s: Step): int =
    match s {
      | SIdle => 0 | SSettled => 0 | SPreflight => 0
      | SCreatePreparing => 1 | SWriteManifest => 1 | SPublish => 1
      | SEvacuate => 1 | SMarkReady => 1
      | SCommit => 2
      | SRevalBeforeRestore => 3 | SRevalAfterRestore => 3
      | SRevalBeforeQuarantine => 3 | SRevalAfterQuarantine => 3
      | SRestore => 4 | SReproducePreflight => 4 | SRemoveWitness => 4
      | SQuarantineRename => 5 | SReturnQuarantine => 5
      | SCreateMarker => 6 | SReplaceStage => 6 | SReplaceExchange => 6
      | SReplaceCleanup => 6 | SDisposeEntry => 6 | SRemoveMarker => 6
      | SRemoveQuarantine => 6 | SDisposeInPlace => 6
    }

  /// The steps at which a crash is a DISTINCT crash boundary: the ones with a
  /// persistent effect.  A crash at a revalidation point is indistinguishable
  /// from a crash at the boundary before it — the step made nothing
  /// persistent — so `FN-24.a`'s "one crash point per step" is enumerated over
  /// exactly these.
  pure val EFFECTFUL_STEPS: Set[Step] =
    ALL_STEPS.filter(s => persistentEffect(s) != ENothing)

  // ---------------------------------------------------------------------------
  // Outcomes, refusals and diagnoses — the catalogue's closed sets
  // ---------------------------------------------------------------------------

  /// The closed refusal-reason set.  Only the members this scope can reach are
  /// spelled out; the rest belong to the task-tree scope and are not restated
  /// here, which is the catalogue's own rule about one vocabulary.
  type Refusal =
    | RRootAbsent
    | RNotLive
    | RWitnessPending
    | RReservedNameOccupied
    | RLayoutUnsupported
    | RNoTrackedDeletion
    | RRootIdentityChanged
    | RUnsupportedEntryType
    // NO LONGER AN ABSTRACTION — the catalogue's own member, and this atom is
    // the finding that put it there.  It was declared here because a finish
    // whose commit did not land rolls back cleanly and ends as a REFUSAL
    // (`FN-29`) that none of the seventeen closed reasons named: every one of
    // them named a precondition or a guard, and none named "the deletion commit
    // is absent".  `docs/specs/semantic-contract.md` §*Outcomes* now carries
    // `DeletionNotCommitted`, and this is its spelling.  The Alloy column
    // reached the same gap independently, under its own name, which is
    // what made it a fact about the catalogue rather than a convenience here.
    // Disposed by `closed-set-additions-k74`; see
    // `docs/adr/a-refusal-leaves-nothing-standing.md`, clause 2.
    | RDeletionNotCommitted

  /// The two blocked diagnoses, closed and exhaustive over blocks (`FN-25`).
  type Diagnosis = RecoveryPending | OwnershipConflict

  /// The outcome of one invocation.  `Reported`, `Empty` and `Ambiguous` are
  /// observation outcomes and belong to the task-tree scope.
  ///
  /// `ONotEntered` IS AN ABSTRACTION THIS MODEL ADDS, and it is declared as one.
  /// The catalogue's closed outcome set has no member for "the transaction was
  /// never entered", and it says so deliberately: `FN-01`'s first preflight
  /// member "produces no refusal at all".  But an action that is total must
  /// return SOMETHING, and if want-of-confirmation produced an absent
  /// transition then `FN-01.a` would be true by construction and unfalsifiable —
  /// which is the exact hazard totality exists to remove.  So the model names
  /// the non-entry, exactly as the catalogue anticipates a model naming the
  /// guard wait it also refuses to put in the set.
  ///
  /// DISPOSED BY `closed-set-additions-k74`, AND THE ABSTRACTION STANDS.  The
  /// catalogue declines to gain a member and now says why in as many words:
  /// GROVE HAS NO CONFIRMATION GATE AT ALL — constraint 5 is *grove guides, it
  /// does not gate*, `finish_commit`'s own contract puts the confirmation on
  /// the calling session, and no Grove binary reads standard input anywhere.  A
  /// transaction not entered for want of confirmation is a call that was never
  /// made, so `FN-01`'s "produces no refusal at all" is literally true of the
  /// product, and the closed set — which covers what a COMPLETED INVOCATION
  /// returns — cannot name it without ceasing to be that set.  §*Outcomes*
  /// generalises the guard-wait paragraph to govern this case too, so the
  /// declaration below is now the rule being followed rather than a finding.
  type Outcome =
    | ONone
    | OApplied
    | ORefused(Refusal)
    | OBlocked(Diagnosis)
    | ONotEntered

  pure def isRefusal(o: Outcome): bool =
    match o { ORefused(_) => true | _ => false }

  pure def isBlock(o: Outcome): bool =
    match o { OBlocked(_) => true | _ => false }

  pure def diagnosisOf(o: Outcome): Set[Diagnosis] =
    match o { OBlocked(d) => Set(d) | _ => Set() }

  /// The seven preflight preconditions, named.  THIS IS AN ABSTRACTION, and the
  /// catalogue asks for it by name: the seven members of `FN-05.a` are not
  /// distinguishable by their outcomes — the first produces no refusal at all,
  /// and two more share `LayoutUnsupported` — so "a model that introduces a
  /// precondition-naming observable to reach them is doing what this obligation
  /// requires".
  type Precondition =
    | PCNone
    | PCConfirmation       // `FN-01`
    | PCGuards             // `SY-07`, `TT-13` — a live finish leaf, no live work
    | PCLayout             // `SY-02`
    | PCQuarantineOperand  // `FN-08`
    | PCRootIdentity       // `FN-06`
    | PCFingerprint        // `FN-07`
    | PCEntryType          // `FN-12.b`

  pure val ALL_PRECONDITIONS: Set[Precondition] = Set(
    PCConfirmation, PCGuards, PCLayout, PCQuarantineOperand, PCRootIdentity,
    PCFingerprint, PCEntryType)

  /// The commit's classification.  A disposition is NOT an outcome: it is an
  /// input to one.  `Committed` settles forward and yields `Applied`;
  /// `NotCommitted` rolls back and yields `Refused`; `Indeterminate` yields
  /// exactly one `Blocked`.
  type Disposition = DCommitted | DNotCommitted | DIndeterminate

  /// The stable states this scope can classify a task root into.  The task-tree
  /// scope owns the rest of the table; what matters here is that a crash lands
  /// in exactly ONE of them (`FN-24.a`).
  type RootState =
    | RAbsent
    | RReservedPreparing
    | RReservedPublished
    | RCurrentLive
    | RCurrentFinishOnly
    | RCurrentSpent

  // ---------------------------------------------------------------------------
  // The world
  // ---------------------------------------------------------------------------

  type World = {
    entries: EntryId -> Entry,
    rootPresent: bool,
    witness: WitnessState,
    mani: Manifest,
    quar: Quarantine,
    repo: Repo,
    // A foreign artifact at the witness's reserved name, beneath the task
    // root.  Grove cannot classify it as its own; `TT-24` fixes what happens
    // in each of the three contexts it is met from.
    foreignAtWitnessName: bool,
    // A foreign artifact at the quarantine's reserved name, beside the task
    // root.  The reaper's `FN-21.c` case.
    foreignAtQuarantineName: bool,
    // The operator input Grove cannot verify (`EN-15`).
    confirmed: bool,
    // The task root's own identity, pinned at preflight and RECHECKED at every
    // later step (`FN-06`).  A swap between two steps changes it.
    rootIdentity: int,
    // `FN-02`: intent persists as the finish leaf.  Declining, or exiting
    // without completing, leaves it live and selectable.
    finishLeafLive: bool,
    // The deterministic guards `FN-01.b` insists are not a substitute for
    // confirmation: a live finish leaf, and no live ordinary work.
    guardsOk: bool,
    // The SECOND of those two guards, as a state of the tree rather than as a
    // verdict about it.  It exists separately because `Current(Live)` is one of
    // the catalogue's stable task-root states and `FN-24.a` quantifies over
    // stable classification: a model that can only refuse entry, and can never
    // BE in that state, leaves a branch the obligation reads unreachable.
    liveWork: bool,
    // `SY-02`/`SY-03`.  The layout gate at lease acquisition, and the SAME
    // question asked again at the transaction's own rename operands (`FN-08`).
    // Two flags, because the whole content of `FN-08` is that the second is
    // never satisfied by the first.
    leaseLayoutOk: bool,
    operandSameDevice: bool,
    // A non-cooperating writer has put the witness under version control.
    witnessTracked: bool,
    // An operator hook that WOULD run on an internal commit (`FN-30`).
    hookInstalled: bool,
    // `FN-17.a`'s second half: on a working-copy-as-commit lane the exact
    // recorded preflight commit must be reproduced before the witness is
    // removed.  When it cannot be, `FN-17.b` blocks rather than proceeds.
    preflightCommitReproducible: bool,
    // `FN-22.h`: a quarantine return that cannot complete.
    returnCanComplete: bool,
    // `FN-06`: the no-follow open of the task root cannot verify its identity.
    rootIdentityVerifiable: bool,
    // `FN-19`: a task root observable half-moved into the quarantine.  Only
    // reachable with `EN-01` relaxed, which is the whole point of the flag.
    rootTorn: bool,
    // Bytes outside the task root, the witness, the quarantine and the scoped
    // commit.  `FN-27` is the claim that this never changes.
    unrelatedBytes: int,
  }

  pure def entryIds(w: World): Set[EntryId] = w.entries.keys()
  pure def entryAt(w: World, e: EntryId, p: Place): bool = w.entries.get(e).place == p
  pure def rootEntries(w: World): Set[EntryId] =
    entryIds(w).filter(e => entryAt(w, e, AtRoot))
  pure def evacuated(w: World): Set[EntryId] =
    entryIds(w).filter(e => entryAt(w, e, InWitness))

  /// Ordinary work still live beneath the task root.  What the finish
  /// transaction is about is that the tree is SPENT, so this is only ever true
  /// BEFORE entry — the deterministic guard (`PCGuards`) is what refuses it —
  /// but it is a state of the world rather than a constant, because
  /// `Current(Live)` is a stable task-root state `FN-24.a` classifies over.
  pure def liveOrdinaryWork(w: World): bool = w.liveWork

  // ---------------------------------------------------------------------------
  // Classification.  `TT-18`'s order, restricted to what this scope reaches.
  // ---------------------------------------------------------------------------

  /// EVERY classification the state satisfies, not the first one an ordered
  /// `if` would return.  That is the whole instrument for `FN-24.a`: a state
  /// classifiable two ways is exactly "a state indistinguishable from a
  /// different one", and an ordered classifier would hide it behind its own
  /// precedence.
  pure def stableStates(w: World): Set[RootState] =
    // A torn quarantine rename (`EN-01` relaxed) leaves the task root half
    // moved: it answers to its own name AND to the quarantine's, so `RAbsent`
    // and a reserved state are BOTH true of it.  That is `FN-24.a`'s
    // counterexample, and it has to be expressible for the claim to be
    // falsifiable at all.
    val absent = if (not(w.rootPresent) or w.rootTorn) Set(RAbsent) else Set()
    // A torn rename (`EN-01` relaxed) leaves BOTH reserved names present, or
    // neither: the first is two stable states, the second is a published
    // witness's contents beneath a root that classifies as current.
    val preparing =
      if (w.rootPresent and (w.witness == WPreparing or w.witness == WTorn))
        Set(RReservedPreparing) else Set()
    val published =
      if (w.rootPresent and (w.witness == WPublished or w.witness == WTorn))
        Set(RReservedPublished) else Set()
    val current =
      if (w.rootPresent and w.witness == WNone)
        (if (liveOrdinaryWork(w)) Set(RCurrentLive)
         else if (rootEntries(w).size() > 0) Set(RCurrentFinishOnly)
         else Set(RCurrentSpent))
      else Set()
    absent.union(preparing).union(published).union(current)

  /// The classification an ordinary invocation returns.  Defined over the same
  /// predicates as `stableStates`, in the catalogue's own order.
  pure def classify(w: World): RootState =
    val ss = stableStates(w)
    if (ss.contains(RReservedPreparing)) RReservedPreparing
    else if (ss.contains(RReservedPublished)) RReservedPublished
    else if (ss.contains(RAbsent)) RAbsent
    else if (ss.contains(RCurrentLive)) RCurrentLive
    else if (ss.contains(RCurrentFinishOnly)) RCurrentFinishOnly
    else RCurrentSpent

  // ---------------------------------------------------------------------------
  // The commit, its evidence, and its classification
  // ---------------------------------------------------------------------------

  pure val RS_OK: int = 0
  pure val RS_FAILED: int = 1
  pure val RS_LOST: int = 2

  pure def ticketFor(at: int): Ticket = { handle: HANDLE, attempt: at }

  /// The exact ATTEMPT-BOUND result.  On a lane with the partial-commit hazard
  /// the deletion stays in the change and the unselected witness moves into a
  /// SUCCESSOR, so success is the exact ticket-named PARENT of that successor —
  /// which is why `CSSuccessor` counts there and nowhere else.
  pure def exactResult(r: Repo, at: int): bool =
    r.tickets.contains(ticketFor(at)) and
    (r.shape == CSExact or (hasPartialCommitHazard(r.lane) and r.shape == CSSuccessor))

  /// `FN-15`.  Derived from the recorded anchor, the expected fingerprint and
  /// the exact immediate result — NEVER from an exit status.
  ///
  /// `r.observable` is `EN-09` made a state variable: a command's exit status
  /// is not a receipt, and a result may be lost or arrive late.  When the exact
  /// immediate result cannot be read, the evidence does not decide and the
  /// disposition is `Indeterminate` — which is the honest answer and the one
  /// the operator has two restorable exits from.
  pure def evidenceDisposition(r: Repo, at: int): Disposition =
    // `EN-05` RELAXED — Q2's counterfactual capability.  With the commit inside
    // the filesystem transaction, the transaction's own atomicity IS the
    // receipt: it applied or it did not, and there is no third answer to read.
    // That is the whole of the capability the counterfactual adds, and it is
    // why `FN-15.d`'s bounded-unreachability check is the evidence Q2 reads.
    if (ATOMIC_COMMIT) (if (exactResult(r, at)) DCommitted else DNotCommitted)
    else if (exactResult(r, at)) DCommitted
    else if (not(r.observable)) DIndeterminate
    else if (r.shape == CSAbsent and r.topology == r.recorded) DNotCommitted
    else DIndeterminate

  /// The MUTANT classifier: the exit status read as a receipt.  This exists so
  /// that `FN-15.a` is falsifiable at all — an evidence-classifying model
  /// satisfies it by construction, and a claim true by construction is a green
  /// tick over no evidence.
  pure def statusDisposition(r: Repo, at: int): Disposition =
    if (r.reported == RS_OK) DCommitted
    else if (r.reported == RS_FAILED) DNotCommitted
    else DIndeterminate

  pure def disposition(r: Repo, at: int): Disposition =
    if (CLASSIFY_FROM_EVIDENCE) evidenceDisposition(r, at) else statusDisposition(r, at)

  /// `FN-16`.  Restoration is licensed only by proof: the recorded anchor still
  /// holds AND the attempt-bound result is absent.
  pure def rollbackLicensed(r: Repo, at: int): bool =
    r.recordedSet and r.topology == r.recorded and not(exactResult(r, at))

  // ---------------------------------------------------------------------------
  // The transaction
  // ---------------------------------------------------------------------------

  /// A compact signature of everything a claim about "byte-identical" is
  /// stated over.  Cheaper than snapshotting the world, and it is exactly the
  /// surface `FN-05.b`, `FN-05.c`, `FN-27` and `FN-29` compare.
  type Sig = {
    places: EntryId -> Place,
    witness: WitnessState,
    manifest: bool,
    quarantine: bool,
    rootPresent: bool,
    unrelated: int,
    tickets: Set[Ticket],
    topology: Anchor,
  }

  pure def sigOf(w: World): Sig = {
    places: w.entries.keys().mapBy(e => w.entries.get(e).place),
    witness: w.witness,
    manifest: w.mani.present,
    quarantine: w.quar.present,
    rootPresent: w.rootPresent,
    unrelated: w.unrelatedBytes,
    tickets: w.repo.tickets,
    topology: w.repo.topology,
  }

  /// The TREE half and the REPOSITORY half of that signature, separately,
  /// because `FN-05.b` and `FN-05.c` are two obligations and a model that
  /// compares one blob cannot tell them apart.
  pure def treeSig(w: World): Sig = { ...sigOf(w), tickets: Set(), topology: { head: 0, change: 0, parents: 0, index: 0 } }
  pure def repoSig(w: World): Sig = { ...sigOf(w), places: Map(), witness: WNone, manifest: false, quarantine: false, rootPresent: true, unrelated: 0 }

  type Txn = {
    running: bool,
    pc: Step,
    // This launch's attempt identity.  Drawn once per launch, and the ONLY
    // thing that distinguishes two attempts on one handle.
    attempt: int,
    // Whether this launch is a RECOVERY of an earlier attempt, in which case
    // the attempt it settles is the manifest's, not its own.
    recovering: bool,
    // The attempt whose result this launch is classifying.
    settling: int,
    disp: Disposition,
    outcome: Outcome,
    precondition: Precondition,
    pinnedIdentity: int,
    // The DELETION FINGERPRINT, recorded at preflight.  Recorded rather than
    // re-read: it is the expected set of tracked paths the deletion commit
    // removes, fixed when the transaction was licensed, and a later change to
    // what the repository tracks does not retroactively empty it.
    fingerprint: Set[EntryId],
    // The number of quarantined entries the cleanup marker records as disposed.
    markerSeen: int,
    before: Sig,
    entered: bool,
    // Whether this launch has already made a PERSISTENT effect.  `FN-06`'s
    // outcome turns on it: the catalogue says a mid-transaction swap is a
    // REFUSAL, and *One artifact, three contexts* says a caller who has already
    // mutated is owed a BLOCK.  Both are satisfiable because the transaction
    // knows which side of its first persistent step it is on.
    persisted: bool,
  }

  pure val IDLE_TXN: Txn = {
    running: false, pc: SIdle, attempt: -1, recovering: false, settling: -1,
    disp: DNotCommitted, outcome: ONone, precondition: PCNone,
    pinnedIdentity: -1, fingerprint: Set(), markerSeen: 0,
    before: { places: Map(), witness: WNone, manifest: false, quarantine: false,
              rootPresent: true, unrelated: 0, tickets: Set(),
              topology: { head: 0, change: 0, parents: 0, index: 0 } },
    entered: false,
    persisted: false,
  }

  // ---------------------------------------------------------------------------
  // The ghosts — what a two-state or whole-trace claim is read off
  // ---------------------------------------------------------------------------

  type Hist = {
    // `FN-24.b` and `FN-24.a`: the steps this trace visited, and the steps it
    // was interrupted at.
    stepsVisited: Set[Step],
    crashPoints: Set[Step],
    // `FN-05.a`: which of the seven preflight preconditions has been reached.
    preconditionsHit: Set[Precondition],
    // `FN-05.b` / `FN-05.c`: a preflight failure that moved the tree, or the
    // repository.
    preflightMovedTree: bool,
    preflightMovedRepo: bool,
    // `TT-24.a` / `FN-21.b` / `FN-31.d`: a mutation against something Grove
    // could not prove is its own.
    mutatedUnproven: bool,
    // `FN-32`, PER ARTIFACT, and the split is what makes the claim falsifiable
    // one artifact at a time.  `mutatedUnproven` is set by three call sites —
    // the witness slot, the quarantine name and the cleanup marker — so a
    // control that flips the ownership gate globally kills any invariant
    // reading it, whichever site actually fired.  `FN-32` names the slot and
    // the marker and deliberately NOT the quarantine, so it reads these two and
    // each has its own control.
    mutatedUnprovenSlot: bool,
    mutatedUnprovenMarker: bool,
    // `EN-13` / `FN-27`: foreign bytes deleted.
    foreignMutated: bool,
    // `FN-27`: bytes outside the task root, the witness, the quarantine and the
    // scoped commit that changed.
    unrelatedMutated: bool,
    unrelatedMutatedOnSuccess: bool,
    unrelatedMutatedOnRefusal: bool,
    unrelatedMutatedOnBlock: bool,
    // `FN-13`: the witness reached a committed tree.
    witnessCommitted: bool,
    // `FN-09.b`: a preparing witness held an evacuated entry.
    preparingHeldEvacuee: bool,
    // `FN-11` / `FN-19` / `SY-05.b`: an absent task root observable before the
    // deletion was proven.
    rootAbsentBeforeProof: bool,
    // `FN-11`: the interval between publication and commit, with the root
    // present, unwalkable, and holding every entry.
    evacuationCompleteBeforeCommit: bool,
    // `FN-18`: the tree reconstructed after the exact commit was proven.
    restoredAfterProof: bool,
    // `FN-20`: the fresh-tree classification differed with and without the
    // leftover artifact.
    artifactReadAsReceipt: bool,
    // `FN-26`: recorded history rewritten.
    historyRewritten: bool,
    // `FN-23`: recoveries run while already terminal, and whether one changed
    // anything.
    noopRecoveries: int,
    recoveryChangedAtRest: bool,
    // `FN-31.a`: a source state from which disposal must REPLACE a marker.
    replaceRequired: bool,
    // `FN-31.b`: a reader observed the marker absent, or two markers.
    markerGapObserved: bool,
    // `FN-31.c`: a replacement interrupted at each of its steps, and resumed to
    // the same terminal state.
    replaceInterruptedAt: Set[Step],
    replaceResumedTerminal: bool,
    // `FN-21`: the reaper's own record.
    disposalResumed: bool,
    reaperDeclinedOwned: bool,
    reaperDeclinedForeign: bool,
    reaperSwept: bool,
    reaperSweptOwned: bool,
    reaperTouchedForeign: bool,
    // `FN-22`: which revalidation points were performed, and what each saw.
    revalPerformed: Set[Step],
    divertedBeforeRestore: bool,
    blockedAfterRestore: bool,
    settledAfterRestore: bool,
    divertedBeforeQuarantine: bool,
    returnedRefused: bool,
    returnedBlocked: bool,
    returnIncomplete: bool,
    appliedAfterQuarantine: bool,
    indeterminateBlockedAt: Set[Step],
    // `FN-15.a`: a result reported as failure while the exact commit exists.
    lostResultClassifiedCommitted: bool,
    classifiedFromStatus: bool,
    // `FN-04` and `FN-15.b`, recorded AT THE MOMENT a disposition is formed.
    // Stated over live state instead they are falsified by an operator dropping
    // the result AFTER the classification — which is a claim about what the
    // world may do rather than about what Grove concluded, and it is exactly
    // the situation `FN-22`'s four revalidation points exist for.  Reading it
    // as a violation would make the model contradict the claim it is modelling.
    committedWithoutTicket: bool,
    committedWithoutExactResult: bool,
    dispositionsSeen: Set[Disposition],
    committedOnLane: Set[int],
    notCommittedOnLane: Set[int],
    indeterminateOnLane: Set[int],
    recoveryPendingOnLane: Set[int],
    ownershipConflictOnLane: Set[int],
    // `FN-04`: an earlier attempt's ticket offered to a later attempt.
    earlierTicketRejected: bool,
    // `FN-03`: a retry with no local trace, settling forward on the ticket.
    settledForwardOnTicketAlone: bool,
    // `FN-02` / `FN-29`: a decline, and a refusal, each followed by a success.
    declined: bool,
    sawRefusal: bool,
    trackedWitnessRefused: bool,
    refusedThenSucceeded: bool,
    declinedThenSucceeded: bool,
    // `FN-28`: a success whose cleanup is still outstanding.
    successWithCleanupOutstanding: bool,
    // `FN-30`: an operator hook that ran during an internal commit.
    hooksRanOnInternalCommit: bool,
    hookSuppressed: bool,
    // `FN-14`: unrelated modified work present across a successful finish.
    unrelatedSurvivedSuccess: bool,
    // `FN-06`: a root identity swap between two steps, whose mutation was not
    // applied elsewhere.
    identitySwapCaught: bool,
    // `FN-06`'s own witness word: the swap REFUSED, which is available exactly
    // while the transaction has made nothing persistent.
    identitySwapRefused: bool,
    mutatedAfterSwap: bool,
    // The preflight preconditions, checked where they are CHECKABLE: at the
    // moment the transaction starts.  Stating them over live state instead
    // would make a later hand edit look like a protocol violation, which is a
    // claim about the world rather than about Grove.
    startedWithBadLayout: bool,
    startedWithBadOperand: bool,
    startedWithEmptyFingerprint: bool,
    startedWithUndigestible: bool,
    startedWithUnverifiableIdentity: bool,
    // The handoff guards, recorded where the handoff is TAKEN.
    restoredWithoutLicence: bool,
    renamedWithoutCommitted: bool,
    reproducedWhileImpossible: bool,
    // `FN-16`: restoration refused for want of the anchor, and for a present
    // attempt-bound result.
    rollbackRefusedAnchor: bool,
    rollbackRefusedResult: bool,
    // `FN-17`: an exact restoration, and one that could not reproduce.
    restorationExact: bool,
    restorationBlocked: bool,
    // `FN-10`: an unpublished witness discarded, and one refused.
    unpublishedDiscarded: bool,
    unpublishedRefused: bool,
    // `FN-12.a`: a manifest interrupted before its ready mark, recovered as not
    // ready.
    notReadyRecovered: bool,
    // `FN-08`: the operand gate failing after the lease gate passed.
    operandGateCaughtWhatLeaseMissed: bool,
    // `FN-01`: a transaction never entered, and one refused by the
    // deterministic guard.
    neverEnteredForConfirmation: bool,
    refusedByDeterministicGuard: bool,
    confirmationClaimed: bool,
    // `FN-24.a`: the classification of the state each crash left behind.
    crashClassCounts: Set[int],
    // `TT-24`-shaped: an ordinary operation, a transaction and the reaper each
    // meeting a foreign artifact at a reserved name.
    ordinaryRefusedReserved: bool,
    leftoverWhileFresh: bool,
    blockNamedArtifactAndTopology: bool,
    markerObservedDuringReplace: bool,
    replaceDeclinedForeign: bool,
    forwardAfterInterruptedEvacuation: bool,
    transactionBlockedReserved: bool,
    // `FN-32` / `FN-29.b`: the same meeting, stopped the OTHER way.  A step that
    // meets an unprovable artifact before this action has persisted anything
    // REFUSES rather than blocks, so `wit_FN_32` must read both flags or it
    // witnesses only half its own claim.
    transactionRefusedReserved: bool,
    // The blocked states swept, for `FN-25.b`'s exhaustive obligation.
    blockedStatesSeen: int,
    blockedWithoutDiagnosis: bool,
    blockedWithBoth: bool,
    nearMissResolvedToOne: bool,
    // `FN-25.b`, THE COMPARISON.  A blocked state whose CARRIED diagnosis
    // differs from the one the independent classifier gives that same state.
    // Without this the obligation checked only that the classifier returns one
    // answer, which a machine that writes the right word at each call site
    // satisfies by definition — see this file's `Classification` note.
    carriedDisagreed: bool,
    // The distinct block SITES the sweep reached, so a witness can say how wide
    // the bounded sweep was instead of asserting that one block was exhaustive.
    blockSitesSwept: Set[str],
  }

  pure val EMPTY_HIST: Hist = {
    stepsVisited: Set(), crashPoints: Set(), preconditionsHit: Set(),
    preflightMovedTree: false, preflightMovedRepo: false,
    mutatedUnproven: false, foreignMutated: false, unrelatedMutated: false,
    mutatedUnprovenSlot: false, mutatedUnprovenMarker: false,
    unrelatedMutatedOnSuccess: false, unrelatedMutatedOnRefusal: false,
    unrelatedMutatedOnBlock: false, witnessCommitted: false,
    preparingHeldEvacuee: false, rootAbsentBeforeProof: false,
    evacuationCompleteBeforeCommit: false, restoredAfterProof: false,
    artifactReadAsReceipt: false, historyRewritten: false,
    noopRecoveries: 0, recoveryChangedAtRest: false,
    replaceRequired: false, markerGapObserved: false,
    replaceInterruptedAt: Set(), replaceResumedTerminal: false,
    disposalResumed: false, reaperDeclinedOwned: false, reaperDeclinedForeign: false,
    reaperSwept: false, reaperSweptOwned: false, reaperTouchedForeign: false,
    revalPerformed: Set(), divertedBeforeRestore: false,
    blockedAfterRestore: false, settledAfterRestore: false,
    divertedBeforeQuarantine: false, returnedRefused: false,
    returnedBlocked: false, returnIncomplete: false,
    appliedAfterQuarantine: false, indeterminateBlockedAt: Set(),
    lostResultClassifiedCommitted: false, classifiedFromStatus: false,
    committedWithoutTicket: false, committedWithoutExactResult: false, dispositionsSeen: Set(),
    committedOnLane: Set(), notCommittedOnLane: Set(), indeterminateOnLane: Set(),
    recoveryPendingOnLane: Set(), ownershipConflictOnLane: Set(),
    earlierTicketRejected: false, settledForwardOnTicketAlone: false,
    declined: false, sawRefusal: false, trackedWitnessRefused: false,
    refusedThenSucceeded: false, declinedThenSucceeded: false,
    successWithCleanupOutstanding: false, hooksRanOnInternalCommit: false,
    hookSuppressed: false, unrelatedSurvivedSuccess: false,
    identitySwapCaught: false, identitySwapRefused: false, mutatedAfterSwap: false,
    startedWithBadLayout: false, startedWithBadOperand: false,
    startedWithEmptyFingerprint: false, startedWithUndigestible: false,
    startedWithUnverifiableIdentity: false,
    restoredWithoutLicence: false, renamedWithoutCommitted: false,
    reproducedWhileImpossible: false, rollbackRefusedAnchor: false,
    rollbackRefusedResult: false, restorationExact: false,
    restorationBlocked: false, unpublishedDiscarded: false,
    unpublishedRefused: false, notReadyRecovered: false,
    operandGateCaughtWhatLeaseMissed: false,
    neverEnteredForConfirmation: false, refusedByDeterministicGuard: false,
    confirmationClaimed: false, crashClassCounts: Set(),
    ordinaryRefusedReserved: false, leftoverWhileFresh: false,
    blockNamedArtifactAndTopology: false, markerObservedDuringReplace: false,
    replaceDeclinedForeign: false, forwardAfterInterruptedEvacuation: false, transactionBlockedReserved: false,
    transactionRefusedReserved: false,
    blockedStatesSeen: 0, blockedWithoutDiagnosis: false,
    blockedWithBoth: false, nearMissResolvedToOne: false,
    carriedDisagreed: false, blockSitesSwept: Set(),
  }

  // ---------------------------------------------------------------------------
  // Reading a world
  // ---------------------------------------------------------------------------

  pure val BIG: int = 1000

  pure def leastOf(s: Set[int]): int = s.fold(BIG, (a, b) => if (b < a) b else a)

  pure def nextEvacuee(w: World): int = leastOf(rootEntries(w))
  pure def nextRestoree(w: World): int = leastOf(evacuated(w))
  pure def nextDisposable(w: World): int =
    leastOf(w.quar.entries.filter(e => w.entries.get(e).place != Disposed))

  pure def disposedCount(w: World): int =
    w.quar.entries.filter(e => w.entries.get(e).place == Disposed).size()

  pure def markerValue(m: MarkerState): int =
    match m { MLive(v) => v | MStaged(p) => p.live | _ => -1 }

  pure def markerIsGroveOwned(m: MarkerState): bool =
    match m { MLive(_) => true | MStaged(_) => true | _ => false }

  /// `FN-11`'s guard, stated where the commit reads it: every ordinary root
  /// entry is inside the published witness, beneath a manifest that has been
  /// written and verified.
  pure def evacuationComplete(w: World): bool =
    w.witness == WPublished and w.mani.present and w.mani.ready and
    rootEntries(w).size() == 0 and
    evacuated(w) == entryIds(w)

  /// `FN-20`'s instrument, and it is a TWO-WORLD comparison rather than a ghost
  /// on the classifier.  A model can always assert "the classifier does not read
  /// the quarantine" by not writing that line; what it cannot fake is that the
  /// answer is the SAME with the artifact and without it.
  pure def withoutLeftovers(w: World): World =
    { ...w, quar: NO_QUARANTINE }

  pure def freshTreeVerdict(w: World): RootState =
    if (LEFTOVERS_ARE_EVIDENCE and not(w.rootPresent) and w.quar.present) RCurrentSpent
    else classify(w)

  /// The three contexts a foreign artifact at a reserved name is met from, and
  /// the catalogue fixes each.  This is the transaction's own context.
  pure def blockedByForeign(w: World): bool = w.foreignAtWitnessName

  // ---------------------------------------------------------------------------
  // Preflight
  // ---------------------------------------------------------------------------

  /// The CLOSED preflight precondition set, in the order the transaction checks
  /// it.  `FN-05.a` says it is exactly seven and that a model reaching an
  /// eighth is a finding about the list.
  ///
  /// The reserved-name gate is deliberately NOT one of the seven and is checked
  /// BEFORE them: a foreign artifact at a name Grove reserves refuses EVERY
  /// operation (`TT-19`, `TT-24.b`), so it is a task-tree gate the finish
  /// transaction inherits rather than a precondition of its own.  Counting it
  /// as an eighth would be reading a shared gate onto this claim.
  pure def preflightFailure(w: World): Precondition =
    if (not(w.confirmed)) PCConfirmation
    else if (not(w.guardsOk)) PCGuards
    else if (not(PREFLIGHT_COMPLETE)) PCNone
    else if (not(w.leaseLayoutOk)) PCLayout
    else if (not(w.operandSameDevice)) PCQuarantineOperand
    else if (not(w.rootIdentityVerifiable)) PCRootIdentity
    else if (w.repo.tracked.size() == 0) PCFingerprint
    else if (entryIds(w).exists(e => w.entries.get(e).etype == ETOpaque)) PCEntryType
    else PCNone

  /// What each member produces.  Six reasons for seven members, and that is the
  /// catalogue's own arithmetic: the first produces no refusal at all, and an
  /// unsupported layout and an unreachable quarantine operand are the SAME
  /// reason because the closed set holds exactly one reason about the
  /// workspace's shape.
  pure def preflightOutcome(p: Precondition): Outcome =
    match p {
      | PCConfirmation      => ONotEntered
      | PCGuards            => ORefused(RNotLive)
      | PCLayout            => ORefused(RLayoutUnsupported)
      | PCQuarantineOperand => ORefused(RLayoutUnsupported)
      | PCRootIdentity      => ORefused(RRootIdentityChanged)
      | PCFingerprint       => ORefused(RNoTrackedDeletion)
      | PCEntryType         => ORefused(RUnsupportedEntryType)
      | PCNone              => ONone
    }

  // ---------------------------------------------------------------------------
  // One step of the transaction
  // ---------------------------------------------------------------------------

  type Frame = { w: World, t: Txn, h: Hist }

  /// A step that cannot proceed because something Grove cannot prove is its own
  /// stands in the way, or because the root was swapped underneath it.  The
  /// transaction has already mutated, so the honest stable state is a BLOCK —
  /// which is the catalogue's own rule in *One artifact, three contexts*.
  /// `FN-29.b`, IN ONE PLACE.  A step that meets an artifact at a reserved name
  /// it cannot prove is Grove's own stops either way; WHICH way is decided by
  /// what this action has left standing, never by the fact of being inside a
  /// transaction.  Nothing persisted yet -> the tree handed back is the tree
  /// received, so `Refused(ReservedNameOccupied)`, a member the closed set has
  /// carried since before either model existed.  Something persisted ->
  /// `Blocked`, diagnosed by `pickDiagnosis`.
  ///
  /// THIS FILE ALREADY ARGUED FOR THE SPLIT AND THEN DID NOT APPLY IT.
  /// `refuseSwap`'s own comment states it exactly, for `FN-06`, and records that
  /// the catalogue did not make the split itself (entry 045, finding 6) — while
  /// three hundred lines below, `SRemoveWitness`'s discard blocked on a
  /// transaction that had persisted nothing.  The Alloy column refused at that
  /// same step and was right.  Routing every such stop through ONE function is
  /// what makes the two answers a consequence of the rule rather than three
  /// independent choices, which is how they came apart in the first place.
  /// `persistedBefore` is the caller's `f0.t.persisted` — read BEFORE `bump`,
  /// exactly as the `FN-06` guard reads it, because `bump` has already marked
  /// the step that is about to run.
  pure def stopReserved(f: Frame, persistedBefore: bool): Frame =
    if (persistedBefore) blockNow(f, "foreign")
    else
      val f1 = { ...f, t: { ...f.t, running: false, pc: SSettled,
                            outcome: ORefused(RReservedNameOccupied) } }
      { ...f1, h: { ...f1.h, transactionRefusedReserved: true } }

  pure def blockNow(f: Frame, why: str): Frame =
    val d = pickDiagnosis(f.w, f.t)
    // `FN-26`.  A block stays blocked and operator-restorable.  With
    // `HISTORY_IMMUTABLE` off the transaction "clears" it by rewriting the
    // recorded history instead — the one move the obligation forbids, and
    // otherwise a move nothing in this model can make, which would leave the
    // claim green on the absence of a transition rather than on the protocol.
    { w: if (HISTORY_IMMUTABLE) f.w
         else { ...f.w, repo: { ...f.w.repo, historyRewritten: true,
                                topology: { ...f.w.repo.topology,
                                            head: f.w.repo.topology.head - 1 } } },
      t: { ...f.t, running: false, outcome: OBlocked(d) },
      h: { ...f.h,
           transactionBlockedReserved: f.h.transactionBlockedReserved or why == "foreign",
           identitySwapCaught: f.h.identitySwapCaught or why == "identity",
           blockedStatesSeen: f.h.blockedStatesSeen + 1,
           blockSitesSwept: f.h.blockSitesSwept.union(Set(why)),
           blockedWithoutDiagnosis: f.h.blockedWithoutDiagnosis or diagnose(f.w, f.t).size() == 0,
           blockedWithBoth: f.h.blockedWithBoth or diagnose(f.w, f.t).size() > 1,
           // `FN-26`.  A block names the artifact holding the transaction, the
           // recorded and the observed topology, and the two restorable exits.
           blockNamedArtifactAndTopology: f.h.blockNamedArtifactAndTopology or
             ((f.w.witness != WNone or f.w.quar.present) and f.w.repo.recordedSet),
           recoveryPendingOnLane: if (d == RecoveryPending)
             f.h.recoveryPendingOnLane.union(Set(f.w.repo.lane)) else f.h.recoveryPendingOnLane,
           ownershipConflictOnLane: if (d == OwnershipConflict)
             f.h.ownershipConflictOnLane.union(Set(f.w.repo.lane)) else f.h.ownershipConflictOnLane } }

  /// The disposition a revalidation point re-reads.  With `REVALIDATE` off the
  /// handoff acts on the disposition it classified before it started, which is
  /// the model mutation `FN-22.a` must die under.
  pure def reobserve(w: World, t: Txn): Disposition =
    if (REVALIDATE) disposition(w.repo, t.settling) else t.disp

  pure def notePoint(h: Hist, s: Step): Hist =
    { ...h, revalPerformed: h.revalPerformed.union(Set(s)) }


  /// `FN-03`, `FN-04`.  The correlation ticket names the finish handle AND the
  /// attempt identity, and the catalogue's own reason is that this is "what
  /// lets a retry that has lost every local trace still tell ITS OWN completed
  /// attempt from someone else's".  A handle-only predicate cannot: an alien
  /// ticket — same handle, another attempt — would settle this one forward,
  /// which is exactly what `FN-04` forbids.  So the proof takes the attempt it
  /// is proving.
  pure def deletionProvenFor(w: World, at: int): bool =
    at >= 0 and w.repo.tickets.contains(ticketFor(at))

  pure def manifestMatches(w: World): bool =
    w.mani.present and
    entryIds(w).forall(e => w.entries.get(e).place == AtRoot) and
    w.mani.fingerprint.size() > 0 and
    entryIds(w).forall(e => w.mani.digests.get(e) == w.entries.get(e).digest)

  /// Where a NEW invocation resumes from, derived from the PERSISTENT STATE and
  /// from nothing else.  This is the whole of `FN-24`: a crash does not hand
  /// the next invocation a program counter, it hands it a tree, and what the
  /// tree says has to be enough.
  ///
  /// `SIdle` means "nothing of Grove's is outstanding"; `SSettled` means "the
  /// deletion is proven and there is nothing left to do but say so".
  pure def resumePoint(w: World): Step =
    if (w.witness == WTorn or w.rootTorn) SIdle
    else if (not(w.rootPresent) and w.quar.present) SRevalAfterQuarantine
    else if (not(w.rootPresent)) SSettled
    else if (w.witness == WPreparing) SRemoveWitness
    else if (w.witness == WPublished) SRevalBeforeRestore
    else SIdle

  // ---------------------------------------------------------------------------
  // One step of the transaction.
  //
  // Every case ends the same way: a new frame.  No case "does nothing"; a step
  // that cannot proceed produces a block, which is a transition and therefore
  // falsifiable.
  // ---------------------------------------------------------------------------

  pure def bump(f: Frame): Frame =
    { ...f,
      t: { ...f.t, persisted: f.t.persisted or persistentEffect(f.t.pc) != ENothing },
      h: { ...f.h, stepsVisited: f.h.stepsVisited.union(Set(f.t.pc)) } }

  pure def go(f: Frame, s: Step): Frame = { ...f, t: { ...f.t, pc: s } }

  /// A step that touches an artifact Grove cannot prove is its own.  With
  /// `OWNERSHIP_PROVEN` on this is a block and nothing moves; with it off the
  /// model mutates anyway, which is what makes `TT-24.a`'s analogue here
  /// falsifiable rather than true by construction.
  pure def meetsUnproven(f: Frame, present: bool): bool = present and OWNERSHIP_PROVEN

  pure def noteUnproven(f: Frame, present: bool): Frame =
    if (present and not(OWNERSHIP_PROVEN))
      { ...f, h: { ...f.h, mutatedUnproven: true, foreignMutated: true } }
    else f

  /// THE SAME NOTE, AT THE TWO SITES `FN-32` IS STATED OVER, flagged per
  /// artifact.  A bundle control — one `OWNERSHIP_PROVEN` dial for all three
  /// sites — cannot say WHICH artifact its kill went through, so an invariant
  /// reading the aggregate flag stays falsifiable while one of its named
  /// artifacts could be tautological.  These two are what let a control isolate.
  pure def noteUnprovenSlot(f: Frame, present: bool): Frame =
    if (present and not(OWNERSHIP_PROVEN))
      { ...f, h: { ...f.h, mutatedUnproven: true, foreignMutated: true,
                   mutatedUnprovenSlot: true } }
    else f

  pure def noteUnprovenMarker(f: Frame, present: bool): Frame =
    if (present and not(OWNERSHIP_PROVEN))
      { ...f, h: { ...f.h, mutatedUnproven: true, foreignMutated: true,
                   mutatedUnprovenMarker: true } }
    else f

  pure def settle(f: Frame, o: Outcome): Frame =
    { ...f, t: { ...f.t, running: false, pc: SSettled, outcome: o } }

  /// `FN-06`'s own outcome word.  The catalogue says a mid-transaction swap is
  /// a REFUSAL; *One artifact, three contexts* says a caller who has already
  /// mutated is owed a BLOCK.  Read together they are not in conflict on the
  /// state — they are in conflict only if the outcome is chosen without asking
  /// whether anything is persistent yet.  Before the transaction's first
  /// persistent effect the tree and the repository are byte-identical, so the
  /// refusal the catalogue names is available and is what this returns; after
  /// it, `blockNow` takes over.  WHAT REMAINS A CATALOGUE FINDING is that the
  /// catalogue does not make that split itself — see
  /// `docs/formalism-findings.md` entry 045, finding 6.
  pure def refuseSwap(f: Frame): Frame =
    val f1 = settle(f, ORefused(RRootIdentityChanged))
    { ...f1, h: { ...f1.h, identitySwapCaught: true, identitySwapRefused: true } }

  pure def applyStep(f0: Frame): Frame =
    val f = bump(f0)
    val w = f.w
    val t = f.t
    // `FN-06`.  The task root's identity is rechecked at EVERY later step, so a
    // mid-transaction swap is a stop rather than a mutation applied elsewhere.
    if (persistentEffect(t.pc) != ENothing and w.rootIdentity != t.pinnedIdentity
        and OWNERSHIP_PROVEN)
      // Nothing persistent yet -> the catalogue's own "refused"; otherwise the
      // honest stable state is a block.  `f0` rather than `f`, because `bump`
      // has already marked the step ABOUT to run.
      (if (f0.t.persisted) blockNow(f, "identity") else refuseSwap(f))
    else
    match t.pc {
      | SCreatePreparing =>
        // `FN-32` + `FN-29.b`.  Nothing of this action has persisted at the
        // FIRST body step, so this stop is a refusal; `stopReserved` decides.
        if (meetsUnproven(f, w.foreignAtWitnessName)) stopReserved(f, f0.t.persisted)
        else
          val f1 = noteUnprovenSlot(f, w.foreignAtWitnessName)
          go({ ...f1, w: { ...w, witness: WPreparing } }, SWriteManifest)

      | SWriteManifest =>
        val m: Manifest = {
          present: true, handle: HANDLE, attempt: t.attempt,
          anchor: w.repo.recorded.head, fingerprint: t.fingerprint,
          digests: entryIds(w).mapBy(e => w.entries.get(e).digest),
          ready: false,
        }
        go({ ...f, w: { ...w, mani: m } }, SPublish)

      | SPublish =>
        // `FN-09.a`.  Exactly ONE atomic same-directory rename.  With `EN-01`
        // relaxed the rename is observable half-applied, and the state is then
        // classifiable two ways — which `FN-24.a` catches.
        go({ ...f, w: { ...w, witness: if (RENAME_ATOMIC) WPublished else WTorn } },
           SEvacuate)

      | SEvacuate =>
        if (rootEntries(w).size() == 0) go(f, SMarkReady)
        else
          val e = nextEvacuee(w)
          val w1 = { ...w, entries: w.entries.set(e, { ...w.entries.get(e), place: InWitness }) }
          // `FN-09.b`.  No PREPARING witness ever holds an evacuated entry.
          val h1 = { ...f.h, preparingHeldEvacuee: f.h.preparingHeldEvacuee or w.witness != WPublished }
          // The LAST evacuation leaves for the ready mark in the same step: an
          // extra transition that only discovers there is nothing left to move
          // is not a crash boundary, because it makes nothing persistent, and
          // spending one of the runner's twenty-four on it costs a witness at
          // the other end of the trace.
          { w: w1, t: { ...t, pc: if (rootEntries(w1).size() == 0) SMarkReady else SEvacuate }, h: h1 }

      | SMarkReady =>
        // `FN-12.a`.  The ready mark is LAST, and it is a same-directory rename.
        go({ ...f, w: { ...w, mani: { ...w.mani, ready: true } } }, SCommit)

      | SRevalBeforeRestore =>
        val h1 = notePoint(f.h, SRevalBeforeRestore)
        val d = reobserve(w, t)
        val f1 = { ...f,
                   h: { ...h1, dispositionsSeen: h1.dispositionsSeen.union(Set(d)),
                        // `FN-15.a`.  Whatever the machine acts on, it must be
                        // what the EVIDENCE says — never what a status line
                        // said.  Recorded at every point a disposition is
                        // formed, so the mutant that reads the exit status is
                        // caught wherever it reads it.
                        classifiedFromStatus: h1.classifiedFromStatus or
                          d != evidenceDisposition(w.repo, t.settling),
                        committedWithoutTicket: h1.committedWithoutTicket or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(w.repo.tickets.contains(ticketFor(t.settling)))),
                        committedWithoutExactResult: h1.committedWithoutExactResult or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(exactResult(w.repo, t.settling))) },
                   t: { ...t, disp: d } }
        if (d == DCommitted)
          // `FN-22.b` and `FN-16.b`: the attempt-bound result is present, so
          // restoration is not licensed; take the forward path and restore
          // nothing.
          go({ ...f1, h: { ...f1.h, divertedBeforeRestore: true, rollbackRefusedResult: true } },
             SRevalBeforeQuarantine)
        else if (d == DIndeterminate)
          // `FN-22.j`, and `FN-16.a` when what failed is the anchor.
          val h2 = { ...f1.h,
                     rollbackRefusedAnchor: f1.h.rollbackRefusedAnchor or w.repo.topology != w.repo.recorded,
                     indeterminateBlockedAt: f1.h.indeterminateBlockedAt.union(Set(SRevalBeforeRestore)) }
          blockNow({ ...f1, h: h2 }, "indeterminate")
        else go(f1, SRestore)

      | SRestore =>
        if (evacuated(w).size() == 0)
          go(f, if (workingCopyIsACommit(w.repo.lane)) SReproducePreflight else SRevalAfterRestore)
        else
          val e = nextRestoree(w)
          val w1 = { ...w, entries: w.entries.set(e, { ...w.entries.get(e), place: AtRoot }) }
          // `FN-18`.  Once the exact commit is proven the tree is NEVER
          // reconstructed.  `FN-16`: and it is restored only under LICENCE —
          // the recorded anchor still holds and the attempt-bound result is
          // absent — recorded here, where the restoration is actually taken.
          val h1 = { ...f.h,
                     restoredAfterProof: f.h.restoredAfterProof or exactResult(w.repo, t.settling),
                     restoredWithoutLicence: f.h.restoredWithoutLicence or
                       not(rollbackLicensed(w.repo, t.settling)) }
          { w: w1,
            t: { ...t, pc: if (evacuated(w1).size() == 0)
                             (if (workingCopyIsACommit(w.repo.lane)) SReproducePreflight
                              else SRevalAfterRestore)
                           else SRestore },
            h: h1 }

      | SReproducePreflight =>
        // `FN-17.b`.  The reproduction is REVALIDATED before it is claimed;
        // with `REVALIDATE` off the handoff proceeds blindly and the
        // obligation dies, which is what makes it a claim rather than a shape
        // of the code.
        if (w.preflightCommitReproducible or not(REVALIDATE))
          go({ ...f,
               w: { ...w, repo: { ...w.repo, preflightCommitReproduced: true } },
               h: { ...f.h, reproducedWhileImpossible: f.h.reproducedWhileImpossible or
                              not(w.preflightCommitReproducible) } },
             SRevalAfterRestore)
        else
          // `FN-17.b`.  A restoration that cannot reproduce it BLOCKS rather
          // than proceeds.
          blockNow({ ...f, h: { ...f.h, restorationBlocked: true } }, "reproduce")

      | SRevalAfterRestore =>
        val h1 = notePoint(f.h, SRevalAfterRestore)
        val d = reobserve(w, t)
        val f1 = { ...f,
                   h: { ...h1, dispositionsSeen: h1.dispositionsSeen.union(Set(d)),
                        // `FN-15.a`.  Whatever the machine acts on, it must be
                        // what the EVIDENCE says — never what a status line
                        // said.  Recorded at every point a disposition is
                        // formed, so the mutant that reads the exit status is
                        // caught wherever it reads it.
                        classifiedFromStatus: h1.classifiedFromStatus or
                          d != evidenceDisposition(w.repo, t.settling),
                        committedWithoutTicket: h1.committedWithoutTicket or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(w.repo.tickets.contains(ticketFor(t.settling)))),
                        committedWithoutExactResult: h1.committedWithoutExactResult or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(exactResult(w.repo, t.settling))) },
                   t: { ...t, disp: d } }
        if (d == DNotCommitted)
          go({ ...f1, h: { ...f1.h, settledAfterRestore: true,
                           restorationExact: f1.h.restorationExact or manifestMatches(w) } },
             SRemoveWitness)
        else if (d == DCommitted)
          // `FN-22.c`.  The witness stays, blocking the restored tree.
          blockNow({ ...f1, h: { ...f1.h, blockedAfterRestore: true } }, "after-restore")
        else
          blockNow({ ...f1, h: { ...f1.h, indeterminateBlockedAt: f1.h.indeterminateBlockedAt.union(Set(SRevalAfterRestore)) } }, "indeterminate")

      | SRemoveWitness =>
        // Reached two ways: as the last step of a successful rollback, and as
        // `FN-10.a`'s discard of an UNPUBLISHED witness.
        val discarding = w.witness == WPreparing
        // `FN-10.b`.  Content the discard cannot classify as Grove's own fails
        // CLOSED — it is never interpreted, and never removed.
        if (discarding and (w.foreignAtWitnessName or (w.mani.present and w.mani.handle != HANDLE)))
          // `FN-10.b` + `FN-29.b`.  THIS SITE MOVED.  It blocked, and the
          // Alloy column refused at the same step; both were green against text
          // that said only *fails closed*.  A recovery meeting unclassifiable
          // content at the witness slot has persisted nothing, so the tree it
          // hands back is the tree it received and the outcome is
          // `Refused(ReservedNameOccupied)` — which is what `refuseSwap`'s own
          // comment in this file already required and what the catalogue now
          // states.  Disposed by `closed-set-additions-k74`.
          stopReserved({ ...f, h: { ...f.h, unpublishedRefused: true } }, f0.t.persisted)
        else
          val h1 = { ...f.h,
                     unpublishedDiscarded: f.h.unpublishedDiscarded or discarding,
                     notReadyRecovered: f.h.notReadyRecovered or (w.mani.present and not(w.mani.ready)) }
          settle({ ...f, w: { ...w, witness: WNone, mani: NO_MANIFEST }, h: h1 },
                 ORefused(RDeletionNotCommitted))

      | SRevalBeforeQuarantine =>
        val h1 = notePoint(f.h, SRevalBeforeQuarantine)
        val d = reobserve(w, t)
        val f1 = { ...f,
                   h: { ...h1, dispositionsSeen: h1.dispositionsSeen.union(Set(d)),
                        // `FN-15.a`.  Whatever the machine acts on, it must be
                        // what the EVIDENCE says — never what a status line
                        // said.  Recorded at every point a disposition is
                        // formed, so the mutant that reads the exit status is
                        // caught wherever it reads it.
                        classifiedFromStatus: h1.classifiedFromStatus or
                          d != evidenceDisposition(w.repo, t.settling),
                        committedWithoutTicket: h1.committedWithoutTicket or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(w.repo.tickets.contains(ticketFor(t.settling)))),
                        committedWithoutExactResult: h1.committedWithoutExactResult or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(exactResult(w.repo, t.settling))) },
                   t: { ...t, disp: d } }
        if (d == DCommitted)
          go(f1, if (ATOMIC_DISPOSAL) SDisposeInPlace else SQuarantineRename)
        else if (d == DNotCommitted)
          // `FN-22.e`.  Divert to the restoration path and rename nothing.
          go({ ...f1, h: { ...f1.h, divertedBeforeQuarantine: true } }, SRevalBeforeRestore)
        else
          blockNow({ ...f1, h: { ...f1.h, indeterminateBlockedAt: f1.h.indeterminateBlockedAt.union(Set(SRevalBeforeQuarantine)) } }, "indeterminate")

      | SQuarantineRename =>
        // `FN-32` + `FN-29.b`.  Publication, evacuation and the commit are
        // behind this step, so an effect stands and the stop is a BLOCK.  Same
        // rule as the witness-slot site above, opposite answer, and that the two
        // differ is the content of the claim rather than an inconsistency.
        if (meetsUnproven(f, w.foreignAtQuarantineName)) stopReserved(f, f0.t.persisted)
        else
          val f1 = noteUnproven({ ...f, h: { ...f.h,
                       renamedWithoutCommitted: f.h.renamedWithoutCommitted or t.disp != DCommitted } },
                     w.foreignAtQuarantineName)
          // `FN-19`.  The WHOLE task root — witness and evacuated tree intact —
          // moves in ONE step.  No partial or empty task root is ever
          // observable, which is exactly what `EN-01` buys.
          if (RENAME_ATOMIC)
            go({ ...f1, w: { ...f1.w,
                             rootPresent: false,
                             // The witness went WITH the root — that is what
                             // "witness and evacuated tree intact" means — so
                             // there is no longer an IN-TREE witness, and the
                             // correlation that survives is the quarantine's.
                             witness: WNone,
                             quar: { present: true, handle: HANDLE, attempt: t.settling,
                                     owned: true, marker: MNone, entries: entryIds(f1.w) } } },
               SRevalAfterQuarantine)
          else
            go({ ...f1, w: { ...f1.w, rootTorn: true } }, SRevalAfterQuarantine)

      | SRevalAfterQuarantine =>
        val h1 = notePoint(f.h, SRevalAfterQuarantine)
        val d = reobserve(w, t)
        val f1 = { ...f,
                   h: { ...h1, dispositionsSeen: h1.dispositionsSeen.union(Set(d)),
                        // `FN-15.a`.  Whatever the machine acts on, it must be
                        // what the EVIDENCE says — never what a status line
                        // said.  Recorded at every point a disposition is
                        // formed, so the mutant that reads the exit status is
                        // caught wherever it reads it.
                        classifiedFromStatus: h1.classifiedFromStatus or
                          d != evidenceDisposition(w.repo, t.settling),
                        committedWithoutTicket: h1.committedWithoutTicket or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(w.repo.tickets.contains(ticketFor(t.settling)))),
                        committedWithoutExactResult: h1.committedWithoutExactResult or
                          (d == DCommitted and not(ATOMIC_COMMIT) and
                           not(exactResult(w.repo, t.settling))) },
                   t: { ...t, disp: d } }
        if (d == DCommitted)
          // `FN-22.i` and `FN-28`.  The finish has SUCCEEDED here; disposal is
          // best-effort cleanup and its being outstanding does not make a
          // proven finish unsuccessful.
          go({ ...f1,
               t: { ...f1.t, outcome: OApplied },
               h: { ...f1.h, appliedAfterQuarantine: true,
                    successWithCleanupOutstanding: true,
                    unrelatedSurvivedSuccess: f1.h.unrelatedSurvivedSuccess or w.repo.unrelatedDirty } },
             SCreateMarker)
        else go(f1, SReturnQuarantine)

      | SReturnQuarantine =>
        if (not(w.returnCanComplete))
          // `FN-22.h`.  Report BOTH the change and the quarantine, and block.
          blockNow({ ...f, h: { ...f.h, returnIncomplete: true } }, "return")
        else
          // The return puts the root back exactly as it was — the published
          // witness with it, which is what makes `FN-22.f`'s "byte-equal to the
          // pre-rename tree" statable.
          val w1 = { ...w, rootPresent: true, witness: WPublished, quar: NO_QUARANTINE }
          if (t.disp == DNotCommitted)
            // `FN-22.f`.  The exact pre-rename state, from which the
            // restoration path runs and the attempt completes as `Refused`.
            go({ ...f, w: w1, h: { ...f.h, returnedRefused: true } }, SRevalBeforeRestore)
          else
            // `FN-22.g`.
            blockNow({ ...f, w: w1,
                       h: { ...f.h, returnedBlocked: true } }, "returned")

      | SCreateMarker =>
        if (meetsUnproven(f, w.quar.marker == MForeign))
          // `FN-31.d`.  Never against a marker Grove cannot prove is its own.
          // `FN-31.d`.  A replacement is never performed against a marker Grove
          // cannot prove is its own.
          settle({ ...f, h: { ...f.h, replaceDeclinedForeign: true } }, OApplied)
        else
          val f1 = noteUnprovenMarker(f, w.quar.marker == MForeign)
          // `FN-21.a`.  Disposal is RE-ENTERABLE from any interruption, so this
          // step is where a resumed disposal rejoins the protocol — and what it
          // rejoins is decided by the marker it finds, never by a program
          // counter a crash did not preserve.  `FN-31.c` is the same property
          // one level down, inside the replacement.
          val resumed = w.quar.marker != MNone
          val f2 = { ...f1, h: { ...f1.h, disposalResumed: f1.h.disposalResumed or resumed } }
          match w.quar.marker {
            | MNone      => go({ ...f2, w: { ...f2.w, quar: { ...f2.w.quar, marker: MLive(0) } } }, SDisposeEntry)
            | MStaged(_) => go(f2, SReplaceExchange)
            | MAbsentDuringReplace => go(f2, SReplaceCleanup)
            | _          => go(f2, SDisposeEntry)
          }

      | SDisposeEntry =>
        if (nextDisposable(w) == BIG) go(f, SRemoveMarker)
        else
          val e = nextDisposable(w)
          val w1 = { ...w, entries: w.entries.set(e, { ...w.entries.get(e), place: Disposed }) }
          // `FN-31.a`.  The source state from which disposal must REPLACE the
          // marker rather than create or remove one: the marker is present,
          // owned, and carries a value the next step must supersede.
          val needsReplace = markerValue(w.quar.marker) < disposedCount(w1)
          { w: w1,
            t: { ...t, pc: if (needsReplace) SReplaceStage else SDisposeEntry },
            h: { ...f.h, replaceRequired: f.h.replaceRequired or needsReplace } }

      | SReplaceStage =>
        go({ ...f, w: { ...w, quar: { ...w.quar,
              marker: MStaged({ live: markerValue(w.quar.marker), staged: disposedCount(w) }) } } },
           SReplaceExchange)

      | SReplaceExchange =>
        // `FN-31.b`.  ONE atomic same-directory exchange: no reader observes
        // the marker absent, nor observes two markers.  With
        // `MARKER_REPLACE_ATOMIC` off it is a remove followed by a create, and
        // the gap between them is observable — which is what the obligation
        // forbids and what makes the exchange load-bearing rather than assumed.
        val m = if (MARKER_REPLACE_ATOMIC) MLive(disposedCount(w)) else MAbsentDuringReplace
        go({ ...f, w: { ...w, quar: { ...w.quar, marker: m } } }, SReplaceCleanup)

      | SReplaceCleanup =>
        val w1 = { ...w, quar: { ...w.quar, marker: MLive(disposedCount(w)) } }
        { w: w1,
          t: { ...t, markerSeen: disposedCount(w),
                     pc: if (nextDisposable(w1) == BIG) SRemoveMarker else SDisposeEntry },
          h: f.h }

      | SRemoveMarker =>
        go({ ...f, w: { ...w, quar: { ...w.quar, marker: MNone } } }, SRemoveQuarantine)

      | SRemoveQuarantine =>
        settle({ ...f, w: { ...w, quar: NO_QUARANTINE } }, OApplied)

      | SDisposeInPlace =>
        // `EN-03` relaxed — Q1's counterfactual capability.  One atomic
        // recursive deletion settles the whole thing: no quarantine, no
        // handoff, no cleanup marker.
        settle({ ...f, w: { ...w, rootPresent: false, witness: WNone,
                            mani: NO_MANIFEST,
                            entries: entryIds(w).mapBy(e => { ...w.entries.get(e), place: Disposed }) } },
               OApplied)

      | _ => settle(f, t.outcome)
    }

  // ---------------------------------------------------------------------------
  // The two diagnoses, classified from the STATE rather than chosen at the call
  // site.  `FN-25` is the claim that the partition is disjoint, exhaustive over
  // blocks, and reachable on every lane — and none of the three is checkable
  // against a model that simply writes the right word at each `blockNow`.
  //
  // So the classifier below is INDEPENDENT of the ten call sites, and `FN-25.b`
  // is stated as the agreement between them.  A call site that names the wrong
  // diagnosis is then a counterexample rather than a definition.
  // ---------------------------------------------------------------------------

  /// A correlated, Grove-owned artifact holds the transaction: it is named by
  /// THIS finish handle and THIS attempt identity, and the outcome cannot yet
  /// be proven either way.
  pure def groveOwnedCorrelated(w: World, t: Txn): bool =
    (w.witness == WPublished and w.mani.present and
     w.mani.handle == HANDLE and w.mani.attempt == t.settling) or
    (w.quar.present and w.quar.owned and w.quar.handle == HANDLE and
     w.quar.attempt == t.settling)

  /// State that is unrelated, ambiguous, or cannot be proved safe to mutate.
  /// An artifact at a name Grove reserves that Grove CANNOT classify as its
  /// own.  Note what this is not: a published witness and a quarantine both sit
  /// at reserved names and are both provably Grove's, which is the whole
  /// content of `FN-25.a`'s near miss.
  pure def cannotClassify(w: World): bool =
    w.foreignAtWitnessName or w.foreignAtQuarantineName or w.quar.marker == MForeign

  /// State that is unrelated, ambiguous, or cannot be proved safe to mutate.
  pure def unprovable(w: World, t: Txn): bool =
    cannotClassify(w) or
    // The task root was SWAPPED under the transaction.  Whatever sits at the
    // reserved name now sits inside a root Grove did not pin, so Grove cannot
    // prove the artifact is its own — which is `OwnershipConflict`'s own
    // definition rather than an extension of it.  Found as a counterexample to
    // `FN-25.b`: without this clause a mid-transaction swap blocks with a
    // diagnosis the catalogue's partition does not cover — STILL TRUE after the
    // `FN-06` outcome split, and measured rather than assumed: removing this
    // line violates `inv_FN_25b` in `base` and in `scenario_edit_txn`.  What the
    // split changed is WHICH swaps reach here.  A swap caught before the
    // transaction's first persistent effect is now a refusal (`refuseSwap`) and
    // is outside `FN-25` entirely, which is where the catalogue's own `FN-06`
    // wording puts it; a swap caught after one is a block, and the partition
    // genuinely needs this clause to cover it.
    (t.pinnedIdentity >= 0 and w.rootIdentity != t.pinnedIdentity) or
    (w.repo.recordedSet and w.repo.topology != w.repo.recorded and
     not(exactResult(w.repo, t.settling))) or
    not(w.returnCanComplete) or
    entryIds(w).exists(e => w.entries.get(e).etype == ETOpaque)

  /// THE NEAR MISS `FN-25.a` asks for, resolved.  A Grove-owned, correlated
  /// artifact sitting at a name Grove also reserves reads true on both
  /// predicates on the surface — and the resolution is not a tie-break but a
  /// reading of what `OwnershipConflict` says: *Grove cannot classify it as its
  /// own*.  Of an artifact Grove has just proved is its own, that is FALSE.
  /// Correlation therefore wins, and it wins for a reason rather than by
  /// precedence.
  pure def diagnose(w: World, t: Txn): Set[Diagnosis] =
    val correlated = groveOwnedCorrelated(w, t)
    if (correlated and not(cannotClassify(w))) Set(RecoveryPending)
    else if (unprovable(w, t)) Set(OwnershipConflict)
    else if (correlated) Set(RecoveryPending)
    else Set()

  /// The diagnosis a block CARRIES.  Derived from the state by the classifier
  /// above rather than chosen at each of the ten call sites — and the fallback
  /// is what `FN-25.b` is checked over: a blocked state the predicates do not
  /// cover gets a diagnosis anyway, and the ghost records that they did not
  /// cover it.  That is what makes "no blocked state carries neither" a
  /// counterexample rather than a definition.
  pure def pickDiagnosis(w: World, t: Txn): Diagnosis =
    if (diagnose(w, t).contains(RecoveryPending)) RecoveryPending else OwnershipConflict

  // ---------------------------------------------------------------------------
  // State
  // ---------------------------------------------------------------------------

  var w: World
  var t: Txn
  var hist: Hist
  var nextAttempt: int
  var envUsed: int

  pure val CLEAN_ANCHOR: Anchor = { head: 1, change: 1, parents: 1, index: 1 }

  pure def freshRepo(l: int): Repo = {
    lane: l,
    topology: CLEAN_ANCHOR,
    recorded: CLEAN_ANCHOR,
    recordedSet: false,
    tickets: Set(),
    shape: CSAbsent,
    reported: RS_OK,
    observable: true,
    tracked: 1.to(ENTRIES),
    hooksRan: false,
    unrelatedDirty: false,
    unrelatedMutated: false,
    historyRewritten: false,
    indexRestored: false,
    preflightCommitReproduced: false,
  }

  action init: bool =
    nondet l = oneOf(LANES)
    all {
      w' = {
        entries: 1.to(ENTRIES).mapBy(e => { etype: ETOrdinary, digest: e, place: AtRoot }),
        rootPresent: true,
        witness: WNone,
        mani: NO_MANIFEST,
        quar: NO_QUARANTINE,
        repo: freshRepo(l),
        foreignAtWitnessName: false,
        foreignAtQuarantineName: false,
        confirmed: false,
        rootIdentity: 1,
        finishLeafLive: true,
        guardsOk: true,
        liveWork: false,
        leaseLayoutOk: true,
        operandSameDevice: true,
        witnessTracked: false,
        hookInstalled: false,
        preflightCommitReproducible: true,
        returnCanComplete: true,
        rootIdentityVerifiable: true,
        rootTorn: false,
        unrelatedBytes: 42,
      },
      t' = IDLE_TXN,
      hist' = EMPTY_HIST,
      nextAttempt' = 1,
      envUsed' = 0,
    }

  // ---------------------------------------------------------------------------
  // The operator's own action (`EN-15`)
  // ---------------------------------------------------------------------------

  /// `confirm`.  Grove cannot verify that it happened, and `ATTESTED_CONFIRMATION`
  /// is the counterfactual capability that says it could.  Nothing in the
  /// protocol may become STRONGER for having it: `FN-01.a` still forbids running
  /// without confirmation and `FN-01.b` still refuses the deterministic guard as
  /// a substitute.  A run in which some obligation does strengthen is the
  /// finding.
  action confirmOp: bool = all {
    not(t.running),
    not(w.confirmed),
    w' = { ...w, confirmed: true },
    t' = t, hist' = hist, nextAttempt' = nextAttempt, envUsed' = envUsed,
  }

  // ---------------------------------------------------------------------------
  // Entry
  // ---------------------------------------------------------------------------

  action beginFinish: bool = all {
    not(t.running),
    w.rootPresent,
    // A reserved witness refuses every ordinary operation; recovery is the
    // action for it, not a second attempt.
    if (w.foreignAtWitnessName)
      all {
        t' = { ...IDLE_TXN, outcome: ORefused(RReservedNameOccupied) },
        hist' = { ...hist, ordinaryRefusedReserved: true },
        w' = w, nextAttempt' = nextAttempt, envUsed' = envUsed,
      }
    else if (w.witness != WNone)
      all {
        t' = { ...IDLE_TXN, outcome: ORefused(RWitnessPending) },
        hist' = hist, w' = w, nextAttempt' = nextAttempt, envUsed' = envUsed,
      }
    else
      val p = preflightFailure(w)
      if (p != PCNone)
        // `FN-05.b` and `FN-05.c`: a preflight failure leaves the tree AND the
        // repository byte-identical.  `PREFLIGHT_PURE` is what makes that a
        // claim rather than a coding habit — see the const's own note.
        val wAfter = if (PREFLIGHT_PURE) w
                     else { ...w, witness: WPreparing,
                            repo: { ...w.repo, tickets: w.repo.tickets.union(Set(ticketFor(nextAttempt))) } }
        all {
          w' = wAfter,
          t' = { ...IDLE_TXN, outcome: preflightOutcome(p), precondition: p },
          hist' = { ...hist,
                    preconditionsHit: hist.preconditionsHit.union(Set(p)),
                    preflightMovedTree: hist.preflightMovedTree or treeSig(wAfter) != treeSig(w),
                    preflightMovedRepo: hist.preflightMovedRepo or repoSig(wAfter) != repoSig(w),
                    declined: hist.declined or p == PCConfirmation,
                    neverEnteredForConfirmation: hist.neverEnteredForConfirmation or p == PCConfirmation,
                    refusedByDeterministicGuard: hist.refusedByDeterministicGuard or p == PCGuards,
                    operandGateCaughtWhatLeaseMissed:
                      hist.operandGateCaughtWhatLeaseMissed or
                      (p == PCQuarantineOperand and w.leaseLayoutOk) },
          nextAttempt' = nextAttempt,
          envUsed' = envUsed,
        }
      else
        all {
          w' = { ...w, repo: { ...w.repo, recorded: w.repo.topology, recordedSet: true } },
          t' = { ...IDLE_TXN,
                 running: true, pc: SCreatePreparing,
                 attempt: nextAttempt, settling: nextAttempt,
                 pinnedIdentity: w.rootIdentity,
                 fingerprint: w.repo.tracked,
                 before: sigOf(w), entered: true },
          hist' = { ...hist,
                    stepsVisited: hist.stepsVisited.union(Set(SPreflight, SIdle)),
                    startedWithBadLayout: hist.startedWithBadLayout or not(w.leaseLayoutOk),
                    startedWithBadOperand: hist.startedWithBadOperand or not(w.operandSameDevice),
                    startedWithEmptyFingerprint: hist.startedWithEmptyFingerprint or w.repo.tracked.size() == 0,
                    startedWithUndigestible: hist.startedWithUndigestible or
                      entryIds(w).exists(e => w.entries.get(e).etype == ETOpaque),
                    startedWithUnverifiableIdentity: hist.startedWithUnverifiableIdentity or
                      not(w.rootIdentityVerifiable) },
          nextAttempt' = nextAttempt + 1,
          envUsed' = envUsed,
        }
  }

  // ---------------------------------------------------------------------------
  // Ghost bookkeeping every completed step passes through
  // ---------------------------------------------------------------------------

  pure def postStep(before: World, f: Frame): Hist =
    val w1 = f.w
    val t1 = f.t
    val settled = not(t1.running)
    val unrelatedMoved = w1.unrelatedBytes != t1.before.unrelated
    { ...f.h,
      // `FN-11`, `FN-19`, `SY-05.b`.  No trace exposes an absent task root
      // before the deletion is proven.
      rootAbsentBeforeProof: f.h.rootAbsentBeforeProof or
        (not(w1.rootPresent) and not(deletionProvenFor(w1, t1.settling))),
      // `FN-31.b`.  A reader observing the marker absent is a reachable STATE,
      // not an event, so the ghost records the state rather than a moment.
      markerGapObserved: f.h.markerGapObserved or w1.quar.marker == MAbsentDuringReplace,
      // `FN-31.c`.  A replacement resumed after an interruption reaches the
      // same terminal state an uninterrupted one does: the marker records
      // exactly what has been disposed.
      replaceResumedTerminal: f.h.replaceResumedTerminal or
        (f.h.replaceInterruptedAt.size() > 0 and t1.pc == SRemoveMarker and
         markerValue(w1.quar.marker) == disposedCount(w1)),
      // `FN-06`.  A persistent effect that landed while the pinned identity no
      // longer matched — a mutation applied ELSEWHERE, which is what the
      // recheck at every later step exists to prevent.
      mutatedAfterSwap: f.h.mutatedAfterSwap or
        (sigOf(w1) != sigOf(before) and w1.rootIdentity != t1.pinnedIdentity),
      // `FN-31.b`.  A state in which a reader could look while the replacement
      // is in flight — the staged name beside the live one — and see exactly
      // one marker.
      markerObservedDuringReplace: f.h.markerObservedDuringReplace or
        match w1.quar.marker { MStaged(_) => true | _ => false },
      // `FN-18`.  A proven commit reached AFTER an interruption mid-evacuation.
      forwardAfterInterruptedEvacuation: f.h.forwardAfterInterruptedEvacuation or
        (f.h.crashPoints.contains(SEvacuate) and f.h.divertedBeforeRestore),
      // `FN-27`, per outcome.
      unrelatedMutated: f.h.unrelatedMutated or unrelatedMoved,
      unrelatedMutatedOnSuccess: f.h.unrelatedMutatedOnSuccess or (settled and t1.outcome == OApplied and unrelatedMoved),
      unrelatedMutatedOnRefusal: f.h.unrelatedMutatedOnRefusal or (settled and isRefusal(t1.outcome) and unrelatedMoved),
      unrelatedMutatedOnBlock: f.h.unrelatedMutatedOnBlock or (settled and isBlock(t1.outcome) and unrelatedMoved),
      // `FN-02`, `FN-29`.  A decline and a refusal each followed by a success.
      sawRefusal: f.h.sawRefusal or (settled and isRefusal(t1.outcome)),
      // A finish has SUCCEEDED the moment the proven commit is settled forward
      // (`FN-28`): outstanding best-effort cleanup does not make it
      // unsuccessful, so these two do not wait for the disposal to finish.
      refusedThenSucceeded: f.h.refusedThenSucceeded or
        (t1.outcome == OApplied and f.h.sawRefusal),
      declinedThenSucceeded: f.h.declinedThenSucceeded or
        (t1.outcome == OApplied and f.h.declined),
      blockedStatesSeen: f.h.blockedStatesSeen + (if (settled and isBlock(t1.outcome)) 1 else 0),
      blockedWithoutDiagnosis: f.h.blockedWithoutDiagnosis or
        (settled and isBlock(t1.outcome) and diagnose(w1, t1).size() == 0),
      // `FN-25.b`, THE COMPARISON.  The diagnosis the outcome CARRIES against
      // the one the independent classifier gives the same state.  A call site
      // that names its own diagnosis is a counterexample here rather than a
      // definition — which is what the obligation's prose has always claimed
      // and what, until this ghost existed, it did not check.
      carriedDisagreed: f.h.carriedDisagreed or
        (settled and isBlock(t1.outcome) and
         diagnose(w1, t1) != diagnosisOf(t1.outcome)),
      blockedWithBoth: f.h.blockedWithBoth or diagnose(w1, t1).size() > 1,
      nearMissResolvedToOne: f.h.nearMissResolvedToOne or
        (groveOwnedCorrelated(w1, t1) and unprovable(w1, t1) and diagnose(w1, t1).size() == 1),
    }

  // ---------------------------------------------------------------------------
  // Stepping the transaction
  // ---------------------------------------------------------------------------

  /// Whether the world may act at this program point at all.  See the search
  /// dial's note: this narrows the SEARCH, never the model — every environment
  /// action stays in the action set, and the base instance admits all of them
  /// at every point.
  def envAdmitted(kind: int): bool =
    envUsed < ENV_BUDGET and ENV_PHASES.contains(phaseOf(t.pc)) and ENV_KINDS.contains(kind)

  action stepTxn: bool = all {
    t.running,
    t.pc != SCommit,
    val f = applyStep({ w: w, t: t, h: hist })
    all {
      w' = f.w,
      t' = f.t,
      hist' = postStep(w, f),
      nextAttempt' = nextAttempt,
      envUsed' = envUsed,
    }
  }

  /// The commit.  Its own action because it is the one step whose result the
  /// world decides: whether it landed, what shape it left, and whether the
  /// exact immediate result can be read at all (`EN-09`).
  action commitStep: bool =
    nondet landed = oneOf(Set(true, false))
    nondet partialShape = oneOf(Set(true, false))
    nondet lost = oneOf(Set(true, false))
    nondet said = oneOf(Set(RS_OK, RS_FAILED))
    all {
      t.running,
      t.pc == SCommit,
      if (w.witnessTracked)
        // `FN-13`.  Every candidate committed tree excludes the witness, so a
        // tracked witness stops rather than commits.  The transaction has
        // already published and evacuated, so an effect stands and the stop is a
        // BLOCK.  THE CATALOGUE NOW SAYS SO: its `FN-13` witness read "refused"
        // and was corrected by `closed-set-additions-k74` against §*Outcomes*'
        // own discriminator, with the Alloy column moving to meet this one.
        // What was a finding here is the contract.
        // THE DIAGNOSIS IS CLASSIFIED, NOT NAMED.  This site once wrote
        // `OwnershipConflict` by hand, and it was wrong: the transaction has
        // published a correlated witness and evacuated every entry, so the
        // state is a Grove-owned correlated artifact whose outcome cannot yet
        // be proven — `RecoveryPending`'s own definition.  `inv_FN_25b` now
        // COMPARES the carried diagnosis with the classifier's, so a site that
        // names its own is a counterexample; this is the site that proved the
        // comparison was worth adding.
        val dTracked = pickDiagnosis(w, t)
        all {
          w' = w,
          t' = { ...t, running: false, outcome: OBlocked(dTracked) },
          hist' = { ...hist,
                    stepsVisited: hist.stepsVisited.union(Set(SCommit)),
                    trackedWitnessRefused: true,
                    blockedStatesSeen: hist.blockedStatesSeen + 1,
                    blockSitesSwept: hist.blockSitesSwept.union(Set("tracked-witness")),
                    blockedWithoutDiagnosis: hist.blockedWithoutDiagnosis or
                      diagnose(w, t).size() == 0,
                    blockedWithBoth: hist.blockedWithBoth or diagnose(w, t).size() > 1,
                    carriedDisagreed: hist.carriedDisagreed or
                      diagnose(w, t) != Set(dTracked),
                    recoveryPendingOnLane: if (dTracked == RecoveryPending)
                      hist.recoveryPendingOnLane.union(Set(w.repo.lane))
                      else hist.recoveryPendingOnLane,
                    ownershipConflictOnLane: if (dTracked == OwnershipConflict)
                      hist.ownershipConflictOnLane.union(Set(w.repo.lane))
                      else hist.ownershipConflictOnLane },
          nextAttempt' = nextAttempt,
          envUsed' = envUsed,
        }
      else
        val shape1 = if (not(landed)) CSAbsent
                     else if (partialShape and hasPartialCommitHazard(w.repo.lane)) CSSuccessor
                     else CSExact
        val repo1: Repo = { ...w.repo,
          shape: shape1,
          tickets: if (landed) w.repo.tickets.union(Set(ticketFor(t.attempt))) else w.repo.tickets,
          topology: if (landed) { ...w.repo.topology, head: w.repo.topology.head + 1,
                                  change: w.repo.topology.change + 1 }
                    else w.repo.topology,
          observable: if (ATOMIC_COMMIT) true else not(lost),
          reported: if (lost) RS_LOST else said,
          // `FN-30`.  No user-supplied hook runs during an internal commit,
          // because such a hook may mutate unrelated working-tree bytes that no
          // index image restores.  `HOOKS_SUPPRESSED` is what makes that a
          // claim rather than a coding habit.
          hooksRan: (not(HOOKS_SUPPRESSED)) and w.hookInstalled,
          // `FN-14`.  The commit is SCOPED: unrelated working-copy work
          // survives it.  `COMMIT_SCOPED` is the same move at `FN-14`.
          unrelatedMutated: w.repo.unrelatedMutated or
            ((not(COMMIT_SCOPED)) and landed and w.repo.unrelatedDirty),
          indexRestored: (not(landed)) and hasIndexImage(w.repo.lane),
        }
        val d = disposition(repo1, t.settling)
        val staleOnly = repo1.tickets.exists(tk => tk.handle == HANDLE and tk.attempt != t.settling)
        all {
          w' = { ...w, repo: repo1 },
          t' = { ...t, disp: d,
                 pc: if (d == DCommitted) SRevalBeforeQuarantine else SRevalBeforeRestore },
          hist' = { ...hist,
            stepsVisited: hist.stepsVisited.union(Set(SCommit)),
            evacuationCompleteBeforeCommit: hist.evacuationCompleteBeforeCommit or evacuationComplete(w),
            dispositionsSeen: hist.dispositionsSeen.union(Set(d)),
            committedOnLane: if (d == DCommitted) hist.committedOnLane.union(Set(w.repo.lane)) else hist.committedOnLane,
            notCommittedOnLane: if (d == DNotCommitted) hist.notCommittedOnLane.union(Set(w.repo.lane)) else hist.notCommittedOnLane,
            indeterminateOnLane: if (d == DIndeterminate) hist.indeterminateOnLane.union(Set(w.repo.lane)) else hist.indeterminateOnLane,
            // `FN-15.a`.  A lost or late result REPORTED AS FAILURE while the
            // exact commit exists — classified `Committed`, because the
            // classification never reads an exit status as a receipt.
            lostResultClassifiedCommitted: hist.lostResultClassifiedCommitted or
              (repo1.reported != RS_OK and d == DCommitted),
            classifiedFromStatus: hist.classifiedFromStatus or
              d != evidenceDisposition(repo1, t.settling),
            committedWithoutTicket: hist.committedWithoutTicket or
              (d == DCommitted and not(ATOMIC_COMMIT) and
               not(repo1.tickets.contains(ticketFor(t.settling)))),
            committedWithoutExactResult: hist.committedWithoutExactResult or
              (d == DCommitted and not(ATOMIC_COMMIT) and not(exactResult(repo1, t.settling))),
            // `FN-04`.  A ticket from an EARLIER attempt does not settle this
            // one.
            earlierTicketRejected: hist.earlierTicketRejected or
              (staleOnly and not(exactResult(repo1, t.settling))),
            hookSuppressed: hist.hookSuppressed or w.hookInstalled,
            hooksRanOnInternalCommit: hist.hooksRanOnInternalCommit or repo1.hooksRan,
            // `FN-14`.  Unrelated working-copy work survives a scoped commit.
            unrelatedSurvivedSuccess: hist.unrelatedSurvivedSuccess or
              (landed and w.repo.unrelatedDirty and not(repo1.unrelatedMutated)),
            witnessCommitted: hist.witnessCommitted or (landed and w.witnessTracked) },
          nextAttempt' = nextAttempt,
          envUsed' = envUsed,
        }
    }

  /// `EN-08`.  Interruption may occur between any two steps.  A crash hands the
  /// next invocation a TREE, never a program counter — which is why `t'` is
  /// reset to idle and `resumePoint` reads the world.
  action crashNow: bool = all {
    CRASHES,
    envAdmitted(0),
    t.running,
    w' = w,
    // A CRASH IS NOT AN OUTCOME.  The invocation never returned, so it reports
    // nothing; the block is what the NEXT invocation produces when it meets the
    // state, and that is where `FN-25` is checked.  `settling` survives because
    // the correlation is on disk, in the manifest, not in the dead process.
    t' = { ...IDLE_TXN, settling: t.settling, outcome: ONone },
    hist' = { ...hist,
              crashPoints: hist.crashPoints.union(Set(t.pc)),
              crashClassCounts: hist.crashClassCounts.union(Set(stableStates(w).size())),
              replaceInterruptedAt:
                if (t.pc == SReplaceStage or t.pc == SReplaceExchange or t.pc == SReplaceCleanup)
                  hist.replaceInterruptedAt.union(Set(t.pc))
                else hist.replaceInterruptedAt,
              blockedStatesSeen: hist.blockedStatesSeen + 1,
              recoveryPendingOnLane: hist.recoveryPendingOnLane.union(Set(w.repo.lane)) },
    nextAttempt' = nextAttempt,
    envUsed' = envUsed,
  }

  // ---------------------------------------------------------------------------
  // Recovery
  // ---------------------------------------------------------------------------

  /// `FN-23`.  Re-running recovery for the same handle and attempt reaches the
  /// same terminal state and makes no further change once it has.  "At rest" is
  /// derived from the world, not remembered.
  pure def atRest(w: World): bool =
    val rp = resumePoint(w)
    rp == SIdle or rp == SSettled

  action recoverOp: bool = all {
    not(t.running),
    val rp = resumePoint(w)
    // WITH NO LOCAL TRACE there is no manifest and no quarantine to read the
    // attempt out of, and the identity does NOT therefore vanish: it is what
    // this invocation is settling, carried across the crash exactly as
    // `crashNow` carries it.  Falling back to `-1` here — "any ticket for the
    // handle will do" — is what let an alien ticket settle this attempt
    // forward.
    val settling = if (w.mani.present) w.mani.attempt
                   else if (w.quar.present) w.quar.attempt
                   else t.settling
    val repaired = if (RECOVERY_IDEMPOTENT) w else { ...w, witness: WPreparing, rootPresent: true }
    val proven = deletionProvenFor(w, settling)
    val rootlessTxn = { ...IDLE_TXN, settling: settling }
    val dRootless = pickDiagnosis(w, rootlessTxn)
    if (rp == SSettled)
      // `FN-03`.  A retry with NO LOCAL TRACE of the attempt, settling forward
      // on the correlation ticket alone.
      all {
        w' = repaired,
        t' = { ...IDLE_TXN, entered: true, settling: settling,
               outcome: if (proven) OApplied else OBlocked(dRootless) },
        hist' = { ...hist,
                  settledForwardOnTicketAlone: hist.settledForwardOnTicketAlone or
                    (proven and not(w.quar.present) and w.witness == WNone),
                  blockSitesSwept: if (proven) hist.blockSitesSwept
                    else hist.blockSitesSwept.union(Set("rootless")),
                  blockedStatesSeen: if (proven) hist.blockedStatesSeen
                    else hist.blockedStatesSeen + 1,
                  blockedWithoutDiagnosis: hist.blockedWithoutDiagnosis or
                    (not(proven) and diagnose(w, rootlessTxn).size() == 0),
                  carriedDisagreed: hist.carriedDisagreed or
                    (not(proven) and
                     diagnose(w, rootlessTxn) != Set(dRootless)),
                  noopRecoveries: hist.noopRecoveries + 1,
                  recoveryChangedAtRest: hist.recoveryChangedAtRest or sigOf(repaired) != sigOf(w) },
        nextAttempt' = nextAttempt,
        envUsed' = envUsed,
      }
    else if (rp == SIdle)
      all {
        w' = repaired,
        t' = { ...IDLE_TXN,
               outcome: if (w.witness == WTorn or w.rootTorn) OBlocked(OwnershipConflict)
                        else ORefused(RNotLive) },
        hist' = { ...hist, noopRecoveries: hist.noopRecoveries + 1,
                  recoveryChangedAtRest: hist.recoveryChangedAtRest or sigOf(repaired) != sigOf(w) },
        nextAttempt' = nextAttempt,
        envUsed' = envUsed,
      }
    else
      all {
        w' = w,
        t' = { ...IDLE_TXN,
               running: true, pc: rp,
               attempt: nextAttempt, recovering: true, settling: settling,
               // An UNPUBLISHED witness means no commit was ever attempted —
               // `FN-11` puts a ready manifest and a complete evacuation in
               // front of every commit — so the disposition a discard starts
               // from is `NotCommitted`, not the unknown a published witness
               // starts from.
               disp: if (rp == SRemoveWitness) DNotCommitted else DIndeterminate,
               pinnedIdentity: w.rootIdentity,
               fingerprint: w.mani.fingerprint,
               before: sigOf(w), entered: true },
        hist' = { ...hist,
                  notReadyRecovered: hist.notReadyRecovered or
                    (w.witness == WPublished and w.mani.present and not(w.mani.ready)),
                  noopRecoveries: 0 },
        nextAttempt' = nextAttempt + 1,
        envUsed' = envUsed,
      }
  }

  // ---------------------------------------------------------------------------
  // The quarantine reaper (`FN-21`)
  // ---------------------------------------------------------------------------

  action reapOp: bool = all {
    not(t.running),
    // The reaper touches ONLY entries carrying Grove's own cleanup manifest,
    // and only when no matching in-tree witness owns them.  A foreign entry at
    // a reserved name is DECLINED, mutating nothing, and the sweep continues
    // over entries the reaper CAN prove are Grove's.
    val ownedByLiveWitness =
      w.rootPresent and w.witness == WPublished and w.mani.present and
      w.mani.attempt == w.quar.attempt
    // `EN-13` relaxed sweeps the reserved namespace: it stops asking whether
    // the entry carries Grove's own cleanup manifest, and stops asking whether
    // a live in-tree witness still owns it.  Both halves of `FN-21.b` die with
    // it, which is what the control is for.
    val maySweep = w.quar.present and
                   (w.quar.owned or REAPER_SWEEPS_RESERVED) and
                   (not(ownedByLiveWitness) or REAPER_SWEEPS_RESERVED)
    val sweepsForeign = REAPER_SWEEPS_RESERVED and w.foreignAtQuarantineName
    all {
      w' = { ...w,
             quar: if (maySweep) NO_QUARANTINE else w.quar,
             entries: if (maySweep)
                        entryIds(w).mapBy(e => if (w.quar.entries.contains(e))
                                                 { ...w.entries.get(e), place: Disposed }
                                               else w.entries.get(e))
                      else w.entries,
             foreignAtQuarantineName: if (sweepsForeign) false else w.foreignAtQuarantineName },
      t' = { ...IDLE_TXN, outcome: if (maySweep) OApplied else ORefused(RNotLive) },
      hist' = { ...hist,
                reaperSwept: hist.reaperSwept or maySweep,
                reaperSweptOwned: hist.reaperSweptOwned or (maySweep and ownedByLiveWitness),
                reaperDeclinedOwned: hist.reaperDeclinedOwned or (w.quar.present and ownedByLiveWitness),
                reaperDeclinedForeign: hist.reaperDeclinedForeign or
                  (w.foreignAtQuarantineName and not(sweepsForeign) and maySweep),
                reaperTouchedForeign: hist.reaperTouchedForeign or sweepsForeign,
                foreignMutated: hist.foreignMutated or sweepsForeign },
      nextAttempt' = nextAttempt,
      envUsed' = envUsed,
    }
  }

  // ---------------------------------------------------------------------------
  // An ordinary tree operation, and `FN-20`'s two-world instrument
  // ---------------------------------------------------------------------------

  action ordinaryOp: bool = all {
    not(t.running),
    // `FN-20`.  No classification reads the quarantine — or any other artifact
    // the transaction owns — as evidence that a finish happened.  Checked by
    // asking the classifier the SAME question twice, once with the leftover
    // artifact and once without it.
    val withArtifact = freshTreeVerdict(w)
    val withoutArtifact = freshTreeVerdict(withoutLeftovers(w))
    all {
      w' = w,
      t' = { ...IDLE_TXN,
             outcome: if (w.foreignAtWitnessName) ORefused(RReservedNameOccupied)
                      else if (w.witness != WNone) ORefused(RWitnessPending)
                      else if (not(w.rootPresent)) ORefused(RRootAbsent)
                      else ORefused(RNotLive) },
      hist' = { ...hist,
                artifactReadAsReceipt: hist.artifactReadAsReceipt or withArtifact != withoutArtifact,
                // `FN-20`'s witness: a leftover artifact PRESENT while the tree
                // is classified fresh.
                leftoverWhileFresh: hist.leftoverWhileFresh or
                  (w.quar.present and not(w.rootPresent) and withArtifact == RAbsent),
                ordinaryRefusedReserved: hist.ordinaryRefusedReserved or w.foreignAtWitnessName },
      nextAttempt' = nextAttempt,
      envUsed' = envUsed,
    }
  }

  // ---------------------------------------------------------------------------
  // The world's own actions
  //
  // ABSTRACTION, stated plainly.  Each of these is one enumerated perturbation
  // rather than a composition of smaller ones, for the reason
  // `crates/grove-task-tree/models/task-tree.qnt` gives for its hand-edit
  // family: the situations the claims are about sit several specific edits
  // deep, and simulation reaches them at a rate that makes the suite flaky
  // rather than slow.  The enumeration is a SEARCH STRATEGY over exactly the
  // space the environment assumptions already grant, not a second assumption.
  // ---------------------------------------------------------------------------

  /// `hand-edit`.  The shapes a preflight precondition, a handoff or a
  /// revalidation is stated over.  Reachable at any time, which is what makes
  /// `FN-06`'s mid-transaction swap and `FN-03`'s late gate failures reachable
  /// at all.
  action handEditTo(k: int): bool = all {
    envAdmitted(1),
    w' = (
      // The guard fails BECAUSE ordinary work is live, which is also the state
      // `Current(Live)` names.  Two fields rather than one so the refusal and
      // the classification are separately checkable.
      if (k == 0)      { ...w, guardsOk: false, liveWork: true }
      else if (k == 1) { ...w, leaseLayoutOk: false }
      // `FN-08`'s witness: a layout that PASSES at lease acquisition and fails
      // at the transaction's own rename operands.
      else if (k == 2) { ...w, operandSameDevice: false, leaseLayoutOk: true }
      else if (k == 3) { ...w, rootIdentityVerifiable: false }
      else if (k == 4) { ...w, repo: { ...w.repo, tracked: Set() } }
      else if (k == 5) { ...w, entries: w.entries.set(1, { ...w.entries.get(1), etype: ETOpaque }) }
      // `FN-06`'s witness: a swap BETWEEN TWO STEPS.
      else if (k == 6) { ...w, rootIdentity: w.rootIdentity + 1 }
      else if (k == 7) { ...w, preflightCommitReproducible: false }
      else if (k == 8) { ...w, returnCanComplete: false }
      else if (k == 9) { ...w, hookInstalled: true }
      else if (k == 10) { ...w, repo: { ...w.repo, unrelatedDirty: true } }
      else if (k == 11) { ...w, witnessTracked: true }
      // An ORPHANED, Grove-owned quarantine from an earlier attempt, sitting
      // beside a task root that is still present.  `FN-21.b`'s situation, and
      // `EN-11` is what grants that a well-formed state is reachable by hand
      // edit rather than only by the sequence that would have produced it.
      else             { ...w, quar: { present: true, handle: HANDLE, attempt: 1,
                                       owned: true, marker: MNone, entries: Set() } }
    ),
    t' = t, hist' = hist, nextAttempt' = nextAttempt, envUsed' = envUsed + 1,
  }

  action handEdit: bool =
    nondet k = oneOf(ENV_EDITS)
    handEditTo(k)

  /// `foreign-write`.  `EN-13`: foreign entries may appear at any name and are
  /// not Grove's to delete.
  action foreignWriteAt(k: int): bool = all {
    FOREIGN_WRITES,
    envAdmitted(2),
    w' = (
      if (k == 0)      { ...w, foreignAtWitnessName: true }
      else if (k == 1) { ...w, foreignAtQuarantineName: true }
      else             { ...w, quar: { ...w.quar, marker: MForeign } }
    ),
    t' = t, hist' = hist, nextAttempt' = nextAttempt, envUsed' = envUsed + 1,
  }

  action foreignWrite: bool =
    nondet k = oneOf(ENV_FOREIGN)
    foreignWriteAt(k)

  /// `topology-change`.  `EN-09`: a command's exit status is not a receipt, and
  /// a result may be lost or arrive late.  These are what make a disposition
  /// change BETWEEN two revalidation points, which is the whole reason `FN-22`
  /// has four of them.
  action topologyChangeAt(k: int): bool = all {
    envAdmitted(3),
    w' = (
      // The result arrives LATE: the commit did land after all.
      if (k == 0)
        { ...w, repo: { ...w.repo,
            tickets: w.repo.tickets.union(Set(ticketFor(t.settling))),
            shape: CSExact, observable: true,
            topology: { ...w.repo.topology, head: w.repo.topology.head + 1,
                        change: w.repo.topology.change + 1 } } }
      // The OPERATOR drops the result.  Grove never rewrites history to clear a
      // block (`FN-26`); the operator may, and revalidation is what sees it.
      else if (k == 1)
        { ...w, repo: { ...w.repo,
            tickets: w.repo.tickets.exclude(Set(ticketFor(t.settling))),
            shape: CSAbsent, observable: true, topology: w.repo.recorded } }
      // The exact immediate result becomes unreadable.
      else if (k == 2)
        { ...w, repo: { ...w.repo, observable: ATOMIC_COMMIT } }
      // A ticket naming this handle from SOMEBODY ELSE'S attempt.
      else if (k == 3)
        { ...w, repo: { ...w.repo,
            tickets: w.repo.tickets.union(Set({ handle: HANDLE, attempt: ALIEN_ATTEMPT })) } }
      // The topology moves under Grove, matching neither the recorded anchor
      // nor the expected result.
      else if (k == 4)
        { ...w, repo: { ...w.repo,
            topology: { ...w.repo.topology, head: w.repo.topology.head + 7 } } }
      // The operator drops the result AND moves the topology — a rebase over
      // the attempt's own commit.  This is the ONLY way `Indeterminate` is
      // reachable AFTER a proven commit: obscuring the result decides nothing
      // once the exact ticket exists, and dropping it alone restores the
      // anchor.  Found by `FN-22.g`'s witness failing to land.
      else
        { ...w, repo: { ...w.repo,
            tickets: w.repo.tickets.exclude(Set(ticketFor(t.settling))),
            shape: CSAbsent, observable: true,
            topology: { ...w.repo.topology, head: w.repo.topology.head + 7 } } }
    ),
    t' = t, hist' = hist, nextAttempt' = nextAttempt, envUsed' = envUsed + 1,
  }

  action topologyChange: bool =
    nondet k = oneOf(ENV_TOPOS)
    topologyChangeAt(k)

  // ---------------------------------------------------------------------------
  // The action menu
  // ---------------------------------------------------------------------------

  action step: bool = any {
    confirmOp,
    beginFinish,
    stepTxn,
    commitStep,
    crashNow,
    recoverOp,
    reapOp,
    ordinaryOp,
    handEdit,
    foreignWrite,
    topologyChange,
  }

  // ===========================================================================
  // The claims.
  //
  // Every `inv_` below is a PROPERTY command and lives here, in the library, so
  // that `base` checks all of them unfocused and `verify_small` inherits
  // exactly them.  Every `wit_` is a REACHABILITY command and lives in
  // `finish-controls.qnt`'s `scenario_` instances, because a twenty-step
  // transaction is not reached by unfocused simulation at any affordable sample
  // count — see the search dial's note.
  // ===========================================================================

  /// A FINISH invocation that has returned.  `t.entered` is what separates one
  /// from the reaper's own sweep and from an ordinary tree operation, both of
  /// which return outcomes of their own and neither of which is a finish.
  val settledNow: bool = not(t.running) and t.outcome != ONone and t.entered

  // --- `FN-01` ---------------------------------------------------------------

  /// No step of the transaction runs without an operator confirmation, and the
  /// transaction makes no claim to have verified one.
  val inv_FN_01a_no_step_runs_without_confirmation: bool =
    (hist.stepsVisited.exclude(Set(SIdle)).size() > 0 implies w.confirmed) and
    not(hist.confirmationClaimed)

  /// The deterministic guards are not a substitute for confirmation: the guard
  /// refusal is only ever REACHED once confirmation has been given, so it can
  /// never be standing in for it.
  val inv_FN_01b_guards_are_not_a_substitute: bool =
    (t.precondition == PCGuards) implies w.confirmed

  // --- `FN-02` ---------------------------------------------------------------

  /// Declining, or exiting without completing, leaves the finish leaf live and
  /// selectable, and writes nothing else.
  val inv_FN_02_intent_persists_as_the_finish_leaf: bool =
    (t.outcome == ONotEntered implies
      (w.finishLeafLive and w.witness == WNone and not(w.mani.present))) and
    (isRefusal(t.outcome) implies w.finishLeafLive)

  // --- `FN-03` ---------------------------------------------------------------

  /// The proof that an attempt completed is the deletion commit's own message.
  /// A success is never reported without one.
  val inv_FN_03_the_ticket_is_the_durable_record: bool =
    (settledNow and t.outcome == OApplied) implies deletionProvenFor(w, t.settling)

  // --- `FN-04` ---------------------------------------------------------------

  /// A commit is accepted as THIS attempt's result only when its attempt
  /// identity matches; a ticket from an earlier attempt does not settle a later
  /// one.
  val inv_FN_04_an_attempt_binds_to_a_live_session: bool =
    not(hist.committedWithoutTicket)

  // --- `FN-05` ---------------------------------------------------------------

  /// The preflight precondition set is CLOSED and exactly the seven.
  val inv_FN_05a_the_precondition_set_is_closed: bool =
    hist.preconditionsHit.subseteq(ALL_PRECONDITIONS) and
    (t.precondition != PCNone implies ALL_PRECONDITIONS.contains(t.precondition))

  /// Each member, failing, leaves the TREE byte-identical.
  val inv_FN_05b_a_failing_precondition_leaves_the_tree: bool =
    not(hist.preflightMovedTree)

  /// Each member, failing, leaves the REPOSITORY byte-identical.
  val inv_FN_05c_a_failing_precondition_leaves_the_repository: bool =
    not(hist.preflightMovedRepo)

  // --- `FN-06` ---------------------------------------------------------------

  /// The task root's identity is pinned and rechecked at every later step, so a
  /// mid-transaction swap never becomes a mutation applied elsewhere.
  val inv_FN_06_the_root_identity_is_pinned_and_rechecked: bool =
    not(hist.mutatedAfterSwap) and not(hist.startedWithUnverifiableIdentity)

  // --- `FN-07` ---------------------------------------------------------------

  /// An empty deletion fingerprint is refused BEFORE any mutation.
  val inv_FN_07_an_untracked_tree_is_refused_before_evacuation: bool =
    not(hist.startedWithEmptyFingerprint) and
    (w.mani.present implies w.mani.fingerprint.size() > 0)

  // --- `FN-08` ---------------------------------------------------------------

  /// The same-device requirement is checked against the transaction's OWN
  /// rename operands, and is never satisfied by the earlier lifecycle check.
  val inv_FN_08_the_operand_gate_is_never_satisfied_by_the_lease: bool =
    not(hist.startedWithBadOperand)

  // --- `FN-09` ---------------------------------------------------------------

  /// Publication is exactly one atomic same-directory rename, and no reader
  /// observes it half-applied.
  val inv_FN_09a_publication_is_one_atomic_rename: bool =
    persistentEffect(SPublish) == ESameDirRename and w.witness != WTorn

  /// No PREPARING witness ever holds an evacuated entry.
  val inv_FN_09b_no_preparing_witness_holds_an_evacuee: bool =
    not(hist.preparingHeldEvacuee)

  // --- `FN-10` --------------------------------------------------------------

  /// Interruption before publication is recovered by DISCARDING the witness,
  /// never by interpreting its contents: the discard path reads no field of the
  /// manifest but its handle.
  val inv_FN_10a_an_unpublished_witness_is_discarded: bool =
    (hist.unpublishedDiscarded implies not(hist.restoredAfterProof))

  /// Content the discard cannot classify as Grove's own fails CLOSED.
  val inv_FN_10b_unclassifiable_content_fails_closed: bool =
    not(hist.mutatedUnproven)

  // --- `FN-11` --------------------------------------------------------------

  /// Every ordinary root entry is inside the published witness, beneath a
  /// written and verified manifest, before any commit is attempted.
  val inv_FN_11_evacuation_precedes_the_commit: bool =
    (t.running and t.pc == SCommit) implies evacuationComplete(w)

  // --- `FN-12` --------------------------------------------------------------

  /// The manifest records the finish handle, the attempt identity, the
  /// repository anchor, the deletion fingerprint and every evacuated entry's
  /// digest, and is marked ready LAST.
  val inv_FN_12a_the_manifest_is_complete_and_ready_last: bool =
    (w.mani.present and w.mani.ready) implies
      (w.mani.handle == HANDLE and w.mani.attempt >= 0 and
       w.mani.anchor >= 0 and w.mani.fingerprint.size() > 0 and
       w.mani.digests.keys() == entryIds(w))

  /// An entry type that cannot be digested is refused BEFORE any mutation.
  val inv_FN_12b_an_undigestible_entry_is_refused_before_mutation: bool =
    not(hist.startedWithUndigestible)

  // --- `FN-13` --------------------------------------------------------------

  /// Every candidate committed tree excludes the witness.
  val inv_FN_13_the_witness_is_never_committed: bool =
    not(hist.witnessCommitted)

  // --- `FN-14` --------------------------------------------------------------

  /// The commit records exactly the expected deletions and no unrelated change;
  /// unrelated working-copy work survives.
  val inv_FN_14_the_commit_is_scoped: bool =
    not(w.repo.unrelatedMutated) and not(hist.unrelatedMutated)

  // --- `FN-15` --------------------------------------------------------------

  /// The classification never reads an exit status as a receipt.
  val inv_FN_15a_the_classification_never_reads_an_exit_status: bool =
    not(hist.classifiedFromStatus)

  /// `Committed` is what the evidence says, not what the machine remembered.
  val inv_FN_15b_committed_agrees_with_the_evidence: bool =
    not(hist.committedWithoutExactResult)

  /// `NotCommitted` agrees with the evidence too — and it is the ONLY
  /// disposition that licenses a rollback.
  val inv_FN_15c_notcommitted_agrees_with_the_evidence: bool =
    not(hist.restoredWithoutLicence)

  /// `Indeterminate` never settles anything: it performs no handoff and yields
  /// exactly one block.
  val inv_FN_15d_indeterminate_settles_nothing: bool =
    (settledNow and t.disp == DIndeterminate and REVALIDATE)
      implies (isBlock(t.outcome) or t.outcome == ORefused(RNotLive))

  // --- `FN-16` --------------------------------------------------------------

  /// Restoration is refused when the recorded anchor no longer holds.
  val inv_FN_16a_restoration_needs_the_anchor: bool =
    not(hist.restoredWithoutLicence)

  /// Restoration is refused when the attempt-bound result is present.
  val inv_FN_16b_restoration_needs_the_result_absent: bool =
    not(hist.restoredAfterProof) and not(hist.restoredWithoutLicence)

  // --- `FN-17` --------------------------------------------------------------

  /// After restoration the tree matches the manifest, and on a
  /// working-copy-as-commit lane the exact recorded preflight commit is
  /// reproduced before the witness is removed.
  val inv_FN_17a_rollback_is_exact: bool =
    (t.running and t.pc == SRemoveWitness and w.witness == WPublished) implies
      (manifestMatches(w) and
       (workingCopyIsACommit(w.repo.lane) implies w.repo.preflightCommitReproduced))

  /// A restoration that cannot reproduce it BLOCKS rather than proceeds.
  val inv_FN_17b_an_inexact_restoration_blocks: bool =
    not(hist.reproducedWhileImpossible)

  // --- `FN-18` --------------------------------------------------------------

  /// Once the exact commit is proven, the tree is never reconstructed.
  val inv_FN_18_forward_recovery_never_restores: bool =
    not(hist.restoredAfterProof)

  // --- `FN-19` --------------------------------------------------------------

  /// A proven commit is settled by renaming the WHOLE task root in one step.
  /// No partial or empty task root is ever observable.
  val inv_FN_19_the_root_moves_in_one_atomic_rename: bool =
    persistentEffect(SQuarantineRename) == ESameDirRename and
    // No PARTIAL or EMPTY task root is ever observable.  Stated over the torn
    // flag rather than over the quarantine's contents: an orphaned quarantine
    // a hand edit installed is not something Grove's rename produced, and
    // reading it onto this claim would make `FN-19` a statement about what the
    // world may leave lying about.
    not(w.rootTorn) and
    // …and the quarantine Grove's own rename produced holds the WHOLE root:
    // every entry, and the witness with them.
    ((w.quar.present and w.quar.owned and w.quar.attempt == t.settling and
      t.entered and not(w.rootPresent))
       implies w.quar.entries == entryIds(w))

  // --- `FN-20` --------------------------------------------------------------

  /// No classification reads the quarantine — or any other artifact the
  /// transaction owns — as evidence that a finish happened.  Checked as a
  /// TWO-WORLD comparison, not as a promise about the classifier's source.
  val inv_FN_20_a_leftover_artifact_is_never_a_receipt: bool =
    not(hist.artifactReadAsReceipt) and
    freshTreeVerdict(w) == freshTreeVerdict(withoutLeftovers(w))

  // --- `FN-21` --------------------------------------------------------------

  /// Disposal is re-enterable from any interruption: what a resumed disposal
  /// rejoins is decided by the marker it finds, never by a program counter a
  /// crash did not preserve.
  val inv_FN_21a_disposal_is_reenterable: bool =
    (w.quar.present and w.quar.owned) implies
      (markerValue(w.quar.marker) <= disposedCount(w))

  /// A reaper touches only entries carrying Grove's own cleanup manifest, and
  /// only when no matching in-tree witness owns them.
  val inv_FN_21b_the_reaper_touches_only_groves_own: bool =
    not(hist.reaperTouchedForeign) and not(hist.reaperSweptOwned)

  /// A reaper declines a foreign entry at a reserved name, mutating nothing.
  val inv_FN_21c_the_reaper_declines_a_foreign_entry: bool =
    not(hist.foreignMutated)

  // --- `FN-22` --------------------------------------------------------------

  /// All four revalidation points are performed, and none is skipped: no
  /// handoff happens without the point before it having been taken.
  val inv_FN_22a_no_handoff_skips_its_revalidation: bool =
    ((t.running and t.pc == SRestore) implies hist.revalPerformed.contains(SRevalBeforeRestore)) and
    ((t.running and t.pc == SQuarantineRename) implies hist.revalPerformed.contains(SRevalBeforeQuarantine)) and
    ((t.running and t.pc == SRemoveWitness and w.witness == WPublished)
       implies hist.revalPerformed.contains(SRevalAfterRestore)) and
    ((t.running and t.pc == SCreateMarker) implies hist.revalPerformed.contains(SRevalAfterQuarantine))

  /// Before restoration, `Committed` diverts to the forward path and restores
  /// nothing.
  val inv_FN_22b_committed_before_restoration_diverts: bool =
    not(hist.restoredWithoutLicence)

  /// After restoration, `Committed` leaves the witness blocking the restored
  /// tree, `Blocked(RecoveryPending)`.
  val inv_FN_22c_committed_after_restoration_blocks: bool =
    (hist.blockedAfterRestore implies (w.witness == WPublished or w.quar.present or w.rootTorn))

  /// After restoration, an unchanged `NotCommitted` settles to a
  /// manifest-matching root with the witness removed and the finish leaf live.
  val inv_FN_22d_notcommitted_after_restoration_settles: bool =
    (settledNow and t.outcome == ORefused(RDeletionNotCommitted)) implies
      (w.witness == WNone and w.rootPresent and w.finishLeafLive)

  /// Before the quarantine rename, `NotCommitted` diverts to the restoration
  /// path and renames nothing.
  val inv_FN_22e_notcommitted_before_the_rename_diverts: bool =
    not(hist.renamedWithoutCommitted)

  /// After the rename, `Committed -> NotCommitted` returns the quarantine
  /// atomically, and a successful return settles to the exact pre-rename state.
  val inv_FN_22f_a_successful_return_is_exact: bool =
    (hist.returnedRefused implies (w.rootPresent and not(w.quar.present)))

  /// After the rename, `Committed -> Indeterminate` returns the quarantine and
  /// blocks.
  val inv_FN_22g_indeterminate_after_the_rename_returns_and_blocks: bool =
    (hist.returnedBlocked implies not(w.quar.present))

  /// A return that cannot complete reports BOTH the change and the quarantine,
  /// and blocks.
  val inv_FN_22h_an_incomplete_return_reports_both: bool =
    (hist.returnIncomplete implies (w.quar.present or w.rootTorn or hist.reaperSwept))

  /// After the rename, an unchanged `Committed` settles to an absent task root
  /// with the quarantine holding it.
  val inv_FN_22i_committed_after_the_rename_applies: bool =
    (hist.appliedAfterQuarantine and t.outcome == OApplied and not(ATOMIC_DISPOSAL))
      implies not(w.rootPresent)

  /// `Indeterminate` observed at any point other than after the quarantine
  /// rename performs NO handoff and blocks `RecoveryPending`.
  val inv_FN_22j_indeterminate_elsewhere_performs_no_handoff: bool =
    hist.indeterminateBlockedAt.forall(s => s != SRevalAfterQuarantine)

  // --- `FN-23` --------------------------------------------------------------

  /// Re-running recovery for the same handle and attempt reaches the same
  /// terminal state and makes no further change once it has.
  val inv_FN_23_recovery_is_idempotent: bool =
    not(hist.recoveryChangedAtRest)

  // --- `FN-24` --------------------------------------------------------------

  /// From a crash at each step boundary, the next invocation classifies the
  /// result into EXACTLY ONE stable state — never into a state that is
  /// indistinguishable from a different one.
  val inv_FN_24a_every_interruption_lands_in_exactly_one_state: bool =
    stableStates(w).size() == 1 and
    hist.crashClassCounts.forall(n => n == 1)

  /// Every step of the transaction has at most one persistent effect, and that
  /// effect is a same-directory rename (`EN-01`) or is itself DECLARED.
  val inv_FN_24b_every_step_has_one_declared_effect: bool =
    ALL_STEPS.forall(s =>
      persistentEffect(s) == ENothing or
      persistentEffect(s) == ESameDirRename or
      DECLARED_STEPS.contains(s))

  // --- `FN-25` --------------------------------------------------------------

  /// The two diagnoses are DISJOINT: no blocked state satisfies both.
  val inv_FN_25a_the_diagnoses_are_disjoint: bool =
    diagnose(w, t).size() <= 1 and not(hist.blockedWithBoth)

  /// The two are jointly EXHAUSTIVE over `Blocked`, and — the second half, and
  /// the one this obligation's prose always claimed — the diagnosis the machine
  /// CARRIED at each block site agrees with the one the independent classifier
  /// gives that same state.  `carriedDisagreed` is what makes the second half a
  /// comparison rather than a restatement: without it the invariant asked only
  /// whether the classifier returns one answer, which a machine naming its own
  /// diagnosis at each site satisfies by definition.
  val inv_FN_25b_the_diagnoses_are_exhaustive: bool =
    not(hist.blockedWithoutDiagnosis) and
    not(hist.carriedDisagreed) and
    (isBlock(t.outcome) implies diagnose(w, t).size() == 1)

  /// Each diagnosis is reachable on each lane — stated as the property that a
  /// diagnosis is never recorded against a lane the model is not on.
  val inv_FN_25c_a_diagnosis_is_recorded_against_its_own_lane: bool =
    hist.recoveryPendingOnLane.subseteq(LANES) and
    hist.ownershipConflictOnLane.subseteq(LANES)

  // --- `FN-26` --------------------------------------------------------------

  /// History is never rewritten to clear a block.
  val inv_FN_26_history_is_never_rewritten: bool =
    not(w.repo.historyRewritten) and not(hist.historyRewritten)

  // --- `FN-27` --------------------------------------------------------------

  val inv_FN_27a_nothing_unrelated_changes_on_success: bool =
    not(hist.unrelatedMutatedOnSuccess) and not(hist.foreignMutated)

  val inv_FN_27b_nothing_unrelated_changes_on_refusal: bool =
    not(hist.unrelatedMutatedOnRefusal) and not(hist.foreignMutated)

  val inv_FN_27c_nothing_unrelated_changes_on_a_block: bool =
    not(hist.unrelatedMutatedOnBlock) and not(hist.foreignMutated)

  // --- `FN-28` --------------------------------------------------------------

  /// A finish succeeds exactly when the exact attempt-bound commit is proven
  /// and the task root is absent; branch, bookmark and worktree topology are
  /// unchanged; and outstanding best-effort cleanup does not make a proven
  /// finish unsuccessful.
  val inv_FN_28_one_successful_exit: bool =
    (t.outcome == OApplied and settledNow) implies
      (not(w.rootPresent) and deletionProvenFor(w, t.settling))

  // --- `FN-29` --------------------------------------------------------------

  /// `FN-29.a`.  `NotCommitted` leaves the grove exactly as it was, with the
  /// finish leaf live, and is distinguishable from a block.
  val inv_FN_29a_a_refusal_is_a_complete_outcome: bool =
    (settledNow and t.outcome == ORefused(RDeletionNotCommitted)) implies
      (w.rootPresent and w.witness == WNone and w.finishLeafLive and
       entryIds(w).forall(e => w.entries.get(e).place == AtRoot) and
       not(isBlock(t.outcome)))

  /// `FN-29.b` — THE DISCRIMINATOR, and the obligation that exists because
  /// nothing could check it.  Every `Refused` outcome is returned with the tree
  /// byte-equal to the tree the action received; an effect that stands and can
  /// be neither completed nor undone is a `Blocked`.  Stated over EVERY refusal
  /// rather than over `NotCommitted` alone, which is what `.a` already covers
  /// and what left the rule uncheckable.
  ///
  /// THE OPERAND IS THIS ACTION'S OWN EFFECTS, AND SAYING SO TOOK THREE
  /// COUNTEREXAMPLES.  The obvious phrasing — a refusal implies the world is in
  /// the untouched shape (root present, not torn, no quarantine, every entry at
  /// the root) — is FALSE, and this suite found it on the run that introduced
  /// it: a quarantine standing in the FREE INITIAL STATE, created by no
  /// transaction, survives a refusal at `SCreatePreparing` and is reported as an
  /// incomplete outcome because of bytes that were never this action's.
  /// `finish.als`'s `FN-29.a` note records the identical defect on the identical
  /// artifact, and its own `FN-29.b` hit it a second time on the witness slot.
  /// §*Outcomes* says *the tree it hands back equals the tree it was GIVEN*, and
  /// the operand in that sentence is doing all the work.
  ///
  /// WHAT REMAINS IS THE EVACUATION, OWNED.  It is the effect the discriminator
  /// actually turns on — it is what makes a stop recoverable rather than
  /// forgettable — and `w.mani` carries the ownership bit that separates this
  /// attempt's from an earlier one's.  A foreign artifact at a reserved name is
  /// deliberately outside the claim: it stood before the action began, the
  /// action leaves it byte-identical (`FN-32`), and excluding it would make
  /// `stopReserved`'s refusing arm unreachable by fiat.
  ///
  /// AND IT SEPARATES.  A mutation making `stopReserved` always refuse settles a
  /// refusal at `SRevalBeforeQuarantine`, over this attempt's own completed
  /// evacuation, and is a counterexample.
  ///
  /// THE BLOCK HALF IS CHECKED, AND IT IS CHECKED ELSEWHERE IN THIS COLUMN.
  /// `finish.als` states `FN-29.b` two-sidedly because it has no neighbour that
  /// does; here `inv_FN_22c` already carries *a block leaves a published
  /// witness, a quarantine or a torn root*, so restating it would make each
  /// mutation unable to kill the other — the hazard `FN-32` and `FN-21.c`
  /// record beside `foreignAtReservedName`.
  val inv_FN_29b_a_refusal_leaves_nothing_standing: bool =
    (settledNow and isRefusal(t.outcome)) implies
      not(evacuationComplete(w) and w.mani.handle == HANDLE
          and w.mani.attempt == t.attempt)

  /// The refusal arm of `FN-29.b`: a step meets an artifact it cannot prove is
  /// Grove's own having persisted nothing, and refuses.  THIS ARM IS NEW — the
  /// site blocked before `closed-set-additions-k74`.
  val wit_FN_29b_a_stop_with_nothing_persisted_refuses: bool =
    hist.transactionRefusedReserved

  /// The block arm of the same rule, reached at the same kind of meeting after
  /// an effect has been applied.  The two arms together are the claim; either
  /// alone is a constant.
  val wit_FN_29b_a_stop_after_an_applied_effect_blocks: bool =
    hist.transactionBlockedReserved

  // --- `FN-30` --------------------------------------------------------------

  /// No user-supplied hook runs during an internal commit.
  val inv_FN_30_internal_commits_run_without_hooks: bool =
    not(w.repo.hooksRan) and not(hist.hooksRanOnInternalCommit)

  // --- `FN-31` --------------------------------------------------------------

  /// There is a `replace-cleanup-marker` transition distinct from creating a
  /// marker and from removing one, and it is decided by REACHABILITY rather
  /// than by construction.
  val inv_FN_31a_the_replace_transition_is_distinct: bool =
    persistentEffect(SReplaceExchange) == ESameDirRename and
    persistentEffect(SCreateMarker) == ECreate and
    persistentEffect(SRemoveMarker) == ERemove

  /// The replacement is atomic with respect to readers: no reader observes the
  /// marker absent, nor observes two markers.
  val inv_FN_31b_the_replacement_is_atomic_for_readers: bool =
    w.quar.marker != MAbsentDuringReplace and not(hist.markerGapObserved)

  /// An interruption inside the replacement is resumable, and resumption
  /// reaches the same terminal state as an uninterrupted replacement.
  val inv_FN_31c_an_interrupted_replacement_resumes: bool =
    (t.running and t.pc == SRemoveMarker) implies
      markerValue(w.quar.marker) == disposedCount(w)

  /// A replacement is never performed against a marker Grove cannot prove is
  /// its own.
  val inv_FN_31d_never_against_a_foreign_marker: bool =
    (w.quar.marker == MForeign) implies not(hist.mutatedUnproven)

  // --- `FN-32` --------------------------------------------------------------

  /// `TT-24`'s SECOND CONTEXT, RE-STATED UNDER AN `FN-` PREFIX.  It was
  /// `TT-24.c`, whose context is a finish context and whose crate is therefore
  /// this one (`docs/adr/obligations-follow-context-not-artifact.md`).
  ///
  /// It says NOTHING about which outcome the step produces, and that is the
  /// finding rather than a hedge: `TT-24.c` said `Blocked(OwnershipConflict)`,
  /// this column blocks at `SRemoveWitness`'s discard branch, and the Alloy
  /// column REFUSES at the same step — both green against `FN-10.b`, whose text
  /// says only *fails closed*.  What both columns agree on is that the artifact
  /// does not move.
  ///
  /// THE TRANSACTION-SIDE FLAGS AND NOT THE REAPER'S: `noteUnprovenSlot` and
  /// `noteUnprovenMarker` set `foreignMutated` too, but the reaper's sweep sets
  /// only that one.  Reading `foreignMutated` here would make this obligation
  /// and `FN-21.c` the same invariant, and a mutation aimed at either would
  /// kill both.
  ///
  /// IT NAMES ITS TWO ARTIFACTS RATHER THAN THE AGGREGATE, and that is a
  /// correction `obligation-placement-k68` landed.  `not(hist.mutatedUnproven)`
  /// was character-for-character `inv_FN_10b`, and it additionally covered the
  /// QUARANTINE name, which the catalogue's `FN-32` explicitly excludes — a
  /// quarantine no marker yet authorises is a state the ordinary forward path
  /// passes through, so it carries no ownership bit.  Reading the two
  /// per-artifact flags states exactly the claim, and lets a control kill this
  /// through the slot and through the marker separately.
  val inv_FN_32_a_transaction_never_mutates_what_it_cannot_prove_is_its_own: bool =
    not(hist.mutatedUnprovenSlot) and not(hist.mutatedUnprovenMarker)

  /// A transaction step meeting an artifact at a reserved name Grove cannot
  /// prove is its own, with the artifact still standing.  This one is the SLOT's
  /// — the marker's is `wit_FN_32_a_transaction_step_meets_an_unprovable_marker`
  /// in `finish-controls.qnt`, because a foreign marker needs a narrowed
  /// environment to reach.
  ///
  /// IT READS BOTH ARMS, AND THAT IS THE DISPOSITION OF `closed-set-additions-k74`
  /// SHOWING UP IN A WITNESS.  This predicate was worded on the blocking half
  /// alone, back when this column blocked at the witness slot and the Alloy
  /// column refused.  `FN-32` says only that the artifact STANDS — which both
  /// outcomes satisfy — so a witness that lands only on a block was reading the
  /// contested outcome into a claim that never carried it.  `FN-29.b` owns the
  /// outcome; this one owns the artifact.
  val wit_FN_32_a_transaction_step_meets_an_unprovable_artifact: bool =
    (hist.transactionBlockedReserved or hist.transactionRefusedReserved)
      and not(hist.mutatedUnprovenSlot)
      and not(hist.mutatedUnprovenMarker)
}

// =============================================================================
// Instances
//
// `finish` above carries `const` declarations and is therefore a LIBRARY, not a
// run: every module below binds the assumption dial and is what
// `quint run --main=` is pointed at.
//
// THE MODULE RULE IS DEFINED IN ONE PLACE — `models/run.sh`, *THE MODULE RULE* —
// and cited here rather than restated.  In brief: a `relax_`, `mutant_` or
// `scenario_` instance carries only the commands written inside it; a `verify_`
// instance is model-checked and inherits the library's PROPERTY commands only;
// every other instance inherits all of them.
//
// The controls and the scenarios live in `finish-controls.qnt`, for the reason
// `crates/grove-task-tree/models/task-tree-controls.qnt` gives: `quint verify`
// serialises every module in the file it is handed, and one file holding the
// library, its instances, six assumption mutations, six model mutations and
// five focused scenarios is beyond quint 0.32.0's own result reporter.
// =============================================================================

/// The run every `FN-` obligation's PROPERTY is checked in.  Every assumption
/// granted, every model mutation off, all three lanes, and an environment
/// budget larger than any trace can spend at every program point — so nothing
/// is narrowed here.
module base {
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
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    ENV_BUDGET = 99,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6),
    ENV_KINDS = Set(0, 1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
    ENV_FOREIGN = Set(0, 1, 2),
    ENV_TOPOS = Set(0, 1, 2, 3, 4, 5),
    ENTRIES = 1
  ).*
}

/// The VERIFICATION instance: the same library, reduced so that a symbolic
/// backend can reach it.  One lane, one entry, one environment action.  What it
/// buys over `base` is the direction simulation cannot give — no counterexample
/// EXISTS within the depth, rather than none was sampled.  What it costs is
/// stated in the README's `VERIFY` line.
module verify_small {
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
    LEFTOVERS_ARE_EVIDENCE = false,
    HOOKS_SUPPRESSED = true,
    COMMIT_SCOPED = true,
    HISTORY_IMMUTABLE = true,
    ENV_BUDGET = 1,
    ENV_PHASES = Set(0, 1, 2, 3, 4, 5, 6),
    ENV_KINDS = Set(0, 1, 2, 3),
    ENV_EDITS = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
    ENV_FOREIGN = Set(0, 1, 2),
    ENV_TOPOS = Set(0, 1, 2, 3, 4, 5),
    ENTRIES = 1
  ).*
}
