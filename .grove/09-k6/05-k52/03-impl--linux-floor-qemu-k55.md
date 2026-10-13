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
