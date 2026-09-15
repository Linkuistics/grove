# witness-concurrent-viewers-k40


## Goal
Prove compatible read-only observation and preparation under continuous viewers.



## Context
Consume k38's native release fixture and public real-launch positive. The
runtime observer's private tests own these controls. Replacement under old
lease bytes and its mutation are k41; foreign processes are k42.

## Done when
- Concurrent public observers report Idle on released Started bytes even with
  overlapping shared probes of both witnesses; returned values retain no locks.
- Continuous public observers span repeated real Started/Reaped launches;
  preparation succeeds each time, every held Started checkpoint is SameTree
  Running for its exact signal, and each Reaped checkpoint is Idle.
- Concurrent snapshots preserve tree/administration bytes for exact aliases,
  absent trees, non-jj trees and missing namespaces, with bounded event waits.
- Existing handoff/recovery controls pass, affected books explain and reconstruct
  the tests, and the focused suite and principal gate pass.

## Notes
This is host concurrency evidence, not k37's paired native platform result.

## Decisions (running log)

Use independent shared descriptors to hold open the overlap window on released
witnesses while public observers run. Continuous observer threads sample between
acknowledged Started/Reaped checkpoints; the runner's synchronous callbacks hold
each checkpoint stable. Timeouts detect failure, never establish ordering.

Use compatible shared release checks while continuous viewers remain active;
an exclusive check there could collide with a legitimate short viewer probe.
The independent exclusive no-escaped-guard check runs after the cohort stops.

## Verification

Host: macOS (Darwin), arm64. Base revision `521826c4` plus this task's diff.
No production behavior changes. Added three tests under
`driver_lease::observation::tests`:

- `witness_concurrent_shared_probes_of_released_started_bytes_stay_idle`:
  three shared holders per witness, three viewers, sixteen acknowledged Idle
  checkpoints, filesystem equality and exclusive post-cohort guard checks.
- `witness_continuous_viewers_allow_repeated_real_launch_preparation`:
  three viewers across four `/bin/sh -c true` launches; both prepared witnesses
  contended, all Started checkpoints SameTree Running with the current signal,
  all Reaped checkpoints Idle, and each subsequent invalidation/preparation
  succeeds. Runtime callbacks and before/after byte snapshots establish order.
- `witness_concurrent_read_only_captures_cover_aliases_and_absent_controls`:
  direct and symlink paths with three viewers, sixteen checkpoints per case;
  absent non-jj tree, present non-jj tree, missing jj namespace, absent jj tree.
  Snapshots and alias targets remain unchanged.

Sensitivity control: replace only `LOCK_SH` with `LOCK_EX` in the production
`WitnessIo::probe` acquisition. Command
`cargo test --locked -p grove-loop --lib witness_concurrent_shared_probes_of_released_started_bytes_stay_idle -- --nocapture`
exited 101: the assertion showed three false SameTree Running values instead
of Idle. Restored the original source bytes in a `finally` block and checked
exact equality. This is a probe-compatibility control, not k41's still-required
epoch-before-preparation mutation.

Source SHA256 (`crates/grove-loop/src/driver_lease/observation.rs`):
restored `29e9400bdf59c849ff70b03e01eeaa19e466a4baf439a94d177086683f85702c`;
mutant `acf2a79f205873f9f0a1bef798d4ff56287536b5289f29791e718e7fc03b48af`.
After restoration,
`cargo test --locked -p grove-loop --lib driver_lease::observation::tests -- --nocapture`
passed all 27 tests, including both existing pause/handoff controls and the
real-launch positive. No test was ignored.

The walkthrough explains/reconstructs the four new helper/test fragments;
its manifest, source ledger, chapter and book totals include the added bytes.
The architecture states the scope and limits of these host controls. k41 and
k42 keep k39/k36 live; k37 still owns native platform evidence and k27 complete
protocol review. No ADR decision changed.

Final gate: `bash scripts/check.sh` exited 0 with all eight principal checks
passing, including workspace tests and all six final book validations. The
loop book reconstructs 16 files / 13,755 lines with no deferred ranges.
Host details: Darwin 25.6.0 arm64, Homebrew rustc/cargo 1.98.1.

Frozen gate subjects: every regular file recursively under `crates/`, `docs/`,
`plugins/`, `scripts/` and `.cargo/`, plus `Cargo.toml` and `Cargo.lock` (1,741
files). Sorted manifest lines are `<SHA256>  <repository-relative path>\n`.
Every path and digest matched after the run; manifest SHA256
`d7889fcae910d5ff3fd394d6345a23dce0cd3ae4247d94e8eec6e566b3568563`.
Principal output SHA256:
`17732c090e77906f3bd8847105ab09fbafcea39667ff89d8e20b88e9caece13a`.
Restored focused-test output SHA256:
`4a910a2bc5ba2def30e237e22f1a7385d568756a7bde36d41d11bd0613ef43c1`.

The leaf's close conditions hold. No parent closes: k41 and k42 retain the
remaining k39/k36 obligations explicitly.
