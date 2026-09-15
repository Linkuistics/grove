# witness-foreign-holders-k42


## Goal
Prove observation-only preparation failure against foreign shared processes.



## Context
Consume k40/k41. LaunchWitnesses::prepare_with already has an after-create
barrier for a foreign holder of the new private file. Existing same-process
holder checks do not discharge this leaf's independent process requirement.

## Done when
- Separate shared-holder processes cover directory and private witnesses.
  Readiness proves their lock acquisition; an independent shared probe succeeds
  while exclusive preparation fails promptly. Real launch and admission still
  succeed with activity Unavailable. Reap and verify release/rollback.
- Preserve the containing-directory mutation lock, existing reported lock-error
  controls, bounded handoff/recovery and read-only/no-escaped-guard controls.
- Update affected books/docs and pass focused tests plus the principal gate.
- Reconcile all of k39 and k36, promote useful context and close only if their
  complete contracts hold. Name any remaining gap as a leaf; k37's native
  macOS/Linux evidence and k27's complete review remain separately live.

## Notes
Use bounded readiness/reap events, never elapsed sleeps as proof. Keep required
process tests named with witness_ for k37's cross-platform suite. No in-session
reviewer beside the scheduled complete protocol review.

## Decisions (running log)

Reuse the private `prepare_launch_using` seam with an explicit preparation
callback, defaulting to `LaunchWitnesses::prepare`. This lets the private-file
case enter the existing after-create barrier while preserving real epoch
invalidation, activation, publication and supervision. A self-exec shared
holder acknowledges its lock through stdout and holds until stdin release.
The parent owns cleanup and bounds preparation with a result channel, so a
blocking-lock regression fails while the foreign lock is still held. Both
cases assert real Started/Reaped callbacks, admission and public Unavailable,
then verify rollback, reap, exclusive reacquisition and next-launch recovery.

The independent exclusive contention probe uses its own explicit `LOCK_NB`,
so changing production locking to block cannot also block the parent's probe.
The parent retains the holder while awaiting preparation, and owns kill/reap
cleanup on a failed assertion or timeout. No new dependencies or public APIs.

## Verification

Host: Darwin 25.6.0 arm64, Homebrew rustc/cargo 1.98.1. Base revision
`7456571d` (k41), plus this leaf's diff. This is host shared-holder evidence;
k37 retains the paired native macOS/Linux process-death and exec-survivor run.

The required new process cases, under `driver_lease::witnesses::tests`, are:

- `witness_foreign_directory_holder_preserves_launch_and_admission`
- `witness_foreign_private_holder_preserves_launch_and_admission`

Both pass with separate self-exec holders, acknowledged shared acquisition,
a successful independent shared probe and contended independent exclusive
probe, preparation completed before release, real Started/Reaped callbacks,
admission and public Unavailable, successful child output, rollback and
post-reap exclusive reacquisition. Invalidation cleans the abandoned private
file, and subsequent ordinary preparation obtains a private witness again.
The private scenario checks the containing-directory mutation lock while the
root witness is exclusively held. `witness_foreign_shared_holder_process` is
the helper: its top-level invocation without the private environment is a no-op;
only the two scenario tests establish process evidence.

Sensitivity control: in `DriverLease::prepare_launch_using`, replace only the
preparation-error `eprintln!` with `return Err(error);`. Both scenarios fail at
mandatory preparation success, with the corresponding task-root/private
contention error. Restore the exact saved bytes before regression runs.
This control checks observation-only failure, independently of the two
release/order mutations already recorded by k38 and k41.

Frozen subjects for each run: every regular file recursively under `crates/`
and `.cargo/`, plus `Cargo.toml` and `Cargo.lock`, sorted by path. Manifest
lines are `<SHA256>  <repository-relative path>\n`. Compare every subject
before and after each run; restoration compared the whole manifest to baseline.
All comparisons matched. Baseline/restored manifest SHA256:
`5cae64fc67d6395dff4373c82d1b41ab3bb7f23682c34a96b0cc45afc88c39f1`; mutant manifest SHA256: `5e938b957fad3eae4ac8fd2f6da8cafd47e7c9f334e708902dec7ee29b863327`.

All filters below use `cargo test --locked -p grove-loop --lib <filter> -- --nocapture`.

| Run | Filter | Exit / result |
|---|---|---|
| baseline-foreign | `witness_foreign_` | 0; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 298 filtered out; finished in 0.52s |
| mutant-fatal-observation | `witness_foreign_` | 101; test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 298 filtered out; finished in 0.01s |
| restored-witness | `witness_` | 0; test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 276 filtered out; finished in 1.94s |
| restored-paired | `paired_` | 0; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 293 filtered out; finished in 0.51s |
| restored-handoff | `paused_runtime_reader_allows_shared_observers_and_bounds_driver_handoff` | 0; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 300 filtered out; finished in 0.00s |
| restored-guards | `capture_pause_and_returned_values_hold_no_epoch_or_tree_lock` | 0; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 300 filtered out; finished in 0.00s |

Restored witness cases (each passed):

- `driver_lease::observation::tests::witness_capture_uses_accepted_pin_after_root_path_replacement`
- `driver_lease::observation::tests::witness_directory_private_and_marker_error_matrix_obeys_final_probe`
- `driver_lease::observation::tests::witness_private_open_and_identity_io_errors_release_the_epoch_guard`
- `driver_lease::observation::tests::witness_release_private_first_and_both_released_are_idle`
- `driver_lease::observation::tests::witness_release_after_directory_verification_obeys_final_private_probe`
- `driver_lease::observation::tests::witness_epoch_preparation_cannot_attach_old_mandate_to_reused_tree`
- `driver_lease::observation::tests::witness_started_public_observation_identifies_the_prepared_launch`
- `driver_lease::observation::tests::witness_tree_absence_failure_and_contention_preserve_runtime_identity`
- `driver_lease::observation::tests::witness_missing_replaced_and_nonregular_evidence_is_unavailable`
- `driver_lease::observation::tests::witness_release_directory_first_reused_identity_and_key_cannot_attach`
- `driver_lease::witnesses::tests::witness_foreign_shared_holder_process`
- `driver_lease::observation::tests::witness_released_active_epoch_is_idle_despite_leftover_bytes`
- `driver_lease::observation::tests::witness_unlocked_matching_directory_is_busy_and_probes_escape_no_locks`
- `driver_lease::observation::tests::witness_marker_prefixes_invalid_bytes_and_release_have_exact_precedence`
- `driver_lease::tests::paired_witness_lease_drop_releases_both_before_replacement_cleanup`
- `driver_lease::observation::tests::witness_epoch_replacement_waits_with_old_bytes_and_idle_viewers`
- `driver_lease::observation::tests::witness_concurrent_shared_probes_of_released_started_bytes_stay_idle`
- `driver_lease::observation::tests::witness_open_and_probe_replacements_reject_stale_descriptors_and_bound_retries`
- `driver_lease::observation::tests::witness_concurrent_read_only_captures_cover_aliases_and_absent_controls`
- `driver_lease::observation::tests::witness_extension_validation_cannot_be_replaced_by_blanket_unavailability`
- `driver_lease::observation::tests::witness_real_launch_started_and_reaped_reach_public_observation`
- `driver_lease::tests::paired_witness_failure_preserves_real_launch_and_admission`
- `driver_lease::witnesses::tests::witness_foreign_directory_holder_preserves_launch_and_admission`
- `driver_lease::witnesses::tests::witness_foreign_private_holder_preserves_launch_and_admission`
- `driver_lease::observation::tests::witness_continuous_viewers_allow_repeated_real_launch_preparation`

The paired suite preserves reported locking-error and local rollback controls;
the two named handoff/guard controls preserve bounded recovery and no returned
advisory guards. Source-exact walkthrough validation passes for 16 files /
14,177 lines, with no deferred bytes. The book explains the process holder,
preparation synchronization, failure scope and release checks; architecture
records their host evidence boundary. No ADR decision changed.

## Parent contract reconciliation

| k39 / k36 obligation | Delivered evidence |
|---|---|
| Directory-first release with forced identity/key reuse; private-first, both released, post-directory release | k38's four native-probe schedules; all remain in the restored witness suite. |
| Directory verification mutation with real Running positive and exact restoration | k38's frozen mutation/restoration record. |
| Epoch-before-preparation, delayed replacement and independent mutation | k41's old-reader acquisition barrier, retained lease-byte snapshots, concurrent Idle observations, and forced-reuse mutation; real Running positive and source manifest recorded there. The preparation callback remains at the same post-invalidation point. |
| Compatible shared probes, released Started bytes, repeated launches with continuous viewers | k40's overlapping shared probes and acknowledged Started/Reaped cohorts; all pass in the restored witness suite. |
| Separate foreign shared-holder processes and observation-only preparation failure | This leaf's two cases and fatal-error sensitivity control. |
| Containing-directory mutation lock and reported locking errors | Private-holder preparation checkpoint plus the restored paired-owner error/rollback suite. |
| Aliases, absent trees, non-jj, missing namespaces, multiple viewers and read-only snapshots | k40's concurrent filesystem snapshots, preserved by the restored witness suite. |
| Bounded handoff/recovery and no escaped guards | The two named restored regression controls, plus each holder's bounded readiness/preparation/reap and post-reap exclusive probes. |
| Books, current architecture, focused tests and principal gate | Source-exact book and focused results above; final gate result below. |

No additional k39/k36 obligation was found. k37 still owns native macOS/Linux
process-death, exec-survivor and real replacement evidence and keeps k26 live.
k27 still owns complete protocol review and final public viewer binding; neither
is discharged by these concurrency controls. No in-session reviewer was used.

### Principal-gate failure and repair

The first principal run passed seven checks but failed the grove-loop library
error-matrix test: `witness_directory_private_and_marker_error_matrix_obeys_final_probe`
expected `directory, marker, private` and saw no probe events. Its output also
reported a wait for the exclusive test epoch lock. This test repeatedly acquires
an exclusive epoch descriptor and lacked the existing fork-sensitive wrapper.
The runtime read returns before witness probing when that epoch remains locked;
a parallel test's fork can inherit it until exec, the same descriptor lifetime
race documented by `fork_sensitive_driver_lease_test_body_runs_here`.

Add that existing isolation wrapper to this matrix test, preserving every
assertion and the production observer. The isolated case passes, and the whole
library then passes all 301 tests with normal parallel test execution. Its
source-exact fragment and ownership totals now include the three-line guard:
16 files / 14,180 lines, no deferred bytes. The source differs from the focused
mutation manifest above only by this test-isolation guard; no production change
followed the restored focused runs. Rerun the complete principal gate against
a fresh frozen source/docs/plugin/script manifest before retirement.

### Final gate and closure

`bash scripts/check.sh` exited 0: all eight principal checks pass, including
workspace tests and all six final book validations. The grove-loop book
reconstructs 16 files / 14,180 lines with no deferred bytes. The repaired
matrix and both new foreign-holder scenarios pass in the full workspace run.

Frozen gate subjects: every regular file recursively under `crates/`, `docs/`,
`plugins/`, `scripts/` and `.cargo/`, plus `Cargo.toml` and `Cargo.lock`
(1741 files). Every subject path and SHA256 matched after the run.
Manifest SHA256: `d6967b0df141a5d5695576b186b10fdf6825d1a14a15356984ed48f7462f09a7`.
Principal output SHA256: `a256a3b9c2c301cfe0db6e615847e329450b0678e2e681dcbd67bec0dc6ac4e1`.

k42's complete contract holds. The reconciliation above closes
witness-replacement-observers-k39 and witness-interleavings-k36; useful context
is promoted to `_witnessed-observation.md`. k26 remains live through k37, and
k27 retains complete protocol review and the final viewer integration. No
missing k39/k36 work requires another leaf.
