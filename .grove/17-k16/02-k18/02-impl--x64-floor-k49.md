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
