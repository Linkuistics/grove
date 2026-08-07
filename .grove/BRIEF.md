# grove.harness-and-model-chosen-via-config — brief

## Goal

Simplify Grove so one bare `grove` command drives every session from the task
tree, while one personal KDL file is the entirety of user launch policy.

## Done when

- `~/.config/grove/config.kdl` is the only user configuration and contains one
  complete command template for every session kind, including any
  harness-specific herdr launch policy.
- A single driver-side pick selects each real leaf; its filename supplies the
  session kind, `${prompt}` carries its stable handle as the mandate, and the
  launched session resolves that handle without picking again.
- Bare `grove` initializes requirements, migrates legacy trees, drives the loop,
  and materializes a resumable finish leaf without subcommands or flags;
  complete config validation precedes every one of those mutations.
- Legacy harness/model routing, stamps, user-settable environment overrides,
  task-body kind/harness fields, one-off retire/migrate commands, review target
  receipts, diversity warnings, structured harness-routing peeks,
  `GROVE_SESSION_TARGET`, and grow-verb harness flags are removed.
- Git and jj behavior, migration, error paths, direct process execution, and the
  self-driving loop are verified through the agreed seams; `content/`, the
  doubt skill, the herdr tree viewer, user documentation, and the minimum
  coherent ADR/spec set all describe the resulting system.
- Herdr turn-level reporting and correct harness re-detection remain available,
  but only through visible command-template policy: `${herdr_settings}` is an
  explicit optional argv splice and `HERDR_AGENT` is set by the configured
  command, never inferred or secretly injected by Grove.

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
  `docs/adr/grove-owns-escalated-review.md`,
  `docs/adr/promotion-transactions-fail-closed.md`, and
  `docs/specs/doubt-grove-review-mechanics.md`.
- Test seams: the bare `grove` process in isolated Git/jj worktrees with fake
  commands, and the internal `grove-llm` tree interface.

## Notes

Confirmed and closed to design: one bare command; one strict nineteen-kind KDL
configuration; one POSIX-shell-word command-template string per kind, executed
directly without a shell; one authoritative driver pick; real requirements and
finish leaves; automatic restart-safe legacy migration; no receipts or diversity
warnings; and no inferred harness, model, or reasoning effort. `${prompt}` occurs
once after a literal executable word, not necessarily last. The exact KDL node
shape, diagnostic rendering, migration transaction implementation, and minimum
coherent ADR/spec reshaping remain design work.
