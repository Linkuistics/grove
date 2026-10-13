# linux-standalone-floor-k52

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
