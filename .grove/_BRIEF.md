# grove.remove-old-config-support — brief

## Goal

Remove support for non-modular configuration, as requested by the human. The
supported configuration language is the existing modular `config { ... }`
form, with command definitions, bindings, routes, and optional parameters and
profiles.

## Done when

- Flat top-level `kind "command template"` entries are rejected in personal
  configuration and the selected local delta, including documents mixing flat
  entries with a modular wrapper.
- Rejection identifies the source and gives an actionable modular-form remedy;
  invalid configuration cannot launch a session or mutate the task tree.
- The flat parser, legacy template scanner, literal-route replacement and
  parameter-reset compatibility paths, and legacy-only inspection machinery
  are removed where they have no modular consumer.
- Existing modular configuration behavior remains intact, including base-only
  configurations, selection/composition, parameters, provenance, safe argument
  expansion, and the personal/local authority boundary.
- Tests for retained behavior use modular fixtures. Packaged examples and
  current documentation describe and exercise the supported format. Old syntax
  appears only where it explains or tests rejection, or in historical evidence
  that does not claim current support.
- Configuration load/expansion tests pass with modular fixtures. Required
  repository checks remain green, including source-exact walkthrough validation
  for affected source changes.

## Decomposition

The modular reader already exists, so no expansion step is needed. Use two
green increments for the broad consumer migration and final removal:

1. `modular-fixtures-k2` migrates fixtures for retained behavior across the
   generic runner, Grove adapter and CLI consumers while both forms still load.
2. `modular-only-k3` removes non-modular support, adapts the remaining
   compatibility-specific assertions, and reconciles examples and durable docs.

The order keeps the consumer migration verifiable before deleting its old input
format. No additional design or planning leaf is needed for this settled change.

## Pointers

- Existing language contract: `docs/specs/modular-configuration.md`.
- Shared public interfaces: `docs/specs/module-decomposition.md`, runner section.
- Authority and execution decisions: `docs/adr/complete-session-configuration.md`
  and `docs/adr/untracked-configuration-delta.md`.
- Glossary terms: Grove configuration, Configuration profile, Command definition,
  Command parameter, Kind route, Command binding, Configuration delta.
- Reader and resolver: `crates/keyed-launch/src/templates.rs` and its `named`
  module; the consumer adapter is `SessionConfig` in `grove-loop`.
- Operator surfaces: configuration reference, usage guide, packaged modular
  examples, and `grove config show` / `grove config examples` acceptance tests.
- Required repository check entry point: `bash scripts/check.sh`.

## Notes

The requested removal is already decided; it does not need a deprecation stage,
a second supported grammar, or automatic rewriting of personal configuration.
Profiles and parameters remain optional. Empty/comment-only documents keep
their existing behavior; removing flat syntax does not require a new wrapper
where no declarations exist.

Update the existing durable specs, ADRs, glossary and guides in place as the
implementation changes their contracts. This brief is the work order, not a new
durable specification. Legacy task-tree layouts, driver epochs and other uses
of the word "legacy" are unrelated to this configuration removal.

## Test seams

The human's direction is: "We just need to make sure the config load/expands
tests work." The agreed acceptance seam is public configuration loading and
expansion, through Catalog/Templates and the existing SessionConfig tests.
Preserve the modular behavior those tests exercise; convert successful flat
fixtures to modular form and retain meaningful load-error assertions for
unsupported top-level entries. Legacy-only success expectations go away.

No new CLI/fake-executable acceptance suite is required. Existing CLI tests may
need fixture migration to keep their current assertions working; that is
consumer maintenance, not an expansion of the agreed testing scope. Existing
repository checks still apply to the files changed by implementation.

## Fixture migration handoff to modular-only-k3

`modular-fixtures-k2` and `consumer-fixtures-k5` delivered the retained runner,
loop, CLI, admission and TUI migrations, including `testing/support.rs`.
`bash scripts/check.sh` passes on the combined result. Remaining compatibility
cases belong to removal:

- Runner flat shape/eager validation, flat inspection, mixed routes and literal
  parameter-reset cases are enumerated in
  `02-k2/01-DONE-impl--runner-fixtures-k4.md` under Compatibility handoff.
- `crates/grove-loop/tests/session_config.rs` retains the two `legacy_delta_*`
  cases for flat shape aggregation and eager local-template validation.
- `crates/grove/tests/config_show.rs` retains the two `legacy_json_*` cases for
  flat-route null fields and literal/reset histories. Preserve useful schema,
  provenance-reference and no-write assertions while removing legacy variants.
- The packaged legacy example and its installer inventory remain for k3.
  Include the flat-generating smoke-test recipe in comments in
  `scripts/release-publish.sh` in the current-documentation sweep. Example
  installation tests deliberately retain opaque invalid active contents because
  installation must not read active policy.

The final consumer inventory is recorded in
`02-k2/02-k5/03-DONE-impl--admission-fixtures-k8.md`. No new CLI acceptance suite
was added; existing assertions and fake executables were preserved.
