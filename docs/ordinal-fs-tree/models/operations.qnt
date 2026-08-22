// ===========================================================================
// ordinal-fs-tree — behavioural model
// ===========================================================================
//
// The subject is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections *Operations*,
// *How an operation runs*, *Refusals* and the [B]-tagged *Invariants*.  Its
// companion is `structure.als`, which settled what a well-formed tree *is*;
// this file asks only whether the operations preserve it.
//
// HOW TO READ IT
//
//   Nothing the document merely *claims* is baked into the transition
//   relation.  Claims are named `val`s, and each is either
//
//     an INVARIANT   -- must hold in every reachable state.  Run with
//                       `--invariant`.  These are the corrected design's
//                       properties.
//     a WITNESS      -- must be *reached*.  Run with `--witnesses`.  Each one
//                       exhibits a refusal that must be live, or a defect the
//                       document had before the model was written.  A witness
//                       reached in 0 traces is a finding: the case is dead.
//
//   The naming convention the runner keys on: `inv_…` must hold, `wit_…` must
//   be witnessed.  `run-quint.sh` in this directory runs both, per instance.
//
// WHAT IS AND IS NOT MODELLED
//
//   * The plan interpreter IS a state machine.  A mutation is not atomic in
//     the model: effects land one at a time and any one of them can fail, so
//     intermediate states are reachable and every invariant is evaluated at
//     each of them.  That is the whole point — three of the document's
//     invariants are two-state claims that `structure.als` could not state.
//   * `Parts` is an OPAQUE atom (an int with no structure).  The only thing
//     anything reads out of it is its species, via `PARTS_SPECIES`, which is
//     the trait law *the species follows from the parts* — not introspection.
//     If an action ever needed to look further inside, the seam would have
//     leaked; nothing here can, because there is nothing inside to look at.
//   * Filenames are NOT modelled.  `format` and `parse` are inverse on
//     producible names (structure.als: `ParseIsCanonical` + `RoundTripDisplay`),
//     so distinct names are distinct filenames and a name collision *is* a
//     filename collision.  The state holds no strings.
//   * The filesystem below the plan interpreter is NOT modelled: `rename(2)`
//     carrying a subtree is an assumption, not a checked property.
//   * Walk *order* is not modelled (reachability is).  `by_key`'s documented
//     "first in walk order" tie-break on a corrupted tree is therefore not
//     checked; the model picks the least id.  See the handoff note at the end.
//
// ===========================================================================

module operations {

  // -------------------------------------------------------------------------
  // Vocabulary
  //
  // An `EntryId` is the model's identity for an object on disk — an inode.  It
  // is NOT a domain concept and NOT the key: a rename moves a *name*, not an
  // entry, and only a stable identity lets a two-state property say "*this*
  // entry kept its key" rather than "*some* entry has that key".
  // -------------------------------------------------------------------------

  type EntryId = int

  /// The tree root: a node that is not an entry.  It has no ordinal, no key
  /// and no parts, and its own name is never parsed.
  pure val ROOT: EntryId = 0

  type Species = LeafS | NodeS | DistS

  /// A name, as the trait exposes it.  Modelled as a sum type rather than
  /// three `Option`s deliberately: `structure.als` found that three
  /// independent `Option`s admit a leaf with no ordinal, and entry 002's
  /// counterfactual is that the type system should forbid it outright.
  type Name =
    | Positioned({ ord: int, key: int, parts: int })
    | Dist

  /// What the consumer's `parse` made of one directory entry.  `Malformed` and
  /// `Reserved` are merged into `Broken`: they differ only in the advice they
  /// carry, and both halt.
  type ObjKind =
    | Named(Name)
    | Foreign
    | Broken

  type Object = { parent: EntryId, kind: ObjKind }

  /// The filesystem, plus its inode allocator.
  type Fs = { objects: EntryId -> Object, nextId: EntryId }

  /// Which entry an operation is aimed at.  The document never says how a
  /// caller designates one; this model designates by key, which is the
  /// document's own advice ("the only thing safe to store in a durable
  /// cross-reference").  The root has no key, hence its own variant.
  type Target = Root | ByKey(int)

  // -------------------------------------------------------------------------
  // Plans and effects
  // -------------------------------------------------------------------------

  /// A primitive effect.  `Remove` never appears in a forward plan — it is
  /// only ever generated as the undo of a `Create`, which is the whole content
  /// of "rollback removes only entries the run itself created".
  type Effect =
    | Create({ id: EntryId, parent: EntryId, name: Name })
    | MoveTo({ id: EntryId, parent: EntryId, name: Name })
    | Remove({ id: EntryId })

  /// Every operation is total: the algebra returns a decision for every input.
  type Decision =
    | Refuse(Outcome)
    | Proceed(List[Effect])

  /// Every stated outcome of an operation.  One variant per refusal in the
  /// document's *Refusals* section, plus the snapshot halt and the two
  /// failure paths.  A refusal with no witness is dead code in the design.
  type Outcome =
    | NotRun
    | Succeeded
    | RefusedTargetMissing
    | RefusedTargetNotNode
    | RefusedNoOccupantAtOrdinal
    | RefusedPromoteNotLeaf
    | RefusedPromoteNoDistinguished
    | RefusedPromotePartsNotNode
    | RefusedRewriteSpeciesChange
    | RefusedDestinationOccupied
    | FailedDestinationOccupied
    | HaltedUnparseable
    | FailedRolledBack
    | FailedPartiallyRolledBack

  /// What the caller asked for, kept so the two-state properties can be stated
  /// against the operation that actually ran.
  type OpTag =
    | TagNone
    | TagAppend({ target: Target, parts: int })
    | TagAppendMany({ target: Target, parts: List[int] })
    | TagInsert({ target: Target, at: int, parts: int })
    | TagPromote({ key: int, parts: int, withChild: bool })
    | TagRewrite({ key: int, parts: int })
    | TagHandEdit

  type Phase =
    | Idle
    | Applying({ pending: List[Effect], undo: List[Effect] })
    | Unwinding({ undo: List[Effect], reason: Outcome })

  /// The operation in flight.  `before` and `created` are ghost fields: they
  /// exist only so the two-state and atomicity properties can be stated.
  type OpState = {
    phase:   Phase,
    outcome: Outcome,
    tag:     OpTag,
    before:  EntryId -> Object,
    created: Set[EntryId],
  }

  // -------------------------------------------------------------------------
  // Scope and configuration
  // -------------------------------------------------------------------------

  /// The domain's parts atoms.  Opaque: an int with no structure, and the
  /// only projection anything takes out of one is its species.
  const PARTS: Set[int]

  /// Which of them imply species `Node`; the rest imply `Leaf`.  Together
  /// these two constants ARE the trait law *the species follows from the
  /// parts* — not introspection: the library asks the NAME for its species,
  /// and this is what the name answers with.
  const NODE_PARTS: Set[int]

  /// Whether `distinguished()` is `Some`.  `false` models a domain with no
  /// distinguished child, where promotion is refused rather than guessed at.
  const HAS_DISTINGUISHED: bool

  /// What a human with `mv` is allowed to do to the tree between operations.
  ///   NoEdits        — only the library ever writes.  A pristine tree.
  ///   WellFormedEdits— hand edits that keep every stated invariant.  Gaps.
  ///   CorruptingEdits— hand edits that break key uniqueness, which the
  ///                    library admits and never checks.
  const HAND_EDITS: bool

  /// Additionally allow an edit that breaks key uniqueness — which the library
  /// admits and never checks.
  const CORRUPTING_EDITS: bool

  /// Additionally allow a name the consumer recognises and cannot parse.  It
  /// is terminal — every later operation halts on it — so it is off except in
  /// the instance that exists to reach that halt.
  const BROKEN_EDITS: bool

  /// Whether an effect may fail with an I/O error.  A failure is enabled at
  /// every step, so leaving it on starves every multi-effect path under random
  /// simulation; the instances that check ordering turn it off and the ones
  /// that check atomicity turn it on.  A scope control, not a design choice.
  const IO_CAN_FAIL: bool

  /// The order a sibling shift renames in.  The document claims highest-first
  /// is load-bearing; lowest-first is here so the claim can be tested rather
  /// than restated.
  const HIGHEST_FIRST: bool

  /// Whether an unwind step may itself fail.  The document promises rollback
  /// for reported errors and says nothing about a rollback that fails, so this
  /// is off by default and on in one instance, to exhibit the gap.
  const ROLLBACK_CAN_FAIL: bool

  /// Scope bounds.  These bound the *exploration*, not the design.
  const MAX_OBJECTS: int
  const MAX_ORD: int
  const MAX_DEPTH: int

  pure val LEAF_PARTS: Set[int] = PARTS.exclude(NODE_PARTS)

  /// The trait law, as one function.  This is the ONLY thing read out of a
  /// parts atom anywhere in this model.  If an action ever needed more, the
  /// seam would have leaked — and there is nothing more to read.
  pure def speciesOfParts(p: int): Species = if (NODE_PARTS.contains(p)) NodeS else LeafS

  // -------------------------------------------------------------------------
  // State
  // -------------------------------------------------------------------------

  var fs: Fs
  var op: OpState

  /// Ghost.  Every key that has appeared in a *quiescent* tree.  This is what
  /// makes "no key is ever reissued" statable at all — it is a trace property,
  /// which `structure.als` recorded as one it could not pose.
  var history: Set[int]

  // -------------------------------------------------------------------------
  // Name accessors
  // -------------------------------------------------------------------------

  pure def isPositioned(n: Name): bool =
    match n { Positioned(_) => true | Dist => false }

  pure def ordOf(n: Name): int =
    match n { Positioned(p) => p.ord | Dist => -1 }

  pure def keyOf(n: Name): int =
    match n { Positioned(p) => p.key | Dist => -1 }

  pure def partsOf(n: Name): int =
    match n { Positioned(p) => p.parts | Dist => -1 }

  /// The species of a name.  The library asks the *name*, never the parts;
  /// `PARTS_SPECIES` is the trait's law, not a peek inside the seam.
  pure def speciesOf(n: Name): Species =
    match n { Positioned(p) => speciesOfParts(p.parts) | Dist => DistS }

  /// `compose(ordinal, key, parts)`.  It places what it is given — that is the
  /// trait obligation `ComposeLawful`, and here it is true by construction,
  /// which is exactly what the obligation buys.
  pure def compose(o: int, k: int, p: int): Name =
    Positioned({ ord: o, key: k, parts: p })

  // -------------------------------------------------------------------------
  // Reading the tree
  // -------------------------------------------------------------------------

  pure def objs(f: Fs): EntryId -> Object = f.objects
  pure def has(f: Fs, i: EntryId): bool = f.objects.keys().contains(i)

  pure def kindOf(f: Fs, i: EntryId): ObjKind = f.objects.get(i).kind
  pure def parentOf(f: Fs, i: EntryId): EntryId = f.objects.get(i).parent

  pure def isEntry(f: Fs, i: EntryId): bool =
    has(f, i) and match kindOf(f, i) { Named(_) => true | Foreign => false | Broken => false }

  pure def nameAt(f: Fs, i: EntryId): Name =
    match kindOf(f, i) { Named(n) => n | Foreign => Dist | Broken => Dist }

  /// A directory a walk descends into: the root, or a recognised node.  Not a
  /// foreign name (disclaimed, and the disclaimer is recursive), and not a
  /// distinguished child (content, not a level).
  pure def isDescendable(f: Fs, i: EntryId): bool =
    i == ROOT or (isEntry(f, i) and speciesOf(nameAt(f, i)) == NodeS)

  pure def childIds(f: Fs, d: EntryId): Set[EntryId] =
    f.objects.keys().filter(i => parentOf(f, i) == d)

  /// Everything the snapshot sees.  Bounded unrolling stands in for the
  /// fixpoint Quint cannot express recursively; MAX_DEPTH >= MAX_OBJECTS makes
  /// the bound unreachable.
  pure def visited(f: Fs): Set[EntryId] =
    range(0, MAX_DEPTH).foldl(Set(ROOT), (acc, _) =>
      acc.union(acc.filter(d => isDescendable(f, d)).map(d => childIds(f, d)).flatten()))

  pure def entries(f: Fs): Set[EntryId] =
    visited(f).filter(i => i != ROOT and isEntry(f, i))

  pure def positionedIn(f: Fs, d: EntryId): Set[EntryId] =
    childIds(f, d).filter(i => isEntry(f, i) and isPositioned(nameAt(f, i)))

  pure def keysOf(f: Fs): Set[int] =
    entries(f).filter(i => isPositioned(nameAt(f, i))).map(i => keyOf(nameAt(f, i)))

  /// `max(key over the whole tree) + 1`.  The names *are* the counter.
  pure def freshKey(f: Fs): int =
    keysOf(f).fold(0, (a, k) => if (k > a) k else a) + 1

  pure def maxOrdIn(f: Fs, d: EntryId): int =
    positionedIn(f, d).fold(0, (a, i) => val o = ordOf(nameAt(f, i)) { if (o > a) o else a })

  pure def idsAtOrdinal(f: Fs, d: EntryId, o: int): Set[EntryId] =
    positionedIn(f, d).filter(i => ordOf(nameAt(f, i)) == o)

  /// `by_key`.  Deterministic pick of the least id where the tree carries a
  /// duplicate key — see the header note: walk order is not modelled.
  pure def idsWithKey(f: Fs, k: int): Set[EntryId] =
    entries(f).filter(i => isPositioned(nameAt(f, i)) and keyOf(nameAt(f, i)) == k)

  pure def leastId(s: Set[EntryId]): EntryId =
    s.fold(-1, (a, i) => if (a == -1 or i < a) i else a)

  pure def resolve(f: Fs, t: Target): EntryId =
    match t { Root => ROOT | ByKey(k) => leastId(idsWithKey(f, k)) }

  /// A destination is occupied when a *different* object in that directory
  /// already carries that name.  Excluding the mover is not decoration: a
  /// rewrite whose new parts equal the old is a rename onto itself, and
  /// without the exclusion the library would refuse a no-op.
  pure def occupied(f: Fs, d: EntryId, n: Name, mover: EntryId): bool =
    childIds(f, d).exists(j =>
      j != mover and isEntry(f, j) and nameAt(f, j) == n)

  // -------------------------------------------------------------------------
  // The snapshot
  //
  // Every mutation begins by reading the tree's names.  A name the consumer
  // recognises as its own must parse completely or halt the operation — so a
  // `Broken` name anywhere the walk reaches refuses the whole operation, not
  // just the level it sits in.  Snapshot scope is whole-tree, so this is
  // whole-tree.
  // -------------------------------------------------------------------------

  pure def snapshotHalts(f: Fs): bool =
    visited(f).exists(i => i != ROOT and kindOf(f, i) == Broken)

  // -------------------------------------------------------------------------
  // The algebra — pure, and unable to reach the filesystem
  // -------------------------------------------------------------------------

  pure def reverseL(l: List[Effect]): List[Effect] =
    l.foldl(List(), (acc, x) => List(x).concat(acc))

  pure def reverseI(l: List[int]): List[int] =
    l.foldl(List(), (acc, x) => List(x).concat(acc))

  /// The ids of siblings at or after `at`, ordered by ordinal.  Highest-first
  /// is the document's rule; lowest-first is the alternative under test.
  pure def shiftIds(f: Fs, d: EntryId, at: int): List[EntryId] =
    // Bounded by the level's OWN maximum, never by a scope constant: a shift
    // pushes ordinals past any fixed bound, and a scope constant used here
    // would silently drop the siblings above it from the plan.
    val asc = range(at, maxOrdIn(f, d) + 1).foldl(List(), (acc, o) =>
      idsAtOrdinal(f, d, o).fold(acc, (a, i) => a.append(i)))
    if (HIGHEST_FIRST) reverseI(asc) else asc

  /// A shift is not an operation: it is `compose(new_ordinal, key, parts)`,
  /// derived, and therefore incapable of disturbing a key or a part.
  pure def shiftEffect(f: Fs, d: EntryId, i: EntryId): Effect =
    val n = nameAt(f, i)
    MoveTo({ id: i, parent: d, name: compose(ordOf(n) + 1, keyOf(n), partsOf(n)) })

  /// Whether a plan can be applied: the algebra folds it through the snapshot,
  /// so it meets each destination in the state the INTERPRETER will meet it.
  ///
  /// Checking every destination against the snapshot instead — the obvious
  /// reading of "a pure function of the snapshot" — is not merely stricter:
  /// it refuses correct inserts, and it makes the shift order buy nothing,
  /// because highest-first and lowest-first are then refused in exactly the
  /// same cases.  Sequencing the check is what gives the ordering rule
  /// anything to do.
  pure def planIsApplicable(f: Fs, plan: List[Effect]): bool =
    plan.foldl((f, true), (acc, e) =>
      if (not(acc._2)) acc
      else if (effectBlocked(acc._1, e)) (acc._1, false)
      else (applyEffect(acc._1, e), true))._2

  pure def guardDestinations(f: Fs, plan: List[Effect]): Decision =
    if (planIsApplicable(f, plan)) Proceed(plan) else Refuse(RefusedDestinationOccupied)

  // --- append ---------------------------------------------------------------

  pure def planAppend(f: Fs, t: Target, p: int): Decision =
    val d = resolve(f, t)
    if (d == -1) Refuse(RefusedTargetMissing)
    else if (not(isDescendable(f, d))) Refuse(RefusedTargetNotNode)
    else guardDestinations(f, List(Create({
      id: f.nextId, parent: d, name: compose(maxOrdIn(f, d) + 1, freshKey(f), p) })))

  // --- append_many ----------------------------------------------------------

  pure def planAppendMany(f: Fs, t: Target, ps: List[int]): Decision =
    val d = resolve(f, t)
    if (d == -1) Refuse(RefusedTargetMissing)
    else if (not(isDescendable(f, d))) Refuse(RefusedTargetNotNode)
    else
      val o0 = maxOrdIn(f, d)
      val k0 = freshKey(f)
      val plan = ps.indices().fold(List(), (acc, ix) =>
        acc.append(Create({ id: f.nextId + ix, parent: d,
                            name: compose(o0 + ix + 1, k0 + ix, ps.nth(ix)) })))
      guardDestinations(f, plan)

  // --- insert ---------------------------------------------------------------

  pure def planInsert(f: Fs, t: Target, at: int, p: int): Decision =
    val d = resolve(f, t)
    if (d == -1) Refuse(RefusedTargetMissing)
    else if (not(isDescendable(f, d))) Refuse(RefusedTargetNotNode)
    // "insert requires an existing occupant at the target ordinal".  This
    // covers past-the-end AND a gap in a hand-edited level — the document
    // names only the first, and gives a rationale that does not fit the second.
    else if (idsAtOrdinal(f, d, at).size() == 0) Refuse(RefusedNoOccupantAtOrdinal)
    else
      val shifts = shiftIds(f, d, at).foldl(List(), (acc, i) => acc.append(shiftEffect(f, d, i)))
      guardDestinations(f, shifts.append(Create({
        id: f.nextId, parent: d, name: compose(at, freshKey(f), p) })))

  // --- promote --------------------------------------------------------------

  pure def planPromote(f: Fs, k: int, p: int, withChild: bool): Decision =
    val i = resolve(f, ByKey(k))
    if (i == -1) Refuse(RefusedTargetMissing)
    else if (speciesOf(nameAt(f, i)) != LeafS) Refuse(RefusedPromoteNotLeaf)
    else if (not(HAS_DISTINGUISHED)) Refuse(RefusedPromoteNoDistinguished)
    else if (speciesOfParts(p) != NodeS) Refuse(RefusedPromotePartsNotNode)
    else
      val n = nameAt(f, i)
      val d = parentOf(f, i)
      val nodeId = f.nextId
      // The node is a NEW directory; the leaf's inode survives as its
      // distinguished child.  Identity is preserved in the KEY, not the inode:
      // the entity is unchanged, its container is new.
      val base = List(
        Create({ id: nodeId, parent: d, name: compose(ordOf(n), keyOf(n), p) }),
        MoveTo({ id: i, parent: nodeId, name: Dist }))
      val plan = if (withChild)
        base.append(Create({ id: f.nextId + 1, parent: nodeId,
                             name: compose(1, freshKey(f), LEAF_PARTS.fold(0, (a, x) => if (a == 0) x else a)) }))
        else base
      guardDestinations(f, plan)

  // --- rewrite --------------------------------------------------------------

  pure def planRewrite(f: Fs, k: int, p: int): Decision =
    val i = resolve(f, ByKey(k))
    if (i == -1) Refuse(RefusedTargetMissing)
    else
      val n = nameAt(f, i)
      if (speciesOfParts(p) != speciesOf(n)) Refuse(RefusedRewriteSpeciesChange)
      else guardDestinations(f, List(MoveTo({
        id: i, parent: parentOf(f, i), name: compose(ordOf(n), keyOf(n), p) })))

  // --- the one entry point --------------------------------------------------

  /// Totality, as a type.  `decide` returns a `Decision` for every state and
  /// every tag; there is no stuck case by construction.  What the witnesses
  /// then test is whether every branch is *live*.
  pure def decide(f: Fs, tag: OpTag): Decision =
    if (snapshotHalts(f)) Refuse(HaltedUnparseable)
    else match tag {
      | TagNone            => Refuse(NotRun)
      | TagHandEdit        => Refuse(NotRun)
      | TagAppend(a)       => planAppend(f, a.target, a.parts)
      | TagAppendMany(a)   => planAppendMany(f, a.target, a.parts)
      | TagInsert(a)       => planInsert(f, a.target, a.at, a.parts)
      | TagPromote(a)      => planPromote(f, a.key, a.parts, a.withChild)
      | TagRewrite(a)      => planRewrite(f, a.key, a.parts)
    }

  // -------------------------------------------------------------------------
  // The interpreter
  // -------------------------------------------------------------------------

  pure def applyEffect(f: Fs, e: Effect): Fs =
    match e {
      | Create(c) => { objects: f.objects.put(c.id, { parent: c.parent, kind: Named(c.name) }),
                       nextId: if (c.id >= f.nextId) c.id + 1 else f.nextId }
      | MoveTo(m) => { ...f, objects: f.objects.set(m.id, { parent: m.parent, kind: Named(m.name) }) }
      | Remove(r) => { ...f, objects: f.objects.keys().filter(i => i != r.id).mapBy(i => f.objects.get(i)) }
    }

  /// The undo of an effect, captured against the state *before* it lands.
  pure def undoOf(f: Fs, e: Effect): Effect =
    match e {
      | Create(c) => Remove({ id: c.id })
      | MoveTo(m) => MoveTo({ id: m.id, parent: parentOf(f, m.id), name: nameAt(f, m.id) })
      | Remove(r) => Remove(r)
    }

  /// The interpreter claims each destination exclusively.  The algebra already
  /// checked the snapshot, so this can only fire if the *plan* creates a
  /// collision with itself — which is precisely what the shift order is
  /// claimed to prevent.
  pure def effectBlocked(f: Fs, e: Effect): bool =
    match e {
      | Create(c) => occupied(f, c.parent, c.name, -1)
      | MoveTo(m) => occupied(f, m.parent, m.name, m.id)
      | Remove(_) => false
    }

  pure def effectCreates(e: Effect): Set[EntryId] =
    match e { Create(c) => Set(c.id) | MoveTo(_) => Set() | Remove(_) => Set() }

  // -------------------------------------------------------------------------
  // Well-formedness, as the structural model settled it
  // -------------------------------------------------------------------------

  pure def keysUnique(f: Fs): bool =
    entries(f).forall(a => entries(f).forall(b =>
      (a != b and isPositioned(nameAt(f, a)) and isPositioned(nameAt(f, b)))
        implies keyOf(nameAt(f, a)) != keyOf(nameAt(f, b))))

  pure def ordinalsDistinct(f: Fs): bool =
    visited(f).filter(d => isDescendable(f, d)).forall(d =>
      positionedIn(f, d).forall(a => positionedIn(f, d).forall(b =>
        a != b implies ordOf(nameAt(f, a)) != ordOf(nameAt(f, b)))))

  pure def dense(f: Fs): bool =
    visited(f).filter(d => isDescendable(f, d)).forall(d =>
      val os = positionedIn(f, d).map(i => ordOf(nameAt(f, i)))
      os.forall(o => 1.to(o).subseteq(os)))

  /// The filesystem's own guarantee: a directory cannot hold two entries of
  /// one name.  Maintained by the interpreter, checked here because it is what
  /// makes *at most one distinguished child per node* a theorem.
  pure def siblingNamesUnique(f: Fs): bool =
    f.objects.keys().forall(a => f.objects.keys().forall(b =>
      (a != b and parentOf(f, a) == parentOf(f, b) and isEntry(f, a) and isEntry(f, b))
        implies nameAt(f, a) != nameAt(f, b)))

  /// A leaf is a regular file, so it holds nothing; so does a distinguished
  /// child.  Only nodes and the root have children.
  pure def onlyNodesHaveChildren(f: Fs): bool =
    f.objects.keys().forall(i =>
      childIds(f, i).size() > 0 implies
        (isEntry(f, i) and speciesOf(nameAt(f, i)) == NodeS))


  // -------------------------------------------------------------------------
  // Phase accessors
  // -------------------------------------------------------------------------

  pure def isIdle(p: Phase): bool =
    match p { Idle => true | Applying(a) => false | Unwinding(u) => false }

  pure def isApplying(p: Phase): bool =
    match p { Idle => false | Applying(a) => true | Unwinding(u) => false }

  pure def isUnwinding(p: Phase): bool =
    match p { Idle => false | Applying(a) => false | Unwinding(u) => true }

  pure def pendingOf(p: Phase): List[Effect] =
    match p { Idle => List() | Applying(a) => a.pending | Unwinding(u) => List() }

  pure def undoListOf(p: Phase): List[Effect] =
    match p { Idle => List() | Applying(a) => a.undo | Unwinding(u) => u.undo }

  pure def reasonOf(p: Phase): Outcome =
    match p { Idle => NotRun | Applying(a) => NotRun | Unwinding(u) => u.reason }

  pure def beforeFs(o: OpState): Fs = { objects: o.before, nextId: 0 }

  // -------------------------------------------------------------------------
  // Initial state
  //
  // `init` is a decision, not a detail.  The initial tree here is EMPTY, and
  // an arbitrary well-formed tree is reached instead through `handEdit…`
  // actions — a human with `mv`, which is the design's own premise ("a tree
  // you can read with `ls`, edit with `mv`").  Modelling it that way rather
  // than as a nondeterministic initial state is what makes the difference
  // between the two answers observable: with `HAND_EDITS = NoEdits` the
  // reachable trees are exactly those the library built, and density holds;
  // with hand edits it does not.  That difference IS the answer to whether a
  // hand-edited tree is in scope.
  // -------------------------------------------------------------------------

  action init: bool = all {
    fs' = { objects: Map(), nextId: 1 },
    op' = { phase: Idle, outcome: NotRun, tag: TagNone,
            before: Map(), created: Set() },
    history' = Set(),
  }

  // -------------------------------------------------------------------------
  // Beginning an operation
  //
  // Every operation is ENABLED for every argument.  A refusal is a transition
  // with a stated outcome, not a disabled action — that is what makes
  // totality checkable: an unmodelled case would have to show up as a missing
  // branch of `decide`, and `decide` is total by its return type.
  // -------------------------------------------------------------------------

  action beginOp(tag: OpTag): bool =
    val dec = decide(fs, tag)
    all {
      isIdle(op.phase),
      fs' = fs,
      history' = history.union(keysOf(fs)),
      op' = match dec {
        | Refuse(o)     => { phase: Idle, outcome: o, tag: tag,
                             before: fs.objects, created: Set() }
        | Proceed(plan) => { phase: Applying({ pending: plan, undo: List() }),
                             outcome: NotRun, tag: tag,
                             before: fs.objects, created: Set() }
      },
    }

  // -------------------------------------------------------------------------
  // Applying the plan, one effect at a time
  // -------------------------------------------------------------------------

  action applyStepOk: bool = all {
    isApplying(op.phase),
    pendingOf(op.phase).length() > 0,
    not(effectBlocked(fs, pendingOf(op.phase).head())),
    fs' = applyEffect(fs, pendingOf(op.phase).head()),
    op' = { ...op,
            phase: Applying({ pending: pendingOf(op.phase).tail(),
                              undo: List(undoOf(fs, pendingOf(op.phase).head()))
                                      .concat(undoListOf(op.phase)) }),
            created: op.created.union(effectCreates(pendingOf(op.phase).head())) },
    history' = history,
  }

  /// The interpreter claims each destination exclusively and finds it taken.
  /// The algebra already cleared every destination against the snapshot, so
  /// this can only fire when the plan collides with ITSELF — which is exactly
  /// what the shift order is claimed to prevent.
  action applyStepBlocked: bool = all {
    isApplying(op.phase),
    pendingOf(op.phase).length() > 0,
    effectBlocked(fs, pendingOf(op.phase).head()),
    fs' = fs,
    op' = { ...op, phase: Unwinding({ undo: undoListOf(op.phase),
                                      reason: FailedDestinationOccupied }) },
    history' = history,
  }

  /// An I/O error at step k.  Any step, any time.
  action applyStepFails: bool = all {
    IO_CAN_FAIL,
    isApplying(op.phase),
    pendingOf(op.phase).length() > 0,
    fs' = fs,
    op' = { ...op, phase: Unwinding({ undo: undoListOf(op.phase),
                                      reason: FailedRolledBack }) },
    history' = history,
  }

  action finishApply: bool = all {
    isApplying(op.phase),
    pendingOf(op.phase).length() == 0,
    fs' = fs,
    op' = { ...op, phase: Idle, outcome: Succeeded },
    history' = history,
  }

  // -------------------------------------------------------------------------
  // Unwinding
  // -------------------------------------------------------------------------

  action unwindStepOk: bool = all {
    isUnwinding(op.phase),
    undoListOf(op.phase).length() > 0,
    not(effectBlocked(fs, undoListOf(op.phase).head())),
    fs' = applyEffect(fs, undoListOf(op.phase).head()),
    op' = { ...op, phase: Unwinding({ undo: undoListOf(op.phase).tail(),
                                      reason: reasonOf(op.phase) }) },
    history' = history,
  }

  /// An undo whose own destination is taken.  If this is ever witnessed the
  /// document's rollback promise is unsound; if it never is, the promise
  /// holds for the reason the plan order gives.
  action unwindStepBlocked: bool = all {
    isUnwinding(op.phase),
    undoListOf(op.phase).length() > 0,
    effectBlocked(fs, undoListOf(op.phase).head()),
    fs' = fs,
    op' = { ...op, phase: Idle, outcome: FailedPartiallyRolledBack },
    history' = history,
  }

  /// A rollback that itself hits an I/O error.  The document is silent on
  /// this; the instance `rollback_fails` turns it on to show what it leaves.
  action unwindStepFails: bool = all {
    ROLLBACK_CAN_FAIL,
    isUnwinding(op.phase),
    undoListOf(op.phase).length() > 0,
    fs' = fs,
    op' = { ...op, phase: Idle, outcome: FailedPartiallyRolledBack },
    history' = history,
  }

  action finishUnwind: bool = all {
    isUnwinding(op.phase),
    undoListOf(op.phase).length() == 0,
    fs' = fs,
    op' = { ...op, phase: Idle, outcome: reasonOf(op.phase) },
    history' = history,
  }

  action interpreterStep: bool = any {
    applyStepOk, applyStepBlocked, applyStepFails, finishApply,
    unwindStepOk, unwindStepBlocked, unwindStepFails, finishUnwind,
  }

  // -------------------------------------------------------------------------
  // The library's own steps
  // -------------------------------------------------------------------------

  /// Targets the caller can name: the root, every key in the tree, and one
  /// key that is not — so `RefusedTargetMissing` is reachable.
  val targetChoices: Set[Target] =
    Set(Root).union(keysOf(fs).map(k => ByKey(k))).union(Set(ByKey(99)))

  val keyChoices: Set[int] = keysOf(fs).union(Set(99))

  action libraryStep: bool =
    nondet t  = targetChoices.oneOf()
    nondet p  = PARTS.oneOf()
    nondet p2 = PARTS.oneOf()
    nondet at = 1.to(MAX_ORD).oneOf()
    nondet k  = keyChoices.oneOf()
    nondet wc = Set(false, true).oneOf()
    all {
      fs.objects.keys().size() < MAX_OBJECTS,
      any {
        beginOp(TagAppend({ target: t, parts: p })),
        beginOp(TagAppendMany({ target: t, parts: List(p, p2) })),
        beginOp(TagInsert({ target: t, at: at, parts: p })),
        beginOp(TagPromote({ key: k, parts: p, withChild: wc })),
        beginOp(TagRewrite({ key: k, parts: p })),
      },
    }

  // -------------------------------------------------------------------------
  // A human with `mv`
  //
  // Hand edits run only between operations.  The lock is advisory and `mv`
  // does not take it, so a hand edit DURING an apply is physically possible
  // and deliberately out of scope: see the handoff note.
  // -------------------------------------------------------------------------

  action handEditAt(d: EntryId, o: int, p: int): bool = all {
    fs' = applyEffect(fs, Create({ id: fs.nextId, parent: d,
                                   name: compose(o, freshKey(fs), p) })),
    op' = { phase: Idle, outcome: Succeeded, tag: TagHandEdit,
            before: fs.objects, created: Set(fs.nextId) },
    history' = history.union(keysOf(fs)),
  }

  action handEditOther(d: EntryId, k: ObjKind): bool = all {
    fs' = { objects: fs.objects.put(fs.nextId, { parent: d, kind: k }),
            nextId: fs.nextId + 1 },
    op' = { phase: Idle, outcome: Succeeded, tag: TagHandEdit,
            before: fs.objects, created: Set(fs.nextId) },
    history' = history.union(keysOf(fs)),
  }

  /// `cp 01-foo-k5.md 02-foo-k5.md` — two siblings sharing a key AND parts at
  /// adjacent ordinals.  This is the only tree shape in which a sibling shift
  /// can collide, which is what makes the shift order load-bearing.
  action handEditClone(i: EntryId): bool =
    val n = nameAt(fs, i)
    val d = parentOf(fs, i)
    all {
      idsAtOrdinal(fs, d, ordOf(n) + 1).size() == 0,
      ordOf(n) + 1 <= MAX_ORD,
      fs' = applyEffect(fs, Create({ id: fs.nextId, parent: d,
                                     name: compose(ordOf(n) + 1, keyOf(n), partsOf(n)) })),
      op' = { phase: Idle, outcome: Succeeded, tag: TagHandEdit,
              before: fs.objects, created: Set(fs.nextId) },
      history' = history.union(keysOf(fs)),
    }

  action handEditStep: bool =
    nondet d = visited(fs).filter(x => isDescendable(fs, x)).oneOf()
    nondet o = 1.to(MAX_ORD).oneOf()
    nondet p = PARTS.oneOf()
    nondet i = entries(fs).filter(x => isPositioned(nameAt(fs, x))).union(Set(-1)).oneOf()
    all {
      isIdle(op.phase),
      HAND_EDITS,
      fs.objects.keys().size() < MAX_OBJECTS,
      any {
        all { idsAtOrdinal(fs, d, o).size() == 0, handEditAt(d, o, p) },
        handEditOther(d, Foreign),
        all { BROKEN_EDITS, handEditOther(d, Broken) },
        all { CORRUPTING_EDITS, i != -1, handEditClone(i) },
      },
    }

  action step: bool = any {
    interpreterStep,
    libraryStep,
    handEditStep,
  }

  // ===========================================================================
  // INVARIANTS — each must hold in every reachable state
  // ===========================================================================

  val atRest: bool = isIdle(op.phase)
  val succeeded: bool = atRest and op.outcome == Succeeded

  pure def isPromote(t: OpTag): bool =
    match t { TagPromote(a) => true | TagNone => false | TagAppend(a) => false
            | TagAppendMany(a) => false | TagInsert(a) => false
            | TagRewrite(a) => false | TagHandEdit => false }

  pure def isInsert(t: OpTag): bool =
    match t { TagInsert(a) => true | TagNone => false | TagAppend(a) => false
            | TagAppendMany(a) => false | TagPromote(a) => false
            | TagRewrite(a) => false | TagHandEdit => false }

  pure def isAppendMany(t: OpTag): bool =
    match t { TagAppendMany(a) => true | TagNone => false | TagAppend(a) => false
            | TagInsert(a) => false | TagPromote(a) => false
            | TagRewrite(a) => false | TagHandEdit => false }

  /// True exactly while a promotion is between its two effects.
  val midPromote: bool = not(atRest) and isPromote(op.tag)

  // --- structural, at every state, mid-apply included -----------------------

  /// The filesystem's own guarantee.  It is what makes *at most one
  /// distinguished child per node* a theorem rather than an invariant.
  val inv_siblingNamesUnique: bool = siblingNamesUnique(fs)

  val inv_onlyNodesHaveChildren: bool = onlyNodesHaveChildren(fs)

  val inv_noFloatingObjects: bool =
    fs.objects.keys().forall(i =>
      parentOf(fs, i) == ROOT or fs.objects.keys().contains(parentOf(fs, i)))

  // --- the shift order ------------------------------------------------------

  /// THE ordering property.  Every intermediate state of an apply has distinct
  /// ordinals — so an operation interrupted by process death leaves a tree
  /// that is at worst GAPPED, which the design admits, rather than one with a
  /// duplicate ordinal, which it does not.
  ///
  /// A promotion is the stated exception: it must create the node before it
  /// can move the leaf's content into it, and the node carries the leaf's own
  /// ordinal and key, so both entries exist at once.  See `wit_promote…`.
  val inv_ordinalsDistinctThroughout: bool = midPromote or ordinalsDistinct(fs)

  /// Nothing a plan does ever finds its destination taken.  Under highest-first
  /// this holds even of a tree carrying a duplicated key; under lowest-first it
  /// fails on exactly those trees.  That is the whole content of the ordering
  /// claim — and note WHICH outcome fails: the refusal is raised by the
  /// ALGEBRA, which can predict the collision, not by the interpreter.
  val inv_destinationNeverOccupied: bool = and {
    op.outcome != RefusedDestinationOccupied,
    op.outcome != FailedDestinationOccupied,
    reasonOf(op.phase) != FailedDestinationOccupied,
  }

  /// The interpreter's own exclusive create never fires.  The algebra folds the
  /// plan through the snapshot, so it already knows what every effect will
  /// meet — under the lock, this check is unreachable.  What keeps it in the
  /// code is the lock being ADVISORY: a writer that does not take it can move
  /// something into a destination between the snapshot and the apply.
  val inv_interpreterNeverFindsADestinationTaken: bool = and {
    op.outcome != FailedDestinationOccupied,
    reasonOf(op.phase) != FailedDestinationOccupied,
  }

  // --- at rest --------------------------------------------------------------

  val inv_ordinalsDistinctAtRest: bool = not(atRest) or ordinalsDistinct(fs)

  val inv_keysUniqueAtRest: bool = not(atRest) or keysUnique(fs)

  val inv_denseAtRest: bool = not(atRest) or dense(fs)

  // --- keys -----------------------------------------------------------------

  /// "No key is ever reissued."  A trace property, which the structural model
  /// recorded as one it could not pose.
  ///
  /// The property is about ALLOCATION, not creation, and the difference is not
  /// pedantry: `promote` creates a new directory that deliberately carries the
  /// promoted leaf's existing key, because the entity is unchanged and only
  /// its shape moved.  Stated over created *objects* the claim is false and
  /// the model says so; stated over allocated *keys* it holds.  A key a run
  /// created and then rolled back was never committed, so reissuing THAT one
  /// is correct and permitted.
  val inv_freshKeysAreFresh: bool =
    not(succeeded) or
      val carried = keysOf(beforeFs(op))
      fs.objects.keys().exclude(op.before.keys()).forall(i =>
        not(isEntry(fs, i)) or not(isPositioned(nameAt(fs, i)))
          or carried.contains(keyOf(nameAt(fs, i)))
          or not(history.contains(keyOf(nameAt(fs, i)))))

  // --- atomicity ------------------------------------------------------------

  /// "After a mutation returns an error, either every effect landed or none
  /// did."  This is the claim the document made in prose and nothing checked.
  val inv_atomicity: bool =
    not(atRest) or op.outcome != FailedRolledBack or fs.objects == op.before

  val inv_blockedIsAtomicToo: bool =
    not(atRest) or op.outcome != FailedDestinationOccupied or fs.objects == op.before

  /// "Rollback removes only entries the run itself created, so it cannot
  /// destroy something that was already there."
  val inv_rollbackRemovesOnlyItsOwn: bool =
    undoListOf(op.phase).foldl(true, (acc, e) => acc and
      match e { Remove(r) => op.created.contains(r.id)
              | Create(c) => true | MoveTo(m) => true })

  // --- two-state properties -------------------------------------------------

  /// "An `insert` changes the ordinals of siblings at or after the target and
  /// *nothing else*.  Every key, every label, every attribute and every
  /// descendant of every shifted entry is bit-identical afterwards."
  val inv_insertOnlyShifts: bool =
    not(succeeded) or not(isInsert(op.tag)) or
      match op.tag {
        | TagInsert(a) =>
            val bf = beforeFs(op)
            val d = resolve(bf, a.target)
            and {
              bf.objects.keys().subseteq(fs.objects.keys()),
              bf.objects.keys().forall(i =>
                val bn = nameAt(bf, i)
                if (parentOf(bf, i) == d and isEntry(bf, i)
                    and isPositioned(bn) and ordOf(bn) >= a.at)
                  fs.objects.get(i) == { parent: d,
                    kind: Named(compose(ordOf(bn) + 1, keyOf(bn), partsOf(bn))) }
                else
                  fs.objects.get(i) == bf.objects.get(i)),
              fs.objects.keys().exclude(bf.objects.keys()).size() == 1,
              fs.objects.keys().exclude(bf.objects.keys()).forall(i =>
                parentOf(fs, i) == d and isEntry(fs, i)
                and ordOf(nameAt(fs, i)) == a.at
                and partsOf(nameAt(fs, i)) == a.parts),
            }
        | TagNone => true | TagAppend(a) => true | TagAppendMany(a) => true
        | TagPromote(a) => true | TagRewrite(a) => true | TagHandEdit => true
      }

  /// "A promoted leaf keeps its ordinal and its key.  The entry that was a
  /// leaf *is* the node."  True of the ENTITY; the inode is new, and the
  /// leaf's own inode survives as the distinguished child holding its content.
  val inv_promoteKeepsIdentity: bool =
    not(succeeded) or not(isPromote(op.tag)) or
      match op.tag {
        | TagPromote(a) =>
            val bf = beforeFs(op)
            val i = resolve(bf, ByKey(a.key))
            val bn = nameAt(bf, i)
            val d = parentOf(bf, i)
            and {
              // the leaf's inode is now the distinguished child of a node
              isEntry(fs, i),
              nameAt(fs, i) == Dist,
              // ...which sits where the leaf sat, with the leaf's ordinal
              // and the leaf's key
              isEntry(fs, parentOf(fs, i)),
              parentOf(fs, parentOf(fs, i)) == d,
              ordOf(nameAt(fs, parentOf(fs, i))) == ordOf(bn),
              keyOf(nameAt(fs, parentOf(fs, i))) == keyOf(bn),
              speciesOf(nameAt(fs, parentOf(fs, i))) == NodeS,
              // nothing else moved
              bf.objects.keys().forall(j =>
                j == i or fs.objects.get(j) == bf.objects.get(j)),
            }
        | TagNone => true | TagAppend(a) => true | TagAppendMany(a) => true
        | TagInsert(a) => true | TagRewrite(a) => true | TagHandEdit => true
      }

  /// "`rewrite` replaces an entry's parts, keeping its ordinal, key and
  /// species."
  val inv_rewriteKeepsPlace: bool =
    not(succeeded) or
      match op.tag {
        | TagRewrite(a) =>
            val bf = beforeFs(op)
            val i = resolve(bf, ByKey(a.key))
            val bn = nameAt(bf, i)
            and {
              fs.objects.keys() == bf.objects.keys(),
              parentOf(fs, i) == parentOf(bf, i),
              ordOf(nameAt(fs, i)) == ordOf(bn),
              keyOf(nameAt(fs, i)) == keyOf(bn),
              speciesOf(nameAt(fs, i)) == speciesOf(bn),
              partsOf(nameAt(fs, i)) == a.parts,
              bf.objects.keys().forall(j =>
                j == i or fs.objects.get(j) == bf.objects.get(j)),
            }
        | TagNone => true | TagAppend(a) => true | TagAppendMany(a) => true
        | TagInsert(a) => true | TagPromote(a) => true | TagHandEdit => true
      }

  /// `append` adds one entry at the end of a level and touches nothing else.
  val inv_appendOnlyAdds: bool =
    not(succeeded) or
      match op.tag {
        | TagAppend(a) =>
            val bf = beforeFs(op)
            val d = resolve(bf, a.target)
            and {
              bf.objects.keys().forall(j => fs.objects.get(j) == bf.objects.get(j)),
              fs.objects.keys().exclude(bf.objects.keys()).size() == 1,
              fs.objects.keys().exclude(bf.objects.keys()).forall(i =>
                parentOf(fs, i) == d
                and ordOf(nameAt(fs, i)) == maxOrdIn(bf, d) + 1
                and keyOf(nameAt(fs, i)) == freshKey(bf)),
            }
        | TagNone => true | TagInsert(a) => true | TagAppendMany(a) => true
        | TagPromote(a) => true | TagRewrite(a) => true | TagHandEdit => true
      }

  // ===========================================================================
  // WITNESSES — each must be REACHED.  A witness reached in 0 traces is a
  // finding: that case is dead in the design as modelled.
  // ===========================================================================

  val wit_succeeded: bool = succeeded

  // --- one per refusal in the document's *Refusals* section -----------------

  val wit_refusedTargetMissing: bool          = atRest and op.outcome == RefusedTargetMissing
  val wit_refusedTargetNotNode: bool          = atRest and op.outcome == RefusedTargetNotNode
  val wit_refusedNoOccupantAtOrdinal: bool    = atRest and op.outcome == RefusedNoOccupantAtOrdinal
  val wit_refusedPromoteNotLeaf: bool         = atRest and op.outcome == RefusedPromoteNotLeaf
  val wit_refusedPromoteNoDistinguished: bool = atRest and op.outcome == RefusedPromoteNoDistinguished
  val wit_refusedPromotePartsNotNode: bool    = atRest and op.outcome == RefusedPromotePartsNotNode
  val wit_refusedRewriteSpeciesChange: bool   = atRest and op.outcome == RefusedRewriteSpeciesChange
  val wit_refusedDestinationOccupied: bool    = atRest and op.outcome == RefusedDestinationOccupied
  val wit_haltedUnparseable: bool             = atRest and op.outcome == HaltedUnparseable
  val wit_failedRolledBack: bool              = atRest and op.outcome == FailedRolledBack
  val wit_failedPartiallyRolledBack: bool     = atRest and op.outcome == FailedPartiallyRolledBack
  val wit_planCollidesWithItself: bool        = atRest and op.outcome == FailedDestinationOccupied

  /// An insert refused because its plan cannot be applied in the order it was
  /// built.  Reachable ONLY under lowest-first, and only on a tree carrying two
  /// siblings that share a key and its parts.
  val wit_shiftOrderRefusesTheInsert: bool =
    atRest and op.outcome == RefusedDestinationOccupied and isInsert(op.tag)

  /// A refusal past the last sibling — `append`'s job, refused rather than
  /// quietly redirected.
  val wit_insertPastTheEnd: bool =
    atRest and op.outcome == RefusedNoOccupantAtOrdinal and
      match op.tag {
        | TagInsert(a) =>
            val bf = beforeFs(op)
            val d = resolve(bf, a.target)
            d != -1 and a.at > maxOrdIn(bf, d)
        | TagNone => false | TagAppend(a) => false | TagAppendMany(a) => false
        | TagPromote(a) => false | TagRewrite(a) => false | TagHandEdit => false
      }

  /// The SAME refusal, on a hole at or below the level's greatest ordinal.  The
  /// document names only the past-the-end case and gives it a rationale
  /// ("`append`'s job") that does not fit this one: no operation fills a hole.
  ///
  /// **This predicate discriminates and does not characterise.**  `a.at <
  /// maxOrdIn` separates this outcome from past-the-end and says nothing more:
  /// it is equally true of a hole *between* two occupants and of one *below
  /// every* occupant, since density is preserved and never established and
  /// `Ordinal::FIRST` is therefore no floor.  Prose derived from this witness
  /// may claim an occupant *above* the request and must not claim one below.
  /// An earlier reading of this comment as "a gap in the middle of a level"
  /// became a refusal message asserting both neighbours;
  /// `docs/formalism-findings.md` entry 013 records it.
  val wit_insertIntoAGap: bool =
    atRest and op.outcome == RefusedNoOccupantAtOrdinal and
      match op.tag {
        | TagInsert(a) =>
            val bf = beforeFs(op)
            val d = resolve(bf, a.target)
            d != -1 and a.at < maxOrdIn(bf, d)
        | TagNone => false | TagAppend(a) => false | TagAppendMany(a) => false
        | TagPromote(a) => false | TagRewrite(a) => false | TagHandEdit => false
      }

  // --- structures the design deliberately admits ----------------------------

  val wit_gappedLevel: bool = atRest and not(dense(fs))
  val wit_duplicateKeysAdmitted: bool = atRest and not(keysUnique(fs))

  // --- the intermediate states an operation passes through ------------------

  /// A promotion transiently carries TWO entries with the same key and the
  /// same ordinal: the leaf, and the node that is about to hold it.  Nothing
  /// in the document says so, and it is unavoidable — the node must exist
  /// before the leaf's content can move into it, and it must carry the leaf's
  /// ordinal and key or identity is not preserved.
  val wit_promoteTransientlyDuplicatesAKey: bool =
    not(atRest) and isPromote(op.tag) and not(keysUnique(fs))

  val wit_promoteTransientlyDuplicatesAnOrdinal: bool =
    not(atRest) and isPromote(op.tag) and not(ordinalsDistinct(fs))

  /// A sibling shift that has landed some of its renames and not the rest.
  /// Under `HIGHEST_FIRST` this state is gapped and well-formed.
  val wit_shiftPartiallyApplied: bool =
    not(atRest) and isInsert(op.tag) and not(dense(fs))

  /// An insert whose intermediate state has a DUPLICATE ORDINAL.  Under
  /// `HIGHEST_FIRST` this must never be reached; under lowest-first it is,
  /// and that is the ordering rule's real payoff.
  val wit_shiftTransientlyDuplicatesAnOrdinal: bool =
    not(atRest) and isInsert(op.tag) and not(ordinalsDistinct(fs))

  /// A rollback that did not finish.  The tree is then neither the state the
  /// operation found nor the one it intended, and the document says nothing
  /// about what the caller should do.
  val wit_partialRollbackLeavesNeitherState: bool =
    atRest and op.outcome == FailedPartiallyRolledBack and fs.objects != op.before

  /// ...and it is worse than "neither state".  A promotion creates the node
  /// carrying the leaf's own ordinal and key BEFORE it can move the leaf into
  /// it, so a rollback that fails at that one step leaves both on disk: a
  /// DUPLICATE KEY, at rest, in a tree the library built.  The document says
  /// a duplicate key "is a defect in the tree, not in the library, and the
  /// library's part is to not create one" — this is the single path by which
  /// it does.
  val wit_partialRollbackLeavesADuplicateKey: bool =
    atRest and op.outcome == FailedPartiallyRolledBack and not(keysUnique(fs))

  val wit_partialRollbackLeavesADuplicateOrdinal: bool =
    atRest and op.outcome == FailedPartiallyRolledBack and not(ordinalsDistinct(fs))

  /// ...and the damaged tree then strands later operations: a destination the
  /// algebra computes is already occupied by the wreckage.  This is the second
  /// live use of the occupancy refusal, and the only one that does not require
  /// a hand edit.
  val wit_damagedTreeStrandsALaterOperation: bool =
    atRest and op.outcome == RefusedDestinationOccupied and not(isInsert(op.tag))

  // --- coverage of the remaining operations ---------------------------------

  val wit_appendManySucceeded: bool = succeeded and isAppendMany(op.tag)

  val wit_promoteWithChild: bool =
    succeeded and match op.tag {
      | TagPromote(a) => a.withChild
      | TagNone => false | TagAppend(a) => false | TagAppendMany(a) => false
      | TagInsert(a) => false | TagRewrite(a) => false | TagHandEdit => false
    }

  /// A rewrite whose new parts equal the old: a rename onto itself.  It must
  /// SUCCEED — the occupancy check excludes the object being moved, and
  /// without that exclusion the library would refuse its own no-op.
  val wit_rewriteToSameParts: bool =
    succeeded and match op.tag {
      | TagRewrite(a) => partsOf(nameAt(beforeFs(op), resolve(beforeFs(op), ByKey(a.key)))) == a.parts
      | TagNone => false | TagAppend(a) => false | TagAppendMany(a) => false
      | TagInsert(a) => false | TagPromote(a) => false | TagHandEdit => false
    }
}

// ===========================================================================
// INSTANCES
//
// A `const` cannot run until it is instantiated.  Each of these fixes one
// question; `run-quint.sh` runs the invariants and witnesses that belong to
// each, and its table is the model's own claim about what each one shows.
//
// The parts atoms are opaque: 1 and 2 are leaf-shaped, 3 and 4 node-shaped,
// and nothing else about them is modelled.
// ===========================================================================

module cfg {
  pure val P  = Set(1, 2, 3, 4)
  pure val NP = Set(3, 4)
}

/// Only the library ever writes.  Every reachable tree is one it built, so
/// density HOLDS — this is the `init = empty tree` answer.
module pristine {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = false, CORRUPTING_EDITS = false, BROKEN_EDITS = false,
    HIGHEST_FIRST = true, IO_CAN_FAIL = false, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// A human edits between operations, keeping every stated invariant.  Density
/// FAILS — this is the `init = arbitrary well-formed tree` answer, and the
/// difference between these two instances is the whole of it.
module hand_edited {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = true, CORRUPTING_EDITS = false, BROKEN_EDITS = false,
    HIGHEST_FIRST = true, IO_CAN_FAIL = false, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// A hand edit that duplicates a key AND its parts at an adjacent ordinal —
/// `cp 01-foo-k5.md 02-foo-k5.md`.  This is the ONLY tree shape in which a
/// sibling shift can collide, which is what the shift order is for.
module corrupted {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = true, CORRUPTING_EDITS = true, BROKEN_EDITS = false,
    HIGHEST_FIRST = true, IO_CAN_FAIL = false, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// The same trees with the shift order reversed.  Everything that
/// distinguishes highest-first from lowest-first shows up here and nowhere
/// else.
module lowest_first {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = true, CORRUPTING_EDITS = true, BROKEN_EDITS = false,
    HIGHEST_FIRST = false, IO_CAN_FAIL = false, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// A domain with no distinguished child, where promotion is refused outright
/// rather than guessed at.
module no_distinguished {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = false,
    HAND_EDITS = true, CORRUPTING_EDITS = false, BROKEN_EDITS = false,
    HIGHEST_FIRST = true, IO_CAN_FAIL = false, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// A name the consumer recognises and cannot parse.  Terminal: every later
/// operation halts on it, whatever level it sits in, because snapshot scope
/// is the whole tree.
module unparseable {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = true, CORRUPTING_EDITS = false, BROKEN_EDITS = true,
    HIGHEST_FIRST = true, IO_CAN_FAIL = false, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// Effects fail.  This is where atomicity and rollback are checked.
module failures {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = true, CORRUPTING_EDITS = false, BROKEN_EDITS = false,
    HIGHEST_FIRST = true, IO_CAN_FAIL = true, ROLLBACK_CAN_FAIL = false,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

/// Rollback itself may fail.  The document is silent on this; the instance
/// exists to show what it leaves behind.
module rollback_fails {
  import cfg.*
  import operations(
    PARTS = P, NODE_PARTS = NP, HAS_DISTINGUISHED = true,
    HAND_EDITS = true, CORRUPTING_EDITS = false, BROKEN_EDITS = false,
    HIGHEST_FIRST = true, IO_CAN_FAIL = true, ROLLBACK_CAN_FAIL = true,
    MAX_OBJECTS = 7, MAX_ORD = 6, MAX_DEPTH = 7
  ).*
}

// ===========================================================================
// HANDOFF
//
// WHAT THIS MODEL COVERS
//
//   Every mutation in the document's *Operations* table — `append`,
//   `append_many`, `insert`, `promote` (with and without a first child) and
//   `rewrite` — as a plan of primitive effects applied one at a time by an
//   interpreter that can fail at any step and unwind what it applied.
//
//   Every refusal in the document's *Refusals* section, as a modelled outcome
//   with a witness proving it is reachable.  Totality is structural: `decide`
//   returns a `Decision` for every state and every argument, so an unmodelled
//   case cannot hide — it would have to be a missing branch of a `match` the
//   typechecker requires to be exhaustive.
//
//   The three [B]-tagged invariants `structure.als` could not state: subtree
//   preservation under shift, identity preservation under promotion, and plan
//   atomicity — plus "no key is ever reissued", which reads structural and is
//   not.
//
// WHAT IT DOES NOT COVER
//
//   * The filesystem beneath the interpreter.  A directory rename carrying its
//     subtree is an ASSUMPTION here, not a checked property: entries reference
//     their parent by a stable id, so subtree preservation is true by
//     construction.  What IS checked is the honest half — that no effect in an
//     `insert`'s plan names a descendant.
//   * Walk ORDER.  Reachability is modelled; the depth-first, distinguished-
//     child-first ordering is not, so `by_key`'s documented "first in walk
//     order" tie-break on a duplicate-key tree is unchecked.  The model picks
//     the least id.
//   * Concurrency.  Hand edits happen only between operations.  The lock is
//     ADVISORY and `mv` does not take it, so a hand edit during an apply is
//     physically possible and deliberately out of scope — it is also the only
//     remaining justification for the interpreter's own occupancy check, which
//     `inv_interpreterNeverFindsADestinationTaken` shows is otherwise dead.
//   * Process death.  Modelled only in its consequences: every intermediate
//     state is a state the invariants are evaluated at, so what an interrupted
//     operation would leave is visible, but the interruption itself is not an
//     action.
//   * Content.  Bytes are not modelled; `promote` moves an object, and what is
//     inside it is out of scope by the same boundary that excludes I/O.
//   * PARTS EQUALITY.  A parts atom is an `int` here, so `p == q` is identity
//     and `speciesOfParts` is a function of it: *equal parts imply equal
//     species* is true in this model by construction and therefore asserted
//     nowhere.  Rust bounds `Parts` by `Eq` alone, which is any lawful
//     equivalence, so a domain may compare a leaf's parts equal to a node's
//     while its `positioned_species` still tells them apart.  `occupied` below
//     compares whole names, which in THIS model settles the species with them;
//     an implementation reading it as a view comparison refuses every valid
//     promotion in such a domain.  That is `promote-k25`'s finding, and the
//     library now compares the species explicitly (`EntryNameExt::same_name`).
//     Widening the model to a coarse equality would need a second parts
//     relation and would check a property of the target language, not of the
//     design; the note is here instead.
//
// WHEN TO UPDATE IT
//
//   The model is the specification.  If it and an implementation disagree, the
//   model wins and the implementation changes — that is this workstream's
//   premise, and every disagreement is a finding for
//   `docs/formalism-findings.md` rather than something to fix in passing.
//   Change the model FIRST, re-run `./run-quint.sh`, and only then the code.
//
//   Adding an operation means: a variant of `OpTag`, a `plan…` function, a
//   branch of `decide`, a witness proving it succeeds, a witness per refusal
//   it introduces, and a two-state invariant saying what it leaves alone.
// ===========================================================================
