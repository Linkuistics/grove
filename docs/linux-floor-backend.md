# Linux floor confinement backend preparation

The installed Grove standalone caller needs Ravel's real Linux filesystem
backend at `/usr/bin/bwrap` or `/bin/bwrap`. The pinned CentOS 7.9 images provide
glibc 2.17 and libcap 2.22 but no bubblewrap. The preparation below builds a
real target executable; it does not enable namespace support or claim that a
standalone invocation has passed.

```sh
# Fresh output; defaults to both Linux targets. Requires Zig 0.17.0 and Docker.
task release:floor:prepare OUT=target/my-floor-backends

# Local changed/missing-input, wrong-architecture/padding and output controls.
task release:floor:controls

# Existing arm64 native + Cortex-A53 and x86-64 Nehalem QEMU diagnostics.
task release:floor:probe BACKENDS=target/my-floor-backends
```

`TARGETS=aarch64-unknown-linux-gnu` restricts preparation to one target. The
probe requires both. Each `OUT/TARGET/` contains `bwrap` and the actual floor
`libcap.so.2` used at link time. `OUT/inputs.tsv` records compiler version,
pinned floor image, source digests and each built executable/library digest.
Source caches live under `target/floor-sources/`; every cached/fetched input is
copied privately and checked before extraction. Changed caches refuse rather
than refetching. Remove or repair a corrupt cache explicitly before retrying.
Unknown targets and existing output paths fail before downloads/builds. All
builds finish before the prepared directory is exposed.
Preparation normalizes the destination after creating its parent, stages beside
that parent, and publishes with a single exclusive native rename. An existing
directory, file or symlink refuses even if it appears during the build. There
is no merge or cross-filesystem copy fallback. The host needs Python 3 and
macOS 10.12+ `renamex_np`, or Linux glibc 2.28+ `renameat2` with filesystem
support for `RENAME_NOREPLACE`; these are host requirements, not guest floors.

## Pinned source and build contract

| Input | SHA-256 |
|---|---|
| [bubblewrap 0.11.0 source](https://github.com/containers/bubblewrap/releases/download/v0.11.0/bubblewrap-0.11.0.tar.xz) | `988fd6b232dafa04b8b8198723efeaccdb3c6aa9c1c7936219d5791a8b7a8646` |
| [libcap 2.77 source, headers only](https://www.kernel.org/pub/linux/libs/security/linux-privs/libcap2/libcap-2.77.tar.xz) | `897bc18b44afc26c70e78cead3dbb31e154acc24bee085a5a09079a88dbf6f52` |
| CentOS arm64 `/usr/lib64/libcap.so.2.22` | `47c73bce965ec946e14062b123a712df3505d79db5ec242602337e8830d91398` |
| CentOS x86-64 `/usr/lib64/libcap.so.2.22` | `79733b599d1c3a2415ff5575b49e4521e5db7d6eefaa40986aadf3e644404fc6` |

The CentOS manifest-list pin and platform mapping remain owned by
`scripts/release-smoke.sh`. Library extraction uses `docker create/cp/rm`
without executing either image. Zig compiles the four files named by upstream
[meson.build](https://github.com/containers/bubblewrap/blob/v0.11.0/meson.build)
for `ARCH-linux-gnu.2.17`, with `_GNU_SOURCE`, required user namespaces and
SELinux integration disabled. Debug output is disabled and source paths map to
`/floor-build`, so random stage paths do not affect executable bytes. No source
is patched. Bubblewrap's libcap use in
[bubblewrap.c](https://github.com/containers/bubblewrap/blob/v0.11.0/bubblewrap.c)
is `cap_from_name`; the newer header is a declaration input, and the actual
runtime stays the pinned floor library. There is no setuid bit, added capability
or new production dependency. ELF64 little-endian target/type/padding checks
refuse another architecture or padding that could escape binfmt registration.

The probe independently admits public Ravel 0.1.0 using its archive SHA-256 and
manifest instrument. It copies the backend first and checks the private staged
ELF bytes used for execution, then stages the backend/library read-only and uses the same
floor image, QEMU versions, address-space check, glibc 2.25 refusal and
CPU-beyond-floor SIGILL/132 plus `-cpu max` positive control. It checks actual
backend and separate Ravel versions and compares the runtime floor library
with the linked bytes. It then attempts real namespace creation and explicitly
reports either success or a recognized namespace refusal. Its successful exit
means those diagnostics ran, never that standalone acceptance passed. It runs
no fake backend and makes no privilege/network/seccomp changes to the existing
routes. This task is not wired into release acceptance as a substitute for
`release:smoke`.

## Environment boundaries

Docker 29.8.1's original native profile refuses unprivileged namespace creation.
[Docker's documented seccomp contract](https://docs.docker.com/engine/security/seccomp/)
explains the blocked calls and custom-profile interface. A native environment
change needs an explicit supported contract and preserved user/capability/
network/read-only-input controls; installing this binary alone is insufficient.

Pinned QEMU user mode also rejects namespace clone flags. The explicit invalid
fork mask in [QEMU 10.2.3 syscall.c](https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/linux-user/syscall.c)
and [10.2.2 syscall.c](https://gitlab.com/qemu-project/qemu/-/blob/v10.2.2/linux-user/syscall.c)
excludes NEWNS/NEWUSER/NEWPID/NEWIPC/NEWUTS, which bubblewrap's `raw_clone`
requires. Target execution and CPU-floor emulation do not imply namespace
support. The real empty sandbox must also retain read-only emulator resources;
the former `/qemu` path lay outside Ravel's implicit system reads. The
[QEMU confinement instrument](linux-floor-qemu.md) now supplies a native kernel
backend and read-only emulator resources under `/usr/libexec`.

The [native floor instrument](linux-floor-native.md) supplies a supported
test-only filtered namespace/procfs environment and actual native acceptance.
The native and QEMU instruments now pass actual separate-prefix callers,
standalone publication and backend-unavailable refusals. The complete k52
contract is reconciled in [the QEMU handoff](linux-floor-qemu.md); k51 retains
final freshly rebuilt all-target/formula/ignored-test delivery acceptance.
Both instruments keep actual sandbox denials and no-publication controls. They
change no original driver/keg/ending channel, owner policy, adjacent checkout,
installation or production archive. The preparation evidence below records the
earlier diagnostic-only stage, before these environments were supplied.

## Prepared handoff — 2026-10-13

`target/k52/backends-final/` holds the prepared pair and exact `inputs.tsv`.
Independent repeat and frozen measured builds match each executable and the
whole input manifest byte for byte. Final executable SHA-256 values are:

| Target | SHA-256 |
|---|---|
| aarch64-unknown-linux-gnu | `21ad63faa809f91a95a036edcac5e9d35547616c90d56013992e988a4844b561` |
| x86_64-unknown-linux-gnu | `4cbf65c81477e5bfb4fa679937db54ba24ac01ba8204797fce69cac4bd06231d` |

**Re-measured 2026-10-18.** A release smoke test refused the aarch64 backend
because the build no longer reproduced the 2026-10-13 hash. The values above are
the current ones. The inputs are unchanged: the bubblewrap 0.11.0 and libcap
2.77 source archives, the floor `libcap.so.2` (`47c73bce…`), the CentOS image
digest and Zig 0.17.0. Repeat builds into different output paths agree byte for
byte. The cause of the drift is not known, and the earlier binaries are not
kept, so the old and new executables were not compared. The backend is a
test-only helper and ships in no archive.

`probe-final.log` verifies both real backends' versions and floor libcap bytes.
Native arm64 returns bubblewrap's EPERM namespace diagnostic; Cortex-A53 and
Nehalem QEMU return its EINVAL unsupported-namespace diagnostic. Every glibc,
CPU and address-space control passes. These are recorded environment refusals,
not successful standalone cases. Native/QEMU support remains k54/k55 work.

`check-final.log` passes all 16 principal checks, 1,378 workspace cases (31
default ignored cases remain), nine doctests and six final/all books; it includes
the new admission/publication controls and explicit installed-Ravel caller and
provenance tasks. `independent-ravel-final.log` passes all three targets through
the independent Ravel-only installed cases. Runtime tests use shell fake agents.
`atomic-linux-final.log` covers the Linux no-replace and EXDEV paths inside the
existing official Python image pinned at
`python@sha256:dddfd7e07f9d15aeeca61529320492139d21cac7f0070c00609243e51e4e0016`,
as uid 1000 with no network and read-only scripts. Native macOS collision tests
run in the principal check. `atomic-mutation-red.log` shows that replacing the
exclusive rename with ordinary rename fails the empty-directory control.
Ruff lint/format results are in `ruff-final.log`.

`before-final.json` and `after-final.json`, produced by `measure.py`, retain each
subject's SHA-256 (and symlink target where applicable): all tracked files in
this and the adjacent Ravel/tap/default Grove checkouts; complete installed
Grove 22.1.0 and Ravel prefixes; owner configs; compiler/checker binaries and
the Zig installation; Cargo metadata/resolved registry sources; caller compiler
dependencies; source caches, prepared backends, Ravel/Grove archives and QEMU
input cache. `digest-comparison.json` matches 35,314 subjects individually.
The sole difference is the mutable shared legacy `records.sqlite3`: read-only
`shared-store-observations.txt` records another live BookScanBuilder session's
ending/new launch during measurement. That store is not a backend execution
input and was neither edited nor restored here. Complete difference hashes are
retained; no blanket history-byte-preservation claim is made. All adjacent
checkout state snapshots remain clean at their original revisions.

One fresh reviewer found four actionable preparation issues. Stable compile
paths, normalized same-filesystem staging, exclusive native publication and
checking the private staged ELF resolve them; exact-byte repeat builds and
normalized-destination/collision/mutation/EXDEV controls verify the fixes.
Graph evidence is Tier 2, best effort: operated-on project paths had no recorded
gap; the existing QEMU script's partial line 188 was read directly, as were
excluded Ravel docs. No complete-graph or kernel-floor claim follows from it.
