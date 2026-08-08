# tree-access-lock-integrate-k56

**Kind:** integrate-review-impl
**Integrates:** tree-access-lock-review-k55

## Goal

Apply the verified findings from `tree-access-lock-review-k55` while preserving the reviewed artifact's contract.

## Context

## Done when

- [x] Reference-taking grow commands resolve and mutate under one exclusive tree guard.
- [x] No-argument `brief-chain` picks and assembles its chain under one shared tree guard.
- [x] `leaf-insert` reports renumbers and scans stale cross-references before releasing its exclusive guard.
- [x] Root initialization has a guard type that exposes only the lifetime it actually owns.
- [x] Focused regressions and the full locked test suite pass.

## Notes

### Review triage

- Integrated F1, F2, and F4 by moving CLI orchestration onto the existing lock-neutral read/grow helpers while one command-owned guard remains live.
- Integrated F3 by running and recording formatting plus the complete locked test suite.
- Integrated F6 with a distinct `RootInitWriteGuard`, removing the unused synthetic Grove root from that weaker contract.
- F5, F7, and F8 remain deferred to their existing owners: `driver-lease-k31`, `lifecycle-cutover-k39`, and `architecture-records-reconciliation-k88`. No duplicate leaves were added.

### Implementation

- Added command-level helpers for `leaf-add`, `leaf-add-chain`, `leaf-add-pair`, `leaf-insert`, and no-argument `brief-chain`; each acquires exactly one guard and performs reference resolution plus the operation through `*_unlocked` helpers.
- Kept `leaf-insert` output and cross-reference scanning inside an under-lock callback, preserving the existing stdout/stderr and stream-failure behavior.
- Retained pre-lock reserved-finish validation on the public grow APIs and repeated it at the unlocked seam used by the CLI.
- Added test-only acquisition counting and a conflicting-lock assertion so the regressions prove both one acquisition and that resolution/picking/reporting happen while the guard is live.

### Verification

- RED: `cargo test --locked --lib llm_cli::tests::reference_taking_commands_acquire_the_tree_lock_once` failed to compile before the command-level guarded helpers existed.
- RED: `cargo test --locked --lib rejects_finish_before_inspecting_the_tree` failed both new compatibility tests before pre-lock validation was restored.
- GREEN: `cargo test --locked --lib llm_cli::tests::` passed 2 tests; `cargo test --locked --lib rejects_finish_before_inspecting_the_tree` passed 2 tests.
- Focused integration suites passed: `cargo test --locked --test tree_access`, `cargo test --locked --test leaf --test leaf_chain --test pick`.
- Final `cargo fmt --check` exited 0.
- Final `cargo test --locked` exited 0, including 350 library tests and every integration/doc-test suite.
- One narrow fresh-context review found three compatibility/test-strength issues; all three were reproduced or mechanically verified, fixed, and included in the final green run.
