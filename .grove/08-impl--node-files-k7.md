# node-files-k7

## Goal

Make Grove read and write `NN-k<key>/_<slug>.md`, with `_BRIEF.md` at the
root, through its complete name, tree and verb interfaces.

## Context

- Depends on `distinguished-names-k6`. The canonical naming ADR and
  `docs/specs/module-decomposition.md` decisions 3 and 4 own the contract.
  Read `node-grammar-k5` decisions 3, 5 and 7 for content rewriting, root
  classification and the accepted interrupted-decompose diagnostic limit.
- Primary surface: `TaskName`, `Handle`, `task_tree`, `task_grow`,
  `tree_lifecycle`, CLI help and renderers, plus every Grove fixture consumer
  across the workspace. These pointers are entry points, not an exhaustive
  file list; follow callers and fixture builders before making the cut.

## Done when

- Node parts are slugless, and parsed node files own their slugs. The name
  module owns the shared key grammar and `Handle::of_leaf` / `Handle::of_node`;
  invalid species pairs have no handle. The tree module supplies the actual
  node file from the same guarded snapshot. Nothing caches a title in node
  parts, parses it from a directory path or reads it from a body.
- Only the canonical grammar is accepted. Every directory requires exactly
  one regular node file in the correct root/node position. Malformed `_` and
  digit-prefixed entries are owned refusals. Missing, competing, misplaced,
  wrong-species and noncanonical names fail with the path, offending names
  where present and the required form. A later malformed subtree cannot hide
  behind an early `pick` or `resolve` match.
- `resolve` by key, full handle and slug finds nodes using their files and
  returns the directory; ambiguous slugs show candidate handles and a node file
  contributes no extra match. `brief-chain` returns root-first node files;
  `pick` returns live leaves only; `kind` has no node/node-file kind.
- `root-init` creates the root file and first leaf in one guarded plan.
  `leaf-decompose` preserves the key and body while supplying the named node
  file and first child in its plan. Its subsequent guarded heading update is
  idempotent, preserves custom headings and does not claim to roll back a
  successful promotion if the content edit fails.
- A missing positioned-node-file diagnostic gives conditional advice to check
  for an interrupted decomposition: an empty directory plus a same-position,
  same-key sibling leaf. It neither claims to have observed that sibling nor
  performs an unguarded second read. Recovery retains the key and leaf body.
  Root classification follows validation: Taskless and Unrecognised both have
  valid root files; an absent file is refused before classification.
- Grow, insert, retire, prune and finish/lifecycle consumers retain their
  existing leaf, allocation, outcome, ordering and lock behavior. A node move
  carries its named file. All workspace fixtures use the grammar expected by
  this build, including driver and finish fixtures outside `grove-loop`.
- The agreed seams pass: `TaskName` grammar/handle tests and its library
  conformance suite; `grove-llm` black-box fixture tests for the behaviors
  above, malformed-tree nonmutation and body-independent titles; the relevant
  lifecycle/lock regressions; then `bash scripts/check.sh`.
- Changed source roots and their explanatory prose land with the complete book
  updates in this commit. Expect `grove-loop` and `grove-llm`; inspect every
  manifest for additional ownership. Update overview prose that would otherwise
  contradict this increment. No book is left red for a later leaf.

## Notes

Demonstrate with an isolated initialized fixture: decompose, resolve the node,
print its brief chain, renumber it, and retire its child; malformed fixtures
refuse before changing anything. Invoke the built `grove-llm` directly, clearing
`GROVE_SIGNAL_FILE` for probes outside this session's own tree. Never launch the
agent-facing binary through `cargo run`.

This live `.grove/` and the installed binary/plugin stay as they are until
`node-cutover-k10`. Use the installed binary for this leaf's bookkeeping.
No migration command or alternate grammar ships. No naming history is added
to docs or comments. Required review work belongs before `node-methodology-k8`.
