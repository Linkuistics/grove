# user-docs-reconciliation-k80

**Kind:** impl

## Goal

Reconcile all user-facing documentation with the complete nineteen-kind KDL
configuration and bare lifecycle.

## Context

- Depends on `methodology-and-viewer-integrate-k69`.
- Primary artifacts: `README.md`, `CHANGELOG.md`, `docs/CONFIGURATION.md`,
  `docs/USAGE.md`, and `docs/RELEASING.md`.
- Durable architecture decisions remain owned by
  `architecture-records-reconciliation-k88`; cite them without restating them.

## Done when

- User docs contain one complete nineteen-node KDL example, direct argv and
  template rules, bare lifecycle/resume/finish behavior, and actionable config
  diagnostics.
- Stamps, routing environment, body kind/harness metadata, legacy commands, and
  hidden harness policy are absent from current user guidance.
- Focused docs/navigation checks, `cargo fmt --check`, and
  `cargo test --locked` pass.

## Notes

This is an independently useful handoff: users can operate the shipped product
correctly before internal architecture prose is reconciled.
