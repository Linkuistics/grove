# durable-docs-reconciliation-k49

**Kind:** impl

## Goal

Reconcile the glossary, user documentation, architecture, ADRs, and specs into
the minimum coherent durable description of the implemented system.

## Context

- Depends on `methodology-and-viewer-integrate-k69`.
- Primary artifacts: `CONTEXT.md`, `CONTEXT-MAP.md`, `README.md`,
  `CHANGELOG.md`,
  `docs/CONFIGURATION.md`, `docs/USAGE.md`, `docs/ARCHITECTURE.md`,
  `docs/RELEASING.md`, the ADR set named by the root brief, and
  `docs/specs/config-driven-sessions.md` /
  `docs/specs/doubt-grove-review-mechanics.md`.
- Follow the in-place minimum-coherent-set discipline: edit/merge/split/delete
  current ADRs/specs rather than appending superseding records, then reconcile
  every citation.

## Done when

- User docs show one complete nineteen-node KDL example, direct argv/template
  rules, bare lifecycle/resume/finish behavior, config diagnostics, and no
  stamps, routing environment, task-body kind/harness, legacy commands, or
  hidden harness policy.
- Architecture describes the configuration, process-ownership, tree/migration,
  bare-driver, finish, provisioning, and Herdr seams at their implemented
  interfaces, with Git/jj symmetry and lock ordering explicit.
- The ADR set remains minimal and coherent. The review-mechanics spec retains
  only promotion/lock/ownership material still binding after receipt and target
  comparison deletion; citations from briefs, other ADRs/specs, and docs resolve.
- `CONTEXT.md` remains a glossary rather than an implementation guide, and
  `CONTEXT-MAP.md` ownership stays accurate.
- Repo-wide legacy-claim checks enumerate then classify candidates and include
  positive and cross-tree controls; docs/navigation tests plus
  `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Record structural facts, not self-invalidating occurrence counts.
