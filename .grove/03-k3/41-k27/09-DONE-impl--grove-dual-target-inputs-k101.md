# grove-dual-target-inputs-k101

## Goal

Add an explicit 2.0 lane to Grove's pinned independent Ravel inputs, target
smoke and Linux floor checks. Keep the 1.1.0 lane and legacy-driver continuity.

## Context

- `scripts/ravel_target_inputs.py`, its tests, `scripts/ravel-target-smoke.sh`,
  `scripts/grove-floor-native-cases.sh` and the floor tasks.
- `scripts/legacy_driver_rehearsal.py` and `docs/legacy-driver-transition.md`.
- Ravel's `task release:archives` builds exact unreleased target archives with
  no version cut.

## Done when

- The target inputs keep the 1.1.0 pins. A second lane accepts 2.0 archives
  identified by source commit and build identity. Before the cut, these come
  from `task release:archives` at the directory build.
- The 2.0 lane refuses an archive whose digest or identity differs from its
  record. It never tests whatever the PATH finds.
- The target smoke and the Linux floor cases run each lane with the configuration
  format of its runtime. Each target that does not run is named.
- `legacy:controls` passes. `legacy:rehearsal` passes with the 1.1.0 keg while
  a 2.0 prefix is also installed. The ending executable of the old driver stays
  usable after each package step.
- A written handoff states how rollout replaces the local 2.0 identities with
  the published archives' SHA-256 before Grove's release.

## Notes

Never change `/opt/homebrew/Cellar/ravel/1.1.0`. It is the ending executable
of the live driver. Build archives in scratch directories.

Controls: an archive with one changed byte must refuse. A 2.0 lane that writes
`policy.ts` must fail its smoke. Watch each fail once.

From grove-directory-confinement-k99: the floor cases now hold an
unreadable-native-guidance case for 1.1.0. Run it on the 2.0 lane with
`config.rhai`. No floor measures the confined directory case on Linux until
that lane exists. Port the macOS case in `crates/grove/tests/directory_route.rs`
to the 2.0 lane, or name it as unmeasured.

On 2026-11-03,
`release:floor:native:run` refused a freshly prepared aarch64 bubblewrap,
SHA-256 `d35694e8915a70cf6ef403639c4d6c78c82c5cf0d1cc32ea6f1c6e93ca026cca`. The
recorded value in `docs/linux-floor-backend.md` is `21ad63fa…`. The QEMU floor
passed with its own prepared backends.

## Decisions (running log)

**2.0 archives.** On 2026-11-03 `task release:archives` ran in
`Ravel.skill-directory-delivery` at output commit
`4f831feb4d0f288f9699b1a4b02fc24cda9843c5`, with an empty working copy. The
build commit is `78332fcc367d`. The macOS archive
`ravel-v1.1.0-aarch64-apple-darwin.tar.xz` has the SHA-256
`0bd671231b4dbfda83cb2b751b810cd54b94059caf336fecd77097bff67b379b`.

Its `bin/ravel` has the SHA-256 `151635d2…`, the same as k90 recorded. The
executable holds the build identity `9feb86b7…` as two ASCII strings. The
archive name keeps `v1.1.0`, because the workspace version is 1.1.0 until the
cut. A copy is in Grove's ignored `target/ravel-archives-2.0/`.

**Linux 2.0 build fails.** Both Linux targets fail to link at glibc 2.17.
`ravel::skills::dir::Dir::rename_noreplace` calls `renameat2`, which glibc
added in 2.28. `ld.lld` reports `undefined symbol: renameat2`. This is a defect
of the Ravel feature line, not of Grove.

No Linux 2.0 archive exists, so the
Linux 2.0 lane cannot run. The lane names both Linux targets as not run with
this reason. A new leaf before k102 owns the Ravel repair and the Linux 2.0
lane of the floors.

**Identity check.** Ravel's `build.rs` embeds the build identity as the
literal `RAVEL_BUILD_ID`. Admission therefore checks the identity in the bytes
of `bin/ravel`, without running the program.

**22.1.0 keg.** No Grove 22.1.0 keg remains on this host. The published
`grove-v22.1.0-aarch64-apple-darwin.tar.xz` has the SHA-256 `b45a030c…`. Its
four executables match `docs/legacy-driver-inventory.json` byte for byte. The
rehearsal uses a scratch keg extracted from that archive.

**Lane record.** `scripts/ravel_target_inputs.py` holds one `Lane` record for
each runtime. The 1.1 lane keeps its pins, manifest and download route. The 2.0
lane records the macOS digest, the source commit, the build identity and the
2.0 manifest. It records each Linux target as not run, with the link failure as
the reason. It has no download route, and its default directory is
`target/ravel-archives-2.0`.

`--lane` selects the record, and the default is
1.1, so every older caller keeps its behaviour.

`--status` prints one line for
each target. A target without a record refuses before admission reads any
archive.

**Cases of the 2.0 lane.** `scripts/ravel-installed-smoke-rhai.sh` ports
Ravel's own installed smoke at `4f831feb`. The runner picks the cases script
by lane, so the call shape of the shared platform runner stays the same. The
target script now takes the version from the admitted archive name.

**Release smoke.** On macOS, `release-smoke.sh` admits the 2.0 lane and passes
its archive as `RAVEL_2_TARGET_ARCHIVE`. `release-smoke-target.sh` extracts it
into its own directory and gives `grove-installed-smoke.sh` its fourth
argument. The Linux routes do not set that variable, so they keep the
three-argument call.

Each runner prints a `not run` line for each target that
the 2.0 lane did not run. Until rollout replaces the record, a Grove release
needs `target/ravel-archives-2.0`. `docs/ravel-target-inputs.md` states the
replacement.

**Rehearsal.** `legacy_driver_rehearsal.py --also-installed PREFIX` puts the
2.0 front first on the rehearsal PATH. After each of six package steps, it runs
the 1.1 front with `--version` and with `end-session` outside a session. It
also compares the inventories of both Ravel installations.

**Linux 2.0 split.** The Linux 2.0 floor work cannot run without a Linux 2.0
archive. This leaf therefore does not wire the Linux mounts, the QEMU
registration or the 2.0 floor cases. grove-linux-two-lane-k103, inserted before
k102, owns the Ravel repair and those cases. `docs/linux-floor-native.md`
names the unmeasured cases, including the confined directory case.

**Results.** Each result below is from 2026-11-03.

- `task ravel:targets:controls ARCHIVES=target/ravel-archives` passed. The 33
  unit cases passed. The real-archive controls of both lanes passed. A 2.0
  archive with one changed byte refused by SHA-256. The 1.1 archive in the 2.0
  directory refused, and the 2.0 archive in the 1.1 directory refused. A
  request for a Linux target refused with the recorded reason. The smoke runner
  refused the changed archive before any execution.
- `task ravel:targets:smoke ARCHIVES=target/ravel-archives` passed. The 1.1
  lane ran on all three targets, Linux natively and under QEMU. The 2.0 lane
  ran on macOS. The run named both Linux targets of the 2.0 lane as not run.
- `task release:smoke TARGETS=aarch64-apple-darwin` passed. Grove 22.8.0 ran
  the 1.1 and 2.0 lanes, direct and through a symlink.
- `task legacy:controls` passed 31 cases. The rehearsal passed with the
  scratch 22.1.0 keg, `RAVEL` set to the 1.1.0 keg front and `ALSO` set to the
  admitted 2.0 prefix. The SHA-256 of the 1.1.0 `bin/ravel` was `c52f990b…`
  before and after.

**Controls seen to fail.** A copy of the inputs script without the identity
check admitted another identity, and the unit case failed. A copy of the 2.0
cases that wrote the 1.1 `sample.ts` as `policy.ts` failed with
`config_format_retired`, and the unchanged cases passed. The continuity check
refused a scratch keg copy whose README changed between two steps. It also
refused a PATH with the 1.1 front first.

**Lint.** Ruff passes at each file's existing line length: 100 for the
rehearsal and 88 for the inputs script and its tests. Strict Pyright reports
two errors, the same two `contextmanager` deprecations that the parent commit
has.

**Gate.** `mise exec bun@1.4.3 -- bash scripts/check.sh`, with
`GROVE_TEST_RAVEL` set to the 1.1 keg front, passed all 20 principal checks.
The workspace suite ran 1,333 tests with no failure. The SHA-256 of the
working-copy diff was `d9348335…` before and after the run.
