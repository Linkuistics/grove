# witness-platforms-k37


## Goal
Obtain and record exact native macOS/Linux process evidence for production
witnessed observation, completing k26's platform obligations.



## Context
Use the parent witnessed-activity-k12 brief's verified Docker digest, uid 1000
and native overlayfs route. The full k26 brief is the evidence contract.

## Done when
- Real launched-session positives reach try_observe. Independent shared probes
  contend on both witnesses after acknowledged readiness; kill and actual reap
  leave both probes successful despite Started bytes. An exec survivor retains
  neither witness and cannot manufacture Running.
- Rename/remove then replace the root with a reused key: a new observer reports
  previous tree. Name required native cases with witness_ and enumerate them
  before executing; zero selected tests cannot pass.
- Record exact test names and per-case results, jj revision/source digest,
  command, kernel/filesystem/architecture for both native platforms. Freeze and
  digest subjects before/after each run. No host bind mount as locking subject.
- Reconcile the node's full acceptance obligations and source-derived books;
  focused tests and principal gate pass. Preserve backend/suspension caveats
  and k27's witnessed-view deferral and complete-protocol review obligation.

## Notes
Keep this leaf and k26 live without both platform results. If Docker becomes
unavailable, prepare exact source/commands/prerequisites and macOS results before
requesting the human Linux run specified by the parent brief. Native process
controls belong here with their evidence, not in a later catch-up task.

## Decisions (running log)

- Use the approved self-exec private test seam: a separate holder prepares a
  real launch, and an exec'd child acknowledges readiness over a Unix socket.
  Independent parent descriptors probe both locks before and after holder
  SIGKILL/actual reap; the child answers a ping after reap. This catches missing
  exclusive ownership, leaked descriptors across exec, and stale Running.
- A second scenario renames/removes the pinned root and creates a new root with
  key k1, asking a fresh production observer for PreviousTree. No numeric
  identity override is used; forced reuse remains k38/k41's evidence.
- The graph generation 2026-09-15T11:44:22Z lacks the newer witness modules;
  coverage reports not_tracked/metadata_changed. Read the relevant source
  directly rather than treating the empty symbol search as absence.
- The exact planned Docker digest was retrieved successfully after its local
  image lookup failed. Keep uid 1000 and container-native fixture paths.

## Execution plan

1. Extend the observer's private tests with holder/exec-child fixtures and the
   two native scenarios; retain real Started/Reaped positive controls.
2. Run focused cases, then reconcile the epoch walkthrough's literal fragments,
   indexes and manifest. Product behavior and k27's deferral do not change.
3. Freeze tracked source into an archive; enumerate the witness suite before
   executing it on native macOS and Linux, compare every case, and record
   fingerprints/platforms. Run the principal gate after final edits.
4. Reconcile k26's contract, retire this leaf and any satisfied parent, describe
   the jj change, and signal completion as the final action.

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

## k26 acceptance reconciliation

| Parent obligation | Delivered evidence |
|---|---|
| Typed Running mandate, independent runtime comparison and verified tree relation | k35 public observer, accepted-pin replacement and tree-error controls; new native positive and replacement cases. |
| Directory-before-private precedence, markers, Idle after release, Busy/Unavailable distinctions | k35 exact marker/error matrices and k38 release schedules, rerun on both platforms. |
| Read-only nonblocking probes, bounded records/retries, grammar, substitutions and I/O errors; unchanged admission | k34/k35 parser, filesystem and replacement controls; full principal regression gate. |
| Real Started/Reaped and killed/reaped holder, both native platforms, exec survivor, real replacement | This leaf's per-case platform table and two independent descriptor-leak sensitivity controls. |
| Forced identity/key reuse and both production-order mutations with Running positives | k38 directory-check and k41 epoch-before-preparation mutation/restoration records; all restored scenarios rerun here. |
| Concurrent observers, released Started bytes, waiting replacement, foreign shared holders | k40/k41/k42 controls, all rerun on both platforms including each foreign holder. |
| Capture/runtime pauses, handoff bound and recovery, no returned guards, filesystem preservation | Existing guard/pause cases retained by the principal suite; k40 concurrent snapshots rerun on both platforms. |
| Conservative Viewer handling until k27, independent captures | k35 Viewer regression remains; this leaf changes no production or viewer behavior. |
| Documentation and limitations | Epoch book reconstructs 16 files / 14,444 lines; indexes, totals and architecture reconciled. Native backend preconditions and suspension risk remain in the ADR. G6 and complete protocol review remain k27's responsibility. |

No additional k26 work is identified. Its closure requires the final principal
gate below. k12 remains live through witnessed-view-k27, which owns public
RUNNING/NEXT binding, G6 reconciliation and complete protocol review. No
competing in-session reviewer was materialised and no ADR decision changed.

## Final gate and closure

`bash scripts/check.sh` exited 0: all eight principal checks passed, including
workspace tests and all six final walkthrough validations. The new native
scenarios passed in the full parallel workspace run as well as in each frozen
platform run. The grove-loop book reconstructs 16 files / 14,444 lines with zero
deferred ranges. All existing admission, handoff, pause and Viewer regressions
remain green.

Gate subjects: all 1,795 regular-file paths from `jj file list`, including
source, Cargo manifests/lock/config, fixtures, scripts, plugin skills, books and
task notes. Each SHA256/path entry matched before and after the run.
Manifest SHA256: `b1b3c94cd8335aebf8a67aec501472064d411847f301481b8ab40fdb80f6fbce`.
Gate output SHA256: `b45fa0689a834c5f9821255fd02b642358f4f2bb29c3942914149a852b9bf35e`.
Only this gate/closure record, the parent closure sentence and retirement follow
that measurement.

The acceptance reconciliation above now closes witnessed-observation-k26.
Useful context and the platform evidence pointer are promoted to k12's brief.
k12 remains live through witnessed-view-k27; no work is added or silently
transferred, and no ADR decision changed.
