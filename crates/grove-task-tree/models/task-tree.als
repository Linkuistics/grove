/*
 * grove-task-tree — the task-tree claims, in Alloy 6
 * ==================================================
 *
 * The subject is `docs/specs/semantic-contract.md`, section *Claims — task
 * tree*.  Nothing else: no Rust module, no helper, no control-flow shape.  Every
 * command below names an OBLIGATION of that document, and the repository runner
 * reads the obligation list out of the document rather than out of this file.
 *
 * COVERAGE SO FAR: TT-01 .. TT-16.  TT-17 .. TT-25 are the `guarding` sibling
 * leaf's; the runner reports their cells empty, which is the truth about this
 * file rather than a defect in it.
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
   them is stated over the reserved CLASS rather than over its members. */
one sig Slot { var occ: lone SlotContent }

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
one sig ReservedR, PartialScaffoldR, LegacyR, ForeignR, MalformedR,
        CurrentLiveR, CurrentFinishOnlyR, CurrentSpentR extends RootState {}
fun currentFamily: set RootState { CurrentLiveR + CurrentFinishOnlyR + CurrentSpentR }

/* The FORMAT family a classification lands in.  TT-17 is stated over this and
   not over the state itself, because the split INSIDE `Current(*)` is
   walk-derived and reading entries is what it is for; what the claim forbids is
   entry text moving the root between the families. */
fun familyOf[s: RootState]: set RootState {
  s = ReservedR                          implies ReservedR
  else s in (PartialScaffoldR + LegacyR) implies (PartialScaffoldR + LegacyR)
  else s = ForeignR                      implies ForeignR
  else                                           (MalformedR + currentFamily)
}

/* PARTIAL SCAFFOLD, as the catalogue defines it: an exact closed SUBSET, never
   "present and witnessless".  Anything outside the subset -- a second
   positioned entry, a differing byte, a foreign entry, a node -- falls through
   to `Legacy`, and that is what makes completion safe rather than an inference
   about someone else's tree. */
pred isPartialScaffold {
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

/* CLASSIFICATION, IN THE FIXED ORDER (TT-18): reserved-witness first, then
   format, then walk-derived -- and `PartialScaffold` before `Legacy`.  A `fun`
   rather than a `var` field, so it adds no free state for the solver to search;
   the order is a claim because REORDERING THIS BODY is a mutation the matrix
   runs, not because the model leaves the order open. */
fun rootState: one RootState {
  some Slot.occ                                        implies ReservedR
  else (no Fmt.fmt and isPartialScaffold)              implies PartialScaffoldR
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
/* The algebra's own refusal, opaque.  TT-10 is the claim that no ordinary
   argument reaches it, because grove's preconditions run in front. */
one sig AlgebraicRefusal extends Result {}
one sig Environmental extends Result {}
/* The catalogue's observation outcomes.  `Empty` and `Ambiguous` are SUCCESSES,
   not refusals, because that is the shipped contract and callers branch on it
   (TT-15); the whole content of that claim is that they sit here rather than
   under `Refused`. */
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
  var recov:   lone Recovery
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
  Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ
}

/* Nothing was named, because nothing pending was met. */
pred noPending { no Sys.pending' and no Sys.recov' }

/* THE ROOT-IDENTITY CASCADE every observation and mutation runs before it looks
   at its own operand (TT-18, TT-19).  Reserved witness first, then format, and
   only then anything the walk derives -- which is why `RefMalformed` is not in
   here: it is walk-derived and classifies LAST.
   A refusal here names the witness and the operation that recovers it; an
   occupant grove cannot classify names no recovery, which is the whole reason
   `WitnessPending` and `ReservedNameOccupied` are two reasons and not one. */
pred reservedRefusal {
  some Slot.occ implies {
    Sys.res' = RefWitnessPending
    Sys.pending' = Slot.occ
    Sys.recov' = (Slot.occ in WitnessClass implies recoveryFor[Slot.occ] else none)
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
    Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ
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
      Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ
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
    Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ
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
    Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ
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
  Sys.act' = HandEdit and Sys.res' = Environmental and no Sys.tgt'
  noPending and no inFlight'
  // `Fmt.fmt'` and `Slot.occ'` are deliberately UNCONSTRAINED: a witness is a
  // file, and a hand edit reaches it exactly as it reaches any other.
}

pred doIdle {
  Sys.act' = Idle and Sys.res' = Environmental and no Sys.tgt' and noTreeChange
  noPending and no inFlight'
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
    Slot.occ' = Slot.occ
  }
}

/* The atomic same-directory rename that makes the witness visible.  One step,
   so no reader observes a torn or premature marker; reachable only from an OPEN
   transaction, so a witness never appears over a tree grove did not scaffold. */
pred doInitPublish {
  Sys.act' = InitPublish and no Sys.tgt'
  reservedRefusal
  (no Slot.occ and some inFlight and isPartialScaffold) implies {
    noPending
    Sys.res' = Applied
    Fmt.fmt' = CurrentFmt
    onDisk' = onDisk and nm' = nm and loc' = loc and dg' = dg
    Slot.occ' = Slot.occ
    no inFlight'
  }
  (no Slot.occ and not (some inFlight and isPartialScaffold)) implies
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
      no Slot.occ'
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

/* An OPEN transaction admits only its own next step or an interruption.  That
   is what makes `some inFlight` transient in the catalogue's sense: no ordinary
   invocation runs while it holds, so no ordinary invocation observes it. */
pred step {
  some inFlight implies (noReport and (doInitPublish or doCrash))
  no inFlight   implies ordinaryStep
}

fact Trace {
  Sys.act = Idle and Sys.res = Environmental and no Sys.tgt
  no Sys.got and no Sys.gotTerm
  no Sys.pending and no Sys.recov
  no inFlight
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

// --- TT-17: format is decided by the witness's content ---------------------

/* Two conjuncts, and the second is the falsifiable one.  The first says which
   family each witness content lands in; a classification that read a task
   entry's text would still satisfy it on the tree it was tuned for.  The second
   is what a hand edit cannot do: change every name in the tree, leave the
   witness alone, and the root does not move between families. */
check TT_17_format_is_decided_by_the_witness_content_alone {
  GroveGrammar implies always {
    no Slot.occ implies {
      (no Fmt.fmt)           iff (rootState in (LegacyR + PartialScaffoldR))
      (Fmt.fmt = ForeignFmt) iff (rootState = ForeignR)
      (Fmt.fmt = CurrentFmt) iff (rootState in (MalformedR + currentFamily))
    }
    (Sys.act' = HandEdit and Fmt.fmt' = Fmt.fmt and Slot.occ' = Slot.occ)
      implies familyOf[rootState'] = familyOf[rootState]
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 2 steps

/* The catalogue's witness: a LEGACY tree whose entries would otherwise read as
   current work.  Every entry is a canonical, known-kind, current-grammar name,
   the tree does not halt, and a reader that classified by looking at the entries
   would call it `Current(Live)`.  It is `Legacy`, because the witness says so
   and nothing else is consulted. */
run witness_TT_17_a_legacy_tree_whose_entries_read_as_current_work {
  GroveGrammar
  no Slot.occ and no Fmt.fmt
  not halted
  some liveOrdinary
  not isPartialScaffold
  rootState = LegacyR
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-18: classification order is fixed ----------------------------------

/* Reserved-witness classification precedes format classification, which
   precedes anything the walk derives — and `PartialScaffold` precedes `Legacy`.
   The last conjunct is the one that makes it an ORDER rather than a list: a
   root reaches `Malformed` only when the two classifications ahead of it have
   both passed, so a halted tree under a reserved witness is `Reserved`. */
check TT_18_classification_order_is_fixed {
  GroveGrammar implies always {
    some Slot.occ implies rootState = ReservedR
    (no Slot.occ and no Fmt.fmt)           implies rootState in (PartialScaffoldR + LegacyR)
    (no Slot.occ and Fmt.fmt = ForeignFmt) implies rootState = ForeignR
    (no Slot.occ and isPartialScaffold)    implies rootState = PartialScaffoldR
    rootState = MalformedR implies (rootClear and halted)
    rootState in currentFamily implies (rootClear and not halted)
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* The catalogue's witness: a tree carrying BOTH a reserved witness and no
   format witness, reported as the former.  Format classification would call it
   `Legacy` and the walk would call it live; neither runs. */
run witness_TT_18_a_reserved_witness_over_a_witnessless_root_reports_reserved {
  GroveGrammar
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
check TT_19_a_reserved_witness_refuses_everything_but_its_matching_recovery {
  GroveGrammar implies always (
    some Slot.occ implies {
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
  GroveGrammar
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
  GroveGrammar
  eventually (Sys.act' = RecoverPublished and Sys.res' = Applied
              and Slot.occ = Published and after no Slot.occ)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-20: the format witness lands last ----------------------------------

/* Four conjuncts.  The first two are *lands last*: the scaffold step never
   publishes, and publication happens only onto a complete scaffold.  The third
   is *no premature marker* — while the transaction is open there is no witness
   to observe, torn or otherwise.  The fourth is what the interruption LEAVES:
   never `Current(*)`, and never `Legacy`. */
check TT_20_the_format_witness_lands_last {
  GroveGrammar implies always {
    (Sys.act' = InitScaffold and Sys.res' = Applied) implies no Fmt.fmt'
    (Sys.act' = InitPublish and Sys.res' = Applied) implies {
      isPartialScaffold and no Fmt.fmt
      after (Fmt.fmt = CurrentFmt)
    }
    some inFlight implies no Fmt.fmt
    (no inFlight and no Slot.occ and no Fmt.fmt and isPartialScaffold) implies {
      rootState = PartialScaffoldR
      rootState not in currentFamily
      rootState != LegacyR
    }
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps

/* The catalogue's witness: scaffold, interrupt, and the STABLE root the
   interruption left.  What makes the last state stable rather than transient is
   the crash — the transaction is closed, so an ordinary invocation may observe
   it — and that is `EN-08`'s whole content.  Three states exactly: it finds no
   instance at 2. */
run witness_TT_20_an_interrupted_initialisation_leaves_a_partial_scaffold {
  GroveGrammar
  eventually {
    Sys.act = InitScaffold and Sys.res = Applied and some inFlight
    Sys.act' = Crash
    after (no inFlight and no Fmt.fmt and isPartialScaffold
           and rootState = PartialScaffoldR)
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The other half, so the check above is not green for want of an initialisation
   that ever completes: scaffold, publish, and a `Current(Live)` root.  Three
   FOUR states, and the reason is one more than the interrupted case needs: the
   crash IS the third state, while the publish must be followed by a state in
   which the published root is observed.  It finds no instance at 3. */
run witness_TT_20_an_uninterrupted_initialisation_publishes_the_witness {
  GroveGrammar
  eventually (Sys.act' = InitPublish and Sys.res' = Applied
              and after (Fmt.fmt = CurrentFmt and rootState = CurrentLiveR))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps


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
