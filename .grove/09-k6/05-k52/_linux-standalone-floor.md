# linux-standalone-floor-k52 — brief

## Goal
Prepare Grove's Linux floor test environments to execute actual standalone
confinement through independently admitted Ravel, before k51 all-target acceptance.

## Context
k49's new installed consumers stage separate Grove/Ravel prefixes and native
macOS lifecycle/standalone tests pass. The existing CentOS 7 floor image lacks
/usr/bin/bwrap. Linux arm64 reaches the real standalone caller and Ravel refuses
`confinement_unusable` before policy selection; target/k49/linux-smoke.log saves
the evidence. Earlier floor cases did not exercise confinement. This is an
environment preparation gap, not permission to skip or use an unconfined path.

Read scripts/release-smoke*.sh, grove-installed-smoke.sh and Ravel's actual Linux
confinement contract/source. Reuse existing pinned platform/glibc/CPU instruments.
Grove's complete installed archives and public Ravel pins are the inputs; no
worker, SDK, backend or runtime is added to Grove's production archives.

## Done when
- Supply a real deterministic target-compatible bubblewrap backend and any
  required environment support to the isolated floor container/QEMU userlands.
  Verify version-dependent decisions against official source and pin fetched
  inputs. Keep exact glibc 2.17 and CPU controls; do not relax admission, network,
  privilege/capability or sandbox claims without an explicit supported contract.
- Actual standalone confinement/output publication and lifecycle/result paths
  pass on Linux arm64 native and QEMU, plus x86_64 QEMU, with separate project
  prefixes/versions and no host Bun/Node. Backend-unavailable controls must
  refuse and never publish output; no skip/fake backend/unconfined fallback.
- Keep independent Ravel-only target smoke working. Record complete logs and
  frozen per-file evidence; update reusable tasks/docs and hand the instrument
  to grove-delivery-acceptance-k51 for final rebuilt all-target acceptance.
- Preserve original installed 22.1.0 driver/keg/ending channel, owner policies
  and adjacent checkouts. This leaf changes test infrastructure only and performs
  no installation, release or teardown.

## Decomposition

- linux-floor-backend-k53 prepares pinned upstream bubblewrap 0.11.0 for both
  target architectures, linked against the pinned floor image's libcap 2.22.
  It verifies real floor execution/version and input refusals without changing
  Docker security or claiming standalone acceptance.
- linux-floor-native-k54 supplies a supported native namespace environment and
  runs actual separate-prefix standalone/lifecycle and unavailable-backend
  publication refusals, preserving uid, capability, network and glibc controls.
- linux-floor-qemu-k55 resolves the namespace/emulation boundary, runs both CPU
  floors with actual confinement and refusal controls, freezes complete evidence,
  repeats independent Ravel-only smoke, and reconciles this whole brief for k51.

## Decisions (running log)

- Decompose after source verification exposed two independent environment gaps
  beyond the missing binary: Docker 29.8.1's default profile refuses native
  unprivileged namespaces; QEMU 10.2.3 rejects CLONE_NEWUSER/NEWNS/NEWPID flags.
  Neither is permission to disable confinement or quietly relax security.
  k53 prepares only the real backend; k54/k55 retain all parent acceptance.
- Use current upstream bubblewrap 0.11.0, source SHA-256 pinned, compiled with
  Zig for glibc 2.17. Its only libcap API call is cap_from_name; link the actual
  libcap.so.2.22 extracted without execution from each pinned CentOS image.
  The pinned libcap 2.77 source supplies headers only, never a new runtime.

## Backend preparation handoff — linux-floor-backend-k53

docs/linux-floor-backend.md describes the pinned bubblewrap 0.11.0 target builds,
Taskfile prepare/controls/probe interface, exclusive publication and exact
environment limits. target/k52/backends-final carries both real executables and
floor libraries; three builds match byte for byte. Native and both pinned QEMU
backend execution/glibc/CPU/address probes pass, with EPERM/EINVAL namespace
refusals recorded explicitly. No standalone acceptance is implied.

Full task check passes 16 principal checks, 1,378 runtime cases, nine doctests
and six books; independent Ravel-only all-target smoke passes. 35,314 frozen
subjects match individually. The one shared live legacy-store difference is
classified with read-only evidence of another BookScanBuilder session's ending/
new launch; the store was not restored or mutated here. Original installed
22.1.0 bytes/config and adjacent checkout bytes/state remain preserved.

k53's preparation contract is met; native support and the supported QEMU boundary
are addressed by the following handoffs. Keep
every parent criterion and original driver/ending contract before k51 acceptance.

## Native handoff — linux-floor-native-k54

docs/linux-floor-native.md records the supported native environment and complete
target/k54 evidence. Native arm64 now passes actual separate-prefix Grove
standalone publication, lifecycle/result and installed imports with real Ravel
0.1.0/bubblewrap 0.11.0. Missing backend, built-in seccomp and masked-proc
controls refuse before policy evaluation and publish nothing. Real read/write,
descendant/symlink, root/system/runtime/proc/dev and mount-flag controls pass.
The filtered pinned allowlist plus explicit systempaths adaptation keeps uid
1000, no outer capabilities, no-new-privileges, no network and read-only inputs.
Exact glibc controls hold; CPU/QEMU controls are unchanged.

The rebuilt native task and final 17-principal-check suite pass; 35,325 stable
per-file subjects match. One mutable shared legacy database change is classified
with read-only evidence of another BookScanBuilder driver ending/relaunching;
installed commands/configs and adjacent checkout bytes/state are preserved.
One reviewer issue is discharged by read-only mount assertions and an actual
writable-proc mutation. No installation/release/policy change or teardown ran.

k55 consumes release:floor:native:run and the exact native contract. It must
resolve both actual QEMU confinement routes, repeat independent Ravel-only smoke
and adopt the supported backend/environment in the general all-target release
runner. The existing release:archives rebuild replaces its entire output, so
rebuild all required targets before that acceptance. k52 remains live; no parent
criterion or k51 final rebuilt acceptance has been waived.
