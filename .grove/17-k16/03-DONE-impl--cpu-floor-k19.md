# cpu-floor-k19

## Goal

Prove the CPU floor for both Linux targets by running the installed smoke under
user-mode emulation with the Nehalem x64 and Cortex-A53 arm64 CPU models. Wire
every delivery check into the release pipeline, and state the documented floors.

## Context

Bun's own baseline verification emulates `-cpu Nehalem` and `-cpu cortex-a53`,
as `runtime-evidence.md` records. User-mode emulation runs on the host kernel,
so the kernel stays a documented claim.

## Done when

- The smoke task's Linux cases also run under user-mode emulation with the
  respective CPU model, inside the glibc-2.17 userland.
- Positive control: a probe using an instruction beyond each model is seen to
  fail under that model. Use AVX2 for x64, and an Armv8.1-or-later instruction
  such as the LSE atomics for arm64. A CPU model that silently accepts newer
  instructions would otherwise pass everything.
- `task release` runs the archive-content assertions and the per-target
  installed smoke before anything is published. Failure stops the release with
  a recovery note in `docs/RELEASING.md`.
- The usage documentation and release notes state the kernel floor as Bun
  1.4.2's documented range, labelled documented rather than executed: 5.1 in
  its README, 3.10 on its installation page. They state the macOS 13.0 minimum
  too.
- The runtime evidence records the emulated runs and both controls.
- As this node's last act: check the node brief's `Done when`. If the compiled
  worker cannot meet a floor, stop and escalate under the worker ADR's reopen
  condition instead of retiring.

## Decisions (running log)

**Bun 1.4.2's floors, read from its own tag.** `README.md` at `bun-v1.4.2`
says kernel 5.6 is recommended and "the minimum is 5.1";
`docs/installation.mdx` says Bun "runs on kernels as old as 3.10 (RHEL 7) with
graceful degradation of newer syscalls", that Bun "requires macOS 13.0 or
later", and that its single x64 build targets Nehalem and "selects AVX2/AVX-512
code paths at runtime". That last sentence is why the x64 CPU model matters:
the worker takes different code paths on a newer CPU.

**The arm64 emulator is Debian's, because the pinned image has none.**
`tonistiigi/binfmt@sha256:400a4873…`'s linux/arm64 image carries
`qemu-x86_64` and other foreign emulators but no `qemu-aarch64`: it leaves
out its own architecture. Debian's `qemu-user` `1:10.2.2+ds-1` for arm64 has
a static-pie `qemu-aarch64`, served permanently by snapshot.debian.org at
`/file/4b7f4762…` (its SHA-1), SHA-256 `f8bf89da…`. The x64 route keeps
tonistiigi's 10.2.3, which `x64-floor-k49` validated; re-validating it on
Debian's build would churn a working instrument for no new claim.

**An arm64 registration on an arm64 host must not match its own tools.**
QEMU's aarch64 mask requires ELF ident bytes 8–15 (`EI_ABIVERSION` and the
padding) to be zero, and Linux's ELF loader ignores them. So the emulator and
interpreter copies in the userland get `EI_ABIVERSION` 1: without it the
interpreter would be its own interpreter until `exec` fails with `ELOOP`. The
helper's own arm64 tools still match, so after registration its `chroot` runs
emulated too; only guest execs follow it.

**Debian's QEMU takes argv0 through `-0`.** Upstream `linux-user/main.c`
(v10.2.2) skips the P flag's argv0 only when its own auxv carries
`AT_FLAGS_PRESERVE_ARGV0`, which the kernel sets for the interpreter it
starts, not for a QEMU that interpreter execs. So the aarch64 interpreter
rewrites binfmt_misc's `path argv0 args…` as `-0 argv0 path args…`, while the
x64 interpreter passes it through to tonistiigi's build as before.

**A helper executable that matches runs the interpreter from the helper's
root.** The first Cortex-A53 run failed with `release-smoke interpreter:
/qemu/qemu: No such file or directory`: the helper's `chroot` matched the
aarch64 registration before it had changed root. A `/qemu` link to the
userland's `qemu/` in the helper's root serves both roots. Registration now
comes after the binds, so `chroot` is the only helper executable that runs
emulated.

**Every ELF file in the archive must match the registration.** The CPU control
shows that a matching executable runs under the model; an archive executable
whose ident bytes 8–15 were not zero would instead run natively on this
LSE-capable host and pass untested. The helper has no `xz`, so the host lists
each archive ELF's first 20 bytes and the helper classifies every one against
the magic and mask, requiring the front and the worker among them. All four in
each Linux archive match.

**Both floors are met; no escalation.** Against the k49 archives (21.12.0,
worker build `bdd58ee3…`), Linux arm64 passed natively in the floor container
and again under Debian's QEMU 10.2.2 at `-cpu cortex-a53`, and Linux x64 under
tonistiigi's QEMU 10.2.3 at `-cpu Nehalem` with the 2^47 guest base. Under each
model the CPU probe printed its first line and died of SIGILL (exit 132, `qemu:
uncaught target signal 4 (Illegal instruction)`), and under the same QEMU with
`-cpu max` it ran to the end.

**The release gate.** `task release` runs `scripts/release-smoke.sh --archives
target/dist` between `scripts/release-build.sh` and `jj git push -b main`, so
the pushes and `release-publish.sh` follow it. The smoke script now also runs
`assert_archive` on each archive first, so `--archives DIR` applies both
release instruments to any directory; the build's own check is unchanged. The
doctor checks an Apple silicon host and a running arm64 Docker with kernel 6.7
or later, so a missing Docker stops the release before the cut. A failure after
the cut leaves everything local: `docs/RELEASING.md` resumes after an
environment fault, and for a defective archive undoes the cut (`git tag -d`,
`jj abandon --retain-bookmarks main`, which jj 0.45.1 documents as moving the
bookmark to the parent) so the rerun cuts the same version. `release.test.sh`
asserts, from Task's dry run, that the smoke follows the build and precedes
each of the three publishing steps.

**Controls seen to fail, each on a scratch copy, the repo's instrument files,
archives and cached package digest-identical afterwards.** CPU model: with
`-cpu max` as the model, x64 and arm64 each refused "not killed by SIGILL".
Twin: with the probe's instruction replaced by one no CPU executes (`ud2`,
`udf #0`), each refused "did not run under the same QEMU with -cpu max". Escape:
an arm64 archive whose worker carries `EI_ABIVERSION` 1 was refused by name;
with the match check removed, that archive passed every other check, so the
hazard is real. An enumeration matching no ELF refused ("lists no
harness-dispatch front or worker"). Without the exemption, the helper's
`chroot` failed with "Too many levels of symbolic links" (`ELOOP`). A mutated
package digest was refused with the SHA-256 found. An aarch64 limit of 2^40
refused a guest mapping at `0xffffac600000`. A mutated platform expectation
was refused by name. A darwin archive with a stray file failed the manifest
before extraction. Unmutated, the scratch copy passed both Linux targets.
Doctor: an unreachable daemon (`DOCKER_HOST` at a missing socket), a mutated
platform and a kernel floor of 6.11 each failed. Release test: removing the
smoke line, moving it after the push and moving it before the build each
failed by name. The first removal failed silently, because `grep` under
`pipefail` ended the script before its message; `dry_line` now tolerates an
absent line. An unmutated scratch run of the test passes, and it first failed
for want of the workspace root, which is why each control set includes one.

**The measured run, on the finished source.** `task release:smoke` rebuilt all
three archives (SHA-256 `b451bf73…` macOS arm64, `381bde71…` Linux arm64,
`5ca21891…` Linux x64; worker build `bdd58ee3…`, Bun 1.4.2) and passed macOS
arm64 natively, Linux arm64 in its native container and under QEMU at
`cortex-a53`, and Linux x64 under QEMU at `Nehalem`, each through both fronts,
with every glibc control, CPU control and archive ELF match passing. The task
exited 0, and its nine subjects had identical digests before and after. A
comment in `release-smoke-qemu.sh` was then corrected, so `release-smoke.sh`
passed the same archives again with the committed scripts, its subjects also
unchanged.

**The node's `Done when` holds; it closes without escalation.** Build and
layout, the formula and version checks, the cargo-release opt-out and the
archive assertions are `release-layout-k17`'s. The installed smoke, macOS
natively and the C library floor with its control, is `installed-smoke-k18`'s.
Both CPU models with their controls, the release gate, the documented kernel
range and macOS minimum, and the recovery path are this leaf's. One wording
differs: x64 cannot run as a container on this arm64 host (the node's notes),
so its single run in the floor userland under QEMU at Nehalem covers both
floors, and arm64 runs as a container and again emulated. A second x64 run at
`-cpu max` would test nothing the Nehalem run does not.
