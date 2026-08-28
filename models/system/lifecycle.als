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
 * COVERAGE: SY-01 .. SY-14, ALL TWENTY-FIVE OBLIGATIONS, IN FOUR SLICES —
 * ADMISSION (SY-01, SY-02, SY-03, SY-11), the loop's guard stack; ITERATION
 * (SY-04, SY-08, SY-10), the loop's own step; ROOTS (SY-05, SY-06, SY-07), the
 * task root's own lifecycle; and SESSIONS (SY-09, SY-12, SY-13, SY-14), the
 * session's own ending, the crash, and the two sweeps.  The `SY-` column is
 * CLOSED: zero empty alloy cells, zero declared gaps, coverage asserted, and
 * `--no-coverage` is gone from `README.md`'s run line.
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
 * TWO ABSTRACTIONS OF THIS FILE'S OWN, both declared in `README.md` and neither
 * a contract: `Proc.waits`/`Deferred` (a guard wait as an observable state) and
 * `IterA` (an iteration boundary, which is not a catalogue action because a
 * boundary is not something the loop DOES).
 *
 * THERE WERE THREE.  `Stopped` and `RefConfigInvalid` were findings about the
 * catalogue's closed sets, and `closed-set-additions-k74` disposed both: the
 * catalogue's REASON set gained `ConfigurationInvalid` and `GenerationContended`,
 * its OUTCOME set gained nothing, and `Stopped` is now `Refused(RefGenContended)`.
 * The declarations below carry the argument; the record is
 * `docs/adr/a-refusal-leaves-nothing-standing.md`.
 *
 * `Blocked` IS NOT ONE OF THEM.  It is the catalogue's own outcome, and the
 * `sessions` slice's composition decision is that it enters carrying NO
 * diagnosis: `SY-14` is stated over *a blocked tree* and `FN-25` over *which
 * block*, so only the first crosses.  `World.blocked` is `lone Flag`.
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

/* THE SIGNAL A SESSION WROTE, AND THE ABSENCE OF ONE IS THE THIRD ENDING.
   `grove-llm complete` writes a relaunch flag to `GROVE_SIGNAL_FILE`;
   `--done` writes the other; a session that crashed or was interrupted wrote
   NEITHER, and `SY-09.c` is the claim that the driver reads that difference
   rather than inferring across it.  The file, its path and its bytes are
   `one-live-driver-per-working-tree`'s and appear nowhere here: what crosses
   the boundary is the two flags and their absence.

   IT IS A STATIC ENUMERATION AND NOT A FREE SIGNATURE, deliberately.  Two
   atoms cost ~10 ms of translation per command each (the static-atom law);
   `Grove`, the file's one free `var`-referenced sig, cost a factor of five in
   the commands that quantify over it.  Nothing here quantifies over a signal —
   it is read, once, by one transition. */
abstract sig Sig {}
one sig RelaunchS, DoneS extends Sig {}

/* WHAT THE REAP CONCLUDED, AND IT IS A DIFFERENT THING FROM WHAT THE SESSION
   WROTE.  `SY-09` is *a session ends in exactly one of three ways*, and the
   whole content of `SY-09.c` is that the third is not the second: an absent
   signal STOPS the loop and is NEVER inferred as done, not even when that
   session committed a teardown.  A model with one field for both would answer
   that by construction — the inference it forbids would be unstatable — so the
   input (`World.signal`) and the conclusion (`World.ending`) are two fields and
   `doReap` is the one place they meet. */
abstract sig Ending {}
one sig RelaunchE, DoneE, NoSignalE extends Ending {}

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
  /* THE ITERATION'S ONE LIFECYCLE **TRANSITION**, SPENT OR NOT, AND IT IS A
     SECOND FLAG BECAUSE `SY-04.a`'s SUBJECT IS NOT `spent`'s.
     `lifecycle-scope-k72` defined the catalogue's *lifecycle transition* as a
     step that advances the grove's own stage, which §*Actions*' Lifecycle group
     contains NONE of — so `spent`, set by the group, could never have carried
     this obligation, and the check that read it was checking this file's
     admission machinery under an obligation's name.  Nothing shares the two:
     `open-epoch` spends the group's turn and must still leave the iteration free
     to take its transition.  Cleared only by `doIter`, like `spent`. */
  var moved:   lone Flag,
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
  var running: lone Leaf,       // the work the live session was launched on
  /* THE SESSION'S OWN SIGNAL, AS AN INPUT.  `none` is *no signal* — the third
     ending — and it is `lone` rather than a three-member field for exactly that
     reason: not writing one is what a crashed session does, and a field that
     could not be empty would delete the ending the claim is about. */
  var signal:  lone Sig,
  /* AND WHAT THE REAP CONCLUDED FROM IT.  Written by `doReap` and by nothing
     else, which is `SY-12`'s *never silently repeats a completed effect* at
     this grain: a restart does not re-read a signal an earlier reap consumed. */
  var ending:  set Ending,
  /* THE LOOP HAS STOPPED.  `SY-09.a` says relaunch CONTINUES the loop and
     `SY-09.b`/`SY-09.c` say the other two do not, so halting has to be an
     observation a mutation can get wrong.  Derived from `ending` it would be
     true by construction — the file's most-recorded failure mode — so `doReap`
     writes it, and `doLaunch` reads it. */
  var halted:  lone Flag,
  /* THE TREE IS BLOCKED, AND THAT IS THE WHOLE OF WHAT CROSSES THE BOUNDARY.
     `SY-14` is stated over a blocked tree; WHAT put it there (an interrupted
     finish transaction recovery could not settle) and WHICH of `FN-25`'s two
     diagnoses it carries are `crates/grove-finish/models/`'s and are absent
     here.  A `Blocked` with internals in this file would be a third copy of the
     finish contract, which is the one thing the node brief exists to prevent.
     It is `lone Flag` and not a new signature for the cost reason `Grove`
     taught this file: prefer an opaque observation on an existing signature. */
  var blocked: lone Flag
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
// `open-epoch`, `launch`, `reap`, `close-epoch`, `release-lease`.  ALL SEVEN
// are below, and `LifecycleAct` is the set both `SY-04` obligations quantify
// over, so an eighth would reach them without either command being edited.
// `reap` is where `SY-09`'s three endings are decided; `SignalA` is the
// SESSION's own step and is deliberately not one of the seven, because the
// Lifecycle group is the driver's.
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
        ForeignWriteA,    // the world's (EN-13): the name RE-OCCUPIED, and with
                          //   the retired identity — entry 039's own trace
        // --- the sessions slice ------------------------------------------
        SignalA,          // the session's own `grove-llm complete` — SY-09
        RecoverA,         // recover, AS FAR AS WHAT IT COULD NOT SETTLE: the
                          //   observation `SY-14` is stated over arrives here
        BlockedRefusalA   // ANY admitted action attempted on a blocked tree —
                          //   SY-14.b's subject, and one opaque step ON PURPOSE
  extends Action {}

/* THE CATALOGUE'S LIFECYCLE GROUP, ALL EIGHT, AND EXACTLY THEM.  `SY-11.a` is
   over the acquisition sites, which is a different set and deliberately so:
   `take-tree` is a GUARD acquisition and not a Lifecycle action, and `select` is
   an Observation.  A ninth Lifecycle action added by a later slice lands in this
   function and every claim over the group sees it.

   `ValidateConfigA` JOINED THE GROUP WITH `lifecycle-scope-k72`, and it did not
   join this model — it was already here, invented because `SY-04.b` requires a
   configuration validation and §*Actions* named no action that performs one.
   `lifecycle.qnt` invented the same action independently and the shipped driver
   performs it twice an iteration (`SessionConfig::load`, `src/loop_driver.rs`).
   The catalogue's table was a row short; this line is that row landing.

   **`SY-04.a` IS NO LONGER QUANTIFIED OVER THIS SET** — see `TransitionAct`. */
fun LifecycleAct: set Action {
  AcquireLeaseA + LayoutPreflightA + ValidateConfigA + OpenEpochA + LaunchA
  + ReapA + CloseEpochA + ReleaseLeaseA
}

/* THE CATALOGUE'S *LIFECYCLE TRANSITION*, WHICH IS NOT THE LIFECYCLE GROUP, AND
   THE DISTANCE BETWEEN THE TWO IS `lifecycle-scope-k72`'s SHARPEST FINDING.

   The catalogue used one word for two sets and defined neither, and the two
   families instantiated it as sets with NO MEMBER IN COMMON: this file read
   `SY-04` over the seven-member Lifecycle group and witnessed all seven of it,
   while `models/system/lifecycle.qnt` read it over the stage-changing steps,
   which the group contains none of.  BOTH WERE GREEN — the `FN-13` shape again,
   in an obligation no enumeration had flagged as underdetermined.

   The *so that* clause decided it: *so an invalid configuration leaves the
   working tree byte-identical*.  A gate in front of `close-epoch` or
   `release-lease` buys that consequence nothing, because neither writes a tree,
   and a claim whose justification reaches only part of its own quantifier is
   stated too wide.  §*Claims — system lifecycle* now defines the term: a step
   that advances the grove's own lifecycle STAGE.

   `ProveCommitA` and `SettleDeletionA` are deliberately NOT here.  They are the
   finish transaction's steps, which belong to the finish leaf's own session and
   are `crates/grove-finish/models/`'s; what an ITERATION does with a transaction
   is enter it (`AllocFinishA`) and recover an interrupted one (`RecoverA`). */
fun TransitionAct: set Action {
  InitRootA + CompleteScaffoldA + AllocFinishA + RecoverA
}

/* WHERE A TRANSITION IS COUNTED, which is not the same as which actions are
   transitions, and the difference is this file's declared split rather than a
   narrowing of the claim.  `doInitRoot` stops at the format witness and
   `doCompleteScaffold` lands it — ONE catalogue action, TWO steps, deliberately,
   because the interval between them is `PartialScaffold` and `SY-06.b` and
   `TT-20` are both about it.  Counting both would report a single uninterrupted
   scaffold as two transitions in one iteration; counting the completion counts
   the pair once, and counts a completion that finishes an EARLIER iteration's
   initialisation as that iteration's own — which is the interrupted case
   `SY-06.b` exists for.  All four guard on `moved`; these three spend it. */
fun TransitionEndAct: set Action {
  CompleteScaffoldA + AllocFinishA + RecoverA
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
  + RecoverA + BlockedRefusalA
}

/* THE ADMITTED SET, AND WHAT IT EXCLUDES.  §*Actions* puts `crash`, `hand-edit`,
   `foreign-write`, `topology-change` and `confirm` in an Environment group whose
   guard column reads *none — these are the world's*.  `SY-13` and `SY-14` are
   both quantified over the ADMITTED actions and the exclusion is load-bearing in
   both: a sweep that counted a hand edit as an exit would find no sink anywhere,
   and `SY-14`'s *until an operator acts* would have nothing to mean.

   `IterA` IS NOT IN HERE AND IS NOT IN THE ENVIRONMENT SET EITHER.  It is this
   file's own boundary abstraction — not something the loop DOES — so it is
   neither admitted nor the world's, and `SY-13`'s sequence lengths are reported
   both with and without it for that reason. */
fun AdmittedAct: set Action {
  AcquireLeaseA + LayoutPreflightA + OpenEpochA + LaunchA + ReapA
  + CloseEpochA + ReleaseLeaseA
  + TakeTreeA + DropTreeA + GrantA + ValidateConfigA + TimeoutA + SelectA
  + SignalA
  + TreeAct
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
abstract sig Refused extends Result {}
/* `RefEpochStale` is the catalogue's `EpochStale`.
   `RefConfigInvalid` and `RefGenContended` WERE THIS FILE'S ABSTRACTIONS AND ARE
   NOW THE CATALOGUE'S OWN MEMBERS, `ConfigurationInvalid` and
   `GenerationContended`.  Disposed by `closed-set-additions-k74`; see
   `docs/adr/a-refusal-leaves-nothing-standing.md`, clause 2.

   `RefGenContended` REPLACES A `Stopped` THAT WAS DECLARED AS A SEVENTH
   OUTCOME, AND THE CORRECTION IS WORTH THE SPACE.  This file argued that
   `SY-10.b`'s visible stop is neither a `Refused` (no reason covered a handoff
   timeout) nor a `Blocked` (§*Outcomes* scopes blocks to a transaction stopped
   part-way), and therefore that the closed OUTCOME set was short.  The second
   half of that argument holds and the first was circular: it rested on the
   REASON set being closed against the case, which is a fact about the reason
   set.  The catalogue widened the reason set and left the six outcomes alone,
   which is much the narrower blast radius — `SY-14`'s exhaustive sweep runs
   through the same classifier the real actions use.  `models/system/lifecycle.qnt`
   placed it as a refusal independently and is the column that had it right.

   ONE WORD MADE THE WIDER READING LOOK NECESSARY.
   `one-live-driver-per-working-tree` says the driver "stops `blocked`" on a
   post-reap invalidation timeout, and that `blocked` is NOT the catalogue's
   `Blocked(b)` — it is the epoch invalidation being blocked.  The shipped path
   returns an error, the loop stops, and the ADR's own next sentence is that a
   timeout performs no tree access or epoch rewrite.  Nothing stands, so it is a
   refusal by §*Outcomes*' discriminator.  A word collision cost a proposed
   widening of the most load-bearing closed set in the catalogue. */
one sig RefLeaseHeld, RefLayoutUnsupported,
        RefEpochStale, RefConfigInvalid, RefGenContended extends Refused {}
/* Both are the catalogue's own, from the closed refusal-reason set.
   `RefFormatLegacy` is `FormatLegacy`: a tree with no format witness that is not
   the known subset is refused rather than completed, which is `SY-06.b`'s whole
   content.  `RefReservedKind` is `ReservedKind`: the finish leaf's kind is
   Grove's to allocate and a session's to refuse, which is `SY-07.b`'s. */
one sig RefFormatLegacy, RefReservedKind extends Refused {}
/* `Blocked` IS THE CATALOGUE'S OWN OUTCOME AND NOT AN ABSTRACTION OF THIS
   FILE'S.  §*Outcomes* lists it beside `Applied` and `Refused` — *a transaction
   stopped part-way that left a stable, recoverable state* — and its two
   neighbour `Deferred` is the one this file DID have to invent.
   It enters here as a `Result` member carrying no diagnosis: `FN-25`'s
   `RecoveryPending`/`OwnershipConflict` partition is the finish model's, and
   `SY-14` reads only *the tree is blocked*. */
one sig Blocked extends Result {}

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
                  and World.partial' = World.partial and World.legacy' = World.legacy
                  /* THE BLOCK IS A ROOT OBSERVATION, so it frames with the root
                     rather than beside it: this slice adds one field and
                     twenty-six transitions must leave it alone, which is a
                     silent frame hole per transition if written out. */
                  and World.blocked' = World.blocked }
pred entriesSame{ World.live' = World.live and World.fin' = World.fin }
pred treeSame   { rootSame and entriesSame }
/* THE WHOLE LAUNCH RECORD, and this slice widens it from two fields to five.
   The session's signal, the ending the reap concluded and the loop's halt are
   the same record the generation and the running work are — written by the
   three transitions that own them and framed by every other. */
pred launchSame { World.gen' = World.gen and World.running' = World.running
                  and World.signal' = World.signal and World.ending' = World.ending
                  and World.halted' = World.halted }
/* What a transition that writes ONE of the launch record's five fields frames of
   the other four — written once, for the reason `actorSameBut` is written once. */
pred sessionSame{ World.signal' = World.signal and World.ending' = World.ending
                  and World.halted' = World.halted }
pred verdictSame{ World.verdict' = World.verdict }
pred aliveSame  { Alive.live' = Alive.live }
pred procSame[p: Proc] { procSameButMoved[p] and p.moved' = p.moved }

/* The same frame with `moved` left to the site, for the three transitions that
   spend it.  Split rather than inlined so that the four fields a transition does
   NOT touch stay framed by one name — a field added to `Proc` and forgotten at a
   transition is the silent frame hole this file's header warns about, and
   `SY_04a`'s second conjunct is what catches one. */
pred procSameButMoved[p: Proc] {
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
  p.sel' = p.sel and p.gen' = p.gen and p.moved' = p.moved
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
    p.sel' = p.sel and p.gen' = p.gen and p.moved' = p.moved
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
  p.moved' = p.moved
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
  World.running' = World.running and sessionSame
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
      p.sel' = p.sel and p.gen' = p.gen and p.moved' = p.moved
      /* THE ROTATION.  A driver writes the record active with a FRESH identity;
         an ambient command only reads it. */
      p.role = DriverR implies (some World.gen' and World.gen' != World.gen)
                        else   World.gen' = World.gen
    } else {
      Sys.res' = Deferred
      p.holds' = p.holds and p.seen' = p.seen
      p.waits' = EpochG and p.leaseOn' = p.leaseOn
      p.sel' = p.sel and p.gen' = p.gen and p.moved' = p.moved
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
  p.moved' = p.moved
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
  p.moved' = p.moved
  procsSameBut[p]
}

pred doDropTree[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  Sys.act' = DropTreeA and Sys.actor' = p and Sys.gu' = TreeG
  Sys.res' = Applied
  p.holds' = p.holds - TreeG and p.seen' = p.seen
  p.waits' = p.waits and p.leaseOn' = p.leaseOn
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  p.moved' = p.moved
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
  p.moved' = p.moved
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
  no World.blocked                        // SY-14.b — see `mayTouchTree`
  /* AND THE ROOT IS CLASSIFIED (`FormatLegacy`, the closed refusal set).  THIS
     CONJUNCT IS A DEFECT REPAIR AND `SY-13`'s SWEEP IS WHAT FOUND IT.  The
     opaque step was written by the `admission` slice, before a classification
     existed; `roots` taught `doInitRoot`, `doAllocFinish` and `doProveCommit`
     about `partial` and `legacy` and did not come back here.  A tree with no
     format witness is not operable — the only thing that may touch one is
     `complete-scaffold`, and it REFUSES the half of them that is `Legacy` —
     but this step would happily append a live leaf to one, which made a
     `Legacy` root look like an ordinary tree with work in it.

     NO INHERITED OBLIGATION COULD SEE IT.  None of the seventeen reads a tree
     operation against a classification; `SY-06.b` owns the legacy refusal and
     owns it at `complete-scaffold`.  What walked into it is `SY-13`'s sweep,
     which asks of every stable state what leaves it.

     A GUARD AND NOT A BRANCH, deliberately.  A refusal branch would be a
     second statement of `SY-06.b` in a step no obligation reads, where the
     guard says only that the step is not available — which is what *stops
     every read and mutation* means for a whole-tree classification. */
  no World.partial and no World.legacy
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
  p.moved' = p.moved
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
  p.moved' = p.moved
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
  World.blocked' = World.blocked
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
  no p.spent' and no p.moved' and no p.sel' and p.seen' = p.holds
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
  p.moved' = p.moved
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
  /* A HALTED LOOP LAUNCHES NOTHING, and this is the operand `SY-09.b`'s and
     `SY-09.c`'s *the loop ends* / *the loop stops* actually have.  Without it
     both endings would be a record nobody reads, and the two obligations would
     be checks about a field rather than about the loop. */
  no World.halted
  Sys.act' = LaunchA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  World.running' = p.sel and World.gen' = World.gen
  /* THE NEW SESSION HAS NOT SIGNALLED YET.  A launch clears the slot rather
     than inheriting it, which is the other half of *never silently repeats a
     completed effect*: a signal one session wrote is never read for the next. */
  no World.signal'
  World.ending' = World.ending and World.halted' = World.halted
  actorSameBut[p] and some p.spent'
  /* The child binds to the live record.  WHICH process becomes the child is
     machinery no `SY-` obligation reads; what matters is that it carries the
     identity the record holds now, so the next rotation makes it stale. */
  some s: Proc - p {
    s.role = SessionR and s in Alive.live
    s.gen' = World.gen
    s.holds' = s.holds and s.seen' = s.seen and s.waits' = s.waits
    s.leaseOn' = s.leaseOn and s.spent' = s.spent and s.sel' = s.sel
    s.moved' = s.moved
    all q: Proc - p - s | procSame[q]
  }
  worldSame and treeSame and verdictSame and aliveSame
}

/* REAP — AND IT IS WHERE THE SESSION ENDS, WHICH IS `SY-09`'S WHOLE SUBJECT.
   The child is gone; the record is NOT written inactive here, because this file
   models only the rotation write — see `World.gen`.

   THREE ENDINGS, AND THE BRANCH IS TOTAL ON `World.signal`.  A relaunch flag,
   a done flag, or NOTHING — and the third is not a missing case but the case
   `SY-09.c` is about.  The reap READS the slot and CONCLUDES; it never infers
   across an absence, which is the sentence *no signal ... SHALL never be
   inferred as done — not even when that session committed a teardown*.  Note
   what the third branch does NOT consult: `World.retired`, `World.proven` and
   `World.rooted` appear nowhere in it, and the mutation that puts one there is
   exactly the implementation the obligation forbids.

   THE SIGNAL IS CONSUMED.  `no World.signal'` in every branch is `SY-12`'s
   *never silently repeats a completed effect* at this grain: a restart after a
   crash cannot re-read a signal an earlier reap already acted on. */
pred doReap[p: Proc] {
  p in Alive.live and p.role = DriverR and some p.leaseOn and no p.waits
  validated and fresh[p]
  some World.running
  Sys.act' = ReapA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  no World.running' and World.gen' = World.gen
  no World.signal'                        // consumed, in every branch
  World.signal = RelaunchS implies {
    World.ending' = RelaunchE
    no World.halted'                      // SY-09.a — the loop CONTINUES
  } else World.signal = DoneS implies {
    World.ending' = DoneE
    some World.halted'                    // SY-09.b — the loop ENDS
  } else {
    World.ending' = NoSignalE             // SY-09.c — and never DoneE
    some World.halted'                    // the loop STOPS
  }
  actorSameBut[p] and some p.spent'
  procsSameBut[p] and worldSame and treeSame and verdictSame and aliveSame
}

/* THE SESSION'S OWN LAST ACT — `grove-llm complete`, and the only writer of
   `World.signal`.  It is the running session's and nobody else's: a driver does
   not write the slot it reads, and a session that is not the launched one has
   nothing to report.

   IT IS NOT A LIFECYCLE ACTION and does not spend the iteration's one
   transition — the seven-member group is the DRIVER's, and this is the child's
   step.  It takes no guard: the signal file is outside the task tree and
   outside every lock, which is `one-build-owns-a-session`'s own arrangement and
   the reason a crashed session simply leaves it unwritten. */
pred doSignal[p: Proc] {
  p in Alive.live and p.role = SessionR
  some World.running and some World.gen and p.gen = World.gen
  no World.signal                         // one session writes it once
  Sys.act' = SignalA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Applied
  some World.signal'
  World.gen' = World.gen and World.running' = World.running
  World.ending' = World.ending and World.halted' = World.halted
  procsSame and worldSame and treeSame and verdictSame and aliveSame
}

/* THE TIMEOUT — `SY-10.b`'s visible stop.

   IT IS NON-DETERMINISTICALLY ENABLED AND CARRIES NO CLOCK, which is the
   catalogue's own instruction: §*Deliberate omissions* models clocks, timeouts
   and retry counts as non-determinism, on the grounds that a bounded handoff
   wait is a liveness property of the implementation and not of the protocol.
   SO THIS FILE NEVER SAYS THE TIMEOUT *WILL* FIRE.  What it says is that when a
   wait ends, it ends visibly — `Refused(RefGenContended)` is a result the caller sees — and that
   the stop performs no tree access and no epoch rewrite, which is the ADR's own
   sentence.  A reader who takes `SY_10b` for a liveness property has read
   fairness into a file that assumes none.

   ONLY A GENERATION WAIT TIMES OUT.  The tree access lock BLOCKS — §*Outcomes*
   says so, and no invocation returns while it is held — so a `TreeG` wait has no
   stop, and `SY-10.b`'s subject is *a contended generation* and nothing else. */
pred doTimeout[p: Proc] {
  p in Alive.live and p.waits = EpochG
  Sys.act' = TimeoutA and Sys.actor' = p and Sys.gu' = EpochG
  Sys.res' = RefGenContended
  no p.waits'
  p.holds' = p.holds and p.seen' = p.seen and p.leaseOn' = p.leaseOn
  p.spent' = p.spent and p.sel' = p.sel and p.gen' = p.gen
  p.moved' = p.moved
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
  /* AND THE TREE IS NOT BLOCKED (`SY-14.b`).  Every admitted action on a
     blocked tree refuses naming the block, so the whole Grove-side surface is
     gated here rather than branched in each transition: what a blocked tree
     admits is `doBlockedRefusal` and nothing else.  The mutation that removes
     this one conjunct is `SY-14.b`'s, and it is a single edit precisely because
     the claim is about EVERY action rather than about which. */
  no World.blocked
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
  /* `SY-04.a`: this iteration has not taken its transition.  `doInitRoot`
     GUARDS on the flag but does not spend it — root initialisation is one
     catalogue action modelled here as two steps, and the transition is
     counted where the format witness lands. */
  no p.moved
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
  World.blocked' = World.blocked
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
  no p.moved
  some World.rooted
  some (World.partial + World.legacy)     // ENABLED BY: no format witness
  Sys.act' = CompleteScaffoldA and Sys.actor' = p and no Sys.gu'
  World.rooted' = World.rooted and World.extant' = World.extant
  World.retired' = World.retired and World.proven' = World.proven
  World.fin' = World.fin and World.blocked' = World.blocked
  some World.partial implies {            // DECIDED BY: the exact known subset
    Sys.res' = Applied
    no World.partial' and no World.legacy'
    some f: Leaf | f not in World.live and World.live' = World.live + f
  } else {
    Sys.res' = RefFormatLegacy            // and the tree is byte-identical
    World.partial' = World.partial and World.legacy' = World.legacy
    World.live' = World.live
  }
  procsSameBut[p] and procSameButMoved[p]
  /* SPENT ONLY WHEN THE TRANSITION APPLIED.  A refused attempt transitions
     nothing and must leave the iteration free to take its one — which is also
     what keeps `SY_04a`'s frame-hole conjunct honest, since that conjunct says
     `moved` changes only where a COUNTED transition happens. */
  (Sys.res' = Applied implies some p.moved' else p.moved' = p.moved)
  worldSame and verdictSame and launchSame and aliveSame
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
  no p.moved
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
  procsSameBut[p] and procSameButMoved[p]
  /* SPENT ONLY WHEN THE TRANSITION APPLIED.  A refused attempt transitions
     nothing and must leave the iteration free to take its one — which is also
     what keeps `SY_04a`'s frame-hole conjunct honest, since that conjunct says
     `moved` changes only where a COUNTED transition happens. */
  (Sys.res' = Applied implies some p.moved' else p.moved' = p.moved)
  worldSame and verdictSame and launchSame and aliveSame
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
  World.blocked' = World.blocked
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
  World.blocked' = World.blocked
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
  World.blocked' = World.blocked
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
  World.blocked' = World.blocked
  World.retired' = World.retired        // PRESERVED — see above
  World.proven' = World.proven
  procsSame and worldSame and verdictSame and launchSame and aliveSame
}

/* RECOVER, AS FAR AS WHAT IT COULD NOT SETTLE — and stopping there is the
   composition boundary rather than an economy.  `recover` is a Finish action
   and the whole of its machinery is `crates/grove-finish/models/`'s: the
   published witness, the manifest, the recorded and observed topology, the two
   restorable exits, and `FN-25`'s partition of the block into
   `RecoveryPending` and `OwnershipConflict`.  NONE of that is here.

   What crosses the boundary is one observation and one outcome: **a
   transaction stopped part-way** — at this scope, a proof standing over a root
   the settle has not yet freed — **and recovery could not settle it**, so the
   tree is `Blocked`.  Which diagnosis it carries is not read by any `SY-`
   obligation, so it is not imported; `SY-14` is stated over *a blocked tree*
   and `FN-25` over *which block*.

   IT IS NON-DETERMINISTICALLY ENABLED, exactly as `doTimeout` is.  Recovery
   that CAN settle is `doSettleDeletion` — the ordinary continuation `SY-12` is
   about — and this is the branch that cannot.  Nothing here says which branch a
   given interruption takes, because that is `FN-20`'s classification and it is
   the finish model's. */
pred doRecover[p: Proc] {
  mayTouchTree[p]
  no p.moved
  some World.rooted and World.proven = World.rooted   // a transaction, part-way
  Sys.act' = RecoverA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Blocked
  some World.blocked'
  World.rooted' = World.rooted and World.extant' = World.extant
  World.retired' = World.retired and World.proven' = World.proven
  World.partial' = World.partial and World.legacy' = World.legacy
  entriesSame
  procsSameBut[p] and procSameButMoved[p]
  /* SPENT ONLY WHEN THE TRANSITION APPLIED.  A refused attempt transitions
     nothing and must leave the iteration free to take its one — which is also
     what keeps `SY_04a`'s frame-hole conjunct honest, since that conjunct says
     `moved` changes only where a COUNTED transition happens. */
  (Sys.res' = Applied implies some p.moved' else p.moved' = p.moved)
  worldSame and verdictSame and launchSame and aliveSame
}

/* ANY ADMITTED ACTION ATTEMPTED ON A BLOCKED TREE, AS ONE OPAQUE STEP — and
   ONE step on purpose, which is the opposite of the rule `doAllocFinish` was
   split out under.  That rule is *prefer splitting out a named transition
   wherever a claim is about WHICH mutation*; `SY-14` is the case it does not
   cover, because the claim is about EVERY action and a per-action branch would
   be twenty-six copies of one sentence with twenty-six chances to omit it.

   `SY-14.a` IS THE FRAME, NOT A REMARK.  Nothing changes: the block persists,
   the root persists, the entries persist.  A blocked tree stays blocked, and
   the only thing that could clear it is an operator action — which is outside
   the admitted set by construction, so it is outside this file. */
pred doBlockedRefusal[p: Proc] {
  p in Alive.live and TreeG in p.holds and no p.waits
  p.role = SessionR implies EpochG in p.holds
  some World.blocked
  Sys.act' = BlockedRefusalA and Sys.actor' = p and no Sys.gu'
  Sys.res' = Blocked
  procsSame and worldSame and treeSame and verdictSame and launchSame and aliveSame
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
                     or doSettleDeletion[p] or doObserveRoot[p]
                     or doSignal[p] or doRecover[p] or doBlockedRefusal[p])
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
// TWO OF THE THREE CLAIMS ARE STATED OVER A SET RATHER THAN OVER A LIST OF
// NAMES, for the reason `SY-11.a` already gave: a list goes stale the moment a
// slice adds a site.  Both `SY-04` obligations quantify over `TransitionAct`
// since `lifecycle-scope-k72` defined the catalogue's *lifecycle transition* —
// which is NOT §*Actions*' Lifecycle group, and the two sets have no member in
// common.  This column read the group and the Quint column read the stages, both
// were green, and the word was doing two jobs; the definition is at the head of
// §*Claims — system lifecycle* and `TransitionAct` is it.

/* SY-04.a, THE CATALOGUE'S CLAIM: at most one lifecycle transition per
   iteration, stated over the trace rather than over this file's `spent` flag.
   After a transition, no second one occurs up to and including the next
   boundary — `releases` rather than `until`, because a lasso trace that takes
   its transition and then idles forever satisfies *at most one* and a strong
   until would report it as a counterexample about nothing. */
// ===========================================================================

// --- SY-04: one lifecycle transition an iteration, under a live config ------

/* WHAT COUNTS AS ONE, AND THE SECOND CONJUNCT IS THIS FILE'S OWN ABSTRACTION
   RATHER THAN A WEAKENING.

   APPLIED ONLY.  A refused attempt transitions nothing, and `SY-04`'s own *so
   that* is about what gets written; `models/system/lifecycle.qnt` counts the
   same way (`afterTree` increments on `OApplied`).  The predecessor here counted
   every branch — *a refusal is a turn of the loop as surely as an application
   is* — which is true of a TURN and not of a TRANSITION, and it is one of the
   two places the word was doing two jobs.

   AND ROOT INITIALISATION IS ONE CATALOGUE ACTION MODELLED AS TWO STEPS.
   `doInitRoot` stops at the format witness and `doCompleteScaffold` lands it,
   deliberately, because the interval between them is `PartialScaffold` and two
   obligations are about it (`SY-06.b`, `TT-20`).  So the completion that
   finishes THIS iteration's own initialisation is not a second transition — the
   pair is one — while a completion that finishes an initialisation from an
   earlier iteration is its own iteration's transition, which is the interrupted
   case `SY-06.b` exists for.  `since` is what tells the two apart, and stating
   the claim without it reported the model's declared split as a violation. */
pred tookATransition { Sys.act' in TransitionEndAct and Sys.res' = Applied }

check SY_04a_at_most_one_lifecycle_transition_per_iteration {
  Assumed implies always {
    /* THE PROHIBITION — a transition happens only in an iteration that has not
       taken one.  Drop `no p.moved` from any of the four sites and this fires. */
    tookATransition implies no Sys.actor'.moved
    /* THE CONSUMPTION — and it takes it.  A site that transitioned without
       marking the iteration would leave the prohibition true and the claim
       false, and only this half sees it. */
    tookATransition implies some Sys.actor'.moved'
    /* AND NOTHING ELSE MOVES THE FLAG, which is the frame-hole detector rather
       than a third claim: `moved` is a `var` on `Proc`, and a predicate that
       forgot to frame it would leave it free and quietly weaken the two
       conjuncts above into statements about an unconstrained field.  Only a
       counted transition sets it and only the iteration boundary clears it. */
    (some p: Proc | p.moved' != p.moved)
      implies (tookATransition or Sys.act' = IterA)
  }
} for 3 but 2 WtId, 5 steps

/* THE GROUP'S OWN TURN-TAKING IS THIS MODEL'S MECHANISM AND IS NO LONGER A
   COMMAND, WHICH IS PART OF `lifecycle-scope-k72`'s DISPOSITION RATHER THAN
   TIDYING.

   `spent` is set by every Lifecycle-GROUP action in every branch and cleared
   only by `doIter`, and until this leaf the pair *a group action needs `fresh`
   and sets `spent`* was checked here under `SY-04.a`'s name.  It is a true
   statement about this file's admission machinery and it is not the catalogue's
   claim — it is that machinery read back to itself, which is the transcription
   shape `obligations-follow-context-not-artifact` records at `TT-24.c`.  Reading
   it as `SY-04.a` is how this column came to offer *layout-preflight alone in an
   iteration* as evidence for an obligation whose own justification is about the
   working tree, and to gate `release-lease` on a configuration.

   The machinery is untouched — every `fresh[p]` guard and every `some p.spent'`
   still stands, and they are what give `IterA` a boundary to be.  What is gone
   is the command that credited a coverage cell for checking them.  `SY-04.a` is
   now stated over the trace and over `TransitionAct`, where a mutation that
   removed a `fresh` guard would still be visible if it let two transitions
   land in one iteration — and where one that did not is correctly no longer
   this obligation's business.

   `ValidateConfigA` is also why the old command could not simply keep its set:
   it joined `LifecycleAct` with this disposition and it does NOT spend the
   iteration's turn, correctly — the driver validates and then transitions in
   the same iteration.  Restating the mechanism over `LifecycleAct -
   ValidateConfigA` would have reintroduced the ad-hoc exemption this leaf just
   removed from `SY_04b`. */

/* The catalogue's own witness is *each transition, taken alone* — a witness PER
   LIFECYCLE TRANSITION and not one witness, so the seven below are one
   instrument applied seven times.  ONE HELPER RATHER THAN SEVEN COPIES, because
   the thing being witnessed is identical in each and a reader should be able to
   see that at a glance: an iteration boundary, one Lifecycle transition, the
   next boundary, all three the same process's. */
pred aloneInAnIteration[a: Action] {
  some p: Proc |
    eventually (Sys.act = IterA and Sys.actor = p and after (
      (Sys.act not in TransitionAct) until (
        Sys.act = a and after (
          (Sys.act not in TransitionAct) until (Sys.act = IterA and Sys.actor = p)))))
}

run witness_SY_04a_initialise_root_alone     { Assumed and aloneInAnIteration[InitRootA]         } for 3 but 2 WtId, 6 steps
run witness_SY_04a_complete_scaffold_alone   { Assumed and aloneInAnIteration[CompleteScaffoldA] } for 3 but 2 WtId, 6 steps
run witness_SY_04a_allocate_finish_leaf_alone{ Assumed and aloneInAnIteration[AllocFinishA]      } for 3 but 2 WtId, 6 steps
run witness_SY_04a_recover_alone             { Assumed and aloneInAnIteration[RecoverA]          } for 3 but 2 WtId, 7 steps

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
    /* OVER `TransitionAct`, NOT OVER THE GROUP, and the change is
       `lifecycle-scope-k72`'s disposition of item 16.  Stated over
       `LifecycleAct - AcquireLeaseA` this conjunct gated `release-lease`, which
       touches no tree and launches nothing — so an invalid personal
       configuration stranded a lease the loop could then escape only by dying,
       and this file recorded that as a second dead end with two repairs on
       offer (exempt the release, or admit process death).  NEITHER IS OWED.
       `release-lease` is not a lifecycle transition, the dead end dissolves with
       the reading, and admitting process death would have moved the quantifier
       of every reachability claim in the catalogue — `crash` is the world's.

       `AcquireLeaseA` no longer needs an exemption either: it was never in this
       set.  `SY-02`'s *before configuration validation* is now a fact about two
       actions in a stated order rather than a hole in this conjunct. */
    (Sys.act' in TransitionAct)
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

     THE STOP IS VISIBLE AND INERT.  `Refused(RefGenContended)` is a result the caller sees, and
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
      implies (Sys.res' = RefGenContended
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
                                      and Sys.res = RefGenContended))
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
// CLAIMS — SY-09, SY-12, SY-13, SY-14   (the `sessions` slice)
//
// THE SESSION'S OWN ENDING, THE CRASH, AND THE TWO SWEEPS — and with them the
// `SY-` column closes.
//
// THE EIGHT BODIES ARE NAMED PREDICATES AND THE OTHER SEVENTEEN OBLIGATIONS'
// ARE NOT, which is a departure from the file's style and is here for one
// reason: `EN-08`'s control owes a demonstration that **every property stays
// green with `crash` removed**, and a body written inside a `check` cannot be
// negated by a second command.  The finish scope discharged its half of `EN-08`
// with the superset argument — no check asserts `EN_08`, so each is already
// checked over the traces with `crash` and the traces without — and that
// argument is sound and is repeated in `README.md`.  It is also unfalsifiable
// as stated, and this slice owes the `SY-12` half rather than a second recital
// of the finish scope's.  Named bodies make it a command.
// ===========================================================================

// --- the slice's shared instruments ----------------------------------------

/* A GOAL, IN `SY-13`'s OWN WORDS: *a live leaf to run, or a terminal
   disposition*.  There are exactly TWO of the latter and the catalogue is
   emphatic about both — a proven successful finish (`FN-28`) and a blocked tree
   (`FN-25`/`FN-26`) — and equally emphatic that a `Malformed(reason)` tree is
   NOT one, because folding it in "would let the claim be satisfied by a tree
   nobody can act on".

   ABSENCE IS READ HERE AS `retired` AND NOT AS `no World.rooted`, for the
   reason the `roots` slice's whole section gives: after the rename the name is
   free and the world may re-occupy it, so a finish that succeeded is a fact
   Grove established rather than a shape the directory has. */
pred atGoal {
  some World.live                                  // a live leaf to run
  or (no World.rooted and some World.retired)      // a proven successful finish
  or some World.blocked                            // a blocked tree
}

/* A CRASH AT ONE LIFECYCLE POINT — `SY-12`'s witness spelled once and applied
   seven times, exactly as `aloneInAnIteration` is for `SY-04.a`.  The step
   COMPLETES and the process that took it then dies, which is the situation
   *restart is ordinary continuation* is about: the effect landed, and the
   next invocation must neither miss it nor repeat it.

   IT NAMES `CrashA` RATHER THAN POSITING THE STATE ONE LEAVES, deliberately.
   `finish-k8` found that two of its `FN-31.c` witnesses POSIT the disk an
   interruption leaves, so removing `crash` left them landing and `EN-08`'s
   control could say nothing about them.  All seven below REACH their state, so
   the control below is a statement about this file rather than an assertion. */
pred crashAfter[a: Action] {
  some p: Proc |
    eventually (Sys.act = a and Sys.actor = p and Sys.res = Applied
                and after (Sys.act = CrashA and Sys.actor = p))
}

// --- SY-09: a session ends in exactly one of three ways ---------------------

/* SY-09.a — RELAUNCH: the loop continues with the next iteration.  Two
   conjuncts, and the second is the one that is not about a field.

   `one World.ending'` IS WHY `World.ending` IS A `set`.  *Exactly one of three*
   is the claim, and a `lone Ending` field would have said *at most one* by
   construction — which is the `World.fin` lesson from the `roots` slice applied
   to a second field, and the file's most-recorded failure mode.  Declared
   `set`, the count is something this check ESTABLISHES and something a mutation
   can break.

   THE HALT IS A SEPARATE FIELD AND NOT A FUNCTION OF THE ENDING, for the same
   reason.  Derived, *the loop continues* would be true by construction; written
   as its own observation that `doReap` sets and `doLaunch` reads, it is a claim
   with a mutation behind it. */
pred SY09a {
  always {
    (Sys.act' = ReapA and World.signal = RelaunchS)
      implies (World.ending' = RelaunchE and no World.halted')
    (Sys.act' = ReapA) implies one World.ending'
  }
}
check SY_09a_a_relaunch_continues_the_loop_and_the_ending_is_one_of_three {
  Assumed implies SY09a
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness — *reached* — and it is pinned to the SIGNAL
   BEING WRITTEN rather than merely being present, because a free initial state
   can hand out a relaunch flag nobody wrote and the witness would then show a
   reap reading the world's own scratch rather than a session's report. */
run witness_SY_09a_relaunch_reached {
  Assumed
  eventually (Sys.act = SignalA and World.signal = RelaunchS)
  eventually (Sys.act = ReapA and World.ending = RelaunchE and no World.halted)
} for 3 but 2 WtId, 5 steps

/* SY-09.b — DONE: the loop ends.  The second conjunct is what makes *ends* mean
   something: a halted loop launches nothing.  It is SHARED WITH `SY-09.c` and
   the sharing is recorded rather than hidden — both endings halt, and what
   separates them is the record, which is `SY-09.c`'s own second conjunct. */
pred SY09b {
  always {
    (Sys.act' = ReapA and World.signal = DoneS)
      implies (World.ending' = DoneE and some World.halted')
    (some World.halted) implies Sys.act' != LaunchA
  }
}
check SY_09b_a_done_signal_ends_the_loop {
  Assumed implies SY09b
} for 3 but 2 WtId, 5 steps

run witness_SY_09b_done_reached {
  Assumed
  eventually (Sys.act = SignalA and World.signal = DoneS)
  eventually (Sys.act = ReapA and World.ending = DoneE and some World.halted)
} for 3 but 2 WtId, 5 steps

/* SY-09.c — NO SIGNAL: the loop stops, and is NEVER INFERRED AS DONE — not even
   when that session committed a teardown.

   THE SECOND CONJUNCT IS THE OBLIGATION AND THE FIRST IS ONLY ITS BRANCH.
   Written as the branch alone the check would say *the third case assigns the
   third value*, which is a restatement of `doReap` and is broken by nothing a
   real implementation would do.  What a real implementation does is INFER: it
   sees `.grove/` gone, or a commit whose message names the finish leaf, and
   concludes the loop is done although no flag was written.  Stated the second
   way — no reap concludes `done` from anything but the flag — that
   implementation is a counterexample, and it is the one the mutation writes.

   A COMMITTED TEARDOWN IS NOT THE FLAG, and the model has the teardown: the
   witness below settles a deletion and never signals.  `CONTEXT.md`'s *Complete
   finish cycle* carries the same sentence as an `_Avoid_`, and the ADR pair
   names the no-signal stop as the loop's ordinary failure exit. */
pred SY09c {
  always {
    (Sys.act' = ReapA and no World.signal)
      implies (World.ending' = NoSignalE and some World.halted')
    (Sys.act' = ReapA and DoneE in World.ending') implies World.signal = DoneS
  }
}
check SY_09c_no_signal_stops_the_loop_and_is_never_inferred_as_done {
  Assumed implies SY09c
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: REACHED, WITH A PROVEN TEARDOWN.  `always
   Sys.act != SignalA` is *no signal* stated over the whole trace rather than
   over one state — the session never wrote one, which is what a crash or a
   Ctrl-C is — and the settled deletion is the teardown the obligation names.
   The reap concludes `NoSignalE`, the loop stops, and nothing in the trace read
   the teardown as evidence. */
run witness_SY_09c_no_signal_after_a_proven_teardown {
  Assumed
  always Sys.act != SignalA
  eventually (Sys.act = SettleDeletionA and some World.retired)
  eventually (Sys.act = ReapA and World.ending = NoSignalE and some World.halted)
} for 3 but 2 WtId, 5 steps

// --- SY-12: restart is ordinary continuation --------------------------------

/* SY-12.  Three groups of conjuncts, and the third is the one no other
   obligation states.

   1. THE CRASH IS THE WORLD'S AND APPLIES NOTHING OF GROVE'S.  It is not a
      rollback, not a cleanup path and not a step: the state it leaves is the
      state the crashed process had already produced, and the process is gone
      with its guards.  That is what makes the successor's path ORDINARY —
      there is no recovery transition here, and `witness_SY_06b_an_interrupted_
      scaffold_completed_by_a_successor` is the same fact seen from the tree.
   2. NO COMPLETED EFFECT IS APPLIED TWICE.  The three one-shot effects this
      scope can see are the mint, the proof and the settle, and each is guarded
      on the WORLD rather than on the process that started it — which is why a
      successor re-running one finds nothing to do instead of doing it again.
      **These three conjuncts are neighbours of `SY-05`'s and are not
      isolating**; they are here because *repeats no completed effect* is what
      the obligation says, and the neighbour list is in `README.md`.
   3. AND NO SIGNAL IS READ TWICE.  This one IS `SY-12`'s alone: the ending is
      written by the reap and by nothing else, and the reap consumes the slot.
      A restart that re-read a signal an earlier reap had acted on would repeat
      a completed effect exactly, and no other obligation forbids it. */
pred SY12 {
  always {
    (Sys.act' = CrashA)
      implies (worldSame and treeSame and verdictSame and launchSame
               and Sys.actor' not in Alive.live')
    (Sys.act' = ProveCommitA)    implies no World.proven
    (Sys.act' = SettleDeletionA) implies World.proven = World.rooted
    (Sys.act' = InitRootA)       implies no World.rooted
    (World.ending' != World.ending) implies Sys.act' = ReapA
    (Sys.act' = ReapA) implies no World.signal'
  }
}
check SY_12_a_crash_applies_nothing_and_no_completed_effect_is_repeated {
  Assumed implies SY12
} for 3 but 2 WtId, 5 steps

/* The catalogue's own witness: ONE CRASH POINT PER LIFECYCLE STEP.  Seven, and
   the set is `LifecycleAct` rather than a list, so a slice adding an eighth
   Lifecycle action reaches this sweep by adding one line beside the others
   rather than by being forgotten. */
run witness_SY_12_crash_after_acquire_lease    { Assumed and crashAfter[AcquireLeaseA]    } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_layout_preflight { Assumed and crashAfter[LayoutPreflightA] } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_validate_config  { Assumed and crashAfter[ValidateConfigA]  } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_open_epoch       { Assumed and crashAfter[OpenEpochA]       } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_launch           { Assumed and crashAfter[LaunchA]          } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_reap             { Assumed and crashAfter[ReapA]            } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_close_epoch      { Assumed and crashAfter[CloseEpochA]      } for 3 but 2 WtId, 5 steps
run witness_SY_12_crash_after_release_lease    { Assumed and crashAfter[ReleaseLeaseA]    } for 3 but 2 WtId, 5 steps

// --- SY-13: no stable state is a sink ---------------------------------------
//
// EXISTENTIAL REACHABILITY, AND DELIBERATELY NOT LIVENESS.  The catalogue says
// so in as many words and gives the reason: stating it as *the loop WILL reach
// one* needs a fairness premise these models have no grounds to grant, because
// nothing here schedules the operator and `EN-15` says Grove cannot verify a
// confirmation.  Neither command below contains an `eventually` inside an
// `always`, and neither is a liveness property.  What carries the EXISTENTIAL
// half is the four `witness_SY_13b_` runs — a run IS an existential over traces
// — and what the two checks carry is the half a run cannot: that no admitted
// action of Grove's own destroys the escape, and that the four classes the
// sweep ranges over are all the non-goal states there are.
//
// A READER WHO TAKES EITHER CHECK FOR A LIVENESS PROPERTY HAS READ FAIRNESS
// INTO A FILE THAT ASSUMES NONE, exactly as `README.md` says of `SY-10.b`.

/* SY-13.a.  Two conjuncts, and both are about GROVE rather than about the
   world, which is this obligation's whole shape once the sink below is found.

   1. GROVE NEVER MANUFACTURES THE OPERATOR-ONLY REFUSAL STATE.  `World.legacy`
      never grows except environmentally.  This is what makes the `no
      World.legacy` antecedent in `SY-13.b` a BOUNDARY GROVE RESPECTS rather
      than a hole in the sweep, and it is the conjunct with the isolating
      mutation (an `initialise-root` that marks the root legacy).
   2. AND IT NEVER TAKES THE LAST LIVE LEAF AWAY EXCEPT BY REACHING A TERMINAL
      DISPOSITION.  Losing the last live leaf is the one admitted step that
      could strand the loop, and the only admitted step that does it is the
      settle — which lands on the first terminal disposition by construction of
      the claim rather than of the transition. */
pred SY13a {
  always {
    (Sys.res' != Environmental) implies World.legacy' in World.legacy
    (some World.live and no World.live' and Sys.res' != Environmental)
      implies (no World.rooted' and some World.retired')
  }
}
check SY_13a_no_admitted_action_strands_the_loop {
  Assumed implies SY13a
} for 3 but 2 WtId, 5 steps

/* THE LONGEST ADMITTED SEQUENCE WITHIN THE BOUND, AND ITS LENGTH — the
   catalogue's own witness for `SY-13.a`, and the number is in `README.md`
   beside the bound.  It starts from the emptiest stable state this scope has —
   no grove at the name, nothing retired, a driver holding nothing — and reaches
   a live leaf to run.

   FIVE ADMITTED ACTIONS AND ONE BOUNDARY.  `IterA` is in the middle of it and
   is NOT one of the five: it is this file's own abstraction and not a catalogue
   action, so the sequence's admitted length is five and its trace length is
   six.  Both numbers are recorded, because a reader counting states and a
   reader counting admitted actions would otherwise get different answers and
   only one of them is the claim's. */
run witness_SY_13a_the_longest_admitted_sequence_within_the_bound {
  Assumed
  some p: Proc {
    p.role = DriverR
    no World.rooted and no World.retired and no p.holds and no p.seen
    eventually (Sys.act = AcquireLeaseA and Sys.actor = p and Sys.res = Applied
      and after eventually (Sys.act = IterA and Sys.actor = p
      and after eventually (Sys.act = OpenEpochA and Sys.actor = p and Sys.res = Applied
      and after eventually (Sys.act = TakeTreeA and Sys.actor = p and Sys.res = Applied
      and after eventually (Sys.act = InitRootA and Sys.actor = p
      and after eventually (Sys.act = CompleteScaffoldA and Sys.res = Applied
                            and some World.live))))))
  }
} for 3 but 2 WtId, 9 steps          // lands at 8; 9 is the one state of margin

/* SY-13.b.  THE EXHAUSTIVE SWEEP, AND THE ANTECEDENT IS THIS LEAF'S MATERIAL
   CATALOGUE FINDING.

   Four arms, each a stable state with a NAMED admitted exit:

     A  no grove and nothing retired          -> initialise-root
     B  a partial scaffold                    -> complete-scaffold
     C  a classified root with no live leaf   -> allocate-finish-leaf
     D  a transaction standing part-way       -> settle-deletion, or recover

   and the check is that within the bound there is NO FIFTH.  It is a case
   analysis and it is meant to be one: what it forbids is a non-goal state the
   four `witness_SY_13b_` runs do not speak for, and the mutation that gives the
   file one is a proof that drifts off the root it is about (`M20` in
   `README.md`), which leaves a present root with no live leaf, no partial and
   a proof naming something else — outside C, outside D, and with no exit.

   `no World.legacy` IS A NARROWING AND IT IS THE FINDING.  Without it the check
   fires immediately, on a trace with no protocol in it: `EN-11` licenses the
   operator to hand-edit a `Legacy` tree into the name, every admitted action
   then refuses `FormatLegacy`, and no admitted sequence reaches a live leaf or
   either terminal disposition.  **A `Legacy` tree is a sink, and `SY-13` as
   worded is false on it.**  The catalogue knows the shape and declines both
   repairs: its own note says a `Malformed` tree is not a terminal disposition
   because folding it in "would let the claim be satisfied by a tree nobody can
   act on" — which is right, and which leaves the claim false rather than weak.
   The repair it does not consider is the one taken here: quantify over the
   stable states THE LOOP'S OWN ADMITTED ACTIONS REACH, and let `SY-13.a`'s
   first conjunct carry the boundary.  Recorded in `README.md` and in
   `docs/formalism-findings.md` entry 043; not fixed here, because the catalogue
   is both families' shared subject and the independence protocol holds. */
pred SY13b {
  always {
    (not atGoal and no World.legacy) implies (
         (no World.rooted and no World.retired)                        // A
      or some World.partial                                             // B
      or (some World.rooted and no World.partial and no World.legacy
          and no World.live and no World.proven)                        // C
      or (some World.rooted and World.proven = World.rooted)            // D
    )
  }
}
check SY_13b_the_stable_states_are_four_and_each_has_a_named_admitted_exit {
  Assumed implies SY13b
} for 3 but 2 WtId, 5 steps

/* THE SWEEP ITSELF: one run per arm, each exhibiting the admitted exit and the
   goal it reaches, at the bound stated beside it.  These are the existential
   half of `SY-13` and they are runs for exactly that reason. */
run witness_SY_13b_no_grove_is_not_a_sink {
  Assumed
  no World.rooted and no World.retired
  eventually (Sys.act = InitRootA
              and after eventually (Sys.act = CompleteScaffoldA and Sys.res = Applied
                                    and some World.live))
} for 3 but 2 WtId, 5 steps

run witness_SY_13b_a_partial_scaffold_is_not_a_sink {
  Assumed
  some World.partial
  eventually (Sys.act = CompleteScaffoldA and Sys.res = Applied and some World.live)
} for 3 but 2 WtId, 4 steps

run witness_SY_13b_a_spent_tree_is_not_a_sink {
  Assumed
  some World.rooted and no World.live and no World.partial and no World.legacy
  eventually (Sys.act = AllocFinishA and Sys.res = Applied and some World.live)
} for 3 but 2 WtId, 4 steps

run witness_SY_13b_a_transaction_part_way_is_not_a_sink {
  Assumed
  some World.rooted and World.proven = World.rooted
  eventually (Sys.act = SettleDeletionA and no World.rooted and some World.retired)
} for 3 but 2 WtId, 4 steps

// --- SY-14: a blocked tree stays blocked until an operator acts -------------

/* SY-14.a — NO ADMITTED ACTION CLEARS A BLOCK, and the sweep is EXHAUSTIVE by
   being a check: `Sys.act' in AdmittedAct` ranges over all twenty-two of them
   in every state within the bound, which is what *an exhaustive sweep of the
   action set against a blocked tree* asks for and what no finite list of runs
   would give.  The run beside it lands the non-vacuity — the same division of
   labour `SY-05.b` uses, and for the same reason.

   THE SECOND CONJUNCT IS THE OTHER DIRECTION and it is what keeps the first
   from being about a field nothing writes: a block arrives from recovery's own
   decline and from nowhere else.

   WHAT IS NOT HERE, AND IT IS A LIMIT RATHER THAN A GAP: the operator action
   that DOES clear a block.  `FN-26` names the two restorable exits and they are
   `crates/grove-finish/models/`'s; §*Actions* puts operator actions outside the
   admitted set by construction, so *until an operator acts* is, at this scope,
   exactly *never, by anything this file has*. */
pred SY14a {
  always {
    (some World.blocked and Sys.act' in AdmittedAct) implies some World.blocked'
    (some World.blocked' and no World.blocked) implies Sys.act' = RecoverA
  }
}
check SY_14a_no_admitted_action_clears_a_block {
  Assumed implies SY14a
} for 3 but 2 WtId, 5 steps

run witness_SY_14a_a_block_reached_and_surviving_an_admitted_action {
  Assumed
  eventually (Sys.act = RecoverA and Sys.res = Blocked and some World.blocked)
  eventually (Sys.act = BlockedRefusalA and some World.blocked)
} for 3 but 2 WtId, 5 steps

/* SY-14.b — EVERY ACTION ON A BLOCKED TREE REFUSES, NAMING IT.

   STATED OVER `TreeAct` AND NOT OVER `AdmittedAct`, AND THE NARROWING IS A
   READING RATHER THAN AN ECONOMY.  A block is a property of the TREE — §*States*
   attaches the diagnosis to a `Reserved` root — so *every action on a blocked
   tree* is every action that acts on the tree.  A lease acquisition consults no
   tree and cannot name a block it has not read; requiring it to would be this
   file inventing a claim.  `TreeAct` is a set rather than a list for the reason
   the `roots` slice made it one: a later tree action reaches this conjunct
   without the command being edited.

   `Sys.res' = Blocked` IS *NAMING IT* AT THIS SCOPE.  The outcome is the
   catalogue's own and it is what a caller branches on; WHICH of `FN-25`'s two
   diagnoses it carries is the finish model's and is not imported, because no
   `SY-` obligation reads it.

   THE ONE-STEP REFUSAL IS DELIBERATE AND IS THE OPPOSITE OF `doAllocFinish`'s
   SPLIT.  That rule — prefer a named transition to a widened opaque one — is
   for a claim about WHICH mutation.  This claim is about EVERY action, and a
   per-action branch would be twenty-six copies of one sentence with
   twenty-six chances to omit it.  What carries the claim is the single
   `no World.blocked` conjunct in `mayTouchTree`, and the mutation is its
   removal. */
pred SY14b {
  always {
    (some World.blocked and Sys.act' in TreeAct) implies Sys.res' = Blocked
    (Sys.act' = BlockedRefusalA) implies (Sys.res' = Blocked and treeSame)
  }
}
check SY_14b_every_tree_action_on_a_blocked_tree_refuses_naming_the_block {
  Assumed implies SY14b
} for 3 but 2 WtId, 5 steps

run witness_SY_14b_an_action_on_a_blocked_tree_refuses_naming_the_block {
  Assumed
  some p: Proc | eventually (Sys.act = BlockedRefusalA and Sys.actor = p
                             and Sys.res = Blocked and some World.blocked
                             and TreeG in p.holds)
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

/* EN-08 — INTERRUPTION MAY OCCUR BETWEEN ANY TWO STEPS.  Class:
   EXERCISE-REMOVAL, and the thing removed is `crash` itself.  The assumption
   table's expected result has TWO halves and the form is `expect_unreachable_`
   for both — a `run` that must find NO instance, which the runner inverts:

     THE WITNESSES DIE.  All seven of `SY-12`'s crash points, in ONE command
     rather than seven, because the disjunction lands if ANY of them survives
     and that is the whole question.  Seven separate commands would report the
     same fact seven times at seven times the cost.
     THE PROPERTIES DO NOT.  Every property this slice reports, negated, under
     the same removal: no instance means each still holds over the crash-free
     traces.  The finish scope discharged its half of `EN-08` with the superset
     argument — no check asserts `EN_08`, so each is already checked over both
     kinds of trace — and that argument is sound and is repeated in
     `README.md`.  It is also unfalsifiable as stated, which is why the eight
     bodies above are named predicates and this is a command.

   AND THE FIRST HALF IS EVIDENCE RATHER THAN A TAUTOLOGY BECAUSE OF HOW
   `crashAfter` IS WRITTEN.  A witness that POSITS the state an interruption
   leaves, instead of running `crash` to reach it, keeps landing when the action
   is removed — `finish-k8` found exactly that in two of `FN-31.c`'s witnesses
   and recorded it as the thing an exercise-removal exists to make visible.  All
   seven here reach. */
run expect_unreachable_EN_08_no_lifecycle_step_is_a_crash_point {
  Assumed and always Sys.act != CrashA
  crashAfter[AcquireLeaseA] or crashAfter[LayoutPreflightA]
  or crashAfter[OpenEpochA] or crashAfter[LaunchA] or crashAfter[ReapA]
  or crashAfter[CloseEpochA] or crashAfter[ReleaseLeaseA]
} for 3 but 2 WtId, 7 steps

run expect_unreachable_EN_08_no_property_fails_when_crash_is_removed {
  Assumed and always Sys.act != CrashA
  not (SY09a and SY09b and SY09c and SY12 and SY13a and SY13b and SY14a and SY14b)
} for 3 but 2 WtId, 6 steps
