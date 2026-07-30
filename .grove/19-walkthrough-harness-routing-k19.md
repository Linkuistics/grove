# walkthrough-harness-routing-k19

**Kind:** impl

## Goal

Reconcile the Codex-harness paragraph in `docs/workflows/multi-step.md` with the
current per-leaf/per-kind harness routing contract.

## Context

The walkthrough currently says the harness exec'd for the loop is whichever was
chosen at bootstrap time and recorded in the grove's stamp. That was the old
contract. Current routing is leaf declaration → exact kind → kind family →
stamp; a review, integration or vendor-pair leaf may therefore run on another
harness while the same `grove do` loop is alive (ADR *model-per-task-kind* and
`CONTEXT.md` § Kind routing).

Surfaced by `confirmation-prose-review-k16` while cold-reading the walkthrough;
kept separate because it is not part of the confirmation-boundary decision.

## Done when

- The walkthrough describes the stamp as the harness fallback, not a binding
  that necessarily runs every task.
- It says that the harness may vary by leaf/kind without changing the on-disk
  tree or CLI cadence the walkthrough demonstrates.
- Any generated or nearby routing claim is checked for the same obsolete
  bootstrap-only model; the existing routing tests pass.

## Notes
