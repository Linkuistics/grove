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
