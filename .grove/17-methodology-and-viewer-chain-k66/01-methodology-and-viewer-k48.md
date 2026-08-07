# methodology-and-viewer-k48

**Kind:** impl

## Goal

Reconcile the embedded Grove methodology and doubt skill with the shipped
bare-command, configured-session, nineteen-kind behavior.

## Context

- Depends on `legacy-review-removal-integrate-k65` so canonical guidance is
  written from the final executable surfaces, not transitional compatibility;
  `herdr-session-kind-viewer-k51` already owns the separately versioned viewer.
- Primary artifacts: `content/SKILL.md`, `content/TASK-FORMAT.md`,
  `content/driving.md`, `content/prompts/`,
  `plugins/linkuistics/skills/doubt-driven-development/`, and
  guidance/navigation tests.
- The root brief explicitly requires prompts, the nineteen-kind taxonomy,
  doubt/Grove review ownership, and filename-only viewer parsing to move in
  lockstep.

## Done when

- Provisioned Bootstrap resolves the prompt-mandated stable handle and does not
  pick; bare `grove` is the sole lifecycle entry; current tasks carry kind only
  in filenames; configuration is the only launch policy; finish is a resumable
  HITL session; receipts/diversity inference and grow-verb harness flags are
  absent.
- The taxonomy consistently names all nineteen kinds, including distinct
  `research-a`/`research-b` and driver-reserved `finish`, across the main skill,
  task format, driving guide, examples, and doubt review-ownership contract.
- `content/prompts/continue.md` is the only embedded launcher. Obsolete
  `start.md` and `retire.md` are deleted, and build/provision tests prove removed
  prompts are not embedded or swept.
- Canonical guidance cites the filename-only viewer contract established by
  `herdr-session-kind-viewer-k51` without coupling methodology provisioning to
  viewer installation or implementation.
- Canonical-guidance, reference-navigation, and provisioning tests pass without
  restoring receipt-era language. `cargo fmt --check` and
  `cargo test --locked` pass.

## Notes

Keep the skill procedural. Durable architecture and user-facing prose belong to
`durable-docs-reconciliation-k49`; viewer code and fixtures stay in k51.
