# smoke-instrument-k48

## Goal

Deliver the reusable installed-layout smoke test of `installed-smoke-k18`, and
execute it wherever this host has a working environment for the target. Leave
the Linux x64 execution to `x64-floor-k49`.

## Done when

- `task release:smoke [TARGETS=…]` rebuilds the archives and smoke-tests the
  named targets or all of them. `scripts/release-smoke.sh --archives DIR` tests
  already-built ones.
- `crates/harness-dispatch/scripts/installed-smoke.sh PREFIX VERSION` runs the
  static TypeScript case through the front and through a relative symlink to
  it. It checks inspect, `run` against a temporary state directory with a fake
  harness, and `record show`, with no Bun or Node on PATH. Cases are a list, so
  `computed-selection-k21` adds `select` without restructuring anything.
- macOS arm64 passes natively. Linux arm64 passes in the pinned CentOS 7
  container, and its glibc control is seen to refuse a 2.25 binary.
- Every assertion has been seen to fail against a violating subject.
- The runtime evidence, `docs/RELEASING.md`, the spec notice and the changelog
  record what executed, where, with which Bun and image, and what did not.

## Decisions (running log)

**Three scripts, split at the package boundary.** The cases are the
package's: `crates/harness-dispatch/scripts/installed-smoke.sh PREFIX VERSION`
exercises an installed pair and knows nothing of Grove's archives, so an
extraction takes it along. Grove's release owns the rest:
`scripts/release-smoke.sh [--archives DIR] [TARGET…]` finds each target's
archive and chooses where it runs, and `scripts/release-smoke-target.sh` runs
inside that environment, asserting it (system, machine, and on Linux glibc
2.17 plus the control), extracting the archive with the target's own `tar`
into a fresh prefix, checking the three executables' versions, then handing
the prefix to the cases. The two in-environment scripts need only bash 3.2,
coreutils, grep and cmp: macOS's `/bin/bash` runs them under `env -i
PATH=/usr/bin:/bin`, and so does CentOS 7's bash 4.2.
A case is a `case_<name> FRONT` function listed in `CASES`, and each runs
through the prefix's front and through a relative symlink to it from another
directory, as Homebrew links a keg, on every target rather than only macOS.
`--archives` exists so `cpu-floor-k19` can smoke `target/dist` in the
release; `task release:smoke` rebuilds `target/archives` first, so the
regression instrument never reads a stale archive.

**The glibc-2.17 userland is CentOS 7.9.2009, pinned by manifest-list digest.**
`centos:7@sha256:be65f488…18ea4` serves linux/amd64 and linux/arm64/v8; both
report `glibc 2.17` from `getconf GNU_LIBC_VERSION`, and both carry GNU tar
1.26, xz and cmp, so extraction uses the target's own `tar`. Containers run
with `--network none`, as an unprivileged uid.

**The control is a zig-built probe pair per architecture.** One source, built
with `zig cc -target <arch>-linux-gnu.2.17` (prints and exits 0) and with
`-target <arch>-linux-gnu.2.25 -DABOVE_FLOOR`, which calls `getrandom`
(`GLIBC_2.25`). Observed on 2026-09-30 in both containers: the floor probe ran;
the newer one exited 1 with ``/lib64/libc.so.6: version `GLIBC_2.25' not found``.
The floor build is what makes the failure discriminating: a container that
could not run that architecture's binaries at all would fail both. Zig is
already a release prerequisite (`cargo zigbuild`).

**The x64 container is emulated.** Docker Desktop 28.1.1 on this arm64 host has
Rosetta off; its VM's binfmt handler for x86_64 is `/usr/bin/qemu-x86_64`
(flags `POCF`). The CPU model under that emulation is not the floor's;
`cpu-floor-k19` owns that dimension.

**No heredoc inside a container: emulated bash 4.2 cannot serve one.** The first
x64 run failed before any shipped code ran: CentOS 7's `cat >file <<'EOF'`
segfaulted (`qemu: uncaught target signal 11`, exit 139), three runs of three.
Bisected in the amd64 container (as root and as uid 1000, with and without
`env -i`): `cat` of a heredoc segfaults and bash's own `read -d ''` of one
hangs, while `cat FILE`, `cat <FILE`, a pipe into `cat`, `printf` into a file
and command substitution all work. The emulator is Docker Desktop's VM
`qemu-x86_64` 8.1.5. So the case script writes its fixtures with `printf`, and
neither in-environment script uses a heredoc; host-side scripts keep theirs.
`cpu-floor-k19` should expect the same class of emulator defect.

**macOS arm64 and Linux arm64 pass; the Linux arm64 archive has now executed at
the floor.** Against the k17 archives (whose shipped source this leaf does not
change): macOS 26.6.2 natively, and CentOS 7.9 aarch64 natively on Docker's
linux/arm64, each through both fronts, each with inspect, a run that exits 42
and a `record show` of it. The arm64 control refused the 2.25 probe with
``version `GLIBC_2.25' not found``.

**Docker Desktop cannot execute the x64 case: the leaf decomposes.** With the
heredocs gone, the x64 front ran, but its worker panicked at
`0xFFFFFFFFFF601000` under Docker's QEMU 8.1.5, the same address CentOS 7's
bash crashed at. The same worker under the same QEMU in ubuntu:24.04 (glibc
2.39) selected correctly, so the defect is this emulator with a glibc-2.17
x86-64 userland, a known class (docker/for-mac#5883, #6261); Docker calls x86
emulation best effort. That QEMU gives x86-64 guests no vDSO (no
`AT_SYSINFO_EHDR`); plain `time()` and `gettimeofday()`, and `time()` in a
forked child, still worked, so the exact trigger is not established. A pinned
QEMU 10.2.3 (from `tonistiigi/binfmt`), registered in a private binfmt_misc
instance inside `unshare --user --map-root-user --mount --pid` in a privileged
arm64 container and run against a CentOS 7 amd64 rootfs chroot, does supply a
vDSO and runs the userland (bash heredoc, `getconf` 2.17), but there the worker
aborted with JSC `MemoryExhaustion` at 33 MB RSS, every limit unlimited. The
investigation is open-ended, so it becomes the node's next leaf rather than
growing this session. x64's glibc control still ran and refused the 2.25 probe
under Docker's emulation.

**Controls seen to fail.** Case script, macOS: bun on PATH, a wrong VERSION, a
front copied (not linked) outside the prefix (exit 5), the worker moved away
(exit 5), the fake harness exiting 0, a wrong argv expectation, and a wrong
record expectation each failed; unmutated and restored runs passed. Floor
instrument, arm64 containers: the target script in ubuntu:24.04 refused glibc
2.39; with that check removed it refused because the 2.25 probe ran (exit 0);
an x86_64 target in the arm64 container refused the machine. Unmutated in CentOS
7 it passed.

**The measured run, on the finished source.** `task release:smoke` rebuilt all
three archives (SHA-256 `38cb7299…` macOS arm64, `0886d23e…` Linux arm64,
`12fadd92…` Linux x64; worker build `bdd58ee3…`, Bun 1.4.2), then smoke-tested
them. macOS arm64 and Linux arm64 (native container) passed through both
fronts, and the arm64 control refused the 2.25 probe. Linux x64 was reported as
emulated by Docker. Its control refused the 2.25 probe, its worker panicked at
`0xFFFFFFFFFF601000`, and the failure named the runtime evidence. The task
exited nonzero. The six instrument files had identical digests before and
after the run.
