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

Budget for the inline fixture modules as source-book work: the `grove-loop`
manifest includes tests inside `task_name.rs`, `task_tree.rs` and
`tree_lifecycle.rs`; the separately excluded `task_grow/tests.rs` is different.
The parser, coupled consumers, fixtures and book reconstructions share this
completion boundary. No independently green split is established by this plan.
If the session cannot finish, seek a verifiable working seam before decomposing;
do not introduce a dual reader or retire a red child to manufacture a boundary.
If no such seam is available, record completed edits, remaining work and actual
check results in this leaf's running log, snapshot with `jj status`, and return
without retirement, sealing or `grove-llm complete`. The human re-runs the
installed `grove` in this workspace; this same live handle is selected again.
Resume from the running log and `jj diff`, preserving the unfinished change,
and seal only after the whole increment and its books pass.

## Decisions (running log)

**1. Implementation follows the reviewed grammar and supplied-name seam.**
`Parts::Node` carries no slug; `TaskName::NodeFile(Slug)` owns positioned-node
file titles and `TaskName::Brief` names the root file. `Handle::of_leaf` and
`Handle::of_node` reject invalid species pairings. Tree lookup composes node
identity using the actual distinguished entry in the same snapshot. Full
handles must match the current title; bare keys remain title-independent.
The existing required-level callback enforces cardinality and placement, with
conditional recovery advice in its missing-node-file error and no extra read.

**2. Verification sequence.** The canonical-name regression failed against
unchanged production code because `_BRIEF.md` was classified Foreign. Finish
name/handle and conformance tests, then tree/lifecycle consumers and every
workspace fixture, black-box contract regressions, source-book fragments and
explanatory prose, and finally `bash scripts/check.sh` plus the isolated verb
demonstration. The graph CLI refused startup due to an active incompatible
generation; source searches and exact reads provide the fallback evidence.

**3. Positive-key construction boundary (human decision).** The focused fresh
review found one contract issue: generic `Key::new(0)` can be passed through the
infallible constructor, but Grove’s parser rejects zero. The human chose to
document domain-valid positive-key inputs rather than redesign the generic API.
The naming ADR and Grove constructors now state that boundary. Parsing and
allocation enforce it on operational paths. The reviewer found no other
operational violation in node identity, guarded validation or promotion.

**4. Contract tests and isolated demonstration.** The new black-box node-file
suite exercises file-derived titles (including misleading bodies), stale
handles after a file rename, ambiguity handles, renumbering and retirement.
Thirteen malformed tree shapes are tried against nine read/mutation verbs;
all refuse before changing tree names or bytes. The heading regression checks
idempotence and custom-heading preservation. These focused tests pass.
An isolated jj fixture initialized through the built binary then decomposed
`plan-k1`, resolved its node, printed `_BRIEF.md` then `_plan.md`, renumbered
the node from `01-k1` to `02-k1`, checked unchanged file contents and retired
its child. `GROVE_SIGNAL_FILE` was cleared for the demonstration. The installed
binary and live tree remain on their existing grammar until cutover.

**5. Source-book reconciliation.** Changed roots belong to the `grove-loop`
and `grove-llm` books. Their literal fragments, ownership spans, manifests,
ledgers, behavior explanations and examples are updated, as is affected overview
prose. Both changed books pass final reconstruction: 13 roots / 10,458 lines
and 4 roots / 1,015 lines respectively, with zero deferred lines. The complete
principal check run passed, as recorded below.

**6. Final verification.** `bash scripts/check.sh` exited 0: all eight principal
checks pass, including formatting, shellcheck, clippy, plugin installation,
methodology conformance and its regression suite, the locked workspace tests,
and all six final source-book checks. The focused heading regression and the
isolated initialized-fixture demonstration also pass. All 1,734 recorded
tracked inputs outside this task tree remain unchanged through the final
verification tail. The task is a root child with live later siblings, so no
parent-node close or upward cascade is due.
