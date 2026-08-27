# task-tree-scope-k70


## Goal

Decide and land the six `TT-` scoped catalogue findings, with both families
answering every changed obligation and `models/run.sh --scope task-tree` green
with coverage asserted in both columns.

## Context

**Run first of the three scope children, and that is forced by the rule rather
than chosen.** `obligations-follow-context-not-artifact` orders the scopes
`grove-task-tree` → `grove-finish` → the joint, and its clause 1 means a
re-scope always moves an obligation **up**. So an item this session re-scopes
hands *forward* into `finish-scope-k71` or `lifecycle-scope-k72`, both still
live; nothing here may hand backward. The one instance in the record moves that
way — `TT-24.c`/`TT-24.d` became `FN-32`/`FN-21.c`.

**`closed-sets-k69` ran before this leaf and froze the vocabulary.** Read its
decisions in the node brief's running log before deciding anything here: three
of the six items below are about what a tree *classifies as*, and the stable
state table is `closed-sets-k69`'s.

**The cost asymmetry is real and it lands on this scope.** Alloy's task-tree
cell is **6888 s of CPU** for 103 commands against Quint's 1209 s for 111 — the
Quint model unrolls a tree walk to `MAX_DEPTH` on every transition and the Alloy
one does not. Budget the Alloy re-run rather than discovering it. `quint verify`
is not available in this scope at all: `base` exhausts a 4 GB JVM heap at
`--max-steps=3`, so **every `TT-` property here is established by bounded
randomized simulation and no green run is a proof over reachable states**.

**The six items, with the evidence each already has.** Numbering is the node
brief's item table.

- **11 · `TT-17` is false as worded** (`MN`). "The classification SHALL depend
  only on the format witness, never on any task entry's text" is contradicted by
  the catalogue's own `PartialScaffold`, which is defined by an exact closed
  subset of the root's **contents**. The model checks it over the
  Current/Legacy/Foreign decision only, declared as a narrowing.
  Site: `crates/grove-task-tree/models/README.md:1262`.
- **12 · `TT-15.a` is false under one tree** (`MN`). A current root with no live
  task and a foreign artifact at a reserved name classifies `CurrentSpent` and
  must report `Empty`, while `TT-24.b` requires that same tree to refuse
  `ReservedNameOccupied` and `TT-18` puts the refusal two stages ahead of
  anything the walk says. Both are the catalogue's; `TT-24.b` is the one whose
  whole purpose is to win. The model states the missing staging premise as
  `walkStageReached`. Site: `crates/grove-task-tree/models/task-tree.qnt:2063`.
- **13 · `TT-20`'s prohibition on `Legacy` is false of shipped behaviour**
  (`MC`). True in the model under one `hand-edit`/`foreign-write` during an open
  scaffold, and true of the product in the window after the charter and before
  the leaf. Retained as
  `witness_finding_a_world_write_during_an_open_scaffold_reaches_legacy` rather
  than excluded. **Entry 048 is explicit that it does not establish harm** — the
  legacy path refuses or migrates and does not silently complete somebody's
  scaffold — and equally explicit that the severity is this leaf's to assign.
  Sites: `task-tree/README:57`, `findings:8819`, `8822`.
- **20 · The shipped *ambiguous partial root scaffold* refusal has no state-table
  member** (`MC`), and entry 048 judges it **a better answer than either model
  gives**. The shipped diagnostic is at `src/tree_lifecycle.rs:474` and `538`;
  the finding is at `findings:8796`. This is the item where the product is ahead
  of the catalogue, so the disposition is to *record what ships*, not to invent.
- **21 · `PartialScaffold`: absence-of-everything-else vs presence-of-its-own**
  (`MC`). Under `EN-13` (a foreign entry at any name) plus one `crash` the two
  catalogue statements are inconsistent, and the interleaving is one action
  deep. In the product it is an interrupted `root-init` plus any stray file —
  an editor swap file, a `.DS_Store`, a partially-synced artifact — after which
  Grove reads its own interrupted work as somebody else's legacy tree. **The
  safety argument survives the change**: every value a completion writes is
  fixed in advance, and a foreign entry is not something completion writes.
  Site: `task-tree/README:1375`.
- **22 · The mid-flight block has no context row** (`MC`). An ordinary mutation
  that has already applied an effect and whose next create is no longer licensed
  is none of the three contexts §*Outcomes* fixes. `findings:7284` is explicit
  that a derived Rust test written *before* this decision would encode the
  model's least-wrong choice as though it were the contract, so decide before
  `handoff-audit-k66` reaches the seam work.

**Item 19 is `closed-sets-k69`'s, not this leaf's**, even though its subject is
`EN-11`'s `TT-24.b` row: it is a controls-table citation with no scope-local
model work, and it rides with the other manifest-neutral cross-scope items.

## Done when

- Items 11, 12, 13, 20, 21 and 22 are each decided and landed in
  `docs/specs/semantic-contract.md`, each marked manifest-neutral or
  manifest-changing at the moment it is decided.
- For every manifest-changing one, **both** families answer the new or changed
  obligation with a property command plus its required witnesses, or with that
  family's own declared gap.
- `models/run.sh --scope task-tree --family alloy` and `--family quint` are each
  green with coverage asserted, and both run lines are recorded here.
- `crates/grove-task-tree/models/README.md` says, in place, how each finding it
  recorded rather than fixed was disposed — so a later reader meeting the
  counterexample finds the decision beside it. Its *Narrowings and
  qualifications* block is the specific target.
- Anything re-scoped upward is written into `finish-scope-k71`'s or
  `lifecycle-scope-k72`'s body before this leaf retires.

## Notes

**A NINETEENTH ITEM ARRIVED FROM `closed-set-additions-k74`, AND IT COMES WITH A
FROZEN SET AND A RULE RATHER THAN AN OPEN QUESTION.** That leaf, sweeping the
closed sets, found a **fourth** instance of the shape it disposed and declined to
decide it because deciding it needs your items 20 and 21 settled first: **an
ordinary operation meeting a `PartialScaffold` has no reason in the catalogue's
closed refusal set.** `crates/grove-task-tree/models/task-tree.qnt` refuses
`WitnessPending(RPreparing)` as the least-wrong member while naming a reserved
witness that is not there, and the deviation is declared at `gateOutcome` and is
entry 044's finding.

What you inherit rather than re-derive:

- **The set is twenty and it is frozen.** `DeletionNotCommitted`,
  `ConfigurationInvalid` and `GenerationContended` were added; the closed
  *outcome* set and the two blocked diagnoses were not touched.
- **The rule for adding a member**, which is why this is no longer three children
  editing one list in ignorance of each other: *the set gains a member exactly
  when a scope asks a question no existing member names, and the member names the
  **question**, never the gate.* The rejected alternative — report under the
  closest true member and distinguish with a model-only observable — is what both
  families did twice and is argued against in
  [`a-refusal-leaves-nothing-standing`](../../../../docs/adr/a-refusal-leaves-nothing-standing.md),
  clause 2.
- **A member survives a redefinition of the state it reports on**, because it
  names the question and not the state's extension. So deciding items 20 and 21
  first costs this item nothing.

It is **manifest-changing in the cascade sense and manifest-neutral in the
count**: a closed-set member adds no obligation line, so `models/run.sh --list`
does not move, but every family command that classifies the state must answer it.
`closed-set-additions-k74` measured that distinction and its running log records
it.

**`TT-10` is deliberately not an item.** It sits in the same README block as 11
– 13 under one `formal-synthesis-k16` sentence, and the README argues it is a
statement qualification rather than a narrowing — `TT-10`'s own text is "no
algebraic refusal reaches an operator *from an ordinary argument*", so the check
is the claim rather than less than it. Recorded so that four bullets under one
sentence do not read as a lost item.

**A disposition is a decision about the contract, so the ADR test applies**
(`content/ADR-FORMAT.md`). Items 21 and 22 are the likeliest to earn a record: a
later reader will want the *cost* of the `PartialScaffold` redefinition, not
only its outcome.

## Decisions (running log)

### The six items are not six: 13, 20, 21 and the inherited nineteenth are one question with four faces

Read in the order the node brief numbers them, the items look independent. Read
against the shipped product they collapse: **every one of them is about a root
that has no format witness**, and the catalogue has exactly one state for that
condition where the product has three.

The product's own discriminator, read off
[`src/tree_lifecycle.rs`](../../../../src/tree_lifecycle.rs)'s
`recover_partial_root_init_unlocked` and corroborated by four shipped tests:

| present | shipped outcome |
|---|---|
| only entries a fresh scaffold writes, each byte-exact | complete the scaffold |
| a **root-init-exclusive** entry, byte-exact, **and** something else | **refuse** — *ambiguous partial root scaffold*, mutating nothing |
| nothing that proves root-init ran | fall through to the legacy path |

**Root-init-exclusive** is the load-bearing term and the product defines it by
what it refuses to treat as proof. `an_untouched_root_brief_does_not_hide_a_legacy_v2_tree`
puts a byte-exact `BRIEF.md` beside a legacy-v2 leaf and **migrates** — because
the charter body is derived from the working-tree name and an older Grove wrote
the same bytes, so a charter proves nothing. Only the reserved format temporary
and the canonical first `requirements` leaf at position 1 key 1 with the exact
template body are entries no other writer produces. That is `TT-24`'s
fail-closed ownership applied at the **root** grain rather than the entry grain,
and it is the same split §*Outcomes* already draws between `WitnessPending` (an
artifact at a reserved name Grove **can** prove is its own) and
`ReservedNameOccupied` (one it cannot classify at all).

So the disposition is one design and four landings, and item 20's instruction —
*record what ships, do not invent* — is what produced it.

### 21 · `PartialScaffold` gains a class, and BOTH proposed repairs are refused

`MC-cascade · MN-count.`

The model proposed defining `PartialScaffold` by the **presence** of the
scaffold's own entries *and ignoring entries outside the task grammar*
(`task-tree/README:1375`). Half of that is right and half is wrong, and the
shipped product is the referee.

- **Right**: falling through to `Legacy` when a stray entry lands beside Grove's
  own half-built scaffold is a defect. Grove reads its own interrupted work as
  somebody else's tree.
- **Wrong**: ignoring the stray entry. Completing a root whose contents Grove
  cannot fully account for is precisely the mutation-without-proof that
  `TT-24` forbids. The safety argument the finding invokes — *every value a
  completion writes is fixed in advance* — establishes that completing is safe
  **for the bytes it writes**, and says nothing about whether this root is
  Grove's to write into at all. Those are two different questions and the
  counterexample answers only the first.

The catalogue therefore keeps the absence clause and **adds a presence clause**,
as two independent tests doing two jobs — **corrected while landing to an
ordered three-way test; see *Three corrections made while landing* below**:

- **presence of a root-init-exclusive entry** separates `PartialScaffold` from
  `Legacy`;
- **presence of anything else** separates the two classes of `PartialScaffold`.

`PartialScaffold` becomes `PartialScaffold(class)`, `Exact` | `Ambiguous`,
mirroring `Reserved(class)` and `Malformed(reason)` — the state table's own
idiom for one classification with several sub-cases. `TT-18`/`TT-19` are already
stated over the reserved *class* "so that removing one member changes no claim";
stating `TT-20`'s prohibition over the scaffold class buys the same property.

### 20 · The shipped ambiguity refusal is that second class, and it needed no new row

`MC-cascade · MN-count.` Entry 048 judged the shipped refusal "a better answer
than either model gives" and this is where it lands: not as a twelfth state
beside `PartialScaffold`, but as its `Ambiguous` class. A separate row would
have asserted that the root is **not** a partial scaffold, which is false —
Grove can prove a root-init happened; what it cannot prove is that the root's
whole contents are its own.

### The nineteenth item · `ScaffoldIncomplete(class)`, and the class is the state's

`MC-cascade · MN-count.` Refusal reasons **20 → 21**.

Applying the frozen rule — *the set gains a member exactly when a scope asks a
question no existing member names, and the member names the question, never the
gate* ([`a-refusal-leaves-nothing-standing`](../../../../docs/adr/a-refusal-leaves-nothing-standing.md),
clause 2). The question an ordinary operation asks of a witnessless root is *did
this root's initialisation complete?* No member names it: `FormatLegacy` is
false and is the answer `TT-20` forbids, `FormatForeign` is false, `RootAbsent`
is false, and `WitnessPending` names a reserved witness that is not there —
which is why `task-tree.qnt`'s `gateOutcome` had to declare a deviation to say
anything at all.

**One member, parameterised, and the argument is the operator's.** Two members
was the live alternative and it fails the ADR's own operator test: an operator
told `ScaffoldIncomplete` about an *ambiguous* root would be pointed at a
completion Grove has already refused to run — a worse lie than no reason,
because it suggests Grove will write into a root it declined to touch. But a
second member was not needed to fix that, because the catalogue already has the
shape: `Reserved(ResClass)` → `WitnessPending(ResClass)` is one state class and
one parameterised reason. `PartialScaffold(ScaffoldClass)` →
`ScaffoldIncomplete(ScaffoldClass)` makes the vocabulary regular instead of
gaining a special case, and the operator reads the class rather than guessing
from the reason.

### 13 · `TT-20`'s `Legacy` prohibition is narrowed to the window where ownership is provable, and the severity is diagnostic

`MC-cascade · MN-count.`

Even with the new class, one window survives: **after the charter and before the
leaf**, with a world write. There, nothing root-init-exclusive has landed yet,
so no evidence distinguishes the root from a legacy tree, and `Legacy` is the
honest classification. Two repairs were considered and both refused:

- **Treat the charter as proof.** Refused — it is the exact thing
  `an_untouched_root_brief_does_not_hide_a_legacy_v2_tree` exists to prevent,
  and its failure mode is strictly worse: writing a format witness into somebody
  else's legacy tree.
- **Guard across the two phases.** Refused — `EN-06` grants only that
  *cooperating* processes are serialized, and the actor here is `EN-13`'s
  non-cooperating writer. A guard buys nothing against the writer that produces
  the counterexample.

So `TT-20`'s fourth conjunct is restated to what is true: never `Current(*)`
(unchanged, and the load-bearing half — it is what stops Grove completing
somebody's tree), and never `Legacy` **once a root-init-exclusive entry has
landed**.

**Severity, which entry 048 left explicitly to this leaf: a diagnostic defect,
not a safety one, and it gets worse rather than better.** No silent completion
happens in the window — the legacy path refuses or migrates. But the approved
breaking change removes migration, after which `Legacy` fails closed and the
operator is told to migrate a tree that is not legacy, by a command that no
longer exists, about a directory Grove itself created and then failed to
recognise. The repair that closes the window is a **product** change, one
reordering: write the reserved format temporary *first* rather than immediately
before its rename, after which every interrupted root-init is provably Grove's
from its first filesystem step. `tree_format::write_current_last` already
validates and reuses a pre-existing temporary, so the code anticipates it. Not
decided here — it is product-facing and the formal phase alters no product
behaviour — and **routed to `handoff-audit-k66`** with the other four
product-facing diagnostic questions (the node brief's item 36).

### 11 · `TT-17` splits, because it states two claims and one of them is false

`MC-cascade · MC-count (+1).`

"The classification SHALL depend only on the format witness, never on any task
entry's text" is false of the catalogue's own state table, and the redefinition
above makes it doubly so: the witnessless decision now reads a task entry's
**name and bytes** twice over. But the claim is not empty — its witness (*a
legacy tree whose slug text would otherwise read as a current kind*) is a real
hazard the product has a test for
(`a_legacy_v2_slug_beginning_with_requirements_is_not_partial_root_init`).

The split separates the true claim from the false one:

- **`TT-17.a`** — where a format witness exists, `Current`/`Foreign` is decided
  by its content alone. Unchanged in force; this is what both families already
  check as `formatDecision`/`familyOf`.
- **`TT-17.b`** — where none exists there is nothing to read, and the decision
  is made from **presence and bytes** compared against fixed expected values,
  never from a task entry's *parse*. Stated directionally, because that is what
  makes it checkable and what the hazard actually is: perturbing task-entry text
  can only move a root **out** of a scaffold classification, never into one.

The narrowing declared in both model READMEs is therefore retired rather than
carried: `TT-17` is no longer checked over less than its text, because the text
no longer says something false.

### 12 · `TT-15.a` gains the staging premise, and the reason is that classification and outcome are different functions

`MN both.`

A current root with no live task and a foreign artifact at a reserved name
classifies `Current(Spent)` and, by `TT-15.a` read literally, must report
`Empty`; `TT-24.b` requires that same tree to refuse. Both are the catalogue's.
The models resolved it with a `walkStageReached` guard and declared a narrowing.

The narrowing is correct and the catalogue owes the premise, because the two
statements are not about the same thing: **classification is a function of the
tree and the outcome is a function of the operation.** A tree may classify
`Current(Spent)` and still refuse every operation, which is exactly what
`TT-18`'s ordering is for. `TT-15` is about what a *completed observation*
reports, and an observation the gate refused never observed. The premise is
added to the obligation's text; no obligation is added, and both families'
existing guards become the catalogue's words rather than a declared narrowing.

### 22 · The mid-flight block is not a block, and the delegated boundary has said so all along

`MC-cascade · MN-count.`

The model returns `Blocked(OwnershipConflict)` for an ordinary mutation that has
applied an effect and then meets a name it cannot prove is its own
(`collisionOutcome`, reached in ~12% of its instance's traces), and records that
the catalogue's *one artifact, three contexts* table has no row for it.

**It has no row because it is not a case.** `crates/ordinal-fs-tree` — which
this contract names as the delegated boundary and "consumes rather than
restates" — applies every mutation through one interpreter that **unwinds what
it applied** on any reported error. Its two error variants are the distinction,
already drawn and already modelled:

- `Error::Failed` — "the run unwound everything it had applied. **The tree is as
  it was found**: this is *plan atomicity*, which `operations.qnt` checks as
  `inv_atomicity`". Under §*Outcomes*' discriminator that is a **`Refused`**.
- `Error::FailedPartiallyRolledBack` — "the tree is in neither the state it was
  found in nor the one intended … the one path by which this library damages a
  tree it was handed". That is a **`Blocked`**.

And the interpreter's destination check exists for *precisely* this scenario:
"the lock is advisory: a writer that does not take it can occupy a destination
between the snapshot and the apply, and that neighbour is the only thing left
that this check catches."

So the ordinary mid-flight collision is a **refusal**, reached by unwinding, and
the model models an interpreter Grove does not have. What the catalogue was
missing is not a context row but an **assumption**: it never granted plan
atomicity, so both families were free to invent an interpreter without it, and
one did.

**`EN-17` — a reported mutation failure unwinds every effect it applied.**
Class *premise-break*; the mutation already exists at the boundary
(`operations.qnt`'s `rollback_fails`, whose
`wit_partialRollbackLeavesADuplicateKey` is reached), and the controls column is
`TT-24.b`.

**`closed-set-additions-k74`'s conclusion survives and its reasoning does not.**
It scoped `FN-29.b` to `grove-finish` alone on the ground that "the task-tree
scope has no block to be distinguished from". A task-tree mutation *can* leave
an effect standing — when the unwind itself fails — so the premise is false. The
conclusion holds for a better reason: that outcome is the **delegated
boundary's**, reported verbatim rather than re-worded
([`CONTEXT-MAP.md`](../../../../CONTEXT-MAP.md), *the table is read at
runtime*), so the catalogue absorbs no member for it. **Blocked diagnoses stay
at 2**, and the reason is recorded rather than left as a coincidence.

### What is handed forward, and to whom

Nothing is re-scoped **upward** in the `obligations-follow-context-not-artifact`
sense — no `TT-` obligation moved to `FN-` or `SY-`. Two consequences travel
anyway and both are written into the receiving leaves' bodies before this one
retires:

- **`lifecycle-scope-k72`** — `SY-06.b` is a declared *cross-scope citation* of
  `TT-18`/`TT-20`, and clause 2 makes a citation carry the cited obligation's
  narrowings. Both narrowings it carries have changed. The citation's text is
  updated here, because leaving a stale citation in one document is worse than
  the split; the **cell and its run** are `k72`'s.
- **`handoff-audit-k66`** — the root-init phase-ordering repair, above.

### Three corrections made while landing, each caught by an instrument rather than by reading

Recorded because in all three the draft was plausible and the run was not.

- **The `PartialScaffold` definition is an *ordered three-way* test, not two
  independent ones.** The first entry above described the presence test as
  separating `PartialScaffold` from `Legacy` in general. Re-deriving it from
  `recover_partial_root_init_unlocked`'s actual branch order shows the
  exclusivity test gates **only** the ambiguous branch: a root holding nothing
  but the scaffold's own byte-exact entries is completed whether or not a
  root-init-exclusive one is among them, which is why a byte-exact charter alone
  is completed rather than migrated. The catalogue carries the branch order,
  and the asymmetry is argued rather than transcribed — positive proof is
  demanded exactly where it changes the answer, because refusing is a strong
  claim about a root that might not be Grove's at all.
- **`TT-17.b`'s first wording was refuted by Alloy in the first command run
  against it.** It read *no perturbation of a task entry's text moves a root
  into a scaffold classification*, and a rename is such a perturbation: a file
  already holding the scaffold's exact bytes under another name **becomes** the
  scaffold leaf when renamed to the scaffold leaf's name, correctly. Both
  columns now state the byte form — *a root none of whose entries carries the
  scaffold's own bytes is not a scaffold, however its entries are spelled* —
  which says what the claim is about without implying anything about renames.
  The refutation is recorded in the catalogue beside the claim.
- **`TT-20`'s mutation control reported no violation over 4000 traces, and the
  invariant was fine.** `mutant_scaffold_absence_only` was first written at
  `FOCUS = 4`, whose action menu is inserts only, so `root-init` never ran.
  A control that cannot reach the situation it mutates reads exactly like a
  control that broke nothing — which is the same silence the control exists to
  remove. At `FOCUS = 5` it fires in 1.5 s.

### Both dispositions that rest on a load-bearing mechanism carry a control that kills them

Two of the decided items are true **by construction** of each model's own code,
which is the shape [`obligations-follow-context-not-artifact`](../../../../docs/adr/obligations-follow-context-not-artifact.md)
records as a transcription at `TT-24.c`. Each therefore gained a `const` dial and
a mutant, so the disposition is a runnable claim rather than an assertion:

| dial | mutant | what dies | time to fire |
|---|---|---|---|
| `SCAFFOLD_AMBIGUITY_CLASSED = false` | `mutant_scaffold_absence_only` | `TT-20`'s `Legacy` prohibition — the pre-disposition definition reads Grove's own interrupted work as somebody else's legacy tree | 1.5 s |
| `MUTATION_UNWINDS = false` | `mutant_no_unwind` | `TT-24.b`'s second clause — the interpreter this model built for itself while `EN-17` was ungranted stops where it is, and its refusal leaves its own shifts standing | 0.1 s |

Neither control exists in the Alloy column and that is structural rather than an
omission: `task-tree.als` applies every mutation as a single atomic step, so it
has no mid-flight state to leave standing, and no dial to write the scaffold
mutation against. Declared in that column's README so its green is not read as
independent evidence of either disposition.

### The central premise was run, not read

Every argument above rests on one factual claim about the product: that
`recover_partial_root_init_unlocked` runs an ordered three-way test and that the
charter is deliberately excluded from the ownership proof. Entry 048 established
it by fixtures; this leaf re-established it by execution, because a premise
carried across four documents and two model families is worth more than a
reading.

```sh
cargo test --lib -- partial_scaffold legacy_v2 root_brief
```

**14 passed, 0 failed**, and the four that carry the claim are all four of the
shipped tests cited in the catalogue, the ADR and both model files:

| test | what it establishes |
|---|---|
| `extra_task_structure_makes_an_exact_partial_scaffold_ambiguous` | the ambiguity refusal exists and mutates nothing — the `Ambiguous` class |
| `an_untouched_root_brief_does_not_hide_a_legacy_v2_tree` | a byte-exact charter **migrates** — the charter exclusion, and the reason a charter is not root-init-exclusive |
| `a_legacy_v2_slug_beginning_with_requirements_is_not_partial_root_init` | a legacy slug reading as the current first leaf is evidence of nothing — `TT-17.b` |
| `a_differing_partial_scaffold_leaf_is_refused_without_writing_a_brief` | the comparison is over **bytes**, and a near miss refuses rather than completing |

This is also the answer to the in-session reviewer question `references/execute.md`
poses. The claim most worth an adversarial read was the one every other decision
here hangs from; running the four tests that would break if it were wrong is
stronger evidence than a fresh context reading the same prose, and it is what was
spent instead.

### Run lines

**Quint — `models/run.sh --scope task-tree --family quint`: exit 0.**
113 commands, `-- cells: 42 complete, 0 declared gaps, 0 empty, of 42`, 42
skipped model-checking properties under the default `QUINT_VERIFY=0`. The
runner's checksum was taken before the run and again after it and **matches**, so
no part of it overlapped an edit to its own instrument.

Every command this leaf added or changed, with its margin:

| command | result |
|---|---|
| `inv_TT_17a_the_format_decision_reads_no_task_entry_text` | holds |
| `wit_TT_17a_a_legacy_tree_whose_text_reads_as_current` | 1992 / 8000 |
| `inv_TT_17b_a_witnessless_decision_reads_bytes_not_only_names` | holds |
| `wit_TT_17b_a_scaffold_name_over_foreign_bytes_is_still_legacy` | 2015 / 8000 |
| `inv_TT_20_an_interrupted_scaffold_is_a_partial_scaffold` | holds |
| `wit_TT_20_an_interruption_before_the_witness_lands` | 64 / 8000 |
| `wit_TT_20_the_window_before_the_first_exclusive_entry_reaches_legacy` | 21 / 8000 |
| `wit_TT_20_a_concurrent_foreign_write_is_ambiguous_not_legacy` (`scenario_scaffold`) | 13 / 8000 |
| `inv_TT_24b_an_ordinary_operation_refuses_reserved_name_occupied` | holds |
| `wit_TT_24b_a_mid_flight_collision_unwinds_and_refuses` | 97 / 8000 |
| `inv_fail_MUT_TT_20_absence_only_reads_grove_s_own_work_as_legacy` | **violated, as the control requires** |
| `inv_fail_MUT_TT_24b_a_refusal_that_leaves_its_own_shifts_standing` | **violated, as the control requires** |

**`models/run-controls.sh`: exit 0** — `-- runner controls: 10 passed, 0 failed`,
8m 51s wall, run alone rather than under load.

**No wall-clock is quoted for the suites themselves.** The two families were run
concurrently on one sixteen-core host, so each was competing with the other.
Counts, coverage and exit status cannot be moved by contention; timings can, and
a figure measured under load would be compared against by a later reader as
though it were the model's cost.

**Alloy — `models/run.sh --scope task-tree --family alloy`: exit 0.**
107 commands, `-- cells: 42 complete, 0 declared gaps, 0 empty, of 42`,
1h 29m 57s wall / 5312s CPU on a 16-core host (the second half of it running
alone). Both new commands and both new witnesses landed:

| command | result |
|---|---|
| `TT_17a_format_is_decided_by_the_witness_content_alone` | no counterexample |
| `witness_TT_17a_a_legacy_tree_whose_entries_read_as_current_work` | instance found |
| `TT_17b_a_witnessless_decision_reads_bytes_not_only_names` | no counterexample |
| `witness_TT_17b_a_scaffold_name_over_foreign_bytes_is_still_legacy` | instance found |
| `TT_20_the_format_witness_lands_last` | no counterexample |
| `witness_TT_20_a_world_write_during_an_open_scaffold_is_ambiguous_not_legacy` | instance found — the disposed finding, now the claim's witness |
| `witness_TT_20_the_window_before_the_first_exclusive_entry_reaches_legacy` | instance found — the surviving window, runnable rather than declared |

**One catalogue edit was made after both runs, and it is established
manifest-neutral rather than asserted.** `TT-20`'s sentence read *SHALL classify
as `PartialScaffold(_)` and never as `Current(*)`; and once any
root-init-exclusive entry has landed it SHALL never classify as `Legacy`* — whose
first clause contradicts the window the second clause concedes. It now reads
*SHALL never classify as `Current(*)`; and once any root-init-exclusive entry has
landed it SHALL classify as `PartialScaffold(_)` and never as `Legacy`*, which is
exactly what both families check. The catalogue **is** the runner's manifest, so
the edit was deferred until both runs had finished rather than made under them,
and `models/run.sh --list` prints `130 obligations` before and after with 42 `TT-`
rows either way — so no obligation moved and both coverage assertions still
stand.
