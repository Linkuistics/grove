# finish-lifecycle-integrate-k45

**Kind:** integrate-review-impl
**Integrates:** finish-lifecycle-review-k44

## Goal

Apply the verified findings from `finish-lifecycle-review-k44` while preserving the reviewed artifact's contract.

## Context

- Verify every `finish-lifecycle-review-k44` finding against the spec and the
  human-confirmation boundary.
- Keep branch/bookmark integration and working-tree removal out of Grove.

## Done when

- Every finding has a recorded disposition; verified issues are fixed with
  isolated Git/jj finish regressions.
- Finish remains a normal configured session plus one deterministic guarded
  teardown helper, with `complete --done` last.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Substantial lifecycle redesign is new work inside
`finish-lifecycle-chain-k42`, not cleanup to absorb here.

## Finding dispositions

- **F1 — valid, fixed.** Colocated-jj finish now prepares and validates a
  `.grove/`-free success index before the irreversible jj commit, then activates
  it only after success; the untouched backup restores command failures. Thus
  unrelated staged state survives without re-staging the deleted task tree.
  `colocated_jj_finish_commit_preserves_unrelated_work_and_the_git_index`
  carries a staged blob distinct from its working-copy content and covers both
  properties.
- **F2 — valid, fixed.** `finish-commit` now refuses migration/promotion
  witnesses and unsupported `FORMAT` values under the already-held exclusive
  lifecycle lock, before selection or deletion. The refusal tests assert the
  tree and `HEAD` remain byte-identical.
- **F3 — valid, fixed at the reviewed seam.** Retrying the helper after deletion
  reports `this grove is already finished` instead of a raw `read_dir` error.
  The broader driver-restart ambiguity is separate lifecycle design and was
  externalized as `post-teardown-restart-k99`.
- **F4 — noise under current behavior.** `vcs_of` has already fixed the jj
  workspace before command construction, and jj 0.44 exhibits no behavior
  change from the inherited `GIT_WORK_TREE`. Changing the environment without
  an observable contract or regression would be a testless consistency edit.
- **F5 — valid, fixed.** Plain-Git finish snapshots the pre-existing index,
  discards the snapshot only after a successful path-scoped commit, and restores
  it when staging or commit fails. A rejecting pre-commit hook proves the
  failure path preserves the complete prior index. The wider task-tree recovery
  problem is lifecycle redesign externalized as `finish-failure-recovery-k100`.

## Narrow completion-review dispositions

- **Post-commit index cleanup could strand unrelated staged state — valid,
  fixed.** The success image is now sanitized before `jj commit`; a forced
  alternate-index lock failure proves refusal happens before the finish commit
  and preserves unrelated staged entries.
- **The F1 fixture did not distinguish staged state from jj's normal export —
  valid, fixed.** The colocated fixture now stages one blob, leaves different
  working-copy content at the same path, and asserts the exact staged entry
  survives.
