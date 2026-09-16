# workspace-selection-k29


## Goal

Connect SessionConfig to Catalog's local/default/empty selection policy, exposing
the same compiled inspection and structured diagnostics used by launches.

## Implementation plan

- Replace selection-guard tests in session_kind_tree and lifecycle_cutover with
  successful empty/default/local selection and active-error refusal cases.
- Add public SessionConfig tests for replacement, provenance, local completion,
  authority and global validation, and discovery/admission diagnostic records.
- Resolve the captured overlay declaration, else primary declaration, else an
  empty Selection in session_config.rs. Preserve existing driver load points.
- Expose Inspection through SessionConfig and diagnostic records through Grove's
  Error, retaining generic ConfigError records and source-attributed local errors.
- Update current configuration documentation and all affected source-exact books;
  run focused tests, then bash scripts/check.sh before retirement and commit.

## Done when

The adapter selects and validates profiles through mutation and launch, preserves
source admission, exposes inspection/errors without a second resolver, and the
focused acceptance plus principal checks pass. Broader workspace arrangements,
isolation, adversarial launched argv and live-child selection/value reload belong
to workspace-reload-k30.

## Decisions (running log)

- The reviewed design already fixes policy and interfaces; implement it without
  reopening requirements. Split k10 at its explicitly suggested usable adapter
  boundary so multi-workspace/reload acceptance gets its own session.

- Expose `SessionConfig::inspect()` and `Error::diagnostics()` using the existing
  runner records. Local discovery/admission failures have an Overlay source path
  and no fabricated span; non-configuration errors return an empty slice.
- Keep generic ConfigError in the error chain and attach its structured source
  paths for human rendering: acceptance found unknown-profile Display omitted
  paths even though the records retained them. No generic resolver change needed.
- One bounded reviewer checks adapter admission/selection and error transport;
  broader isolation/reload remains k30's explicit charter.
- The single adversarial review found no actionable issues in adapter policy,
  source admission or diagnostic transport. No second review is needed.
- Expanded acceptance found a preexisting driver bootstrap gap: a valid snapshot
  without requirements authority creates the initial tree before expansion
  refuses the kind. Externalized as bootstrap-kind-admission-k31 before k30;
  k29 checks active global errors before creation and local-only refusal on an
  existing tree, without claiming that missing-kind bootstrap gap repaired.


## Verification

- Public SessionConfig suite: 24 passing tests; focused real leaf-add and driver
  selected-policy acceptance pass. The old selection guard was observed failing
  the new adapter test before replacement. Source-attribution acceptance exposed
  and verified the human error-chain repair.
- `bash scripts/check.sh`: all eight checks passed, including workspace tests
  and final source-exact validation of all six books. Grove-loop reconstructs
  16 files / 14,594 lines with no deferred fragments.
- SHA-256 comparison of all 1,809 versioned input paths before and after the
  principal run reported no changes. Only task-note cleanup, this verification
  record and retirement bookkeeping follow that measurement.
- k10 remains open with bootstrap-kind-admission-k31 and workspace-reload-k30;
  no parent close is claimed. The ADRs already describe the intended authority
  and composition contract and need no decision change in this increment.
