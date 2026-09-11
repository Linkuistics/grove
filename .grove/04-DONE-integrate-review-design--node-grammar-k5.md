# node-grammar-k5

**Integrates:** node-grammar-k4

## Goal

Triage the findings of the `review-design` leaf `node-grammar-k4` against the
node grammar design at `node-grammar-k2`, apply the ones that are real, and
leave the design set — ADRs, specs, both glossaries, the library architecture
and both models — current and coherent for `node-grammar-k3` to consume.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `usotvzzt` (`b795f2e3`) and to `path:line` coordinates in that tree, and no
  leaf has run between the review and this one, so the coordinates hold.
- The review questions the design was read against are in the same file, and
  the requirements are `plan-k1` and the root brief. The interview is complete;
  do not re-interview.
- Several findings are about the models' claim inventories. A change to a
  model is verified by its runner (`docs/ordinal-fs-tree/models/run-alloy.sh`,
  `run-quint.sh`), and a disagreement between a model and the design is
  recorded in `docs/formalism-findings.md`, as `node-grammar-k2` did.
- No file under `crates/` is part of this design change. Where a finding names
  source, it names it to show what the design has to account for, not to be
  edited here.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- The applied changes leave the ADR set a minimum coherent set, the specs and
  glossaries current, every anchor that existed still existing, and both model
  runners passing — or the disagreement recorded where the models say to
  record it.
- Nothing under `crates/` changed. `node-grammar-k3` still follows this leaf.

## Notes

- Substantial redesign is not this leaf's: externalise it as a new producer
  review chain beside this leaf rather than absorbing it.

## Decisions (running log)

**1. F1 — applied; the evidence contract was overstated.** `ReaderAccepts`
defines the three properties its checks repeat; remove those checks and label
the predicate as a definition. Keep the positive and refusal witnesses.
`beginOp` makes every `Refuse` effect-free by construction; remove
`inv_invalidLevelIsAtomic` and correct its citations. Preservation through
executed plans remains `inv_successHasValidLevels`, with its recorded mutation
control. The review was read from commit `31759c52`, not adopted from this
leaf's summary.

**2. F2 — applied; standing constructor witnesses are missing.** Add success
witnesses for append, batch, insert, rewrite and promotion with a child to
`required`, plus bare-node insert/batch refusals and distinct misplaced and
wrong-species promotion refusals. Initialization's existing with-distinguished
witness is not a with-positioned-child witness; add the latter explicitly.
The policy still does not distinguish arbitrary node parts, so that limit stays
visible rather than claiming that a node rewrite tests such a predicate.

**3. F3 — applied; clarify the content-write boundary.** Preserve the existing
Grove-side guarded, idempotent ` — brief` header rewrite after promotion. The
store creates the file and moves bytes verbatim; Grove subsequently edits its
heading, preserves custom headings, and does not roll back promotion if that
edit fails. No content callback is added to the library.

**4. F4 — applied; preserve recovery at the earlier refusal boundary.** For a
missing node file, Grove enriches the level error using listings under the same
guard: an empty node plus exactly one same-position, same-key sibling leaf
receives interrupted-decompose advice. Keep both paths and the canonical form;
offer removal of the empty directory or movement of the leaf to its named node
file. Never suggest adding a fresh key or manufacturing a brief. Other shapes
keep the ordinary level error. This is diagnostic wrapping, not a new reader.

**5. F5 — applied; root classification follows successful validation.** An
empty root or foreign-only root without its node file fails at open. Taskless
and Unrecognised both require a valid root node file; grammar errors precede
classification. Reconcile the ADR, glossary and architecture together.

**6. F6 — applied; the ADR owns the grammar.** Replace the spec's duplicate
grammar and token/cardinality rules with a citation, retaining how whole-tree
validation and name ownership affect consumers. No ADR is added or removed.

**7. F4 refinement — accept loss of automatic recognition visibly.** Further
source inspection shows `task_tree::open_write` receives an error only after
the library has dropped its guard. Decision 4's retained-guard diagnostic would
therefore add a public store affordance, beyond this repair. Instead, every
missing positioned-node-file error includes conditional interrupted-decompose
advice: check for an empty directory and a same-position, same-key sibling leaf
before creating a brief. The operator performs that check; the error does not
claim the shape was observed. This preserves actionable recovery without a
second reader, an unguarded listing or a new library interface. The loss of
automatic recognition is the accepted trade-off, recorded in the canonical
naming ADR and architecture; the alternative would need a producer/review chain
if exact diagnosis became a requirement.

**8. Repository and structural verification.** All eight checks in
`scripts/check.sh` pass, including workspace tests and final validation of all
six books. Alloy's 25 remaining commands pass. Quint typechecking and
ShellCheck on its runner pass. An explicit-anchor comparison against the
review commit preserves every anchor in the 47 design documents checked; a
deliberately missing anchor fails the comparison. Relative link targets in
changed design documents exist. No file under `crates/` changed. The graph CLI
could not start because of a generation conflict; cited source was read
directly instead. Quint simulation is still running at the standing budgets.

**9. Operational verification complete.** The Quint runner exits successfully
at its default 2,000 samples and 24 steps (12,000 samples for rollback failure).
Every claim row matches the runner's inventory, instance by instance; deleting
the bare-node insert witness from the comparison is detected. All 16 required
instance witnesses are reached, including node rewrite, insert, batch and the
two invalid promotion-name cases. The malformed instance reaches missing,
competing and misplaced-file refusals. SHA-256 digests of both models and both
runners match before and after the runs. These are sampled traces, not exhaustive
verification; the arbitrary-node-predicate and implementation limits in
formalism finding 049 remain. All findings are resolved by the repairs above
and F4's explicit accepted trade-off. Planning remains live, so no ancestor
closes with this leaf.
