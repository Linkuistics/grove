# linux-floor-backend-k53


## Goal
Prepare a reusable, deterministic real bubblewrap backend for both Linux floor
userlands without changing sandbox or container privileges.

## Context
Read linux-standalone-floor-k52 and scripts/release-smoke*.sh. Upstream
bubblewrap 0.11.0 requires libcap; CentOS's pinned images already provide 2.22.
The native container currently refuses namespace creation under default seccomp;
QEMU rejects namespace clone flags. k54/k55 own those environment decisions.

## Done when
- A Taskfile entry builds both target backends from SHA-256 pinned official
  source, using existing pinned floor-image mechanics and exact glibc 2.17.
- Target architecture, source pins and floor libcap bytes are checked before
  publication into a fresh output directory. Corrupt/missing inputs, unknown
  targets and existing output refuse; no source substitution or fake backend.
- Both backends execute/version in the pinned floor userlands; existing glibc
  and CPU controls stay valid, with complete frozen evidence. Native default
  namespace refusal is recorded accurately, without claiming standalone success.
- Reusable docs describe the exact interface for k54/k55 and remaining acceptance.
  No installation, release, owner-config or adjacent-checkout edits occur.

## Notes
Retire only this child. k52 remains live for native and QEMU acceptance before k51.

## Decisions (running log)

- Keep backend preparation/probe as explicit test-only tasks. Do not wire the
  execution diagnostic into release:smoke or call namespace refusal a passing
  standalone case. k54/k55 will consume the built executable/library and preserve
  all parent acceptance, with no relaxation performed by k53.
- Reuse existing Bash test style and pinned native/QEMU instrument; add exact
  source/library digest and ELF target/padding controls, with real mutated-file
  red/green evidence. No new testing framework or production runtime is needed.
- One in-session reviewer found four actionable issues: random stage paths in
  binaries, absent/../existing destination merging, cross-filesystem partial
  publication, and ELF checks preceding the executed private copy. Fix all four
  with stable prefix mapping/debug suppression, normalized sibling staging and
  exclusive native rename, and validation of the private staged executable.
  Reproduce normalized-path failure, run native rename/collision/EXDEV controls,
  and compare both rebuilt executables individually. These executable seams
  discharge the findings without a second reviewer; no new sandbox decision.

## Handoff

The child contract is met. docs/linux-floor-backend.md owns the durable input,
Taskfile, build/publication and remaining-environment contract. Prepared inputs
are target/k52/backends-final (exact manifest and executable/library hashes).
Three independent builds match each binary and inputs.tsv byte for byte.

Frozen probe-final.log executes actual bubblewrap 0.11.0 under native arm64 and
both existing QEMU CPU floors; glibc/CPU/address controls pass. Native namespace
creation returns EPERM and QEMU returns EINVAL. Neither is standalone acceptance.
k54/k55 retain all real confinement/publication/refusal and security decisions.

check-final.log passes 16 principal checks, 1,378 workspace cases, nine doctests
and six final/all books. independent-ravel-final.log passes every target's
installed Ravel-only route. Native/macOS and pinned Linux atomic collision/
EXDEV controls, the deliberate ordinary-rename mutation failure, source/ELF/
normalized-output controls and Ruff results all pass at their named boundaries.

before-final.json/after-final.json and digest-comparison.json retain 35,314
matching individual subjects. One mutable shared original dispatch database
changed as another live BookScanBuilder driver recorded its ending/new launch;
read-only shared-store-observations.txt identifies that event and both differing
hashes remain recorded. It was not edited/restored or read as a backend input.
Installed driver/keg/front/worker/helper, policies, input archives and all adjacent
tracked checkout bytes match; those checkout state snapshots remain clean.

k52 remains live with k54/k55 before k51. This session performed no installation,
publication, owner-policy change, parent close or teardown. The original installed
22.1.0 grove-llm and harness-dispatch exit channel remain the ending contract.
