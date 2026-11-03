# The QEMU floor route of the Linux release smoke

`task release:smoke` runs each Linux archive in the CentOS 7 userland, the
glibc 2.17 floor. A pinned user-mode QEMU emulates the CPU floor of the target. `scripts/release-smoke.sh` stages the inputs, and
`scripts/release-smoke-qemu.sh` runs inside a helper container. The emulator,
its interpreters and the probes are test-only resources. They stay out of the
production archives.

```sh
# Build the archives of the working copy, then run every target:
task release:smoke
# One target only:
task release:smoke TARGETS=x86_64-unknown-linux-gnu
```

On arm64 Docker, Linux arm64 first runs natively in a container of the floor
image. Then it runs again under QEMU. Linux x86-64 runs under QEMU only, and
that one run covers both floors. Archive admission precedes every target
invocation. It refuses a missing, changed, mismatched or incomplete Ravel
archive.

## Namespaces and emulated execution

The helper runs as the root of the container. It creates a private user and
mount namespace with identity uid and gid maps, and mounts a private
binfmt_misc instance there. So no registration of the Docker host changes.

The entry launcher `scripts/floor-qemu-enter.c` switches to the userland through
[`pivot_root`](https://man7.org/linux/man-pages/man2/pivot_root.2.html). It
keeps the submounts and detaches the old root. The binfmt_misc mount stays
inside the new root. If the mount went, x86-64 executables would fail to start
and arm64 executables would run natively, outside the CPU model. The
[`F` flag](https://docs.kernel.org/admin-guide/binfmt-misc.html) of the
registration opens the interpreter when the helper registers it.

Each ELF file of each archive stays unchanged and must match the registration
of the target. Only named test infrastructure, such as QEMU, the interpreters
and the entry launcher, uses the ELF padding exemption. These checks are
mandatory:

- The CPU model is the Cortex-A53 for arm64 and Nehalem for x86-64.
- A probe built against glibc 2.17 runs, and the userland refuses its glibc
  2.25 twin.
- The guest has a vDSO, and no mapping above its address limit.
- A probe that executes one instruction beyond the floor dies of SIGILL. The
  same probe runs under the same QEMU with `-cpu max`.

The emulator is at `/usr/libexec/floor-qemu` in the userland. The helper
removes the `-cpu max` interpreter after its control. These CPU and address
claims cover the automatic exec paths of the kernel. An agent that invokes the
emulator directly can choose another CPU or guest base. The route is a test
instrument. It does not prevent a direct invocation of the emulator.

## Security of the helper

The helper keeps the default Docker capabilities for its private set-up. It
adds no outer SYS_ADMIN. It runs with no network, and its inputs are bound
read-only. Its seccomp profile is the pinned default-deny Moby allowlist of
`scripts/floor_seccomp.py`. The profile adds only `mount`, `pivot_root`,
`umount2` and `unshare(NEWUSER|NEWNS)`.

The helper keeps the masked system paths of Docker, because it binds `/proc`
and mounts no procfs of its own. No daemon, sysctl or LSM setting changes. `task release:floor:seccomp:controls`
checks the derivation and its refusal of a changed or missing source.

After the pivot, the entry launcher sets no-new-privileges. It clears the
supplementary groups and the whole bounding set. Then it drops gid and uid to
1000 and runs the smoke test of the target. On the 2.0 lane that smoke test
includes the directory case, `scripts/grove-floor-directory-case.sh`.

## Verified handoff — 2026-10-14

This section records a run of the earlier confinement route. Bubblewrap and
its route left Grove after the withdrawal of confinement, which
[the withdrawal record](confinement-withdrawal-evidence.md#linux-floor-evidence)
states. No build after the withdrawal can repeat these results.

<!-- tc: pending because this dated record predates the conversion -->

`all-target-complete.log` passes the normal release runner for all three freshly
built Grove 22.1.0 archives against admitted public Ravel 0.1.0. It includes
native arm64's three refusals and real positive case, both pinned QEMU floors'
missing/emulated-unusable/real-backend cases, direct/symlink caller imports,
actual standalone publication and all filesystem/security/CPU controls.
`outcomes.json` checks every named target/route/control outcome individually.
`qemu-task-complete.log` separately passes the new Taskfile entry point for both
floors. `independent-ravel-complete.log` passes independent Ravel-only smoke on
all three targets, including exact/empty/multiline/flag and stale results.
Every QEMU input is hashed before/after its container; each native mode likewise
compares its privately staged inputs. The exclusive backend preparation repeats
byte for byte, including its input manifest, in `inputs/native` and
`inputs/native-repeat`. The real native bwrap SHA-256 before the one-byte ELF
registration exemption is
`e78c51fa0c634bf2377cf810a84425df2ca31e7cf3d6b6fd4a5604e720f33e44`.

`check-complete.log` passes all 18 principal checks, 1,378 workspace runtime
cases (31 default ignored cases remain), nine doctests and all six books, plus
explicit installed-Ravel caller assets/provenance. The initial gate correctly
refused this shell's Bun 1.4.3; the successful gate uses existing pinned 1.4.2
from Ravel's local tooling on a temporary verification PATH. No installation or
production dependency changed. `profile-red.log`/`profile-green.log` show the
helper-profile extension test fail before implementation and pass after it.
`bind-mutation-red.log` moves the private registration mount outside the new
root: ordinary arm64 cases then run natively, but the actual sandbox CPU
control fails at exit 94 and Grove publishes nothing. This discharges the lost
registration risk with an observed counterexample. Ruff and shellcheck pass.

`before-complete.json`/`after-complete.json` and `digest-comparison.json` cover
35,356 individually named subjects: project/adjacent tracked files, source and
archive inputs, complete original Grove 22.1.0 and installed Ravel prefixes,
Grove/Ravel/legacy configuration, tools and Zig installation, resolved Cargo
sources and caller compiler dependencies, source caches, prepared backends and
QEMU package. All 35,355 stable subjects match; the sole difference is shared
legacy `records.sqlite3`. The read-only immutable/query audit in
`shared-store-observations.json` records another BookScanBuilder driver ending
and relaunching at 14:01:36 UTC during measurement, with this run still live.
The database retained its post-measurement digest during that audit and was not
restored or mutated here. Adjacent checkout states match individually. The
first narrower freeze's 35,346 subjects all matched; the final pass adds the
previously omitted Grove configuration and temporary compiler binding.

Graph evidence is Tier 2, best effort. Operated-on paths have no recorded gaps
apart from the QEMU script's partial arithmetic line, read directly; Ravel's
excluded documentation and target evidence were read directly too. The
host's registration directory was not visible in the unprivileged inventory;
no global-registration byte-preservation claim is inferred from that empty
read. Private namespace setup, kernel ownership and both independent smoke
routes provide the applicable boundary evidence. CPU emulation remains a
trusted automatic-exec measurement and does not observe a kernel floor.

k52's backend, native and QEMU criteria are met. k51 retains the final freshly
rebuilt all-target/formula/ignored-test delivery acceptance and k6 close. The
original 22.1.0 driver/keg/configuration/ending channel remain preserved; no
installation, release, owner-policy change, production backend/runtime addition
or teardown occurred. This session still ends with `harness-dispatch exit`.
