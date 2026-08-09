# finish-transaction-implementation-k110

**Kind:** impl

## Goal

Implement the reviewed fail-closed finish transaction across tree lifecycle,
repository adapters, driver recovery, methodology, and acceptance tests.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` sections "Pre-commit
  transaction and recovery", "Crash and retry semantics", "Scoped Git and
  Jujutsu commits", and "Test seams"; ADR
  `task-tree-transactions-fail-closed`; glossary term Finish transaction.
- Consume the integrated result of `finish-transaction-contract-integrate-k108`
  and the post-commit exact-result seam delivered by
  `post-teardown-restart-contract-integrate-k104`; do not recreate competing
  proof rules.
- Current code deletes `.grove/` in `tree_lifecycle::finish_commit` before
  `repo::commit_finish`; move orchestration into one deep transaction module.
  Keep Git/native-jj/colocated-jj mechanics behind a typed repository outcome
  seam.
- The finish manifest records the stable handle, repository anchor, expected
  tracked deletion fingerprint, and symlink-safe root-entry recovery data.
  `FINISHING-*` is recognized before normal tree parsing; a ready witness blocks
  every unrelated operation.
- Plain Git disables hooks for its internal scoped commit. Post-commit root
  removal is one atomic rename to a preflighted same-device workspace-control
  quarantine before best-effort recursive disposal; cross-device control layout
  refuses before mutation.

## Done when

- Repository validation failure leaves the live finish tree and prior Git index
  untouched; witness preparation rejects collisions/tampering before moving a
  source entry.
- Every evacuation and rollback transition is restart-safe. `Not committed`
  requires the recorded Git/jj topology plus restored repository auxiliaries;
  otherwise the witness remains `Recovery pending` with an actionable
  diagnostic.
- Exact Git/native-jj/colocated-jj commit proof recovers forward idempotently,
  activates the correct colocated success index, and atomically quarantines the
  whole task root before disposal. It never restores the old tree.
- Bare lifecycle recovery occurs after full config validation and under the
  universal lock: pre-commit state exposes a fresh finish HITL session;
  committed state hands into the existing rootless/fresh restart contract.
- Scoped commits and every failure preserve unrelated staged, working-tree, and
  working-copy bytes; plain Git hook suppression is regression-tested.
- Unit transition tests cover every prepared/evacuated/rollback/quarantine
  boundary, including symlinks, foreign entries, collisions, corruption,
  rollback failure, unexpected repository topology, lost results, and cleanup
  failure. Process tests cover plain Git, native jj, colocated jj, and driver
  restart.
- `content/SKILL.md`, CLI diagnostics/help, architecture/user documentation, and
  test seams describe the implemented behavior without adding lifecycle state.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Use TDD at the transaction interface and injected repository/filesystem failure
seams. Do not use real user hooks as the commit-failure injector once finish
commits deliberately disable hooks.
