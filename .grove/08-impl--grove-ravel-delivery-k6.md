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
- README, usage/release/architecture docs, glossary, ADR/spec set, active
  walkthroughs and plugin guidance agree on ownership and installation. Reconcile
  current-state decisions instead of adding a duplicate extraction specification.
  Existing book/reference validation and `task check` pass.
- Enumerate remaining old-name/dependency references over the actual affected
  surfaces, classify necessary history/compatibility fixtures, and verify removal
  claims with positive/cross-tree controls. Preserve all live-session guards.

## Notes

Verify source and unreleased archives here. The driver-owned finish session
owns final Grove integration/publication under the project's existing rule;
do not install replacement Grove binaries during this live source cutover.
Ravel's release was independently delivered earlier.
