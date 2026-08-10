# post-teardown-restart-contract-k102

**Kind:** impl

## Goal

Implement the settled post-teardown restart contract in the provisioned
methodology and executable acceptance seams without adding lifecycle state.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` sections "Fresh tree",
  "Existing live tree", and "Crash and retry semantics"; ADR
  `one-live-driver-per-working-tree`; glossary terms Complete finish cycle,
  root-init, Driver lease, and Session epoch.
- At a driver lifecycle transition, `.grove/` task-root absence always means a
  fresh grove. VCS history and unlocked lease/epoch/signal artifacts are not
  rootless-driver finish receipts; only a retry of the current handle-named
  teardown command may verify its own immediate VCS result.
- A finish target that exits without a signal retains its real status and
  elapsed-time diagnostic. A `finish-commit` retry distrusts task-root absence;
  only the exact immediate handle-named, `.grove/`-scoped Git or jj commit makes
  it idempotently successful and licenses `complete --done` in the still-live,
  already-confirmed session.
- A new task tree may reuse `plan-k1`; epoch rotation excludes the stale
  cooperating session's `grove-llm` operations.
- Existing lifecycle code already supplies fresh-root selection, no-signal
  reporting, and epoch handoff. Add the narrow retry proof and missing acceptance
  coverage without a second lifecycle command, prompt, tombstone, or
  rootless-driver VCS heuristic.

## Done when

- `content/SKILL.md` describes the bounded retry window, ordinary no-signal
  stop, and fresh reinitialization after post-commit driver death without
  contradicting the complete finish cycle.
- Process-level regression coverage proves successful `finish-commit` followed
  by no signal stops with the real child outcome, and a later bare invocation
  initializes and launches a fresh `requirements` `plan-k1` rather than
  recovering the deleted finish.
- Lost-result regressions in plain Git, native jj, and colocated jj prove the
  exact immediate teardown commit makes `finish-commit` idempotent, while absent
  `.grove/` without that proof refuses; prior teardown history plus reused
  handles cannot satisfy a new invocation.
- A deterministic driver-death regression covers `done` written after the
  successful deletion commit but before post-reap interpretation; abandoned
  signal cleanup does not become a finish receipt and restart follows the epoch
  handoff/fresh-root contract.
- The post-finish orphan-guard regression proves the first replacement stops
  `blocked` without creating `.grove/` and a later invocation after guard release
  initializes the fresh task tree.
- Epoch/handle-reuse coverage proves the old cooperating session's `grove-llm`
  operations cannot act on the new task tree, reusing an existing seam when it
  already demonstrates the complete property.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Keep `finish-failure-recovery-k100` separate: it owns failures before a finish
commit succeeds; this chain owns only the successful-commit/no-observed-done
window.
