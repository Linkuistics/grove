# grove.harness-and-model-chosen-via-config — brief

## Goal

Simplify Grove so one bare `grove` command drives every session from the task
tree, while one personal KDL file is the entirety of user launch policy.

## Done when

- `~/.config/grove/config.kdl` is the only user configuration and contains one
  complete command template for every session kind.
- A single driver-side pick selects each real leaf; its filename supplies the
  session kind and the launched session does not pick again.
- Bare `grove` initializes requirements, migrates legacy trees, drives the loop,
  and materializes a resumable finish leaf without subcommands or flags.
- Legacy harness/model routing, stamps, user-settable environment overrides,
  task-body kind/harness fields, one-off retire/migrate commands, review target
  receipts, and diversity warnings are removed.
- Git and jj behavior, migration, error paths, direct process execution, and the
  self-driving loop are verified through the agreed seams; user documentation
  and the minimum coherent ADR/spec set describe the resulting system.

## Decomposition

- `plan-chain-k2`: establish, adversarially review, and integrate the shared
  requirements captured in this brief and `CONTEXT.md`.
- `config-driven-sessions-chain-k5`: design and review the durable command,
  configuration, task-tree, lifecycle, migration, and deletion contracts.
- `implementation-slices-chain-k9`: after design integration, cut and review the
  vertical implementation slices; implementation work is intentionally deferred
  to the tree this planning stage grows.

## Pointers

- Glossary: `CONTEXT.md` — Grove configuration, Session kind, Kind routing,
  Session-kind migration, Review target diversity, Pick, Complete finish cycle.
- Current behavior to reconcile: `docs/CONFIGURATION.md`, `docs/USAGE.md`, and
  `docs/ARCHITECTURE.md`.
- Existing records in scope: `docs/adr/review-target-receipts.md`,
  `docs/adr/grove-owns-escalated-review.md`, and
  `docs/specs/doubt-grove-review-mechanics.md`.
- Test seams: the bare `grove` process in isolated Git/jj worktrees with fake
  commands, and the internal `grove-llm` tree interface.

## Notes

The chosen configuration representation is one POSIX-shell-word command-template
string per session kind, parsed into argv and executed directly without a shell.
Grove substitutes only escaped scalar data and never infers the harness, model,
or reasoning effort from the configured command.
