# routing-policy-removal-k82

**Kind:** impl

## Goal

Remove obsolete environment routing, target inference, and hidden launch policy
behind the already-complete bare configured driver.

## Context

- Depends on `legacy-command-surface-removal-k77`.
- Primary surfaces: `src/launch.rs`, `src/loop_driver.rs`, `src/harness.rs`,
  configuration guards, environment scrubbing, and launch tests.
- Preserve provisioning's target-directory registry and module-local test
  injection; neither chooses user launch policy.

## Done when

- All `GROVE_<KIND|FAMILY>_HARNESS`, `GROVE_*_MODEL`, executable, skill-dir,
  kill-grace, and `GROVE_LLM_BIN` user overrides are absent from production and
  tests.
- Harness/model inference, sandbox probing or grants, hidden naming/model/Herdr
  argv, and implicit `HERDR_AGENT` injection are gone; configured argv and the
  visible `${herdr_settings}` splice are the only launch policy.
- Direct-process launch tests, `cargo fmt --check`, and `cargo test --locked`
  pass.

## Notes

The prior child leaves the product working, so this contraction does not need
to land atomically with the public CLI removal.
