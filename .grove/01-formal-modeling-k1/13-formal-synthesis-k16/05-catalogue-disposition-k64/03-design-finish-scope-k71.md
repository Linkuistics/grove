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
