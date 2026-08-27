# catalogue-disposition-k64 — brief


## Goal

Dispose every catalogue finding the formal phase recorded rather than fixed, and
land each disposition so that both model families still answer the manifest the
catalogue defines.

## Context

**`docs/specs/semantic-contract.md` is the runner's manifest, not a document
about the models.** `models/run.sh` reads the obligation list *out of* the
catalogue (its header, obligation 3, and the catalogue's own *Model paths and
the runner*). So a disposition that adds, removes or re-scopes an obligation
opens an empty `(family, obligation)` cell that **both** families must fill
before any coverage-asserting run is green. Classify every disposition as
**manifest-neutral** (prose, wording, a citation) or **manifest-changing**
before deciding it, because the second carries a model-work cascade and the
first does not.

**Enumerate the inherited set; do not work from the list below.** The list below
is classes and sharp items, written by `experiment-synthesis-k62` to size the
work — it is not the enumeration, and a list written in one session goes stale
against the tree. The instrument, with the controls that make its output
evidence rather than a clean grep:

```sh
grep -rn "formal-synthesis-k16" . \
  --exclude-dir=.jj --exclude-dir=target --exclude-dir=.grove \
  --exclude-dir=_apalache-out --exclude-dir=.review-tmp
```

97 sites across 11 files at the revision this leaf was cut. **Controls:** the
same command for a live sibling handle (`cross-model-replay-k15`) must find its
own sites — it found 14 — and for an invented handle (`formal-synthesis-k99`)
must find none. Clean-here alone proves nothing; clean-here plus dirty-there
cannot be produced by a broken instrument. Then classify every site: a
disposition, a pointer to one, or prose that merely names the leaf.

**The classes, and the sharp items in each.**

- **Closed-set additions — every one manifest-changing.** `RRolledBack`
  (a clean rollback has no member), `RConfigInvalid` (no refusal reason names an
  invalid configuration), `RGenContended` (the contention timeout's visible
  stop), `ONotEntered`, and `Stopped`. Each was *declared as an addition* by a
  model rather than smuggled into the catalogue's set, which is why they are
  here.
- **Claims false as literally worded.** `FN-25.b`; `SY-13` over the
  `Legacy`/`Foreign`/`Malformed` sinks; `TT-17`, contradicted by the
  catalogue's own `PartialScaffold`; `TT-15.a`, contradicted by `TT-24.b` and
  `TT-18` under one tree; `TT-20`'s prohibition on `Legacy`, which entry 048
  showed is **false of shipped behaviour** in the window after the charter and
  before the leaf.
- **Underdetermined claims the two columns resolved in OPPOSITE directions.**
  `FN-13`'s refuse-or-block, and whether `reap` is gated on the root's
  classification. **These are the two sharpest items on the list**, because an
  opposite resolution by two independent readers is stronger evidence that the
  text is underdetermined than either reader's own account. `reap` additionally
  has the product answer: `src/loop_driver.rs` reads no root classification on
  the reap path, so the catalogue gap is real and no product defect stands
  behind it.
- **Scoping.** Whether `SY-04.b` owes `SY-03`'s *a preflight is never a licence*
  prohibition for the **configuration** as well as for the layout — Quint's
  retained counterexample has `outcomeOn` gating on the driver's recorded
  verdict while the layout gate three lines below reads live. Whether
  `SY-04.b`/`SY-14.b` over-apply to `release-lease`, which touches no tree.
  `EN-11`'s controls column mis-attributing `TT-24.b`, whose dependency is
  `EN-13` — a row that has now been wrong twice, which is worth more than the
  one-word fix.
- **State-table completeness.** The shipped *ambiguous partial root scaffold*
  refusal is a fail-closed outcome the state table has no member for, and entry
  048 judges it **a better answer than either model gives**. Also the mid-flight
  block: an ordinary mutation that has already applied an effect and whose next
  create is no longer licensed is none of the three contexts *Outcomes* fixes.
- **Prose, manifest-neutral.** The catalogue's *`models/run.sh` is the one
  repository runner* section states **three** obligations where its own Q4
  paragraph makes **four**; the runner numbers four.
- **Model findings, not catalogue findings — route, do not dispose.** The
  `SY-10.b` / `SY-11.b` collision over `WGen`, where one obligation's apparatus
  constructs the state another forbids and no predicate over `waits` separates
  the two senses. `TT-24.c`'s uncontrolled Quint transcription belongs to
  `obligation-placement-k63`.

**What is already settled and must not be re-opened.** `TT-24.c`/`TT-24.d`
placement and the five other instances of that shape are
`obligation-placement-k63`'s, and this leaf applies the rule that leaf lands.
The ordinal root-lifecycle verdict is `finish-verdicts-k65`'s.

**The rule that leaf landed, and what it changed under this leaf's feet.** It is
[`obligations-follow-context-not-artifact`](../../../../docs/adr/obligations-follow-context-not-artifact.md):
an obligation belongs to the scope that can execute its context, ordered by the
approved crate dependency direction; a clause a scope cannot check stays in place
only as a declared **cross-scope citation**, carrying the cited obligation's
declared narrowings; and a gap declared by **both** families is the signal to
apply the rule rather than a place to rest. Four consequences for this leaf:

- **`TT-24.c` and `TT-24.d` are retired**, to the new `FN-32` and to `FN-21.c`.
  The catalogue carries **128** obligations, not 129. Nothing here reopens that.
- **A seventh opposite-resolution item joins the list above, and it is sharp.**
  `finish.als` **refuses** where `finish.qnt` **blocks** — an unclassifiable
  artifact at a reserved name met inside a transaction — and both are green
  against `FN-10.b`, whose text says only *fails closed*. `FN-32` states only
  what both agree on (the artifact is not mutated), so **which outcome the
  catalogue fixes is this leaf's**, beside `FN-13`'s refuse-or-block. Note that
  Quint's `wit_FN_32` is worded on the blocking half and moves with the answer.
- **`EN-13`'s controls row was edited, and only for the retirement.** `TT-24.d`
  is gone from it; the row now reads `TT-04`, `TT-24.b`, `FN-21.b`, `FN-27`.
  The **`TT-24.b` mis-attribution on that same row is still this leaf's** — it
  was not touched, and a row that has been wrong twice is worth more than the
  one-word fix.
- **The catalogue's *one artifact, three contexts, one decided outcome* table
  now has a `checked by` column and a second row that no longer fixes an
  outcome.** That row is where the refuse-or-block answer lands.

## Done when

- Every enumerated site is classified, and every disposition is **decide**,
  **route to a named sibling**, or **prose-only**. A site that turns out to name
  no decision is recorded as such rather than dropped.
- Each decided item lands in `docs/specs/semantic-contract.md`, and each is
  marked manifest-neutral or manifest-changing at the moment it is decided.
- For every manifest-changing disposition, **both** families answer the new or
  changed obligation with a property command plus its required witnesses, or
  with that family's own declared gap, and each affected scope's
  `models/run.sh --scope <scope> --family <family>` is green with coverage
  asserted. A run line is recorded for every scope touched.
- The model READMEs that recorded a finding *rather than fixing it* are updated
  in place to say how it was disposed, so a later reader meeting the
  counterexample finds the decision beside it.
- No catalogue finding is left saying `formal-synthesis-k16` owns it.

## Decomposition

Four children. The order encodes one gate and one dependency direction, and the
argument for both is in the running log below under *The split is by scope*.

1. **`closed-sets-k69`** — the shared vocabulary, and everything that belongs to
   no single scope. The five closed-set additions (items 1 – 5), the two
   refuse-or-block questions that cannot be answered without them (6 – 7), the
   `reap` gating question (8), the one prose fix (25), item 19's twice-wrong
   `EN-11` row, and every routed item (26 – 36). **First, because §*Vocabulary*
   sits above all three claim sections and `obligations-follow-context-not-artifact`
   clause 4 says a term partitioned across groups belonging to different scopes
   has no owning scope** — so three scope children each editing one closed list
   would collide, and each would decide in ignorance of the other two. The
   routed items ride here because 26 – 30 and 36 gate this node's own next
   siblings, `finish-verdicts-k65` and `handoff-audit-k66`.

   **It proved bigger than one session and is now a node of two**, split along
   the seam its own `Notes` predicted: `routing-and-prose-k73` took everything
   that costs no model work — items 8, 19 and 25 decided, and the whole routed
   set re-pointed — and `closed-set-additions-k74` carries items 1 – 7, every
   one manifest-changing. The halves touch disjoint regions of the catalogue
   (§*Actions*, the `EN-11` assumption row and the runner's obligation list,
   against §*Outcomes*' closed sets), so the second opens on a document the
   first left settled rather than half-moved.
2. **`task-tree-scope-k70`** — items 11, 12, 13, 20, 21, 22.
3. **`finish-scope-k71`** — items 9, 14, 23, 24, and 28 (re-routed here from
   the routed set while `closed-sets-k69` landed it).
4. **`lifecycle-scope-k72`** — items 10, 15, 16, 17, 18.

**Children 2 – 4 run in the crate dependency direction and that is forced, not
chosen.** The rule orders the scopes `grove-task-tree` → `grove-finish` → the
joint, and its clause 1 means a re-scope always moves an obligation **up**: an
obligation naming something from a scope above belongs to the highest scope its
text names. The one instance in the record moves that way — `TT-24.c`/`TT-24.d`
became `FN-32`/`FN-21.c`. So task-tree first hands any re-scoped item *forward*,
into a child that has not run; lifecycle first would need a hand backward into a
retired one.

**Each scope child owns its own green run and nothing else's.** The `Done when`
requires `models/run.sh --scope <scope> --family <family>` green with coverage
asserted for every scope a child touched, so a child that re-scopes an item
forward hands the receiving child the cell, not the run. `closed-sets-k69` is
the exception by construction: a closed set is swept in more than one scope, so
it owes runs in every scope its decisions reach, and it should expect that to be
all three.

**The whole-node obligation is the last child's.** *No catalogue finding is left
saying `formal-synthesis-k16` owns it* is only checkable once every child has
landed, so `lifecycle-scope-k72` re-runs the instrument with both controls as
its closing act. It is not a fifth child: a session whose whole content is a
grep is the empty session laziness exists to remove.

## Notes

**Expect this leaf to decompose, most likely by scope.** Three scopes, two
families, and a 1,537-line catalogue whose every manifest-changing edit costs
model work in both columns. Decompose only when the work in hand proves bigger
than one session, and cut the split `obligation-placement-k63`'s rule implies
rather than the split that looks tidy today.

**A disposition is a decision about the contract, so the ADR test applies.**
`content/ADR-FORMAT.md`'s when-to-write test decides whether an item earns a
record in `docs/adr/`; several of these — the closed-set additions especially —
are contract changes a later reader will want the cost of, not just the outcome.

`docs/formalism-findings.md` is a log and is **not** revised by this leaf beyond
recording an outcome in place where an entry named one as owed.

## Decisions (running log)

### The enumeration ran, the instrument is validated, and the set is 93 sites

```sh
grep -rn "formal-synthesis-k16" . \
  --exclude-dir=.jj --exclude-dir=target --exclude-dir=.grove \
  --exclude-dir=_apalache-out --exclude-dir=.review-tmp
```

**93 sites across the same 11 files**, against the 97 the task file recorded at
the revision this leaf was cut. The four that went are
`obligation-placement-k63`'s: `TT-24.c` and `TT-24.d` are retired to `FN-32` and
`FN-21.c`, and the sites that named the placement question went with them. Both
controls hold in the same command — the live sibling handle
`cross-model-replay-k15` finds **15** (14 at the cut revision, one added by the
placement work), and the invented handle `formal-synthesis-k99` finds **0**. A
broken instrument cannot produce clean-there *and* dirty-here in one invocation,
so the 93 is evidence rather than a grep that exited 0.

Per file: `docs/formalism-findings.md` 31 · `crates/grove-finish/models/README.md`
22 · `models/system/README.md` 15 · `crates/grove-finish/models/finish.als` 6 ·
`models/system/lifecycle.qnt` 5 · `docs/specs/semantic-contract.md` 4 ·
`crates/grove-task-tree/models/README.md` 4 · `models/system/lifecycle.als` 2 ·
`docs/adr/root-lifecycle-stays-with-its-receipt.md` 2 · `models/README.md` 1 ·
`crates/grove-task-tree/models/task-tree.qnt` 1.

**The manifest is 128 obligations**, confirmed by running the instrument that
reads it rather than by counting the document: `models/run.sh --list` prints
`-- 128 obligations in scope`, exit 0. That is `obligation-placement-k63`'s
arithmetic standing, and nothing here reopens it.

### The classification: 24 items, and the sites that carry each

Every one of the 93 sites is classified below and no site appears twice. A site
that names no decision is recorded as **prose** rather than dropped, which is
what the `Done when` asks for.

**D = decide here · R = route to a named owner · P = prose that names the leaf
and hands over nothing.** **MC = manifest-changing** (adds, removes or re-scopes
an obligation, or changes a closed set an obligation sweeps — opens a
`(family, obligation)` cell both families must fill) · **MN = manifest-neutral**.

#### Closed-set additions — every one MC, and none of them scope-local

| # | item | scope | cls | sites |
|---|---|---|---|---|
| 1 | `RRolledBack` / `RefRollbackNotCommitted` — a rolled-back finish has no refusal reason; `NoTrackedDeletion` and `RootIdentityChanged` are each *false* of it. Both families added an atom **independently**. Two exits: widen the closed set, or restate the outcome. | FN | D·MC | `finish.als:567`; `finish/README:1405` |
| 2 | `RConfigInvalid` — `SY-04.b` requires an invalid configuration to refuse, and no closed reason names one. | SY | D·MC | `lifecycle.qnt:328`; `lifecycle.als:58`, `460`; `system/README:377`, `753`; `findings:6070` |
| 3 | `RGenContended` / `Stopped` — `SY-10.b`'s contended-generation **visible stop**: not a `Refused` (no reason for a handoff timeout; `EpochStale` is `SY-10.a`'s mismatch) and not a `Blocked` (§*Outcomes* scopes blocks to a part-way transaction). Shipped: `one-live-driver-per-working-tree` says the driver "stops `blocked`". | SY | D·MC | `lifecycle.qnt:337`; `lifecycle.als:58`, `460`; `system/README:377`, `753`; `findings:6070`, `8730` |
| 4 | `ONotEntered` — `FN-01`'s first preflight member "produces no refusal at all", but a total action must return something or `FN-01.a` is true by construction. | FN | D·MC | **no k16 site** — declared at `finish/README:3392`, which names no owner; inherited through this leaf's own `Context` |
| 5 | `W8WitnessTracked` — `FN-13`'s stated witness is *a commit attempted while the witness is tracked, refused*, and no closed reason names a tracked witness. An operator cannot be told from the reason that the **repository** is what blocks. | FN | D·MC | `finish.als:603`; `finish/README:1423`; `findings:4333` |

#### Opposite-resolution items — the two columns answered the same text differently

| # | item | scope | cls | sites |
|---|---|---|---|---|
| 6 | **`FN-13` refuse-or-block.** `finish.als` refuses (following the catalogue, "the sole input to the formal phase"); `finish.qnt` blocks (on the three-contexts rule). `task-tree-transactions-fail-closed` says a tracked witness "keeps the witness unwalkable as **Recovery pending**" — a `Blocked` — and the catalogue says refused, so the two *documents* disagree too. | FN | D·MC | `findings:4333`, `8622` |
| 7 | **`FN-10.b` / `FN-32` refuse-or-block.** `finish.als` refuses where `finish.qnt` blocks, at an unclassifiable artifact at a reserved name met inside a transaction, and **both are green** against text that says only *fails closed*. Quint's `wit_FN_32` is worded on the blocking half and moves with the answer. | FN | D·MC | **no k16 site** — `semantic-contract:613` names *this leaf* directly, which is why it is not in the 93; `finish.als:3271` reads the neighbouring `FN-15.d` as Q2 evidence |
| 8 | **Whether `reap` is gated on the root's classification.** Entry 046 finding 4. The product settles the second half: `src/loop_driver.rs` reads no root classification on the reap path, so the catalogue gap is real and **no product defect stands behind it**. | SY | D·MN | `findings:8622` (tabulated), `lifecycle.qnt:2195` region |

#### Claims false as literally worded

| # | item | scope | cls | sites |
|---|---|---|---|---|
| 9 | `FN-25.b` — `RecoveryPending`'s third sentence (*the outcome cannot yet be proven either way*) is **false of two of its own table's rows**, read as a conjunct; and `OwnershipConflict`'s three printed examples are not exhaustive of its own general clause. The two are one sentence read from both sides. Catalogue owes: move the sentence out of the definition, and add the correlation proviso to the second clause. Both citation-sized. | FN | D·MN | `finish/README:1723`, `2682`, `2698`; `findings:5530` |
| 10 | `SY-13` over the `Legacy`/`Foreign`/`Malformed` sinks — under the literal text every one of the three is a sink and **both obligations are FALSE**. Two independent readings (`system-k59` and the model) reached the same verdict. The repair the catalogue does not consider: quantify over the stable states the loop's own **admitted** actions reach. | SY | D·MN | `system/README:1078`, `1081`, `1531`; `findings:7821` |
| 11 | `TT-17` — "the classification SHALL depend only on the format witness, never on any task entry's text" is contradicted by the catalogue's own `PartialScaffold`, which is defined by an exact closed subset of the root's **contents**. | TT | D·MN | `task-tree/README:1262` |
| 12 | `TT-15.a` — a current root with no live task and a foreign artifact at a reserved name classifies `CurrentSpent` and must report `Empty`, while `TT-24.b` requires that same tree to refuse and `TT-18` puts the refusal two stages ahead. Inconsistent under one tree; `TT-24.b` is the one whose whole purpose is to win. | TT | D·MN | `task-tree.qnt:2063` |
| 13 | `TT-20`'s prohibition on `Legacy` — **false of shipped behaviour**, in the window after the charter and before the leaf, and false in the model under one `hand-edit`/`foreign-write` during an open scaffold. Does not establish harm; the severity is this leaf's to assign. | TT | D·MC | `task-tree/README:57`; `findings:8819`, `8822` |
| 14 | `FN-28` — *absent* reads as a fact about the disk and is a fact the protocol cannot hold: after the quarantine rename the task-root **name** is the world's to occupy. Operands must be stated as things Grove *establishes and preserves*. The durable evidence a finish succeeded is the correlation ticket and nothing else. | FN | D·MN | `finish.als:5371`; `finish/README:2728` |

#### Scoping

| # | item | scope | cls | sites |
|---|---|---|---|---|
| 15 | Whether `SY-04.b` owes `SY-03`'s *a preflight is never a licence* for the **configuration** as well as the layout. `outcomeOn` gates on the driver's **recorded verdict** while the layout gate three lines below reads **live**, so a `configChange` between validation and transition leaves the validation standing as a licence. Retained counterexample, operator's hands out of it. | SY | D·MC | `system/README:819`, `1584`; `lifecycle.qnt:2165`; `findings:8730` (shared with #3) |
| 16 | `SY-04.b`/`SY-14.b` over-applied to `release-lease`, which touches no tree and launches nothing. Gating it means an invalid personal configuration strands a lease the loop can only escape by dying. Two repairs: exempt the release, or admit process death. | SY | D·MN | `system/README:969`; `findings:6626`, `6737` |
| 17 | `SY-04.b`'s *an invalid configuration leaves the working tree byte-identical* is true of **Grove's own transitions** and false of the world's, while §*Actions* puts `hand-edit` and `foreign-write` in the same table. One clause owed. Entry 042's class, second instance. | SY | D·MN | `system/README:474`; `findings:6361`, `6452` |
| 18 | `SY-14.b`'s *every action* read as every action **on the tree** — the literal quantifier reaches `acquire-lease`, `validate-config` and `release-lease`, so a blocked tree could not release its own lease and `FN-26`'s two operator-restorable exits would be unreachable. | SY | D·MN | `system/README:1531` (§2 heading region) |
| 19 | `EN-11`'s controls column mis-attributes `TT-24.b`, whose dependency is `EN-13`. The row has now been **wrong twice** — `obligation-placement-k63` edited it only for the `TT-24.d` retirement and left this — which is worth more than the one-word fix. Catalogue line 661. | TT | D·MN | **no k16 site** — `task-tree/README`'s *`EN-11` does not gate `TT-24.b`* section names no owner; inherited through this leaf's own `Context`, which says the row "was not touched" |

#### State-table completeness

| # | item | scope | cls | sites |
|---|---|---|---|---|
| 20 | The shipped *ambiguous partial root scaffold* refusal is a fail-closed outcome the state table has **no member for**, and entry 048 judges it a better answer than either model gives. | TT | D·MC | `findings:8819`, `8822` (shared with #13); evidence at `findings:8796` and `src/tree_lifecycle.rs:474`, `538` |
| 21 | `PartialScaffold` defined by the **absence of everything else** vs by the **presence** of the scaffold's own entries. Under `EN-13` (a foreign entry at any name) plus one `crash`, Grove reads its own interrupted work as somebody else's legacy tree — an editor swap file or a `.DS_Store` is enough. The safety argument (every value a completion writes is fixed in advance) survives the change. | TT | D·MC | `task-tree/README:1375` |
| 22 | The **mid-flight block**: an ordinary mutation that has already applied an effect and whose next create is no longer licensed is none of the three contexts *Outcomes* fixes. A derived test written before the decision would encode the model's least-wrong choice as the contract. | TT | D·MC | `findings:7284` |
| 23 | A disposal that has **released its reserved witness while its quarantine still stands** is a disk the state table has no row for; without a member it classifies `Current(Spent)`, which §*States*' load-bearing property forbids. Adding a member is licensed by the catalogue in as many words. | FN | D·MC | `finish.als:1033` |
| 24 | The whole `Reserved` class ordered **before** `Absent` — taken literally the table classifies the post-quarantine-rename disk as `Absent`, which the same section's load-bearing property forbids. "A one-word edit either way, and not this subtree's to make." | FN | D·MN | `finish.als:1083`; `finish/README:2546` |

#### Prose fix — manifest-neutral, and the only one

| # | item | cls | sites |
|---|---|---|---|
| 25 | The catalogue's *`models/run.sh` is the one repository runner* section states **three** obligations; its own Q4 paragraph makes a fourth (the removal matrix) and `models/run.sh` numbers four. Verified against the runner's own header, which lists 1–4. | D·MN | `models/README:91` |

#### Routed — a named owner other than this leaf

| # | item | owner | sites |
|---|---|---|---|
| 26 | `TODO.finish_process.md` Q1 – Q4, and which of two findings decides Q3 (the replace transition is *reachable* in the incumbent at 40.9% of `scenario_march` traces, while its Q4 row reads `none`). Also Q4-105–107 read as one bundled row with three names. | `finish-verdicts-k65` | `semantic-contract:256`; `finish/README:2847`, `3565`, `3577`; `findings:7531` |
| 27 | The ordinal root-lifecycle verdict — **already decided** at `root-lifecycle-stays-with-its-receipt`, so no leaf is inserted before `extract-task-tree-k24`; the narrowed root-*creation* successor question is deferred to the node. | `finish-verdicts-k65` | `root-lifecycle ADR:97`; `findings:8409` |
| 28 | Whether the catalogue gains or declines a **general** form of *once the caller grades an effect applied it never ungrades it* (carried today only in lane-shaped form as `FN-26`). **Re-routed while landing it:** the node brief first sent this to `finish-verdicts-k65` with the ordinal successor question it sits beside, and it is separable — gaining the general form adds an `FN-` claim and therefore a cell in both families, which is scope work rather than a verdict. | `finish-scope-k71` | `root-lifecycle ADR:69` |
| 29 | Crate-facing seams, and the derived Rust tests each finding owes. Explicitly *"once the models have shown where the boundaries actually fall"*. | `handoff-audit-k66` | `semantic-contract:1630`; `findings:7935`, `8822` |
| 30 | `FN-13`'s **class register** disagreement — *shared safety* in the register, *incumbent mechanics* in the finish README's own commit-slice note; the consequence is a row of Q4's matrix. | `finish-verdicts-k65` | `finish/README:2767`; `findings:5767` |
| 31 | The `SY-10.b` / `SY-11.b` collision over `WGen` — one obligation's apparatus constructs the state another forbids, and no predicate over `waits` separates the two senses. **A model finding, not a catalogue finding**; the task file says route, do not dispose. | model owners | `system/README:1623`; `lifecycle.qnt:714` |
| 32 | `SY_11a` is blind to a **repeat acquisition at an existing site** — the shape chosen to be robust against a sixth acquisition *site*. No `SY-` obligation states it. | model owners | `system/README:705`; `findings:6121` |
| 33 | `SY-05.b`'s stronger claim needs a seam importing independently established component outcomes rather than manufacturing both operands together — an abstraction change with its own cost. | model owners | `system/README:1206`; `lifecycle.qnt:2195` |
| 34 | Whether to split the ghost record out of Quint's finish state / re-encode task-tree reachability as something other than an unrolled fixpoint, so `quint verify` becomes affordable. Both explicitly *"weigh rather than this leaf"*. | model owners | `finish/README:3162`; `task-tree/README:1057` |
| 35 | An `EN-` assumption row for **process death**, which `SY-01.b` rests on entirely and which is granted silently in both families by whoever writes the crash transition. | model owners | `findings:5891` |
| 36 | Whether the **shipped diagnostic** distinguishes the two `LayoutUnsupported` gates; adopts the two `RecoveryPending`/`OwnershipConflict` names; names both the change and the quarantine on a failed return (`FN-22.h`); and adopts the `OwnershipConflict` precedence where both `FN-25` arms hold. Four product-facing questions, all *"decides on that evidence"*. | `handoff-audit-k66` | `semantic-contract:614`, `1043`; `finish/README:1132`, `1293`, `1391` |

#### Prose that names the leaf and hands over nothing

**Twenty-three sites** are statements *about* what a green run does not cover,
about method, or about the record — each already discharged where it stands,
none of them a decision, and no catalogue edit follows from any of them. They
are recorded here rather than dropped, which is what the `Done when` asks for:
`finish.als:3271`; `finish/README:70`, `88` (the `obligation-placement-k63`
prediction, already marked SETTLED in place), `1232`, `1497`, `1616`, `1685`,
`2515`; `system/README:669`, `721`, `725`; `findings:3752`, `4533`, `4945`,
`5099`, `6686`, `7369`, `7984`, `8120`, `8896`, `8906`, `9265`, `9962`.

### The per-site ledger — all 93, each accounted for

The item tables above give each item its sites; this is the same mapping read
the other way, so that no site is lost. A site carrying two items is listed with
both, and three items (#4, #7, #19) reach this leaf through the task file's own
`Context` rather than through any site — recorded as `—` so the difference is
visible rather than silently absorbed.

| file | site → item |
|---|---|
| `docs/specs/semantic-contract.md` | 256→26 · 614→36 · 1043→36 · 1630→29 |
| `models/README.md` | 91→25 |
| `crates/grove-task-tree/models/task-tree.qnt` | 2063→12 |
| `crates/grove-task-tree/models/README.md` | 57→13 · 1057→34 · 1262→11,12,13 · 1375→21 |
| `crates/grove-finish/models/finish.als` | 567→1 · 603→5 · 1033→23 · 1083→24 · 3271→prose · 5371→14 |
| `crates/grove-finish/models/README.md` | 70→prose · 88→prose · 1132→36 · 1232→prose · 1293→36 · 1391→36 · 1405→1 · 1423→5 · 1497→prose · 1616→prose · 1685→prose · 1723→9 · 2515→prose · 2546→24 · 2682→9 · 2698→9 · 2728→14 · 2767→30 · 2847→26 · 3162→34 · 3565→26 · 3577→26 |
| `models/system/lifecycle.als` | 58→2,3 · 460→2,3 |
| `models/system/lifecycle.qnt` | 328→2 · 337→3 · 714→31 · 2165→15 · 2195→8,33 |
| `models/system/README.md` | 377→2,3 · 474→17 · 669→prose · 705→32 · 721→prose · 725→prose · 753→2,3 · 819→15 · 969→16 · 1078→10 · 1081→10 · 1206→33 · 1531→10,18 · 1584→15 · 1623→31 |
| `docs/adr/root-lifecycle-stays-with-its-receipt.md` | 69→28 · 97→27 |
| `docs/formalism-findings.md` | 3752→prose · 4333→5,6 · 4533→prose · 4945→prose · 5099→prose · 5530→9 · 5767→30 · 5891→35 · 6070→2,3 · 6121→32 · 6361→17 · 6452→17 · 6626→16 · 6686→prose · 6737→16 · 7284→22 · 7369→prose · 7531→26 · 7821→10 · 7935→29 · 7984→prose · 8120→prose · 8409→27 · 8622→6,8 · 8730→3,15 · 8819→13,20 · 8822→13,20,29 · 8896→prose · 8906→prose · 9265→prose · 9962→prose |
| `#4`, `#7`, `#19` | — inherited through this leaf's `Context`, no site |

**Two items besides those three are inherited rather than sited**, and both are
worth naming because a later reader will look for them: item 4 (`ONotEntered`)
is declared at `finish/README:3392` under *Deliberate omissions*, which names no
owner, and item 7 (`FN-10.b`/`FN-32` refuse-or-block) is at
`semantic-contract:613`, which already names **`catalogue-disposition-k64`**
directly — which is exactly why neither is in the 93. A future re-run of the
instrument will not find them, so they must travel in prose.

**A fourth task-tree qualification is deliberately *not* an item.** `TT-10` sits
in `task-tree/README`'s narrowings block beside items 11 – 13 and is declared
there as a **statement qualification rather than a narrowing** — the README
argues `TT-10`'s own text ("no algebraic refusal reaches an operator *from an
ordinary argument*") is "the claim rather than less than it", and the mid-flight
case it excludes has `TT-21.b` and `TT-24.a`. It hands over nothing and is
recorded here so that a later reader meeting four bullets under one
`formal-synthesis-k16` sentence does not read a lost item.

### This leaf is bigger than one session, and the measurement is the cascade

Fifteen of the twenty-four decided items are **manifest-changing**, and the
`Done when` requires each to land with *both* families answering the new or
changed obligation with a property plus its required witnesses, and each
affected scope's `models/run.sh --scope <scope> --family <family>` green with
coverage asserted. The cost is not the catalogue edit. Measured from the
producing sessions' own run lines: Alloy's task-tree cell is **6888 s of CPU**
for 103 commands and its finish cell 14 m 33 s for 180; the whole repository is
791 commands over 258 cells. Fifteen manifest changes across three scopes and
two families is model work in six columns plus re-runs measured in hours, and it
sits beside a 1,673-line catalogue and ~7,000 lines of model README.

### The split is by scope, in the crate dependency direction, behind a vocabulary child

Decomposed with `grove-llm leaf-decompose`. Two arguments fix the shape, and
neither is the split that looks tidy.

**Why a vocabulary child comes first.** The closed sets — outcomes, refusal
reasons, blocked diagnoses, and the stable-state table — live in §*Vocabulary*,
**above** all three claim sections, and
[`obligations-follow-context-not-artifact`](../../../../docs/adr/obligations-follow-context-not-artifact.md)
clause 4 is exactly about such terms: a term partitioned across groups that
belong to different scopes **has no single owning scope**. So a closed-set
addition is not any one scope's to make. Three scope children each editing the
same closed list would collide, and worse, each would decide in ignorance of the
other two's members — which is the catalogue's own stated hazard, *a new member
of a closed set imposes a matching outcome on the [other] column*. Items 1 – 5
are five such additions arriving from two different scopes. And the sharpest
items on the list, 6 and 7, are **refuse-or-block** questions, which cannot be
answered without knowing whether the refusal set has a member for the case —
so the vocabulary gates them too. This is the parent brief's own ordering
argument at one grain finer: a shared thing is frozen before its dependents move.

**Why the scope children run TT → FN → SY.** The rule orders the three scopes by
the approved crate dependency direction, and clause 1 means a re-scope always
moves an obligation **up** — an obligation naming something from above belongs to
the highest scope its text names. The known instance moves that way:
`TT-24.c`/`TT-24.d` became `FN-32`/`FN-21.c`. Running task-tree first therefore
hands any re-scoped item **forward**, into a child that has not run yet; running
lifecycle first would need a hand backward into a retired one. The order is
forced by the rule rather than chosen for tidiness.

Item 19 (`EN-11`'s controls row), item 25 (the runner's three-vs-four) and every
routed item ride with the **vocabulary** child: none is scope-local, all are
manifest-neutral, and items 26 – 30 gate `finish-verdicts-k65`, which is this
node's very next sibling.
