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
 * COVERAGE SO FAR: FN-01 and FN-05 .. FN-08 — the transaction's ENTRY surface.
 * Every other `FN-` obligation belongs to a sibling leaf of `finish-k8` and the
 * runner reports its cell empty, which is the truth about this file rather than
 * a defect in it.
 *
 * WHY THE ENTRY SURFACE IS A SLICE AND NOT A LAYER.  Every obligation here ends
 * in a refusal or in a transaction that is never entered, so the file needs the
 * transaction's entry and none of its body: no witness species, no evacuation,
 * no commit, no disposition, no quarantine.  That is what makes it verifiable on
 * its own.  It is also what a green run here does NOT prove — see `README.md`,
 * which says so in more detail than this header can.
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

sig Entry { et: one EType, role: one Role }

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

/* The reserved name a witness would be built at.  Nothing in this slice ever
   occupies it — which is exactly what `FN-05.b` is about, and why the slot is
   in the signature rather than deferred to the `witness` sibling: a claim that
   preflight mutates nothing needs something preflight COULD have mutated. */
one sig Slot { var occ: lone Reserved }
one sig Reserved {}

one sig Repo { var rev: one Rev, var tracked: set Entry }

/* Confirmation is an operator input Grove cannot verify (`EN-15`).  It is set
   by the world's own action and by nothing else, which is the checkable half of
   *and is never attested*. */
one sig Op { var confirmed: lone Confirmation }
one sig Confirmation {}

abstract sig Phase {}
one sig Fresh, Opened, Entered extends Phase {}
one sig Verdict {}
one sig Txn {
  var phase: one Phase,
  var pinned: lone RootId,    // the identity pinned at open, rechecked at every later step
  var leaseOk: lone Verdict   // the LEASE gate's recorded verdict — never a licence
}


// ===========================================================================
// ACTIONS, OUTCOMES, AND THE SEVEN PRECONDITIONS
// ===========================================================================

abstract sig Action {}
one sig Idle, Confirm, Decline, TxnOpen, Preflight, Swap, TopologyChange extends Action {}
/* The transaction's own steps.  `FN-01.a` is stated over exactly this set. */
fun txnActs: set Action { TxnOpen + Preflight }

abstract sig Result {}
one sig Applied, NoOp, Environmental extends Result {}
abstract sig Refused extends Result {}
/* Five members of the catalogue's closed refusal-reason set.  The set is the
   catalogue's; this file adds none. */
one sig RefNotLive, RefLayoutUnsupported, RefRootIdentityChanged,
        RefNoTrackedDeletion, RefUnsupportedEntryType extends Refused {}

/* A MODEL-ONLY OBSERVABLE, and declared as one in `README.md`.  The catalogue
   fixes seven preconditions and seventeen refusal reasons and never states the
   mapping between them; two of the seven — an unsupported layout and an
   unreachable quarantine operand — are the same reason at different gates.  So
   the reason alone cannot say WHICH member refused, and `FN-05.a` requires the
   seven to be individually reachable.  `why` is what makes them so.  Nothing in
   the shipped contract corresponds to it. */
abstract sig Precondition {}
one sig P1Confirm, P2Work, P3Layout, P4Quarantine, P5Identity, P6Fingerprint,
        P7EntryType extends Precondition {}

one sig Sys { var act: one Action, var res: one Result, var why: lone Precondition }

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

fun failedPre: set Precondition {
  { p: Precondition |
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
fun reasonOf[p: Precondition]: lone Refused {
  p = P2Work        implies RefNotLive             else
  p = P3Layout      implies RefLayoutUnsupported   else
  p = P4Quarantine  implies RefLayoutUnsupported   else
  p = P5Identity    implies RefRootIdentityChanged else
  p = P6Fingerprint implies RefNoTrackedDeletion   else
  p = P7EntryType   implies RefUnsupportedEntryType else none
}

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


// ===========================================================================
// FRAMING
// ===========================================================================

pred treeSame  { Root.holds' = Root.holds and Root.rid' = Root.rid and Slot.occ' = Slot.occ }
pred repoSame  { Repo.rev' = Repo.rev and Repo.tracked' = Repo.tracked }
pred worldSame { World.lane' = World.lane }
pred opSame    { Op.confirmed' = Op.confirmed }
pred txnSame   { Txn.phase' = Txn.phase and Txn.pinned' = Txn.pinned
                 and Txn.leaseOk' = Txn.leaseOk }
pred noWhy     { no Sys.why' }


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
  } else {
    one Sys.why'
    Sys.why' in failedPre
    Sys.res' = reasonOf[Sys.why']
    // The attempt ends and the finish leaf stays live and selectable; the pin
    // goes with it, which is what keeps `Fresh` and `no pinned` in step.
    Txn.phase' = Fresh and no Txn.pinned'
  }
}

/* The world swapping the task root between two steps of the transaction. */
pred doSwap {
  some Root.rid
  Sys.act' = Swap and Sys.res' = Environmental and noWhy
  some Root.rid' and Root.rid' != Root.rid
  Root.holds' = Root.holds and Slot.occ' = Slot.occ
  repoSame and worldSame and opSame and txnSame
}

/* The world changing the repository or the workspace layout under the
   transaction.  `SY-03` is why the second is possible at all. */
pred doTopologyChange {
  Sys.act' = TopologyChange and Sys.res' = Environmental and noWhy
  (Repo.rev' != Repo.rev or World.lane' != World.lane)
  Repo.tracked' = Repo.tracked
  treeSame and opSame and txnSame
}

pred step {
  doIdle or doConfirm or doDecline or doTxnOpen or doPreflight
  or doSwap or doTopologyChange
}

/* TRUE BY CONSTRUCTION, and asserted of the free initial state so that state 0
   is a state the transitions could have produced.  The initial tree is
   otherwise unconstrained — `EN-11` cashed out as a modelling decision, exactly
   as the sibling scope does it — which is what lets every witness below run at
   three states instead of running up to its situation from an empty root. */
fact TxnStateWellFormed {
  always {
    Txn.phase = Fresh iff no Txn.pinned
    Txn.phase != Fresh implies some Txn.leaseOk
  }
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
    (Op.confirmed' != Op.confirmed) implies Sys.act' = Confirm
  }
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

/* The transaction never entered for want of confirmation — with the
   DETERMINISTIC guard `FN-01` names (a live finish leaf, no live ordinary work)
   holding, so the trace says confirmation and not the guard. */
run witness_FN_01a_a_transaction_never_entered_for_want_of_confirmation {
  always no Op.confirmed
  always Txn.phase = Fresh
  gateWork
  eventually Sys.act = Decline
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

/* FN-01.b.  The other direction, and the non-redundant one: a CONFIRMED attempt
   whose deterministic guard fails is still refused.  Confirmation is not a
   substitute for the guard any more than the guard is for it. */
check FN_01b_confirmation_is_not_a_substitute_for_the_deterministic_guard {
  always ((Sys.act' = Preflight and some Op.confirmed and not pre2Work)
            implies Sys.res' in Refused)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

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
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

run witness_FN_05a_p1_confirmation_absent {
  eventually (Sys.act = Decline and Sys.why = P1Confirm)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

run witness_FN_05a_p2_no_live_finish_leaf_or_live_ordinary_work {
  eventually (Sys.act = Preflight and Sys.why = P2Work and Sys.res = RefNotLive)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

/* The layout is unsupported AT THE PREFLIGHT, having been supported at the
   lease gate — which is `SY-03` stated as a trace rather than as prose. */
run witness_FN_05a_p3_layout_unsupported {
  eventually (Sys.act = TopologyChange and no World.lane)
  eventually (Sys.act = Preflight and Sys.why = P3Layout
              and Sys.res = RefLayoutUnsupported)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

run witness_FN_05a_p4_quarantine_target_unreachable {
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine
              and Sys.res = RefLayoutUnsupported)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

run witness_FN_05a_p5_task_root_identity_unverified {
  eventually (Sys.act = Preflight and Sys.why = P5Identity
              and Sys.res = RefRootIdentityChanged)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

run witness_FN_05a_p6_empty_deletion_fingerprint {
  eventually (Sys.act = Preflight and Sys.why = P6Fingerprint
              and Sys.res = RefNoTrackedDeletion)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

run witness_FN_05a_p7_an_entry_type_that_cannot_be_digested {
  eventually (Sys.act = Preflight and Sys.why = P7EntryType
              and Sys.res = RefUnsupportedEntryType)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

/* FN-05.b.  Quantified over EVERY reported failure, which is what makes the
   seven witnesses above discharge *each of the seven, with the tree unchanged*:
   the check's antecedent is `some Sys.why'`, and each of the seven is reachable
   under it. */
check FN_05b_a_failed_precondition_leaves_the_tree_byte_identical {
  always (some Sys.why' implies treeSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

run witness_FN_05b_a_refusal_with_the_tree_unchanged {
  eventually (some Sys.why and Sys.act in (Preflight + Decline))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

check FN_05c_a_failed_precondition_leaves_the_repository_byte_identical {
  always (some Sys.why' implies repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

/* The repository is exercised in the same trace that refuses — a topology
   change moves it, and the preflight step does not.  A witness that only
   reached a refusal would be equally consistent with a model whose repository
   cannot change at all. */
run witness_FN_05c_a_refusal_with_the_repository_unchanged {
  eventually (Sys.act' = TopologyChange and Repo.rev' != Repo.rev)
  eventually (some Sys.why' and Sys.act' = Preflight and repoSame)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

// --- FN-06: the task root's identity is pinned and rechecked ----------------

/* A mid-transaction swap is a REFUSAL rather than a mutation applied elsewhere:
   the pinned identity is rechecked at the later step, and the tree the
   transaction was pointed at is left byte-identical. */
check FN_06_the_task_roots_identity_is_pinned_and_rechecked {
  always ((Sys.act' = Preflight and Root.rid != Txn.pinned)
            implies (Sys.res' in Refused and treeSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

/* Two consecutive transitions, which is why this one runs at four states: the
   swap, then the preflight that catches it. */
run witness_FN_06_a_swap_between_two_steps_is_refused {
  eventually (Sys.act = Swap and after (Sys.act = Preflight and Sys.why = P5Identity))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

// --- FN-07: an untracked tree is refused before evacuation ------------------

check FN_07_an_empty_deletion_fingerprint_is_refused_before_any_mutation {
  always ((Sys.act' = Preflight and no (Root.holds & Repo.tracked))
            implies (Sys.res' in Refused and treeSame and repoSame))
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

run witness_FN_07_a_wholly_untracked_tree {
  eventually (Sys.act = Preflight and no Repo.tracked and some Root.holds
              and Sys.res = RefNoTrackedDeletion)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps

// --- FN-08: the quarantine target is proved reachable before mutation -------

/* The lease gate's verdict proves `wtDev = qDev` and nothing else.  This check
   says entry is never granted on it: the transaction's OWN operands — the task
   root and the quarantine parent — must agree, whatever the earlier gate found. */
check FN_08_the_lease_gates_verdict_never_licenses_the_transactions_operands {
  always ((Sys.act' = Preflight and Sys.res' = Applied)
            implies World.rootDev = World.qDev)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps

/* A layout that passes at lease acquisition and fails here.  It needs two
   devices, and that is exactly what `EN-02` removes below. */
run witness_FN_08_a_layout_that_passes_at_lease_acquisition_and_fails_here {
  some Txn.leaseOk
  World.wtDev = World.qDev
  World.rootDev != World.qDev
  eventually (Sys.act = Preflight and Sys.why = P4Quarantine)
} for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 3 steps


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
} for 3 but 1 Device, 2 RootId, 2 Rev, 3 Entry, 4 steps
