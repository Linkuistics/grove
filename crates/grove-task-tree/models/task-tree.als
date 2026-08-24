/*
 * grove-task-tree — the task-tree claims, in Alloy 6
 * ==================================================
 *
 * The subject is `docs/specs/semantic-contract.md`, section *Claims — task
 * tree*.  Nothing else: no Rust module, no helper, no control-flow shape.  Every
 * command below names an OBLIGATION of that document, and the repository runner
 * reads the obligation list out of the document rather than out of this file.
 *
 * COVERAGE SO FAR: TT-01 .. TT-10.  TT-11 .. TT-25 are the two sibling leaves'
 * (`selection`, `guarding`); the runner reports their cells empty, which is the
 * truth about this file rather than a defect in it.
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

pred GroveGrammar { ParseIsCanonical and GrammarIsTotal }

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
fun allKeys:           set Int { entries.nm.fKey }

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

pred halted { rMalformedEntry or rSpeciesMismatch or rPositionsNotGapless
              or rKeyReissued or rNodeWithoutCharter }
pred treeOk { not halted }


// ===========================================================================
// ACTIONS, OUTCOMES, AND THE ALGEBRAIC BOUNDARY
//
// Every action is TOTAL: it returns exactly one outcome, and a guard that fails
// produces a named refusal rather than an absent transition.
// ===========================================================================

abstract sig Action {}
one sig Idle, AddLeaf, InsertLeaf, Decompose, Retire, Prune, HandEdit extends Action {}

abstract sig Result {}
one sig Applied extends Result {}
/* the catalogue's closed refusal reasons, restricted to those a task-tree
   mutation in this slice can produce */
abstract sig Refused extends Result {}
one sig RefMalformed, RefNotAnEntry, RefNotLive, RefAlreadyTerminal,
        RefReservedKind extends Refused {}
/* The algebra's own refusal, opaque.  TT-10 is the claim that no ordinary
   argument reaches it, because grove's preconditions run in front. */
one sig AlgebraicRefusal extends Result {}
one sig Environmental extends Result {}

one sig Sys {
  var act: one Action,
  var res: one Result,
  var tgt: lone Obj
}

/* What `ordinal-fs-tree` itself would refuse, given the argument as handed to
   it.  Assumed, not implemented: this is the imported boundary. */
pred algebraWouldRefuse[a: Action, t: Obj] {
  (a in (InsertLeaf + Decompose + Retire + Prune) and t not in entries)
  or (a = AddLeaf and t not in nodeDirs)
}

pred noTreeChange { onDisk' = onDisk and nm' = nm and loc' = loc and dg' = dg }

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
    some dgt: Digest | dg' = dg ++ (o -> dgt)
  }
}

// --- insert-leaf: insert, shifting later siblings ---------------------------

pred doInsertLeaf[t: Obj, o: Obj, f: Filename] {
  Sys.act' = InsertLeaf and Sys.tgt' = t
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
      some dgt: Digest | dg' = dg ++ (o -> dgt)
    }
  }
}

// --- decompose-leaf: promotion ---------------------------------------------

pred doDecompose[t: Obj, n: Obj, c: Obj, k: Obj, nf, kf: Filename] {
  Sys.act' = Decompose and Sys.tgt' = t
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
    some a, b: Digest |
      dg' = (dg - (t -> Digest)) ++ (c -> t.dg) ++ (n -> a) ++ (k -> b)
  }
  (not halted and t in entries and t.nm.fSpec = LeafSp and t.nm.fOut in terminalInfix)
      implies (Sys.res' = RefAlreadyTerminal and noTreeChange)
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
    nm' = nm ++ (t -> g)
  }
}

// --- the world's own actions ------------------------------------------------

/* EN-11: any well-formed tree is reachable by hand edit.  Unconstrained beyond
   the filesystem facts, and that is the point: it is how a witness posits a
   tree grove's own actions could not build. */
pred doHandEdit {
  Sys.act' = HandEdit and Sys.res' = Environmental and no Sys.tgt'
}

pred doIdle { Sys.act' = Idle and Sys.res' = Environmental and no Sys.tgt' and noTreeChange }

/* The fresh objects an operation introduces are quantified at their SPECIES
   rather than over `Obj`.  It changes no meaning — the bodies already require
   it — and it is the difference between `5^4` and `2 * 3 * 3` combinations for
   the promotion disjunct, which is most of what the solver spends its time on.
   The TARGET stays `Obj`, because an action must be total over what an operator
   can name: a node handed to `retire` has to reach its refusal. */
pred step {
  doIdle
  or doHandEdit
  or (some d: Obj, o: FileObj, f: Filename | doAddLeaf[d, o, f])
  or (some t: Obj, o: FileObj, f: Filename | doInsertLeaf[t, o, f])
  or (some t: Obj, n: DirObj, disj c, k: FileObj, nf, kf: Filename |
        doDecompose[t, n, c, k, nf, kf])
  or (some t: Obj, i: Infix, g: Filename | doRewrite[t, i, g])
}

fact Trace {
  Sys.act = Idle and Sys.res = Environmental and no Sys.tgt
  always step
}

/* A grove action, as against the world's. */
fun groveActs: set Action { AddLeaf + InsertLeaf + Decompose + Retire + Prune }
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
  GroveGrammar implies (all disj f, g: entryName | not sameReading[f, g])
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* DEFECT, without the ADR's rule.  Two spellings both parse, so two files in
   one directory are the same entry: same key, same position, same everything. */
run witness_TT_01a_two_spellings_would_both_parse {
  StatedGrammar
  some disj a, b: entries | a.nm != b.nm and sameReading[a.nm, b.nm]
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* TT-01.b.  Parse-then-render reproduces the input exactly, and any other
   spelling of the same reading is refused with the canonical one to hand. */
check TT_01b_a_noncanonical_spelling_is_refused_naming_the_canonical_one {
  GroveGrammar implies {
    all f: entryName | f.canon = f
    all f: shaped | (f.canon != f and (f.fSpec = NodeSp or f.fKind in known)) implies
      (f in malformedName and f.canon in entryName and sameReading[f, f.canon])
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

run witness_TT_01b_a_noncanonical_spelling_refused {
  GroveGrammar
  some o: visited - TaskRoot | o.nm in malformedName and o.nm.canon in entryName
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-02: a name declares its species and must be it ---------------------

/* TT-02.a.  A leaf name at a directory is MALFORMED — read as work and then
   refused — rather than foreign, which would be skipped.  The distinction is
   the whole claim: a skipped directory takes its live subtree with it. */
check TT_02a_a_leaf_name_at_a_directory_is_malformed_not_foreign {
  GroveGrammar implies always (
    (some o: (visited & DirObj) - TaskRoot | o.nm in entryName and o.nm.fSpec = LeafSp)
      implies (rSpeciesMismatch and halted
               and (Sys.act' in groveActs implies (Sys.res' = RefMalformed and noTreeChange))))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_02a_leaf_name_at_a_directory {
  GroveGrammar
  some o: (visited & DirObj) - TaskRoot | o.nm in entryName and o.nm.fSpec = LeafSp
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

/* TT-02.b.  The converse: a node name at a file. */
check TT_02b_a_node_name_at_a_file_is_malformed_not_foreign {
  GroveGrammar implies always (
    (some o: (visited & FileObj) | o.nm in entryName and o.nm.fSpec = NodeSp)
      implies (rSpeciesMismatch and halted
               and (Sys.act' in groveActs implies (Sys.res' = RefMalformed and noTreeChange))))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_02b_node_name_at_a_file {
  GroveGrammar
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
  GroveGrammar implies always (
    halted implies (Sys.act' in groveActs implies (Sys.res' = RefMalformed and noTreeChange)))
} for 3 but 4 Int, 3 FileObj, 2 DirObj, 5 Filename, 2 Slug, 2 Digest, 3 steps

/* The case where SKIPPING would report a finished grove, and the `visited` rule
   sharpens it: a malformed node directory is not descended into, so the live
   work inside it is invisible to the walk — `liveLeaves` is empty and a reader
   that merely skipped the directory would call the grove done.  What prevents
   that is not the walk but the halt: the directory is itself an entry at its
   parent's level, its name is malformed, and the whole tree stops. */
run witness_TT_03_a_malformed_node_hides_live_work {
  GroveGrammar
  some d: (visited & DirObj) - TaskRoot | {
    d.nm in malformedName
    some o: kidsOf[d] | o.nm in entryName and o.nm.fSpec = LeafSp and o.nm.fOut = LiveI
  }
  no liveLeaves
  halted
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

// --- TT-04: foreign entries are ignored and preserved ----------------------

check TT_04_foreign_entries_are_ignored_and_preserved {
  GroveGrammar implies {
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
  GroveGrammar
  some d: (onDisk & DirObj) - TaskRoot, o: onDisk |
    d.nm in foreignName and o.loc = d and o.nm in entryName
    and o not in entries and o.nm.fKey not in allKeys
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 1 steps

run witness_TT_04_foreign_survives_a_sibling_rename {
  GroveGrammar
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
  GroveGrammar implies always (
    (Sys.act' = AddLeaf and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_05_keys_never_reissued_on_insert {
  GroveGrammar implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The standard bound rather than the promotion witnesses' wider one: three
   files and two directories is exactly what a promotion needs — the leaf, its
   charter, its first child, the task root and the new node — and the wider
   bound did not finish in five minutes. */
check TT_05_keys_never_reissued_on_promotion {
  GroveGrammar implies always (
    (Sys.act' = Decompose and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_05_keys_never_reissued_on_rewrite {
  GroveGrammar implies always (
    (Sys.act' in (Retire + Prune) and Sys.res' = Applied) implies keysArePermanent)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* The witness the catalogue asks for: an allocation whose maximum comes from a
   TERMINAL entry — which is why retirement is a rename and never a removal. */
run witness_TT_05_allocation_max_comes_from_a_terminal_entry {
  GroveGrammar
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied and
    (some t: entries | t.nm.fOut in terminalInfix and t.nm.fKey = max[allKeys]))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-06: positions are per-directory and gapless ------------------------

check TT_06a_append_lands_at_n_plus_one_and_closes_no_gap {
  GroveGrammar implies always (
    (Sys.act' = AddLeaf and Sys.res' = Applied) implies {
      all o: onDisk' - onDisk | {
        all s: entriesIn[o.loc'] | s.nm.fPos < o.nm'.fPos
        after gaplessAt[o.loc]          // the level it landed on, not every level
      }
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_06a_append_lands_at_n_plus_one {
  GroveGrammar
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied and
    (some o: onDisk' - onDisk | o.nm'.fPos > 1
       and (all s: entriesIn[o.loc'] | s.nm.fPos < o.nm'.fPos)))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_06b_insert_shifts_every_later_sibling {
  GroveGrammar implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies
      (let t = Sys.tgt', d = t.loc, p = t.nm.fPos | {
         all s: entriesIn[d] | s.nm.fPos >= p implies s.nm'.fPos = plus[s.nm.fPos, 1]
         all s: entriesIn[d] | s.nm.fPos <  p implies s.nm' = s.nm
         after gaplessAt[d]             // the level it shifted, not every level
       }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_06b_insert_at_an_occupied_position_shifts {
  GroveGrammar
  eventually (Sys.act' = InsertLeaf and Sys.res' = Applied and
    (some s: entriesIn[Sys.tgt'.loc] | s.nm.fPos >= Sys.tgt'.nm.fPos and s != Sys.tgt'))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-07: a shift preserves everything but position ----------------------

check TT_07_a_shift_changes_only_positions {
  GroveGrammar implies always (
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
  GroveGrammar
  eventually (Sys.act' = InsertLeaf and Sys.res' = Applied and
    (let d = Sys.tgt'.loc | {
       some s: entriesIn[d] | s.nm.fSpec = NodeSp
       some s: entriesIn[d] | s.nm.fSpec = LeafSp and s.nm.fOut in terminalInfix
       some s: foreignEntries | s.loc = d
     }))
} for 6 but 4 Int, 5 FileObj, 2 DirObj, 8 Filename, 2 Slug, 3 Digest, 3 steps

// --- TT-08: decomposition preserves the key --------------------------------

check TT_08_decomposition_preserves_the_key {
  GroveGrammar implies always (
    (Sys.act' = Decompose and Sys.res' = Applied) implies
      (let t = Sys.tgt' | {
         some n: onDisk' - onDisk | n.nm'.fKey = t.nm.fKey and n.nm'.fPos = t.nm.fPos
                                    and n.nm'.fSlug = t.nm.fSlug and n.nm'.fSpec = NodeSp
         allKeys in allKeys'
         all s: onDisk - t | s.nm' = s.nm and s.loc' = s.loc and s.dg' = s.dg
       }))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_08_promotion_of_the_maximum_key {
  GroveGrammar
  eventually (Sys.act' = Decompose and Sys.res' = Applied and
              Sys.tgt'.nm.fKey = max[allKeys])
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-09: one algebraic operation plus a domain precondition -------------

check TT_09a_append_adds_exactly_one_entry_and_renames_nothing {
  GroveGrammar implies always (
    (Sys.act' = AddLeaf and Sys.res' = Applied) implies {
      one onDisk' - onDisk
      no  onDisk - onDisk'
      all s: onDisk | s.nm' = s.nm and s.loc' = s.loc and s.dg' = s.dg
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09a_append { GroveGrammar and eventually (Sys.act' = AddLeaf and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_09b_insert_adds_exactly_one_entry_and_removes_none {
  GroveGrammar implies always (
    (Sys.act' = InsertLeaf and Sys.res' = Applied) implies {
      one onDisk' - onDisk
      no  onDisk - onDisk'
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09b_insert { GroveGrammar and eventually (Sys.act' = InsertLeaf and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_09c_promotion_replaces_exactly_the_target {
  GroveGrammar implies always (
    (Sys.act' = Decompose and Sys.res' = Applied) implies {
      onDisk - onDisk' = Sys.tgt'
      #(onDisk' - onDisk) = 3          // the node, its charter, its first child
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09c_promotion { GroveGrammar and eventually (Sys.act' = Decompose and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

check TT_09d_rewrite_renames_exactly_one_entry {
  GroveGrammar implies always (
    (Sys.act' in (Retire + Prune) and Sys.res' = Applied) implies {
      onDisk' = onDisk
      one s: onDisk | s.nm' != s.nm
      all s: onDisk | s.loc' = s.loc and s.dg' = s.dg
    })
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

run witness_TT_09d_rewrite { GroveGrammar and eventually (Sys.act' in (Retire + Prune) and Sys.res' = Applied) }
  for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// --- TT-10: no algebraic refusal reaches an operator -----------------------

check TT_10_no_algebraic_refusal_reaches_an_operator {
  GroveGrammar implies always {
    Sys.res' != AlgebraicRefusal
    (Sys.act' in groveActs and algebraWouldRefuse[Sys.act', Sys.tgt'])
      implies Sys.res' in Refused
  }
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

/* An argument the algebra itself would have refused, shown pre-empted by
   grove's own precondition: the operator sees a refusal this catalogue names. */
run witness_TT_10_an_algebraic_refusal_is_preempted {
  GroveGrammar
  eventually (Sys.act' in groveActs and algebraWouldRefuse[Sys.act', Sys.tgt']
              and Sys.res' in Refused)
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps

// ===========================================================================
// VACUITY GUARDS
//
// Every check above has the form `GroveGrammar implies P`.  If the law bundle
// were unsatisfiable over a populated tree, every one of them would pass for no
// reason at all.
// ===========================================================================

run witness_vacuity_the_law_bundle_admits_a_working_grove {
  GroveGrammar
  eventually (Sys.act' = AddLeaf and Sys.res' = Applied)
  eventually (Sys.act' in (Retire + Prune) and Sys.res' = Applied)
  some entries
  treeOk
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 8 Filename, 2 Slug, 3 Digest, 4 steps
