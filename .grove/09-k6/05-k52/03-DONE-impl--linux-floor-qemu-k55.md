# linux-floor-qemu-k55


## Goal
Resolve and prove actual standalone confinement under both pinned QEMU CPU
floors, then reconcile linux-standalone-floor-k52 for final k51 acceptance.

## Context
Consume k53 backend preparation and k54 native instrument. Read their durable
docs and logs, Ravel's actual confinement source and scripts/release-smoke*.sh.
Official QEMU 10.2.3 linux-user/syscall.c: CLONE_INVALID_FORK_FLAGS explicitly
rejects NEWNS/NEWUSER/NEWPID/NEWIPC/NEWUTS. Bubblewrap 0.11.0 raw_clone uses those
flags, so merely copying a target bwrap into the existing user-mode chroot will
not satisfy the contract. Kernel namespaces require a real supported boundary.
Existing interpreter/qemu live in /qemu, outside Ravel's implicit system reads;
a real empty sandbox also needs its interpreter resources available read-only.
Do not exempt Grove/Ravel payload ELF files from the CPU model, substitute a
fake backend, strip sandbox options or use unconfined fallback. Preserve the
Docker host's binfmt registrations and original 22.1.0 driver/channel.

## Done when
- Supported real confinement works for arm64 cortex-a53 and x86_64 Nehalem at
  exact glibc 2.17, with existing positive CPU/glibc/address-space controls.
- Actual separately admitted Ravel 0.1.0 and complete Grove archives pass
  standalone publication, lifecycle/result, direct/symlink caller imports,
  sandbox denials and absent/unusable backend no-publication controls.
- Independent Ravel-only all-target smoke still passes. Freeze inputs/tools/
  installed driver/config/adjacent checkouts individually before/after complete
  measurements and keep full logs. Update reusable tasks/docs and pass task check.
- Reconcile every k52 criterion and promote the exact instrument and remaining
  limits to k6/k51 for final freshly rebuilt all-target acceptance. No installation,
  release, production archive backend/runtime, owner-policy edit or teardown.

## Notes

## Decisions (running log)

- Test the real host-native bubblewrap against pinned arm64 glibc 2.17/libcap 2.22 inside each private target userland. Keep every Grove/Ravel ELF unchanged under automatic pinned binfmt execution. Namespace syscalls belong to the real kernel backend, not QEMU's unsupported clone translation; no sandbox options are stripped.
- Replace chroot with a native pivot_root followed by all-bounding-capability, group and uid/gid drops plus no-new-privileges. Linux forbids unprivileged user namespace creation from a chroot. Keep the private binfmt mount inside the new root so detaching the old root cannot discard the registration. The first x86 pivot experiment refused Exec format, revealing that lost mount; add in-sandbox CPU refusal to catch silent native-arm fallback too.
- Relocate emulator resources under Ravel's existing read-only /usr/libexec grant. One fresh reviewer found explicit raw QEMU invocation could choose another CPU/base; the instrument measures trusted deterministic automatic exec paths, not malicious anti-emulator bypass resistance. Remove the max interpreter from the sandbox-visible tree after its positive control, and exercise the beyond-floor instruction in an actual sandbox descendant. All actual payload archive ELFs remain unexempted.
- Retain the pinned native default-deny seccomp allowlist and add only helper unshare(NEWUSER|NEWNS). The root setup helper must regain capabilities after identity mapping, so no-new-privileges is set by the native launcher after pivot, before target execution. Outer default capabilities do not increase; target uid 1000 has no effective or bounding capability, no network, and read-only inputs. Native security assertions verify the actual kernel state.
- Confirm the host-native backend's complete isolated glibc 2.17/libcap 2.22/libattr closure with the actual loader on both targets. Zig 0.17 emits RUNPATH, so force libattr as a direct needed library; do not rely on a requested legacy-dtags flag or globally grant LD_LIBRARY_PATH. Independent prepared outputs/manifests match byte for byte.
- Use existing pinned Bun 1.4.2 on a temporary developer-check PATH after the default host Bun 1.4.3 correctly refused the check gate. Final preservation includes previously omitted ~/.config/grove files. No tool installation or production runtime/dependency is changed.

## Handoff

All leaf criteria are met; docs/linux-floor-qemu.md is the durable instrument,
security/measurement contract and evidence index. Normal all-target release
smoke now runs actual native/QEMU namespace environments automatically, through
separate admitted Grove 22.1.0/Ravel 0.1.0 prefixes. Both pinned QEMU CPUs pass
missing-backend and real-emulated-unusable pre-selection/no-publication controls,
actual standalone publication, lifecycle/results, direct/symlink imports and
filesystem denials. Existing glibc/CPU/address controls and an actual sandbox
CPU descendant refusal pass. Native arm64 repeats its three refusal controls
and positive case. No payload ELF is exempted, sandbox option stripped, fake
backend substituted or unconfined fallback used.

Complete target/k55/all-target-complete.log, qemu-task-complete.log and
independent-ravel-complete.log pass; outcomes.json verifies all 42 named target/
route/control outcomes individually. check-complete.log passes 18 principal
checks, 1,378 workspace cases, nine doctests and six books, plus explicit
installed-Ravel caller assets/provenance. Profile red/green and an actual lost-
binfmt-mount mutation demonstrate the controls fail when their premises fail.
Every staged container input is individually hashed before/after.

The final expanded freeze measures 35,356 subjects; 35,355 stable file/symlink
subjects match, including complete installed original 22.1.0/Ravel prefixes,
owner Grove/Ravel/legacy configs and all adjacent checkout bytes/state. The sole
shared legacy SQLite change is classified by read-only evidence of another
BookScanBuilder driver ending/relaunching at 14:01:36 UTC; this run remains live.
No store restore/write was performed. Full snapshots, comparison and restricted
metadata audit remain in target/k55. Raw explicit emulator invocation is outside
the trusted automatic-exec CPU claim; no kernel-floor or invisible host-binfmt
inventory preservation claim is inferred. Graph partial/excluded ranges were
read directly; evidence remains best effort.

Reconcile/close linux-standalone-floor-k52 and promote the exact supported
instrument to k6/k51. k51 still owns freshly rebuilt final whole-delivery,
formula, explicit ignored tests and parent acceptance. No installation,
release, production backend/runtime, owner-policy edit or teardown occurred; keep
original 22.1.0 admission/exit authority and end with harness-dispatch exit.
