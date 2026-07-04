# plan-k1

**Kind:** planning

## Goal
Encode the **approved** design into the task tree, then hand off to execution.
This is an encode-and-decompose pass, **not** a fresh grill — the design is
already settled through a prior brainstorming session and captured in the spec.

## Context
Read `docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` (the mandate) and
the root `BRIEF.md`. The spec ends with a **proposed decomposition** into ~6
leaves — treat it as the starting shape, finalize it, and grow the tree with
`grove-llm leaf-add`. Re-open a decision only if a genuine contradiction surfaces.

Two-repo work: the `decision-records` skill lands in `../skills` (its own git
repo); all other edits in `grove`.

## Done when
- The tree is grown into the execution leaves (per the spec's decomposition):
  linkuistics skill, grove ADR note, grove process prose, corpus disposition
  (planning, with a human approval checkpoint before any delete/merge), corpus
  rework, citation reconcile — with sensible ordering/dependencies encoded.
- The spec is lifted into `docs/specs/` as the canonical mandate (already placed
  there) and committed with this planning task.

## Notes
Keep the corpus **disposition** (classify all 35 ADRs) as its own planning leaf
with a required human checkpoint — do not delete or merge ADRs without approval.
The citation-reconcile leaf runs *after* the corpus settles so slugs are final.
