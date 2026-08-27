# closed-set-additions-k74


## Goal

Decide the five closed-set additions and the two refuse-or-block questions that
turn on them, land them in `docs/specs/semantic-contract.md`, and get both
families answering every changed obligation with every scope the decisions reach
green under a coverage-asserting run.

## Context

**Read the pattern before the five members.** Three of the additions are one
shape recorded three times in the finish scope alone — *seven preconditions
against six reasons* (entry 031), *a tracked witness with no reason* (entry 032),
*a rolled-back finish with no reason* — and
`crates/grove-finish/models/README.md` says in as many words that **the pattern
is not three accidents**. The lifecycle scope then produced two more. Both
families had to invent the same device twice to work around it: `Sys.why` in
Alloy and its Quint counterpart exist because *a refusal reason names the
question asked, not the gate that refused*. So the first question is not "which
five members" but **whether the closed refusal set is under-populated or whether
its reason vocabulary is answering the wrong question**. If that resolves into a
rule, the rule is one ADR rather than five.

**The strongest single piece of evidence on the list is an agreement, not a
disagreement.** Both columns reached the rolled-back-finish gap independently and
each *declared* an atom rather than smuggling one in — `RefRollbackNotCommitted`
in `finish.als`, `RRolledBack` in `finish.qnt`. Two independently built families
inventing the same missing member is the catalogue being wrong, not a modelling
convenience.

**The seven items.**

- **1 · `RRolledBack` / `RefRollbackNotCommitted`.** The catalogue maps the
  `NotCommitted` disposition to *rolls back and yields `Refused`*, and none of
  the seventeen closed reasons names it: `NoTrackedDeletion` and
  `RootIdentityChanged` are each **false** of a transaction whose fingerprint was
  fine and whose root never moved. Reporting it under one of them "would be a lie
  the model could not be caught in". Two exits: widen the closed set, or restate
  the outcome. Sites: `finish.als:567`, `finish/README:1405`.
- **2 · `RConfigInvalid`.** `SY-04.b` requires an invalid configuration to refuse
  with the tree byte-identical, and no closed reason names one. Declared in
  `lifecycle.qnt` and `lifecycle.als`, recorded in `models/system/README.md`.
- **3 · `RGenContended` / `Stopped`.** `SY-10.b` requires a contended generation
  to time out into a **visible stop**, and the closed *outcome* set cannot name
  one. It is not a `Refused` — no reason covers a handoff timeout, and
  `EpochStale` is `SY-10.a`'s *mismatch*, a different fact — and not a `Blocked`,
  because §*Outcomes* scopes blocks to a transaction stopped part-way while
  `FN-25`'s two diagnoses are both about finish ownership. **It is shipped**:
  `one-live-driver-per-working-tree` says the driver "stops `blocked`" on a
  post-reap invalidation timeout. This is the one addition that would touch the
  **outcome** set rather than the reason set, which is a wider blast radius —
  `SY-14`'s exhaustive sweep runs through the same classifier the real actions
  use.
- **4 · `ONotEntered`.** `FN-01`'s first preflight member "produces no refusal at
  all" and the catalogue's closed set deliberately has no member for it — but a
  total action must return something, and if want-of-confirmation produced an
  *absent* transition then `FN-01.a` would be true by construction and
  unfalsifiable. Declared at `finish/README:3392`, which names no owner: **this
  item has no `formal-synthesis-k16` site and reaches you only through this
  body.**
- **5 · `W8WitnessTracked`.** `FN-13`'s stated witness is *a commit attempted
  while the witness is tracked, refused*, and no closed reason names a tracked
  witness. `finish.als` reports it under `WitnessPending` — the closest true
  statement the set admits — and keeps it distinguishable through `Sys.why`. The
  consequence is that **an operator cannot be told from the reason alone that the
  repository, not the filesystem, is what is blocking.**
- **6 · `FN-13` refuse-or-block.** `finish.als` **refuses**, following the
  catalogue "because the catalogue is the sole input to the formal phase";
  `finish.qnt` **blocks**, on the three-contexts rule. The two *documents*
  disagree too: `task-tree-transactions-fail-closed` says a tracked witness
  "keeps the witness unwalkable as **Recovery pending**" — a `Blocked` — and the
  catalogue says refused. **Note the coupling: resolving 6 as *block* may retire
  item 5 entirely**, since a block carries a diagnosis rather than a reason.
- **7 · `FN-10.b` / `FN-32` refuse-or-block.** `finish.als` refuses where
  `finish.qnt` blocks at the same step, on an unclassifiable artifact at a
  reserved name met inside a transaction, and **both are green** against text
  that says only *fails closed*. `FN-32` states only what both agree on (the
  artifact is not mutated). The catalogue's *one artifact, three contexts, one
  decided outcome* table now carries a `checked by` column and a second row that
  deliberately fixes no outcome — **that row is where this answer lands**, and
  `semantic-contract.md` names **this leaf by handle** there, so the row is
  addressed to you rather than to an ancestor. Quint's
  `wit_FN_32` is worded on the blocking half and **moves with the answer**.

**Two arguments the catalogue already supplies, and they point opposite ways.**
For *widening*: `finish.als`'s state-table member note observes that adding a
member is licensed in as many words — *`TT-18`/`TT-19` are stated over the
reserved CLASS rather than over its members so that removing one member changes
no claim*. Against: §*Outcomes* refuses a guard wait a member on the explicit
ground that *the set covers what a completed invocation returns*, and tells a
model needing one to declare an abstraction instead. Both are about closed sets
and they are not obviously reconcilable; deciding which governs is most of this
leaf.

## Done when

- Items 1 – 7 are each decided and landed in `docs/specs/semantic-contract.md`,
  and each is marked manifest-changing at the moment it is decided.
- **Both** families answer every new or changed obligation with a property
  command plus its required witnesses, or with that family's own declared gap.
- Every scope the decisions reach is green with coverage asserted under
  `models/run.sh --scope <scope> --family <family>`, with a run line recorded per
  scope. Expect finish and lifecycle at minimum; check whether the task-tree
  scope sweeps any set that moved before assuming it is untouched.
- `models/run-controls.sh` passes.
- `finish.als`, `finish.qnt`, `lifecycle.als`, `lifecycle.qnt` and their three
  READMEs say **in place** how each declared addition was disposed, so a later
  reader meeting the declaration finds the decision beside it. Every one of them
  currently names this leaf.
- The findings log carries an appended disposition beside each affected entry —
  appended, never rewritten.

## Notes

**Budget the runs rather than discovering them.** Alloy's finish cell is
14 m 33 s for 180 commands; Quint's is 4 m 25 s for 228; the lifecycle cells are
73 and 93 commands at ~4 m 27 s. The finish scope **can** afford
`QUINT_VERIFY=1` and the repository default stays 0 because the task-tree scope
cannot.

**A disposition is a decision about the contract, so the ADR test applies**
(`content/ADR-FORMAT.md`). These seven are the items on the whole disposition
list most likely to earn a record, because a later reader will want the **cost**
of widening a closed set — a matching outcome imposed on every column that
sweeps it — and not only the outcome. If the pattern paragraph above resolves
into a rule about what a refusal reason names, that rule is **one** record rather
than five.

**Sibling `routing-and-prose-k73` already landed a rule you will want.** The
catalogue's assumption table now states that *an exercise-removal row's controls
column is a claim of unreachability, to be established by running the removal
rather than by reading the witness*, and names the two different failures the
shape produces. Any new member that changes an assumption's reachability inherits
that obligation.
