# chain-node-review-k10

**Kind:** review-impl

## Goal

Adversarially review what `chain-node-k9` shipped: the two construction verbs
writing a chain node, their tests, and the prose reconciliation. Produce findings,
not fixes.

## Context

The artifact under review is `chain-node-k9`'s commit. The design it must satisfy:
`docs/specs/task-kind-taxonomy.md` § *A chain is a node directory*,
`docs/adr/task-tree-scheme.md` (two node species, charter discriminator),
`CONTEXT.md` §§ *Node directory*, *Review chain / vendor pair*.

This is a **flat** chain, so nothing links these leaves but adjacency — read
`chain-node-k9` for the full brief.

## Done when

Findings are recorded, each verified rather than asserted. Beyond the standard
`review-impl` read (correctness, security, tests, house conventions), four things
are specific to this change and are where a plausible-looking implementation most
easily goes wrong:

1. **Does anything key on the `-chain` / `-pair` slug token?** It must not — the
   discriminator is `BRIEF.md`'s presence. A name parse here reintroduces exactly
   the convention-reading the design forbids, and would pass every happy-path test.
2. **Is whole-shape-or-nothing actually preserved?** Try to construct a failure
   that leaves a node directory behind. The happy path proves nothing; the reason
   the verb exists is the partial state.
3. **Do the untouched verbs really cope with a brief-less node?** `brief-chain`,
   `pick`, `resolve`, `leaf-decompose` on a chain step, `leaf-prune` on a chain
   node, and the Retire cascade. The design *claims* they all do — check, do not
   take it on trust. A `leaf-decompose` of a chain step nests a brief-carrying node
   inside a brief-less one, which is the shape least likely to have been exercised.
4. **Is the prose reconciliation complete and non-parallel?** Old flat-shape
   guidance left standing beside new is the specific failure mode
   *compose-task-chains-k29* hit. Grep rather than trust the file list in
   `chain-node-k9`; it was written before the work and may be short.

## Notes

Verify the no-migration claim empirically: an existing flat chain — including the
one this leaf is part of — must still be walked correctly by `pick` after the
change.

## Findings

Review target: `bc1d33e411224576b3469ef755c2a3210ee291bf`
(`chain-node-k9`). The repository was indexed as
`Users-antony-Development-grove.using-codebase-memory` before symbol discovery.

### High — key overflow breaks whole-shape-or-nothing

`add_run` creates the node directory before deriving each child's key
(`src/tree_grow.rs:153-167`), and derives those keys with unchecked
`node_key + 1 + i` arithmetic. This leaves a mutation between the last
validation and an operation that can panic.

Reproduction: in an otherwise valid grove containing
`01-old-k4294967292.md`, run:

```text
grove-llm leaf-add-chain . sync --kind impl
```

The debug binary panicked at `src/tree_grow.rs:158` with `attempt to add with
overflow` and left this live partial shape behind:

```text
02-sync-chain-k4294967293/
  01-sync-k4294967294.md
  02-sync-review-k4294967295.md
```

That directly refutes both “validate and resolve before the first write” and
“anything that still fails mid-write rolls the run back”. Release arithmetic
wraps instead, producing `k0` and violating the four-fresh-consecutive-keys
contract. The four keys must be checked/precomputed before `create_dir`; an
exhausted keyspace should refuse with no filesystem mutation.

### Medium — the normative spec still carries pre-implementation claims

`docs/specs/task-kind-taxonomy.md` was only partly reconciled:

- Lines 378-379 and 491-495 say `leaf-add` is untouched, but this commit
  deliberately changes `resolve_parent_node` so `leaf-add` accepts a brief-less
  node.
- Lines 604-613 say the verbs “do not yet write” chain nodes and that
  `content/` / `docs/grove.md` still describe the flat shape; this commit makes
  the verbs write nodes and updates those surfaces.

These are current-state normative statements, not quoted history. They coexist
with the new node-shape text in the same spec, reproducing the parallel-guidance
failure this task explicitly required the implementation to remove.

### Medium — other authoritative prose still assumes every node has a brief

The reconciliation also missed readers outside the implementation leaf's file
list:

- `src/tree_id.rs:8-10` defines a node as “a directory holding BRIEF.md +
  children”, and lines 48-59 say every node directory holds exactly one
  `BRIEF.md`. This is the core ID-model commentary future tree readers are most
  likely to follow, and it now contradicts the two-species model.
- `docs/adr/pruning.md:29-34` describes the HITL guard as the cascade's “ask
  before treating a node as done”, without the new brief-carrying qualifier.
- `docs/workflows/multi-step.md:111-128` presents node-level retirement as
  universally asked rather than explicitly scoping the walkthrough to its
  brief-carrying decomposition node.

The first is a direct false invariant; the latter two can teach a future session
to ask on a brief-less chain node even though the skill now says it closes
silently.

## Confirmed checks

- No production reader keys on the `-chain` / `-pair` slug token. A graph-backed
  code search found those tokens only in constructors, diagnostics, and tests;
  searches for name predicates (`contains`, `ends_with`, `starts_with`,
  `strip_suffix`, `matches!`, `is_match`) found no classifier.
- A real brief-less chain-node fixture verified `pick`, `brief-chain`, `resolve`,
  `kind --with-harness`, nested `leaf-decompose`, `leaf-retire`, and
  `leaf-prune`. After decomposing the producer, `brief-chain` returned the root
  brief and nested decomposition brief while correctly skipping the chain-node
  level. Pruning the outer chain node left the nested `DONE` leaf untouched and
  marked both remaining steps `ABANDONED`; `pick` then reported no live leaves.
- The tree viewer rendered that finished brief-less chain node as one collapsed
  row with `1 done · 2 pruned` and required no name-specific change.
- The current grove itself confirms the no-migration decision: bootstrap `pick`
  walked the existing flat chain to `chain-node-review-k10` normally.
- `cargo test` passed in full (the sandboxed run's two Unix-socket permission
  failures disappeared when rerun with socket permission); `cargo fmt --check`
  and `cargo clippy --all-targets --all-features -- -D warnings` passed.

## Rejected candidate

A fresh-context review also flagged concurrent composite calls allocating the
same position and keys. That is not a defect under the recorded contract: ADR
*task-tree-scheme* explicitly defines a grove tree as single-worktree,
single-writer. The implementation's one-snapshot logic is correct only under that
documented assumption; no lock or cross-process reservation belongs in this
integration leaf.
