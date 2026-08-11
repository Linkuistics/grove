# tree-vcs-acceptance-verification-k90

**Kind:** impl

## Goal

Prove current tree grammar and operations, migration, finish, and scoped Git/jj
behavior through their public seams.

## Context

- Depends on `process-acceptance-verification-k81`.
- Map the tree/VCS half of `docs/specs/config-driven-sessions.md`'s test matrix
  to executable tests or explicit already-covered evidence.
- Exercise isolated plain Git, native jj, and colocated jj worktrees.

## Done when

- The `grove-llm` matrix covers all nineteen kinds, malformed and terminal
  trees, finish rules, pair generation, stable resolution, promotion after
  insertion, migration-witness refusal, and current filename parsing.
- Git/jj scoped commits and unrelated-work preservation are proven.
- Only focused omissions are fixed, and focused tree/VCS tests plus
  `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Release and documentation acceptance remains a separate final handoff.
