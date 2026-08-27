# finish-scope-k71


## Goal

Decide and land the four `FN-` scoped catalogue findings, with both families
answering every changed obligation and `models/run.sh --scope finish` green with
coverage asserted in both columns.

## Context

**Second of the three scope children.** `task-tree-scope-k70` ran first and may
have re-scoped an item upward into this one — read its retired body before
starting, because a re-scoped item arrives as a new `FN-` obligation with an
empty cell in both families rather than as a note. Anything this leaf re-scopes
goes forward to `lifecycle-scope-k72`, never backward.

**`closed-sets-k69` froze the vocabulary and answered the two refuse-or-block
questions.** Both were this scope's — `FN-13` and `FN-10.b`/`FN-32` — so read
its decisions first; items 9 and 24 below both sit against the same §*Outcomes*
text those answers changed. Note that Quint's `wit_FN_32` is worded on the
blocking half and moves with the answer.

**Run cost, measured rather than guessed:** Alloy's finish cell is 14 m 33 s for
180 commands, Quint's 4 m 25 s for 228. Unlike the task-tree scope this one
**can afford `QUINT_VERIFY=1`**, and the finish README says so; the repository
default stays 0 because the task-tree scope cannot.

**The four items, with the evidence each already has.** Numbering is the node
brief's item table.

- **9 · `FN-25.b` — one sentence read from two sides** (`MN`, and the catalogue
  edit is citation-sized). `RecoveryPending`'s third sentence — *and the outcome
  cannot yet be proven either way* — reads as a conjunct of the definition, and
  **two rows of the same table are blocks whose outcome IS proven**: *after
  restoration, `Committed` leaves the witness blocking the restored tree*, and
  *after the rename, a return that cannot complete*. Both are diagnosed
  `RecoveryPending`, so as a conjunct the sentence makes `FN-25.b` false on
  both. Separately, `OwnershipConflict`'s three printed examples are not
  exhaustive of its own general clause: a Grove-**owned** artifact whose
  manifest names another handle falls through both diagnoses under the examples
  and is caught by the sentence. The finish README states the edit in as many
  words — *move "the outcome cannot yet be proven either way" out of
  `RecoveryPending`'s definition, and add to `OwnershipConflict`'s second clause
  the proviso that it applies when Grove cannot correlate the state to its own
  attempt.* **A closed set whose members are defined by a general sentence plus
  examples is not a closed set until a model asks which of the two it is** — that
  question is the finding rather than the answer.
  Sites: `finish/README:1723`, `2682`, `2698`; `findings:5530`.
- **14 · `FN-28`'s operands are stated as facts that hold rather than as things
  Grove establishes** (`MN`). *A finish succeeds exactly when the exact
  attempt-bound commit is proven and the task root is absent*, and *absent* reads
  as a fact about the disk — a fact the protocol cannot hold, because after the
  quarantine rename the task-root **name is free** and the world may occupy it.
  Three separate formulations of conjuncts (a) and (c) were each falsified by
  exactly that trace and by nothing else. What follows is worth more than the
  check: **the only durable evidence a finish succeeded is the correlation
  ticket**, which is `FN-03`'s subject and is why that claim exists. A
  `grove finish` that decided success by stat-ing the task root would report
  failure on a grove someone simply started using again, and success on one
  where the quarantine had been moved back over it.
  Sites: `finish.als:5371`; `finish/README:2728`.
- **23 · A disposal that has released its reserved witness while its quarantine
  still stands has no state-table row** (`MC`). Task root present, nothing at the
  witness's name, Grove's own quarantine standing. Without a member that disk
  classifies `Current(Spent)` — an ordinary spent grove — which is exactly what
  §*States*' load-bearing property forbids. **Adding a member is licensed by the
  catalogue in as many words**: *`TT-18`/`TT-19` are stated over the reserved
  CLASS rather than over its members so that removing one member changes no
  claim.* Mutation 50 is the evidence that the member is load-bearing rather
  than decorative. Site: `finish.als:1033`.
- **24 · The `Reserved` class must be ordered before `Absent`** (`MN`, "a
  one-word edit either way"). Taken literally the table classifies the disk an
  interruption after the quarantine rename leaves — task-root name free, Grove's
  quarantine holding the root — as `Absent`, which the same section's
  load-bearing property forbids. The two are in tension only once a reserved
  name can be occupied while the task-root name is free, **which the finish
  protocol creates and the task-tree scope never does**. `FN-24.a`'s third
  conjunct is what catches the other order; mutation row 49 restores the
  catalogue's order and the check goes red. Either the table's order is wrong
  for a scope that reserves names beside the task root, or the `Absent` row
  needs the qualification the property below it already implies.
  Sites: `finish.als:1083`; `finish/README:2546`.

- **28 · A general form of *once the caller grades an effect applied it never
  ungrades it*** (`MC` if gained). Re-routed here by `closed-sets-k69` from the
  node brief's routed set: it arrived through
  [`root-lifecycle-stays-with-its-receipt`](../../../../docs/adr/root-lifecycle-stays-with-its-receipt.md)
  beside the ordinal successor question, and the two are separable — the ordinal
  verdict is `finish-verdicts-k65`'s, while **gaining the general form adds an
  `FN-` claim and therefore a cell in both families**, which is this scope's
  work. The catalogue carries it today only in lane-shaped form, as `FN-26` —
  *history is never rewritten to clear a block* — which the Quint finish model
  dials as `HISTORY_IMMUTABLE`. The prototype's own finding is that four
  revalidation points are necessary and **not sufficient**: after the last one
  there is always a suffix in which the caller's grade can move, and by then
  disposal has destroyed the ability to return. Note the ADR's own qualification
  — the general form is *statable in general, unverifiable by the library*, so
  gaining it puts the obligation on `grove-finish` rather than on
  `ordinal-fs-tree`.

**What is not this leaf's.** Item 30 (`FN-13`'s class-register disagreement —
*shared safety* in the register, *incumbent mechanics* in the README's own
commit-slice note) is routed to `finish-verdicts-k65`, because its consequence
is a row of Q4's matrix. Item 36's four shipped-diagnostic questions are
`handoff-audit-k66`'s. Items 34 (splitting Quint's ghost record) is a modelling
cost decision routed to the model owners, not a catalogue disposition.

## Done when

- Items 9, 14, 23 and 24 are each decided and landed in
  `docs/specs/semantic-contract.md`, each marked manifest-neutral or
  manifest-changing at the moment it is decided.
- For every manifest-changing one, **both** families answer the new or changed
  obligation with a property command plus its required witnesses, or with that
  family's own declared gap.
- `models/run.sh --scope finish --family alloy` and `--family quint` are each
  green with coverage asserted, and both run lines are recorded here.
- `crates/grove-finish/models/README.md` says, in place, how each finding it
  recorded rather than fixed was disposed. Its *What `formal-synthesis-k16`
  inherits from these three* section is the specific target and should be
  retitled once its content is disposed rather than left naming a leaf that no
  longer owns it.
- Anything re-scoped upward is written into `lifecycle-scope-k72`'s body before
  this leaf retires.

## Notes

**`closed-set-additions-k74` LEFT YOU A BRANCH THAT IS ONLY NOW A QUESTION, AND
THAT IS THE POINT OF IT.** The catalogue gained `FN-29.b` — *every `Refused` is
returned with the tree byte-equal to the tree that action received, and an effect
that stands and can be neither completed nor undone is `Blocked`* — and
`crates/grove-finish/models/finish.als`'s `doCommitAttempt` has a second else
branch, `W9SlotPending`: **a commit attempted while the witness is published but
the evacuation is not complete, reported as a `Refused`.** The published witness
is an effect of that action.

It **survives** `FN-29.b` as written, because that obligation's antecedent is the
completed evacuation (`gateEvacuatedNext`) and this branch is short of it. That
is a fact about where the line was drawn, not evidence that the branch is on the
right side of it. Deciding it needs the restoration path's own obligations — is a
published-but-empty witness an effect the action can still *undo*, and does the
restoration path undo it before returning? — which is scope work rather than
vocabulary, and no session referred it to `k74`, which is why it was named rather
than re-decided under cover of a disposition. The site carries the note in place.

**Item 28's general form now has a neighbour worth reading beside it.** *Once the
caller grades an effect applied it never ungrades it* is the same subject as
`FN-29.b` from the other side: `.b` says what an outcome may be given what
stands, and 28 asks whether a grading may be withdrawn. If both land they should
be stated so that neither is the other restated —
[`a-refusal-leaves-nothing-standing`](../../../../docs/adr/a-refusal-leaves-nothing-standing.md)
is the record to cite rather than to re-argue.

**Item 9 and `FN-29.b` touch one sentence from opposite ends.** `FN-25.b`'s third
sentence (*the outcome cannot yet be proven either way*) is item 9's, and
`closed-set-additions-k74` deliberately did **not** lean on it when it fixed
`FN-13`'s diagnosis as `RecoveryPending`: that argument rests on `FN-25`'s
**first** sentence — the artifact is provably this attempt's — precisely so that
your repair of the third does not disturb it.

**Item 9's edit is citation-sized and its finding is not.** The finish README is
explicit that what is *not* citation-sized is whether the shipped diagnostic
adopts the `OwnershipConflict` precedence where both arms hold — and that is
item 36, routed to `handoff-audit-k66`. Land the catalogue edit here; do not
absorb the product question.

**A disposition is a decision about the contract, so the ADR test applies**
(`content/ADR-FORMAT.md`). Item 14 is the likeliest to earn a record: *the
correlation ticket is the only durable evidence of success* is a positive design
constraint a later implementer needs, and it is currently stated only inside a
model comment.

## Handed forward by `task-tree-scope-k70` — no cell, and one correction to a record you inherit

**Nothing is re-scoped into this leaf.** No `TT-` obligation moved to `FN-`, and
the new closed-set member costs this scope nothing: `finish.qnt`'s `type Refusal`
spells out "only the members this scope can reach", no `FN-` obligation
quantifies over the refusal type exhaustively, and neither finish model carries a
`PartialScaffold` state at all. Checked rather than assumed, which is what
`closed-set-additions-k74` asked of a scope meeting a widened set.

**One thing you inherit is different from what the node brief hands you, and it
touches your item 28's neighbourhood.**
[`a-refusal-leaves-nothing-standing`](../../../../docs/adr/a-refusal-leaves-nothing-standing.md)
now has a **fourth** consequence under clause 1 — *the action includes its
unwind* — and a paragraph correcting the argument by which `FN-29.b` was scoped
to `grove-finish` alone.

- **The conclusion stands and the reasoning does not.** `k74` scoped `FN-29.b`
  here on the ground that "the task-tree scope has no block to be distinguished
  from". That ground is false: a task-tree mutation *can* leave an effect
  standing, when the unwind itself fails
  (`crates/ordinal-fs-tree`'s `Error::FailedPartiallyRolledBack`). The placement
  holds for a better reason — that outcome reaches the operator in the delegated
  boundary's own vocabulary, so the catalogue absorbs no member for it and the
  **blocked diagnoses stay at 2**.
- **`EN-17` is new** and grants what the boundary supplies: a reported mutation
  failure unwinds every effect it applied. Its mutation lives at the boundary
  (`operations.qnt`'s `rollback_fails`), which `models/run.sh` already delegates
  to, so it costs neither family here a control.
- **Why it may matter to you.** Your `doCommitAttempt` `W9SlotPending` item asks
  whether a branch that refuses with a **published witness standing** is on the
  right side of `FN-29.b`. The fourth consequence sharpens the test: the question
  is not whether the step moved anything but whether the **action** can hand back
  the tree it was given — and unlike an ordinary tree mutation, a finish
  transaction keeps a witness rather than unwinding, which is exactly why its
  stop is recoverable. That asymmetry is now stated where you can cite it.

## Decisions (running log)

### The enumeration, with both controls, and it is eleven sites

```sh
grep -rn "finish-scope-k71" . \
  --exclude-dir=.jj --exclude-dir=target --exclude-dir=.grove \
  --exclude-dir=_apalache-out --exclude-dir=.review-tmp
```

**11 sites across 5 files.** `finish.als` 4 (1047→item 23, 1097→item 24,
1829→the `W9SlotPending` branch, 5441→item 14) · `finish/README` 5 (1552→`EN-08`'s
row for `FN-31.c`, 2599→item 24, 2735 and 2751→item 9, 2781→item 14) ·
`findings:8460`→item 28 · `root-lifecycle` ADR:69→item 28.

**The same command for `formal-synthesis-k16` now finds 31, all of them in
`docs/formalism-findings.md`** — the log this node does not revise beyond
recording an outcome in place — against the 93 across 11 files
`catalogue-disposition-k64` enumerated. Every model and catalogue site that
named the node has been re-pointed by `routing-and-prose-k73`,
`closed-set-additions-k74` and `task-tree-scope-k70`. Controls, in the same
invocation: the live sibling `cross-model-replay-k15` finds **15**, the invented
handle `formal-synthesis-k99` finds **0**. Clean-there plus dirty-here in one
command is what makes the eleven evidence rather than a grep that exited 0.

**One site is not in the node brief's item table and is mine anyway**:
`finish/README:1552`, where `routing-and-prose-k73` disposed the `EN-08`
controls-column finding and left *meeting* the row for `FN-31.c` here. It is
model work rather than a disposition; it is taken up below.

### 23 and 24 are one repair, and the state table had neither half

**Landed in §*States*: `Reserved(Quarantined)` is the reserved class's fourth
member, and the whole class is ordered before `Absent`.** Classified
**manifest-changing** — it changes a closed set an obligation sweeps, so both
families owe a matching outcome — with `models/run.sh --list` unchanged at
**130**, which is `closed-set-additions-k74`'s measurement met a second time: a
closed-set addition costs the columns without moving the count.

**The argument that decides it is that the quarantine rename is not the end of
the protocol**, and this is not the argument the leaf started with.

- `FN-22`'s **fourth** revalidation point runs *after* the rename, and two of its
  three rows return the quarantine: a re-observed `NotCommitted` rolls the handoff
  back, an `Indeterminate` returns it and blocks. So between the rename and that
  point the task-root name is free and the disposition is **unsettled** — which is
  *a task root whose deletion is not yet proven*, word for word.
- The shipped protocol has the same shape, established by reading the code rather
  than the model: `proof.revalidate()` runs after `cleanup.handoff()` and a
  failure calls `cleanup.restore()` (`src/finish_transaction.rs:1949-1969`).
- **`SY-05.b` is where it cashes out**, and it names `FN-19` by identifier: *no
  trace exposes an absent task root before the deletion is proven*, which is what
  makes `SY-05.a`'s *a missing task root means start a new grove* sound. With
  `Absent` classified first that trace exists, and the loop would scaffold a fresh
  grove over an unsettled finish. **The ordering is what makes `SY-05.b` true.**

**The alternative repairs are rejected, and there are two of them rather than
one.** Qualifying the `Absent` **row** — *and nothing of Grove's at a reserved
name either* — makes `FN-24.a`'s third conjunct true by construction, so the
departure becomes invisible to the check that exists to catch it. **Narrowing the
model's own state vector** to stop classifying the quarantine at all is the same
defect wearing a different hat, and it is rejected for the same reason.

### The second alternative was provisionally LANDED and then broken, and the episode is the leaf's one adversarial spend

This is recorded rather than tidied away, because a disposition that was reversed
under challenge is stronger evidence than one that was never challenged.

**What was landed for a while:** that the quarantine lives in the workspace's VCS
control directory (`.git/grove/FINISHED-<handle>-<attempt>`,
`src/finish_transaction.rs:322-329`), that §*States* classifies a *task root* and
so reaches it in neither direction, and that both of `finish.als`'s findings were
consequences of its own state vector. It reached the catalogue, both model files
and the control file before it was reversed.

**The reviewer's spend.** `references/execute.md` allows one in-session reviewer
across the leaf; it was spent here, on the narrowest claim with the widest blast
radius — *the state table must not gain the member*. The four-step pass was run
with one flaw worth naming: the artifact still carried the conclusion in its
comments, so the reviewer was not reading a stripped subject. It broke the claim
anyway, on citations that were then **re-verified in this session** rather than
taken on trust.

Classified four ways, as the pass requires:

- **Valid and actionable — the load-bearer was false.** *The deletion is proven
  when the rename lands.* `finish_transaction.rs:1952` disproves it. This is the
  kill.
- **Valid and actionable — the premise of the `TT-18`/`TT-19` argument was
  false.** `FN-24.a`'s third and fourth conjuncts are stated over *something of
  Grove's at a reserved name*, and `SY-05.b` names `FN-19`. Two more claims reach
  the situation. And the inference was wrong independently: membership in the
  reserved class is intensional, and `SY-06.b` reaching `PartialScaffold(Exact)`
  while refusing to complete `PartialScaffold(Ambiguous)` is a standing
  counterexample to *a member neither of a class's claims reaches is not a
  member*.
- **Valid and actionable — the symmetry I missed.** Narrowing the vector makes
  `FN-24.a`'s third conjunct vacuous exactly as narrowing the row does, and I had
  already rejected the row narrowing for that reason.
- **A contract I stated unclearly.** *`FN-20` under the wide reading forbids the
  reaper reading its own marker.* That is right and it is an argument for the
  narrow reading, which is what landed — but it reads as an objection because my
  `FN-20` text had not said which classification it meant. It now does.
- **Actionable, and now done.** `FN-19`'s own witness said *an absent task root*
  and now says *a free task-root name*; Quint's `FN-20` witness required the
  post-rename disk to classify `RAbsent` and is restated over the disposition.
- **Corrected fact, mine.** I cited `classify_unlocked` via `transition_to_current`
  as "the shipped classifier"; that wrapper has **no production caller**. The
  production entry is `transition_driver_to_current`, which reaps the control
  directory **before** classifying — which is *deal with the reservation first*
  realised as a sweep, and is now cited that way in §*States*.
- **Noise at the time, real now.** *The working copy asserts both dispositions.*
  It did — the reviewer read it mid-revert. All four files move together now.

**And one thing the challenge produced that neither disposition contains.** On
the crash path there is no in-tree cleanup owner, so `reap_orphaned` disposes the
quarantine **without re-reading the disposition** (`src/finish_cleanup/reaper.rs`)
— `FN-22`'s fourth revalidation point is never performed there, and the same is
true on the best-effort reap-failure path. That is a **product** question, not a
catalogue one, and it is routed below rather than decided here.

**What the near-miss cost and what it bought.** It cost a full revert of two
catalogue sections, two model files and a control file, and then a revert of the
revert. It bought the thirteenth finding below — `FN-20`'s two subjects — which
was invisible until a candidate disposition would have broken one of them; it
bought the `SY-05.b` argument, which is stronger than the one `finish.als`
recorded; and it bought the `FN-19` witness repair. **A disposition is a
measurement too.**

### 9 · one sentence read from two sides, and a third thing under it

Landed in §*Outcomes*, **manifest-neutral**, as three edits that are one repair:
the partition was carried by the diagnoses' **illustrations** rather than by
their **definitions**.

- *And the outcome cannot yet be proven either way* leaves `RecoveryPending`'s
  definition. Two rows of `FN-22`'s own table are blocks whose outcome **is**
  proven and are diagnosed `RecoveryPending`, so as a conjunct the sentence made
  `FN-25.b` false on two states the protocol reaches by design.
- `OwnershipConflict`'s second instance gains the correlation proviso. It is
  `Indeterminate` written out, and `FN-22` diagnoses `Indeterminate`
  `RecoveryPending` on every row — so read literally **every** `RecoveryPending`
  state satisfied the other name's second instance and the two were not a
  partition at all.
- The instances are declared **non-exhaustive** of the general sentence. A
  Grove-**owned** artifact whose manifest names another handle falls through all
  three and is caught only by *cannot be proved safe to mutate*.

**One thing beyond the two edits the README predicted, and it is `FN-25.a`'s
rather than the product's.** Where a correlated incomplete attempt and an
unclassifiable artifact are both present, both definitions hold and disjointness
is a model's choice; both families chose `OwnershipConflict` independently. The
catalogue now states the precedence as `TT-24`'s rule applied to a diagnosis —
*the outcome names the strongest thing Grove cannot account for* — because
without it `FN-25.a` is not a claim. **The product question is not absorbed**:
whether the shipped diagnostic adopts the precedence, or the two names at all,
stays `handoff-audit-k66`'s.

### 14 · `FN-28`'s operands are restated over Grove's own steps

Landed, **manifest-neutral**: *a finish succeeds exactly when the exact
attempt-bound commit is proven and Grove itself has taken the task root away and
not put it back*, with the three step-shaped conjuncts spelled out. *Absent* is
a fact about the disk and the protocol cannot hold it — after the rename the
task-root **name** is free and the world owns the namespace.

The claim earns an ADR, and the AND test is met on all three legs: it is hard to
reverse (`FN-03`, `FN-20`, `FN-28` and the whole recovery path are stated against
it), surprising without context (stat-ing the task root is the intuitive success
test, and three formulations of the check were written that way before being
falsified), and it settles a real trade-off against a cheaper alternative an
implementer would reach for — deciding success from the filesystem, which needs
no version-control read on the success path and is wrong in **both** directions.

### 28 · the general form is DECLINED, and the catalogue's own table is the refutation

*Once the caller grades an effect applied it never ungrades it* is **not**
gained. `MN` rather than `MC`, and the reason is not cost:

- **It is false of the incumbent.** `FN-22`'s table has two rows that are exactly
  the forbidden transition, and the catalogue insists the two must not be
  collapsed because collapsing them lets a block be reported as a refusal.
- **Granting it as an assumption deletes the states those rows need**, and the
  Alloy column paid for that already — its fourth finding, where an append-only
  history under every step made the disposition monotone and left `FN-22.f` and
  `FN-22.g` answerable by construction.
- **What is true is narrower and already stated twice** — `FN-26`, and `FN-22.a`'s
  four points, which is `SY-03`'s *a preflight is never a licence* at this grain.

So the general form is a **caller obligation a coordinator-shaped design would
impose**, not a property this protocol has. That is a **stronger** argument
against widening `ordinal-fs-tree`'s seam than
[`root-lifecycle-stays-with-its-receipt`](../../../../docs/adr/root-lifecycle-stays-with-its-receipt.md)'s
own *the library cannot verify it*: the caller could not honour it even if it
wanted to, because this contract requires the regrade. The ADR is edited in place
to carry it.

### `W9SlotPending` · the branch returns NOTHING, and the outcome set is over actions

`closed-set-additions-k74` named `doCommitAttempt`'s second else branch and
routed it rather than deciding it, which was right: deciding it needed the
restoration path's obligations rather than the vocabulary. **Decided:
`Sys.res' = NoOp`, and the branch gains its own `why`.** `MN` — no obligation
moves, and the closed reason set gains and loses nothing.

Two things were wrong with `Refused(WitnessPending)` there and they are one
thing.

- **The transaction is still live at that branch** (`txnSame` keeps it at
  `PublishedP`), so nothing has been returned to a caller and the next step may
  still complete or unwind it. A member of the closed outcome set there puts a
  *completed-invocation* outcome over a tree the action has already published a
  witness into and part-evacuated — §*Outcomes*' discriminator at the action
  grain, met from the inside. It escaped `FN-29.b` only because that check's
  antecedent names the **completed** evacuation, which is a fact about where the
  line was drawn rather than evidence the branch was on the right side of it.
- **`W9SlotPending` is `doWPrepare`'s**, where it names a genuinely different
  situation: Grove's own artifact already at the reserved witness name, nothing
  mutated, the attempt over. That is a real `Refused(WitnessPending)` and it
  stays one. The commit-attempt branch had been sharing a `why` with a case it is
  not, and now has `W18EvacuationIncomplete`.

The catalogue gains the general rule beside *a guard wait is not an outcome*:
**the closed set is over actions, and an internal step's own ordering guard
returns nothing.** A model that widens a step's enabling surface to keep an
ordering claim falsifiable — which is exactly why `doCommitAttempt` is enabled
before the evacuation completes, so `FN-11` is not true by construction —
declares the widened branch and gives it no member of the set. `FN-11` is
unaffected: `gateEvacuated` still refuses the early attempt. **This is the second
time the same step/action confusion has been found at that step** — the first was
`FN-13`, whose outcome moved to a block — and it is the last branch there.

### `EN-08`'s row for `FN-31.c` · declared, and the declaration is a finding about two assumptions

`routing-and-prose-k73` landed the rule (an exercise-removal row's controls
column is a claim of unreachability, established by RUNNING the removal) and left
**meeting** the row here. It cannot be met in the Alloy column, and the reason is
worth more than an attempt would have been.

`fact TransactionsStartWhereAProcessStarts` narrows `EN-11` for the transaction's
volatile phase: every trace starts at *no transaction, or one just opened*, and
everything past `Opened` is reached by running the steps. So reaching the
interrupted replacement through `crash` means running the whole six-step body,
the commit, the classification, the quarantine rename and disposal from state 0
before the crash — about seventeen states against that file's thirteen-state
maximum, on the dearest cell in the repository. **A model that POSITS a disk
under `EN-11` cannot also EXERCISE `EN-08` at that disk**, and the two controls
are in tension by construction rather than by oversight.

The row **is met by `finish.qnt`**, whose
`wit_unreach_EN_08_an_interrupted_replacement_resumed` runs the protocol and
stops landing when `crash` is removed. So the row's claim is established; what
the Alloy column reports is a fact about its own realisation, which is the
distinction the assumption table already draws. The general form is landed in the
catalogue's `EN-08` note and the declaration is in the finish README beside the
two witnesses. `MN`.

### Run lines — two cells, both exit 0, coverage asserted, and the Quint cell model-checked

The model files, the catalogue and the finish README were **frozen before the
batch started**, their digests recorded, and recorded again at the end
**unchanged** — the runner reads the catalogue as its manifest and the scope
README for declared gaps, so all of them are subjects rather than bystanders.
Started `2026-08-27T23:55:25Z`, both finished by `00:24:06Z`.

```sh
models/run.sh --scope finish --family alloy                 # 186 commands, 63 of 63 cells, exit 0
QUINT_VERIFY=1 models/run.sh --scope finish --family quint  # 301 commands, 63 of 63 cells, exit 0
```

Both report `-- cells: 63 complete, 0 declared gaps, 0 empty, of 63` and
`10 of 10 rows` of Q4's removal matrix. **No cell is contested** — neither family
declared a gap where the other answered.

**The Alloy cell is 186 commands, unchanged**, and that is the honest measure of
what this leaf did to that column: comments, one branch's `Sys.res'`/`Sys.why'`,
and nothing else. **The Quint cell is 301**, which is 238 non-verify (236 at
`closed-set-additions-k74` plus this leaf's two mutants) plus the 63 properties
`verify_small` model-checks when `QUINT_VERIFY=1` turns its `SKIP` lines into
work. The two new controls are in the reported run and both read
`violated, as the control requires`:

| control | what it restores | result |
|---|---|---|
| `inv_fail_MUT_FN_24a_a_standing_quarantine_reads_as_an_ordinary_grove` | §*States* without `Reserved(Quarantined)` | violated |
| `inv_fail_MUT_FN_24a_the_post_rename_disk_reads_as_absent` | §*States*' former table order | violated |

**`inv_FN_24a`'s restatement was model-checked, not only simulated** — the
`verify_small` module inherits the library's property commands, so all 63 ran
under Apalache to depth 4 with no counterexample, the new third and fourth
conjuncts among them. That is stronger evidence than the bounded simulation the
task-tree scope has to settle for, and it is the whole reason this scope's body
said it could afford `QUINT_VERIFY=1`.

**Wall time, and one correction to the inherited budget.** Alloy **27 m 44 s /
1935 s CPU**, Quint **12 m 53 s / 376 s CPU**, run concurrently on a 16-core
host. `closed-set-additions-k74` measured the Alloy cell at ~18 min and recorded
that concurrent cells *barely contend*; that stops being true when one of them is
**Apalache**, which is heap- and CPU-hungry in a way the simulator is not. The
extra ~10 minutes is contention, not this leaf's edits — the command count did
not move. Budget a verify cell alone, or serially. Recorded in the finish
README's own run-line block so the next session budgets from it.

**`models/run-controls.sh`: exit 0** — `-- runner controls: 10 passed, 0 failed`,
8 m 36 s, run alone after the batch. It is run even though this leaf did not
touch `models/run.sh`, because two commands of a shape the runner classifies
(`inv_fail_MUT_<OB>_…`) entered the suite and the controls are what assert the
classification rather than the commands.

### A `review-design` leaf is cut, and the signal that earned it is mechanical rather than a feeling

`grove-llm leaf-insert lifecycle-scope-k72 finish-scope --kind review-design` →
`04-review-design-finish-scope-k75.md`; `lifecycle-scope-k72` shifted 04 → 05.

**`references/execute.md` allows one in-session reviewer and names the second
need as the signal.** This leaf spent its one, the reviewer **broke the
disposition that had been landed**, and the reversal that followed touched the
catalogue, both model files, the control file, both scope READMEs, the glossary
and two ADRs. That is the substantive non-mechanical fix the rule names, and no
fresh context has read the state it produced — the reviewer read the *mid-flight*
state, with this leaf's conclusion still written into the model comments. That
flaw in the pass is declared rather than hidden, and it is the second half of the
argument for the leaf.

**It is INSERTED rather than appended**, at `lifecycle-scope-k72`'s slot, and the
argument is this node's own: `k72` is chartered to edit §*States* and both model
families against exactly the change under review, and the node brief already
records that *a gate checked after everything it gates is not a gate*.
`obligation-placement-k63`'s review was inserted for the same reason.

Its body carries five specific doubts rather than a goal sentence — the residual
objection the reviewer itself left standing (`FN-19`'s own witness had used the
loose phrase), whether the reserved class has a third claim the enumeration missed
**a second time**, what `FN-20`'s narrowed subject stops catching, whether
`FN-29.b`'s antecedent shrank into vacuity, and whether the `EN-08` declaration is
a finding or an excuse.
