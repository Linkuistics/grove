# node-cutover-k10

## Goal

Release and install the checked binary and matching plugin, cut this tree over
to the node-file grammar, and hand the stopped loop back to the human for a
restart that selects the remaining migration work.

## Context

- Depends on `node-documentation-k9` and any review/integration it earns.
  This is the last code/release leaf before `grove-migration-k11`.
- `docs/RELEASING.md` owns the release route: prepare/check, the default
  colocated jj workspace, version cut, archives, repository and Homebrew tap
  publication, install verification. Read the current procedure and release
  tooling; do not infer shell commands or a release version from this brief.
- The current driver is installed 20.2.0, already running in memory; changing
  PATH binaries cannot upgrade it. Cached skills also change only on refresh.
  `plan-k1` decisions 7 and 10 and the root brief authorize the migration
  workflow and require this human stop/restart boundary.

## Done when

- Before asking for release approval, prepare a concrete reviewable release:
  finished changelog, version decision, green `bash scripts/check.sh`, release
  doctor result, exact commits/artifacts to publish, plugin-refresh route, and
  an inspected rename plan for this tree. Resolve routine failures first.
- Re-enumerate live groves under `~/Development`. Prepare temporary converted
  copies for new-binary preflight and compare selected permanent keys/outcomes
  to the untouched trees; terminal trees need no fabricated live selection.
  Expose malformed or unexpected shapes before publishing. Keep this workflow
  outside shipped code, with no product migration verb or dual-grammar reader.
- The human handoff states the sequence explicitly: stop the driving loop and
  other active Grove loops; cut/publish and install the release; refresh the
  plugin; rename this tree; verify it; re-run `grove`. Other groves remain
  stopped until `grove-migration-k11` verifies them. Prepare everything possible
  before this handoff; approval concerns the concrete publication and runtime
  switch, not routine edits already authorized.
- The release is actually published and installed, and both `grove` and
  `grove-llm` resolve to the intended build. Verify the installed plugin's
  bootstrap/format instructions against the published source. Follow the
  release procedure's integration and version/tag checks in the default
  workspace; keep unrelated work out of the release.
- This workspace's `.grove/` has `_BRIEF.md` at its root and exactly one
  `_<slug>.md` in every `NN-k<key>/` directory. Preserve node keys, leaf
  names/outcomes, bytes, headings and order; reject collisions before renames.
  Use bottom-up directory renames so deep descendants stay addressable. Keep
  a recoverable snapshot and item-by-item mapping outside shipped artifacts.
- Open this tree with the installed new binary and verify handles, brief
  chains and the remaining migration selection. Clear stale `GROVE_SIGNAL_FILE`
  for post-stop administrative probes; invoke `grove-llm` directly. Retire this
  leaf only after its release/install/rename work is done, using the new binary,
  and include the retirement and task-tree conversion in its sealed jj change.

## Notes

**This leaf ends by stopping the loop, without `grove-llm complete`.** Returning
a relaunch signal would send the in-memory 20.2.0 driver back into a tree it
cannot read. Coordinate the human stop before the runtime switch; a stop that
interrupts the agent leaves this handle live, to resume explicitly for the rest
of the same cutover work. Never mark it done merely because the preparation or
handoff is ready. Once the completed cutover is committed, return without a
signal and let the human start the newly installed `grove` for migration.

The four other known trees are `APIAnyware.add-ocaml-target`, `InTheLoop`,
`Writegood` and `grove.gh-issue-12`, all under `~/Development`. Do not let their
drivers restart between install and migration. If a prepared preflight reveals
product defects, insert precise repair work ahead of this leaf before entering
the cutover window. Any release source edit still owes its book in the same
commit; a release version edit may affect a manifest reconstructed by a book.
