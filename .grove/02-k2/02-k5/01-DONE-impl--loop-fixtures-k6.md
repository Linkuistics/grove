# loop-fixtures-k6


## Goal
Migrate grove-loop retained-behavior fixtures to modular configuration.



## Context
`crates/grove-loop/tests/session_config.rs`, `tests/driver_lease.rs`, and inline
launch fixtures in `src/driver_lease.rs` and `src/loop_driver.rs`.

## Done when
Retained fixtures are modular, semantic and source assertions remain meaningful,
grove-loop tests and `bash scripts/check.sh` pass, and covered walkthrough bytes
match the changed inline fixtures.

## Notes
Keep compatibility-only cases explicitly identified for `modular-only-k3`.

## Decisions (running log)

Personal fixtures define alternate commands; local fixtures select them through
bindings/routes. `source()` identifies the command definition's personal file,
so the provenance test also checks each winning route assignment's source.
Named-template failures occur at Catalog resolution and report command spans;
the cross-crate error test now resolves the catalog before comparing refusals.

The two `legacy_delta_*` cases retain flat-only shape aggregation and eager
local-template validation for k3: modular deltas cannot define templates.
Retained source-selection, profile-composition, admission and argv cases are
converted. Inline lease/observation/witness fixtures require matching fragments
in the grove-loop walkthrough; production behavior is unchanged.

The bounded independent reviewer found no actionable issues in the migration's
test-purpose preservation, admission, argv or provenance assertions.

## Validation

- SessionConfig baseline and final focused run each passed all 24 tests.
- `bash scripts/check.sh` passed all eight principal checks, including the
  locked workspace tests and all six final source-exact walkthrough validations.
- SHA-256 digests of all 1,792 tracked files matched before and after that run;
  the subjects included source, tests, manifests, scripts, fixtures and books.
