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
  in Ravel's ordinary project documentation and push the retired bookmark to
  origin before replacing active sources. Advance main by ordinary descendant
  commits; do not rewrite or force-push away the old history. Commit through jj.
  Recheck version tags: inspection found only `structured-plan-migration`, no
  released version. Use 0.0.0 for the extracted development pair and make the
  first minor cut 0.1.0, independently of Grove's inherited 22.1.0. Keep
  front/worker identity paired at the actual Ravel build version.
- Ravel's own root Taskfile builds, checks and installs the front/worker pair.
  Preserve pinned Bun/dependencies, matching build identity, hostile probe
  controls, SQLite/runtime notices and the local runner tests. An installed
  prefix works through a symlink to `bin/ravel` with no host Bun/Node or Grove.
- The public CLI, SDK, defaults, generic examples and active README describe
  Ravel. Grove adapter, Grove routing samples and Grove task conventions are
  excluded from the embedded set, declarations, default sample and active
  inspection/run output. Preserve historical record readability with the
  smallest compatible treatment of old fields; do not lose existing records.
  Enumerate and classify every Grove-specific reference across Ravel's whole
  active tree (including dotfiles, help, SDK, build inputs, scripts, fixtures,
  samples and docs), with positive and cross-tree controls seen to fail before
  crediting a clean sweep. Rewrite a useful generic policy sample; trimming the
  Grove sample is insufficient. No active adapter field or embedded registration
  survives. Historical records may retain opaque old fields without interpreting
  them as caller semantics; retired source history is the other explicit exception.
- Recheck and copy the actual contents of `~/.config/harness-dispatch` into
  `~/.config/ravel`, preserving both source and any destination edits. Preserve
  policy/settings format, routes, environment grants, bounds, local selections
  and access to existing history. Change only necessary SDK/import locations.
  Inspect the copied personal policy from this Grove workspace and another
  ordinary directory; do not activate a replacement sample or launch a model.
  Use a separate Ravel store at `~/.local/state/ravel/records.sqlite3`. Seed it
  once from a consistent SQLite backup of the selected legacy store, never a
  live file copy; preserve IDs, launches and observations. Preserve an existing
  destination and refuse conflicting imports rather than replacing it. Neither
  Ravel nor k7 upgrades/writes the legacy store, including one selected by an
  explicit state-dir setting. Document the explicit relocation for that case.
  This is a snapshot, not continuous replication: later old-supervisor records
  remain accessible through the retained old front/store. Record the snapshot
  boundary and a final catch-up import procedure before retiring that front.
  Test the old 22.1.0 front can still open, append and reap into its original
  store after Ravel writes its own; its application ID/version and existing
  immutable documents stay unchanged. Include a legacy late-reap fixture.
- Transfer executable-owned current-state documentation to Ravel alongside its
  runnable implementation: `docs/specs/harness-selection-and-execution.md`,
  the generic parts of the four decisions `harness-selection-is-owned-by-policy`,
  `policy-evaluation-precedes-the-launch`, `dispatch-supervises-the-harness` and
  `the-launched-child-is-a-job`, and
  `docs/design/harness-selection-and-execution/runtime-evidence.md` with its
  instruments/assets. Separate Grove caller clauses without importing Grove
  conventions into Ravel. Chase every moved source/test/script citation; keep
  the pinned-Bun upgrade/probe rule attached to its actual evidence. Grove's
  copies remain during expansion; k12 reconciles them to caller-only material.
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
grove-ravel-caller-k5. Release preparation is ravel-delivery-k4 and independent
publication is ravel-publication-k11;
controlled-session behavior is ravel-session-control-k7. Do not add either here.
Return to the session workspace to retire/commit this leaf, naming the Ravel
commit in the handoff. Keep the launch prompt's original ending binary usable.

## Public rename contract

Document and test this table in Ravel's ordinary CLI/SDK documentation before
caller migration. Old names are compatibility inputs or history only; they
never become the current output spelling.

| Surface | Current spelling / contract |
|---|---|
| Front and SDK import | `ravel`; `ravel/sdk` |
| Private worker/layout | `libexec/ravel/ravel-policy`, resolved from the front's real path |
| Config and local choice | `~/.config/ravel/{policy.ts,settings.json}`; `.ravel-choice` |
| Default state | `~/.local/state/ravel/records.sqlite3` |
| Harness authority/identity | `RAVEL_EXIT_FILE`, `RAVEL_RUN_ID`, `RAVEL_STATE_DIR` |
| Continue command | `ravel exit` |
| Caller ending report | `schemaVersion: 1`, `source: "ravel"`, observed ending `exit_signal` |
| Public arguments | Preserve existing documented arguments and config format, including `--exit-dir`, `--ending-file`, `--state-dir`, selection/context/confinement parameters |
| Legacy exit input | If `RAVEL_EXIT_FILE` is unset, `ravel exit` may use `HARNESS_DISPATCH_EXIT_FILE`; a present empty new variable disables fallback. Never signal both. Preserve the old empty-file/idempotent/no-supervisor behavior. |
| Guarding inherited controls | Scrub both old and new exit/run/state variables and Grove launch/retired signal controls from workers/tests as appropriate; no caller-specific grant advice belongs in generic CLI help |

Test the legacy exit input only on an isolated old 22.1.0 supervisor and temporary
channels. This compatibility path does not replace the old launch prompt's
mandated `harness-dispatch exit`; k6 preserves that command through installation.
