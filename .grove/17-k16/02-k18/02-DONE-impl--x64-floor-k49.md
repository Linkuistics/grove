# x64-floor-k49

## Goal

Execute the installed smoke test's x64 case in a glibc-2.17 x86-64 userland,
with its glibc control, so that the Linux x64 archive has run at the floor. This
host is arm64, so every x86-64 environment here is emulated or translated.

## Context

`smoke-instrument-k48` delivered the instrument. Read its running log and the
runtime evidence, `#installed-smoke`, for what was observed. In short:

- Under Docker Desktop's default amd64 emulation, the VM's `qemu-x86_64` 8.1.5
  with Rosetta off, CentOS 7's own bash crashes serving a heredoc. The x64
  worker then panics at `0xFFFFFFFFFF601000`. The same worker under the same
  QEMU with glibc 2.39 works. That QEMU gives x86-64 guests no vDSO. The glibc
  control does run there, and it refuses the 2.25 probe.
- A pinned QEMU 10.2.3 (`tonistiigi/binfmt`), registered in a private
  binfmt_misc instance inside `unshare --user --map-root-user --mount --pid` in
  a privileged arm64 container, runs a chroot of the CentOS 7 amd64 root
  filesystem correctly and does give guests a vDSO. There, the worker aborted
  with JavaScriptCore `MemoryExhaustion` at 33 MB RSS, with no limit set.
  Extracting as the mapped root needs `tar --no-same-owner`, because the
  archive records the builder's uid.
- The front scrubs the worker's environment to HOME, PATH, TMPDIR, LANG and
  `LC_*`. So a `QEMU_*` variable never reaches the worker's emulator, and
  emulator options must come from the binfmt registration or a wrapper
  interpreter. The same holds for `cpu-floor-k19`'s CPU models.

`release-smoke.sh`'s `smoke_in_floor_container` is the route to replace for a
platform foreign to Docker. The arm64 route and the in-environment scripts
should need nothing new.

## The trade-off

Which x86-64 implementation counts as this floor's instrument:

1. **A pinned user-mode QEMU in a private binfmt instance** (the 10.2.3 route
   above). It is self-contained, pinned by digest and changes nothing global.
   It is also what `cpu-floor-k19` needs for `-cpu Nehalem`. It is blocked, for
   now, on the JSC reservation failure.
2. **Docker Desktop's Rosetta** ("Use Rosetta for x86_64/amd64 emulation on
   Apple Silicon"). It is cheap to try, but it changes the human's Docker
   configuration, and it makes a release prerequisite of one host setting.
   Rosetta cannot model Nehalem either, so `cpu-floor-k19` would still need
   QEMU.
3. **Full-system emulation**, `qemu-system-x86_64` (the host's QEMU 11.1)
   booting a kernel with the CentOS 7 userland. It has a real kernel address
   space and vDSO, and TCG enforces a `-cpu` model. But it is a VM rather than
   the container and user-mode emulation the spec's floor table names, so it
   is a spec change through `design`.

**Recommendation: 1, time-boxed.** Try QEMU's reserved-VA and guest-base options
and one or two other QEMU releases. Apply them through a registration or a
wrapper interpreter, and read JSC's allocation path in Bun 1.4.2's WebKit for
what it reserves. If user-mode QEMU cannot run the worker on a glibc-2.17
x86-64 userland at all, the spec's CPU instrument is unreachable for x64 too.
Then stop and escalate between 2 and 3 with that evidence, and do not weaken
the floor. Ask the human before changing Docker Desktop's settings or its
global binfmt registrations.

## Done when

- `task release:smoke` passes for `x86_64-unknown-linux-gnu`. The case runs
  in the CentOS 7.9 x86-64 userland on a named, pinned x86-64 implementation,
  and its glibc control is seen to refuse the 2.25 probe there.
- Or the escalation above is made, with the evidence, and this leaf stays
  live.
- The runtime evidence, `docs/RELEASING.md`, the spec notice and the changelog
  say where x64 executed. `cpu-floor-k19` can reuse the environment and add a
  CPU model.

## Decisions (running log)

**The JSC failure is QEMU's guest layout, not a reservation limit.** Under
`qemu-x86_64` 10.2.3 run directly (`-L` on the CentOS 7 amd64 rootfs, no
binfmt), the worker reproduces `MemoryExhaustion` in
`LocalAllocator::allocateSlowCase`, and QEMU's `-strace` shows no failing
syscall before it. The guest's own `/proc/self/maps` shows why: libc, the stack
and the vDSO sit at `0xffff…`, bit 47 set, which no x86-64 kernel gives a
process (user space ends at `0x7fffffffffff`). QEMU 9.2.2, 10.0.4, 10.1.3 and
Docker Desktop's own 10.2.3 build lay the guest out the same way on this
48-bit-VA arm64 kernel; only 8.1.5 keeps it below 2^47 (from `0x2aaaaaaab000`),
and 8.1.5 has no x86-64 vDSO, which is what crashes glibc 2.17 there. `-R`
cannot help: every reservation size fails with "Cannot allocate vsyscall page",
since QEMU maps the vsyscall page at `0xffffffffff600000`. A guest base of 2^47
(`-B 0x800000000000`) maps the host's upper half onto guest `[0, 2^47)`; the
guest's maps then read `0x7fff…`, and the worker starts and sends its hello.

**The instrument: pinned QEMU 10.2.3 with that guest base, behind a native
interpreter, in a private binfmt instance.** Option 1 of the trade-off, now
unblocked; no Docker setting or global registration changes, so neither
escalation is needed. The front scrubs `QEMU_*`, so a static arm64 wrapper,
built by Zig, is the registered interpreter and execs QEMU with the options.
This build (`tonistiigi/binfmt` `qemu-v10.2.3-68`) always takes the argument
after the file name as argv0, which is the `P` flag's layout, so the wrapper
passes the kernel's argv through unchanged after its options. Registration
uses `PF`: `F` opens the wrapper at registration, while the wrapper execs QEMU
by a path inside the chroot. Prototyped by hand on 2026-09-30, the unchanged
target script and cases passed the k17 x64 archive through both fronts, as uid
1000, with the control refusing the 2.25 probe, in about 7.5 s.

**The namespace is identity-mapped, so the run stays uid 1000.** Only a new
user namespace gets a private binfmt instance. `unshare --map-root-user` maps
uid 0 alone, so nothing can drop to 1000, and `--map-users` needs `newuidmap`,
which ubuntu:24.04 lacks. The container is real root, so it writes
`0 0 65536` into the child's `uid_map` and `gid_map` itself. The child, blocked
on a FIFO until then, re-execs and so gains its namespace capabilities. It
mounts binfmt_misc, registers, binds the inputs read-only with `/proc` and
`/dev`, and runs `chroot --userspec=1000:1000`. The user cannot write `/etc`
in the chroot.

**No privileged container.** The helper runs with Docker's default seccomp
profile, no added capability and no network, and still creates the user
namespace, mounts binfmt_misc and chroots; `--privileged`, `SYS_ADMIN`,
`seccomp=unconfined` and `apparmor=unconfined` each passed as well, and none is
needed. Two namespace rules shaped the script. A remount of the `:ro` volume
fails, because the namespace cannot clear flags Docker locked, so a plain bind
keeps it read-only. `/proc` and `/dev` are bound with `--rbind`, so no new
procfs, and so no pid namespace, is needed.

**The route, as built.** `scripts/release-smoke.sh` runs a target native to
Docker in the floor container, as before, and `x86_64-unknown-linux-gnu` on an
arm64 Docker through `smoke_under_qemu`; any other foreign pairing is refused
by name, since no other emulated route has executed. The host exports the
floor image's amd64 filesystem, copies `/usr/bin/qemu-x86_64` out of
`tonistiigi/binfmt@sha256:400a4873…` (qemu-v10.2.3-68), and builds the
interpreter with Zig (`aarch64-linux-musl`, static), its options a
per-architecture list that `cpu-floor-k19` extends with `-cpu`. The new
`scripts/release-smoke-qemu.sh` runs in `ubuntu:24.04@sha256:008173c2…`. It
unpacks the userland, sets up the namespace, and asserts the guest's own maps:
a vDSO, and nothing but the vsyscall page at or above 2^47. Then it runs the
unchanged target script and cases as uid 1000. The in-environment scripts need
no change.

**Controls seen to fail, each on a scratch copy of the scripts, whose repo
digests were unchanged afterwards.** With no guest base, the layout check
refused libc at `0xffff7f200000`. With the check also removed, the worker
aborted with `MemoryExhaustion` and inspect exited 5. With Docker Desktop's own
QEMU 8.1.5 (`tonistiigi/binfmt@sha256:a870fb64…`, desktop-v8.1.5-44), the
check refused a guest with no vDSO. With ubuntu:24.04's amd64 filesystem as
the userland, the target script refused glibc 2.39. With that check removed,
the control failed because the 2.25 probe ran. With the route condition
unmatched, x64 was refused by name.

**The measured run, on the finished source.** One run was stopped two minutes
in, during the archive build, to bound the address check to fifteen hex digits
so bash's signed arithmetic cannot wrap. Its no-guest-base control was then seen
to fail again against the revised check. Then `task release:smoke` rebuilt all three
archives (SHA-256 `84d17bdf…` macOS arm64, `c13c51ab…` Linux arm64,
`5c92e48f…` Linux x64; worker build `bdd58ee3…`, Bun 1.4.2) and passed every
target through both fronts: macOS arm64 natively, Linux arm64 in the native
floor container, and Linux x64 under QEMU 10.2.3 with the guest address check
passing. Both Linux controls refused the 2.25 probe. The task exited 0. The
eight instrument files it reads had identical digests before and after.
