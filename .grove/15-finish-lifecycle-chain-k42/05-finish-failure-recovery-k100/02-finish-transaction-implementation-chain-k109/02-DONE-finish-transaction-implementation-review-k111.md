# finish-transaction-implementation-review-k111

**Kind:** review-impl
**Reviews:** finish-transaction-implementation-k110
**Producer launch:** {"producer":"finish-transaction-implementation-k110","session":"finish-teardown-docs-acceptance-k164","generation":"k164","harness":"claude","model":"opus"}

## Goal

Adversarially review `finish-transaction-implementation-k110` and record concrete findings for its integration step.

## Context

- Review against the integrated contract from
  `finish-transaction-contract-integrate-k108`, not the producer's comments.
- Attack every phase boundary, ambiguous VCS result, anchor mismatch, witness or
  quarantine collision, partial rollback, index backup/restore/activation,
  symlink traversal, Git hook execution, jj successor topology, unrelated-work
  consumption, driver recovery ordering, and accidental rootless finish
  inference.
- Inspect the producer's committed diff and recorded verification. Produce
  findings only; fixes and reruns belong to the integration leaf.

## Done when

- Findings cite exact source/test/design locations, severity, and a reproducer,
  or explicitly record no findings after inspecting all three VCS adapters.
- The review checks that every public/internal interface is as small as the
  design claims and no caller reimplements repository outcome classification.
- No production, test, methodology, or durable design artifact is changed.

## Notes

## Findings

### F1 — High — plain-Git exact-result proof accepts a merge commit

`src/repo/finish_commit.rs:1514` (`validate_exact_git_finish`) asks
`git rev-parse HEAD^` for only the first parent and accepts the result when that
parent is the recorded start. It never proves that `HEAD` has exactly one
parent. The otherwise parallel rootless classifier at
`src/repo/finish_commit.rs:1408` enumerates the lineage and explicitly refuses a
second parent, so the two plain-Git proof paths disagree on what can be a finish
teardown commit.

This reaches the destructive boundary in both paths that use
`GitFinishProof::revalidate`: synchronous command-result classification and
`recover_plain_git_finish`. On recovery, the accepted proof flows through
`finish_transaction::resolve_pending` at `src/finish_transaction.rs:309` into
`quarantine_and_dispose_with_checkpoint`, so a witnessed live task tree can be
quarantined and disposed even though the repository topology is ambiguous
rather than the exact single-parent result the adapter produced.

Reproducer: let `A` be the recorded start with a tracked `.grove/`; construct a
commit `M` with the exact handle-and-attempt finish message, a tree equal to
`A` except for `.grove/` deletion, and parents `A` and any `B` (for example with
`git commit-tree <tree> -p A -p B`), then move `HEAD` to `M` before recovery.
`HEAD^` is `A`, the message and deletion fingerprint match, `.grove/` is absent,
and `git diff A M` contains only `.grove/` deletions, so every current predicate
passes. The second parent is never observed. This must remain `Recovery
pending`, not be classified as committed.

## Review coverage

- Inspected plain Git, native jj, and colocated jj across preparation, commit
  classification, recovery, proof revalidation, rollback, index restoration or
  activation, quarantine handoff, disposal, and rootless retry. No additional
  findings remain after accounting for the accepted bounded staging-leak
  disposition recorded by `reap-attributable-staging-leftovers-k161`.
- The interface remains small: `finish_transaction` owns transaction
  orchestration, while `repo::prepare_finish`, `PreparedFinish::commit`, and
  `repo::recover_finish` expose typed committed/not-committed/recovery-pending
  outcomes. Production callers do not reimplement repository outcome
  classification; F1 is inside the adapter's shared proof predicate.
- Fresh verification: `cargo fmt --check` exited 0 and
  `cargo test --locked --quiet` exited 0. The latter included 566 unit tests and
  all repository integration-test binaries. No production, test, methodology,
  or durable design artifact was changed by this review.
