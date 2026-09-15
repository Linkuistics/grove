# captured-tree-k20

## Goal
Move guarded tree/file capture into grove-loop and migrate the existing viewer,
retaining a directory pin without an advisory lock.

## Done when
- try_observe returns a typed captured tree, vacancy or contention; errors remain
  distinct. A captured tree exposes its snapshot, root path, selected key/bytes
  and opaque TreeLifetime. The tree guard is released before returning.
- The viewer derives presentation rows and performs two-capture acceptance;
  production selected-file reads and lifetime implementation belong to the loop.
- Tests establish selected leaf/branch/root bytes, fallback selection, malformed
  refusal, contention, retained lifetime across replacement and no escaped lock.
- Source-derived books and seam documentation match; focused tests and the
  principal gate pass.

## Implementation plan
1. Add public-seam capture tests and observe failure for the absent operation.
2. Add ReadGuard::into_snapshot so captured names can outlive their tree lock;
   implement loop capture using this value and the existing directory-pin logic.
3. Migrate viewer capture; keep lifecycle aggregation, folds and consistency in
   grove-tui. Run existing public Viewer regressions.
4. Repair source fragments/indexes and describe the shipped tree-only stage.
   Run focused tests, then bash scripts/check.sh; retire and seal this child.

## Decisions (running log)
- The full typed-observation contract splits at capture versus runtime evidence.
  This child ships a consumed tree-only try_observe operation; bounded-runtime
  extends it with independent activity results. No fake Idle or Running result.
- Snapshot currently cannot leave ReadGuard. Add a consuming into_snapshot
  operation to the generic tree seam instead of cloning or reparsing names.
- The consuming snapshot seam releases the reader descriptor; an independent
  exclusive-lock assertion fails under a deliberate descriptor-leak mutation
  and passes after restoration. Existing Viewer tests cover the migration.
- The full typed-observation review remains owned by bounded-runtime-k21 after
  runtime evidence lands; no competing in-session review is commissioned.
- The book-validator fixtures also pin the generic filesystem module's source
  ranges and coverage totals; update those with the new consuming accessor.
