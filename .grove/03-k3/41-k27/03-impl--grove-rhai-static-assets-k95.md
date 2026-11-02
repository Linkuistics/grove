# grove-rhai-static-assets-k95

## Goal

Add the Rhai sample configuration and the Rhai static example. Put one case
table under the checks of both asset sets.

## Context

- The "Caller assets" and "Runtime parity" sections of
  `docs/specs/codex-skill-provisioning.md`.
- `assets/ravel-policy/sample/policy.ts`, `examples/grove-static.ts` and their
  checks in `crates/grove/tests/ravel_policy_assets.rs`.
- Ravel's "Frozen comparison rules" and "Measured reads" in
  `docs/specs/configuration-evaluation.md` at the directory build.
- The runtime-explicit test seam of grove-ravel-runtime-fixtures-k94.

## Done when

- `sample/config.rhai` and `examples/grove-static.rhai` sit beside their
  TypeScript counterparts. Grove ships its own copy of Ravel's static example,
  imported by its path. Each asset names its runtime and schema.
- The Rhai sample keeps the four arrangements, the two modifiers, the default
  choice and the refusals. It reads the choice file with the 4,096-byte bound.
  It reads the jj store path through a measured read. It keeps the secondary
  jj workspace grants.
- The Rhai sample passes `request.prompt` to each harness. For a harness that
  needs explicit directories, it adds one read grant for each `snapshotRoot` in
  `request.skills`. With delivery disabled, it adds no grant.
- One case table drives the static and sample checks of both runtimes. Each
  case states the expected outcome for each runtime. It compares routing
  choices, argument vectors, context values and refusal code, stage, exit and
  remedy.
- The three accepted differences each have a case with both outcomes and the
  reason beside it. They are the oversized choice file, U+FEFF and U+0085, and
  the jj store path that is not UTF-8.
- The TypeScript checks keep their strict typecheck against the explicit 1.1
  build. The Rhai checks launch through public `start-session` of the explicit
  2.0 build with fake agents.
- The assets README states both sets, their runtimes, the layout and how to
  copy each sample. No owner configuration is written.

## Notes

Keep the TypeScript files at their installed paths, byte for byte, unless a
check needs a change. Controls: a changed expected outcome in the table must
fail. A Rhai sample that passes `request.originalPrompt` must fail. A grant
for a `snapshotRoot` with delivery disabled must fail. Watch each fail once.

Earn a `review-impl` if a non-accepted runtime difference remains after the
work.
