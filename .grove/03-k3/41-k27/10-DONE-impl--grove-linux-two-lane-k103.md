# grove-linux-two-lane-k103

## Goal

Make Ravel's directory build link at the glibc 2.17 floor. Record its Linux
archives in the 2.0 lane, and run that lane on the Linux floors. Without this
repair, no Linux 2.0 archive exists, and Ravel 2.0.0 cannot be cut, because
Ravel's own release builds all three targets.

## Context

- grove-dual-target-inputs-k101 found the defect on 2026-11-03. `task
  release:archives` in `Ravel.skill-directory-delivery` at output commit
  `4f831feb4d0f` built macOS. Both Linux targets failed to link:
  `ld.lld: error: undefined symbol: renameat2`, referenced by
  `ravel::skills::dir::Dir::rename_noreplace`. glibc added `renameat2` in 2.28,
  and the release links Linux against glibc 2.17 through `cargo zigbuild`.
- The 2.0 lane record is in `scripts/ravel_target_inputs.py`. It records the
  macOS archive and states this reason for both Linux targets.
  `docs/ravel-target-inputs.md` holds the lane, its tables and the rollout
  handoff.
- k101 gave the 2.0 lane to the target smoke and to the macOS release smoke.
  The Linux routes still run the 1.1 lane only. These are the routes:
  `release-smoke.sh` (`stage_linux_inputs`, `smoke_linux`),
  `release-smoke-target.sh` (it reads `RAVEL_2_TARGET_ARCHIVE` only on macOS),
  `release-floor-native.sh`, `release-floor-qemu.sh`,
  `release-smoke-qemu.sh` (`assert_archive_matches` requires the 1.x worker
  `libexec/ravel/ravel-policy`, and a 2.0 prefix has none) and
  `grove-floor-native-cases.sh` (three arguments, `policy.ts` only).
- `docs/linux-floor-native.md`, section "Runtime lanes", names the floor cases
  that no run measures on the 2.0 lane.

## Done when

- Ravel's directory build links at glibc 2.17 on both Linux targets. It keeps
  the behaviour of `renameat2` with `RENAME_NOREPLACE`. Settle the behaviour on
  a kernel or file system without that flag against the Ravel contract.
  Ravel's applicable checks pass, including
  `task test:native-linux`. The Ravel design evidence records the new build
  commit, output commit and build identity.
- The 2.0 lane records all three archives of that build, with the new source
  commit and identity, and `unrecorded` is empty. The Ravel builds section of
  the node brief names the new build.
- The target smoke runs the 2.0 lane natively in the floor container and under
  QEMU for each Linux target. The QEMU registration check accepts a 2.0 prefix
  with no worker and still refuses an incomplete 1.1 pair.
- The native and QEMU floor cases run the 2.0 lane with `config.rhai`. The
  cases are the caller lifecycle, standalone and imports, the sandbox denial
  task and the unreadable native guidance case. Port the confined directory case of
  `crates/grove/tests/directory_route.rs` to the Linux floor, or name it as
  unmeasured with the reason.
- Controls, each seen to fail once: a 2.0 floor lane that writes `policy.ts`
  fails. A Linux 2.0 archive with one changed byte refuses.

## Notes

The node brief says "Do not move the bookmark here" for the k27 stages. This
leaf is the exception: the repair is a Ravel stage like
ravel-local-sources-k90. Add the repair as a new commit above `4f831feb` on the
local bookmark `skill-directory-delivery`, and do not push it. Rebase only if
Ravel `main` moved, and record the bases as k90 did.

The release smoke keeps 1.1 and 2.0 prefixes apart by directory. Before the
cut, the archives of both lanes have the name `ravel-v1.1.0-TARGET.tar.xz`.
Keep that separation in the Linux mounts.

From k101: `release:floor:native:run` refused a freshly prepared aarch64
bubblewrap on 2026-11-03. Its SHA-256 was `d35694e8…`, and
`docs/linux-floor-backend.md` records `21ad63fa…`. Settle that before the
native floor can run.

## Decisions (running log)

**Ravel repair.** glibc 2.17 has no `renameat2` wrapper, but Linux has had
the system call since 3.15. On Linux, `rename_noreplace` now enters
`SYS_renameat2` through `libc::syscall`, with `RENAME_NOREPLACE`. The
behaviour is the same as the glibc 2.28 wrapper, which only enters the call.

**Platform without the flag.** The Ravel contract requires an atomic
publication that does not replace an existing object. A plain rename replaces
an empty directory, so no fallback is admissible. `ENOSYS` (a kernel before
3.15) and `EINVAL` (a file system without the flag) refuse as
`skill_cache_unavailable`, with a remedy that names the requirement. macOS
`ENOTSUP` takes the same refusal. Another rename error keeps the general
remedy. `docs/specs/skill-discovery.md` owns the rule, and the delivery
contract links to it.

**Ravel tests and controls.** A `dir.rs` unit test renames onto an empty and
a full directory and requires refusal both times. With the flag set to 0 the
test failed, because the plain rename replaced the empty directory. A new
`renamed` fault hook stands for a failed rename. The publication test
requires the precise refusal, no object and no staging for each unsupported
errno, and the general remedy for `EIO`. With the new branch disabled, the
test failed. `task check` passed 730 tests on macOS.

**Ravel bases.** Ravel `main` is still `24a545236185`, so no rebase. The
build commit is `23e50ce90037`, above `4f831feb4d0f`.

**Build identity.** The identity is `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb`. A script that applies the rule of
`crates/ravel/build.rs` computed it. The same script reproduced k90's
`9feb86b7…` from the tree of `4f831feb`. Each of the three executables
contains the new identity.

**Floor bubblewrap.** Two builds into different paths with mise's Zig 0.17.0
and one with Homebrew's Zig 0.17.0 agree byte for byte: aarch64
`d35694e8915a70cf6ef403639c4d6c78c82c5cf0d1cc32ea6f1c6e93ca026cca`, x86_64
`64bdc5fcfc4014e7d4dcb6bd8eaaef9297663bb25e62553c409ff355e270828f`. Every recorded input matches. The cause of the drift
since 2026-10-18 is not known, as for the earlier drift. The executable is a
test-only helper, so this leaf re-pins it, as the 2026-10-18 re-measure did.

**Linux archives.** `task release:archives` at the build commit, with an
empty working copy, linked all three targets. The archive SHA-256 values are
macOS `6fd7e44a…`, aarch64 Linux `331008d4…` and x86_64 Linux `2a7e62f7…`.
The installed macOS `bin/ravel` is `8e10e311…`, the same bytes as the archive.

**Lane record.** The 2.0 lane records all three archives, and `unrecorded` is
empty. Its source commit is the build commit `23e50ce9…`, because the archives
were built there. Two unit cases now use a synthetic lane for the
unrecorded-target refusal and the status line, and a new case requires the
real record to cover every target.

**Lane label.** Before the cut both lanes' archives are named
`ravel-v1.1.0-TARGET.tar.xz`, so a path cannot tell them apart. Each Linux
mount stages the 2.0 archive in `ravel-2.0/`. `archive_elf_headers` puts
`ravel-2.0/` before each path of a 2.0 archive, and the target smoke sets
`SUBJECT_LANE` when the subject archive is itself the 2.0 archive. The QEMU
check needs a complete 1.1 pair, a 2.0 front with no worker, at least one
Ravel prefix and a complete or absent Grove pair.

**Floor cases.** `grove-floor-native-cases.sh` takes an optional fourth
prefix and runs the denial task and the native guidance case for each lane in
its own HOME. A Rhai configuration cannot write a file. The 2.0 positive
selection control is therefore a `print` from `select()`, which Ravel sends to
stderr.

**Directory case.** `grove-floor-directory-case.sh` ports the confined
standalone case of `directory_route.rs`. The floor has no Git, so its sources
are two plain local directories. On macOS with the 2.0 prefix it passed, and
its unconfined control showed every protected read and outside write
succeeding. With the source file granted, it failed on `source-read=ok`.

**Ravel Linux checks.** `task test:native-linux PLATFORM=arm64` passed all 12
steps and 723 tests. With `RENAME_NOREPLACE` replaced by 0 in the Linux
container, the `dir.rs` test failed.

**Emulated amd64.** `task test:native-linux PLATFORM=amd64` failed in four
steps. nextest stopped at the first failures, so the full suite then ran with
`--no-fail-fast` in the same emulated environment, once on the build commit
and once on a temporary workspace at `4f831feb`. Both runs failed the same 94
tests, item by item: confinement, namespace and signal tests that QEMU
user-mode does not reproduce. Ravel's `docs/native-entry-campaign-plan.json`
already lists E04 and E07 as such rows. The build commit ran 723 tests and the
baseline 721. The two new tests passed on x86_64, through system call 316.
k90 recorded no amd64 run.

**Ravel release smoke.** `task release:smoke` in Ravel passed all three
targets. Each Linux target ran natively or under QEMU at glibc 2.17, with the
GLIBC_2.25 refusal and the SIGILL CPU control. The rebuilt archives have other
SHA-256 values than the recorded ones, but every file in them is
byte-identical. Only the tar member times differ. The lane keeps the pins of
the first build, whose copies are in `target/ravel-archives-2.0`.

**Ravel output.** The output commit `1ca4aecbeae14df87b24cfa712e7b7cbf42b8630` adds the "Linux floor repair
record" to `docs/skill-discovery-design-evidence.md` and changes nothing else.
The local bookmark `skill-directory-delivery` now points at it. Nobody pushed
it. The temporary baseline workspace `k103-baseline` is forgotten and removed.

**Grove Linux runs.** Each result below is from 2026-11-03.

- `task ravel:targets:smoke ARCHIVES=target/ravel-archives` on both Linux
  targets passed both lanes, natively in the floor container and under QEMU.
  It named no target as not run. The QEMU check admitted the 2.0 prefix with
  its one executable.
- `task release:registration:test` with both lanes passed on both Linux
  targets. It refused a missing `grove-llm`, a missing 1.1 worker, a 2.0
  prefix with a worker and a 2.0 front without its label.
- `task release:floor:native:run` with both lanes passed all four
  environments on each lane. The supported environment passed the 2.0
  lifecycle, standalone and imports, the denial task, the native guidance case
  and the confined directory case with real bubblewrap.
- `task release:floor:qemu` with both lanes passed the three environments on
  each target and each lane. The confined directory case passed on aarch64
  under Cortex-A53 and on x86_64 under Nehalem.
- `task ravel:targets:controls` passed. Each Linux 2.0 archive with one
  changed byte refused by SHA-256, and the smoke runner refused each one before
  any execution.

**Controls seen to fail.** A copy of the floor cases whose 2.0 lane wrote the
1.1 `policy.ts` failed the native floor: Ravel refused with
`config_format_retired`, and the case reported `2.0 wrong refusal reason`. The
restored script matched its earlier SHA-256.

**Defects found by the runs.** The first native floor run failed because an
existing loop variable `runtime` overwrote the lane variable, now
`lane_runtime`. The first gate failed because the usage comments named the
placeholder `GROVE_PREFIX`, which the surface classifier reads as a variable.
The comments now use the lowercase form. The floor runs used the scripts
before that comment change.

**Gate.** `mise exec bun@1.4.3 -- bash scripts/check.sh`, with
`GROVE_TEST_RAVEL` set to the 1.1 keg front, passed all 20 principal checks.
The workspace suite ran 1,333 tests with no failure. The SHA-256 of the
working-copy diff was `47bdabf7…` before and after the run. After the gate,
`ruff format` wrapped one line of `test_ravel_target_inputs.py`. Ruff then
passed, Pyright reported no error and the 34 unit cases passed. The SHA-256
of the 1.1.0 `bin/ravel` stayed `c52f990b…`.
