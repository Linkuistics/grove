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
