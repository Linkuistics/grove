# lifecycle-cutover-integrate-k41

**Kind:** integrate-review-impl
**Integrates:** lifecycle-cutover-review-k40

## Goal

Apply the verified findings from `lifecycle-cutover-review-k40` while preserving the reviewed artifact's contract.

## Context

- Verify every `lifecycle-cutover-review-k40` finding against the binding flow.
- Preserve the direct configured-command model; do not repair a finding by
  reconstructing harness identity or hidden defaults.

## Done when

- Every finding has a recorded disposition; verified issues are fixed through
  the bare-process/fake-command seam.
- Bare `grove` is the sole active lifecycle implementation for all non-finish
  states, even though obsolete compatibility surfaces await deletion.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Finish materialization and teardown belong to `finish-lifecycle-k43`.
