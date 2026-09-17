# runner-fixtures-k4

## Goal

Migrate generic keyed-launch fixtures for retained loading, expansion, diagnostics,
conformance and process behavior to modular declarations. Preserve dedicated
flat/mixed compatibility assertions for `modular-only-k3` and identify them.

## Done when

- Retained generic tests use modular fixtures and preserve their assertions.
- Semantic errors still reach the intended validation, with real source spans.
- `cargo test -p keyed-launch` and `bash scripts/check.sh` pass.

## Decisions (running log)

- Split `modular-fixtures-k2` into this generic batch and `consumer-fixtures-k5`:
  the generic runner and downstream consumers have independent test seams, and
  migrating both with their diagnostic differences exceeds a focused session.
- Keep dedicated legacy eager-validation, flat shape and literal-reset cases
  until `modular-only-k3`; these assert behavior specifically being removed.
  Migrate retained command semantics through active modular routes.
- Modular semantic failures belong to resolution, so the conformance failure case
  now checks the report returned by `conformance::check`. Aggregate template
  errors assert command identity and exact captured source spans.
- `Templates::source` remains the command definition's personal path after a
  local redirect; the redirect test separately asserts the binding assignment's
  local origin. Local files contain no command definitions in retained tests.
- The one in-session reviewer found a valid actionable gap: the converted
  structural-suppression test only called `Catalog::load`, which cannot observe
  modular semantic failures. It now uses `Templates::load` and a paired control:
  removing only the structural fault exposes both active invalid templates.
  The executable control covers the correction; no second review is needed.

## Compatibility handoff to modular-only-k3

Dedicated old-format cases remain in `crates/keyed-launch/tests/`:

- `templates.rs`: invalid unadmitted overlay eager validation, old key shapes
  (child/property, annotations, argument count/type, duplicate locations), and
  eager NUL templates in both files. Retain modular NUL coverage in
  `named_commands.rs` while removing the eager compatibility case.
- `catalog.rs`: the flat `select`/`profile` names at the end of the selection
  shape test, and `flat_catalog_keeps_eager_validation_and_refuses_wrapper_shapes`.
- `diagnostics.rs`: `eager_template_errors_aggregate_after_structure_passes`.
- `inspection.rs`: `captured_flat_inspection_retains_history_and_matches_expansion`
  checks `LiteralTemplate` history; modular inspection is covered by the named
  command, parameter and profile tests.
- `named_commands.rs`: mixed flat/route duplicate in the structural matrix,
  the legacy dollar rule at the end of the scanner test, the self-contained
  flat-to-named redirect test, and the literal-reset portions of
  `route_switches_preserve_maps_and_literal_replacement_records_resets`.
- `profiles.rs`: `literal_transitions_reset_profile_parameters_and_record_the_reset`.

Other downstream fixtures remain the responsibility of `consumer-fixtures-k5`.
The keyed-launch walkthrough corpus excludes integration tests, so this batch
changes no reconstructed source blocks.

## Validation

- `cargo test -p keyed-launch --no-fail-fast` passed; the review correction also
  passed focused diagnostics and named-command runs.
- Final `bash scripts/check.sh` passed all eight principal checks, including
  `cargo test --locked --workspace` and all six final walkthrough validations.
- SHA-256 manifests of every tracked non-`.grove/` file matched before and after
  the final check. These subjects include source/tests, manifests, fixtures,
  scripts, skills and documentation. An earlier check run was stopped before
  the review correction and is not the completion evidence.
