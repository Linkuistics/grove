# Item-status verification evidence

This preserves the native-platform evidence recorded by `witness-platforms-k37`
and the accepted limitations from the item-status implementation reviews.
These are historical results for the frozen source identified below, not a new
execution against the current checkout. The behavior contract is
[item-status](specs/item-status.md); the ownership and supported-backend boundary
is [one live driver per working tree](adr/one-live-driver-per-working-tree.md).

## Accepted review limitations

The observation reader accepts a plain witness basename; the cooperating writer
and cleanup use `witness-` plus 32 lowercase hexadecimal digits. A corrupted epoch
naming a matching control file could reach a shared probe there and briefly
interfere with replacement acquisition. Integration `witnessed-activity-k46`
retained that behavior within the ADR's repository-control corruption exclusion.
Reopen the grammar decision if control-record corruption becomes supported input.

Idle-observation review retained a test limitation: alias retargeting during
control discovery and replacement immediately before locking or after copying
have source-order checks but no deterministic injection tests. The guarded
replacement window and eight-attempt bound are tested. The duplicate selector
call was retained because both calls use the same snapshot and shared rule.

The later protocol integration added subprocess isolation to eleven observer
tests and clarified that the private probe uses its validated descriptor.
Its recorded comparison found production protocol bytes unchanged, preserving
the scope of the native results below. Its principal gate reported all eight
checks and six exact-source book validations passing. This document does not
extend those results to new platforms or claim a later native Linux rerun.

## Required native suite (enumerated before execution)


Each full name below must be listed and pass on each platform. The standalone
`witness_native_exec_child`, `witness_native_holder_process` and
`witness_foreign_shared_holder_process` return without their private environment;
only the scenario cases establish their process evidence.

| Required case | macOS | Linux |
|---|---|---|
| `driver_lease::observation::tests::witness_capture_uses_accepted_pin_after_root_path_replacement` | PASS | PASS |
| `driver_lease::observation::tests::witness_concurrent_read_only_captures_cover_aliases_and_absent_controls` | PASS | PASS |
| `driver_lease::observation::tests::witness_concurrent_shared_probes_of_released_started_bytes_stay_idle` | PASS | PASS |
| `driver_lease::observation::tests::witness_continuous_viewers_allow_repeated_real_launch_preparation` | PASS | PASS |
| `driver_lease::observation::tests::witness_directory_private_and_marker_error_matrix_obeys_final_probe` | PASS | PASS |
| `driver_lease::observation::tests::witness_epoch_preparation_cannot_attach_old_mandate_to_reused_tree` | PASS | PASS |
| `driver_lease::observation::tests::witness_epoch_replacement_waits_with_old_bytes_and_idle_viewers` | PASS | PASS |
| `driver_lease::observation::tests::witness_extension_validation_cannot_be_replaced_by_blanket_unavailability` | PASS | PASS |
| `driver_lease::observation::tests::witness_marker_prefixes_invalid_bytes_and_release_have_exact_precedence` | PASS | PASS |
| `driver_lease::observation::tests::witness_missing_replaced_and_nonregular_evidence_is_unavailable` | PASS | PASS |
| `driver_lease::observation::tests::witness_native_exec_child` | PASS | PASS |
| `driver_lease::observation::tests::witness_native_holder_process` | PASS | PASS |
| `driver_lease::observation::tests::witness_native_killed_holder_releases_both_locks_with_exec_survivor` | PASS | PASS |
| `driver_lease::observation::tests::witness_native_replaced_root_reports_previous_tree_to_new_observer` | PASS | PASS |
| `driver_lease::observation::tests::witness_open_and_probe_replacements_reject_stale_descriptors_and_bound_retries` | PASS | PASS |
| `driver_lease::observation::tests::witness_private_open_and_identity_io_errors_release_the_epoch_guard` | PASS | PASS |
| `driver_lease::observation::tests::witness_real_launch_started_and_reaped_reach_public_observation` | PASS | PASS |
| `driver_lease::observation::tests::witness_release_after_directory_verification_obeys_final_private_probe` | PASS | PASS |
| `driver_lease::observation::tests::witness_release_directory_first_reused_identity_and_key_cannot_attach` | PASS | PASS |
| `driver_lease::observation::tests::witness_release_private_first_and_both_released_are_idle` | PASS | PASS |
| `driver_lease::observation::tests::witness_released_active_epoch_is_idle_despite_leftover_bytes` | PASS | PASS |
| `driver_lease::observation::tests::witness_started_public_observation_identifies_the_prepared_launch` | PASS | PASS |
| `driver_lease::observation::tests::witness_tree_absence_failure_and_contention_preserve_runtime_identity` | PASS | PASS |
| `driver_lease::observation::tests::witness_unlocked_matching_directory_is_busy_and_probes_escape_no_locks` | PASS | PASS |
| `driver_lease::tests::paired_witness_failure_preserves_real_launch_and_admission` | PASS | PASS |
| `driver_lease::tests::paired_witness_lease_drop_releases_both_before_replacement_cleanup` | PASS | PASS |
| `driver_lease::witnesses::tests::witness_foreign_directory_holder_preserves_launch_and_admission` | PASS | PASS |
| `driver_lease::witnesses::tests::witness_foreign_private_holder_preserves_launch_and_admission` | PASS | PASS |
| `driver_lease::witnesses::tests::witness_foreign_shared_holder_process` | PASS | PASS |

### Sensitivity controls

The new killed-holder test passed before mutation. Separately clearing FD_CLOEXEC
on the prepared directory descriptor and on the private descriptor made it fail
at `shared(&directory_probe)` and `shared(&private_probe)` respectively after
holder reap and the survivor ping (exit 101 each). Both were restored byte for
byte. Subjects: all regular files recursively under crates/ and .cargo/, plus
Cargo.toml and Cargo.lock; sorted SHA256/path manifest compared before/after
each run and after restoration. Baseline manifest SHA256:
`31ed464fee5109e89e929a399ef6e3b749985ad6265b1ec65ddfe4fcdc799f06`.
Directory mutant: `c4dde9db549470f75efcec8995725e86d873056702ade64c0f0ca006bfe9d874`;
private mutant: `14e1bcad21c39c2881808203ef9dc781443ff31c35e90f0db0d2cdd67d2dba08`.
Filter: `cargo test --locked -p grove-loop --lib witness_native_killed_holder -- --nocapture --test-threads=1`.
The new scenarios use existing production APIs; no production behavior changed.
Complete protocol review remains assigned to k27.

## Native platform results

Both commands exited 0. Each actual `--list` matched the nonempty required list
above before execution. Each per-case result matched that list afterwards:
29 passed, zero failed/ignored on each host, including the two foreign holders.
Standalone helper PASS entries are not independent process evidence.

| Platform | Kernel / architecture | Source and fixture filesystem | Toolchain | Runtime |
|---|---|---|---|---|
| macOS | Darwin 25.6.0 arm64 | APFS, `/private/tmp/k37-native-source`; TempDir under host `/var/folders/.../T` on the same Data volume | Homebrew rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05) | 3.24 s |
| Linux | Linux 6.10.14-linuxkit aarch64 | overlayfs, `/tmp/work` and `/tmp` | rustc 1.85.1 (4eb161250 2025-03-15), cargo 1.85.1 (d73d2caf9 2024-12-31) | 4.21 s |

Linux used the parent's exact pinned image digest
`sha256:e51d0265072d2d9d5d320f6a44dde6b9ef13653b035098febd68cce8fa7c0bc4`,
Docker `desktop-linux`, uid/gid 1000:1000, `CARGO_HOME=/tmp/cargo`, `TMPDIR=/tmp`,
and `--init` to reap the orphaned exec survivor. No bind mounts or inherited loop
control variables. The child responds after holder reap, then exits its socket
protocol; only the holder's exit is asserted through the parent's actual wait.

Frozen source: jj change `wprupsrvmtxvskmmpysmllmpoqpyykuk`, revision
`8aa00937c51ee04dc7cba3821fa9f215348e0490` (base `3d31f47a`, plus this leaf's
source/book/test-list diff). The archive contains all 1,795 paths from
`jj file list`, including Cargo.lock and .cargo/config.toml, plus a SHA256
manifest and the previously enumerated required-test list. No production/test
source edits follow this snapshot; subsequent changes record results, reconcile
architecture/briefs and retire the task.

- Source archive SHA256: `87a0382ea04a2a6178250d75a76ce82aac6b0886093a98952fe4cae14184e7d4`.
- Sorted SHA256/path manifest: `42d19f9d416ce5d07e8044e935ff3f77424ff31ab8dfb792cbaa8f5f48d65d57`.
- macOS run log: `cd7c27e49756ba62ae9ea02e18dda547254ac28b32cc117f71b49b9a4dd1e263`.
- Linux run log: `ab9e8a0f8ce2e1a416e54eddf1e72ae2dddd38124a84ca181e840a8fdb490009`.

Every archived file digest matched before and after each run. Each platform
extracted its own copy of the same source.tar. macOS used `shasum -a 256 -c
subjects.sha256`; Linux used `sha256sum -c subjects.sha256`. Both compared all
per-file validation results, not just a total. The required list and archive
were frozen before either run. Local raw artifacts are at `/tmp/k37-evidence/`;
this task preserves the commands and results independently of those scratch files.

Both hosts ran, from their extracted source directory:

```sh
cargo test --locked -p grove-loop --lib witness_ -- --list
cargo test --locked -p grove-loop --lib witness_ -- --nocapture --test-threads=1
```

Linux execution (source.tar includes subjects.sha256 and required-tests.txt):

```sh
docker --context desktop-linux run --rm --init -i --user 1000:1000 \
  --env CARGO_HOME=/tmp/cargo --env TMPDIR=/tmp --workdir /tmp \
  rust@sha256:e51d0265072d2d9d5d320f6a44dde6b9ef13653b035098febd68cce8fa7c0bc4 \
  sh -ec 'mkdir work; tar -xf - -C work; cd work;
    uname -srm; id; rustc --version; cargo --version;
    stat -f -c %T .; stat -f -c %T /tmp;
    sha256sum -c subjects.sha256 > /tmp/before.log;
    cargo test --locked -p grove-loop --lib witness_ -- --list > /tmp/list.txt;
    sed -n "s/: test$//p" /tmp/list.txt > /tmp/actual.txt;
    test -s /tmp/actual.txt; diff -u required-tests.txt /tmp/actual.txt;
    cat /tmp/actual.txt;
    cargo test --locked -p grove-loop --lib witness_ -- --nocapture --test-threads=1;
    sha256sum -c subjects.sha256 > /tmp/after.log;
    cmp /tmp/before.log /tmp/after.log; echo SUBJECTS_UNCHANGED' < source.tar
```

