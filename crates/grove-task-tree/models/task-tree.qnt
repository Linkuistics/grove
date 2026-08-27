// =============================================================================
// Grove's task tree, in Quint.  The `TT-` scope of
// `docs/specs/semantic-contract.md`, checked as executable guarded actions with
// total outcomes.
//
// WHAT THIS MODEL IS ABOVE.  Generic ordered-tree algebra — append, insert,
// promotion, rewrite, and the shifting and key allocation they imply — is the
// `ordinal-fs-tree` contract and is ASSUMED here, not re-derived
// (`docs/ordinal-fs-tree/models/operations.qnt` is where it is checked).  What
// this model adds is Grove's own vocabulary on top of it: session kinds,
// outcome infixes, the format witness, reserved witnesses, foreign entries,
// selection, and fail-closed ownership.  `TT-09` is the seam itself, stated as
// the claim that every mutation Grove makes is one of the algebra's four
// operations plus a domain precondition.
//
// TOTALITY.  No action here "does nothing when its guard is false".  Every
// action computes a `Decision` from its snapshot and transitions in every case;
// a refusal is a value, not an absent transition.  That is what makes the
// refusal claims checkable at all, and it is why the outcome set is closed.
//
// THE MODEL PARAMETERS ARE THE ASSUMPTION DIAL.  Each `const` below names an
// `EN-` assumption the catalogue grants.  The base instance grants them all;
// the `relax_EN_*` modules at the foot of this file each turn exactly one off
// and carry the control commands the catalogue's expected result names.
// =============================================================================

module taskTree {

  // ---------------------------------------------------------------------------
  // Model parameters — the `EN-` dial
  // ---------------------------------------------------------------------------

  /// `EN-01`.  A same-directory rename is atomic with respect to namespace
  /// visibility.  Off: a rename is observable half-applied.
  const RENAME_ATOMIC: bool

  /// `EN-06`.  Locks are advisory — a direct edit is outside the guarantee.
  /// Off: the non-cooperating writer is removed.
  const FOREIGN_WRITES: bool

  /// `EN-08`.  Interruption may occur between any two steps.  Off: `crash` is
  /// removed.
  const CRASHES: bool

  /// `EN-10`.  The names are the counter: entries are never removed.  Off: an
  /// entry may be removed.
  const ENTRIES_REMOVABLE: bool

  /// `EN-11`.  Any well-formed tree is reachable by hand edit.  Off: `hand-edit`
  /// is removed.
  const HAND_EDITS: bool

  /// `EN-13`.  Foreign entries may appear at any name and are not Grove's to
  /// delete.  Off: the reaper sweeps the reserved namespace.
  const REAPER_SWEEPS_RESERVED: bool

  /// A focused search.  `0` explores the whole space; a non-zero value narrows
  /// the hand-edit family and the action menu onto one situation, so a witness
  /// that random simulation reaches in ~0.03% of traces gets an instance of its
  /// own instead of making the suite flaky.  This is a SEARCH STRATEGY: it
  /// removes no behaviour the base instance has, and every claim is still
  /// checked in the unfocused run.
  const FOCUS: int

  /// Bounds.
  const MAX_OBJECTS: int
  const MAX_DEPTH: int
  const MAX_POS: int

  // ---------------------------------------------------------------------------
  // Identities
  // ---------------------------------------------------------------------------

  /// An object's identity on disk — an inode, not a domain concept and not the
  /// permanent key.  A rename moves a NAME; only a stable identity lets a
  /// two-state property say "*this* entry kept its key".
  type Id = int

  /// The working-tree root.  It exists before the task root and outlives its
  /// deletion (`EN-14`), which is what makes it the thing the guard is taken
  /// on.
  pure val WT: Id = 0

  /// The task root.  Present iff it is in the object map; its absence is
  /// first-class.
  pure val ROOT: Id = 1

  type Species = SLeaf | SNode

  /// The closed kind set, plus the two off-grammar cases a name can carry.
  /// `KUnknown` is a kind token that is not one of the nineteen; `KAbsent` is a
  /// task-shaped name with no kind field at all.  Both malform (`TT-03`).
  type SessionKind =
    | KRequirements | KDesign | KPrototype | KImpl | KFinish
    | KUnknown | KAbsent

  type LeafState = SLive | SDone | SAbandoned

  /// A parsed name.  The name is the whole of an entry's metadata: there is
  /// nothing anywhere else except the freeform prose inside a leaf, which
  /// nothing parses.
  ///
  /// `spelling` is what makes `TT-01` statable.  `0` is the canonical spelling
  /// of these parts; any other value is a distinct filename that would denote
  /// the same entry if it were admitted.  Grove does not admit it.
  type TaskName = {
    pos: int,
    key: int,
    slug: int,
    skind: SessionKind,
    state: LeafState,
    species: Species,
    spelling: int,
  }

  /// The format witness's content.  `0` is the current format; anything else is
  /// something else, and `TT-17` is the claim that only this decides the format.
  type FormatTag = int
  pure val FORMAT_CURRENT: FormatTag = 0

  type ResClass = RPreparing | RPublished | RMigrating

  /// What one directory entry beneath the task root is.
  type EntryKind =
    // The task root directory itself.  Not an entry: its own name is never
    // parsed and it holds no position.
    | RootDir
    | Charter
    | FormatWitness(FormatTag)
    | ReservedWitness(ResClass)
    | Task(TaskName)
    // Grove-owned material evacuated to a reserved name.  The reaper may prove
    // this is its own and dispose of it; it is not a task entry and the walk
    // never sees it.
    | Quarantined
    | Foreign

  /// What the object actually IS on disk, as against what its name declares.
  /// `TT-02` is exactly the gap between the two.
  type ObjType = OFile | ODir

  /// `atReserved` marks an object sitting at a name Grove reserves.  A foreign
  /// one there is the `TT-24` situation: Grove cannot prove it is its own.
  ///
  /// `digest` is the catalogue's **entry digest** — an opaque equality, never
  /// constructed.  `TT-07`'s byte clause is discharged by it.
  type Obj = {
    parent: Id,
    kind: EntryKind,
    otype: ObjType,
    atReserved: bool,
    digest: int,
  }

  type Fs = { objs: Id -> Obj, nextId: Id }

  // ---------------------------------------------------------------------------
  // Reading an Fs
  // ---------------------------------------------------------------------------

  pure def has(f: Fs, i: Id): bool = f.objs.keys().contains(i)
  pure def kindOf(f: Fs, i: Id): EntryKind = f.objs.get(i).kind
  pure def parentOf(f: Fs, i: Id): Id = f.objs.get(i).parent
  pure def typeOf(f: Fs, i: Id): ObjType = f.objs.get(i).otype

  pure def rootPresent(f: Fs): bool = has(f, ROOT)

  pure def isTask(f: Fs, i: Id): bool =
    has(f, i) and match kindOf(f, i) { Task(_) => true | _ => false }

  pure def nameAt(f: Fs, i: Id): TaskName =
    match kindOf(f, i) {
      | Task(n) => n
      | _ => { pos: 0, key: 0, slug: 0, skind: KAbsent,
               state: SLive, species: SLeaf, spelling: 0 }
    }

  pure def isForeign(f: Fs, i: Id): bool =
    has(f, i) and match kindOf(f, i) { Foreign => true | _ => false }

  /// A task-shaped name that parses COMPLETELY: canonical spelling, and a kind
  /// that is one of the nineteen.  Anything else is task-shaped and malformed
  /// (`TT-01.b`, `TT-03`) — never skipped, never a foreign entry.
  pure def parsesCompletely(n: TaskName): bool =
    n.spelling == 0 and
    (if (n.species == SNode) n.skind == KAbsent
     else (n.skind != KUnknown and n.skind != KAbsent))

  /// The species the on-disk object actually is.
  pure def actualSpecies(f: Fs, i: Id): Species =
    if (typeOf(f, i) == ODir) SNode else SLeaf

  /// A directory the walk DESCENDS INTO: the task root, or a task entry whose
  /// name declares a node and which is a directory.  Never a foreign name —
  /// and the disclaimer is recursive, which is the whole of `TT-04`'s second
  /// half.
  pure def isDescendable(f: Fs, i: Id): bool =
    i == ROOT or
    (isTask(f, i) and nameAt(f, i).species == SNode and typeOf(f, i) == ODir)

  pure def childIds(f: Fs, d: Id): Set[Id] =
    f.objs.keys().filter(i => i != d and parentOf(f, i) == d)

  /// Everything the walk REACHES.  Bounded unrolling stands in for a fixpoint
  /// Quint cannot express recursively; `MAX_DEPTH >= MAX_OBJECTS` makes the
  /// bound unreachable.
  pure def reachedFrom(f: Fs): Set[Id] =
    if (not(rootPresent(f))) Set()
    else range(0, MAX_DEPTH).foldl(Set(ROOT), (acc, _) =>
      acc.union(acc.filter(d => isDescendable(f, d))
                   .map(d => childIds(f, d)).flatten()))

  /// The reached ENTRIES: everything the walk sees beneath the root, root
  /// excluded.  Every `TT-` claim that says "every entry" means these.
  pure def reached(f: Fs): Set[Id] = reachedFrom(f).exclude(Set(ROOT))

  pure def reachedTasks(f: Fs): Set[Id] =
    reached(f).filter(i => isTask(f, i))

  /// Reached task entries whose names parse completely — the ones a walk would
  /// act on if the tree classified as current.
  pure def recognised(f: Fs): Set[Id] =
    reachedTasks(f).filter(i => parsesCompletely(nameAt(f, i)))

  pure def reachedDirs(f: Fs): Set[Id] =
    reachedFrom(f).filter(d => isDescendable(f, d))

  pure def positionedIn(f: Fs, d: Id): Set[Id] =
    childIds(f, d).filter(i => isTask(f, i))

  pure def formatWitnessIds(f: Fs): Set[Id] =
    if (not(rootPresent(f))) Set()
    else childIds(f, ROOT).filter(i =>
      match kindOf(f, i) { FormatWitness(_) => true | _ => false })

  pure def reservedWitnessIds(f: Fs): Set[Id] =
    if (not(rootPresent(f))) Set()
    else childIds(f, ROOT).filter(i =>
      match kindOf(f, i) { ReservedWitness(_) => true | _ => false })

  pure def charterIds(f: Fs, d: Id): Set[Id] =
    childIds(f, d).filter(i => match kindOf(f, i) { Charter => true | _ => false })

  pure def formatTagOf(f: Fs): FormatTag =
    val ws = formatWitnessIds(f)
    if (ws.size() == 0) -1
    else match kindOf(f, ws.fold(-1, (a, i) => if (a == -1) i else a)) {
      | FormatWitness(t) => t
      | _ => -1
    }

  pure def resClassOf(f: Fs): ResClass =
    val ws = reservedWitnessIds(f)
    if (ws.size() == 0) RPreparing
    else match kindOf(f, ws.fold(-1, (a, i) => if (a == -1) i else a)) {
      | ReservedWitness(c) => c
      | _ => RPreparing
    }

  pure def keysOf(f: Fs): Set[int] =
    recognised(f).map(i => nameAt(f, i).key)

  /// One past the maximum key over EVERY entry in the tree, terminal ones
  /// included.  The names are the counter (`EN-10`, `TT-05`).
  pure def freshKey(f: Fs): int =
    keysOf(f).fold(0, (a, k) => if (k > a) k else a) + 1

  pure def liveIds(f: Fs): Set[Id] =
    recognised(f).filter(i =>
      nameAt(f, i).species == SLeaf and nameAt(f, i).state == SLive)

  pure def liveFinishIds(f: Fs): Set[Id] =
    liveIds(f).filter(i => nameAt(f, i).skind == KFinish)

  pure def liveOrdinaryIds(f: Fs): Set[Id] =
    liveIds(f).filter(i => nameAt(f, i).skind != KFinish)

  // ---------------------------------------------------------------------------
  // Malformity, and the whole-tree classification
  // ---------------------------------------------------------------------------

  /// The closed reason set.  A model reaching a malformity not on this list is a
  /// finding about the list, not a licence to invent a seventh.
  type MalReason =
    | MalformedEntry(Id)
    | SpeciesMismatch(Id)
    | PositionsNotGapless(Id)
    | KeyReissued(int)
    | MultipleLiveFinish
    | NodeWithoutCharter(Id)

  /// `TT-02`.  A name declares its species and must BE it.
  pure def speciesMismatched(f: Fs, i: Id): bool =
    isTask(f, i) and nameAt(f, i).species != actualSpecies(f, i)

  /// `TT-01.b`/`TT-03`.  Task-shaped and does not parse completely.
  pure def entryMalformed(f: Fs, i: Id): bool =
    isTask(f, i) and not(parsesCompletely(nameAt(f, i)))

  /// `TT-06`.  A reached directory's positions are `1..n`, no repetition, no
  /// gap.  A directory the walk does not enter has no positions to be gapless.
  /// `n` distinct positions all lying in `[1, n]` ARE exactly `1..n`, which is
  /// why this is stated as a cardinality plus a bound rather than as
  /// `ps == 1.to(n)`.  The equivalent spelling is not merely longer: Apalache
  /// refuses a non-constant integer range, so `1.to(n)` puts the whole model
  /// beyond `quint verify`.
  pure def gapless(f: Fs, d: Id): bool =
    val ids = positionedIn(f, d)
    val ps = ids.map(i => nameAt(f, i).pos)
    val n = ids.size()
    ps.size() == n and ps.forall(p => p >= 1 and p <= n)

  pure def keyDuplicated(f: Fs, k: int): bool =
    recognised(f).filter(i => nameAt(f, i).key == k).size() > 1

  /// A node directory carries a charter.  (Grove writes one for every node it
  /// promotes; a hand-edited node without one is malformed.)
  pure def nodeWithoutCharter(f: Fs, i: Id): bool =
    isTask(f, i) and nameAt(f, i).species == SNode and typeOf(f, i) == ODir and
    charterIds(f, i).size() == 0

  /// The FIRST malformity, in a fixed order so the reported reason is a
  /// function of the tree rather than of the search.  `MultipleLiveFinish` is a
  /// whole-tree reason and names no entry: `TT-13.c` is precisely the case an
  /// entry-shaped state cannot say.
  pure def malformityOf(f: Fs): Set[MalReason] =
    val es = reachedTasks(f)
    val m1 = es.filter(i => entryMalformed(f, i)).map(i => MalformedEntry(i))
    val m2 = es.filter(i => speciesMismatched(f, i)).map(i => SpeciesMismatch(i))
    val m3 = reachedDirs(f).filter(d => not(gapless(f, d))).map(d => PositionsNotGapless(d))
    val m4 = keysOf(f).filter(k => keyDuplicated(f, k)).map(k => KeyReissued(k))
    val m5 = if (liveFinishIds(f).size() > 1) Set(MultipleLiveFinish) else Set()
    val m6 = es.filter(i => nodeWithoutCharter(f, i)).map(i => NodeWithoutCharter(i))
    m1.union(m2).union(m3).union(m4).union(m5).union(m6)

  pure def isMalformed(f: Fs): bool = malformityOf(f).size() > 0

  /// A deterministic representative of the malformity set.  Only its EXISTENCE
  /// is load-bearing for any claim; which one is reported is not.
  pure def isReasonKind(m: MalReason, k: int): bool =
    match m {
      | MalformedEntry(_)     => k == 1
      | SpeciesMismatch(_)    => k == 2
      | PositionsNotGapless(_) => k == 3
      | KeyReissued(_)        => k == 4
      | MultipleLiveFinish    => k == 5
      | NodeWithoutCharter(_) => k == 6
    }

  pure def firstOfKind(ms: Set[MalReason], k: int): Set[MalReason] =
    ms.filter(m => isReasonKind(m, k))

  pure def someMalformity(f: Fs): MalReason =
    val ms = malformityOf(f)
    val ordered = range(1, 7).foldl(Set(), (acc, k) =>
      if (acc.size() > 0) acc else firstOfKind(ms, k))
    ordered.fold(MultipleLiveFinish, (a, m) => m)

  type RootState =
    | Absent
    | Reserved(ResClass)
    | PartialScaffold
    | Legacy
    | ForeignFmt(FormatTag)
    | MalformedRoot(MalReason)
    | CurrentLive
    | CurrentFinishOnly
    | CurrentSpent

  /// `TT-20`/`SY-06`.  The exact, closed subset a fresh scaffold leaves behind
  /// when it is interrupted before the format witness lands.  Defined by what
  /// it IS, never by the absence of the witness: everything a completion would
  /// write is fixed in advance, so completing is a comparison followed by at
  /// most one append, never an inference about someone else's tree.
  ///
  /// The scaffold writes a charter and one `requirements` leaf at position 1
  /// with key 1, with fixed bytes — here, fixed digests.
  pure val SCAFFOLD_CHARTER_DIGEST: int = 101
  pure val SCAFFOLD_LEAF_DIGEST: int = 102

  pure def isScaffoldCharter(f: Fs, i: Id): bool =
    match kindOf(f, i) { Charter => true | _ => false } and
    f.objs.get(i).digest == SCAFFOLD_CHARTER_DIGEST

  pure def isScaffoldLeaf(f: Fs, i: Id): bool =
    isTask(f, i) and
    nameAt(f, i) == { pos: 1, key: 1, slug: 0, skind: KRequirements,
                      state: SLive, species: SLeaf, spelling: 0 } and
    typeOf(f, i) == OFile and
    f.objs.get(i).digest == SCAFFOLD_LEAF_DIGEST

  pure def isPartialScaffold(f: Fs): bool =
    rootPresent(f) and
    formatWitnessIds(f).size() == 0 and
    childIds(f, ROOT).forall(i => isScaffoldCharter(f, i) or isScaffoldLeaf(f, i)) and
    childIds(f, ROOT).filter(i => isScaffoldCharter(f, i)).size() <= 1 and
    childIds(f, ROOT).filter(i => isScaffoldLeaf(f, i)).size() <= 1

  /// The FORMAT STAGE of the classification, isolated.  `TT-17` is the claim
  /// that this depends only on the format witness's content — never on any task
  /// entry's text — and isolating it is what makes that checkable rather than
  /// asserted.
  type FormatStage = FLegacy | FPartial | FForeign(FormatTag) | FCurrent

  pure def formatStage(f: Fs): FormatStage =
    if (formatWitnessIds(f).size() == 0)
      (if (isPartialScaffold(f)) FPartial else FLegacy)
    else if (formatTagOf(f) != FORMAT_CURRENT) FForeign(formatTagOf(f))
    else FCurrent

  /// `TT-18`.  The order is itself a claim: reserved-witness classification
  /// precedes format classification, which precedes any walk-derived
  /// classification.
  pure def classify(f: Fs): RootState =
    if (not(rootPresent(f))) Absent
    else if (reservedWitnessIds(f).size() > 0) Reserved(resClassOf(f))
    else match formatStage(f) {
      | FPartial    => PartialScaffold
      | FLegacy     => Legacy
      | FForeign(t) => ForeignFmt(t)
      | FCurrent    =>
          if (isMalformed(f)) MalformedRoot(someMalformity(f))
          else if (liveOrdinaryIds(f).size() > 0) CurrentLive
          else if (liveFinishIds(f).size() > 0) CurrentFinishOnly
          else CurrentSpent
    }

  pure def isCurrent(s: RootState): bool =
    match s { CurrentLive => true | CurrentFinishOnly => true
            | CurrentSpent => true | _ => false }

  // ---------------------------------------------------------------------------
  // The walk: pre-order without recursion
  // ---------------------------------------------------------------------------
  //
  // Quint has no recursion, so depth-first PRE-ORDER is encoded as a
  // positional number over the path from the root: position `p` at depth `d`
  // contributes `p * BASE^(MAX_DEPTH-d)`, with `BASE > MAX_POS`.  Levels a
  // shallower entry does not have pad with `0`, and `0` is below every real
  // position — which is exactly why a node sorts BEFORE its own children and
  // AFTER its earlier siblings.  That is pre-order, and it is a total order, so
  // `TT-11` is checkable as a minimum rather than as a traversal.

  pure def RADIX: int = MAX_POS + 1

  /// The chain `[ROOT, ..., i]`.  Bounded unrolling, `MAX_DEPTH` deep.
  pure def upChain(f: Fs, i: Id): List[Id] =
    range(0, MAX_DEPTH).foldl(List(i), (acc, _) =>
      val h = acc.head()
      if (h == ROOT or not(has(f, h))) acc
      else List(parentOf(f, h)).concat(acc))

  pure def preorderKey(f: Fs, i: Id): int =
    val chain = upChain(f, i).select(j => j != ROOT)
    range(0, MAX_DEPTH).foldl(0, (acc, d) =>
      val p = if (d < chain.length()) nameAt(f, chain.nth(d)).pos else 0
      acc * RADIX + p)

  pure def minBy(f: Fs, s: Set[Id]): Id =
    s.fold(-1, (a, i) =>
      if (a == -1 or preorderKey(f, i) < preorderKey(f, a)) i else a)

  // ---------------------------------------------------------------------------
  // Observations
  // ---------------------------------------------------------------------------

  type Query = ByKey(int) | BySlug(int)

  type Resolved = { id: Id, state: LeafState }

  type Report =
    | RSelected(Id)
    | RResolved(Resolved)
    | RChain(List[Id])
    | RKind(SessionKind)

  /// `TT-11`, `TT-12`, `TT-13`.  The first live leaf in pre-order, skipping
  /// terminal entries and skipping a live finish leaf while any non-finish leaf
  /// is live.  Stateless: it depends on nothing outside the tree.
  pure def selectIn(f: Fs): Set[Id] =
    val ord = liveOrdinaryIds(f)
    if (ord.size() > 0) Set(minBy(f, ord))
    else
      val fin = liveFinishIds(f)
      if (fin.size() > 0) Set(minBy(f, fin)) else Set()

  pure def matchesQuery(f: Fs, i: Id, q: Query): bool =
    match q {
      | ByKey(k)  => nameAt(f, i).key == k
      | BySlug(s) => nameAt(f, i).slug == s
    }

  /// `TT-15`, `TT-16`.  Resolution searches live, `Done` AND `Abandoned` alike,
  /// and reports the entry's terminality with it.
  pure def resolveIn(f: Fs, q: Query): Set[Id] =
    recognised(f).filter(i => matchesQuery(f, i, q))

  /// The charter chain of a leaf's ancestor DIRECTORIES, root -> leaf.  A level
  /// with no charter is skipped silently.
  pure def chainIn(f: Fs, i: Id): List[Id] =
    upChain(f, i).select(j => j != i and isDescendable(f, j))
                 .foldl(List(), (acc, d) =>
                   val cs = charterIds(f, d)
                   if (cs.size() == 0) acc
                   else acc.append(cs.fold(-1, (a, c) => if (a == -1) c else a)))

  // ---------------------------------------------------------------------------
  // Perturbations — the instruments `TT-14` and `TT-17` are read through
  // ---------------------------------------------------------------------------
  //
  // A claim of the form "X depends only on Y" is not checkable by inspecting X.
  // It is checkable by perturbing everything that is NOT Y and observing that X
  // is unmoved.  These two perturbations are those instruments, and they are
  // pure functions on an `Fs` so that no transition is needed to apply one.

  pure val PERTURBED_SLUG: int = 77

  /// Every task entry's slug replaced.  Slug is the only free TEXT a name
  /// carries; `TT-14` says selection may not read it.
  pure def perturbSlugs(f: Fs): Fs =
    { ...f, objs: f.objs.keys().mapBy(i =>
        val o = f.objs.get(i)
        match o.kind {
          | Task(n) => { ...o, kind: Task({ ...n, slug: PERTURBED_SLUG }) }
          | _ => o
        }) }

  /// Every task entry's slug AND session kind replaced by something that reads
  /// as current work.  `TT-17` says the FORMAT decision may not read either.
  pure def perturbText(f: Fs): Fs =
    { ...f, objs: f.objs.keys().mapBy(i =>
        val o = f.objs.get(i)
        match o.kind {
          | Task(n) => { ...o, kind: Task({ ...n, slug: PERTURBED_SLUG,
                                            skind: KDesign }) }
          | _ => o
        }) }

  // ---------------------------------------------------------------------------
  // Outcomes — the closed set.  Every action returns exactly one.
  // ---------------------------------------------------------------------------

  type Refusal =
    | RootAbsent
    | FormatLegacy
    | FormatForeign
    | WitnessPending(ResClass)
    | ReservedNameOccupied(Id)
    | RefMalformed(MalReason)
    | NotLive
    | AlreadyTerminal
    | ReservedKind
    | NotAnEntry
    | DestinationOccupied
    | LayoutUnsupported
    | LeaseHeld
    | EpochStale
    | NoTrackedDeletion
    | RootIdentityChanged
    | UnsupportedEntryType

  type Blocked = RecoveryPending | OwnershipConflict

  type Outcome =
    | Idle
    | Applied
    | Reported(Report)
    | Empty
    | Ambiguous(Set[int])
    | Refused(Refusal)
    | BlockedO(Blocked)

  pure def isRefusal(o: Outcome): bool =
    match o { Refused(_) => true | _ => false }

  pure def isSuccessObservation(o: Outcome): bool =
    match o { Reported(_) => true | Empty => true | Ambiguous(_) => true | _ => false }

  // ---------------------------------------------------------------------------
  // The algebra seam — `TT-09`
  // ---------------------------------------------------------------------------
  //
  // Every mutation Grove makes is ONE `ordinal-fs-tree` operation plus a domain
  // precondition.  The four are named here so that "no action moves an entry
  // outside the algebra" is a property over the applied effects rather than an
  // assertion about the code.

  type Algebraic = AAppend | AInsert | APromote | ARewrite | ANone

  type Effect =
    | ECreate({ id: Id, parent: Id, kind: EntryKind, otype: ObjType,
                atReserved: bool, digest: int })
    // A same-directory rename: the entry's NAME changes and nothing else.
    | ERename({ id: Id, name: TaskName })
    // The format witness made visible by an atomic same-directory rename.
    | EPublish({ id: Id, tag: FormatTag })
    | ERemove({ id: Id })
    // Promotion: an entry moves into a directory created for it.
    | EReparent({ id: Id, parent: Id, kind: EntryKind })

  pure def effTarget(e: Effect): Id =
    match e {
      | ECreate(c)   => c.id
      | ERename(r)   => r.id
      | EPublish(p)  => p.id
      | ERemove(r)   => r.id
      | EReparent(m) => m.id
    }

  pure def applyEffect(f: Fs, e: Effect): Fs =
    match e {
      | ECreate(c) =>
          { objs: f.objs.put(c.id, { parent: c.parent, kind: c.kind, otype: c.otype,
                                     atReserved: c.atReserved, digest: c.digest }),
            nextId: if (c.id >= f.nextId) c.id + 1 else f.nextId }
      | ERename(r) =>
          if (not(has(f, r.id))) f
          else { ...f, objs: f.objs.put(r.id, { ...f.objs.get(r.id), kind: Task(r.name) }) }
      | EPublish(p) =>
          { objs: f.objs.put(p.id,
              { parent: ROOT, kind: FormatWitness(p.tag), otype: OFile,
                atReserved: false, digest: 0 }),
            nextId: if (p.id >= f.nextId) p.id + 1 else f.nextId }
      | ERemove(r) => { ...f, objs: f.objs.keys().exclude(Set(r.id)).mapBy(i => f.objs.get(i)) }
      | EReparent(m) =>
          if (not(has(f, m.id))) f
          else { ...f, objs: f.objs.put(m.id,
                   { ...f.objs.get(m.id), parent: m.parent, kind: m.kind }) }
    }

  pure def applyAll(f: Fs, es: List[Effect]): Fs = es.foldl(f, (a, e) => applyEffect(a, e))

  /// A step whose destination is already taken.  The plan was licensed by the
  /// operation's own listing (`TT-21`), and a non-cooperating writer may have
  /// moved the world since (`EN-06`) — so the create is not licensed any more.
  /// It must NOT proceed: overwriting what is there would be a mutation of an
  /// entry whose ownership the operation cannot prove (`TT-24.a`).
  ///
  /// WHAT IT PRODUCES INSTEAD IS NOT THE MODEL'S CHOICE — see
  /// `collisionOutcome`, which reads it off the catalogue's own
  /// *one artifact, three contexts* table.
  pure def effectBlocked(f: Fs, e: Effect): bool =
    match e {
      | ECreate(c) => has(f, c.id)
      // Publishing the format witness onto a name someone else has taken is
      // the same defect wearing a rename: the second half of a TORN publish is
      // licensed (the name already holds Grove's own witness), an overwrite of
      // anything else is not.
      | EPublish(p) =>
          has(f, p.id) and
          not(match kindOf(f, p.id) { FormatWitness(_) => true | _ => false })
      | _ => false
    }

  // ---------------------------------------------------------------------------
  // Guards — `TT-22`
  // ---------------------------------------------------------------------------

  pure val PROCS: Set[int] = Set(1, 2)

  type Holder = HNone | HShared(Set[int]) | HExcl(int)

  pure def sharedSet(h: Holder): Set[int] =
    match h { HShared(s) => s | _ => Set() }

  pure def exclHolder(h: Holder): int =
    match h { HExcl(p) => p | _ => -1 }

  pure def holders(h: Holder): Set[int] =
    match h { HNone => Set() | HShared(s) => s | HExcl(p) => Set(p) }

  // ---------------------------------------------------------------------------
  // Operations
  // ---------------------------------------------------------------------------

  type OpTag =
    | TSelect
    | TResolve(Query)
    | TChain(Query)
    | TKindOf(Query)
    | TInitRoot
    | TAddLeaf(Id)
    | TInsertLeaf({ parent: Id, at: int })
    | TDecompose(int)
    | TRetire(int)
    | TPrune(int)
    | TBulkMark(List[int])
    | TReap
    | TFinishTxn
    | TNone

  pure def isObservation(t: OpTag): bool =
    match t { TSelect => true | TResolve(_) => true | TChain(_) => true
            | TKindOf(_) => true | _ => false }

  pure def isTransaction(t: OpTag): bool =
    match t { TFinishTxn => true | _ => false }

  /// One in-flight operation.  `snap` is THE listing (`TT-21`): every
  /// classification the operation makes is computed from it and never from the
  /// live tree.  `cls` is the ghost list of classifications actually made, which
  /// is what turns `TT-21` from a coding convention into a checkable property.
  type Op = {
    actor: int,
    tag: OpTag,
    running: bool,
    snap: Fs,
    cls: List[RootState],
    outcome: Outcome,
    alg: Algebraic,
    pending: List[Effect],
    applied: List[Effect],
    plan: List[Effect],
    foreignDuring: bool,
    coopDuring: bool,
    preempted: bool,
    canonicalNamed: bool,
    declined: Set[Id],
    allocated: int,
    beforeFs: Fs,
    // What `decide` returned from the operation's OWN arguments and its one
    // listing, kept because `outcome` is later overwritten by what the world
    // did (`stepOpBlocked`, `crashNow`).  `TT-10` is a claim about arguments,
    // and separating the two is what keeps it one.  (A `///` here is a parse
    // error inside a record type; entry 044 records the same trap.)
    decided: Outcome,
  }

  pure val EMPTY_FS: Fs = { objs: Map(), nextId: 2 }

  pure val IDLE_OP: Op = {
    actor: 0, tag: TNone, running: false, snap: EMPTY_FS, cls: List(),
    outcome: Idle, alg: ANone, pending: List(), applied: List(), plan: List(),
    foreignDuring: false, coopDuring: false, preempted: false,
    canonicalNamed: false, declined: Set(), allocated: -1, beforeFs: EMPTY_FS,
    decided: Idle,
  }

  pure def isQuarantined(f: Fs, i: Id): bool =
    has(f, i) and match kindOf(f, i) { Quarantined => true | _ => false }

  pure def pick(s: Set[Id]): Id = s.fold(-1, (a, i) => if (a == -1) i else a)

  /// Objects sitting at a name Grove reserves.  A `Foreign` one there is the
  /// `TT-24` situation exactly: an artifact Grove can neither classify as its
  /// own nor name a recovery for.
  pure def atReservedIds(f: Fs): Set[Id] =
    if (not(rootPresent(f))) Set()
    else childIds(f, ROOT).filter(i => f.objs.get(i).atReserved)

  pure def foreignAtReserved(f: Fs): Set[Id] =
    atReservedIds(f).filter(i => isForeign(f, i))

  /// Everything beneath a foreign entry, transitively — the subtree `TT-04`
  /// makes invisible to every read and untouched by every mutation.
  pure def foreignClosure(f: Fs): Set[Id] =
    val roots = f.objs.keys().filter(i => isForeign(f, i))
    range(0, MAX_DEPTH).foldl(roots, (acc, _) =>
      acc.union(f.objs.keys().filter(i => acc.contains(parentOf(f, i)))))

  /// What Grove can PROVE is its own: the charter, the format witness, a
  /// reserved witness it wrote, a task entry, and quarantined material it
  /// evacuated.  Everything else is not Grove's to touch (`TT-24.a`).
  pure def provablyOwned(f: Fs, i: Id): bool =
    has(f, i) and
    match kindOf(f, i) {
      | Foreign => false
      | Charter => true
      | FormatWitness(_) => true
      | ReservedWitness(_) => true
      | Quarantined => true
      | RootDir => true
      | Task(_) => true
    }

  // ---------------------------------------------------------------------------
  // The gate — classification order, `TT-18`/`TT-19`/`TT-24`
  // ---------------------------------------------------------------------------
  //
  // Ordered exactly as the catalogue's state table is, and the order is itself
  // the claim.  `Idle` here means "the gate passed"; every other value is the
  // operation's whole outcome.
  //
  // DECLARED DEVIATION, recorded as a finding rather than fixed under the
  // independence barrier: an ordinary operation meeting `PartialScaffold` has
  // NO reason in the catalogue's closed refusal set.  `FormatLegacy` is the one
  // `TT-20` explicitly forbids and `WitnessPending` names a reserved witness
  // that is not there.  The model refuses `WitnessPending(RPreparing)` as the
  // least-wrong member — Grove-owned, incomplete, and naming a recovery — and
  // the gap is entry 044's finding.

  pure def gateOutcome(f: Fs, t: OpTag): Outcome =
    if (t == TInitRoot)
      (if (rootPresent(f)) Refused(DestinationOccupied) else Idle)
    else if (not(rootPresent(f))) Refused(RootAbsent)
    else if (foreignAtReserved(f).size() > 0)
      (if (t == TReap) Idle
       else if (isTransaction(t)) BlockedO(OwnershipConflict)
       else Refused(ReservedNameOccupied(pick(foreignAtReserved(f)))))
    else if (reservedWitnessIds(f).size() > 0) Refused(WitnessPending(resClassOf(f)))
    else match formatStage(f) {
      | FPartial    => Refused(WitnessPending(RPreparing))
      | FLegacy     => Refused(FormatLegacy)
      | FForeign(_) => Refused(FormatForeign)
      | FCurrent    =>
          if (isMalformed(f)) Refused(RefMalformed(someMalformity(f))) else Idle
    }

  pure def gatePassed(o: Outcome): bool = match o { Idle => true | _ => false }

  // ---------------------------------------------------------------------------
  // Decisions — every action is total
  // ---------------------------------------------------------------------------

  type Decision = {
    outcome: Outcome,
    alg: Algebraic,
    effects: List[Effect],
    canonicalNamed: bool,
    preempted: bool,
    allocated: int,
    declined: Set[Id],
  }

  pure def refuse(r: Refusal): Decision =
    { outcome: Refused(r), alg: ANone, effects: List(), canonicalNamed: false,
      preempted: false, allocated: -1, declined: Set() }

  pure def report(o: Outcome): Decision =
    { outcome: o, alg: ANone, effects: List(), canonicalNamed: false,
      preempted: false, allocated: -1, declined: Set() }

  pure def mutate(a: Algebraic, es: List[Effect], alloc: int, pre: bool): Decision =
    { outcome: Applied, alg: a, effects: es, canonicalNamed: false,
      preempted: pre, allocated: alloc, declined: Set() }

  pure def idOf(f: Fs, k: int): Id =
    pick(recognised(f).filter(i => nameAt(f, i).key == k))

  pure def maxPosIn(f: Fs, d: Id): int =
    positionedIn(f, d).fold(0, (a, i) => if (nameAt(f, i).pos > a) nameAt(f, i).pos else a)

  pure def freshLeaf(pos: int, key: int, slug: int, k: SessionKind): TaskName =
    { pos: pos, key: key, slug: slug, skind: k, state: SLive,
      species: SLeaf, spelling: 0 }

  pure def mkCreate(id: Id, parent: Id, n: TaskName, dig: int): Effect =
    ECreate({ id: id, parent: parent, kind: Task(n),
              otype: if (n.species == SNode) ODir else OFile,
              atReserved: false, digest: dig })

  /// A shift: every sibling at or after `at` moves up one position and NOTHING
  /// else about it changes.  `TT-07` is the claim that this is so — including
  /// the entry's bytes, which the entry digest discharges because `ERename`
  /// touches only the name.
  pure def shiftEffects(f: Fs, d: Id, at: int): List[Effect] =
    positionedIn(f, d).filter(i => nameAt(f, i).pos >= at)
      .fold(List(), (acc, i) =>
        acc.append(ERename({ id: i, name: { ...nameAt(f, i), pos: nameAt(f, i).pos + 1 } })))

  pure def markEffect(f: Fs, i: Id, st: LeafState): Effect =
    ERename({ id: i, name: { ...nameAt(f, i), state: st } })

  pure def markable(f: Fs, k: int): Outcome =
    val i = idOf(f, k)
    if (i == -1) Empty
    else if (nameAt(f, i).species == SNode) Refused(NotAnEntry)
    else if (nameAt(f, i).skind == KFinish) Refused(ReservedKind)
    else if (nameAt(f, i).state != SLive) Refused(AlreadyTerminal)
    else Idle

  pure val TORN_TAG: FormatTag = -2

  /// `TT-20`.  The scaffold, then the witness — in that order, and the witness
  /// by an atomic same-directory rename.  With `RENAME_ATOMIC` off the publish
  /// is two observable halves, which is `EN-01`'s premise-break.
  pure def initEffects(f: Fs): List[Effect] =
    val rootObj = ECreate({ id: ROOT, parent: WT, kind: RootDir, otype: ODir,
                            atReserved: false, digest: 0 })
    val charter = ECreate({ id: ROOT + 1, parent: ROOT, kind: Charter, otype: OFile,
                            atReserved: false, digest: SCAFFOLD_CHARTER_DIGEST })
    val leaf = ECreate({ id: ROOT + 2, parent: ROOT,
                         kind: Task(freshLeaf(1, 1, 0, KRequirements)), otype: OFile,
                         atReserved: false, digest: SCAFFOLD_LEAF_DIGEST })
    val wid = ROOT + 3
    val publish =
      if (RENAME_ATOMIC) List(EPublish({ id: wid, tag: FORMAT_CURRENT }))
      else List(EPublish({ id: wid, tag: TORN_TAG }),
                EPublish({ id: wid, tag: FORMAT_CURRENT }))
    List(rootObj, charter, leaf).concat(publish)

  /// The quarantine name a finish transaction evacuates to.
  pure def quarantineEffects(f: Fs): List[Effect] =
    List(ECreate({ id: f.nextId, parent: ROOT, kind: Quarantined, otype: ODir,
                   atReserved: true, digest: 900 }))

  /// `TT-23.b`'s SECOND DIAL, and it is a dial rather than a decision for the
  /// same reason `ONE_SNAPSHOT` is: the requirement it encodes is one the
  /// catalogue implies and does not state, and a requirement asserted only by
  /// writing the model the right way is unfalsifiable.  On: a bulk member
  /// already in the plan's target state is admissible and a no-op.  Off: a bulk
  /// plan is validated exactly the way a single mark is, `AlreadyTerminal`
  /// refuses the member the interrupted run already marked, and
  /// `mutant_bulk_strict` is the run that watches `TT-23.b` die.
  const BULK_TARGET_IDEMPOTENT: bool

  /// A bulk member is admissible if it is a markable leaf OR IS ALREADY IN THE
  /// TARGET STATE.  The second half is what `TT-23.b` requires and what a naive
  /// reading of the catalogue denies: `AlreadyTerminal` is a refusal for a
  /// single mark, so validating a bulk plan the same way makes a re-run after a
  /// partial application refuse — and the plan can then NEVER converge, which
  /// is the property `bulk-marks-are-not-atomic` exists to buy.  Idempotence on
  /// the target state is therefore a requirement the claim implies and the
  /// catalogue does not say; it is recorded as an observation, as a derived
  /// test, and as the mutation above.
  pure def bulkMemberOk(f: Fs, k: int, target: LeafState): bool =
    val i = idOf(f, k)
    i != -1 and nameAt(f, i).species == SLeaf and nameAt(f, i).skind != KFinish and
    (nameAt(f, i).state == SLive or
     (BULK_TARGET_IDEMPOTENT and nameAt(f, i).state == target))

  pure def bulkPlan(f: Fs, ks: List[int]): List[Effect] =
    ks.foldl(List(), (acc, k) =>
      if (nameAt(f, idOf(f, k)).state == SLive) acc.append(markEffect(f, idOf(f, k), SDone))
      else acc)

  pure def bulkInvalid(f: Fs, ks: List[int]): bool =
    ks.foldl(false, (acc, k) => acc or not(bulkMemberOk(f, k, SDone)))

  /// The whole decision, per action.  EVERY branch returns a `Decision`: there
  /// is no input for which an action here has no transition, which is what the
  /// refusal claims are stated over.
  pure def decide(f: Fs, t: OpTag): Decision =
    val g = gateOutcome(f, t)
    if (not(gatePassed(g)))
      { outcome: g, alg: ANone, effects: List(),
        canonicalNamed: match g {
          | Refused(r) =>
              match r {
                | RefMalformed(m) => match m { MalformedEntry(_) => true | _ => false }
                | _ => false
              }
          | _ => false },
        preempted: false, allocated: -1, declined: Set() }
    else match t {
      | TInitRoot => mutate(AAppend, initEffects(f), 1, false)

      | TSelect =>
          val s = selectIn(f)
          if (s.size() == 0) report(Empty) else report(Reported(RSelected(pick(s))))

      | TResolve(q) =>
          val ms = resolveIn(f, q)
          if (ms.size() == 0) report(Empty)
          else if (ms.size() > 1) report(Ambiguous(ms.map(i => nameAt(f, i).key)))
          else val i = pick(ms)
               report(Reported(RResolved({ id: i, state: nameAt(f, i).state })))

      | TChain(q) =>
          val ms = resolveIn(f, q)
          if (ms.size() == 0) report(Empty)
          else if (ms.size() > 1) report(Ambiguous(ms.map(i => nameAt(f, i).key)))
          else report(Reported(RChain(chainIn(f, pick(ms)))))

      | TKindOf(q) =>
          val ms = resolveIn(f, q)
          if (ms.size() == 0) report(Empty)
          else if (ms.size() > 1) report(Ambiguous(ms.map(i => nameAt(f, i).key)))
          else report(Reported(RKind(nameAt(f, pick(ms)).skind)))

      | TAddLeaf(d) =>
          if (not(reachedDirs(f).contains(d))) refuse(NotAnEntry)
          else val k = freshKey(f)
               mutate(AAppend,
                      List(mkCreate(f.nextId, d,
                                    freshLeaf(maxPosIn(f, d) + 1, k, k, KImpl), 500)),
                      k, false)

      | TInsertLeaf(a) =>
          if (not(reachedDirs(f).contains(a.parent))) refuse(NotAnEntry)
          else if (a.at < 1 or a.at > maxPosIn(f, a.parent) + 1) refuse(NotAnEntry)
          else val k = freshKey(f)
               val occupied = a.at <= maxPosIn(f, a.parent)
               mutate(AInsert,
                      shiftEffects(f, a.parent, a.at)
                        .append(mkCreate(f.nextId, a.parent,
                                         freshLeaf(a.at, k, k, KImpl), 501)),
                      k, occupied)

      | TDecompose(k) =>
          val m = markable(f, k)
          if (not(gatePassed(m)))
            (match m { Empty => report(Empty) | _ => refuse(NotLive) })
          else val i = idOf(f, k)
               val n = nameAt(f, i)
               val nk = freshKey(f)
               mutate(APromote,
                      List(ECreate({ id: f.nextId, parent: parentOf(f, i),
                                     kind: Task({ ...n, skind: KAbsent, species: SNode }),
                                     otype: ODir, atReserved: false, digest: 600 }),
                           EReparent({ id: i, parent: f.nextId, kind: Charter }),
                           mkCreate(f.nextId + 1, f.nextId,
                                    freshLeaf(1, nk, nk, n.skind), 601)),
                      nk, false)

      | TRetire(k) =>
          val m = markable(f, k)
          if (not(gatePassed(m)))
            (match m { Empty => report(Empty) | _ => { ...refuse(NotLive), outcome: m } })
          else mutate(ARewrite, List(markEffect(f, idOf(f, k), SDone)), -1, false)

      | TPrune(k) =>
          val m = markable(f, k)
          if (not(gatePassed(m)))
            (match m { Empty => report(Empty) | _ => { ...refuse(NotLive), outcome: m } })
          else mutate(ARewrite, List(markEffect(f, idOf(f, k), SAbandoned)), -1, false)

      | TBulkMark(ks) =>
          if (ks.length() == 0) report(Empty)
          else if (bulkInvalid(f, ks)) refuse(AlreadyTerminal)
          else mutate(ARewrite, bulkPlan(f, ks), -1, false)

      | TReap =>
          val mine = atReservedIds(f).filter(i => isQuarantined(f, i))
          val theirs = foreignAtReserved(f)
          val sweep = if (REAPER_SWEEPS_RESERVED) mine.union(theirs) else mine
          { outcome: Applied, alg: ARewrite,
            effects: sweep.fold(List(), (acc, i) => acc.append(ERemove({ id: i }))),
            canonicalNamed: false, preempted: false, allocated: -1,
            declined: if (REAPER_SWEEPS_RESERVED) Set() else theirs }

      | TFinishTxn => mutate(ARewrite, quarantineEffects(f), -1, false)

      | TNone => report(Idle)
    }

  // ---------------------------------------------------------------------------
  // The mutation dial — a control on the MODEL, not on the world
  // ---------------------------------------------------------------------------
  //
  // `TT-21` says every classification an operation makes comes from one
  // listing.  A model that only ever classifies from `op.snap` satisfies that
  // by construction, and a claim true by construction is the vacuous-invariant
  // hazard wearing a green tick.  `ONE_SNAPSHOT` is therefore a dial: with it
  // off, later steps classify from the LIVE tree, which is the defect `TT-21`
  // forbids, and `mutant_two_listings` is the run that watches the claim die.
  const ONE_SNAPSHOT: bool

  // ---------------------------------------------------------------------------
  // History — the ghosts a trace property needs
  // ---------------------------------------------------------------------------

  type Hist = {
    issued: Set[int],
    reissued: bool,
    initCrashCls: Set[RootState],
    initCrashUnderForeign: Set[RootState],
    crashed: bool,
    handEdited: bool,
    foreignWritten: bool,
    foreignMutated: bool,
    unownedMutated: bool,
    entryRemoved: bool,
    bulkInvalidApplied: bool,
    bulkPartial: bool,
    // THE INTERRUPTED REQUEST ITSELF, not a sticky boolean.  `TT-23.b` is
    // "re-running IT converges", so the ghost has to hold what was run: a flag
    // that any later bulk mark can set proves that SOME bulk mark completed,
    // which is a different and much weaker claim.
    bulkPlanKeys: List[int],
    bulkRepaired: bool,
    bulkDiverged: bool,
    reapDisturbed: bool,
    reapDeclined: bool,
  }

  pure val EMPTY_HIST: Hist = {
    issued: Set(), reissued: false, initCrashCls: Set(),
    initCrashUnderForeign: Set(), crashed: false,
    handEdited: false, foreignWritten: false, foreignMutated: false,
    unownedMutated: false, entryRemoved: false, bulkInvalidApplied: false,
    bulkPartial: false, bulkPlanKeys: List(), bulkRepaired: false,
    bulkDiverged: false,
    reapDisturbed: false, reapDeclined: false,
  }

  /// The same request, spelled the same way.  `[1, 2]` and `[2, 1]` are two
  /// requests, and a model that accepted either as the repair of the other
  /// would be establishing that A bulk mark converges rather than that THE
  /// interrupted one does.
  pure def sameRequest(a: List[int], b: List[int]): bool =
    a.length() == b.length() and
    range(0, a.length()).foldl(true, (acc, i) => acc and a.nth(i) == b.nth(i))

  /// The retry Grove is being asked about: this operation is a bulk mark, an
  /// earlier bulk mark was interrupted part-way, and this one carries the
  /// identical request.
  pure def isBulkRetry(h: Hist, t: OpTag): bool =
    h.bulkPartial and
    match t { TBulkMark(ks) => sameRequest(ks, h.bulkPlanKeys) | _ => false }

  var fs: Fs
  var op: Op
  var guard: Holder
  var pend: Set[int]
  var hist: Hist

  // ---------------------------------------------------------------------------
  // Applying one effect, with the ownership ghosts `TT-04`/`TT-24` are read off
  // ---------------------------------------------------------------------------

  pure def effectTouchesForeign(f: Fs, e: Effect): bool =
    val i = effTarget(e)
    has(f, i) and foreignClosure(f).contains(i)

  pure def effectTouchesUnowned(f: Fs, e: Effect): bool =
    val i = effTarget(e)
    has(f, i) and not(provablyOwned(f, i))

  pure def effectRemovesEntry(f: Fs, e: Effect): bool =
    match e { ERemove(r) => reachedTasks(f).contains(r.id) | _ => false }

  pure def bumpHist(h: Hist, f: Fs, e: Effect): Hist =
    { ...h,
      foreignMutated: h.foreignMutated or effectTouchesForeign(f, e),
      unownedMutated: h.unownedMutated or effectTouchesUnowned(f, e),
      entryRemoved: h.entryRemoved or effectRemovesEntry(f, e) }

  pure def observedCls(o: Op, live: Fs): RootState =
    if (ONE_SNAPSHOT) classify(o.snap) else classify(live)

  // ---------------------------------------------------------------------------
  // Actions
  // ---------------------------------------------------------------------------

  action init: bool = all {
    // The world before Grove: a working-tree root and no task root.  Every
    // other tree in this model is REACHED — by Grove's own actions, or by
    // `hand-edit`, which is what `EN-11` grants.
    fs' = { objs: Map(WT -> { parent: WT, kind: RootDir, otype: ODir,
                              atReserved: false, digest: 0 }),
            nextId: 2 },
    op' = IDLE_OP,
    guard' = HNone,
    pend' = Set(),
    hist' = EMPTY_HIST,
  }

  /// Start an operation.  The guard is taken on the working-tree root, shared
  /// for observation and exclusive for mutation.
  ///
  /// A CALLER THAT CANNOT TAKE THE GUARD PRODUCES NO OUTCOME, and that is the
  /// catalogue's own position: Grove's tree lock blocks, so no invocation
  /// returns while it is held, and a wait is not a return.  The waiting state
  /// is therefore modelled as membership of `pend` rather than as a member of
  /// the closed outcome set — which keeps `TT-22` falsifiable (a failed guard
  /// is a real transition, not an absent one) without putting a tree-level twin
  /// of `LeaseHeld` into a refusal list Grove never produces.
  action beginOp(p: int, t: OpTag): bool =
    val obs = isObservation(t)
    val admitted =
      if (obs) match guard { HExcl(_) => false | _ => true }
      else (guard == HNone and not(op.running))
    if (not(admitted)) all {
      pend' = pend.union(Set(p)),
      op' = { ...op, coopDuring: op.coopDuring or op.running },
      fs' = fs, guard' = guard, hist' = hist,
    } else
      val d = decide(fs, t)
      val newOp: Op = {
        actor: p, tag: t, running: d.effects.length() > 0,
        snap: fs, cls: List(classify(fs)), outcome: d.outcome, alg: d.alg,
        pending: d.effects, applied: List(), plan: d.effects,
        foreignDuring: false, coopDuring: false, preempted: d.preempted,
        canonicalNamed: d.canonicalNamed, declined: d.declined,
        allocated: d.allocated, beforeFs: fs, decided: d.outcome,
      }
      all {
        op' = newOp,
        guard' = if (obs) HShared(sharedSet(guard).union(Set(p))) else HExcl(p),
        fs' = fs,
        pend' = pend,
        // A key is ISSUED when the operation that allocated it COMPLETES, not
        // when it is planned: an interrupted operation whose entry never landed
        // issued nothing, because the names are the counter (`EN-10`).  See
        // `finishOp`.
        hist' = { ...hist,
                  reapDeclined: hist.reapDeclined or d.declined.size() > 0,
                  // A REFUSED RETRY IS A DIVERGENCE, and it has to be caught
                  // here: a refusal never becomes a running operation, so it
                  // never reaches `finishOp`, and a convergence ghost that only
                  // watches completions cannot see the very failure `TT-23.b`
                  // is about.  Re-running the identical request after a partial
                  // application and being told `AlreadyTerminal` is precisely
                  // "does not reach the same result".
                  //
                  // ONLY A REFUSAL THE BULK VALIDATOR ITSELF PRODUCED, which is
                  // what `gatePassed` qualifies.  Every gate refusal — a
                  // foreign artifact at a reserved name, a reserved witness, a
                  // root that is no longer current — refuses EVERY operation,
                  // and reading those onto `TT-23.b` would make the claim a
                  // statement about what the world did to the tree rather than
                  // about whether re-running the plan converges.  `TT-24.b` and
                  // `TT-19` are the obligations that own those, and they do.
                  bulkDiverged: hist.bulkDiverged or
                                (isBulkRetry(hist, t) and isRefusal(d.outcome) and
                                 gatePassed(gateOutcome(fs, t))) },
      }

  /// Release a shared guard.  Observations hold it only long enough for a
  /// second one to join, which is `TT-22.a`'s whole content.
  action endObs(p: int): bool = all {
    sharedSet(guard).contains(p),
    guard' = if (sharedSet(guard) == Set(p)) HNone else HShared(sharedSet(guard).exclude(Set(p))),
    pend' = if (sharedSet(guard) == Set(p)) Set() else pend,
    fs' = fs, op' = op, hist' = hist,
  }

  /// Apply the next effect of a running operation.  Each step records the
  /// classification it made, which is what `TT-21` is checked over.
  action stepOp: bool = all {
    op.running,
    op.pending.length() > 0,
    not(effectBlocked(fs, op.pending.head())),
    val e = op.pending.head()
    all {
      fs' = applyEffect(fs, e),
      op' = { ...op,
              pending: op.pending.tail(),
              applied: op.applied.append(e),
              cls: op.cls.append(observedCls(op, fs)) },
      hist' = { ...bumpHist(hist, fs, e),
                bulkInvalidApplied: hist.bulkInvalidApplied or
                  match op.tag { TBulkMark(ks) => bulkInvalid(op.snap, ks) | _ => false } },
      guard' = guard,
      pend' = pend,
    }
  }

  /// What an operation produces when its next step is no longer licensed.
  /// Never a silent overwrite and never a pretence that the mutation landed —
  /// and WHICH non-mutating outcome it is is not the model's choice.  The
  /// catalogue fixes it by how far the caller had already gone (*Outcomes*,
  /// **One artifact, three contexts, one decided outcome**):
  ///
  ///   - NOTHING APPLIED YET.  The tree is byte-identical, so the honest
  ///     outcome is a REFUSAL, and `Blocked` — "a transaction stopped part-way
  ///     and left a stable, recoverable state" — would be a lie about state
  ///     that never moved.  The reason is `ReservedNameOccupied` exactly when
  ///     the occupant is the artifact `TT-24.b` is about, and
  ///     `DestinationOccupied` otherwise.
  ///   - AN EFFECT HAS ALREADY LANDED, or this is a transaction.  Now nothing
  ///     can say "nothing happened", and `Blocked(OwnershipConflict)` is the
  ///     honest stable state — on the table's own reasoning, that a caller who
  ///     has already mutated is owed a block rather than a refusal.
  ///
  /// The ORDINARY half of that second case is the one the table has no row for:
  /// its three contexts are an ordinary operation *before any transaction*, a
  /// finish transaction, and the reaper.  A mid-flight ordinary mutation is
  /// none of them, and that — not the pre-effect collision, which the closed
  /// set answers perfectly well — is the missing-outcome finding.
  pure def collisionOutcome(f: Fs, o: Op): Outcome =
    val i = effTarget(o.pending.head())
    if (o.applied.length() > 0 or isTransaction(o.tag)) BlockedO(OwnershipConflict)
    else if (has(f, i) and f.objs.get(i).atReserved and isForeign(f, i))
      Refused(ReservedNameOccupied(i))
    else Refused(DestinationOccupied)

  action stepOpBlocked: bool = all {
    op.running,
    op.pending.length() > 0,
    effectBlocked(fs, op.pending.head()),
    fs' = fs,
    op' = { ...op, running: false, pending: List(),
            outcome: collisionOutcome(fs, op) },
    guard' = HNone,
    pend' = Set(),
    hist' = hist,
  }

  action finishOp: bool = all {
    op.running,
    op.pending.length() == 0,
    op' = { ...op, running: false },
    guard' = HNone,
    pend' = Set(),
    fs' = fs,
    hist' = { ...hist,
              issued: if (op.allocated >= 0) hist.issued.union(Set(op.allocated))
                      else hist.issued,
              reissued: hist.reissued or
                        (op.allocated >= 0 and hist.issued.contains(op.allocated)),
              // Only the IDENTICAL request repairs the interrupted one.
              bulkRepaired: hist.bulkRepaired or isBulkRetry(hist, op.tag),
              // Convergence is decided HERE, when the repairing run completes,
              // against the tree it produced.  A sticky flag compared against
              // the LIVE tree would be falsified by the next hand edit, which
              // says nothing about whether the bulk mark converged.
              //
              // "The same result" is every member of the INTERRUPTED plan in
              // the target state — not merely "no member still live", which a
              // member someone abandoned in between would satisfy while the
              // re-run plainly did not reach the first run's result.
              bulkDiverged: hist.bulkDiverged or
                (isBulkRetry(hist, op.tag) and
                 hist.bulkPlanKeys.foldl(false, (acc, k) =>
                   acc or idOf(fs, k) == -1 or
                   nameAt(fs, idOf(fs, k)).state != SDone)) },
  }

  /// `EN-08`.  Interruption may occur between any two steps.  What it leaves is
  /// a stable state, and which one is the claim.
  action crashNow: bool = all {
    CRASHES,
    op.running,
    fs' = fs,
    guard' = HNone,
    pend' = Set(),
    op' = { ...op, running: false, pending: List(), outcome: BlockedO(RecoveryPending) },
    hist' = { ...hist,
              crashed: true,
              // Only an interruption with WORK STILL PENDING is an interrupted
              // scaffold.  A crash after the last effect landed but before the
              // guard was released leaves a COMPLETE current root, which is
              // what `TT-20` wants and not what it is stated over.
              initCrashCls: if (op.tag == TInitRoot and op.pending.length() > 0
                                and not(op.foreignDuring))
                              hist.initCrashCls.union(Set(classify(fs)))
                            else hist.initCrashCls,
              initCrashUnderForeign: if (op.tag == TInitRoot and op.pending.length() > 0
                                and op.foreignDuring)
                              hist.initCrashUnderForeign.union(Set(classify(fs)))
                            else hist.initCrashUnderForeign,
              // The FIRST interruption fixes the request the retry is judged
              // against; a later one cannot quietly re-point the ghost at a
              // different plan.
              bulkPartial: hist.bulkPartial or
                           (match op.tag { TBulkMark(_) => op.applied.length() > 0 | _ => false }),
              bulkPlanKeys:
                if (not(hist.bulkPartial))
                  match op.tag {
                    | TBulkMark(ks) => if (op.applied.length() > 0) ks else hist.bulkPlanKeys
                    | _ => hist.bulkPlanKeys
                  }
                else hist.bulkPlanKeys },
  }

  // ---------------------------------------------------------------------------
  // The world's actions — `hand-edit` and `foreign-write`
  // ---------------------------------------------------------------------------
  //
  // ABSTRACTION, stated plainly.  `EN-11` grants that ANY well-formed tree is
  // reachable by hand edit.  A model that reached those trees by composing
  // single-object edits would be faithful and useless: the situations the
  // task-tree claims are about — two live finish leaves in different subtrees, a
  // canonical name inside a foreign directory — sit four or five specific edits
  // deep, and random simulation would visit them at a rate that makes the suite
  // flaky rather than slow.  `hand-edit` is therefore modelled as the operator
  // installing one of an ENUMERATED FAMILY of well-formed trees, plus a local
  // perturbation.  That is a search strategy over exactly the space `EN-11`
  // already grants, not a second assumption — and removing `hand-edit`
  // (`relax_EN_11`) removes the whole family with it, which is what makes the
  // exercise-removal control mean something.

  pure def Ob(parent: Id, k: EntryKind, ty: ObjType, res: bool, dig: int): Obj =
    { parent: parent, kind: k, otype: ty, atReserved: res, digest: dig }

  pure def Lf(pos: int, key: int, slug: int, sk: SessionKind, st: LeafState): EntryKind =
    Task({ pos: pos, key: key, slug: slug, skind: sk, state: st,
           species: SLeaf, spelling: 0 })

  pure def Nd(pos: int, key: int, slug: int): EntryKind =
    Task({ pos: pos, key: key, slug: slug, skind: KAbsent, state: SLive,
           species: SNode, spelling: 0 })

  pure def Bad(pos: int, key: int, slug: int, sk: SessionKind, sp: Species, spell: int): EntryKind =
    Task({ pos: pos, key: key, slug: slug, skind: sk, state: SLive,
           species: sp, spelling: spell })

  pure val WT_OBJ: Obj = Ob(WT, RootDir, ODir, false, 0)
  pure val ROOT_OBJ: Obj = Ob(WT, RootDir, ODir, false, 0)

  /// The three objects every current-format root has: the working-tree root,
  /// the task root, the root charter and the format witness.
  pure val CURRENT_ROOT: Id -> Obj =
    Map(WT -> WT_OBJ,
        ROOT -> ROOT_OBJ,
        2 -> Ob(ROOT, Charter, OFile, false, 10),
        3 -> Ob(ROOT, FormatWitness(FORMAT_CURRENT), OFile, false, 11))

  /// The same without the format witness — a legacy root.
  pure val LEGACY_ROOT: Id -> Obj =
    Map(WT -> WT_OBJ, ROOT -> ROOT_OBJ, 2 -> Ob(ROOT, Charter, OFile, false, 10))

  pure def fsOf(m: Id -> Obj): Fs = { objs: m, nextId: 40 }

  pure val SITUATIONS: Set[int] = 1.to(25)

  pure def situation(s: int): Fs =
    if (s == 1) fsOf(CURRENT_ROOT.put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20)))

    // TT-01: two spellings of one entry, which would otherwise both parse.
    else if (s == 2) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Bad(1, 1, 1, KDesign, SLeaf, 1), OFile, false, 21)))

    // TT-02.a: a leaf name at a directory.
    else if (s == 3) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), ODir, false, 20)))

    // TT-02.b: a node name at a file.
    else if (s == 4) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Nd(1, 1, 1), OFile, false, 20)))

    // TT-03: a malformed node directory whose subtree holds live work.
    else if (s == 5) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Bad(1, 1, 1, KAbsent, SNode, 1), ODir, false, 20))
      .put(5, Ob(4, Charter, OFile, false, 12))
      .put(6, Ob(4, Lf(1, 2, 2, KImpl, SLive), OFile, false, 21)))

    // TT-04: a foreign directory holding a canonical, known-kind task name,
    // beside ordinary siblings an insert will renumber.
    else if (s == 6) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SLive), OFile, false, 21))
      .put(6, Ob(ROOT, Foreign, ODir, false, 90))
      .put(7, Ob(6, Lf(1, 3, 3, KImpl, SLive), OFile, false, 91)))

    // TT-05: the maximum key is on a terminal entry.
    else if (s == 7) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 5, 2, KImpl, SDone), OFile, false, 21)))

    // TT-06, TT-07, TT-10: three ordinary siblings.
    else if (s == 8) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SLive), OFile, false, 21))
      .put(6, Ob(ROOT, Lf(3, 3, 3, KImpl, SLive), OFile, false, 22)))

    // TT-07: a directory containing every species and every leaf state.
    else if (s == 9) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Nd(2, 2, 2), ODir, false, 21))
      .put(6, Ob(5, Charter, OFile, false, 12))
      .put(7, Ob(5, Lf(1, 6, 6, KImpl, SLive), OFile, false, 23))
      .put(8, Ob(ROOT, Lf(3, 3, 3, KImpl, SDone), OFile, false, 22))
      .put(9, Ob(ROOT, Lf(4, 4, 4, KImpl, SAbandoned), OFile, false, 24)))

    // TT-08: the leaf to promote carries the tree's maximum key.
    else if (s == 10) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SDone), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 7, 2, KImpl, SLive), OFile, false, 21)))

    // TT-13.a: a live finish leaf EARLIER than live ordinary work.
    else if (s == 11) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KFinish, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SLive), OFile, false, 21)))

    // TT-13.b: the finish leaf is the only live leaf.
    else if (s == 12) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KFinish, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SDone), OFile, false, 21)))

    // TT-13.c: two individually well-formed live finish leaves, in different
    // subtrees.
    else if (s == 13) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KFinish, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Nd(2, 2, 2), ODir, false, 21))
      .put(6, Ob(5, Charter, OFile, false, 12))
      .put(7, Ob(5, Lf(1, 3, 3, KFinish, SLive), OFile, false, 22)))

    // TT-15.c: two entries sharing one slug.
    else if (s == 14) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 5, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 5, KImpl, SDone), OFile, false, 21)))

    // TT-16: a `Done` entry and an `Abandoned` entry, each resolvable.
    else if (s == 15) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SDone), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SAbandoned), OFile, false, 21))
      .put(6, Ob(ROOT, Lf(3, 3, 3, KImpl, SLive), OFile, false, 22)))

    // TT-17: a LEGACY tree whose slug text and kinds read exactly as current
    // work.  Two entries, so it is not a partial scaffold.
    else if (s == 16) fsOf(LEGACY_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SLive), OFile, false, 21)))

    // TT-18: a reserved witness AND no format witness.
    else if (s == 17) fsOf(LEGACY_ROOT
      .put(4, Ob(ROOT, ReservedWitness(RPublished), OFile, true, 30))
      .put(5, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20)))

    // TT-19: `Reserved(Preparing)` over a tree that looks perfectly walkable.
    else if (s == 18) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, ReservedWitness(RPreparing), OFile, true, 30))
      .put(5, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(6, Ob(ROOT, Lf(2, 2, 2, KImpl, SLive), OFile, false, 21)))

    // TT-24.b: a foreign artifact at a name Grove reserves.
    else if (s == 19) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Foreign, OFile, true, 95))
      .put(5, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20)))

    // The reaper's sweep meets both — Grove's own quarantined material, and a
    // foreign artifact it cannot prove is its own.  Kept for `TT-04` and for
    // `relax_EN_13`; `TT-24.d` itself is retired to `FN-21.c`.
    else if (s == 20) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Quarantined, ODir, true, 96))
      .put(5, Ob(ROOT, Foreign, OFile, true, 95))
      .put(6, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20)))

    // TT-12, TT-25.a: a wholly terminal node the walk crosses on its way to
    // live work.
    else if (s == 21) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Nd(1, 1, 1), ODir, false, 21))
      .put(5, Ob(4, Charter, OFile, false, 12))
      .put(6, Ob(4, Lf(1, 2, 2, KImpl, SDone), OFile, false, 22))
      .put(7, Ob(4, Lf(2, 3, 3, KImpl, SAbandoned), OFile, false, 23))
      .put(8, Ob(ROOT, Lf(2, 4, 4, KImpl, SLive), OFile, false, 24)))

    // TT-11, TT-25.b: selection must DESCEND the node at position 1 before
    // visiting the leaf at position 2.
    else if (s == 22) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Nd(1, 1, 1), ODir, false, 21))
      .put(5, Ob(4, Charter, OFile, false, 12))
      .put(6, Ob(4, Lf(1, 2, 2, KImpl, SLive), OFile, false, 22))
      .put(7, Ob(ROOT, Lf(2, 3, 3, KImpl, SLive), OFile, false, 24)))

    // A spent tree — `TT-15.a`.
    else if (s == 23) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SDone), OFile, false, 20)))

    // TT-23.a: a bulk plan whose LATER member is invalid.  `Abandoned` rather
    // than `Done`: a member already in the plan's TARGET state is admissible
    // (see `bulkMemberOk`), so `Done` would no longer be invalid.
    else if (s == 24) fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SAbandoned), OFile, false, 21)))

    // TT-23.b: a two-member bulk plan, both valid, interruptible between the
    // two renames.
    else fsOf(CURRENT_ROOT
      .put(4, Ob(ROOT, Lf(1, 1, 1, KDesign, SLive), OFile, false, 20))
      .put(5, Ob(ROOT, Lf(2, 2, 2, KImpl, SLive), OFile, false, 21)))

  action handEditTo(s: int): bool = all {
    HAND_EDITS,
    not(op.running),
    fs' = situation(s),
    op' = IDLE_OP,
    guard' = HNone,
    pend' = Set(),
    // The names ARE the counter (`EN-10`): after the operator installs a
    // different tree, what has been issued is what that tree's names say.
    // Carrying the previous tree's keys forward would make `TT-05` fail on the
    // model's own hand-edit abstraction rather than on anything Grove did.
    // The operator installed a DIFFERENT TREE, so a plan interrupted against
    // the previous one is no longer a plan anyone can re-run; keeping it
    // pending would falsify `TT-23.b` on the hand-edit abstraction rather than
    // on anything a bulk mark did.  What was already established — that a
    // repair happened, or that one diverged — is a fact about a completed
    // history and stays.
    hist' = { ...hist, handEdited: true,
              bulkPartial: false, bulkPlanKeys: List(),
              issued: keysOf(situation(s)) },
  }

  pure val focusFamily: Set[int] =
    if (FOCUS == 1) Set(25)
    else if (FOCUS == 2) Set(9)
    else if (FOCUS == 3) Set(10)
    else if (FOCUS == 4) Set(6)
    // The verification family: three trees rather than twenty-five, because a
    // symbolic backend pays for every branch a simulator merely skips.
    else if (FOCUS == 6) Set(1, 8, 21)
    else SITUATIONS

  action handEdit: bool = {
    nondet s = focusFamily.oneOf()
    handEditTo(s)
  }

  /// `EN-06`.  A writer that does not cooperate with the lock, editing at any
  /// point — including in the middle of an operation.
  action foreignWriteAt(r: bool): bool = all {
    FOREIGN_WRITES,
    rootPresent(fs),
    fs.objs.keys().size() < MAX_OBJECTS,
    fs' = { objs: fs.objs.put(fs.nextId, Ob(ROOT, Foreign, OFile, r, 95)),
            nextId: fs.nextId + 1 },
    op' = { ...op, foreignDuring: op.foreignDuring or op.running },
    guard' = guard,
    pend' = pend,
    hist' = { ...hist, foreignWritten: true },
  }

  action foreignWrite: bool = {
    nondet r = Set(true, false).oneOf()
    foreignWriteAt(r)
  }

  /// `EN-10`.  The names are the counter, and entries are never removed.  With
  /// the assumption relaxed, one is — and key allocation re-issues.
  action removeEntryAt(i: Id): bool = all {
    ENTRIES_REMOVABLE,
    recognised(fs).contains(i),
    fs' = { ...fs, objs: fs.objs.keys().exclude(Set(i)).mapBy(j => fs.objs.get(j)) },
    op' = op, guard' = guard, pend' = pend,
    hist' = { ...hist, entryRemoved: true },
  }

  action removeEntry: bool = {
    nondet i = recognised(fs).oneOf()
    removeEntryAt(i)
  }

  // ---------------------------------------------------------------------------
  // The step relation
  // ---------------------------------------------------------------------------

  val keyChoices: Set[int] = keysOf(fs).union(Set(9))
  val slugChoices: Set[int] = recognised(fs).map(i => nameAt(fs, i).slug).union(Set(9))
  val dirChoices: Set[Id] = reachedDirs(fs).union(Set(ROOT))
  val posChoices: Set[int] = 1.to(4)

  val allTags: Set[OpTag] =
    Set(TSelect, TInitRoot, TReap, TFinishTxn,
        TBulkMark(List(1, 2)), TBulkMark(List(2, 1)), TBulkMark(List(1)))
      .union(keyChoices.map(k => TResolve(ByKey(k))))
      .union(slugChoices.map(s => TResolve(BySlug(s))))
      .union(slugChoices.map(s => TChain(BySlug(s))))
      .union(keyChoices.map(k => TKindOf(ByKey(k))))
      .union(dirChoices.map(d => TAddLeaf(d)))
      .union(dirChoices.map(d => posChoices.map(a => TInsertLeaf({ parent: d, at: a }))).flatten())
      .union(keyChoices.map(k => TDecompose(k)))
      .union(keyChoices.map(k => TRetire(k)))
      .union(keyChoices.map(k => TPrune(k)))

  val tagChoices: Set[OpTag] =
    if (FOCUS == 1) Set(TBulkMark(List(1, 2)), TBulkMark(List(2, 1)))
    else if (FOCUS == 2 or FOCUS == 4)
      dirChoices.map(d => posChoices.map(a => TInsertLeaf({ parent: d, at: a }))).flatten()
    else if (FOCUS == 3) keyChoices.map(k => TDecompose(k))
    else if (FOCUS == 5)
      Set(TInitRoot)
        .union(keyChoices.map(k => TRetire(k)))
        .union(keyChoices.map(k => TResolve(ByKey(k))))
    // A CONSTANT menu.  `allTags` is built by mapping and flattening over sets
    // derived from the current tree, which a symbolic backend must encode in
    // full at every step; a fixed seven-element set is what makes the model
    // reachable by `quint verify` at all.  It is a reduction of the SEARCH, and
    // the README says which obligations are model-checked under it and which
    // are simulation-only.
    else if (FOCUS == 6)
      Set(TSelect, TInitRoot, TReap, TFinishTxn,
          TRetire(1), TAddLeaf(ROOT), TInsertLeaf({ parent: ROOT, at: 1 }))
    else if (FOCUS == 7)
      Set(TInitRoot, TAddLeaf(ROOT), TRetire(1))
    else allTags

  action anyBegin: bool = {
    nondet p = PROCS.oneOf()
    nondet t = tagChoices.oneOf()
    beginOp(p, t)
  }

  action anyEndObs: bool = {
    nondet p = PROCS.oneOf()
    endObs(p)
  }

  action step: bool = any {
    anyBegin,
    stepOp,
    stepOpBlocked,
    finishOp,
    crashNow,
    anyEndObs,
    handEdit,
    foreignWrite,
    removeEntry,
  }

  // ---------------------------------------------------------------------------
  // Reading the claims off a state
  // ---------------------------------------------------------------------------

  val atRest: bool = not(op.running)
  val didRun: bool = op.tag != TNone
  val ordinary: bool = didRun and op.tag != TInitRoot

  /// The gate reaches the WALK stage only when nothing earlier in the fixed
  /// order has already answered: the root is present, no artifact sits at a
  /// reserved name, no reserved witness exists, and the format is current.
  /// Every claim about what the walk or a malformity DOES has to be stated
  /// under this, or it is really a claim about the classification order
  /// (`TT-18`) wearing another claim's name.
  pure def walkStageReached(f: Fs): bool =
    rootPresent(f) and foreignAtReserved(f).size() == 0 and
    reservedWitnessIds(f).size() == 0 and formatStage(f) == FCurrent

  pure def underForeign(f: Fs): Set[Id] =
    val fo = f.objs.keys().filter(i => isForeign(f, i))
    range(0, MAX_DEPTH).foldl(f.objs.keys().filter(i => fo.contains(parentOf(f, i))),
      (acc, _) => acc.union(f.objs.keys().filter(i => acc.contains(parentOf(f, i)))))

  /// Positions reversed within every directory: the SAME work, ordered the other
  /// way.  `TT-14`'s witness is that this selects differently, which is what
  /// "the only mechanisms are position and terminality" cashes out to.
  pure def mirrorPositions(f: Fs): Fs =
    { ...f, objs: f.objs.keys().mapBy(i =>
        val o = f.objs.get(i)
        match o.kind {
          | Task(n) =>
              { ...o, kind: Task({ ...n, pos: maxPosIn(f, o.parent) + 1 - n.pos }) }
          | _ => o
        }) }

  /// `TT-17`, narrowed and DECLARED.  The catalogue's `PartialScaffold` is
  /// defined by an exact comparison against a task entry's name AND bytes, so
  /// the literal reading of `TT-17` — the classification never reads a task
  /// entry's text — is contradicted by the catalogue's own state table.  This
  /// model checks the claim over the Current/Legacy/Foreign decision, which is
  /// what `TT-17`'s own witness is about, and records the contradiction as a
  /// finding rather than editing the catalogue under the barrier.
  pure def formatDecision(f: Fs): FormatStage =
    if (formatWitnessIds(f).size() == 0) FLegacy
    else if (formatTagOf(f) != FORMAT_CURRENT) FForeign(formatTagOf(f))
    else FCurrent

  pure def outcomeIsRefusedMalformed(o: Outcome): bool =
    match o { Refused(r) => match r { RefMalformed(_) => true | _ => false } | _ => false }

  pure def outcomeIsReservedNameOccupied(o: Outcome): bool =
    match o { Refused(r) => match r { ReservedNameOccupied(_) => true | _ => false } | _ => false }

  pure def outcomeIsWitnessPending(o: Outcome): bool =
    match o { Refused(r) => match r { WitnessPending(_) => true | _ => false } | _ => false }

  pure def outcomeIsDestinationOccupied(o: Outcome): bool =
    match o { Refused(r) => match r { DestinationOccupied => true | _ => false } | _ => false }

  pure def selectedId(o: Outcome): Id =
    match o {
      | Reported(r) => match r { RSelected(i) => i | _ => -1 }
      | _ => -1
    }

  pure def resolvedOf(o: Outcome): Resolved =
    match o {
      | Reported(r) => match r { RResolved(x) => x | _ => { id: -1, state: SLive } }
      | _ => { id: -1, state: SLive }
    }

  pure def isEmptyO(o: Outcome): bool = match o { Empty => true | _ => false }
  pure def isAmbiguous(o: Outcome): bool = match o { Ambiguous(_) => true | _ => false }
  pure def isApplied(o: Outcome): bool = match o { Applied => true | _ => false }
  pure def isBlockedConflict(o: Outcome): bool =
    match o { BlockedO(b) => match b { OwnershipConflict => true | _ => false } | _ => false }

  // ===========================================================================
  // Claims — `TT-01` .. `TT-25`
  //
  // `inv_<OB>_<mnemonic>` must HOLD; `wit_<OB>_<mnemonic>` must be REACHED.
  // The obligation identifier is spelled without its separators, so `TT-02.b`
  // is `TT_02b` — that is what `models/run.sh` matches against the catalogue.
  // ===========================================================================

  // --- TT-01 — a name has exactly one spelling -------------------------------

  val inv_TT_01a_distinct_names_never_denote_one_entry: bool =
    recognised(fs).forall(i => recognised(fs).forall(j =>
      (i != j) implies (nameAt(fs, i) != nameAt(fs, j))))

  val wit_TT_01a_two_spellings_of_one_entry: bool =
    reachedTasks(fs).exists(i => reachedTasks(fs).exists(j =>
      i != j and parentOf(fs, i) == parentOf(fs, j) and
      nameAt(fs, i).spelling != nameAt(fs, j).spelling and
      { ...nameAt(fs, i), spelling: 0 } == { ...nameAt(fs, j), spelling: 0 }))

  val inv_TT_01b_noncanonical_refused_naming_the_canonical: bool =
    and {
      recognised(fs).forall(i => nameAt(fs, i).spelling == 0),
      (atRest and ordinary and walkStageReached(op.snap) and
       reachedTasks(op.snap).exists(i => nameAt(op.snap, i).spelling != 0) and
       reachedTasks(op.snap).forall(i =>
         nameAt(op.snap, i).spelling != 0 or parsesCompletely(nameAt(op.snap, i))))
        implies (outcomeIsRefusedMalformed(op.outcome) and op.canonicalNamed and
                 op.applied.length() == 0),
    }

  val wit_TT_01b_a_noncanonical_spelling_refused: bool =
    atRest and ordinary and op.canonicalNamed and
    outcomeIsRefusedMalformed(op.outcome) and
    reachedTasks(op.snap).exists(i => nameAt(op.snap, i).spelling != 0)

  // --- TT-02 — a name declares its species and must be it --------------------

  val inv_TT_02a_leaf_name_at_a_directory_is_malformed: bool =
    reachedTasks(fs).forall(i =>
      (nameAt(fs, i).species == SLeaf and typeOf(fs, i) == ODir)
        implies malformityOf(fs).contains(SpeciesMismatch(i)))

  val wit_TT_02a_leaf_name_at_a_directory: bool =
    reachedTasks(fs).exists(i =>
      nameAt(fs, i).species == SLeaf and typeOf(fs, i) == ODir) and
    isMalformed(fs)

  val inv_TT_02b_node_name_at_a_file_is_malformed: bool =
    reachedTasks(fs).forall(i =>
      (nameAt(fs, i).species == SNode and typeOf(fs, i) == OFile)
        implies malformityOf(fs).contains(SpeciesMismatch(i)))

  val wit_TT_02b_node_name_at_a_file: bool =
    reachedTasks(fs).exists(i =>
      nameAt(fs, i).species == SNode and typeOf(fs, i) == OFile) and
    isMalformed(fs)

  // --- TT-03 — malformed halts, and never skips ------------------------------

  val inv_TT_03_malformed_halts_every_read_and_mutation: bool =
    (atRest and ordinary and walkStageReached(op.snap) and isMalformed(op.snap))
      implies (isRefusal(op.outcome) and op.applied.length() == 0)

  val wit_TT_03_a_malformed_node_whose_subtree_holds_live_work: bool =
    atRest and ordinary and outcomeIsRefusedMalformed(op.outcome) and
    reachedTasks(op.snap).exists(i =>
      nameAt(op.snap, i).species == SNode and
      not(parsesCompletely(nameAt(op.snap, i))) and
      liveIds(op.snap).exists(j => parentOf(op.snap, j) == i))

  // --- TT-04 — foreign entries are ignored and preserved ---------------------

  val inv_TT_04_foreign_subtrees_are_invisible_and_untouched: bool =
    and {
      not(hist.foreignMutated),
      underForeign(fs).forall(i => not(reached(fs).contains(i))),
      underForeign(fs).forall(i =>
        not(isTask(fs, i)) or not(keysOf(fs).contains(nameAt(fs, i).key)) or
        recognised(fs).exists(j => j != i and nameAt(fs, j).key == nameAt(fs, i).key)),
    }

  val wit_TT_04_a_canonical_task_name_inside_a_foreign_directory: bool =
    underForeign(fs).exists(i => isTask(fs, i) and parsesCompletely(nameAt(fs, i)))

  // --- TT-05 — keys are unique, permanent and never reissued -----------------

  val inv_TT_05_no_key_is_ever_issued_twice: bool = not(hist.reissued)

  val wit_TT_05_allocation_whose_maximum_comes_from_a_terminal_entry: bool =
    atRest and isApplied(op.outcome) and op.allocated >= 0 and
    recognised(op.beforeFs).exists(i =>
      nameAt(op.beforeFs, i).key == op.allocated - 1 and
      nameAt(op.beforeFs, i).state != SLive)

  // --- TT-06 — positions are per-directory and gapless -----------------------

  val inv_TT_06a_append_lands_at_n_plus_one: bool =
    (atRest and isApplied(op.outcome) and isCurrent(classify(op.beforeFs)))
      implies reachedDirs(fs).forall(d => gapless(fs, d))

  val wit_TT_06a_an_append_that_closes_no_gap: bool =
    atRest and isApplied(op.outcome) and op.alg == AAppend and op.tag != TInitRoot and
    op.applied.length() == 1

  val inv_TT_06b_an_insert_shifts_every_later_sibling: bool =
    (atRest and isApplied(op.outcome) and op.alg == AInsert)
      implies reachedDirs(fs).forall(d => gapless(fs, d))

  val wit_TT_06b_an_insert_at_an_occupied_position: bool =
    atRest and isApplied(op.outcome) and op.alg == AInsert and op.preempted and
    op.applied.length() > 1

  // --- TT-07 — a shift preserves everything but position ---------------------

  val inv_TT_07_a_shift_changes_position_and_nothing_else: bool =
    (atRest and isApplied(op.outcome) and op.alg == AInsert) implies
      op.applied.foldl(true, (acc, e) =>
        match e {
          | ERename(r) =>
              acc and has(fs, r.id) and has(op.beforeFs, r.id) and
              fs.objs.get(r.id).digest == op.beforeFs.objs.get(r.id).digest and
              { ...nameAt(fs, r.id), pos: 0 } == { ...nameAt(op.beforeFs, r.id), pos: 0 } and
              nameAt(fs, r.id).pos == nameAt(op.beforeFs, r.id).pos + 1
          | _ => acc
        })

  // --- TT-08 — decomposition preserves the key -------------------------------

  val inv_TT_08_promotion_preserves_the_key: bool =
    (atRest and isApplied(op.outcome) and op.alg == APromote) implies
      and {
        recognised(op.beforeFs).forall(i =>
          not(has(fs, i)) or not(isTask(fs, i)) or
          nameAt(fs, i).key == nameAt(op.beforeFs, i).key),
        recognised(op.beforeFs).forall(i =>
          keysOf(fs).contains(nameAt(op.beforeFs, i).key)),
      }

  // --- TT-09 — every mutation is one algebraic operation ---------------------

  pure def effectsAreAlgebraic(a: Algebraic, es: List[Effect]): bool =
    match a {
      | AAppend  => es.foldl(true, (acc, e) => acc and match e {
                      | ECreate(_) => true | EPublish(_) => true | _ => false })
      | AInsert  => es.foldl(true, (acc, e) => acc and match e {
                      | ECreate(_) => true | ERename(_) => true | _ => false })
      | APromote => es.foldl(true, (acc, e) => acc and match e {
                      | ECreate(_) => true | EReparent(_) => true | _ => false })
      | ARewrite => es.foldl(true, (acc, e) => acc and match e {
                      | ERename(_) => true | ECreate(_) => true | ERemove(_) => true
                      | _ => false })
      | ANone    => es.length() == 0
    }

  /// One invariant PER OBLIGATION, not one per claim: the manifest's unit is
  /// the obligation, and a single `inv_TT_09_…` would leave all four of
  /// `TT-09.a` – `TT-09.d` with no property command while looking covered.
  pure def algebraIntact(a: Algebraic, alg: Algebraic, es: List[Effect],
                         settled: bool, applied: bool): bool =
    and {
      (settled and applied) implies alg != ANone,
      alg != a or effectsAreAlgebraic(a, es),
    }

  val inv_TT_09a_append_stays_in_the_algebra: bool =
    algebraIntact(AAppend, op.alg, op.applied, atRest, isApplied(op.outcome))

  val inv_TT_09b_insert_stays_in_the_algebra: bool =
    algebraIntact(AInsert, op.alg, op.applied, atRest, isApplied(op.outcome))

  val inv_TT_09c_promotion_stays_in_the_algebra: bool =
    algebraIntact(APromote, op.alg, op.applied, atRest, isApplied(op.outcome))

  val inv_TT_09d_rewrite_stays_in_the_algebra: bool =
    algebraIntact(ARewrite, op.alg, op.applied, atRest, isApplied(op.outcome))

  val wit_TT_09a_append: bool = atRest and isApplied(op.outcome) and op.alg == AAppend
  val wit_TT_09b_insert: bool = atRest and isApplied(op.outcome) and op.alg == AInsert
  val wit_TT_09d_rewrite: bool = atRest and isApplied(op.outcome) and op.alg == ARewrite
  // --- TT-10 — no algebraic refusal reaches an operator ----------------------

  /// QUALIFIED, AND THE QUALIFICATION IS DECLARED.  Read over `op.outcome`
  /// this would be a claim about the WORLD as well as about arguments: a
  /// non-cooperating writer can take a create's destination after the listing
  /// and before the step, and the refusal the operator then sees is caused by
  /// `EN-06`, not by an argument the domain failed to pre-empt.  `TT-10`'s own
  /// text is "no algebraic refusal reaches an operator FROM AN ORDINARY
  /// ARGUMENT", so it is stated over `op.decided` — what the operation
  /// concluded from its arguments and its one listing.  This is not a
  /// catalogue conflict and not a gap: the mid-flight case has its own
  /// obligations (`TT-21.b`, `TT-24.a`) and its own outcome, decided by
  /// `collisionOutcome`.
  val inv_TT_10_the_domain_runs_in_front_of_the_algebra: bool =
    outcomeIsDestinationOccupied(op.decided) implies (op.tag == TInitRoot)

  val wit_TT_10_an_argument_that_would_reach_an_algebraic_refusal_preempted: bool =
    atRest and isApplied(op.outcome) and op.preempted

  // --- TT-11 — selection is a stateless pre-order walk -----------------------

  val inv_TT_11_selection_is_the_preorder_minimum: bool =
    (atRest and op.tag == TSelect and selectedId(op.outcome) != -1) implies
      val i = selectedId(op.outcome)
      and {
        liveIds(op.snap).contains(i),
        liveOrdinaryIds(op.snap).forall(j => preorderKey(op.snap, i) <= preorderKey(op.snap, j)),
      }

  val wit_TT_11_selection_descends_a_node_before_a_later_sibling: bool =
    atRest and op.tag == TSelect and selectedId(op.outcome) != -1 and
    parentOf(op.snap, selectedId(op.outcome)) != ROOT and
    liveIds(op.snap).exists(j => parentOf(op.snap, j) == ROOT and
      nameAt(op.snap, j).pos > nameAt(op.snap, parentOf(op.snap, selectedId(op.outcome))).pos)

  // --- TT-12 — terminal entries are skipped, never removed -------------------

  val inv_TT_12_terminal_entries_are_never_removed: bool = not(hist.entryRemoved)

  val wit_TT_12_a_walk_crossing_a_wholly_terminal_node: bool =
    atRest and op.tag == TSelect and selectedId(op.outcome) != -1 and
    reachedTasks(op.snap).exists(d =>
      nameAt(op.snap, d).species == SNode and typeOf(op.snap, d) == ODir and
      recognised(op.snap).exists(j => parentOf(op.snap, j) == d) and
      not(liveIds(op.snap).exists(j => parentOf(op.snap, j) == d)) and
      preorderKey(op.snap, d) < preorderKey(op.snap, selectedId(op.outcome)))

  // --- TT-13 — finish is reserved, not blocking ------------------------------

  val inv_TT_13a_the_walk_skips_a_live_finish_leaf: bool =
    (liveOrdinaryIds(fs).size() > 0) implies
      selectIn(fs).forall(i => nameAt(fs, i).skind != KFinish)

  val wit_TT_13a_a_finish_leaf_earlier_than_live_ordinary_work: bool =
    atRest and op.tag == TSelect and selectedId(op.outcome) != -1 and
    nameAt(op.snap, selectedId(op.outcome)).skind != KFinish and
    liveFinishIds(op.snap).exists(f =>
      preorderKey(op.snap, f) < preorderKey(op.snap, selectedId(op.outcome)))

  val inv_TT_13b_the_walk_returns_the_only_live_finish_leaf: bool =
    (liveOrdinaryIds(fs).size() == 0 and liveFinishIds(fs).size() == 1) implies
      selectIn(fs) == liveFinishIds(fs)

  val wit_TT_13b_the_finish_leaf_is_the_only_live_leaf: bool =
    atRest and op.tag == TSelect and selectedId(op.outcome) != -1 and
    nameAt(op.snap, selectedId(op.outcome)).skind == KFinish

  val inv_TT_13c_two_live_finish_leaves_malform_the_whole_tree: bool =
    (liveFinishIds(fs).size() > 1 and walkStageReached(fs)) implies
      and {
        classify(fs) == MalformedRoot(MultipleLiveFinish),
        malformityOf(fs).contains(MultipleLiveFinish),
      }

  val wit_TT_13c_two_live_finish_leaves_in_different_subtrees: bool =
    liveFinishIds(fs).size() > 1 and
    liveFinishIds(fs).exists(i => liveFinishIds(fs).exists(j =>
      i != j and parentOf(fs, i) != parentOf(fs, j))) and
    classify(fs) == MalformedRoot(MultipleLiveFinish)

  // --- TT-14 — selection is not a scheduler ----------------------------------

  val inv_TT_14_selection_reads_no_text: bool =
    selectIn(fs).map(i => preorderKey(fs, i)) ==
    selectIn(perturbSlugs(fs)).map(i => preorderKey(perturbSlugs(fs), i))

  val wit_TT_14_two_orderings_of_the_same_work_select_differently: bool =
    isCurrent(classify(fs)) and selectIn(fs).size() == 1 and
    selectIn(mirrorPositions(fs)).size() == 1 and
    selectIn(fs) != selectIn(mirrorPositions(fs))

  // --- TT-15 — an empty or ambiguous observation is a success ----------------

  pure def observationIsClean(t: OpTag, o: Outcome, applied: List[Effect], settled: bool): bool =
    (settled and isObservation(t)) implies
      and {
        applied.length() == 0,
        isSuccessObservation(o) or isRefusal(o) or o == Idle,
      }

  val inv_TT_15a_selection_on_a_spent_tree_reports_empty: bool =
    and {
      observationIsClean(op.tag, op.outcome, op.applied, atRest),
      // NARROWED, AND THE NARROWING IS DECLARED — the third of them, beside
      // `TT-17` and `TT-20`, and a catalogue conflict of the same kind rather
      // than a modelling convenience.
      //
      // The literal claim is that every snapshot classifying `CurrentSpent`
      // returns `Empty`.  A current root with no live task and a FOREIGN
      // artifact at a reserved name classifies `CurrentSpent` — `classify`
      // reaches its walk stage and finds nothing live — while `TT-24.b`
      // requires that same tree to refuse `ReservedNameOccupied`, and
      // `TT-18` puts that refusal two stages ahead of anything the walk says.
      // Both are the catalogue's; under one tree they are inconsistent, and
      // `TT-24.b` is the one whose whole purpose is to win.
      //
      // `walkStageReached` states the staging premise the claim leaves
      // implicit: on a tree the gate actually walks, selection on a spent tree
      // reports `Empty`.  Without it this obligation is a claim about the
      // classification ORDER wearing selection's name.  Recorded rather than
      // fixed — the catalogue is frozen under the independence barrier, and
      // `formal-synthesis-k16` owns the disposition.
      (atRest and op.tag == TSelect and walkStageReached(op.snap) and
       classify(op.snap) == CurrentSpent)
        implies isEmptyO(op.outcome),
    }

  val inv_TT_15b_a_resolution_matching_nothing_reports_empty: bool =
    and {
      observationIsClean(op.tag, op.outcome, op.applied, atRest),
      (atRest and isSuccessObservation(op.outcome) and
       match op.tag { TResolve(q) => resolveIn(op.snap, q).size() == 0 | _ => false })
        implies isEmptyO(op.outcome),
    }

  val inv_TT_15c_a_resolution_matching_several_reports_ambiguous: bool =
    and {
      observationIsClean(op.tag, op.outcome, op.applied, atRest),
      (atRest and isSuccessObservation(op.outcome) and
       match op.tag { TResolve(q) => resolveIn(op.snap, q).size() > 1 | _ => false })
        implies isAmbiguous(op.outcome),
    }

  val wit_TT_15a_selection_on_a_spent_tree_reports_empty: bool =
    atRest and op.tag == TSelect and isEmptyO(op.outcome) and
    classify(op.snap) == CurrentSpent

  val wit_TT_15b_a_resolution_matching_nothing_reports_empty: bool =
    atRest and isEmptyO(op.outcome) and
    match op.tag { TResolve(_) => true | _ => false }

  val wit_TT_15c_a_resolution_matching_several_reports_ambiguous: bool =
    atRest and isAmbiguous(op.outcome) and
    match op.tag { TResolve(_) => true | _ => false }

  // --- TT-16 — a resolved terminal entry is never mistaken for live ----------

  pure def resolutionIsFaithful(o: Outcome, snap: Fs, st: LeafState, settled: bool): bool =
    (settled and resolvedOf(o).id != -1 and
     nameAt(snap, resolvedOf(o).id).state == st)
      implies resolvedOf(o).state == st

  val inv_TT_16a_a_resolved_done_entry_is_reported_terminal: bool =
    resolutionIsFaithful(op.outcome, op.snap, SDone, atRest)

  val inv_TT_16b_a_resolved_abandoned_entry_is_reported_terminal: bool =
    resolutionIsFaithful(op.outcome, op.snap, SAbandoned, atRest)

  val wit_TT_16a_a_resolved_done_entry: bool =
    atRest and resolvedOf(op.outcome).id != -1 and resolvedOf(op.outcome).state == SDone

  val wit_TT_16b_a_resolved_abandoned_entry: bool =
    atRest and resolvedOf(op.outcome).id != -1 and resolvedOf(op.outcome).state == SAbandoned

  // --- TT-17 — format is decided by the witness's content --------------------

  val inv_TT_17_the_format_decision_reads_no_task_entry_text: bool =
    formatDecision(fs) == formatDecision(perturbText(fs))

  val wit_TT_17_a_legacy_tree_whose_text_reads_as_current: bool =
    classify(fs) == Legacy and
    recognised(fs).exists(i => nameAt(fs, i).skind == KDesign or
                               nameAt(fs, i).skind == KImpl)

  // --- TT-18 — classification order is fixed ---------------------------------

  val inv_TT_18_reserved_precedes_format_precedes_the_walk: bool =
    and {
      (rootPresent(fs) and reservedWitnessIds(fs).size() > 0) implies
        match classify(fs) { Reserved(_) => true | _ => false },
      (rootPresent(fs) and reservedWitnessIds(fs).size() == 0 and
       formatWitnessIds(fs).size() == 0) implies
        (classify(fs) == Legacy or classify(fs) == PartialScaffold),
      isCurrent(classify(fs)) implies formatStage(fs) == FCurrent,
    }

  val wit_TT_18_a_reserved_witness_and_no_format_witness: bool =
    reservedWitnessIds(fs).size() > 0 and formatWitnessIds(fs).size() == 0 and
    match classify(fs) { Reserved(_) => true | _ => false }

  // --- TT-19 — a reserved witness refuses everything else --------------------

  val inv_TT_19_a_reserved_witness_refuses_everything_else: bool =
    (atRest and ordinary and rootPresent(op.snap) and
     reservedWitnessIds(op.snap).size() > 0 and
     foreignAtReserved(op.snap).size() == 0)
      implies (outcomeIsWitnessPending(op.outcome) and op.applied.length() == 0)

  val wit_TT_19_a_preparing_tree_that_looks_perfectly_walkable: bool =
    atRest and ordinary and outcomeIsWitnessPending(op.outcome) and
    resClassOf(op.snap) == RPreparing and liveIds(op.snap).size() > 1

  // --- TT-20 — the format witness lands last ---------------------------------

  // `Absent` is admitted alongside `PartialScaffold` and the admission is the
  // model's, not the catalogue's: an interruption before the FIRST step lands
  // leaves no task root at all, and `TT-20` is stated over "the root it leaves
  // behind".  Root absence is a first-class state (`SY-05`), not a torn one.
  // What the claim forbids — `Current(*)` and `Legacy` — stays forbidden.
  //
  // NARROWED, AND THE NARROWING IS THE FINDING.  Only interruptions with NO
  // foreign write during the initialisation are collected here.  With one, the
  // interrupted root classifies `Legacy` — the classification `TT-20` names as
  // forbidden — because the catalogue defines `PartialScaffold` by an EXACT
  // closed subset of the root's contents while `EN-13` grants that a foreign
  // entry may appear at any name.  The two are inconsistent under interleaving,
  // and the counterexample is `wit_finding_...` below.  Recorded rather than
  // fixed: the catalogue is frozen under the independence barrier.
  val inv_TT_20_an_interrupted_scaffold_is_a_partial_scaffold: bool =
    hist.initCrashCls.forall(c => c == PartialScaffold or c == Absent)

  /// NOT an obligation command — a finding's counterexample, kept runnable.
  /// A stray foreign entry beside a half-written scaffold makes Grove read its
  /// own interrupted work as someone else's legacy tree.
  val wit_finding_partial_scaffold_is_not_robust_to_a_foreign_write: bool =
    hist.initCrashUnderForeign.exists(c => c != PartialScaffold and c != Absent)

  val wit_TT_20_an_interruption_before_the_witness_lands: bool =
    hist.initCrashCls.contains(PartialScaffold)

  // --- TT-21 — one snapshot per operation ------------------------------------

  val inv_TT_21a_every_classification_comes_from_one_listing: bool =
    op.cls.foldl(true, (acc, c) => acc and (op.cls.length() == 0 or c == op.cls.head()))

  val wit_TT_21a_a_cooperating_writer_shown_serialized: bool =
    op.cls.length() > 1 and op.coopDuring

  val inv_TT_21b_a_noncooperating_writer_does_not_falsify_the_claim: bool =
    and {
      op.cls.foldl(true, (acc, c) => acc and (op.cls.length() == 0 or c == op.cls.head())),
      (op.foreignDuring and atRest) implies
        (isApplied(op.outcome) or isRefusal(op.outcome) or
         match op.outcome { BlockedO(_) => true | _ => false }),
    }

  val wit_TT_21b_a_foreign_write_between_two_classifications: bool =
    op.cls.length() > 1 and op.foreignDuring

  // --- TT-22 — shared for observation, exclusive for mutation ----------------

  val inv_TT_22a_two_observations_are_admitted: bool =
    and {
      match guard { HExcl(p) => holders(guard) == Set(p) | _ => true },
      (op.running and not(isObservation(op.tag))) implies guard == HExcl(op.actor),
    }

  val wit_TT_22a_two_concurrent_observations: bool = sharedSet(guard).size() > 1

  val inv_TT_22b_observation_and_mutation_are_serialized: bool =
    (op.running and not(isObservation(op.tag))) implies sharedSet(guard).size() == 0

  val wit_TT_22b_a_caller_deferred_behind_a_held_guard: bool =
    pend.size() > 0 and guard != HNone

  // --- TT-23 — a bulk mark validates before it moves, and converges ----------

  val inv_TT_23a_the_whole_plan_is_validated_before_the_first_rename: bool =
    not(hist.bulkInvalidApplied)

  val wit_TT_23a_a_later_invalid_member_refused_before_the_first_rename: bool =
    atRest and op.applied.length() == 0 and isRefusal(op.outcome) and
    match op.tag { TBulkMark(ks) => ks.length() > 1 | _ => false }

  val inv_TT_23b_re_running_after_a_partial_application_converges: bool =
    not(hist.bulkDiverged)

  // --- TT-24 — fail-closed ownership -----------------------------------------

  val inv_TT_24a_no_action_mutates_what_it_cannot_prove_is_its_own: bool =
    not(hist.unownedMutated)

  val wit_TT_24a_a_mutation_against_an_unprovable_entry_not_taken: bool =
    atRest and op.applied.length() == 0 and
    (outcomeIsReservedNameOccupied(op.outcome) or isBlockedConflict(op.outcome))

  val inv_TT_24b_an_ordinary_operation_refuses_reserved_name_occupied: bool =
    (atRest and ordinary and not(isTransaction(op.tag)) and op.tag != TReap and
     rootPresent(op.snap) and foreignAtReserved(op.snap).size() > 0)
      implies (outcomeIsReservedNameOccupied(op.outcome) and op.applied.length() == 0)

  val wit_TT_24b_an_ordinary_operation_meets_a_foreign_reserved_name: bool =
    atRest and outcomeIsReservedNameOccupied(op.outcome)

  // `TT-24.c` AND `TT-24.d` WERE HERE AND ARE RETIRED, not deleted for
  // convenience.  Both are stated over `grove-finish`'s contexts — a live
  // transaction, and the quarantine reaper — and `grove-finish` depends on
  // `grove-task-tree` rather than the reverse, so neither is an obligation this
  // crate can deliver
  // (`docs/adr/obligations-follow-context-not-artifact.md`).  They are `FN-32`
  // and `FN-21.c`, both answered in `crates/grove-finish/models/`.
  //
  // `inv_TT_24c` was a transcription of `gateOutcome`'s own branch that no
  // control killed, and the Alloy column declared the same cell out-of-bounds
  // rather than fill it that way.  The two columns disagreeing about one cell
  // is what surfaced the placement question; `TT-24.a` above is quantified over
  // every action and is what the shared-safety register means by `TT-24`.
  //
  // `TFinishTxn`, `TReap` and `BlockedO` STAY, because surviving obligations
  // read them: `wit_TT_24a` accepts either non-mutating outcome, `TT-24.b`
  // excludes both non-ordinary tags by name, and `relax_EN_13` needs the sweep
  // for `TT-04`.  What went with the two obligations is `hist.reapWitness`,
  // whose only reader was `inv_TT_24d`.

  // --- TT-25 — a node is never marked ----------------------------------------

  /// A shift renames a node — that is a POSITION, not a state.  What `TT-25`
  /// forbids is writing a node's done-ness, so the clause below is about the
  /// state field and nothing else.
  val noActionWritesANodesState: bool =
    op.applied.foldl(true, (acc, e) =>
      acc and match e {
        | ERename(r) => r.name.species != SNode or r.name.state == SLive
        | _ => true
      })

  val inv_TT_25a_a_wholly_terminal_node_is_derived_done_unmarked: bool =
    and {
      noActionWritesANodesState,
      recognised(fs).forall(i =>
        (nameAt(fs, i).species == SNode and
         not(liveIds(fs).exists(j => parentOf(fs, j) == i)))
          implies nameAt(fs, i).state == SLive),
    }

  val inv_TT_25b_a_node_with_a_live_leaf_is_derived_live_unmarked: bool =
    and {
      noActionWritesANodesState,
      recognised(fs).forall(i =>
        (nameAt(fs, i).species == SNode and
         liveIds(fs).exists(j => parentOf(fs, j) == i))
          implies nameAt(fs, i).state == SLive),
    }

  val wit_TT_25a_a_wholly_terminal_node_is_derived_done_unmarked: bool =
    recognised(fs).exists(d =>
      nameAt(fs, d).species == SNode and typeOf(fs, d) == ODir and
      recognised(fs).exists(j => parentOf(fs, j) == d) and
      not(liveIds(fs).exists(j => parentOf(fs, j) == d)) and
      nameAt(fs, d).state == SLive)

  val wit_TT_25b_a_node_with_a_live_leaf_beneath_it_is_derived_live: bool =
    recognised(fs).exists(d =>
      nameAt(fs, d).species == SNode and typeOf(fs, d) == ODir and
      liveIds(fs).exists(j => parentOf(fs, j) == d) and
      nameAt(fs, d).state == SLive)
}

// =============================================================================
// Instances
//
// `taskTree` above carries `const` declarations and is therefore a LIBRARY, not
// a run: every module below binds the assumption dial and is what `quint run
// --main=` is pointed at.
//
// THE MODULE RULE IS DEFINED IN ONE PLACE — `models/run.sh`, *THE MODULE RULE*
// — and cited here rather than restated, because a convention written out in
// four files is a convention with four versions.  In brief: a `relax_`,
// `mutant_` or `scenario_` instance carries only its own commands; a `verify_`
// instance is model-checked and inherits the library's PROPERTY commands only;
// every other instance inherits all of them and must satisfy every one.
// =============================================================================

module base {
  import taskTree(
    RENAME_ATOMIC = true,
    FOREIGN_WRITES = true,
    CRASHES = true,
    ENTRIES_REMOVABLE = false,
    HAND_EDITS = true,
    REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true,
    FOCUS = 0, MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).*
}

/// The VERIFICATION instance: the same library, reduced so that a symbolic
/// backend can reach it.  Smaller bounds, a three-tree hand-edit family and a
/// constant action menu; every assumption still granted, every action still
/// present, `crash` and `foreign-write` included.  What it buys over `base` is
/// the direction simulation cannot give — no counterexample EXISTS within the
/// depth, rather than none was sampled.  What it costs is stated where it
/// belongs, in the README: it is a smaller world, and a green here is not a
/// green over `base`'s.
module verify_small {
  import taskTree(
    FOCUS = 7,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = false, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true,
    MAX_OBJECTS = 6, MAX_DEPTH = 2, MAX_POS = 2
  ).*
}
