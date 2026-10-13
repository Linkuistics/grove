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

## Decisions (running log)

- Native preparation uses a digest-pinned upstream Moby seccomp allowlist,
  extended only for bubblewrap's exact namespace clone, unshare(NEWUSER), mount,
  pivot_root and umount2. Keep uid/gid 1000, drop all outer capabilities, enable
  no-new-privileges, keep network none and every input read-only. Docker's
  systempaths=unconfined is also necessary for private procfs: retain controls
  reproducing both default-seccomp and masked-proc refusals. This is an explicit
  test-environment contract, never a production archive or Ravel change.
- Implementation sequence: pin/validate/derive the profile with changed-input
  controls; add a native-only runner reusing archive admission and glibc probes;
  exercise existing installed callers plus real denial and pre-selection refusal
  cases; freeze per-file subjects and complete the handoff check. QEMU and CPU
  emulation remain k55's existing instrument and acceptance obligations.
- The one fresh reviewer identified a real attribution gap: system/proc write
  failures alone can be ordinary DAC/namespace refusals. Require existing paths
  and payload mountinfo read-only flags for /, /usr/bin, /proc and /dev before
  attempting writes. Keep executable positive/negative mount controls; this
  narrow tested correction does not require another reviewer.
- The full check caught GROVE_ARCHIVES only in the native script's usage
  placeholder. Rename it ARCHIVES to match the task interface; introduce no new
  runtime environment surface or classifier exception. The first frozen run's
  subjects matched individually despite its inventory-test failure; retain that
  failed log and repeat complete verification after this correction.

## Handoff

The native contract is met. docs/linux-floor-native.md describes the explicit
filtered seccomp/private-procfs test environment, reusable native tasks and
complete evidence. The native rebuild and final frozen native run pass actual
Grove standalone publication, direct/symlink lifecycle/result/import paths,
material sandbox denials and missing/default-seccomp/masked-proc pre-selection
refusals with no output publication. uid/gid 1000, empty outer capability
bounding set, no-new-privileges, network none, read-only inputs and glibc 2.17
controls hold. CPU/QEMU instruments are unchanged and remain k55's obligation.

target/k54/check-final.log passes 17 principal checks, 1,378 nextest cases,
nine doctests and six final/all books. The one review issue is resolved by
payload mount/existence assertions and a real writable-proc mutation control.
The initial usage-placeholder inventory failure and corrected 12-test focused
check remain saved; the corrected whole check passes on frozen subjects.

35,325 subjects match individually before/after final execution. One shared mutable
original dispatch database differs when another BookScanBuilder session ended/
relaunched; read-only immutable SQLite observations classify it. No history
restoration/mutation occurred. Original installed 22.1.0 fronts/complete keg,
independent installed Ravel, configs and adjacent tracked checkout bytes/state
are preserved. Pre-edit source and generated-archive changes are separately
recorded; release:archives replaces the entire generated archive directory.

k52 stays live for k55; k51 retains final rebuilt all-target acceptance. No
release, installation, owner-policy change or teardown occurred. Retire only
k54, seal its artifact/coordination changes through jj, then send the original
harness-dispatch exit signal as this session's last action.
