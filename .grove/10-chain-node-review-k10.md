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
