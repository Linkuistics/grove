# driver-lease-k31

**Kind:** impl

## Goal

Give one bare driver exclusive, process-scoped ownership of its exact working
tree through a VCS-administration control seam, while preserving the current
launch behavior for the later lifecycle cutover.

## Context

- Depends on the repository seams established by
  `session-kind-migration-integrate-k29`.
- Binding design: `docs/adr/one-live-driver-per-working-tree.md` and
  `docs/specs/config-driven-sessions.md` section "Process ownership and session
  epochs" through driver-lease ownership.
- Primary code surfaces: `src/repo.rs`, a focused new process-ownership module,
  `src/loop_driver.rs`, `src/herdr.rs`, and isolated Git/jj driver tests.
- This is an expand slice: acquire the lease around the existing driver before
  `lifecycle-cutover-k39` replaces its routing policy.

## Done when

- The closest on-disk `.jj`/`.git` marker, jj-first and independent of VCS
  discovery environment, resolves one workspace-scoped control directory for
  native/colocated/secondary jj and plain/linked Git shapes.
- Driver acquisition opens and pins the canonical working-tree root, takes a
  nonblocking exclusive lease, revalidates locked path identity with bounded
  retries, writes a fresh OS-random 128-bit process nonce, and marks every
  descriptor close-on-exec.
- A second driver for the same alias-equivalent working tree fails immediately
  before configuration/tree access or launch, reports/retains Herdr `blocked`,
  while distinct worktrees/workspaces remain independent.
- The owner revalidates its lease before lifecycle/launch transitions and holds
  it through final disposition; normal return, panic, and process death release
  ownership without PID cleanup.
- Tests cover aliases, shared-repository workspaces, conflicting Git/TMPDIR
  environment, path replacement retries/failure, unwritable controls,
  descriptor inheritance, and release after normal/forced exit.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

The lease is the loop-lifetime owner, not the shorter Tree access lock. Epoch
admission and random per-launch signals belong to `session-epoch-k35`.
