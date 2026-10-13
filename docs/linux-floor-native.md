# Native Linux standalone confinement at the glibc floor

Grove's native arm64 floor acceptance uses the actual independently released
Ravel 0.1.0 and k53's real bubblewrap 0.11.0. It runs both Grove callers from
separate extracted prefixes, with different project versions and no host
Bun/Node. The backend is test infrastructure; Grove's production archive
manifest does not change.

```sh
task release:floor:prepare OUT=target/floor-backends
task release:floor:seccomp:controls
# Rebuild the native Grove archive before acceptance:
task release:floor:native BACKENDS=target/floor-backends
# Explicit already-built archives, useful for final release handoff:
task release:floor:native:run BACKENDS=target/floor-backends \
  ARCHIVES=target/archives RAVEL_ARCHIVES=target/ravel-archives
```

The optional `RAVEL_ARCHIVES` supplies local public archives, admitted by the
same immutable URL/digest/manifest instrument as downloaded ones. Missing or
different target inputs refuse before invocation. The native runner privately
copies and checks the backend and floor libcap against k53's reproducible
digests in [linux-floor-backend.md](linux-floor-backend.md), then mounts the
copies read-only. It pins Zig 0.17.0 for its controls and requires an arm64
Docker. It is deliberately native-only; it does not claim a CPU emulation or
kernel floor. Existing Nehalem/Cortex-A53 QEMU controls remain unchanged and
required for final all-target acceptance.

The rebuild task delegates to `release:archives`, which replaces its entire
`target/archives/` output directory even for one target. Use `native:run` with
an explicit existing archive directory when retaining other prepared targets;
rebuild all targets before the next all-target handoff.

## Supported test environment

The guest is the existing digest-pinned CentOS 7.9 arm64 userland, with exactly
glibc 2.17 and libcap 2.22. Every route runs the glibc 2.17 success and glibc
2.25 refusal pair. The host-tested environment is Docker Desktop 4.93.0,
Engine 29.8.1, LinuxKit 7.0.14. Other kernels/LSMs must support unprivileged
user namespaces and private procfs; an unsupported environment fails the real
Ravel empty-sandbox probe. No sysctl, daemon, host binfmt or installed package
is modified. No AppArmor/SELinux override is supplied.

Each container has `--network none`, `--user 1000:1000`, `--cap-drop ALL`, and
`--security-opt no-new-privileges`. Inputs are read-only bind mounts. The real
backend has neither setuid nor setgid; no outer SYS_ADMIN or other capability
is added. The managed payload is uid 1000 with zero effective capabilities.

Two explicit Docker security adaptations are required. They are limited to this
isolated deterministic fake-agent test, and do not prescribe production policy:

- A filtered custom seccomp allowlist, derived from unchanged
  [Moby profiles commit 4b8214fc4971e06172938a1f1554ccff6647ee80](https://github.com/moby/profiles/blob/4b8214fc4971e06172938a1f1554ccff6647ee80/seccomp/default.json).
  Its vendored source and Apache license are under `scripts/floor-seccomp/`;
  SHA-256 admission precedes derivation. This pinned baseline is explicit,
  rather than asserted identical to Engine 29.8.1's built-in profile.
  [Docker supports this client-supplied profile interface](https://docs.docker.com/engine/security/seccomp/).
  Retain its default `SCMP_ACT_ERRNO` and all existing rules. Append only
  arm64 `clone(0x3c020011)` (SIGCHLD + NEWNS/NEWUSER/NEWPID/NEWIPC/NEWUTS),
  `unshare(0x10000000)` (NEWUSER), and mount/pivot_root/umount2. These are
  [bubblewrap 0.11.0's setup operations](https://github.com/containers/bubblewrap/blob/v0.11.0/bubblewrap.c).
  Namespace-scoped setup privileges remain kernel-gated; no initial-user-
  namespace capability is available. `clone3` retains the baseline ENOSYS
  fallback, and arbitrary namespace clone/unshare flags remain blocked.
- [`systempaths=unconfined`](https://docs.docker.com/reference/cli/docker/container/run/#security-opt)
  removes Docker's masked/read-only system-path overlays. With those overlays,
  the kernel refuses bubblewrap's private procfs mount after namespace creation.
  This exposes more outer-container system paths and is a real relaxation:
  the outer process still has uid 1000, no capabilities, no-new-privileges and
  the filtered syscall policy. Ravel then grants only its documented read-only
  system paths plus the invocation workspace; it mounts its own procfs and
  freezes `/`, `/proc`, and `/dev`. Host mounts are limited to the staged
  read-only inputs and backend. The environment has no external network,
  real model or owner configuration.

The outer test probe checks actual uid/gid, seccomp filter mode,
no-new-privileges and an empty capability bounding set. It materially attempts
outer mount, NEWNET unshare and kernel-keyring syscalls and requires EPERM.
The guest checks the input mount's read-only flag. The successful payload
checks uid, effective capabilities and the actual read-only mount flags of
`/`, `/usr/bin`, `/proc` and `/dev`, then attempts unauthorized reads/writes,
including from a descendant and through a symlink, outside the task workspace;
root, system binary, Ravel runtime, proc and dev writes must fail. Authorized
system reads and a real published output supply the positive controls.

## Refusal and handoff boundary

The same Grove standalone command runs in four fresh containers. Missing
backend, built-in seccomp, and retained masked-proc controls must each report
`confinement_unusable` at CLI admission, leave a policy-evaluation marker absent,
and publish no new output. Their diagnostics must identify the particular
restriction. A neighboring existing file and the owner-secret fixture retain
their bytes. The supported environment must evaluate the same installed policy,
run the denial task, consume its opaque result and publish the requested output.
It also runs the existing direct/symlink lifecycle, standalone and installed
caller-import cases. Complete diagnostics and input digests go to the saved
runner log; no refusal is counted as successful standalone execution.

This native instrument is independent of `release:floor:probe`, which still
reports namespace availability without caller acceptance. The existing general
`release:smoke` route has not yet adopted the supported environment/backend:
linux-floor-qemu-k55 must combine this native handoff with actual QEMU support
and reconcile the all-target release route. Its namespace/emulator issues remain
live; linux-standalone-floor-k52 and grove-delivery-acceptance-k51 remain open.
The original installed Grove 22.1.0 driver, complete keg and exit channel,
owner policies, adjacent checkouts and production release state are preserved.

## Completed native handoff — 2026-10-14

`target/k54/native-rebuild.log` records the reusable native rebuild/acceptance
task. The final measured input is Grove 22.1.0 arm64 archive SHA-256
`52b9e1e5c05c18c69e33d0552355ce7c5e02d415158e5065137a322912f67e54`;
its admitted independent Ravel archive is
`19a20dc895a09440cbe3d8da1ce80b96311fbc1c3e6be5e2bf1eb18bb6cc0f7e`.
Bubblewrap/libcap retain the k53 pins. `native-final.log` records all four
environments, actual glibc success/refusal, outer security denials, both
direct/symlink caller paths and the successful confined denial/publication task.
Each staged script, archive, backend, library, derived profile and compiled
probe has its individual SHA-256 in that log.

`seccomp-final.log`, `shellcheck-final.log` and `ruff-final.log` pass.
`mount-mutation.log` accepts actual read-only mounts and rejects a deliberately
writable private procfs with exit 92. The sole fresh reviewer finding was that
ordinary permissions could explain system/proc write failures; path-existence
and mountinfo checks plus that mutation control resolve it. `reviewer.txt`
records the classification; no second reviewer was materialised.

`check-final.log` passes all 17 principal checks, all 1,378 workspace nextest
cases (31 default ignored cases remain), nine doctests and six final/all books.
`check-first.log` retains the earlier inventory failure for the native usage
placeholder, followed by `inventory-green.log`'s 12 passing focused tests.
The corrected complete check ran on frozen source, not files edited beneath it.

`before-final.json`, `after-final.json`, `measure.py` and `run-final.sh` retain
the complete per-file subjects and execution recipe. 35,325 subjects match
individually: tracked files in this and adjacent Ravel/tap/default Grove
checkouts, the complete original Grove 22.1.0 and installed Ravel prefixes,
owner configs, compiler/checker tools and source dependencies, floor source
caches/backends, input archives, QEMU cache and measurement instruments.
`digest-comparison.json` retains one additional differing preservation subject:
the mutable shared original dispatch `records.sqlite3`. Immutable read-only
SQLite observations in `shared-store-observations.json` identify another
BookScanBuilder session ending/relaunching at 2026-10-13T13:11:13Z. This store
is not a test execution input, and was neither written nor restored here.
There is no blanket history-byte-preservation claim. Adjacent checkout state
snapshots match, and installation/config bytes remain preserved.

The pre-edit comparison separately records intended local source edits, the
updated measurement script and the native rebuild's replacement of the
generated archive directory; it does not confuse those with protected state.
The first frozen run matched all its subjects despite its inventory failure.
Final documentation/handoff notes were appended after the measured run.
Graph evidence is Tier 2 and best effort: operated paths had no recorded gap;
new/stale documentation and shell discovery were checked in source directly.
Neither complete graph coverage nor a kernel-floor claim is implied.

Native k54's contract is met. k55 consumes this supported native task and
retains both QEMU CPU-floor, independent Ravel-only and all-target route work.
No release, installation, live-policy edit, parent close or teardown occurred.
