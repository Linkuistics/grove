# admission-k51


## Goal

Start `models/system/lifecycle.als` with the loop's guard stack: `SY-01`,
`SY-02`, `SY-03` and `SY-11` (six obligations), plus the two premise-break
mutations that control them — `EN-07` and `EN-14`.


## Context

- `docs/specs/semantic-contract.md` §*Claims — system lifecycle* `SY-01` –
  `SY-03` and `SY-11` is this leaf's claim scope; §*Actions* fixes the
  seven-member Lifecycle group (`acquire-lease`, `layout-preflight`,
  `open-epoch`, `launch`, `reap`, `close-epoch`, `release-lease`) and its guard
  as *lease, then epoch*; §*Outcomes* fixes `LeaseHeld`, `LayoutUnsupported` and
  `EpochStale` as the refusal reasons this slice's guards name.
- `CONTEXT.md` §*Formal contract* and the *Driver lease*, *Workspace layout
  preflight*, *Session epoch* and *Tree access lock* entries. Each carries an
  `_Avoid_` line that is a claim in this scope — a contended lease is refused
  and never queued (`SY-01.a`), the preflight is not a licence the finish
  transaction may consult (`SY-03`), and the tree access lock is a *different*
  guard that must be released before foreground launch (`SY-11.b`).
- `models/run.sh` is finished and needs no work here. The measuring invocation
  is `models/run.sh --scope lifecycle --family alloy --no-coverage`.
- `crates/grove-finish/models/finish.als` is the house style for a temporal
  model at this size. Its machinery is **not** importable — the runner treats
  each scope directory as its own family file set, and an `FN_` command here is
  a placement failure — so what carries over is the style, not the signatures.

Every obligation in this slice is about **admission to the loop**: who may hold
it, on what layout, and in what order the three guards are taken. So this slice
needs the guard stack and none of the loop's body — no iteration boundary, no
configuration, no selection, no session, no task-root classification beyond
presence, no finish, no crash-point sweep. That is what makes it the first child
rather than a horizontal layer.


## Done when

- `models/system/lifecycle.als` exists and answers each of `SY-01.a`, `SY-01.b`,
  `SY-02`, `SY-03`, `SY-11.a` and `SY-11.b` with a `check` and its required
  `witness_` runs, all green.
- **`SY-02`'s witness is a refusal that leaves an empty working tree
  untouched** — the refusal must land at lease acquisition, *before* any
  observation, creation or mutation of the tree, and the witness must show a
  trace in which no task root ever exists. A model that refuses after a root is
  present has witnessed a weaker claim than the catalogue's.
- **`SY-03`'s witness is a layout that changes between the two gates.** The
  model therefore needs two distinct check points reading the layout
  independently, and a `topology-change` between them. A single recorded verdict
  consulted twice makes the claim true by construction and is the shape of a
  false-confidence incident rather than a finding.
- `SY-11.b` is checked as the **exhaustive absence of a cycle within a stated
  bound**, with the bound recorded — not as an ordering property, which is
  `SY-11.a`'s. The two are separate obligations and a single command answering
  both leaves one cell filled by nothing.
- `EN-07` runs as `check expect_fail_EN_07_SY_11b_<mnemonic>` with the
  assumption table's expected result: under a **shared-lock** scope — two open
  descriptions of one directory sharing a lock — `SY-11.b` **fails**, and the
  counterexample is the cycle `bulk-marks-are-not-atomic` records. It is a
  *premise-break*, so a green check is a **survivor** and a defect in the
  mutation, not a pass.
- `EN-14` runs as `check expect_fail_EN_14_SY_01_<mnemonic>` with its stated
  expected result: in a scope where the working-tree root does not outlive the
  task root, `SY-01` fails — ownership has nothing to be held on, so a second
  driver is admitted. The row also names `SY-05`; that half is `roots`' and is
  recorded as owed rather than answered here.
- `models/system/README.md` records tool version, solver, bounds per command,
  the abstractions and deliberate omissions this file adopts, what a green run
  does not prove, the obligations claimed so far, the witness bound at which
  each first lands, the composition boundary (which `TT-`/`FN-` facts are
  imported as opaque observations and which sibling model owns each), and any
  declared gap in the shape the runner parses.
- One mutation per reported obligation, run before the green is believed, each
  with **evidence that it actually fires** — an unsatisfiable mutation reports
  exactly as a survivor does, and three sessions across the two sibling scopes
  have now produced one.
- `models/run.sh --scope lifecycle --family alloy --no-coverage` exits 0 and
  reports exactly the expected empty cells (`SY-04` – `SY-10`, `SY-12` – `SY-14`).
- Experiment 2 entry 040 is appended with the six required fields plus the
  pre-registration's four additions.
- The next leaf (`iteration`, `SY-04`, `SY-08`, `SY-10`) is cut as a sibling,
  its body carrying the machinery question this session's file actually leaves
  open.


## Notes

**Three decisions this leaf owns for the whole subtree, and all three are the
first file's to make.**

1. **How coarse the composition boundary is.** The node brief fixes the rule —
   compose at observations, never at machinery — but this file is where it
   becomes a signature. No `SY-` obligation in this slice reads a filename, a
   position, a key, a digest, a manifest or a quarantine, so the candidate is a
   task root that is *present or absent* and nothing more, with the working-tree
   root as the thing ownership is held on. Record the choice and what it costs.
2. **Whether the guard stack is three relations or one ordered relation.**
   `SY-11.a` is *every path takes lease, then epoch, then tree, in that order*
   and `SY-11.b` is *no path waits for a generation while holding a tree guard*.
   A single ordered relation makes `SY-11.a` true by construction and leaves
   `SY-11.b` with nothing to refute; three independently acquirable guards make
   both checkable and make `EN-07`'s shared-lock mutation expressible at all.
   Decide it here, because a retrofit re-runs every command before it.
3. **Whether a guard *wait* is observable.** §*Outcomes* says a guard wait is
   not an outcome and Grove's tree lock blocks, so no invocation returns while
   it is held — but `SY-11.b` is a claim about *waiting*, and a model in which a
   failed guard is an absent transition makes it true by construction. The
   sibling task-tree model met this and introduced `Deferred` as an abstraction
   of its own, recording it as one; `SY-11.b` needs the same and needs it named
   in the README as an abstraction rather than as a contract.

**`SY-01.b` is the one obligation whose subject is not Grove's own code.**
Ownership is released *by process death as ordinarily as by return* — the
kernel's release of an advisory lock, not a cleanup path. Its witness is a
crashed driver whose successor proceeds, so this slice needs `crash` even though
the crash-point sweep is `sessions`'. Model release as a consequence of the
holder ceasing to exist and never as a step the holder takes, or the claim
becomes a statement about a cleanup handler Grove does not have.

**`SY-02` and `SY-03` are easy to collapse into each other.** `SY-02` is about
*when* the layout is proved (at lease acquisition, before any tree exists) and
`SY-03` is about *how many times* (every gate revalidates, none consults an
earlier verdict). A model with one layout reading satisfies both and has checked
neither. The finish model already carries the second gate's side of this —
`witness_FN_05a_p3_layout_unsupported` is `SY-03` stated as a trace — so the
composition question is whether this file re-states the second gate or observes
the finish model's; the README must say which, and `formal-synthesis-k16` reads
that answer.


## Decisions (running log)

**The composition boundary is a task root that is present or absent, and a
working-tree root that is only the thing ownership is held on.** No filename,
position, key, digest, manifest, quarantine or lane appears in
`models/system/lifecycle.als`. What it costs is stated where it bites:
`SY-02`'s fourth conjunct cannot be stated for a session, because a session
reaches the tree through a generation this slice does not model, and the session
half is `SY-10`'s. Recorded in `models/system/README.md` §*The composition
boundary* and §*What is owed elsewhere*.

**The guard stack is three independently acquirable guards, not one ordered
relation.** One relation would make `SY-11.a` true by construction and leave
`SY-11.b` nothing to refute, and `EN-07`'s shared-lock mutation would not be
expressible at all. The order lives in `below` as data and in `ordered` as the
discipline every acquisition site applies.

**A guard wait IS observable, as this file's own abstraction.** §*Outcomes* is
explicit that a wait is not an outcome, but `SY-11.b` is a claim about waiting
and a model in which a failed guard is an absent transition makes it true by
construction. `Proc.waits` and the `Deferred` result are declared as
abstractions in the family README, exactly as the task-tree model declared its
own `Deferred`.

**Two process roles, and no more.** A driver and an ambient session. With one
role only the lease-holder ever reaches the epoch, nothing ever contends, and
`SY-11.b` would be checked over an empty wait-for graph. Nothing here reads a
generation *value* — that is `SY-10`'s.

**`SY-01.b`'s release half is recorded as unkillable rather than declared a
gap.** Two mutations against it are made unsatisfiable by the model's own facts,
because the content is a platform property (the kernel releases an advisory lock
when the holder dies) and not a protocol one. The obligation is checked and
witnessed for the half that is not construction. The durable consequence — the
assumption table has no row for kernel lock release, though `SY-01.b` rests
entirely on it — is `formal-synthesis-k16`'s to settle and is written up in
Experiment 2 entry 040 rather than acted on here.

**`SY-11.a` and `SY-11.b` share their only firing mutation, and that is recorded
as the neighbour list rather than retried.** This is the finish scope's sixth
mutation failure mode met for the first time in this scope.
