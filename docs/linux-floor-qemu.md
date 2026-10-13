# Actual Linux confinement under the pinned QEMU CPU floors

Grove's ordinary `release:smoke` now uses actual namespace environments for its
Linux standalone cases. The test-only backend and emulator resources stay out
of production archives. The same independently admitted public Ravel 0.1.0
runs alongside Grove's complete, differently versioned archive in separate
prefixes, without host Bun/Node.

```sh
# Complete working-copy build followed by native/macOS and both Linux CPU floors:
task release:smoke
# Explicit complete archive inputs, without replacing their directory:
task release:floor:qemu ARCHIVES=target/archives RAVEL_ARCHIVES=target/ravel-archives
# Inspect the real native backend and isolated dependency closure independently:
task release:floor:qemu:prepare OUT=target/my-qemu-backend
```

`TARGETS=aarch64-unknown-linux-gnu` or `TARGETS=x86_64-unknown-linux-gnu`
restricts the QEMU task; its default is both. The general release runner also
repeats the [native environment](linux-floor-native.md) on arm64 Docker, and
macOS installed caller cases. Admission still precedes target invocation and
refuses missing, changed, mismatched or incomplete independent archives.

## Kernel namespaces and emulated execution

[QEMU 10.2.3's user-mode clone implementation](https://raw.githubusercontent.com/qemu/qemu/v10.2.3/linux-user/syscall.c)
rejects the namespace flags required by real bubblewrap. The supported boundary
runs upstream bubblewrap 0.11.0 as a host-native arm64 executable, with its own
relocated arm64 glibc 2.17, libcap 2.22 and libattr closure extracted from the
same digest-pinned CentOS image. Sources, compiler and source digests match the
[backend preparation](linux-floor-backend.md). The real unpatched backend gets
every Ravel sandbox argument and performs actual kernel namespace/mount work.
Its relocated loader and library search path prevent x86 target libraries from
being used for the native backend. The actual loader lists the resolved closure
in each supported environment.

The setup helper uses a private user/mount namespace with identity uid/gid maps
and a private binfmt_misc instance. It switches to the target mount root through
[`pivot_root`](https://man7.org/linux/man-pages/man2/pivot_root.2.html), preserving
submounts and detaching the old root. A chroot cannot create unprivileged user
namespaces. The binfmt mount lives inside the new root: losing it would cause
x86 exec-format refusal and silent native arm64 execution. No host registration
is changed. The interpreter remains pinned through binfmt's
[`F` flag](https://docs.kernel.org/admin-guide/binfmt-misc.html), including when
bubblewrap enters its own empty filesystem and descendant user namespace.

Every Grove/Ravel archive ELF remains unchanged and must match the target
registration. Only named native test infrastructure (QEMU, interpreter, real
bubblewrap, entry launcher and kernel security probe) uses the existing ELF
ABI-padding exemption. Cortex-A53 and Nehalem, the exact glibc 2.17 success /
2.25 refusal pair, guest vDSO/address ceiling and beyond-floor SIGILL / same-QEMU
`-cpu max` controls remain mandatory. Emulator resources reside under
`/usr/libexec/floor-qemu`, within Ravel's existing read-only system grants. The
max interpreter is removed after its positive control. An actual sandbox
shell descendant also must die with 132 at the beyond-floor instruction.

These CPU/address claims cover the deterministic tests' automatic kernel exec
paths. Raw QEMU is necessarily available read-only to the interpreter, and an
agent explicitly invoking it could choose another CPU or guest-base option.
This is a test instrument, not adversarial prevention of emulator invocation;
no malicious CPU-enforcement claim follows from the archive header assertion.
The real filesystem denials are tested independently of that limit.

## Security and refusal contract

The helper keeps the existing Docker default capabilities for private setup,
never adds outer SYS_ADMIN, and runs with no network and read-only input binds.
It uses the pinned default-deny Moby seccomp derivation from the native
instrument, adding only `unshare(NEWUSER|NEWNS)` for private binfmt setup.
`systempaths=unconfined` admits the private procfs mount, as in the native task.
No daemon/sysctl/LSM setting changes. The root setup re-exec must regain its
namespace capabilities after mapping, so the native entry launcher sets
no-new-privileges after pivot and before target exec. It clears supplementary
groups and the entire bounding set, then drops gid/uid to 1000. The native
kernel probe checks those actual properties, seccomp filtering, and outer
mount/network-namespace/keyring refusal.

Each CPU floor runs three real cases: missing backend, the real target backend
under QEMU (namespace-unusable), and the real native backend. Both negative
cases must refuse `confinement_unusable` before policy selection, publish no
output and preserve the existing sentinel. The positive case runs direct and
symlink lifecycle/result/standalone/import cases and actual read/write,
descendant/symlink, root/system/runtime/proc/dev denials. The only writable
task output is published after successful cleanup. No fake backend, stripped
sandbox options or unconfined fallback is used. Independent Ravel-only smoke
continues through the same pinned CPU/glibc machinery.

## Acceptance evidence

The complete measured logs and per-file comparison are retained under
`target/k55/`. Final target acceptance and input preservation are recorded
below after measurement. k51 must freshly rebuild every archive before its
final complete delivery acceptance; this instrument neither installs nor
publishes Grove/Ravel and preserves the original Grove 22.1.0 driver contract.

## Verified handoff — 2026-10-14

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
