# ravel-local-delivery-k23 — brief

## Goal

Deliver a useful and complete local-source directory route through the public
resolution and launch boundaries of Ravel, while all source packs remain in
Grove.

## Done when

- Configured local sources, roots and mandatory files produce immutable
  retained resources, a directory-only descriptor and reusable loading
  instructions. Disabled delivery remains byte-identical and has no cache or
  network side effect.
- Public launch prepares automatically before configuration evaluation. It
  uses the reviewed original and prepared request fields, a matching Rhai
  front, worker and host, complete frame bounds and exact caller and ending
  bytes. No helper for each policy and no resolver of named skills exists.
- Local freshness, publication and concurrency, lifetime, protected origins
  and descriptor recovery pass public fixtures. Actual confined children read
  the exact external resources on the supported macOS and Linux backends. They
  make no writes and have no access to the protected configuration. A failure
  never retries unconfined.
- Owning Ravel documentation, examples, book and source excerpts and reusable
  tasks match this delivered subset. Ravel checks and temporary installed-pair
  smoke checks pass. The stage records its commits and reproducible build
  inputs.

## Decomposition

The first child plans this increment from current source. Grow implementation
just in time. A public `skills resolve` reader can be a useful stage only if it
actually delivers reusable complete resources and instructions. Do not split a
schema or internal resolver from its only usable reader. Launch preparation,
worker interpretation and required read grants must have a green handoff.

`ravel-local-delivery-k24` grew these ordered children. Each one leaves the
line green with useful public behaviour.

1. `ravel-schema-four-k53`: schema 4 with delivery disabled, the frame bound
   and its check. This is the native-only 2.0 contract.
2. `ravel-skills-resolve-k54`: the `skills` settings object, local capture,
   publication, the descriptor and the policy-free `skills resolve`.
3. `ravel-skill-launch-k55`: automatic preparation in `start-session`, the
   request fields, the notice and `RAVEL_SKILL_DIRECTORIES`. A confined launch
   with delivery enabled refuses until the confinement leaf.
4. `ravel-skill-publication-k56`: change detection, path identity,
   concurrent publication and cache safety.
5. `ravel-skill-bounds-k57`: the bounds, the deadline and cancellation.
6. `ravel-skill-lifetime-k58`: lifetime, nested sessions and the recovery
   environment.
7. `ravel-skill-confinement-k59`: read grants on both real backends.
8. `ravel-smoke-sandbox-home-k60`: the narrow sandbox home of the smoke tasks.
9. `ravel-local-handoff-k61`: the final rebase check, documentation, a
   temporary installed pair and the recorded commits.

Every child works in the workspace `Ravel.skill-directory-delivery`. It
advances the bookmark `skill-directory-delivery` by jj and never pushes it. It
records its Ravel output commit in its running log and its Grove commit
message. It runs the Ravel tasks that its change touches. It moves the book,
the examples and `CHANGELOG.md` with its behaviour. A producer that earns a
`review-impl` inserts it before `ravel-local-handoff-k61`.

## Pointers

- Owning repository: `/Users/antony/Development/AgenticOrchestration/Ravel`.
- The reviewed Ravel contract is commit `822c0ed2f67d`, on the bookmark
  `skill-directory-delivery`. ravel-rhai-skill-delivery-k50 integrated the
  review of the k34 commit `fc87cb41a422` there. Read its
  `docs/specs/skill-discovery.md` and `docs/skill-discovery-design-evidence.md`.
  Read its changes to `docs/specs/configuration-evaluation.md` and
  `docs/specs/session-execution.md`.
- Then read Grove's adoption contract from grove-rhai-skill-delivery-k35, its
  review grove-rhai-skill-delivery-k51 and the integration
  grove-rhai-skill-delivery-k52. It asks this stage for no behaviour that is specific to Grove.
  Grove needs neither `RAVEL_SKILL_DIRECTORIES` nor `skills resolve`. The
  historical design `b93bb927` supplies intent only.
- ADRs `skill-preparation-precedes-evaluation`,
  `skill-resources-outlive-sessions` and `ravel-supervises-the-session`.

## Settled inputs

- The feature ships in the combined Ravel 2.0.0. The script and request
  `SCHEMA_VERSION` is 4, and a schema-3 script refuses as
  `unsupported_version`, also with delivery disabled. The worker protocol
  stays 1.
- The `skills` settings object, the descriptor and `skills resolve --json` each
  have schema 1. The cache root is `$HOME/.cache/ravel/skills/v1`.
- Item 3 of the stage handoff in the Ravel evidence document lists the work of
  this stage. It includes the evaluate-frame bound of 38,541,312 bytes and the
  check of that bound before the worker receives the frame. The check receives
  its limit as an argument for the three frame-bound tests. The stage
  establishes on which release platforms a request can pass the bound.
- Item 3 also includes the narrow sandbox home of the smoke tasks. It refuses
  an inherited `CLAUDE_CONFIG_DIR` and the three Claude Code plugin-location
  variables. Fake-harness controls in `task harness:smoke:test` prove each
  refusal.
- The Ravel tasks are `task setup`, `task check`, `task test:cli`,
  `task smoke` and `task install PREFIX=<prefix>`. The book tasks are
  `task docs:book:build`, `task docs:examples:test`, `task docs:excerpts:test`,
  `task docs:figures:test`, `task docs:ste:test`, `task docs:breaks:test` and
  `task book:integrity`.
- `task harness:smoke:test` checks the smoke script. `task docs:skills` serves
  the design views on port 8778. No TypeScript SDK, typecheck or worker-build
  task exists on main.
- The manifest of the line says 1.1.0 until the cut of rollout, so the build of
  this stage reports `ravel 1.1.0`. Grove admits that output as 1.x. Record
  each installed-pair input by its commit and build identity, not by its
  version output.

## Notes

Use local-only settings for the acceptance of this stage. This stage refused
a valid Git source temporarily, with status 3 and no stable code. Ravel later
withdrew Git sources. ravel-local-sources-k90 replaces that refusal with the
permanent `settings_invalid` refusal of ravel-local-sources-k78 and removes
its fixture. Do not publish the local subset.

Keep the repaired configuration and API shapes intact. An unclosed public
decision earns a design leaf before dependent work. Use independent temporary
prefixes and isolated HOME settings, never shared activation.
common-loading-acceptance-k29 owns real LLM adherence.

## Integration handoff

Ravel main was `24a54523` at this integration. The design repair records the
exact current-main base before implementation. Work stays on the Ravel bookmark
`skill-directory-delivery`, outside main, through ravel-local-sources-k90 and
acceptance. The owning local implementation session rebases onto the
then-current main at entry. It reconciles upstream changes and reruns affected
checks before it records its exact base and output commits.

ravel-local-sources-k90 repeats that procedure. Rollout owns the final rebase, full
checks, landing of the complete feature on main and release. No incomplete
local build may enter an unrelated release. Hand this contract to the Ravel
owner before cross-repository implementation. Do not edit the tree of the other
workstream or reuse its working-copy change.
