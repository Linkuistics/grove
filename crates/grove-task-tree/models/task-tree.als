/*
 * grove-task-tree — the task-tree claims, in Alloy 6
 * ==================================================
 *
 * The subject is `docs/specs/semantic-contract.md`, section *Claims — task
 * tree*.  Nothing else: no Rust module, no helper, no control-flow shape.  Every
 * command below names an OBLIGATION of that document, and the repository runner
 * reads the obligation list out of the document rather than out of this file.
 *
 * COVERAGE SO FAR: TT-01 .. TT-23.  TT-24 and TT-25 are the `ownership` sibling
 * leaf's; the runner reports their cells empty, which is the truth about this
 * file rather than a defect in it.
 *
 * TWO PROCESS SCOPES.  TT-01 .. TT-20 are stated over ONE cooperating process
 * and over operations that are one transition each; TT-21 .. TT-23 are about
 * what happens DURING an operation and about what a second process may do while
 * it runs.  `Env.concOn` is a STATIC switch selecting between them, and every
 * command pins it: `CurrentRootThroughout` and the root-identity commands pin it
 * off, `Guarding` turns it on.  Leaving it free is not an optimisation question
 * -- in the concurrent scope no grove mutation exists, so an unpinned witness is
 * unreachable and an unpinned check is vacuous.
 *
 * HOW TO READ IT — the house style of `docs/ordinal-fs-tree/models/`:
 *
 *   Nothing the catalogue merely CLAIMS is a `fact`.  Claims are named
 *   predicates, and every command says which ones it assumes.  Facts hold only
 *   what is true by construction — a filesystem cannot hold two entries of one
 *   name in one directory, an inode is a file or a directory and not both.
 *
 *     check TT_nn[x]_<mnemonic>            must find NO counterexample
 *     run   witness_TT_nn[x]_<mnemonic>    must find an instance
 *
 * WHAT IS ASSUMED RATHER THAN MODELLED.  `ordinal-fs-tree` is an imported
 * algebraic boundary (`docs/ordinal-fs-tree/ARCHITECTURE.md`): ordering,
 * shifting and allocation are ITS properties, and this file states grove's
 * domain preconditions in front of them.  `TT-10` is the claim that the seam
 * holds; the algebra's own refusal is a single opaque `AlgebraicRefusal`, and
 * this file never reimplements what would produce it.
 *
 * A GREEN RUN OF THIS FILE IS NOT, BY ITSELF, EVIDENCE.  It has already reported
 * itself green — witnesses included — while checking nothing at all, and what
 * separated the fiction from the fact was one mutation per obligation.  The
 * matrix, the incident and the bounds caveats are in `README.md` beside this
 * file; read it before trusting a green run, and re-run the mutations after
 * changing a transition.
 *
 * Run it with `models/run.sh --scope task-tree --family alloy --no-coverage`
 * from the repository root.
 */
module task_tree

open util/integer


// ===========================================================================
// VOCABULARY
//
// Positions and keys are Int because the claims about them are arithmetic:
// allocation is `max + 1` (TT-05) and a shift is `+ 1` (TT-06.b).  Slugs are
// opaque atoms; nothing in the catalogue reads one.
// ===========================================================================

sig Slug {}

abstract sig Kind {}
one sig FinishK, OrdinaryK, UnknownK extends Kind {}
/* the closed nineteen, abstracted to what any claim distinguishes: the
   driver-reserved one, an ordinary one, and one this build does not know. */
fun known: set Kind { FinishK + OrdinaryK }

abstract sig Infix {}
one sig LiveI, DoneI, AbandonedI extends Infix {}
fun terminalInfix: set Infix { DoneI + AbandonedI }

abstract sig Species {}
one sig LeafSp, NodeSp extends Species {}

/* An entry's opaque content equality — the *entry digest* of the catalogue's
   vocabulary, which the deliberate-omission row grants as an opaque equality.
   It is what lets TT-07's "never any file's bytes" be checked at all. */
sig Digest {}

/* A bare filename.  A name is the tuple of its parts and NOTHING else; a
   filename that carries no parts is either the charter or foreign.
   A NODE name carries no session kind and no outcome infix — that is the
   grammar, and it is what makes a name declare its own species (TT-02). */
sig Filename {
  fPos:   lone Int,
  fKey:   lone Int,
  fSlug:  lone Slug,
  fKind:  lone Kind,
  fOut:   lone Infix,
  fSpec:  lone Species,
  canon:  lone Filename        // the canonical spelling of this same reading
}

one sig CharterF in Filename {}   // the node charter's name; one atom, shared

pred isShaped[f: Filename] {
  some f.fPos and some f.fKey and some f.fSlug and some f.fSpec
  f.fSpec = LeafSp implies (some f.fKind and some f.fOut)
  f.fSpec = NodeSp implies (no f.fKind and no f.fOut)
}

pred sameReading[f, g: Filename] {
  f.fPos = g.fPos and f.fKey = g.fKey and f.fSlug = g.fSlug
  f.fKind = g.fKind and f.fOut = g.fOut and f.fSpec = g.fSpec
}

fun shaped: set Filename { { f: Filename | isShaped[f] } }

/* The grammar: which filenames it reads as entries.  A set rather than a
   function, so that the stated grammar and the corrected one are two predicates
   over one relation rather than two models. */
one sig Grammar { accepts: set Filename }

fact NamesByConstruction {
  // a charter carries no parts, and is not a task-shaped name
  no CharterF.(fPos + fKey + fSlug + fKind + fOut + fSpec + canon)
  // `canon` is defined exactly on shaped names, is idempotent, and preserves
  // the reading: a spelling's canonical form is a spelling of the SAME entry.
  all f: Filename | some f.canon iff isShaped[f]
  all f: shaped | isShaped[f.canon] and f.canon.canon = f.canon and sameReading[f, f.canon]
  // Two spellings of one entry share a canonical form.  Structural, and what
  // lets `canon` stand in for "the canonical spelling" a refusal names.
  all disj f, g: shaped | sameReading[f, g] implies f.canon = g.canon
  // positions and keys are drawn from the positive range; nothing rides on the
  // bound except instance legibility.
  all f: shaped | f.fPos > 0 and f.fKey > 0
  // The grammar reads entries out of task-shaped names carrying a known session
  // kind, and out of nothing else.  Definitional: this is what "task-shaped" and
  // "the closed nineteen" mean, not a claim anyone could contradict.
  Grammar.accepts in shaped
  all f: Grammar.accepts | f.fSpec = LeafSp implies f.fKind in known
}


// ===========================================================================
// THE LAWS THE CATALOGUE STATES ABOUT NAMES
//
// The one place the two readings differ.  Nothing below is a `fact`.
// ===========================================================================

/* TT-01, as the ADR `task-names-are-canonical` settles it: the grammar accepts
   only canonical spellings, and every other spelling of the same reading is
   refused with its canonical form to hand. */
pred ParseIsCanonical {
  all f: Grammar.accepts | f.canon = f
}

/* The grammar refuses SPELLINGS, not entries: every canonical, known-kind name
   is accepted.  Without it a model can pass every check by having a grammar
   that accepts nothing, and no mutation can ever find a name to write. */
pred GrammarIsTotal {
  all f: shaped | (f.canon = f and (f.fSpec = NodeSp or f.fKind in known))
                    implies f in Grammar.accepts
}

/* EN-12 rides in the bundle rather than beside it, for a solver reason worth
   stating: `Rendering.collide` is a free static relation, so leaving it
   unpinned would be paid for by every command in the file rather than by the
   one that drops it. */
pred GroveGrammar { ParseIsCanonical and GrammarIsTotal and EN_12 }

/* The grammar WITHOUT the canonicity rule — a round trip stated in one
   direction only, which is what a grammar looks like before anyone asks whether
   two spellings can name one entry.  Kept so the defect has somewhere to
   appear. */
pred StatedGrammar {
  GrammarIsTotal
  all f: shaped | (f.fSpec = NodeSp or f.fKind in known) implies f in Grammar.accepts
}

fun entryName:      set Filename { Grammar.accepts }
/* Shaped, and not accepted: task-shaped and malformed — `MalformedEntry` of the
   catalogue's reason table.  A non-canonical spelling and an unknown session
   kind land in the same place, and the refusal names `f.canon`. */
fun malformedName:  set Filename { shaped - Grammar.accepts }
fun foreignName:    set Filename { Filename - shaped - CharterF }


// ===========================================================================
// THE FILESYSTEM
//
// An inode is a file or a directory and does not become the other: that is why
// promotion REPLACES an object rather than mutating one, and why the key rather
// than the atom is what TT-08 preserves.
// ===========================================================================

abstract sig Obj {
  var nm:  lone Filename,
  var loc: lone Obj,
  var dg:  lone Digest
}
sig FileObj extends Obj {}
sig DirObj  extends Obj {}
one sig TaskRoot extends DirObj {}

var sig onDisk in Obj {}

fact Filesystem {
  always {
    TaskRoot in onDisk
    no TaskRoot.loc and no TaskRoot.nm and no TaskRoot.dg
    all o: Obj - onDisk | no o.nm and no o.loc and no o.dg
    all o: onDisk - TaskRoot | one o.nm and one o.dg and o.loc in onDisk & DirObj
    all o: onDisk | o not in o.^loc                         // acyclic
    all d: onDisk & DirObj, f: Filename | lone (loc.d & nm.f)  // names unique per dir
    onDisk = TaskRoot.*(~loc)                               // nothing floating
  }
}

fun kidsOf[d: Obj]:    set Obj { loc.d & onDisk }

/* THE WALK DESCENDS INTO NODES, AND INTO NOTHING ELSE.  A directory whose own
   name is outside the task grammar is foreign, and grove never opens it — so a
   task-shaped file inside one is not an entry, does not hold a position on any
   level grove orders, and its key is not part of the counter.  Modelling
   `entries` as *any parseable name transitively beneath the task root* — which
   is what the catalogue's own wording said — makes a foreign directory's
   contents into a level whose positions start wherever they happen to start,
   and `TT-06.b` finds it. */
fun descendable: set Obj {
  TaskRoot + { d: (onDisk & DirObj) - TaskRoot | d.nm in entryName and d.nm.fSpec = NodeSp }
}
fun visited: set Obj { TaskRoot.*(descendable <: ~loc) & onDisk }

fun entries:           set Obj { { o: visited - TaskRoot | o.nm in entryName } }
fun malformedEntries:  set Obj { { o: visited - TaskRoot | o.nm in malformedName } }
fun charters:          set Obj { { o: visited - TaskRoot | o.nm in CharterF } }
/* Foreign is not restricted to the walk: TT-04 is about bytes grove must not
   touch, and it must not touch them wherever they sit. */
fun foreignEntries:    set Obj { { o: onDisk - TaskRoot | o.nm in foreignName } }
fun entriesIn[d: Obj]: set Obj { kidsOf[d] & entries }
fun nodeDirs:          set Obj { TaskRoot + { o: entries | o.nm.fSpec = NodeSp } }
fun liveLeaves:        set Obj { { o: entries | o.nm.fSpec = LeafSp and o.nm.fOut = LiveI } }
fun liveFinish:        set Obj { { o: liveLeaves | o.nm.fKind = FinishK } }
fun liveOrdinary:      set Obj { liveLeaves - liveFinish }
fun allKeys:           set Int { entries.nm.fKey }

/* DERIVED DONE-NESS (TT-25).  A node is done when no live leaf is beneath it,
   and there is nowhere else for the answer to come from: a node NAME carries no
   outcome infix at all (`isShaped`), so this model cannot even spell a marked
   node.  What is left to check is behavioural -- that the answer tracks the
   subtree, and that the transition which makes a node done writes NOTHING to
   the node. */
pred nodeDone[d: Obj] { no (liveLeaves & d.^(~loc)) }

/* ---------------------------------------------------------------------------
   THE WALK (TT-11).  Depth-first PRE-ORDER over positions, and it needs no rank
   relation: `a` precedes `b` exactly when `a` is an ancestor of `b`, or some
   ancestor-or-self of each are siblings ordered by position.  `loc` is the
   parent relation, so `x: a.*loc` is `a` and its ancestors.

   Stated as a function of `loc` and `nm` and NOTHING else.  That is how TT-11's
   "depends on no state outside the tree" is answered: by construction, not by a
   command — a model cannot check the absence of a variable it does not have.
   --------------------------------------------------------------------------- */
pred precedes[a, b: Obj] {
  b in a.^(~loc)
  or (some x: a.*loc, y: b.*loc | x.loc = y.loc and some x.loc and x.nm.fPos < y.nm.fPos)
}

/* TT-13: a live finish leaf is RESERVED, not blocking — skipped while any
   ordinary work is live, returned when it is the only live leaf. */
fun eligible: set Obj { some liveOrdinary implies liveOrdinary else liveFinish }

/* TT-11 + TT-14: the `precedes`-minimal eligible leaf.  Position and terminality
   are the only mechanisms that enter; there is nowhere for a priority to go. */
fun selected: set Obj { { o: eligible | no p: eligible - o | precedes[p, o] } }

/* A resolution's argument: a permanent key, a slug, or both — the CLI's `[n]`,
   a bare slug, and the full `<slug>-k<key>` handle.  Nothing else about the
   reference matters to TT-15/TT-16, and the model reads no slug's content. */
one sig Query { qKey: lone Int, qSlug: lone Slug }
fact QueryIsNonEmpty { some Query.qKey or some Query.qSlug }

/* Resolution searches live, DONE and ABANDONED entries alike — which is exactly
   why TT-16 exists: a match is not evidence of liveness. */
fun matched[q: Query]: set Obj {
  { o: entries | (some q.qKey  implies o.nm.fKey  = q.qKey)
             and (some q.qSlug implies o.nm.fSlug = q.qSlug) }
}

// ===========================================================================
// THE TASK ROOT'S OWN IDENTITY (TT-17 .. TT-20)
//
// Modelled BESIDE the ordered entries rather than among them.  A witness holds
// no position, carries no permanent key, and is never ordered, so nothing any
// `TT-17` .. `TT-20` obligation says about one reads a `Filename` -- what those
// obligations read is its PRESENCE and its CONTENT.  Keeping witnesses out of
// `Obj` is also what keeps the existing slice's bounds where they are: a
// witness costs no `FileObj` and no `Filename` atom.
// ===========================================================================

/* What the format witness HOLDS.  TT-17 is the claim that classification reads
   this and nothing else -- never any task entry's text -- so it has to be
   variable independently of every name in the tree. */
abstract sig Format {}
one sig CurrentFmt, ForeignFmt extends Format {}

/* What sits at a name grove RESERVES.  Either an artifact grove can prove is
   its own -- a finish witness, whose class names the recovery that settles it
   (`WitnessPending(class)`) -- or one it cannot classify at all (`Unowned`,
   which is `ReservedNameOccupied`'s content and `ownership`'s to claim). */
abstract sig SlotContent {}
abstract sig WitnessClass extends SlotContent {}
one sig Preparing, Published, Migrating extends WitnessClass {}
one sig Unowned extends SlotContent {}

/* The format witness.  `some Fmt.fmt` is "the witness is present"; publication
   is one step, which is the atomic same-directory rename of TT-20. */
one sig Fmt { var fmt: lone Format }

/* The reserved name.  ONE slot: no TT- obligation counts them, and every one of
   them is stated over the reserved CLASS rather than over its members.

   `occAt` IS WHERE THE RESERVED NAME IS, and it is why this model needs no
   reserved `Filename`.  `TT-24.b`'s refusal reason CARRIES THE ENTRY -- an
   occupant grove cannot classify is named, so that the operator is told what is
   in the way -- and `SlotContent` cannot name an `Obj`.  What it names is a real
   filesystem object, so the occupant IS one; what makes it *reserved* is that
   the slot points at it, exactly as what makes a witness reserved is that the
   slot holds it.  The alternative -- `one sig ReservedF in Filename`, the
   reserved spelling as an atom -- would consume a `Filename` atom in EVERY
   command in the file, and the nine-minute `TT-07` witness runs at six with
   nothing spare. */
one sig Slot {
  var occ:   lone SlotContent,
  var occAt: lone Obj
}

/* WHAT THE OCCUPANT IS, and when there is one.  A witness is grove's own
   artifact and holds no object (`TT-17` .. `TT-20` read its presence and its
   content, never a name); an `Unowned` occupant is someone else's bytes, and
   naming them is the whole of `TT-24.b`.  Its own name is foreign -- grove
   cannot parse it -- which is what `cannot classify at all` means here. */
fact ReservedOccupancy {
  always {
    some Slot.occAt iff Slot.occ = Unowned
    all o: Slot.occAt | o in onDisk - TaskRoot and o.nm in foreignName
  }
}

/* An initialisation transaction in flight.  `some inFlight` is the catalogue's
   TRANSIENT state -- it exists only between two filesystem steps of one
   operation -- and every state TT-20 is stated over is a STABLE one the
   transaction has already left behind. */
one sig Txn {}
var sig inFlight in Txn {}

/* The bytes a fresh scaffold writes.  An `in` subset, so it costs no atom; the
   charter's and the first leaf's are one atom rather than two, which no TT-
   obligation distinguishes (the digest is an opaque equality). */
one sig ScaffoldD in Digest {}

/* The catalogue's root states, less `Absent`: no TT- obligation reads it and
   `SY-05` owns it.  `Reserved` is one state rather than three because TT-18 and
   TT-19 are stated over the reserved CLASS. */
abstract sig RootState {}
one sig ReservedR, PartialScaffoldR, AmbiguousScaffoldR, LegacyR, ForeignR,
        MalformedR, CurrentLiveR, CurrentFinishOnlyR, CurrentSpentR
        extends RootState {}
fun currentFamily: set RootState { CurrentLiveR + CurrentFinishOnlyR + CurrentSpentR }

/* The catalogue's `PartialScaffold(class)`, as TWO atoms where `Reserved(class)`
   is one, and the asymmetry is the claims rather than the encoding.  No claim in
   this scope distinguishes `Reserved`'s three members, so collapsing them costs
   nothing; the scaffold classes are distinguished by two — `SY-06.b` completes
   `Exact` and must NOT complete `Ambiguous`, and the refusal an ordinary
   operation gets carries the class.  TT-18 and TT-20 are still stated over the
   FAMILY, which is what keeps them insensitive to a member being added or
   removed. */
fun scaffoldFamily: set RootState { PartialScaffoldR + AmbiguousScaffoldR }

/* The FORMAT family a classification lands in.  TT-17 is stated over this and
   not over the state itself, because the split INSIDE `Current(*)` is
   walk-derived and reading entries is what it is for; what the claim forbids is
   entry text moving the root between the families. */
fun familyOf[s: RootState]: set RootState {
  s = ReservedR                          implies ReservedR
  else s in (scaffoldFamily + LegacyR)   implies (scaffoldFamily + LegacyR)
  else s = ForeignR                      implies ForeignR
  else                                           (MalformedR + currentFamily)
}

/* THE ORDERED THREE-WAY TEST the catalogue's `States` defines for a witnessless
   root.  It replaces a single `isPartialScaffold` whose exact closed SUBSET was
   the whole test: anything outside the subset -- a second positioned entry, a
   differing byte, a foreign entry, a node -- fell through to `Legacy`, so a
   stray file beside grove's OWN half-written scaffold made grove read its own
   interrupted work as somebody else's legacy tree.  That is entry 044's
   counterexample, replayed here by `cross-model-replay-k15` and disposed by
   `task-tree-scope-k70`. */

/* Branch one: nothing but the fresh scaffold's own byte-exact entries.  This is
   the old `isPartialScaffold`, unchanged, and it is what makes completion safe
   -- every value the completion writes is fixed in advance, so completing is a
   comparison followed by at most one append. */
pred isExactScaffold {
  no Fmt.fmt
  no foreignEntries
  no malformedEntries
  charters in kidsOf[TaskRoot]
  lone charters
  entries in kidsOf[TaskRoot]
  lone entries
  all e: entries | {
    e in FileObj
    e.nm.fSpec = LeafSp and e.nm.fOut = LiveI and e.nm.fKind = OrdinaryK
    e.nm.fPos = 1 and e.nm.fKey = 1
    e.dg = ScaffoldD
  }
  all c: charters | c.dg = ScaffoldD
  // nothing else beneath the root at all
  onDisk - TaskRoot = entries + charters
}

/* ROOT-INIT-EXCLUSIVE: an entry only THIS format's root initialisation writes.
   The CHARTER IS DELIBERATELY NOT ONE.  Its bytes derive from the working-tree
   name and every earlier format wrote the same ones, so it is evidence that
   SOME grove was here and never evidence of THIS format's initialisation --
   which is the content of the shipped
   `an_untouched_root_brief_does_not_hide_a_legacy_v2_tree`, where a byte-exact
   charter beside a legacy leaf migrates rather than completing.

   The catalogue names two exclusives and this model reaches one: it has no
   reserved format temporary, since `doInitPublish` makes the witness visible in
   one step.  The abstraction is safe in the direction that matters -- fewer
   exclusives means MORE roots fall to `Legacy`, so TT-20's declared window is
   modelled at its widest -- and it coincides exactly with the shipped window,
   after the charter and before the leaf. */
pred hasRootInitExclusive {
  some e: entries & kidsOf[TaskRoot] | {
    e in FileObj
    e.nm.fSpec = LeafSp and e.nm.fOut = LiveI and e.nm.fKind = OrdinaryK
    e.nm.fPos = 1 and e.nm.fKey = 1
    e.dg = ScaffoldD
  }
}

/* Branch two, reached only when branch one fails: proof that initialisation ran,
   standing beside something a fresh scaffold does not write.  Grove can prove a
   root-init happened here; what it cannot prove is that the root's WHOLE
   contents are its own, so it refuses and mutates nothing.  That is TT-24's
   fail-closed ownership rule at the ROOT grain, and it is what ships --
   `recover_partial_root_init_unlocked`'s *ambiguous partial root scaffold*. */
pred isAmbiguousScaffold {
  no Fmt.fmt
  not isExactScaffold
  hasRootInitExclusive
}

/* CLASSIFICATION, IN THE FIXED ORDER (TT-18): reserved-witness first, then
   format, then walk-derived -- and `PartialScaffold` before `Legacy`.  A `fun`
   rather than a `var` field, so it adds no free state for the solver to search;
   the order is a claim because REORDERING THIS BODY is a mutation the matrix
   runs, not because the model leaves the order open. */
fun rootState: one RootState {
  some Slot.occ                                        implies ReservedR
  else (no Fmt.fmt and isExactScaffold)                implies PartialScaffoldR
  else (no Fmt.fmt and isAmbiguousScaffold)            implies AmbiguousScaffoldR
  else no Fmt.fmt                                      implies LegacyR
  else Fmt.fmt = ForeignFmt                            implies ForeignR
  else halted                                          implies MalformedR
  else some liveOrdinary                               implies CurrentLiveR
  else some liveFinish                                 implies CurrentFinishOnlyR
  else                                                         CurrentSpentR
}

/* The root an ordinary operation may act on at all. */
pred rootClear { no Slot.occ and Fmt.fmt = CurrentFmt }

/* The assumption bundle TT-01 .. TT-16 were always stated over, said out loud
   now that there is a root classification to say it against: a current-format
   root, no reserved witness, and no transaction in flight.  It PINS the new
   state rather than leaving it free, which is why those thirty commands cost
   what they cost before this layer arrived. */
pred CurrentRootThroughout {
  // ONE COOPERATING PROCESS, pinned rather than left free.  Without it the
  // solver may pick the concurrent scope, in which no grove mutation exists at
  // all -- every witness below would become unreachable and every check
  // vacuous, which is this file's retained false-confidence incident wearing a
  // third set of clothes.
  SingleProc
  always {
    rootClear
    no inFlight
    // AND NO ROOT-LIFECYCLE ACTION.  Not decoration: `initialise-root` and the
    // three recoveries are transitions the solver encodes in every trace even
    // where their guards can never fire, and TT-03's check -- already the
    // tightest command in the file, run one filename short of its neighbours --
    // stops finishing without this clause.  It narrows the ANTECEDENT rather
    // than the bound, which is the trade recorded on TT-05's four commands.
    //
    // Nothing is lost by it.  `Crash` under `no inFlight` is a pure stutter, and
    // `doIdle` already supplies one; the four root actions all refuse on a
    // current-format root, so no trace they could have contributed reaches an
    // `Applied` any TT-01..TT-16 command is stated over.
    Sys.act' not in (rootActs + Crash)
  }
}

// ===========================================================================
// THE CONCURRENCY LAYER (TT-21 .. TT-23)
//
// EVERYTHING ABOVE IS STATED OVER ONE COOPERATING PROCESS.  The README already
// recorded that as a bound ("one cooperating process"); `Env.concOn` is the
// bound made OPERATIONAL.  It is a STATIC switch, so a command that does not
// turn it on pays for these transitions in Alloy's translation and not in the
// solver's search -- which is the standing rule this file learned when four
// root-lifecycle transitions took `TT-03` from 68s to not finishing.
//
// WHAT AN OPERATION IS, HERE.  Above, an operation is one atomic transition.
// TT-21 and TT-22 are claims about what happens DURING one, so here an
// operation has a duration:
//
//   observation   Open(Shared) . Classify* . Release        -- the guard is HELD
//   mutation      Mark                                       -- one step, and the
//                                                               guard is CONSUMED
//
// The asymmetry is not a simplification, it is the ADR.  `bulk-marks-are-not-
// atomic` records that a mutating method CONSUMES its `WriteGuard`, so N marks
// are N critical sections; an exclusive guard therefore never spans a state
// boundary and `holds` only ever carries `Shared`.  TT-22.b is checked over the
// mark's own acquisition rather than over a held exclusive guard, and the
// mutation that breaks it is the removal of that acquisition test.
// ===========================================================================

/* TWO cooperating processes.  Two, because every TT-21/TT-22 obligation is
   about one operation and one other party, and a third pays for a claim no
   obligation makes. */
abstract sig Proc {
  /* The guard this process HOLDS.  `Shared` while an observation is in flight,
     and nothing otherwise -- see the asymmetry above. */
  var holds:   lone Mode,
  /* THE ONE LISTING (TT-21).  What the operation saw when it took its guard,
     and the only thing any of its classifications may read.  It is state rather
     than a re-read precisely so that a classification CAN be got wrong: a model
     whose classifications are functions of the live tree has answered TT-21 by
     construction. */
  var snapOn:  set Obj,
  var snapNm:  Obj -> lone Filename,
  /* The classifications the operation has made: what it asked about, and what
     it concluded.  Written by the transition, for the reason `Sys.got` is. */
  var cls:     set Obj,
  var clsLive: set Obj,
  /* A bulk mark's REMAINING plan, and the listing it was validated against.
     TWO fields rather than one, and the reason is the ADR: a plan OUTLIVES the
     guard that validated it, while an observation's listing dies with its
     guard.  Sharing `snapOn`/`snapNm` between them lets a `Release` orphan a
     live plan -- which is a counterexample this file found and retained, not a
     hypothetical. */
  var plan:    set Obj,
  var planNm:  Obj -> lone Filename
}
one sig P1, P2 extends Proc {}

abstract sig Mode {}
one sig Shared, Exclusive extends Mode {}

/* THE ENVIRONMENT SWITCHES.  Four static `lone Txn` fields -- reusing the `Txn`
   atom, so none of them costs an atom -- each pinning one thing a command may
   turn off.  Static rather than `var` deliberately: an assumption is a property
   of the SCOPE a command runs in, and a variable one would be free state every
   command in the file paid for. */
one sig Env {
  crashOn:   lone Txn,   // EN-08: interruption is a first-class action
  descShare: lone Txn,   // EN-07 BROKEN: one guard covers a whole bulk run
  wtRoot:    lone Txn,   // EN-14: the working-tree root the guard is held on
  concOn:    lone Txn,   // the process scope: one process, or two
  handEditOn: lone Txn   // EN-11: any well-formed tree is reachable by hand edit
}
pred EN_08 { some Env.crashOn }
/* EN-11 -- ANY WELL-FORMED TREE IS REACHABLE BY HAND EDIT.  It is realised HERE
   in two places and not one, and that is the whole content of its control: the
   `hand-edit` action, and the UNCONSTRAINED INITIAL STATE the README's `3 steps`
   argument rests on ("every single transition is reachable from state 0").
   Removing only the action leaves every witness it is supposed to control
   reachable at state 0, and the exercise-removal reports green while removing
   nothing -- so the switch takes away both, and what is left is a world grove's
   own actions had to build from an empty task root. */
pred EN_11 { some Env.handEditOn }
pred EN_07 { no Env.descShare }
pred EN_14 { some Env.wtRoot }
pred Concurrent  { some Env.concOn }
pred SingleProc  { no Env.concOn }

/* THE ONE LISTING, read.  A classification asks whether an object was a live
   leaf, and it answers from `snapOn`/`snapNm` -- never from `onDisk`/`nm`. */
pred liveInSnap[p: Proc, o: Obj] {
  o in p.snapOn
  p.snapNm[o] in entryName
  p.snapNm[o].fSpec = LeafSp
  p.snapNm[o].fOut = LiveI
}
/* What the listing SUPPORTS about the objects this operation asked about.
   TT-21 is the claim that `clsLive` equals it. */
fun liveBySnap[p: Proc]: set Obj { { o: p.cls | liveInSnap[p, o] } }

/* WHAT THE PLAN'S OWN LISTING LICENSED: every member the run is still working
   through was a live, non-reserved leaf in the listing the plan was validated
   against.  TT-21.b's "a mutation that was licensed by that listing", and the
   standing form of TT-23.a's whole-plan validation. */
pred licensedByPlan[p: Proc, o: Obj] {
  p.planNm[o] in entryName
  p.planNm[o].fSpec = LeafSp
  p.planNm[o].fOut = LiveI
  // NOT `fKind != FinishK`: whether the plan should have contained a reserved
  // leaf at all is TT-23.a's whole-plan validation, and folding it in here made
  // one mutation break both obligations for one defect.
}

/* GUARD COMPATIBILITY (TT-22): shared for observation, exclusive for mutation,
   both taken on the WORKING-TREE ROOT.  EN-14 is what the guard is held ON: with
   no working-tree root there is nothing to `flock`, no guard is taken, and the
   compatibility test has no subject -- which is the premise-break the assumption
   table asks for and the reason it is written as an implication. */
pred compatible[p: Proc, m: Mode] {
  EN_14 implies {
    m = Shared    implies (no q: Proc - p | q.holds = Exclusive)
    m = Exclusive implies (no q: Proc - p | some q.holds)
  }
}

/* A bulk mark's plan, and its validity, both computed from ONE listing.  The
   target is the task root: no TT-23 obligation reads a narrower subtree, and a
   narrower one costs a second `DirObj` every command in the slice would pay
   for.  A `finish` leaf is what makes a plan invalid -- the ADR's "a leaf the
   mark cannot address, a `finish` leaf, a leaf whose destination the tree
   already occupies", less the third, which this file does not represent. */
pred planValid { no o: liveLeaves | o.nm.fKind = FinishK }

/* Every process's state, unchanged.  The world's actions do not touch a
   process's listing -- that is the whole content of TT-21.b. */
pred procAllFrame {
  all q: Proc | {
    q.holds' = q.holds and q.snapOn' = q.snapOn and q.snapNm' = q.snapNm
    q.cls' = q.cls and q.clsLive' = q.clsLive
    q.plan' = q.plan and q.planNm' = q.planNm
  }
}
/* Every OTHER process's state, unchanged. */
pred procFrame[p: Proc] {
  all q: Proc - p | {
    q.holds' = q.holds and q.snapOn' = q.snapOn and q.snapNm' = q.snapNm
    q.cls' = q.cls and q.clsLive' = q.clsLive
    q.plan' = q.plan and q.planNm' = q.planNm
  }
}
/* This process's state, unchanged: what a `Deferred` leaves behind. */
pred procSelfFrame[p: Proc] {
  p.holds' = p.holds and p.snapOn' = p.snapOn and p.snapNm' = p.snapNm
  p.cls' = p.cls and p.clsLive' = p.clsLive
  p.plan' = p.plan and p.planNm' = p.planNm
}
/* No process exists at all: the single-process scope, pinned rather than left
   free, so the thirty-odd commands above pay nothing for this layer. */
pred procQuiet {
  no Sys.who' and no Sys.mode'
  all q: Proc | no q.holds' and no q.snapOn' and no q.snapNm'
                and no q.cls' and no q.clsLive' and no q.plan' and no q.planNm'
}

/* EN-12: A NAME RENDERS AS EXACTLY ONE PATH COMPONENT.  The base model gets it
   from the filesystem fact -- a name is unique within its directory and a level
   is a directory -- so the assumption has nowhere to be false and nothing to
   control.  `collide` is where it is given one: a rendering under which two
   distinct spellings reach one entry, which is what a separator inside a part
   buys.  Static, and pinned empty by `EN_12`, so every command that assumes the
   assumption pays nothing for its existence. */
one sig Rendering { collide: set Filename -> Filename }
pred EN_12 { no Rendering.collide }
pred denotesSame[f, g: Filename] {
  sameReading[f, g] or (f -> g) in Rendering.collide or (g -> f) in Rendering.collide
}

/* A name declares its species and the on-disk object must be it (TT-02). */
pred speciesAgrees[o: Obj] {
  (o.nm.fSpec = LeafSp implies o in FileObj) and (o.nm.fSpec = NodeSp implies o in DirObj)
}

/* The whole-tree malformity reasons this slice can reach.  Every one stops
   every read and mutation (TT-03), which is why they are a single predicate. */
pred rMalformedEntry   { some malformedEntries }
pred rSpeciesMismatch  { some o: entries | not speciesAgrees[o] }
/* `1..n` with no repetition and no gap, said WITHOUT counting.  The cardinality
   spelling (`#positions != #entries`) is equivalent and costs an Int-arithmetic
   encoding that took this one command past three minutes on its own; the
   relational spelling is seconds.  Three ways to fail: a repeated position, a
   level that does not start at 1, and a position whose predecessor is absent. */
pred gaplessAt[d: Obj] {
  no disj a, b: entriesIn[d] | a.nm.fPos = b.nm.fPos
  some entriesIn[d] implies (some e: entriesIn[d] | e.nm.fPos = 1)
  all e: entriesIn[d] |
    e.nm.fPos > 1 implies (some s: entriesIn[d] | s.nm.fPos = minus[e.nm.fPos, 1])
}
pred rPositionsNotGapless { some d: nodeDirs | not gaplessAt[d] }
pred rKeyReissued      { some disj a, b: entries | a.nm.fKey = b.nm.fKey }
pred rNodeWithoutCharter { some d: nodeDirs - TaskRoot | no (kidsOf[d] & charters) }
/* TT-13.c.  A WHOLE-TREE reason, the shape `rKeyReissued` has and not the shape
   `rSpeciesMismatch` has: both entries are individually well formed, and what is
   wrong is the tree.  `MultipleLiveFinish` of the catalogue's reason table. */
pred rMultipleLiveFinish { not lone liveFinish }

pred halted { rMalformedEntry or rSpeciesMismatch or rPositionsNotGapless
              or rKeyReissued or rNodeWithoutCharter or rMultipleLiveFinish }
pred treeOk { not halted }


// ===========================================================================
// ACTIONS, OUTCOMES, AND THE ALGEBRAIC BOUNDARY
//
// Every action is TOTAL: it returns exactly one outcome, and a guard that fails
// produces a named refusal rather than an absent transition.
// ===========================================================================

abstract sig Action {}
one sig Idle, AddLeaf, InsertLeaf, Decompose, Retire, Prune, HandEdit,
        Select, Resolve extends Action {}
/* Root initialisation is TWO filesystem steps, and it is the only action in
   this file that is not one.  It has to be: TT-20 is a claim about the state
   BETWEEN them, so an atomic initialisation would answer the question by
   construction the way a folded `replace-cleanup-marker` answers Q3. */
one sig InitScaffold, InitPublish extends Action {}
/* EN-08: interruption may occur between any two steps.  `Crash` is what ENDS an
   open transaction without completing it, which is what makes the state it
   leaves behind STABLE rather than transient -- and removing this action is
   exactly the exercise-removal the assumption table asks for. */
one sig Crash extends Action {}
/* The NON-cooperating writer.  Distinct from `hand-edit` because the two are
   distinct rows of the assumption table: `EN-11`'s exercise-removal takes away
   `hand-edit`, and TT-21.b's witness is a `foreign-write` -- so folding them
   would let `ownership`'s mutation silently take this leaf's witness with it. */
one sig ForeignWrite extends Action {}
/* THE CONCURRENCY LAYER'S OWN ACTIONS (TT-21 .. TT-23).  `Open` and `Release`
   bracket an observation; `Classify` is one reading from its listing; `Mark` is
   one rename of a bulk plan, and it acquires and consumes its own exclusive
   guard in the same step because that is what the ADR says a mutation does. */
one sig Open, Classify, Release, Mark extends Action {}
/* The recovery that settles a reserved witness.  One per class, because TT-19's
   content is that the MATCHING recovery is admitted and everything else -- a
   non-matching recovery included -- refuses. */
abstract sig Recovery extends Action {}
one sig RecoverPreparing, RecoverPublished, RecoverMigrating extends Recovery {}
fun recoveryFor: WitnessClass -> one Recovery {
  Preparing -> RecoverPreparing + Published -> RecoverPublished
    + Migrating -> RecoverMigrating
}

abstract sig Result {}
one sig Applied extends Result {}
/* the catalogue's closed refusal reasons, restricted to those a task-tree
   mutation in this slice can produce */
abstract sig Refused extends Result {}
one sig RefMalformed, RefNotAnEntry, RefNotLive, RefAlreadyTerminal,
        RefReservedKind extends Refused {}
/* The root-identity refusals.  `WitnessPending` names a witness grove CAN prove
   is its own and the operation that recovers it; `FormatLegacy` and
   `FormatForeign` are what the format classification refuses with. */
one sig RefWitnessPending, RefFormatLegacy, RefFormatForeign extends Refused {}
/* `ReservedNameOccupied(entry)` -- the OTHER half of the reserved situation
   (TT-24.b).  Split from `RefWitnessPending` because telling an operator to run
   a recovery against someone else's bytes is exactly the fail-closed violation
   the two reasons exist to keep apart: this one names the entry and NO
   recovery. */
one sig RefReservedNameOccupied extends Refused {}
/* The algebra's own refusal, opaque.  TT-10 is the claim that no ordinary
   argument reaches it, because grove's preconditions run in front. */
one sig AlgebraicRefusal extends Result {}
one sig Environmental extends Result {}
/* The catalogue's observation outcomes.  `Empty` and `Ambiguous` are SUCCESSES,
   not refusals, because that is the shipped contract and callers branch on it
   (TT-15); the whole content of that claim is that they sit here rather than
   under `Refused`. */
/* A GUARD THAT WAS HELD, and an operation that therefore has not begun.  It is
   NOT one of the catalogue's outcomes, and that is deliberate: the closed set
   covers what a completed invocation returns, and a lock wait is not a return.
   Modelling it as an ABSENT transition -- the obvious alternative -- would make
   TT-22 true by construction, since incompatible guard states would simply not
   occur, and it would break this file's totality rule besides. */
one sig Deferred extends Result {}
one sig Reported, Empty, Ambiguous extends Result {}
fun observations: set Result { Reported + Empty + Ambiguous }

one sig Sys {
  var act: one Action,
  var res: one Result,
  var tgt: lone Obj,
  /* What an observation REPORTED.  Modelled as a field rather than derived, so
     that a check restating the intended report is broken by a mutation of the
     transition — a derived value could not be got wrong. */
  var got:     set Obj,        // the entries the observation reported
  var gotTerm: set Obj,        // those of them it reported as TERMINAL (TT-16)
  /* What a `WitnessPending` refusal NAMED.  Written by the transition rather
     than derived by the reader, for the reason `got` is: a derived value could
     not be got wrong, and TT-19 is precisely the claim that the refusal carries
     the witness and its recovery. */
  var pending: lone SlotContent,
  var recov:   lone Recovery,
  /* WHAT A `ReservedNameOccupied` REFUSAL NAMED (TT-24.b).  Written by the
     transition for the reason `pending` and `got` are: a derived value could
     not be got wrong, and the claim is precisely that the refusal carries the
     entry.  It is separate from `pending` because the two reasons are separate:
     one names a class and a recovery, the other an entry and no recovery. */
  var occupant: lone Obj,
  /* WHICH GUARD THE OPERATION ASKED FOR.  Without it a DEFERRED open is
     indistinguishable from a deferred mark, and TT-22.a -- which is about
     shared opens specifically -- would have no subject. */
  var mode:    lone Mode,
  /* WHICH PROCESS ACTED.  `none` for the world's actions and for every state of
     the single-process scope. */
  var who:     lone Proc
}

/* What `ordinal-fs-tree` itself would refuse, given the argument as handed to
   it.  Assumed, not implemented: this is the imported boundary. */
pred algebraWouldRefuse[a: Action, t: Obj] {
  (a in (InsertLeaf + Decompose + Retire + Prune) and t not in entries)
  or (a = AddLeaf and t not in nodeDirs)
}

/* Byte-identical, and the witnesses are part of the tree: a refusal that left
   the format witness or the reserved slot moved would not be a refusal. */
pred noTreeChange {
  onDisk' = onDisk and nm' = nm and loc' = loc and dg' = dg
  Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
}

/* Nothing was named, because nothing that names anything was met.  `occupant`
   rides here rather than in a predicate of its own so that every transition
   already framing `pending` frames it too -- an under-framed transition is a
   latent inconsistency AND a search-space multiplier, which is the guarding
   slice's retained lesson. */
pred noPending { no Sys.pending' and no Sys.recov' and no Sys.occupant' }

/* THE ROOT-IDENTITY CASCADE every observation and mutation runs before it looks
   at its own operand (TT-18, TT-19).  Reserved witness first, then format, and
   only then anything the walk derives -- which is why `RefMalformed` is not in
   here: it is walk-derived and classifies LAST.
   A refusal here names the witness and the operation that recovers it; an
   occupant grove cannot classify names no recovery, which is the whole reason
   `WitnessPending` and `ReservedNameOccupied` are two reasons and not one. */
pred reservedRefusal {
  // AN ARTIFACT GROVE CAN PROVE IS ITS OWN: the refusal names the witness and
  // the operation that settles it.
  //
  // `some (Slot.occ & WitnessClass)` AND NOT `Slot.occ in WitnessClass`.  The
  // slot is a `lone` field, and the empty set is a subset of every set: written
  // with `in`, this antecedent is TRUE on a root with no reserved artifact at
  // all, and every ordinary transition in the file is then forced to refuse
  // `RefWitnessPending`.  It made every applied mutation unsatisfiable.
  some (Slot.occ & WitnessClass) implies {
    Sys.res' = RefWitnessPending
    Sys.pending' = Slot.occ
    Sys.recov' = recoveryFor[Slot.occ]
    no Sys.occupant'
    noTreeChange
    no inFlight'
  }
  // ONE IT CANNOT CLASSIFY AT ALL (TT-24.b): the refusal names the ENTRY, and
  // no recovery -- not the class either, because there is no class it could
  // honestly report.  Byte-identical, like every refusal in this file.
  Slot.occ = Unowned implies {
    Sys.res' = RefReservedNameOccupied
    Sys.occupant' = Slot.occAt
    no Sys.pending' and no Sys.recov'
    noTreeChange
    no inFlight'
  }
}

/* The format half.  Split from the reserved half because ROOT INITIALISATION
   runs the first and not the second: a witnessless root is what it is FOR, and
   an operation that refused there could never create one.  The split is the
   ordering of TT-18 made operational — reserved classification runs for every
   operation, format classification only for the ones that need a format. */
pred formatRefusal {
  (no Slot.occ and no Fmt.fmt) implies
    (Sys.res' = RefFormatLegacy and noTreeChange and noPending and no inFlight')
  (no Slot.occ and Fmt.fmt = ForeignFmt) implies
    (Sys.res' = RefFormatForeign and noTreeChange and noPending and no inFlight')
}

pred rootRefusal { reservedRefusal and formatRefusal }

/* Renames that the algebra performs, as name-level relations. */
pred shiftedUp[f, g: Filename] {
  g.fPos = f.fPos.plus[1] and g.fKey = f.fKey and g.fSlug = f.fSlug
  g.fKind = f.fKind and g.fOut = f.fOut and g.fSpec = f.fSpec
  g in entryName
}
pred rewritten[f, g: Filename, i: Infix] {
  g.fPos = f.fPos and g.fKey = f.fKey and g.fSlug = f.fSlug
  g.fKind = f.fKind and g.fOut = i and g.fSpec = f.fSpec
  g in entryName
}


// --- add-leaf: append -------------------------------------------------------

pred appendable[d: Obj] { d in nodeDirs }

pred doAddLeaf[d: Obj, o: Obj, f: Filename] {
  Sys.act' = AddLeaf and Sys.tgt' = d
  rootRefusal
  rootClear implies { noPending and no inFlight'
  halted implies (Sys.res' = RefMalformed and noTreeChange)
  (not halted and not appendable[d]) implies (Sys.res' = RefNotAnEntry and noTreeChange)
  (not halted and appendable[d] and f.fKind = FinishK) implies
      (Sys.res' = RefReservedKind and noTreeChange)
  (not halted and appendable[d] and f.fKind != FinishK) implies {
    Sys.res' = Applied
    o in FileObj and o not in onDisk
    f in entryName and f.fSpec = LeafSp and f.fOut = LiveI
    // TT-06.a: the append lands one past the last, closing no gap.  Stated as
    // `max + 1` rather than `#entries + 1` for the reason above; on a gapless
    // level the two agree, and gaplessness is `not halted`.
    f.fPos = (no entriesIn[d] implies 1 else plus[max[entriesIn[d].nm.fPos], 1])
    f.fKey = (no allKeys implies 1 else plus[max[allKeys], 1])   // TT-05
    onDisk' = onDisk + o
    nm' = nm ++ (o -> f)
    loc' = loc ++ (o -> d)
    Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
    some dgt: Digest | dg' = dg ++ (o -> dgt)
  }
  }
}

// --- insert-leaf: insert, shifting later siblings ---------------------------

pred doInsertLeaf[t: Obj, o: Obj, f: Filename] {
  Sys.act' = InsertLeaf and Sys.tgt' = t
  rootRefusal
  rootClear implies { noPending and no inFlight'
  halted implies (Sys.res' = RefMalformed and noTreeChange)
  (not halted and t not in entries) implies (Sys.res' = RefNotAnEntry and noTreeChange)
  (not halted and t in entries and f.fKind = FinishK) implies
      (Sys.res' = RefReservedKind and noTreeChange)
  (not halted and t in entries and f.fKind != FinishK) implies {
    Sys.res' = Applied
    let d = t.loc, p = t.nm.fPos, later = { s: entriesIn[d] | s.nm.fPos >= p } | {
      o in FileObj and o not in onDisk
      f in entryName and f.fSpec = LeafSp and f.fOut = LiveI and f.fPos = p
      f.fKey = plus[max[allKeys], 1]
      onDisk' = onDisk + o
      loc' = loc ++ (o -> d)
      o.nm' = f
      // every later sibling shifts by one; NOTHING else about it changes
      all s: later | shiftedUp[s.nm, s.nm']
      all s: onDisk - later | s.nm' = s.nm
      Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
      some dgt: Digest | dg' = dg ++ (o -> dgt)
    }
  }
  }
}

// --- decompose-leaf: promotion ---------------------------------------------

pred doDecompose[t: Obj, n: Obj, c: Obj, k: Obj, nf, kf: Filename] {
  Sys.act' = Decompose and Sys.tgt' = t
  rootRefusal
  rootClear implies { noPending and no inFlight'
  halted implies (Sys.res' = RefMalformed and noTreeChange)
  (not halted and t not in entries) implies (Sys.res' = RefNotAnEntry and noTreeChange)
  (not halted and t in entries and t.nm.fSpec = NodeSp) implies
      (Sys.res' = RefNotLive and noTreeChange)
  (not halted and t in liveLeaves) implies {
    Sys.res' = Applied
    n in DirObj and c in FileObj and k in FileObj
    n not in onDisk and c not in onDisk and k not in onDisk
    n != c and c != k and n != k
    // the node keeps the decomposed leaf's own position, key and slug (TT-08)
    nf in entryName and nf.fSpec = NodeSp
    nf.fPos = t.nm.fPos and nf.fKey = t.nm.fKey and nf.fSlug = t.nm.fSlug
    // the first child inherits the decomposed leaf's kind, at position 1
    kf in entryName and kf.fSpec = LeafSp and kf.fOut = LiveI
    kf.fPos = 1 and kf.fKind = t.nm.fKind and kf.fKey = plus[max[allKeys], 1]
    onDisk' = onDisk - t + n + c + k
    nm'  = (nm  - (t -> Filename)) ++ (n -> nf) ++ (c -> CharterF) ++ (k -> kf)
    loc' = (loc - (t -> Obj)) ++ (n -> t.loc) ++ (c -> n) ++ (k -> n)
    // the leaf's body becomes the node's charter: same bytes, new name
    Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
    some a, b: Digest |
      dg' = (dg - (t -> Digest)) ++ (c -> t.dg) ++ (n -> a) ++ (k -> b)
  }
  (not halted and t in entries and t.nm.fSpec = LeafSp and t.nm.fOut in terminalInfix)
      implies (Sys.res' = RefAlreadyTerminal and noTreeChange)
  }
}

// --- retire / prune: rewrite ------------------------------------------------

pred doRewrite[t: Obj, i: Infix, g: Filename] {
  // `i` is the mark being written, and only a TERMINAL mark can be written: a
  // rewrite to `Live` is not an operation grove has.  Said explicitly because
  // without it the action's own name is unconstrained for `i = LiveI` — the
  // predicate would then admit a rewrite calling itself an append.
  i in terminalInfix
  Sys.act' = (i = DoneI implies Retire else Prune)
  Sys.tgt' = t
  rootRefusal
  rootClear implies { noPending and no inFlight'
  halted implies (Sys.res' = RefMalformed and noTreeChange)
  (not halted and t not in entries) implies (Sys.res' = RefNotAnEntry and noTreeChange)
  (not halted and t in entries and t.nm.fSpec = NodeSp) implies
      (Sys.res' = RefNotLive and noTreeChange)
  (not halted and t in entries and t.nm.fSpec = LeafSp and t.nm.fOut in terminalInfix)
      implies (Sys.res' = RefAlreadyTerminal and noTreeChange)
  (not halted and t in liveLeaves) implies {
    Sys.res' = Applied
    rewritten[t.nm, g, i]
    onDisk' = onDisk and loc' = loc and dg' = dg
    Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
    nm' = nm ++ (t -> g)
  }
  }
}

// --- select / resolve: the observations ------------------------------------
//
// Both are TOTAL like every other action, and both are READS: a malformed tree
// stops them exactly as it stops a mutation (TT-03's rule, which the reason
// table generalises to "stops every read and mutation").  What they add to the
// vocabulary is the pair of outcomes that are SUCCESSES rather than refusals.

pred doSelect {
  Sys.act' = Select and no Sys.tgt' and noTreeChange and no Sys.gotTerm'
  rootRefusal
  (not rootClear) implies no Sys.got'
  rootClear implies { noPending and no inFlight'
  halted implies (Sys.res' = RefMalformed and no Sys.got')
  not halted implies {
    Sys.got' = selected
    no selected   implies Sys.res' = Empty        // TT-15.a
    some selected implies Sys.res' = Reported
  }
  }
}

pred doResolve[q: Query] {
  Sys.act' = Resolve and no Sys.tgt' and noTreeChange
  rootRefusal
  (not rootClear) implies (no Sys.got' and no Sys.gotTerm')
  rootClear implies { noPending and no inFlight'
  halted implies (Sys.res' = RefMalformed and no Sys.got' and no Sys.gotTerm')
  not halted implies {
    Sys.got' = matched[q]
    // TT-16: what a resolution reports about terminality, reported rather than
    // derived by the reader — a `Done` or `Abandoned` entry comes back WITH its
    // terminality, so a caller cannot read a match as liveness.
    Sys.gotTerm' = { o: matched[q] | o.nm.fOut in terminalInfix }
    no matched[q]   implies Sys.res' = Empty          // TT-15.b
    one matched[q]  implies Sys.res' = Reported
    (some matched[q] and not one matched[q]) implies Sys.res' = Ambiguous  // TT-15.c
  }
  }
}

// --- the world's own actions ------------------------------------------------

/* EN-11: any well-formed tree is reachable by hand edit.  Unconstrained beyond
   the filesystem facts, and that is the point: it is how a witness posits a
   tree grove's own actions could not build. */
pred doHandEdit {
  EN_11
  Sys.act' = HandEdit and Sys.res' = Environmental and no Sys.tgt'
  // `inFlight` is FRAMED, not cleared.  A hand edit is the world's, and the
  // world does not close grove's open transaction -- writing `no inFlight'`
  // here let one action of the world's end one of grove's, which is what
  // `cross-model-replay-k15` found by replaying entry 044's TT-20
  // counterexample into this file.
  noPending and inFlight' = inFlight
  no Sys.who' and no Sys.mode' and procAllFrame
  // `Fmt.fmt'`, `Slot.occ'` and `Slot.occAt'` are deliberately UNCONSTRAINED: a
  // witness is a file, and a hand edit reaches it exactly as it reaches any
  // other -- including by leaving someone else's bytes at a name grove reserves,
  // which is the only way `TT-24.b`'s situation arises at all (`EN-11`).
}

pred doIdle {
  Sys.act' = Idle and Sys.res' = Environmental and no Sys.tgt' and noTreeChange
  noPending and no inFlight'
  no Sys.who' and no Sys.mode' and procAllFrame
}

/* EN-06's writer, which no guard excludes.  Unconstrained on the tree exactly as
   `hand-edit` is, and it leaves every process's LISTING alone -- an operation
   that re-read the world would see it, and TT-21.b is the claim that none does. */
pred doForeignWrite {
  Sys.act' = ForeignWrite and Sys.res' = Environmental and no Sys.tgt'
  noPending and inFlight' = inFlight   // framed, for `doHandEdit`'s reason
  no Sys.who' and no Sys.mode' and procAllFrame
}


// --- root initialisation, in two steps (TT-20) ------------------------------
//
// The ONLY action in this file that is not one filesystem step, and it has to
// be: TT-20 is a claim about the state BETWEEN the two.  Folding them the way
// promotion is folded would answer the question by construction.

pred doInitScaffold[c: FileObj, l: FileObj, lf: Filename] {
  Sys.act' = InitScaffold and no Sys.tgt'
  // The RESERVED half of the cascade, and not the format half: a witnessless
  // root is what this action is for, so `RefFormatLegacy` here would make a
  // current-format root uncreatable.  A reserved witness still refuses it, and
  // TT-19 is the check that caught this action skipping the cascade entirely.
  reservedRefusal
  // The guard: nothing of grove's is here yet.  Anything else present is
  // someone's tree, and initialising over it is the fail-closed violation.
  (no Slot.occ and (some Fmt.fmt or some (onDisk - TaskRoot))) implies
    (Sys.res' = RefNotAnEntry and noTreeChange and noPending and no inFlight')
  (no Slot.occ and no Fmt.fmt and no (onDisk - TaskRoot)) implies {
    noPending
    Sys.res' = Applied
    some inFlight'                            // the transaction is now OPEN
    c != l and c not in onDisk and l not in onDisk
    lf in entryName and lf.fSpec = LeafSp and lf.fOut = LiveI and lf.fKind = OrdinaryK
    lf.fPos = 1 and lf.fKey = 1
    onDisk' = onDisk + c + l
    nm'  = nm  ++ (c -> CharterF) ++ (l -> lf)
    loc' = loc ++ (c -> TaskRoot) ++ (l -> TaskRoot)
    dg'  = dg  ++ (c -> ScaffoldD) ++ (l -> ScaffoldD)
    Fmt.fmt'  = Fmt.fmt                       // THE WITNESS DOES NOT LAND HERE
    Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
  }
}

/* The atomic same-directory rename that makes the witness visible.  One step,
   so no reader observes a torn or premature marker; reachable only from an OPEN
   transaction, so a witness never appears over a tree grove did not scaffold. */
pred doInitPublish {
  Sys.act' = InitPublish and no Sys.tgt'
  reservedRefusal
  (no Slot.occ and some inFlight and isExactScaffold) implies {
    noPending
    Sys.res' = Applied
    Fmt.fmt' = CurrentFmt
    onDisk' = onDisk and nm' = nm and loc' = loc and dg' = dg
    Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
    no inFlight'
  }
  (no Slot.occ and not (some inFlight and isExactScaffold)) implies
    (Sys.res' = RefNotAnEntry and noTreeChange and noPending and inFlight' = inFlight)
}

/* EN-08.  Interruption between any two steps.  What it does that no other
   action does is END an open transaction without completing it -- which is what
   turns the state the transaction had reached into a STABLE one an ordinary
   invocation may observe, and it is why removing this action makes TT-20's
   witness unreachable rather than merely rarer. */
pred doCrash {
  Sys.act' = Crash and Sys.res' = Environmental and no Sys.tgt' and noPending
  noTreeChange
  no inFlight'
  // EN-08.  The removal this switch performs is the exercise-removal the
  // assumption table asks for, and it is what makes TT-20's and TT-23.b's
  // witnesses unreachable rather than merely rarer.
  EN_08
  // A crashed process holds no guard, keeps no listing, and has lost its plan.
  // That is what makes the RE-RUN of a bulk mark a fresh plan over the tree the
  // interruption left, which is TT-23.b's whole content.
  procQuiet
}

/* TT-19's one exception: the MATCHING recovery is admitted while its witness is
   held, and settles it.  A non-matching recovery is not an exception at all --
   it meets the cascade like every other operation, and an occupant grove cannot
   classify has no matching recovery to be an exception for. */
pred doRecover[r: Recovery] {
  Sys.act' = r and no Sys.tgt' and no inFlight'
  let matching = (some Slot.occ and Slot.occ in WitnessClass
                  and recoveryFor[Slot.occ] = r) | {
    matching implies {
      Sys.res' = Applied
      no Slot.occ' and no Slot.occAt'
      onDisk' = onDisk and nm' = nm and loc' = loc and dg' = dg
      Fmt.fmt' = Fmt.fmt
      noPending
    }
    (not matching) implies {
      rootRefusal
      rootClear implies (Sys.res' = RefNotAnEntry and noTreeChange and noPending)
    }
  }
}

// --- the concurrency layer's transitions (TT-21 .. TT-23) -------------------

/* One rename of a bulk mark: `leaf-prune`'s `ABANDONED` infix, one entry, under
   the exclusive guard the step acquired and consumed. */
pred markRename[o: Obj, g: Filename] {
  o in onDisk
  rewritten[o.nm, g, AbandonedI]
  onDisk' = onDisk and loc' = loc and dg' = dg
  nm' = nm ++ (o -> g)
  Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ and Slot.occAt' = Slot.occAt
}

/* AN OBSERVATION TAKES ITS GUARD AND ITS ONE LISTING, in the same step: the
   listing is what the guard is FOR.  A guard that is held elsewhere does not
   refuse -- it defers, and the operation has not begun. */
pred doOpen[p: Proc, m: Mode] {
  Sys.act' = Open and Sys.who' = p and Sys.mode' = m and no Sys.tgt'
  noReport and noPending and no inFlight' and noTreeChange and procFrame[p]
  (no p.holds and compatible[p, m]) implies {
    Sys.res' = Applied
    p.holds' = m
    p.snapOn' = onDisk - TaskRoot
    p.snapNm' = (onDisk - TaskRoot) <: nm
    no p.cls' and no p.clsLive'
    p.plan' = p.plan and p.planNm' = p.planNm
  }
  (some p.holds or not compatible[p, m]) implies {
    Sys.res' = Deferred and procSelfFrame[p]
  }
}

/* ONE CLASSIFICATION, ANSWERED FROM THE ONE LISTING (TT-21).  `clsLive` is
   written by the transition rather than derived by the reader, for the reason
   `Sys.got` is: a derived answer could not be got wrong, and TT-21 is precisely
   the claim that the answer came from the listing the guard was taken with. */
pred doClassify[p: Proc, o: Obj] {
  Sys.act' = Classify and Sys.who' = p and no Sys.mode'
  noReport and noPending and no inFlight' and noTreeChange and procFrame[p]
  (some p.holds) implies {
    Sys.res' = Applied and Sys.tgt' = o
    p.cls' = p.cls + o
    p.clsLive' = p.clsLive + (liveInSnap[p, o] implies o else none)
    p.holds' = p.holds and p.snapOn' = p.snapOn and p.snapNm' = p.snapNm
    p.plan' = p.plan and p.planNm' = p.planNm
  }
  (no p.holds) implies {
    Sys.res' = Deferred and no Sys.tgt' and procSelfFrame[p]
  }
}

/* The operation ends and drops its guard.  Its listing and its classifications
   go with it; its PLAN does not, because a plan outlives the guard it was
   validated under and that is the whole of `bulk-marks-are-not-atomic`. */
pred doRelease[p: Proc] {
  Sys.act' = Release and Sys.who' = p and no Sys.mode' and no Sys.tgt'
  noReport and noPending and no inFlight' and noTreeChange and procFrame[p]
  (some p.holds) implies {
    Sys.res' = Applied
    no p.holds' and no p.snapOn' and no p.snapNm' and no p.cls' and no p.clsLive'
    p.plan' = p.plan and p.planNm' = p.planNm
  }
  (no p.holds) implies { Sys.res' = Deferred and procSelfFrame[p] }
}

/* ONE MARK OF A BULK PRUNE, and it is one whole critical section: it acquires
   the exclusive guard, renames one entry, and CONSUMES the guard again, which
   is what `bulk-marks-are-not-atomic` records a mutating method doing.  Three
   branches, and each is an obligation:
   - the PLANNING mark validates the whole plan against ONE listing and only
     then takes the first rename (TT-23.a).  An invalid member refuses with
     nothing renamed.
   - a LATER mark of the same plan renames the next member (TT-23), and the plan
     it works from is the one the first guard's listing validated.
   - a mark with no plan and nothing live to plan reports `Empty` and changes
     nothing, which is the state a completed -- or a RE-RUN and already
     converged -- prune reaches (TT-23.b).
   EN-07 lives in the acquisition test: two open descriptions of one directory
   do not share a lock, so a process that already holds its own outer guard
   DEADLOCKS here rather than nesting.  That is the option
   `bulk-marks-are-not-atomic` rejects, and breaking the assumption is what
   makes it reachable. */
pred doMark[p: Proc, o: Obj, g: Filename] {
  Sys.act' = Mark and Sys.who' = p and Sys.mode' = Exclusive
  noReport and noPending and no inFlight' and procFrame[p]
  let acquired = (compatible[p, Exclusive] and (EN_07 implies no p.holds)) | {
    (not acquired) implies {
      Sys.res' = Deferred and no Sys.tgt' and noTreeChange and procSelfFrame[p]
    }
    acquired implies {
      (no p.plan) implies {
        (not planValid) implies {
          Sys.res' = RefReservedKind and no Sys.tgt' and noTreeChange
          procSelfFrame[p]
        }
        (planValid and no liveLeaves) implies {
          Sys.res' = Empty and no Sys.tgt' and noTreeChange and procSelfFrame[p]
        }
        (planValid and some liveLeaves) implies {
          Sys.res' = Applied and Sys.tgt' = o
          o in liveLeaves
          markRename[o, g]
          // THE PLAN AND ITS LISTING, and nothing else.  `planNm` records what
          // this guard's listing showed for every member the run will work
          // through, taken before the first rename and outliving the guard that
          // took it.  A mark touches NO observation field: the two lifetimes are
          // disjoint, which is the whole of what TT-21.b's counterexample taught.
          p.plan' = liveLeaves - o
          p.planNm' = (liveLeaves - o) <: nm
          p.holds' = p.holds and p.snapOn' = p.snapOn and p.snapNm' = p.snapNm
          p.cls' = p.cls and p.clsLive' = p.clsLive
        }
      }
      (some p.plan) implies {
        (no (p.plan & liveLeaves)) implies {
          // The run is over: nothing it planned is live any more, which is what
          // a RE-RUN of an already-converged prune reaches (TT-23.b).
          Sys.res' = Empty and no Sys.tgt' and noTreeChange
          no p.plan' and no p.planNm'
          p.holds' = p.holds and p.snapOn' = p.snapOn and p.snapNm' = p.snapNm
          p.cls' = p.cls and p.clsLive' = p.clsLive
        }
        (some (p.plan & liveLeaves)) implies {
          Sys.res' = Applied and Sys.tgt' = o
          o in p.plan & liveLeaves
          markRename[o, g]
          p.plan' = p.plan - o
          p.planNm' = (p.plan - o) <: p.planNm
          p.holds' = p.holds and p.snapOn' = p.snapOn and p.snapNm' = p.snapNm
          p.cls' = p.cls and p.clsLive' = p.clsLive
        }
      }
    }
  }
}

/* The concurrent scope's step.  The world's actions are here too: `EN-06`
   grants only that COOPERATING processes are serialized, so `hand-edit` and
   `foreign-write` land at any point during an operation and no guard excludes
   them. */
pred concStep {
  noReport and (
    doIdle or doHandEdit or doForeignWrite or doCrash
    or (some p: Proc, m: Mode | doOpen[p, m])
    or (some p: Proc, o: Obj | doClassify[p, o])
    or (some p: Proc | doRelease[p])
    or (some p: Proc, o: Obj, g: Filename | doMark[p, o, g])
  )
}

/* The fresh objects an operation introduces are quantified at their SPECIES
   rather than over `Obj`.  It changes no meaning — the bodies already require
   it — and it is the difference between `5^4` and `2 * 3 * 3` combinations for
   the promotion disjunct, which is most of what the solver spends its time on.
   The TARGET stays `Obj`, because an action must be total over what an operator
   can name: a node handed to `retire` has to reach its refusal. */
pred noReport { no Sys.got' and no Sys.gotTerm' }

pred ordinaryStep {
  (noReport and (doIdle
    or doHandEdit
    or doCrash
    or (some d: Obj, o: FileObj, f: Filename | doAddLeaf[d, o, f])
    or (some t: Obj, o: FileObj, f: Filename | doInsertLeaf[t, o, f])
    or (some t: Obj, n: DirObj, disj c, k: FileObj, nf, kf: Filename |
          doDecompose[t, n, c, k, nf, kf])
    or (some t: Obj, i: Infix, g: Filename | doRewrite[t, i, g])
    or (some disj c, l: FileObj, lf: Filename | doInitScaffold[c, l, lf])
    or (some r: Recovery | doRecover[r])))
  or doSelect
  or (some q: Query | doResolve[q])
}

/* An OPEN transaction admits only its own next step, an interruption, OR THE
   WORLD.  The first three are what makes `some inFlight` transient in the
   catalogue's sense: no ordinary INVOCATION runs while it holds, so no ordinary
   invocation observes it.  The world is not an invocation and no guard excludes
   it -- `EN-06` serializes only COOPERATING processes and `EN-13` grants that a
   foreign entry may appear at any name -- which is exactly what `concStep`'s own
   comment says two predicates above.
   Excluding it here is what kept this file from reaching entry 044's TT-20
   counterexample; `cross-model-replay-k15` found it by replaying that
   counterexample, and `run TT_20_replay_a_world_write_during_an_open_scaffold`
   below is the situation the exclusion had made unreachable.
   `doForeignWrite` stays behind `Concurrent`, exactly as it is in `step`'s
   other branch: it is the NON-COOPERATING writer, and the single-process scope
   reaches the world through `doHandEdit`. */
pred step {
  some inFlight implies (noReport and procQuiet and
                         (doInitPublish or doCrash or doHandEdit
                          or (Concurrent and doForeignWrite)))
  no inFlight   implies (SingleProc implies (procQuiet and ordinaryStep)
                                    else concStep)
}

fact Trace {
  Sys.act = Idle and Sys.res = Environmental and no Sys.tgt
  no Sys.got and no Sys.gotTerm
  no Sys.pending and no Sys.recov and no Sys.occupant
  no Sys.who and no Sys.mode
  all p: Proc | no p.holds and no p.snapOn and no p.snapNm
                and no p.cls and no p.clsLive and no p.plan and no p.planNm
  no inFlight
  // EN-11's OTHER HALF.  With the assumption in force this says nothing and the
  // initial tree is free, which is what every witness in this file rests on;
  // with it removed the world starts as grove would have found it before it had
  // done anything -- an empty task root, no witness, nothing at a reserved name.
  not EN_11 implies (onDisk = TaskRoot and no Fmt.fmt and no Slot.occ)
  always step
}

/* A grove action, as against the world's. */
fun groveActs: set Action { AddLeaf + InsertLeaf + Decompose + Retire + Prune }
/* The observations, which are reads: no `groveAct` names one, so every command
   written about mutation is unchanged by their arrival. */
fun observeActs: set Action { Select + Resolve }
/* The root's own actions, kept OUT of `groveActs` so that every command written
   about tree mutation before this layer arrived is unchanged by it. */
fun rootActs: set Action { InitScaffold + InitPublish + Recovery }
pred groveStep { Sys.act' in groveActs }


// ===========================================================================
// CLAIMS — TT-01 .. TT-10
//
// Each command names the obligation it answers and the law bundle it assumes.
// A `check` must find no counterexample; a `witness_` run must find an
// instance, and it is what keeps the check from passing for want of a reachable
// situation.
//
// WHY EVERY BEHAVIOURAL COMMAND RUNS AT `3 steps` AND NOT `2`.  An Alloy 6 trace
// is a lasso, so the last state must loop.  A state reached by a tree-changing
// action cannot loop back to the initial idle state — the tree differs — and
// cannot loop to itself either, since repeating the action would change the tree
// again.  At `2 steps` NO applied mutation exists at all, and every check
// conditioned on `Sys.res' = Applied` is vacuously true.  Three states is the
// minimum that admits one mutation followed by a stutter; the vacuity guard,
// which needs two mutations, runs at four.  The purely static name claims
// (`TT-01`, and the classification witnesses) stay at `1 steps` because they are
// about one state and nothing else.
// ===========================================================================

// --- TT-01: a name has exactly one spelling --------------------------------

/* TT-01.a.  Under the canonical grammar, no two accepted spellings share a
   reading — so two on-disk entries can never BE one entry. */
check TT_01a_distinct_filenames_never_denote_one_entry {
  GroveGrammar and CurrentRootThroughout implies
    (all disj f, g: entryName | not denotesSame[f, g])
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* DEFECT, without the ADR's rule.  Two spellings both parse, so two files in
   one directory are the same entry: same key, same position, same everything. */
run witness_TT_01a_two_spellings_would_both_parse {
  StatedGrammar and CurrentRootThroughout and EN_12
  some disj a, b: entries | a.nm != b.nm and sameReading[a.nm, b.nm]
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* TT-01.b.  Parse-then-render reproduces the input exactly, and any other
   spelling of the same reading is refused with the canonical one to hand. */
check TT_01b_a_noncanonical_spelling_is_refused_naming_the_canonical_one {
  GroveGrammar and CurrentRootThroughout implies {
    all f: entryName | f.canon = f
    all f: shaped | (f.canon != f and (f.fSpec = NodeSp or f.fKind in known)) implies
      (f in malformedName and f.canon in entryName and sameReading[f, f.canon])
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

run witness_TT_01b_a_noncanonical_spelling_refused {
  GroveGrammar and CurrentRootThroughout
  some o: visited - TaskRoot | o.nm in malformedName and o.nm.canon in entryName
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-02: a name declares its species and must be it ---------------------

/* TT-02.a.  A leaf name at a directory is MALFORMED — read as work and then
   refused — rather than foreign, which would be skipped.  The distinction is
   the whole claim: a skipped directory takes its live subtree with it. */
check TT_02a_a_leaf_name_at_a_directory_is_malformed_not_foreign {
  GroveGrammar and CurrentRootThroughout implies always (
    (some o: (visited & DirObj) - TaskRoot | o.nm in entryName and o.nm.fSpec = LeafSp)
      implies (rSpeciesMismatch and halted
               and (Sys.act' in groveActs implies (Sys.res' = RefMalformed and noTreeChange))))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_02a_leaf_name_at_a_directory {
  GroveGrammar and CurrentRootThroughout
  some o: (visited & DirObj) - TaskRoot | o.nm in entryName and o.nm.fSpec = LeafSp
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* TT-02.b.  The converse: a node name at a file. */
check TT_02b_a_node_name_at_a_file_is_malformed_not_foreign {
  GroveGrammar and CurrentRootThroughout implies always (
    (some o: (visited & FileObj) | o.nm in entryName and o.nm.fSpec = NodeSp)
      implies (rSpeciesMismatch and halted
               and (Sys.act' in groveActs implies (Sys.res' = RefMalformed and noTreeChange))))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_02b_node_name_at_a_file {
  GroveGrammar and CurrentRootThroughout
  some o: visited & FileObj | o.nm in entryName and o.nm.fSpec = NodeSp
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-03: malformed halts, and never skips -------------------------------

/* The one check whose antecedent is the WHOLE halting condition rather than one
   action, so the solver must exhaust every halted tree against every action.
   It runs one filename short of the others — 5 rather than 6 — because that one
   atom is the difference between 68 seconds and not finishing in three minutes.
   The bound is recorded rather than hidden: at 5 filenames a three-entry tree
   with a rename in flight is expressible, which is what the claim needs. */
check TT_03_malformed_halts_and_never_skips {
  GroveGrammar and CurrentRootThroughout implies always (
    halted implies (Sys.act' in groveActs implies (Sys.res' = RefMalformed and noTreeChange)))
} for 3 but 4 Int, 3 FileObj, 2 DirObj, 5 Filename, 2 Slug, 2 Digest, 3 steps

/* The case where SKIPPING would report a finished grove, and the `visited` rule
   sharpens it: a malformed node directory is not descended into, so the live
   work inside it is invisible to the walk — `liveLeaves` is empty and a reader
   that merely skipped the directory would call the grove done.  What prevents
   that is not the walk but the halt: the directory is itself an entry at its
   parent's level, its name is malformed, and the whole tree stops. */
run witness_TT_03_a_malformed_node_hides_live_work {
  GroveGrammar and CurrentRootThroughout
  some d: (visited & DirObj) - TaskRoot | {
    d.nm in malformedName
    some o: kidsOf[d] | o.nm in entryName and o.nm.fSpec = LeafSp and o.nm.fOut = LiveI
  }
  no liveLeaves
  halted
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-04: foreign entries are ignored and preserved ----------------------

check TT_04_foreign_entries_are_ignored_and_preserved {
  GroveGrammar and CurrentRootThroughout implies {
    always no (foreignEntries & entries)                    // never read as work
    always ((Sys.act' in groveActs and Sys.res' = Applied) implies
      (all o: foreignEntries | o in onDisk' and o.nm' = o.nm and o.loc' = o.loc and o.dg' = o.dg))
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* THE COUNTEREXAMPLE THAT PRODUCED THE `visited` RULE, kept as a witness.  A
   canonical, known-kind, perfectly parseable task name sitting inside a FOREIGN
   directory is not work: not an entry, not on any level grove orders, and its
   key is not in the counter. */
run witness_TT_04_a_task_name_under_a_foreign_directory_is_not_work {
  GroveGrammar and CurrentRootThroughout
  some d: (onDisk & DirObj) - TaskRoot, o: onDisk |
    d.nm in foreignName and o.loc = d and o.nm in entryName
    and o not in entries and o.nm.fKey not in allKeys
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

run witness_TT_04_foreign_survives_a_sibling_rename {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = InsertLeaf and Sys.res' = Applied and
    (some o: foreignEntries | some s: entriesIn[o.loc] | s.nm' != s.nm and o.nm' = o.nm))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-05: keys are unique, permanent and never reissued ------------------

/* One command per action rather than one over `groveActs`.  The claim is the
   same conjunction four times; what changes is that each antecedent pins the
   transition, and the broad version did not finish in three minutes at this
   bound while each narrow one does.  Narrowing the ANTECEDENT rather than the
   bound is the trade to prefer: a smaller bound buys the green run at the cost
   of what the run was evidence about. */
pred keysArePermanent {
  allKeys in allKeys'                                    // no key is withdrawn
  all k: allKeys' - allKeys | all j: allKeys | k > j     // a fresh key tops them all
}

check TT_05_keys_never_reissued_on_append {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = AddLeaf and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_05_keys_never_reissued_on_insert {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The standard bound rather than the promotion witnesses' wider one: three
   files and two directories is exactly what a promotion needs — the leaf, its
   charter, its first child, the task root and the new node — and the wider
   bound did not finish in five minutes. */
check TT_05_keys_never_reissued_on_promotion {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Decompose and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_05_keys_never_reissued_on_rewrite {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' in (Retire + Prune) and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The witness the catalogue asks for: an allocation whose maximum comes from a
   TERMINAL entry — which is why retirement is a rename and never a removal. */
run witness_TT_05_allocation_max_comes_from_a_terminal_entry {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied and
    (some t: entries | t.nm.fOut in terminalInfix and t.nm.fKey = max[allKeys]))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-06: positions are per-directory and gapless ------------------------

check TT_06a_append_lands_at_n_plus_one_and_closes_no_gap {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = AddLeaf and Sys.res' = Applied) implies {
      all o: onDisk' - onDisk | {
        all s: entriesIn[o.loc'] | s.nm.fPos < o.nm'.fPos
        after gaplessAt[o.loc]          // the level it landed on, not every level
      }
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_06a_append_lands_at_n_plus_one {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied and
    (some o: onDisk' - onDisk | o.nm'.fPos > 1
       and (all s: entriesIn[o.loc'] | s.nm.fPos < o.nm'.fPos)))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_06b_insert_shifts_every_later_sibling {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies
      (let t = Sys.tgt', d = t.loc, p = t.nm.fPos | {
         all s: entriesIn[d] | s.nm.fPos >= p implies s.nm'.fPos = plus[s.nm.fPos, 1]
         all s: entriesIn[d] | s.nm.fPos <  p implies s.nm' = s.nm
         after gaplessAt[d]             // the level it shifted, not every level
       }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_06b_insert_at_an_occupied_position_shifts {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = InsertLeaf and Sys.res' = Applied and
    (some s: entriesIn[Sys.tgt'.loc] | s.nm.fPos >= Sys.tgt'.nm.fPos and s != Sys.tgt'))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-07: a shift preserves everything but position ----------------------

check TT_07_a_shift_changes_only_positions {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies
      (all s: onDisk | {
         s in onDisk'
         s.dg'  = s.dg                     // never any file's bytes
         s.loc' = s.loc
         s.nm'.fKey  = s.nm.fKey  and s.nm'.fSlug = s.nm.fSlug
         s.nm'.fKind = s.nm.fKind and s.nm'.fOut  = s.nm.fOut
         s.nm'.fSpec = s.nm.fSpec
       }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* A shift across a directory holding every species the level can carry: a live
   leaf, a terminal leaf, a node, and a foreign entry. */
/* The widest bound in the file, and the arithmetic is why: the level must carry
   a node (a directory), the node's own charter (a file, one level down), a
   terminal leaf, a foreign entry and the shift's target — five files and two
   directories before the insert adds its own. */
run witness_TT_07_shift_across_every_species {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = InsertLeaf and Sys.res' = Applied and
    (let d = Sys.tgt'.loc | {
       some s: entriesIn[d] | s.nm.fSpec = NodeSp
       some s: entriesIn[d] | s.nm.fSpec = LeafSp and s.nm.fOut in terminalInfix
       some s: foreignEntries | s.loc = d
     }))
} for 6 but 4 Int, 5 FileObj, 2 DirObj, 8 Filename, 2 Slug, 3 Digest, 3 steps

// --- TT-08: decomposition preserves the key --------------------------------

check TT_08_decomposition_preserves_the_key {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Decompose and Sys.res' = Applied) implies
      (let t = Sys.tgt' | {
         some n: onDisk' - onDisk | n.nm'.fKey = t.nm.fKey and n.nm'.fPos = t.nm.fPos
                                    and n.nm'.fSlug = t.nm.fSlug and n.nm'.fSpec = NodeSp
         allKeys in allKeys'
         all s: onDisk - t | s.nm' = s.nm and s.loc' = s.loc and s.dg' = s.dg
       }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_08_promotion_of_the_maximum_key {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Decompose and Sys.res' = Applied and
              Sys.tgt'.nm.fKey = max[allKeys])
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-09: one algebraic operation plus a domain precondition -------------

check TT_09a_append_adds_exactly_one_entry_and_renames_nothing {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = AddLeaf and Sys.res' = Applied) implies {
      one onDisk' - onDisk
      no  onDisk - onDisk'
      all s: onDisk | s.nm' = s.nm and s.loc' = s.loc and s.dg' = s.dg
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09a_append { GroveGrammar and CurrentRootThroughout and eventually (Sys.act' = AddLeaf and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_09b_insert_adds_exactly_one_entry_and_removes_none {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies {
      one onDisk' - onDisk
      no  onDisk - onDisk'
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09b_insert { GroveGrammar and CurrentRootThroughout and eventually (Sys.act' = InsertLeaf and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_09c_promotion_replaces_exactly_the_target {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Decompose and Sys.res' = Applied) implies {
      onDisk - onDisk' = Sys.tgt'
      #(onDisk' - onDisk) = 3          // the node, its charter, its first child
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09c_promotion { GroveGrammar and CurrentRootThroughout and eventually (Sys.act' = Decompose and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_09d_rewrite_renames_exactly_one_entry {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' in (Retire + Prune) and Sys.res' = Applied) implies {
      onDisk' = onDisk
      one s: onDisk | s.nm' != s.nm
      all s: onDisk | s.loc' = s.loc and s.dg' = s.dg
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09d_rewrite { GroveGrammar and CurrentRootThroughout and eventually (Sys.act' in (Retire + Prune) and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-10: no algebraic refusal reaches an operator -----------------------

check TT_10_no_algebraic_refusal_reaches_an_operator {
  GroveGrammar and CurrentRootThroughout implies always {
    Sys.res' != AlgebraicRefusal
    (Sys.act' in groveActs and algebraWouldRefuse[Sys.act', Sys.tgt'])
      implies Sys.res' in Refused
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* An argument the algebra itself would have refused, shown pre-empted by
   grove's own precondition: the operator sees a refusal this catalogue names. */
run witness_TT_10_an_algebraic_refusal_is_preempted {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' in groveActs and algebraWouldRefuse[Sys.act', Sys.tgt']
              and Sys.res' in Refused)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps


// ===========================================================================
// CLAIMS — TT-11 .. TT-16, the selection scope
//
// WHY THESE COMMANDS RUN AT `2 steps` AND THE MUTATION ONES AT `3`.  The lasso
// argument in the header is about a TREE-CHANGING action: the state it reaches
// can loop to neither the initial state nor itself.  An observation changes
// nothing, so the state it reaches loops to ITSELF — repeat the observation on
// an unchanged tree and every component of the state recurs.  Two states is
// therefore the minimum that admits an applied observation, and the witnesses
// below are what prove that rather than assume it.  `TT-14` is the exception:
// it needs two observations with a hand edit between them, which is four.
// ===========================================================================

// --- TT-11: selection is a stateless pre-order walk ------------------------

/* The walk returns the `precedes`-minimal eligible leaf, and returns exactly
   one when there is one to return.  `lone Sys.got'` is not a formality: the
   minimum is unique only because `precedes` is total on a well-formed tree, and
   THAT holds only because gaplessness makes sibling positions distinct.  Drop
   gaplessness from `halted` and this check finds two minima. */
check TT_11_selection_is_the_first_eligible_leaf_in_preorder {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Select and not halted) implies {
      noTreeChange
      Sys.got' in liveLeaves
      some eligible implies one Sys.got'
      all o: Sys.got' | no p: eligible - o | precedes[p, o]
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* The catalogue's witness: a selection that descends a node before visiting a
   later sibling.  Pre-order and not breadth-first, and not "shallowest first". */
run witness_TT_11_selection_descends_a_node_before_a_later_sibling {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Select and Sys.res' = Reported and
    (some o: Sys.got', n: entries | {
       o.loc = n
       n.nm.fSpec = NodeSp
       some s: entriesIn[n.loc] | s in liveLeaves and s.nm.fPos > n.nm.fPos
     }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

// --- TT-12: terminal entries are skipped, never removed --------------------

check TT_12_terminal_entries_are_skipped_never_removed {
  GroveGrammar and CurrentRootThroughout implies always {
    (Sys.act' = Select and Sys.res' = Reported) implies
      (all o: Sys.got' | o.nm.fOut = LiveI)
    // skipping is not deletion: no grove action takes a terminal entry off disk.
    // (`hand-edit` is the world's, and EN-11 is what makes it unconstrained.)
    Sys.act' in groveActs implies
      (all o: entries | o.nm.fOut in terminalInfix implies o in onDisk')
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* A walk crossing a WHOLLY terminal node: the node holds entries, none of them
   live, it precedes the selection, and it is still on disk. */
run witness_TT_12_the_walk_crosses_a_wholly_terminal_node {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Select and Sys.res' = Reported and
    (some n: entries, o: Sys.got' | {
       n.nm.fSpec = NodeSp
       some (entries & n.^(~loc))
       no (liveLeaves & n.^(~loc))
       precedes[n, o]
     }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

// --- TT-13: finish is reserved, not blocking -------------------------------

check TT_13a_a_live_finish_leaf_is_skipped_while_ordinary_work_is_live {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Select and not halted and some liveOrdinary) implies
      no (Sys.got' & liveFinish))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* The case where the skip rule is the ONLY thing preventing teardown: the
   finish leaf sits at an earlier position than live ordinary work, so a walk
   that merely took the first live leaf would return it. */
run witness_TT_13a_a_finish_leaf_earlier_than_live_work_is_skipped {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Select and Sys.res' = Reported and
    (some fl: liveFinish, o: Sys.got' | precedes[fl, o]))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

check TT_13b_the_finish_leaf_is_returned_when_it_is_the_only_live_leaf {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Select and not halted and no liveOrdinary and some liveFinish)
      implies (Sys.res' = Reported and one Sys.got' and Sys.got' in liveFinish))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

run witness_TT_13b_the_finish_leaf_is_the_only_live_leaf {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Select and Sys.res' = Reported and
              Sys.got' in liveFinish and some terminalInfix & entries.nm.fOut)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* TT-13.c.  The reason classifies the TREE.  What makes that a claim rather
   than a restatement is the second conjunct: the tree halts even when no
   entry-local reason holds — both finish leaves are individually well formed,
   and there is nothing to name but the tree. */
check TT_13c_two_live_finish_leaves_malform_the_whole_tree {
  GroveGrammar and CurrentRootThroughout implies always (
    rMultipleLiveFinish implies {
      halted
      (Sys.act' in groveActs + observeActs) implies
        (Sys.res' = RefMalformed and noTreeChange and no Sys.got' and no Sys.gotTerm')
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* Two individually well-formed live finish leaves in DIFFERENT subtrees, and no
   other malformity anywhere: nothing is wrong with either entry. */
run witness_TT_13c_two_well_formed_live_finish_leaves_in_different_subtrees {
  GroveGrammar and CurrentRootThroughout
  rMultipleLiveFinish
  not (rMalformedEntry or rSpeciesMismatch or rPositionsNotGapless
       or rKeyReissued or rNodeWithoutCharter)
  some disj a, b: liveFinish | a.loc != b.loc
  eventually (Sys.act' = Select and Sys.res' = RefMalformed)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

// --- TT-14: selection is not a scheduler -----------------------------------

/* Among eligible SIBLINGS, position decides and nothing else does — not the
   key, not the slug, not the kind, not the order they were created in.  The
   mutation that breaks it is a walk that prefers the smallest key. */
check TT_14_position_and_terminality_are_the_only_mechanisms {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Select and Sys.res' = Reported) implies
      (all o: Sys.got', p: eligible - o | p.loc = o.loc implies o.nm.fPos < p.nm.fPos))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* The catalogue's witness, and the only command here that needs four states:
   TWO ORDERINGS OF THE SAME WORK SELECTING DIFFERENTLY.  Select, hand-edit the
   positions of two live leaves while keeping every other part of both names,
   select again — and a different leaf comes back.  Nothing but the order moved. */
run witness_TT_14_two_orderings_of_the_same_work_select_differently {
  GroveGrammar and CurrentRootThroughout
  some disj a, b: FileObj |
    eventually {
      Sys.act = Select and Sys.res = Reported and Sys.got = a
      liveLeaves = a + b
      Sys.act' = HandEdit
      a.loc' = a.loc and b.loc' = b.loc
      a.nm'.fKey  = a.nm.fKey  and b.nm'.fKey  = b.nm.fKey
      a.nm'.fSlug = a.nm.fSlug and b.nm'.fSlug = b.nm.fSlug
      a.nm'.fKind = a.nm.fKind and b.nm'.fKind = b.nm.fKind
      a.nm'.fOut  = a.nm.fOut  and b.nm'.fOut  = b.nm.fOut
      a.nm'.fSpec = a.nm.fSpec and b.nm'.fSpec = b.nm.fSpec
      a.nm'.fPos != a.nm.fPos
      after after (Sys.act = Select and Sys.res = Reported and Sys.got = b)
    }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps

// --- TT-15: an empty or ambiguous observation is a SUCCESS -----------------
//
// The whole content is that `Empty` and `Ambiguous` are in `observations` and
// not in `Refused`, that the tree is untouched, and that the three outcomes are
// told apart by the reported value alone.

check TT_15a_selection_on_a_spent_tree_reports_empty {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Select and not halted) implies {
      noTreeChange
      Sys.res' in observations                 // never a refusal
      (no eligible)      iff (Sys.res' = Empty)
      (Sys.res' = Empty) iff (no Sys.got')
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* A SPENT tree, not an empty one: entries exist and every one of them is
   terminal.  An empty tree would satisfy the claim for the wrong reason. */
run witness_TT_15a_a_spent_tree_reports_empty {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Select and Sys.res' = Empty and
              some entries and no liveLeaves)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

check TT_15b_a_resolution_matching_nothing_reports_empty {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Resolve and not halted) implies {
      noTreeChange
      Sys.res' in observations
      (no Sys.got')      iff (Sys.res' = Empty)
      (some Query.qKey  implies (all o: Sys.got' | o.nm.fKey  = Query.qKey))
      (some Query.qSlug implies (all o: Sys.got' | o.nm.fSlug = Query.qSlug))
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* Matching nothing on a POPULATED tree — the reference is well formed and simply
   names no entry. */
run witness_TT_15b_a_resolution_matching_nothing_reports_empty {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Resolve and Sys.res' = Empty and some entries)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

check TT_15c_a_resolution_matching_several_reports_ambiguous {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Resolve and not halted) implies {
      noTreeChange
      (one Sys.got')  iff (Sys.res' = Reported)
      (not lone Sys.got') iff (Sys.res' = Ambiguous)
      Sys.res' = Ambiguous implies Sys.got' in entries
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* Several matches from a bare SLUG: a key resolves at most one entry, so this
   outcome exists only because a slug is not an identity. */
run witness_TT_15c_a_bare_slug_matching_several_reports_ambiguous {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Resolve and Sys.res' = Ambiguous and no Query.qKey)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

// --- TT-16: a resolved terminal entry is never mistaken for live -----------

check TT_16a_a_resolved_done_entry_is_reported_terminal {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Resolve and Sys.res' in (Reported + Ambiguous)) implies {
      all o: Sys.got' | o.nm.fOut = DoneI implies o in Sys.gotTerm'
      no (Sys.gotTerm' & liveLeaves)
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

run witness_TT_16a_a_resolved_done_entry {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Resolve and Sys.res' = Reported and
    (some o: Sys.got' | o.nm.fOut = DoneI and o in Sys.gotTerm'))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

check TT_16b_a_resolved_abandoned_entry_is_reported_terminal {
  GroveGrammar and CurrentRootThroughout implies always (
    (Sys.act' = Resolve and Sys.res' in (Reported + Ambiguous)) implies {
      all o: Sys.got' | o.nm.fOut = AbandonedI implies o in Sys.gotTerm'
      no (Sys.gotTerm' & liveLeaves)
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

run witness_TT_16b_a_resolved_abandoned_entry {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = Resolve and Sys.res' = Reported and
    (some o: Sys.got' | o.nm.fOut = AbandonedI and o in Sys.gotTerm'))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

// ===========================================================================
// CLAIMS — TT-17 .. TT-20, the root-identity scope
//
// WHAT SEPARATES THESE FROM EVERYTHING ABOVE.  `TT-01` .. `TT-16` are stated
// over a tree grove may act on at all, which is what `CurrentRootThroughout`
// says out loud.  These four are stated over the roots grove may NOT act on —
// so none of them assumes that bundle, and `rootState` rather than `halted` is
// the thing they read.
//
// WHY `TT-20` NEEDS FOUR STATES AND THE OTHER THREE NEED ONE OR TWO.  Three of
// the four are claims about ONE state — a classification is a function of the
// state it classifies — so they run at `1 steps`, exactly as `TT-01`'s do.
// `TT-19` needs THREE, and the reason is the lasso argument in the README met
// from a new direction.  Most of TT-19 is refusals, and a refusal changes
// nothing, so two states would do — but its exception clause is about the
// MATCHING RECOVERY, and a recovery that settles a witness is a tree change.  At
// `2 steps` no applied recovery exists, that clause is vacuous, and the mutation
// aimed at it SURVIVES: the check reports green exactly as it would if the
// mutation had been caught and forgiven.  `TT-20` is the first claim in this scope about an action
// INTERRUPTED PART-WAY — scaffold, crash, and the stable state the crash left
// behind — and that is THREE states, not more: the README's `3 steps` argument
// reaches it after all, and the witness is what shows so, finding an instance
// at 3 and none at 2.  Its CHECK runs at 4, because the check is the thing that
// wants room the witness does not: a fourth state admits an initialisation
// followed by an ordinary mutation, which is where a premature witness would
// have somewhere to appear.
// ===========================================================================

// --- TT-17: a format decision reads the witness, a witnessless one reads bytes

/* TT-17 WAS ONE CLAIM AND IS NOW TWO, and the split is `task-tree-scope-k70`'s
   disposition of the narrowing this file used to declare.  The one-sentence
   form -- "the classification SHALL depend only on the format witness, never on
   any task entry's text" -- is FALSE of the catalogue's own state table, which
   decides a witnessless root by an exact comparison against a task entry's name
   AND bytes, and doubly false since that decision gained a second branch.  Both
   families narrowed the check to the Current/Legacy/Foreign decision and
   declared it; the narrowing was right and the text was wrong.

   `TT-17.a` is that decision, unchanged in force.  Two conjuncts, and the
   second is the falsifiable one: the first says which family each witness
   content lands in, which a classification that read a task entry's text would
   still satisfy on the tree it was tuned for; the second is what a hand edit
   cannot do -- change every name in the tree, leave the witness alone, and the
   root does not move between families. */
check TT_17a_format_is_decided_by_the_witness_content_alone {
  (GroveGrammar and SingleProc) implies always {
    no Slot.occ implies {
      (no Fmt.fmt)           iff (rootState in (LegacyR + scaffoldFamily))
      (Fmt.fmt = ForeignFmt) iff (rootState = ForeignR)
      (Fmt.fmt = CurrentFmt) iff (rootState in (MalformedR + currentFamily))
    }
    (Sys.act' = HandEdit and Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ)
      implies familyOf[rootState'] = familyOf[rootState]
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* `TT-17.b` -- the WITNESSLESS decision reads BYTES and not only names.  A root
   whose entries carry none of the scaffold's own bytes is `Legacy` however its
   entries are spelled, which is the shipped
   `a_legacy_v2_slug_beginning_with_requirements_is_not_partial_root_init`: a
   legacy slug that happens to read as a current session kind is evidence of
   nothing.  A scaffold test written over names alone dies on this.

   IT WAS FIRST WRITTEN AS A PERTURBATION -- *a hand edit that leaves the digests
   alone never moves a root INTO a scaffold classification* -- AND THIS TOOL
   REFUTED IT, in the first command run against the draft.  A rename is such an
   edit, and a file already holding the scaffold's exact bytes under some other
   name BECOMES the scaffold leaf when renamed to the scaffold leaf's name --
   correctly, since grove cannot and must not tell it from the one its own
   initialisation would have written.  The perturbation form asserted something
   about renames when the claim is about what is CONSULTED.  The refutation is
   recorded in the catalogue beside the claim. */
check TT_17b_a_witnessless_decision_reads_bytes_not_only_names {
  (GroveGrammar and SingleProc) implies always {
    (no Fmt.fmt and no Slot.occ
     and some (onDisk - TaskRoot)
     and no ((onDisk - TaskRoot) & dg.ScaffoldD))
      implies rootState not in scaffoldFamily
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* `TT-17.b`'s witness: a witnessless root whose entry carries the scaffold's
   exact NAME and somebody else's BYTES.  It is `Legacy` -- not a scaffold of
   either class -- because the digest is what is consulted. */
run witness_TT_17b_a_scaffold_name_over_foreign_bytes_is_still_legacy {
  GroveGrammar and SingleProc
  no Slot.occ and no Fmt.fmt
  some e: entries & kidsOf[TaskRoot] | {
    e in FileObj
    e.nm.fSpec = LeafSp and e.nm.fOut = LiveI and e.nm.fKind = OrdinaryK
    e.nm.fPos = 1 and e.nm.fKey = 1
    e.dg != ScaffoldD
  }
  rootState = LegacyR
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* `TT-17.a`'s witness: a LEGACY tree whose entries would otherwise read as
   current work.  Every entry is a canonical, known-kind, current-grammar name,
   the tree does not halt, and a reader that classified by looking at the entries
   would call it `Current(Live)`.  It is `Legacy`, because the witness says so
   and nothing else is consulted. */
run witness_TT_17a_a_legacy_tree_whose_entries_read_as_current_work {
  GroveGrammar and SingleProc
  no Slot.occ and no Fmt.fmt
  not halted
  some liveOrdinary
  not isExactScaffold and not hasRootInitExclusive
  rootState = LegacyR
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-18: classification order is fixed ----------------------------------

/* Reserved-witness classification precedes format classification, which
   precedes anything the walk derives — and `PartialScaffold` precedes `Legacy`.
   The last conjunct is the one that makes it an ORDER rather than a list: a
   root reaches `Malformed` only when the two classifications ahead of it have
   both passed, so a halted tree under a reserved witness is `Reserved`. */
check TT_18_classification_order_is_fixed {
  (GroveGrammar and SingleProc) implies always {
    some Slot.occ implies rootState = ReservedR
    (no Slot.occ and no Fmt.fmt)           implies rootState in (scaffoldFamily + LegacyR)
    (no Slot.occ and Fmt.fmt = ForeignFmt) implies rootState = ForeignR
    (no Slot.occ and isExactScaffold)      implies rootState = PartialScaffoldR
    (no Slot.occ and isAmbiguousScaffold)  implies rootState = AmbiguousScaffoldR
    rootState = MalformedR implies (rootClear and halted)
    rootState in currentFamily implies (rootClear and not halted)
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* The catalogue's witness: a tree carrying BOTH a reserved witness and no
   format witness, reported as the former.  Format classification would call it
   `Legacy` and the walk would call it live; neither runs. */
run witness_TT_18_a_reserved_witness_over_a_witnessless_root_reports_reserved {
  GroveGrammar and SingleProc
  some Slot.occ
  no Fmt.fmt
  some liveOrdinary
  rootState = ReservedR
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-19: a reserved witness refuses everything else ---------------------

/* Every observation and mutation refuses, the tree is byte-identical, the
   refusal NAMES the witness, and it names the operation that can recover it —
   except when the occupant is one grove cannot classify, which names no
   recovery at all.  Telling an operator to run a recovery against someone
   else's bytes is exactly the fail-closed violation the two reasons are split
   to prevent. */
/* NARROWED TO THE WITNESS, which is what the claim says in words: *a reserved
   WITNESS* refuses everything but its matching recovery.  Before `TT-24.b` the
   slot's other content -- an occupant grove cannot classify -- had no consumer,
   so `some Slot.occ` and `Slot.occ in WitnessClass` were the same set and the
   wider spelling was free.  They are two refusals now, and the wider spelling
   would assert that an unclassifiable occupant refuses with a WITNESS reason:
   the very confusion the two reasons are split to prevent. */
check TT_19_a_reserved_witness_refuses_everything_but_its_matching_recovery {
  (GroveGrammar and SingleProc) implies always (
    some (Slot.occ & WitnessClass) implies {
      (Sys.act' in groveActs + observeActs + InitScaffold + InitPublish) implies {
        Sys.res' = RefWitnessPending
        Sys.pending' = Slot.occ
        noTreeChange
        no Sys.got' and no Sys.gotTerm'
      }
      (Sys.act' in Recovery and Sys.act' not in recoveryFor[Slot.occ & WitnessClass])
        implies (Sys.res' = RefWitnessPending and noTreeChange)
      (Sys.res' = RefWitnessPending) implies
        Sys.recov' = recoveryFor[Slot.occ & WitnessClass]
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The catalogue's witness: a `Reserved(Preparing)` tree whose ordinary entries
   are all still in place, which therefore looks PERFECTLY WALKABLE — current
   format, no malformity, live work waiting — and whose `select` refuses
   anyway, naming the witness and `RecoverPreparing`. */
run witness_TT_19_a_preparing_witness_over_a_perfectly_walkable_tree {
  GroveGrammar and SingleProc
  always (Slot.occ = Preparing and Fmt.fmt = CurrentFmt)
  not halted
  some liveOrdinary
  eventually (Sys.act' = Select and Sys.res' = RefWitnessPending
              and Sys.pending' = Preparing and Sys.recov' = RecoverPreparing)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* The exception, so that "except the matching recovery" is not green for want of
   a recovery that ever applies.  Three states, for the reason the check needs
   three: an applied recovery is a tree change. */
run witness_TT_19_the_matching_recovery_is_admitted_and_settles_the_witness {
  GroveGrammar and SingleProc
  eventually (Sys.act' = RecoverPublished and Sys.res' = Applied
              and Slot.occ = Published and after no Slot.occ)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-20: the format witness lands last ----------------------------------

/* Four conjuncts.  The first two are *lands last*: the scaffold step never
   publishes, and publication happens only onto a complete scaffold.  The third
   is *no premature marker*.  The fourth is what the interruption LEAVES: never
   `Current(*)`, and never `Legacy`.

   THE THIRD AND FOURTH WERE BOTH RESTATED BY `cross-model-replay-k15`, and both
   for reasons this file had already written down about other claims.

   The third read `some inFlight implies no Fmt.fmt` — *while the transaction is
   open there is no witness to observe*.  That is a claim about the WORLD, and
   only grove's half of it is true: `doHandEdit` leaves `Fmt.fmt'` deliberately
   unconstrained, so once `step` admits the world during an open transaction the
   conjunct has a three-state counterexample.  It is this file's own rule —
   A CLAIM ABOUT WHAT A PROTOCOL NEVER DOES IS NEVER A CLAIM ABOUT WHAT THE
   WORLD NEVER DOES — met for the fourth time, and the first time inside a claim
   rather than inside a model defect.  It is stated over grove's own applied
   step instead.

   The fourth was a THEOREM OF `rootState`'s OWN BODY: given `no Slot.occ`, `no
   Fmt.fmt` and `isExactScaffold`, the second branch of `rootState` returns
   `PartialScaffoldR` by construction.  `check` it with NO protocol premise —
   no `GroveGrammar`, no `SingleProc`, no transition relation — and it is green.
   It therefore reported nothing about the protocol for the life of this file,
   and it is why entry 044's TT-20 counterexample was invisible here: that
   counterexample is a tree which STOPS being a partial scaffold, so it never
   enters this antecedent at all.  Restated over what an interruption LEAVES,
   which is a fact about `doInitScaffold`'s effects and dies under a mutation to
   them.

   THE NARROWING THAT STOOD HERE IS DISPOSED, AND BOTH HALVES MOVED.  It read
   only initialisations THE WORLD DID NOT TOUCH, because with one foreign write
   the interrupted root classified `Legacy` — which the prose forbids.  The
   catalogue now classifies that root `AmbiguousScaffoldR`, so the world's write
   is admitted into the claim rather than fenced out of it, and what survives is
   a smaller and true prohibition on `Legacy`: never, ONCE A ROOT-INIT-EXCLUSIVE
   ENTRY HAS LANDED.  Before that the root carries no evidence distinguishing it
   from a legacy tree — a charter is not evidence — so `Legacy` is honest there.
   That window is the shipped one, after the charter and before the leaf, and it
   is kept REACHABLE as a witness below rather than merely declared. */
check TT_20_the_format_witness_lands_last {
  (GroveGrammar and SingleProc) implies always {
    (Sys.act' = InitScaffold and Sys.res' = Applied) implies no Fmt.fmt'
    (Sys.act' = InitPublish and Sys.res' = Applied) implies {
      // ONLY an EXACT scaffold is ever published, which is the completion half
      // of the same disposition: grove completes a root whose whole contents it
      // can account for and refuses one it cannot.
      isExactScaffold and no Fmt.fmt
      after (Fmt.fmt = CurrentFmt)
    }
    (Sys.res' = Applied and some inFlight') implies no Fmt.fmt'
    (Sys.act' = InitScaffold and Sys.res' = Applied) implies
      after ((Sys.act' = Crash) implies after {
        rootState in (scaffoldFamily + LegacyR)
        rootState not in currentFamily
        hasRootInitExclusive implies rootState != LegacyR
      })
  }
} for 4 but 4 Int, 4 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* THE DISPOSED COUNTEREXAMPLE, NOW THE CLAIM'S OWN SECOND WITNESS.  Entry 044's
   TT-20 finding was that a scaffold, one world write and an interrupt classified
   `LegacyR` — the classification TT-20's prose forbids — because a stray entry
   dropped the root out of the exact closed subset.  With the third branch it
   classifies `AmbiguousScaffoldR`, which REFUSES, where `Legacy` MIGRATES; the
   trace is the same and the answer is not.  It credits a cell now, so it is a
   `witness_TT_20_`. */
run witness_TT_20_a_world_write_during_an_open_scaffold_is_ambiguous_not_legacy {
  GroveGrammar and SingleProc
  eventually {
    Sys.act' = InitScaffold and Sys.res' = Applied and some inFlight'
    after (Sys.act' = HandEdit and some inFlight'
           and after (Sys.act' = Crash
                      and after (no inFlight and no Fmt.fmt
                                 and some foreignEntries
                                 and hasRootInitExclusive
                                 and rootState = AmbiguousScaffoldR)))
  }
} for 4 but 4 Int, 4 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* THE DECLARED WINDOW, KEPT REACHABLE.  Before any root-init-exclusive entry has
   landed there is nothing to distinguish the root from a legacy tree, so a
   concurrent world write DOES send an interrupted initialisation to `LegacyR` —
   and TT-20 no longer forbids it.  A narrowing nobody can run is
   indistinguishable from one nobody declared, which is why this is a command
   rather than a sentence. */
run witness_TT_20_the_window_before_the_first_exclusive_entry_reaches_legacy {
  GroveGrammar and SingleProc
  eventually {
    Sys.act' = InitScaffold and Sys.res' = Applied and some inFlight'
    after (Sys.act' = HandEdit and some inFlight'
           and after (Sys.act' = Crash
                      and after (no inFlight and no Fmt.fmt
                                 and some foreignEntries
                                 and not hasRootInitExclusive
                                 and rootState = LegacyR)))
  }
} for 4 but 4 Int, 4 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* The catalogue's witness: scaffold, interrupt, and the STABLE root the
   interruption left.  What makes the last state stable rather than transient is
   the crash — the transaction is closed, so an ordinary invocation may observe
   it — and that is `EN-08`'s whole content.  Three states exactly: it finds no
   instance at 2. */
run witness_TT_20_an_interrupted_initialisation_leaves_a_partial_scaffold {
  GroveGrammar and SingleProc
  eventually {
    Sys.act = InitScaffold and Sys.res = Applied and some inFlight
    Sys.act' = Crash
    after (no inFlight and no Fmt.fmt and isExactScaffold
           and rootState = PartialScaffoldR)
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The other half, so the check above is not green for want of an initialisation
   that ever completes: scaffold, publish, and a `Current(Live)` root.  Three
   FOUR states, and the reason is one more than the interrupted case needs: the
   crash IS the third state, while the publish must be followed by a state in
   which the published root is observed.  It finds no instance at 3. */
run witness_TT_20_an_uninterrupted_initialisation_publishes_the_witness {
  GroveGrammar and SingleProc
  eventually (Sys.act' = InitPublish and Sys.res' = Applied
              and after (Fmt.fmt = CurrentFmt and rootState = CurrentLiveR))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps


// ===========================================================================
// CLAIMS — TT-21 .. TT-23, the guarding scope
//
// WHAT SEPARATES THESE FROM EVERYTHING ABOVE.  `TT-01` .. `TT-20` are stated
// over ONE cooperating process and over operations that are one transition
// each.  These six obligations are about what happens DURING an operation and
// about what a SECOND process may do while it runs, so none of them assumes
// `CurrentRootThroughout` -- they assume `Guarding`, which turns the process
// scope on instead.
//
// WHY EN-07 IS NOT IN THE BUNDLE.  `EN_07` is left FREE in `Guarding`, so every
// check below is checked over both the incumbent and the broken assumption at
// once.  That is not carelessness: the assumption table's expected-result column
// for `EN-07` names `SY-11.b`, the lifecycle scope's, and what this slice owes
// is the `TT-` half.  Leaving it free is what makes "no `TT-` obligation depends
// on EN-07" a checked result rather than an unrun one.
// ===========================================================================

/* The guarding scope.  Two cooperating processes, a working-tree root to hold
   the guard on, and a task root grove may act on at all -- the last for the
   same reason `TT-01` .. `TT-16` carry `CurrentRootThroughout`: a claim about
   what two operations may do concurrently was never a claim about a root either
   of them refuses to touch. */
pred Guarding {
  GroveGrammar
  Concurrent
  EN_14
  always rootClear
  always no inFlight
  // AND A WALKABLE TREE.  The four concurrency transitions do NOT run the halt
  // cascade, which is a deliberate omission rather than a slip: halting is
  // `TT-02`/`TT-03`'s subject and is already checked there over every read and
  // every mutation, and adding a fifth copy of it here would be paid for by
  // every command in the file while answering no `TT-21` .. `TT-23` obligation.
  // Pinning the tree walkable is what keeps the omission from licensing a mark
  // on a tree grove would refuse -- the same narrowing, and the same reason, as
  // `CurrentRootThroughout`.
  always not halted
}

// --- TT-21: one snapshot per operation -------------------------------------

/* TT-21.a.  A COOPERATING writer is excluded by the guard: while any process
   holds one, no step taken by anyone ELSE that is not the world's own writer
   moves the tree.  That is the invariant two classifications of one operation
   rest on, and it is stated over the TREE rather than over the mark's guard so
   that it is a different formula from `TT-22.b` -- the same mechanism seen at
   the classification level rather than at the acquisition. */
check TT_21a_a_cooperating_writer_cannot_move_the_tree_under_a_held_guard {
  Guarding implies always (
    all p: Proc |
      (some p.holds and Sys.who' != p and Sys.act' not in (HandEdit + ForeignWrite))
        implies (onDisk' = onDisk and nm' = nm and loc' = loc and dg' = dg)
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 4 steps

/* The catalogue's witness: that interleaving, SHOWN SERIALIZED.  P1 opens, makes
   one classification, P2's mark is deferred, and P1 makes its second
   classification -- both from the one listing P1's guard was taken with. */
run witness_TT_21a_a_cooperating_writer_between_two_classifications_is_serialized {
  Guarding
  eventually {
    Sys.act = Open and Sys.who = P1 and Sys.res = Applied
    after (Sys.act = Classify and Sys.who = P1 and Sys.res = Applied
      and after (Sys.act = Mark and Sys.who = P2 and Sys.res = Deferred
        and after (Sys.act = Classify and Sys.who = P1 and Sys.res = Applied
                   and #P1.cls = 2 and P1.clsLive = liveBySnap[P1])))
  }
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-21.b.  A NON-cooperating writer is NOT excluded, and the claim survives it:
   every classification an operation made still comes from its one listing, and
   every member of a plan it is still working through was licensed by the listing
   that validated it.  TT-21 is internal consistency, not exclusion of the world
   -- a model that serialized `foreign-write` would have answered `EN-06` by
   construction, which is the shape of a false-confidence incident. */
check TT_21b_every_classification_and_every_plan_member_comes_from_the_one_listing {
  Guarding implies always (
    all p: Proc | {
      p.clsLive = liveBySnap[p]
      all m: p.plan | licensedByPlan[p, m]
    }
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 4 steps

/* The catalogue's witness: a `foreign-write` landing between two
   classifications, with the operation's classifications still mutually
   consistent.  The last conjunct is what makes it the interleaving rather than a
   quiet trace -- the operation's listing no longer describes the tree. */
run witness_TT_21b_a_foreign_write_lands_between_two_classifications {
  Guarding
  eventually {
    Sys.act = Open and Sys.who = P1 and Sys.res = Applied
    after (Sys.act = Classify and Sys.who = P1 and Sys.res = Applied
      and after (Sys.act = ForeignWrite
        and after (Sys.act = Classify and Sys.who = P1 and Sys.res = Applied
                   and #P1.cls = 2 and P1.clsLive = liveBySnap[P1]
                   and P1.snapNm != (onDisk - TaskRoot) <: nm)))
  }
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 5 steps

// --- TT-22: shared for observation, exclusive for mutation -----------------

/* TT-22.a.  A shared guard never excludes another shared guard: an open asking
   for `Shared` is admitted whenever the process holds nothing and no exclusive
   guard is out.  Stated over `Sys.mode'` because a DEFERRED open is otherwise
   indistinguishable from a deferred mark. */
check TT_22a_a_shared_guard_never_excludes_another_shared_guard {
  Guarding implies always (
    all p: Proc |
      (Sys.act' = Open and Sys.who' = p and Sys.mode' = Shared and no p.holds
       and (no q: Proc - p | q.holds = Exclusive))
        implies Sys.res' = Applied
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 3 steps

/* Reached: two observations holding at once. */
run witness_TT_22a_two_observations_hold_the_root_together {
  Guarding
  eventually (some P1.holds and some P2.holds)
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 4 steps

/* TT-22.b.  An observation and a mutation are SERIALIZED.  The mutation's
   exclusive guard is acquired and consumed inside its own step
   (`bulk-marks-are-not-atomic`), so the claim is stated over the acquisition:
   no mark ever applies while another process holds. */
check TT_22b_an_observation_and_a_mutation_are_serialized {
  Guarding implies always (
    (Sys.act' = Mark and Sys.res' != Deferred) implies
      (no q: Proc - Sys.who' | some q.holds)
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 3 steps

/* Reached, and reached as a SERIALIZATION rather than as a refusal: P2's mark
   is deferred while P1 observes, and applies once P1 has released. */
run witness_TT_22b_a_mark_waits_for_an_observation_and_then_applies {
  Guarding
  eventually {
    Sys.act = Open and Sys.who = P1 and Sys.res = Applied
    after (Sys.act = Mark and Sys.who = P2 and Sys.res = Deferred
      and after (Sys.act = Release and Sys.who = P1 and Sys.res = Applied
        and after (Sys.act = Mark and Sys.who = P2 and Sys.res = Applied)))
  }
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 6 steps

// --- TT-23: a bulk mark validates before it moves, and converges -----------

/* TT-23.a.  Two conjuncts.  The whole plan is validated against ONE listing
   before the first rename -- so a run whose FIRST mark applies is a run in which
   no live leaf anywhere was a `finish` leaf -- and a mark that refuses renames
   nothing. */
check TT_23a_the_whole_plan_is_validated_before_the_first_rename {
  Guarding implies always (
    all p: Proc | {
      (Sys.act' = Mark and Sys.who' = p and Sys.res' = Applied and no p.plan)
        implies (no o: liveLeaves | o.nm.fKind = FinishK)
      // and the validation STANDS for the rest of the run, across the guards
      // the ADR gives each mark of its own.
      all m: p.plan | p.planNm[m].fKind != FinishK
      (Sys.act' = Mark and Sys.res' in Refused) implies noTreeChange
    }
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 3 steps

/* The catalogue's witness: a plan whose LATER member is invalid, refused before
   the first rename lands.  Both leaves are still live at the refusal, which is
   what "before the first rename" means on a tree of two. */
run witness_TT_23a_a_plan_with_an_invalid_member_is_refused_before_the_first_rename {
  Guarding
  eventually (Sys.act = Mark and Sys.res = RefReservedKind
              and some liveOrdinary and some liveFinish)
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 2 steps

/* TT-23.b.  Convergence, as two falsifiable conjuncts.  An already-terminal
   entry is never renamed again -- it is skipped silently, which is what makes
   the re-run idempotent on the part that already landed -- and a run with
   nothing left to mark reports `Empty` rather than refusing, which is what makes
   the re-run REACH the same result instead of failing at it. */
check TT_23b_a_rerun_skips_what_landed_and_converges {
  Guarding implies always (
    all p: Proc | {
      (Sys.act' = Mark and Sys.res' = Applied and some Sys.tgt')
        implies Sys.tgt'.nm.fOut = LiveI
      (Sys.act' = Mark and Sys.who' = p and Sys.res' != Deferred and no liveLeaves)
        implies Sys.res' = Empty
    }
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 3 steps

/* The catalogue's witness: a bulk mark interrupted mid-run, repaired by running
   it again.  The interruption is `crash` (`EN-08`), which is what loses the plan
   and makes the second run a FRESH plan over the tree the first one left. */
run witness_TT_23b_an_interrupted_bulk_mark_is_repaired_by_rerunning_it {
  Guarding
  eventually {
    Sys.act = Mark and Sys.res = Applied and some liveLeaves and some Sys.who.plan
    after (Sys.act = Crash
      and after (Sys.act = Mark and Sys.res = Applied and no liveLeaves))
  }
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps


// ===========================================================================
// CLAIMS — TT-24 .. TT-25, fail-closed ownership and derived done-ness
//
// WHAT SEPARATES THESE FROM THE SLICE BEFORE THEM.  `TT-21` .. `TT-23` are the
// only claims in this file about two processes; these two are single-process
// claims again, so every command below PINS the scope.  That is not a style
// rule: in the concurrent scope no grove mutation exists at all, so an unpinned
// witness is unreachable and an unpinned check is vacuous — the file's retained
// false-confidence incident in a fourth set of clothes.
//
// `TT-24` IS ONE ARTIFACT MET IN THREE CONTEXTS, and the catalogue fixes the
// outcome of each rather than letting a model choose.  ONLY THE FIRST IS THIS
// FILE'S, and that is now the catalogue's own arrangement rather than a gap
// this column declared: `TT-24.c` and `TT-24.d` were declared out-of-bounds
// here — one had no outcome in this `Result` set, the other no subject — and
// `obligation-placement-k63` retired both, because a live transaction and the
// quarantine reaper are `grove-finish`'s actions and that crate depends on this
// one (`docs/adr/obligations-follow-context-not-artifact.md`).  They are
// `FN-32` and `FN-21.c`.  Inventing a fourth outcome to make a cell green was
// the thing the catalogue's table exists to prevent, and declining to was the
// right call: the sibling column filled them by importing the machinery, and
// its answer for `TT-24.c` restated its own gate.
// ===========================================================================

/* WHAT GROVE CANNOT PROVE IS ITS OWN.  A foreign entry anywhere the walk can or
   cannot reach, and the occupant of the reserved name.  `Malformed` is NOT here
   and the distinction is the catalogue's: a malformed name is grove's own and
   broken, which is why it halts with recovery advice rather than being left
   alone. */
fun unprovable: set Obj { foreignEntries + Slot.occAt }

// --- TT-24.a: nothing unprovable is ever mutated ---------------------------

/* Stated over every action THIS FILE ADMITS and over EVERY root, which is what
   separates it from `TT-04`.  `TT-04` is a claim about grove's five tree
   mutations on a root grove may act on at all; this one reaches the
   root-lifecycle actions and the roots `CurrentRootThroughout` excludes — an
   `initialise-root` over a directory that already holds someone's bytes is the
   fail-closed violation nothing else in this file would catch.

   IT DOES NOT REACH A FINISH TRANSACTION'S STEPS OR THE QUARANTINE REAPER, and
   the catalogue now says so: `action` spans scopes, so an obligation quantified
   over it is read over its own scope's admitted set
   (`docs/adr/obligations-follow-context-not-artifact.md`, clause 4).  Those two
   contexts are `FN-32`'s and `FN-21.c`'s, and a green here is not evidence
   about either. */
check TT_24a_no_action_mutates_what_it_cannot_prove_is_its_own {
  (GroveGrammar and SingleProc) implies always (
    Sys.act' not in (HandEdit + ForeignWrite) implies
      (all o: unprovable |
         o in onDisk' and o.nm' = o.nm and o.loc' = o.loc and o.dg' = o.dg))
// THREE `FileObj`, AND THE MUTATION IS WHAT ASKED FOR THEM.  The claim mentions
// two objects — an actor and something unprovable — but the transition most
// likely to violate it is `InitScaffold`, which introduces a charter and a first
// leaf of its OWN before it can trample anything, and that is three files before
// the tree has one.  At `2 FileObj` the mutation aimed at this obligation cannot
// fire, and reports green exactly as a survivor would.
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The catalogue's witness: a mutation ATTEMPTED against an entry grove cannot
   prove is its own, shown not taken.  A refusal changes nothing, so the trace
   closes on itself and two states are enough. */
run witness_TT_24a_a_mutation_against_a_foreign_entry_is_not_taken {
  GroveGrammar and CurrentRootThroughout
  eventually (some o: foreignEntries |
    Sys.act' in (Retire + Prune) and Sys.tgt' = o and Sys.res' = RefNotAnEntry
    and o in onDisk' and o.nm' = o.nm and o.dg' = o.dg)
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 2 steps

// --- TT-24.b: an occupied reserved name refuses, naming the entry ----------

/* Four conjuncts, and the middle two are the claim.  The refusal NAMES THE
   ENTRY — `Sys.occupant'`, written by the transition, because a derived value
   could not be got wrong — and it names NO RECOVERY and no class, because there
   is none it could honestly report.  Telling an operator to run a recovery
   against someone else's bytes is the fail-closed violation `WitnessPending`
   and `ReservedNameOccupied` are two reasons to prevent. */
check TT_24b_an_occupied_reserved_name_refuses_naming_the_entry_and_no_recovery {
  (GroveGrammar and SingleProc) implies always (
    Slot.occ = Unowned implies (
      Sys.act' in (groveActs + observeActs + rootActs) implies {
        Sys.res' = RefReservedNameOccupied
        Sys.occupant' = Slot.occAt
        no Sys.recov' and no Sys.pending'
        noTreeChange
        no Sys.got' and no Sys.gotTerm'
      }))
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 3 steps

/* The catalogue's witness, built as `TT-19`'s is: a tree that looks PERFECTLY
   WALKABLE — current format, nothing malformed, live ordinary work waiting —
   and whose ordinary operation refuses anyway, because someone's bytes sit at a
   name grove reserves.  The tree is byte-identical and no recovery is named. */
run witness_TT_24b_an_ordinary_operation_meets_a_foreign_entry_at_the_reserved_name {
  GroveGrammar and SingleProc
  always (Slot.occ = Unowned and Fmt.fmt = CurrentFmt)
  not halted
  some liveOrdinary
  eventually (Sys.act' in groveActs and Sys.res' = RefReservedNameOccupied
              and some Sys.occupant' and Sys.occupant' = Slot.occAt
              and no Sys.recov' and no Sys.pending' and noTreeChange)
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 2 steps

// --- TT-25: a node is never marked -----------------------------------------

/* TT-25.a.  Two conjuncts, and the second is the falsifiable one — the same
   shape `TT-17` has.  The first is true BY CONSTRUCTION of the grammar: a node
   name carries no outcome infix at all (`isShaped`), so this model cannot spell
   a marked node, and that is the honest place to say so rather than a command
   pretending to check it.  The second is behavioural and is what "done-ness is
   DERIVED" actually forbids: the transition that makes a node done writes
   NOTHING to the node — not its name, not its bytes, not its place. */
check TT_25a_a_node_becomes_done_by_its_subtree_and_is_never_written_to {
  (GroveGrammar and CurrentRootThroughout) implies always {
    all d: nodeDirs - TaskRoot | no d.nm.fOut
    all d: nodeDirs - TaskRoot |
      (Sys.act' in groveActs and Sys.res' = Applied
       and not nodeDone[d] and after nodeDone[d])
        implies (d in onDisk' and d.nm' = d.nm and d.loc' = d.loc and d.dg' = d.dg)
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* Reached: the retirement of the last live leaf beneath a node, after which the
   node is derived done and its own entry is byte-identical to what it was. */
run witness_TT_25a_retiring_the_last_leaf_beneath_a_node_derives_it_done_unmarked {
  GroveGrammar and CurrentRootThroughout
  eventually (some d: nodeDirs - TaskRoot |
    Sys.act' in (Retire + Prune) and Sys.res' = Applied
    and not nodeDone[d] and after nodeDone[d]
    and d.nm' = d.nm and no d.nm'.fOut and d.dg' = d.dg)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* TT-25.b.  Stated over `d.^(~loc)` — every leaf ANYWHERE beneath the node —
   rather than over `nodeDone`, so that the mechanism has somewhere to be wrong.
   That is `TT-14`'s shape: `TT-14` names `fPos` rather than `precedes` for the
   same reason, and re-defining done-ness to read only the node's own children
   breaks this check while leaving `TT-25.a` green.
   THREE `DirObj`, and it is the claim that asks for it: a node, a node beneath
   it, and the task root.  At two the live leaf is necessarily a direct child,
   the two readings of done-ness agree, and the mutation aimed at this
   obligation survives for want of a place to differ. */
check TT_25b_a_node_with_a_live_leaf_anywhere_beneath_it_is_derived_live {
  (GroveGrammar and CurrentRootThroughout) implies always (
    all d: nodeDirs - TaskRoot |
      (some (liveLeaves & d.^(~loc))) implies (not nodeDone[d] and no d.nm.fOut))
} for 4 but 4 Int, 3 FileObj, 3 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* Reached, and reached at the DEPTH the claim is about: the live leaf is two
   levels beneath the node it keeps live, which is the situation a done-ness
   that read only the node's children would get wrong. */
run witness_TT_25b_a_node_is_live_on_a_leaf_two_levels_beneath_it {
  GroveGrammar and CurrentRootThroughout
  some d, e: nodeDirs - TaskRoot | some o: liveLeaves | {
    e.loc = d and o.loc = e
    not nodeDone[d] and not nodeDone[e]
    no d.nm.fOut and no e.nm.fOut
    no (liveLeaves & kidsOf[d])
  }
} for 4 but 4 Int, 3 FileObj, 3 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps


// ===========================================================================
// THE ALLOY-OWNED ASSUMPTION MUTATIONS THIS SLICE RUNS
//
// `models/run.sh`'s two inverted forms.  `expect_fail_` must find a
// counterexample; `expect_unreachable_` must find none.  They are inverted
// deliberately: a mutation whose control is "this named obligation fails"
// cannot be reported by a runner that treats every failing check as a defect.
// ===========================================================================

/* EN-04 — counterfactual-capability: THERE IS NO ATOMIC REPLACEMENT OF A FILE
   BY A DIFFERENTLY NAMED DIRECTORY, and the candidate grants one.  This model
   already carries the candidate: promotion is one step, which the abstraction
   table records.  The control is therefore that the capability is really in
   force — the half-applied promotion the incumbent would expose is unreachable
   — and the retained obligations `TT-07`, `TT-08` and `TT-09` are green beside
   it in this same file, at no wider bound.
   `TT-02.b`'s witness lands by hand edit rather than through a half-promoted
   entry, which is the exercised half: see `witness_TT_02b_node_name_at_a_file`. */
run expect_unreachable_EN_04_promotion_is_never_observed_half_applied {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act = Decompose and Sys.res = Applied and Sys.tgt in onDisk)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* EN-12 — premise-break: A NAME RENDERS AS EXACTLY ONE PATH COMPONENT.  Drop it
   and rendering becomes many-to-one, so two accepted spellings reach one entry
   and `TT-01.a` fails — which is the assumption table's stated expected result.
   The bundle is `ParseIsCanonical and GrammarIsTotal` rather than
   `GroveGrammar`, because `GroveGrammar` is what carries `EN_12`. */
check expect_fail_EN_12_TT_01a_a_name_that_renders_as_two_components {
  ParseIsCanonical and GrammarIsTotal and CurrentRootThroughout implies
    (all disj f, g: entryName | not denotesSame[f, g])
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* EN-07 — premise-break: TWO OPEN DESCRIPTIONS OF ONE DIRECTORY DO NOT SHARE A
   LOCK.  It is the assumption that rejects `bulk-marks-are-not-atomic`'s third
   considered option — hold Grove's own exclusive guard around the whole run and
   let the library take its guard inside it — because both `flock` the directory
   containing the tree root and the inner acquisition would deadlock against the
   outer one.  The control is therefore that the deadlock is really in force:
   under the incumbent, a process holding its own guard can never take a mark.

   THE FINDING IS THAT NO `TT-` OBLIGATION DEPENDS ON IT, which is what the
   assumption table predicts in its own expected-result column — it names
   `SY-11.b`, the lifecycle scope's.  Every `TT-21` .. `TT-23` check above leaves
   `EN_07` FREE, so all six are checked over the broken assumption as well as
   over the incumbent, and all six are green either way.  An assumption carrying
   no weight in this scope is a legitimate result of this control and not a
   defect in it. */
run expect_unreachable_EN_07_an_outer_guard_is_never_held_across_a_mark {
  Guarding and EN_07
  eventually (Sys.act = Mark and Sys.res != Deferred and some Sys.who.holds)
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps

/* The fire evidence for it, and the other half of the premise-break: once the
   two descriptions DO share a lock the nested acquisition is admitted, so the
   command above is unreachable because of the assumption rather than because
   the situation cannot be built. */
run witness_EN_07_the_outer_guard_is_admitted_once_two_descriptions_share_a_lock {
  Guarding and not EN_07
  eventually (Sys.act = Mark and Sys.res = Applied and some Sys.who.holds)
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps

/* EN-14 — premise-break: THE WORKING-TREE ROOT EXISTS BEFORE THE TASK ROOT AND
   OUTLIVES ITS DELETION.  It is what the guard is held ON — the task root is
   `TaskRoot` and cannot be it, since finish deletes that and the lease outlives
   it.  Remove the working-tree root and there is nothing to `flock`: no guard is
   taken, the compatibility test has no subject, and a mutation lands while an
   observation is mid-flight.  `TT-22.b` fails, which is the `TT-` half of the
   assumption table's expected result (its own column names `SY-01`, the
   lifecycle scope's second driver). */
check expect_fail_EN_14_TT_22b_with_no_root_to_guard_a_mark_lands_mid_observation {
  (GroveGrammar and Concurrent and not EN_14 and always rootClear
     and always no inFlight) implies always (
    (Sys.act' = Mark and Sys.res' != Deferred) implies
      (no q: Proc - Sys.who' | some q.holds)
  )
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug, 2 Digest, 3 steps

/* EN-08 — exercise-removal: INTERRUPTION MAY OCCUR BETWEEN ANY TWO STEPS.  Run
   against the two NAMED WITNESS SETS it controls rather than against the whole
   file, which is what the assumption table's `TT-` column names: `TT-20`'s
   interrupted initialisation (`roots` landed it) and `TT-23.b`'s interrupted
   bulk mark (this slice's).  With `crash` removed both are unreachable.

   EVERY PROPERTY CHECK STAYS GREEN, and that half needs no run of its own: no
   check in this file asserts `EN_08`, so each is already checked over the traces
   that contain `crash` AND the traces that do not.  Green over the superset is
   green over the subset. */
run expect_unreachable_EN_08_the_interrupted_initialisation_witness_needs_crash {
  GroveGrammar and SingleProc and not EN_08
  eventually {
    Sys.act = InitScaffold and Sys.res = Applied and some inFlight
    Sys.act' = Crash
    after (no inFlight and no Fmt.fmt and isExactScaffold
           and rootState = PartialScaffoldR)
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run expect_unreachable_EN_08_the_interrupted_bulk_mark_witness_needs_crash {
  Guarding and not EN_08
  eventually {
    Sys.act = Mark and Sys.res = Applied and some liveLeaves and some Sys.who.plan
    after (Sys.act = Crash
      and after (Sys.act = Mark and Sys.res = Applied and no liveLeaves))
  }
} for 4 but 4 Int, 2 FileObj, 1 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps


/* EN-11 — exercise-removal: ANY WELL-FORMED TREE IS REACHABLE BY HAND EDIT.  Run
   against the NAMED WITNESS SETS the assumption table lists rather than against
   the whole file, which is what an exercise-removal asks for.

   WHAT THE REMOVAL HAS TO TAKE AWAY, and it is two things rather than one.  The
   assumption is realised here as the `hand-edit` action AND as the
   unconstrained initial state — the README's `3 steps` argument rests on the
   second in as many words ("every single transition is reachable from state 0").
   Removing only the action leaves every witness reachable at state 0 and the
   control reports green while removing nothing.  `not EN_11` takes both, and
   what is left is a world grove's own actions had to build from an empty task
   root.

   `SingleProc` RATHER THAN `CurrentRootThroughout`, and the difference is what
   makes these commands mean anything.  `CurrentRootThroughout` excludes the
   root-lifecycle actions, so under it an empty start can never be populated at
   all and every command below would be unreachable for the trivial reason.  The
   scope here admits `initialise-root`, so grove has a way to build — and the
   companion witness is what shows it takes it.

   `5 steps`, and it is the shortest run-up that lets grove build anything:
   scaffold, publish, one mutation, and the state that closes the lasso. */
run witness_EN_11_groves_own_actions_still_build_a_tree_without_a_hand_edit {
  GroveGrammar and SingleProc and not EN_11
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-02: a name that declares one species over an object of the other.  Grove
   never writes one — the species a name declares is the species of the object
   the operation creates — so this witness is `hand-edit`'s or it is nothing. */
run expect_unreachable_EN_11_a_species_mismatch_needs_a_hand_edit {
  GroveGrammar and SingleProc and not EN_11
  eventually (some o: (visited & DirObj) - TaskRoot |
                o.nm in entryName and o.nm.fSpec = LeafSp)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-03: a malformed node hiding live work.  Every name grove writes is
   accepted by the grammar, so a malformed one has no other way in. */
run expect_unreachable_EN_11_a_malformed_node_hiding_live_work_needs_a_hand_edit {
  GroveGrammar and SingleProc and not EN_11
  eventually (some d: (visited & DirObj) - TaskRoot | d.nm in malformedName
    and some o: kidsOf[d] |
      o.nm in entryName and o.nm.fSpec = LeafSp and o.nm.fOut = LiveI)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-13.c: two live finish leaves.  `add-leaf` refuses `FinishK` with
   `RefReservedKind` (the driver allocates the finish leaf, not the operator),
   so grove cannot write even ONE, let alone the second that halts the tree. */
run expect_unreachable_EN_11_two_live_finish_leaves_need_a_hand_edit {
  GroveGrammar and SingleProc and not EN_11
  eventually rMultipleLiveFinish
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-24.b: someone else's bytes at a name grove reserves.  Grove writes
   witnesses at that name and never an artifact it cannot classify, so an
   `Unowned` slot is by definition not grove's work. */
run expect_unreachable_EN_11_an_occupied_reserved_name_needs_a_hand_edit {
  GroveGrammar and SingleProc and not EN_11
  eventually (Slot.occ = Unowned)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-25: a live leaf two levels beneath a node.  Grove CAN build this one in
   principle — two promotions — but not inside the bound: each promotion brings a
   node, a charter and a leaf, and the file's scope holds three files.  Recorded
   as an unreachability at THIS bound rather than as a claim about grove's
   reach, which is the distinction the whole `expect_unreachable_` form rests
   on. */
run expect_unreachable_EN_11_a_leaf_two_levels_deep_needs_a_hand_edit_at_this_bound {
  GroveGrammar and SingleProc and not EN_11
  eventually (some d, e: nodeDirs - TaskRoot | some o: liveLeaves |
                e.loc = d and o.loc = e)
} for 4 but 4 Int, 3 FileObj, 3 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps

/* TT-16, AND IT IS THE FINDING: the assumption table lists `TT-16` among the
   witnesses `EN-11` controls, and it does not control them.  A resolved
   TERMINAL entry is something grove's own actions build — scaffold, publish,
   retire, resolve — so the witness needs no hand edit and removing `hand-edit`
   leaves it standing.  Shipped as a POSITIVE control, because that is what the
   evidence supports: an `expect_unreachable_` here would be a command written to
   the table rather than to the model.  The row is corrected in the catalogue. */
run witness_EN_11_a_resolved_terminal_entry_needs_no_hand_edit {
  GroveGrammar and SingleProc and not EN_11
  eventually (Sys.act' = Resolve and Sys.res' = Reported and
    (some o: Sys.got' | o.nm.fOut = DoneI and o in Sys.gotTerm'))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 5 steps


// ===========================================================================
// VACUITY GUARDS
//
// Every check above has the form `GroveGrammar and CurrentRootThroughout
// implies P`.  If the law bundle
// were unsatisfiable over a populated tree, every one of them would pass for no
// reason at all.
// ===========================================================================

run witness_vacuity_the_law_bundle_admits_a_working_grove {
  GroveGrammar and CurrentRootThroughout
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied)
  eventually (Sys.act' in (Retire + Prune) and Sys.res' = Applied)
  some entries
  treeOk
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 8 Filename, 2 Slug, 3 Digest, 4 steps
