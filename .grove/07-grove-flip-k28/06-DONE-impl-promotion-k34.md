# promotion-k34

## Goal

Move `leaf-decompose` onto the library's `promote`. One verb, one leaf, because
promotion is the only operation whose intermediate state breaks an invariant and
the only one whose rollback can leave a tree in neither the state it was found in
nor the one intended.

## Context

- `docs/ordinal-fs-tree/ARCHITECTURE.md`, *Mutating*, *Promotion is not atomic
  against the invariants*, and *When rollback fails*. All three are short and all
  three matter here.
- `crates/ordinal-fs-tree/src/fs/mod.rs` — `WriteGuard::promote`, including its
  optional first child in the same unit.
- `crates/ordinal-fs-tree/tests/promoting_on_disk.rs` — the library's own
  coverage, and the nearest thing to a specification of the edge cases.
- `src/tree_lifecycle.rs::leaf_decompose`, and `grove-llm leaf-decompose --help`,
  which is the contract: leaf file → node directory with the **key preserved**,
  the body moved in as `BRIEF.md` with its `# <slug>-k<key>` header retitled
  ` — brief`, and a first child grown atomically so the node is never childless.
- Suites: `leaf_ops`, `session_kind_tree`, `composition_verbs`, `brief_chain`.

## Done when

- `leaf-decompose` runs through `promote`, with the first child created in the
  same unit rather than as a second operation — the library offers exactly that,
  and it is what *atomically growing a first child* already promises.
- The key is preserved, the ordinal is preserved, and the body lands verbatim as
  the node's distinguished child. The header retitling is grove's own edit to the
  bytes and stays grove's.
- The first child still inherits the decomposed leaf's kind unless `--kind`
  overrides it.
- The two promotion refusals grove can reach are handled and their messages are
  what `refusals-k30` decided: `PromoteNotLeaf`, and `PromotePartsNotNode` if
  grove can compose leaf parts for a node — check, because the syllabus could not
  and marked it unreachable.
- The whole suite passes; changed tests recorded in the node brief.

## Notes

**`PromoteNoDistinguished` is unreachable for grove and that is worth asserting
rather than assuming.** The trait's `distinguished()` returns `None` for a domain
with no distinguished child, and promotion is then refused rather than guessed
at. grove has `BRIEF.md`, so the refusal cannot fire — assert the fact the way
`docs/ordinal-fs-tree/CLI.md`'s table does, rather than leaving a reader to
wonder.

**The intermediate state is the thing to understand before writing code.** A
promotion creates the node before it can move the leaf's content into it, and the
node carries the leaf's own ordinal and key — that is what identity preservation
means — so between its two effects both are on disk sharing an ordinal and a key.
There is no ordering that avoids it. The invariants hold of **quiescent** trees,
and the lock is what makes that safe. A grove reader that runs without the guard
would see the intermediate state; check that none does.

**Rollback failure on this path is worse than untidy**, and the architecture says
so explicitly. Read *When rollback fails* before deciding what grove reports. If
grove's current `leaf_decompose` has a recovery story the library does not, that
difference is a finding for the node brief — and possibly for
`docs/formalism-findings.md`, since it is the kind of thing a model would have to
be asked about to notice.
