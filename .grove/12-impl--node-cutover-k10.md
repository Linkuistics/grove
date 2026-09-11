# node-cutover-k10

## Goal

Release and install the checked binary and matching plugin, cut this tree over
to the node-file grammar, and reach the remaining migration work under a
compatible driver, through the human restart when the running driver is old.

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
- This is the secondary jj workspace
  `~/Development/grove.dont-put-item-title-in-folder-names`; the default is
  `~/Development/grove`. Release integration materializes this tracked tree
  there too. The secondary workspace owns the actual tree conversion; after
  its final sealed cutover change, advance the default workspace onto that
  change, preserving unrelated work. Include both copies in preflight and
  installed-reader verification. Do not start a driver in the default copy.

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
- The preparation session records the concrete plan, artifact identities,
  workspace commands and any still-needed publication/runtime approval in this
  leaf's running log. Preserve existing explicit authorization. If approval
  is outstanding at the unattended handoff, snapshot with `jj status`, leave
  this handle live, and return without a signal: that stops the driving loop.
  Do not retire or seal a completed-cutover commit at preparation alone.
- The human stops the other active Grove loops, records approval of the named
  plan and confirmation of those stops in this log (or supplies an explicit
  reply that the session records). After a stopped preparation session, the
  human re-runs `grove` in this secondary workspace. If both confirmations
  arrive while preparation is still running, that session may continue with
  execution below. Restart itself is not approval. With no recorded authorization,
  the resumed session stops again without publishing or signalling. Other
  groves remain stopped until `grove-migration-k11` verifies them.
- The resumed session rechecks the plan against current files, then performs
  the approved publication, installation, plugin refresh, conversion and
  verification. Its own driver remains waiting for it throughout; do not kill
  that driver mid-session. If the human executes the plan instead, they record
  what actually completed and ensure this tree and the plugin match the newly
  installed reader before restarting; the session verifies those results and
  performs only the remaining work. Never restart a new reader on this tree
  before conversion, or the old reader after conversion.
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
  Ensure that change descends from the release change without losing either
  workspace's edits, then advance the default workspace to the sealed result
  and verify its actual tree too. The release tag stays on the release cut.

## Notes

**Choose the ending from the mandate's running Grove version.** An unfinished
handoff always leaves the handle live and returns without a signal. After all
work is verified, retired and sealed, a session launched by 20.2.0 (or another
driver predating this grammar release) returns without `grove-llm complete`;
the human then starts the newly installed `grove` in this workspace for
migration. A resumed session already launched by the compatible new driver
uses ordinary `grove-llm complete` as its last action. PATH replacement does
not change which case applies. After an interruption, inspect the running log,
jj diff and installed versions before resuming this same live handle; if the
tree is partially converted, finish or restore from the recorded mapping
outside the loop before restarting a reader that can open it.

The four other known trees are `APIAnyware.add-ocaml-target`, `InTheLoop`,
`Writegood` and `grove.gh-issue-12`, all under `~/Development`. Do not let their
drivers restart between install and migration. If a prepared preflight reveals
product defects, insert precise repair work ahead of this leaf before entering
the cutover window. Any release source edit still owes its book in the same
commit; a release version edit may affect a manifest reconstructed by a book.
