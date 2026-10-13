# grove-delivery-contraction-k49


## Goal
Remove bundled dispatch delivery and ship only Grove executables/caller assets,
with independently installed Ravel as Homebrew's runtime dependency.

## Context

Use k47's detached compiler and k48's validated separate target layouts. This
wide refactor must land atomically across workspace/lock, gate/tasks, archive
manifest/build/assertions, formula and installed smoke consumers. Read k42/k46's
test/asset inventory before removing the old package.

## Done when

- Remove crates/harness-dispatch and every obsolete build/typecheck/probe/compatibility recipe only after every applicable caller fixture/asset has a preserved new owner. Grove's own keyed-launch remains local; no shared Ravel library or bundled front/worker/notices ships.
- Exact archives contain Grove executables and assets/ravel-policy relative imports in an installed readable layout; formula depends on linkuistics/taps/ravel, tests caller assets and public CLI integration without host runtime or equal-version assumptions.
- Normal task check uses independent installed Ravel and passes; release pipeline/manifest tests and focused installed lifecycle/standalone paths pass with separate prefixes/different versions. k51 owns final rebuilt all-target acceptance.
- Update affected source-pinned walkthroughs and gate-facing install/release instructions. Enumerate old-name/dependency tokens across the actual affected surfaces with observed positive/cross-tree controls, classifying necessary historical/authority/compatibility remnants. Keep all live-session guards.

## Notes
