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
 * loop's guard stack — plus SY-04, SY-08 and SY-10, the ITERATION slice, the
 * loop's own step — plus SY-05, SY-06 and SY-07, the ROOTS slice: the task
 * root's own lifecycle.  SY-09 and SY-12 .. SY-14 are the `sessions` sibling
 * leaf's; the runner reports their cells empty, which is the truth about this
 * file rather than a defect in it.
 *
 * THIS FILE COMPOSES AT OBSERVATIONS, NEVER AT MACHINERY.  It is the joint of
 * the task-tree and finish contracts, and it reads them through the smallest
 * observation that decides a claim — a task root that is present or absent, a
 * gate that refuses or does not.  There is no `Filename`, no position, no key,
 * no digest, no manifest, no witness slot, no quarantine and no lane anywhere
 * below, and a signature that grows one is this file becoming a third copy of
 * two contracts rather than the joint of them.
 *
 * THE ROOTS SLICE MAKES ONE COMPOSITION DECISION AND IT IS THE FILE'S SHARPEST.
 * `SY-06.b` is the first obligation that cannot be answered by *present or
 * absent*: it must complete a `PartialScaffold` and REFUSE a `Legacy` tree,
 * which is two of `TT-18`'s eleven states.  Importing two is not importing
 * eleven, and this file imports neither: `World.partial` and `World.legacy` are
 * OPAQUE OBSERVATIONS whose content — the exact known subset, the byte
 * comparison, the classification ORDER — is `crates/grove-task-tree/models/`'s
 * and appears nowhere here.  What this file owns is the difference between the
 * observation that ENABLES the completion (their union, which is exactly *the
 * format witness is absent*) and the one that DECIDES it (`partial` alone).
 * That difference is `SY-06.b`, and it is checkable without a format witness in
 * the signature.
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
 * THREE ABSTRACTIONS OF THIS FILE'S OWN, all declared in `README.md` and none of
 * them a contract: `Proc.waits`/`Deferred` (a guard wait as an observable
 * state), `Stopped` (SY-10.b's visible stop, which the catalogue's closed
 * outcome set cannot name — as it cannot name `RefConfigInvalid`), and `IterA`
 * (an iteration boundary, which is not a catalogue action because a boundary is
 * not something the loop DOES).  The first is inherited; the other two arrive
 * with this slice and the second of them is a FINDING about the catalogue.
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

/* THE IDENTITY OF ONE GROVE — an INCARNATION of the task root, not its path and
   not its contents.  The admission and iteration slices read no finer than
   presence, and `roots` is where absence becomes load-bearing: `finish-k8`
   falsified three formulations of `FN-28` on one trace because after the
   quarantine rename THE NAME IS FREE, the world can occupy it, and it can give
   what it put there the quarantined root's OWN IDENTITY
   (`docs/formalism-findings.md` entry 039).  A model whose task root is a `one
   sig` cannot state that trace, and a model that states absence as an invariant
   re-derives the counterexample at its own cost.

   So absence is never something that HOLDS here.  It is something Grove
   ESTABLISHES — `World.retired`, the groves whose deletion Grove has proven —
   and PRESERVES — that set is never shrunk, not even when the world puts a tree
   back at the name carrying a retired identity.  `TT-18`'s eleven states are
   still the task-tree model's; what a `SY-` obligation reads of a grove is its
   IDENTITY and nothing else. */
sig Grove {}

abstract sig Layout {}
one sig SupportedL, UnsupportedL extends Layout {}

/* THE CONFIGURATION, AS THE ONLY THING `SY-04.b` READS OF IT: whether full
   validation passes.  `complete-session-configuration` says the personal file is
   validated IN FULL before every tree mutation and again before every launch, so
   what a lifecycle transition needs from it is one bit.  It is `var` for the same
   reason `layout` is: a configuration a gate proved once can change under it, and
   a model whose configuration cannot change answers *precedes EVERY transition*
   by construction — which is the `SY-02`/`SY-03` false-confidence shape the
   sibling gate already met. */
abstract sig Cfg {}
one sig ValidCfg, InvalidCfg extends Cfg {}

/* A WORK-ITEM HANDLE, AND NOTHING ELSE ABOUT A LEAF.  `SY-08` cannot be stated
   without something for selection to RETURN, so this is the smallest observation
   that decides it: an opaque identity with no name, position, key, kind or
   terminality.  `World.live` is a SET and never a sequence — WHICH live leaf the
   walk returns is `TT-11`'s (*selection is a stateless pre-order walk*) and is
   imported here as a non-deterministic choice among the live ones.  A signature
   that grew an order would be this file re-stating the task-tree contract. */
sig Leaf {}

/* A LAUNCH GENERATION, as an opaque identity.  `admission-k51` deliberately read
   no generation VALUE — one transition served both the driver's rotation and an
   ambient command's match — and `SY-10` is where the two part company: a stale
   session is exactly one whose generation is not the record's.  What the identity
   is made of (a 128-bit nonce, a working-tree identity, a signal path) is
   `one-live-driver-per-working-tree`'s and is opaque here. */
sig Gen {}

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
  var leaseOn: lone WtId,
  /* THE ITERATION'S ONE LIFECYCLE TRANSITION, SPENT OR NOT.  `SY-04.a` is a
     claim about a COUNT PER ITERATION, and a count needs a boundary to be
     counted between; `spent` is that boundary's residue.  It is set by every
     Lifecycle transition in every branch — a refusal is a turn of the loop as
     surely as an application is — and cleared only by `doIter`. */
  var spent:   lone Flag,
  /* THE ITERATION'S AUTHORITATIVE SELECTION (`SY-08`).  Taken once, never
     recomputed, and cleared at the boundary rather than by the launch, which is
     what makes a leaf added during the launch window the NEXT iteration's. */
  var sel:     lone Leaf,
  /* THE GENERATION THIS PROCESS WAS LAUNCHED UNDER.  A driver's is empty: the
     driver owns the record rather than matching it. */
  var gen:     lone Gen
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
  var cfg:     one Cfg,         // whether full configuration validation passes
  var rooted:  lone Grove,      // the grove OCCUPYING the task-root name, if any
  /* THE GROVES THAT EXIST ANYWHERE, and the field `EN-14` is about.  Under the
     assumption a grove exists exactly while it occupies the name, so this
     tracks `rooted` step for step and `SY-05.a`'s third conjunct — *absence is
     the COMPLETE fresh-tree discriminator* — is green.  `EN-14`'s mutation
     replaces the working-tree root under the loop: the grove is still there, in
     the directory Grove can no longer reach, and the two part company.  That
     divergence IS the mutation's counterexample. */
  var extant:  set Grove,
  /* THE DELETIONS GROVE HAS PROVEN.  Monotone, and it is the whole of what
     `SY-05` means by absence: not a state of the filesystem — the world owns
     that name the moment the rename lands — but a fact Grove ESTABLISHED and
     thereafter PRESERVES.  The correlation ticket is its durable realisation and
     is `crates/grove-finish/models/`'s; what reaches this file is the fact. */
  var retired: set Grove,
  /* THE COMMIT WHOSE PROOF `FN-11` AND `FN-19` OWN, as one opaque observation.
     `some World.proven` is *the deletion of the grove at the name is proven*;
     the witness slot, the manifest, the evacuation and the atomic rename that
     make it true are the finish model's and appear nowhere here.  `SY-05.b` is
     stated over this field and names that model as the owner of the steps. */
  var proven:  lone Grove,
  /* THE TWO ROOT STATES THIS SCOPE READS, AS OPAQUE OBSERVATIONS — see the
     header.  Their content is `TT-18`'s and `TT-20`'s: `partial` is *present, no
     format witness, and the contents are exactly the known proper subset*, and
     `legacy` is *present, no format witness, and not that*.  Neither the subset,
     the bytes, nor the classification ORDER is here; what is here is that they
     are DIFFERENT and that only one of them is completable. */
  var partial: lone Grove,
  var legacy:  lone Grove,
  var live:    set Leaf,        // the live leaves, unordered — see `Leaf`
  /* THE FINISH LEAF, AND IT IS A `set` ON PURPOSE.  `SY-07.a` claims EXACTLY ONE
     driver-owned finish leaf, and a `lone` field would say so by construction —
     which is the failure mode `README.md` records twice.  Declared `set`, the
     count is something `SY_07a` CHECKS and something a mutation can break. */
  var fin:     set Leaf,
  var verdict: lone Ok,         // what the lease gate recorded
  /* THE SESSION EPOCH RECORD.  `some` is an active record with that identity;
     `none` is an inactive one.  ONLY ROTATION IS MODELLED: `open-epoch` writes
     it active with a fresh identity, and the record's two *inactive* write
     points (after lease acquisition, after reap) are collapsed away.  That is a
     deliberate omission and is declared in `README.md`: the glossary names
     rotation as the stronger mechanism — it "catches stale sessions between
     every launch as well as after finish plus root recreation" — and `SY-10.a`'s
     whole content rides on it. `none` remains reachable as a free initial
     state. */
  var gen:     lone Gen,
  var running: lone Leaf        // the work the live session was launched on
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
        ValidateConfigA,  // full configuration validation — SY-04.b
        LayoutPreflightA, // THE LATER GATE — see SY-03
        LaunchA,          // launch — the spawn, and SY-08's window closes here
        ReapA,            // reap
        SelectA,          // select (Observation group) — SY-08
        IterA,            // THE ITERATION BOUNDARY — see below
        TimeoutA,         // the contended-generation stop — SY-10.b
        TopologyChangeA,  // the world's
        ConfigEditA,      // the world's
        CrashA,           // the world's
        NestedAcquireA,   // EN-07's mutation only
        RemoveRootA,      // EN-14's mutation only
        // --- the roots slice ---------------------------------------------
        InitRootA,        // initialise-root, AS FAR AS THE FORMAT WITNESS
        CompleteScaffoldA,// the append that finishes it — and that completes an
                          //   INTERRUPTED one.  One operation, two arrivals.
        AllocFinishA,     // allocate-finish-leaf — SY-07
        ProveCommitA,     // FN-11/FN-19's proof, as ONE opaque step
        SettleDeletionA,  // the atomic rename's observable effect: the name frees
        ObserveRootA,     // resolve — an observation of the root's presence
        HandEditA,        // the world's (EN-11): a legacy tree at the name
        ForeignWriteA     // the world's (EN-13): the name RE-OCCUPIED, and with
                          //   the retired identity — entry 039's own trace
  extends Action {}

/* THE CATALOGUE'S LIFECYCLE GROUP, ALL SEVEN, AND EXACTLY THEM.  `SY-04.a` is
   quantified over this set and `SY-11.a` over the acquisition sites, which are
   different sets and deliberately so: `take-tree` is a GUARD acquisition and not
   a Lifecycle action, and `select` is an Observation.  A sixth Lifecycle action
   added by a later slice lands in this function and both claims see it. */
fun LifecycleAct: set Action {
  AcquireLeaseA + LayoutPreflightA + OpenEpochA + LaunchA + ReapA
  + CloseEpochA + ReleaseLeaseA
}

/* EVERY OBSERVATION, CREATION OR MUTATION OF THE TASK TREE, as a set rather than
   as a list of names — `SY-11.a`'s discipline applied to `SY-02`'s fourth
   conjunct and `SY-10.a`'s third.  The admission slice stated both over
   `TreeOpA` alone because `TreeOpA` was the only one; this slice adds six, and a
   claim written as a list would have gone stale silently rather than loudly.
   `SY-02` and `SY-10.a` are STRENGTHENED by this and neither was edited
   otherwise, which is what the set was for. */
fun TreeAct: set Action {
  TreeOpA + InitRootA + CompleteScaffoldA + AllocFinishA
  + ProveCommitA + SettleDeletionA + ObserveRootA
}

/* THE ITERATION BOUNDARY IS NOT A CATALOGUE ACTION, and is declared as this
   file's own abstraction in `README.md` beside `Proc.waits`.  §*Actions* has no
   boundary in it, because a boundary is not something the loop DOES — but
   `SY-04.a` says *at most one lifecycle transition PER ITERATION* and an
   iteration with no observable edge has no referent for the count.  `IterA` is
   that edge and nothing else: it takes no guard, returns no outcome the
   catalogue names, and touches no part of the world. */

abstract sig Result {}
one sig Applied, Environmental extends Result {}
/* `Empty` — an observation that matched nothing, AND IT IS A SUCCESS.  §*Outcomes*
   is explicit, and `TT-15.a` is its claim.  It arrives with this slice because
   `SY-07` is where `doSelect`'s `some World.live` guard comes off: selection on a
   spent tree is not an absent transition, it is the observation that PRECEDES
   the finish leaf's allocation. */
one sig Empty extends Result {}
/* An observation returning a value — the catalogue's `Reported(v)`.  This file
   reads no value off a selection, so the payload is absent and `TT-15`'s
   `Empty`/`Ambiguous` distinctions are the task-tree model's. */
one sig Reported extends Result {}
/* `Deferred` is the wait, and it is this file's abstraction rather than the
   catalogue's outcome set — see `Proc.waits` above. */
one sig Deferred extends Result {}
/* `Stopped` IS THIS FILE'S SECOND ABSTRACTION, AND ITS EXISTENCE IS A FINDING.
   `SY-10.b` requires a contended generation to time out into a VISIBLE STOP, and
   the catalogue's closed outcome set cannot name it: it is not a `Refused`,
   because no refusal reason covers a handoff timeout (`EpochStale` is `SY-10.a`'s
   MISMATCH, not a contention), and it is not a `Blocked`, because §*Outcomes*
   scopes blocks to a transaction stopped part-way and `FN-25`'s two diagnoses are
   both about finish ownership.  Declared here, recorded in `README.md`, and named
   for `formal-synthesis-k16` — exactly as `SY-05`'s design constraint was. */
one sig Stopped extends Result {}
abstract sig Refused extends Result {}
/* `RefEpochStale` is the catalogue's `EpochStale`.  `RefConfigInvalid` is NOT in
   the closed refusal-reason set — the second half of the same finding, and the
   reason the two are recorded together rather than separately. */
one sig RefLeaseHeld, RefLayoutUnsupported,
        RefEpochStale, RefConfigInvalid extends Refused {}
/* Both are the catalogue's own, from the closed refusal-reason set.
   `RefFormatLegacy` is `FormatLegacy`: a tree with no format witness that is not
   the known subset is refused rather than completed, which is `SY-06.b`'s whole
   content.  `RefReservedKind` is `ReservedKind`: the finish leaf's kind is
   Grove's to allocate and a session's to refuse, which is `SY-07.b`'s. */
one sig RefFormatLegacy, RefReservedKind extends Refused {}

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

/* THE THREE DISCIPLINES THIS SLICE ADDS, EACH A PREDICATE THE SITES APPLY AND
   THE CHECKS ARE STATED OVER — never a `fact`, for the reason the header gives.

   `validated` is `SY-04.b`'s: a transition reads the LIVE configuration, exactly
   as `doLayoutPreflight` reads the live layout.  `doAcquireLease` deliberately
   does NOT apply it — `SY-02` says an unsupported workspace is refused at lease
   acquisition *before configuration validation*, so the lease gate is the one
   Lifecycle transition that runs under an unvalidated configuration, and gating
   it here would make `SY-02` and `SY-04.b` contradict each other.

   `fresh` is `SY-04.a`'s: this iteration has not yet spent its one transition.

   `stale` is `SY-10.a`'s: an ambient operation whose generation is not the live
   record's.  A session with NO generation is stale — it was never launched — and
   an inactive record makes every session stale, which is what the record's
   inactive state is for. */
pred validated { World.cfg = ValidCfg }
pred fresh[p: Proc] { no p.spent }
pred stale[p: Proc] { p.role = SessionR and (no p.gen or no World.gen or p.gen != World.gen) }

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

pred worldSame  { World.wt' = World.wt and World.layout' = World.layout
                  and World.cfg' = World.cfg }
/* THE ROOT'S OWN FRAME, split out of `treeSame` because six of this slice's
   transitions change the entries without touching the root's identity or its
   classification, and three change the root without touching the entries.
   Written once for the reason `actorSameBut` is: a field added to `World` that
   one transition forgot is a silent frame hole, and this slice adds six. */
pred rootSame   { World.rooted' = World.rooted and World.extant' = World.extant
                  and World.retired' = World.retired and World.proven' = World.proven
                  and World.partial' = World.partial and World.legacy' = World.legacy }
pred entriesSame{ World.live' = World.live and World.fin' = World.fin }
pred treeSame   { rootSame and entriesSame }
pred launchSame { World.gen' = World.gen and World.running' = World.running }
pred verdictSame{ World.verdict' = World.verdict }
pred aliveSame  { Alive.live' = Alive.live }
pred procSame[p: Proc] {
  p.holds' = p.holds and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
}
/* What a Lifecycle transition leaves alone about its own actor, everything but
   `spent`.  Written once because six transitions repeat it and a field added to
   `Proc` that one of them forgot would be a silent frame hole. */
pred actorSameBut[p: Proc] {
  p.holds' = p.holds and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  p.sel' = p.sel and p.gen' = p.gen
}
pred procsSame        { all p: Proc | procSame[p] }
pred procsSameBut[p: Proc] { all q: Proc - p | procSame[q] }


// ===========================================================================
// TRANSITIONS
// ===========================================================================

pred doIdle {
  Sys.act' = IdleA and no Sys.actor' and no Sys.gu' and Sys.res' = Environmental
  procsSame and worldSame and treeSame and verdictSame and launchSame and aliveSame
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
  /* `fresh` and NOT `validated`.  The lease gate spends the iteration's one
     Lifecycle transition like every other, but it runs BEFORE configuration
     validation — that is `SY-02`'s own word — so it is the one site `SY-04.b`'s
     check exempts, and the exemption is stated there rather than assumed here. */
  fresh[p]
  Sys.act' = AcquireLeaseA and Sys.actor' = p and Sys.gu' = LeaseG
  worldSame and treeSame and launchSame and aliveSame
  some p.spent'

  World.layout = UnsupportedL implies {
    Sys.res' = RefLayoutUnsupported
    actorSameBut[p] and procsSameBut[p] and verdictSame
  } else (some q: Alive.live - p | q.leaseOn = World.wt) implies {
    Sys.res' = RefLeaseHeld
    actorSameBut[p] and procsSameBut[p] and verdictSame
  } else {
    mayTake[p, LeaseG]
    Sys.res' = Applied
    World.verdict' = Ok
    p.holds' = p.holds + LeaseG and p.seen' = p.seen + LeaseG
    p.leaseOn' = World.wt and no p.waits'
    p.sel' = p.sel and p.gen' = p.gen
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
  validated and fresh[p]
  Sys.act' = ReleaseLeaseA and Sys.actor' = p and Sys.gu' = LeaseG
  Sys.res' = Applied
  p.holds' = p.holds - LeaseG and p.seen' = p.seen
  no p.leaseOn' and no p.waits'
  p.sel' = p.sel and p.gen' = p.gen and some p.spent'
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* OPEN-EPOCH — WHERE THE DRIVER'S ROTATION AND AN AMBIENT COMMAND'S MATCH PART
   COMPANY, which `admission-k51` said would happen here and could not model
   because it read no generation VALUE.  Three outcomes now, and they are total:

     STALE (`SY-10.a`)  an ambient operation whose generation is not the live
                        record's is REFUSED, and it takes nothing — not the
                        guard, not a wait.  This is the whole of *before it
                        touches the tree*: `doTreeOp` requires a session to HOLD
                        the epoch guard, and this is the only site that hands a
                        session one.
     FREE               the guard is taken; a DRIVER additionally ROTATES the
                        record, which is what makes every session launched under
                        the old identity stale from the next state on.
     HELD               a contended epoch BLOCKS — the ordinary tree-guard
                        discipline and the opposite of the lease's — and the
                        wait's only visible exit is `doGrant` or `doTimeout`.

   `mayTake` MOVED INSIDE THE NON-STALE BRANCH, deliberately.  A stale refusal
   acquires nothing, so making it wait on the guard ORDER would refuse a stale
   session for the wrong reason and leave `SY-10.a`'s antecedent unreachable
   whenever the order happened not to admit the acquisition. */
pred doOpenEpoch[p: Proc] {
  p in Alive.live and (no p.waits or p.waits = EpochG)
  validated and fresh[p]
  Sys.act' = OpenEpochA and Sys.actor' = p and Sys.gu' = EpochG
  worldSame and treeSame and verdictSame and aliveSame
  World.running' = World.running
  some p.spent'
  procsSameBut[p]

  stale[p] implies {
    Sys.res' = RefEpochStale
    actorSameBut[p]
    World.gen' = World.gen
  } else {
    mayTake[p, EpochG]
    no holder[EpochG] implies {
      Sys.res' = Applied
      p.holds' = p.holds + EpochG and p.seen' = p.seen + EpochG
      no p.waits' and p.leaseOn' = p.leaseOn
      p.sel' = p.sel and p.gen' = p.gen
      /* THE ROTATION.  A driver writes the record active with a FRESH identity;
         an ambient command only reads it. */
      p.role = DriverR implies (some World.gen' and World.gen' != World.gen)
                        else   World.gen' = World.gen
    } else {
      Sys.res' = Deferred
      p.holds' = p.holds and p.seen' = p.seen
      p.waits' = EpochG and p.leaseOn' = p.leaseOn
      p.sel' = p.sel and p.gen' = p.gen
      World.gen' = World.gen
    }
  }
}

pred doCloseEpoch[p: Proc] {
  p in Alive.live and EpochG in p.holds and TreeG not in p.holds and no p.waits
  validated and fresh[p]
  Sys.act' = CloseEpochA and Sys.actor' = p and Sys.gu' = EpochG
  Sys.res' = Applied
  p.holds' = p.holds - EpochG and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  p.sel' = p.sel and p.gen' = p.gen and some p.spent'
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

pred doTakeTree[p: Proc] {
  p in Alive.live and (no p.waits or p.waits = TreeG)
  mayTake[p, TreeG]
  Sys.act' = TakeTreeA and Sys.actor' = p and Sys.gu' = TreeG
  worldSame and treeSame and verdictSame and launchSame and aliveSame
  no holder[TreeG] implies {
    Sys.res' = Applied
    p.holds' = p.holds + TreeG and p.seen' = p.seen + TreeG
    no p.waits' and p.leaseOn' = p.leaseOn
  } else {
    Sys.res' = Deferred
    p.holds' = p.holds and p.seen' = p.seen
    p.waits' = TreeG and p.leaseOn' = p.leaseOn
  }
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  procsSameBut[p]
}

pred doDropTree[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  Sys.act' = DropTreeA and Sys.actor' = p and Sys.gu' = TreeG
  Sys.res' = Applied
  p.holds' = p.holds - TreeG and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* GRANT — a blocked process proceeding once its guard is free.  A SEPARATE
   ACQUISITION SITE, and the one an implementation is likeliest to write without
   the order rule, because the process "already asked".

   MEASURED, AND THE ANSWER IS NOT THE ONE THIS COMMENT FIRST CLAIMED: removing
   `mayTake` from here SURVIVES `SY-11.a`.  A grant cannot violate an order the
   wait already satisfied — `seen` does not change while a process is blocked —
   so the clause is a belt on a fastened braces.  It stays because a later slice
   that resets `seen` per iteration would make it load-bearing, and its
   survival is recorded in `README.md` rather than papered over.

   AND IT IS A SECOND ADMISSION SITE FOR THE GENERATION MATCH, which `SY-10.a`
   does not say and which this slice found by checking it.  A session refused as
   stale never waits, so a WAITING session was fresh when it asked — but the
   record can rotate while it is blocked, and a grant that only resumes the wait
   hands the guard to a session whose generation is no longer live.  The ADR's
   own words settle it: *shared-guard acquisition is the admission boundary*, and
   a grant IS an acquisition.  The re-check is here, and `SY_10a`'s second
   conjunct is what makes the omission of it visible. */
pred doGrant[p: Proc] {
  p in Alive.live and some p.waits and no holder[p.waits]
  mayTake[p, p.waits]
  Sys.act' = GrantA and Sys.actor' = p and Sys.gu' = p.waits
  (p.waits = EpochG and stale[p]) implies {
    Sys.res' = RefEpochStale
    p.holds' = p.holds and p.seen' = p.seen
  } else {
    Sys.res' = Applied
    p.holds' = p.holds + p.waits and p.seen' = p.seen + p.waits
  }
  no p.waits' and p.leaseOn' = p.leaseOn
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* ANY observation, creation or mutation of the task tree, as one opaque step.
   What distinguishes the three is the task-tree model's; what this slice needs
   is that NONE of them happens before the layout is proved, that NONE of them
   happens under an unvalidated configuration (`SY-04.b`), and that an ambient
   one happens only through an admission that matched (`SY-10.a`).

   `World.live in World.live'` is `EN-10` at this grain — the names are the
   counter and entries are never removed — and it is what lets `SY-08`'s witness
   INSERT a leaf during the launch window.  WHERE the entry lands and what it is
   called are `TT-01` – `TT-10`'s and appear nowhere here. */
pred doTreeOp[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  validated
  p.role = SessionR implies EpochG in p.holds
  Sys.act' = TreeOpA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  /* IT NEEDS A TREE AND NO LONGER MAKES ONE.  The admission slice wrote `some
     World.rooted'` — the tree exists OR IS CREATED HERE — because `initialise-
     root` had nowhere else to be.  It has somewhere now, and leaving creation
     inside the opaque step would answer `SY-06.a` by construction: a scaffold
     that is one indivisible mutation always carries whatever it carries. */
  some World.rooted
  /* AND IT NEVER ALLOCATES THE FINISH LEAF.  `SY-07.b` is a claim about the
     ACTOR of one specific mutation, and a single opaque step cannot carry an
     actor rule for one mutation and not another — so the mutation is split out
     (`doAllocFinish`) and this step frames the field it is about. */
  World.fin' = World.fin
  World.live in World.live'
  rootSame
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* FULL CONFIGURATION VALIDATION, and `SY-04.b` owns its content.  It is the
   configuration's exact analogue of `doLayoutPreflight`: it reads the LIVE
   configuration and consults no recorded verdict, and `SY_04b`'s biconditional
   is what turns that absence into a checked fact rather than a coding habit.
   `complete-session-configuration` is the subject — the personal file is
   validated in full before every tree mutation and again before every launch —
   and the model reads one bit of it, which is all any `SY-` obligation does. */
pred doValidateConfig[p: Proc] {
  p in Alive.live and some p.leaseOn
  Sys.act' = ValidateConfigA and Sys.actor' = p and no Sys.gu'
  World.cfg = ValidCfg implies Sys.res' = Applied
                        else   Sys.res' = RefConfigInvalid
  procsSame and worldSame and treeSame and verdictSame and launchSame and aliveSame
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
  validated and fresh[p]
  Sys.act' = LayoutPreflightA and Sys.actor' = p and no Sys.gu'
  World.layout = UnsupportedL implies Sys.res' = RefLayoutUnsupported
                              else    Sys.res' = Applied
  actorSameBut[p] and some p.spent'
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* The world's.  The layout is mutable while the lease is held — which is the
   glossary's `_Avoid_` on the preflight, and the whole reason `SY-03` exists. */
pred doTopologyChange {
  Sys.act' = TopologyChangeA and no Sys.actor' and no Sys.gu'
  Sys.res' = Environmental
  World.layout' != World.layout
  World.wt' = World.wt and World.cfg' = World.cfg
  procsSame and treeSame and verdictSame and launchSame and aliveSame
}

/* The world's.  Death is not a step the process takes: the kernel releases the
   advisory locks and the lease goes with them. */
pred doCrash[p: Proc] {
  p in Alive.live
  Sys.act' = CrashA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Environmental
  Alive.live' = Alive.live - p
  no p.holds' and no p.waits' and no p.leaseOn' and p.seen' = p.seen
  /* Death releases what the KERNEL holds and rewrites nothing else: `spent`,
     `sel` and `gen` are the dead process's own record, not a resource. */
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame
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
  worldSame and treeSame and verdictSame and launchSame and aliveSame
  no holder[EpochG] implies {
    Sys.res' = Applied
    p.holds' = p.holds + EpochG and p.seen' = p.seen + EpochG
    no p.waits' and p.leaseOn' = p.leaseOn
  } else {
    Sys.res' = Deferred
    p.holds' = p.holds and p.seen' = p.seen
    p.waits' = EpochG and p.leaseOn' = p.leaseOn
  }
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
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
  World.layout' = World.layout and World.cfg' = World.cfg
  /* AND THE TASK ROOT GOES WITH IT — which is the `SY-05` half of this row and
     the reason `treeSame` is gone from this predicate.  A new directory at the
     path has no task tree in it, so Grove now observes ABSENCE; but the grove
     itself is untouched, sitting in the old directory nobody can reach.
     `World.extant` keeps it, `World.rooted` loses it, and *absence is the
     complete fresh-tree discriminator* stops being true.  Grove would start a
     new grove over a live one, which is precisely what the assumption grants
     cannot happen. */
  no World.rooted' and no World.live' and no World.fin'
  no World.partial' and no World.legacy'
  World.extant' = World.extant                 // THE GROVE IS STILL THERE
  World.retired' = World.retired and World.proven' = World.proven
  procsSame and verdictSame and launchSame and aliveSame
}

// --- the loop's own step, which is what this slice adds --------------------

/* THE ITERATION BOUNDARY.  Three things end together, and that co-incidence is
   the definition of an iteration rather than an economy:

     `spent`  the turn's one Lifecycle transition is available again;
     `sel`    the selection stops being authoritative, which is precisely why a
              leaf added during the launch window is the NEXT iteration's work;
     `seen`   RESET, and `admission-k51` named this leaf as its owner.

   `seen' = p.holds` AND NOT `no p.seen'`.  `HeldImpliesTaken` is a construction
   fact — nothing holds a guard it has not acquired — so emptying `seen` while the
   driver still holds its lease across the boundary would make the boundary
   unsatisfiable for exactly the process whose loop it is.  The honest reading is
   that `seen` records the guards taken IN THIS ITERATION, and an iteration begins
   holding whatever the last one did not release.

   IT IS GUARDED ON `no p.waits`.  A blocked process has not finished its turn:
   its only exits are `doGrant` and `doTimeout`, which is what makes `SY-10.b`'s
   *never a silent park* checkable at all. */
pred doIter[p: Proc] {
  p in Alive.live and no p.waits
  Sys.act' = IterA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Environmental
  no p.spent' and no p.sel' and p.seen' = p.holds
  p.holds' = p.holds and p.waits' = p.waits
  p.leaseOn' = p.leaseOn and p.gen' = p.gen
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* SELECT — an Observation, NOT a Lifecycle action, so it does not spend the
   iteration's one transition.  `no p.sel` is *exactly once per iteration*; the
   pre-order walk that decides WHICH live leaf is `TT-11`'s and reaches this file
   as a non-deterministic choice among `World.live`.

   `some World.live` WAS A GUARD AND IS NOW A BRANCH, and this slice is where it
   changed.  The `iteration` slice declared exhaustion out of scope by making
   selection on a spent tree UNREACHABLE rather than by branching on it, and
   named `roots` as the owner.  `SY-07`'s antecedent is exactly that state — *when
   no live leaf remains* — so a guard that deletes it deletes the obligation's
   whole subject.  `Empty` is a SUCCESS (`TT-15.a`, §*Outcomes*), not a refusal;
   WHICH live leaf the non-empty branch returns is still `TT-11`'s. */
pred doSelect[p: Proc] {
  p in Alive.live and p.role = DriverR and some p.leaseOn and no p.waits
  validated
  no p.sel
  Sys.act' = SelectA and Sys.actor' = p and no Sys.gu'
  some World.live implies {
    Sys.res' = Reported
    p.sel' in World.live and some p.sel'
  } else {
    Sys.res' = Empty
    no p.sel'
  }
  p.holds' = p.holds and p.seen' = p.seen and p.waits' = p.waits
  p.leaseOn' = p.leaseOn and p.spent' = p.spent and p.gen' = p.gen
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* LAUNCH — the spawn, and the state that CLOSES `SY-08`'s window.

   `no p.holds & (EpochG + TreeG)` IS THE WINDOW'S SHAPE AND NOT AN OPTIMISATION.
   The glossary is explicit that the tree access lock is released before a
   foreground launch, and the ADR that every exclusive guard is released before
   spawn.  A launch that held the epoch guard would additionally make `SY-10`
   unstatable — no ambient command could ever match a generation, and the whole
   session path would be an empty universe.

   `World.running' = p.sel` IS THE CLAIM'S SUBJECT.  The driver launches the
   value it selected, and does not recompute from `World.live`; that one operand
   is what a mutation replaces, and `SY_08` is what catches it. */
pred doLaunch[p: Proc] {
  p in Alive.live and p.role = DriverR and some p.leaseOn and no p.waits
  validated and fresh[p]
  some p.sel
  no p.holds & (EpochG + TreeG)
  some World.gen                          // a live launch generation to bind
  no World.running                        // the loop runs one session at a time
  Sys.act' = LaunchA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  World.running' = p.sel and World.gen' = World.gen
  actorSameBut[p] and some p.spent'
  /* The child binds to the live record.  WHICH process becomes the child is
     machinery no `SY-` obligation reads; what matters is that it carries the
     identity the record holds now, so the next rotation makes it stale. */
  some s: Proc - p {
    s.role = SessionR and s in Alive.live
    s.gen' = World.gen
    s.holds' = s.holds and s.seen' = s.seen and s.waits' = s.waits
    s.leaseOn' = s.leaseOn and s.spent' = s.spent and s.sel' = s.sel
    all q: Proc - p - s | procSame[q]
  }
  worldSame and treeSame and verdictSame and aliveSame
}

/* REAP.  The child is gone; the record is NOT written inactive here, because
   this file models only the rotation write — see `World.gen`. */
pred doReap[p: Proc] {
  p in Alive.live and p.role = DriverR and some p.leaseOn and no p.waits
  validated and fresh[p]
  some World.running
  Sys.act' = ReapA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  no World.running' and World.gen' = World.gen
  actorSameBut[p] and some p.spent'
  procsSameBut[p] and worldSame and treeSame and verdictSame and aliveSame
}

/* THE TIMEOUT — `SY-10.b`'s visible stop.

   IT IS NON-DETERMINISTICALLY ENABLED AND CARRIES NO CLOCK, which is the
   catalogue's own instruction: §*Deliberate omissions* models clocks, timeouts
   and retry counts as non-determinism, on the grounds that a bounded handoff
   wait is a liveness property of the implementation and not of the protocol.
   SO THIS FILE NEVER SAYS THE TIMEOUT *WILL* FIRE.  What it says is that when a
   wait ends, it ends visibly — `Stopped` is a result the caller sees — and that
   the stop performs no tree access and no epoch rewrite, which is the ADR's own
   sentence.  A reader who takes `SY_10b` for a liveness property has read
   fairness into a file that assumes none.

   ONLY A GENERATION WAIT TIMES OUT.  The tree access lock BLOCKS — §*Outcomes*
   says so, and no invocation returns while it is held — so a `TreeG` wait has no
   stop, and `SY-10.b`'s subject is *a contended generation* and nothing else. */
pred doTimeout[p: Proc] {
  p in Alive.live and p.waits = EpochG
  Sys.act' = TimeoutA and Sys.actor' = p and Sys.gu' = EpochG
  Sys.res' = Stopped
  no p.waits'
  p.holds' = p.holds and p.seen' = p.seen and p.leaseOn' = p.leaseOn
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  procsSameBut[p] and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* The world's, and the configuration's exact analogue of `doTopologyChange`.
   A configuration proved once can change under the loop, which is what makes
   *validation precedes EVERY transition* a claim with content rather than a
   statement about a single startup check. */
pred doEditConfig {
  Sys.act' = ConfigEditA and no Sys.actor' and no Sys.gu'
  Sys.res' = Environmental
  World.cfg' != World.cfg
  World.wt' = World.wt and World.layout' = World.layout
  procsSame and treeSame and verdictSame and launchSame and aliveSame
}


// --- the task root's own lifecycle, which is what THIS slice adds -----------

/* WHAT EVERY GROVE-SIDE TREE ACTION BELOW REQUIRES, written once.  It is
   `doTreeOp`'s own preamble and nothing more: the exclusive guard, an unblocked
   process, a live configuration (`SY-04.b`), and — for an ambient process — an
   admission that matched (`SY-10.a`).  None of these transitions is a Lifecycle
   action, so none of them spends the iteration's one transition; `initialise-
   root` and `allocate-finish-leaf` are Tree-mutation and Finish actions in
   §*Actions*, and `resolve` is an Observation. */
pred mayTouchTree[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  validated
  p.role = SessionR implies EpochG in p.holds
}

/* INITIALISE-ROOT, AS FAR AS THE FORMAT WITNESS — and stopping there is the
   point rather than an economy.  §*States* says root initialisation makes the
   format witness visible LAST, and `PartialScaffold` is defined by the interval
   that leaves open.  A model in which scaffolding is one indivisible mutation
   has no such interval, so `SY-06.b`'s subject does not exist in it and the
   obligation is answered by construction.  Declared in `README.md` as this
   slice's abstraction: ONE catalogue action, TWO steps, because the state
   between them is what two obligations are about.

   THE IDENTITY IS FRESH AND THAT IS `SY-05.a`'s OWN WORD.  A missing task root
   means *start a NEW grove*: the minted identity is one no grove has ever
   carried — not a live one, and above all not a RETIRED one, which is the
   identity `finish-k8`'s counterexample handed back. */
pred doInitRoot[p: Proc] {
  mayTouchTree[p]
  no World.rooted
  Sys.act' = InitRootA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  some g: Grove {
    g not in (World.extant + World.retired)
    World.rooted' = g and World.extant' = World.extant + g
    World.partial' = g                    // the witness has not landed yet
  }
  no World.legacy'
  no World.live' and no World.fin'        // work arrives with the completion
  World.retired' = World.retired and World.proven' = World.proven
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* THE COMPLETION, AND IT IS ONE OPERATION WITH TWO ARRIVALS.  §*States* is
   explicit that every value the completion would write is fixed in advance, so
   completing is *a comparison followed by at most one append* — which means the
   step that finishes a scaffold Grove just started and the step that completes
   one an interruption left behind are THE SAME STEP.  Modelling them as two
   would make `SY-06.b`'s witness a different operation from `SY-06.a`'s, and the
   claim is that they are not.

   THE ENABLING OBSERVATION AND THE DECIDING OBSERVATION ARE DIFFERENT, AND THAT
   DIFFERENCE IS THE WHOLE OF `SY-06.b`.  What makes Grove LOOK at a root is that
   the format witness is absent — and at this scope *the format witness is
   absent* is exactly `partial + legacy`, their union, with no format witness in
   the signature.  What makes Grove COMPLETE one is `partial` alone: the exact
   known subset.  A gate that decided on the union would complete a `Legacy` tree
   as though Grove had scaffolded it, which is the sentence the obligation ends
   on, and `SY_06b`'s biconditional is what tells the two gates apart. */
pred doCompleteScaffold[p: Proc] {
  mayTouchTree[p]
  some World.rooted
  some (World.partial + World.legacy)     // ENABLED BY: no format witness
  Sys.act' = CompleteScaffoldA and Sys.actor' = p and no Sys.gu'
  World.rooted' = World.rooted and World.extant' = World.extant
  World.retired' = World.retired and World.proven' = World.proven
  World.fin' = World.fin
  some World.partial implies {            // DECIDED BY: the exact known subset
    Sys.res' = Applied
    no World.partial' and no World.legacy'
    some f: Leaf | f not in World.live and World.live' = World.live + f
  } else {
    Sys.res' = RefFormatLegacy            // and the tree is byte-identical
    World.partial' = World.partial and World.legacy' = World.legacy
    World.live' = World.live
  }
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* ALLOCATE-FINISH-LEAF.  Split out of `doTreeOp` rather than folded into it,
   because `SY-07.b` is a claim about the ACTOR OF ONE SPECIFIC MUTATION and one
   opaque step cannot carry an actor rule for one mutation and not another.

   TOTAL FOR A SESSION, GUARDED FOR A DRIVER, and the asymmetry is the
   catalogue's.  *No session SHALL create one* carries no precondition, so the
   refusal is reachable on any tree; *when no live leaf remains* is the driver's
   own precondition, so the driver's branch is exhaustion's.

   APPEND OR REUSE, and `World.fin` is a `set` so that EXACTLY ONE is checked
   rather than declared.  The reuse branch adds no entry: a finish leaf that
   already exists is made live again, which is what *appends OR REUSES* is
   distinguishing between. */
pred doAllocFinish[p: Proc] {
  mayTouchTree[p]
  some World.rooted and no World.partial and no World.legacy
  Sys.act' = AllocFinishA and Sys.actor' = p and no Sys.gu'
  rootSame
  p.role = SessionR implies {
    Sys.res' = RefReservedKind
    entriesSame
  } else {
    no World.live                         // EXHAUSTION — the claim's antecedent
    Sys.res' = Applied
    some World.fin implies {              // REUSE
      World.fin' = World.fin
      World.live' = World.live + World.fin
    } else {                              // APPEND
      some f: Leaf {
        f not in World.live
        World.fin' = f and World.live' = World.live + f
      }
    }
  }
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* THE PROOF, AS ONE OPAQUE STEP, AND IT IS `FN-11`/`FN-19`'s.  Everything that
   makes a commit provable — the witness slot, the manifest written and verified,
   the evacuation, the commit itself — is `crates/grove-finish/models/`'s and is
   named there.  What crosses the boundary is the single observation `SY-05.b`
   reads: the deletion of the grove at the name IS PROVEN.  Note what has NOT
   happened yet: the root is still present, holding every entry, which is
   `FN-11`'s own witness interval. */
pred doProveCommit[p: Proc] {
  mayTouchTree[p]
  some World.rooted and no World.proven
  no World.partial and no World.legacy
  Sys.act' = ProveCommitA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  World.proven' = World.rooted
  World.rooted' = World.rooted and World.extant' = World.extant
  World.retired' = World.retired
  World.partial' = World.partial and World.legacy' = World.legacy
  entriesSame
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* THE ATOMIC RENAME'S OBSERVABLE EFFECT, and only its effect: THE NAME FREES.
   `FN-19` owns the step — one rename, witness and evacuated tree intact — and
   the quarantine it renames INTO is the finish model's and is absent here.  What
   this file needs is that the name becomes free EXACTLY WHEN the deletion is
   proven, and that the fact Grove keeps about it is `retired` rather than the
   emptiness of a directory entry.

   THE GROVE LEAVES `extant` AND ENTERS `retired` IN THE SAME STEP, which is what
   makes absence a fact Grove ESTABLISHES.  Nothing afterwards can take it back:
   the world may put a tree at the freed name — `doForeignWrite` does exactly
   that, with this grove's own identity — and `retired` still says the deletion
   was proven.  That is `SY-05.a`'s second conjunct and it is entry 039 stated as
   a property. */
pred doSettleDeletion[p: Proc] {
  mayTouchTree[p]
  some World.rooted and World.proven = World.rooted
  Sys.act' = SettleDeletionA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  no World.rooted' and no World.proven'
  no World.partial' and no World.legacy'
  no World.live' and no World.fin'
  World.extant'  = World.extant  - World.rooted
  World.retired' = World.retired + World.rooted
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* RESOLVE — AN OBSERVATION OF THE ROOT'S PRESENCE, AND THE SITE `SY-05.a` EXISTS
   TO CONSTRAIN.  It reports and concludes NOTHING: `retired` is untouched here,
   and a mutant that grows it on an absent root is a driver reading absence as
   evidence that an earlier grove was torn down.  That mutant is wrong twice over
   — the name may be free because a finish succeeded, or because the world
   emptied it, and it may be OCCUPIED by a tree carrying a retired identity — so
   the only sound reading of the observation is the one below, which is none. */
pred doObserveRoot[p: Proc] {
  mayTouchTree[p]
  Sys.act' = ObserveRootA and Sys.actor' = p and no Sys.gu'
  some World.rooted implies Sys.res' = Reported else Sys.res' = Empty
  procsSame and worldSame and treeSame and verdictSame and launchSame and aliveSame
}

/* THE WORLD'S (`EN-11`: any well-formed tree is reachable by hand edit).  An
   operator's own tree at the name: present, no format witness, and NOT the known
   subset — a `Legacy` tree, which is the thing `SY-06.b` refuses to complete.
   It arrives by hand edit rather than by a Grove step because Grove has no
   action that builds one, and positing it in the initial state would make the
   `SY-06.b` refusal a property of state 0 rather than of the gate. */
pred doHandEdit {
  Sys.act' = HandEditA and no Sys.actor' and no Sys.gu'
  Sys.res' = Environmental
  no World.rooted
  some g: Grove {
    g not in (World.extant + World.retired)
    World.rooted' = g and World.extant' = World.extant + g
    World.legacy' = g
  }
  no World.partial'
  no World.live' and no World.fin'
  World.retired' = World.retired and World.proven' = World.proven
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* THE WORLD'S, AND IT IS ENTRY 039'S OWN TRACE (`EN-13`: foreign entries may
   appear at any name).  After the rename the task-root name is FREE.  The world
   occupies it — and it may give what it puts there the quarantined root's own
   identity, because an identity is bytes and bytes are copyable.  This is the
   step that killed three formulations of `FN-28` in the finish scope, and it is
   here so that `SY-05` is stated against it rather than beside it.

   `World.retired' = World.retired` IS THE CLAIM'S SUBJECT, not a frame.  The
   whole finding is that the re-occupation changes nothing Grove knows: the
   deletion was proven, the proof is durable, and a tree at the name is not
   evidence against it. */
pred doForeignWrite {
  Sys.act' = ForeignWriteA and no Sys.actor' and no Sys.gu'
  Sys.res' = Environmental
  no World.rooted
  some g: World.retired {
    World.rooted' = g and World.extant' = World.extant + g
  }
  no World.partial' and no World.legacy'
  no World.live' and no World.fin'
  World.retired' = World.retired        // PRESERVED — see above
  World.proven' = World.proven
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

pred step {
  doIdle
  or (some p: Proc | doAcquireLease[p] or doReleaseLease[p]
                     or doOpenEpoch[p] or doCloseEpoch[p]
                     or doTakeTree[p] or doDropTree[p] or doGrant[p]
                     or doTreeOp[p] or doValidateConfig[p]
                     or doLayoutPreflight[p] or doCrash[p]
                     or doNestedAcquire[p]
                     or doIter[p] or doSelect[p] or doLaunch[p]
                     or doReap[p] or doTimeout[p]
                     or doInitRoot[p] or doCompleteScaffold[p]
                     or doAllocFinish[p] or doProveCommit[p]
                     or doSettleDeletion[p] or doObserveRoot[p])
  or doTopologyChange
  or doEditConfig
  or doRemoveRoot
  or doHandEdit
  or doForeignWrite
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

/* THE CLASSIFICATION IS OF THE ROOT THAT IS THERE, and AT MOST ONE STATE HOLDS.
   Both are properties of a classification rather than claims about Grove:
   `TT-18` classifies a task root in a FIXED ORDER, and an ordered classification
   yields one state.  WHICH one — the subset comparison, the byte equality, the
   order that puts `PartialScaffold` before `Legacy` — is
   `crates/grove-task-tree/models/`'s and is imported here as the bare fact that
   the two are different.  A `SY-` obligation reads no more of `TT-18` than
   that, and importing more would be this file re-stating it. */
fact TheClassificationIsOfTheRootThatIsThere {
  always (World.partial + World.legacy) in World.rooted
}
fact AClassificationYieldsOneState {
  always no World.partial & World.legacy
}

/* A tree that is not there has no entries in it. */
fact AnAbsentRootHasNoEntries {
  always (no World.rooted implies (no World.live and no World.fin))
}

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

/* Likewise state 0 only, and likewise WHAT THE STEP ALREADY REQUIRED: `doSelect`
   returns a member of `World.live`, so a free initial state handing a driver a
   selection of something that was never in the tree is a state no step produces.
   NOT `always` — deliberately.  As an invariant it would be doing `SY-08`'s work
   for it, and `entries are never removed` (`EN-10`) is the task-tree model's
   claim rather than a fact this file may help itself to. */
fact SelectionsStartInsideTheTree {
  all p: Proc | some p.sel implies p.sel in World.live
}

/* A DRIVER MATCHES NO GENERATION; it owns the record.  `stale` is stated over
   `SessionR` alone for that reason, and this clause keeps the free initial state
   from handing a driver a generation binding no step could have given it — the
   only writer of `Proc.gen` is `doLaunch`, and it writes a session's. */
fact OnlyASessionCarriesAGeneration {
  always all p: Proc | p.role = DriverR implies no p.gen
}

/* State 0 only, and likewise WHAT THE STEPS ALREADY REQUIRE.  Every step keeps
   `extant` and `rooted` in step — the grove at the name is the grove that
   exists — so a free initial state handing out a grove that exists nowhere, or a
   grove existing with no name, is a state no step produces.

   NOT `always`, AND THAT IS THE WHOLE POINT.  As an invariant it would assert
   `SY-05.a`'s third conjunct — *absence is the COMPLETE fresh-tree
   discriminator* — as a construction fact, which is the shape `README.md`
   records twice as making a claim vacuous and every mutation against it survive.
   `EN-14`'s mutation must break the correspondence BY RUNNING, and it does:
   `doRemoveRoot` takes the name away and leaves the grove.

   The `proven` clause is the same move: a proof is about the grove at the name,
   and only `doProveCommit` writes one. */
fact GrovesStartAtTheirName {
  World.extant = World.rooted
  some World.proven implies World.proven = World.rooted
}

/* Likewise state 0 only, and likewise WHAT THE STEP ALREADY REQUIRED.
   `doAllocFinish` is the only site that puts a finish leaf into `World.fin` and
   it writes exactly one; every other site frames the field or empties it with
   the whole tree.  So a free initial state holding TWO is a state no step
   produces — and it was `SY_07a`'s first counterexample, at state 0, with no
   transition in the trace at all.

   NOT `always`, and the distinction is the whole reason `World.fin` is a `set`.
   As an invariant this would assert `SY-07.a`'s *exactly one* by construction
   and every mutation against it would survive, which is the shape `README.md`
   records twice.  As a state-0 fact it says only that the trace starts
   somewhere the steps could have reached, and the mutation that appends a
   second finish leaf over an existing one still fires from a legal start.
   WHAT IT IS NOT: `TT-13`'s *more than one live finish leaf malforms the whole
   tree*.  That is a task-tree claim about a hand-edited tree, and it is
   `crates/grove-task-tree/models/`'s. */
fact TheTreeStartsWithAtMostOneFinishLeaf { lone World.fin }

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
} for 3 but 2 WtId, 4 steps

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
    /* STATED OVER `TreeAct` AND NOT OVER `TreeOpA`, which is this slice's one
       edit to an inherited check and is a strengthening.  The admission slice
       had one tree action; `roots` adds six, and a claim written as a list would
       have gone stale silently.  A seventh added by `sessions` reaches this
       conjunct without the command being touched. */
    (Sys.act' in TreeAct and Sys.actor'.role = DriverR)
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
} for 3 but 2 WtId, 4 steps

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
} for 3 but 2 WtId, 6 steps

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
} for 3 but 2 WtId, 7 steps

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
// CLAIMS — SY-04, SY-08, SY-10   (the `iteration` slice)
//
// THE LOOP'S OWN STEP.  Everything above is admission — who may hold the loop,
// on what layout, in what guard order.  Everything here is the turn the loop
// takes once it holds it: the boundary, the validation ahead of every
// transition, the selection taken once, the launch window, and the generation
// staleness with its visible stop.
//
// TWO OF THE THREE CLAIMS ARE STATED OVER THE TRANSITION SET RATHER THAN OVER A
// LIST OF NAMES, for the reason `SY-11.a` already gave: a list goes stale the
// moment a slice adds a site.  `SY-04.a` quantifies over `LifecycleAct` and
// `SY-04.b` over the same set minus its one exemption, so the two obligations
// `sessions` will add reach both without either command being edited.
// ===========================================================================

// --- SY-04: one lifecycle transition an iteration, under a live config ------

/* SY-04.a.  TWO CONJUNCTS AND THEY ARE COMPLEMENTARY, which is what makes *at
   most one* a claim rather than a guard restated:

     the PROHIBITION — a Lifecycle transition happens only in an iteration that
     has not spent one.  Drop the guard from any site and this half fires.
     the CONSUMPTION — and it spends it.  A site that took its transition
     without marking the iteration would leave the prohibition true and the
     claim false, and only this half sees it.

   `Sys.actor'.spent` reads the actor's PRE-state through its post-state
   identity, which is the file's inherited idiom (`Sys.actor'.waits' =
   Sys.actor'.waits` in `SY_01a`). */
check SY_04a_at_most_one_lifecycle_transition_per_iteration {
  Assumed implies always {
    (Sys.act' in LifecycleAct)
      implies (no Sys.actor'.spent and some Sys.actor'.spent')
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness is *each transition, taken alone* — a witness PER
   LIFECYCLE TRANSITION and not one witness, so the seven below are one
   instrument applied seven times.  ONE HELPER RATHER THAN SEVEN COPIES, because
   the thing being witnessed is identical in each and a reader should be able to
   see that at a glance: an iteration boundary, one Lifecycle transition, the
   next boundary, all three the same process's. */
pred aloneInAnIteration[a: Action] {
  some p: Proc |
    eventually (Sys.act = IterA and Sys.actor = p and after (
      (Sys.act not in LifecycleAct) until (
        Sys.act = a and Sys.actor = p and after (
          (Sys.act not in LifecycleAct) until (Sys.act = IterA and Sys.actor = p)))))
}

run witness_SY_04a_acquire_lease_alone     { Assumed and aloneInAnIteration[AcquireLeaseA]    } for 3 but 2 WtId, 6 steps
run witness_SY_04a_layout_preflight_alone  { Assumed and aloneInAnIteration[LayoutPreflightA] } for 3 but 2 WtId, 6 steps
run witness_SY_04a_open_epoch_alone        { Assumed and aloneInAnIteration[OpenEpochA]       } for 3 but 2 WtId, 6 steps
run witness_SY_04a_launch_alone            { Assumed and aloneInAnIteration[LaunchA]          } for 3 but 2 WtId, 6 steps
run witness_SY_04a_reap_alone              { Assumed and aloneInAnIteration[ReapA]            } for 3 but 2 WtId, 6 steps
run witness_SY_04a_close_epoch_alone       { Assumed and aloneInAnIteration[CloseEpochA]      } for 3 but 2 WtId, 6 steps
run witness_SY_04a_release_lease_alone     { Assumed and aloneInAnIteration[ReleaseLeaseA]    } for 3 but 2 WtId, 6 steps

/* SY-04.b.  Three conjuncts, and together they are *full configuration
   validation precedes every transition, so an invalid configuration leaves the
   working tree byte-identical*:

     1  A BICONDITIONAL, exactly as `SY-03`'s is, and for the same reason.
        Validation that consulted a recorded verdict would pass on a
        configuration that had since gone invalid, which left-to-right catches;
        one that refused a valid configuration because no verdict existed is a
        different bug, which right-to-left catches.
     2  every Lifecycle transition BUT THE LEASE GATE runs under a valid
        configuration.  The exemption is not a weakening, it is `SY-02`'s own
        word: an unsupported workspace is refused at lease acquisition *before
        configuration validation*, so the gate that runs before validation
        cannot be gated on it.  The two obligations would otherwise contradict.
     3  and therefore an invalid configuration LEAVES THE TREE ALONE.

   *BYTE-IDENTICAL*, READ AT THIS SCOPE'S GRAIN, AND THE READING IS PART OF THE
   CLAIM.  This file's task root is present or absent and its leaves are opaque
   handles — there is no byte here to compare.  What conjunct 3 says is the
   strongest thing the composition boundary admits: under an invalid
   configuration the tree's presence does not change and no entry appears.  A
   model that could see a byte would say more; this one says what it can and
   `README.md` records that it is a reading rather than the claim entire. */
check SY_04b_full_validation_precedes_every_transition {
  Assumed implies always {
    (Sys.act' = ValidateConfigA)
      implies ((Sys.res' in Refused) iff (World.cfg' = InvalidCfg))
    (Sys.act' in LifecycleAct - AcquireLeaseA)
      implies World.cfg' = ValidCfg
    /* `Sys.res' != Environmental` IS NEW WITH THIS SLICE AND IT IS A CORRECTION
       RATHER THAN A WEAKENING.  `roots` is the first slice to give the WORLD a
       way to write the tree — `hand-edit` (`EN-11`) and `foreign-write`
       (`EN-13`), both catalogue Environment actions — and an invalid
       configuration constrains GROVE, not the operator.  Without the exclusion
       this conjunct reads *a bad config stops the operator editing their own
       directory*, which is false and is not what `SY-04.b` says. */
    (World.cfg = InvalidCfg and Sys.res' != Environmental)
      implies (World.rooted' = World.rooted and World.live' = World.live)
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue words this witness *reached*, which a runner can land with
   almost nothing.  What it lands instead is the situation the claim exists for:
   A CONFIGURATION THAT GOES INVALID UNDER A RUNNING LOOP.  The refusal must come
   from a validation that ran AFTER the edit — a model in which the configuration
   is fixed for the trace witnesses a startup check and calls it *every*. */
run witness_SY_04b_a_configuration_that_goes_invalid_leaves_the_tree_untouched {
  Assumed
  always no World.rooted
  eventually (Sys.act = ConfigEditA and World.cfg = InvalidCfg
              and after eventually (Sys.act = ValidateConfigA
                                    and Sys.res = RefConfigInvalid))
} for 3 but 2 WtId, 5 steps

// --- SY-08: selection is authoritative once per iteration -------------------

/* SY-08.  Two conjuncts, and the claim's *so that* clause is their consequence
   rather than a third:

     TAKEN ONCE — a standing selection changes only at ITS OWN process's
     iteration boundary.  `doSelect` refuses to run with one standing, so this
     conjunct is what forbids every OTHER site from quietly rewriting it.
     NOT RECOMPUTED — the launch launches the value selected.  This is the one
     operand a mutation replaces: a launch that read `World.live` afresh would
     be indistinguishable from this one on every trace where nothing was
     inserted, and identical to the bug on every trace where something was.

   Together they are *a leaf added during the launch window becomes the next
   iteration's work*: the window is the states between the select and the
   launch, and nothing that happens inside it can reach `p.sel`. */
check SY_08_selection_is_authoritative_and_is_not_recomputed {
  Assumed implies always {
    all p: Proc | (some p.sel and not (Sys.act' = IterA and Sys.actor' = p))
      implies p.sel' = p.sel
    (Sys.act' = LaunchA) implies World.running' = Sys.actor'.sel
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: A LEAF INSERTED DURING THE LAUNCH WINDOW.  The
   three states are consecutive on purpose — the insertion must fall STRICTLY
   between the select and the launch, or the trace witnesses an insertion before
   selection (which the claim says nothing about) or after the launch (which is
   the next iteration's ordinary case).  `b not in World.live` at the select and
   `b in World.live` at the launch is what makes the window a state the trace
   PASSES THROUGH rather than an atomic step. */
run witness_SY_08_a_leaf_inserted_during_the_launch_window {
  Assumed
  some disj a, b: Leaf | some d: Proc {
    d.role = DriverR
    eventually (Sys.act = SelectA and Sys.actor = d and d.sel = a
                and b not in World.live
                and after (Sys.act = TreeOpA and b in World.live
                           and after (Sys.act = LaunchA and Sys.actor = d
                                      and World.running = a
                                      and b in World.live)))
  }
} for 3 but 2 WtId, 6 steps

// --- SY-10: a stale session cannot act --------------------------------------

/* SY-10.a.  Three conjuncts, and the middle one is the claim's teeth.

     1  the REFUSAL: an ambient operation whose generation is not the record's
        is refused, and takes nothing — not the guard, not even a wait.  A model
        that let it queue would satisfy *refused* and lose *before it touches the
        tree*, since a queued session is granted later.
     2  A SESSION ACQUIRES THE EPOCH GUARD ONLY AT A MATCHING GENERATION, over
        every acquisition rather than at the one site — which is `SY-11.a`'s
        shape borrowed deliberately, and it is what found the grant site.  The
        catalogue reads as though admission happened once; a wait spans a
        rotation, and a grant that only resumes the wait admits a session whose
        generation died while it was blocked.
     3  and an ambient TREE OPERATION happens only while holding a guard so
        acquired.  This is the seam `admission-k51` declared owed: `SY-02`'s
        fourth conjunct is a driver's only, because a session reaches the tree
        through a generation and not through a lease.  IT LANDS HERE, as
        `SY-10`'s, and not by widening `SY_02`.

   Conjunct 2 is why 1 and 3 compose into *cannot act* rather than merely
   *is refused once*: the guard is the only thing that admits a tree operation,
   and the match is the only thing that admits the guard.

   SEVEN STATES, AND THE NUMBER IS A CORRECTION RATHER THAN A CHOICE.  This
   check was written at five, was green, and its conjunct-2 mutation SURVIVED —
   because the defect needs a rotation to happen WHILE a session is blocked, and
   that is six transitions: the wait, the holder's release, the driver's own
   iteration boundary, the driver's rotation, the driver's death (a crash, which
   is not a Lifecycle transition and so needs no second boundary), and the grant.
   At six it still survived.  The sibling scopes' second vacuity predictor says
   exactly this — THE BOUND MUST HOLD THE MACHINERY OF THE TRANSITIONS THE
   OBLIGATION QUANTIFIES OVER, not only the objects it names — and *every
   acquisition by a session* quantifies over the grant site, whose antecedent no
   five-state trace can build. */
check SY_10a_a_stale_session_is_refused_before_it_touches_the_tree {
  Assumed implies always {
    (Sys.act' = OpenEpochA and Sys.actor'.role = SessionR
      and (no Sys.actor'.gen' or Sys.actor'.gen' != World.gen'))
      implies (Sys.res' = RefEpochStale
               and Sys.actor'.holds' = Sys.actor'.holds
               and no Sys.actor'.waits'
               and World.rooted' = World.rooted
               and World.live' = World.live)
    all p: Proc | (p.role = SessionR and EpochG in (p.seen' - p.seen))
      implies (some p.gen' and p.gen' = World.gen')
    (Sys.act' in TreeAct and Sys.actor'.role = SessionR)
      implies EpochG in Sys.actor'.holds'
  }
} for 3 but 2 WtId, 7 steps

/* The catalogue's own witness: A STALE SESSION REFUSED.  Run through the
   rotation rather than from a free initial mismatch: the session is launched
   under the live record, the driver rotates, and the SAME session is then
   refused, which is the situation the glossary describes and a free state-0
   mismatch would only resemble.

   `always Sys.act not in TreeAct` IS *BEFORE IT TOUCHES THE TREE*, AND IT
   REPLACES `always no World.rooted`, WHICH THE `roots` SLICE BROKE.  The
   iteration slice said it with the root's absence — no task root in any state,
   so the refusal cannot be one that arrived after a read — and that reading
   stopped being available the moment `AnAbsentRootHasNoEntries` landed: a
   rootless trace now has no live leaf, so it has no selection, so it has no
   LAUNCH, and the witness needs one.  The replacement is the stronger statement
   and the one the claim actually makes: no tree action occurs anywhere in the
   trace, whether or not a tree is there to act on.  Recorded in `README.md` as
   this slice's one witness correction. */
run witness_SY_10a_a_stale_session_refused {
  Assumed
  always Sys.act not in TreeAct
  some s: Proc {
    s.role = SessionR
    eventually (Sys.act = LaunchA and s.gen = World.gen
                and after eventually (Sys.act = OpenEpochA and Sys.actor = s
                                      and Sys.res = RefEpochStale))
  }
} for 3 but 2 WtId, 7 steps

/* SY-10.b.  Two conjuncts.

   THIS IS NOT A LIVENESS PROPERTY AND MUST NOT BE READ AS ONE.  §*Deliberate
   omissions* models clocks, timeouts and retry counts as NON-DETERMINISM, on the
   stated grounds that a bounded handoff wait is a liveness property of the
   implementation rather than of the protocol.  Nothing below says the timeout
   will fire; these models assume no fairness and have no grounds to.

     THE STOP IS VISIBLE AND INERT.  `Stopped` is a result the caller sees, and
     the ADR's own sentence is the rest: *a timeout performs no tree access or
     epoch rewrite*.
     AND NO WAIT ENDS SILENTLY.  A generation wait ends only in the WAITER'S OWN
     step, and that step reports something — never in another process's step and
     never as an environmental non-event.  That is what *never a silent park*
     can mean without a clock: the caller is never left having waited and been
     told nothing. */
check SY_10b_a_contended_generation_stops_visibly_and_never_parks_silently {
  Assumed implies always {
    (Sys.act' = TimeoutA)
      implies (Sys.res' = Stopped
               and no Sys.actor'.waits'
               and World.gen' = World.gen
               and World.rooted' = World.rooted
               and World.live' = World.live)
    all p: Proc | (p.waits = EpochG and no p.waits' and p in Alive.live')
      implies (Sys.actor' = p and Sys.res' != Environmental)
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: A TIMEOUT REPORTED.  Both halves in one trace —
   a wait a step PRODUCED, and the stop that ends it.  A timeout with no
   preceding `Deferred` would witness a stop nobody was waiting for. */
run witness_SY_10b_a_contended_generation_times_out_into_a_visible_stop {
  Assumed
  some p: Proc |
    eventually (Sys.res = Deferred and Sys.actor = p and p.waits = EpochG
                and after eventually (Sys.act = TimeoutA and Sys.actor = p
                                      and Sys.res = Stopped))
} for 3 but 2 WtId, 5 steps



// ===========================================================================
// CLAIMS — SY-05, SY-06, SY-07   (the `roots` slice)
//
// THE TASK ROOT'S OWN LIFECYCLE.  Admission is who may hold the loop; iteration
// is the turn it takes; this is what the loop is holding — a grove that is
// scaffolded, worked, exhausted, finished, and whose name then belongs to
// nobody.
//
// ABSENCE IS NOT A STATE HERE, IT IS A FACT GROVE ESTABLISHES.  Every command
// below is stated over `World.retired` and `World.extant` rather than over the
// emptiness of `World.rooted`, and the reason is one trace: after the rename the
// NAME IS FREE, the world may occupy it, and it may give what it puts there the
// quarantined root's own identity.  `finish-k8` killed three formulations of
// `FN-28` on it (`docs/formalism-findings.md` entry 039) and
// `witness_SY_05a_the_name_reoccupied_with_the_retired_identity` reproduces it
// here — as a WITNESS, because in this formulation it is the ordinary case
// rather than a counterexample.
// ===========================================================================

// --- SY-05: absence is the complete fresh-tree discriminator ----------------

/* SY-05.a.  Four conjuncts, and they are the four things *a missing task root
   means start a new grove, and is never read as evidence about an earlier one*
   decomposes into once absence is a fact rather than a state:

     ESTABLISHED — `retired` is written by the proven deletion and by nothing
     else.  This is the conjunct with a driver-shaped mutation behind it: a
     `resolve` that concluded *the earlier grove was torn down* from an empty
     directory is a real implementation, and it is wrong in both directions —
     the name may be free because the world emptied it, and it may be OCCUPIED
     by a tree carrying a retired identity.
     PRESERVED — and nothing takes it back.  `doForeignWrite` is the adversary
     and this conjunct is what it is stated against.
     COMPLETE — the claim's own title.  Absence discriminates a fresh tree only
     because a grove that is not at the name does not exist, and that is exactly
     what `EN-14` grants and what `expect_fail_EN_14_SY_05a` removes.
     FRESH — *a NEW grove*.  The minted identity is one no grove has carried:
     not a live one, and above all not a retired one.

   THE THIRD CONJUNCT LOOKS LIKE A CONSTRUCTION FACT AND DELIBERATELY IS NOT
   ONE.  `GrovesStartAtTheirName` constrains state 0 and says nothing about
   `always`; written as an invariant it would assert this conjunct, make the
   `EN-14` control unsatisfiable, and report exactly as a pass.  The file has
   already made that mistake once — `always all g: Guard | lone holds.g` — and
   the control caught it then too. */
check SY_05a_absence_is_established_preserved_complete_and_fresh {
  Assumed implies always {
    (World.retired' != World.retired)
      implies (Sys.act' = SettleDeletionA and World.proven = World.rooted)
    World.retired in World.retired'
    no World.rooted implies no World.extant
    (Sys.act' = InitRootA)
      implies (no World.rooted and some World.rooted'
               and World.rooted' not in (World.extant + World.retired))
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: A COMPLETED TEARDOWN WHOSE DRIVER NEVER OBSERVED
   THE SIGNAL, FOLLOWED BY A FRESH SCAFFOLD.  `always Sys.act != ObserveRootA` is
   *never observed*, and it is what makes the witness say something: the fresh
   scaffold is not a decision taken on evidence about the old grove, because no
   observation of the root happens anywhere in the trace.  `g1 != g0` is the
   *new* in *start a NEW grove*. */
run witness_SY_05a_a_completed_teardown_and_then_a_fresh_scaffold {
  Assumed
  always Sys.act != ObserveRootA
  some disj g0, g1: Grove |
    eventually (Sys.act = SettleDeletionA and no World.rooted and g0 in World.retired
                and after eventually (Sys.act = InitRootA and World.rooted = g1))
} for 3 but 2 WtId, 6 steps

/* ENTRY 039, REPRODUCED — and it is a WITNESS here rather than a counterexample,
   which is the whole content of the formulation this leaf was handed.  The
   finish scope met this trace as the thing that falsified *the task root is
   absent*; stated over `retired`, the same trace is the ordinary case: the world
   owns the name, it has given its tree the quarantined root's identity, and
   Grove's record of the proven deletion is untouched.  A file that had stated
   absence as an invariant would be reading this instance as a defect. */
run witness_SY_05a_the_name_reoccupied_with_the_retired_identity {
  Assumed
  eventually (Sys.act = SettleDeletionA
              and after eventually (Sys.act = ForeignWriteA
                                    and some World.rooted
                                    and World.rooted in World.retired))
} for 3 but 2 WtId, 6 steps

/* SY-05.b.  THE JOINT CLAIM, AND ITS PLACEMENT IS THIS SUBTREE'S SHARPEST
   QUESTION.  The catalogue says `SY-05` and `FN-11`/`FN-19` SHALL be checked
   together, and an `FN_`-prefixed command in `models/system/` is a placement
   failure the runner refuses.  What this file owes is therefore the OBSERVATION
   — no trace exposes an absent task root before the deletion is proven — stated
   over its OWN transitions, with `crates/grove-finish/models/` named as the
   owner of the steps underneath: the published witness, the verified manifest,
   the evacuation and the one atomic rename are `FN-11`'s and `FN-19`'s and are
   `doProveCommit` and `doSettleDeletion` here.

   THE CHECK IS THE WITNESS.  The catalogue words `SY-05.b`'s witness as *the
   exhaustive absence of such a trace within the bound*, which no runner lands as
   an instance; what lands beside it is the non-vacuity run below, because an
   exhaustive absence over an unreachable antecedent is the vacuous invariant the
   pre-registration names.  The bound is five states and `README.md` records it,
   which is what *within the bound* obliges. */
check SY_05b_no_absent_task_root_before_the_deletion_is_proven {
  Assumed implies always {
    (some World.rooted and no World.rooted')
      implies (Sys.act' = SettleDeletionA
               and World.proven = World.rooted
               and World.rooted in World.retired')
  }
} for 3 but 2 WtId, 5 steps

/* The non-vacuity run: ABSENCE IS REACHED, and only through the proof.  Without
   it the check above is exhaustive over nothing. */
run witness_SY_05b_absence_is_reached_and_only_after_a_proven_deletion {
  Assumed
  eventually (Sys.act = ProveCommitA
              and after eventually (no World.rooted and some World.retired))
} for 3 but 2 WtId, 5 steps

// --- SY-06: a fresh root carries a first live leaf --------------------------

/* SY-06.a.  *Scaffolding SHALL produce work, not only a charter, so a fresh
   grove is never indistinguishable from a finished one* — and the second clause
   is what the conjuncts are chosen to make checkable.  At this scope a fresh
   grove and a finished one differ in exactly one observation, `some World.live`,
   and the check says the completion lands on the near side of it.  The
   classification clauses are the same sentence's other half: a completed
   scaffold is `Current`, so neither of the no-format-witness observations
   survives it, and a scaffold that completed into a still-partial root would be
   a grove nobody could tell from an interrupted one. */
check SY_06a_a_completed_scaffold_carries_a_first_live_leaf {
  Assumed implies always {
    (Sys.act' = CompleteScaffoldA and Sys.res' = Applied)
      implies (some World.rooted' and some World.live'
               and no World.partial' and no World.legacy')
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: A FRESH ROOT, DISTINGUISHABLE FROM A SPENT ONE.
   Both observations in ONE trace, or the witness shows a fresh grove and leaves
   *distinguishable* to the reader.  The spent grove is a `Current` root with no
   live leaf — reached here from the free initial state rather than by retiring
   leaves one at a time, because retirement is `TT-17`'s and this file has no
   leaf state; `EN-11` is what makes positing it legitimate. */
run witness_SY_06a_a_fresh_root_distinguishable_from_a_spent_one {
  Assumed
  eventually (some World.rooted and no World.live
              and no World.partial and no World.legacy)
  eventually (Sys.act = CompleteScaffoldA and Sys.res = Applied and some World.live)
} for 3 but 2 WtId, 7 steps

/* SY-06.b.  A BICONDITIONAL AND A REFUSAL, and the biconditional is the whole
   instrument — as it is in `SY-03` and `SY-04.b`, and for the same reason.

   The transition is ENABLED on `partial + legacy`, which at this scope is
   exactly *the format witness is absent*.  Left to right, the biconditional
   catches a gate that decided on that union: it would complete a `Legacy` tree
   as though Grove had scaffolded it, which is the sentence the obligation ends
   on.  Right to left, it catches a gate that refused a genuine partial scaffold
   — the failure `PartialScaffold` was invented to prevent, since the state
   exists only so that an interrupted `initialise-root` is completable at all.
   An implication either way would leave one of them invisible.

   The second conjunct is `Refused`'s own definition applied at this grain: the
   tree is byte-identical, which here is the root, its entries and its
   classification all unchanged. */
check SY_06b_the_exact_subset_completes_and_a_legacy_tree_is_refused {
  Assumed implies always {
    (Sys.act' = CompleteScaffoldA)
      implies ((Sys.res' = Applied) iff some World.partial)
    (Sys.act' = CompleteScaffoldA and some World.legacy)
      implies (Sys.res' = RefFormatLegacy
               and World.rooted' = World.rooted
               and World.live'   = World.live
               and World.legacy' = World.legacy)
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's first witness: AN INTERRUPTED SCAFFOLD, COMPLETED — and the
   interruption is a `crash` rather than a trace that simply stops, which is
   `EN-08` doing the work it exists for.  A SUCCESSOR completes it: the process
   that started the scaffold is dead, so the completion is not the same
   invocation finishing its own transaction, which is the situation the state
   exists for and the one a shorter trace would quietly not show. */
run witness_SY_06b_an_interrupted_scaffold_completed_by_a_successor {
  Assumed
  some disj p, q: Proc {
    p.role = DriverR and q.role = DriverR
    eventually (Sys.act = InitRootA and Sys.actor = p and some World.partial
                and after eventually (Sys.act = CrashA and Sys.actor = p
                  and after eventually (Sys.act = CompleteScaffoldA and Sys.actor = q
                                        and Sys.res = Applied)))
  }
} for 3 but 2 WtId, 8 steps

/* The catalogue's second witness: A LEGACY TREE, REFUSED RATHER THAN COMPLETED.
   The hand edit is what puts it there — Grove has no action that builds one —
   and the refusal is the state after it. */
run witness_SY_06b_a_legacy_tree_refused_rather_than_completed {
  Assumed
  eventually (Sys.act = HandEditA and some World.legacy
              and after eventually (Sys.act = CompleteScaffoldA
                                    and Sys.res = RefFormatLegacy))
} for 3 but 2 WtId, 4 steps          // 5 costs 10.42 s for the same margin — see README

// --- SY-07: exhaustion yields exactly one finish leaf -----------------------

/* SY-07.a.  Five conjuncts inside the implication plus one invariant beside it,
   and the invariant is the one that could not be a declaration.  `World.fin` is
   a `set`, so *EXACTLY ONE driver-owned finish leaf* is something the check
   establishes: `lone World.fin` in every state, and `one World.fin'` at the
   allocation.  Declared `lone`, the claim would have been true by construction
   and every mutation against it would have survived — which is the failure this
   file has recorded twice and predicted once.

   `World.fin' = World.fin or no World.fin` IS *APPENDS OR REUSES* AND NOTHING
   ELSE: either the finish leaf that was already there is the one that comes
   back, or there was none and one is appended.  A third possibility — a second
   leaf appended over an existing one — is what the disjunction excludes and what
   `one World.fin'` catches from the other side. */
check SY_07a_exhaustion_yields_exactly_one_driver_owned_finish_leaf {
  Assumed implies always {
    (Sys.act' = AllocFinishA and Sys.res' = Applied)
      implies (Sys.actor'.role = DriverR
               and no World.live
               and one World.fin'
               and World.fin' in World.live'
               and (World.fin' = World.fin or no World.fin))
    lone World.fin
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witnesses, and it names TWO — *an append; a reuse* — which
   is one instrument applied to the two branches the claim distinguishes.  Each
   pins the PRE-state, because *appends or reuses* is a statement about what was
   already there and a run that only pinned the post-state would land the same
   instance twice. */
run witness_SY_07a_an_append {
  Assumed
  eventually (some World.rooted and no World.fin and no World.live
              and after (Sys.act = AllocFinishA and Sys.res = Applied
                         and one World.fin and World.fin in World.live))
} for 3 but 2 WtId, 4 steps

run witness_SY_07a_a_reuse {
  Assumed
  some f: Leaf |
    eventually (World.fin = f and no World.live
                and after (Sys.act = AllocFinishA and Sys.res = Applied
                           and World.fin = f and f in World.live))
} for 3 but 2 WtId, 4 steps

/* SY-07.b.  Two conjuncts, and the second is the one that makes the first mean
   *no session CREATES one* rather than *no session takes this transition*.

     the REFUSAL — a session's allocation is refused, naming the reserved kind,
     and the tree is byte-identical.
     and THE ONLY WAY ONE COMES INTO EXISTENCE is a driver's allocation.  This is
     stated over every transition, which is why `allocate-finish-leaf` had to be
     SPLIT OUT of `doTreeOp` rather than folded into it: `doTreeOp` is one opaque
     step available to both roles, and one opaque step cannot carry an actor rule
     for one mutation and not another.  Without the split this conjunct would be
     false on every trace where a session touched the tree at all.

   `some World.fin'` scopes conjunct 2 to a finish leaf ARRIVING.  A finish leaf
   ceasing to exist is a whole-tree deletion — `doSettleDeletion` and, under its
   own mutation, `doRemoveRoot` — and neither is a creation. */
check SY_07b_no_session_creates_a_finish_leaf {
  Assumed implies always {
    (Sys.act' = AllocFinishA and Sys.actor'.role = SessionR)
      implies (Sys.res' = RefReservedKind
               and World.fin' = World.fin and World.live' = World.live
               and World.rooted' = World.rooted)
    (World.fin' != World.fin and some World.fin')
      implies (Sys.act' = AllocFinishA and Sys.actor'.role = DriverR)
  }
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: A REFUSED CREATION.  The session must be one that
   could otherwise have mutated the tree — it holds the epoch guard and the tree
   guard — or the trace witnesses a refusal that the admission gates had already
   made unreachable, which is a different claim and a weaker one. */
run witness_SY_07b_a_refused_creation {
  Assumed
  some s: Proc {
    s.role = SessionR
    eventually (Sys.act = AllocFinishA and Sys.actor = s
                and EpochG in s.holds and TreeG in s.holds
                and Sys.res = RefReservedKind)
  }
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
   so a second driver is admitted. */
check expect_fail_EN_14_SY_01a_ownership_has_nothing_to_be_held_on {
  (some Env.rootGone and EN07) implies always lone { p: Alive.live | some p.leaseOn }
} for 3 but 2 WtId, 5 steps

/* EN-14's SECOND HALF, AND IT IS THE ONE THE ADMISSION SLICE DECLARED OWED.  The
   assumption table's row names `SY-01` and `SY-05` together, and the reason is
   one step: the working-tree root is replaced under the loop, so the task root
   goes with it — a new directory at the path has no task tree in it — while the
   GROVE is untouched, sitting in the directory nobody can reach any more.
   Expected: `SY-05.a`'s third conjunct fails.  Absence stops discriminating a
   fresh tree, and Grove would scaffold a new grove over a live one.

   STATED OVER THE THIRD CONJUNCT ALONE, on purpose.  `SY-05.b` fails under the
   same scope and for the same step, and a control that named both would be
   reporting one counterexample as two; `README.md` records the second
   consequence rather than commanding it. */
check expect_fail_EN_14_SY_05a_absence_stops_discriminating_a_fresh_tree {
  (some Env.rootGone and EN07)
    implies always (no World.rooted implies no World.extant)
} for 3 but 2 WtId, 5 steps
