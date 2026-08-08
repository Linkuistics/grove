# process-acceptance-verification-k81

**Kind:** impl

## Goal

Prove the integrated Grove through its public configuration, foreground process,
lifecycle, lease, and epoch seams, fixing only focused omissions.

## Context

- Depends on `durable-docs-reconciliation-integrate-k73` and every preceding
  reviewed implementation slice.
- Map the process half of `docs/specs/config-driven-sessions.md`'s test matrix
  to executable tests or explicit already-covered evidence.
- Use isolated homes and worktrees; scrub this meta-grove's ambient loop
  authority.

## Done when

- Acceptance evidence covers KDL diagnostics and argv, two config reads,
  sibling-tool skew, metadata-only flags, root/migration/live/finish
  transitions, one pick and mandate, direct child lifecycle, lease/epoch races
  and timeouts, and the absence of hidden harness-specific launch policy.
- Only focused defects discovered by the sweep are fixed; subsystem redesign is
  externalized as a new leaf.
- Focused process tests, `cargo fmt --check`, and `cargo test --locked` pass.

## Notes

This child is independently useful process-level acceptance evidence; tree and
VCS coverage follows without reopening it.
