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
 * FN-09 .. FN-13, the RESERVED WITNESS; and FN-03, FN-04, FN-14 .. FN-18, the
 * COMMIT AND ITS DISPOSITION.  Every other `FN-` obligation belongs to a sibling
 * leaf of `finish-k8` — `handoff` and `exits` — and the runner reports its cell
 * empty, which is the truth about this file rather than a defect in it.
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
  var wcWork: set Entry
}

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
        Settled extends Phase {}
one sig Verdict {}

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
        QuarRename extends Action {}

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
}

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

/* `FN-14`'s scope, over the state the commit moves.  *Exactly the expected
   deletions at their original paths* is the fingerprint leaving the tracked set
   and nothing else leaving it; *unrelated working-copy work SHALL survive* is
   `wcWork` untouched. */
pred commitIsScoped {
  Repo.tracked' = Repo.tracked - Man.mFinger
  World.wcWork' = World.wcWork
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
/* The root's ENTRIES alone.  `doQuarRename` frames them in both branches and
   moves the identity in one, so it cannot use `rootSame`. */
pred rootSameHolds { Root.holds' = Root.holds }
pred treeSame  { rootSame and slotSame and manSame and quarSame }
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
pred worldSame { World.lane' = World.lane and World.wcWork' = World.wcWork }
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
  Root.holds' = Root.holds and slotSame and manSame and quarSame
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
  treeSame and opSame and txnSame
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
  rootSame and quarSame and repoSame and worldSame and opSame
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
  rootSame and slotSame and quarSame and repoSame and worldSame and opSame
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
  rootSame and slotSame and quarSame and repoSame and worldSame and opSame
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
  rootSame and manSame and quarSame and repoSame and worldSame and opSame
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
  manSame and quarSame and repoSame and worldSame and opSame
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
  rootSame and quarSame and worldSame and opSame and txnSame
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

   IT IS RE-RUNNABLE, INCLUDING AFTER A SETTLE, and that is not an oversight:
   `FN-03`'s witness is a retry that has lost every artifact the transaction
   owned, and the state after a forward settle is exactly that tree.  A second
   classification there reads the ticket and nothing else. */
pred doClassify {
  some Op.confirmed
  Txn.phase in (Attempted + Classified + Settled)
  Sys.act' = Classify and Sys.res' = Applied and noWhy
  treeSame and repoSame and worldSame and opSame
  (Txn.phase = Settled) implies Txn.phase' = Settled else Txn.phase' = Classified
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
  rootSameHolds and repoSame and worldSame and opSame
  no Quar.qRid implies {
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
  /* THE FORWARD SETTLE IS NO LONGER AVAILABLE AT `Classified`.  A `Committed`
     classification renames first (`FN-19`) and settles after, so the settle's
     forward branch is guarded by the phase the rename produced.  That is the
     phase machine doing what it has done since the witness slice — each step's
     guard is the phase its predecessor produced — and it is why a `Committed`
     at `Classified` enables nothing here rather than blocking. */
  (Txn.phase = Quarantined) or (Txn.phase = Classified and Txn.disp != Committed)
  Sys.act' = Settle
  worldSame and opSame
  Txn.phase = Quarantined implies {
    /* FORWARD, and it is now the disposal of a quarantine rather than a release
       in place: the root left in one rename and what remains is the quarantine
       holding it.  This step is STILL AN ABSTRACTION of disposal — nothing here
       claims it is re-entrant, marker-guarded or bounded to Grove's own, which
       are `FN-21` and `FN-31` and are the `disposal` sibling's. */
    Sys.res' = Applied and noWhy
    rootSame
    no Quar.qRid'
    no Slot.occ' and no Slot.owner' and no Slot.wHolds' and manEmptyNext
    repoSameReleasingWitness
    Txn.phase' = Settled and txnCarried and txnResultSame
  } else {
    quarSame
    (Txn.disp = NotCommitted and rollbackLicensed) implies {
      canReproduceHere implies {
        Sys.res' = RefRollbackNotCommitted and Sys.why' = W11NotCommitted
        treeMatchesManifest and Root.rid' = Root.rid
        no Slot.occ' and no Slot.owner' and no Slot.wHolds' and manEmptyNext
        Repo.rev' = Repo.rev and Repo.tracked' = Repo.tracked
        Repo.tickets' = Repo.tickets and no Repo.wTracked'
        Repo.canReproduce' = Repo.canReproduce
        preflightCommitReproduced
        (World.lane not in wcAsCommitLanes) implies Repo.reproduced' = Repo.reproduced
        Txn.phase' = Settled and txnCarried and txnResultSame
      } else {
        Sys.res' = BlockedOutcome and Sys.why' = W13CannotReproduce
        treeSame and repoSame
        txnGone
      }
    } else {
      Sys.res' = BlockedOutcome and Sys.why' = W12Indeterminate
      treeSame and repoSame
      txnGone
    }
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
    some Txn.disp   implies Txn.phase in (Classified + Quarantined + Settled)
    some Txn.report implies Txn.phase in (Attempted + Classified
                                          + Quarantined + Settled)
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
       honest. */
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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

/* The transaction never entered for want of confirmation — with the
   DETERMINISTIC guard `FN-01` names (a live finish leaf, no live ordinary work)
   holding, so the trace says confirmation and not the guard. */
run witness_FN_01a_a_transaction_never_entered_for_want_of_confirmation {
  always no Op.confirmed
  always Txn.phase = Fresh
  gateWork
  eventually Sys.act = Decline
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

/* FN-01.b.  The other direction, and the non-redundant one: a CONFIRMED attempt
   whose deterministic guard fails is still refused.  Confirmation is not a
   substitute for the guard any more than the guard is for it. */
check FN_01b_confirmation_is_not_a_substitute_for_the_deterministic_guard {
  always ((Sys.act' = Preflight and some Op.confirmed and not pre2Work)
            implies Sys.res' in Refused)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

run witness_FN_05a_p1_confirmation_absent {
  eventually (Sys.act = Decline and Sys.why = P1Confirm)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

run witness_FN_05a_p2_no_live_finish_leaf_or_live_ordinary_work {
  eventually (Sys.act = Preflight and Sys.why = P2Work and Sys.res = RefNotLive)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

/* The layout is unsupported AT THE PREFLIGHT, having been supported at the
   lease gate — which is `SY-03` stated as a trace rather than as prose. */
run witness_FN_05a_p3_layout_unsupported {
  eventually (Sys.act = TopologyChange and no World.lane)
  eventually (Sys.act = Preflight and Sys.why = P3Layout
              and Sys.res = RefLayoutUnsupported)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

run witness_FN_05a_p4_quarantine_target_unreachable {
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine
              and Sys.res = RefLayoutUnsupported)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

run witness_FN_05a_p5_task_root_identity_unverified {
  eventually (Sys.act = Preflight and Sys.why = P5Identity
              and Sys.res = RefRootIdentityChanged)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

run witness_FN_05a_p6_empty_deletion_fingerprint {
  eventually (Sys.act = Preflight and Sys.why = P6Fingerprint
              and Sys.res = RefNoTrackedDeletion)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

run witness_FN_05a_p7_an_entry_type_that_cannot_be_digested {
  eventually (Sys.act = Preflight and Sys.why = P7EntryType
              and Sys.res = RefUnsupportedEntryType)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

run witness_FN_05b_a_refusal_with_the_tree_unchanged {
  eventually (some Sys.why and Sys.act in (Preflight + Decline))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

check FN_05c_a_failed_precondition_leaves_the_repository_byte_identical {
  always ((some Sys.why' and Sys.act' in (Preflight + Decline)) implies repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

/* The repository is exercised in the same trace that refuses — a topology
   change moves it, and the preflight step does not.  A witness that only
   reached a refusal would be equally consistent with a model whose repository
   cannot change at all. */
run witness_FN_05c_a_refusal_with_the_repository_unchanged {
  eventually (Sys.act' = TopologyChange and Repo.rev' != Repo.rev)
  eventually (some Sys.why' and Sys.act' = Preflight and repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

// --- FN-06: the task root's identity is pinned and rechecked ----------------

/* A mid-transaction swap is a REFUSAL rather than a mutation applied elsewhere:
   the pinned identity is rechecked at the later step, and the tree the
   transaction was pointed at is left byte-identical. */
check FN_06_the_task_roots_identity_is_pinned_and_rechecked {
  always ((Sys.act' = Preflight and Root.rid != Txn.pinned)
            implies (Sys.res' in Refused and treeSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

/* Two consecutive transitions, which is why this one runs at four states: the
   swap, then the preflight that catches it. */
run witness_FN_06_a_swap_between_two_steps_is_refused {
  eventually (Sys.act = Swap and after (Sys.act = Preflight and Sys.why = P5Identity))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

// --- FN-07: an untracked tree is refused before evacuation ------------------

check FN_07_an_empty_deletion_fingerprint_is_refused_before_any_mutation {
  always ((Sys.act' = Preflight and no (Root.holds & Repo.tracked))
            implies (Sys.res' in Refused and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

run witness_FN_07_a_wholly_untracked_tree {
  eventually (Sys.act = Preflight and no Repo.tracked and some Root.holds
              and Sys.res = RefNoTrackedDeletion)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

// --- FN-08: the quarantine target is proved reachable before mutation -------

/* The lease gate's verdict proves `wtDev = qDev` and nothing else.  This check
   says entry is never granted on it: the transaction's OWN operands — the task
   root and the quarantine parent — must agree, whatever the earlier gate found. */
check FN_08_the_lease_gates_verdict_never_licenses_the_transactions_operands {
  always ((Sys.act' = Preflight and Sys.res' = Applied)
            implies World.rootDev = World.qDev)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

/* A layout that passes at lease acquisition and fails here.  It needs two
   devices, and that is exactly what `EN-02` removes below. */
run witness_FN_08_a_layout_that_passes_at_lease_acquisition_and_fails_here {
  some Txn.leaseOk
  World.wtDev = World.qDev
  World.rootDev != World.qDev
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps



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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

/* The licence every body witness below runs on, and the file's first `Applied`
   preflight.  Two consecutive transitions, so four states. */
run witness_FN_09a_the_transaction_is_entered_by_a_preflight {
  eventually Sys.act = TxnOpen
  eventually (Sys.act = Preflight and Sys.res = Applied and Txn.phase = Entered)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

/* An interruption IMMEDIATELY AFTER publication — the state `EN-01` is the only
   reason is not a torn one.  Five transitions: prepare, manifest, ready,
   publish, crash. */
run witness_FN_09a_an_interruption_immediately_after_publication {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = WPublish and Slot.occ = Published
              and after (Sys.act = Crash and Slot.occ = Published))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

run witness_FN_09b_an_interruption_inside_the_build {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = WManifest and Slot.occ = Preparing)
  eventually (Sys.act = Crash and Slot.occ = Preparing and no Slot.wHolds
              and manWritten)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

run witness_FN_10a_a_discard {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = Crash and Slot.occ = Preparing)
  eventually (Sys.act = Discard and Sys.res = Applied and no Slot.occ)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

/* FN-10.b.  Content the discard cannot classify as Grove's own fails closed:
   refused, with the tree and the repository byte-identical.  It is the
   `ReservedNameOccupied` half of the catalogue's split, and it names no
   recovery — telling an operator to run one against someone else's bytes is the
   fail-closed violation the split exists to prevent. */
check FN_10b_content_the_discard_cannot_classify_fails_closed {
  always ((Sys.act' = Discard and not gateOwned)
            implies (Sys.res' in Refused and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

/* Reached from the free initial state and not from a run-up, because a foreign
   artifact at a reserved name is TREE state and `EN-11` is exactly the licence
   for it. */
run witness_FN_10b_a_refusal_to_discard_unclassifiable_content {
  Txn.phase = Fresh and Slot.occ = Preparing and no Slot.owner
  eventually (Sys.act = Discard and Sys.res = RefReservedNameOccupied
              and Sys.why = W10SlotForeign and Slot.occ = Preparing)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

// --- FN-11: evacuation precedes deletion ------------------------------------

/* Every ordinary root entry is inside the PUBLISHED witness, beneath a manifest
   written and verified, before any commit is attempted.  `doCommitAttempt` is
   deliberately enabled at `PublishedP` as well as at `Evacuated`, so the early
   attempt is a REACHABLE refusal rather than an absent transition and this check
   has an antecedent it could fail on. */
check FN_11_evacuation_precedes_any_attempted_commit {
  always ((Sys.act' = CommitAttempt and Sys.res' = Applied)
            implies evacuationComplete)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

/* A manifest interrupted before its ready mark, RECOVERED AS NOT READY: the
   crash leaves a written, unmarked manifest inside a preparing witness, and what
   the next invocation does with it is discard it — never read it. */
run witness_FN_12a_a_manifest_interrupted_before_its_ready_mark {
  Txn.phase = Fresh and no Slot.occ
  eventually (Sys.act = Crash and Slot.occ = Preparing
              and manWritten and no Man.mReady)
  eventually (Sys.act = Discard and Sys.res = Applied and no Man.mReady)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps

run witness_FN_12b_a_refused_entry_type {
  eventually (Sys.act' = Preflight and (some e: Root.holds | e.et = OpaqueT)
              and Sys.res' = RefUnsupportedEntryType and Sys.why' = P7EntryType
              and treeSame and repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 3 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps


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
check FN_03_the_ticket_is_the_durable_record_and_outlives_the_artifacts {
  always {
    Repo.tickets in Repo.tickets'
    (Sys.act' = Classify and Txn.disp' = Committed)
      implies Txn.attempt in Txn.handle.(Repo.tickets)
    (Sys.act' = Classify and no Slot.occ and manEmpty
       and Txn.attempt in Txn.handle.(Repo.tickets))
      implies Txn.disp' = Committed
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

/* A retry with no local trace of the attempt, settling forward on the ticket
   alone.  The forward settle releases every artifact the transaction owns, and
   the classification that follows it reads an empty slot, an empty manifest and
   a ticket. */
run witness_FN_03_a_retry_with_no_local_trace_settling_forward_on_the_ticket_alone {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Settle and Sys.res = Applied and no Slot.occ and manEmpty)
  eventually (Sys.act = Classify and Txn.disp = Committed
              and Txn.phase = Settled and no Slot.occ and manEmpty)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

// --- FN-14: the commit is scoped --------------------------------------------

/* Whenever a ticket lands, exactly the recorded fingerprint left the tracked set
   and the unrelated working-copy work is untouched.  Stated over the ticket
   rather than over the step, so a future step that commits is caught by the same
   sentence. */
check FN_14_the_commit_records_exactly_the_expected_deletions {
  always ((some (Repo.tickets' - Repo.tickets)) implies commitIsScoped)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

/* Unrelated modified work present across a successful finish — present before
   the commit, present after it, and never part of what the deletion recorded. */
run witness_FN_14_unrelated_modified_work_present_across_a_successful_finish {
  interruptedMidEvacuation
  no Repo.tickets
  always some World.wcWork
  eventually (Sys.act = CommitAttempt and some Repo.tickets)
  eventually (Sys.act = Classify and Txn.disp = Committed and some World.wcWork)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

/* FN-15.b .. FN-15.d are REACHABILITY obligations, so each one's check states
   the other half — what the disposition is reached ON.  A model in which
   `Committed` were reachable without a proven result would satisfy the witness
   and fail the check, which is the pair the catalogue asks for. */
check FN_15b_committed_is_reached_only_on_a_proven_result {
  always ((Sys.act' = Classify and Txn.disp' = Committed) implies resultProven)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

run witness_FN_15b_nativejj_committed_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

run witness_FN_15b_colocatedjj_committed_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = Committed)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

check FN_15c_notcommitted_is_reached_only_with_the_anchor_intact_and_no_result {
  always ((Sys.act' = Classify and Txn.disp' = NotCommitted)
            implies (anchorHolds and not resultProven))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

run witness_FN_15c_git_notcommitted_reached {
  always World.lane = GitL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

run witness_FN_15c_nativejj_notcommitted_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

run witness_FN_15c_colocatedjj_notcommitted_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Classify and Txn.disp = NotCommitted)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 7 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

run witness_FN_15d_git_indeterminate_reached {
  always World.lane = GitL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Classify and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

run witness_FN_15d_nativejj_indeterminate_reached {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Classify and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

run witness_FN_15d_colocatedjj_indeterminate_reached {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = TopologyChange and Txn.phase = Attempted
              and Repo.rev != Txn.anchor)
  eventually (Sys.act = Classify and Txn.disp = Indeterminate)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

// --- FN-16: rollback is licensed only by proof ------------------------------

/* A RESTORATION IS IDENTIFIED STRUCTURALLY — entries coming back into the task
   root — rather than by the action that did it.  That is deliberate: the claim
   is about restoration, not about `Settle`, and a future step that put entries
   back by another route would be caught by the same sentence.  Nothing else in
   this file ever grows `Root.holds`. */
check FN_16a_restoration_is_refused_when_the_recorded_anchor_no_longer_holds {
  always ((some (Root.holds' - Root.holds)) implies anchorHolds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

check FN_16b_restoration_is_refused_when_the_attempt_bound_result_is_present {
  always ((some (Root.holds' - Root.holds)) implies not resultProven)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

/* Reached: the attempt-bound result IS present — the ticket landed — so the
   settle goes forward and no entry ever comes back. */
run witness_FN_16b_a_settle_with_the_attempt_bound_result_present_restores_nothing {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = CommitAttempt and some Repo.tickets)
  eventually (Sys.act = Settle and Sys.res = Applied
              and no Root.holds and no Slot.occ)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

// --- FN-17: rollback is exact -----------------------------------------------

/* Two conjuncts, and the second is what *before the witness is removed* is worth
   in a model whose settle is one step: the removal is CONDITIONED on the
   reproduction, so a restoration that skipped it could not release the witness.
   Whether the step must itself be decomposed — one persistent effect per step —
   is `FN-24.b`'s and is the `exits` sibling's; `README.md` records that this
   file states the ordering as a conjunction and does not check it as one. */
check FN_17a_a_restoration_matches_the_manifest_and_reproduces_the_preflight_commit {
  always {
    (some (Root.holds' - Root.holds))
      implies (treeMatchesManifest and preflightCommitReproduced)
    (Sys.act' = Settle and Txn.disp = NotCommitted and some Slot.occ and no Slot.occ')
      implies (treeMatchesManifest and preflightCommitReproduced)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

/* A restoration that reproduces it, on a working-copy-as-commit lane — which is
   the only obligation in this slice that reads the lane at all. */
run witness_FN_17a_a_restoration_that_reproduces_the_exact_preflight_commit {
  always World.lane = NativeJjL
  interruptedMidEvacuation
  no Repo.tickets
  always some Repo.canReproduce
  eventually (Sys.act = Settle and Sys.res = RefRollbackNotCommitted
              and some Root.holds and no Slot.occ
              and Repo.reproduced = Txn.anchor)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

/* FN-17.b.  A restoration that cannot reproduce it BLOCKS rather than proceeds:
   the outcome is `Blocked`, and the tree and the repository are byte-identical,
   so the witness still stands and a later recovery has something to read. */
check FN_17b_a_restoration_that_cannot_reproduce_it_blocks_rather_than_proceeds {
  always ((Sys.act' = Settle and Txn.disp = NotCommitted
             and World.lane in wcAsCommitLanes and no Repo.canReproduce)
            implies (Sys.res' = BlockedOutcome and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

run witness_FN_17b_a_restoration_that_cannot_reproduce_it_blocks {
  always World.lane = ColocatedJjL
  interruptedMidEvacuation
  no Repo.tickets
  always no Repo.canReproduce
  eventually (Sys.act = Settle and Sys.res = BlockedOutcome
              and Sys.why = W13CannotReproduce
              and Slot.occ = Published and no Root.holds)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

// --- FN-18: forward recovery never restores ---------------------------------

/* Two conjuncts.  A forward settle puts nothing back and empties the witness
   rather than unpacking it; and once THIS attempt's commit is proven, no later
   state in the trace ever grows the task root again. */
check FN_18_a_proven_commit_is_never_followed_by_a_reconstruction {
  always {
    (Sys.act' = Settle and Txn.disp = Committed)
      implies (rootSame and no Slot.wHolds')
    (resultProven and once (Sys.act = Classify and Txn.disp = Committed))
      implies no (Root.holds' - Root.holds)
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

/* A proven commit reached after an interruption mid-evacuation: the recovery
   adopts the interrupted attempt, finishes the evacuation, commits, proves it,
   and settles FORWARD — the tree is never reconstructed. */
run witness_FN_18_a_proven_commit_reached_after_an_interruption_mid_evacuation {
  interruptedMidEvacuation
  no Repo.tickets
  eventually (Sys.act = Recover and Txn.phase = PublishedP and some Root.holds)
  eventually (Sys.act = Classify and Txn.disp = Committed and no Root.holds)
  eventually (Sys.act = Settle and Sys.res = Applied
              and no Root.holds and no Slot.occ)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps


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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 10 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 8 steps


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
} for 3 but 1 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 4 steps


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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps
