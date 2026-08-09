# migration-hook-suppression-k114

**Kind:** impl

## Goal

Make automatic plain-Git migration commits obey the same no-user-hooks
preservation rule as internal finish commits.

## Context

- Externalized from `finish-transaction-contract-review-k107` F7 while
  integrating `finish-transaction-contract-integrate-k108`.
- Migration and finish are both unattended, Grove-authored, path-scoped commits
  that promise unrelated staged and working-tree work is preserved. The finish
  contract disables user hooks because an index image cannot reverse arbitrary
  hook side effects; migration currently has the same failure mode without the
  same guard.
- Keep migration's fail-closed witness, rollback, scoped commit, and Git/jj
  symmetry intact. This leaf does not redesign finish recovery.

## Done when

- Plain-Git migration commits run with an empty workspace-control hooks path,
  while signing and injected repository failures remain testable through the
  repository seam.
- A regression proves a mutating or rejecting user hook is not invoked and
  unrelated staged and working-tree bytes remain unchanged on migration
  success and failure.
- The migration spec, ADR, methodology/user documentation, and diagnostics no
  longer imply a weaker hook policy than finish.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Use the existing internal Git commit adapter rather than adding hook policy to
user configuration.
