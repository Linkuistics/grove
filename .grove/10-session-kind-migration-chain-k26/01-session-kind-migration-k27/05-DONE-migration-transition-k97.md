# migration-transition-k97

**Kind:** impl

## Goal

Expose one guarded lifecycle transition interface that completes fresh-tree
recovery or legacy migration and leaves a current tree ready for authoritative
selection.

## Context

- Integrates `session-kind-plan-k93` through `migration-scoped-commit-k96`.
- `lifecycle-cutover-k39` will own bare-driver ordering and config reloads; this
  leaf supplies the one transition it calls.

## Done when

- One interface classifies absent, exact partial scaffold, pending migration,
  accepted legacy, current, and unknown-format states under one exclusive Tree
  access guard without nested reacquisition.
- Accepted states finish or recover to a fully current tree; current is a no-op;
  unknown or ambiguous states fail before mutation.
- Existing standalone migrate/adoption callers are reconciled without
  duplicating transition order in the driver.
- End-to-end Git, native-jj, and colocated-jj tests exercise successful restart,
  refusal paths, and a final current-tree pick.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Do not implement the bare `grove` lifecycle cutover here.
