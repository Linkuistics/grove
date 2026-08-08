# release-acceptance-verification-k91

**Kind:** impl

## Goal

Prove removed runtime/documentation policy, release health, and the root brief's
complete acceptance rollup from the final integrated tree.

## Context

- Depends on `tree-vcs-acceptance-verification-k90`.
- Use positive and cross-tree controls for every legacy-surface claim and
  preserve legitimate historical discussion.
- Verification commands include `cargo fmt --check`, `cargo test --locked`, and
  `scripts/release-doctor.sh`.

## Done when

- Legacy-surface sweeps classify every candidate across runtime, methodology,
  and durable documentation; legitimate loop-control and historical references
  remain explicit.
- The root brief's complete Done-when list maps directly to landed code, tests,
  content, docs, ADRs, specs, and recorded acceptance evidence with no unnamed
  residue.
- `cargo fmt --check`, `cargo test --locked`, and
  `scripts/release-doctor.sh` pass; `jj st` shows only focused acceptance work
  before retirement.

## Notes

Do not publish, integrate, or tear down the worktree; those remain outside this
grove.
