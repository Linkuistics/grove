# valid-levels-k16

## Goal

Enforce the reviewed distinguished-name level contract on every reader and
projected final plan, with independent expected-level conformance fixtures.

## Context

- The parent's supplied-name child lands the explicit initialization/promotion
  API, removes the factory, compares rendered distinguished identity and adapts
  every caller and book. `EntryName::validate_distinguished` is present but not
  called from readers or plans yet. Its default accepts absence.
- Follow `docs/ordinal-fs-tree/ARCHITECTURE.md`, the entry-name seam ADR, and
  formalism finding 049. Existing models establish only their stated bounded
  properties; node-parts policies require an executable Rust test.
- Inspect `fs/read.rs`, `fs/mod.rs`, `plan.rs`, `ops.rs`, `error.rs`,
  `conformance.rs` and their existing tests. The `TwoOverviews` conformance test
  demonstrates supplied names and preserved bytes; it does not prove validation.

## Done when

- A shared check invokes the domain on the complete direct distinguished set,
  preserving its grammar error and level path, then independently rejects more
  than one distinguished child with all competing renderings.
- Every complete listing is checked before exposure; every reachable subtree
  is checked before a snapshot or search answer, including malformed later
  subtrees despite an early match, under both guard modes.
- The same check validates projected final levels before effects for append,
  batch, insert, initialized root and initial entries, promoted node and optional
  child, and node rewrites. Required files, root/node placement and policies
  depending on node parts are all tested. Refusal leaves the tree unchanged;
  reported-failure rollback and interruption limits are preserved.
- Conformance accepts explicit root/node contexts and expected verdicts from
  fixtures for missing, single, competing, permuted and misplaced names. It
  distinguishes finite sample coverage from reader/planner enforcement.
- A domain with different allowed names per level successfully initializes and
  promotes, and refuses invalid reads and plans. A permissive domain still cannot
  expose competing names. Tests exercise readers and planners independently of
  the conformance kit, plus node-parts policies not established by the models.
- The library kit still runs over `TaskName`; Grove's current grammar and CLI
  fixtures pass. Its filename switch remains `node-files-k7`'s work.
- All changed source roots land with every affected book, ledger, index and
  explanatory paragraph; `bash scripts/check.sh` passes.

## Notes

This child completes the parent's remaining validation and conformance criteria.
Do not introduce a second domain seam, parse labels, read contents, change the
installed binary/plugin, or rename this live tree onto the new grammar. If the
contract conflicts with model evidence, record the discrepancy and use the
standing model runners before changing it. Judge whole-artifact review before
`node-files-k7`; any review/integration must stay ahead of that consumer.
