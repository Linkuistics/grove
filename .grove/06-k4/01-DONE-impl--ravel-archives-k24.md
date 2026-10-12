# ravel-archives-k24


## Goal
Deliver Ravel's independent three-target unreleased archives, exact manifest
controls, CPU/glibc-floor installed checks and tested formula template.

## Context

First slice of ravel-delivery-k4; work in /Users/antony/Development/Ravel from
51b73cbffaa46882d7f9ebad15058e17dbd7bec4. Adapt the actual source project's
release-common/build/smoke/smoke-target/smoke-qemu and manifest tests. Keep the
runtime pins, supported targets and floors in the parent brief.

## Done when

- Root tasks build all three unreleased 0.0.0 archives and run their installed
  seam without Grove or host Bun/Node. Every target and floor control passes.
- Archives match an exact independent manifest: ravel, matching ravel-policy,
  generic SDK/static/sample assets, notices, LICENSE and README only. All
  omission/extra/type/mode/probe controls fire; a substituted probe worker is
  refused at the installed-layout seam.
- ravel.rb template installs that prefix and its fake-agent opaque-result tests
  pass from a symlinked temporary prefix; ravel-lite.rb/tap remain intact.
- Commit recipes/instruments/docs in Ravel through jj. Record source revision,
  paths/digests, individual target/control evidence and full logs for k25/k11.
  No tag, public cut, permanent install or Grove/live-driver replacement.

## Notes

Implementation sequence: adapt manifest tests first and observe failure without
the pipeline; port build/smoke helpers and formula; validate focused controls;
freeze subjects, build/smoke all targets and run one full relevant check; record
handoff and seal both repositories. Release cut/notes/prerequisite/publish and
partial-release recovery are ravel-release-lifecycle-k25's own slice.

## Decisions (running log)

- 2026-10-12: Keep the production worker build and its pinned Bun fetch intact.
  Transfer the existing release archive/floor instruments mechanically, adjusting
  only Ravel-owned names/layout and using its already verified installed seam.
  Formula tests must exercise two public verbs because Ravel has no inspect API.
- 2026-10-12: Compare the manifest to a fresh production worker build, not the
  pre-existing target/libexec directory. That comparison fired on a stale SQLite
  notice; Ravel's source emits only NOTICES.md and bun-LICENSE.md. The manifest
  and independent-required-assets control now match that fresh output.
- 2026-10-12: Execute the formula's real install and test methods through the
  installed Homebrew API, redirecting only prefix/testpath to owned temporary
  directories. This avoids a substitute Formula DSL and performs no keg/tap
  installation. Executable mutation/omission/floor/identity controls are the
  doubt instrument for this mechanical pipeline transfer; a duplicate prose
  reviewer adds no independent evidence for those behavioral claims.

## Handoff

Ravel main is sealed at `454ecc817342860b645af06a3c42212fe72baeb9`, followed by
an empty change. It descends from configured source 51b73cbf. Working source
and instruments, delivery docs, the formula template and durable independent
evidence are committed there; no Ravel version cut/tag/push/publication or
permanent installation occurred.

The durable evidence is `/Users/antony/Development/Ravel/docs/archive-evidence.md`.
All unreleased artifacts are in `/Users/antony/Development/Ravel/target/archives/`:

| Artifact | SHA-256 |
|---|---|
| ravel-v0.0.0-aarch64-apple-darwin.tar.xz | 591e93826b249744094cce3ac3903a4f7115a79941b3e5fca15b7fc89894683b |
| ravel-v0.0.0-aarch64-unknown-linux-gnu.tar.xz | 85d1968e8743acc257a4324b2b1ec2bf4d9279002abb36ce941cc7564c5ed9a9 |
| ravel-v0.0.0-x86_64-unknown-linux-gnu.tar.xz | c4341611ee2264d889b728c88deb2f3a2f85967c20ddacfae10e30643d54df32 |
| ravel.rb, unpublished template rendering | cfe1d980757c1601ec1a0ac82afcbb30f37abb40f26cdade1c8fe284c4cbdc1a |

`task release:smoke` rebuilt all three, asserted exact manifests and passed
direct/symlink installed seams without Grove/Bun/Node. macOS ran natively;
Linux arm64 ran natively on Docker and under QEMU 10.2.2 Cortex-A53; Linux
x86-64 ran under QEMU 10.2.3 Nehalem. Both Linux floors enforced glibc 2.17,
refused GLIBC_2.25 with exit 1, killed above-floor instructions with SIGILL/132
and ran the same probes with -cpu max. Both archive ELFs matched each emulation
registration; guest mappings were bounded and had a vDSO. No target was skipped.

`task release:test` passed every-file omissions, unexpected caller/cache/probe
assets, type/mode/top-level and AppleDouble/xattr controls, comparing a fresh
production worker build to the exact manifest. `task release:formula` built
native probes and passed the actual Homebrew install/test methods in a private
prefix with spaces/Unicode, through a symlink. All four substituted probe
workers refused with worker_identity_mismatch at that archive-derived layout.
`task check` passed all diagnostics and 134 runtime tests. Its 14 ignored
subprocess fixture roles are invoked by the real acceptance tests; they are not
unrun target checks. No prerequisite was missing.

All 14,171 frozen subjects retained matching individual digests, including
production source/dependencies/instruments, Homebrew's actual Ruby API,
configuration, tap formulas and the original installed Grove/dispatch pair.
Only the durable evidence document was added after measurement. The tap is
clean at e7416e57 with ravel-lite.rb intact and ravel.rb absent. Owner policy
digests match k16; the original driver/dispatch ending contract is preserved.

Complete logs and instruments under Ravel target/: `k24-handoff-smoke.log`,
`k24-handoff-archives.log`, `release-archives.7aMcNf`,
`k24-handoff-formula.log`, `release-formula.zJHaNX`, `worker-check.eyljnw`,
`check.c8wwOj`, `k24-subjects.py`, `k24-subjects.json`,
`k24-subjects-after.json`. The durable evidence describes per-route outcomes
and the measurement's scope. Graph generation 2026-10-12T12:19:50Z was refreshed
for Ravel; fast mode excludes scripts/docs, which were read directly. Relevant
Rust identity-test graph/snippet and coverage checks were used; no graph-wide
completeness claim was made.

The child contract is met. k4 remains live through ravel-release-lifecycle-k25:
release-doctor/prepare/publish/notes/config and cut/prerequisite/tap/order tests,
dry-cut validation and complete recovery commands are **unfinished**, not skips.
No version-cut or publication command has been validated by k24. k11 must wait
for that slice and recheck tags/source/tap, rebuild 0.1.0 tagged artifacts and
rerun every target/formula check before publishing.
