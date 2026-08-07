# legacy-launch-removal-k46

**Kind:** impl

## Goal

Contract the obsolete human-command, harness-stamp, environment-routing, and
hidden harness-specific launch surfaces now that the bare configured driver is
complete.

## Context

- Depends on `finish-lifecycle-integrate-k45`.
- Binding removal list: `docs/specs/config-driven-sessions.md` section "Removed
  surfaces and compatibility" and the root brief.
- Primary code surfaces include `src/cli.rs`, `src/launch.rs`,
  `src/loop_driver.rs`, `src/harness.rs`, `src/harness_stamp.rs`, `src/lib.rs`,
  `tests/launch.rs`, `tests/harness*.rs`, `tests/loop_driver.rs`, help fixtures,
  `.gitignore`, `Cargo.toml`, and release metadata.
- This is a wide mechanical contract step. Preserve provisioning's independent
  target-directory registry even though no registry may choose launch policy.

## Done when

- Human `do`/`migrate`/`retire`, `--harness`, `--no-launch`, dry-run routing,
  stamp creation/lookup, `.grove-stamps/` rules, and their tests/fixtures are
  deleted; bare `grove`, `--help`, and `--version` are the complete human CLI.
- All `GROVE_<KIND|FAMILY>_HARNESS`, `GROVE_*_MODEL`, executable, skill-dir,
  kill-grace, and `GROVE_LLM_BIN` user overrides are removed from production
  and tests; internal tool/clock/grace injection remains module-local.
- Harness/model inference, sandbox probing/grants, hidden naming/model/Herdr
  argv, and `HERDR_AGENT` injection are gone from the driver. The configured
  argv and visible `${herdr_settings}` splice are the only launch policy.
- `start.md`/`retire.md` code paths are unreachable pending their artifact
  deletion in `methodology-and-viewer-k48`; provisioning still refreshes every
  installed known skill directory before driver ownership.
- Removed-surface sweeps use positive and cross-tree controls, and the full
  `cargo fmt --check` / `cargo test --locked` suite passes.

## Notes

Do not remove composition relationships or review evidence here;
`legacy-review-removal-k47` owns that independently rejectable contraction.
