# legacy-review-removal-review-k64

**Kind:** review-impl
**Reviews:** legacy-review-removal-k47
**Producer launch:** {"producer":"legacy-review-removal-k47","session":"relationship-contraction-k85","generation":"k85","harness":"claude","model":"opus"}

## Goal

Adversarially review `legacy-review-removal-k47` and record concrete findings for its integration step.

## Context

- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `legacy-review-removal-integrate-k65` owns every fix and
  all post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The review relies on the producer's recorded verification evidence; no test,
  build, lint, or format command is run.
- No production or test code is changed.

## Notes

## Findings

### F1 — Medium — the marker-surface “Git versus jj” control exercises the same untracked rename path twice

`tests/task_marker_surface.rs:180-212` initializes either Git or jj and then
writes the entire `.grove/` fixture, including the producer, without adding it
to Git's index. The two advertised cross-tree tests at
`tests/task_marker_surface.rs:293-303` therefore do not take different
promotion paths. For the Git fixture,
`tree_promotion::promote_new` calls `capture_git_index_entry` at
`src/tree_promotion.rs:137`, which returns `None` for an untracked producer
(`src/tree_rename.rs:69-75`), and `rename_entry` at
`src/tree_promotion.rs:176` selects `plain_rename` for that producer
(`src/tree_rename.rs:51-58`). The jj fixture selects the same `plain_rename`
branch. The test comment at `tests/task_marker_surface.rs:298-300` claims the
opposite and also reverses the production order: generated review/integration
steps are written at `src/tree_promotion.rs:164-174` before the producer move,
not afterwards.

This leaves `legacy-review-removal-k47`'s required cross-tree control
unestablished: both marker sweeps prove the filesystem-only shape, while the
tracked-Git `git mv` plus index-preparation path is outside this evidence. Make
the initial Git producer tracked before `generate_every_shape` (and positively
assert that fact, so the fixture cannot silently regress); keep the native-jj
`.git` absence assertion. Then the two tests actually cover the distinct paths
their names and the producer's recorded verification claim.

## Review coverage

- Inspected the aggregate committed diff for `review-routing-removal-k78`,
  `review-receipt-removal-k84`, and `relationship-contraction-k85`, including
  production call paths, changed tests, the binding ADRs/specs, and each
  commit's recorded gate evidence.
- Confirmed the removed routing target, structured peek, receipt types,
  generation helpers, and diversity comparison have no remaining source/test
  callers. The surviving relationship lookup preserves the prior exact-match,
  node-charter, cardinality, and tree-order behavior behind the smaller typed
  interface.
- Stale public methodology and durable-record claims are already assigned to
  `review-methodology-k87` and `architecture-records-reconciliation-k88`; they
  are planned later slices, not findings against this producer.
- Per this review leaf's inspection-only mandate, no test, build, lint, or
  format command was run, and no production or test file was changed.
