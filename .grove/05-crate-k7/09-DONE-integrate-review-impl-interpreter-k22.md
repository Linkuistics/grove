# interpreter-k22

**Integrates:** interpreter-k21

## Goal

Integrate all five actionable findings from `interpreter-k21` before `insert`,
`promote`, `rewrite` and the CLI build on the plan interpreter. Close the name
confinement hole, make the model-required no-op move succeed, make reports honest
about effect order, and add the two missing mechanism-level controls for
atomicity and destination claiming.

## Context

Beyond the brief chain and its *Read first* list:

- `interpreter-k21` in full. Its `path:line` coordinates are against producer
  commit `0bd965a5`; no later implementation leaf intervened.
- `interpreter-k10`, the reviewed producer, and
  `docs/formalism-findings.md` entries 009–010. Entry 010's counterfactual is the
  integration worklist: follow one predicate across the next boundary and count
  mechanisms again there.
- `operations.qnt` predicates `wit_rewriteToSameParts`, `inv_atomicity`,
  `inv_interpreterNeverFindsADestinationTaken` and the handoff's explicit
  exclusions of strings, bytes and the filesystem.
- `ARCHITECTURE.md` on the name/triple isomorphism, exclusive destination
  claims, report-as-surface and the rule that prose owns cases the models cannot
  pose. If confinement becomes a seventh `EntryName` obligation, reconcile the
  architecture, trait docs and conformance kit as one contract; if it becomes a
  library refusal, state that instead. Do not leave it implicit.

## Done when

- No `EntryName` rendering used by a filesystem effect can be absolute, `.`,
  `..`, contain a path separator, or otherwise name more than one normal path
  component. A deliberately adversarial domain that satisfies the other six
  obligations cannot create, move, report or roll back outside the supplied
  root. The architecture and public error/obligation surface say where this is
  enforced and provide actionable recovery advice.
- A same-path `MoveTo` proceeds through both the algebra and interpreter, so the
  future `rewrite` of an entry to its existing parts satisfies
  `wit_rewriteToSameParts`. A distinct occupied rename destination is still
  refused and preserved. Cover the end-to-end hand-built plan, not only
  `Plan::guarded`.
- `Report::paths()` either returns paths in exact effect-landing order for
  `MoveTo…Create` and `Create…MoveTo…Create` plans, or its public contract is
  deliberately narrowed and every consumer-facing document agrees. Preserve
  `created()` and `renamed()` order within their own species.
- The internal failure seam reaches the interval after a leaf destination is
  exclusively created and before its content write completes. The resulting
  error removes the partial file and is `Error::Failed`, and a mutation control
  that moves undo registration after the failing write is caught.
- Node creation has its own uncooperative-neighbour control. Occupy a node
  destination after the snapshot; require `AlreadyExists`, preserve the
  neighbour directory, and ensure a later unwind cannot remove it. A mutation
  from exclusive `create_dir` semantics to idempotent `create_dir_all` semantics
  is caught.
- Re-read all forty-two producer tests against their predicates after the fixes;
  adjust the twenty-eight/fourteen claim account if new controls change it.
- Re-run both model suites, the crate and grove test suites, formatting and
  workspace clippy after integration. Append this episode to
  `docs/formalism-findings.md` before retiring.

## Notes

The review accepted four architectural judgements outright: consuming the
write guard, splitting forward effects from undo, landing `MoveTo` and
`Level::Created` in the substrate, and deriving singleton append from
`append_many`. It also accepted the look-then-rename race only within the stated
boundary: a writer arriving after the look is concurrent mutation during apply
and outside the advisory-lock contract. Do not widen this leaf into portable
atomic no-replace rename design.

Codebase-memory could not index the review workspace because active-daemon
coordination was unavailable, and the existing grove graph predates this crate.
Retry coverage if the integration environment permits it, but use
`interpreter-k21`'s complete direct-source citations as the authoritative
handoff rather than re-deriving findings from an empty graph.
