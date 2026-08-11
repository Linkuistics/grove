# workspace-layout-preflight-impl-k169

**Kind:** impl

## Goal

Implement the workspace layout preflight designed by
`workspace-layout-preflight-k113`: prove during driver-lease acquisition that the
workspace can supply the atomic same-filesystem rename target finish teardown
needs, so an unfinishable layout stops the invocation before it holds a task
tree.

## Context

- Durable design: `docs/specs/config-driven-sessions.md` (*Workspace layout
  preflight*) and ADR `supported-workspace-layouts`. `CONTEXT.md` defines the
  term; `docs/ARCHITECTURE.md` and `docs/USAGE.md` already describe the behavior.
- The mechanism is small and the operands are already in hand. `DriverLease::acquire`
  (`src/driver_lease.rs:123`) resolves `repo::workspace_control`, creates the
  control directory, opens the working-tree root, and records its device in
  `worktree_identity`. The check is one `stat` of the created control directory
  and one device comparison, sited after that identity is pinned and before the
  epoch record is initialized.
- Reuse the existing failure vocabulary rather than inventing one. The finish
  transaction's `ensure_same_device` (`src/finish_transaction.rs:1977`) already
  formats this class of refusal, and the resolver's marker kind — the `.git`
  file/directory branch in `repo::workspace_control` (`src/repo.rs:78`) — is what
  supplies the gitdir target the diagnostic must name.
- Do not weaken or short-circuit the finish-time preflight, and add no durable
  capability marker: the design records three independent reasons the two checks
  must stay separate.

## Done when

- Bare `grove` refuses an unsupported layout before configuration validation and
  before any `.grove/` observation or mutation, as a resumable no-mutation stop.
- The diagnostic names the working-tree root and its device, the control
  directory and its device, the marker that produced the resolution including a
  `.git` file's gitdir target, and both remedies; it is distinguishable from the
  unwritable-control-directory failure.
- The spec's two new acceptance-seam bullets pass: the layout matrix through the
  bare-process seam (cross-device linked worktree in rootless and existing-tree
  shapes, symlinked `.git`/`.jj` markers, and every admitted layout), and the
  independence of the two preflights (finish still refuses a layout that changed
  after acquisition; a `.grove/` mount point passes acquisition and is refused at
  finish; an operator retry re-measures).
- Ambient `grove-llm` tree verbs are unchanged.

## Notes

Deliberately a plain `impl` leaf rather than a review chain. The change is one
`stat` and one comparison in already-reviewed machinery, its failure mode is a
loud refusal rather than silent corruption, and the design task enumerated the
layout matrix as explicit seams — so a fresh adversarial context has little the
seams do not already cover. Cut a chain instead if implementation finds the
device comparison is not sufficient to characterize a supported layout.
