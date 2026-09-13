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

## Decisions (running log)

1. Follow the reviewed design without reopening the interview. Graph startup
   refuses due to an incompatible active generation; use direct source reads.
2. Keep domain level errors generic and path-bearing, separate from algebraic
   `Refusal`. Share the domain-first/cardinality-second check between the full
   snapshot reader and final plan projection. Projection retains entry identity
   while applying name and parent changes; it checks all final node levels,
   including shifted and rewritten names. Initialization checks before creating
   the root. No interpreter rollback or interruption semantics change.

3. The conformance API receives explicit `LevelSample` fixtures in addition to
   name and triple samples. It repeats verdicts and samples rotations/reversals,
   reporting missing root/node contexts; this is finite evidence, not exhaustive
   permutation coverage. Expected acceptance is supplied by the fixture, and
   remains separate from the library's cardinality rejection.
4. Final projection retains snapshot and created-effect identities separately,
   updates moved names/parents, and rebuilds levels from indexed child lists.
   Sibling shifts therefore recheck the final containing name too. Domain level
   errors use `InvalidLevel`; permissive competition uses `CompetingDistinguished`.
5. Synchronize the book-validator's fixed source inventory, expected totals and
   corruption fixtures with the changed source ranges. Its production code is
   unchanged; its complete suite passes, including the controls that initially
   stopped mutating their intended rows when the recorded line counts moved.
6. Reader, planner and permutation bypass controls each fail their targeted test.
   Each source file is restored byte-for-byte after its control. Whole-artifact
   review is earned for projection identity and enforcement coverage before the
   grammar consumer; cut it beside this producer, keeping the parent live.

## Implementation plan

- Add public-boundary regression fixtures for required root/node names,
  permissive competition, malformed later subtrees and every constructor/rewrite;
  observe the missing enforcement failures before implementing the shared check.
- Add the shared check and path-bearing errors; validate each read listing and
  final plan projection at apply, with initialization preflight before creation.
- Extend conformance with explicit expected-level fixtures and permutation checks;
  exercise the kit over the reference domain and TaskName, keeping finite-sample
  claims separate from enforcement.
- Synchronize source fragments, ledgers, indexes and explanations in affected
  books; run the focused Rust suites then `bash scripts/check.sh` on frozen inputs.
- Judge review before the grammar consumer, retire and seal the focused change,
  then signal completion as the last action.

## Verification

- The preserved reader, cardinality and mutation fixtures failed before the
  enforcement was added. The deliberately wrong expected-level fixture also
  failed before its check was implemented. The completed public-boundary suite
  passes, including node-parts and shifted-ordinal policies.
- Temporarily bypassing reader validation fails the malformed-later-subtree
  test; bypassing plan validation fails the pre-effect cardinality test; removing
  the kit's permutations fails the order-dependent-domain test. Each mutated
  source was restored byte-for-byte before final verification.
- `bash scripts/check.sh` exits 0 with all eight principal checks passing:
  formatting, shellcheck, clippy, plugin installation, methodology conformance
  and its controls, locked workspace tests, and final reconstruction of all six
  books. The complete book-validation suite also passes after its fixed source
  inventory and corruption fixtures were synchronized.
- SHA-256 inventories of all non-ignored repository files, including hidden
  task files, sources, fixtures, scripts, manifests and books, match before and
  after the final run. This verification record and retirement bookkeeping
  follow that run.
- `valid-levels-k18` reviews this enforcement boundary before `node-files-k7`.
  No in-session reviewer was used. The parent remains live for that review;
  no parent-chain close is due. Installed binaries, skills and tree grammar
  remain untouched.
