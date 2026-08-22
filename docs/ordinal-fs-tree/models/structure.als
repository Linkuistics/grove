/*
 * ordinal-fs-tree — structural model
 * ==================================
 *
 * The subject is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections *The model*,
 * *Names belong to the consumer*, *The seam* and *Invariants*.  What an
 * operation *does* belongs to the behavioural model; this file asks only
 * whether the shape is coherent and whether it can represent what the
 * operations need.
 *
 * HOW TO READ IT
 *
 *   Nothing that the document merely *claims* is a `fact`.  Claims are named
 *   predicates, and every command says which ones it assumes.  That is what
 *   lets one file hold two things at once:
 *
 *     check X    -- must find NO counterexample.  These are the corrected
 *                   design's properties.
 *     run witness_X
 *                -- must find an instance.  Each one is a defect the model
 *                   found in the document as it stood, reproducible on demand.
 *                   `witness_` is the naming convention the runner keys on.
 *
 *   Facts hold only what is true by construction in Rust — a `Verdict` enum is
 *   total and disjoint, a filesystem directory cannot hold two entries of the
 *   same name — plus the shape of a filesystem tree.
 *
 * Run it with `./run-alloy.sh` in this directory.
 */
module structure

open util/ordering[Ordinal]


// ---------------------------------------------------------------------------
// Vocabulary
//
// Ordinal is ordered because *density* is a claim about ordering; nothing else
// in this file depends on the order.  Key, Label and Attrs are opaque atoms:
// the library compares them and never interprets them.
// ---------------------------------------------------------------------------

sig Ordinal {}
sig Key     {}
sig Label   {}
sig Attrs   {}

abstract sig Species {}
one sig LeafS, NodeS, DistS extends Species {}

/* "label plus domain attributes", opaque to the library.  `pSpecies` encodes
   the document's claim that *the species follows from the parts*. */
sig Parts {
  pLabel:   one Label,
  pAttrs:   one Attrs,
  pSpecies: one Species
}

/* What a directory listing reports about an object, before anything parses its
   name.  KOther covers symlinks and everything the document lumps with them. */
abstract sig FileKind {}
one sig KFile, KDir, KOther extends FileKind {}

/* A bare filename.  Opaque: the library never looks inside one. */
sig Filename {}

/* A value of the consumer's name type.  The four accessors of `EntryName` are
   `lone` because the trait returns `Option` for three of them. */
sig Name {
  nOrd:     lone Ordinal,
  nKey:     lone Key,
  nParts:   lone Parts,
  nSpecies: one  Species,
  shown:    one  Filename          // the Display impl
}

/* `Verdict<N, E>`.  Modelled as a sum type, which is what it is in Rust. */
abstract sig Verdict {}
one sig ForeignV, MalformedV, ReservedV extends Verdict {}
sig EntryV extends Verdict { seen: one Name }

/* The trait impl, as one object: two constructors and one classifier.
   `parseR` takes the on-disk kind as well as the filename.  The trait as
   written does *not* — that restriction is the predicate `ParseIgnoresKind`
   below, so the model can hold both versions and compare them. */
one sig Trait {
  composeR: Ordinal -> Key -> Parts -> Name,
  dist:     lone Name,
  parseR:   Filename -> FileKind -> Verdict
}

fact ByConstruction {
  // `parse` returns exactly one Verdict: totality and disjointness of the
  // trichotomy are guaranteed by the return *type*, not by any invariant.
  all f: Filename, k: FileKind | one Trait.parseR[f][k]
  // `compose` returns a value, not a set.  (Totality is `ComposeTotal`, which
  // is a claim about the signature and is asserted where it is needed.)
  all o: Ordinal, k: Key, p: Parts | lone Trait.composeR[o][k][p]
  // one verdict atom per name, so instances stay legible
  all disj a, b: EntryV | a.seen != b.seen
  all v: Verdict | v in Trait.parseR[Filename][FileKind]
}


// ---------------------------------------------------------------------------
// The laws the document states about the trait
// ---------------------------------------------------------------------------

/* "The species follows from the parts."  And, by the document's account of the
   distinguished child, a name without parts is the distinguished one. */
pred SpeciesFromParts {
  all n: Name | some n.nParts implies n.nSpecies = n.nParts.pSpecies
  all n: Name | no   n.nParts implies n.nSpecies = DistS
}

/* "A distinguished child carries neither an ordinal nor a key." */
pred DistLawful {
  no Trait.dist.nOrd
  no Trait.dist.nKey
  all n: Trait.dist | n.nSpecies = DistS
}

/* Every name the consumer's own code can hand the library. */
fun producible: set Name { Trait.composeR[Ordinal][Key][Parts] + Trait.dist }

/* "For every entry name the consumer can produce, parse(format(n)) yields that
   same name."  The document states this without reference to the on-disk kind. */
pred RoundTripDisplay {
  all n: producible | some v: EntryV | v in Trait.parseR[n.shown][FileKind] and v.seen = n
}

/* The trait as written: `fn parse(name: &str) -> Verdict`.  The signature takes
   the filename and nothing else, so the verdict cannot depend on what is
   actually on disk under that name. */
pred ParseIgnoresKind {
  all f: Filename, k1, k2: FileKind | Trait.parseR[f][k1] = Trait.parseR[f][k2]
}

pred StatedTraitLaws {
  SpeciesFromParts
  DistLawful
  RoundTripDisplay
  ParseIgnoresKind
}


// ---------------------------------------------------------------------------
// The laws the document does NOT state, and needs
// ---------------------------------------------------------------------------

/* `compose` must actually place its arguments in the name it returns.  Without
   this the "isomorphic to a triple" claim is empty and every derived operation
   — the sibling shift above all — is free to destroy identity. */
pred ComposeLawful {
  all o: Ordinal, k: Key, p: Parts, n: Trait.composeR[o][k][p] |
    n.nOrd = o and n.nKey = k and n.nParts = p
}

/* `compose` is infallible and total: `fn compose(..) -> Self`. */
pred ComposeTotal {
  all o: Ordinal, k: Key, p: Parts | one Trait.composeR[o][k][p]
}

/* A name that is not the distinguished one carries all three of ordinal, key
   and parts.  The trait returns `Option` for each, and the document never says
   when each is `Some`.

   DISCHARGED AT THE BOUNDARY, like `SpeciesAgreementIsParsed`.  The Rust seam
   folds the three accessors into one — `fn triple(&self) -> Option<Triple<'_,
   Parts>>` — so a name carrying some of the three and not the others cannot be
   written, and no implementation can violate this law.  It stays an assumed
   predicate here rather than a `fact` because the model is about the design and
   not about Rust, and because the defect it closes is real in any language whose
   name type is three independent optional fields: that is what
   `witness_leaf_name_without_an_ordinal` still exhibits.  See
   `docs/formalism-findings.md` entry 004. */
pred PositionedNamesAreComplete {
  all n: Name | n != Trait.dist implies (some n.nOrd and some n.nKey and some n.nParts)
}

/* The grammar is canonical: `format(parse(f)) = f`.  The document states the
   round trip in one direction only, which leaves a grammar free to accept two
   spellings of one name — and then two on-disk entries *are* the same entry,
   with the same key, at the same ordinal. */
pred ParseIsCanonical {
  all f: Filename, k: FileKind, v: (Trait.parseR[f][k] & EntryV) | v.seen.shown = f
}

/* `distinguished()` returns *the* distinguished name, so no other name may
   claim that species.  This is what makes "at most one distinguished child per
   node" true, given that a directory cannot hold two entries of one name. */
pred OneDistinguishedName {
  all n: Name | n.nSpecies = DistS implies n = Trait.dist
}

/* Which on-disk kind each species requires. */
pred agreesWith[s: Species, k: FileKind] {
  (s = LeafS and k = KFile) or (s = NodeS and k = KDir) or (s = DistS and k = KFile)
}

/* Species agreement, discharged where it can be: `parse` sees the on-disk kind
   and refuses to return `Entry` when the name contradicts it. */
pred SpeciesAgreementIsParsed {
  all f: Filename, k: FileKind, v: Trait.parseR[f][k] |
    v in EntryV implies agreesWith[v.seen.nSpecies, k]
}

pred CorrectedTraitLaws {
  SpeciesFromParts
  DistLawful
  RoundTripDisplay
  ParseIsCanonical
  ComposeLawful
  PositionedNamesAreComplete
  OneDistinguishedName
  SpeciesAgreementIsParsed
  // ParseIgnoresKind is dropped: the corrected `parse` sees the on-disk kind.
}


// ---------------------------------------------------------------------------
// The filesystem
// ---------------------------------------------------------------------------

abstract sig FsObject { fname: one Filename, fkind: one FileKind }
sig FsFile  extends FsObject {}
sig FsDir   extends FsObject { kids: set FsObject }
sig FsOther extends FsObject {}                    // symlink, socket, …

one sig Tree { root: one FsDir }

fact Filesystem {
  all x: FsFile  | x.fkind = KFile
  all d: FsDir   | d.fkind = KDir
  all x: FsOther | x.fkind = KOther
  no  x: FsObject | x in x.^kids                    // acyclic
  all x: FsObject | lone kids.x                     // one parent at most
  all d: FsDir, f: Filename | lone (d.kids & fname.f)   // sibling names unique
  FsObject = Tree.root.*kids                        // nothing floating
  no kids.(Tree.root)                               // the root is the root
}

/* The verdict on an object, and the name it carries if it has one. */
fun vd: FsObject -> Verdict { { x: FsObject, v: Verdict | v in Trait.parseR[x.fname][x.fkind] } }
fun nm: FsObject -> Name    { { x: FsObject, n: Name |
                                  some v: EntryV | v in Trait.parseR[x.fname][x.fkind] and n = v.seen } }

/* A walk descends into the root and into recognised nodes.  Nothing else:
   foreign names are skipped entirely, and a distinguished child is content
   rather than a level of the tree. */
fun descendable: set FsDir { Tree.root + { d: FsDir | some d.nm and d.nm.nSpecies = NodeS } }
fun visited:     set FsObject { Tree.root.*(descendable <: kids) }
/* the directories a walk actually enters: descendable *and* reached */
fun descended:   set FsDir { descendable & visited }
fun entries:     set FsObject { { x: visited | x != Tree.root and some x.nm } }
fun positioned:  set FsObject { { x: entries | some x.nm.nOrd } }
fun levelOf[d: FsDir]: set FsObject { d.kids & entries }


// ---------------------------------------------------------------------------
// The invariants the document states, as far as they are structural
//
// Three of the document's eight are behavioural — subtree preservation under
// shift, identity preservation under promotion, plan atomicity — and belong to
// the behavioural model.  These five are the structural remainder.
// ---------------------------------------------------------------------------

/* "No key appears twice in a tree." */
pred KeysUnique {
  all disj a, b: entries |
    (some a.nm.nKey and some b.nm.nKey) implies a.nm.nKey != b.nm.nKey
}

/* "Within one node, no two children share an ordinal." */
pred OrdinalsDistinct {
  all d: descended, disj a, b: (levelOf[d] & positioned) | a.nm.nOrd != b.nm.nOrd
}

/* "Ordinals being exactly 1..n" — claimed to be preserved but not established. */
pred Dense {
  all d: descended | let os = (levelOf[d] & positioned).nm.nOrd | all o: os | o.prevs in os
}

/* "No recognised name is silently skipped": nothing in a traversed directory
   is left unclassified, and nothing recognised is passed over.  Malformed and
   Reserved halt, so a tree the library can work on carries neither. */
pred Operable { no x: visited | x.vd in (MalformedV + ReservedV) }

/* "A name declaring a leaf is a regular file; a name declaring a node is a
   directory."  As stated, this says nothing about the distinguished child. */
pred SpeciesAgreementStated {
  all x: entries | (x.nm.nSpecies = LeafS implies x.fkind = KFile)
               and (x.nm.nSpecies = NodeS implies x.fkind = KDir)
}

pred SpeciesAgreementCorrected {
  SpeciesAgreementStated
  all x: entries | x.nm.nSpecies = DistS implies x.fkind = KFile
}

/* Every structural invariant the document states, together. */
pred AllStatedInvariants {
  Operable
  KeysUnique
  OrdinalsDistinct
  SpeciesAgreementStated
  RoundTripDisplay
}


// ---------------------------------------------------------------------------
// What the library can name
//
// `Parts` is `Clone + Eq`.  The library can copy a Parts value and compare two
// of them; it has no constructor for one.  So the only names it can build are
// those `compose` yields from parts it already holds, plus `distinguished()`.
// ---------------------------------------------------------------------------

fun libraryCanBuild[known: set Parts]: set Name {
  Trait.composeR[Ordinal][Key][known] + Trait.dist
}


// ===========================================================================
// CHECKS — the corrected design.  Each must find no counterexample.
// ===========================================================================

/* The parse trichotomy is total and disjoint.  It holds with no invariant at
   all, because `Verdict` is a sum type: this is a guarantee the type system
   already gives, and the model adds nothing to it. */
assert TrichotomyIsTotalAndDisjoint {
  all x: FsObject | one x.vd
}
check TrichotomyIsTotalAndDisjoint for 4

/* At most one distinguished child per node — a theorem, not an invariant to
   enforce, given that `distinguished()` names one thing and a directory cannot
   hold two entries of one name. */
assert DistinguishedIsUniquePerNode {
  CorrectedTraitLaws implies
    all d: descended | lone { x: levelOf[d] | x.nm.nSpecies = DistS }
}
check DistinguishedIsUniquePerNode for 4

/* A sibling shift is `compose(new_ordinal, key, parts)`.  For that to be a
   shift and not a corruption, compose must preserve what it was given. */
assert ShiftPreservesIdentity {
  CorrectedTraitLaws implies
    all n: Name, o: Ordinal, m: Trait.composeR[o][n.nKey][n.nParts] |
      m.nOrd = o and m.nKey = n.nKey and m.nParts = n.nParts
}
check ShiftPreservesIdentity for 4

/* No name is both positioned and distinguished. */
assert PositionedAndDistinguishedAreDisjoint {
  CorrectedTraitLaws implies
    all n: Name | some n.nOrd implies n != Trait.dist
}
check PositionedAndDistinguishedAreDisjoint for 4

/* Promotion, with the node's parts supplied by the caller — the correction.
   The library needs no new trait method: it needs an argument.  Given parts
   that carry the leaf's own label and imply species Node, `compose` names the
   promoted node with its ordinal, its key and its label intact. */
assert PromoteCanNameItsOutputFromCallerParts {
  (CorrectedTraitLaws and ComposeTotal) implies
    all n: Name, p: Parts |
      (n.nSpecies = LeafS and p.pSpecies = NodeS and p.pLabel = n.nParts.pLabel) implies
        some m: Trait.composeR[n.nOrd][n.nKey][p] |
          m.nOrd = n.nOrd and m.nKey = n.nKey and
          m.nSpecies = NodeS and m.nParts.pLabel = n.nParts.pLabel
}
check PromoteCanNameItsOutputFromCallerParts for 3 but
  exactly 2 Ordinal, exactly 2 Key, exactly 2 Parts, 9 Name, 12 Verdict, 9 Filename

/* With `parse` seeing the on-disk kind, species agreement is discharged at the
   boundary and is a property of every tree the library will walk. */
assert SpeciesAgreementHoldsWhenParsed {
  CorrectedTraitLaws implies SpeciesAgreementCorrected
}
check SpeciesAgreementHoldsWhenParsed for 4

/* Nothing in a traversed directory escapes classification. */
assert NothingRecognisedIsSkipped {
  CorrectedTraitLaws and Operable implies
    all d: descended, x: d.kids | some x.nm or x.vd = ForeignV
}
check NothingRecognisedIsSkipped for 4


// ===========================================================================
// WITNESSES — each must find an instance.  Each is a defect in the document as
// it stood, or a structure the document deliberately admits.
// ===========================================================================

/* DEFECT.  The document never says that `compose` places its arguments in the
   name it returns.  Under everything it *does* say, a sibling shift can move
   an entry's key and attributes onto another entry's position. */
run witness_shift_corrupts_identity {
  StatedTraitLaws
  some n: Name, o: Ordinal, m: Trait.composeR[o][n.nKey][n.nParts] |
    m.nKey != n.nKey or m.nParts != n.nParts or m.nOrd != o
} for 4

/* DEFECT.  `promote` must name a node with the leaf's ordinal and key.  Its
   parts must imply species Node; the library's only Parts value is the leaf's,
   which implies Leaf.  Nothing the trait offers closes the gap — this is entry
   001's defect in a second place, and `distinguished()` does not reach it. */
run witness_promote_cannot_name_its_output {
  CorrectedTraitLaws and ComposeTotal
  some p: Parts | p.pSpecies = NodeS          // node-shaped parts do exist
  some n: Name | n.nSpecies = LeafS and
    no m: libraryCanBuild[Name.nParts] |      // every Parts value in the snapshot
      m.nOrd = n.nOrd and m.nKey = n.nKey and
      m.nSpecies = NodeS and m.nParts.pLabel = n.nParts.pLabel
} for 3 but exactly 2 Ordinal, exactly 2 Key, exactly 2 Parts, 9 Name, 12 Verdict, 9 Filename

/* DEFECT.  `by_label` cannot be implemented behind this seam.  The trait names
   no label type, so the only query the library can be handed is a `Parts`
   value, and Parts equality answers "same label" only when the label alone
   determines the parts — which contradicts parts being label *plus*
   attributes. */
run witness_by_label_is_not_computable {
  CorrectedTraitLaws
  AllStatedInvariants
  some disj a, b: entries |
    a.nm.nParts.pLabel = b.nm.nParts.pLabel and a.nm.nParts != b.nm.nParts
} for 5

/* DEFECT.  `parse` is given a filename and nothing else, so it cannot tell a
   name that declares a node from the directory-or-file it actually names.  The
   library detects the mismatch but has no `Self::Err` to report it with, and
   the invariant says such a name is malformed — a verdict nothing can issue. */
run witness_species_mismatch_is_unclassifiable {
  StatedTraitLaws
  Operable
  some x: entries | not agreesWith[x.nm.nSpecies, x.fkind]
} for 4

/* DEFECT.  Species agreement covers Leaf and Node and says nothing about the
   distinguished child, and a walk does not descend into one.  A distinguished
   child that is a directory therefore hides an entire subtree from every
   traversal while every stated invariant holds — the exact failure the parse
   trichotomy exists to prevent. */
run witness_distinguished_directory_hides_a_subtree {
  StatedTraitLaws
  AllStatedInvariants
  some x: entries | x.nm.nSpecies = DistS and x in FsDir and some x.kids and no (x.kids & visited)
} for 5

/* DEFECT.  Nothing forces `distinguished()` to be the only name of its species,
   so a node can hold two distinguished children while every stated invariant
   holds. */
run witness_two_distinguished_children {
  StatedTraitLaws
  AllStatedInvariants
  some d: descended | #{ x: levelOf[d] | x.nm.nSpecies = DistS } > 1
} for 5

/* ADMITTED.  A gapped level is well-formed.  Density is not an invariant of a
   tree; whether every operation preserves it is the behavioural model's
   question. */
run witness_gapped_level_is_wellformed {
  CorrectedTraitLaws
  AllStatedInvariants
  not Dense
} for 4

/* ADMITTED — and the document must say so.  Key uniqueness is a precondition on
   the tree the library is handed, not something it checks.  A hand-edited tree
   with a repeated key satisfies everything the library can observe, and
   `by_key` then has two answers. */
run witness_duplicate_keys_are_admitted {
  CorrectedTraitLaws
  Operable
  OrdinalsDistinct
  SpeciesAgreementCorrected
  RoundTripDisplay
  not KeysUnique
} for 4

/* ADMITTED — and untyped.  The root is a node with no ordinal, no key and no
   parts; no triple names it, and it is not an entry.  `ancestors` is
   documented as returning an entry's containing nodes "root-first", so its
   element type cannot be the entry type. */
run witness_root_is_a_node_that_is_not_an_entry {
  CorrectedTraitLaws
  AllStatedInvariants
  some entries
  no Tree.root.nm
} for 4

/* DEFECT.  The round trip is stated in one direction only, so nothing forbids
   two filenames from parsing to the same name.  Then two on-disk entries *are*
   one entry: same key, same ordinal, same everything — and the tree carries a
   duplicate key that no stated invariant rules out. */
run witness_two_filenames_name_one_entry {
  StatedTraitLaws
  Operable
  some disj a, b: entries | a.nm = b.nm
} for 4

/* DEFECT.  `ordinal()`, `key()` and `parts()` all return `Option`, and the
   document never says when each is `Some`.  A name of species Leaf with no
   ordinal is therefore admitted — an entry that cannot be ordered, shifted or
   promoted, and that no triple names. */
run witness_leaf_name_without_an_ordinal {
  StatedTraitLaws
  Operable
  some x: entries | x.nm.nSpecies = LeafS and no x.nm.nOrd
} for 4


// ===========================================================================
// VACUITY GUARDS
//
// Every `check` above has the form `Laws implies P`.  If `Laws` were
// unsatisfiable each one would pass for no reason at all, so each law bundle
// is shown to admit a populated tree, at the same scope its checks run under.
// ===========================================================================

/* The corrected design admits the document's own example: a node with a
   distinguished child and ordered children, dense, with every corrected law
   and every stated invariant holding. */
run witness_corrected_design_is_satisfiable {
  CorrectedTraitLaws
  AllStatedInvariants
  SpeciesAgreementCorrected
  Dense
  some x: entries | x.nm.nSpecies = NodeS
  some x: entries | x.nm.nSpecies = DistS
  some x: entries | x.nm.nSpecies = LeafS
} for 6 but exactly 3 Ordinal, 3 Key, 3 Parts, 3 Label, 3 Attrs, 6 Name

/* The same, at the tighter scope the promotion checks use — where `compose`
   must be total, so the name space is fixed by the scope arithmetic. */
run witness_promote_scope_is_populated {
  CorrectedTraitLaws and ComposeTotal
  some p: Parts | p.pSpecies = NodeS
  some p: Parts | p.pSpecies = LeafS
  some n: Name | n.nSpecies = LeafS
} for 3 but exactly 2 Ordinal, exactly 2 Key, exactly 2 Parts, 9 Name, 12 Verdict, 9 Filename
