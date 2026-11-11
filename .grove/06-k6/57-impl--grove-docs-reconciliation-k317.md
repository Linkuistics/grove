# grove-docs-reconciliation-k317

## Goal

Make Grove's guides, specifications, formula caveats and release notes
describe the final common path for the compatible Grove release.

## Context

- The parent brief, Done when, first item.
- `docs/common-loading-acceptance-record.md`, "Gap register" and "Known
  limits". The parent brief's "Other facts for rollout" names two findings.
- `docs/specs/coordinated-release.md`, "Compatible tap states", states C
  and D.
- `README.md`, `docs/USAGE.md`, `docs/ARCHITECTURE.md`, `docs/RELEASING.md`,
  `docs/specs/codex-skill-provisioning.md`,
  `docs/specs/harness-selection-and-execution.md`, the Grove formula template
  and `CHANGELOG.md`.

## Done when

- An enumeration of every candidate statement precedes each correction.
  The enumeration covers removed provisioning claims, embedded-file
  references, package inventories, native installation routes, `policy.ts`
  and `config.rhai`, `ravel@2` and the default skill directory. Each
  candidate is classified as current, historical or stale.
- Each stale statement is corrected. Valid historical records stay history.
- The Grove formula's caveats and samples cover both formats. Before
  promotion, directory delivery selects `ravel@2` explicitly. After
  promotion, a lone `policy.ts` refuses with `config_format_retired`, and the
  remedy names `config.rhai`.
- The guides document the `token_budget` limit of Codex. That feature lost the
  whole conversation, and the agent did not recover.
- The guides record the second finding as a known limit. After the loss of the
  whole preamble, `RAVEL_SKILL_DIRECTORIES` alone did not lead to recovery on
  either harness. They name a later design that puts the variable in the
  surviving system or developer instructions.
- The `Unreleased` changelog describes the compatible minor release for users.
- The source-exact walkthrough checks and `bash scripts/check.sh` pass.

## Notes

Do not change `plugins/grove` or `plugins/packs.json`. Either change moves
core's source digest and needs a new candidate keg, staging and dry run. If a
correction there is necessary, record it and carry that work as live leaves.
