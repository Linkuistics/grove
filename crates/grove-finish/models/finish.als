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
 * COVERAGE SO FAR: FN-01, FN-05 .. FN-08 — the transaction's ENTRY surface —
 * and FN-09 .. FN-13, the RESERVED WITNESS.  Every other `FN-` obligation
 * belongs to a sibling leaf of `finish-k8` and the runner reports its cell
 * empty, which is the truth about this file rather than a defect in it.
 *
 * WHY THESE TWO SLICES AND NOT A LAYER.  The entry surface ends in a refusal or
 * in a transaction that is never entered.  The witness slice adds the six steps
 * that build and publish the witness and evacuate the tree into it, plus the two
 * things a step list is for: `crash` between any two steps, and `discard` as the
 * recovery an unpublished witness admits.  It stops at the point a commit is
 * ATTEMPTED: no commit, no disposition, no rollback, no quarantine, no reaper.
 * That is what makes it verifiable on its own.  It is also what a green run here
 * does NOT prove — see `README.md`, which says so in more detail than this
 * header can.
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
  qDev:    one Device    // the quarantine parent's device  — the other one
}

one sig Root { var rid: lone RootId, var holds: set Entry }

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
one sig Repo { var rev: one Rev, var tracked: set Entry, var wTracked: lone Tracked }

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
        Evacuated extends Phase {}
one sig Verdict {}
one sig Txn {
  var phase: one Phase,
  var pinned: lone RootId,      // the identity pinned at open, rechecked at every later step
  var leaseOk: lone Verdict,    // the LEASE gate's recorded verdict — never a licence
  var attempt: lone AttemptId,  // drawn once per launch; what a rootless retry accepts by
  var anchor:  lone Rev         // the recorded starting topology a rollback must find
}


// ===========================================================================
// ACTIONS, OUTCOMES, AND THE SEVEN PRECONDITIONS
// ===========================================================================

abstract sig Action {}
one sig Idle, Confirm, Decline, TxnOpen, Preflight, Swap, TopologyChange,
        WPrepare, WManifest, WReady, WPublish, WEvacuate, CommitAttempt,
        Crash, Discard extends Action {}

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
  p = W10SlotForeign   implies RefReservedNameOccupied else none
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
pred treeSame  { rootSame and slotSame and manSame }
pred repoSame  { Repo.rev' = Repo.rev and Repo.tracked' = Repo.tracked
                 and Repo.wTracked' = Repo.wTracked }
pred worldSame { World.lane' = World.lane }
pred opSame    { Op.confirmed' = Op.confirmed }
pred txnSame   { Txn.phase' = Txn.phase and Txn.pinned' = Txn.pinned
                 and Txn.leaseOk' = Txn.leaseOk
                 and Txn.attempt' = Txn.attempt and Txn.anchor' = Txn.anchor }
/* The transaction's volatile state, carried across a body step unchanged: the
   phase is what each step advances and is written by the step itself. */
pred txnCarried { Txn.pinned' = Txn.pinned and Txn.leaseOk' = Txn.leaseOk
                  and Txn.attempt' = Txn.attempt and Txn.anchor' = Txn.anchor }
/* The transaction is gone: a crash, or a refusal that leaves nothing behind.
   THE LEASE VERDICT IS NOT PART OF IT.  It is recorded by the driver's own gate
   BEFORE the transaction opens (`FN-08`), so a transaction that ends does not
   un-record it — and `doPreflight` frames it explicitly, which is what makes
   clearing it here a contradiction rather than a choice.  It was one, once: the
   preflight's whole refusal branch became unsatisfiable and eight of
   `entry-k39`'s fourteen witnesses reported *no instance* in the same run. */
pred txnGone   { Txn.phase' = Fresh and no Txn.pinned'
                 and no Txn.attempt' and no Txn.anchor'
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
  /* The attempt identity is DRAWN here — once per launch — and the anchor is the
     repository's topology as it stands at open.  Neither is read by anything in
     this slice except the manifest `FN-12.a` requires; the classification that
     reads them back is the `commit` sibling's. */
  some Txn.attempt' and Txn.anchor' = Repo.rev
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
  Root.holds' = Root.holds and slotSame and manSame
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
  rootSame and repoSame and worldSame and opSame
  no Slot.occ implies {
    Sys.res' = Applied and noWhy
    Slot.occ' = Preparing and Slot.owner' = Txn.attempt and no Slot.wHolds'
    manEmptyNext
    Txn.phase' = Prepared and txnCarried
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
  rootSame and slotSame and repoSame and worldSame and opSame
  Txn.phase' = Manifested and txnCarried
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
  rootSame and slotSame and repoSame and worldSame and opSame
  Txn.phase' = ReadyP and txnCarried
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
  rootSame and manSame and repoSame and worldSame and opSame
  Txn.phase' = PublishedP and txnCarried
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
  manSame and repoSame and worldSame and opSame
  (no Root.holds') implies Txn.phase' = Evacuated else Txn.phase' = PublishedP
  txnCarried
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
  treeSame and repoSame and worldSame and opSame and txnSame
  (gateEvacuated and gateWitnessUntracked) implies {
    Sys.res' = Applied and noWhy
  } else {
    not gateWitnessUntracked implies {
      Sys.res' = RefWitnessPending and Sys.why' = W8WitnessTracked
    } else {
      Sys.res' = RefWitnessPending and Sys.why' = W9SlotPending
    }
  }
}

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
  rootSame and worldSame and opSame and txnSame
  Repo.rev' = Repo.rev and Repo.tracked' = Repo.tracked
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

pred step {
  doIdle or doConfirm or doDecline or doTxnOpen or doPreflight
  or doSwap or doTopologyChange
  or doWPrepare or doWManifest or doWReady or doWPublish or doWEvacuate
  or doCommitAttempt
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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 9 steps

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
