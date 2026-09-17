# cli-fixtures-k7


## Goal
Migrate retained grove CLI configuration, loop and lifecycle fixtures to modular form.



## Context
Start with `crates/grove/tests/config_show.rs`, `loop_driver.rs`, and
`lifecycle_cutover.rs`; inspect other configuration writers in that crate too.

## Done when
Existing consumer assertions retain their purpose under modular fixtures;
crate tests and `bash scripts/check.sh` pass. Reconcile covered walkthroughs.

## Notes
Define alternate executables in personal commands, redirect local routes/bindings,
and preserve admission and diagnostic semantics. Identify compatibility-only
tests for k3. Keep fake executables and the cargo signal guard.
