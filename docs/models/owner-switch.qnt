// ===========================================================================
// owner-switch — behavioural model of the switch's recovery
// ===========================================================================
//
// The subject is `docs/specs/owner-skill-delivery.md`, the sections "The
// switch inside finish", "Concurrent writers", "Resuming the switch", "The
// rehearsed plan" and "Rollback".  The model is a design artifact: it asks
// whether the recovery rules keep four properties when interruptions, outside
// writers, inverses, classifications and receipts interleave.  The executable
// rehearsal of order 6 stays with the cutover contract.
//
// HOW TO READ IT
//
//   Claims are named `val`s.  `inv_…` must hold in every reachable state of
//   its instance; `wit_…` must be reached.  `run-owner-switch.sh` runs both,
//   per instance.  A mutant instance removes one mechanism, and its `wit_…`
//   reaches the violation that the mechanism prevents.  Each such witness
//   writes the violated conjuncts out inline, so it keeps its meaning if an
//   invariant's definition changes.
//
// THE FOUR PROPERTIES
//
//   P1 inv_noSilentLoss            No write by another program is lost
//                                  without a report.  The accepted case is a
//                                  cell write of the cell's earlier or
//                                  switched value; the run's change list
//                                  reports every value that it replaces.
//   P2 inv_exactlyOneState         Exactly one row of the state table holds.
//      inv_stateMatchesTable       The run's decision equals that row.
//      inv_interruptedOnlyWhenOpen An interruption is read only while a run
//      inv_settledOnlyWhenClosed   is really open, and a settled state only
//                                  while none is.
//   P3 inv_receiptRecordsSwitched  No receipt certifies a layout that is not
//      inv_receiptMatchesProbes    switched, and a receipt records the layout
//      inv_completeOnlySwitched    that the probes saw.
//   P4 inv_onlyRehearsedApplied    The real HOME applies only a rehearsed
//                                  plan.
//
// WHAT IS MODELLED
//
//   * A part is one atomic step of the switch.  Each part has one CELL: the
//     value that the switch changes.  Value 0 is the first plan's earlier
//     value, value 1 is the switched value, and every other value is written
//     by an outside program.  A file part also has REST: the other keys of
//     the file, as the set of outside writes that reached them.
//   * An exchange puts the prepared content at the path and the displaced
//     content under the stage name, which becomes a KEPT COPY.  Before the
//     exchange, the run records the content that the copy must hold.  A copy
//     is FLAGGED when it holds neither that content nor the prepared one, and
//     every later check reports a flagged copy.
//   * Outside programs write cells and rest at any step, and a descriptor
//     opened before an exchange writes into a kept copy.  No exclusion is
//     modelled: P1 must hold without one.
//   * A crash ends the run at any step.  Only the layout, the kept copies,
//     the backups, the plans and the rehearsal records survive it.
//   * The backup's run record is `begun`, `receipt`, `invBegun`, `invEnded`.
//
// WHAT IS NOT MODELLED — the worklist for the rehearsal
//
//   * Entry parts.  The model treats every part as a file.  An entry moves by
//     an exclusive rename, so a move overwrites nothing.  The inverse moves
//     back an entry of any identity, and a changed identity is a third value
//     afterwards.  The rehearsal controls own that rule.
//   * Bytes, digests, metadata and names.  "Same content" is equality of the
//     cell and the rest.  The prepared file's mode, ACL and extended
//     attributes, and the digest names of kept copies, are prose.
//   * The program.  The model has one program.  The rule that a backup keeps
//     the program that began it, and that its continuations run that copy,
//     is prose.
//   * The probes, the native probe, `mac-check` and the manual.  A probe
//     passes or fails.  That the manual never presents a receipt as current
//     is prose.
//   * The manifest of entries outside the switch set.  A changed entry there
//     behaves as a third value in a native state.
//   * Readers.  The exclusion protects readers, captures and progress, and no
//     property here depends on it.
//   * Results under `quint run` are sampled, not proved.  The runner quotes
//     the step bound and the sample count beside each result.
//
// ===========================================================================

module switch {

  // -------------------------------------------------------------------------
  // Dials.  A dial narrows the search; it removes no behaviour that a
  // property needs, and each instance says which world it explores.
  // -------------------------------------------------------------------------
  const N: int                 // parts, in forward order 0 .. N-1
  const MAX_OUTSIDE: int       // budget of outside writes of every kind
  const MAX_CRASH: int         // budget of interruptions
  const MAX_PLANS: int         // budget of plans, the first one included
  const OUTSIDE_CELLS: bool
  const OUTSIDE_REST: bool
  const LATE_WRITES: bool
  const PROBES_CAN_FAIL: bool
  // When outside writes may act: "any" step, only while a "run" is live, or
  // only after a "receipt".  The unfocused worlds use "any".  A focused
  // world is a strict subset that still reaches its subject, and it only
  // hunts witnesses: a green there says nothing about the wide world.
  const OUTSIDE_PHASE: str

  // Mechanisms.  The corrected design sets each one as its comment says; a
  // mutant instance flips exactly one.
  const RECORD_EXPECT: bool       // true: record the expected displaced content
  const RUN_RECORD: bool          // true: the state reads the run record
  const EARLY_END: bool           // false: `invEnded` only after the inverse
  const RECEIPT_RECHECK: bool     // true: re-read the layout before a receipt
  const REHEARSAL_CHECK: bool     // true: refuse an unrehearsed plan
  const INVERSE_SKIPS_THIRD: bool // true: the inverse leaves a third value

  pure val PARTS: Set[int] = 0.to(N - 1)
  pure val SW: int = 1
  pure val FRESH: int = -1
  pure val MAX_KEPT: int = 40

  // -------------------------------------------------------------------------
  // State.  One record, so that every action assigns the whole world.
  // -------------------------------------------------------------------------
  type Content = { cell: int, by: int, rest: Set[int] }   // `by` is ghost
  type Kept = {
    part: int,
    holds: Content,
    expect: Content,       // the recorded content that the copy must hold
    hasExpect: bool,       // false only in the mutant that records nothing
    prepared: Content,
    exchanged: bool,       // ghost: only a displaced inode has a descriptor
    resolved: bool,        // the owner merged the copy ...
    resolvedHolds: Content // ... while it held this content
  }
  type Backup = {
    plan: int,
    last: str,
    hasReceipt: bool,
    receiptCells: int -> int,
    receiptProbe: int -> int
  }
  type Run = {
    mode: str,             // idle | fwd | inv | probe | receipt
    owner: bool,           // the owner's inverse
    fromFwd: bool,         // an inverse that a live forward run started
    plan: int,
    idx: int,
    step: str,
    k: int,                // the kept copy of the current part
    snap: Content,         // what the run read for the current part
    changed: Set[int],     // the parts this run exchanged
    probe: int -> int,     // the cells when the probes started
    check: int -> int      // the cells that the receipt check read
  }
  type Write = { id: int, part: int, kind: str, value: int }
  type Reset = { part: int, value: int, by: int, auto: bool }
  type World = {
    live: int -> Content,
    kept: List[Kept],
    backups: List[Backup],
    plans: List[int -> int],
    prepared: int,
    rehearsed: Set[int],
    proc: Run,
    // ghost
    writes: Set[Write],
    late: Set[int],        // the writes that landed in a kept copy
    superseded: Set[int],
    nextId: int,
    outside: int,
    crashes: int,
    gOpen: bool,           // a run began and has not finished its direction
    unrehearsed: bool,
    resets: Set[Reset],
    sawInterrupted: bool,
    ownerInvDone: bool,
    probeFailed: bool
  }

  var st: World

  pure val NO_CONTENT: Content = { cell: 0, by: 0, rest: Set() }
  pure val NO_KEPT: Kept = {
    part: 0, holds: NO_CONTENT, expect: NO_CONTENT, hasExpect: false,
    prepared: NO_CONTENT, exchanged: false, resolved: false,
    resolvedHolds: NO_CONTENT
  }
  pure val NO_BACKUP: Backup = {
    plan: 0, last: "none", hasReceipt: false, receiptCells: Map(),
    receiptProbe: Map()
  }
  pure val IDLE: Run = {
    mode: "idle", owner: false, fromFwd: false, plan: 0, idx: 0, step: "",
    k: -1, snap: NO_CONTENT, changed: Set(), probe: Map(), check: Map()
  }

  pure def same(a: Content, b: Content): bool =
    a.cell == b.cell and a.rest == b.rest

  // A check reports a copy that holds neither its recorded nor its prepared
  // content, unless the owner merged it and it has not changed since.
  pure def flagged(c: Kept): bool = and {
    c.hasExpect,
    not(same(c.holds, c.expect)),
    not(same(c.holds, c.prepared)),
    not(c.resolved and same(c.holds, c.resolvedHolds))
  }

  // -------------------------------------------------------------------------
  // Reading the world
  // -------------------------------------------------------------------------
  def keptAt(j: int): Kept =
    if (j >= 0 and j < st.kept.length()) st.kept[j] else NO_KEPT
  val hasBackup: bool = st.backups.length() > 0
  val latestB: Backup =
    if (hasBackup) st.backups[st.backups.length() - 1] else NO_BACKUP
  def earlierOf(pl: int, p: int): int = st.plans[pl].get(p)
  def cellOf(p: int): int = st.live.get(p).cell
  def isSw(p: int): bool = cellOf(p) == SW
  def isEa(pl: int, p: int): bool = cellOf(p) == earlierOf(pl, p)
  val allSw: bool = PARTS.forall(p => isSw(p))
  val cells: int -> int = PARTS.mapBy(p => cellOf(p))
  val anyFlagged: bool = st.kept.indices().exists(j => flagged(st.kept[j]))
  // Parts 0 .. k-1 switched and the rest earlier, for some k.
  def prefix(pl: int): bool =
    0.to(N).exists(k => PARTS.forall(p => if (p < k) isSw(p) else isEa(pl, p)))
  def third(pl: int): bool =
    PARTS.exists(p => not(isSw(p)) and not(isEa(pl, p)))

  // A backup's run is open until `invEnded`.  The mutant without a run record
  // keeps the earlier rule: the latest backup always governs.
  val open: bool =
    hasBackup and (not(RUN_RECORD) or latestB.last != "invEnded")

  // The run's decision, as step 3 makes it.
  val state: str =
    if (open) {
      val pl = latestB.plan
      if (RUN_RECORD) {
        if (latestB.last == "receipt") { if (allSw) "complete" else "drifted" }
        else if (third(pl) or not(prefix(pl))) "drifted"
        else if (latestB.last == "begun" and allSw) "active"
        else "interrupted"
      } else {
        if (third(pl)) "drifted"
        else if (allSw) { if (latestB.hasReceipt) "complete" else "active" }
        else if (PARTS.forall(p => isEa(pl, p))) "native"
        else "interrupted"
      }
    } else if (PARTS.forall(p => isEa(st.prepared, p))) "native"
    else "changed"

  // The state table of the specification, row by row, written independently
  // of `state`.
  val tComplete: bool = open and latestB.last == "receipt" and allSw
  val tActive: bool = open and latestB.last == "begun" and allSw
  val tInterrupted: bool = and {
    open,
    latestB.last == "begun" or latestB.last == "invBegun",
    prefix(latestB.plan),
    not(latestB.last == "begun" and allSw)
  }
  val tDrifted: bool = open and or {
    third(latestB.plan),
    not(prefix(latestB.plan)),
    latestB.last == "receipt" and not(allSw)
  }
  val tNative: bool = not(open) and PARTS.forall(p => isEa(st.prepared, p))
  val tChanged: bool = not(open) and PARTS.exists(p => not(isEa(st.prepared, p)))

  // -------------------------------------------------------------------------
  // World transformers
  // -------------------------------------------------------------------------
  pure def setLast(w: World, last: str): World = {
    val i = w.backups.length() - 1
    w.with("backups", w.backups.replaceAt(i, w.backups[i].with("last", last)))
  }

  // A live forward run stops and inverts its own parts.
  pure def toInverse(w: World): World =
    setLast(w, if (EARLY_END) "invEnded" else "invBegun")
      .with("proc", w.proc.with("mode", "inv").with("fromFwd", true)
                        .with("idx", N - 1).with("step", "read").with("k", -1))

  pure def nextPart(w: World): World =
    if (w.proc.mode == "fwd") {
      if (w.proc.idx + 1 < N) w.with("proc", w.proc.with("idx", w.proc.idx + 1).with("step", "read"))
      else w.with("proc", w.proc.with("step", "verify"))
    } else {
      if (w.proc.idx > 0) w.with("proc", w.proc.with("idx", w.proc.idx - 1).with("step", "read"))
      else w.with("proc", w.proc.with("step", "verify"))
    }

  pure def stage(w: World, part: int, cur: Content, prep: Content): World =
    w.with("kept", w.kept.append({
        part: part, holds: prep, expect: cur, hasExpect: RECORD_EXPECT,
        prepared: prep, exchanged: false, resolved: false,
        resolvedHolds: NO_CONTENT }))
     .with("proc", w.proc.with("step", "exchange").with("k", w.kept.length())
                       .with("snap", cur))

  // -------------------------------------------------------------------------
  // The run
  // -------------------------------------------------------------------------

  // Step 8: a new backup, only from a native layout, with a rehearsed plan
  // and no copy awaiting a merge.
  action beginForward = all {
    st.proc.mode == "idle",
    state == "native",
    not(REHEARSAL_CHECK) or st.rehearsed.contains(st.prepared),
    not(anyFlagged),
    st' = st
      .with("backups", st.backups.append(NO_BACKUP.with("plan", st.prepared)
                                                  .with("last", "begun")))
      .with("proc", IDLE.with("mode", "fwd").with("plan", st.prepared)
                       .with("step", "read"))
      .with("gOpen", true)
  }

  // Step 3 reads an interruption: resume the inverse.
  action resumeInverse = all {
    st.proc.mode == "idle",
    state == "interrupted",
    st' = setLast(st, if (EARLY_END) "invEnded" else "invBegun")
      .with("proc", IDLE.with("mode", "inv").with("plan", latestB.plan)
                       .with("idx", N - 1).with("step", "read"))
      .with("sawInterrupted", true)
  }

  // Step 3 reads an active switch: run the probes and the receipt.
  action resumeActive = all {
    st.proc.mode == "idle",
    state == "active",
    st' = st.with("proc", IDLE.with("mode", "probe").with("plan", latestB.plan)
                             .with("step", "start"))
  }

  action fwdRead = all {
    st.proc.mode == "fwd",
    st.proc.step == "read",
    val i = st.proc.idx
    val cur = st.live.get(i)
    st' = if (cur.cell == earlierOf(st.proc.plan, i))
            stage(st, i, cur, { cell: SW, by: 0, rest: cur.rest })
          else toInverse(st)
  }

  action invRead = all {
    st.proc.mode == "inv",
    st.proc.step == "read",
    val i = st.proc.idx
    val cur = st.live.get(i)
    val e = earlierOf(st.proc.plan, i)
    val mine = if (st.proc.fromFwd) st.proc.changed.contains(i) else true
    val reset = mine and (cur.cell == SW or (not(INVERSE_SKIPS_THIRD) and cur.cell != e))
    st' = if (reset) stage(st, i, cur, { cell: e, by: 0, rest: cur.rest })
          else nextPart(st)
  }

  // One atomic exchange: prepared content to the path, displaced content to
  // the stage name.
  action exchange = all {
    st.proc.mode == "fwd" or st.proc.mode == "inv",
    st.proc.step == "exchange",
    val i = st.proc.idx
    val c = keptAt(st.proc.k)
    val cur = st.live.get(i)
    val replaced = cur.by != 0 and cur.cell != c.holds.cell
    st' = st
      .with("live", st.live.set(i, c.holds))
      .with("kept", st.kept.replaceAt(st.proc.k,
                      c.with("holds", cur).with("exchanged", true)))
      .with("proc", st.proc.with("step", "compare")
                         .with("changed", st.proc.changed.union(Set(i))))
      .with("unrehearsed", st.unrehearsed or not(st.rehearsed.contains(st.proc.plan)))
      .with("resets", if (replaced)
              st.resets.union(Set({ part: i, value: cur.cell, by: cur.by,
                                    auto: not(st.proc.owner) }))
            else st.resets)
  }

  // Compare the displaced content with what the run read.  A forward
  // conflict inverts the run's own parts; an inverse conflict stops.
  action compare = all {
    st.proc.mode == "fwd" or st.proc.mode == "inv",
    st.proc.step == "compare",
    val ok = same(keptAt(st.proc.k).holds, st.proc.snap)
    st' = if (ok) nextPart(st)
          else if (st.proc.mode == "fwd") toInverse(st)
          else st.with("proc", IDLE)
  }

  // After the last part: every cell switched and no copy flagged.
  action fwdVerify = all {
    st.proc.mode == "fwd",
    st.proc.step == "verify",
    st' = if (allSw and not(anyFlagged))
            st.with("proc", st.proc.with("mode", "probe").with("step", "start"))
          else toInverse(st)
  }

  // The inverse ends only when no cell is switched.
  action invVerify = all {
    st.proc.mode == "inv",
    st.proc.step == "verify",
    st' = if (PARTS.forall(p => not(isSw(p))))
            setLast(st, "invEnded")
              .with("proc", IDLE).with("gOpen", false)
              .with("ownerInvDone", st.ownerInvDone or st.proc.owner)
          else st.with("proc", IDLE)
  }

  action probeStart = all {
    st.proc.mode == "probe",
    st.proc.step == "start",
    st' = st.with("proc", st.proc.with("probe", cells).with("step", "end"))
  }

  action probeEnd(pass: bool): bool = all {
    st.proc.mode == "probe",
    st.proc.step == "end",
    pass or PROBES_CAN_FAIL,
    st' = if (pass) st.with("proc", st.proc.with("mode", "receipt").with("step", "check"))
          else toInverse(st).with("probeFailed", true)
  }

  // Step 13, first half: re-read the layout.  A changed cell inverts; a
  // flagged copy stops for the owner's merge.
  action receiptCheck = all {
    st.proc.mode == "receipt",
    st.proc.step == "check",
    val cellsOk = allSw and cells == st.proc.probe
    st' = if (not(RECEIPT_RECHECK))
            st.with("proc", st.proc.with("step", "write"))
          else if (cellsOk and not(anyFlagged))
            st.with("proc", st.proc.with("check", cells).with("step", "write"))
          else if (not(cellsOk)) toInverse(st)
          else st.with("proc", IDLE)
  }

  // Step 13, second half: append the receipt.  Outside writes can land
  // between the two halves.
  action receiptWrite = all {
    st.proc.mode == "receipt",
    st.proc.step == "write",
    val i = st.backups.length() - 1
    val b = st.backups[i]
    val recorded = if (RECEIPT_RECHECK) st.proc.check else cells
    st' = st
      .with("backups", st.backups.replaceAt(i, b.with("last", "receipt")
              .with("hasReceipt", true).with("receiptCells", recorded)
              .with("receiptProbe", st.proc.probe)))
      .with("proc", IDLE).with("gOpen", false)
  }

  // -------------------------------------------------------------------------
  // The owner
  // -------------------------------------------------------------------------

  // The owner's inverse, after finish or from a drifted state.
  action ownerInverse = all {
    st.proc.mode == "idle",
    open,
    Set("complete", "active", "drifted", "interrupted").contains(state),
    st' = setLast(st, if (EARLY_END) "invEnded" else "invBegun")
      .with("proc", IDLE.with("mode", "inv").with("owner", true)
                       .with("plan", latestB.plan).with("idx", N - 1)
                       .with("step", "read"))
      .with("gOpen", true)
  }

  // The owner merges a flagged copy into the live file and records it.
  action ownerMerge(j: int): bool = all {
    st.proc.mode == "idle",
    j < st.kept.length(),
    flagged(keptAt(j)),
    val c = keptAt(j)
    val cur = st.live.get(c.part)
    val takeCell = c.holds.by != 0 and c.holds.cell != SW
                   and c.holds.cell != earlierOf(st.prepared, c.part)
    val merged = if (takeCell)
                   { cell: c.holds.cell, by: c.holds.by, rest: cur.rest.union(c.holds.rest) }
                 else cur.with("rest", cur.rest.union(c.holds.rest))
    st' = st
      .with("live", st.live.set(c.part, merged))
      .with("kept", st.kept.replaceAt(j, c.with("resolved", true)
                                          .with("resolvedHolds", c.holds)))
      .with("superseded", if (takeCell and cur.by != 0 and cur.by != c.holds.by)
                            st.superseded.union(Set(cur.by))
                          else st.superseded)
  }

  // A classification makes a new plan from a layout with no switched cell.
  action classify = all {
    st.proc.mode == "idle",
    state == "changed",
    st.plans.length() < MAX_PLANS,
    PARTS.forall(p => not(isSw(p))),
    st' = st.with("plans", st.plans.append(cells))
            .with("prepared", st.plans.length())
  }

  action rehearse = all {
    st.proc.mode == "idle",
    not(st.rehearsed.contains(st.prepared)),
    st' = st.with("rehearsed", st.rehearsed.union(Set(st.prepared)))
  }

  // -------------------------------------------------------------------------
  // Outside programs and interruptions
  // -------------------------------------------------------------------------
  val outsideMayAct: bool = and {
    st.outside < MAX_OUTSIDE,
    or {
      OUTSIDE_PHASE == "any",
      OUTSIDE_PHASE == "run" and st.proc.mode != "idle",
      OUTSIDE_PHASE == "receipt" and latestB.hasReceipt
    }
  }

  action outsideCell(p: int, v: int): bool = all {
    OUTSIDE_CELLS,
    outsideMayAct,
    val id = st.nextId
    val value = if (v == FRESH) id else v
    val cur = st.live.get(p)
    all {
      value != cur.cell,
      st' = st
        .with("live", st.live.set(p, { cell: value, by: id, rest: cur.rest }))
        .with("writes", st.writes.union(Set({ id: id, part: p, kind: "cell", value: value })))
        .with("superseded", if (cur.by != 0) st.superseded.union(Set(cur.by)) else st.superseded)
        .with("nextId", id + 1).with("outside", st.outside + 1)
    }
  }

  action outsideRest(p: int): bool = all {
    OUTSIDE_REST,
    outsideMayAct,
    val id = st.nextId
    val cur = st.live.get(p)
    st' = st
      .with("live", st.live.set(p, cur.with("rest", cur.rest.union(Set(id)))))
      .with("writes", st.writes.union(Set({ id: id, part: p, kind: "rest", value: 0 })))
      .with("nextId", id + 1).with("outside", st.outside + 1)
  }

  // A descriptor opened before an exchange writes into the displaced inode.
  action lateWrite(j: int): bool = all {
    LATE_WRITES,
    outsideMayAct,
    j < st.kept.length(),
    keptAt(j).exchanged,
    val id = st.nextId
    val c = keptAt(j)
    st' = st
      .with("kept", st.kept.replaceAt(j, c.with("holds",
              c.holds.with("rest", c.holds.rest.union(Set(id))))))
      .with("writes", st.writes.union(Set({ id: id, part: c.part, kind: "rest", value: 0 })))
      .with("late", st.late.union(Set(id)))
      .with("nextId", id + 1).with("outside", st.outside + 1)
  }

  action crash = all {
    st.proc.mode != "idle",
    st.crashes < MAX_CRASH,
    st' = st.with("proc", IDLE).with("crashes", st.crashes + 1)
  }

  action stutter = st' = st

  action init = st' = {
    live: PARTS.mapBy(p => NO_CONTENT),
    kept: List(),
    backups: List(),
    plans: List(PARTS.mapBy(p => 0)),
    prepared: 0,
    rehearsed: Set(0),
    proc: IDLE,
    writes: Set(),
    late: Set(),
    superseded: Set(),
    nextId: 2,
    outside: 0,
    crashes: 0,
    gOpen: false,
    unrehearsed: false,
    resets: Set(),
    sawInterrupted: false,
    ownerInvDone: false,
    probeFailed: false
  }

  action step = any {
    beginForward, resumeInverse, resumeActive,
    fwdRead, invRead, exchange, compare, fwdVerify, invVerify,
    probeStart, nondet pass = Set(true, false).oneOf() probeEnd(pass),
    receiptCheck, receiptWrite,
    ownerInverse, classify, rehearse,
    nondet j = 0.to(MAX_KEPT).oneOf() ownerMerge(j),
    nondet p = PARTS.oneOf() nondet v = Set(0, SW, FRESH).oneOf() outsideCell(p, v),
    nondet p = PARTS.oneOf() outsideRest(p),
    nondet j = 0.to(MAX_KEPT).oneOf() lateWrite(j),
    crash,
    stutter
  }

  // -------------------------------------------------------------------------
  // P1 — no write is lost without a report
  // -------------------------------------------------------------------------
  def inFlaggedCopy(w: Write): bool =
    st.kept.indices().exists(j => and {
      st.kept[j].part == w.part,
      flagged(st.kept[j]),
      if (w.kind == "rest") st.kept[j].holds.rest.contains(w.id)
      else st.kept[j].holds.by == w.id
    })
  // The accepted case: a cell write of a plan's earlier value or of the
  // switched value.  The run's change list reports each replaced value.
  def absorbed(w: Write): bool =
    w.kind == "cell" and
      (w.value == SW or st.plans.indices().exists(pl => st.plans[pl].get(w.part) == w.value))
  val inv_noSilentLoss: bool = st.writes.forall(w => or {
    w.kind == "rest" and st.live.get(w.part).rest.contains(w.id),
    w.kind == "cell" and st.live.get(w.part).by == w.id,
    w.kind == "cell" and st.superseded.contains(w.id),
    absorbed(w),
    inFlaggedCopy(w)
  })

  // -------------------------------------------------------------------------
  // P2 — exactly one state, read from durable facts that match history
  // -------------------------------------------------------------------------
  val inv_exactlyOneState: bool =
    [tComplete, tActive, tInterrupted, tDrifted, tNative, tChanged]
      .foldl(0, (n, b) => if (b) n + 1 else n) == 1
  val inv_stateMatchesTable: bool = and {
    (state == "complete") == tComplete,
    (state == "active") == tActive,
    (state == "interrupted") == tInterrupted,
    (state == "drifted") == tDrifted,
    (state == "native") == tNative,
    (state == "changed") == tChanged
  }
  val inv_interruptedOnlyWhenOpen: bool = state == "interrupted" implies st.gOpen
  val inv_settledOnlyWhenClosed: bool =
    (state == "native" or state == "changed") implies not(st.gOpen)

  // -------------------------------------------------------------------------
  // P3 — no receipt certifies a layout that is not switched
  // -------------------------------------------------------------------------
  val inv_receiptRecordsSwitched: bool = st.backups.indices().forall(i =>
    st.backups[i].hasReceipt implies
      PARTS.forall(p => st.backups[i].receiptCells.get(p) == SW))
  val inv_receiptMatchesProbes: bool = st.backups.indices().forall(i =>
    st.backups[i].hasReceipt implies
      st.backups[i].receiptCells == st.backups[i].receiptProbe)
  val inv_completeOnlySwitched: bool = state == "complete" implies allSw

  // -------------------------------------------------------------------------
  // P4 — only a rehearsed plan reaches the real HOME
  // -------------------------------------------------------------------------
  val inv_onlyRehearsedApplied: bool = not(st.unrehearsed)

  // -------------------------------------------------------------------------
  // Witnesses — each case that the specification says is live
  // -------------------------------------------------------------------------
  val wit_complete: bool = state == "complete"
  val wit_completeAfterCrash: bool = state == "complete" and st.crashes > 0
  val wit_resumedToNative: bool = st.sawInterrupted and state == "native"
  val wit_probeFailedToNative: bool = st.probeFailed and state == "native"
  val wit_ownerInverseToNative: bool = st.ownerInvDone and state == "native"
  // An exchange displaced a write that the live file then lacked.
  val wit_conflictFlagged: bool = st.kept.indices().exists(j => and {
    flagged(st.kept[j]),
    st.kept[j].holds.rest.exists(id =>
      not(st.late.contains(id)) and not(st.live.get(st.kept[j].part).rest.contains(id)))
  })
  // A descriptor wrote into a kept copy, and a check can report it.
  val wit_lateWriteFlagged: bool = st.kept.indices().exists(j =>
    flagged(st.kept[j]) and st.kept[j].holds.rest.exists(id => st.late.contains(id)))
  // A conflict at a cell: an outside value is kept and flagged.
  val wit_cellConflictFlagged: bool = st.kept.indices().exists(j => and {
    flagged(st.kept[j]),
    st.kept[j].holds.by != 0,
    st.kept[j].holds.cell != SW
  })
  val wit_mergedAndComplete: bool =
    st.kept.indices().exists(j => st.kept[j].resolved) and state == "complete"
  val wit_driftedAfterReceipt: bool = state == "drifted" and latestB.last == "receipt"
  val wit_driftedDuringRun: bool = state == "drifted" and latestB.last != "receipt"
  val wit_coincidenceReset: bool =
    st.resets.exists(r => r.auto and r.value == SW)
  val wit_changedNeedsPlan: bool = state == "changed"
  val wit_secondPlanComplete: bool = state == "complete" and latestB.plan > 0
  val wit_activeAfterCrash: bool = state == "active" and st.crashes > 0

  // -------------------------------------------------------------------------
  // Mutant witnesses — each reaches the violation that its mechanism
  // prevents.  The violated conjuncts are written out inline.
  // -------------------------------------------------------------------------

  // Without a recorded expectation, a write sits in a copy that no check can
  // report, and the live file lacks it.
  val wit_m_writeLostSilently: bool = st.writes.exists(w => and {
    w.kind == "rest",
    not(st.live.get(w.part).rest.contains(w.id)),
    st.kept.indices().forall(j => not(and {
      st.kept[j].hasExpect,
      st.kept[j].holds.rest.contains(w.id),
      not(same(st.kept[j].holds, st.kept[j].expect))
    }))
  })

  // Without a run record, an outside write after a finished inverse reads as
  // an interruption, and the run resumes an inverse that nobody began.
  val wit_m_interruptionWithoutRun: bool =
    state == "interrupted" and not(st.gOpen)

  // With `invEnded` written early, an interrupted inverse reads as settled.
  val wit_m_settledWhileOpen: bool =
    (state == "native" or state == "changed") and st.gOpen

  // Without the re-read, a receipt records a cell that is not switched.
  val wit_m_receiptOnUnswitched: bool = st.backups.indices().exists(i =>
    st.backups[i].hasReceipt and
      PARTS.exists(p => st.backups[i].receiptCells.get(p) != SW))

  // Without the rehearsal check, an unrehearsed plan reaches the real HOME.
  val wit_m_unrehearsedApplied: bool = st.unrehearsed

  // An inverse that resets a third value destroys an outside write: the copy
  // holds what the inverse read, so nothing reports it.
  val wit_m_thirdValueLost: bool = st.writes.exists(w => and {
    w.kind == "cell",
    w.value != SW,
    st.plans.indices().forall(pl => st.plans[pl].get(w.part) != w.value),
    not(st.superseded.contains(w.id)),
    st.live.get(w.part).by != w.id,
    st.kept.indices().forall(j => not(and {
      st.kept[j].holds.by == w.id,
      not(same(st.kept[j].holds, st.kept[j].expect))
    }))
  })
}

// ===========================================================================
// Instances.  Each fixes one world.  The corrected design sets the six
// mechanisms to true, true, false, true, true, true.
// ===========================================================================

/// Interruptions only, at any step, with failing probes and the owner's
/// inverse.
module crashes {
  import switch(
    N = 3, MAX_OUTSIDE = 0, MAX_CRASH = 2, MAX_PLANS = 1,
    OUTSIDE_CELLS = false, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = true, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

/// Outside writes to cells, to the rest of files and into kept copies, with
/// no interruption.
module writers {
  import switch(
    N = 3, MAX_OUTSIDE = 2, MAX_CRASH = 0, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = true, LATE_WRITES = true,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

/// Outside writes and interruptions together, with classification into a
/// second plan.
module both {
  import switch(
    N = 3, MAX_OUTSIDE = 2, MAX_CRASH = 1, MAX_PLANS = 2,
    OUTSIDE_CELLS = true, OUTSIDE_REST = true, LATE_WRITES = true,
    PROBES_CAN_FAIL = true, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

/// Outside cell writes only, with two plans: the classification route.
module classify_world {
  import switch(
    N = 3, MAX_OUTSIDE = 1, MAX_CRASH = 0, MAX_PLANS = 2,
    OUTSIDE_CELLS = true, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

/// Outside cell writes and one interruption: the coincidence case.
module coincidence {
  import switch(
    N = 2, MAX_OUTSIDE = 1, MAX_CRASH = 1, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

// --- Mutants: one mechanism removed each ----------------------------------

module m_no_expect {
  import switch(
    N = 2, MAX_OUTSIDE = 1, MAX_CRASH = 1, MAX_PLANS = 1,
    OUTSIDE_CELLS = false, OUTSIDE_REST = true, LATE_WRITES = true,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = false, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

module m_no_run_record {
  import switch(
    N = 2, MAX_OUTSIDE = 1, MAX_CRASH = 0, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = true, OUTSIDE_PHASE = "receipt",
    RECORD_EXPECT = true, RUN_RECORD = false, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

module m_early_end {
  import switch(
    N = 2, MAX_OUTSIDE = 0, MAX_CRASH = 1, MAX_PLANS = 1,
    OUTSIDE_CELLS = false, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = true, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = true,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

module m_receipt_no_recheck {
  import switch(
    N = 2, MAX_OUTSIDE = 1, MAX_CRASH = 0, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "run",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = false, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

module m_no_rehearsal_check {
  import switch(
    N = 2, MAX_OUTSIDE = 1, MAX_CRASH = 0, MAX_PLANS = 2,
    OUTSIDE_CELLS = true, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = false, INVERSE_SKIPS_THIRD = true
  ).*
}

module m_inverse_resets_third {
  import switch(
    N = 2, MAX_OUTSIDE = 1, MAX_CRASH = 0, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = false, LATE_WRITES = false,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "any",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = false
  ).*
}

// --- Focused worlds: witness hunting only ---------------------------------

/// Outside writes only after a receipt: drift after a complete switch, and
/// the owner's inverse from it.
module after_receipt {
  import switch(
    N = 3, MAX_OUTSIDE = 2, MAX_CRASH = 1, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = true, LATE_WRITES = true,
    PROBES_CAN_FAIL = false, OUTSIDE_PHASE = "receipt",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}

/// Outside writes only while a run is live: conflicts at exchanges.
module during_run {
  import switch(
    N = 3, MAX_OUTSIDE = 2, MAX_CRASH = 1, MAX_PLANS = 1,
    OUTSIDE_CELLS = true, OUTSIDE_REST = true, LATE_WRITES = true,
    PROBES_CAN_FAIL = true, OUTSIDE_PHASE = "run",
    RECORD_EXPECT = true, RUN_RECORD = true, EARLY_END = false,
    RECEIPT_RECHECK = true, REHEARSAL_CHECK = true, INVERSE_SKIPS_THIRD = true
  ).*
}
