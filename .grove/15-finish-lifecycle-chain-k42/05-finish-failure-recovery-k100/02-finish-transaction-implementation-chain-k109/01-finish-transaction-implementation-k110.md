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
  non-empty tracked deletion fingerprint, and canonical recursive no-follow
  root-entry digests. `FINISHING-*` is recognized before normal tree parsing; a
  ready witness blocks every unrelated operation and must remain absent from
  every candidate committed tree.
- Model jj partial commits explicitly: the selected deletion remains in the
  recorded working-copy change at its recorded parents and becomes the exact
  parent of a new successor containing unrelated work plus the witness. `Not
  committed` requires exact teardown-result absence, the current working-copy
  commit itself still be that recorded change, and post-restore reproduction of
  the manifest's exact preflight commit ID; mere change-id presence is
  insufficient. In colocated jj, preserve the user's Git index before any
  preflight snapshot/export.
- Plain Git disables hooks for its internal scoped commit. Post-commit root
  removal is one atomic rename to a preflighted same-device workspace-control
  quarantine before descriptor-rooted no-follow disposal; cross-device control
  layout and a wholly untracked task tree refuse before mutation.
- `Recovery pending` is operator-recoverable: diagnostics name the witness,
  recorded/observed topology, and the exact-start rollback versus exact-result
  forward-recovery procedures. Grove never rewrites divergent history itself.
  `finish-commit` owns immediate cleanup and a later lease-owning driver reaps
  orphaned internal quarantines/auxiliaries without treating them as receipts.
- Keep the repository outcome guarded through tree handoff: revalidate before
  and after rollback/quarantine rename, remove the witness only after the second
  stable result, and atomically restore quarantine to `.grove/` when forward
  proof changes.
- Use the active session epoch's launch nonce as an opaque finish-attempt
  identity, store it in the manifest and handle-named deletion commit, and
  require it for rootless same-launch retry. Open and identity-revalidate `.grove/` itself as a
  real no-follow directory; use descriptor-relative transaction operations.

## Done when

- Repository validation failure leaves the live finish tree and prior Git index
  untouched; witness preparation rejects empty tracked deletion, collisions,
  unsupported special entries, undefined directory digests, and tampering before
  moving a source entry.
- Every evacuation and rollback transition is restart-safe. `Not committed`
  requires the recorded Git/jj topology plus restored repository auxiliaries;
  otherwise the witness remains `Recovery pending` with an actionable
  diagnostic.
- Exact Git/native-jj/colocated-jj commit proof recovers forward idempotently,
  activates the correct colocated success index, and atomically quarantines the
  whole task root before no-follow disposal. It never restores the old tree.
- Bare lifecycle recovery occurs after full config validation and under the
  universal lock: pre-commit state exposes a fresh finish HITL session;
  committed state hands into the existing rootless/fresh restart contract.
- Scoped commits and every failure preserve unrelated staged, working-tree, and
  working-copy bytes; plain Git hook suppression is regression-tested.
- Unit transition tests cover every prepared/evacuated/rollback/quarantine
  boundary, including symlinks, foreign entries, collisions, corruption,
  rollback failure, pre/post-handoff repository races, unexpected repository
  topology, a tracked witness, symlinked task root, reused handles across
  distinct attempt identities, lost results, operator-restorable recovery,
  cleanup reaping, and cleanup failure. Process tests cover plain Git, native
  jj, colocated jj, and driver restart.
- `content/SKILL.md`, CLI diagnostics/help, architecture/user documentation, and
  test seams describe the implemented behavior without adding lifecycle state.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Use TDD at the transaction interface and injected repository/filesystem failure
seams. Do not use real user hooks as the commit-failure injector once finish
commits deliberately disable hooks.
