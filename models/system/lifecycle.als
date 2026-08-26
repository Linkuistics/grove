/*
 * grove — the system-lifecycle claims, in Alloy 6
 * ===============================================
 *
 * The subject is `docs/specs/semantic-contract.md`, section *Claims — system
 * lifecycle*.  Nothing else: no Rust module, no helper, no control-flow shape.
 * Every command below names an OBLIGATION of that document, and the repository
 * runner reads the obligation list out of the document rather than out of this
 * file.
 *
 * COVERAGE SO FAR: SY-01, SY-02, SY-03 and SY-11 — the ADMISSION slice, the
 * loop's guard stack.  SY-04 .. SY-10 and SY-12 .. SY-14 are the `iteration`,
 * `roots` and `sessions` sibling leaves'; the runner reports their cells empty,
 * which is the truth about this file rather than a defect in it.
 *
 * THIS FILE COMPOSES AT OBSERVATIONS, NEVER AT MACHINERY.  It is the joint of
 * the task-tree and finish contracts, and it reads them through the smallest
 * observation that decides a claim — a task root that is present or absent, a
 * gate that refuses or does not.  There is no `Filename`, no position, no key,
 * no digest, no manifest, no witness slot, no quarantine and no lane anywhere
 * below, and a signature that grows one is this file becoming a third copy of
 * two contracts rather than the joint of them.
 *
 * HOW TO READ IT — the house style of `docs/ordinal-fs-tree/models/`, and of
 * both sibling scopes:
 *
 *   Nothing the catalogue merely CLAIMS is a `fact`.  Claims are named
 *   predicates, and every command says which ones it assumes.  Facts hold only
 *   what is true by construction — one advisory lock has at most one holder, a
 *   process that has ceased to exist holds nothing.
 *
 *     check SY_nn[x]_<mnemonic>            must find NO counterexample
 *     run   witness_SY_nn[x]_<mnemonic>    must find an instance
 *     check expect_fail_EN_nn_<OB>_<m>     must find a counterexample
 *
 * TWO STATIC SWITCHES, BOTH PINNED OFF BY EVERY ORDINARY COMMAND.  `Env.shared`
 * admits the nested acquisition `EN-07` forbids; `Env.rootGone` admits the root
 * removal `EN-14` forbids.  Each is left on by exactly one `expect_fail_`
 * command.  Leaving either free is not an optimisation question: with `shared`
 * free, `SY-11.b`'s check reports the mutation's own counterexample and the
 * control stops being a control.
 *
 * A GREEN RUN OF THIS FILE IS NOT, BY ITSELF, EVIDENCE.  Both sibling scopes
 * have reported themselves green — witnesses included — while checking nothing,
 * and what separated the fiction from the fact was one mutation per obligation.
 * The mutation matrix, the bounds caveats and the composition boundary are in
 * `README.md` beside this file; read it before trusting a green run.
 *
 * Run it with `models/run.sh --scope lifecycle --family alloy --no-coverage`
 * from the repository root.
 */
module lifecycle


// ===========================================================================
// VOCABULARY
//
// Deliberately coarse.  No obligation in this slice reads a name, an entry or
// a repository, so the task root is PRESENT OR ABSENT and nothing more, and the
// working-tree root is only the thing ownership is held ON.
// ===========================================================================

/* The identity of a working-tree root as an OPEN DIRECTORY — not its path.
   `EN-14` grants that the root exists before the task root and outlives its
   deletion, so under the assumption this never changes; the whole content of
   the `EN-14` mutation is that it can. */
sig WtId {}

/* The task root, as this scope observes it.  `TT-18`'s eleven-state
   classification is the task-tree model's; nothing in the admission slice reads
   finer than presence, and `roots` is where absence becomes load-bearing. */
one sig TaskRoot {}

abstract sig Layout {}
one sig SupportedL, UnsupportedL extends Layout {}

/* The recorded verdict of the workspace layout preflight.  It EXISTS in this
   model on purpose: `SY-03` is the claim that no later gate consults it, and a
   model with no verdict to consult would answer that by construction. */
one sig Ok {}

/* THE THREE GUARDS, and the order the catalogue states over them.  `below` is
   data rather than arithmetic: three guards need no `Int`, and the relation is
   what `SY-11.a` is checked against.

   They are three DIFFERENT resources and the glossary is emphatic about the
   third: the *tree access lock* serializes one tree observation or mutation and
   is released before a foreground launch, where the *driver lease* is held for
   the whole loop.  A model that collapsed them would make `SY-11` unstatable. */
abstract sig Guard {}
one sig LeaseG, EpochG, TreeG extends Guard {}

fun below: Guard -> Guard { LeaseG->EpochG + LeaseG->TreeG + EpochG->TreeG }

/* Two roles, and they are the smallest thing that makes a WAIT reachable.  A
   driver holds the lease for the loop; an ambient `grove-llm` holds none and
   matches the live generation before it may touch the tree.  With one role only
   the lease-holder ever reaches the epoch, no two processes ever contend, and
   `SY-11.b` would be checked over an empty wait-for graph.
   The roles' own claim — a stale session cannot act — is `SY-10`, the
   `iteration` sibling's; nothing here reads a generation VALUE. */
abstract sig Role {}
one sig DriverR, SessionR extends Role {}

/* A cooperating process (`EN-06`: only cooperating processes are serialized).
   `seen` is the guards this process has ACQUIRED so far, which is what makes
   `SY-11.a` a claim about an acquisition SEQUENCE rather than about a nesting:
   the ambient path takes the epoch, releases it, and only then takes the tree.
   It is never reset — within these bounds a process runs one admission cycle,
   and resetting it would be modelling the loop's iteration, which is `SY-04`'s
   and the `iteration` sibling's.

   `waits` IS AN ABSTRACTION OF THIS FILE'S OWN, recorded as one in `README.md`.
   The catalogue is explicit that a guard wait is not an outcome — Grove's tree
   lock blocks and no invocation returns while it is held — but `SY-11.b` is a
   claim ABOUT waiting, and a model in which a failed guard is an absent
   transition makes it true by construction.  The sibling task-tree model met
   this and introduced `Deferred`; this is the same move under a different
   name. */
sig Proc {
  role:        one Role,
  var holds:   set Guard,
  var seen:    set Guard,
  var waits:   lone Guard,
  var leaseOn: lone WtId
}

/* Process liveness.  `SY-01.b` is about ownership released BY PROCESS DEATH as
   ordinarily as by return, so death has to be a thing that happens TO a process
   rather than a step the process takes: `doCrash` removes it from `live` and the
   guards go with it, which is the kernel's release of an advisory lock and not a
   cleanup path Grove runs. */
one sig Alive { var live: set Proc }

one sig World {
  var wt:      lone WtId,       // the working-tree root, as an open directory
  var layout:  one Layout,
  var rooted:  lone TaskRoot,   // whether a task tree exists at all
  var verdict: lone Ok          // what the lease gate recorded
}

/* THE TWO STATIC SWITCHES.  Not `var`: an assumption mutation is a SCOPE, not
   an event, and a switch that could flip mid-trace would let a command report a
   counterexample the assumption never granted. */
one sig Flag {}
one sig Env {
  shared:   lone Flag,          // EN-07 broken: nested acquisition admitted
  rootGone: lone Flag           // EN-14 broken: the working-tree root may vanish
}


// ===========================================================================
// ACTIONS AND OUTCOMES
//
// Each action is TOTAL: it returns exactly one outcome, and a guard that fails
// produces a named refusal rather than an absent transition.
//
// The catalogue's Lifecycle group is `acquire-lease`, `layout-preflight`,
// `open-epoch`, `launch`, `reap`, `close-epoch`, `release-lease`.  Five of the
// seven are below.  `launch` and `reap` are NOT MODELLED HERE and are declared
// as such in `README.md`: no obligation in this slice reads a spawn, and
// `SY-08`'s launch window and `SY-09`'s three endings are the `iteration` and
// `sessions` siblings'.
// ===========================================================================

abstract sig Action {}
one sig IdleA,            // the stutter an Alloy lasso needs
        AcquireLeaseA,    // acquire-lease, WITH the workspace layout preflight
        ReleaseLeaseA,    // release-lease
        OpenEpochA,       // open-epoch (driver), and an ambient generation match
        CloseEpochA,      // close-epoch
        TakeTreeA,        // the tree access lock
        DropTreeA,
        GrantA,           // a blocked process proceeding once the guard is free
        TreeOpA,          // ANY observation, creation or mutation of the tree
        ValidateConfigA,  // opaque here; SY-04.b owns its content
        LayoutPreflightA, // THE LATER GATE — see SY-03
        TopologyChangeA,  // the world's
        CrashA,           // the world's
        NestedAcquireA,   // EN-07's mutation only
        RemoveRootA       // EN-14's mutation only
  extends Action {}

abstract sig Result {}
one sig Applied, Environmental extends Result {}
/* `Deferred` is the wait, and it is this file's abstraction rather than the
   catalogue's outcome set — see `Proc.waits` above. */
one sig Deferred extends Result {}
abstract sig Refused extends Result {}
one sig RefLeaseHeld, RefLayoutUnsupported extends Refused {}

one sig Sys {
  var act:   one Action,
  var actor: lone Proc,
  var gu:    lone Guard,
  var res:   one Result
}


// ===========================================================================
// THE DISCIPLINE, AS A NAMED PREDICATE
//
// `mayTake` is this model's account of GROVE'S CODE, not of the catalogue's
// claim.  Two clauses, and they are complementary rather than redundant:
//
//   the PROHIBITION — no guard already taken is at or above the one being
//   taken.  This is what `SY-11.a` checks the acquisition sites for, and what
//   `EN-07`'s mutation bypasses.
//   the REQUIREMENT  — what a step needs in order to be meaningful at all: only
//   a driver takes the lease, only a lease-holder opens an epoch, and nothing
//   touches the tree without a generation behind it.
//
// `SY-11.a` is a UNIFORMITY check over the five acquisition sites, and this is
// stated plainly rather than overclaimed: it does not prove the order is the
// right one — the order IS the design — it proves every site applies it,
// including the two that are easy to write without it (`doGrant`, which
// resumes a blocked process, and `doAcquireLease`, which is a gate rather than
// an ordinary acquisition).
// ===========================================================================

pred ordered[p: Proc, g: Guard] { all h: p.seen | h->g in below }

pred needs[p: Proc, g: Guard] {
  g = LeaseG implies p.role = DriverR
  g = EpochG implies (p.role = DriverR implies LeaseG in p.holds)
}

/* WHAT IS NOT IN `needs`, AND WHY.  An earlier pass required `EpochG in p.seen`
   before the tree guard — *an ambient command matches the live generation
   before it may touch the tree*.  That is TRUE and it is `SY-10`'s, not this
   claim's, and putting it here made `SY-11.a` VACUOUS: with the requirement in
   place no process can ever have taken the tree without having taken the epoch,
   so the ordering violation the check looks for is unreachable whether or not
   any site applies the order rule.  Three separate mutations survived against
   the vacuous form and that is how it was found.  Physically nothing stops a
   process from locking the tree directory without reading a generation; only
   Grove's discipline does, and discipline belongs in `ordered`. */

pred mayTake[p: Proc, g: Guard] {
  g not in p.holds
  ordered[p, g]
  needs[p, g]
}

fun holder[g: Guard]: lone Proc { holds.g }

/* The wait-for graph, per state: p waits on q when q holds what p is blocked
   on.  `SY-11.b`'s acyclicity is stated over this and nothing else. */
fun waitsOn: Proc -> Proc { { p, q: Proc | some p.waits and p.waits in q.holds } }


/* THE TWO ASSUMPTIONS, AS NAMED PREDICATES, because a check states its
   assumptions as an ANTECEDENT and never as a conjunct.  A `check` looks for a
   counterexample to the whole formula, so `no Env.shared and always {...}`
   is falsified by an instance that simply turns the switch on — the assumption
   becomes part of what the check is trying to break.  This cost the first
   compile of this file and is the sibling scopes' house form. */
pred EN07 { no Env.shared }     // descriptions of one directory do not share a lock
pred EN14 { no Env.rootGone }   // the working-tree root outlives the task root
pred Assumed { EN07 and EN14 }


// ===========================================================================
// FRAME CONDITIONS
// ===========================================================================

pred worldSame  { World.wt' = World.wt and World.layout' = World.layout }
pred rootSame   { World.rooted' = World.rooted }
pred verdictSame{ World.verdict' = World.verdict }
pred aliveSame  { Alive.live' = Alive.live }
pred procSame[p: Proc] {
  p.holds' = p.holds and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
}
pred procsSame        { all p: Proc | procSame[p] }
pred procsSameBut[p: Proc] { all q: Proc - p | procSame[q] }


// ===========================================================================
// TRANSITIONS
// ===========================================================================

pred doIdle {
  Sys.act' = IdleA and no Sys.actor' and no Sys.gu' and Sys.res' = Environmental
  procsSame and worldSame and rootSame and verdictSame and aliveSame
}

/* ACQUIRE-LEASE, and the workspace layout preflight that runs inside it.  Three
   outcomes and they are total.  The layout is read FIRST, which is `SY-02`: an
   unsupported workspace is refused before the lease is taken, before
   configuration validation, and before anything observes, creates or mutates
   the tree.

   THE CONTENDED CASE IS REFUSED AND NEVER QUEUED (`SY-01.a`), and that is the
   one place this file deliberately differs from the other two guards: `waits'`
   stays empty here where `doTakeTree` and `doOpenEpoch` set it.  A second driver
   would issue duplicate mandates, so it is refused immediately rather than
   queued as an ordinary tree operation — the glossary's own `_Avoid_` line. */
pred doAcquireLease[p: Proc] {
  p in Alive.live and no p.leaseOn and no p.waits and p.role = DriverR
  Sys.act' = AcquireLeaseA and Sys.actor' = p and Sys.gu' = LeaseG
  worldSame and rootSame and aliveSame

  World.layout = UnsupportedL implies {
    Sys.res' = RefLayoutUnsupported
    procsSame and verdictSame
  } else (some q: Alive.live - p | q.leaseOn = World.wt) implies {
    Sys.res' = RefLeaseHeld
    procsSame and verdictSame
  } else {
    mayTake[p, LeaseG]
    Sys.res' = Applied
    World.verdict' = Ok
    p.holds' = p.holds + LeaseG and p.seen' = p.seen + LeaseG
    p.leaseOn' = World.wt and no p.waits'
    procsSameBut[p]
  }
}

/* GUARDS ARE RELEASED IN REVERSE, which is a precondition of the STEP and not
   a claim of the catalogue: the driver lease is held with the open working-tree
   root THROUGH THE WHOLE LOOP, so releasing it is the last thing a driver does.
   Without this a driver could drop the lease while still holding the tree, and
   `SY-02`'s fourth conjunct would be false for a reason that is about this
   model rather than about Grove. */
pred doReleaseLease[p: Proc] {
  p in Alive.live and p.holds = LeaseG and no p.waits
  Sys.act' = ReleaseLeaseA and Sys.actor' = p and Sys.gu' = LeaseG
  Sys.res' = Applied
  p.holds' = p.holds - LeaseG and p.seen' = p.seen
  no p.leaseOn' and no p.waits'
  procsSameBut[p] and worldSame and rootSame and verdictSame and aliveSame
}

/* OPEN-EPOCH — the driver rotating the launch generation, and an ambient
   command matching it.  One transition for both because nothing in this slice
   reads a generation VALUE; `SY-10` is where the two part company.
   A contended epoch BLOCKS, which is the ordinary tree-guard discipline and the
   opposite of the lease's. */
pred doOpenEpoch[p: Proc] {
  p in Alive.live and (no p.waits or p.waits = EpochG)
  mayTake[p, EpochG]
  Sys.act' = OpenEpochA and Sys.actor' = p and Sys.gu' = EpochG
  worldSame and rootSame and verdictSame and aliveSame
  no holder[EpochG] implies {
    Sys.res' = Applied
    p.holds' = p.holds + EpochG and p.seen' = p.seen + EpochG
    no p.waits' and p.leaseOn' = p.leaseOn
  } else {
    Sys.res' = Deferred
    p.holds' = p.holds and p.seen' = p.seen
    p.waits' = EpochG and p.leaseOn' = p.leaseOn
  }
  procsSameBut[p]
}

pred doCloseEpoch[p: Proc] {
  p in Alive.live and EpochG in p.holds and TreeG not in p.holds and no p.waits
  Sys.act' = CloseEpochA and Sys.actor' = p and Sys.gu' = EpochG
  Sys.res' = Applied
  p.holds' = p.holds - EpochG and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  procsSameBut[p] and worldSame and rootSame and verdictSame and aliveSame
}

pred doTakeTree[p: Proc] {
  p in Alive.live and (no p.waits or p.waits = TreeG)
  mayTake[p, TreeG]
  Sys.act' = TakeTreeA and Sys.actor' = p and Sys.gu' = TreeG
  worldSame and rootSame and verdictSame and aliveSame
  no holder[TreeG] implies {
    Sys.res' = Applied
    p.holds' = p.holds + TreeG and p.seen' = p.seen + TreeG
    no p.waits' and p.leaseOn' = p.leaseOn
  } else {
    Sys.res' = Deferred
    p.holds' = p.holds and p.seen' = p.seen
    p.waits' = TreeG and p.leaseOn' = p.leaseOn
  }
  procsSameBut[p]
}

pred doDropTree[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  Sys.act' = DropTreeA and Sys.actor' = p and Sys.gu' = TreeG
  Sys.res' = Applied
  p.holds' = p.holds - TreeG and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  procsSameBut[p] and worldSame and rootSame and verdictSame and aliveSame
}

/* GRANT — a blocked process proceeding once its guard is free.  A SEPARATE
   ACQUISITION SITE, and the one an implementation is likeliest to write without
   the order rule, because the process "already asked".

   MEASURED, AND THE ANSWER IS NOT THE ONE THIS COMMENT FIRST CLAIMED: removing
   `mayTake` from here SURVIVES `SY-11.a`.  A grant cannot violate an order the
   wait already satisfied — `seen` does not change while a process is blocked —
   so the clause is a belt on a fastened braces.  It stays because a later slice
   that resets `seen` per iteration would make it load-bearing, and its
   survival is recorded in `README.md` rather than papered over. */
pred doGrant[p: Proc] {
  p in Alive.live and some p.waits and no holder[p.waits]
  mayTake[p, p.waits]
  Sys.act' = GrantA and Sys.actor' = p and Sys.gu' = p.waits
  Sys.res' = Applied
  p.holds' = p.holds + p.waits and p.seen' = p.seen + p.waits
  no p.waits' and p.leaseOn' = p.leaseOn
  procsSameBut[p] and worldSame and rootSame and verdictSame and aliveSame
}

/* ANY observation, creation or mutation of the task tree, as one opaque step.
   What distinguishes the three is the task-tree model's; what this slice needs
   is that NONE of them happens before the layout is proved. */
pred doTreeOp[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  Sys.act' = TreeOpA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  some World.rooted'                      // the tree exists, or is created here
  procsSame and worldSame and verdictSame and aliveSame
}

/* Opaque here.  `SY-04.b` — full configuration validation precedes every
   transition, so an invalid configuration leaves the working tree
   byte-identical — is the `iteration` sibling's cell, and this action exists
   only so `SY-02`'s *before configuration validation* has something to be
   before. */
pred doValidateConfig[p: Proc] {
  p in Alive.live and some p.leaseOn
  Sys.act' = ValidateConfigA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  procsSame and worldSame and rootSame and verdictSame and aliveSame
}

/* THE LATER GATE.  It stands for EVERY subsequent revalidation of the layout —
   the finish transaction's own preflight among them, which is the finish
   model's `FN-05.a` member 3 and is owned there.  What this file owes is that a
   later gate reads the LAYOUT and not the recorded verdict, which is `SY-03`.
   Note what it does NOT read: `World.verdict` appears nowhere in this
   predicate, and the biconditional in `SY_03` is what turns that absence into
   a checked fact rather than a coding habit. */
pred doLayoutPreflight[p: Proc] {
  p in Alive.live and no p.waits
  Sys.act' = LayoutPreflightA and Sys.actor' = p and no Sys.gu'
  World.layout = UnsupportedL implies Sys.res' = RefLayoutUnsupported
                              else    Sys.res' = Applied
  procsSame and worldSame and rootSame and verdictSame and aliveSame
}

/* The world's.  The layout is mutable while the lease is held — which is the
   glossary's `_Avoid_` on the preflight, and the whole reason `SY-03` exists. */
pred doTopologyChange {
  Sys.act' = TopologyChangeA and no Sys.actor' and no Sys.gu'
  Sys.res' = Environmental
  World.layout' != World.layout
  World.wt' = World.wt
  procsSame and rootSame and verdictSame and aliveSame
}

/* The world's.  Death is not a step the process takes: the kernel releases the
   advisory locks and the lease goes with them. */
pred doCrash[p: Proc] {
  p in Alive.live
  Sys.act' = CrashA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Environmental
  Alive.live' = Alive.live - p
  no p.holds' and no p.waits' and no p.leaseOn' and p.seen' = p.seen
  procsSameBut[p] and worldSame and rootSame and verdictSame
}

/* EN-07's MUTATION, and nothing else reaches it.  Two open descriptions of one
   directory not sharing a lock is why Grove's architecture is *two locks, one
   at a time*: an outer guard held across an inner acquisition self-deadlocks,
   so Grove releases before it takes the next.  The shared-lock scope removes the
   reason and therefore the discipline — a process may hold the tree guard
   across a generation acquisition — and that is exactly the option
   `bulk-marks-are-not-atomic` rejected.  Across two processes it is a cycle. */
pred doNestedAcquire[p: Proc] {
  some Env.shared
  p in Alive.live and no p.waits
  TreeG in p.holds                       // the back edge, and only it
  EpochG not in p.holds
  Sys.act' = NestedAcquireA and Sys.actor' = p and Sys.gu' = EpochG
  worldSame and rootSame and verdictSame and aliveSame
  no holder[EpochG] implies {
    Sys.res' = Applied
    p.holds' = p.holds + EpochG and p.seen' = p.seen + EpochG
    no p.waits' and p.leaseOn' = p.leaseOn
  } else {
    Sys.res' = Deferred
    p.holds' = p.holds and p.seen' = p.seen
    p.waits' = EpochG and p.leaseOn' = p.leaseOn
  }
  procsSameBut[p]
}

/* EN-14's MUTATION, and nothing else reaches it.  The working-tree root is
   removed and another appears at the same path with a different identity.  A
   lease already held is held on the OLD open directory: it is not released, and
   it no longer excludes anything. */
pred doRemoveRoot {
  some Env.rootGone
  Sys.act' = RemoveRootA and no Sys.actor' and no Sys.gu'
  Sys.res' = Environmental
  some World.wt' and World.wt' != World.wt
  World.layout' = World.layout
  procsSame and rootSame and verdictSame and aliveSame
}

pred step {
  doIdle
  or (some p: Proc | doAcquireLease[p] or doReleaseLease[p]
                     or doOpenEpoch[p] or doCloseEpoch[p]
                     or doTakeTree[p] or doDropTree[p] or doGrant[p]
                     or doTreeOp[p] or doValidateConfig[p]
                     or doLayoutPreflight[p] or doCrash[p]
                     or doNestedAcquire[p])
  or doTopologyChange
  or doRemoveRoot
}


// ===========================================================================
// TRUE BY CONSTRUCTION
//
// Everything here is a property of locks, processes and open directories — not
// one of them is a claim of the catalogue, and each is asserted of the FREE
// INITIAL STATE as well, so that state 0 is a state the transitions could have
// produced.  The initial state is otherwise left wide open, which is what lets
// the witnesses below run at three or four states instead of running up to
// their situation from an empty working tree.
// ===========================================================================

/* THE LEASE IS DELIBERATELY NOT IN HERE, and that is `EN-14`'s whole subject.
   A lock is exclusive PER OPEN DIRECTORY, not per abstract guard: `lone
   holds.LeaseG` would say *one live driver per working tree* by construction,
   which is `SY-01`, and the catalogue's rule is that nothing it merely claims is
   a fact.  The construction fact is the one below — one holder per ROOT — and
   under `EN-14` two roots make two holders.  This file said `all g: Guard` on
   its first pass and the `EN-14` control was what caught it: the mutation could
   not fire, which reports exactly as a survivor. */
fact OneLockOneHolder      { always all g: Guard - LeaseG | lone holds.g }
fact OneLeaseHolderPerRoot { always all w: WtId | lone leaseOn.w }
fact HeldImpliesTaken      { always all p: Proc | p.holds in p.seen }
/* `some p.waits implies` IS LOAD-BEARING, not defensive.  `p.waits` is `lone`,
   and `none in p.holds` is TRUE for any `p.holds`, so the bare `p.waits not in
   p.holds` is FALSE exactly when a process is unblocked — the fact then reads
   *every process is blocked in every state*, which made the whole file
   unsatisfiable.  Recorded in `README.md` and in Experiment 2. */
fact NoOneWaitsForWhatItHas {
  always all p: Proc | some p.waits implies p.waits not in p.holds
}
fact TheLeaseIsAGuard      { always all p: Proc | LeaseG in p.holds iff some p.leaseOn }
fact OnlyADriverLeases     { always all p: Proc | p.role = SessionR implies LeaseG not in p.seen }
fact TheDeadHoldNothing {
  always all p: Proc - Alive.live | no p.holds and no p.waits and no p.leaseOn
}
/* `EN-14`: the working-tree root exists before the task root and outlives its
   deletion.  Under the assumption it is present in every state; the mutation's
   `doRemoveRoot` replaces it rather than emptying it, so the field stays
   populated and the divergence is in what the LEASES are bound to. */
fact TheWorkingTreeRootExists { always some World.wt }

/* TRUE BY CONSTRUCTION, and the reason the free initial state stays honest.
   Every clause is what an ACQUISITION SITE ALREADY REQUIRED (`needs`) or what
   reverse release already guarantees; not one of them is a claim, and none says
   anything about the ORDER the guards were taken in, which is `SY-11.a`'s and
   is checked rather than assumed.

   NONE OF THIS IS DECORATION.  Written without it, state 0 hands out a driver
   holding the tree guard with no lease anywhere — a RUNNING SESSION nobody
   started rather than a hand-edited world — and `SY-02` reported exactly that
   counterexample twice. */
fact StateZeroIsAStateTheStepsCouldHaveProduced {
  always all p: Proc {
    (p.role = DriverR and EpochG in p.holds) implies LeaseG in p.holds
    (p.role = DriverR and TreeG in p.holds)  implies LeaseG in p.holds
  }
}
/* THE CLAUSES ABOUT `seen` ARE DELIBERATELY ABSENT.  Two more were written here
   — *the tree implies the epoch was seen*, *the epoch implies the lease was
   seen* — and they are the ORDER, which is `SY-11.a`.  A construction fact that
   states a claim makes the claim vacuous and makes every mutation against it
   survive, which is what happened.  What stays are the two clauses about what is
   HELD, and they rest on `needs` plus reverse release rather than on any
   prohibition. */

/* WHERE A TRACE STARTS, and the one narrowing this slice needs.  A process
   starts UNBLOCKED: a wait is something a transition produces, and a free
   initial state that hands out blocked processes hands `SY-11.b` a cycle no
   execution reaches — the shape of a false counterexample, which reports
   exactly as a real one.  Everything else about state 0 stays free: who holds
   what, what each has already taken, the layout, the verdict, whether a task
   root exists.  Note the absence of `always` — this constrains state 0 and
   nothing else, so every wait a trace shows was produced by a step. */
fact TracesStartWithNobodyBlocked { no Proc.waits }

/* Likewise state 0 only.  Under `EN-14` a lease is held on the working-tree
   root that exists; the mutation is the only thing that breaks it, and it must
   break it by RUNNING, not by being handed a broken state 0. */
fact LeasesStartBoundToTheLiveRoot {
  all p: Proc | some p.leaseOn implies p.leaseOn = World.wt
}

fact Trace {
  Sys.act = IdleA and Sys.res = Environmental and no Sys.actor and no Sys.gu
  always step
}


// ===========================================================================
// CLAIMS — SY-01, SY-02, SY-03, SY-11
//
// WHY THE BEHAVIOURAL COMMANDS RUN AT `3 steps` OR `4 steps`.  An Alloy 6 trace
// is a lasso, so the last state must loop.  A state reached by a state-changing
// action loops neither back to the initial state nor to itself, so at `2 steps`
// no applied transition exists at all and every check conditioned on an outcome
// is vacuously true.  Three states admit one transition followed by a stutter;
// a witness needing two consecutive transitions runs at four, and the wait-then-
// cycle witnesses at five.
//
// THE BOUND MUST HOLD THE MACHINERY OF THE TRANSITIONS AN OBLIGATION QUANTIFIES
// OVER, not only the objects it names — the sibling scopes' first vacuity
// predictor.  `3 Proc` is not decoration: `SY-11.b`'s cycle needs two blocked
// processes and its non-vacuity witness needs a third holder.
// ===========================================================================

// --- SY-01: one live driver per working tree --------------------------------

/* SY-01.a.  Two conjuncts, and the second is the one that is not true by
   construction.  The first — at most one live process holds a lease — follows
   from the acquisition gate, and saying so plainly is better than pretending
   otherwise; it is here because it is what `EN-14` breaks.  The second is the
   claim's own word IMMEDIATELY: a contended lease refuses and the contender is
   never left blocked, where the other two guards do exactly the opposite. */
check SY_01a_a_second_driver_is_refused_immediately_and_never_queued {
  Assumed implies always {
    lone { p: Alive.live | some p.leaseOn }
    (Sys.act' = AcquireLeaseA and Sys.res' = RefLeaseHeld)
      implies (no Sys.actor'.waits' and no Sys.actor'.leaseOn')
    (Sys.act' = AcquireLeaseA) implies Sys.actor'.waits' = Sys.actor'.waits
  }
} for 3 but 2 WtId, 4 steps

run witness_SY_01a_a_second_driver_refused_while_the_first_holds {
  Assumed
  eventually (Sys.act = AcquireLeaseA and Sys.res = RefLeaseHeld
              and some q: Alive.live - Sys.actor | some q.leaseOn)
} for 3 but 2 WtId, 3 steps

/* SY-01.b.  Ownership is released by process death AS ORDINARILY AS BY RETURN,
   so the check states the two together: whatever ceases to hold the lease, the
   lease is gone, and the two ways of ceasing are a return and a death. */
check SY_01b_death_releases_ownership_as_ordinarily_as_return {
  Assumed implies always {
    all p: Proc | (p not in Alive.live') implies (no p.leaseOn' and no p.holds')
    (Sys.act' = ReleaseLeaseA and Sys.res' = Applied) implies no Sys.actor'.leaseOn'
  }
} for 3 but 2 WtId, 4 steps

/* The catalogue's own witness: a crashed driver whose successor proceeds.  Both
   halves are required in one trace — a crash alone would be equally consistent
   with a model in which nobody can ever acquire again. */
run witness_SY_01b_a_crashed_driver_whose_successor_proceeds {
  Assumed
  some disj p, q: Proc {
    p.role = DriverR and q.role = DriverR
    /* `some p.leaseOn` must be read in the crash's PRE-state: `Sys.act` names
       the action that PRODUCED the state, and by then the kernel has already
       released what `p` held.  Written as a conjunct at the same state the
       witness is unreachable, and unreachable reports exactly as wrong. */
    eventually (some p.leaseOn and after (Sys.act = CrashA and Sys.actor = p))
    eventually (Sys.act = AcquireLeaseA and Sys.actor = q and Sys.res = Applied)
  }
} for 3 but 2 WtId, 5 steps

// --- SY-02: the layout is proved before any tree exists ---------------------

/* SY-02.  Four conjuncts, and together they are *refused at lease acquisition,
   before configuration validation and before any observation, creation or
   mutation of the tree*:

     1  an unsupported layout refuses AT the lease gate;
     2  and takes nothing, so nothing downstream of the lease is reachable;
     3  configuration validation happens only under a lease — which is what
        *before configuration validation* means once the gate is the only way to
        get one;
     4  a DRIVER's tree operation happens only under a lease, which is the
        same for *before any observation, creation or mutation*.

   THE FOURTH CONJUNCT IS THE DRIVER'S ONLY, AND THAT IS A COMPOSITION SEAM
   RATHER THAN A WEAKENING.  A session process reaches the tree by matching a
   live generation, and a live generation exists only because a driver that
   held a lease opened one — but `launch` is not modelled in this slice, so
   stating the session half here would state it over machinery this file does
   not have.  Written unqualified it is FALSE at state 0: the free initial
   state hands a session the tree guard with no driver anywhere, and the check
   reported exactly that.  The session half is `SY-10`'s — a stale session
   cannot act — and is declared owed in `README.md`. */
check SY_02_an_unsupported_layout_is_refused_before_anything_touches_the_tree {
  Assumed implies always {
    (Sys.act' = AcquireLeaseA and World.layout' = UnsupportedL)
      implies (Sys.res' = RefLayoutUnsupported
               and no Sys.actor'.leaseOn'
               and World.verdict' = World.verdict
               and World.rooted' = World.rooted)
    (Sys.act' = ValidateConfigA) implies some Sys.actor'.leaseOn'
    (Sys.act' = TreeOpA and Sys.actor'.role = DriverR)
      implies some Sys.actor'.leaseOn'
  }
} for 3 but 2 WtId, 4 steps

/* The catalogue's own witness: A REFUSAL THAT LEAVES AN EMPTY WORKING TREE
   UNTOUCHED.  Every conjunct after the first is what *empty* and *untouched*
   mean, and dropping any of them would witness a weaker claim: no task root in
   any state of the trace, no verdict ever recorded, and no tree action anywhere
   in it. */
run witness_SY_02_a_refusal_leaving_an_empty_working_tree_untouched {
  Assumed
  always World.layout = UnsupportedL
  always no World.rooted
  always no World.verdict
  always Sys.act not in (TreeOpA + ValidateConfigA)
  eventually (Sys.act = AcquireLeaseA and Sys.res = RefLayoutUnsupported)
} for 3 but 2 WtId, 3 steps

// --- SY-03: a preflight is never a licence ----------------------------------

/* SY-03.  A BICONDITIONAL, and that is the whole instrument.  A later gate that
   consulted the recorded verdict would pass on an unsupported layout whenever a
   verdict existed, which the left-to-right direction catches; a gate that
   refused on a supported layout because no verdict existed would be a different
   bug, which the right-to-left direction catches.  An implication either way
   would leave one of them invisible.

   The verdict is READ NOWHERE in `doLayoutPreflight`, and this check is what
   turns that absence into a fact rather than a coding habit. */
check SY_03_a_later_gate_revalidates_against_its_own_operands {
  Assumed implies always {
    (Sys.act' = LayoutPreflightA)
      implies ((Sys.res' in Refused) iff (World.layout' = UnsupportedL))
  }
} for 3 but 2 WtId, 4 steps

/* The catalogue's own witness: A LAYOUT THAT CHANGES BETWEEN THE TWO GATES.
   The verdict must be RECORDED — `some World.verdict` at the later gate — or
   the trace would show a gate refusing with nothing to have wrongly consulted,
   which is not this claim. */
run witness_SY_03_a_layout_that_changes_between_the_two_gates {
  Assumed
  eventually (Sys.act = AcquireLeaseA and Sys.res = Applied
              and World.layout = SupportedL)
  eventually (Sys.act = TopologyChangeA and World.layout = UnsupportedL)
  eventually (Sys.act = LayoutPreflightA and Sys.res = RefLayoutUnsupported
              and some World.verdict)
} for 3 but 2 WtId, 5 steps

// --- SY-11: the guard order admits no cycle ---------------------------------

/* SY-11.a.  THE UNIFORMITY CHECK.  Stated over every acquisition — every guard
   that appears in some process's `seen` between two states — rather than over a
   list of transition names, because a list goes stale the moment a sixth
   acquisition site is added and this is exactly the file `sessions` will add
   one to.

   A SECOND CONJUNCT WAS WRITTEN HERE AND REMOVED, because it was vacuous: *what
   a process has taken is totally ordered by `below`* is true of ANY subset of
   three guards, `below` being a total order on them, so it read as content and
   checked nothing.  Recorded rather than silently dropped — a vacuous conjunct
   beside a real one is how a check comes to look stronger than it is. */
check SY_11a_every_acquisition_site_applies_the_guard_order {
  Assumed implies
    always all p: Proc | all g: p.seen' - p.seen | all h: p.seen | h->g in below
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: THE FULL ORDER, REACHED.  One process that has
   taken all three, and the trace shows it taking them rather than starting with
   them — otherwise the free initial state would discharge the witness without
   any acquisition site running at all.

   RUN AT 6 AND FIRST LANDS AT 5, deliberately.  Three consecutive acquisitions
   from an empty `seen` need five states, and at `5 steps` this witness had NO
   MARGIN — the pre-registration's *scope trap* is a bound too small to reach the
   defect, and a witness landing exactly at its own bound is that bound's
   boundary. */
run witness_SY_11a_the_full_order_reached {
  Assumed
  some p: Proc {
    p.role = DriverR
    no p.seen
    eventually (Sys.act = AcquireLeaseA and Sys.actor = p and Sys.res = Applied)
    eventually (Sys.act = OpenEpochA and Sys.actor = p and Sys.res = Applied)
    eventually (Sys.act = TakeTreeA and Sys.actor = p and Sys.res = Applied)
    eventually p.seen = LeaseG + EpochG + TreeG
  }
} for 3 but 2 WtId, 6 steps

/* SY-11.b.  Two conjuncts, and the catalogue states them as one obligation
   because the first is the shape the second forbids: no path waits for a
   generation while holding a tree guard, AND no cycle exists within the bound.
   `waitsOn` is the wait-for graph and `^` is its transitive closure, so the
   second conjunct is exhaustive over the states the bound reaches — which is
   what *with its bound* means and why `README.md` records the number rather
   than leaving a reader to infer it.

   WHAT THIS DOES NOT PROVE.  With `Env.shared` off, the back edge is
   unreachable BY CONSTRUCTION — `mayTake` refuses it — so the first conjunct is
   green for a reason the check cannot distinguish from a real one.  That is
   precisely what `expect_fail_EN_07_SY_11b` is for, and it is the reason the
   witness below insists on a NON-EMPTY wait-for graph: a green acyclicity check
   over an empty graph would be the vacuous invariant the pre-registration
   names. */
check SY_11b_no_generation_wait_under_a_tree_guard_and_no_cycle {
  Assumed implies always {
    no p: Proc | p.waits = EpochG and TreeG in p.holds
    no p: Proc | p in p.^waitsOn
  }
} for 3 but 2 WtId, 5 steps

/* The witness the catalogue words as *the exhaustive absence of a cycle*, which
   no runner can land as an instance.  What it lands instead is the thing that
   makes the absence mean anything: A REAL WAIT, produced by a step, between two
   live processes — a non-empty acyclic wait-for graph.  Without it the check
   above is green over nothing. */
run witness_SY_11b_a_real_wait_that_is_not_a_cycle {
  Assumed
  eventually (some waitsOn and Sys.res = Deferred)
  always no p: Proc | p in p.^waitsOn
} for 3 but 2 WtId, 4 steps


// ===========================================================================
// THE ASSUMPTION CONTROLS
//
// Both are PREMISE-BREAK, so the expected result is A NAMED OBLIGATION FAILS.
// A green check here is a SURVIVOR — a defect in the mutation, not a pass — and
// `expect_fail_` is what tells the runner to invert its verdict.
// ===========================================================================

/* EN-07 — two open descriptions of one directory do not share a lock.
   Expected: `SY-11.b` fails, and the counterexample is the cycle
   `bulk-marks-are-not-atomic` records: an outer guard held across an inner
   acquisition, which across two processes closes. */
check expect_fail_EN_07_SY_11b_a_shared_lock_scope_reintroduces_the_cycle {
  (some Env.shared and EN14) implies always {
    no p: Proc | p.waits = EpochG and TreeG in p.holds
    no p: Proc | p in p.^waitsOn
  }
} for 3 but 2 WtId, 5 steps

/* EN-14 — the working-tree root exists before the task root and outlives its
   deletion.  Expected: `SY-01.a` fails — ownership has nothing to be held on,
   so a second driver is admitted.  The assumption table names `SY-05` in the
   same row; that half is the `roots` sibling's and is declared owed in
   `README.md` rather than answered here. */
check expect_fail_EN_14_SY_01a_ownership_has_nothing_to_be_held_on {
  (some Env.rootGone and EN07) implies always lone { p: Alive.live | some p.leaseOn }
} for 3 but 2 WtId, 5 steps
