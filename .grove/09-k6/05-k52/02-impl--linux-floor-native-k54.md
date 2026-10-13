# linux-floor-native-k54


## Goal
Enable and prove actual native arm64 standalone confinement in the pinned floor
userland using k53's real bubblewrap backend.

## Context
Read docs/linux-floor-backend.md and linux-standalone-floor-k52. Docker 29.8.1's
default profile refuses unshare/clone namespaces for uid 1000 without SYS_ADMIN.
Use an explicit supported environment contract, justified against official
Docker/bubblewrap source; do not casually switch native to seccomp=unconfined,
add capabilities, use setuid, or run caller work as root. Original native route
has --network none, --user 1000:1000 and read-only inputs. Ravel 0.1.0 probes
an actual empty sandbox before policy selection and requires /usr/bin/bwrap.

## Done when
- Prepare native floor backend/environment through reusable release tasks and
  scripts. Preserve exact glibc 2.17 and CPU probes, no host Bun/Node, separate
  project prefixes/versions, no production archive additions.
- Actual Grove standalone output publication, lifecycle/result paths and
  installed caller imports pass. Missing/unusable backend controls refuse
  before selection and never publish outputs. Test sandbox denials materially.
- Record supported security/environment contract and complete frozen per-file
  logs, including installation/config/adjacent-checkout preservation.
- Hand working native instrument to k55; k52 stays live for both QEMU targets.

## Notes
