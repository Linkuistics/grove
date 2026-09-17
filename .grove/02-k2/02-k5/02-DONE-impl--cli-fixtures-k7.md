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

## Decisions (running log)

- Migrate the CLI's direct fixtures and both complete-policy generators to
  command/bind/route declarations; keep existing launch and refusal assertions.
  Alternate local executables are personal definitions selected by local bindings.
- Preserve the two JSON inspection cases whose subjects are legacy null
  command/binding fields and literal/reset provenance as explicitly named legacy
  cases for k3. Other JSON inspection coverage uses modular policy.
- Named-template failures carry command spans rather than legacy route names;
  assert the missing-prompt diagnostic, source, and byte location. Reload tests
  mutate selection inside the existing wrapper rather than adding a second one.
- Review confirmed the local route in the missing-personal-target test needs
  its required parameter values too: preserve an independently complete local
  command so missing parameters cannot mask an admission regression.

## Validation

- `cargo test --locked -p grove` passed during migration; final
  `bash scripts/check.sh` passed all eight checks, including workspace tests
  and all six source-exact walkthroughs. These integration-test files are
  outside the books' source corpora, so no book edits were needed.
- SHA-256 digests of all 1,792 tracked files matched before and after the final
  check, including source, test fixtures, manifests, scripts and documentation.
- One focused reviewer checked fixture semantics and remaining flat writers;
  its required-parameter finding is resolved above. No production code changed.
