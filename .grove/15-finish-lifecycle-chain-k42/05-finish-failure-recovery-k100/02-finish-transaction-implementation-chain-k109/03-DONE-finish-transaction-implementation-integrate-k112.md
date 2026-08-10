# finish-transaction-implementation-integrate-k112

**Kind:** integrate-review-impl
**Integrates:** finish-transaction-implementation-review-k111

## Goal

Apply the verified findings from `finish-transaction-implementation-review-k111` while preserving the reviewed artifact's contract.

## Context

- Verify every `finish-transaction-implementation-review-k111` finding against
  the integrated design before changing code.
- Preserve the three repository dispositions, manifest anchor/fingerprint,
  symlink-safe in-root witness, positive unchanged-topology proof, hook-free
  plain-Git commit, atomic post-commit quarantine, and rootless/fresh restart
  semantics.

## Done when

- Every finding has a recorded disposition; each valid issue is fixed at the
  narrowest transaction or adapter seam with a regression.
- All documented Git/native-jj/colocated-jj failure and restart properties hold
  without consuming unrelated work or adding durable lifecycle state.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

- F1 (High, plain-Git exact-result proof accepts a merge commit): **valid, fixed**.
  Reproduced exactly as reported — a `git commit-tree`-built merge whose first
  parent is the recorded start, whose message is the handle-and-attempt teardown
  message, and whose tree is the start's less `.grove/`, was classified
  `Committed` by `GitFinishProof::revalidate`. `rev-parse HEAD^` selects a merge's
  *first* parent as readily as a sole parent, and every remaining predicate
  compares trees, which a merge can satisfy regardless of its second parent.
- Fixed at the adapter's shared proof predicate, the narrowest seam: a new
  `git_sole_parent` derives the parent through the full parent list and refuses
  both a root commit and a merge. `validate_exact_git_finish` and
  `verify_lost_plain_git_finish` now share it, so the two plain-Git proof paths no
  longer disagree about what may be a teardown commit. No caller gained repository
  outcome classification, and the interfaces are unchanged.
- The merge now leaves `recover_finish` at `RecoveryPending` via the start-proof
  divergence check, as the finding required: no live task tree is quarantined or
  disposed on an ambiguous topology.
- Regression: `a_merge_over_the_recorded_start_is_not_a_plain_git_finish_result`
  asserts both the predicate refusal and the fail-closed recovery outcome. Verified
  failing-first — restoring `rev-parse HEAD^` panics it at the
  `revalidate().unwrap_err()` assertion because the merge is proven committed.
- No durable record changed. `docs/adr/task-tree-transactions-fail-closed.md`, the
  spec, and `CONTEXT.md` already required the exact *immediate* result and a diff
  against the commit's own parent; the contract was right and the code was not.
- Verification (2026-08-11): `cargo fmt --check` exited 0 and `cargo test --locked`
  exited 0 across all 39 test binaries, including 567 unit tests (566 before this
  regression) and every repository integration-test binary.
