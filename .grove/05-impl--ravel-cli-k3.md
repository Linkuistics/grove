# ravel-cli-k3

## Goal

Deliver a runnable, caller-independent `ravel` CLI and matching private worker
from the existing Ravel checkout, with the owner's copied configuration.

## Context

- Reuse `/Users/antony/Development/Ravel` and Linkuistics/Ravel. Recheck its jj
  state; requirements found README.md modified in uwlrknks/93f202ba over
  main/2f452e51. Preserve the complete old revision, including later local edits,
  under a named retired bookmark before replacing active sources. Do not discard
  unrelated work or rewrite the old daemon design into the new requirements.
- Source: `crates/harness-dispatch` and `crates/keyed-launch`; source recipes:
  `Taskfile.yml`, `scripts/check.sh`, and dispatch's build/install/smoke scripts.
  Transfer workspace dependency, toolchain, lockfile and lint requirements needed
  to build these packages independently. Ravel owns its local runner copy.
- The worker embeds caller modules in `worker/src/main.ts`; build identity,
  declaration generation, typecheck fixtures, sample installation and record
  reporting also refer to them. This is more than a directory rename.

## Done when

- Before source replacement, a named retired revision resolves to the old
  Ravel project and includes the owner's README edit. Record its name and commit
  in Ravel's ordinary project documentation. Commit Ravel changes through jj.
- Ravel's own root Taskfile builds, checks and installs the front/worker pair.
  Preserve pinned Bun/dependencies, matching build identity, hostile probe
  controls, SQLite/runtime notices and the local runner tests. An installed
  prefix works through a symlink to `bin/ravel` with no host Bun/Node or Grove.
- The public CLI, SDK, defaults, generic examples and active README describe
  Ravel. Grove adapter, Grove routing samples and Grove task conventions are
  excluded from the embedded set, declarations, default sample and active
  inspection/run output. Preserve historical record readability with the
  smallest compatible treatment of old fields; do not lose existing records.
- Recheck and copy the actual contents of `~/.config/harness-dispatch` into
  `~/.config/ravel`, preserving both source and any destination edits. Preserve
  policy/settings format, routes, environment grants, bounds, local selections
  and access to existing history. Change only necessary SDK/import locations.
  Inspect the copied personal policy from this Grove workspace and another
  ordinary directory; do not activate a replacement sample or launch a model.
- Existing command-seam cases run against the real Ravel worker and fake agents:
  generic selection, execution, bare exit/ending file, records and old-record
  lookup; cancellation, nested/stale channels, confinement and terminal cleanup.
  Run the transferred package check and local runner suite, then the installed
  smoke cases from a fresh prefix with host runtimes removed from PATH.
- Cargo and directly executed test helpers clear all live authority variables,
  including original and renamed Ravel controls and Grove's launch directory.
  Verify the live session's channel was not created by the checks.

## Notes

This is an expand step: the current Grove source, installed dispatch and its
configuration remain usable. Caller assets stay preserved there for
grove-ravel-caller-k5. Independent publication is ravel-delivery-k4;
controlled-session behavior is ravel-session-control-k7. Do not add either here.
Return to the session workspace to retire/commit this leaf, naming the Ravel
commit in the handoff. Keep the launch prompt's original ending binary usable.
