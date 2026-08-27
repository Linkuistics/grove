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
