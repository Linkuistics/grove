# library-k6

## Goal

Decompose increment 1 — `ordinal-fs-tree` and its CLI, standing alone — into
leaves, now that the design has landed and the operation set is fixed.

## Context

`architecture-k2` closed with the operation set settled and both models green.
The root brief's horizon item *"Implementing the operations, each modelled
first. Cannot be leafed until the design lands"* is therefore unblocked, and
this is the leaf that discharges it. `operations-model-k4` did not cut those
leaves itself: a `design` session cutting `impl` leaves has drifted into
planning's job.

Read first, in this order:

- `docs/ordinal-fs-tree/ARCHITECTURE.md` — the specification of record for
  everything the models do not cover, and the explanation for everything they
  do.
- `docs/ordinal-fs-tree/models/structure.als` and `operations.qnt`, with
  `run-alloy.sh` and `run-quint.sh`. **The models lead**: where a model and a
  test disagree, the model wins and the test changes. `operations.qnt`'s
  closing handoff block states what it does and does not cover, and what
  changing it obliges.
- `docs/formalism-findings.md`, entries 001–003 — in particular the misses,
  which name what the models did *not* establish and therefore what the
  implementation still has to get right unaided.

## What the decomposition has to account for

Not a decomposition — inputs to one. The root brief's constraint is that the
algebra stays free of `std::fs`, enforced by a test rather than by convention.

- **The seam and the name type** come before anything that uses them, and the
  five trait obligations are the consumer's, unchecked by the library.
- **The five mutations** — `append`, `append_many`, `insert`, `promote`,
  `rewrite` — each already have a modelled plan, a modelled refusal set and a
  two-state property. That is what "one leaf per operation" was waiting for.
- **The plan interpreter** is one leaf's worth on its own, and it is where the
  atomicity and rollback claims live. Note that `operations.qnt` shows a failed
  *rollback* is the one path by which the library damages a tree, and the
  document now states the recovery.
- **The reading operations**, the lock, and the crate skeleton.
- **The CLI's own shape** is a root-brief horizon item still, and whether it is
  in this increment's decomposition or its own is part of what this leaf
  decides.
- **H3 is untested and needs a deliberate test, not an impression.** Whether a
  checked model actually drives an implementation better than prose is the
  least certain of the three hypotheses. The decomposition is the only place
  that test can be designed in — deciding it afterwards means the evidence is
  already contaminated.

## Done when

- Increment 1's work is cut as leaves under the grove root, each a vertical
  slice that can be verified on its own.
- The root brief's horizon is reconciled: items this decomposition graduates
  are removed from it, and anything still too dim to phrase stays.
- How H3 gets tested is decided and written into whichever leaves carry it.

## Notes

The ~130 existing CLI-contract tests are regression cover for the *flip*
increment, not assurance for this one — this increment has no consumer to
constrain it, which is why the models are load-bearing. Do not plan around
adapting them here.
