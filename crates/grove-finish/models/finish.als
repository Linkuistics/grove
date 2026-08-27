/*
 * grove-finish — the finish/recovery claims, in Alloy 6
 * =====================================================
 *
 * The subject is `docs/specs/semantic-contract.md`, section *Claims — finish
 * and recovery*.  Nothing else: no Rust module, no helper, no control-flow
 * shape.  Every command below names an OBLIGATION of that document, and the
 * repository runner reads the obligation list out of the document rather than
 * out of this file.
 *
 * COVERAGE SO FAR: FN-01, FN-05 .. FN-08 — the transaction's ENTRY surface;
 * FN-09 .. FN-13, the RESERVED WITNESS; FN-03, FN-04, FN-14 .. FN-18, the
 * COMMIT AND ITS DISPOSITION; FN-19, FN-20, the QUARANTINE AND ITS ATOMIC ROOT
 * RENAME; FN-22, the FOUR REVALIDATION POINTS AND THE TEN-ROW TABLE; and
 * FN-21, FN-31, DISPOSAL — its re-entrancy, the cleanup marker's create /
 * replace / remove transitions, and the reaper; and FN-24, THE CRASH SLICE —
 * what the disk a crash leaves classifies as, and what one step of the
 * transaction is allowed to change.  Every other `FN-` obligation belongs to a
 * later child of `exits-k46`, and the runner reports its cell empty, which is
 * the truth about this file rather than a defect in it.
 *
 * THE CRASH SLICE ADDS NO TRANSITION AND NO `var` FIELD, AND IT IS THE FIRST
 * SLICE IN THIS SCOPE THAT DOES NOT.  `crash` has been enabled at every step
 * boundary since `witness-k40`; what was never asked until now is what the disk
 * it leaves CLASSIFIES AS, and whether the step it interrupted had one
 * persistent effect or several.  Both are functions of state written as DATA
 * beside `observed` and the ten-row table, so what the slice costs is bound and
 * nothing else — and `README.md` records that the cost came out flat per
 * command rather than proportional, which is a shape the cost law had not met.
 *
 * THE DISPOSAL SLICE IS WHERE THE FORWARD SETTLE STOPS BEING ONE STEP.  Every
 * slice before it disposed the quarantine in the same transition that
 * revalidated: `FN-22.i`'s stable state — task root ABSENT, quarantine holding
 * the root — was passed through and out of in one move.  `FN-21.a`'s *disposal
 * is re-enterable from any interruption* is what turns that one step into
 * three, and `EN-03` is why: there is no atomic recursive directory deletion,
 * so an interruption mid-disposal is a state the protocol has to be able to
 * resume from, and the CLEANUP MARKER is the only thing that survives it to say
 * the removal still has to happen.  Write the marker; remove what it authorises
 * removing; remove the marker.  Nothing but the marker distinguishes a disposal
 * that stopped from a directory that was never Grove's.
 *
 * AND IT IS WHERE THE FILE FIRST RUNS OUTSIDE THE PHASE MACHINE.  `doTxnOpen`
 * requires `some Root.rid`, so no transaction can be opened on the disk an
 * interruption immediately after the rename leaves — the state
 * `witness_FN_19` demonstrates and that nothing in the file could leave.  The
 * catalogue's answer is the REAPER, a sweep rather than a transaction, and it
 * is what resumes an interrupted disposal.  It is deliberately NOT in
 * `bodySteps` or `txnActs`: it is not a step of the finish transaction, it
 * carries no operator confirmation, and `FN-24.b` should not be asked of it.
 *
 * WHY THESE THREE SLICES AND NOT A LAYER.  The entry surface ends in a refusal or
 * in a transaction that is never entered.  The witness slice adds the six steps
 * that build and publish the witness and evacuate the tree into it, plus the two
 * things a step list is for: `crash` between any two steps, and `discard` as the
 * recovery an unpublished witness admits.  The COMMIT slice adds the repository
 * as something that changes: the scoped deletion commit, the correlation ticket
 * that names the handle and the attempt, the three dispositions classified from
 * evidence rather than from exit status, the rollback licence and its exactness,
 * and forward recovery.  It stops before the quarantine: no atomic root rename,
 * no revalidation table, no cleanup marker, no reaper, no `Blocked` DIAGNOSIS.
 * That is what makes it verifiable on its own.  It is also what a green run here
 * does NOT prove — see `README.md`, which says so in more detail than this
 * header can.
 *
 * THE REVALIDATION SLICE IS WHERE THE FILE STOPS TRUSTING A RECORDED
 * DISPOSITION.  Everything before it acts on what the classification wrote;
 * `FN-22` says the disposition is rechecked immediately before and after each of
 * the two filesystem handoffs, and that every observation at every point has a
 * stated corrective action and a stated stable state.  Three consequences run
 * through this file rather than sitting in one section: the restoration is split
 * so that there is a state AFTER it to observe; the rename gains an inverse; and
 * the classification is no longer re-runnable at `Classified`, because a step
 * that re-derives a disposition and takes no corrective action is a fifth
 * revalidation point the catalogue does not have.
 *
 * `EN-05` IS THE SHAPE OF THE THIRD SLICE.  No filesystem transaction can include
 * a version-control commit, so the commit sits outside the body and the interval
 * between the evacuation and the recorded result cannot be closed.  Everything
 * from `doCommitAttempt` to `doSettle` is about what can be KNOWN across that
 * interval and what may be done on the strength of it.
 *
 * THE STEP LIST IS A NAMED THING (`bodySteps`), not an accretion of predicates.
 * `EN-08` grants interruption between any two steps and says nothing about what
 * a step is, so the list is what the grant is worth — and `FN-24.b` will
 * quantify over exactly this set when the `exits` sibling reaches it.
 *
 * HOW TO READ IT — the house style of `crates/grove-task-tree/models/` and
 * `docs/ordinal-fs-tree/models/`:
 *
 *   Nothing the catalogue merely CLAIMS is a `fact`.  Claims are named
 *   predicates, and every command says which ones it assumes.  Facts hold only
 *   what is true by construction — a verdict recorded by a gate that passed
 *   cannot have been recorded by one that did not.
 *
 *     check FN_nn[x]_<mnemonic>            must find NO counterexample
 *     run   witness_FN_nn[x]_<mnemonic>    must find an instance
 *     run   expect_unreachable_EN_nn_<m>   must find NO instance
 *
 * THE LANE IS A MODEL PARAMETER FROM THE FIRST COMMAND, not three models and
 * not a dimension added later.  Nothing in THIS slice differs by lane — none of
 * the seven preconditions is lane-specific — but `EN-16`'s control is a
 * COLLAPSE to one lane, and a parameter retrofitted in a later slice is a
 * parameter every earlier command was never checked under.  It is also what
 * carries `SY-02`: an absent lane IS an unsupported layout, which is how
 * `FN-05.a`'s third member gets a state to fail in.
 *
 * A GREEN RUN OF THIS FILE IS NOT, BY ITSELF, EVIDENCE.  The sibling scope has
 * already reported itself green — witnesses included — while checking nothing at
 * all, and what separated the fiction from the fact was one mutation per
 * obligation.  The matrix and the bounds caveats are in `README.md` beside this
 * file; read it before trusting a green run, and re-run the mutations after
 * changing a transition.
 *
 * Run it with `models/run.sh --scope finish --family alloy --no-coverage` from
 * the repository root.
 */
module finish


// ===========================================================================
// VOCABULARY
//
// Deliberately coarse.  No `FN-` claim quantifies over names, positions, keys
// or slugs, so this file has no filename grammar: an entry is an opaque object
// with a TYPE (which decides whether it can be digested) and a ROLE (which
// decides whether it is live work).  That is the whole of what the seven
// preconditions read, and rebuilding `task-tree.als`'s grammar here would pay
// for machinery no claim in this scope reads.
// ===========================================================================

// --- the lane, and therefore the layout ------------------------------------
abstract sig Lane {}
one sig GitL, NativeJjL, ColocatedJjL extends Lane {}

// --- the environment's opaque identities -----------------------------------
sig Device {}     // a filesystem boundary; `EN-02` is the scope that has two
sig RootId {}     // the task root's identity, as an open no-follow directory
sig Rev {}        // the repository's recorded topology

// --- entries ---------------------------------------------------------------
abstract sig EType {}
one sig FileT, DirT extends EType {}
/* The entry type that cannot be digested: a socket, a fifo, a device node.
   `FN-12` refuses it; `FN-05.a`'s seventh member is that refusal seen from the
   preflight. */
one sig OpaqueT extends EType {}

abstract sig Role {}
one sig FinishLiveR, OrdinaryLiveR, TerminalR, ForeignR extends Role {}

/* The digest is the catalogue's OPAQUE EQUALITY and nothing finer: `FN-12`
   needs digests to distinguish entries, not to be collision-resistant, so
   nothing here constructs one and two entries may share one. */
sig Digest {}

sig Entry { et: one EType, role: one Role, digest: one Digest }

/* The workspace.  `lane` is VAR and the devices are not, and the asymmetry is
   `SY-03`'s: a preflight is never a licence, so the layout must be able to
   CHANGE between the lease gate and the transaction's own gate.  A device
   reading that changed would say the same thing twice; a lane that changes says
   it once, cheaply, and gives `FN-05.a`'s third member a reachable state. */
one sig World {
  var lane: lone Lane,   // absent == the workspace layout is unsupported (SY-02)
  wtDev:   one Device,   // the working-tree root's device  — the LEASE gate's operand
  rootDev: one Device,   // the task root's device          — a TRANSACTION operand
  qDev:    one Device,   // the quarantine parent's device  — the other one
  /* UNRELATED MODIFIED WORKING-COPY WORK — the commit slice's, and the only
     thing in this file that lives OUTSIDE the task root.  `FN-14` requires it to
     survive a successful finish, so it has to be able to change: the world edits
     it (`doTopologyChange` leaves it free) and every Grove step frames it.  A
     `wcWork` that no transition could move would make `FN-14`'s second half a
     tautology rather than a claim. */
  var wcWork: set Entry,
  /* THE OPERATOR'S OWN HOOKS — `FN-30`'s two halves, and they are the exits
     slice's ONE new observable.  `hookInstalled` is STATIC: a hook is
     configuration the operator supplied before any of this ran, and what
     changes is whether it has RUN.  `hookRan` is `var` and is the only `var`
     field this slice adds.

     WHY IT IS ON `World` AND NOT ON `Repo` OR `Sys`.  Exactly one transition in
     this file does not frame the world — `doTopologyChange`, the operator's own
     commit — so putting the observable here makes *a hook can run, and the
     world is what runs it* true for the cost of one conjunct and no new
     transition.  That is `FN-26`'s third conjunct's idiom exactly: the half
     `doCommitMoves` shows the WORLD can do and Grove cannot.  Without it,
     `FN-30` would be a claim about a field nothing could ever set, which is the
     false-confidence shape this file has recorded six times.

     IT IS NOT `FN-27`'s SUBJECT AND `FN-27` DOES NOT NAME IT.  The catalogue's
     reason for `FN-30` — *such a hook may mutate unrelated working-tree bytes
     that no index image restores* — is a reason to have two claims, not one:
     `FN-27` is about the bytes and `FN-30` is about the hook, and a claim that
     described both from one side would be `disposal-k45`'s fourth rule about
     aim met a third time. */
  hookInstalled: lone Hook,
  var hookRan:   lone Hook
}

/* A user-supplied hook, as ONE static atom.  Nothing here is a hook's content,
   its trigger or its exit status: `FN-30` is a claim about whether one RAN, and
   two atoms would say nothing the one says — there is no claim in this scope
   about *which* hook, the way `2 AttemptId` exists so that recording THIS value
   is not the same statement as recording SOME value.  So it adds no scope
   dimension. */
one sig Hook {}

/* THE THREE LANES, PARTITIONED THE ONE WAY AN OBLIGATION IN THIS SLICE READS
   THEM.  `FN-17.a` is the catalogue's first lane-specific claim: *on a
   working-copy-as-commit lane the exact recorded preflight commit SHALL be
   reproduced before the witness is removed*.  Nothing else here distinguishes
   the three, and that a twelve-obligation slice needs exactly one lane split is
   itself recorded in `README.md` as a finding. */
fun wcAsCommitLanes: set Lane { NativeJjL + ColocatedJjL }

one sig Root { var rid: lone RootId, var holds: set Entry }

/* THE QUARANTINE — the handoff slice's whole new signature, and it is ONE
   FIELD on purpose.  The catalogue settles a proven commit by *renaming the
   whole task root — witness and evacuated tree intact — into the quarantine in
   one step*, so the only thing the rename changes is WHERE THE ROOT LIVES.
   Modelling the quarantine as a second place a `RootId` can be, rather than as
   a copy of the root's contents, is what makes "intact" a FRAME CONDITION
   rather than a list of equalities to keep in step — and it is also what keeps
   the step to at most one persistent effect, which is what `FN-24.b` will ask
   of it.

   `World.qDev` is the device this directory sits on, and `FN-08` has read it
   since `entry-k39`; this is the first slice in which the directory itself
   exists.  Nothing here is a cleanup marker or a reaper: `FN-21` and `FN-31`
   are the `disposal` sibling's, and this file's disposal is still an
   abstraction — see `doSettle`. */
one sig Quar { var qRid: lone RootId }

/* THE CLEANUP MARKER — `FN-31`'s subject, and it is a SET OVER A SIGNATURE
   rather than a `lone` field on a singleton, which is the one modelling
   decision in this slice that a reader should not skip.

   `FN-31.b` says *no reader observes the marker absent, nor observes two
   markers*.  With the marker as `one sig Mark { var there: lone Marker }`, TWO
   MARKERS IS INEXPRESSIBLE and half the claim is true by construction — which
   is the false-confidence shape this corpus has recorded four times.  So the
   markers are atoms and what is `var` is WHICH ARE PRESENT at the reserved
   name: `#Cleanup.present = 2` is a state the model can be in, a replacement
   written as remove-then-create is a trace the model can take, and `FN-31.b`
   is a claim that can be false.

   `cOwner` and `cTarget` are STATIC because a marker's bytes are written once:
   the attempt identity that wrote it, and the quarantined root it authorises
   removing.  An owner PRESENT is what Grove can prove is its own; an owner
   ABSENT is a foreign document at the reserved name, which is `Slot.owner`'s
   trick applied to the artifact `FN-31.d` is about.  A marker has no attempt of
   its own to be owned by — it is owned by the attempt that WROTE it, which is
   exactly what the catalogue's *entries carrying Grove's own cleanup manifest*
   means.

   Nothing here is a filename.  The catalogue gives the marker a per-handle,
   per-attempt reserved name and this file has no filename grammar; what it has
   instead is `cTarget`, which is the only thing the reaper reads a name FOR. */
sig CMark { cOwner: lone AttemptId, cTarget: lone RootId }
one sig Cleanup { var present: set CMark }

/* THE RESERVED WITNESS.  Two names in ONE directory, which is the whole of
   `FN-09.a`: `PREPARING-FINISH-<handle>-<attempt>/` while it is being built and
   `FINISHING-<handle>/` once it is published, so publication is exactly one
   same-directory rename and `EN-01` grants it atomicity.  `occ` is therefore the
   witness's CLASS — the same word the closed refusal reason `WitnessPending(class)`
   uses — and not a single reserved marker.

   `owner` is the attempt identity the preparing name carries, and it is how a
   later invocation classifies the artifact: present means Grove can prove the
   witness is its own and can name the recovery (`WitnessPending`), absent means
   it cannot classify it at all (`ReservedNameOccupied`).  That split is the
   catalogue's, not this file's.

   `wHolds` is what has been evacuated INTO the published witness.  It is empty
   while the witness is preparing, and that is `FN-09.b` — a CLAIM, checked, and
   deliberately not a fact. */
abstract sig WClass {}
one sig Preparing, Published extends WClass {}

one sig Slot {
  var occ:    lone WClass,
  var owner:  lone AttemptId,
  var wHolds: set Entry
}

/* The attempt identity: the opaque value drawn once per launch that binds one
   transaction to one still-live session epoch.  Two atoms, because a manifest
   that records SOME attempt identity and a manifest that records THIS one are
   the same statement at one. */
sig AttemptId {}

/* THE EVACUATION MANIFEST, written inside the preparing witness and marked
   ready LAST.  Its five recorded things are the catalogue's, verbatim: the
   finish handle, the attempt identity, the repository anchor, the deletion
   fingerprint, and every evacuated entry's type and digest.

   `mReady` is the ready mark.  This file reads it as BOTH the "written and
   verified" record `FN-11` asks for and the mark `FN-12.a` asks for, and says
   so in `README.md` as an abstraction: the ADR writes and verifies the manifest
   and then marks it ready, so the mark is the durable evidence the verification
   passed, and a separate `verified` field would carry no state the mark does
   not. */
one sig ReadyMark {}
one sig Man {
  var mHandle:  lone Entry,
  var mAttempt: lone AttemptId,
  var mAnchor:  lone Rev,
  var mFinger:  set Entry,
  var mEntries: set Entry,
  var mType:    Entry -> lone EType,
  var mDigest:  Entry -> lone Digest,
  var mReady:   lone ReadyMark
}

/* `wTracked` is the repository having taken the WITNESS itself into its
   tracked set — a jj snapshot, a stray `git add`.  It is the antecedent of
   `FN-13` and the only thing in this file that makes a candidate committed tree
   differ from the deletion fingerprint. */
one sig Tracked {}

/* WHETHER THE EXACT PREFLIGHT COMMIT CAN BE REPRODUCED AT ALL.  It is the
   world's, not Grove's — a jj working copy whose recorded change identity the
   operator has since moved on from cannot be put back — and `FN-17.b` is the
   claim that a restoration which cannot reproduce it BLOCKS rather than
   proceeds.  Without an observable for the impossibility, `FN-17.b`'s branch
   would be an unreachable arm and its mutation would not be a control. */
one sig Reproducible {}

/* THE REPOSITORY, and the three things the commit slice adds to it.

   `tickets` IS THE CORRELATION TICKET, in the only shape the catalogue gives it:
   the deletion commit's own message, naming the finish HANDLE and the ATTEMPT
   IDENTITY.  It sits in version-control history rather than in any artifact the
   transaction can destroy, which is the whole of `FN-03`, and it is a RELATION
   rather than a set because `FN-04` — a ticket from an earlier attempt shall not
   settle a later one — is a claim about the pair and not about either half.

   `reproduced` is the exact preflight commit a working-copy-as-commit lane has
   put back, and `canReproduce` is whether it can be.  Both are `FN-17`'s.

   Nothing here is a quarantine, a marker or a reaper: disposal is `handoff`'s. */
one sig Repo {
  var rev:          one Rev,
  var tracked:      set Entry,
  var wTracked:     lone Tracked,
  var tickets:      Entry -> AttemptId,
  var reproduced:   lone Rev,
  var canReproduce: lone Reproducible
}

/* Confirmation is an operator input Grove cannot verify (`EN-15`).  It is set
   by the world's own action and by nothing else, which is the checkable half of
   *and is never attested*. */
one sig Op { var confirmed: lone Confirmation }
one sig Confirmation {}

/* THE PHASE MACHINE IS THE STEP LIST.  `Fresh` .. `Entered` are the entry
   surface's; the four after them are one per body step, and each body step's
   guard is the phase its predecessor produced.  Writing the order this way
   rather than as a refusal on every out-of-order step is a deliberate choice
   and is recorded in `README.md`: the totality rule — every action returns
   exactly one outcome, and a failed guard is a NAMED REFUSAL — is about what an
   INVOCATION returns, and the body's steps are internal control flow rather
   than separately invocable operations.  The three places a body step really
   can refuse an operator are reachable and are checked; a step taken out of
   order is not one of them. */
abstract sig Phase {}
one sig Fresh, Opened, Entered, Prepared, Manifested, ReadyP, PublishedP,
        Evacuated, Attempted, Classified,
        /* THE HANDOFF SLICE'S ONE PHASE.  It sits BETWEEN `Classified` and
           `Settled` on the forward path and on no other, which is what makes
           the quarantine rename a step of the protocol rather than a detail of
           the settle: a `Committed` classification no longer settles, it
           renames, and the settle that follows disposes what the rename
           produced. */
        Quarantined,
        /* THE REVALIDATION SLICE'S ONE PHASE, and it exists because `FN-22`'s
           *after restoration* row CANNOT BE STATED WITHOUT IT.  `commit-k41`'s
           settle restored the tree, reproduced the preflight commit and released
           the witness in ONE step, and the table needs a state between the
           restoration and the release: *after restoration, `Committed` leaves
           the witness blocking the RESTORED tree*.  A one-step settle could
           only observe what it observed before its own effect — and the restore
           branch frames the whole repository, so that observation is the SAME
           one — which would make the row unreachable by construction.

           Splitting the step is therefore forced by the claim rather than
           chosen, and it turns `FN-17.a`'s *before the witness is removed* from
           a conjunction into a real ordering.  `README.md` records that it
           REMOVES an abstraction rather than adding one. */
        Restored,
        /* THE DISPOSAL SLICE'S TWO, and they exist because `FN-21.a` says
           disposal is re-enterable from any interruption and `EN-03` says the
           removal is not atomic.  `Disposing` is the marker written and the
           content still there; `Disposed` is the content gone and the marker
           still there.  Those are the two interruption points disposal has, and
           each of them is a state a later sweep can tell apart from a directory
           that was never Grove's — which is the whole of what the marker buys.

           NEITHER GETS A CLAUSE IN `BodyPhaseMatchesDisk`, for the reason
           `Quarantined` and `Restored` do not: everything true of the disk at
           them is what `FN-21` and `FN-31` CLAIM, and a clause here would make
           those claims' own mutations unsatisfiable. */
        Disposing, Disposed,
        Settled extends Phase {}
one sig Verdict {}

/* THE FOUR REVALIDATION POINTS, AND THEY ARE STATES RATHER THAN EVENTS.  The
   catalogue fixes exactly two filesystem handoffs — the restoration and the
   quarantine rename — and requires the disposition rechecked immediately before
   and after each.  In this file each of the four is a STATE the transaction can
   be in with a handoff pending, which is what makes them free: a `var` field
   naming the current point would cost a fifth of the state space at a bound of
   ten, and the cost law this scope measured is (phase, guard) points times the
   bound they are reachable at.

   `Txn.disp = Indeterminate` at `Classified` is deliberately NOT a point: no
   handoff was ever pending there, and the classification's own block is what
   `commit-k41` already wrote and `witness_FN_16a` already reaches. */
abstract sig RevPoint {}
one sig BeforeRestore, AfterRestore, BeforeRename, AfterRename extends RevPoint {}

/* THE DISPOSITION IS NOT AN OUTCOME, and the catalogue says so in as many
   words: it is the classification of the COMMIT and an INPUT to the outcome.
   `Committed` settles forward and yields `Applied`; `NotCommitted` rolls back
   and yields a refusal; `Indeterminate` yields exactly one `Blocked`. */
abstract sig Disposition {}
one sig Committed, NotCommitted, Indeterminate extends Disposition {}

/* THE COMMIT'S IMMEDIATE RESULT, AS REPORTED — and `EN-09` is the whole reason
   it is in the signature: *a command's exit status is not a receipt: a result
   may be lost or arrive late*.  ABSENT means lost, or not yet arrived.  Nothing
   in the classification reads it, which is `FN-15.a`, and the fact that it is
   here at all is what gives that claim something to be false about. */
abstract sig Report {}
one sig OkReport, FailReport extends Report {}

one sig Txn {
  var phase: one Phase,
  var pinned: lone RootId,      // the identity pinned at open, rechecked at every later step
  var leaseOk: lone Verdict,    // the LEASE gate's recorded verdict — never a licence
  var attempt: lone AttemptId,  // drawn once per launch; what a rootless retry accepts by
  var anchor:  lone Rev,        // the recorded starting topology a rollback must find
  /* THE FINISH HANDLE, PINNED.  `entry-k39` and `witness-k40` could read the
     handle off the tree (`finishLive`) because nothing in either slice ran past
     the evacuation.  This one does: after `WEvacuate` the root holds nothing and
     `finishLive` is empty, so a classification that read the tree for its handle
     would have nothing to read.  Pinned at `TxnOpen` and adopted from the
     manifest by a recovery, it is a LIVE SESSION's operand rather than an
     artifact — which is exactly what `FN-03` needs it to be.

     It is `set Entry` rather than `lone Entry` deliberately: `gateWork` is not
     checked until the preflight, so `finishLive` may hold two entries at open,
     and a `lone` field would make `doTxnOpen` silently UNAVAILABLE on such a
     tree instead of letting the preflight refuse it. */
  var handle:  set Entry,
  var disp:    lone Disposition,
  var report:  lone Report
}


// ===========================================================================
// ACTIONS, OUTCOMES, AND THE SEVEN PRECONDITIONS
// ===========================================================================

abstract sig Action {}
one sig Idle, Confirm, Decline, TxnOpen, Preflight, Swap, TopologyChange,
        WPrepare, WManifest, WReady, WPublish, WEvacuate, CommitAttempt,
        Crash, Discard,
        /* THE COMMIT SLICE'S FOUR.  `Recover` is the catalogue's `recover`: a
           later launch adopting an interrupted attempt's published witness.
           `Classify` and `Settle` are the two halves the interval is between —
           and they are two ACTIONS rather than one because `EN-09` grants a
           result that arrives LATE, and "late" has no meaning unless there is a
           classification for it to be late for.  `ResultArrives` is that
           arrival, and it is the WORLD's. */
        Recover, Classify, Settle, ResultArrives,
        /* THE HANDOFF SLICE'S ONE TRANSITION.  `FN-19`'s *one atomic rename*,
           and by the cost law the dearest single transition this file has
           added: it is reachable only at the far end of the longest trace in
           the scope. */
        QuarRename,
        /* THE REVALIDATION SLICE'S FOUR, and only two of them are Grove's.

           `Revalidate` is the recheck AFTER the restoration, and it is the step
           `Restored` exists for: it observes, and then either completes the
           refusal by releasing the witness or leaves the witness standing over
           the restored tree.  `QuarReturn` is the recheck after the quarantine
           rename taking its corrective action — the rename's INVERSE, which the
           file did not have.

           `CommitMoves` and `RootNameTaken` are the WORLD's, and both exist so
           that the table's rows have reachable antecedents rather than arguable
           ones.  `CommitMoves` is `EN-09` at the grain this table needs: a
           commit whose result *arrives late* can arrive as the commit ITSELF
           landing after the transaction gave up on it, and a commit that landed
           can be undone by an operator's `jj undo` between two of Grove's steps.
           Without it, `Committed` is MONOTONE — see the note on `doCommitMoves`
           — and the two `Committed` departures the catalogue's table is most
           careful to distinguish are unreachable by construction.
           `RootNameTaken` is the world occupying the task-root name while the
           quarantine holds the root, which is what `FN-22.h`'s *a return that
           cannot complete* is. */
        Revalidate, QuarReturn, CommitMoves, RootNameTaken,
        /* THE DISPOSAL SLICE'S FIVE.  Four are Grove's own disposal, and the
           catalogue requires the first three to be DISTINCT TRANSITIONS rather
           than one step with three branches: `FN-31` says the
           `replace-cleanup-marker` transition is *distinct from creating a
           marker and from removing one*, because `TODO.finish_process.md` Q3
           asks whether replacement is reachable at all and a model that folds
           it away answers Q3 by construction.

           `Reap` is the fifth and it is not a transaction.  It is the sweep the
           catalogue names for entries no transaction can reach — the disk
           `witness_FN_19` leaves, where the task root is absent and
           `doTxnOpen` is therefore unavailable — and it is what makes
           `FN-21.a`'s *resumed disposal* reachable at all.  It is not in
           `bodySteps`, not in `txnActs`, and takes no operator confirmation. */
        MarkerCreate, MarkerReplace, Dispose, MarkerRemove, Reap extends Action {}

/* THE STEP LIST.  `EN-08` grants interruption between any two steps and says
   nothing about what a step is, so the grant is worth exactly what this list
   says.  In order: create the preparing witness; write the manifest into it;
   verify it and mark it ready; publish it by one same-directory rename; move
   the root's entries into it; attempt the commit.  `crash` is enabled at every
   boundary between two of them, and `FN-24.b` — the `exits` sibling's — will
   quantify over this set to ask whether each step has at most one persistent
   effect. */
fun bodySteps: set Action {
  WPrepare + WManifest + WReady + WPublish + WEvacuate + CommitAttempt
  + Recover + Classify + QuarRename + Settle
  + Revalidate + QuarReturn
  + disposalSteps
}

/* DISPOSAL'S OWN FOUR, named as one thing because three claims quantify over
   exactly them: `FN-18`'s *a proven commit is never followed by a
   reconstruction* (which used to be stated over `Settle` and no longer can be —
   see the note there), `FN-21.a`'s re-entrancy, and `FN-31.b`'s atomicity.
   `Reap` is deliberately absent: it is a sweep, not a step of the transaction. */
fun disposalSteps: set Action { MarkerCreate + MarkerReplace + Dispose + MarkerRemove }

/* The transaction's own steps.  `FN-01.a` is stated over exactly this set, and
   the set GREW when the body arrived: `entry-k39` could state it over the entry
   surface alone because the body did not exist. */
fun txnActs: set Action { TxnOpen + Preflight + bodySteps }

abstract sig Result {}
one sig Applied, NoOp, Environmental extends Result {}
abstract sig Refused extends Result {}
/* Five members of the catalogue's closed refusal-reason set.  The set is the
   catalogue's; this file adds none. */
one sig RefNotLive, RefLayoutUnsupported, RefRootIdentityChanged,
        RefNoTrackedDeletion, RefUnsupportedEntryType,
        RefWitnessPending, RefReservedNameOccupied extends Refused {}

/* A SECOND FINDING OF THE SAME SHAPE AS `FN-13`'s, and the file's only added
   refusal atom.  The catalogue maps `NotCommitted` to *rolls back and yields
   `Refused`*, and NONE of its seventeen closed refusal reasons names a finish
   that was rolled back: the closest members — `NoTrackedDeletion`,
   `RootIdentityChanged` — are each false of it.  Reporting it under one of them
   would be a lie the model could not be caught in, so the atom is added HERE,
   named for what it is, and recorded in `README.md` as a finding for
   `formal-synthesis-k16` rather than smuggled into the catalogue's set.
   Nothing but `doSettle`'s rollback branch produces it. */
one sig RefRollbackNotCommitted extends Refused {}

/* `Blocked` — the catalogue's sixth outcome: *a transaction stopped part-way and
   left a stable, recoverable state*.  IT CARRIES NO DIAGNOSIS HERE, and the
   omission is deliberate rather than lazy: the closed partition over
   `RecoveryPending` and `OwnershipConflict` is `FN-25`'s, which is the `exits`
   sibling's, and a commit slice that named the two would answer `FN-25.a`'s
   totality and disjointness by construction — the shape of a false-confidence
   incident rather than a finding.  What this slice needs is an OUTCOME for
   `Indeterminate` and for `FN-17.b`, and that is all it takes. */
one sig BlockedOutcome extends Result {}

/* A MODEL-ONLY OBSERVABLE, and declared as one in `README.md`.  The catalogue
   fixes seven preconditions and seventeen refusal reasons and never states the
   mapping between them; two of the seven — an unsupported layout and an
   unreachable quarantine operand — are the same reason at different gates.  So
   the reason alone cannot say WHICH member refused, and `FN-05.a` requires the
   seven to be individually reachable.  `why` is what makes them so.  Nothing in
   the shipped contract corresponds to it.

   THE WITNESS SLICE ADDS THREE MORE, and they are not preconditions — which is
   why the signature is `Why` and no longer `Precondition`.  Two of them
   (`W9SlotPending`, `W10SlotForeign`) are the two halves of the reserved-name
   situation the catalogue splits, and the third is the one this file could not
   name from the closed set:

   `W8WitnessTracked` IS A FINDING, recorded in `README.md` and in Experiment 2.
   `FN-13`'s stated witness is *a commit attempted while the witness is tracked,
   REFUSED*, and the catalogue's closed refusal-reason set has no member that
   names a tracked witness.  This file reports it under `WitnessPending`, which
   is the closest true statement — an artifact at a reserved name that Grove can
   prove is its own — and uses `why` to keep it distinguishable, exactly the
   device the two `LayoutUnsupported` members already needed.  What follows is
   that an operator cannot be told from the reason alone that the REPOSITORY is
   what is blocking, which is `formal-synthesis-k16`'s to settle. */
abstract sig Why {}
one sig P1Confirm, P2Work, P3Layout, P4Quarantine, P5Identity, P6Fingerprint,
        P7EntryType extends Why {}
one sig W8WitnessTracked, W9SlotPending, W10SlotForeign extends Why {}
/* The commit slice's three, all post-classification.  `W11NotCommitted` rides
   with the added refusal atom above; the other two name the two ways a settle
   blocks, and neither is a blocked DIAGNOSIS — see `BlockedOutcome`. */
one sig W11NotCommitted, W12Indeterminate, W13CannotReproduce extends Why {}
/* THE HANDOFF SLICE'S ONE, and it is a `why` rather than a blocked DIAGNOSIS
   for the reason `BlockedOutcome` gives: the closed partition over
   `RecoveryPending` and `OwnershipConflict` is `FN-25`'s, which is `exits`', and
   a slice that named `OwnershipConflict` here to describe a quarantine target
   Grove cannot prove is free would answer `FN-25.a`'s totality by construction.
   What this slice needs is a name for the branch, and `why` is that name —
   exactly the device the two `LayoutUnsupported` members already needed. */
one sig W14QuarantineOccupied extends Why {}
/* THE REVALIDATION SLICE'S TWO, AND BOTH ARE ROWS OF THE CATALOGUE'S TABLE
   WHOSE `Blocked` THE CATALOGUE DIAGNOSES `RecoveryPending`.  The table
   produces four `Blocked` rows and names the diagnosis for every one of them;
   this file names none of them as an OUTCOME, for the reason `BlockedOutcome`
   gives — the closed partition over `RecoveryPending` and `OwnershipConflict`
   is `FN-25`'s and is `exits`', and a slice that named `RecoveryPending` here
   would answer `FN-25.a`'s totality and exhaustiveness by construction.  So the
   two conditions get a model-only `why` apiece, exactly as
   `W14QuarantineOccupied` did, and `README.md` says why the outcome was not
   extended.  The table's other two `Blocked` rows are `Indeterminate` observed
   away from the rename, which `W12Indeterminate` already names. */
one sig W15CommittedAfterRestore, W16ReturnIncomplete extends Why {}
/* THE DISPOSAL SLICE'S ONE, AND IT IS THE FIRST `why` IN THIS FILE THAT THE
   CATALOGUE NAMES.  `FN-21.c` asks for `OwnershipConflict` BY NAME — *a reaper
   declines a foreign entry at a reserved name*, which the catalogue's
   three-context table fixes as the reaper's answer — and `FN-31.d` asks for the
   same condition at the other gate: a marker Grove cannot prove is its own.
   (That sentence was `TT-24.d` until `obligation-placement-k63` retired it here;
   `FN-32` is the same table's second row.)

   IT IS STILL A `Sys.why` MEMBER AND NOT AN EXTENSION OF THE OUTCOME, and that
   is the same decision `quarantine-k43` and `revalidation-k44` each recorded.
   `FN-25`'s closed partition over `RecoveryPending` and `OwnershipConflict` is
   `exits`', and a slice that made `BlockedOutcome` carry a diagnosis here would
   answer `FN-25.a`'s totality, disjointness and exhaustiveness BY CONSTRUCTION.
   Naming the condition is what this slice needs; naming the PARTITION is not.
   `exits` therefore inherits FOUR model-only `why` values its partition has to
   absorb, and this is the only one of the four the catalogue itself names.

   ONE MEMBER SERVES BOTH GATES, exactly as `RefLayoutUnsupported` serves
   `P3Layout` and `P4Quarantine`: it is the same question — *can Grove prove
   this is its own?* — asked of a quarantine by a sweep and of a marker by a
   transaction.  What an operator cannot learn from the `why` alone is which
   gate refused, and `README.md` records that. */
one sig W17OwnershipConflict extends Why {}

one sig Sys { var act: one Action, var res: one Result, var why: lone Why }

fun finishLive:   set Entry { { e: Root.holds | e.role = FinishLiveR } }
fun ordinaryLive: set Entry { { e: Root.holds | e.role = OrdinaryLiveR } }

/* The catalogue's seven, stated as the DOCUMENT states them — read only by the
   claims, never by a transition.  The transition reads the `gate*` predicates
   below, which are written separately on purpose: a divergence between what the
   catalogue requires and what the transaction checks is then a counterexample
   to `FN-05.a` rather than a definition. */
pred pre1Confirm     { some Op.confirmed }
pred pre2Work        { one finishLive and no ordinaryLive }
pred pre3Layout      { some World.lane }
pred pre4Quarantine  { World.rootDev = World.qDev }
pred pre5Identity    { some Root.rid and Root.rid = Txn.pinned }
pred pre6Fingerprint { some (Root.holds & Repo.tracked) }
pred pre7EntryType   { no e: Root.holds | e.et = OpaqueT }

fun failedPre: set Why {
  { p: Why |
       (p = P1Confirm     and not pre1Confirm)
    or (p = P2Work        and not pre2Work)
    or (p = P3Layout      and not pre3Layout)
    or (p = P4Quarantine  and not pre4Quarantine)
    or (p = P5Identity    and not pre5Identity)
    or (p = P6Fingerprint and not pre6Fingerprint)
    or (p = P7EntryType   and not pre7EntryType) }
}

/* The mapping this file chose, recorded as a decision in `README.md`.  Members
   three and four share `LayoutUnsupported` because `SY-03` makes them the same
   question asked at two gates; the other four are distinct. */
fun reasonOf[p: Why]: lone Refused {
  p = P2Work        implies RefNotLive             else
  p = P3Layout      implies RefLayoutUnsupported   else
  p = P4Quarantine  implies RefLayoutUnsupported   else
  p = P5Identity    implies RefRootIdentityChanged else
  p = P6Fingerprint implies RefNoTrackedDeletion   else
  p = P7EntryType   implies RefUnsupportedEntryType else
  p = W8WitnessTracked implies RefWitnessPending      else
  p = W9SlotPending    implies RefWitnessPending      else
  p = W10SlotForeign   implies RefReservedNameOccupied else
  p = W11NotCommitted  implies RefRollbackNotCommitted  else none
}

/* The deletion fingerprint: the expected, non-empty set of tracked paths the
   commit removes.  `pre6Fingerprint` is the claim that it is non-empty; this is
   the value the manifest records. */
fun fingerprint: set Entry { Root.holds & Repo.tracked }

/* What the TRANSACTION checks.  Stated separately from the seven above. */
pred gateWork        { one finishLive and no ordinaryLive }
pred gateLayout      { some World.lane }
pred gateQuarantine  { World.rootDev = World.qDev }
pred gateIdentity    { some Root.rid and Root.rid = Txn.pinned }
pred gateFingerprint { some (Root.holds & Repo.tracked) }
pred gateEntryType   { no e: Root.holds | e.et = OpaqueT }
pred preflightGates {
  gateWork and gateLayout and gateQuarantine
  and gateIdentity and gateFingerprint and gateEntryType
}

/* THE WITNESS SLICE'S THREE GATES, and the three claims they are checked
   against.  Same discipline as the seven above: the `gate*` predicate is what
   the TRANSACTION reads and the other is what the CATALOGUE requires, written
   apart so a divergence is a counterexample rather than a definition. */

// FN-11, the transaction's side and the catalogue's.
pred gateEvacuated       { no Root.holds and Slot.occ = Published and some Man.mReady }
pred evacuationComplete  {
  Slot.occ = Published            // inside the PUBLISHED witness
  no Root.holds                   // every ordinary root entry is out of the root
  some Root.rid                   // the task root is still present
  some Man.mReady                 // beneath a manifest written and verified
}

// FN-13, likewise.  A candidate committed tree is the repository's tracked set
// as it stands at the attempt; excluding the witness is `no Repo.wTracked`.
pred gateWitnessUntracked   { no Repo.wTracked }
pred candidateExcludesWitness { no Repo.wTracked }

// FN-10.b: what the discard can classify as Grove's own.
pred gateOwned  { some Slot.owner }
pred discardable { Slot.occ = Preparing and some Slot.owner }

/* THE DISPOSAL SLICE'S FOUR PREDICATES, and every one of them is the CATALOGUE's
   side rather than a transition's, in the discipline this file has used since
   the seven preconditions: what the sweep is ALLOWED to touch is written here,
   what it DOES touch is written in `doReap`, and a divergence between them is a
   counterexample to `FN-21.b` rather than a definition of it.

   `FN-21` states three conditions and this file states all three: every marker
   at the reserved name is provably Grove's (`not markerForeign`); one of them
   names the thing being removed (`markerAuthorises`); and no matching in-tree
   witness still owns it (`not inTreeWitnessOwns`).

   `inTreeWitnessOwns` IS AN ABSTRACTION AND IT ERRS TOWARDS DECLINING.  The
   catalogue's *no matching in-tree witness owns them* distinguishes a
   `FINISHING-<handle>/` still standing in the task root from the one that rode
   into the quarantine with it.  This file has one `Slot` and no filename
   grammar, so it cannot tell the two apart; what it reads instead is *there is
   a task root present to hold a witness, that witness is published, and it
   names an attempt one of the markers names*.  On the disk the rename leaves
   the task root is absent and the sweep proceeds; where the world has put
   something back at the task-root name (`doRootNameTaken`), the sweep DECLINES
   where the shipped protocol might have proceeded.  That is the fail-closed
   direction — *Grove never mutates what it cannot prove is its own* — and
   `README.md` records it as an abstraction rather than as a claim. */
pred markerForeign    { some m: Cleanup.present | no m.cOwner }
pred markerAuthorises { some m: Cleanup.present | some m.cOwner and m.cTarget = Quar.qRid }
pred inTreeWitnessOwns {
  some Root.rid and Slot.occ = Published and some (Slot.owner & Cleanup.present.cOwner)
}
pred reapable {
  some Cleanup.present
  not markerForeign
  some Quar.qRid implies markerAuthorises
  not inTreeWitnessOwns
}

/* WHAT `FN-21.c` IS ABOUT, WRITTEN APART FROM `reapable` SO THE TWO CAN
   DISAGREE.  *A foreign entry at a reserved name* is narrower than *not
   reapable*: a quarantine whose in-tree witness still owns it is Grove's own
   and is declined for a different reason (`FN-21.b`).  `reapable` implies
   `not foreignAtReservedName` and nothing states the converse, which is what
   keeps the two obligations' mutations from killing each other. */
pred foreignAtReservedName {
  markerForeign or (some Quar.qRid and not markerAuthorises)
}

/* DISPOSAL'S TERMINAL STATE — the one `FN-21.a` and `FN-31.c` both require a
   resumption to reach, whichever of the two interruption points it resumed
   from.

   IT IS STATED OVER THE TWO NAMES DISPOSAL OWNS AND NOT OVER THE TREE, and a
   counterexample is what taught the difference.  Written as *the quarantine
   gone, the marker gone, AND the reserved witness gone*, `FN-21.a` is false —
   `FN_21a`'s first run found a sweep retiring a stale marker while an
   UNRELATED preparing witness stood at the reserved name, owned by nobody and
   nothing to do with the disposal being resumed.  The sweep was right and the
   predicate was wide: disposal's business is the quarantine and the document
   that authorises removing it, and a witness that is not inside the quarantine
   is another claim's subject (`FN-10`).  Retained in `README.md`.

   What the release of the artifacts INSIDE the quarantine is worth is carried
   separately, by `FN-21.a`'s fourth conjunct, which is stated over the step
   that removes the content rather than over the terminal state. */
pred disposalTerminalNext { no Quar.qRid' and no Cleanup.present' }

/* FN-12.a's five, stated as the catalogue states them.  It is evaluated in the
   state the ready mark is set FROM, which is what "written and verified, then
   marked ready last" means as a trace. */
pred manifestComplete {
  Man.mHandle  = finishLive
  some Man.mAttempt and Man.mAttempt = Txn.attempt
  some Man.mAnchor  and Man.mAnchor  = Txn.anchor
  some Man.mFinger  and Man.mFinger  = fingerprint
  Man.mEntries = Root.holds
  Man.mType    = Root.holds <: et
  Man.mDigest  = Root.holds <: digest
}


// ---------------------------------------------------------------------------
// THE COMMIT SLICE'S EVIDENCE, AND THE THREE THINGS IT DECIDES
//
// `FN-15` says the classification SHALL be derived from THE RECORDED ANCHOR,
// THE EXPECTED FINGERPRINT AND THE EXACT IMMEDIATE RESULT — three operands, and
// the reported exit status is not among them.  Everything below is written out
// of exactly those three, which is what makes `FN-15.a` a claim about this
// file rather than a restatement of it.
// ---------------------------------------------------------------------------

/* The attempts this handle has a landed deletion commit for.  `Txn.handle` is
   the live session's pin, never an artifact, which is what lets a classification
   run with the witness and the manifest gone (`FN-03`). */
fun ticketedAttempts: set AttemptId { Txn.handle.(Repo.tickets) }

/* THE EXACT IMMEDIATE RESULT, PROVEN.  Two conjuncts and both are evidence:
   a correlation ticket naming THIS handle and THIS attempt, and the expected
   deletions actually gone from the repository's tracked set.

   THE SECOND CONJUNCT GOES VACUOUS ONCE THE MANIFEST IS RELEASED, and that is
   `FN-03` rather than a hole: after a forward settle there is no manifest to
   read a fingerprint out of, and the ticket alone is what the claim says the
   durable record is.  Recorded in `README.md` under what a green run does not
   prove, because a reader could take the conjunction for a stronger test than
   it is at the point it matters most. */
pred resultProven { Txn.attempt in ticketedAttempts and no (Man.mFinger & Repo.tracked) }

/* THE RECORDED ANCHOR STILL HOLDS.  Lane-blind, and that is a finding rather
   than a shortcut: the catalogue gives the three lanes three different anchors —
   a head revision, a working-copy change identity with its parents, both plus an
   index image — and states the rollback licence over the ROLE each plays, not
   over its shape.  `Repo.rev` is that role.  `README.md` says so. */
pred anchorHolds { Repo.rev = Txn.anchor }

/* EVERY ARTIFACT THIS TRANSACTION OWNS, as one named thing, so that `FN-20` can
   be stated over the ROLE the catalogue states it over — *no artifact a
   transaction leaves behind is a receipt for it* — rather than over the
   quarantine, which is only the incumbent realisation of that role.  Q1 is
   decided against the role, so a claim written over the quarantine alone would
   be evidence about the incumbent and about nothing else. */
pred leftoverArtifact { some Quar.qRid or some Slot.occ or manWritten }

/* `FN-16`'s licence, stated as the catalogue states it and NOT as the
   classification computes it — the same discipline as `pre*` against `gate*`.
   The two coincide in this file; writing them apart is what makes a mutation to
   the classification a counterexample to `FN-16` rather than a new definition
   of it. */
pred rollbackLicensed { anchorHolds and not resultProven }

/* `FN-17.a`'s exactness, in the two halves the lanes table gives it. */
pred treeMatchesManifest { Root.holds' = Man.mEntries }
pred preflightCommitReproduced {
  (World.lane in wcAsCommitLanes) implies Repo.reproduced' = Txn.anchor
}
pred canReproduceHere {
  (World.lane in wcAsCommitLanes) implies some Repo.canReproduce
}
/* THE REPRODUCTION AS IT STANDS AT THE RELEASE, and it exists because SPLITTING
   THE RESTORATION OPENED A WINDOW THAT THE ONE-STEP SETTLE DID NOT HAVE.
   `World.lane` is `var` — `SY-03` requires it, because a preflight is never a
   licence — so the workspace layout can change BETWEEN the restoration and the
   release.  A tree restored on a Git lane, where `FN-17.a` asks for no
   reproduction at all, can be released on a jj lane, where it does; the
   reproduction was never performed and nothing in the release would notice.
   `FN_17a`'s second conjunct found the trace.

   The answer is `SY-03`'s own rule applied to the new gate: the release
   revalidates against ITS operands rather than resting on what the restoration
   found.  A release that cannot show the reproduction blocks, exactly as
   `FN-17.b` has the restoration itself block, and `README.md` records the
   window as a finding of the split rather than as a defect of the protocol. */
pred reproductionStands {
  (World.lane in wcAsCommitLanes) implies Repo.reproduced = Txn.anchor
}

/* `FN-14`'s scope, over the state the commit moves.  *Exactly the expected
   deletions at their original paths* is the fingerprint leaving the tracked set
   and nothing else leaving it; *unrelated working-copy work SHALL survive* is
   `wcWork` untouched. */
pred commitIsScoped {
  Repo.tracked' = Repo.tracked - Man.mFinger
  World.wcWork' = World.wcWork
}


// ---------------------------------------------------------------------------
// THE REVALIDATION SLICE — WHAT A POINT OBSERVES, WHERE THE POINTS ARE, AND THE
// CATALOGUE'S TEN-ROW TABLE WRITTEN AS DATA
//
// `FN-22` is the largest single claim group in the finish scope and it is the
// only one in this file whose subject is a TABLE.  A table is exactly the shape
// a mutation cannot falsify row by row — a per-row mutation kills a row, and
// nothing kills *the table is complete* — so the table is written HERE, as a
// total function over four points and three dispositions, apart from every
// transition that acts on it.  `FN_22a` then binds every Grove step taken at a
// point to it.  A row deleted from `tableAction` makes the function PARTIAL,
// `Sys.act' = tableAction[..]` false, and the check red: the missing row is a
// counterexample rather than a silence, which is what the leaf's brief asks for.
// ---------------------------------------------------------------------------

/* WHAT A REVALIDATION POINT OBSERVES.  The catalogue's *rechecked* is the
   classification run again over the same three operands, so this is written out
   of `resultProven` and `anchorHolds` and out of nothing else — the same two
   predicates `FN-15.a`'s biconditionals are stated over.

   IT IS WRITTEN APART FROM `doClassify`, WHICH COMPUTES THE SAME FUNCTION
   INLINE, AND THAT IS DELIBERATE ON BOTH SIDES.  The two are bound to each
   other THROUGH the evidence predicates — `FN-15.a` checks that the
   classification is exactly this function of them, and `FN-22`'s rows are stated
   over this one — so neither is free to drift.  What the separation buys is
   mutation isolation: a mutation aimed at `doClassify` is a control for `FN-15`
   and leaves `FN-22`'s rows standing, and a mutation aimed at a corrective
   action is a control for `FN-22` and leaves `FN-15` standing.  A shared
   definition would have made every mutation kill a neighbour, which this file
   has recorded as the third way for a mutation to fail its aim. */
fun observed: one Disposition {
  { d: Disposition |
       (d = Committed     and resultProven)
    or (d = NotCommitted  and not resultProven and anchorHolds)
    or (d = Indeterminate and not resultProven and not anchorHolds) }
}

/* WHERE THE FOUR POINTS ARE.  Two handoffs, each rechecked immediately before
   and after, and each point is a STATE with a handoff pending or just done.

   The two BEFORE points are the same moment in the trace distinguished by which
   handoff the classification pointed at, because the incumbent protocol never
   performs both handoffs in one attempt: `Committed` heads for the rename and
   `NotCommitted` heads for the restoration.  That is what makes the table's two
   *divert* rows meaningful — a divert is the other handoff's guard becoming the
   enabled one — and it is why they are two points and not one. */
pred atRevPoint[p: RevPoint] {
  p = BeforeRestore implies (Txn.phase = Classified and Txn.disp = NotCommitted)
  p = BeforeRename  implies (Txn.phase = Classified and Txn.disp = Committed)
  p = AfterRestore  implies (Txn.phase = Restored)
  p = AfterRename   implies (Txn.phase = Quarantined)
}
fun currentPoint: lone RevPoint { { p: RevPoint | atRevPoint[p] } }

/* THE TABLE'S *CORRECTIVE ACTION* COLUMN, TOTAL OVER 4 x 3.  Twelve
   combinations; the catalogue's ten rows cover all of them because its last row
   is stated over *any point*.  Nothing below reads a transition.

   THE AFTER-RENAME `Committed` ROW NOW READS THE MARKER, AND THAT IS A
   DELEGATION RATHER THAN A LEAK.  The catalogue's corrective action there is
   *complete: dispose (`FN-21`)* — it names another claim group rather than one
   move — and `FN-31` requires disposal's first step to be a CREATE when no
   marker stands at the reserved name and a REPLACE when one does, as two
   distinct transitions.  So the row is a function of the marker as well as of
   the point and the observation, which is what `tableOutcome` has done since
   `revalidation-k44` for the occupied target and the unreproducible commit.
   It is still DATA and still total: delete the row and the function goes
   partial exactly as before.  `README.md` records the decision. */
fun tableAction[p: RevPoint, d: Disposition]: one Action {
  p = BeforeRestore implies Settle       else
  p = BeforeRename  implies QuarRename   else
  p = AfterRestore  implies Revalidate   else
  (d = Committed implies (no Cleanup.present implies MarkerCreate else MarkerReplace)
                 else QuarReturn)
}

/* THE TABLE'S *OUTCOME* COLUMN, likewise total.  Two rows are conditional on
   something other than the observation, and both conditions are the catalogue's
   own: the rename's target may be occupied (`FN-19`, and `quarantine-k43` chose
   `Blocked` there rather than a refusal), and the return may be unable to
   complete (`FN-22.h`).  `FN-17.b`'s *cannot reproduce the exact preflight
   commit* is the third, and it is the before-restoration row's. */
fun tableOutcome[p: RevPoint, d: Disposition]: one Result {
  p = BeforeRestore implies (
      d = Indeterminate implies BlockedOutcome else
      (d = NotCommitted and not canReproduceHere) implies BlockedOutcome else
      Applied)                                                        else
  p = BeforeRename implies (
      d = Indeterminate implies BlockedOutcome else
      (d = Committed and some Quar.qRid) implies BlockedOutcome else
      Applied)                                                        else
  p = AfterRestore implies (
      (d = NotCommitted and reproductionStands) implies RefRollbackNotCommitted
      else BlockedOutcome)                                                else
  (d = Committed implies (markerForeign implies BlockedOutcome else Applied) else
   (some Root.rid implies BlockedOutcome else Applied))
}


// ---------------------------------------------------------------------------
// THE CRASH SLICE — WHAT A DISK CLASSIFIES AS, AND WHAT A STEP DOES TO IT
//
// `FN-24` is the only claim group in this file whose two obligations are about
// the model's own SHAPE rather than about a transition: one says every disk a
// crash can leave classifies as exactly one stable state, the other says every
// step of the transaction changes exactly one thing.  Both are written HERE, as
// data, apart from every transition that produces the states they range over —
// the same discipline `observed` and `tableAction` are written under, and for
// the same reason: a classification a transition defines is a classification
// that cannot disagree with one.
// ---------------------------------------------------------------------------

/* THE STABLE STATES THIS FILE'S DISK CAN BE IN.  The catalogue's task-root state
   table (§*States*) has eleven rows; six of them are reachable here, and one is
   this file's own — see below.  `Reserved(Migrating)`, `PartialScaffold`,
   `Legacy`, `Foreign` and `Malformed` are the task-tree scope's and no finish
   transition produces one, which `README.md` records as a deliberate omission
   rather than a gap.

   `SReservedQuarantined` IS A MODEL-ONLY MEMBER OF THE RESERVED CLASS, AND IT
   IS LOAD-BEARING RATHER THAN DECORATIVE — mutation 50 is the evidence.  A
   disposal that has released the reserved witness while its quarantine is still
   standing is a disk the catalogue's table has no row for: the task root is
   present, nothing is at the witness's name, and Grove's own quarantine is.
   Without this member that disk classifies `Current(Spent)` — an ordinary spent
   grove — which is exactly what §*States*' load-bearing property forbids.
   Adding a member is licensed by the catalogue in as many words: *`TT-18`/
   `TT-19` are stated over the reserved CLASS rather than over its members so
   that removing one member changes no claim*.  `README.md` records it for
   `formal-synthesis-k16` rather than smuggling it into the catalogue's table. */
abstract sig Stable {}
one sig SAbsent, SReservedPreparing, SReservedPublished, SReservedQuarantined,
        SCurrentLive, SCurrentFinishOnly, SCurrentSpent extends Stable {}

/* WHAT THE DISK MATCHES, BEFORE THE ORDER IS APPLIED — and the arms OVERLAP on
   purpose.  Writing each row's own condition, unordered, is what makes
   `FN-24.a`'s *exactly one* a claim about the ORDER rather than a consequence of
   how the arms were phrased: an evacuated tree matches `Reserved(Published)` and
   `Current(Spent)` at once, and the catalogue says in as many words that it is
   the first and never the second.  A set of arms made disjoint by their guards
   would have answered that by construction.

   EVERY ARM IS THE CATALOGUE'S ROW, VERBATIM — `SAbsent` INCLUDED, AND THAT IS
   WHAT PUTS THE WEIGHT ON THE ORDER.  The row reads *no task root* and nothing
   more, so an arm strengthened to *and nothing at a reserved name either* would
   make `FN-24.a`'s third conjunct true by construction and the departure below
   invisible.  Stated verbatim, the post-rename disk matches `Absent` and
   `Reserved(Published)` at once and the ORDER is what has to resolve it — which
   is where this slice's second finding is. */
fun classifiedRaw: set Stable {
  { s: Stable |
       (s = SAbsent              and no Root.rid)
    or (s = SReservedPreparing   and Slot.occ = Preparing)
    or (s = SReservedPublished   and Slot.occ = Published)
    or (s = SReservedQuarantined and some Quar.qRid)
    or (s = SCurrentLive         and some Root.rid and some ordinaryLive)
    or (s = SCurrentFinishOnly   and some Root.rid and some finishLive
                                 and no ordinaryLive)
    or (s = SCurrentSpent        and some Root.rid and no finishLive
                                 and no ordinaryLive) }
}

/* THE CLASSIFICATION ORDER, WRITTEN AS DATA AND AS A STRICT PRECEDENCE — the
   same device the ten-row table uses, and for the same reason: delete a pair
   from it and two rows survive the resolution, which makes `FN-24.a`'s *exactly
   one* red rather than silently weaker.  `s -> t` reads *`s` is classified
   before `t`*.

   IT DEPARTS FROM THE CATALOGUE'S TABLE ORDER IN ONE PLACE, AND THE DEPARTURE
   IS THIS SLICE'S SECOND FINDING.  §*States* lists `Absent` FIRST and the whole
   `Reserved` class after it.  Taken literally that classifies the disk an
   interruption after the quarantine rename leaves — the task-root name free,
   Grove's own quarantine holding the root — as `Absent`, which is exactly what
   the same section's load-bearing property forbids: *a task root whose deletion
   is not yet proven is never `Absent`*.  The two are in tension only once a
   reserved name can be occupied while the task-root name is free, which is a
   situation the finish protocol creates and the task-tree scope never does.
   This file therefore orders the WHOLE RESERVED CLASS BEFORE `Absent`, and
   `FN-24.a`'s third conjunct is what would catch the other order.  `README.md`
   records it for `formal-synthesis-k16`. */
fun earlierThan: Stable -> Stable {
    SReservedPreparing   -> (SReservedPublished + SReservedQuarantined + SAbsent
                             + SCurrentLive + SCurrentFinishOnly + SCurrentSpent)
  + SReservedPublished   -> (SReservedQuarantined + SAbsent
                             + SCurrentLive + SCurrentFinishOnly + SCurrentSpent)
  + SReservedQuarantined -> (SAbsent
                             + SCurrentLive + SCurrentFinishOnly + SCurrentSpent)
  + SAbsent              -> (SCurrentLive + SCurrentFinishOnly + SCurrentSpent)
  + SCurrentLive         -> (SCurrentFinishOnly + SCurrentSpent)
  + SCurrentFinishOnly   -> SCurrentSpent
}

/* The catalogue's three `Current(...)` rows, named as one thing because
   `FN-24.a`'s fourth conjunct is stated over the class rather than over a
   member: what the claim prohibits is a disk with something of Grove's at a
   reserved name reading as an ORDINARY tree, and which of the three it would
   read as depends only on what the root happens to hold. */
fun currentStates: set Stable { SCurrentLive + SCurrentFinishOnly + SCurrentSpent }

/* WHAT THE NEXT INVOCATION READS: what the disk matches, less everything
   something it matches is classified before. */
fun classified: set Stable { classifiedRaw - classifiedRaw.earlierThan }

/* ---------------------------------------------------------------------------
   A STEP'S PERSISTENT EFFECTS, AT THE GRAIN `FN-24.b` STATES THEM
 
   The obligation's grain is THE EFFECT, not the field, and three things follow
   that a field-by-field count gets wrong.  A same-directory rename touches two
   names and `EN-01` makes it ONE effect.  Removing a directory removes what is
   inside it, so a step that releases the reserved witness has not separately
   written its manifest.  And moving entries between two names is one move
   however many entries move.  Counted by field instead, the completed refusal
   would read as four persistent effects and the atomic root rename as two, and
   a correct protocol would report as a defective one.
   --------------------------------------------------------------------------- */

abstract sig Effect {}
one sig ERootName, EQuarName, EWitnessName, EManifest, EReady, EEntries,
        EMarkerName, ECommit, EReproduce extends Effect {}

pred rootNameChanged    { Root.rid' != Root.rid }
pred quarNameChanged    { Quar.qRid' != Quar.qRid }
pred witnessNameChanged { Slot.occ' != Slot.occ or Slot.owner' != Slot.owner
                          or Repo.wTracked' != Repo.wTracked }
pred markerChanged      { Cleanup.present' != Cleanup.present }
pred repoHistoryChanged { Repo.rev' != Repo.rev or Repo.tracked' != Repo.tracked
                          or Repo.tickets' != Repo.tickets }
pred reproducedChanged  { Repo.reproduced' != Repo.reproduced }
/* The three that are INSIDE the reserved witness, and each is suppressed when
   the witness name itself changed: a directory that goes takes its contents
   with it, and counting the contents again would make one removal four
   effects. */
pred manifestChanged {
  not witnessNameChanged
  (Man.mHandle' != Man.mHandle or Man.mAttempt' != Man.mAttempt
   or Man.mAnchor' != Man.mAnchor or Man.mFinger' != Man.mFinger
   or Man.mEntries' != Man.mEntries or Man.mType' != Man.mType
   or Man.mDigest' != Man.mDigest)
}
pred readyChanged   { not witnessNameChanged and Man.mReady' != Man.mReady }
pred entriesMoved   { not witnessNameChanged
                      and (Root.holds' != Root.holds
                           or Slot.wHolds' != Slot.wHolds) }

/* `EN-01`'s ONE SAME-DIRECTORY RENAME, in either direction: the task root's
   identity moving to the quarantine name, or the quarantine's moving back.
   Both names change and the step has ONE persistent effect, which is the whole
   of what `EN-01` grants and the only atomicity this file assumes. */
pred atomicRootRename {
  (some Root.rid and no Root.rid' and no Quar.qRid and Quar.qRid' = Root.rid)
  or (some Quar.qRid and no Quar.qRid' and no Root.rid and Root.rid' = Quar.qRid)
}

fun effectsAt: set Effect {
  { e: Effect |
       (e = ERootName    and rootNameChanged)
    or (e = EQuarName    and quarNameChanged)
    or (e = EWitnessName and witnessNameChanged)
    or (e = EManifest    and manifestChanged)
    or (e = EReady       and readyChanged)
    or (e = EEntries     and entriesMoved)
    or (e = EMarkerName  and markerChanged)
    or (e = ECommit      and repoHistoryChanged)
    or (e = EReproduce   and reproducedChanged) }
}

/* The rename counted once.  It is counted AT THE QUARANTINE NAME rather than at
   the task root's, arbitrarily and stated as arbitrary: what matters to the
   claim is that a rename is one effect, not which of its two names carries it. */
fun persistentEffects: set Effect {
  atomicRootRename implies (effectsAt - ERootName) else effectsAt
}

/* THE TWO STEPS THIS FILE DECLARES, and `FN-24.b` asks for exactly this — *a
   step that is neither is DECLARED, with what it would take to decompose it*.
   `README.md` carries both declarations in full; in one line each:

   `Dispose` clears the quarantine AND the reserved witness, because in this
   model they are two `one sig`s and in the shipped protocol the witness is
   INSIDE the root the rename moved.  Decomposing it means giving the model a
   containment relation between the two names, which is the abstraction
   `EN-03` — no atomic recursive deletion — already forces the shipped removal
   to take entry by entry.

   `doSettle`'s RESTORE BRANCH puts the tree back AND reproduces the exact
   preflight commit, and on a working-copy-as-commit lane those are two
   persistent effects.  Decomposing it means a phase between the restoration and
   the reproduction — which is what `revalidation-k44` did to the settle once
   already, for `FN-22`'s after-restoration row, and what a fifth revalidation
   point would cost is `FN-22`'s answer rather than this claim's. */
pred declaredMultiEffect {
  Sys.act' = Dispose
  or (Sys.act' = Settle and Txn.phase' = Restored)
}

// ---------------------------------------------------------------------------
// THE BLOCKED SLICE — THE DIAGNOSIS PARTITION, AND WHAT A BLOCK'S DIAGNOSTIC
// NAMES
//
// `FN-25` is the claim four slices deliberately did not build.  `commit-k41`,
// `quarantine-k43`, `revalidation-k44` and `disposal-k45` each reached a
// condition the catalogue diagnoses and each gave it a model-only `Sys.why`
// rather than extending `BlockedOutcome`, recording why in as many words: a
// slice that named the partition in the SIGNATURE would answer `FN-25.a`'s
// totality, disjointness and exhaustiveness BY CONSTRUCTION.  That abstinence
// is spendable exactly once and this is the slice that spends it — so the
// partition is written HERE, as data over static atoms, apart from every
// transition that produces the states it ranges over.  `BlockedOutcome` still
// carries no diagnosis and `Sys.why` is still model-only; nothing below is read
// by a guard, and deleting the whole block leaves every other command in this
// file exactly as it was.
//
// THE CLAUSES ARE THE CATALOGUE'S OWN, ONE PREDICATE EACH, and the two arms are
// assembled out of them rather than stated whole.  That is what made the
// findings under *A seventh finding, an eighth and a ninth* findable: read
// literally, `OwnershipConflict`'s second clause holds at every block the
// catalogue's own revalidation table diagnoses `RecoveryPending`.
//
// THE DIAGNOSIS IS A FUNCTION OF THE STATE THE BLOCK IS DECIDED IN, NOT OF THE
// STATE IT LEAVES, and every command below reads it unprimed against a primed
// `Sys.res'`.  That is forced rather than chosen: `doSettle`, `doRevalidate`
// and `doQuarReturn` all block through `txnGone`, so in the state a block
// LANDS in there is no attempt identity, no handle and no anchor left to
// correlate anything against — `Txn.attempt` is empty, `resultProven`'s first
// conjunct reads `none in ...` and is vacuously true, and every block would
// classify alike.  An outcome is what an INVOCATION returns, and the invocation
// still holds its operands when it decides.  `README.md` records this as the
// slice's first counterexample.
// ---------------------------------------------------------------------------

abstract sig Diagnosis {}
one sig DRecoveryPending, DOwnershipConflict extends Diagnosis {}

/* `RecoveryPending`'s clause: *a correlated Grove-owned attempt is incomplete.
   The artifact holding the transaction is provably Grove's, named by THIS
   finish handle and THIS attempt identity.*

   THE CATALOGUE'S THIRD SENTENCE — *and the outcome cannot yet be proven either
   way* — IS NOT A CONJUNCT HERE, and the narrowing is declared rather than
   quiet.  Two rows of the revalidation table are blocks whose outcome IS
   proven: *after restoration, `Committed` leaves the witness blocking the
   restored tree*, and *after the rename, a return that cannot complete*, both
   diagnosed `RecoveryPending` by the table.  As a conjunct the sentence makes
   `FN-25.b` false on both rows.  It appears below as `dgTopologyUnmatched`,
   which is where the catalogue states the same condition a second time and
   under the OTHER diagnosis. */
pred dgCorrelatedIncompleteAttempt {
  some Txn.attempt
  some Slot.occ and Slot.owner = Txn.attempt
  some Txn.handle and Man.mHandle = Txn.handle
  leftoverArtifact
}

/* `OwnershipConflict`'s first clause: *an artifact sits at a name Grove
   reserves but Grove cannot classify it as its own*.  This file has three
   reserved names and each already carries its own correlation test, so the
   clause is the same question — CAN GROVE PROVE THIS IS ITS OWN? — asked three
   times: of the witness slot, of the marker, and of the quarantine.

   THE WITNESS ARM IS `no Slot.owner` WIDENED, AND THE WIDENING IS THIS SLICE'S
   SECOND COUNTEREXAMPLE.  A published witness with an owner whose manifest
   names a DIFFERENT handle than the running attempt is an artifact Grove owns
   and cannot correlate; at `no Slot.owner` alone it fell through both arms and
   `FN-25.b` was false.  The catalogue's clause is the general sentence — *state
   is unrelated, ambiguous, or cannot be proved safe to mutate* — and the three
   examples printed under it are examples.  What the widening costs is recorded
   under *what a green run does not prove*: this arm and
   `dgCorrelatedIncompleteAttempt`'s correlation are complements at the witness
   name, so `FN-25` has no content THERE and all of its content at the other two
   names and at the topology clause.

   The quarantine's arm needed the opposite care.  `foreignAtReservedName` is
   `FN-21.c`'s predicate and is written from the REAPER's standpoint, where no
   transaction is live; used here it fires between `QuarRename` and
   `MarkerCreate` on every ordinary forward path, and the three revalidation
   rows that block in that interval would diagnose `OwnershipConflict` against
   the table's own `RecoveryPending`. */
pred dgWitnessNotProvablyThisAttempts {
  some Slot.occ
  not (some Slot.owner and Slot.owner = Txn.attempt
       and some Txn.handle and Man.mHandle = Txn.handle)
}
pred dgUnclassifiableAtReservedName {
  dgWitnessNotProvablyThisAttempts
  or markerForeign
  or (some Quar.qRid and no Txn.attempt and not markerAuthorises)
}

/* `OwnershipConflict`'s second clause: *the observed topology matches neither
   the recorded anchor nor the expected result*.  It is `Indeterminate` written
   out — `Committed` is `resultProven`, `NotCommitted` is `anchorHolds and not
   resultProven`, and this is the negation of both.

   THE PROVISO IS THIS FILE'S AND IT IS THE SLICE'S THIRD COUNTEREXAMPLE.
   Without `not dgCorrelatedIncompleteAttempt` the clause holds at every
   `Indeterminate` block, all of which the revalidation table diagnoses
   `RecoveryPending`, and `FN-25.a` is not nearly false but flatly so.  The
   catalogue states one condition under two diagnoses and only the table
   disambiguates it; the proviso is the table's answer, written once. */
pred dgTopologyUnmatched {
  not resultProven
  not anchorHolds
  not dgCorrelatedIncompleteAttempt
}

/* `OwnershipConflict`'s third clause: *an entry is of a type Grove refuses to
   touch*. */
pred dgUndigestibleEntry { some e: Root.holds | e.et = OpaqueT }

/* THE TWO ARMS, WHICH OVERLAP — exactly as `classifiedRaw`'s do, and for the
   same reason: an arm narrowed until it cannot meet its neighbour is an arm
   that answers `FN-25.a` by construction. */
fun diagnosedRaw: set Diagnosis {
  (dgCorrelatedIncompleteAttempt implies DRecoveryPending else none)
  + ((dgUnclassifiableAtReservedName or dgTopologyUnmatched or dgUndigestibleEntry)
       implies DOwnershipConflict else none)
}

/* THE TWO PLACES THE ARMS MEET, NAMED HERE AND NOWHERE ELSE, so that narrowing
   the check and declaring the overlap are the same edit — the discipline
   `declaredMultiEffect` is written under.

   The first is `FN-25.a`'s stated witness: a correlated Grove-owned attempt in
   progress WITH a document Grove cannot classify at another name it reserves.
   Reachable, and cheaply — a foreign cleanup marker standing while the attempt
   settles.

   THE SECOND WAS NOT FORESEEN AND IS THE SLICE'S FOURTH COUNTEREXAMPLE: an
   entry of a type Grove refuses to touch, sitting in the task root at a
   correlated block.  The seven preconditions are the ENTRY SURFACE'S — a
   recovery adopts a published witness and never re-runs them — so a settle that
   restores a manifest recording an undigestible entry reaches a correlated
   block with `dgUndigestibleEntry` true.  `README.md` records it.

   WHAT THIS DECLARATION LEAVES CHECKABLE is the clause it does NOT name:
   `FN_25a`'s first conjunct now says the arms never meet through
   `dgTopologyUnmatched`, which is false the moment that predicate's proviso is
   removed.  That is mutation 5. */
pred declaredDiagnosisOverlap {
  dgCorrelatedIncompleteAttempt
  and (dgUnclassifiableAtReservedName or dgUndigestibleEntry)
}

/* THE RESOLUTION, AS A STRICT PRECEDENCE AND AS DATA — the crash slice's
   `earlierThan` over two atoms.  `OwnershipConflict` wins, and the reason is
   the fail-closed rule rather than a preference: if a document Grove cannot
   classify sits at a name it reserves, Grove cannot prove the state safe to
   mutate, however completely it has correlated the rest of it. */
fun earlierDiagnosis: Diagnosis -> Diagnosis {
  DOwnershipConflict -> DRecoveryPending
}

fun diagnosed: set Diagnosis { diagnosedRaw - diagnosedRaw.earlierDiagnosis }

/* WHAT A BLOCK'S DIAGNOSTIC NAMES — `FN-26`'s four, as four static atoms, so
   that *the diagnostic carries all four* is a set equality rather than four
   conjuncts a mutation can pick off one at a time.

   `BObserved` IS TRUE IN EVERY STATE OF THIS FILE AND IS DECLARED AS SUCH.
   `Repo.rev` is `one Rev`: this model has no unreadable repository, so *the
   diagnostic names the observed topology* is a fact of the signature here and
   not a claim.  The arm is kept because deleting it would make the remaining
   three look like the whole of the catalogue's sentence, and `README.md`
   records it under *what a green run does not prove*. */
abstract sig BlockField {}
one sig BArtifact, BRecorded, BObserved, BExits extends BlockField {}

fun blockDiagnostic: set BlockField {
    (leftoverArtifact implies BArtifact else none)
  + (some Txn.anchor implies BRecorded else none)
  + (some Repo.rev implies BObserved else none)
  + ((some Txn.anchor and some Txn.attempt) implies BExits else none)
}



// ===========================================================================
// FRAMING
// ===========================================================================

/* `treeSame` is the byte-identical tree `FN-05.b` is about, and it GREW with
   the witness: the reserved slot's class, its owner, what it holds and the
   manifest inside it are all tree bytes.  Every transition that framed the tree
   before therefore frames the new state too, without being touched. */
pred slotSame  { Slot.occ' = Slot.occ and Slot.owner' = Slot.owner
                 and Slot.wHolds' = Slot.wHolds }
pred manSame   { Man.mHandle'  = Man.mHandle  and Man.mAttempt' = Man.mAttempt
                 and Man.mAnchor'  = Man.mAnchor  and Man.mFinger'  = Man.mFinger
                 and Man.mEntries' = Man.mEntries and Man.mType'    = Man.mType
                 and Man.mDigest'  = Man.mDigest  and Man.mReady'   = Man.mReady }
pred rootSame  { Root.holds' = Root.holds and Root.rid' = Root.rid }
/* THE QUARANTINE IS TREE BYTES, so it joins `treeSame` and every transition
   that framed the tree before frames it now without being touched — the same
   way the reserved slot and the manifest joined in the witness slice.  It is
   named separately as well, because eight transitions frame the tree
   field-by-field rather than through `treeSame`. */
pred quarSame  { Quar.qRid' = Quar.qRid }
/* THE CLEANUP MARKER IS TREE BYTES TOO — a document at a reserved name beside
   the quarantine — so it joins `treeSame` the way the quarantine did, and every
   transition that framed the tree before frames it now without being touched.
   The twelve transitions that frame the tree field-by-field name it explicitly,
   which is the same bookkeeping `quarSame` needed. */
pred markSame  { Cleanup.present' = Cleanup.present }
/* The root's ENTRIES alone.  `doQuarRename` frames them in both branches and
   moves the identity in one, so it cannot use `rootSame`. */
pred rootSameHolds { Root.holds' = Root.holds }
pred treeSame  { rootSame and slotSame and manSame and quarSame and markSame }
/* THE REPOSITORY'S FRAME GREW WITH THE COMMIT, and every transition that framed
   it before frames the new state too without being touched — history, the
   reproduced preflight commit, and whether one can be reproduced at all.
   `repoHistorySame` is the same frame MINUS the witness's tracked bit, for the
   two places that legitimately clear it: the discard, and a settle that releases
   the witness. */
pred repoHistorySame { Repo.rev' = Repo.rev and Repo.tracked' = Repo.tracked
                 and Repo.tickets' = Repo.tickets
                 and Repo.reproduced' = Repo.reproduced
                 and Repo.canReproduce' = Repo.canReproduce }
pred repoSame  { repoHistorySame and Repo.wTracked' = Repo.wTracked }
/* Releasing the witness: the artifact is gone, so the repository cannot still
   be tracking it. */
pred repoSameReleasingWitness { repoHistorySame and no Repo.wTracked' }
/* THE WORLD'S FRAME GREW WITH THE HOOK, and every transition that framed the
   world before frames it now without being touched — which is twenty-eight of
   the file's twenty-nine transitions.  The twenty-ninth is `doTopologyChange`,
   and its absence from this list is `FN-30`'s whole falsifiability. */
pred worldSame { World.lane' = World.lane and World.wcWork' = World.wcWork
                 and World.hookRan' = World.hookRan }
pred opSame    { Op.confirmed' = Op.confirmed }
pred txnSame   { Txn.phase' = Txn.phase and txnCarried and txnResultSame }
/* The transaction's volatile state, carried across a body step unchanged: the
   phase is what each step advances and is written by the step itself.  The
   handle joins the pins here for the same reason it exists — it is the live
   session's, and a step that dropped it would leave the classification with
   nothing to look a ticket up by. */
pred txnCarried { Txn.pinned' = Txn.pinned and Txn.leaseOk' = Txn.leaseOk
                  and Txn.attempt' = Txn.attempt and Txn.anchor' = Txn.anchor
                  and Txn.handle' = Txn.handle }
/* The disposition and the reported result.  Separate from `txnCarried` because
   exactly two steps write them — `Classify` and `ResultArrives` — and every
   other transition frames both. */
pred txnResultSame { Txn.disp' = Txn.disp and Txn.report' = Txn.report }
pred txnResultEmpty { no Txn.disp' and no Txn.report' }
/* The transaction is gone: a crash, or a refusal that leaves nothing behind.
   THE LEASE VERDICT IS NOT PART OF IT.  It is recorded by the driver's own gate
   BEFORE the transaction opens (`FN-08`), so a transaction that ends does not
   un-record it — and `doPreflight` frames it explicitly, which is what makes
   clearing it here a contradiction rather than a choice.  It was one, once: the
   preflight's whole refusal branch became unsatisfiable and eight of
   `entry-k39`'s fourteen witnesses reported *no instance* in the same run. */
pred txnGone   { Txn.phase' = Fresh and no Txn.pinned'
                 and no Txn.attempt' and no Txn.anchor'
                 and no Txn.handle' and txnResultEmpty
                 and Txn.leaseOk' = Txn.leaseOk }
pred noWhy     { no Sys.why' }

pred manEmpty      { no Man.mHandle  and no Man.mAttempt  and no Man.mAnchor
                     and no Man.mFinger  and no Man.mEntries and no Man.mType
                     and no Man.mDigest  and no Man.mReady }
pred manEmptyNext  { no Man.mHandle' and no Man.mAttempt' and no Man.mAnchor'
                     and no Man.mFinger' and no Man.mEntries' and no Man.mType'
                     and no Man.mDigest' and no Man.mReady' }
pred manWritten    { some Man.mHandle or some Man.mAttempt or some Man.mAnchor
                     or some Man.mEntries }


// ===========================================================================
// TRANSITIONS
//
// Every action is TOTAL: it returns exactly one outcome, and a guard that fails
// produces a named refusal rather than an absent transition.  That is the
// catalogue's rule and it is what keeps `FN-01.b` and `FN-05.a` from being true
// for want of a reachable situation.
// ===========================================================================

pred doIdle {
  Sys.act' = Idle and Sys.res' = Environmental and noWhy
  treeSame and repoSame and worldSame and opSame and txnSame
}

/* The operator's, and no grove action appears here — which is what
   `FN-01.a`'s second conjunct checks. */
pred doConfirm {
  no Op.confirmed
  Sys.act' = Confirm and Sys.res' = Environmental and noWhy
  Op.confirmed' = Confirmation
  treeSame and repoSame and worldSame and txnSame
}

/* Declining.  It is NOT a transaction step — that is the whole of `FN-01.a`'s
   first conjunct — so it is what `FN-05.a`'s first member is reached by, and it
   writes nothing. */
pred doDecline {
  no Op.confirmed
  Sys.act' = Decline and Sys.res' = NoOp and Sys.why' = P1Confirm
  treeSame and repoSame and worldSame and opSame and txnSame
}

pred doTxnOpen {
  some Op.confirmed
  some Txn.leaseOk
  Txn.phase = Fresh
  some Root.rid
  Sys.act' = TxnOpen and Sys.res' = Applied and noWhy
  Txn.phase' = Opened and Txn.pinned' = Root.rid and Txn.leaseOk' = Txn.leaseOk
  /* The attempt identity is DRAWN here — once per launch — the anchor is the
     repository's topology as it stands at open, and the handle is the live finish
     leaf as the tree stands at open.  ALL THREE ARE NOW READ BACK: the
     classification compares the anchor, looks a ticket up by the handle, and
     accepts one only for this attempt.  That is what `FN-03`, `FN-04` and
     `FN-15` turn them from pins into operands. */
  some Txn.attempt' and Txn.anchor' = Repo.rev and Txn.handle' = finishLive
  txnResultEmpty
  treeSame and repoSame and worldSame and opSame
}

pred doPreflight {
  some Op.confirmed
  Txn.phase = Opened
  Sys.act' = Preflight
  treeSame and repoSame and worldSame and opSame
  Txn.leaseOk' = Txn.leaseOk
  preflightGates implies {
    Sys.res' = Applied and noWhy
    Txn.phase' = Entered and Txn.pinned' = Txn.pinned
    Txn.attempt' = Txn.attempt and Txn.anchor' = Txn.anchor
    Txn.handle' = Txn.handle and txnResultEmpty
  } else {
    one Sys.why'
    Sys.why' in failedPre
    Sys.res' = reasonOf[Sys.why']
    // The attempt ends and the finish leaf stays live and selectable; the pin
    // goes with it, which is what keeps `Fresh` and `no pinned` in step.
    txnGone
  }
}

/* The world swapping the task root between two steps of the transaction. */
pred doSwap {
  some Root.rid
  Sys.act' = Swap and Sys.res' = Environmental and noWhy
  some Root.rid' and Root.rid' != Root.rid
  Root.holds' = Root.holds and slotSame and manSame and quarSame and markSame
  repoSame and worldSame and opSame and txnSame
}

/* The world changing the repository or the workspace layout under the
   transaction.  `SY-03` is why the second is possible at all.

   It is ALSO how the witness comes to be tracked: `Repo.wTracked'` is left free
   here and framed everywhere else, so a jj snapshot or a stray `git add` that
   takes the published witness into the repository's tracked set is one of this
   transition's outcomes rather than an action of its own.  That is `FN-13`'s
   whole antecedent, and it costs no reachable transition. */
pred doTopologyChange {
  Sys.act' = TopologyChange and Sys.res' = Environmental and noWhy
  (Repo.rev' != Repo.rev or World.lane' != World.lane)
  Repo.tracked' = Repo.tracked
  /* HISTORY IS NOT THE WORLD'S TO WRITE.  A correlation ticket lands by a commit
     and by nothing else, which is what makes `FN-03`'s *the ticket survives* a
     claim about Grove's steps rather than about what the world happens to leave
     alone.  The reproduced preflight commit is likewise Grove's; `canReproduce`
     and `wcWork` are left FREE, because whether a working copy can be put back
     and what unrelated work sits beside it are exactly the world's. */
  Repo.tickets' = Repo.tickets and Repo.reproduced' = Repo.reproduced
  /* AND IT IS WHERE A USER-SUPPLIED HOOK RUNS.  An operator's own commit runs
     the operator's own hooks; a hook that was never installed cannot run, which
     is a guard on the transition rather than a `fact` deliberately — the free
     initial state may still hand-edit a hook into having run, and a fact
     forbidding it would take reachable disks away from every other claim.
     Leaving it free WITHOUT the containment would let the world run a hook
     nobody supplied, which is not what `EN-11` grants. */
  World.hookRan' in World.hookInstalled
  treeSame and opSame and txnSame
}

/* THE COMMIT MOVING UNDER THE TRANSACTION — the world's, and the transition
   without which `FN-22`'s table is half unreachable.

   WHY IT HAD TO EXIST, STATED AS THE ARITHMETIC THAT FORCED IT.  `resultProven`
   is a ticket naming this attempt AND the recorded fingerprint gone from the
   tracked set.  Before this transition, `Repo.tickets` grew only by
   `commitLands` and `Repo.tracked` shrank only by it, and `doTopologyChange`
   framed both — so `resultProven` was MONOTONE, and once a classification
   reached `Committed` no later observation could reach anything else.  The
   catalogue's table has two rows that are exactly that transition
   (`Committed -> NotCommitted` and `Committed -> Indeterminate`) and says in as
   many words that collapsing them lets a block be reported as a refusal.  A
   file in which they are unreachable answers `FN-22.f` and `FN-22.g` by
   construction, which is the false-confidence shape rather than a green.

   WHAT IT MODELS, AND WHY IT IS NOT A NEW ASSUMPTION.  Two things the shipped
   contract already grants.  `EN-09` — *a result may be lost or arrive late* —
   at the grain the table needs: the commit itself can LAND after the
   transaction has classified without it, which is precisely the danger *after
   restoration, `Committed`* exists for.  And `EN-11` at the repository: an
   operator's `jj undo` between two of Grove's steps takes the ticket back out
   of history.  Neither is Grove's action, which is why `FN-03`'s first conjunct
   had to be narrowed to Grove's own steps — see the note there.

   THE ANTECEDENT IS NARROWED TO THREE PHASES, AND THAT IS A COST DECISION
   RECORDED AS ONE.  The three are exactly the phases at which a revalidation
   point can observe the movement.  A ticket that moved earlier — between the
   attempt and the classification — is observed by the CLASSIFICATION, which is
   `FN-15`'s subject and is already reached by `commitLands`' own free branch.
   By this scope's cost law a transition is priced by the (phase, guard) points
   it is enabled at times the bound they are reachable at, and three is the
   fewest that leaves every row of the table with an antecedent. */
pred doCommitMoves {
  Txn.phase in (Classified + Quarantined + Restored)
  Sys.act' = CommitMoves and Sys.res' = Environmental and noWhy
  /* Either this attempt's own commit lands late — SCOPED, because it is the
     same commit `doCommitAttempt` issued and `FN-14` is stated over every
     ticket that lands — or it is undone.  The world never writes a ticket for
     an attempt that did not issue one: that is `FN-04`'s first conjunct, and
     leaving `Repo.tickets'` free here would break it without saying anything
     the catalogue grants. */
  (   (Txn.attempt not in ticketedAttempts
       and Repo.tickets' = Repo.tickets + (Txn.handle -> Txn.attempt)
       and Repo.tracked' = Repo.tracked - Man.mFinger)
   or (Txn.attempt in ticketedAttempts
       and Repo.tickets' = Repo.tickets - (Txn.handle -> Txn.attempt)
       and Repo.tracked' = Repo.tracked) )
  /* `Repo.rev'` IS LEFT FREE, and that is what separates the table's two
     `Committed` departures: an undo that puts the repository back at the
     recorded anchor is observed `NotCommitted` and ends the attempt as a
     refusal, and one that does not is observed `Indeterminate` and blocks. */
  World.wcWork' = World.wcWork and Repo.wTracked' = Repo.wTracked
  Repo.reproduced' = Repo.reproduced and Repo.canReproduce' = Repo.canReproduce
  treeSame and worldSame and opSame and txnSame
}

/* THE TASK-ROOT NAME OCCUPIED WHILE THE QUARANTINE HOLDS THE ROOT — the world's,
   and `FN-22.h`'s whole antecedent.  Its guard is the narrowest in the file:
   the task root absent and the quarantine holding one is a situation `doQuarRename`
   produces and nothing else does, so by the cost law this is a deep transition
   with exactly one enabling point, which `quarantine-k43` measured at +5% on the
   widest command rather than the +30% the earlier form of the law predicted.

   `doSwap` cannot serve: it requires `some Root.rid`, because it is the world
   swapping a root that is THERE.  This is the world putting something at a name
   that is free. */
pred doRootNameTaken {
  no Root.rid
  some Quar.qRid
  Sys.act' = RootNameTaken and Sys.res' = Environmental and noWhy
  some Root.rid' and Root.rid' != Quar.qRid
  rootSameHolds and slotSame and manSame and quarSame and markSame
  repoSame and worldSame and opSame and txnSame
}


// ---------------------------------------------------------------------------
// THE TRANSACTION'S BODY — six steps, in the order the witness needs them
//
// Each step's guard is the phase its predecessor produced, and each carries the
// transaction's pinned identity, lease verdict, attempt identity and anchor
// forward untouched.  A step taken OUT of order is not enabled; see the note on
// `Phase` for why that is not a hole in the totality rule.
// ---------------------------------------------------------------------------

/* STEP 1 — create the witness under the PREPARING name.  It is created before
   any repository preparation, so that every auxiliary the adapters may write is
   already owned on disk by a named handle and attempt.

   This is the body's one operator-facing refusal that is not about the
   repository: the reserved name may already be occupied.  The catalogue splits
   that situation in two by what Grove can PROVE — its own attempt's artifact is
   `WitnessPending(class)` and names a recovery, and anything it cannot classify
   is `ReservedNameOccupied(entry)` and names none.  Nothing has been mutated at
   this point, so both are refusals and both leave the tree byte-identical. */
pred doWPrepare {
  some Op.confirmed
  Txn.phase = Entered
  Sys.act' = WPrepare
  rootSame and quarSame and markSame and repoSame and worldSame and opSame
  no Slot.occ implies {
    Sys.res' = Applied and noWhy
    Slot.occ' = Preparing and Slot.owner' = Txn.attempt and no Slot.wHolds'
    manEmptyNext
    Txn.phase' = Prepared and txnCarried and txnResultSame
  } else {
    some Slot.owner implies {
      Sys.res' = RefWitnessPending and Sys.why' = W9SlotPending
    } else {
      Sys.res' = RefReservedNameOccupied and Sys.why' = W10SlotForeign
    }
    slotSame and manSame
    txnGone
  }
}

/* STEP 2 — write the manifest into the preparing witness.  The five things it
   records are the catalogue's, and they are written HERE rather than asserted:
   `manifestComplete` states the same five separately, as the claim, so a
   divergence between what the catalogue requires and what the step writes is a
   counterexample to `FN-12.a` rather than a definition. */
pred doWManifest {
  some Op.confirmed
  Txn.phase = Prepared
  Sys.act' = WManifest and Sys.res' = Applied and noWhy
  Man.mHandle'  = finishLive
  Man.mAttempt' = Txn.attempt
  Man.mAnchor'  = Txn.anchor
  Man.mFinger'  = fingerprint
  Man.mEntries' = Root.holds
  Man.mType'    = Root.holds <: et
  Man.mDigest'  = Root.holds <: digest
  no Man.mReady'
  rootSame and slotSame and quarSame and markSame and repoSame and worldSame and opSame
  Txn.phase' = Manifested and txnCarried and txnResultSame
}

/* STEP 3 — verify the manifest and mark it READY, which is the last thing
   written into the witness before it is published.  The mark is this file's
   record that the verification passed; see the note on `Man`. */
pred doWReady {
  some Op.confirmed
  Txn.phase = Manifested
  Sys.act' = WReady and Sys.res' = Applied and noWhy
  Man.mReady' = ReadyMark
  Man.mHandle'  = Man.mHandle  and Man.mAttempt' = Man.mAttempt
  Man.mAnchor'  = Man.mAnchor  and Man.mFinger'  = Man.mFinger
  Man.mEntries' = Man.mEntries and Man.mType'    = Man.mType
  Man.mDigest'  = Man.mDigest
  rootSame and slotSame and quarSame and markSame and repoSame and worldSame and opSame
  Txn.phase' = ReadyP and txnCarried and txnResultSame
}

/* STEP 4 — PUBLISH: exactly one atomic same-directory rename, and the only
   atomicity this file assumes.  `EN-01` grants it; nothing else in the step
   moves, which is what makes "exactly one rename" a frame condition rather than
   a comment. */
pred doWPublish {
  some Op.confirmed
  Txn.phase = ReadyP
  Sys.act' = WPublish and Sys.res' = Applied and noWhy
  Slot.occ' = Published
  Slot.owner' = Slot.owner and Slot.wHolds' = Slot.wHolds
  rootSame and manSame and quarSame and markSame and repoSame and worldSame and opSame
  Txn.phase' = PublishedP and txnCarried and txnResultSame
}

/* STEP 5 — EVACUATE the root's entries into the published witness.  It moves a
   NON-EMPTY SUBSET rather than the whole set, which is what makes a partial
   evacuation — and therefore a crash inside one — a reachable state.  The task
   root stays present and keeps its identity throughout: what changes is that it
   no longer holds its entries at their own names, which is the "unwalkable" in
   `FN-11`'s witness. */
pred doWEvacuate {
  some Op.confirmed
  Txn.phase = PublishedP
  some Root.holds
  Sys.act' = WEvacuate and Sys.res' = Applied and noWhy
  /* A non-empty subset leaves the root and arrives in the witness, stated
     first-order: `set`-quantifying the moved entries is higher-order and Alloy
     cannot skolemize it. */
  Root.holds' in Root.holds
  Root.holds' != Root.holds
  Slot.wHolds' = Slot.wHolds + (Root.holds - Root.holds')
  Root.rid' = Root.rid
  Slot.occ' = Slot.occ and Slot.owner' = Slot.owner
  manSame and quarSame and markSame and repoSame and worldSame and opSame
  (no Root.holds') implies Txn.phase' = Evacuated else Txn.phase' = PublishedP
  txnCarried and txnResultSame
}

/* STEP 6 — ATTEMPT the commit.  This slice models the ATTEMPT and nothing past
   it: no commit, no correlation ticket, no anchor comparison, no disposition.
   That is deliberate and is what keeps `FN-11` and `FN-13` from being answered
   with the `commit` sibling's machinery — both need a commit to have been
   attempted, and neither needs one to have happened.

   The step is available from `PublishedP` as well as from `Evacuated` ON
   PURPOSE.  If it were available only once evacuation had completed, `FN-11`
   would be true by construction; instead `gateEvacuated` is what refuses the
   early attempt, and `evacuationComplete` states the same requirement
   separately as the claim. */
pred doCommitAttempt {
  some Op.confirmed
  Txn.phase in (PublishedP + Evacuated)
  Sys.act' = CommitAttempt
  treeSame and worldSame and opSame
  (gateEvacuated and gateWitnessUntracked) implies {
    Sys.res' = Applied and noWhy
    Txn.phase' = Attempted and txnCarried and no Txn.disp'
    /* THE COMMIT EITHER LANDS OR IT DOES NOT, AND GROVE DOES NOT GET TO SAY
       WHICH.  `EN-05` puts the commit outside the filesystem transaction, so the
       interval between the evacuation and the recorded result is irreducible —
       it is the whole of what `TODO.finish_process.md` is about and it is Q2's
       counterfactual.  Modelling the landing as a free branch of THIS step, and
       the reported result as a separate `lone` value, is what makes
       `Indeterminate` reachable rather than argued about.

       When it lands it lands SCOPED (`FN-14`) and it lands a CORRELATION TICKET
       naming this handle and this attempt (`FN-03`, `FN-04`). */
    (commitLands or commitDoesNotLand)
    /* `Txn.report'` is left FREE — `OkReport`, `FailReport`, or absent.  Absent
       is the lost result; a `FailReport` over a landed commit is the exit status
       that is not a receipt.  `EN-09` is the assumption, and the classification
       below never reads this field. */
  } else {
    not gateWitnessUntracked implies {
      Sys.res' = RefWitnessPending and Sys.why' = W8WitnessTracked
    } else {
      Sys.res' = RefWitnessPending and Sys.why' = W9SlotPending
    }
    repoSame and txnSame
  }
}

/* The two branches of the landing, written apart from `commitIsScoped` so that
   `FN-14` is checked against the step rather than defined by it. */
pred commitLands {
  Repo.tickets' = Repo.tickets + (Txn.handle -> Txn.attempt)
  Repo.rev' != Repo.rev
  Repo.tracked' = Repo.tracked - Man.mFinger
  World.wcWork' = World.wcWork
  Repo.wTracked' = Repo.wTracked
  Repo.reproduced' = Repo.reproduced and Repo.canReproduce' = Repo.canReproduce
}
pred commitDoesNotLand { repoSame }

/* INTERRUPTION, between any two steps — `EN-08` as a first-class action.  It
   destroys the transaction's volatile state and the launch's confirmation with
   it, and leaves every persistent byte exactly where it was.  What the next
   invocation finds is therefore a tree, not a transaction, which is the whole
   reason `Slot.owner` and `Man.mReady` are on disk. */
pred doCrash {
  Txn.phase != Fresh
  Sys.act' = Crash and Sys.res' = Environmental and noWhy
  treeSame and repoSame and worldSame
  no Op.confirmed'
  txnGone
}

/* RECOVERY of an UNPUBLISHED witness: discard it.  Never interpret its
   contents — nothing comes back out of `Slot.wHolds` and the root is left
   byte-identical — and fail closed on anything the discard cannot classify as
   Grove's own.  `gateOwned` is what classifies; `discardable` states the same
   requirement separately, as the claim. */
pred doDiscard {
  Txn.phase = Fresh
  Slot.occ = Preparing
  Sys.act' = Discard
  rootSame and quarSame and markSame and worldSame and opSame and txnSame
  repoHistorySame
  gateOwned implies {
    Sys.res' = Applied and noWhy
    no Slot.occ' and no Slot.owner' and no Slot.wHolds'
    manEmptyNext
    no Repo.wTracked'
  } else {
    Sys.res' = RefReservedNameOccupied and Sys.why' = W10SlotForeign
    slotSame and manSame
    Repo.wTracked' = Repo.wTracked
  }
}

// ---------------------------------------------------------------------------
// THE COMMIT SLICE'S FOUR TRANSITIONS
//
// `EN-05` is the shape of all four: no filesystem transaction can include a
// version-control commit, so the commit sits OUTSIDE the six-step body and the
// interval between the evacuation and the recorded result is the problem.
// `CommitAttempt` opens the interval, `Classify` closes it, `Settle` acts on the
// classification, and `ResultArrives` is the world delivering a result that may
// come too late to be read.
// ---------------------------------------------------------------------------

/* RECOVERY of a PUBLISHED witness — the catalogue's `recover`, and the only way
   this file reaches a second launch over one attempt's artifacts.  A later
   session adopts the recorded attempt, anchor and handle out of the manifest,
   which is exactly what the `RecoveryPending` diagnosis describes: an artifact
   provably Grove's, named by THIS finish handle and THIS attempt identity.

   WHY IT ADOPTS RATHER THAN REWRITES.  `FN-12.a` freezes every manifest field
   once the ready mark is on, so a recovery that re-stamped the manifest with its
   own launch's identity would be a counterexample to a claim two slices old.
   Adopting moves the identity the other way — into the live session — and leaves
   the artifact byte-identical, which is also what keeps `FN-04` true through a
   recovery: the ticket that settles this transaction still has to name the
   attempt the live session is settling.

   IT IS DELIBERATELY NARROW.  The four revalidation points, the ten-row table
   and the re-entrancy `FN-22` states are `handoff`'s; what is here is the one
   guarded adoption `FN-18`'s witness needs to exist. */
pred doRecover {
  some Op.confirmed
  Txn.phase = Entered
  Slot.occ = Published
  some Slot.owner and Slot.owner = Man.mAttempt
  some Man.mReady
  Sys.act' = Recover and Sys.res' = Applied and noWhy
  Txn.phase' = PublishedP
  Txn.pinned' = Txn.pinned and Txn.leaseOk' = Txn.leaseOk
  Txn.attempt' = Man.mAttempt
  Txn.anchor'  = Man.mAnchor
  Txn.handle'  = Man.mHandle
  txnResultEmpty
  treeSame and repoSame and worldSame and opSame
}

/* CLASSIFICATION, from evidence and not from exit status.  Three operands and
   the catalogue names all three: the recorded anchor, the expected fingerprint,
   and the exact immediate result.  `Txn.report` is not among them and does not
   appear below, which is the whole of `FN-15.a` and the thing its mutation
   removes.

   IT IS RE-RUNNABLE AFTER A SETTLE, and that is not an oversight: `FN-03`'s
   witness is a retry that has lost every artifact the transaction owned, and
   the state after a forward settle is exactly that tree.  A second
   classification there reads the ticket and nothing else.

   IT IS NO LONGER RE-RUNNABLE AT `Classified`, AND THE REVALIDATION TABLE IS
   WHY — `FN_22j` FOUND IT.  `commit-k41` left the classification enabled at
   `Classified` as well, which was harmless while nothing acted on a stale
   disposition.  It is not harmless once `FN-22` exists: `Classified` with a
   disposition is a state where a HANDOFF IS PENDING, and a classification there
   re-derives the disposition and takes NO CORRECTIVE ACTION — a fifth
   revalidation point the catalogue does not have, at which the protocol can
   observe a change and do nothing about it.  `FN_22j`'s counterexample is
   exactly that trace: `Committed` pending the rename, `Indeterminate` observed,
   and a `Classify` that rewrites the disposition and reports `Applied` where
   the table requires a block.  `quarantine-k43` had already established the
   principle from the other side by refusing to open this step to `Quarantined`;
   this is the same rule at the phase before it. */
pred doClassify {
  some Op.confirmed
  Txn.phase in (Attempted + Settled)
  Sys.act' = Classify and Sys.res' = Applied and noWhy
  treeSame and repoSame and worldSame and opSame
  (Txn.phase = Settled) implies Txn.phase' = Settled else Txn.phase' = Classified
  // (the branch above is now `Attempted -> Classified` or `Settled -> Settled`)
  txnCarried and Txn.report' = Txn.report
  resultProven implies Txn.disp' = Committed
    else (anchorHolds implies Txn.disp' = NotCommitted
                       else Txn.disp' = Indeterminate)
}

/* THE QUARANTINE RENAME — `FN-19`, and the handoff slice's one transition.

   IT REPLACES A STAND-IN.  `commit-k41`'s forward settle released the witness
   and the manifest in place and recorded that release as an ABSTRACTION of
   disposal: `FN-18` needed the artifacts gone so that `FN-03`'s retry had no
   local trace to read, and needed nothing about how.  This is the real thing
   for the first half of it — the whole task root leaves in one rename — and the
   settle that follows disposes what the rename produced.  Disposal's
   re-entrancy, the cleanup marker and the reaper are still absent and are the
   `disposal` sibling's.

   ONE PERSISTENT EFFECT, AND IT IS THE MOVE.  `Quar.qRid'` gains the identity
   and `Root.rid'` loses it, in the same step.  Everything the root held —
   the published witness, its evacuated entries, the manifest inside it — is
   FRAMED rather than copied, which is what "witness and evacuated tree intact"
   is worth in a model whose quarantine is a second place a root can be.

   THE OCCUPIED TARGET BLOCKS RATHER THAN REFUSES, and that is the catalogue's
   shape rather than a convenience.  The transaction has a PROVEN commit at this
   point; ending it as a refusal would say the finish did not happen when the
   ticket in history says it did.  So the attempt ends, the tree keeps its
   published witness and its present task root, and a later launch recovers,
   re-classifies `Committed` on the ticket and tries again — which is exactly
   what `RecoveryPending` describes.  It is reported as a `Blocked` with a
   model-only `why`; naming the diagnosis is `FN-25`'s and `exits`'. */
pred doQuarRename {
  some Op.confirmed
  Txn.phase = Classified
  Txn.disp = Committed
  Sys.act' = QuarRename
  rootSameHolds and markSame and repoSame and worldSame and opSame
  /* REVALIDATION POINT 3 — IMMEDIATELY BEFORE THE QUARANTINE RENAME.  The
     disposition the classification wrote is `Committed`; what the rename acts
     on is what is observed NOW.  Three rows: proceed, divert to the restoration
     path, or perform no handoff at all. */
  observed = NotCommitted implies {
    /* *do not rename; take the restoration path* — the corrective action is to
       put the transaction back at the other handoff's point, which is what
       rewriting the disposition at an unchanged phase does.  Nothing moves. */
    Sys.res' = Applied and noWhy
    Root.rid' = Root.rid and slotSame and manSame and quarSame
    Txn.phase' = Classified and Txn.disp' = NotCommitted
    txnCarried and Txn.report' = Txn.report
  } else observed = Indeterminate implies {
    /* the table's *any point* row: no handoff is performed. */
    Sys.res' = BlockedOutcome and Sys.why' = W12Indeterminate
    Root.rid' = Root.rid and slotSame and manSame and quarSame
    txnGone
  } else no Quar.qRid implies {
    Sys.res' = Applied and noWhy
    Quar.qRid' = Root.rid
    no Root.rid'
    slotSame and manSame
    Txn.phase' = Quarantined and txnCarried and txnResultSame
  } else {
    Sys.res' = BlockedOutcome and Sys.why' = W14QuarantineOccupied
    Root.rid' = Root.rid and slotSame and manSame and quarSame
    txnGone
  }
}

/* THE QUARANTINE RETURN — `FN-22.f`, `.g` and `.h`, and the rename's inverse.
   The catalogue's corrective action for a disposition that CHANGED after the
   rename is *return the quarantine atomically*, and the stable state it names
   is the exact pre-rename one: `Reserved(Published)` with the disposition the
   return observed, from which the attempt runs the path that disposition calls
   for.  So this step is the rename read backwards — one persistent effect, the
   identity moving from `Quar.qRid` to `Root.rid` — and everything the root
   holds rides along framed, exactly as it did on the way out.

   THE TWO DEPARTURES ARE SEPARATED HERE AND NOWHERE ELSE.  A successful return
   under `NotCommitted` lands at the before-restoration point and the attempt
   completes as a REFUSAL; one under `Indeterminate` lands with a disposition no
   handoff acts on and the attempt BLOCKS.  Collapsing them would let a block be
   reported as a refusal, which is the distinction `FN-29` requires an operator
   to be able to make — the catalogue says so in as many words, and this is the
   single branch that honours it.

   A RETURN THAT CANNOT COMPLETE REPORTS BOTH.  If the task-root name has been
   taken while the quarantine held the root, the return has nowhere to go: the
   attempt ends `Blocked` with the quarantine still standing and the changed
   disposition still observable, which is what *report both the change and the
   quarantine* is worth in a model with no diagnostic text. */
pred doQuarReturn {
  some Op.confirmed
  Txn.phase = Quarantined
  observed != Committed
  Sys.act' = QuarReturn
  rootSameHolds and markSame and repoSame and worldSame and opSame
  no Root.rid implies {
    Sys.res' = Applied and noWhy
    Root.rid' = Quar.qRid
    no Quar.qRid'
    slotSame and manSame
    Txn.phase' = Classified and Txn.disp' = observed
    txnCarried and Txn.report' = Txn.report
  } else {
    Sys.res' = BlockedOutcome and Sys.why' = W16ReturnIncomplete
    Root.rid' = Root.rid and slotSame and manSame and quarSame
    txnGone
  }
}

/* REVALIDATION POINT 2 — IMMEDIATELY AFTER THE RESTORATION.  The step the
   `Restored` phase exists for, and the one that turns `FN-17.a`'s *before the
   witness is removed* from a conjunction into an ordering: the tree is already
   back, the witness is still standing over it, and what happens to the witness
   is decided by what is observed HERE.

   Three rows, and the middle one is the reason the whole split was necessary.
   An unchanged `NotCommitted` completes the refusal by releasing the witness.
   A `Committed` — this attempt's commit having landed LATE, after the rollback
   — leaves the witness blocking the restored tree, because the tree now says
   the finish did not happen and history says it did, and a released witness
   would leave nothing for a recovery to read.  An `Indeterminate` blocks the
   same way and for the same reason. */
pred doRevalidate {
  some Op.confirmed
  Txn.phase = Restored
  Sys.act' = Revalidate
  worldSame and opSame and quarSame and markSame
  observed = NotCommitted implies {
    reproductionStands implies {
      Sys.res' = RefRollbackNotCommitted and Sys.why' = W11NotCommitted
      rootSame
      no Slot.occ' and no Slot.owner' and no Slot.wHolds' and manEmptyNext
      repoSameReleasingWitness
      Txn.phase' = Settled and txnCarried and txnResultSame
    } else {
      Sys.res' = BlockedOutcome and Sys.why' = W13CannotReproduce
      rootSame and slotSame and manSame and repoSame
      txnGone
    }
  } else observed = Committed implies {
    Sys.res' = BlockedOutcome and Sys.why' = W15CommittedAfterRestore
    rootSame and slotSame and manSame and repoSame
    txnGone
  } else {
    Sys.res' = BlockedOutcome and Sys.why' = W12Indeterminate
    rootSame and slotSame and manSame and repoSame
    txnGone
  }
}

/* SETTLING — the one action the three dispositions are inputs to, and the
   catalogue's mapping taken verbatim: `Committed` settles forward and yields
   `Applied`, `NotCommitted` rolls back and yields a refusal, `Indeterminate`
   yields exactly one `Blocked`.

   FORWARD NEVER RESTORES (`FN-18`).  The root stays as the evacuation left it
   and the artifacts the transaction owns are RELEASED.  That release is an
   ABSTRACTION of disposal and is recorded as one: the quarantine rename, the
   cleanup marker and the reaper are `handoff`'s, and nothing here claims the
   release is atomic, re-entrant, or a single rename.

   THE ROLLBACK BRANCH IS GUARDED BY THE DISPOSITION AND CHECKED AGAINST THE
   LICENCE.  `rollbackLicensed` states `FN-16` as the catalogue states it and is
   written apart from the classification that computes the same thing, so a
   mutation to either is a counterexample rather than a redefinition.  On a
   working-copy-as-commit lane a restoration that cannot reproduce the exact
   preflight commit BLOCKS and touches nothing (`FN-17.b`); one that can restores
   the tree to the manifest and reproduces it, and only then is the witness
   released (`FN-17.a`). */
pred doSettle {
  some Op.confirmed
  /* THE FORWARD SETTLE IS GONE FROM THIS STEP ENTIRELY, AND `FN-21.a` IS WHY.
     `quarantine-k43` moved it from `Classified` to `Quarantined` because a
     `Committed` classification renames first; the disposal slice takes it out
     of `doSettle` altogether, because *complete: dispose* is no longer one
     move.  What the after-rename point hands off to is `doMarkerCreate` or
     `doMarkerReplace`, and disposal runs from there under its own marker.

     WHAT IS LEFT HERE IS THE RESTORATION PATH AND THE CLASSIFICATION'S OWN
     BLOCK, which is what `Settle` meant before the quarantine existed.  The
     step is still in `bodySteps`, still the before-restoration point's
     corrective action, and still `FN-16` and `FN-17`'s subject. */
  Txn.phase = Classified and Txn.disp != Committed
  Sys.act' = Settle
  worldSame and opSame and markSame
  Txn.disp = Indeterminate implies {
    /* NOT A REVALIDATION POINT: no handoff was ever pending.  This is the
       classification's own block, unchanged since `commit-k41`, and it is what
       `witness_FN_16a` reaches. */
    Sys.res' = BlockedOutcome and Sys.why' = W12Indeterminate
    treeSame and repoSame
    txnGone
  } else {
    /* REVALIDATION POINT 1 — IMMEDIATELY BEFORE THE RESTORATION.  The
       disposition the classification wrote is `NotCommitted`; the restoration
       runs on what is observed NOW.

       `rollbackLicensed` AND `observed = NotCommitted` ARE THE SAME CONDITION,
       and noticing it is worth a line rather than a silent coincidence: the
       licence is `anchorHolds and not resultProven`, which is exactly the
       middle arm of `observed`.  `FN-16` is therefore the before-restoration
       revalidation stated as a claim about the licence, and `FN-22`'s row is
       the same requirement stated as a claim about the point.  They are still
       written apart, so a mutation to either is a control for one of them. */
    quarSame
    observed = Committed implies {
      /* *do not restore; take the forward path* — the disposition is rewritten
         at an unchanged phase, which puts the transaction at the other
         handoff's point.  Nothing is restored, which is the row's own words. */
      Sys.res' = Applied and noWhy
      rootSame and slotSame and manSame and repoSame
      Txn.phase' = Classified and Txn.disp' = Committed
      txnCarried and Txn.report' = Txn.report
    } else observed = Indeterminate implies {
      /* the table's *any point* row. */
      Sys.res' = BlockedOutcome and Sys.why' = W12Indeterminate
      rootSame and slotSame and manSame and repoSame
      txnGone
    } else {
      canReproduceHere implies {
        /* *proceed with the restoration* — AND STOP THERE.  The witness is NOT
           released here: `Restored` is a state of the protocol, and what
           becomes of the witness is revalidation point 2's to decide.  The
           evacuated entries come back out of the witness and into the root, so
           the witness stands empty over a restored tree with its manifest
           intact — which is exactly what a later recovery would have to read if
           this attempt were interrupted between the two steps. */
        Sys.res' = Applied and noWhy
        treeMatchesManifest and Root.rid' = Root.rid
        no Slot.wHolds'
        Slot.occ' = Slot.occ and Slot.owner' = Slot.owner
        manSame
        Repo.rev' = Repo.rev and Repo.tracked' = Repo.tracked
        Repo.tickets' = Repo.tickets and Repo.wTracked' = Repo.wTracked
        Repo.canReproduce' = Repo.canReproduce
        preflightCommitReproduced
        (World.lane not in wcAsCommitLanes) implies Repo.reproduced' = Repo.reproduced
        Txn.phase' = Restored and txnCarried and txnResultSame
      } else {
        Sys.res' = BlockedOutcome and Sys.why' = W13CannotReproduce
        rootSame and slotSame and manSame and repoSame
        txnGone
      }
    }
  }
}


// ---------------------------------------------------------------------------
// THE DISPOSAL SLICE'S FIVE TRANSITIONS
//
// `EN-03` is the shape of all five: THERE IS NO ATOMIC RECURSIVE DIRECTORY
// DELETION.  The quarantine rename put the whole task root somewhere Grove owns
// in one atomic move; getting rid of it is the one thing in the protocol that
// cannot be one move, so it is the one thing an interruption can leave
// half-done.  The cleanup marker is what makes the half-done state legible: a
// document at a reserved name, naming the attempt that wrote it and the root it
// authorises removing.  Write it, remove what it authorises, remove it.
//
// AND THE FOURTH IS NOT A TRANSACTION.  A crash after the rename leaves the
// task root absent, `doTxnOpen` requires `some Root.rid`, and no transaction can
// be opened on that disk — which `README.md` has carried as *a state this file
// demonstrates and cannot leave* since `quarantine-k43`.  The reaper is the
// sweep that leaves it, and it is what makes `FN-21.a`'s *resumed disposal*
// reachable rather than argued about.
// ---------------------------------------------------------------------------

/* DISPOSAL, STEP 1a — CREATE the cleanup marker.  The after-rename point's
   corrective action when the reserved name is free.

   IT IS A SEPARATE TRANSITION FROM THE REPLACEMENT, AND `TODO.finish_process.md`
   Q3 IS WHY.  Q3 asks whether the marker-replacement sub-transaction — a whole
   crash-safe protocol nested inside the cleanup of a crash-safe protocol — is
   reachable at all.  A model with one `write-marker` step branching internally
   would answer it by construction, whichever way the branch fell; two
   transitions make the answer a REACHABILITY QUESTION, which is what `FN-31.a`
   asks and what the catalogue means by *decide by reachability rather than by
   construction*. */
pred doMarkerCreate {
  some Op.confirmed
  Txn.phase = Quarantined
  /* REVALIDATION POINT 4 — IMMEDIATELY AFTER THE QUARANTINE RENAME, the
     *`Committed` unchanged* row, unchanged from `revalidation-k44` except in
     which step carries it.  Disposal is not reachable on the strength of the
     disposition the classification wrote; it is reachable on what is observed
     HERE, and anything else hands the state to `doQuarReturn`. */
  observed = Committed
  no Cleanup.present
  Sys.act' = MarkerCreate and Sys.res' = Applied and noWhy
  /* ONE PERSISTENT EFFECT: the marker appears.  It names THIS attempt — which
     is what `FN-21.b`'s *Grove's own cleanup manifest* is worth in a file with
     no filename grammar — and THIS quarantined root, which is what a later
     sweep reads to know what the marker authorises removing. */
  one Cleanup.present'
  Cleanup.present'.cOwner = Txn.attempt
  Cleanup.present'.cTarget = Quar.qRid
  rootSame and slotSame and manSame and quarSame
  repoSame and worldSame and opSame
  Txn.phase' = Disposing and txnCarried and txnResultSame
}

/* DISPOSAL, STEP 1b — REPLACE a cleanup marker that is already standing.  One
   atomic same-directory rename (`EN-01`), which is what `FN-31.b`'s *no reader
   observes the marker absent, nor observes two markers* costs: the supersession
   is one transition, and a protocol that removed and then created would be two.

   WHY A MARKER IS EVER ALREADY STANDING — the answer to Q3, and it is the whole
   value of this transition being separate.  `doMarkerRemove` is disposal's LAST
   step, so an interruption between the content's removal and the marker's leaves
   an owned marker whose target is gone.  The reaper will collect it; a NEW
   attempt that reaches the after-rename point first will not, and must supersede
   it with its own.  That is a reachable source state and `witness_FN_31a` runs
   the protocol up to it rather than positing it.

   AND IT IS THE FILE'S SECOND `FN-31.d`-SHAPED GATE.  A marker Grove cannot
   prove is its own is not replaced, is not removed, and is not written over: the
   attempt blocks with the quarantine standing, exactly as an occupied quarantine
   target does and for the same reason — there is a proven commit at this point,
   so a refusal would say the finish did not happen while the ticket says it
   did. */
pred doMarkerReplace {
  some Op.confirmed
  Txn.phase = Quarantined
  observed = Committed
  some Cleanup.present
  Sys.act' = MarkerReplace
  rootSame and slotSame and manSame and quarSame
  repoSame and worldSame and opSame
  markerForeign implies {
    // FN-31.d — never against a marker Grove cannot prove is its own.
    Sys.res' = BlockedOutcome and Sys.why' = W17OwnershipConflict
    markSame
    txnGone
  } else {
    Sys.res' = Applied and noWhy
    /* THE SUPERSESSION, AND IT IS ONE STEP.  Exactly one marker before, exactly
       one after, and no state between: that is what `FN-31.b` asks of it, and
       the mutation that splits it into a remove and a create is what makes the
       claim falsifiable rather than true by construction. */
    one Cleanup.present'
    Cleanup.present'.cOwner = Txn.attempt
    Cleanup.present'.cTarget = Quar.qRid
    Txn.phase' = Disposing and txnCarried and txnResultSame
  }
}

/* DISPOSAL, STEP 2 — REMOVE WHAT THE MARKER AUTHORISES REMOVING.  The
   quarantine goes, and the published witness, its evacuated entries and the
   manifest inside it go with it, because they are inside the root the rename
   moved.

   IT IS GUARDED BY THE MARKER AND NOT BY THE PHASE ALONE, and that is the
   whole of `FN-21.b` at the transaction's own gate: the step reads the document
   at the reserved name, checks that it is Grove's and that it names THIS
   quarantine, and removes nothing otherwise.  A reaper reads exactly the same
   document, which is what makes the two resumable into each other.

   ONE STEP IS AN ABSTRACTION AND `README.md` RECORDS IT AS ONE.  `EN-03` says
   the removal is not atomic, so the shipped protocol takes it entry by entry;
   this file has no filename grammar and one `Quar.qRid`, so it cannot decompose
   the removal further than the marker protocol's own two boundaries.  What is
   modelled is that the removal is MARKER-GUARDED and RE-ENTERABLE, which is what
   `FN-21.a` claims; what is not modelled is a partial removal within it. */
pred doDispose {
  some Op.confirmed
  Txn.phase = Disposing
  some Quar.qRid
  some m: Cleanup.present | m.cOwner = Txn.attempt and m.cTarget = Quar.qRid
  Sys.act' = Dispose and Sys.res' = Applied and noWhy
  no Quar.qRid'
  no Slot.occ' and no Slot.owner' and no Slot.wHolds' and manEmptyNext
  rootSame and markSame
  repoSameReleasingWitness and worldSame and opSame
  Txn.phase' = Disposed and txnCarried and txnResultSame
}

/* DISPOSAL, STEP 3 — REMOVE THE MARKER, and it is LAST for the reason the
   manifest's ready mark is written last: the document is the evidence that the
   work it authorises has not been done, so it cannot go before the work does.
   `FN-21.a`'s second conjunct is exactly that ordering, and it is what makes a
   resumption able to tell an interrupted disposal from a finished one. */
pred doMarkerRemove {
  some Op.confirmed
  Txn.phase = Disposed
  Sys.act' = MarkerRemove and Sys.res' = Applied and noWhy
  no Cleanup.present'
  rootSame and slotSame and manSame and quarSame
  repoSame and worldSame and opSame
  Txn.phase' = Settled and txnCarried and txnResultSame
}

/* THE REAPER — `FN-21.b` and `FN-21.c`, which is where `TT-24.d`'s content
   landed when that obligation was retired.  A SWEEP RATHER THAN A TRANSACTION, and the first thing in this file that runs outside the phase
   machine entirely.

   WHY IT CANNOT BE A TRANSACTION.  `doTxnOpen` requires `some Root.rid`, and
   the disk an interruption immediately after the rename leaves has none — the
   task root left in the rename.  `witness_FN_19` has demonstrated that state
   since `quarantine-k43` and `README.md` has recorded that nothing in the file
   could leave it.  This is what leaves it.

   ITS GUARD IS NARROWED TO *THERE IS SOMETHING AT A RESERVED NAME*, AND THAT IS
   A COST DECISION RECORDED AS ONE.  `Txn.phase = Fresh` is a state a trace can
   REST in — the dwell shape `revalidation-k44` named as the expensive one — so
   a sweep enabled at every `Fresh` state would be this file's dearest
   transition by a distance.  A sweep over nothing is a no-op; requiring
   something to sweep costs no reachable behaviour and takes the enabling
   surface down to the states the protocol actually leaves.

   IT TAKES NO CONFIRMATION, and that is not an omission.  `FN-01.a` is stated
   over `txnActs` and the reaper is not in it: an operator confirms a FINISH, and
   collecting the garbage a crashed finish left is not a second finish.  It is
   also why `FN-24.b`'s *at most one persistent effect per step* should not be
   asked of it — AND THE CRASH SLICE CORRECTED THIS SENTENCE, which used to end
   *though as written each firing has exactly one*.  It does not: the
   content-removal branch clears the quarantine name and the reserved witness
   name together, exactly as `doDispose` does and for the same modelling reason.
   The exclusion was right; the reason offered for it was one sentence too
   generous, and `README.md` records the correction under *what a green run does
   not prove*.

   IT RESUMES IN THE ORDER DISPOSAL RUNS IN: the content first, then the marker.
   That is what makes `FN-21.a`'s *resumption reaches the same terminal state*
   true of an interruption at either of disposal's two points, and it is why the
   sweep is idempotent: run it on a terminal state and its guard is false. */
pred doReap {
  Txn.phase = Fresh
  some Cleanup.present or some Quar.qRid
  Sys.act' = Reap
  worldSame and opSame and txnSame
  reapable implies {
    Sys.res' = Applied and noWhy
    some Quar.qRid implies {
      // resume at disposal's step 2 — remove what the marker authorises
      no Quar.qRid'
      no Slot.occ' and no Slot.owner' and no Slot.wHolds' and manEmptyNext
      rootSame and markSame
      repoSameReleasingWitness
    } else {
      // resume at disposal's step 3 — the content is gone, collect the marker
      no Cleanup.present'
      rootSame and slotSame and manSame and quarSame
      repoSame
    }
  } else {
    /* THE CATALOGUE'S THIRD CONTEXT, VERBATIM: *declines the entry, mutating
       nothing, and reports it; the sweep continues over entries the reaper CAN
       prove are Grove's*.  It is neither a refusal nor a block — nothing was
       ever entered, so there is nothing to leave stable — and `NoOp` is the
       outcome this file already uses for an action that reports and mutates
       nothing (`doDecline`).  The `why` is the diagnosis `FN-21.c` names. */
    Sys.res' = NoOp and Sys.why' = W17OwnershipConflict
    treeSame and repoSame
  }
}

/* THE RESULT ARRIVING — the world's, and `EN-09` as a first-class action.  A
   result may be lost or arrive LATE, and late is only meaningful against a
   classification that has already happened.  Removing this action is exactly the
   assumption table's exercise-removal for `EN-09`, and the command that runs it
   is at the foot of this file. */
pred doResultArrives {
  Txn.phase in (Attempted + Classified + Settled)
  no Txn.report
  Sys.act' = ResultArrives and Sys.res' = Environmental and noWhy
  some Txn.report'
  treeSame and repoSame and worldSame and opSame
  Txn.phase' = Txn.phase and txnCarried and Txn.disp' = Txn.disp
}

pred step {
  doIdle or doConfirm or doDecline or doTxnOpen or doPreflight
  or doSwap or doTopologyChange
  or doWPrepare or doWManifest or doWReady or doWPublish or doWEvacuate
  or doCommitAttempt
  or doRecover or doClassify or doQuarRename or doSettle or doResultArrives
  or doRevalidate or doQuarReturn or doCommitMoves or doRootNameTaken
  or doMarkerCreate or doMarkerReplace or doDispose or doMarkerRemove or doReap
  or doCrash or doDiscard
}

/* TRUE BY CONSTRUCTION, and asserted of the free initial state so that state 0
   is a state the transitions could have produced.  The initial tree is
   otherwise unconstrained — `EN-11` cashed out as a modelling decision, exactly
   as the sibling scope does it — which is what lets every witness below run at
   three states instead of running up to its situation from an empty root. */
fact TxnStateWellFormed {
  always {
    Txn.phase = Fresh iff no Txn.pinned
    Txn.phase = Fresh iff no Txn.attempt
    Txn.phase = Fresh iff no Txn.anchor
    Txn.phase != Fresh implies some Txn.leaseOk
    /* The handle is `set Entry`, so it cannot be pinned by an `iff` — a tree
       whose live finish leaves number two would otherwise make `TxnOpen`
       unavailable rather than make the preflight refuse.  One direction is all
       that is true by construction. */
    Txn.phase = Fresh implies no Txn.handle
    /* A disposition exists only once something has classified, and a reported
       result only once something has been attempted.  Both are volatile and both
       go with the transaction. */
    some Txn.disp   implies Txn.phase in (Classified + Quarantined
                                          + Restored + Disposing + Disposed
                                          + Settled)
    some Txn.report implies Txn.phase in (Attempted + Classified
                                          + Quarantined + Restored
                                          + Disposing + Disposed + Settled)
  }
}

/* TRUE BY CONSTRUCTION, and the reason the free initial state stays honest once
   the body is six steps long.  A body phase names WHERE THE TRANSACTION IS, and
   what is on disk at each of them is what the previous step wrote; a state that
   claims a phase its own disk contradicts is not a state any execution reaches.

   NONE OF THIS IS A CLAIM.  `FN-09.b` — no PREPARING witness ever holds an
   evacuated entry — is deliberately absent from here and checked instead: it is
   the one relation between a phase and the disk that the protocol has to earn.
   The clauses below say only that the phase and the disk agree about which STEP
   has run, never about what a step was allowed to do. */
fact BodyPhaseMatchesDisk {
  always {
    Txn.phase in (Prepared + Manifested + ReadyP) implies
      (Slot.occ = Preparing and Slot.owner = Txn.attempt)
    Txn.phase in (PublishedP + Evacuated) implies
      (Slot.occ = Published and Slot.owner = Txn.attempt)
    Txn.phase = Prepared   implies manEmpty
    Txn.phase = Manifested implies (manWritten and no Man.mReady)
    Txn.phase in (ReadyP + PublishedP + Evacuated) implies some Man.mReady
    Txn.phase = Evacuated  implies no Root.holds
    /* The commit slice's three.  `Attempted` and `Classified` are the two ends
       of `EN-05`'s interval, and what is on disk across it is what `gateEvacuated`
       let the attempt through on.  `Settled` is after the artifacts are released,
       whichever branch released them. */
    Txn.phase in (Attempted + Classified) implies
      (Slot.occ = Published and Slot.owner = Txn.attempt
       and no Root.holds and some Man.mReady)
    Txn.phase = Settled implies (no Slot.occ and manEmpty)
    /* `Quarantined` GETS NO CLAUSE, AND THE ABSENCE IS THE POINT.  Everything
       true of the disk at that phase — the root gone from its own name, the
       quarantine holding it, the witness and the manifest intact inside it — is
       what `FN-19` CLAIMS about the rename.  A clause here would make the
       claim's own mutation UNSATISFIABLE, and an unsatisfiable mutation reports
       exactly as a surviving one; this file has recorded that trap twice
       already.  The phase is reachable by the rename and by nothing else —
       state 0 is `Fresh + Opened` — so nothing is needed here to keep it
       honest.  `Restored` GETS NO CLAUSE FOR THE SAME REASON: everything true
       of the disk there — the tree back at the manifest, the witness standing
       empty over it, the manifest intact — is what `FN-22`'s before-restoration
       row CLAIMS about the restoration, and a clause here would make that row's
       mutation unsatisfiable. */
  }
}

/* WHERE A TRACE STARTS, and the one place this slice had to narrow `EN-11`.
   `entry-k39` leaves the whole initial state free and cites *any well-formed
   tree is reachable by hand edit*.  That licence is about the TREE, and the
   entry surface could take it whole because its transactions are two steps
   long.  A six-step body cannot: an initial state at `Txn.phase = ReadyP` is not
   a hand-edited tree, it is a RUNNING TRANSACTION nobody started, and three
   separate checks below failed on one — a manifest half-written by no step, a
   published witness over an absent task root, an undigestible entry inside an
   entered transaction the preflight would have refused.

   So the DISK stays completely free — the slot, its owner, what it holds, the
   manifest, the root, the repository, all of it, which is what keeps a foreign
   reserved name and an interrupted manifest reachable at state 0 — and the
   transaction's VOLATILE phase starts where a process starts: no transaction,
   or one just opened.  Everything past `Opened` is reached by running the
   steps.  Note the absence of `always`: this constrains state 0 and nothing
   else, so a crash still leaves any body's disk behind at `Fresh`, which is
   exactly what recovery has to read. */
fact TransactionsStartWhereAProcessStarts { Txn.phase in (Fresh + Opened) }

/* TRUE BY CONSTRUCTION.  An entry is beneath the task root or it is not, and
   `wcWork` is the catalogue's *unrelated working-copy work* — which is exactly
   the entries that are not.  Without this the world could satisfy `FN-14`'s
   *unrelated work survives* with an entry the evacuation was carrying. */
fact UnrelatedWorkIsOutsideTheTaskRoot {
  always no (World.wcWork & (Root.holds + Slot.wHolds))
}

/* TRUE BY CONSTRUCTION.  An unoccupied reserved name is a name with nothing at
   it: no owner, no evacuated entries, no manifest, and nothing for the
   repository to have tracked. */
fact EmptySlotHoldsNothing {
  always (no Slot.occ implies
    (no Slot.owner and no Slot.wHolds and manEmpty and no Repo.wTracked))
}

/* The lease gate's verdict cannot have been recorded by a gate that did not
   pass.  It reads the WORKING-TREE ROOT's device against the quarantine
   parent's — never the task root's, which is `FN-08`'s whole subject. */
fact LeaseVerdictIsHonest {
  always (some Txn.leaseOk implies (World.wtDev = World.qDev and once some World.lane))
}

fact Trace {
  Sys.act = Idle and Sys.res = Environmental and no Sys.why
  always step
}


// ===========================================================================
// CLAIMS — FN-01, FN-05 .. FN-08
//
// WHY EVERY BEHAVIOURAL COMMAND RUNS AT `3 steps`.  An Alloy 6 trace is a
// lasso, so the last state must loop.  A state reached by a state-changing
// action loops neither back to the initial idle state nor to itself, so at
// `2 steps` no applied transition exists at all and every check conditioned on
// an outcome is vacuously true.  Three states admit one transition followed by a
// stutter.  `FN-06`'s witness needs TWO consecutive transitions — a swap, then
// the preflight that catches it — and runs at four.
// ===========================================================================

// --- FN-01: confirmation enables, and is never attested ---------------------

/* FN-01.a.  Two conjuncts, and the second is what *never attested* means in a
   model: confirmation changes only by the world's own action, so the
   transaction cannot manufacture its own. */
check FN_01a_no_transaction_step_runs_unconfirmed_and_none_is_attested {
  always {
    (Sys.act' in txnActs) implies some Op.confirmed
    (Op.confirmed' != Op.confirmed) implies Sys.act' in (Confirm + Crash)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* The transaction never entered for want of confirmation — with the
   DETERMINISTIC guard `FN-01` names (a live finish leaf, no live ordinary work)
   holding, so the trace says confirmation and not the guard. */
run witness_FN_01a_a_transaction_never_entered_for_want_of_confirmation {
  always no Op.confirmed
  always Txn.phase = Fresh
  gateWork
  eventually Sys.act = Decline
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

/* FN-01.b.  The other direction, and the non-redundant one: a CONFIRMED attempt
   whose deterministic guard fails is still refused.  Confirmation is not a
   substitute for the guard any more than the guard is for it. */
check FN_01b_confirmation_is_not_a_substitute_for_the_deterministic_guard {
  always ((Sys.act' = Preflight and some Op.confirmed and not pre2Work)
            implies Sys.res' in Refused)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* Distinct from `witness_FN_01a` in the state it reaches, which is what the
   catalogue's *distinct from the previous* asks for: there, no confirmation and
   a satisfied guard; here, a confirmation and a failed one.

   It requires the `Confirm` ACTION rather than letting the free initial state
   supply a confirmation already in place, and that is not decoration: `doConfirm`
   is the only transition permitted to change `Op.confirmed`, so without this the
   file would check `FN-01.a`'s second conjunct over a transition no command ever
   demonstrates.  It costs one extra state. */
run witness_FN_01b_a_confirmed_attempt_refused_for_want_of_the_guard {
  eventually (Sys.act = Confirm and some Op.confirmed)
  eventually (some Op.confirmed and Sys.act = Preflight and Sys.why = P2Work)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

// --- FN-05: preflight mutates nothing ---------------------------------------

/* FN-05.a.  The set is closed and exactly seven-membered, checked as a
   BICONDITIONAL between what the catalogue requires and what the transaction
   gates on: the preflight refuses exactly when some member fails, the reported
   member is genuinely failing, and exactly one is reported.  The `Decline`
   conjunct is the first member, which is not a preflight step at all.

   WHAT MAKES THIS CHECK EVIDENCE IS THE MUTATION, not the check.  `preflightGates`
   and the seven `preN` predicates are stated separately so a divergence is a
   counterexample, but a mutation that removes a member from BOTH is invisible
   here and visible in the matrix.  `README.md` records which. */
check FN_05a_the_preflight_precondition_set_is_closed_and_exactly_seven {
  always {
    (Sys.act' = Preflight) implies {
      (Sys.res' in Refused) iff (some failedPre)
      (Sys.res' in Refused) implies (one Sys.why' and Sys.why' in failedPre
                                     and Sys.res' = reasonOf[Sys.why'])
    }
    (Sys.act' = Decline) implies (not pre1Confirm and Sys.why' = P1Confirm)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

run witness_FN_05a_p1_confirmation_absent {
  eventually (Sys.act = Decline and Sys.why = P1Confirm)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

run witness_FN_05a_p2_no_live_finish_leaf_or_live_ordinary_work {
  eventually (Sys.act = Preflight and Sys.why = P2Work and Sys.res = RefNotLive)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

/* The layout is unsupported AT THE PREFLIGHT, having been supported at the
   lease gate — which is `SY-03` stated as a trace rather than as prose. */
run witness_FN_05a_p3_layout_unsupported {
  eventually (Sys.act = TopologyChange and no World.lane)
  eventually (Sys.act = Preflight and Sys.why = P3Layout
              and Sys.res = RefLayoutUnsupported)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

run witness_FN_05a_p4_quarantine_target_unreachable {
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine
              and Sys.res = RefLayoutUnsupported)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

run witness_FN_05a_p5_task_root_identity_unverified {
  eventually (Sys.act = Preflight and Sys.why = P5Identity
              and Sys.res = RefRootIdentityChanged)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

run witness_FN_05a_p6_empty_deletion_fingerprint {
  eventually (Sys.act = Preflight and Sys.why = P6Fingerprint
              and Sys.res = RefNoTrackedDeletion)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

run witness_FN_05a_p7_an_entry_type_that_cannot_be_digested {
  eventually (Sys.act = Preflight and Sys.why = P7EntryType
              and Sys.res = RefUnsupportedEntryType)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

/* FN-05.b.  Quantified over EVERY reported PREFLIGHT failure, which is what
   makes the seven witnesses above discharge *each of the seven, with the tree
   unchanged*: the check's antecedent is a reported `why` at a preflight step,
   and each of the seven is reachable under it.

   THE ANTECEDENT NARROWED WHEN THE BODY ARRIVED, deliberately.  `entry-k39`
   wrote it as `some Sys.why'` over every action, which was the same set when
   `why` could only be reported by `Preflight` and `Decline`.  The witness slice
   gives `why` three post-flight members, and a check that quantified over them
   too would be stating `FN-27` — *nothing unrelated is mutated, on any outcome*
   — under `FN-05`'s name.  `FN-27` is the `exits` sibling's, at its own bounds,
   with its own witnesses; absorbing it here would report a cell filled that no
   command had reached. */
check FN_05b_a_failed_precondition_leaves_the_tree_byte_identical {
  always ((some Sys.why' and Sys.act' in (Preflight + Decline)) implies treeSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

run witness_FN_05b_a_refusal_with_the_tree_unchanged {
  eventually (some Sys.why and Sys.act in (Preflight + Decline))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

check FN_05c_a_failed_precondition_leaves_the_repository_byte_identical {
  always ((some Sys.why' and Sys.act' in (Preflight + Decline)) implies repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* The repository is exercised in the same trace that refuses — a topology
   change moves it, and the preflight step does not.  A witness that only
   reached a refusal would be equally consistent with a model whose repository
   cannot change at all. */
run witness_FN_05c_a_refusal_with_the_repository_unchanged {
  eventually (Sys.act' = TopologyChange and Repo.rev' != Repo.rev)
  eventually (some Sys.why' and Sys.act' = Preflight and repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

// --- FN-06: the task root's identity is pinned and rechecked ----------------

/* A mid-transaction swap is a REFUSAL rather than a mutation applied elsewhere:
   the pinned identity is rechecked at the later step, and the tree the
   transaction was pointed at is left byte-identical. */
check FN_06_the_task_roots_identity_is_pinned_and_rechecked {
  always ((Sys.act' = Preflight and Root.rid != Txn.pinned)
            implies (Sys.res' in Refused and treeSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* Two consecutive transitions, which is why this one runs at four states: the
   swap, then the preflight that catches it. */
run witness_FN_06_a_swap_between_two_steps_is_refused {
  eventually (Sys.act = Swap and after (Sys.act = Preflight and Sys.why = P5Identity))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

// --- FN-07: an untracked tree is refused before evacuation ------------------

check FN_07_an_empty_deletion_fingerprint_is_refused_before_any_mutation {
  always ((Sys.act' = Preflight and no (Root.holds & Repo.tracked))
            implies (Sys.res' in Refused and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

run witness_FN_07_a_wholly_untracked_tree {
  eventually (Sys.act = Preflight and no Repo.tracked and some Root.holds
              and Sys.res = RefNoTrackedDeletion)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

// --- FN-08: the quarantine target is proved reachable before mutation -------

/* The lease gate's verdict proves `wtDev = qDev` and nothing else.  This check
   says entry is never granted on it: the transaction's OWN operands — the task
   root and the quarantine parent — must agree, whatever the earlier gate found. */
check FN_08_the_lease_gates_verdict_never_licenses_the_transactions_operands {
  always ((Sys.act' = Preflight and Sys.res' = Applied)
            implies World.rootDev = World.qDev)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* A layout that passes at lease acquisition and fails here.  It needs two
   devices, and that is exactly what `EN-02` removes below. */
run witness_FN_08_a_layout_that_passes_at_lease_acquisition_and_fails_here {
  some Txn.leaseOk
  World.wtDev = World.qDev
  World.rootDev != World.qDev
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps



// ===========================================================================
// CLAIMS — FN-09 .. FN-13, THE RESERVED WITNESS
//
// WHERE THESE WITNESSES START, AND WHY IT IS NOT THE FREE INITIAL STATE.  The
// entry surface's witnesses start wherever they like: `EN-11` — any well-formed
// tree is reachable by hand edit — is cashed out here as an unconstrained
// initial state, and a preflight is one step away from anywhere.  That licence
// covers the TREE.  It does not cover the transaction's VOLATILE state, and this
// slice's body is six steps long, so starting a witness at `Txn.phase = ReadyP`
// would be asserting the very run-up the witness exists to demonstrate.
//
// So every body witness below starts at `Txn.phase = Entered` — the state
// `Preflight`'s success branch produces — and runs the body for real.  `Entered`
// itself is paid for ONCE, by `witness_FN_09a_the_transaction_is_entered_by_a_
// preflight`, which is also the file's first successful preflight: `entry-k39`'s
// fourteen witnesses are all refusals, so until this command the `Applied` branch
// of `doPreflight` was reached by no run in the file.
//
// WHAT IS NOT HERE.  No commit, no correlation ticket, no disposition, no
// rollback, no quarantine, no reaper, no revalidation table.  `FN-11` and
// `FN-13` both need a commit to have been ATTEMPTED and neither needs one to
// have happened, so `doCommitAttempt` stops at the attempt and the `commit`
// sibling's machinery stays out of this file.
// ===========================================================================

// --- FN-09: build, then publish, in one atomic step -------------------------

/* Two things, and the second is the one `EN-01` pays for.  `Published` appears
   only by publication and only out of `Preparing`; and the publishing step moves
   the name and NOTHING else — no entry, no manifest field, no repository byte —
   which is what "exactly one atomic same-directory rename" is worth as a frame
   condition.

   *No reader observes it half-applied* is not checked here and is not a gap:
   `Slot.occ` is `lone WClass` and has no third value, which is `EN-01`'s grant
   taken as given.  The premise-break control that removes it — a rename
   observable half-applied — is QUINT's in the assumption table, not this
   family's. */
check FN_09a_publication_is_exactly_one_atomic_same_directory_rename {
  always {
    (Slot.occ' = Published and Slot.occ != Published)
      implies (Sys.act' = WPublish and Slot.occ = Preparing)
    (Sys.act' = WPublish and Sys.res' = Applied) implies {
      Slot.occ = Preparing and Slot.occ' = Published
      Slot.owner' = Slot.owner and Slot.wHolds' = Slot.wHolds
      rootSame and manSame and repoSame and worldSame
    }
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

/* The licence every body witness below runs on, and the file's first `Applied`
   preflight.  Two consecutive transitions, so four states. */
run witness_FN_09a_the_transaction_is_entered_by_a_preflight {
  eventually Sys.act = TxnOpen
  eventually (Sys.act = Preflight and Sys.res = Applied and Txn.phase = Entered)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* An interruption IMMEDIATELY AFTER publication — the state `EN-01` is the only
   reason is not a torn one.  Five transitions: prepare, manifest, ready,
   publish, crash. */
run witness_FN_09a_an_interruption_immediately_after_publication {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = WPublish and Slot.occ = Published
              and after (Sys.act = Crash and Slot.occ = Published))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

/* FN-09.b.  The preparing witness holds nothing that was evacuated, ever —
   which is what makes an interruption before publication DISCARDABLE rather
   than a partial tree someone has to interpret.  Deliberately a check and not a
   fact: it is the one relation between the phase and the disk that the protocol
   has to earn, and `BodyPhaseMatchesDisk` is written to leave it alone.

   STATED OVER THE TRANSITION RELATION, AND THAT IS A FINDING.  Written the
   obvious way — `always (Slot.occ = Preparing implies no Slot.wHolds)` — this
   check FAILS, at state 0, on a free initial state that hand-edits a preparing
   witness with something inside it.  `EN-11` cashed out as an unconstrained
   initial state licenses exactly that, so under it EVERY "never" claim about
   tree SHAPE is false unless it is restated as a claim about what the protocol
   DOES.  A fact would make the check vacuous and the mutation unsatisfiable,
   which is the trap `README.md` records twice already; the two conjuncts below
   are the honest form:

     (i) nothing is ever moved into a witness that is not published, and
     (ii) the witness this transaction builds is built EMPTY

   and together they are the claim for every witness Grove itself created. */
check FN_09b_no_preparing_witness_ever_holds_an_evacuated_entry {
  always {
    (some (Slot.wHolds' - Slot.wHolds))
      implies (Sys.act' = WEvacuate and Slot.occ = Published and Slot.occ' = Published)
    (Sys.act' = WPrepare and Sys.res' = Applied) implies no Slot.wHolds'
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run witness_FN_09b_an_interruption_inside_the_build {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = WManifest and Slot.occ = Preparing)
  eventually (Sys.act = Crash and Slot.occ = Preparing and no Slot.wHolds
              and manWritten)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

// --- FN-10: an unpublished witness is discardable ---------------------------

/* Three conjuncts.  Only an unpublished witness is discarded at all; the
   outcome is a total function of the OWNERSHIP CLASSIFICATION and of nothing
   else — which is the checkable half of *never by interpreting its contents*,
   because a discard that read the manifest would have a second input; and a
   discard leaves the root byte-identical, because nothing comes back out of the
   witness. */
check FN_10a_an_unpublished_witness_is_discarded_never_interpreted {
  always ((Sys.act' = Discard) implies {
    Slot.occ = Preparing
    (Sys.res' = Applied) iff discardable
    (Sys.res' = Applied) implies (no Slot.occ' and no Slot.wHolds' and rootSame)
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run witness_FN_10a_a_discard {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = Crash and Slot.occ = Preparing)
  eventually (Sys.act = Discard and Sys.res = Applied and no Slot.occ)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

/* FN-10.b.  Content the discard cannot classify as Grove's own fails closed:
   refused, with the tree and the repository byte-identical.  It is the
   `ReservedNameOccupied` half of the catalogue's split, and it names no
   recovery — telling an operator to run one against someone else's bytes is the
   fail-closed violation the split exists to prevent. */
check FN_10b_content_the_discard_cannot_classify_fails_closed {
  always ((Sys.act' = Discard and not gateOwned)
            implies (Sys.res' in Refused and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* Reached from the free initial state and not from a run-up, because a foreign
   artifact at a reserved name is TREE state and `EN-11` is exactly the licence
   for it. */
run witness_FN_10b_a_refusal_to_discard_unclassifiable_content {
  Txn.phase = Fresh and Slot.occ = Preparing and no Slot.owner
  eventually (Sys.act = Discard and Sys.res = RefReservedNameOccupied
              and Sys.why = W10SlotForeign and Slot.occ = Preparing)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

// --- FN-11: evacuation precedes deletion ------------------------------------

/* Every ordinary root entry is inside the PUBLISHED witness, beneath a manifest
   written and verified, before any commit is attempted.  `doCommitAttempt` is
   deliberately enabled at `PublishedP` as well as at `Evacuated`, so the early
   attempt is a REACHABLE refusal rather than an absent transition and this check
   has an antecedent it could fail on. */
check FN_11_evacuation_precedes_any_attempted_commit {
  always ((Sys.act' = CommitAttempt and Sys.res' = Applied)
            implies evacuationComplete)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* THE INTERVAL, which is why this is the file's widest command.  `task-tree-k7`'s
   first bound-vacuity predictor says an interval claim needs interval-many
   states: the task root present, unwalkable and holding every entry is not a
   state but a stretch of trace with a publication before it and an attempted
   commit after it.  Six transitions — prepare, manifest, ready, publish,
   evacuate, attempt — so eight. */
run witness_FN_11_the_interval_between_publication_and_commit {
  Txn.phase = Fresh and no Slot.occ
  some Root.holds
  eventually (Sys.act = WPublish and Slot.occ = Published and some Root.holds)
  eventually (Sys.act = WEvacuate and Slot.occ = Published
              and no Root.holds            // unwalkable
              and some Root.rid            // and still present
              and Slot.wHolds = Man.mEntries   // holding every entry
              and some Man.mReady)
  eventually (Sys.act = CommitAttempt and Sys.res = Applied)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-12: the manifest is complete and marked ready last ------------------

/* Three conjuncts, and the middle one is what *last* means in a trace rather
   than in a sentence: once the mark is on, no field under it ever moves again
   while the witness stands. */
check FN_12a_the_manifest_is_complete_and_marked_ready_last {
  always {
    (no Man.mReady and some Man.mReady')
      implies (Sys.act' = WReady and manifestComplete)
    (some Man.mReady and some Man.mReady') implies {
      Man.mHandle'  = Man.mHandle  and Man.mAttempt' = Man.mAttempt
      Man.mAnchor'  = Man.mAnchor  and Man.mFinger'  = Man.mFinger
      Man.mEntries' = Man.mEntries and Man.mType'    = Man.mType
      Man.mDigest'  = Man.mDigest
    }
    (Sys.act' = WPublish and Sys.res' = Applied) implies some Man.mReady
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

/* A manifest interrupted before its ready mark, RECOVERED AS NOT READY: the
   crash leaves a written, unmarked manifest inside a preparing witness, and what
   the next invocation does with it is discard it — never read it. */
run witness_FN_12a_a_manifest_interrupted_before_its_ready_mark {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = Crash and Slot.occ = Preparing
              and manWritten and no Man.mReady)
  eventually (Sys.act = Discard and Sys.res = Applied and no Man.mReady)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

/* FN-12.b.  An entry type that cannot be digested is refused BEFORE ANY
   MUTATION, and the gate that does it is the preflight's seventh member — the
   same predicate, not a second one.  `FN-05.a`'s member and this obligation
   therefore agree by construction rather than by duplication, which is the
   point: the digest step never has to refuse, because the transaction it would
   refuse in is never entered.

   THE CONSEQUENCE IS RECORDED IN `README.md` AS A LIMIT.  `Root.holds` changes
   only by evacuation in this slice — `EN-11` is a free initial state and not a
   `hand-edit` transition — so a manifest-time revalidation of the entry types
   has no reachable antecedent and this file does not write one.  `SY-03` would
   ask for it; the check below is stated over the whole body so that it would
   catch one if the world could ever produce it. */
check FN_12b_an_undigestible_entry_type_is_refused_before_any_mutation {
  always {
    (Sys.act' = Preflight and not pre7EntryType)
      implies (Sys.res' in Refused and treeSame and repoSame)
    (Sys.act' = Preflight and Sys.res' = Applied) implies pre7EntryType
    (Sys.act' in bodySteps and Sys.res' = Applied)
      implies (no e: Root.holds | e.et = OpaqueT)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

run witness_FN_12b_a_refused_entry_type {
  eventually (Sys.act' = Preflight and (some e: Root.holds | e.et = OpaqueT)
              and Sys.res' = RefUnsupportedEntryType and Sys.why' = P7EntryType
              and treeSame and repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

// --- FN-13: the witness is never committed ----------------------------------

/* A candidate committed tree is what the repository would record at the attempt.
   The witness is excluded from every one of them, in both directions: no attempt
   is ever APPLIED over a tracked witness, and an attempt that meets one is
   refused with the tree and the repository byte-identical.

   THE REFUSAL REASON IS A FINDING, not a modelling choice — see the note on
   `Why`.  The catalogue's closed reason set has no member that names a tracked
   witness, so this reports `WitnessPending` and distinguishes the case by the
   model-only `why`. */
check FN_13_every_candidate_committed_tree_excludes_the_witness {
  always {
    (Sys.act' = CommitAttempt and Sys.res' = Applied)
      implies candidateExcludesWitness
    (Sys.act' = CommitAttempt and not candidateExcludesWitness)
      implies (Sys.res' in Refused and treeSame and repoSame)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* A commit attempted while the witness is tracked, refused.  Seven transitions:
   the five that build and publish and evacuate, the world's snapshot that takes
   the witness into the tracked set, and the attempt that meets it. */
run witness_FN_13_a_commit_attempted_while_the_witness_is_tracked_refused {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = TopologyChange and some Repo.wTracked
              and Slot.occ = Published)
  eventually (Sys.act' = CommitAttempt and no Root.holds
              and Sys.res' = RefWitnessPending and Sys.why' = W8WitnessTracked
              and treeSame and repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps


// ===========================================================================
// CLAIMS — FN-03, FN-04, FN-14 .. FN-18, THE COMMIT AND ITS DISPOSITION
//
// `EN-05` IS THE SHAPE OF THIS WHOLE SECTION.  No filesystem transaction can
// include a version-control commit, so the commit sits outside the six-step body
// and there is an INTERVAL between the evacuation and the recorded result that
// no protocol here can close.  Every obligation below is about what can be known
// across that interval and what may be done on the strength of it.
//
// WHERE THESE WITNESSES START.  The witness slice paid twice — once for the
// free initial state's honesty (`fact TransactionsStartWhereAProcessStarts`) and
// once for running every body witness from `Txn.phase = Entered`.  This slice
// pays a third time and stops.  Reaching a SETTLED disposition from a fresh
// grove is ten transitions; reaching a retry that has lost its artifacts is
// twelve; and `witness-k40` measured this file's widest command at ten states
// already.  So nine of the commands below start from `interruptedMidEvacuation`
// — the DISK an interruption mid-evacuation leaves — and recover forward.
//
// That is not a weakening of the witness slice's rule, it is that rule applied:
// the licence covers the TREE and this predicate constrains only tree state, at
// `Txn.phase = Fresh`, which is precisely what a later launch finds.  The
// transaction's volatile state still starts where a process starts, and every
// witness below still runs `TxnOpen`, `Preflight` and `Recover` for real.  What
// is NOT demonstrated is the six-step body followed by a commit in one trace;
// `witness_FN_11` demonstrates the body and `witness_FN_15b_git` demonstrates
// the run-up from a fresh grove to a classified disposition, and the cost of the
// missing join is recorded in `README.md`.
// ===========================================================================

/* THE DISK AN INTERRUPTION MID-EVACUATION LEAVES: a published witness this
   attempt owns, a ready manifest inside it, part of the tree already moved, and
   the task root still present and still holding the live finish leaf.  Nothing
   volatile — the transaction is `Fresh`, which is what `doCrash` produces and
   what a later launch reads. */
pred interruptedMidEvacuation {
  Txn.phase = Fresh
  some Op.confirmed
  // a supported workspace on one device, past the lease gate
  some World.lane and World.rootDev = World.qDev and World.wtDev = World.qDev
  some Txn.leaseOk
  // the published witness, owned, with a ready manifest, part-filled
  Slot.occ = Published
  some Slot.owner and Slot.owner = Man.mAttempt
  some Man.mReady and some Slot.wHolds
  no Repo.wTracked
  // the task root, present, unemptied, and holding exactly the finish leaf
  some Root.rid
  Root.holds = finishLive and one finishLive and no ordinaryLive
  no e: Root.holds | e.et = OpaqueT
  Root.holds in Repo.tracked
  // the manifest the interrupted attempt wrote, and never rewrote (FN-12.a)
  Man.mHandle = finishLive
  some Man.mAnchor and Man.mAnchor = Repo.rev
  some Man.mFinger and Man.mFinger in Repo.tracked
  Man.mEntries = Slot.wHolds + Root.holds
}

// --- FN-03: the correlation ticket is the durable record --------------------

/* Three conjuncts, and the third is the one the claim is actually about.
   History is append-only under Grove's own steps, so the ticket cannot be
   destroyed by anything the transaction owns; a `Committed` disposition is never
   reached without one; and — the load-bearing one — a classification run with NO
   WITNESS AND NO MANIFEST LEFT still settles forward on the ticket alone.

   THAT THIRD CONJUNCT IS ALSO WHERE `resultProven` GOES VACUOUS IN ITS SECOND
   HALF, and `README.md` says so: with the manifest released there is no recorded
   fingerprint to compare, so the ticket carries the whole proof.  That is the
   catalogue's own position — *the deletion commit's own message ... SHALL survive
   the destruction of every artifact the transaction owns* — and not a hole, but
   a reader could mistake the conjunction for a stronger test than it is at the
   exact moment it matters most. */
/* THE FIRST CONJUNCT'S ANTECEDENT WAS NARROWED BY THE REVALIDATION SLICE, AND
   IT IS THE THIRD TIME THIS CORPUS HAS LEARNED ONE RULE AT A NEW GRAIN.  It
   read `always Repo.tickets in Repo.tickets'` — history never shrinks, under
   ANY step, the world's included.  The claim it answers says something
   narrower: the ticket *SHALL survive the destruction of every artifact the
   transaction owns*.  The comment beneath it has said `under Grove's own steps`
   since `commit-k41`; the check said more than the comment, and more than the
   catalogue.

   WHAT THE OVER-STATEMENT COST WAS NOT VISIBLE UNTIL `FN-22`.  With history
   append-only under every step, `resultProven` is MONOTONE, so a `Committed`
   observation can never become anything else and the two `Committed` departures
   the catalogue's revalidation table is most careful to distinguish are
   unreachable BY CONSTRUCTION — a green `FN-22.f` and `FN-22.g` would have been
   fiction.  The narrowing is the same one `FN-19`'s third conjunct took after
   the `doSwap` counterexample, and the same rule the witness slice's first
   retained counterexample states about a free initial state: **a claim about
   what a protocol never does is never a claim about what the world never does.**
   This file now carries it at three grains — a free initial state, a world
   transition over the tree, and a world transition over history. */
check FN_03_the_ticket_is_the_durable_record_and_outlives_the_artifacts {
  always {
    (Sys.act' in txnActs) implies Repo.tickets in Repo.tickets'
    (Sys.act' = Classify and Txn.disp' = Committed)
      implies Txn.attempt in Txn.handle.(Repo.tickets)
    (Sys.act' = Classify and no Slot.occ and manEmpty
       and Txn.attempt in Txn.handle.(Repo.tickets))
      implies Txn.disp' = Committed
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

/* A retry with no local trace of the attempt, settling forward on the ticket
   alone.  The forward settle releases every artifact the transaction owns, and
   the classification that follows it reads an empty slot, an empty manifest and
   a ticket. */
run witness_FN_03_a_retry_with_no_local_trace_settling_forward_on_the_ticket_alone {
  interruptedMidEvacuation
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = MarkerRemove and Sys.res = Applied
              and no Slot.occ and manEmpty and no Cleanup.present and no Quar.qRid)
  eventually (Sys.act = Classify and Txn.disp = Committed
              and Txn.phase = Settled and no Slot.occ and manEmpty)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

// --- FN-04: an attempt binds to a live session ------------------------------

/* Two conjuncts.  A ticket that lands names EXACTLY this handle and this
   attempt — so history can never acquire a ticket for an attempt that did not
   commit it — and no classification reaches `Committed` without a ticket naming
   the attempt the live session is settling, however many tickets the handle
   already carries. */
check FN_04_a_ticket_from_an_earlier_attempt_never_settles_a_later_one {
  always {
    (some (Repo.tickets' - Repo.tickets))
      implies (Repo.tickets' - Repo.tickets) = (Txn.handle -> Txn.attempt)
    (Sys.act' = Classify and Txn.attempt not in Txn.handle.(Repo.tickets))
      implies Txn.disp' != Committed
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

/* Two attempts on one handle, the earlier ticket rejected by the later.  The
   earlier ticket is in history at state 0 — history is not the transaction's
   volatile state and `EN-11`'s licence over the tree has a counterpart here:
   a repository whose past a later launch did not write is exactly the situation
   the claim is about.  The recovery adopts the manifest's attempt, the earlier
   ticket names a different one, and the disposition is `NotCommitted` WITH a
   ticket for the handle sitting there unread. */
run witness_FN_04_two_attempts_on_one_handle_the_earlier_ticket_rejected {
  interruptedMidEvacuation
  some a: AttemptId | a != Man.mAttempt and Repo.tickets = Man.mHandle -> a
  eventually (Sys.act = CommitAttempt and Sys.res = Applied)
  eventually (Sys.act = Classify and Txn.disp = NotCommitted
              and some Txn.handle.(Repo.tickets))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

// --- FN-14: the commit is scoped --------------------------------------------

/* Whenever a ticket lands, exactly the recorded fingerprint left the tracked set
   and the unrelated working-copy work is untouched.  Stated over the ticket
   rather than over the step, so a future step that commits is caught by the same
   sentence. */
check FN_14_the_commit_records_exactly_the_expected_deletions {
  always ((some (Repo.tickets' - Repo.tickets)) implies commitIsScoped)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

/* Unrelated modified work present across a successful finish — present before
   the commit, present after it, and never part of what the deletion recorded. */
run witness_FN_14_unrelated_modified_work_present_across_a_successful_finish {
  interruptedMidEvacuation
  no Repo.tickets
  always some World.wcWork
  eventually (Sys.act = CommitAttempt and some Repo.tickets)
  eventually (Sys.act = Classify and Txn.disp = Committed and some World.wcWork)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

// --- FN-15: disposition is classified from evidence, not from exit status ----

/* FN-15.a.  The classification is a TOTAL FUNCTION of the two evidence
   predicates and of nothing else — stated as three biconditionals, so a
   classification that consulted `Txn.report` would have to break one of them.
   The reported result appears nowhere in this check, which is the point: the
   mutation that makes `doClassify` read it is what turns this from a definition
   into a control. */
check FN_15a_the_classification_is_a_function_of_the_evidence_and_not_of_the_report {
  always ((Sys.act' = Classify) implies {
    Txn.disp' = Committed     iff resultProven
    Txn.disp' = NotCommitted  iff (not resultProven and anchorHolds)
    Txn.disp' = Indeterminate iff (not resultProven and not anchorHolds)
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

/* The catalogue's witness: a lost or late result reported as failure while the
   exact commit exists — classified `Committed`.  The commit lands, the immediate
   result is LOST, the classification settles on the evidence, and the failure
   report arrives afterwards to find a disposition that never read it.  That
   late arrival is `EN-09`'s exercise, and the control at the foot of this file
   removes it. */
run witness_FN_15a_a_failure_reported_after_the_classification_over_an_exact_commit {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = CommitAttempt and some Repo.tickets and no Txn.report)
  eventually (Sys.act = Classify and Txn.disp = Committed and no Txn.report)
  eventually (Sys.act = ResultArrives and Txn.report = FailReport
              and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

/* FN-15.b .. FN-15.d are REACHABILITY obligations, so each one's check states
   the other half — what the disposition is reached ON.  A model in which
   `Committed` were reachable without a proven result would satisfy the witness
   and fail the check, which is the pair the catalogue asks for. */
check FN_15b_committed_is_reached_only_on_a_proven_result {
  always ((Sys.act' = Classify and Txn.disp' = Committed) implies resultProven)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* THE LANE IS PINNED FOR THE WHOLE TRACE IN EACH OF THE NINE COMMANDS BELOW,
   because *reached, on each lane* is three statements and Alloy has no way to
   parameterise a command.  `EN-16` — the collapse control that makes a
   lane-blind model visible — is the `exits` sibling's and runs over all of them.

   `witness_FN_15b_git` is the ONE command in this slice that runs the whole
   protocol from a fresh grove: `TxnOpen`, the preflight, the six body steps and
   the classification.  It is the file's widest command and it exists so that the
   other eight are demonstrably shortcuts of something, not of nothing. */
run witness_FN_15b_git_committed_reached_from_a_fresh_grove {
  always World.lane = GitL
  Txn.phase = Fresh and no Slot.occ and some Root.holds
  no Repo.tickets
  eventually (Sys.act = WPublish and Slot.occ = Published)
  eventually (Sys.act = CommitAttempt and Sys.res = Applied and some Repo.tickets)
  eventually (Sys.act = Classify and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_15b_nativejj_committed_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

run witness_FN_15b_colocatedjj_committed_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

check FN_15c_notcommitted_is_reached_only_with_the_anchor_intact_and_no_result {
  always ((Sys.act' = Classify and Txn.disp' = NotCommitted)
            implies (anchorHolds and not resultProven))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

run witness_FN_15c_git_notcommitted_reached {
  always World.lane = GitL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

run witness_FN_15c_nativejj_notcommitted_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

run witness_FN_15c_colocatedjj_notcommitted_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

/* FN-15.d.  `Indeterminate` IS REACHABLE, with a witness on each lane, so this
   file takes the catalogue's first branch and neither the bounded-unreachability
   check nor a `defer`.  What makes it reachable is exactly `EN-05`: the commit
   is outside the transaction, so between the attempt and the classification the
   world may move the repository, and a moved anchor with no ticket for this
   attempt is a state in which NEITHER outcome can be proven.  That is Q2's
   evidence and `formal-synthesis-k16` reads it as such. */
check FN_15d_indeterminate_is_reached_only_when_neither_outcome_can_be_proven {
  always ((Sys.act' = Classify and Txn.disp' = Indeterminate)
            implies (not anchorHolds and not resultProven))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_15d_git_indeterminate_reached {
  always World.lane = GitL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Classify and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_15d_nativejj_indeterminate_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Classify and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_15d_colocatedjj_indeterminate_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Classify and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

// --- FN-16: rollback is licensed only by proof ------------------------------

/* A RESTORATION IS IDENTIFIED STRUCTURALLY — entries coming back into the task
   root — rather than by the action that did it.  That is deliberate: the claim
   is about restoration, not about `Settle`, and a future step that put entries
   back by another route would be caught by the same sentence.  Nothing else in
   this file ever grows `Root.holds`. */
check FN_16a_restoration_is_refused_when_the_recorded_anchor_no_longer_holds {
  always ((some (Root.holds' - Root.holds)) implies anchorHolds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* Reached: the anchor has moved under the transaction, and the settle that
   follows restores nothing — it blocks, and the witness and the evacuated tree
   stay exactly where they were for a later recovery to read. */
run witness_FN_16a_a_settle_with_the_recorded_anchor_moved_restores_nothing {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Settle and Sys.res = BlockedOutcome
              and Sys.why = W12Indeterminate
              and no Root.holds and Slot.occ = Published)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

check FN_16b_restoration_is_refused_when_the_attempt_bound_result_is_present {
  always ((some (Root.holds' - Root.holds)) implies not resultProven)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* Reached: the attempt-bound result IS present — the ticket landed — so the
   settle goes forward and no entry ever comes back. */
run witness_FN_16b_a_settle_with_the_attempt_bound_result_present_restores_nothing {
  interruptedMidEvacuation
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = CommitAttempt and some Repo.tickets)
  /* THE FORWARD PATH NO LONGER *SETTLES*, IT DISPOSES.  What this witness needs
     is a forward step taken with the ticket present and nothing coming back, and
     the first of those is now the marker the disposal writes. */
  eventually (Sys.act = MarkerCreate and Sys.res = Applied and no Root.holds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-17: rollback is exact -----------------------------------------------

/* Two conjuncts, and the second is what *before the witness is removed* is worth
   in a model whose settle is one step: the removal is CONDITIONED on the
   reproduction, so a restoration that skipped it could not release the witness.
   Whether the step must itself be decomposed — one persistent effect per step —
   THE SECOND CONJUNCT IS NO LONGER A CONJUNCTION, AND THE REVALIDATION SLICE
   IS WHY.  `commit-k41` restored the tree, reproduced the commit and released
   the witness in ONE step, and could only state *before the witness is removed*
   as a condition rather than as an ordering; `README.md` recorded that as an
   abstraction and left the decomposition question to `FN-24.b`.  `FN-22`'s
   *after restoration* row forced the split — a table row about what is observed
   after the restoration needs a state after the restoration — so the removal is
   now a SEPARATE STEP, taken from a state in which the tree already matches the
   manifest and the commit has already been reproduced.  The conjunct below
   reads the UNPRIMED state for both, which is what makes it an ordering: the
   restoration happened in an earlier transition, and this one is licensed by
   what that transition left.  The abstraction is removed rather than restated. */
check FN_17a_a_restoration_matches_the_manifest_and_reproduces_the_preflight_commit {
  always {
    (some (Root.holds' - Root.holds))
      implies (treeMatchesManifest and preflightCommitReproduced)
    (Sys.act' = Revalidate and some Slot.occ and no Slot.occ')
      implies (Root.holds = Man.mEntries
               and ((World.lane in wcAsCommitLanes)
                      implies Repo.reproduced = Txn.anchor))
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* A restoration that reproduces it, on a working-copy-as-commit lane — which is
   the only obligation in this slice that reads the lane at all. */
run witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  always some Repo.canReproduce
  /* TWO STEPS NOW, AND THE WITNESS SAYS SO RATHER THAN HIDING IT.  The
     restoration puts the tree back and reproduces the exact preflight commit
     with the witness STILL STANDING; the release is a second step, licensed by
     what the first left.  That is `FN-17.a`'s *before the witness is removed*
     as an ordering, and it costs this witness one state. */
  eventually (Sys.act = Settle and Sys.res = Applied
              and some Root.holds and Slot.occ = Published
              and Repo.reproduced = Txn.anchor)
  eventually (Sys.act = Revalidate and Sys.res = RefRollbackNotCommitted
              and some Root.holds and no Slot.occ
              and Repo.reproduced = Txn.anchor)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* FN-17.b.  A restoration that cannot reproduce it BLOCKS rather than proceeds:
   the outcome is `Blocked`, and the tree and the repository are byte-identical,
   so the witness still stands and a later recovery has something to read. */
/* THE ANTECEDENT NARROWED WHEN THE REVALIDATION ARRIVED, AND THE NARROWING IS
   THE CLAIM GETTING SHARPER RATHER THAN WEAKER.  It read *a settle over a
   `NotCommitted` disposition*, which was the same set of steps while the
   recorded disposition was the only thing the settle could act on.  It is not
   the same set now: `FN-22`'s before-restoration row DIVERTS a settle whose
   recorded disposition is `NotCommitted` but whose fresh observation is
   `Committed`, and a divert restores nothing, so whether the exact preflight
   commit could be reproduced is not a question it asks.  `FN-17.b` is about a
   RESTORATION that cannot reproduce it, so the antecedent is now the
   observation that actually attempts one. */
check FN_17b_a_restoration_that_cannot_reproduce_it_blocks_rather_than_proceeds {
  always ((Sys.act' = Settle and Txn.disp = NotCommitted and observed = NotCommitted
             and World.lane in wcAsCommitLanes and no Repo.canReproduce)
            implies (Sys.res' = BlockedOutcome and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_17b_a_restoration_that_cannot_reproduce_it_blocks {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  always no Repo.canReproduce
  eventually (Sys.act = Settle and Sys.res = BlockedOutcome
              and Sys.why = W13CannotReproduce
              and Slot.occ = Published and no Root.holds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

// --- FN-18: forward recovery never restores ---------------------------------

/* Two conjuncts.  A forward step puts nothing back and empties the witness
   rather than unpacking it; and once THIS attempt's commit is proven, no later
   state in the trace ever grows the task root again.

   THE FIRST CONJUNCT MOVED FROM `Settle` TO `disposalSteps`, AND IT HAD TO.
   `commit-k41` and `quarantine-k43` both ran the forward path through
   `doSettle`, so *a forward settle with a `Committed` disposition* was where the
   claim bit.  The disposal slice takes the forward path out of `doSettle`
   entirely, and a conjunct still stated over `Sys.act' = Settle` would have gone
   VACUOUS — `doSettle`'s only remaining `Committed` branch is the
   before-restoration DIVERT, which runs with `Txn.disp = NotCommitted` in the
   unprimed state.  A vacuous conjunct reports exactly as a green one, and
   mutation 29 would have stopped firing without saying so.  Stated over
   disposal's four steps it bites where the forward path now is. */
check FN_18_a_proven_commit_is_never_followed_by_a_reconstruction {
  always {
    (Sys.act' in disposalSteps) implies rootSame
    (Sys.act' = Dispose and Sys.res' = Applied) implies no Slot.wHolds'
    (resultProven and once (Sys.act = Classify and Txn.disp = Committed))
      implies no (Root.holds' - Root.holds)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* A proven commit reached after an interruption mid-evacuation: the recovery
   adopts the interrupted attempt, finishes the evacuation, commits, proves it,
   and settles FORWARD — the tree is never reconstructed. */
run witness_FN_18_a_proven_commit_reached_after_an_interruption_mid_evacuation {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Recover and Txn.phase = PublishedP and some Root.holds)
  no Cleanup.present
  eventually (Sys.act = Classify and Txn.disp = Committed and no Root.holds)
  eventually (Sys.act = Dispose and Sys.res = Applied
              and no Root.holds and no Slot.occ and no Quar.qRid)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps


// ===========================================================================
// CLAIMS — FN-19, FN-20, THE HANDOFF
//
// The slice that first REMOVES THE TASK ROOT.  Every claim before this one
// could take the root's presence for granted; `evacuationComplete`'s *the task
// root is still present* and `gateEvacuated`'s silence about it have been
// written apart since `witness-k40` against exactly this day, and what that
// divergence turned out to be worth is recorded in `README.md`.
// ===========================================================================

// --- FN-19: the root moves in one atomic rename -----------------------------

/* Four conjuncts, and the last two are one claim stated the only way a claim
   about SHAPE can be stated under a free initial state.

   (a) and (b) are the rename itself: the quarantine acquires a root only by
   this step and never by any other, the identity leaves the task root in the
   SAME state it arrives in the quarantine, and nothing inside the root moves
   with it.  (b) is what *witness and evacuated tree intact* is worth here —
   a frame condition on the step rather than a list of equalities.

   (c) IS AN INVARIANT THE PROTOCOL PRESERVES, NOT A SHAPE CLAIM, AND ITS
   ANTECEDENT IS NARROWED TO GROVE'S OWN STEPS BECAUSE OF A COUNTEREXAMPLE.
   The witness slice's first retained counterexample is the rule: under `EN-11`
   cashed out as a free initial state, every "never" claim about tree shape is
   false unless it is restated over the transition relation, because state 0 can
   hand-edit the violation.  Written over EVERY step, (c) is still false, and
   `doSwap` is why: the world swapping the task root can put the QUARANTINE's
   own identity at the task-root path — which is what moving the quarantine
   directory back over `.grove/` looks like from the inside — and the model has
   no way to know the quarantine went with it.  That is the same lesson as the
   witness slice's, met from a new direction: the hand edit is a TRANSITION here
   rather than a free initial state, and a claim about the protocol's shape has
   to be stated over the protocol's own steps.  Retained in `README.md`.

   (d) is the other half — a task root never simply disappears under a
   transaction step; if it goes, the quarantine gained exactly it. */
check FN_19_the_root_moves_into_the_quarantine_in_one_atomic_rename {
  always {
    (some (Quar.qRid' - Quar.qRid))
      implies (Sys.act' = QuarRename
               and Quar.qRid' = Root.rid and no Root.rid')
    (Sys.act' = QuarRename and Sys.res' = Applied)
      implies (Root.holds' = Root.holds and slotSame and manSame)
    (Sys.act' in txnActs and no (Root.rid & Quar.qRid))
      implies (no (Root.rid' & Quar.qRid'))
    (Sys.act' in txnActs and some Root.rid and no Root.rid')
      implies Quar.qRid' = Root.rid
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* The catalogue's witness verbatim: an interruption immediately after the
   rename, leaving a complete quarantine and an absent task root.  It runs the
   whole forward path for real from the disk an interruption mid-evacuation
   leaves — recover, finish the evacuation, attempt the commit, land it,
   classify it `Committed`, rename — and then crashes, which is what makes
   *immediately after* a fact about the trace rather than about the predicate.

   WHAT IT LEAVES IS ALSO WHAT NOTHING IN THIS FILE CAN YET CLEAN UP, and that
   is recorded in `README.md` rather than fixed here: with the task root absent,
   `doTxnOpen` is unavailable, so the quarantine this crash leaves is disposed
   of by a REAPER and by nothing else — which is `FN-21`, and the `disposal`
   sibling's. */
run witness_FN_19_an_interruption_immediately_after_the_rename {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (Sys.act = QuarRename and Sys.res = Applied
              and no Root.rid and some Quar.qRid)
  eventually (Sys.act = Crash
              and no Root.rid                  // the task root is absent
              and some Quar.qRid               // the quarantine holds it
              and Slot.occ = Published         // the witness, intact
              and some Slot.wHolds             // the evacuated tree, intact
              and some Man.mReady)             // the manifest, intact
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-20: a leftover artifact is garbage, never a receipt ------------------

/* TWO CONJUNCTS, AND ONLY THE SECOND IS NEW IN THIS FILE — which is recorded
   here and in `README.md` rather than left for a reader to work out, because a
   cell filled by a check that adds nothing is the shape of a false-confidence
   incident.

   (a) PRESENCE IS NEVER SUFFICIENT.  Whatever the transaction has left lying
   about — a quarantine, a published witness, a written manifest — it never
   makes a finish provable.  In THIS file that is ENTAILED by `FN-04`'s second
   conjunct, which is stated without the leftover antecedent and is therefore
   strictly stronger.  It is restated here because `FN-20` quantifies over every
   artifact where `FN-04` quantifies over the ticket, and a candidate protocol
   that leaves something else behind is checked against the role by this
   conjunct and by nothing else in the file.

   (b) PRESENCE IS NEVER NECESSARY, AND NEVER OBSTRUCTS.  With a ticket naming
   this handle and this attempt, and the expected deletions gone, the
   classification reaches `Committed` — whatever is or is not still on disk.
   `FN-03`'s third conjunct reaches only the EMPTY-TREE half of that, because it
   carries `no Slot.occ and manEmpty` in its own antecedent; (b) drops that
   antecedent, so it is the half that says the transaction's own artifacts,
   still sitting there, are not allowed to WITHHOLD the answer either.

   THE MUTATION HAD TO BE RE-AIMED, AND THAT IS RECORDED IN `README.md` RATHER
   THAN QUIETLY FIXED.  The obvious mutation — `doClassify` refusing to reach
   `Committed` while a QUARANTINE exists — kills (b) and kills `FN-03` with it,
   because `FN-03`'s third conjunct says nothing about a quarantine.  A mutation
   that kills its target and a neighbour has not isolated what the target
   uniquely says.  The one that does is `doClassify` requiring the WITNESS to be
   gone, which `FN-03`'s antecedent already assumes and (b)'s does not.

   THE STRONGEST FORM OF `FN-20` IS NOT STATED HERE AND CANNOT BE.  *No
   classification reads the quarantine* is non-interference: two traces
   differing only in the leftover reach the same disposition.  Alloy quantifies
   over traces one at a time, so the property is inexpressible as a check and
   (a) and (b) are the reachable approximation.  `README.md` carries it under
   what a green run does not prove. */
check FN_20_no_artifact_the_transaction_leaves_behind_is_ever_a_receipt {
  always {
    (Sys.act' = Classify and leftoverArtifact
       and Txn.attempt not in Txn.handle.(Repo.tickets))
      implies Txn.disp' != Committed
    (Sys.act' = Classify and Txn.attempt in Txn.handle.(Repo.tickets)
       and no (Man.mFinger & Repo.tracked))
      implies Txn.disp' = Committed
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

/* The catalogue's witness: a leftover artifact present while the tree is
   classified fresh.  An earlier attempt's quarantine sits beside a task root
   that is NOT the one inside it — two `RootId` atoms is exactly what makes the
   two distinguishable, and at one atom the witness would be inexpressible
   rather than false — and the classification of the live attempt reads the
   ticket, finds none, and settles on `NotCommitted`.  The leftover is garbage. */
run witness_FN_20_a_leftover_artifact_present_while_the_tree_is_classified_fresh {
  interruptedMidEvacuation
  some Quar.qRid and Quar.qRid != Root.rid
  no Repo.tickets
  eventually (Sys.act = Classify
              and some Quar.qRid                // the leftover, present
              and Slot.occ = Published          // and this attempt's own, too
              and Txn.disp = NotCommitted)      // and the tree classifies fresh
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps


// ===========================================================================
// CLAIMS — FN-22, THE FOUR REVALIDATION POINTS AND THE TEN-ROW TABLE
//
// The largest single claim group in the finish scope, and the first in this
// file whose subject is a TABLE rather than a sentence.  The machinery it is
// stated over — `observed`, `atRevPoint`, `tableAction`, `tableOutcome` — is at
// the head of this file, written apart from every transition that acts on it,
// for the reason recorded there: a per-row mutation kills a row and nothing
// kills *the table is complete*, so the completeness has to be carried by the
// table's own totality plus a witness per row.
//
// WHERE THESE WITNESSES START, AND THE ONE THING THIS SLICE CHECKED THAT NO
// EARLIER ONE COULD.  Fifteen of `commit-k41`'s eighteen witnesses start from
// `interruptedMidEvacuation`, a POSITED disk written to be exactly what the six
// body steps plus a `crash` produce — and never checked to be.  `README.md`
// carried that as a limit and named this leaf's rows as the check.  It is
// checked below, by `witness_FN_22a_the_posited_recovery_disk_is_reachable`,
// which runs the whole body from a fresh grove, crashes mid-evacuation and
// reconfirms: THE DISK IS REACHABLE, and it first lands at ELEVEN states —
// one above the ceiling every earlier slice ran under, which is why no earlier
// slice could have found it.  Every witness resting on the predicate is
// therefore testifying about a state an execution reaches.
// ===========================================================================

// --- FN-22.a: all four points are performed, and none is skipped ------------

/* FOUR CONJUNCTS, AND THE FIRST IS THE ONE THE LEAF'S BRIEF ASKS FOR: *a table
   with a missing row is a counterexample, not a silence*.

   (i) binds every Grove step taken AT a revalidation point to the table's own
   corrective action and outcome.  `tableAction` is TOTAL over four points and
   three dispositions, so a row deleted from it makes the function partial,
   `Sys.act' = tableAction[..]` false wherever that combination is reached, and
   this check red.  That is the difference between a table and a list of
   branches: the branches are the transitions, the table is data, and (i) is
   what makes disagreeing with the data a counterexample.  What (i) CANNOT do is
   notice a combination that is reachable in the world but enables no Grove step
   at all — that is a silence, and the ten witnesses below are what fill it.

   (ii) and (iii) are *none is skipped*, stated over the two handoffs and their
   two completions rather than over the actions that perform them.  A
   restoration only ever happens at the before-restoration point; the rename
   only at the before-rename point; the quarantine is only ever emptied — by
   disposal or by a return — at the after-rename point; and the witness is only
   ever released, on the rollback path, at the after-restoration point.  The
   second of those is entailed by `FN-19`'s first conjunct together with
   `doQuarRename`'s guard, and is restated here because `FN-22` quantifies over
   the POINT where `FN-19` quantifies over the action — the same relationship
   `FN-20`'s first conjunct has to `FN-04`'s second. */
check FN_22a_all_four_revalidation_points_are_performed_and_none_is_skipped {
  always {
    (Sys.act' in txnActs and some currentPoint) implies {
      Sys.act' = tableAction[currentPoint, observed]
      Sys.res' = tableOutcome[currentPoint, observed]
    }
    (some (Root.holds' - Root.holds)) implies atRevPoint[BeforeRestore]
    (some (Quar.qRid' - Quar.qRid))   implies atRevPoint[BeforeRename]
    /* THE TWO COMPLETION CONJUNCTS MOVED WITH THE DISPOSAL, AND THE THIRD IS
       WHAT KEEPS THEM AS STRONG AS THEY WERE.  Until the disposal slice, the
       quarantine was emptied and the witness released by the forward settle
       AT the after-rename point, in one step.  Disposal is now three steps and
       the last two run at `Disposing` and `Disposed`, which are not revalidation
       points — so a conjunct still requiring `atRevPoint[AfterRename]` would be
       false of a protocol that does exactly what the catalogue asks.

       The strength is restored by the third conjunct rather than surrendered:
       DISPOSAL ONLY EVER BEGINS AT THE AFTER-RENAME POINT.  `Disposing` is
       entered by `doMarkerCreate` and `doMarkerReplace` and by nothing else, and
       both are guarded by `Txn.phase = Quarantined` with `observed = Committed`
       — so *the quarantine is emptied only by a disposal that revalidated first*
       is carried by the pair, exactly as before.

       BOTH ANTECEDENTS GAIN `Sys.act' in txnActs`, WHICH EXCLUDES THE REAPER,
       AND THAT IS NOT A WEAKENING SLIPPED IN.  `FN-22` is *the disposition is
       revalidated across every handoff*: the reaper is not a handoff and never
       had a disposition to revalidate — it is a sweep over what a crashed
       transaction left, which is `FN-21`'s subject and is checked there.  The
       corpus's oldest rule in this file says the same thing from the other
       side: a claim about what a transaction never does is never a claim about
       what every actor never does. */
    (Sys.act' in txnActs and some (Quar.qRid - Quar.qRid'))
      implies (atRevPoint[AfterRename] or Txn.phase = Disposing)
    (Sys.act' in txnActs and some Slot.occ and no Slot.occ')
      implies (atRevPoint[AfterRestore] or Txn.phase = Disposing)
    (Txn.phase != Disposing and Txn.phase' = Disposing)
      implies atRevPoint[AfterRename]
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* THE FOUR POINTS, REACHED — the catalogue's own witness for `FN-22.a`, one
   command each so that a point no execution reaches is a missing instance
   rather than a quiet conjunct. */
run witness_FN_22a_the_point_before_the_restoration_is_reached {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (atRevPoint[BeforeRestore] and Sys.act' = Settle)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_22a_the_point_after_the_restoration_is_reached {
  interruptedMidEvacuation
  no Repo.tickets
  always some Repo.canReproduce
  eventually (atRevPoint[AfterRestore] and Sys.act' = Revalidate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_22a_the_point_before_the_quarantine_rename_is_reached {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (atRevPoint[BeforeRename] and Sys.act' = QuarRename)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_22a_the_point_after_the_quarantine_rename_is_reached {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  no Cleanup.present
  eventually (atRevPoint[AfterRename] and Sys.act' = MarkerCreate and Sys.res' = Applied)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* THE DEBT `commit-k41` TOOK ON, PAID.  Fifteen of that slice's witnesses — and
   every witness in this file that starts from `interruptedMidEvacuation` — begin
   from a disk that was WRITTEN to be what the body plus a crash produce and was
   not checked to be one.  `README.md` recorded it as a limit and named these ten
   rows as the check; the honest form of that check is simply to run the body up
   to the disk, which nothing in the file had done.

   It runs `TxnOpen`, the preflight, all six body steps with a partial
   evacuation, a `crash`, and the confirmation a later launch supplies — nine
   transitions — and then asserts the predicate itself.  It FIRST LANDS AT
   ELEVEN STATES and finds nothing at ten, which is why no earlier slice could
   have run it: ten was the ceiling from `witness-k40` onward.

   It is filed under `FN-22.a` because that is the obligation whose subject is
   *the four points are performed* — a point performed over a disk no execution
   reaches is not performed at all — and because the catalogue named this
   leaf's rows as the check for it.  `README.md` records the filing. */
run witness_FN_22a_the_posited_recovery_disk_is_reachable {
  Txn.phase = Fresh and no Slot.occ and no Quar.qRid and no Repo.tickets
  eventually (Sys.act = Crash and Slot.occ = Published and some Slot.wHolds)
  eventually (Sys.act = Confirm and interruptedMidEvacuation)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-22.b: before restoration, Committed diverts and restores nothing -----

/* The corrective action is *do not restore; take the forward path*, and the
   check states both halves: nothing is restored and nothing else moves, and the
   transaction is left at the OTHER handoff's point rather than ended.  The
   second half is what makes it a divert rather than a refusal. */
check FN_22b_before_the_restoration_committed_diverts_and_restores_nothing {
  always ((Sys.act' in txnActs and atRevPoint[BeforeRestore] and observed = Committed)
    implies (treeSame and repoSame and Sys.res' = Applied
             and Txn.phase' = Classified and Txn.disp' = Committed))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* THE COMMIT LANDS LATE, AFTER A CLASSIFICATION THAT GAVE UP ON IT.  The attempt
   is made, nothing lands, the classification reads `NotCommitted`, and then this
   attempt's own commit arrives in history — which `EN-09` grants and
   `doCommitMoves` is.  The restoration is one step away and does not happen. */
run witness_FN_22b_a_late_landing_observed_before_the_restoration {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
  eventually (Sys.act = CommitMoves and some Repo.tickets)
  eventually (atRevPoint[BeforeRestore] and observed = Committed
              and Sys.act' = Settle and Txn.disp' = Committed
              and no Root.holds and no Root.holds')
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-22.c: after restoration, Committed leaves the witness blocking -------

/* The tree is back and history says the finish happened, so releasing the
   witness would leave a recovery nothing to read.  Three things are checked:
   the outcome is a block, the witness is still published over the restored
   tree, and nothing at all was mutated by the observation. */
check FN_22c_after_the_restoration_committed_leaves_the_witness_blocking {
  always ((Sys.act' in txnActs and atRevPoint[AfterRestore] and observed = Committed)
    implies (Sys.res' = BlockedOutcome and Sys.why' = W15CommittedAfterRestore
             and Slot.occ' = Published and Root.holds' = Man.mEntries
             and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_22c_a_late_landing_observed_after_the_restoration {
  interruptedMidEvacuation
  no Repo.tickets
  always some Repo.canReproduce
  eventually (Sys.act = Settle and Txn.phase = Restored)
  eventually (Sys.act = CommitMoves and some Repo.tickets)
  eventually (Sys.act = Revalidate and Sys.res = BlockedOutcome
              and Sys.why = W15CommittedAfterRestore
              and Slot.occ = Published and some Root.holds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-22.d: after restoration, unchanged NotCommitted completes as Refused -

/* The catalogue's stable state: the task root present and matching the
   manifest, the witness gone and the manifest gone — a refusal is a complete
   outcome (`FN-29`).  `Man.mEntries` is read UNPRIMED because the manifest is
   released by this very step.

   *AND THE FINISH LEAF LIVE* IS NOT A CONJUNCT HERE, AND THE OMISSION IS A
   FIFTH INSTANCE OF THIS CORPUS'S OLDEST RULE RATHER THAN A GAP.  Written as
   `one finishLiveNext`, it has a counterexample: `EN-11` is cashed out as a free
   initial state, so state 0 may hand-edit a manifest that records no live finish
   leaf — or two — and a restoration that puts back exactly what such a manifest
   recorded leaves no leaf live.  No protocol step produced that manifest, and
   `Root.holds' = Man.mEntries` already says everything this check can say about
   the tree; a conjunct conditioned on the manifest being a real one would be
   `FN-12.a` restated under `FN-22`'s name and true by arithmetic.  So the live
   leaf is demonstrated where it means something — in the WITNESS, over a disk
   this file now checks is reachable — and `README.md` records the division.
   The rule is the witness slice's first retained counterexample at a new grain:
   a shape claim under a free initial state is a claim about what the protocol
   DOES, and this one is `doWManifest`'s. */
check FN_22d_after_the_restoration_an_unchanged_notcommitted_refuses_completely {
  always ((Sys.act' in txnActs and atRevPoint[AfterRestore]
           and observed = NotCommitted and reproductionStands)
    implies (Sys.res' = RefRollbackNotCommitted
             and Root.holds' = Man.mEntries and some Root.rid'
             and no Slot.occ' and manEmptyNext))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_22d_a_rollback_that_completes_as_a_refusal {
  interruptedMidEvacuation
  no Repo.tickets
  always some Repo.canReproduce
  eventually (Sys.act = Settle and Sys.res = Applied and Txn.phase = Restored
              and Slot.occ = Published and some Root.holds)
  eventually (Sys.act = Revalidate and Sys.res = RefRollbackNotCommitted
              and no Slot.occ and manEmpty and some Root.rid
              and one finishLive)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-22.e: before the rename, NotCommitted diverts to the restoration -----

check FN_22e_before_the_rename_notcommitted_diverts_and_renames_nothing {
  always ((Sys.act' in txnActs and atRevPoint[BeforeRename] and observed = NotCommitted)
    implies (treeSame and repoSame and Sys.res' = Applied
             and Txn.phase' = Classified and Txn.disp' = NotCommitted))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* THE COMMIT IS UNDONE BETWEEN THE CLASSIFICATION AND THE RENAME — an operator's
   `jj undo` in another terminal, which `EN-11` grants over the repository the
   same way it grants a hand edit over the tree.  The repository goes back to the
   recorded anchor, so what is observed is `NotCommitted` and the rename does not
   happen. */
run witness_FN_22e_an_undone_commit_observed_before_the_rename {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = Committed)
  eventually (Sys.act = CommitMoves and no Repo.tickets and Repo.rev = Txn.anchor)
  eventually (atRevPoint[BeforeRename] and observed = NotCommitted
              and Sys.act' = QuarRename
              and Txn.disp' = NotCommitted and no Quar.qRid')
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-22.f: after the rename, Committed -> NotCommitted returns ------------

/* TWO CONJUNCTS, AND THE FIRST IS THE ATOMICITY AND THE EXACTNESS TOGETHER.  A
   successful return is one move of the identity back — the same shape as the
   rename read backwards — and everything the root holds is FRAMED, which is
   what *byte-equal to the pre-rename tree* is worth in a model whose quarantine
   is a second place a root can be.  The second conjunct is the departure:
   under `NotCommitted` the return lands at the before-restoration point, which
   is the state *from which the restoration path runs*, and the attempt
   completes as `Refused` by `FN-22.d`'s row. */
check FN_22f_a_successful_return_restores_the_exact_pre_rename_state {
  always ((Sys.act' = QuarReturn and Sys.res' = Applied) implies {
    Root.rid' = Quar.qRid and no Quar.qRid'
    Root.holds' = Root.holds and slotSame and manSame and repoSame and worldSame
    Txn.phase' = Classified and Txn.disp' = observed
    (observed = NotCommitted) implies after atRevPoint[BeforeRestore]
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_22f_a_committed_becomes_notcommitted_after_the_rename {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (Sys.act = QuarRename and Sys.res = Applied and no Root.rid)
  eventually (Sys.act = CommitMoves and no Repo.tickets and Repo.rev = Txn.anchor)
  eventually (Sys.act = QuarReturn and Sys.res = Applied
              and some Root.rid and no Quar.qRid
              and Slot.occ = Published and some Man.mReady
              and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-22.g: after the rename, Committed -> Indeterminate returns and blocks -

/* TWO CONJUNCTS, AND THE SECOND IS THE WHOLE POINT OF SEPARATING THE TWO
   DEPARTURES.  The return is the same atomic move either way; what differs is
   the state it lands in.  Under `Indeterminate` it lands with a disposition no
   handoff acts on, and the attempt BLOCKS with the witness still standing —
   never the refusal `FN-22.f` ends in.  Collapsing the two would let a block be
   reported as a refusal, which is exactly the distinction `FN-29` requires the
   operator to be able to make. */
check FN_22g_after_the_rename_indeterminate_returns_and_then_blocks {
  always {
    /* THE ATOMICITY AND THE EXACTNESS OF THE RETURN ARE `FN-22.f`'s AND ARE NOT
       RESTATED HERE.  Stating `Root.rid' = Quar.qRid and no Quar.qRid'` in both
       places made a mutation aimed at the return's exactness kill this
       obligation too, which is the third way a mutation fails its aim and is
       recorded in `README.md`.  What is `.g`'s alone is the DEPARTURE: the
       witness still stands and the disposition the return lands with is
       `Indeterminate`, and the state it lands in blocks. */
    (Sys.act' = QuarReturn and Sys.res' = Applied and observed = Indeterminate)
      implies (Slot.occ' = Published and Txn.disp' = Indeterminate)
    (Sys.act' = Settle and Txn.phase = Classified and Txn.disp = Indeterminate)
      implies (Sys.res' = BlockedOutcome and Sys.res' not in Refused
               and Slot.occ' = Published and treeSame and repoSame)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_22g_a_committed_becomes_indeterminate_after_the_rename {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (Sys.act = QuarRename and Sys.res = Applied and no Root.rid)
  eventually (Sys.act = CommitMoves and no Repo.tickets and Repo.rev != Txn.anchor)
  eventually (Sys.act = QuarReturn and Sys.res = Applied
              and some Root.rid and no Quar.qRid
              and Slot.occ = Published
              and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-22.h: a return that cannot complete reports both and blocks ----------

/* *Report both the change and the quarantine, both named in the diagnostic* has
   no diagnostic text in a model with no strings, so what is checked is that
   both are OBSERVABLE in the state the attempt ends in: the quarantine still
   holds the root, the witness still stands over the reserved name, and the
   observation that differs from the recorded disposition is still what
   `observed` returns.  `README.md` records the abstraction. */
check FN_22h_a_return_that_cannot_complete_reports_both_and_blocks {
  always ((Sys.act' = QuarReturn and Sys.res' != Applied) implies {
    Sys.res' = BlockedOutcome and Sys.why' = W16ReturnIncomplete
    some Quar.qRid' and Slot.occ' = Published
    observed != Committed and Txn.disp = Committed
    treeSame and repoSame
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

run witness_FN_22h_the_task_root_name_taken_while_the_quarantine_holds_the_root {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (Sys.act = QuarRename and Sys.res = Applied and no Root.rid)
  eventually (Sys.act = RootNameTaken and some Root.rid and some Quar.qRid)
  eventually (Sys.act = QuarReturn and Sys.res = BlockedOutcome
              and Sys.why = W16ReturnIncomplete
              and some Quar.qRid and some Root.rid and Slot.occ = Published)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

// --- FN-22.i: after the rename, an unchanged Committed disposes --------------

/* The catalogue's stable state is *task root `Absent`, quarantine holding the
   root* and the corrective action is *complete: dispose (`FN-21`)*.

   THIS ROW IS WHERE THE DISPOSAL SLICE ENTERS THE FILE, AND THE CHECK CHANGED
   SHAPE BECAUSE THE PROTOCOL DID.  `revalidation-k44` could check that the
   quarantine was emptied by the very step the point enabled, because the
   forward settle passed through the catalogue's stable state and out of it in
   ONE transition.  `FN-21.a` is what turns that one step into three, so what
   the point now hands off to is disposal's FIRST step, and *the quarantine is
   emptied* is `FN-21`'s to carry rather than this row's.  What is left here is
   what the row is actually about: the corrective action taken is the one the
   TABLE names, the stable state it is taken from is the catalogue's — the
   quarantine holding a root — and the TASK ROOT IS LEFT EXACTLY AS THE PROTOCOL
   LEFT IT.

   `rootSame` rather than `no Root.rid'` because `doRootNameTaken` lets the WORLD
   put something at the task-root name while the quarantine holds the root, and
   the catalogue's *task root `Absent`* is a statement about what the rename
   left, not a promise about what the world does next; `FN-19`'s fourth conjunct
   already carries the former.

   THE FOREIGN-MARKER ARM IS NAMED HERE RATHER THAN LEFT TO `tableOutcome`,
   because the row's own words are *complete: dispose* and a disposal that
   cannot prove the document at the reserved name is Grove's does not complete.
   `FN-31.d` is what that arm is checked by; this conjunct only says the row
   knows about it.

   AND WHAT THE MARKER ITSELF CONTAINS IS NOT STATED HERE, DELIBERATELY.  A
   first form of this check also required exactly one marker afterwards, naming
   this quarantine.  It is true, and it is `FN-31.b`'s and `FN-21.b`'s — and
   stating it here made two of this slice's seven mutations kill a neighbour,
   which is the third way a mutation fails its aim.  The row is about the
   corrective action and the stable state it is taken from; the document is
   `FN-31`'s subject. */
check FN_22i_after_the_rename_an_unchanged_committed_disposes_and_applies {
  always ((Sys.act' in txnActs and atRevPoint[AfterRename] and observed = Committed)
    implies (Sys.act' in (MarkerCreate + MarkerReplace)
             and some Quar.qRid and rootSame
             and (markerForeign implies Sys.res' = BlockedOutcome
                  else (Sys.res' = Applied and Txn.phase' = Disposing))))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_22i_an_unchanged_committed_disposes_after_the_rename {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = QuarRename and Sys.res = Applied
              and no Root.rid and some Quar.qRid)
  eventually (atRevPoint[AfterRename] and observed = Committed
              and Sys.act' = MarkerCreate and Sys.res' = Applied
              and one Cleanup.present')
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

// --- FN-22.j: Indeterminate away from the rename performs no handoff ---------

/* One check over the three remaining points and a witness at each, which is
   what *reached at each remaining point* asks for.  *Performs no handoff* is
   `treeSame`: nothing is restored, nothing is renamed, nothing is released. */
check FN_22j_indeterminate_away_from_the_rename_performs_no_handoff_and_blocks {
  always ((Sys.act' in txnActs and observed = Indeterminate
           and (atRevPoint[BeforeRestore] or atRevPoint[AfterRestore]
                or atRevPoint[BeforeRename]))
    implies (Sys.res' = BlockedOutcome and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_22j_indeterminate_observed_before_the_restoration {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
  eventually (Sys.act = TopologyChange and Txn.phase = Classified
              and Repo.rev != Txn.anchor)
  eventually (atRevPoint[BeforeRestore] and observed = Indeterminate
              and Sys.act' = Settle and Sys.res' = BlockedOutcome)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_22j_indeterminate_observed_before_the_rename {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = Committed)
  eventually (Sys.act = CommitMoves and no Repo.tickets and Repo.rev != Txn.anchor)
  eventually (atRevPoint[BeforeRename] and observed = Indeterminate
              and Sys.act' = QuarRename and Sys.res' = BlockedOutcome
              and no Quar.qRid')
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_22j_indeterminate_observed_after_the_restoration {
  interruptedMidEvacuation
  no Repo.tickets
  always some Repo.canReproduce
  eventually (Sys.act = Settle and Sys.res = Applied and Txn.phase = Restored)
  eventually (Sys.act = TopologyChange and Txn.phase = Restored
              and Repo.rev != Txn.anchor)
  eventually (atRevPoint[AfterRestore] and observed = Indeterminate
              and Sys.act' = Revalidate and Sys.res' = BlockedOutcome
              and Slot.occ = Published and some Root.holds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps




// ===========================================================================
// CLAIMS — FN-21 AND FN-31, DISPOSAL
//
// The last claim group of the handoff subtree, and the first in this file whose
// subject is not a transaction at all.  Two things about where its witnesses
// start are worth reading before the commands.
//
// WHERE THESE WITNESSES START, AND WHY MOST OF THEM ARE SHORT.  The reaper runs
// at `Txn.phase = Fresh`, which is where a trace starts, so a sweep witness
// needs no run-up: the disk is posited — the same `EN-11` licence every witness
// in this file rests on, covering the TREE at `Fresh` — and the sweep runs in
// two or three states.  That is the opposite shape from `FN-22`'s, whose ten
// rows each needed nine transitions of protocol in front of them.
//
// AND THE TWO POSITED DISKS ARE CHECKED REACHABLE RATHER THAN ASSERTED, WHICH
// IS THE ONE PIECE OF METHOD THIS SLICE INHERITED AND DID NOT WANT TO PAY
// AGAIN.  `commit-k41` posited `interruptedMidEvacuation`, recorded its
// unchecked reachability as a limit, and `revalidation-k44` discharged it three
// slices later with a witness that simply ran the body up to it.  The lesson
// that cost three slices is that the honest instrument was a WITNESS and the
// thing preventing it was a BOUND.  So `interruptedMidDisposal` and
// `staleMarkerLeftBehind` each get one, here, in the slice that posits them —
// and `staleMarkerLeftBehind`'s is not optional at any price, because
// `FN-31.a`'s whole answer to `TODO.finish_process.md` Q3 rests on that disk
// being one an execution reaches.
// ===========================================================================

/* THE DISK AN INTERRUPTION MID-DISPOSAL LEAVES: the task root gone into the
   quarantine, the quarantine still holding it, the published witness and its
   ready manifest still inside it, and the cleanup marker written and naming
   both.  Nothing volatile — the transaction is `Fresh` and the confirmation
   went with it, which is what `doCrash` produces and what a later sweep reads. */
pred interruptedMidDisposal {
  Txn.phase = Fresh
  no Op.confirmed
  some World.lane and World.rootDev = World.qDev and World.wtDev = World.qDev
  no Root.rid                    // the task root left in the rename
  no Root.holds
  some Quar.qRid                 // the quarantine holds it
  Slot.occ = Published           // the witness, intact, inside the quarantine
  some Slot.owner
  some Man.mReady
  no Repo.wTracked
  // the marker the interrupted disposal wrote: provably Grove's, naming the
  // attempt that wrote it and the root it authorises removing
  one Cleanup.present
  Cleanup.present.cOwner = Slot.owner
  Cleanup.present.cTarget = Quar.qRid
}

/* THE DISK AN INTERRUPTION BETWEEN THE CONTENT'S REMOVAL AND THE MARKER'S
   LEAVES, and it is the source state `FN-31.a` is about: a marker Grove can
   prove is its own, standing at the reserved name, whose target is gone.  The
   quarantine it authorised removing has been removed; the document saying so
   has not.  A sweep collects it.  A NEW attempt that reaches the after-rename
   point first must SUPERSEDE it, which is the transition Q3 asks about. */
pred staleMarkerLeftBehind {
  Txn.phase = Fresh
  no Quar.qRid
  one Cleanup.present
  some Cleanup.present.cOwner
  Cleanup.present.cTarget != Root.rid
}

// --- FN-21.a: disposal is re-enterable from any interruption ----------------

/* THREE CONJUNCTS, AND THEY ARE THE THREE THINGS RE-ENTERABILITY IS MADE OF —
   the evidence survives, the evidence is not retired early, and every
   resumption lands in the same place.

   (a) THE EVIDENCE SURVIVES THE INTERRUPTION.  A crash at either of disposal's
   two points leaves a marker, and one Grove can prove is its own.  Without it
   the next launch meets a quarantine it cannot classify, which the catalogue
   sends to `FN-21.c` — declined, mutating nothing, for ever.

   (b) THE EVIDENCE IS NOT RETIRED BEFORE THE WORK IT AUTHORISES.  No disposal
   step removes the marker while the quarantine it names is still there.  This is
   the manifest's ready mark at a second grain: the document is the record that
   the work has NOT been done, so it goes last.  Written as an ordering on the
   step rather than as a phase clause, because a phase clause would make the
   mutation unsatisfiable — the trap this file has recorded twice.

   (c) EVERY RESUMPTION REACHES THE SAME TERMINAL STATE.  Whatever retires the
   marker — Grove's own third step or the reaper's second firing — leaves
   exactly the terminal state and not a partial one.  Together with (a) and (b)
   this is what *reaches the same terminal state as an uninterrupted disposal*
   is worth as a safety property; that a resumption EXISTS is a reachability
   question and is the witness's. */
check FN_21a_disposal_is_re_enterable_from_any_interruption {
  always {
    (Sys.act' = Crash and Txn.phase in (Disposing + Disposed))
      implies (some Cleanup.present' and no m: Cleanup.present' | no m.cOwner)
    (Sys.act' in disposalSteps and some Cleanup.present and no Cleanup.present')
      implies (no Quar.qRid and no Quar.qRid')
    ((Sys.act' in disposalSteps or Sys.act' = Reap)
      and Sys.res' = Applied and no Cleanup.present')
      implies disposalTerminalNext
    /* (d) AND THE REMOVAL THE MARKER AUTHORISES IS THE WHOLE OF IT.  Whichever
       quarantine goes and everything inside it goes with it, in one move.
       Stated over the step rather than over the terminal state, because a
       witness standing at the reserved name that is NOT inside the quarantine
       is `FN-10`'s subject and not disposal's; see the note on
       `disposalTerminalNext`.

       IT IS GROVE'S OWN REMOVAL AND NOT THE SWEEP'S, and the division is a
       mutation-isolation decision recorded as one.  The sweep does the same
       thing in the same order and `FN-31.c`'s second conjunct is where that is
       stated; written over both actors here, a mutation aimed at either would
       take down the other obligation as well, which is the third way a mutation
       fails its aim. */
    (Sys.act' = Dispose and Sys.res' = Applied)
      implies (no Quar.qRid' and no Slot.occ' and no Slot.wHolds' and manEmptyNext)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* THE CATALOGUE'S WITNESS: A RESUMED DISPOSAL.  It resumes from the FIRST of
   disposal's two interruption points — the marker written, the quarantine still
   standing — and runs the sweep twice: once to remove what the marker
   authorises, once to retire the marker.  The terminal state it reaches is the
   one an uninterrupted disposal reaches. */
run witness_FN_21a_a_disposal_interrupted_mid_disposal_and_resumed {
  interruptedMidDisposal
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Quar.qRid and no Slot.occ and manEmpty and some Cleanup.present)
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Quar.qRid and no Cleanup.present and no Slot.occ and manEmpty)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* AND THE DISK IT RESUMES FROM IS REACHED RATHER THAN POSITED.  It runs the
   whole forward path for real from the disk an interruption mid-evacuation
   leaves — recover, finish the evacuation, attempt the commit, land it, classify
   it `Committed`, rename, write the marker — and then crashes, which is what
   makes *mid-disposal* a fact about the trace rather than about the predicate.
   This is `witness_FN_19`'s trace with disposal's first step on the end. */
run witness_FN_21a_the_interrupted_disposal_disk_is_reachable {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = MarkerCreate and Sys.res = Applied)
  eventually (Sys.act = Crash and interruptedMidDisposal)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-21.b: the reaper touches only Grove's own, and only when unowned -----

/* THE `pre*`/`gate*` DISCIPLINE AT A THIRD GATE.  `reapable` is what the SWEEP
   reads; the three conjuncts below are what the CATALOGUE requires, written
   apart so a divergence is a counterexample rather than a definition — exactly
   as the seven preconditions are written apart from the six gates.

   The fourth conjunct is *mutating nothing* for the declining case, which the
   catalogue states of both `FN-21.b`'s and `FN-21.c`'s declines and which is
   what makes a sweep safe to run against a disk it does not understand. */
check FN_21b_the_reaper_touches_only_groves_own_and_only_when_unowned {
  always {
    (Sys.act' = Reap and Sys.res' = Applied) implies {
      not markerForeign
      some Quar.qRid implies markerAuthorises
      not inTreeWitnessOwns
    }
    (Sys.act' = Reap and Sys.res' != Applied) implies (treeSame and repoSame)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 5 steps

/* THE CATALOGUE'S WITNESS: A REAPER DECLINING AN ENTRY WHOSE IN-TREE WITNESS
   STILL OWNS IT.  A quarantine carrying Grove's own cleanup manifest — the
   sweep can prove every part of it — beside a published witness at the reserved
   name naming the same attempt.  The attempt is not finished with it: a
   revalidation after the rename may still RETURN the quarantine (`FN-22.f`), and
   a sweep that had eaten it would have made the return impossible.  So the
   sweep declines, and mutates nothing. */
run witness_FN_21b_a_reaper_declining_an_entry_its_in_tree_witness_still_owns {
  Txn.phase = Fresh
  some Root.rid
  Slot.occ = Published and some Slot.owner
  some Quar.qRid
  one Cleanup.present
  Cleanup.present.cOwner = Slot.owner
  Cleanup.present.cTarget = Quar.qRid
  eventually (Sys.act = Reap and Sys.res = NoOp
              and Sys.why = W17OwnershipConflict
              and some Quar.qRid and one Cleanup.present
              and Slot.occ = Published)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

// --- FN-21.c: a foreign entry at a reserved name is declined ----------------

/* THE SWEEP'S CONTEXT, UNDER THE `FN-` PREFIX THAT OWNS IT — this obligation is
   where `TT-24.d` landed, and the catalogue's three-context table is what fixes
   the outcome.  An ordinary tree operation refuses (`TT-24.b`); a transaction
   stops without moving the artifact (`FN-32`); the reaper *declines the entry,
   mutating nothing, and reports it*.  It is neither a refusal nor a block, because nothing was
   entered, and `NoOp` is the outcome this file already gives an action that
   reports and mutates nothing.

   THE `why` IS THE CATALOGUE'S OWN WORD.  `OwnershipConflict` is one half of
   `FN-25`'s partition and `FN-25` is `exits`'; naming it as a `Sys.why` member
   rather than as a `Blocked` diagnosis is what lets this obligation say what the
   catalogue says without answering `FN-25.a`'s totality by construction.
   `README.md` records the decision and the three `why` values `exits` inherits
   beside it. */
check FN_21c_a_reaper_declines_a_foreign_entry_at_a_reserved_name {
  always ((Sys.act' = Reap and foreignAtReservedName)
    implies (Sys.res' = NoOp and Sys.why' = W17OwnershipConflict
             and treeSame and repoSame and txnSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 5 steps

/* Reached: a directory at the quarantine's reserved name carrying no cleanup
   manifest at all.  Grove cannot prove it is its own — it has nothing to prove
   it WITH — so the sweep passes over it and changes nothing. */
run witness_FN_21c_a_foreign_entry_at_a_reserved_name_is_declined {
  Txn.phase = Fresh
  some Quar.qRid
  no Cleanup.present
  no Root.rid
  eventually (Sys.act = Reap and Sys.res = NoOp
              and Sys.why = W17OwnershipConflict and some Quar.qRid)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

// --- FN-31.a: the replacement's source state is reachable -------------------

/* `TODO.finish_process.md` Q3 IS ANSWERED HERE, AND BY A WITNESS RATHER THAN BY
   A CONSTRUCTION.  The catalogue gives the obligation two admissible
   instruments — the source state as a witness, or a bounded-unreachability check
   of `FN-15.d`'s form recorded per lane — and the leaf's brief names the third
   possibility, a `defer`, as legitimate.  **This file answers with the witness.**
   The source state is reachable, it first lands at eleven states, and what makes
   it reachable is not an artefact of the encoding: `doMarkerRemove` is
   disposal's LAST step because `FN-21.a` requires the marker to outlive the work
   it authorises, so an interruption between the removal and the marker's
   retirement leaves an owned marker whose target is gone.  A later attempt that
   reaches the after-rename point before the sweep does must supersede it.

   THE ENUMERATION Q3 ASKS FOR IS THEREFORE ONE CLASS AND NOT A LIST: *a marker
   left standing by a disposal that completed its removal and was interrupted
   before retiring it.*  `README.md` records it as the answer, with the bound.

   THE MARKER IS SUPERSEDED RATHER THAN REMOVED AND REWRITTEN, and that is what
   makes this a REPLACEMENT rather than a remove followed by a create: `FN-31.b`
   forbids the state between.  The witness asserts both halves — one marker
   before, one after, and the one after names this attempt and this quarantine. */
run witness_FN_31a_a_source_state_from_which_disposal_must_replace_a_marker {
  interruptedMidEvacuation
  staleMarkerLeftBehind
  no Repo.tickets
  eventually (Sys.act = QuarRename and Sys.res = Applied and some Quar.qRid)
  eventually (Sys.act = MarkerReplace and Sys.res = Applied
              and one Cleanup.present
              and Cleanup.present.cOwner = Txn.attempt
              and Cleanup.present.cTarget = Quar.qRid)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* AND THE STALE MARKER IS PRODUCED RATHER THAN POSITED.  This is the command
   that keeps the answer above from being an artefact of a hand-edited disk:
   it runs the protocol from the disk an interruption mid-evacuation leaves,
   through the rename, the marker, and the removal the marker authorises, and
   crashes before the marker is retired — leaving exactly
   `staleMarkerLeftBehind`.  Twelve states.

   Without this command `witness_FN_31a` would be evidence about a disk `EN-11`
   permits rather than about one the protocol produces, and Q3's answer would
   rest on the free initial state.  That is the shape `commit-k41` left as a
   debt and `revalidation-k44` paid; this slice does not open a second one. */
run witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = Dispose and Sys.res = Applied and no Quar.qRid)
  eventually (Sys.act = Crash and Txn.phase = Fresh
              and no Quar.qRid and one Cleanup.present
              and some Cleanup.present.cOwner)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

/* AND THE PROPERTY SIDE OF THE CELL: THE REPLACEMENT IS A REPLACEMENT.  A
   reachability obligation still needs a check, and what there is to check here
   is that the transition the witness reaches is the one the catalogue asked for
   — *distinct from creating a marker and from removing one*.

   Three conjuncts.  The replacement is only ever performed where the catalogue
   puts disposal, and only over a name that is already occupied; a successful one
   leaves the name occupied too, so it is neither a create nor a remove; and the
   CREATE is never performed over an occupied name, which is what makes the two
   a partition of the after-rename point rather than two spellings of one step.
   The third is the one that would go red if a later slice folded the two
   together, which is the shape this obligation exists to prevent. */
check FN_31a_the_replacement_is_a_replacement_and_not_a_create_or_a_remove {
  always {
    (Sys.act' = MarkerReplace)
      implies (some Cleanup.present and atRevPoint[AfterRename])
    (Sys.act' = MarkerReplace and Sys.res' = Applied)
      implies (some Cleanup.present and some Cleanup.present')
    (Sys.act' = MarkerCreate) implies no Cleanup.present
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-31.b: the replacement is atomic with respect to readers -------------

/* THREE CONJUNCTS, AND THE THIRD IS THE ONE A `lone` FIELD WOULD HAVE MADE
   UNSTATEABLE.  *No reader observes the marker absent, nor observes two
   markers* is two prohibitions and this file can express both, because the
   markers are atoms and what is `var` is which of them stand at the reserved
   name.  Under `one sig Mark { var there: lone Marker }` the second prohibition
   is true by construction and the claim is half a claim — the false-confidence
   shape this corpus has recorded four times.

   (a) the replacement itself: one marker before, one after.
   (b) no step ever leaves two standing.
   (c) THE MARKER GOES ABSENT ONLY AT THE STEP WHOSE JOB IT IS.  This is what
   forbids a replacement implemented as a remove followed by a create — the
   trace in which a reader observes the marker absent — and it is what the
   mutation aims at.  Stated over Grove's own steps and the sweep, never over
   the free initial state, which is this file's oldest rule. */
check FN_31b_the_replacement_is_atomic_with_respect_to_readers {
  always {
    /* (a) THE REPLACEMENT LEAVES EXACTLY ONE.  The PRE-state is deliberately
       not constrained here: `EN-11` is cashed out as a free initial state, so
       *two markers at state 0* is a hand edit and a conjunct reading
       `one Cleanup.present` unprimed is a claim about what the world never does
       rather than about what the protocol never does.  That is this file's
       oldest rule and `FN_31b`'s first run met it again — the counterexample was
       a replacement over a hand-edited pair.  Conjunct (b) is what carries the
       pair's impossibility, over the transition relation where it belongs. */
    (Sys.act' = MarkerReplace and Sys.res' = Applied)
      implies one Cleanup.present'
    ((Sys.act' in txnActs or Sys.act' = Reap) and lone Cleanup.present)
      implies lone Cleanup.present'
    /* (c) AND THE REPLACEMENT NEVER LEAVES THE NAME EMPTY.  This is *no
       reader observes the marker absent*, and it is what forbids a replacement
       implemented as a remove followed by a create.  Stated over
       `MarkerReplace` ALONE rather than over every step: *the marker is not
       retired before the work it authorises* is disposal-wide and is
       `FN-21.a`'s second conjunct, and a version of this stated that widely
       made one of this slice's mutations kill it — the third way a mutation
       fails its aim, met here at the point where two obligations describe the
       same document from two directions. */
    (Sys.act' = MarkerReplace) implies some Cleanup.present'
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* THE CATALOGUE'S WITNESS: AN OBSERVATION INTERLEAVED WITH THE REPLACEMENT.
   This file has no reader action — nothing in the finish scope observes the
   marker except the sweep and the disposal steps themselves — so what an
   interleaved observation could see is stated over the STATES the trace passes
   through: every one of them holds at most one marker, the state before the
   replacement holds exactly one and so does the state after, and they are
   different markers.  `README.md` records the abstraction. */
run witness_FN_31b_an_observation_interleaved_with_the_replacement_sees_one_marker {
  interruptedMidEvacuation
  staleMarkerLeftBehind
  no Repo.tickets
  always lone Cleanup.present
  /* Stated forwards, which is the only direction Alloy 6 gives an EXPRESSION:
     `before` is a formula operator and there is no past-state expression, so
     the two sides of the replacement are named as the state it is taken FROM
     and that state's primed successor. */
  eventually (Sys.act' = MarkerReplace and Sys.res' = Applied
              and one Cleanup.present and one Cleanup.present'
              and Cleanup.present' != Cleanup.present)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

// --- FN-31.c: an interruption inside the replacement is resumable -----------

/* THE REPLACEMENT IS ONE TRANSITION, SO *INSIDE* IT IS ITS TWO BOUNDARIES, and
   saying so is the honest reading rather than a dodge: `FN-31.b` is the claim
   that there is no observable state between, and a claim that an interruption
   inside an atomic step is resumable is a claim about the two states either side
   of it.  Both are checked and both are witnessed, and the two resume
   DIFFERENTLY — the earlier one has a marker whose target is gone and the later
   one a marker whose target is standing — which is exactly why the sweep reads
   the marker rather than the phase.

   THE SECOND CONJUNCT IS *THE SAME TERMINAL STATE AS AN UNINTERRUPTED
   REPLACEMENT*, and it is stated over the completion rather than over the
   interruption: whatever retires the marker leaves the terminal state, so a
   resumption cannot land anywhere an uninterrupted disposal would not.
   `FN-21.a`'s third conjunct says the same thing about disposal as a whole; this
   one is stated over the replacement's own trace and is what `FN-23`'s
   idempotence will inherit. */
check FN_31c_an_interruption_at_either_boundary_of_the_replacement_is_resumable {
  always {
    /* (a) BOTH MARKERS THE REPLACEMENT TOUCHES ARE ONES A SWEEP CAN ACT ON —
       the one it supersedes and the one it writes.  That is what makes a
       resumption EXIST from either boundary: an interruption before the
       replacement leaves the first, an interruption after it leaves the second,
       and `reapable` accepts both.  A replacement that wrote a marker the sweep
       could not classify would strand the disposal at the later boundary for
       ever, which is the failure this conjunct is aimed at. */
    (Sys.act' = MarkerReplace and Sys.res' = Applied)
      implies (not markerForeign and (no m: Cleanup.present' | no m.cOwner))
    /* (b) AND THE SWEEP RESUMES IN THE RIGHT ORDER FROM EITHER OF THEM.  From
       the earlier boundary the marker's target is already gone and one firing
       retires it; from the later one the target is standing and the sweep
       removes it FIRST, keeping the marker.  Written over `Reap` alone, which
       is what keeps a mutation aimed at it from killing `FN-21.a`'s
       `disposalSteps` conjuncts — the third way a mutation fails its aim, and
       the one this file has recorded most recently. */
    (Sys.act' = Reap and Sys.res' = Applied and some Quar.qRid)
      implies (no Quar.qRid' and some Cleanup.present')
    (Sys.act' = Reap and Sys.res' = Applied and no Cleanup.present')
      implies disposalTerminalNext
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* THE EARLIER BOUNDARY, RESUMED: the interruption left the marker the
   replacement was going to supersede — Grove's own, its target already gone —
   and the sweep retires it in one firing.  Terminal. */
run witness_FN_31c_an_interruption_before_the_replacement_is_resumed {
  staleMarkerLeftBehind
  no Root.rid
  no Slot.occ
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Cleanup.present and no Quar.qRid)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 3 steps

/* THE LATER BOUNDARY, RESUMED: the interruption left the marker the replacement
   WROTE, naming a quarantine that is still standing, and the sweep runs
   disposal's remaining two steps in that order.  Terminal, and it is the same
   terminal state. */
run witness_FN_31c_an_interruption_after_the_replacement_is_resumed {
  interruptedMidDisposal
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Quar.qRid and some Cleanup.present)
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Cleanup.present and no Quar.qRid
              and no Slot.occ and manEmpty)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

// --- FN-31.d: never against a marker Grove cannot prove is its own ----------

/* `TT-24.a`'s SUBJECT INSIDE A TRANSACTION, AND THE CATALOGUE'S TABLE FIXES THE
   OUTCOME AS A BLOCK.  Two conjuncts: the replacement does not happen, and
   NOTHING happens — the foreign document is not superseded, not removed, not
   written beside.  A block rather than a refusal because the transaction has a
   PROVEN COMMIT at this point; ending it as a refusal would report that the
   finish did not happen while the ticket in history says it did.  That is the
   same reasoning `quarantine-k43` recorded for the occupied quarantine target
   and it is the catalogue's, not this file's.

   THE SWEEP'S SIDE OF THE SAME CONDITION IS `FN-21.c`, and the two are written
   over different antecedents — `markerForeign` here, `foreignAtReservedName`
   there — so a mutation aimed at one leaves the other standing.  They share a
   `why` and nothing else. */
check FN_31d_a_replacement_is_never_performed_against_a_foreign_marker {
  always {
    (Sys.act' = MarkerReplace and markerForeign)
      implies (Sys.res' = BlockedOutcome and Sys.why' = W17OwnershipConflict
               and treeSame and repoSame)
    (Sys.act' in disposalSteps and markerForeign) implies markSame
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

/* Reached: a document at the marker's reserved name that Grove cannot prove is
   its own — no attempt identity in it — met by a disposal that has just renamed
   the task root into the quarantine.  The attempt blocks with the quarantine
   standing and the foreign document untouched, which is what a later launch,
   or an operator, has to be able to find. */
run witness_FN_31d_a_foreign_marker_is_declined {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  one Cleanup.present
  no Cleanup.present.cOwner
  eventually (Sys.act = QuarRename and Sys.res = Applied and some Quar.qRid)
  eventually (Sys.act = MarkerReplace and Sys.res = BlockedOutcome
              and Sys.why = W17OwnershipConflict
              and one Cleanup.present and no Cleanup.present.cOwner
              and some Quar.qRid)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps


// ===========================================================================
// `FN-24` — EVERY INTERRUPTION LANDS IN EXACTLY ONE STABLE STATE
//
// The crash slice, and the first claim group in this file whose subject is the
// model's own shape rather than a transition.  `crash` has been enabled at
// every step boundary since `witness-k40`; what was never asked until now is
// what the disk it leaves CLASSIFIES AS, and whether the step it interrupted
// had one persistent effect or several.
//
// `FN-24.a`'S ANTECEDENT IS THE WIDEST IN THE FILE, and deliberately so: it
// quantifies over all sixteen members of `bodySteps`.  It adds no transition —
// which is what makes it cheap by the scope's cost law — but it is reachable at
// every state a body step is, so the bound is the whole of its price.
//
// WHERE THE BOUNDS COME FROM.  Both checks quantify over `bodySteps`, whose
// deepest member (`MarkerRemove`) first occurs at twelve states, and the
// witness-bound rule's own floor is thirteen.  The two rules agree here, which
// they did not for `FN-31.c`; the antecedent rule is applied anyway, which is
// the discipline `disposal-k45` left rather than an observation about this
// claim.
// ===========================================================================

/* THE DISK A LATER LAUNCH FINDS WITH NOTHING RESERVED — the run-up the four
   witness-building steps need, and the counterpart of
   `interruptedMidEvacuation` for the front half of the body.  It constrains
   TREE state at `Txn.phase = Fresh` and nothing else, which is the licence
   `witness-k40` established and `commit-k41` restated. */
pred freshGroveDisk {
  Txn.phase = Fresh
  some Op.confirmed
  some World.lane and World.rootDev = World.qDev and World.wtDev = World.qDev
  some Txn.leaseOk
  no Slot.occ and no Quar.qRid and no Cleanup.present
  some Root.rid
  Root.holds = finishLive and one finishLive and no ordinaryLive
  no e: Root.holds | e.et = OpaqueT
  Root.holds in Repo.tracked
  no Repo.tickets
}

/* One crash point, at one step's boundary.  `after Sys.act = Crash` is the
   interruption IMMEDIATELY after the step, which is what "between any two
   steps" means when the steps are consecutive. */
pred crashAfter[a: Action] { eventually (Sys.act = a and after Sys.act = Crash) }

// --- FN-24.a: the full step-boundary sweep ----------------------------------

/* FOUR CONJUNCTS, AND ONLY THE FIRST TWO ARE THE CLAIM'S OWN WORDS.  The other
   two are the load-bearing property §*States* states beneath the table — *no
   transient state may be observable as a different stable state* — instantiated
   at the two places this file can reach it.  Both were found by writing the
   classification down, and both are recorded in `README.md` as findings about
   the catalogue rather than about the protocol.

   (a) TOTALITY.  Every disk a crash leaves matches at least one row.  This is
   the conjunct a new phase or a new artifact breaks: a state the table has no
   row for reports here and nowhere else, and it is how
   `SReservedQuarantined` came to exist.

   (b) EXACTLY ONE, AFTER THE ORDER.  `classifiedRaw`'s arms overlap; the order
   is what resolves them, and it is written as a strict precedence so that
   deleting a pair leaves two survivors rather than silently changing which one
   wins.

   (c) `Absent` MEANS NOTHING OF GROVE'S IS LEFT AT A RESERVED NAME.  *A task
   root whose deletion is not yet proven is never `Absent`*, and the arm is the
   catalogue's row verbatim — *no task root* — so this conjunct is a claim about
   the ORDER and about nothing else.  The catalogue's own table orders `Absent`
   first; this file orders the reserved class before it, and this is the conjunct
   that would catch the other choice.

   (d) A DISK WITH SOMETHING OF GROVE'S AT A RESERVED NAME IS NEVER AN ORDINARY
   CURRENT TREE.  *An evacuated tree is `Reserved(Published)` and never
   `Malformed` or `Current(Spent)`* is the catalogue's own instance of it; the
   conjunct is stated over the whole `Current` class and over the quarantine as
   well as the witness, because a disposal that has released its witness with the
   quarantine still standing is the OTHER disk that would otherwise read as an
   ordinary spent grove.  That second half is what makes `SReservedQuarantined`
   load-bearing rather than decorative. */
check FN_24a_from_a_crash_at_any_step_boundary_the_result_classifies_as_exactly_one_stable_state {
  always {
    (Sys.act in bodySteps and after Sys.act = Crash) implies after {
      some classifiedRaw
      one classified
      classified = SAbsent implies (no Slot.occ and no Quar.qRid)
      classified in currentStates implies (no Slot.occ and no Quar.qRid)
    }
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE FULL INTERRUPTION SEQUENCE, ONE CRASH POINT PER STEP — sixteen commands,
   one for each member of `bodySteps`, because that is what the catalogue's
   witness says and because a single crash shown once would leave fifteen
   boundaries unproven while `FN_24a` reported green over all sixteen.  Each
   runs at its own step's depth rather than at the check's, which is what keeps
   the sweep affordable. */
run witness_FN_24a_a_crash_after_the_witness_is_prepared {
  freshGroveDisk
  crashAfter[WPrepare]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 6 steps

run witness_FN_24a_a_crash_after_the_manifest_is_written {
  freshGroveDisk
  crashAfter[WManifest]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run witness_FN_24a_a_crash_after_the_manifest_is_marked_ready {
  freshGroveDisk
  crashAfter[WReady]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

run witness_FN_24a_a_crash_after_the_witness_is_published {
  freshGroveDisk
  crashAfter[WPublish]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_24a_a_crash_after_the_tree_is_evacuated {
  interruptedMidEvacuation
  crashAfter[WEvacuate]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run witness_FN_24a_a_crash_after_the_commit_is_attempted {
  interruptedMidEvacuation
  crashAfter[CommitAttempt]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run witness_FN_24a_a_crash_after_a_recovery_adopts_the_witness {
  interruptedMidEvacuation
  crashAfter[Recover]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 6 steps

run witness_FN_24a_a_crash_after_the_classification {
  interruptedMidEvacuation
  crashAfter[Classify]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_24a_a_crash_after_the_quarantine_rename {
  interruptedMidEvacuation
  no Quar.qRid
  crashAfter[QuarRename]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_24a_a_crash_after_the_settle {
  interruptedMidEvacuation
  crashAfter[Settle]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run witness_FN_24a_a_crash_after_the_revalidation {
  interruptedMidEvacuation
  crashAfter[Revalidate]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_24a_a_crash_after_the_quarantine_is_returned {
  interruptedMidEvacuation
  no Quar.qRid
  crashAfter[QuarReturn]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

run witness_FN_24a_a_crash_after_the_cleanup_marker_is_created {
  interruptedMidEvacuation
  no Quar.qRid
  crashAfter[MarkerCreate]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_24a_a_crash_after_the_cleanup_marker_is_replaced {
  interruptedMidEvacuation
  no Quar.qRid
  one Cleanup.present
  some Cleanup.present.cOwner
  crashAfter[MarkerReplace]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps

run witness_FN_24a_a_crash_after_the_quarantine_is_disposed {
  interruptedMidEvacuation
  no Quar.qRid
  crashAfter[Dispose]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

run witness_FN_24a_a_crash_after_the_cleanup_marker_is_removed {
  interruptedMidEvacuation
  no Quar.qRid
  crashAfter[MarkerRemove]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

// --- FN-24.b: at most one persistent effect per step ------------------------

/* THE STEP LIST'S OWN DISCIPLINE, quantified over exactly `bodySteps` — which is
   what the set was named for, and what the header has said it was for since
   `witness-k40`.

   `Reap` IS OUTSIDE IT AND THE EXCLUSION IS THE CATALOGUE'S, NOT A CONVENIENCE.
   A sweep is not a step of the transaction, takes no confirmation, and never
   had a disposition to revalidate.  Writing the enumeration down did correct
   one sentence of this file, though, and `README.md` records it: the note on
   `doReap` says *as written each firing has exactly one persistent effect*, and
   the content-removal branch has TWO — the quarantine and the reserved witness
   — for exactly the reason `Dispose` does.  The exclusion was right; the reason
   given for it was one sentence too generous.

   THE TWO DECLARED STEPS ARE NAMED IN `declaredMultiEffect` AND NOWHERE ELSE,
   so that narrowing the check and declaring an abstraction are the same edit.
   A check quietly weakened until it passes and a declaration are otherwise
   indistinguishable in a green run, which is the shape this file exists to
   refuse. */
check FN_24b_every_step_of_the_transaction_has_at_most_one_persistent_effect {
  always {
    (Sys.act' in bodySteps and not declaredMultiEffect)
      implies lone persistentEffects
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 12 steps

/* THE STEP LIST, ENUMERATED, EACH STEP'S PERSISTENT EFFECT NAMED — one command
   per EFFECT rather than one per step, because the enumeration the catalogue
   asks for is of what the steps DO and sixteen steps produce seven kinds of
   effect between them.  `README.md` carries the step-by-step table; these seven
   are what make each of its columns a reachable fact rather than a reading of
   the source.  The eighth is the declaration made reachable: a step this file
   declares, shown having the two effects it is declared for. */
run witness_FN_24b_a_step_whose_one_effect_is_at_the_reserved_witness_name {
  freshGroveDisk
  eventually (Sys.act' = WPrepare and persistentEffects = EWitnessName)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 5 steps

run witness_FN_24b_a_step_whose_one_effect_is_the_manifest {
  freshGroveDisk
  eventually (Sys.act' = WManifest and persistentEffects = EManifest)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 6 steps

run witness_FN_24b_a_step_whose_one_effect_is_the_ready_mark {
  freshGroveDisk
  eventually (Sys.act' = WReady and persistentEffects = EReady)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run witness_FN_24b_a_step_whose_one_effect_moves_entries {
  freshGroveDisk
  eventually (Sys.act' = WEvacuate and persistentEffects = EEntries)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_24b_a_step_whose_one_effect_is_the_commit {
  interruptedMidEvacuation
  eventually (Sys.act' = CommitAttempt and persistentEffects = ECommit)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

/* THE ONE THE GRAIN EXISTS FOR.  Both names change and the step has ONE
   persistent effect, because `EN-01` grants atomicity to a same-directory
   rename — which is the second half of `FN-24.b`'s sentence, shown rather than
   asserted. */
run witness_FN_24b_a_step_whose_one_effect_is_the_atomic_root_rename {
  interruptedMidEvacuation
  no Quar.qRid
  eventually (Sys.act' = QuarRename and atomicRootRename
              and persistentEffects = EQuarName)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_24b_a_step_whose_one_effect_is_at_the_cleanup_marker_name {
  interruptedMidEvacuation
  no Quar.qRid
  eventually (Sys.act' = MarkerCreate and persistentEffects = EMarkerName)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

/* THE DECLARATION, REACHED.  `Dispose` clears the quarantine name and the
   reserved witness name in one step, because in this model they are two `one
   sig`s and in the shipped protocol the second is inside the first.  A
   declaration nothing demonstrates is a claim about the source rather than
   about the model, which is why it has a command. */
run witness_FN_24b_the_declared_step_with_two_persistent_effects {
  interruptedMidEvacuation
  no Quar.qRid
  eventually (Sys.act' = Dispose
              and persistentEffects = EQuarName + EWitnessName)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps



// ===========================================================================
// `FN-25` — A BLOCK IS EXACTLY ONE OF THE TWO DIAGNOSES
//
// The partition itself is written as data above, apart from every transition;
// what is here are the three obligations stated over it.  Every command reads
// the diagnosis UNPRIMED against a primed `Sys.res'` — see the note above the
// signature for why the state a block LANDS in cannot carry it.
//
// WHERE THE BOUNDS COME FROM.  All three checks quantify over every blocking
// transition in the file, whose deepest is `QuarReturn`'s incomplete return
// (`witness_FN_22h`, twelve states); the widest first-landing bound among these
// obligations' own witnesses is nine.  The antecedent rule is the larger of the
// two and gives thirteen, which is also where `FN_24a` sits.
// ===========================================================================

/* FN-25.a.  TWO CONJUNCTS, AND THE FIRST IS THE CLAIM'S OWN WORDS.  *No blocked
   state satisfies both* is `lone diagnosedRaw`; the file's one exception is
   named in `declaredDiagnosisOverlap` and nowhere else, so weakening the check
   and declaring the overlap are the same edit.  The second conjunct is what the
   catalogue's witness means by *resolved to exactly one*, and it is the strict
   precedence doing its work. */
check FN_25a_the_two_diagnoses_are_disjoint_and_the_declared_overlap_resolves_to_one {
  always (Sys.res' = BlockedOutcome implies {
    lone diagnosedRaw or declaredDiagnosisOverlap
    one diagnosed
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE CATALOGUE'S OWN WITNESS: *a state that nearly does — a Grove-owned,
   correlated artifact sitting at a name Grove also reserves — resolved to
   exactly one*.  A foreign cleanup marker stands while this attempt's own
   published witness, manifest and handle correlate perfectly; both arms hold
   and the fail-closed precedence returns `OwnershipConflict`. */
run witness_FN_25a_a_correlated_attempt_at_a_name_grove_also_reserves_resolved_to_one {
  interruptedMidEvacuation
  one Cleanup.present and no Cleanup.present.cOwner
  eventually (Sys.res' = BlockedOutcome
              and dgCorrelatedIncompleteAttempt
              and dgUnclassifiableAtReservedName
              and diagnosed = DOwnershipConflict)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

/* FN-25.b.  TWO CONJUNCTS, AND THE SECOND IS A BOUNDED UNREACHABILITY RATHER
   THAN A CLAIM OF THE CATALOGUE'S.  *No blocked state carries neither* is the
   first.  The second records, as a checked fact rather than as prose, that
   `OwnershipConflict`'s SECOND clause never fires alone in this model: every
   state whose topology matches neither the anchor nor the result is also a
   state whose reserved witness this attempt cannot prove is its own.  A witness
   for the topology clause alone was sought at twelve states and does not exist;
   stating it here is what makes a future slice that reaches one see a red
   command rather than an unremarked green. */
check FN_25b_every_block_carries_at_least_one_of_the_two_diagnoses {
  always (Sys.res' = BlockedOutcome implies {
    some diagnosedRaw
    dgTopologyUnmatched implies dgUnclassifiableAtReservedName
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* The catalogue's witness for `FN-25.b` is *an exhaustive sweep of the blocked
   states within the bound*, which is the check above and not a `run`.  What the
   two commands below add is that each arm is reachable ALONE — a sweep over an
   arm nothing ever satisfies is exhaustive and empty. */
run witness_FN_25b_a_block_whose_only_arm_is_recovery_pending {
  interruptedMidEvacuation
  no Cleanup.present
  eventually (Sys.res' = BlockedOutcome and diagnosedRaw = DRecoveryPending)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_25b_a_block_whose_only_arm_is_ownership_conflict {
  Txn.phase = Fresh
  /* NO MARKER ANYWHERE IN THE TRACE, which is what makes this witness say
     something the near-miss above does not: `OwnershipConflict` is reachable
     through the reserved WITNESS name alone, and does not depend on the cleanup
     marker.  Q4's removal matrix reads that line — the marker's removal would
     not take this diagnosis with it. */
  always no Cleanup.present
  eventually (Sys.res' = BlockedOutcome and diagnosedRaw = DOwnershipConflict
              and dgUnclassifiableAtReservedName)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

/* FN-25.c IS A REACHABILITY OBLIGATION, so the six witnesses do the reaching
   and the check states the property that makes them mean something — the shape
   `FN-15.b` .. `FN-15.d` are written in.

   THE PROPERTY IS LANE-BLINDNESS, and the first form of it was FALSE.  *A
   diagnosis is only ever produced under a selected lane* looked like the
   companion property and is not: `World.lane` is `var` because `SY-03` requires
   a preflight never to be a licence, so the world can withdraw the layout
   between two of Grove's steps and the block that follows is decided with no
   lane at all.  That is a property of the protocol rather than a defect, and it
   is `README.md`'s fifth counterexample.

   What is true, and is what `each diagnosis on each lane` rests on, is that no
   clause of the partition READS the lane: a step that changes the lane and
   nothing else moves no atom into or out of `diagnosed`.  Together with the six
   witnesses that is *reachable on each lane*; without it, six witnesses could
   each be reaching a lane-specific condition that happens to exist three
   times. */
check FN_25c_the_partition_reads_no_lane_so_no_diagnosis_is_confined_to_one {
  always (( some World.lane and some World.lane'
            and treeSame and repoSame and opSame and txnSame
            and World.wcWork' = World.wcWork )
          implies (all d: Diagnosis | d in diagnosed iff after (d in diagnosed)))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE LANE IS PINNED FOR THE WHOLE TRACE IN EACH OF THE SIX, for the reason
   `FN-15.b`'s nine give: *reached, on each lane* is three statements and Alloy
   has no way to parameterise a command.  `EN-16` runs over them. */
run witness_FN_25c_git_recovery_pending_reached {
  always World.lane = GitL
  interruptedMidEvacuation
  no Cleanup.present
  eventually (Sys.res' = BlockedOutcome and diagnosed = DRecoveryPending)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_25c_nativejj_recovery_pending_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Cleanup.present
  eventually (Sys.res' = BlockedOutcome and diagnosed = DRecoveryPending)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_25c_colocatedjj_recovery_pending_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Cleanup.present
  eventually (Sys.res' = BlockedOutcome and diagnosed = DRecoveryPending)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_25c_git_ownership_conflict_reached {
  always World.lane = GitL
  interruptedMidEvacuation
  one Cleanup.present and no Cleanup.present.cOwner
  eventually (Sys.res' = BlockedOutcome and diagnosed = DOwnershipConflict)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_25c_nativejj_ownership_conflict_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  one Cleanup.present and no Cleanup.present.cOwner
  eventually (Sys.res' = BlockedOutcome and diagnosed = DOwnershipConflict)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run witness_FN_25c_colocatedjj_ownership_conflict_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  one Cleanup.present and no Cleanup.present.cOwner
  eventually (Sys.res' = BlockedOutcome and diagnosed = DOwnershipConflict)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps


// ===========================================================================
// `FN-26` — HISTORY IS NEVER REWRITTEN TO CLEAR A BLOCK
//
// THREE CONJUNCTS, AND THE FIRST IS NARROWED TO GROVE'S OWN STEPS FOR THE
// REASON `FN_03`'s FIRST CONJUNCT WAS.  `doCommitMoves` is the world taking a
// ticket back out of history — an operator's `jj undo` between two of Grove's
// steps — and a claim about what a protocol never does is never a claim about
// what the world never does.  Stated over every step this conjunct would delete
// the states two rows of the revalidation table need, which is the failure mode
// `README.md` records under *A fourth finding*.
// ===========================================================================

check FN_26_history_is_never_rewritten_to_clear_a_block {
  always {
    /* (i) the blocking transition itself writes no history — a block is a
       stopping, not a correction. */
    Sys.res' = BlockedOutcome implies repoHistorySame
    /* (ii) nor does the step that meets the block. */
    (Sys.res = BlockedOutcome and Sys.act' in txnActs) implies repoHistorySame
    /* (iii) and from the block onward Grove never takes a ticket back out —
       which is the half `doCommitMoves` shows the WORLD can do and Grove
       cannot.  Stated over `Repo.tickets` rather than over `repoHistorySame`
       because a later attempt's own commit may legitimately append. */
    Sys.res = BlockedOutcome implies
      always (Sys.act' in txnActs implies Repo.tickets in Repo.tickets')
    /* (iv) the diagnostic names the artifact, the recorded topology and the
       observed one, on every block. */
    Sys.res' = BlockedOutcome implies BArtifact + BRecorded + BObserved in blockDiagnostic
    /* (v) and names the two restorable exits as well when the diagnosis is
       `RecoveryPending`, which is the only one the catalogue promises them
       for. */
    (Sys.res' = BlockedOutcome and diagnosed = DRecoveryPending)
      implies blockDiagnostic = BlockField
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* The catalogue's witness, in its two halves: *a block whose diagnostic carries
   all four*, and *no trace in which recorded history changes* — the second read
   as the world never taking the ticket back, which is the only thing in this
   file that rewrites rather than appends. */
run witness_FN_26_a_block_whose_diagnostic_names_all_four_with_history_unchanged {
  interruptedMidEvacuation
  no Cleanup.present
  always Sys.act != CommitMoves
  eventually (Sys.res' = BlockedOutcome and blockDiagnostic = BlockField)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps


// ===========================================================================
// THE EXITS SLICE'S DATA — four funs, four preds, and one posited disk
//
// Written apart from every transition, in the discipline the seven
// preconditions established and `Stable`, `Effect` and `Diagnosis` continued:
// what the claims range over is here, what the protocol does is above, and a
// divergence between them is a counterexample rather than a definition.
//
// NOTHING HERE IS A `var` FIELD, A TRANSITION OR A `fact`.  The one `var` field
// this slice adds is `World.hookRan`, at the signature, and what it costs is
// argued in `README.md` rather than measured: a `lone` field over a `one sig`
// that no guard reads is MONOTONE on the state space — every instance of the
// file before it extends to an instance after it by setting the field absent in
// every state — so no existing witness's first-landing bound can rise.
// ===========================================================================

/* GROVE'S OWN ACTIONS, as one named thing.  `txnActs` is the TRANSACTION's
   steps and `FN-01.a` is stated over exactly that set; `FN-27` and `FN-30` are
   stated about GROVE, which is wider: the decline is Grove reporting, the
   discard is Grove's recovery of an unpublished witness, and the reaper is
   Grove's sweep.  The world's four — `Swap`, `TopologyChange`, `CommitMoves`,
   `RootNameTaken` — and `Crash`, `Confirm` and `ResultArrives` are deliberately
   outside it: a claim about what a protocol never does is never a claim about
   what the world never does, which is the narrowing `FN-03` and `FN-26` each
   had to make and the reason this set is named rather than written out. */
fun groveActs: set Action { txnActs + Decline + Discard + Reap }

/* THE INTERNAL COMMITS — `FN-30`'s *during an internal commit*, and there are
   exactly two in this file.  `CommitAttempt` issues the deletion commit;
   `Settle`'s restore branch reproduces the exact recorded preflight commit on a
   working-copy-as-commit lane (`preflightCommitReproduced`).  Both are
   version-control writes Grove performs on the operator's repository, which is
   what the catalogue means by internal. */
fun internalCommitActs: set Action { CommitAttempt + Settle }

/* THE RECOVERY ACTIONS — `FN-23`'s subject, and it is a CLASS rather than the
   marker protocol.  `Discard` recovers an unpublished witness, `Recover` adopts
   a published one, and `Reap` resumes an interrupted disposal.  `FN-21.a` and
   `FN-31.c` are the same word at the incumbent's own grain and are *incumbent
   mechanics*; `FN-23` is shared safety and is stated over the role, which is
   what keeps a mutation to either from being a control for the other. */
fun recoveryActs: set Action { Discard + Recover + Reap }

/* `manWritten` AND `leftoverArtifact` IN THE NEXT STATE.  Both exist because
   `FN-29` is stated about what an outcome LEAVES, and every outcome in this
   file is written on the primed side. */
pred manWrittenNext { some Man.mHandle' or some Man.mAttempt' or some Man.mAnchor'
                      or some Man.mEntries' }
pred leftoverArtifactNext { some Quar.qRid' or some Slot.occ' or manWrittenNext }

/* WHAT IS UNRELATED, NAMED ONCE.  `FN-27` exempts four things — the task root,
   the reserved witness, the quarantine and the scoped commit — which in this
   file are `Root`, `Slot` + `Man`, `Quar` + `Cleanup` and `Repo`.  What is left
   is the WORKSPACE: the unrelated modified working-copy work, the workspace
   layout, and the operator's own confirmation.

   THREE THINGS ARE DELIBERATELY ABSENT AND EACH IS ANOTHER OBLIGATION'S
   SUBJECT.  `World.hookRan` is `FN-30`'s.  `Repo.rev`, `Repo.tracked` and
   `Repo.tickets` are the scoped commit's, which `FN-14` states over the ticket
   and `FN-28` states over the topology.  The devices are static.  This is
   `disposal-k45`'s fourth rule applied before the fact rather than after a
   neighbour kill: `FN-27`'s subject is the widest in the scope, so what it does
   NOT describe is the load-bearing half of writing it down. */
pred unrelatedUnchanged {
  World.wcWork' = World.wcWork
  World.lane'   = World.lane
  Op.confirmed' = Op.confirmed
}

/* THE SINGLE SUCCESSFUL EXIT, AS THE CATALOGUE'S TWO OPERANDS AND NOT AS A
   PHASE.  *A finish succeeds exactly when the exact attempt-bound commit is
   proven and the task root is absent* — `resultProven` is the first and
   `taskRootAbsent` is the second, and neither is `Txn.phase = Settled`: a finish
   that has succeeded may still owe its cleanup, which is the whole of the
   claim's third sentence and of its witness.

   `some Txn.attempt` IS THE SIXTH VACUITY GRAIN PAID OFF, and it is not
   decoration.  `resultProven` reads `Txn.attempt in ticketedAttempts`, and in a
   state the transaction has LEFT — which is every state a block or a refusal
   lands in, because `txnGone` clears the attempt — that reads `none in ...` and
   is vacuously TRUE.  Without this conjunct every abandoned disk with an absent
   task root would classify as a success, greenly.  `blocked-k48` found the
   grain on `Sys.res'`; this is the same grain met on the operand side. */
/* *THE TASK ROOT IS ABSENT*, AND IT IS `Txn.pinned not in Root.rid` RATHER THAN
   `no Root.rid` BECAUSE A COUNTEREXAMPLE SAYS SO.  The world can occupy the
   task-root name while the quarantine holds the root — that is
   `doRootNameTaken`, which exists for `FN-22.h` — so a success stated as *the
   name is empty* is FALSE of a protocol that did everything right: the finish
   is proven, the pinned directory is gone, and a stranger has taken the name
   one step later.  What the catalogue means by absent is that THE TASK ROOT is
   no longer there, and the task root is the identity the transaction pinned.
   This is `FN-19`'s third conjunct's lesson at a new grain — a claim about the
   protocol's shape has to survive the world's own steps — and `README.md`
   records the reading as an abstraction. */
pred taskRootAbsent { Txn.pinned not in Root.rid }
pred finishSucceeded { some Txn.attempt and resultProven and taskRootAbsent }

/* THE SAME POSITED DISK AS `interruptedMidEvacuation`, UNCONFIRMED.  `FN-02`'s
   witness is *a decline followed by a later successful attempt on the same
   handle*, a decline requires `no Op.confirmed`, and nothing in this file can
   take a confirmation back except `Crash` — which would also take the
   transaction, and there is none yet.  So the disk is posited without one and
   the trace confirms before it opens.  Every other conjunct is
   `interruptedMidEvacuation`'s, unchanged; the two are written apart rather
   than parameterised because twenty-two commands read the confirmed one and a
   parameter would have moved all of them. */
pred interruptedMidEvacuationUnconfirmed {
  Txn.phase = Fresh
  no Op.confirmed
  some World.lane and World.rootDev = World.qDev and World.wtDev = World.qDev
  some Txn.leaseOk
  Slot.occ = Published
  some Slot.owner and Slot.owner = Man.mAttempt
  some Man.mReady and some Slot.wHolds
  no Repo.wTracked
  some Root.rid
  Root.holds = finishLive and one finishLive and no ordinaryLive
  no e: Root.holds | e.et = OpaqueT
  Root.holds in Repo.tracked
  Man.mHandle = finishLive
  some Man.mAnchor and Man.mAnchor = Repo.rev
  some Man.mFinger and Man.mFinger in Repo.tracked
  Man.mEntries = Slot.wHolds + Root.holds
}


// ===========================================================================
// `FN-02` — INTENT PERSISTS AS THE FINISH LEAF
//
// WHERE THE BOUNDS COME FROM.  The check's antecedent names `Decline` (state 1)
// and every refusing or blocking member of `txnActs`, whose deepest is
// `Revalidate`'s completed refusal; the witness is the deepest command this
// slice adds, because it is the only one that runs a decline AND a whole
// successful attempt.  Thirteen is the larger of the two rules and is where
// `FN_24a`, `FN_25a` .. `FN_25c` and `FN_26` already sit.
//
// THE LEAF IS `FN-02`'s SUBJECT AND `FN-29` DOES NOT DESCRIBE IT.  The
// catalogue gives both claims a sentence about the finish leaf being live and
// selectable; `disposal-k45`'s fourth rule says the one whose subject it is not
// should not describe it at all, and *intent persists as the finish leaf* is
// this one.  `FN-29` gets the completeness and the distinguishability instead.
// ===========================================================================

/* THREE CONJUNCTS, AND THE SECOND IS THE ONE THE CLAIM IS ACTUALLY ABOUT.

   (i) A DECLINE WRITES NOTHING ELSE.  Every frame in the file at once, which is
   the whole of *and SHALL write nothing else* in a model with no filename
   grammar.
   
   (ii) NO EXIT WITHOUT COMPLETION DESTROYS THE PINNED HANDLE.  Stated over
   `Txn.handle` — the LIVE SESSION's pin, never an artifact — rather than over
   `finishLive`, because after the evacuation the root holds nothing and a claim
   read off the tree would have nothing to read.  The three places the leaf may
   be are its own name, inside the published witness, and recorded in the
   manifest: all three are somewhere a later launch reaches it, which is what
   *live and selectable* is worth across an interruption.
   
   IT IS A PRESERVATION AND NOT A PRESENCE, AND A COUNTEREXAMPLE IS WHY.
   Written as *the handle is findable afterwards*, this conjunct is FALSE under
   `EN-11` cashed out as a free initial state: state 0 may sit at `Opened` with
   a handle pinned to an entry the tree does not hold, and the preflight's own
   refusal then reports an exit that never had a leaf to leave.  Restated as
   *whatever of the handle was findable stays findable*, it is a claim about the
   transition relation, which is the witness slice's first retained
   counterexample applied rather than met again.  Retained in `README.md`.
   
   (iii) AND THE COMPLETED REFUSAL PUTS IT BACK AT ITS OWN NAME.  This is the
   conjunct that would be false under a hand-edited state 0 — a manifest whose
   recorded entries omit the handle it also records — so it carries the
   antecedent that says the manifest is well-formed, which is the witness
   slice's first retained counterexample applied rather than re-learned. */
check FN_02_declining_or_exiting_without_completing_leaves_the_finish_leaf_live {
  always {
    Sys.act' = Decline implies
      (treeSame and repoSame and worldSame and opSame and txnSame)
    (Sys.act' in txnActs and Sys.res' in (Refused + BlockedOutcome))
      implies (Txn.handle & (Root.holds + Slot.wHolds + Man.mEntries))
                in (Root.holds' + Slot.wHolds' + Man.mEntries')
    (Sys.res' = RefRollbackNotCommitted and Txn.handle in Man.mEntries)
      implies Txn.handle in Root.holds'
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE CATALOGUE'S OWN WITNESS: *a decline followed by a later successful
   attempt on the same handle*.  The decline is at state 1 over the same tree
   the attempt later finishes, so *the same handle* is carried by the tree being
   untouched rather than by an equality — which is conjunct (i) doing its work
   in a `run` instead of a `check`.
   
   IT IS THE DEEPEST COMMAND THIS SLICE ADDS, and the reason is the node brief's:
   every witness whose subject is an OUTCOME costs the run-up and nothing else,
   and this one costs the run-up TWICE OVER — a decline and a confirmation
   before the transaction can open at all. */
run witness_FN_02_a_decline_followed_by_a_later_successful_attempt_on_the_same_handle {
  interruptedMidEvacuationUnconfirmed
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = Decline and Sys.res = NoOp and Sys.why = P1Confirm)
  eventually (Sys.act = QuarRename and Sys.res = Applied and finishSucceeded)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps


// ===========================================================================
// `FN-23` — RECOVERY IS IDEMPOTENT
//
// STATED OVER THE ROLE AND NOT OVER THE MARKER PROTOCOL, which is what keeps it
// apart from `FN-21.a` and `FN-31.c`.  Those two say a resumption of DISPOSAL
// reaches the same terminal state and are *incumbent mechanics*; this one is
// shared safety and is a claim about every recovery action a candidate protocol
// might have: it only ever removes, and it is not enabled once there is nothing
// left to remove.  Together those two are *reaches the same terminal state, and
// makes no further change once it has* — for a cleanup, idempotence IS
// monotone removal plus a guard that goes false.
//
// WHERE THE BOUNDS COME FROM.  The antecedent names `Reap`, whose first
// occurrence is at four states from the posited disk and at twelve from a fresh
// grove; the witness lands at five.  Thirteen is taken for the check, which is
// the antecedent rule applied to the deepest member of `recoveryActs` reachable
// by running the body rather than by positing it.
// ===========================================================================

/* THREE CONJUNCTS, AND THE FIRST IS THE ONE A CANDIDATE PROTOCOL WOULD HAVE TO
   SUPPLY.
   
   (i) A RECOVERY ONLY EVER REMOVES.  No recovery action creates a quarantine,
   creates a marker, or puts a witness back at the reserved name.  That is what
   makes re-running one converge rather than oscillate, and it is stated as
   three subset conditions rather than as an equality because removing is
   exactly what these steps are for.
   
   (ii) AND IS NOT ENABLED AT ITS OWN TERMINAL STATE.  Written as two
   implications over the disk rather than as a claim about a phase, because the
   phase a sweep runs at is `Fresh` and says nothing.  Together with (i) this is
   *makes no further change once it has*: the second firing has nothing to
   remove, the third is not available at all.
   
   (iii) AND A RECOVERY NEVER WRITES RECORDED HISTORY.  A recovery is a cleanup
   of the filesystem, and a candidate protocol that recovered by committing
   would be reaching for `EN-05`'s far side.  `Repo.tickets` is stated as an
   equality rather than as a subset — a recovery does not append either. */
check FN_23_recovery_is_idempotent {
  always {
    (Sys.act' in recoveryActs) implies {
      Quar.qRid' in Quar.qRid
      Cleanup.present' in Cleanup.present
      Slot.occ' in Slot.occ
    }
    (no Cleanup.present and no Quar.qRid) implies Sys.act' != Reap
    (no Slot.occ) implies Sys.act' != Discard
    (Sys.act' in recoveryActs) implies repoHistorySame
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE CATALOGUE'S WITNESS: *three consecutive recoveries, the second and third
   changing nothing*.  It is read as the catalogue's sentence rather than as its
   arithmetic, and the difference is worth a line.  The FIRST firing removes what
   the marker authorises; the SECOND retires the marker; the THIRD is an
   INVOCATION over a terminal disk, and in this model an invocation with nothing
   to sweep is not a `Reap` at all — the guard is false, so what a third
   invocation does is stutter.  *Changing nothing* is therefore two states of
   the disk being equal across an idle step, which is the honest reading of the
   catalogue's sentence in a file where the sweep is a transition rather than a
   process. */
run witness_FN_23_three_consecutive_recoveries_the_second_and_third_changing_nothing {
  interruptedMidDisposal
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Quar.qRid and some Cleanup.present)
  eventually (Sys.act = Reap and Sys.res = Applied
              and no Quar.qRid and no Cleanup.present)
  eventually (Sys.act = Idle and no Quar.qRid and no Cleanup.present
              and no Slot.occ)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps


// ===========================================================================
// `FN-27` — NOTHING UNRELATED IS MUTATED, ON ANY OUTCOME
//
// THE WIDEST-SUBJECT CLAIM IN THE SCOPE, AND WHAT IT DOES NOT SAY IS WHAT MAKES
// IT ONE CLAIM RATHER THAN A RESTATEMENT OF SIX.  The node brief predicted the
// overlap with every frame condition in the file and two slices confirmed it at
// a smaller scale; the answer taken here is to give the claim a subject of its
// own — `unrelatedUnchanged`, three fields, none of them another obligation's —
// rather than to state it over the whole frame and then discover which
// neighbours it had annexed.
//
// THE THREE OBLIGATIONS ARE ONE PROPERTY UNDER THREE ANTECEDENTS, and they are
// three commands rather than one because the catalogue states them as three and
// because a single check would report one counterexample where the class
// register wants to know WHICH outcome leaked.
//
// WHERE THE BOUNDS COME FROM.  Each antecedent quantifies over `groveActs`,
// whose deepest member is `MarkerRemove` at twelve states; the widest
// first-landing bound among the three witnesses is nine.  Thirteen is the
// larger of the two.
// ===========================================================================

check FN_27a_nothing_unrelated_changes_on_success {
  always ((Sys.act' in groveActs and Sys.res' = Applied) implies unrelatedUnchanged)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* *Reached* is the whole of the catalogue's witness for all three, so each of
   the three runs asks for the outcome WITH UNRELATED WORK ACTUALLY PRESENT —
   which is what `witness_FN_14` established is not free: without
   `always some World.wcWork` the claim is reached over an empty set and the
   command proves the antecedent rather than the property. */
run witness_FN_27a_a_success_with_unrelated_work_present {
  interruptedMidEvacuation
  no Repo.tickets
  no Cleanup.present
  always some World.wcWork
  eventually (Sys.act = QuarRename and Sys.res = Applied and finishSucceeded
              and some World.wcWork)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

check FN_27b_nothing_unrelated_changes_on_refusal {
  always ((Sys.act' in groveActs and Sys.res' in Refused) implies unrelatedUnchanged)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

run witness_FN_27b_a_refusal_with_unrelated_work_present {
  interruptedMidEvacuation
  no Cleanup.present
  always some World.wcWork
  eventually (Sys.res' = RefRollbackNotCommitted and some World.wcWork)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

check FN_27c_nothing_unrelated_changes_on_a_block {
  always ((Sys.act' in groveActs and Sys.res' = BlockedOutcome) implies unrelatedUnchanged)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

run witness_FN_27c_a_block_with_unrelated_work_present {
  interruptedMidEvacuation
  no Cleanup.present
  always some World.wcWork
  eventually (Sys.res' = BlockedOutcome and some World.wcWork)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps


// ===========================================================================
// `FN-28` — ONE SUCCESSFUL EXIT
//
// FOUR CONJUNCTS, AND THE CLAIM'S FOUR SENTENCES MAP ONTO THEM ONE FOR ONE.
// The definition, the single door, the outstanding cleanup, and the topology.
//
// WHERE THE BOUNDS COME FROM.  Conjunct (c) names every member of
// `disposalSteps`, whose deepest is `MarkerRemove` at twelve states; the
// witness lands at nine.  Thirteen is the larger.
// ===========================================================================

/* THE SECOND OPERAND IS NOT A STATE PREDICATE ANY CLAIM CAN ASSERT, AND THAT IS
   THIS SLICE'S LARGEST FINDING.  *The task root is absent* reads as a fact
   about the disk, and it is one the WORLD can forge away at any point after the
   rename: `doRootNameTaken` puts something at the free name, and `doSwap` may
   then give that something the QUARANTINED ROOT'S OWN IDENTITY — which at
   `2 RootId` is one step and is exactly the retained counterexample `FN-19`'s
   third conjunct carries, *moving the quarantine directory back over `.grove/`
   as seen from the inside*.  Written as `after finishSucceeded`, `implies
   finishSucceeded`, or `Txn.pinned not in Root.rid'`, conjuncts (a) and (c)
   are each FALSE for that reason and for no other.
   
   SO THE OPERANDS ARE STATED AS THINGS GROVE ESTABLISHES AND PRESERVES, never
   as things that hold.  What follows for the shipped protocol is worth more than
   the check: a finish cannot report success by looking at the task-root NAME,
   because the name is the world's to occupy.  What it can read is the
   correlation ticket, which is `FN-03`'s subject and is why that claim exists.
   Recorded in `README.md` as a finding for `formal-synthesis-k16`.
   
   (a) THE STEP THAT COMPLETES THE FINISH IS REACHED ONLY OVER A PROVEN COMMIT.
   `MarkerRemove` is disposal's last step and the only thing in this file that
   ends the forward path.  The second operand is (b)'s and (c)'s, for the reason
   above.
   
   (b) AND THE FORWARD PATH HAS EXACTLY ONE DOOR.  Nothing under a transaction
   takes the task root away except the quarantine rename, and the rename does it
   only on a proven result.  This is *succeeds EXACTLY when* stated as the only
   thing a model can state it as: not a definition repeated, but the uniqueness
   of the transition that establishes it.
   
   (c) GROVE NEVER PUTS THE TASK ROOT BACK WHILE THE COMMIT STANDS PROVEN.  A
   preservation across Grove's own step rather than an assertion about the disk,
   for the reason above; the one Grove step that DOES put the pinned root back
   is `doQuarReturn`, and it runs only when the observation is not `Committed`,
   so `resultProven` is exactly what excludes it.
   
   *BEST-EFFORT CLEANUP OUTSTANDING DOES NOT MAKE A PROVEN FINISH UNSUCCESSFUL*
   IS THE WITNESS'S AND NOT A CONJUNCT'S, AND THE ATTEMPT TO MAKE IT ONE IS A
   SIXTH GRAIN OF THE VACUITY RULE MET FROM THE OTHER SIDE.  Written as
   `(finishSucceeded and Sys.act' in disposalSteps) implies after
   finishSucceeded`, it is FALSE on a `MarkerReplace` that meets a foreign
   marker: `FN-31.d` blocks, `txnGone` clears the attempt, and
   `finishSucceeded`'s `some Txn.attempt` — the conjunct added so that a state
   the transaction has LEFT does not read as a success vacuously — is exactly
   what stops holding.  The finish has not become unsuccessful; the SESSION has
   ended.  **An operand added to defeat vacuity can make the property
   unobservable at the very states it was added for**: `blocked-k48` found the
   grain where the subject was read too early, and this is the same grain where
   it is read too late.  What the sentence actually asks for is a REACHABILITY —
   a success with its cleanup still outstanding — which is the catalogue's own
   witness and is the `run` below.  Recorded in `README.md`.
   
   (d) NO INTEGRATION OR REMOVAL.  Grove's own steps move recorded topology only
   AT AN INTERNAL COMMIT; branch, bookmark and worktree topology have no
   separate observable in this file and their absence is declared in `README.md`
   as an abstraction.  An integration or a removal is a repository write at a
   step that is not a commit, and this is that sentence.  `FN-14` states the
   other half — what a landing ticket is scoped to — and the two are written
   apart so that a mutation to either is a control for one of them.
   
   IT WAS FIRST WRITTEN AS *SOME TICKET LANDED* AND THAT IS FALSE, WHICH IS A
   FINDING RATHER THAN A SLIP.  `commitLands` writes
   `Repo.tickets' = Repo.tickets + (Txn.handle -> Txn.attempt)`, so an attempt
   whose handle is EMPTY moves `Repo.rev` and lands no ticket at all — a commit
   with nothing to correlate.  The state is a hand-edited `Opened` with no
   handle, which `fact TxnStateWellFormed` permits because the handle is
   `set Entry` and only its `Fresh` direction is true by construction.  Stating
   (d) over the ACTION rather than over the ticket says what the claim means
   and does not rest on the pin.  Retained in `README.md`. */
check FN_28_a_finish_succeeds_exactly_when_the_commit_is_proven_and_the_root_is_absent {
  always {
    (Sys.act' = MarkerRemove and Sys.res' = Applied)
      implies (some Txn.attempt and resultProven)
    (Sys.act' in txnActs and some Root.rid and no Root.rid')
      implies (Sys.act' = QuarRename and resultProven)
    (Sys.act' in groveActs and resultProven and taskRootAbsent)
      implies Txn.pinned not in Root.rid'
    (Sys.act' in groveActs and Repo.rev' != Repo.rev)
      implies Sys.act' in internalCommitActs
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE CATALOGUE'S WITNESS: *a success whose cleanup is still outstanding*.  The
   quarantine rename has landed, both operands hold, and the quarantine and the
   witness inside it are still standing with disposal not yet begun — which is
   the state the claim's third sentence exists for and the state a crash here
   leaves for the reaper. */
run witness_FN_28_a_success_whose_cleanup_is_still_outstanding {
  interruptedMidEvacuation
  no Repo.tickets
  no Cleanup.present
  eventually (Sys.act = QuarRename and Sys.res = Applied
              and finishSucceeded
              and some Quar.qRid and Slot.occ = Published and no Cleanup.present)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps


// ===========================================================================
// `FN-29` — A REFUSAL IS A COMPLETE OUTCOME
//
// TWO CONJUNCTS, AND THE SECOND IS WHERE THE CLAIM'S THIRD SENTENCE STOPS BEING
// TRUE BY CONSTRUCTION.  *Distinguishable by the operator from a block* is
// worth nothing stated over the outcome atoms — `RefRollbackNotCommitted` and
// `BlockedOutcome` are distinct signatures and no model can confuse them.  What
// an operator actually meets is a DISK, so the claim is stated over what the
// two outcomes leave on it: a completed refusal leaves no artifact of the
// transaction, and every block this file can reach leaves one.  That is a claim
// which can be false, and the mutation below makes it so.
//
// THE FIRST CONJUNCT NAMES THE ARTIFACTS THIS OUTCOME OWNS AND NOT EVERY
// ARTIFACT ON THE DISK, AND A COUNTEREXAMPLE IS WHY.  Written as
// `not leftoverArtifactNext`, the conjunct is FALSE: a quarantine standing at
// state 0 — one no transaction created and the restoration path never touches —
// survives the refusal, and the refusal is reported as incomplete because of
// bytes that were never its.  `EN-11`'s free initial state again, met on the
// widest of the three names.  The restoration path owns the reserved witness
// and the manifest, so those are stated as absences and the quarantine is
// stated as *not created*.  Retained in `README.md`.
//
// IT IS A PAIR WITH `FN-22.d` AND THE PAIR IS DELIBERATE, exactly as `FN-16`
// and the before-restoration row are one.  `FN-22.d` states the same three
// absences at ONE REVALIDATION POINT; this states them over the OUTCOME,
// wherever it arises.  In this file the two coincide because there is exactly
// one refusing exit — which is itself the finding `RefRollbackNotCommitted`
// was added for — and in a candidate protocol with two they do not.  Written
// apart so that a mutation to either is a control for one of them.
//
// AND THIS CLAIM HAS NO ISOLATING MUTATION, WHICH WAS ESTABLISHED BY TRYING
// THREE.  Aimed at the second conjunct it kills `FN-22.a` first, whose own third
// conjunct is *the witness is only ever released, on the rollback path, at the
// after-restoration point* — so a block that leaves nothing is a counterexample
// there before it is one here.  Aimed at the first conjunct's absences it is
// unsatisfiable against `fact BodyPhaseMatchesDisk`.  Aimed at the task root's
// own name it fires, and takes `FN-19`, `FN-22.d`, `FN-24.b` and `FN-28` with
// it.  Every conjunct this claim carries is another claim's subject; the claim
// is still checked and its mutation is still a control, but not for this claim
// alone.  `README.md` records it as a sixth way for a mutation to fail its aim.
//
// THE FINISH LEAF IS NOT DESCRIBED HERE.  The catalogue gives this claim a
// clause about the leaf being live and selectable and gives `FN-02` the same
// clause; `FN-02` owns it, and this claim owns the completeness and the
// distinction.  `disposal-k45`'s fourth rule, applied before the neighbour kill
// rather than after it.
//
// WHERE THE BOUNDS COME FROM.  Both antecedents name `Revalidate` and every
// blocking transition, whose deepest is `QuarReturn`'s incomplete return at
// twelve states; the witness lands at ten.  Thirteen is the larger.
// ===========================================================================

check FN_29_a_refusal_is_a_complete_outcome_and_is_distinguishable_from_a_block {
  always {
    (Sys.res' = RefRollbackNotCommitted)
      implies (some Root.rid' and no Slot.occ' and manEmptyNext
               and Quar.qRid' in Quar.qRid)
    (Sys.act' in txnActs and Sys.res' = BlockedOutcome)
      implies leftoverArtifactNext
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE CATALOGUE'S WITNESS: *a refused attempt followed by a successful one*.
   The refusal completes at the after-restoration point, leaving nothing of the
   transaction on disk; the world then lands the commit that was never proven,
   and a second attempt over the same handle succeeds.  Two attempts in one
   trace is what makes it the deepest of this claim's commands. */
run witness_FN_29_a_refused_attempt_followed_by_a_successful_one {
  interruptedMidEvacuation
  no Cleanup.present
  no Quar.qRid
  eventually (Sys.res' = RefRollbackNotCommitted and not leftoverArtifactNext)
  eventually (Sys.act = Classify and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps


// ===========================================================================
// `FN-30` — INTERNAL COMMITS RUN WITHOUT OPERATOR HOOKS
//
// THE ONE NEW OBSERVABLE IN THIS SLICE, AND THE ONLY `var` FIELD.  Everything
// that makes the claim falsifiable is at the signature: a hook is installed
// statically, `World.hookRan` says whether one has run, and exactly one
// transition in the file — `doTopologyChange`, the operator's own commit — does
// not frame the world.  Remove that one conjunct from `worldSame` and the
// claim is false; without the world's ability to run a hook at all, the claim
// would be true of a field nothing could set.
//
// WHERE THE BOUNDS COME FROM.  Conjunct (ii) names every member of `groveActs`,
// whose deepest is `MarkerRemove` at twelve states; the witness lands at nine.
// Thirteen is the larger.
// ===========================================================================

/* TWO CONJUNCTS, AND THE SECOND IS WIDER THAN THE CLAIM ON PURPOSE.
   
   (i) THE CLAIM'S OWN WORDS, over the two internal commits this file has.
   
   (ii) AND NO STEP OF GROVE'S RUNS ONE AT ALL.  The catalogue's REASON — *such
   a hook may mutate unrelated working-tree bytes that no index image restores*
   — is a reason about every step of a filesystem transaction and not only about
   the two that commit, and the file can state the wider fact for nothing.  It
   is stated as a second conjunct rather than instead of the first so that a
   candidate protocol with a third internal commit is caught by (i) at the
   grain the catalogue wrote it. */
check FN_30_no_user_supplied_hook_runs_during_an_internal_commit {
  always {
    (Sys.act' in internalCommitActs) implies World.hookRan' = World.hookRan
    (Sys.act' in groveActs)          implies World.hookRan' = World.hookRan
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* THE CATALOGUE'S WITNESS: *a hook that would have run, shown suppressed*, and
   it takes THREE conjuncts rather than one because *would have run* is a
   counterfactual and a model can only show it as a reachability.  A hook is
   installed; the operator's own commit runs it, so running is something this
   model can do; and Grove's internal commit happens first, with no hook having
   run.  Drop the middle conjunct and the command still lands — over a model in
   which no hook ever runs, which proves nothing at all. */
run witness_FN_30_a_hook_that_would_have_run_shown_suppressed {
  interruptedMidEvacuation
  no Repo.tickets
  no Cleanup.present
  some World.hookInstalled
  eventually (Sys.act = CommitAttempt and Sys.res = Applied
              and some Repo.tickets and no World.hookRan)
  eventually (Sys.act = TopologyChange and some World.hookRan)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps


// ===========================================================================
// `FN-32` — A TRANSACTION NEVER MUTATES AN ARTIFACT IT CANNOT PROVE IS ITS OWN
//
// `TT-24`'s SECOND CONTEXT, RE-STATED UNDER AN `FN-` PREFIX.  The obligation was
// `TT-24.c` and this file's sibling declared it `out-of-bounds` because the
// context it names is a finish context; the placement rule that moved it is
// `docs/adr/obligations-follow-context-not-artifact.md`.
//
// WHAT IT DOES NOT SAY IS AS DELIBERATE AS WHAT IT DOES.  `TT-24.c` said the
// step returns `Blocked(OwnershipConflict)`, and this file contradicts that at
// `FN_10b`, where an unclassifiable artifact at the witness name REFUSES.  The
// Quint column blocks at the same step.  So the OUTCOME is underdetermined by
// the catalogue and is not stated here; what both columns agree on, and what
// fail-closed ownership actually needs, is that the artifact does not move.
//
// TWO RESERVED NAMES, BECAUSE THEY ARE THE TWO THIS FILE CAN ASK OWNERSHIP OF.
// A witness slot occupied with no owner, and a cleanup marker with no `cOwner`,
// are both "at a name Grove reserves and not provably Grove's".  The QUARANTINE
// is deliberately absent: `foreignAtReservedName`'s second arm is true on the
// ordinary forward path between `QuarRename` and `MarkerCreate`, so the
// quarantine carries no ownership bit of its own and its case is the reaper's
// (`FN-21.c`).
//
// `Reap` IS EXCLUDED FROM THE ANTECEDENT and that is what keeps this obligation
// and `FN-21.c` separable: a mutation aimed at the sweep must not kill this,
// and one aimed at a transaction step must not kill the sweep.
//
// WHERE THE BOUNDS COME FROM.  The antecedent quantifies over `groveActs - Reap`,
// whose deepest member is `MarkerRemove` at twelve states, so thirteen — where
// `FN_27a` .. `FN_27c` already sit.  The witness lands at four: a foreign
// preparing witness met by a `Discard` is reachable from the free initial
// state, which is `EN-11`'s licence and is how `witness_FN_10b` reaches it.
// ===========================================================================

check FN_32_a_transaction_never_mutates_an_artifact_it_cannot_prove_is_its_own {
  always (Sys.act' in (groveActs - Reap) implies {
    (some Slot.occ and no Slot.owner) implies slotSame
    markerForeign implies markSame
  })
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 13 steps

/* Reached with BOTH artifacts present at once, which is what makes the witness
   say something the two conjuncts do not say apart: a foreign preparing witness
   and a foreign cleanup marker stand together, a transaction step meets them,
   and neither moves. */
run witness_FN_32_a_transaction_step_meets_an_unprovable_artifact_and_it_stands {
  Txn.phase = Fresh and Slot.occ = Preparing and no Slot.owner
  one Cleanup.present and no Cleanup.present.cOwner
  eventually (Sys.act = Discard and Sys.act in (groveActs - Reap)
              and Slot.occ = Preparing and no Slot.owner
              and one Cleanup.present and no Cleanup.present.cOwner)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps

/* THE MARKER HALF, WITNESSED AT THE ONE STEP THAT COULD MUTATE THE MARKER, and
   the reason it is a SECOND command rather than a stronger constraint on the
   first.  `witness_FN_32` above lands beside a `Discard`, and `doDiscard` frames
   the marker with an UNCONDITIONAL `markSame`: the foreign marker stands there
   because no step in that trace could move it, not because an ownership gate
   held.  A witness whose relevant step cannot mutate its subject is evidence
   about the framing and not about the claim, which is what
   `obligation-placement-k67` found.

   `MarkerReplace` is the only `groveActs - Reap` member whose marker mutation is
   gated on ownership — every other step frames the marker unconditionally — so
   this is where the claim has content, and the same state `witness_FN_31d`
   reaches.  DELIBERATELY THE SAME STATE: `FN-31.d` says it as incumbent
   mechanics, `FN-32` says it as shared safety, and the class is the whole
   difference (`docs/specs/semantic-contract.md`, the class register). */
run witness_FN_32_a_transaction_step_meets_an_unprovable_marker_and_it_stands {
  interruptedMidEvacuation
  no Quar.qRid
  no Repo.tickets
  one Cleanup.present
  no Cleanup.present.cOwner
  eventually (Sys.act' = MarkerReplace and markerForeign
              and Sys.act' in (groveActs - Reap) and markSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 11 steps


// ===========================================================================
// `EN-08` — INTERRUPTION MAY OCCUR BETWEEN ANY TWO STEPS.  Class:
// EXERCISE-REMOVAL, and the thing removed is `crash` itself.  The assumption
// table's expected result is that every NAMED witness becomes unreachable and
// the run fails on zero work rather than reporting green.
//
// Three commands, run against the named witness sets rather than against the
// whole file: `FN-24`'s step-boundary sweep, `FN-09`'s and `FN-10`'s
// interruptions inside the build, and `FN-31.c`'s two resumptions.  The third
// is a FINDING and is recorded in `README.md`: `FN-31.c`'s witnesses POSIT the
// disk an interruption leaves rather than running `crash` to reach it, so they
// keep landing with `crash` removed.  The assumption table names `FN-31.c`
// among `EN-08`'s controlled obligations and this file's realisation of it does
// not depend on the action — which is exactly the kind of thing an
// exercise-removal exists to make visible, and it is invisible without one.
// ===========================================================================

run expect_unreachable_EN_08_no_step_boundary_is_an_interruption_point {
  always Sys.act != Crash
  freshGroveDisk
  crashAfter[WPublish]
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps

run expect_unreachable_EN_08_no_interruption_inside_the_build_is_reachable {
  always Sys.act != Crash
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = WManifest and Slot.occ = Preparing)
  eventually (Sys.act = Crash and Slot.occ = Preparing and no Slot.wHolds
              and manWritten)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 7 steps

run expect_unreachable_EN_08_no_unpublished_witness_is_discarded_after_an_interruption {
  always Sys.act != Crash
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = WPrepare and Slot.occ = Preparing)
  eventually (Sys.act = Crash and Slot.occ = Preparing)
  eventually (Sys.act = Discard and Sys.res = Applied)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps



// ===========================================================================
// THE ASSUMPTION MUTATION THIS FILE RUNS
//
// `EN-02` — a rename cannot cross a filesystem boundary.  Class:
// EXERCISE-REMOVAL, so the expected result is that a NAMED WITNESS becomes
// unreachable while the property check stays green.  A green `FN-08` under one
// device is the expected result and not a survivor; what would be a finding is
// the witness still landing, because then it was never exercising the boundary.
// ===========================================================================

run expect_unreachable_EN_02_a_single_device_layout_cannot_fail_the_operand_gate {
  some Txn.leaseOk
  World.wtDev = World.qDev
  World.rootDev != World.qDev
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine)
} for 3 but 1 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 4 steps


// ===========================================================================
// `EN-09` — a command's exit status is not a receipt: a result may be lost or
// arrive late.  Class: EXERCISE-REMOVAL, and the dimension removed is the LATE
// ARRIVAL: the same trace as `witness_FN_15a`, with `ResultArrives` gone.
//
// The expected result is that the witness becomes unreachable while every
// property check stays green — a failure report can still be produced AT the
// attempt, but it can no longer arrive after a classification that never read
// it, which is the exact situation `FN-15.a` is stated about.
// ===========================================================================

run expect_unreachable_EN_09_a_failure_cannot_be_reported_after_a_classification_without_a_late_arrival {
  interruptedMidEvacuation
  no Repo.tickets
  always Sys.act != ResultArrives
  eventually (Sys.act = CommitAttempt and some Repo.tickets and no Txn.report)
  eventually (Sys.act = Classify and Txn.disp = Committed and no Txn.report)
  eventually (Sys.act = ResultArrives and Txn.report = FailReport
              and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps


// ===========================================================================
// `EN-16` — THE THREE LANES DIFFER IN MECHANISM AND AGREE ON ABSTRACT OUTCOME.
// Class: EXERCISE-REMOVAL, and the dimension removed is the LANE ITSELF: the
// lane is a model parameter, and the mutation collapses it to one.
//
// The assumption table's expected result is that `FN-25.c`'s per-lane witnesses
// become unreachable and `FN-17`'s working-copy-as-commit obligation has no
// instance, WHILE EVERY `FN-` PROPERTY STAYS GREEN.  That last clause is the
// whole point of the control: a lane-blind model passes every check in this
// file, so nothing but an unreachability says the dimension was ever exercised.
//
// The collapse is `always World.lane = GitL` and the three commands below run
// against the NAMED WITNESS SETS rather than the whole file — `FN-25.c`'s six,
// `FN-17`'s two, and `FN-15.b` .. `FN-15.d`'s nine.  Each asks for the
// witness's own content on a lane the collapse has removed, which makes it
// INEXPRESSIBLE rather than false; that is `EN-02`'s shape and the reason an
// exercise-removal's expected result is *no instance* rather than *a failure*.
//
// THE NAMED SET WAS CHECKED AGAINST THE FILE AND IS EXACT — `crash-k47` found
// `EN-08`'s overstated its reach by one obligation, so this one was asked the
// same question.  Eleven commands in this file pin a lane and every one of them
// answers `FN-15.b`, `FN-15.c`, `FN-15.d` or `FN-17`; the six this slice adds
// answer `FN-25.c`; no other command's content depends on which lane is
// selected, because `preflightCommitReproduced`, `canReproduceHere` and
// `reproductionStands` are the only three lane-sensitive predicates in the file
// and all three are vacuous on `GitL`.  `README.md` records the count.
// ===========================================================================

run expect_unreachable_EN_16_no_diagnosis_is_reached_on_a_second_lane {
  always World.lane = GitL
  interruptedMidEvacuation
  no Cleanup.present
  eventually (Sys.res' = BlockedOutcome and diagnosed = DRecoveryPending
              and World.lane in wcAsCommitLanes)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 9 steps

run expect_unreachable_EN_16_no_working_copy_as_commit_restoration_exists {
  always World.lane = GitL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Settle and Sys.res = Applied
              and World.lane in wcAsCommitLanes
              and Repo.reproduced = Txn.anchor)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 10 steps

run expect_unreachable_EN_16_no_disposition_is_reached_on_a_second_lane {
  always World.lane = GitL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and some Txn.disp and World.lane != GitL)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark, 8 steps
