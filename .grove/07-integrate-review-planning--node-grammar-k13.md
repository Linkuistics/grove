# node-grammar-k13

**Integrates:** node-grammar-k12

## Goal

Triage the findings of the `review-planning` leaf `node-grammar-k12` against
the implementation plan at `node-grammar-k3`, apply the ones that are real, and
leave the root brief and the six `impl` bodies current and executable for
`distinguished-names-k6` to consume.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `tpoponvo` (`091abec6`) and to `path:line` coordinates in that tree. The
  review's own insert of this leaf shifted the implementation leaves from
  positions 07–12 to 08–13 afterwards; handles are unchanged.
- The review questions the plan was read against are in the same file. The
  requirements are `plan-k1` and the root brief; the design is
  `node-grammar-k2` as integrated at `node-grammar-k5`. The interview is
  complete; do not re-interview.
- The artifact is task-tree prose: the root brief and the leaf bodies. Nothing
  under `crates/`, `docs/` or `plugins/` is part of this change. Where a
  finding names source, it names it to show what a leaf body has to account
  for, not to be edited here.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- The applied changes leave each `impl` body naming its surface, its seams and
  the book it owes, the cutover body's handoff executable by the sessions and
  the human it names, and the migration body's inventory matching the machine.
- `distinguished-names-k6` still follows this leaf; positions and keys of the
  implementation leaves are otherwise untouched.

## Notes

- Substantial replanning is not this leaf's: externalise it as a new producer
  review chain beside this leaf rather than absorbing it.
