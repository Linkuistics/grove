# stage-evidence-reconciliation-k125

## Goal

Reconcile the stage evidence in the acceptance record, and decide which stage
checks need another run.

## Context

- `docs/grove-dual-ravel-stage-record.md`.
- The Ravel design evidence: the "Local stage record", "Local-sources stage
  record" and "Linux floor repair record" sections.
- The triage section of `docs/common-loading-acceptance-record.md`.

## Done when

- The record reconciles actual confinement on macOS and Linux. It states the
  backend of each result and which result came from a real harness.
- The record compares the native-only lifecycle with native standalone access
  on each route.
- The record reconciles the absence of native mutation on fresh install and on
  upgrade, and the dual-major archive, formula and floor evidence.
- The record states, for each stage check, whether it runs again in
  `integrated-acceptance-checks-k126`, and why.

## Notes

A Linux real-harness call needs a Linux sign-in. If none exists, state the
Linux real-harness results as gaps, apart from the Linux fixture results.
