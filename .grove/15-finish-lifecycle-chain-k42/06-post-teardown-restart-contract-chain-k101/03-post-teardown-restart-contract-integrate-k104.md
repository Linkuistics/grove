# post-teardown-restart-contract-integrate-k104

**Kind:** integrate-review-impl
**Integrates:** post-teardown-restart-contract-review-k103

## Goal

Apply the verified findings from `post-teardown-restart-contract-review-k103`
while preserving the reviewed artifact's contract.

## Context

- Verify every finding against the post-teardown design before changing the
  producer artifact.
- Preserve `.grove/` task-root absence as fresh start, signal-only session
  disposition, narrow handle-named retry proof, and epoch-scoped handle reuse.
  Do not absorb pre-commit failure recovery from `finish-failure-recovery-k100`.

## Done when

- Every review finding has a recorded disposition and every verified issue is
  fixed at the narrowest seam.
- The methodology, lifecycle behavior, and process regressions agree on the
  post-commit/no-observed-done window for Git and jj.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes
