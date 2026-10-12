# ravel-release-lifecycle-k25


## Goal
Complete Ravel's independent release tasks and tested version-cut/publication
preparation, ready for ravel-publication-k11.

## Context

Consume ravel-archives-k24's exact Ravel source commit, three unreleased archives,
formula template and target/floor evidence. Work in /Users/antony/Development/Ravel.
The parent ravel-delivery-k4 retains the complete acceptance contract.

k24's sealed Ravel main is `454ecc817342860b645af06a3c42212fe72baeb9`, followed
by an empty change. Read its durable `docs/archive-evidence.md` and
`docs/RELEASING.md`, plus k24's terminal handoff. It supplies working root
release:archives/smoke/test/formula tasks, the exact manifest, real Homebrew
install/test adapter and all passing target/floor/probe checks. The current
release-build.sh deliberately accepts only --unreleased; adapt its release
mode from the actual Grove source pipeline. release-formula.sh already renders
and validates all target digests from one current-version directory; integrate
it with tagged dist production without losing its controls. No lifecycle
cut/preflight/publish/order/prerequisite test or recovery command is claimed
validated by k24. The archives/rendered 0.0.0 formula are local, unpublished.

## Done when

- Adapt actual release-doctor/prepare/publish/notes and release tests/config into
  Ravel root tasks for patch/minor/major cuts. Keep sequential preflight, tap
  checks, archive smoke before bookmark/tag/GitHub/tap publication, release lock,
  jj state preservation and unrelated work intact.
- First minor cut is 0.0.0 to 0.1.0 after rechecking remote version tags; do not
  cut/tag/publish here. Preparation handles the absent v0.0.0 baseline explicitly
  and refuses rerunning any partial cut. Notes require no Grove dependency or
  model launches during validation; retain existing meaningful notes and test
  empty/missing notes and changelog replacement controls.
- Moved tests prove missing prerequisites stop before a cut, dirty/detached/diverged
  taps stop before publication, and installed smoke precedes all publishing steps.
  Validate dry cut/config and formula preparation in an isolated fixture only.
- Document prerequisites, publication commands and recovery at each partial
  boundary without repeating a bump. Test actual tasks, commit via jj, update
  k4/k11 handoff with exact source/evidence and recheck checkout/config/tap state.
- Reconcile all of k4's acceptance criteria before closing the parent. Unrun
  checks or missing prerequisites remain named work, never skips.

## Notes

Independent publication/installation belongs to k11. Do not alter running Grove,
its original installed pair or this driver's harness-dispatch exit contract.

## Decisions (running log)

- 2026-10-12: Adopt the mandated adaptation of the actual sequential Grove
  pipeline. Preserve jj's unrelated working change by identity, the release
  lock, tap refusal before publication and installed smoke before all pushes.
  Test the actual Taskfile and scripts against isolated colocated repos/local
  remotes; replace only expensive builds or external publication boundaries.
- 2026-10-12: Ravel notes use deterministic commit descriptions or an explicit
  reviewed Markdown input; no Grove/model dependency. Preserve meaningful
  existing Unreleased notes. Treat an absent v0.0.0 as initial history only when
  no version tags exist; a local/remote/GitHub/tap partial cut must be resumed,
  never bumped again. Remote currently has only structured-plan-migration.

## Implementation sequence

1. Adapt lifecycle refusal/order fixtures first and observe missing production
   scripts/tasks fail. Add notes/prepare/state-preservation and real cargo-release
   dry/config/replacement controls in isolated repos.
2. Adapt doctor/prepare/publish, release mode/formula preparation, changelog and
   cargo-release config. Run focused fixture tasks until all controls pass.
3. Freeze production scripts/config/dependencies and protected owner/tap/installed
   subjects; run actual lifecycle/archive/formula tasks and the full Ravel check.
   Record per-item digests, complete logs and recovery boundaries.
4. Seal Ravel through jj, reconcile every k4 criterion, promote exact handoff to
   k11/root, retire k25 and seal this coordination change. End only through the
   inherited harness-dispatch exit contract.
- 2026-10-12: The leaf's one read-only reviewer found three actionable P2
  gaps: Bash-floor admission after the cut, incomplete/draft GitHub releases
  passing the completion guard, and wildcard publication of unsmoked extras.
  Add executable controls and fix each before handoff. No trade-off/noise
  finding was returned. These fixes are covered by the isolated fixture seam;
  no second review is needed. Recovery explicitly resumes individual boundaries.


## Handoff

Ravel main is sealed at `64b1e6e7b92ebd3eac28c9765e6c705d40c3e95d`, followed by an empty
change, based on k24's 454ecc81. Its working tree is clean, version 0.0.0.
No production cut/tag/push/publication, permanent installation or model launch
occurred. The remote still has only structured-plan-migration, no version tags.

Durable evidence: `/Users/antony/Development/Ravel/docs/release-lifecycle-evidence.md`.
Publication and every partial boundary's recovery commands:
`/Users/antony/Development/Ravel/docs/RELEASING.md`. Root tasks own patch/minor/
major, notes, doctor, archive/floor/formula and isolated lifecycle tests.
The first actual minor remains 0.1.0, owned by ravel-publication-k11.

`task release:lifecycle:test` passed actual Taskfile refusal/order/lock tests,
real jj/local-remotes, real cargo-release 1.1.2 dry/config/replacement controls,
notes/history/state preservation, baseline/partial/draft/missing-asset refusals,
tagged-build admission, and doctor/Bash controls. Tap-only recovery downloads
and compares every uploaded archive, refuses corruption and retains unrelated
tap work and ravel-lite. The one adversarial review's three P2 findings were
all actionable, fixed and covered by executable controls; none remains open.

`task release:smoke` rebuilt and passed all native/QEMU/glibc/CPU-floor routes,
with direct/symlink installed layouts and no host Grove/Bun/Node. `release:test`
passed all manifest controls; `release:formula` passed the actual Homebrew
methods in private prefixes and all four probe substitutions. `task check`
passed diagnostics and 134 runtime tests (14 ignored subprocess fixture roles,
not unrun target checks). Every preparation prerequisite passed; no check is
missing or skipped.

Current local artifacts under `/Users/antony/Development/Ravel/target/archives/`:

| Artifact | SHA-256 |
|---|---|
| ravel-v0.0.0-aarch64-apple-darwin.tar.xz | 3a96143765421539d38878acc79675f7f99c39d5f992e5879727c5090ff664a5 |
| ravel-v0.0.0-aarch64-unknown-linux-gnu.tar.xz | ae920d63bf327f89c8bbab58e29b01c4efc47fef57f1c1a5d41cae1ba8a18288 |
| ravel-v0.0.0-x86_64-unknown-linux-gnu.tar.xz | 022df70515f7f817c4a02edc97c324e6e436ef7e8ad2ca4aa35645ac56c9f074 |
| ravel.rb, unpublished preparation | 5b10f0dceb53b92b44fcafb91b7c581ba937412636289bf1e304bb2402eca489 |

These supersede k24's local digests. Individual route/control outcomes and
full logs are in the durable evidence; main logs are `release-lifecycle.aBzyix`,
`k25-handoff-smoke.log`, `release-archives.br0ybt`, `release-formula.jdIkis`,
`check.N5kpCt`, `k25-handoff-doctor.log` and `k25-handoff-tap.log` in Ravel target/.
All 14,183 individually frozen source/dependency/Homebrew API/config/tap/
original-installed subjects matched afterward (`k25-subjects*.json`). Only the
new durable evidence document was added afterward. The copied owner config is
unchanged. Tap main is e7416e57, clean, ravel-lite intact, ravel.rb absent.
Original installed Grove/grove-llm/dispatch still report 22.1.0 and match bytes.
This session still ends with its inherited `harness-dispatch exit` contract.

k4's whole preparation contract is met by k24 plus this increment, and closes
with this leaf. k11 still owns the public cut, all fresh tagged 0.1.0 builds/
floor/formula checks, publication, installation and immutable URLs/digests.
Recheck source/tags/tap/config and partial state immediately before that work;
never publish the local 0.0.0 artifacts or repeat a partial version bump.
