# driver-lease-integrate-k33

**Kind:** integrate-review-impl
**Integrates:** driver-lease-review-k32

## Goal

Apply the verified findings from `driver-lease-review-k32` while preserving the reviewed artifact's contract.

## Context

- Verify every `driver-lease-review-k32` finding against the design's explicit
  workflow-consistency and repository-corruption limits.
- Do not solve epoch admission early by widening the lease interface.

## Done when

- Every finding has a recorded disposition; verified issues are fixed with
  deterministic race or multi-workspace regression coverage.
- The driver lease remains a small process-owned interface and leaves tree
  operation serialization to the existing tree seam.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Substantial epoch work is externalized under `session-epoch-chain-k34`. The
separate compatibility-transition gap surfaced by the hostile migration
regression is externalized as
`adoption-migration-session-kind-transition-k98`.

## Finding dispositions

- `driver-lease-review-k32 F1` — fixed. The leased on-disk worktree is
  authoritative after acquisition. A shared subprocess boundary anchors Git
  operations to it for repository discovery, migration commits, the internal
  `grove-llm kind` read, and the foreground harness. Separate-process hostile
  environment regressions cover migration and the driver launch.
- `driver-lease-review-k32 F2` — fixed by recording fresh format and full-suite
  evidence below after integration.
- `driver-lease-review-k32 F3` — fixed. The environment-mutating repository
  regression now runs in its own integration-test process rather than relying
  on a file-local mutex that sibling tests never acquire.
- `driver-lease-review-k32 F4` — fixed. The second-driver regression replaces
  the live driver's tree with a legacy v1-flat fixture and asserts no rename,
  format witness, or migration commit occurs. A mutation check proved the test
  fails when acquisition is moved below migration.
- `driver-lease-review-k32 F5` — fixed for the surviving compatibility surface.
  The documented read-only `--no-launch` readiness check does not acquire the
  lifetime lease, returns before adoption migration, and remains available
  beside a running driver. A pending legacy migration is reported without
  changing the tree;
  `legacy-command-surface-removal-k77` still owns removing the flag.
- `driver-lease-review-k32 F6` — fixed. Revalidation compares each current path
  with the identity pinned at acquisition; it no longer repeats invariant
  `fstat` comparisons on its already-open descriptors.
- `driver-lease-review-k32 F7` — fixed. A direct unit assertion covers
  `FD_CLOEXEC` on both owned descriptors, while the exec-descendant regression
  now keeps the helper as parent and kills and reaps the descendant explicitly.
- `driver-lease-review-k32 F8` — fixed. The live-driver fixture is owned by a
  drop guard that gracefully terminates and reaps the driver on assertion
  unwind, letting the driver's watcher terminate and reap the foreground
  harness. The regression asserts the recorded harness PID no longer exists.
- `driver-lease-review-k32 F9` — deferred unchanged to `session-epoch-k35`.
  This integration does not widen the process-owned lease interface before the
  epoch consumer establishes the shared/exclusive acquisition abstraction it
  needs.

## Verification

- The hostile-Git-environment launch regression failed first at worktree
  discovery, then at the internal kind read. The final coverage also failed at
  migration staging and foreground-harness repository selection before every
  subprocess boundary was anchored to the on-disk worktree.
- The `--no-launch` concurrency regression failed while readiness still
  acquired the lifetime lease. A legacy-tree variant then failed because
  readiness still performed adoption migration; both pass after the read-only
  path stopped taking ownership and returns before mutation.
- Moving lease acquisition below migration made the strengthened second-driver
  regression fail on an observed v1-flat rename; restoring the required order
  made it pass without a migration witness or commit.
- The exec-descendant regression exposed and then eliminated a test handshake
  race; its explicit post-release synchronization now passes while the execed
  descendant remains alive.
- The integration leaf spent its one narrow fresh-context review allowance.
  Its three Important boundary findings and one Minor fixture-lifecycle finding
  were verified and fixed; the separate adoption/session-kind composition gap
  it surfaced is recorded in
  `adoption-migration-session-kind-transition-k98` rather than absorbed here.
- `cargo fmt --all --check` passes.
- `cargo test --locked --quiet` passes, including 406 library tests and every
  integration-test binary.
