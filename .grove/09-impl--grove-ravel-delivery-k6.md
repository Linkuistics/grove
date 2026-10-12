# grove-ravel-delivery-k6

## Goal

Stop building and shipping dispatch inside Grove, and make Homebrew supply
Ravel as an independently versioned runtime dependency.

## Context

- Requires grove-ravel-caller-k5's passing runtime/caller migration.
- Update Grove's workspace/lockfile, Taskfile/check gate, release scripts and
  tests, exact archive manifest and `scripts/templates/grove.rb.tmpl` together.
- Caller assets moved by the preceding leaf remain Grove-owned and ship through
  Grove's delivery. The local runner remains in each project's own source.

## Done when

- Grove archives ship its own executables and caller assets; they contain no
  Ravel/dispatch front or private worker. Grove's formula depends on
  `linkuistics/taps/ravel`; no front/worker version comparison crosses projects.
- Remove the old dispatch package and its build/typecheck/probe recipes only
  after preserving/migrating every caller test and asset. Grove's normal check
  path verifies integration through independently installed Ravel.
- Archive assertions and release-pipeline tests pass with the new manifest.
  Installed-layout tests on all current targets put Ravel in a separate prefix
  with a different version and prove both lifecycle and standalone paths work.
  The formula test checks the dependency and caller assets without a host runtime.
  Obtain each target's Ravel layout from k11's immutable archive URL and digest;
  check target, manifest and digest before invoking it, including Linux/QEMU
  where Homebrew is absent. Missing/mismatched archives fail, never skip. Reuse
  the existing archive and CPU/glibc seams without installing a host runtime.
- Source-pinned walkthroughs affected by workspace/manifest/package removal and
  gate-facing release/install documentation change with this leaf. `task check`
  stays green; broader ownership/prose reconciliation belongs to k12.
- Enumerate remaining old-name/dependency references over the actual affected
  surfaces, classify necessary history/compatibility fixtures, and verify removal
  claims with positive/cross-tree controls. Preserve all live-session guards.
- Prepare and test the live-install transition before k13 can publish: upgrading
  Grove unlinks its old dispatch front, and the running driver keeps its original
  canonical dispatch path for future launches. Retain the complete original keg
  (front, worker, SDK and notices) with Homebrew automatic cleanup disabled for
  the release. Preserve bare `harness-dispatch exit` through a temporary link to
  that original front in an already inherited stable PATH directory, such as
  `~/.local/bin`; refuse to overwrite an existing entry. Do not copy only the
  front or repoint the old canonical path at Ravel. Preserve access to the
  original grove-llm for its admission/teardown contract as well.
  Test an isolated upgrade/unlink with the actual old 22.1.0 front/worker: it can
  launch the next fake session, reap/write the old store, and the bare command
  signals only its temporary legacy channel. Test Ravel's legacy-channel exit
  fallback separately; it is not permission to change the original prompt.
  Record paths/digests, cleanup suppression, link ownership and a post-driver
  cleanup/catch-up procedure. Keep the legacy installation until the driver has
  ended; neither k13 nor final finish removes it before its final signal.

## Notes

Verify source and unreleased archives here. k13 performs the human-requested
mid-grove extraction release after k12; final finish releases session-control
behavior later. No installation changes here. k6's preservation is a temporary
transition instrument outside Grove's new archives/formula, not bundled Ravel.
