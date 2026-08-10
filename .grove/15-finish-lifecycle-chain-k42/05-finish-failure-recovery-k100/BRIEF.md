# finish-failure-recovery-k100 — brief

## Goal

Make finish teardown recoverable when repository validation, index preparation,
staging, commit, rollback, or success cleanup is interrupted or fails.

## Context

- Surfaced while integrating `finish-lifecycle-review-k44` F5 and the narrow
  completion review of `finish-lifecycle-integrate-k45`: index state can be
  preserved, but the current caller deletes the task tree before entering the
  repository commit seam.
- Preserve path/fileset-scoped commits, unrelated Git/jj work, the universal
  tree lock, explicit finish confirmation, and the rule that a successful
  finish leaves no `.grove/` in the integrated history.
- Coordinate with `post-teardown-restart-k99`, which owns the distinct crash
  window after a successful deletion commit.
- The first in-session review of `finish-transaction-contract-k105` disproved
  the draft's finish-leaf proof, witness cleanup ordering, starting-revision
  proof, and hook-side-effect assumptions. Scheduled design review owns the
  required fresh re-review after integration.

## Done when

- A reviewed design states the transaction boundary and recovery behavior for
  every pre-commit, commit, rollback, and cleanup failure in plain Git, native
  jj, and colocated jj.
- A reported failure either restores a live, selectable finish tree or leaves a
  fail-closed recoverable witness with an actionable diagnostic; it never makes
  a failed finish look like a fresh rootless grove.
- The minimum coherent spec/ADR/glossary set records the settled contract.
- A separate reviewed implementation delivers the transaction, repository
  adapters, driver recovery, methodology, and Git/native-jj/colocated-jj
  regressions.

## Decomposition

- `finish-transaction-contract-chain`: settle, review, and integrate the
  transaction contract after the in-session disproof.
- `finish-transaction-implementation-chain`: implement, adversarially review,
  and integrate the settled contract.

## Notes

Do not fold this lifecycle transaction redesign into the index-preservation
fixes in `finish-lifecycle-integrate-k45`.

The reviewed design closed through `recovery-contract-finalize-k116`. The
binding spec, transaction and driver-ownership ADRs, glossary, and context map
now agree on guarded repository handoff, exact jj start/result proof,
attempt-bound rootless retry, descriptor-bound no-follow task-root access, and
operator recovery. `finish-transaction-implementation-k110` carries the
executable implementation and regression-test handoff.

`finish-transaction-implementation-k110` closed. The transaction ships as one
deep module — `finish_transaction`, with `finish_cleanup` owning post-commit
quarantine and VCS-administration auxiliaries: preflight, a `PREPARING-FINISH-`
witness published to `FINISHING-` by atomic rename, evacuation, the
three-disposition repository seam, rollback, quarantine handoff, driver
recovery, and the attempt-bound rootless retry. `content/SKILL.md`, `grove-llm
finish-commit` help, `docs/ARCHITECTURE.md`, `docs/USAGE.md`,
`docs/CONFIGURATION.md`, the ADR, the spec, and `CONTEXT.md` now describe that
behavior, and nothing tells a reader that an absent `.grove/` proves teardown
succeeded.

Scope boundary for `finish-transaction-implementation-review-k111`: the legacy
launch and review surfaces were deliberately left alone. `grove do`, `--harness`
flags, harness/model policy, producer launch receipts, and the `start.md` /
`retire.md` prompts still appear across `content/SKILL.md`, `docs/USAGE.md`,
`docs/CONFIGURATION.md`, and `docs/ARCHITECTURE.md`; they belong to
`legacy-launch-removal-k46`, `legacy-review-removal-chain-k62`,
`methodology-and-viewer-chain-k66`, and `durable-docs-reconciliation-chain-k70`.
Findings against those surfaces are not this review's.
